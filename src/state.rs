use std::collections::HashSet;
<<<<<<< HEAD

use serde::Serialize;
=======
use std::sync::Arc;

use futures::executor::block_on;
use sunshine_solana::commands::solana::add_pubkey;
use sunshine_solana::commands::solana::create_account;
use sunshine_solana::CommandConfig;
>>>>>>> master

use crate::command::commands_map;
use crate::event::Coords;
use crate::event::Event;
<<<<<<< HEAD
use crate::input::{CapturedLifetime, Context, Input, MappingKind};

use crate::model::NodeModel;
use crate::model::PortId;
use crate::model::WidgetKind;
use crate::model::{Model, NodeId};
=======
use crate::input::{CapturedLifetime, Context, Input};
use crate::model::GraphId;
use crate::model::NodeEdgeModel;
use crate::model::NodeModel;
use crate::model::WidgetKind;
use crate::model::{Model, NodeId};
use crate::model::{NodeEdgeId, PortId};
use crate::utils::Rect;

use sunshine_solana::commands::solana;
use sunshine_solana::commands::solana::create_token;
use sunshine_solana::commands::solana::generate_keypair;
use sunshine_solana::commands::solana::generate_keypair::Arg;
use sunshine_solana::commands::solana::mint_token;
use sunshine_solana::commands::solana::request_airdrop;
use sunshine_solana::commands::solana::transfer;
use sunshine_solana::ContextConfig;
use sunshine_solana::{
    commands::simple::Command as SimpleCommand, commands::simple::CommandKind as SimpleCommandKind,
    commands::CommandKind,
};
>>>>>>> master

#[derive(Debug)]
pub struct State {
    model: Model,
    input: Input,
    pub ui_state: UiState,
<<<<<<< HEAD
    pub selected_node_ids: HashSet<NodeId>,
    pub active_node: Option<NodeId>,
    pub transform: Transform,
    pub transform_screenshot: Transform,
    pub canvas: Canvas,
    pub mapping_kind: MappingKind,
=======
    selected_node_ids: HashSet<NodeId>,
    pub active_node: Option<NodeId>,
    pub viewport: Viewport,
>>>>>>> master
    // pub req_id: u64,
}

#[derive(Clone, Copy, Debug, PartialEq)]
<<<<<<< HEAD
pub struct Transform {
=======
pub struct Viewport {
>>>>>>> master
    pub x: f64,
    pub y: f64,
    pub scale: f64,
}

<<<<<<< HEAD
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Canvas {
    pub width: u64,
    pub height: u64,
}

impl Default for Transform {
=======
impl Default for Viewport {
>>>>>>> master
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            scale: 1.0,
        }
    }
}

impl State {
<<<<<<< HEAD
    pub fn new(db_path: String, log_path: String, canvas_width: u64, canvas_height: u64) -> Self {
        let model = Model::new(db_path, log_path);

        Self {
            model,
            input: Input::default(),
            ui_state: UiState::Default,
            selected_node_ids: HashSet::new(),
            active_node: None,
            transform: Transform::default(), // req_id: u64::default(),
            transform_screenshot: Transform::default(), // req_id: u64::default(),
            canvas: Canvas {
                width: canvas_width,
                height: canvas_height,
            },
            mapping_kind: MappingKind::Mouse,
=======
    pub fn new(db_path: String) -> Self {
        let mut model = Model::new(db_path);

        Self {
            model,
            input: Input::new(),
            ui_state: UiState::Default,
            selected_node_ids: HashSet::new(),
            active_node: None,
            viewport: Viewport::default(), // req_id: u64::default(),
>>>>>>> master
        }
    }

    /// GETTERS
    ///

    pub fn input(&self) -> &Input {
        &self.input
    }

    pub fn input_mut(&mut self) -> &mut Input {
        &mut self.input
    }

    pub fn ui_state(&self) -> &UiState {
        &self.ui_state
    }

    pub fn model(&self) -> &Model {
        &self.model
    }

    pub fn model_mut(&mut self) -> &mut Model {
        &mut self.model
    }

    /// RESET STATE
    ///
    pub fn reset(&mut self) {
        self.clear_selection();
        self.ui_state = UiState::Default;
    }

    /// SELECTION

    pub fn selected_node_ids(&self) -> impl Iterator<Item = &NodeId> {
        self.selected_node_ids.iter()
    }

    pub fn clear_selection(&mut self) {
        self.selected_node_ids.clear();
        self.active_node = None;
<<<<<<< HEAD
        //dbg!(&self.selected_node_ids, self.active_node);
=======
        dbg!(&self.selected_node_ids, self.active_node);
>>>>>>> master
        // self.update_selection();
    }

    pub fn add_to_selection(&mut self, node_id: NodeId) {
        self.selected_node_ids.insert(node_id);
        self.active_node = Some(node_id);
<<<<<<< HEAD
        //dbg!(&self.selected_node_ids, self.active_node);
    }

    pub fn add_or_remove_from_selection(&mut self, node_id: NodeId) {
        if self.selected_node_ids.contains(&node_id) {
            self.selected_node_ids.remove(&node_id);
            if self.active_node == Some(node_id) { //FIXME removes active node and panics
                if self.selected_node_ids.clone().len()>0{

                    self.active_node = self.selected_node_ids.clone().into_iter().next(); //fixme 
                }else{
                    self.active_node =None;
                }

            }
            // dbg!(&self.selected_node_ids, self.active_node);
        } else {
            self.selected_node_ids.insert(node_id);
            self.active_node = Some(node_id);
            // dbg!(&self.selected_node_ids, self.active_node);
        }
=======
        dbg!(&self.selected_node_ids, self.active_node);
>>>>>>> master
    }

    /*pub fn update_active_node(&mut self, node_id: NodeId) {
        self.active_node = Some(node_id);
    }*/

    pub fn on_flutter_mouse_event<'a>(
        &'a mut self,
        msg: &str,
    ) -> impl CapturedLifetime<'a> + Iterator<Item = Event> {
        self.input.on_flutter_mouse_event(
            msg,
            Context {
                model: &self.model,
<<<<<<< HEAD
                transform: self.transform,
                ui_state: &self.ui_state,
                selected_node_ids: &self.selected_node_ids,
                mapping_kind: self.mapping_kind,
=======
                ui_state: &self.ui_state,
                selected_node_ids: &self.selected_node_ids,
>>>>>>> master
            },
        )
    }

