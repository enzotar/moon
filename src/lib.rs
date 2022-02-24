// #![feature(map_first_last)]
//  clear && ./sh/clean && cargo build && ./sh/bindgen && flutter run -d linux
#![feature(result_option_inspect)]

//  0% fixed edges not updated for the last node movement frame
//  0% fixed doubleclick do not cancelled by movement while first click

mod api;
mod command;
mod event;
mod flow_context;
mod input;
mod model;
mod state;
mod utils;
mod view;

use std::collections::HashMap;
use std::collections::HashSet;
use std::str::FromStr;

use api::Response;
use futures::executor::block_on;
use model::BasicWidgetKind;
use model::GraphId;
use model::WidgetKind;

use serde::Deserialize;

use sunshine_core::msg::Action;
use sunshine_core::msg::QueryKind;
use sunshine_core::store::Datastore;
use sunshine_solana::commands::solana;
use sunshine_solana::CommandConfig;
use sunshine_solana::{commands::simple::CommandKind as SimpleCommandKind, commands::CommandKind};
use uuid::Uuid;

use event::{Coords, Event};
use rid::RidStore;
use view::EdgeView;
use view::NodeView;
use view::NodeViewType;
use view::ViewEdgeType;
use view::{
    commands_view_map, generate_default_text_commands, Command, LastViewChanges, Selection, View,
};

use crate::command::*;
use crate::model::PortId;
use crate::view::NodeChange;
use crate::view::NodeViewType::{DummyEdgeHandle, WidgetBlock, WidgetTextInput};

use api::*;
use input::FlutterPointerEvent;
use model::{NodeId, NodeModel};
use state::*;

#[rid::store]
#[rid::structs(View, LastViewChanges)]
#[derive(rid::Config)]
pub struct Store {
    #[rid(skip)]
    state: Option<State>,
    view: View,
    last_view_changes: LastViewChanges,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct InputEvent {
    node_id: String,
    text: String,
}

impl RidStore<Msg> for Store {
    fn create() -> Self {
        Self {
            state: None,
            view: View::default(),
            last_view_changes: LastViewChanges::default(),
        }
    }

