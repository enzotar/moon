use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;

use futures::executor::block_on;
use serde::Deserialize;
use serde::Serialize;
use sunshine_core::msg::*;
use sunshine_core::store::*;

use dashmap::DashMap;
use serde_json::{json, Value as JsonValue};
use std::sync::Mutex;
use sunshine_indra::store::generate_uuid_v1;
use sunshine_indra::store::DbConfig;
use sunshine_indra::store::DB;
use sunshine_solana::commands::simple;
use sunshine_solana::commands::solana;
use sunshine_solana::COMMAND_NAME_MARKER;
use sunshine_solana::{
    commands::simple::Command as SimpleCommand, commands::simple::CommandKind as SimpleCommandKind,
    commands::CommandKind, CommandConfig, ContextConfig, COMMAND_MARKER, CTX_EDGE_MARKER,
    CTX_MARKER, INPUT_ARG_NAME_MARKER, OUTPUT_ARG_NAME_MARKER, START_NODE_MARKER,
};
use uuid::Uuid;

use std::str::FromStr;

use crate::command::commands_map;
use crate::command::INPUT_SIZE;
use crate::flow_context::FlowContext;

//use crate::model_ext::WidgetType;

use crate::event::Coords;
use crate::utils::Rect;

pub const COORDS_MARKER: &str = "COORDS_MARKER";
pub const DIMENSIONS_MARKER: &str = "DIMENSIONS_MARKER";
pub const BLOCK_MARKER: &str = "BLOCK_MARKER";
pub const TEXT_INPUT_MARKER: &str = "TEXT_INPUT_MARKER";
pub const DATA_MARKER: &str = "DATA_MARKER";
pub const BLOCK_TO_CMD_EDGE_MARKER: &str = "BLOCK_TO_CMD_EDGE_MARKER";
pub const FLOW_GRAPH_MARKER: &str = "FLOW_GRAPH_MARKER";
pub const TEXT_MARKER: &str = "TEXT_MARKER";
pub const REQ_ID: &str = "REQ_ID";

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct GraphId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct NodeId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct EdgeId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct NodeEdgeId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct InputId(pub Uuid);

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct OutputId(pub Uuid);

#[rid::model]
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct GraphEntry {
    id: String,
    name: String,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub enum PortId {
    Input(InputId),
    Output(OutputId),
}

#[derive(Clone)]
pub struct Db(Arc<DB>);

#[derive(Debug)]
pub struct Model {
    db: Db,
    pub graph_list: Vec<GraphEntry>,
    graph_id: Arc<Mutex<GraphId>>,
    context_node_id: NodeId,
    nodes: HashMap<NodeId, NodeModel>,
    node_edges: HashMap<NodeEdgeId, NodeEdgeModel>,
    flow_edges: HashMap<EdgeId, FlowEdgeModel>,
    inputs: HashMap<InputId, InputModel>,
    outputs: HashMap<OutputId, OutputModel>,
    flow_context: FlowContext,
    pub run_status: Arc<DashMap<NodeId, bool>>,
    pub req_id: Arc<Mutex<u64>>,
}

impl fmt::Debug for Db {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Db").finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type")] // https://serde.rs/container-attrs.html
pub enum NodeModel {
    Widget(WidgetNodeData),
    // Data(DataNodeData),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct WidgetNodeData {
    pub coords: Coords,
    pub dimensions: NodeDimensions,
    pub command_name: Option<String>,
    pub kind: WidgetKind,
    pub text: String,
}

impl WidgetNodeData {
    fn new_command(command_name: &str, kind: WidgetKind, width: i64, height: i64) -> Self {
        Self {
            coords: Coords { x: 0, y: 0 },
            dimensions: NodeDimensions { width, height },
            command_name: Some(command_name.to_owned()),
            kind,
            text: String::new(),
        }
    }

    // fn new_const_command(value: Value, kind: WidgetKind, width: i64, height: i64) -> Self {
    //     Self {
    //         coords: Coords { x: 0, y: 0 },
    //         dimensions: NodeDimensions { width, height },
    //         kind,
    //         text: match value {
    //             Value::Integer(_) => todo!(),
    //             Value::Keypair(_) => todo!(),
    //             Value::String(string) => string,
    //             Value::NodeId(_) => todo!(),
    //             Value::DeletedNode(_) => todo!(),
    //             Value::Pubkey(_) => todo!(),
    //             Value::Success(_) => todo!(),
    //             Value::Balance(_) => todo!(),
    //             Value::U8(_) => todo!(),
    //             Value::U16(_) => todo!(),
    //             Value::U64(_) => todo!(),
    //             Value::F64(_) => todo!(),
    //             Value::Bool(_) => todo!(),
    //             Value::StringOpt(_) => todo!(),
    //             Value::Empty => todo!(),
    //             Value::NodeIdOpt(_) => todo!(),
    //             Value::NftCreators(_) => todo!(),
    //             Value::MetadataAccountData(_) => todo!(),
    //         },
    //     }
    // }

    fn new_block(coords: Coords) -> Self {
        Self {
            coords,
            dimensions: NodeDimensions {
                height: 75,
                width: 300,
            },
            command_name: None,
            kind: WidgetKind::Basic(BasicWidgetKind::Block),
            text: String::new(),
        }
    }