    pub fn on_flutter_keyboard_event<'a>(
        &'a mut self,
        msg: &str,
    ) -> impl CapturedLifetime<'a> + Iterator<Item = Event> {
        self.input.on_flutter_keyboard_event(
            msg,
            Context {
                model: &self.model,
<<<<<<< HEAD
                transform: self.transform,
                ui_state: &self.ui_state,
                selected_node_ids: &self.selected_node_ids,
                mapping_kind: self.mapping_kind,
=======
                ui_state: &self.ui_state,
                selected_node_ids: &self.selected_node_ids,
>>>>>>> master
            },
        )
    }

    pub fn set_node_coords(&mut self, node_id: &NodeId, coords: Coords) {
        self.model.set_node_coords(node_id, coords)
    }

<<<<<<< HEAD
    pub fn apply_command(&mut self, node_id: &str, command_name: &str) -> Result<(), ()> {
=======
    pub fn apply_command(&mut self, command_name: &str) -> Result<(), ()> {
>>>>>>> master
        println!("apply command input: {}", &command_name);
        self.ui_state = UiState::Default;

        // get current node from selection
<<<<<<< HEAD
        // dbg!(
        //     self.selected_node_ids().collect::<Vec<_>>(),
        //     self.active_node
        // );
        let node_id = NodeId(node_id.parse().unwrap());
        //let node_id = self.active_node.unwrap();
=======
        dbg!(
            self.selected_node_ids().collect::<Vec<_>>(),
            self.active_node
        );
        let node_id = self.active_node.unwrap();
>>>>>>> master
        let node_model = self.model().nodes().get(&node_id).unwrap();

        // coords
        let coords = match node_model {
            NodeModel::Widget(widget_node_data) => widget_node_data.coords,
            // NodeModel::Data(_) => panic!(),
        };

        let commands_map = commands_map();
        let command = commands_map.get(command_name);
        if let Some(command) = command {
            let config = command.config();
            let widget_name = command.widget_name();
            let dimensions = command.dimensions().clone();

            let command_kind = WidgetKind::Command(config.clone());
            // ideally we should store command_name in CommandConfig (without option)
            self.model_mut().into_command_block(
                node_id,
                coords,
                command_name,
                command_kind,
                dimensions,
            );
            Ok(())
        } else {
            Err(())
        }

        //self.refresh_ui();
        //rid::post(Confirm::RefreshUI(req_id));
    }
}

<<<<<<< HEAD
#[derive(Clone, Debug, Serialize)]
=======
#[derive(Clone, Debug)]
>>>>>>> master
pub enum UiState {
    Default,
    MaybeSelection(Coords),
    Selection(Coords, Coords),
<<<<<<< HEAD
    MaybeTransformMove(Coords),
    TransformMove(Coords),
    MaybeNodeMove(NodeId, Coords),
=======
    MaybeViewportMove(Coords),
    ViewportMove(Coords, Coords),
    MaybeNodeMove(Coords),
>>>>>>> master
    NodeMove(Coords, Coords),
    MaybeEdge(PortId),
    Edge(PortId, Coords),
    UiInput,
    CommandInput(String),
}

impl Default for UiState {
    fn default() -> Self {
        Self::Default
    }
}