    fn update(&mut self, req_id: u64, msg: Msg) {
        match msg {
            Msg::Initialize(app_dir) => {
                // TODO add app_dir for release
                let db_path = "SUNSHINE_DB".to_owned();
                assert!(self.state.is_none());
                self.state = Some(State::new(db_path.clone()));
                self.update_graph_list();
                self.refresh_ui();

                rid::post(Confirm::Initialized(req_id, format!("{:?}", db_path,)));
            }
            Msg::MouseEvent(ev) => {
                let state = self.state.as_mut().unwrap();
                let events: Vec<_> = state.on_flutter_mouse_event(&ev).collect();
                self.apply(events, req_id);
                //rid::post(Confirm::ReceivedEvent(req_id, format!("{:?}", ev)));
                rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
            }
            Msg::KeyboardEvent(ev) => {
                let state = self.state.as_mut().unwrap();
                let events: Vec<_> = state.on_flutter_keyboard_event(&ev).collect();
                self.apply(events, req_id);
                //rid::post(Confirm::ReceivedEvent(req_id, format!("{:?}", ev)));
                rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
            }
            Msg::LoadGraph(ev) => {
                let state = self.state.as_mut().unwrap();
                let model = state.model_mut();
                if ev == "new" {
                    model.new_graph();
                } else {
                    let graph_id = GraphId(Uuid::from_str(&ev).unwrap());
                    model.read_graph(graph_id);
                }
                state.reset();
                self.refresh_ui();

                rid::post(Confirm::LoadGraph(req_id, "".to_owned()));
            }
            Msg::Debug(ev) => {
                let model = self.state.as_ref().unwrap().model();
                let graph_id = model.graph_id();

                let graph = block_on(
                    model
                        .db()
                        .execute(Action::Query(QueryKind::ReadGraph(graph_id.0))),
                )
                .unwrap()
                .into_graph()
                .unwrap();

                println!("{:#?}", graph);

                dbg!(self.state.as_ref().unwrap().model());
                rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
            }
            Msg::StartInput(ev) => {
                // when focus on textinput
                let mut state = self.state.as_mut().unwrap();
                state.ui_state = UiState::UiInput;
            }
            Msg::CancelInput(ev) => {
                // when we loose focus and do not want to apply smth (e.g. on Escape)
                let mut state = self.state.as_mut().unwrap();
                match &state.ui_state {
                    UiState::UiInput => state.ui_state = UiState::Default,
                    _ => {}
                }
            }
            Msg::ApplyInput(ev) => {
                // on textinput sumbit/enter
                let mut state = self.state.as_mut().unwrap();
                match state.ui_state {
                    UiState::UiInput => {
                        let event: InputEvent = serde_json::from_str(&ev).unwrap();
                        println!("{:?}", event);
                        state.ui_state = UiState::Default;
                        let node_id = NodeId(Uuid::parse_str(&event.node_id).unwrap());
                        self.handle_text_input(node_id, &event.text, true, req_id);
                    }
                    _ => {}
                }
            }
            // Msg::SetText(ev) => {
            //     // set text but try to apply command

            //     // TODO: check if we are in UiState::UiInput
            //     // TODO: check can be added if StartInput and CancelInput properly called

            //     let event: InputEvent = serde_json::from_str(&ev).unwrap();
            //     println!("{:?}", event);

            //     // NodeId is Option only while we have no way to detect it
            //     // let node_id = match event.node_id.as_str() {
            //     //     "dummy" => state.active_node.unwrap(),
            //     //     node_id => NodeId(Uuid::parse_str(&node_id).unwrap()),
            //     // };

            //     let node_id = NodeId(Uuid::parse_str(&event.node_id).unwrap());

            //     self.handle_text_input(node_id, &event.text, false, req_id);
            //     rid::post(Confirm::ReceivedEvent(req_id, ev.to_owned()));
            // }
            Msg::SetText(ev) => {
                let event: InputEvent = serde_json::from_str(&ev).unwrap();
                println!("{:?}", event);
                let node_id = NodeId(Uuid::parse_str(&event.node_id).unwrap());

                // save to model
                self.state
                    .as_mut()
                    .unwrap()
                    .model_mut()
                    .set_node_text(&node_id, event.text.to_owned());

                // save to db
                self.state
                    .as_mut()
                    .unwrap()
                    .model_mut()
                    .save_text_to_db(&node_id, &event.text);

                rid::post(Confirm::ReceivedEvent(req_id, "".to_owned()));
            }
            Msg::ApplyCommand(command_name) => {
                self.state
                    .as_mut()
                    .unwrap()
                    .apply_command(&command_name)
                    .unwrap();
                self.refresh_ui();

                rid::post(Confirm::ApplyCommand(req_id, command_name.to_owned()));
            }
            Msg::ApplyAutocomplete(ev) => {}
            Msg::SendJson(ev) => {
                let event: InputEvent = serde_json::from_str(&ev).unwrap();
                println!("{:?}", event);

                let model = self.state.as_mut().unwrap().model_mut();

                let node_id = NodeId(Uuid::parse_str(&event.node_id).unwrap());

                model.set_node_text(&node_id, event.text.clone());
                model.update_const_command(node_id, &event.text);

                rid::post(Confirm::ReceivedEvent(req_id, ev.to_owned()));
            }
            Msg::Deploy(ev) => {
                let mut req_id_lock = self.state.as_ref().unwrap().model().req_id.lock().unwrap();
                *req_id_lock = req_id;

                self.state.as_ref().unwrap().model().deploy();

                // rid::post(Confirm::Deployed(req_id, ev.to_owned()));
            }
            Msg::UnDeploy(ev) => {
                self.state.as_ref().unwrap().model().undeploy();
                rid::post(Confirm::UnDeployed(req_id, ev.to_owned()));
            }
            Msg::Request(ev) => {
                let request: Request = serde_json::from_str(&ev).unwrap();
                let response = match request {
                    Request::Initialize(data) => self.on_initialize(data),
                    Request::Mouse(data) => self.on_mouse(data),
                    Request::TextInput(data) => self.on_text_input(data),
                };
                let response = serde_json::to_string(&response).unwrap();
                rid::post(Confirm::Response(req_id, response));
            }
            Msg::Refresh(ev) => {
                println!("{}", ev);
                self.refresh_ui();
                rid::post(Confirm::RefreshUI(req_id));
            }
        };
        // rid::post(Confirm::ReceivedEvent(req_id, String::new()));
    }
}

impl Store {
    pub fn handle_text_input(
        &mut self,
        node_id: NodeId,
        text: &str,
        is_finished: bool,
        req_id: u64,
    ) {
        let mut state = self.state.as_mut().unwrap();

        println!("{:?}", text);
        let data = if is_finished {
            text.split_once("/").and_then(|(before, after)| {
                after
                    .split_once(&[' ', '\n'])
                    .map(|(command, after)| (before, command, after))
                    .or_else(|| Some((before, after, "")))
            })
        } else {
            text.split_once("/").and_then(|(before, after)| {
                after
                    .split_once(&[' ', '\n'])
                    .map(|(command, after)| (before, command, after))
            })
        };

        if let Some((before, command, after)) = data {
            match state.apply_command(command) {
                Ok(()) => {
                    state.ui_state = UiState::Default;
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id));
                    // send StopInput to flutter
                }
                /*CommandAction::Replace(replacement) => {
                    state
                        .model_mut()
                        .set_node_text(&node_id, format!("{}{}{}", before, replacement, after));
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id));
                    // send updated text to flutter
                }*/
                Err(()) => {
                    // todo: check all commands
                    state.model_mut().set_node_text(&node_id, text.to_owned());
                }
            }
        } else {
            state.model_mut().set_node_text(&node_id, text.to_owned());
        }
    }

    fn on_initialize(&mut self, data: InitializeRequest) -> Option<Response> {
        // let db_path = "/home/amir/SUNSHINE_DB".to_owned(); //FIXME
        // assert!(self.state.is_none());
        // self.state = Some(State::new(db_path.clone()));
        // self.update_graph_list();
        // self.refresh_ui();
        // Some(Response::RefreshUi)
        todo!()
    }

    fn on_mouse(&mut self, data: FlutterPointerEvent) -> Option<Response> {
        todo!()
    }

    fn on_text_input(&mut self, data: InputRequest) -> Option<Response> {
        todo!()
    }
}

#[rid::message(Confirm)]
#[derive(Debug, Clone)]
pub enum Msg {
    Initialize(String),
    MouseEvent(String),
    KeyboardEvent(String),
    LoadGraph(String), //StartEdge(String), // { input: { ... } } | { scheduled: { id: 32 | 64 } }
    Debug(String),
    SendJson(String),
    StartInput(String),
    CancelInput(String),
    ApplyInput(String),
    SetText(String), // { node_id, text }
    // SetText2(String),     // { node_id, text }
    ApplyCommand(String), // { node_id, text }
    ApplyAutocomplete(String),
    Deploy(String),
    UnDeploy(String),
    Request(String), //
    Refresh(String),
}

