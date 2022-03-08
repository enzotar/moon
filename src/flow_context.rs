use std::str::FromStr;
use std::sync::Arc;

use crate::model::{Db, GraphId, NodeId, RunStatusEntry};
use crate::Confirm;
use dashmap::DashMap;
use futures::executor::block_on;
use std::sync::Mutex;
use sunshine_core::msg::Action;
use sunshine_core::msg::QueryKind;
use sunshine_core::store::Datastore;
use sunshine_solana::FlowContext as InnerFlowContext;
use sunshine_solana::Schedule;
use sunshine_solana::RUN_ID_MARKER;
use tokio::runtime;
use tokio::sync::{mpsc, oneshot};
use uuid::Uuid;

#[derive(Debug)]
pub struct FlowContext {
    tx: mpsc::UnboundedSender<Packet>,
    run_id: Arc<Mutex<Uuid>>,
}

#[derive(Debug)]
struct Packet {
    cmd: Cmd,
    res: oneshot::Sender<()>,
}

#[derive(Debug)]
enum Cmd {
    Deploy(GraphId),
    Undeploy(GraphId),
    Stop,
}

impl FlowContext {
    pub fn new(
        db: Arc<dyn Datastore>,
        run_status: Arc<DashMap<NodeId, RunStatusEntry>>,
        req_id: Arc<Mutex<u64>>,
        graph_id: Arc<Mutex<GraphId>>,
    ) -> FlowContext {
        let run_id = Arc::new(Mutex::new(Uuid::new_v4()));

        let (tx, mut rx) = mpsc::unbounded_channel::<Packet>();

        let run_id_mod = run_id.clone();

        std::thread::spawn(move || {
            let threaded_rt = tokio::runtime::Builder::new_multi_thread()
                .enable_all()
                .build()
                .unwrap();

            let flow_ctx = InnerFlowContext::new(db.clone());

            std::fs::create_dir("SUNSHINE_LOGS").ok();

            let current_run_id = run_id_mod.clone();

            threaded_rt.spawn(async move {
                let mut interval = tokio::time::interval(tokio::time::Duration::from_secs(1));

                loop {
                    interval.tick().await;

                    let flow_node = {
                        let graph_id = graph_id.lock().unwrap();
                        block_on(db.execute(Action::Query(QueryKind::ReadNode(graph_id.0))))
                            .unwrap()
                            .into_node()
                            .unwrap()
                    };

                    let mut changed = false;

                    for edge in flow_node.outbound_edges {
                        let props = db.read_edge_properties(edge).await.unwrap();

                        if let Some(other_run_id) = props.get(RUN_ID_MARKER) {
                            {
                                let current_run_id = current_run_id.lock().unwrap();
                                if &current_run_id.to_string() != other_run_id.as_str().unwrap() {
                                    continue;
                                }
                            }

                            let log_graph = db.read_graph(edge.to).await.unwrap();

                            let log_content = format!("{:#?}", log_graph);

                            std::fs::write(
                                format!(
                                    "SUNSHINE_LOGS/{}.log",
                                    props.get("timestamp").unwrap().as_i64().unwrap(),
                                ),
                                log_content.as_bytes(),
                            )
                            .unwrap();

                            for node in log_graph.nodes {
                                let node_id = node
                                    .properties
                                    .get("original_node_id")
                                    .unwrap()
                                    .as_str()
                                    .unwrap();
                                let node_id = uuid::Uuid::from_str(node_id).unwrap();

                                let entry = RunStatusEntry {
                                    success: node
                                        .properties
                                        .get("success")
                                        .map(|s| s.as_bool().unwrap())
                                        .unwrap_or(false),
                                    error: node
                                        .properties
                                        .get("error")
                                        .map(|e| e.as_str().unwrap().to_owned()),
                                    print_output: node
                                        .properties
                                        .get("__print_output")
                                        .map(|e| e.as_str().unwrap().to_owned()),
                                    running: node.properties.contains_key("running"),
                                };

                                if let Some(before) =
                                    run_status.insert(NodeId(node_id), entry.clone())
                                {
                                    if before != entry {
                                        changed = true;
                                        println!(
                                            "run status: {:?}, {:?}",
                                            node.properties.get("kind").unwrap(),
                                            entry
                                        );
                                    }
                                } else {
                                    changed = true;
                                }
                            }
                        }
                    }
                    if changed {
                        let id = *req_id.lock().unwrap();
                        // println!("{}", id);
                        rid::post(Confirm::RequestRefresh(id));
                    }
                }
            });

            threaded_rt.block_on(async move {
                while let Some(packet) = rx.recv().await {
                    match packet.cmd {
                        Cmd::Deploy(flow_id) => {
                            let new_run_id = flow_ctx
                                .deploy_flow(Schedule::Once, flow_id.0)
                                .await
                                .unwrap();

                            if let Some(new_run_id) = new_run_id {
                                let mut run_id = run_id_mod.lock().unwrap();
                                *run_id = new_run_id;
                            }

                            packet.res.send(()).unwrap();
                        }
                        Cmd::Undeploy(flow_id) => {
                            flow_ctx.undeploy_flow(flow_id.0).ok();
                            {
                                let mut run_id = run_id_mod.lock().unwrap();
                                *run_id = Uuid::new_v4();
                            }
                            packet.res.send(()).unwrap();
                        }
                        Cmd::Stop => {
                            packet.res.send(()).unwrap();
                            break;
                        }
                    }
                }
            });
        });

        FlowContext { tx, run_id }
    }

    pub async fn deploy(&self, flow_id: GraphId) {
        let (tx, rx) = oneshot::channel();

        self.tx
            .send(Packet {
                cmd: Cmd::Deploy(flow_id),
                res: tx,
            })
            .unwrap();

        rx.await.unwrap();
    }

    pub async fn undeploy(&self, flow_id: GraphId) {
        let (tx, rx) = oneshot::channel();

        self.tx
            .send(Packet {
                cmd: Cmd::Undeploy(flow_id),
                res: tx,
            })
            .unwrap();

        rx.await.unwrap();
    }
}

impl Drop for FlowContext {
    fn drop(&mut self) {
        let (tx, rx) = oneshot::channel();

        self.tx
            .send(Packet {
                cmd: Cmd::Stop,
                res: tx,
            })
            .unwrap();

        block_on(rx).unwrap();
    }
}