    fn new_text_input() -> Self {
        Self {
            coords: Coords { x: 0, y: 0 },
            dimensions: NodeDimensions {
                height: 70,
                width: 300,
            },
            command_name: None,
            kind: WidgetKind::Basic(BasicWidgetKind::TextInput),
            text: String::new(),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct DataNodeData {}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct NodeEdgeModel {
    pub from: NodeId,
    pub to: NodeId,
    pub data: EdgeModelData,
}

/// From, to coords needed to render the edges on Flutter side
///
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct EdgeModelData {
    pub edge_type: EdgeType,
    pub from_coords: Option<Coords>,
    pub to_coords: Option<Coords>,
}
#[derive(Debug, Clone, Eq, Hash, PartialEq, Deserialize, Serialize)]
pub struct NodeDimensions {
    pub height: i64,
    pub width: i64,
}

#[derive(Debug, Clone, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub enum EdgeType {
    Child,
    Data,
    Flow,
}
#[derive(Clone, Debug)]
pub struct NodeAndDataResult {
    // pub data_node_id: NodeId,
    pub widget_node_id: NodeId,
    // pub edge_id: NodeEdgeId,
}

/// Model for input output edges
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct FlowEdgeModel {
    pub input_id: InputId,
    pub output_id: OutputId,
    pub edge_type: EdgeType,
    pub db_edge_id: EdgeId,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct InputModel {
    pub parent_node_id: NodeId,
    pub command_id: NodeId,
    pub local_coords: Coords,
    pub label: String,
    pub index: i64,
}

#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct OutputModel {
    pub parent_node_id: NodeId,
    pub command_id: NodeId,
    pub local_coords: Coords,
    pub label: String,
    pub index: i64,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum WidgetKind {
    Basic(BasicWidgetKind),
    Command(CommandConfig),
    Context(ContextConfig),
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BasicWidgetKind {
    Block,
    TextInput,
    Dummy,
}

impl Model {
    pub fn new(db_path: String) -> Self {
        // database configuration
        let cfg = DbConfig { db_path };

        let db = DB::new(&cfg).unwrap();
        let db = Arc::new(db);

        // Get list of graphs in database

        let graph_list = block_on(db.execute(Action::Query(QueryKind::ListGraphs)))
            .unwrap()
            .into_node_list()
            .unwrap()
            .into_iter()
            .filter(|(_, properties)| properties.contains_key(FLOW_GRAPH_MARKER))
            .map(|(node_id, properties)| GraphEntry {
                id: node_id.to_string(),
                name: properties.get("name").unwrap().as_str().unwrap().to_owned(),
            })
            .collect::<Vec<_>>();

        // load graph id or create a new one

        let graph_id = match graph_list.len() {
            0 => {
                let properties = json!({
                    "name":&Self::random_name(),
                    FLOW_GRAPH_MARKER: true,
                });

                let properties = match properties {
                    JsonValue::Object(props) => props,
                    _ => unreachable!(),
                };

                let graph_id = block_on(db.execute(Action::CreateGraph(properties)))
                    .unwrap()
                    .as_id()
                    .unwrap();

                create_wallet_and_context(Db(db.clone()), GraphId(graph_id));

                graph_id
            }
            _ => Uuid::from_str(&graph_list.last().unwrap().id).unwrap(),
        };
        // let graph_id = match graph_list.len() {
        //     0 =>
        //     _ =>
        // };

        let graph_list = block_on(db.execute(Action::Query(QueryKind::ListGraphs)))
            .unwrap()
            .into_node_list()
            .unwrap()
            .into_iter()
            .filter(|(_, properties)| properties.contains_key(FLOW_GRAPH_MARKER))
            .map(|(node_id, properties)| GraphEntry {
                id: node_id.to_string(),
                name: properties.get("name").unwrap().as_str().unwrap().to_owned(),
            })
            .collect::<Vec<_>>();

        let context_node_id = NodeId(generate_uuid_v1());

        let run_status = Arc::new(DashMap::new());
        let req_id = Arc::new(Mutex::new(u64::default()));

        let graph_id = Arc::new(Mutex::new(GraphId(graph_id)));

        let mut model = Self {
            db: Db(db.clone()),
            graph_id: graph_id.clone(),
            context_node_id, // this will be replaced when we call read_graph
            nodes: HashMap::new(),
            flow_edges: HashMap::new(),
            node_edges: HashMap::new(),
            inputs: HashMap::new(),
            outputs: HashMap::new(),
            graph_list,
            flow_context: FlowContext::new(
                db.clone(),
                run_status.clone(),
                req_id.clone(),
                graph_id.clone(),
            ),
            run_status,
            req_id,
        };

        model.read_graph(model.graph_id());

        assert_ne!(context_node_id, model.context_node_id);

        model
    }

    /// GETTERS
    ///
    ///

    /// DB
    ///
    pub fn db(&self) -> &Arc<DB> {
        &self.db.0
    }

    pub fn get_node(&self, node_id: &NodeId) -> Option<&NodeModel> {
        self.nodes.get(node_id)
    }

    pub fn nodes(&self) -> &HashMap<NodeId, NodeModel> {
        &self.nodes
    }

    pub fn inputs(&self) -> &HashMap<InputId, InputModel> {
        &self.inputs
    }

    pub fn outputs(&self) -> &HashMap<OutputId, OutputModel> {
        &self.outputs
    }

    pub fn flow_edges(&self) -> &HashMap<EdgeId, FlowEdgeModel> {
        &self.flow_edges
    }

    pub fn node_edges(&self) -> &HashMap<NodeEdgeId, NodeEdgeModel> {
        &self.node_edges
    }

    pub fn graph_id(&self) -> GraphId {
        let graph_id = self.graph_id.lock().unwrap();

        *graph_id
    }

    // pub fn save_req_id(&mut self, req_id: u64) {
    //     let mut props = Properties::new();

    //     props.insert(REQ_ID.into(), serde_json::to_value(req_id).unwrap());

    //     block_on(self.db.0.execute(Action::Mutate(
    //         self.graph_id().0,
    //         MutateKind::CreateNode(props),
    //     )))
    //     .unwrap();
    // }

    // TODO how to pass req_id to refresh UI?
    pub fn deploy(&self) {
        /*
        let graph = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadGraph(self.graph_id().0))),
        )
        .unwrap()
        .into_graph()
        .unwrap();

        println!("{:#?}", graph);
        */

        self.run_status.clear();
        block_on(self.flow_context.deploy(self.graph_id()));
    }

    pub fn undeploy(&self) {
        self.run_status.clear();
        block_on(self.flow_context.undeploy(self.graph_id()));

        // TODO: refresh ui
    }

    pub fn iter_widget_nodes(&self) -> impl Iterator<Item = (&NodeId, &WidgetNodeData)> {
        self.nodes.iter().filter_map(|(node_id, node)| match node {
            NodeModel::Widget(data) => Some((node_id, data)),
            //NodeModel::Data(_) => None,
        })
    }

    pub fn iter_node_edges(&self) -> impl Iterator<Item = (&NodeEdgeId, &NodeEdgeModel)> {
        self.node_edges.iter()
    }

    fn random_name() -> String {
        use bip39::{Language, Mnemonic, MnemonicType};

        let mnemonic = Mnemonic::new(MnemonicType::Words12, Language::English);
        let phrase = mnemonic.into_phrase();

        phrase.split(' ').next().unwrap().to_owned()
    }

    pub fn new_graph(&mut self) {
        self.run_status.clear();

        let properties = json!({
            "name": &Self::random_name(),
            FLOW_GRAPH_MARKER: true,
        });

        let properties = match properties {
            JsonValue::Object(props) => props,
            _ => unreachable!(),
        };

        let graph_id = block_on(self.db.0.execute(Action::CreateGraph(properties)))
            .unwrap()
            .as_id()
            .unwrap();

        create_wallet_and_context(self.db.clone(), GraphId(graph_id));

        self.read_graph(GraphId(graph_id));

        self.graph_list = block_on(self.db.0.execute(Action::Query(QueryKind::ListGraphs)))
            .unwrap()
            .into_node_list()
            .unwrap()
            .into_iter()
            .filter(|(_, properties)| properties.contains_key(FLOW_GRAPH_MARKER))
            .map(|(node_id, properties)| GraphEntry {
                id: node_id.to_string(),
                name: properties.get("name").unwrap().as_str().unwrap().to_owned(),
            })
            .collect::<Vec<_>>();
    }

    pub fn read_graph(&mut self, graph_id: GraphId) {
        self.undeploy();
        self.run_status.clear();

        // get graph, nodes, and edges
        let graph = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadGraph(graph_id.0))),
        )
        .unwrap()
        .into_graph()
        .unwrap();

        {
            let mut graph_id_ref = self.graph_id.lock().unwrap();

            *graph_id_ref = graph_id
        }

        self.nodes = HashMap::new();

        let get_widget_kind = |properties: &Properties| {
            if let Some(command_config) = properties.get(COMMAND_MARKER) {
                let command_config = serde_json::from_value(command_config.clone()).unwrap();

                return Some(WidgetKind::Command(command_config));
            }

            if let Some(context_config) = properties.get(CTX_MARKER) {
                let context_config = serde_json::from_value(context_config.clone()).unwrap();

                return Some(WidgetKind::Context(context_config));
            }

            if properties.get(BLOCK_MARKER).is_some() {
                return Some(WidgetKind::Basic(BasicWidgetKind::Block));
            }
            if properties.get(TEXT_INPUT_MARKER).is_some() {
                return Some(WidgetKind::Basic(BasicWidgetKind::TextInput));
            }

            None
        };

        for node in graph.nodes.iter() {
            let kind = match get_widget_kind(&node.properties) {
                Some(WidgetKind::Context(_)) => {
                    self.context_node_id = NodeId(node.node_id);
                    continue;
                }
                Some(widget_kind) => widget_kind,
                None => continue,
            };

            // dbg!(node.properties.clone());
            let coords = node.properties.get(COORDS_MARKER).unwrap();
            let coords = serde_json::from_value(coords.clone()).unwrap();
            let dimensions = node.properties.get(DIMENSIONS_MARKER).unwrap();
            let dimensions = serde_json::from_value(dimensions.clone()).unwrap();

            // let text = match kind {
            //     WidgetKind::Basic(_) => {}
            //     WidgetKind::Command(cfg) => match cfg {
            //         CommandConfig::Simple(cmd) => match cmd {
            //             SimpleCommand::Const(value) => {
            //                 let text = node.properties.get(TEXT_MARKER).unwrap(); //unwrap_or(&empty_text);
            //                 // let text = text.into();
            //                 let text = serde_json::from_value(text.clone()).unwrap();
            //                 text
            //             }
            //             SimpleCommand::Print => {}
            //             SimpleCommand::HttpRequest(_) => {}
            //             SimpleCommand::JsonExtract(_) => {}
            //         },
            //         CommandConfig::Solana(_) => {}
            //     },
            //     WidgetKind::Context(_) => {}
            // };
            //WidgetKind::Command(CommandConfig::Simple(SimpleCommandKind::Const)) {}
            let empty_text = JsonValue::String("".to_string());
            let text = node.properties.get(TEXT_MARKER).unwrap_or(&empty_text);
            let text = serde_json::from_value(text.clone()).unwrap();

            let command_name = node.properties.get(COMMAND_NAME_MARKER).unwrap();
            let command_name = match command_name {
                JsonValue::Null => None,
                JsonValue::String(name) => Some(name.clone()),
                JsonValue::Bool(_) => panic!(),
                JsonValue::Number(_) => panic!(),
                JsonValue::Array(_) => panic!(),
                JsonValue::Object(_) => panic!(),
            };

            self.nodes.insert(
                NodeId(node.node_id),
                NodeModel::Widget(WidgetNodeData {
                    kind,
                    command_name,
                    coords,
                    dimensions,
                    text,
                }),
            );
        }

        self.node_edges = HashMap::new();

        for node in graph.nodes.iter() {
            for edge in node.inbound_edges.iter() {
                let props = block_on(
                    self.db
                        .0
                        .execute(Action::Query(QueryKind::ReadEdgeProperties(*edge))),
                )
                .unwrap()
                .into_properties()
                .unwrap();

                if props.get(BLOCK_TO_CMD_EDGE_MARKER).is_none() {
                    continue;
                }

                self.node_edges.insert(
                    NodeEdgeId(edge.id),
                    NodeEdgeModel {
                        from: NodeId(edge.from),
                        to: NodeId(edge.to),
                        data: EdgeModelData {
                            edge_type: EdgeType::Child,
                            from_coords: None,
                            to_coords: None,
                        },
                    },
                );
            }
        }

        self.inputs = HashMap::new();
        self.outputs = HashMap::new();

        // INPUT OUTPUT EDGES
        for (node_id, node) in self.nodes.iter() {
            let node = match &node {
                NodeModel::Widget(w) => w,
                _ => continue,
            };

            let width = node.dimensions.width;

            let cmd = match &node.kind {
                WidgetKind::Command(cmd) => cmd,
                _ => continue,
            };

            let command_name = node.command_name.as_ref().unwrap(); // because it is command

            //get parent coords
            let coords = Coords { x: 0, y: 0 };

            let (_, edge) = self
                .node_edges
                .iter()
                .find(|(_, edge)| &edge.to == node_id)
                .unwrap();
            let block_id = edge.from;

            let (inputs, outputs) =
                Self::generate_ports(*node_id, command_name, cmd, (block_id, coords), width);

            for input in inputs {
                self.inputs.insert(InputId(generate_uuid_v1()), input);
            }

            for output in outputs {
                self.outputs.insert(OutputId(generate_uuid_v1()), output);
            }
        }

        self.flow_edges = HashMap::new();

        for (&node_id, node) in self.nodes.iter() {
            let node = match &node {
                NodeModel::Widget(w) => w,
                _ => continue,
            };

            // let from_cmd = match &node.kind {
            //     WidgetKind::Command(cmd) => cmd,
            //     _ => continue,
            // };

            for edge in graph
                .nodes
                .iter()
                .find(|node| node.node_id == node_id.0)
                .unwrap()
                .outbound_edges
                .iter()
            {
                let props = block_on(
                    self.db
                        .0
                        .execute(Action::Query(QueryKind::ReadEdgeProperties(*edge))),
                )
                .unwrap()
                .into_properties()
                .unwrap();

                if props.get(INPUT_ARG_NAME_MARKER).is_none() {
                    continue;
                }

                let (input_id, _) = self
                    .inputs
                    .iter()
                    .find(|(_input_id, input)| {
                        input.command_id.0 == edge.to
                            && input.label
                                == props.get(INPUT_ARG_NAME_MARKER).unwrap().as_str().unwrap()
                    })
                    .unwrap();

                let (output_id, _) = self
                    .outputs
                    .iter()
                    .find(|(_output_id, output)| {
                        output.command_id.0 == edge.from
                            && output.label
                                == props.get(OUTPUT_ARG_NAME_MARKER).unwrap().as_str().unwrap()
                    })
                    .unwrap();

                self.flow_edges.insert(
                    EdgeId(edge.id),
                    FlowEdgeModel {
                        input_id: *input_id,
                        output_id: *output_id,
                        edge_type: EdgeType::Flow,
                        db_edge_id: EdgeId(edge.id),
                    },
                );
            }
        }
    }

    // has both the node id of the block and of the command
    pub fn generate_ports(
        node_id: NodeId,
        command_name: &str,
        cfg: &CommandConfig,
        parent: (NodeId, Coords),
        width: i64,
    ) -> (Vec<InputModel>, Vec<OutputModel>) {
        const INPUT_OFFSET: i64 = 50;
        const Y_INPUT_OFFSET: i64 = 30; // offset for block title
        const X_INPUT_OFFSET: i64 = 0;

        // get parent coordinates and add them

        let (parent_id, _coords) = parent;

        let commands_map = commands_map();
        let command = commands_map.get(command_name).unwrap();
        let mut inputs: Vec<_> = command.inputs().iter().map(|input| input.name).collect();
        let outputs: Vec<_> = command.outputs().iter().map(|output| output.name).collect();

        let mut y = Y_INPUT_OFFSET - INPUT_OFFSET;
        let mut index = 0;
        let input = |label: &str| {
            y += INPUT_OFFSET;
            index += 1;
            InputModel {
                index,
                parent_node_id: parent_id,
                local_coords: Coords {
                    x: X_INPUT_OFFSET, // - INPUT_OFFSET,
                    y: y,
                },
                label: label.to_owned(),
                command_id: node_id,
            }
        };

        let mut y = Y_INPUT_OFFSET - INPUT_OFFSET;
        let mut index = 0;

        let output = |label: &str| {
            y += INPUT_OFFSET;
            index += 1;
            OutputModel {
                index,

                parent_node_id: parent_id,
                local_coords: Coords {
                    x: width - INPUT_OFFSET,
                    y: y,
                },
                label: label.to_owned(),
                command_id: node_id,
            }
        };

        (
            inputs.iter().copied().map(input).collect(),
            outputs.iter().copied().map(output).collect(),
        )
    }

    pub fn set_node_text(&mut self, node_id: &NodeId, text: String) {
        let node = self.nodes.get_mut(node_id).unwrap();
        let node = match node {
            NodeModel::Widget(node) => node,
        };
        node.text = text;

        // update db
    }

    pub fn save_text_to_db(&mut self, node_id: &NodeId, text: &str) {
        let mut props = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(node_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap()
        .properties;

        dbg!(props.clone());
        //update 'text' key
        props
            .insert(TEXT_MARKER.into(), JsonValue::String(text.to_string()))
            .unwrap();

        dbg!(props.clone());

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((node_id.0, props)),
        )))
        .unwrap();
    }

    pub fn update_const_command(&mut self, node_id: NodeId, text: &str) {
        // dbg!(text.clone());
        let cfg: SimpleCommand = match serde_json::from_str(text) {
            Ok(cfg) => cfg,
            _ => return,
        };

        let cfg = CommandConfig::Simple(cfg);

        let mut props = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(node_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap()
        .properties;

        props
            .insert(COMMAND_MARKER.into(), serde_json::to_value(&cfg).unwrap())
            .unwrap();

        //update 'text' key
        props
            .insert(TEXT_MARKER.into(), JsonValue::String(text.to_string()))
            .unwrap();

        dbg!(props.clone());

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((node_id.0, props)),
        )))
        .unwrap();
    }

    /*
         let input = InputModel {
                       node_id,
                       local_coords: Coords { x: 10, y: 40 },
                       label: "Input 2".to_owned(),
                   };
                   let id = InputId(generate_uuid_v1());
                   let prev = self.inputs.insert(id, input);
                   assert!(prev.is_none());

                   let output = OutputModel {
                       node_id,
                       local_coords: Coords { x: 40, y: 10 },
                       label: "Output 1".to_owned(),
                   };
    */

    /// CREATE NODE
    ///
    pub fn add_node(&mut self, node: NodeModel, parent: Option<(NodeId, Coords)>) -> NodeId {
        let data = match &node {
            NodeModel::Widget(data) => data,
        };
        let width = data.dimensions.width;

        let node_id = NodeId(generate_uuid_v1());
        match &node {
            NodeModel::Widget(widget_node_data) => {
                let mut props = match widget_node_data.kind.clone() {
                    WidgetKind::Command(config) => {
                        let command_name = widget_node_data.command_name.as_ref().unwrap();
                        let (parent_node_id, coords) = parent.unwrap();
                        let (inputs, outputs) = Self::generate_ports(
                            node_id,
                            &command_name,
                            &config,
                            (parent_node_id, coords),
                            width,
                        );
                        //
                        for input in inputs {
                            let id = InputId(generate_uuid_v1());
                            self.inputs.insert(id, input);
                        }
                        for output in outputs {
                            let id = OutputId(generate_uuid_v1());
                            self.outputs.insert(id, output);
                        }

                        // save to db
                        let mut props = Properties::new();

                        props.insert(COMMAND_MARKER.into(), serde_json::to_value(config).unwrap());
                        props.insert(
                            COMMAND_NAME_MARKER.into(),
                            JsonValue::String(command_name.clone()),
                        );
                        props.insert(START_NODE_MARKER.into(), JsonValue::Bool(true));
                        props.insert(TEXT_MARKER.into(), JsonValue::String(String::new()));

                        props
                    }
                    WidgetKind::Context(_) => todo!(), //TODO don't do this, new context created on start
                    WidgetKind::Basic(kind) => match kind {
                        BasicWidgetKind::Block => {
                            // save to db
                            let mut props = Properties::new();

                            props.insert(BLOCK_MARKER.into(), JsonValue::Bool(true));
                            props.insert(COMMAND_NAME_MARKER.into(), JsonValue::Null);
                            props
                        }
                        BasicWidgetKind::TextInput => {
                            // save to db
                            let mut props = Properties::new();

                            props.insert(TEXT_INPUT_MARKER.into(), JsonValue::Bool(true));
                            props.insert(TEXT_MARKER.into(), JsonValue::String(String::new()));
                            props.insert(COMMAND_NAME_MARKER.into(), JsonValue::Null);
                            props
                        }
                        BasicWidgetKind::Dummy => Properties::new(),
                    },
                };

                props.insert(
                    DIMENSIONS_MARKER.into(),
                    serde_json::to_value(widget_node_data.dimensions.clone()).unwrap(),
                );

                props.insert(
                    COORDS_MARKER.into(),
                    serde_json::to_value(widget_node_data.coords.clone()).unwrap(),
                );

                block_on(self.db.0.execute(Action::Mutate(
                    self.graph_id().0,
                    MutateKind::CreateNodeWithId((node_id.0, props)),
                )))
                .unwrap();

                match widget_node_data.kind.clone() {
                    WidgetKind::Command(c) => match c.kind() {
                        CommandKind::Solana(_) => {
                            let mut properties = Properties::new();

                            properties.insert(CTX_EDGE_MARKER.into(), JsonValue::Bool(true));

                            block_on(self.db.0.execute(Action::Mutate(
                                self.graph_id().0,
                                MutateKind::CreateEdge(CreateEdge {
                                    from: self.context_node_id.0,
                                    to: node_id.0,
                                    properties,
                                }),
                            )))
                            .unwrap();
                        }
                        _ => (),
                    },
                    _ => (),
                }
            } // NodeModel::Data(_) => {}
        };

        //
        // dbg!(self.inputs.clone());
        let prev = self.nodes.insert(node_id, node);
        assert!(prev.is_none());

        node_id
    }

    // TODO correct for nested commands
    pub fn remove_node(&mut self, node_id: NodeId) {
        // DELETE FROM MODEL

        // node
        self.nodes.remove_entry(&node_id);

        // TODO: remove all node where parent_node_id = node_id
        // flow edges
        self.flow_edges = self
            .flow_edges
            .drain()
            .filter(|(_, edge)| {
                let input = self.inputs.get(&edge.input_id).unwrap();
                let output = self.outputs.get(&edge.output_id).unwrap();
                input.parent_node_id != node_id && output.parent_node_id != node_id
            })
            .collect();

        // node edges
        self.node_edges = self
            .node_edges
            .drain()
            .filter(|(_, node_edge_model)| {
                let output = node_edge_model.from;
                let input = node_edge_model.to;
                input != node_id && output != node_id
            })
            .collect();

        // DELETE FROM DB

        // node and all inbound/outbound edges
        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::DeleteNode(node_id.0),
        )))
        .unwrap();
    }

    // ADD INPUT OUTPUT EDGE
    // TODO when creating edge, toggle start marker
    pub fn add_or_remove_flow_edge(
        &mut self,
        input_id: InputId,
        output_id: OutputId,
    ) -> Option<EdgeId> {
        for (&edge_id, edge) in &self.flow_edges {
            if edge.input_id == input_id && edge.output_id == output_id {
                block_on(self.db.0.execute(Action::Mutate(
                    self.graph_id().0,
                    MutateKind::DeleteEdge(Edge {
                        id: edge.db_edge_id.0,
                        from: self.outputs.get(&edge.output_id).unwrap().command_id.0,
                        to: self.inputs.get(&edge.input_id).unwrap().command_id.0,
                    }),
                )))
                .unwrap();

                self.flow_edges.remove(&edge_id);
                return None;
            }
        }

        let input_model = self.inputs.get(&input_id).unwrap();
        let output_model = self.outputs.get(&output_id).unwrap();

        let mut properties = serde_json::Map::new();

        properties.insert(
            INPUT_ARG_NAME_MARKER.into(),
            serde_json::to_value(&input_model.label).unwrap(),
        );
        properties.insert(
            OUTPUT_ARG_NAME_MARKER.into(),
            serde_json::to_value(&output_model.label).unwrap(),
        );
        dbg!(properties.clone());

        let edge_id = block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::CreateEdge(CreateEdge {
                from: output_model.command_id.0,
                to: input_model.command_id.0,
                properties,
            }),
        )))
        .unwrap()
        .as_id()
        .unwrap();

        let mut props = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(input_model.command_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap()
        .properties;

        props.remove(START_NODE_MARKER);

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((input_model.command_id.0, props)),
        )))
        .unwrap();

        let edge_id = EdgeId(edge_id);

        // Save to model
        let prev = self.flow_edges.insert(
            edge_id,
            FlowEdgeModel {
                input_id,
                output_id,
                edge_type: EdgeType::Flow,
                db_edge_id: EdgeId(edge_id.0),
            },
        );
        assert!(prev.is_none());

        Some(edge_id)
    }

    /// CREATE NODE EDGE
    /// inserts to db and model
    ///
    pub fn add_node_edge(&mut self, edge: NodeEdgeModel) -> NodeEdgeId {
        let mut properties = Properties::new();

        properties.insert(BLOCK_TO_CMD_EDGE_MARKER.into(), JsonValue::Bool(true));
        dbg!(edge.clone());
        let edge_id = block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::CreateEdge(CreateEdge {
                from: edge.from.0,
                to: edge.to.0,
                properties,
            }),
        )))
        .unwrap()
        .as_id()
        .unwrap();

        let edge_id = NodeEdgeId(edge_id);

        let prev = self.node_edges.insert(edge_id, edge);
        assert!(prev.is_none());

        edge_id
    }

    /// UPDATE NODE
    ///
    pub fn set_node_coords(&mut self, node_id: &NodeId, coords: Coords) {
        let node = self.nodes.get_mut(node_id).unwrap();

        match node {
            NodeModel::Widget(ref mut data) => {
                data.coords = coords.clone();
            } // NodeModel::Data(_) => panic!(),
        };

        let mut properties = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(node_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap()
        .properties;

        properties.insert(COORDS_MARKER.into(), serde_json::to_value(&coords).unwrap());

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((node_id.0, properties)),
        )))
        .unwrap();
    }

    /// UPDATE NODE DIMENSIONS
    /// some test /command more text
    ///
    pub fn set_node_dimensions(&mut self, node_id: &NodeId, dimensions: NodeDimensions) {
        let node = self.nodes.get_mut(node_id).unwrap();

        match node {
            NodeModel::Widget(ref mut data) => {
                data.dimensions = dimensions.clone();
            } // NodeModel::Data(_) => panic!(),
        };

        let mut properties = block_on(
            self.db
                .0
                .execute(Action::Query(QueryKind::ReadNode(node_id.0))),
        )
        .unwrap()
        .into_node()
        .unwrap()
        .properties;

        properties.insert(
            DIMENSIONS_MARKER.into(),
            serde_json::to_value(&dimensions).unwrap(),
        );

        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::UpdateNode((node_id.0, properties)),
        )))
        .unwrap();
    }

    /// CREATE WIDGET NODE and DATA NODE
    ///
    pub fn create_node(
        &mut self,
        widget_node_data: WidgetNodeData,
        parent: Option<(NodeId, Coords)>,
    ) -> NodeId {
        let node_id = match parent {
            Some((parent_node_id, coords)) => {
                let node = NodeModel::Widget(widget_node_data.clone());
                let widget_node_id = self.add_node(node, Some((parent_node_id, coords)));

                widget_node_id
            }
            None => {
                let node = NodeModel::Widget(widget_node_data.clone());
                let widget_node_id = self.add_node(node, None);

                widget_node_id
            }
        };

        node_id
    }

    //
    pub fn add_child_edge(
        &mut self,
        parent_widget_node_id: NodeId,
        child_widget_node_id: NodeId,
    ) -> NodeEdgeId {
        let widget_edge_id = self.add_node_edge(NodeEdgeModel {
            from: parent_widget_node_id,
            to: child_widget_node_id,
            data: EdgeModelData {
                edge_type: EdgeType::Child,
                from_coords: None,
                to_coords: None,
            },
        });
        widget_edge_id
    }

    pub fn next_input_at<'a>(
        &'a self,
        coords: &'a Coords,
    ) -> Option<(&'a InputId, &'a InputModel)> {
        self.inputs_at(coords).next()
    }

    pub fn next_output_at<'a>(
        &'a self,
        coords: &'a Coords,
    ) -> Option<(&'a OutputId, &'a OutputModel)> {
        self.outputs_at(coords).next()
    }

    pub fn next_input_or_output_at<'a>(&'a self, coords: &'a Coords) -> Option<PortId> {
        self.next_input_at(coords)
            .map(|(input_id, _)| PortId::Input(*input_id))
            .or_else(|| {
                self.next_output_at(coords)
                    .map(|(output_id, _)| PortId::Output(*output_id))
            })
    }

    pub fn next_movable_widget_node_at<'a>(
        &'a self,
        coords: &'a Coords,
    ) -> Option<(&'a NodeId, &'a WidgetNodeData)> {
        self.movable_widget_nodes_at(coords).next()
    }

    pub fn movable_widget_nodes_at<'a>(
        &'a self,
        coords: &'a Coords,
    ) -> impl Iterator<Item = (&'a NodeId, &'a WidgetNodeData)> {
        self.iter_widget_nodes().filter(|(_node_id, node)| {
            //dbg!(node);
            match node.kind {
                WidgetKind::Basic(BasicWidgetKind::Block) => (),
                _ => return false,
            }

            let rect = Rect {
                x1: node.coords.x,
                x2: node.coords.x + node.dimensions.width,
                y1: node.coords.y,
                y2: node.coords.y + node.dimensions.height,
            };

            rect.contains(coords.x, coords.y)
        })
    }

    // TODO not used, remove or keep?
    // pub fn command_at<'a>(&'a self, coords: &'a Coords) -> NodeId {
    //     *self
    //         .iter_widget_nodes()
    //         .filter(|(_node_id, node)| {
    //             match node.kind {
    //                 WidgetKind::Command(_) => (),
    //                 _ => return false,
    //             }

    //             let rect = Rect {
    //                 x1: node.coords.x,
    //                 x2: node.coords.x + node.dimensions.width,
    //                 y1: node.coords.y,
    //                 y2: node.coords.y + node.dimensions.height,
    //             };

    //             rect.contains(coords.x, coords.y)
    //         })
    //         .next()
    //         .unwrap()
    //         .0
    // }

    pub fn inputs_at<'a>(
        &'a self,
        coords: &'a Coords,
    ) -> impl Iterator<Item = (&'a InputId, &'a InputModel)> {
        self.inputs.iter().filter(|(_, input)| {
            let node = self.nodes.get(&input.parent_node_id).unwrap();
            let data = match node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };
            let rect = Rect {
                x1: data.coords.x + input.local_coords.x,
                x2: data.coords.x + input.local_coords.x + INPUT_SIZE,
                y1: data.coords.y + input.local_coords.y,
                y2: data.coords.y + input.local_coords.y + INPUT_SIZE,
            };
            // dbg!(rect.clone());
            rect.contains(coords.x, coords.y)
        })
    }

    pub fn outputs_at<'a>(
        &'a self,
        coords: &'a Coords,
    ) -> impl Iterator<Item = (&'a OutputId, &'a OutputModel)> {
        self.outputs.iter().filter(|(_, output)| {
            let node = self.nodes.get(&output.parent_node_id).unwrap();
            let data = match node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };
            let rect = Rect {
                x1: data.coords.x + output.local_coords.x,
                x2: data.coords.x + output.local_coords.x + INPUT_SIZE,
                y1: data.coords.y + output.local_coords.y,
                y2: data.coords.y + output.local_coords.y + INPUT_SIZE,
            };
            // dbg!(rect.clone());
            rect.contains(coords.x, coords.y)
        })
    }

    /// TEMPLATE
    /// block
    ///     text_input
    /// Given coordinates, create a starting block node nested with a text_input field node
    ///
    pub fn create_starting_node_block(&mut self, coords: Coords) -> NodeId {
        // block properties
        let block_widget = WidgetNodeData::new_block(coords);

        // child properties

        let child_widget = WidgetNodeData::new_text_input();

        // create widget
        let block_node_id = self.create_node(block_widget, None);

        // create child
        let child_node_id = self.create_node(child_widget, None);

        // connect with child edge
        self.add_child_edge(block_node_id, child_node_id);

        block_node_id
    }

    /// TEMPLATE
    ///
    /// block
    ///     command
    ///
    /// add command widget, remove text_input
    ///
    pub fn into_command_block(
        &mut self,
        block_id: NodeId,
        coords: Coords,
        command_name: &str,
        kind: WidgetKind,
        dimensions: NodeDimensions,
    ) {
        // let dimensions = kind.default_widget_dimensions();
        let command_widget_node_data =
            WidgetNodeData::new_command(command_name, kind, dimensions.width, dimensions.height);

        // update block dimensions
        self.set_node_dimensions(&block_id, command_widget_node_data.clone().dimensions);

        // get edge between block and text input
        // get child of block widget
        let (_, node_edge_model) = self
            .iter_node_edges()
            .find(|(_n, node_edge_model)| {
                node_edge_model.from == block_id
                    && node_edge_model.data.edge_type == EdgeType::Child
            })
            .unwrap();

        // get text input node id
        let text_input_id = node_edge_model.to;

        // create command node
        let command_node_id = self.create_node(command_widget_node_data, Some((block_id, coords)));

        // connect block widget to command widget
        self.add_node_edge(NodeEdgeModel {
            from: block_id,
            to: command_node_id,
            data: EdgeModelData {
                edge_type: EdgeType::Child,
                from_coords: None,
                to_coords: None,
            },
        });

        // Remove text input edge and node
        self.remove_node_and_edges(text_input_id); //FIXME edge properties return null when queried to be removed
    }

    pub fn remove_node_and_edges(&mut self, node_id: NodeId) {
        // from db
        block_on(self.db.0.execute(Action::Mutate(
            self.graph_id().0,
            MutateKind::DeleteNode(node_id.0),
        )))
        .unwrap();

        //from ui
        let removed_node = self.nodes.remove_entry(&node_id).unwrap();

        //check outbound edges
        // let outbound_edges: Vec<(&NodeEdgeId, &NodeEdgeModel)> =
        let edges: Vec<(NodeEdgeId, NodeEdgeModel)> = self
            .node_edges
            .clone()
            .into_iter()
            .filter(|(_edge, edge_model)| edge_model.to == removed_node.0)
            .collect();

        for edge in edges {
            self.node_edges.remove_entry(&edge.0);
        }
    }
}

// Create wallet graph and context node
pub fn create_wallet_and_context(db: Db, graph_id: GraphId) -> ContextConfig {
    // create wallet graph
    let wallet_graph_id = block_on(db.0.execute(Action::CreateGraph(Default::default())))
        .unwrap()
        .as_id()
        .unwrap();

    let solana_context_config = solana::Config {
        solana_url: "https://api.devnet.solana.com".into(),
        wallet_graph: wallet_graph_id,
        solana_arweave_url: "https://arloader.io/".into(),
    };

    // create context node
    let mut props = serde_json::Map::new();

    props.insert(
        CTX_MARKER.into(),
        serde_json::to_value(&solana_context_config).unwrap(),
    );

    let solana_ctx_node_id =
        block_on(db.0.execute(Action::Mutate(graph_id.0, MutateKind::CreateNode(props))))
            .unwrap()
            .as_id()
            .unwrap();

    solana_context_config
}