#[rid::reply]
#[derive(Clone)]
pub enum Confirm {
    ReceivedEvent(u64, String), // { updates: [ ... ], scheduled: [[1.0, 64], [2.0, 32]] }
    RefreshUI(u64),
    Initialized(u64, String),
    LoadGraph(u64, String),
    Deployed(u64, String),
    UnDeployed(u64, String),
    ApplyCommand(u64, String),
    Response(u64, String), //
    Refresh(u64),
    CreateNode(u64),
    RemoveNode(u64),
}

impl Store {
    fn apply(&mut self, events: impl IntoIterator<Item = Event>, req_id: u64) {
        for event in events {
            let state = self.state.as_mut().unwrap();
            // println!("{:?}", event);
            match event {
                // BASIC SELECTION
                Event::Unselect => {
                    // FIXME: background doen't send events if node loses focus
                    println!("clear : ");
                    state.clear_selection();
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id))
                }
                Event::SelectNode(node_id) => {
                    println!("clear : ");
                    state.clear_selection();

                    println!("select node {:?}", node_id);
                    state.add_to_selection(node_id);
                    //state.update_active_node(node_id);
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id))
                }
                Event::AddNodeToSelection(node_id) => {
                    println!("select node {:?}", node_id);
                    state.add_to_selection(node_id);
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id))
                }

                // CRUD
                Event::CreateNode(coords) => {
                    let node_id = state.model_mut().create_starting_node_block(coords);
                    println!("selected node {:?}", node_id);
                    state.add_to_selection(node_id);
                    self.refresh_ui();
                    rid::post(Confirm::CreateNode(req_id))
                }
                Event::EditNode(_) => {}
                Event::RemoveNodes(removable_node_ids) => {
                    println!("remove nodes {:?}", removable_node_ids);
                    for node_id in removable_node_ids {
                        state.model_mut().remove_node(node_id);
                    }
                    self.refresh_ui();
                    rid::post(Confirm::RemoveNode(req_id))
                    /*let mut all_removable_nodes = HashSet::new();
                    let mut parents: HashMap<NodeId, NodeId> = HashMap::new();

                    for (edge_id, edge) in state.model().node_edges() {
                        parents.insert(edge.to, edge.from);
                    }

                    for (node_id, node) in state.model().nodes() {
                        let mut parent_id = Some(node_id);
                        while let Some(child_id) = parent_id {
                            if removable_node_ids.contains(&child_id) {
                                all_removable_nodes.insert(node_id);
                                break;
                            }

                            parent_id = parents.get(child_id);
                        }
                    }*/
                }

                // SELECTION RECTANGLE
                Event::MaybeStartSelection(start) => {
                    state.ui_state = UiState::MaybeSelection(start);
                }
                Event::NotASelection => {
                    state.ui_state = UiState::Default;
                }
                Event::StartSelection(start_coords, coords) => {
                    state.ui_state = UiState::Selection(start_coords, coords);
                }
                Event::ContinueSelection(_, _) => {}
                Event::EndSelection(_, _) => {
                    state.ui_state = UiState::Default; // question
                }
                Event::CancelSelection => {
                    println!(":  cancelled");
                    state.ui_state = UiState::Default;
                }

                // SELECTION RECTANGLE
                Event::MaybeStartViewportMove(start) => {
                    state.ui_state = UiState::MaybeViewportMove(start);
                }
                Event::NotAViewportMove => {
                    state.ui_state = UiState::Default;
                }
                Event::StartViewportMove(start_coords, coords) => {
                    let viewport = &mut state.viewport;
                    viewport.x -= (coords.x - start_coords.x) as f64 / viewport.scale;
                    viewport.y -= (coords.x - start_coords.y) as f64 / viewport.scale;
                    state.ui_state = UiState::ViewportMove(start_coords, coords);
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id))
                }
                Event::ContinueViewportMove(old_coords, new_coords) => {
                    let viewport = &mut state.viewport;
                    viewport.x -= (new_coords.x - old_coords.x) as f64 / viewport.scale;
                    viewport.y -= (new_coords.x - old_coords.y) as f64 / viewport.scale;
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id))
                }
                Event::EndViewportMove(old_coords, new_coords) => {
                    let viewport = &mut state.viewport;
                    viewport.x -= (new_coords.x - old_coords.x) as f64 / viewport.scale;
                    viewport.y -= (new_coords.x - old_coords.y) as f64 / viewport.scale;
                    state.ui_state = UiState::Default;
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id))
                }
                Event::CancelViewportMove => {
                    state.ui_state = UiState::Default;
                }
                // MOVE
                Event::MaybeStartNodeMove(node_id, coords) => {
                    state.ui_state = UiState::MaybeNodeMove(coords);
                    println!("clear : ");
                    state.clear_selection();
                    println!("select node {:?}", node_id);
                    state.add_to_selection(node_id.clone());
                }
                Event::NotANodeMove => {
                    state.ui_state = UiState::Default;
                }
                Event::StartNodeMove(start_coords, coords) => {
                    state.ui_state = UiState::NodeMove(start_coords, coords);
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id))
                    // select node
                    // state.clear_selection(); // clears :  on multi select // FIXME: workaround

                    // state.add_to_selection(node_id.clone());
                    /*
                    let node_ids: Vec<NodeId> = state.selected_node_ids().copied().collect();

                    for node_id in node_ids {
                        let node_id_str = node_id.0.to_string();
                        let node_view = self.view.nodes.get_mut(&node_id_str).unwrap();

                        // set origin coordinates
                        let node = state.model().get_node(&node_id).unwrap();

                        let node_coords = match node {
                            NodeModel::Widget(data) => data.coords,
                            NodeModel::Data(_) => panic!(),
                        };
                        node_view.origin_x = node_coords.x;
                        node_view.origin_y = node_coords.y;
                    }*/
                }
                Event::ContinueNodeMove(start_coords, coords) => {
                    state.ui_state = UiState::NodeMove(start_coords, coords);
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id))
                    /*
                    let node_ids: Vec<NodeId> = state.selected_node_ids().copied().collect();

                    for node_id in node_ids {
                        // move only in view, not in model
                        let node_id_str = node_id.0.to_string();
                        let node_view = self.view.nodes.get_mut(&node_id_str).unwrap();

                        node_view.x = node_view.origin_x + coords.x - start_coords.x;
                        node_view.y = node_view.origin_y + coords.y - start_coords.y;
                    }*/
                }
                Event::EndNodeMove(start_coords, coords) => {
                    let node_ids: Vec<NodeId> = state.selected_node_ids().copied().collect();

                    for node_id in node_ids {
                        let node = state.model().get_node(&node_id).unwrap();
                        let data = match node {
                            NodeModel::Widget(data) => data,
                            // NodeModel::Data(_) => panic!(),
                        };
                        let x = data.coords.x + coords.x - start_coords.x;
                        let y = data.coords.y + coords.y - start_coords.y;
                        state.set_node_coords(&node_id, Coords { x, y });
                    }

                    state.ui_state = UiState::Default;
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id));
                }
                Event::CancelNodeMove => {
                    /*let node_ids: Vec<NodeId> = state.selected_node_ids().copied().collect();

                    for node_id in node_ids {
                        // move only in view, not in model
                        let node_id_str = node_id.0.to_string();
                        let node_view = self.view.nodes.get_mut(&node_id_str).unwrap();

                        node_view.x = node_view.origin_x;
                        node_view.y = node_view.origin_y;
                    }*/

                    state.ui_state = UiState::Default;
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id));
                }
                /*Event::Typing(chars) => {
                    match state.ui_state {
                        UiState::Typing(node_id, text) => {
                            state.ui_state = UiState::Typing(node_id, text + &chars)
                        }
                        UiState::Default || ... => panic!(),
                    }
                }*/
                Event::MaybeStartEdge(port_id) => {
                    state.ui_state = UiState::MaybeEdge(port_id);
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id));
                }
                Event::NotAEdge => {
                    state.ui_state = UiState::Default;
                }
                Event::StartEdge(port_id, coords) => {
                    state.ui_state = UiState::Edge(port_id, coords);
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id));
                }
                Event::ContinueEdge(port_id, coords) => {
                    state.ui_state = UiState::Edge(port_id, coords);
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id));
                }
                Event::EndEdge(port_id, output_id) => {
                    println!("Connect Edge {:?} {:?}", port_id, output_id);
                    state
                        .model_mut()
                        .add_or_remove_flow_edge(port_id, output_id);
                    state.ui_state = UiState::Default; // Question? should it be last?
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id));
                }
                Event::CancelEdge(_) => {
                    state.ui_state = UiState::Default;
                    self.refresh_ui();
                    rid::post(Confirm::RefreshUI(req_id));
                } /*Event::StartCommandInput(command) | Event::ModifyCommandInput(command) => {
                      println!("command input: {}", &command);
                      state.ui_state = UiState::CommandInput(command);
                      self.refresh_ui();
                      rid::post(Confirm::RefreshUI(req_id));
                  }*/
                  /*Event::ApplyCommandInput(command) => {
                  }
                  Event::CancelCommandInput => {
                      state.ui_state = UiState::Default;
                      println!("cancel command input");
                      self.refresh_ui();
                      rid::post(Confirm::RefreshUI(req_id));
                  }*/
            }
        }
    }

    fn update_graph_list(&mut self) {
        let state = self.state.as_ref().unwrap();

        self.view.graph_list = state.model().graph_list.clone();
    }

    fn refresh_ui(&mut self) {
        //let old_view = std::mem::take(&mut self.view);

        let state = self.state.as_ref().unwrap();

        // SELECTION
        let selection = if let UiState::Selection(start_coords, coords) = &state.ui_state {
            Selection {
                is_active: true,
                x1: start_coords.x,
                y1: start_coords.y,
                x2: coords.x,
                y2: coords.y,
            }
        } else {
            Selection {
                is_active: false,
                x1: 0,
                y1: 0,
                x2: 0,
                y2: 0,
            }
        };

        // COMMAND INPUT
        let command = if let UiState::CommandInput(command) = &state.ui_state {
            Command {
                is_active: true,
                command: command.to_owned(),
            }
        } else {
            Command {
                is_active: false,
                command: String::new(),
            }
        };

        // GET WIDGET NODES
        let widget_nodes = state
            .model()
            .iter_widget_nodes()
            .map(|(node_id, widget_node_data)| {
                (
                    node_id.0.to_string(),
                    NodeView {
                        index: i64::default(),
                        parent_id: "".to_owned(),
                        origin_x: widget_node_data.coords.x,
                        origin_y: widget_node_data.coords.y,
                        x: widget_node_data.coords.x,
                        y: widget_node_data.coords.y,
                        height: widget_node_data.dimensions.height,
                        width: widget_node_data.dimensions.width,
                        text: widget_node_data.text.to_owned(),
                        outbound_edges: HashMap::new(),
                        widget_type: match &widget_node_data.kind {
                            WidgetKind::Basic(BasicWidgetKind::Block) => WidgetBlock,
                            WidgetKind::Basic(BasicWidgetKind::TextInput) => WidgetTextInput,
                            WidgetKind::Basic(BasicWidgetKind::Dummy) => DummyEdgeHandle,
                            WidgetKind::Command(cfg) => {
                                // command_name should be always exist for command widget
                                if let Some(command_name) = &widget_node_data.command_name {
                                    let command_view_map = commands_view_map();
                                    let command_view =
                                        command_view_map.get(command_name.as_str()).unwrap();
                                    command_view.view_type()
                                } else {
                                    NodeViewType::Print // TODO: FIXME :wrong default
                                }
                            }
                            WidgetKind::Context(_) => todo!(),
                            // WidgetKind::Context(_) => todo!(),
                        },
                        flow_inbound_edges: vec![],
                        flow_outbound_edges: vec![],
                        success: match state.model().run_status.get(node_id) {
                            Some(v) => match v.value() {
                                true => "success",
                                false => "fail",
                            },
                            None => "waiting",
                        }
                        .to_owned(),
                    },
                )
            });

        let input_nodes = state.model().inputs().iter().map(|(input_id, input)| {
            let parent_node_id = input.parent_node_id;
            let input_node = state.model().nodes().get(&parent_node_id).unwrap();
            //dbg!(input.clone());
            let input_data = match input_node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };
            (
                input_id.0.to_string(),
                NodeView {
                    index: input.index,
                    parent_id: input.command_id.0.to_string(),
                    origin_x: input_data.coords.x + input.local_coords.x,
                    origin_y: input_data.coords.y + input.local_coords.y,
                    x: input_data.coords.x + input.local_coords.x,
                    y: input_data.coords.y + input.local_coords.y,
                    height: INPUT_SIZE,
                    width: INPUT_SIZE,
                    text: input.label.to_owned(),
                    outbound_edges: HashMap::new(),
                    widget_type: NodeViewType::WidgetInput,
                    flow_inbound_edges: vec![],
                    flow_outbound_edges: state
                        .model()
                        .flow_edges()
                        .iter()
                        .filter(|(_, edge)| &edge.input_id == input_id)
                        .map(
                            |(edge_id, flow_edge_model)| {
                                edge_id.0.to_string()
                                //flow_edge_model.input_id.0.to_string() // FIXME: should be the edge id, not the input id
                            }, /* edge_id.0.to_string() */
                        )
                        .collect(),
                    success: String::new(),
                },
            )
        });

        let output_node = state.model().outputs().iter().map(|(output_id, output)| {
            let parent_node_id = output.parent_node_id;
            let output_node = state.model().nodes().get(&parent_node_id).unwrap();
            let output_data = match output_node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };
            (
                output_id.0.to_string(),
                NodeView {
                    index: output.index,
                    parent_id: output.command_id.0.to_string(),
                    origin_x: output_data.coords.x + output.local_coords.x,
                    origin_y: output_data.coords.y + output.local_coords.y,
                    x: output_data.coords.x + output.local_coords.x,
                    y: output_data.coords.y + output.local_coords.y,
                    height: INPUT_SIZE,
                    width: INPUT_SIZE,
                    text: output.label.to_owned(),
                    outbound_edges: HashMap::new(),
                    widget_type: NodeViewType::WidgetOutput,
                    flow_outbound_edges: state
                        .model()
                        .flow_edges()
                        .iter()
                        .filter(|(_, edge)| &edge.output_id == output_id)
                        .map(
                            |(edge_id, flow_edge_model)| edge_id.0.to_string(), //flow_edge_model.output_id.0.to_string(), // FIXME: should be the edge id, not the input id
                        )
                        .collect(),
                    flow_inbound_edges: vec![],
                    success: String::new(),
                },
            )
        });

        let mut node_views: HashMap<String, NodeView> =
            widget_nodes.chain(input_nodes).chain(output_node).collect();
        //dbg!(node_views.clone());

        // insert node edges
        for (edge_id, edge_model) in state.model().iter_node_edges() {
            let node_id = edge_model.from.0.to_string();

            let edge = EdgeView {
                from: node_id.clone(),
                to: edge_model.to.0.to_string(),
                edge_type: match edge_model.data.edge_type {
                    model::EdgeType::Child => ViewEdgeType::Child,
                    model::EdgeType::Data => ViewEdgeType::Data,
                    model::EdgeType::Flow => ViewEdgeType::Flow,
                },
                from_coords_x: match edge_model.data.from_coords {
                    Some(coords) => coords.x,
                    None => 0,
                },
                from_coords_y: match edge_model.data.from_coords {
                    Some(coords) => coords.y,
                    None => 0,
                },
                to_coords_x: match edge_model.data.to_coords {
                    Some(coords) => coords.x,
                    None => 0,
                },
                to_coords_y: match edge_model.data.to_coords {
                    Some(coords) => coords.y,
                    None => 0,
                },
            };

            // TODO: always add for widget nodes
            if let Some(node) = node_views.get_mut(&node_id) {
                node.outbound_edges.insert(edge_id.0.to_string(), edge);
            }
        }

        let node_ids: HashSet<NodeId> = state.selected_node_ids().copied().collect();
        // move currently movable nodes
        let (dx, dy) = if let UiState::NodeMove(start_coords, coords) = state.ui_state {
            let dx = coords.x - start_coords.x;
            let dy = coords.y - start_coords.y;

            for node_id in &node_ids {
                let node_id_str = node_id.0.to_string();
                let node_view = node_views.get_mut(&node_id_str).unwrap();

                node_view.x += dx;
                node_view.y += dy;
            }
            (dx, dy)
        } else {
            (0, 0)
        };

        //self.view.flow_edges.clear();
        let mut flow_edges = HashMap::new();
        // insert input/output edges
        for (edge_id, edge) in state.model().flow_edges() {
            let input = state.model().inputs().get(&edge.input_id).unwrap();
            let output = state.model().outputs().get(&edge.output_id).unwrap();

            let input_node = state.model().nodes().get(&input.parent_node_id).unwrap();
            let input_data = match input_node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };
            let output_node = state.model().nodes().get(&output.parent_node_id).unwrap();
            let output_data = match output_node {
                NodeModel::Widget(data) => data,
                // NodeModel::Data(_) => panic!(),
            };

            //let node_view = node_views
            //    .get_mut(&input.parent_node_id.0.to_string())
            //   .unwrap();

            let (dx1, dy1) = if node_ids.contains(&input.parent_node_id) {
                (dx, dy)
            } else {
                (0, 0)
            };

            let (dx2, dy2) = if node_ids.contains(&output.parent_node_id) {
                (dx, dy)
            } else {
                (0, 0)
            };

            // node_view.outbound_edges
            flow_edges.insert(
                edge_id.0.to_string(),
                EdgeView {
                    from: edge.input_id.0.to_string(), // FIXME: input.label.clone(),
                    to: edge.output_id.0.to_string(),  // FIXME: output.label.clone(),
                    edge_type: ViewEdgeType::Flow,
                    from_coords_x: input_data.coords.x + input.local_coords.x + dx1 + 15, //input
                    from_coords_y: input_data.coords.y
                        + input.local_coords.y
                        + dy1
                        + INPUT_SIZE / 2, //half width of input size
                    to_coords_x: output_data.coords.x + output.local_coords.x + dx2 + 35, //output
                    to_coords_y: output_data.coords.y
                        + output.local_coords.y
                        + dy2
                        + INPUT_SIZE / 2,
                },
            );
        }

        let mut highlighted = vec![];

        // SELECTED NODE IDS
        let selected_node_ids = self
            .state
            .as_ref()
            .unwrap()
            .selected_node_ids()
            .map(|uuid| uuid.0.to_string())
            .collect();

        // Add currently creatable edge
        //
        if let UiState::Edge(input_id, coords) = state.ui_state {
            const DUMMY_EDGE_ID: &'static str = "dummy_edge";
            const DUMMY_NODE_ID: &'static str = "dummy_node";

            let commands_map = commands_map();
            // UPDATE when adding commands
            let command_by_command_name =
                |command_name: &str| commands_map.get(command_name).unwrap();

            match input_id {
                PortId::Input(input_id) => {
                    let input = state.model().inputs().get(&input_id).unwrap();
                    let node_id = input.parent_node_id;
                    let input_node = state.model().nodes().get(&node_id).unwrap();
                    // dbg!(input_node);
                    let input_data = match input_node {
                        NodeModel::Widget(data) => data,
                        // NodeModel::Data(_) => panic!(),
                    };

                    let node_id_str = node_id.0.to_string();
                    let node_view = node_views.get_mut(&node_id_str).unwrap();

                    flow_edges.insert(
                        DUMMY_EDGE_ID.to_owned(),
                        EdgeView {
                            from: node_id.0.to_string(),
                            to: DUMMY_NODE_ID.to_owned(),
                            edge_type: ViewEdgeType::Flow,
                            // +15 +25 is adjustment for offset port and edge in flutter dragging
                            from_coords_x: input_data.coords.x + input.local_coords.x + 15,
                            from_coords_y: input_data.coords.y
                                + input.local_coords.y
                                + INPUT_SIZE / 2,
                            to_coords_x: coords.x,
                            to_coords_y: coords.y,
                        },
                    );

                    node_views
                        .get_mut(&node_id.0.to_string())
                        .unwrap()
                        .flow_inbound_edges
                        .push(DUMMY_EDGE_ID.to_owned());

                    node_views.insert(
                        DUMMY_NODE_ID.to_owned(),
                        NodeView {
                            index: 0,
                            parent_id: "".to_owned(),
                            origin_x: coords.x,
                            origin_y: coords.y,
                            x: coords.x,
                            y: coords.y,
                            width: 0,
                            height: 0,
                            text: "".to_owned(),
                            outbound_edges: HashMap::new(),
                            widget_type: NodeViewType::DummyEdgeHandle,
                            flow_inbound_edges: vec![],
                            flow_outbound_edges: vec![DUMMY_EDGE_ID.to_owned()],
                            success: String::new(),
                        },
                    );

                    // get command_node_id
                    let command_node_id = input.command_id;

                    let command_node = state.model().nodes().get(&command_node_id).unwrap();
                    //dbg!(command_node);
                    let input_data = match command_node {
                        NodeModel::Widget(data) => data,
                        // NodeModel::Data(_) => panic!(),
                    };

                    //dbg!(&input_data.kind);
                    let input_command = input_data
                        .command_name
                        .as_ref()
                        .map(|name| command_by_command_name(&name)); // <-- CommandBlock data
                                                                     //dbg!(&input_command);
                    let command_input = input_command.and_then(|input_command| {
                        input_command
                            .inputs()
                            .iter()
                            .find(|command_input| command_input.name == input.label)
                    });
                    //dbg!(&command_input);

                    if let Some(command_input) = command_input {
                        highlighted.extend(
                            state
                                .model()
                                .outputs()
                                .iter()
                                .filter(|(_, output)| {
                                    if output.command_id == input.command_id {
                                        return false;
                                    }
                                    let output_node =
                                        state.model().nodes().get(&output.command_id).unwrap();
                                    let output_data = match output_node {
                                        NodeModel::Widget(data) => data,
                                    };
                                    //dbg!(&output_data.kind);
                                    let output_command = output_data
                                        .command_name
                                        .as_ref()
                                        .map(|name| command_by_command_name(&name));

                                    // dbg!(&output_command);
                                    let command_output =
                                        output_command.and_then(|output_command| {
                                            output_command.outputs().iter().find(|command_output| {
                                                command_output.name == output.label
                                            })
                                        });
                                    //dbg!(&command_output);
                                    if let Some(command_output) = command_output {
                                        /*dbg!(
                                            &command_input.acceptable_types,
                                            command_output.r#type
                                        );*/
                                        command_input
                                            .acceptable_types
                                            .contains(&command_output.r#type)
                                    } else {
                                        false
                                    }
                                })
                                .map(|(output_id, _)| output_id.0.to_string()),
                        );
                    }
                }
                PortId::Output(output_id) => {
                    let output = state.model().outputs().get(&output_id).unwrap();
                    let node_id = output.parent_node_id;
                    let output_node = state.model().nodes().get(&node_id).unwrap();
                    let output_data = match output_node {
                        NodeModel::Widget(data) => data,
                        // NodeModel::Data(_) => panic!(),
                    };

                    node_views
                        .get_mut(&node_id.0.to_string())
                        .unwrap()
                        .flow_outbound_edges
                        .push(DUMMY_EDGE_ID.to_owned());

                    node_views.insert(
                        DUMMY_NODE_ID.to_owned(),
                        NodeView {
                            index: i64::default(),

                            parent_id: "".to_owned(),
                            origin_x: coords.x,
                            origin_y: coords.y,
                            x: coords.x,
                            y: coords.y,
                            width: 100,
                            height: 100,
                            text: "".to_owned(),
                            outbound_edges: HashMap::new(),
                            widget_type: NodeViewType::DummyEdgeHandle,
                            flow_inbound_edges: vec![DUMMY_EDGE_ID.to_owned()],
                            flow_outbound_edges: vec![],
                            success: String::new(),
                        },
                    );

                    flow_edges.insert(
                        DUMMY_EDGE_ID.to_owned(),
                        EdgeView {
                            from: DUMMY_NODE_ID.to_owned(), // FIXME: "".to_owned(),
                            to: node_id.0.to_string(),      // FIXME: output.label.clone(),
                            edge_type: ViewEdgeType::Flow,
                            from_coords_x: coords.x,
                            from_coords_y: coords.y,
                            // +35 +25 is adjustment for offset port and edge in flutter dragging
                            to_coords_x: output_data.coords.x + output.local_coords.x + 35,
                            to_coords_y: output_data.coords.y + output.local_coords.y + 25,
                        },
                    );

                    // get command_node_id
                    let command_node_id = output.command_id;

                    let command_node = state.model().nodes().get(&command_node_id).unwrap();
                    // dbg!(command_node);
                    let output_data = match command_node {
                        NodeModel::Widget(data) => data,
                        // NodeModel::Data(_) => panic!(),
                    };
                    // dbg!(&output_data.kind);

                    let output_command = output_data
                        .command_name
                        .as_ref()
                        .map(|name| command_by_command_name(&name));
                    // dbg!(&output_command);

                    let command_output = output_command.and_then(|output_command| {
                        output_command
                            .outputs()
                            .iter()
                            .find(|command_output| command_output.name == output.label)
                    });
                    // dbg!(&command_output);

                    if let Some(command_output) = command_output {
                        highlighted.extend(
                            state
                                .model()
                                .inputs()
                                .iter()
                                .filter(|(_, input)| {
                                    if output.command_id == input.command_id {
                                        return false;
                                    }
                                    let input_node =
                                        state.model().nodes().get(&input.command_id).unwrap();
                                    let input_data = match input_node {
                                        NodeModel::Widget(data) => data,
                                    };
                                    // dbg!(&input_data.kind);
                                    let input_command = input_data
                                        .command_name
                                        .as_ref()
                                        .map(|name| command_by_command_name(&name));

                                    // dbg!(&input_command);
                                    let command_input = input_command.and_then(|input_command| {
                                        input_command
                                            .inputs()
                                            .iter()
                                            .find(|command_input| command_input.name == input.label)
                                    });
                                    // dbg!(&command_input);
                                    if let Some(command_input) = command_input {
                                        /*dbg!(
                                            &command_input.acceptable_types,
                                            command_output.r#type
                                        );*/
                                        command_input
                                            .acceptable_types
                                            .contains(&command_output.r#type)
                                    } else {
                                        false
                                    }
                                })
                                .map(|(input_id, _)| input_id.0.to_string()),
                        );
                    }
                }
            }
        }

        let viewport = crate::view::Camera {
            x: (state.viewport.x * 4294967296.0) as i64,
            y: (state.viewport.y * 4294967296.0) as i64,
            scale: (state.viewport.scale * 4294967296.0) as i64,
        };

        // self.view.flow_edges = flow_edges.clone();
        // dbg!(self.view.flow_edges.clone());
        let nodes = node_views;

        let text_commands = self.view.text_commands.clone();
        let graph_list = self.view.graph_list.clone();

        let old_view = &self.view;
        let new_view = View {
            nodes,
            flow_edges,
            selected_node_ids,
            selection,
            command,
            text_commands,
            graph_list,
            highlighted,
            viewport,
        };

        let node_ids: HashSet<_> = old_view.nodes.keys().chain(new_view.nodes.keys()).collect();
        let changed_nodes_ids: HashMap<String, NodeChange> = node_ids
            .into_iter()
            .filter_map(|node_id| {
                match (
                    old_view.nodes.get(node_id.as_str()),
                    new_view.nodes.get(node_id.as_str()),
                ) {
                    (Some(old_node), Some(new_node)) => {
                        if old_node == new_node {
                            None
                        } else {
                            Some((
                                node_id.clone(),
                                NodeChange {
                                    kind: view::NodeChangeKind::Modified,
                                },
                            ))
                        }
                    }
                    (None, Some(new_node)) => Some((
                        node_id.clone(),
                        NodeChange {
                            kind: view::NodeChangeKind::Modified,
                        },
                    )),
                    (Some(old_node), None) => Some((
                        node_id.clone(),
                        NodeChange {
                            kind: view::NodeChangeKind::Removed,
                        },
                    )),
                    (None, None) => unreachable!(),
                }
            })
            .collect();

        let flow_edges_ids: HashSet<_> = old_view
            .flow_edges
            .keys()
            .chain(new_view.flow_edges.keys())
            .collect();
        let changed_flow_edges_ids = flow_edges_ids
            .into_iter()
            .filter(|node_id| {
                old_view.flow_edges.get(node_id.as_str())
                    != new_view.flow_edges.get(node_id.as_str())
            })
            .map(Clone::clone)
            .collect();

        let is_nodes_changed = old_view.nodes != new_view.nodes;
        let is_selected_node_ids_changed = old_view.selected_node_ids != new_view.selected_node_ids;
        let is_selection_changed = old_view.selection != new_view.selection;
        let is_command_changed = old_view.command != new_view.command;
        let is_text_commands_changed = old_view.text_commands != new_view.text_commands;
        let is_graph_list_changed = old_view.graph_list != new_view.graph_list;
        let is_highlighted_changed = old_view.highlighted != new_view.highlighted;
        let is_viewport_changed = old_view.viewport != new_view.viewport;

        let changes = LastViewChanges {
            changed_nodes_ids,
            // is_nodes_changed,
            changed_flow_edges_ids,
            is_selected_node_ids_changed,
            is_selection_changed,
            is_command_changed,
            is_text_commands_changed,
            is_graph_list_changed,
            is_highlighted_changed,
            is_viewport_changed,
        };
        self.view = new_view;
        self.last_view_changes = changes;

        //dbg!(&self.view.nodes);
        //dbg!(&self.view.flow_edges);
        //dbg!(&self.view.highlighted);
        //dbg!(&self.last_view_changes);
        dbg!(&state.viewport);
        dbg!(&self.view.viewport);
    }
}

/*
Re
*/
/*
{
    "Const": {
        "String": "hello"
    }
}
*//*
{
    "Const": {
        "String": "beach soldier piano click essay sock stable cover angle wear aunt advice"
    }
}
{
    "Const": {
        "Bool": true
    }
}

{
    "Const": "Empty"
}
{
    "Const": {
        "U64": 50000000
    }
}

{
    "Const": {
        "String": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1c2VySW5mb3JtYXRpb24iOnsiaWQiOiI1NmVjMTNhOS03NTYyLTRiZjMtODgzOS00ZjVlY2YxOTFmMDYiLCJlbWFpbCI6ImVuem90YXIzMDAwQGdtYWlsLmNvbSIsImVtYWlsX3ZlcmlmaWVkIjp0cnVlLCJwaW5fcG9saWN5Ijp7InJlZ2lvbnMiOlt7ImlkIjoiTllDMSIsImRlc2lyZWRSZXBsaWNhdGlvbkNvdW50IjoxfV0sInZlcnNpb24iOjF9LCJtZmFfZW5hYmxlZCI6ZmFsc2V9LCJhdXRoZW50aWNhdGlvblR5cGUiOiJzY29wZWRLZXkiLCJzY29wZWRLZXlLZXkiOiJiYWNkZGI0MTFiZDQ1ZGQzYTUzMSIsInNjb3BlZEtleVNlY3JldCI6IjE5ZGFhMzczY2Q0NTM3YjRjNzg3NDJjOWRkNGI3NTU1MGI4OWNmZDY5YTQ5YjhjNDkxYTc0NDkzM2M4NTY4MGIiLCJpYXQiOjE2NDQ5NTQ0ODl9.9AUY-lYSMpWSS7IQcnkv52J_MYiPDhagWbUT2rv7yTk"
    }
}

{
    "Const": {
        "String": "https://api.pinata.cloud"
    }
}
{
    "Const": {
        "String": "image.jpg"
    }
}
*/
