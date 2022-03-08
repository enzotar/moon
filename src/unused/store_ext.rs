// use std::collections::HashMap;
// use std::collections::HashSet;
// use std::sync::Arc;
// use std::sync::RwLock;
// use std::sync::RwLockReadGuard;
// use std::sync::RwLockWriteGuard;

// use anyhow::Error;
// use futures::executor::block_on;
// use serde::Serialize;
// use serde_json::{json, Value as JsonValue};

// use uuid::Uuid;

// use crate::event::Coords;
// use crate::model::Model;
// use crate::model::NodeId;
// use crate::model::NodeModel;
// use crate::model_ext::DataNodeProperties;
// use crate::model_ext::Edge;
// use crate::model_ext::EdgeProperties;
// use crate::model_ext::NodeCoords;
// use crate::model_ext::NodeDimensions;
// use crate::model_ext::NodeType;
// use crate::model_ext::WidgetNodeProperties;
// use crate::model_ext::WidgetType;
// use crate::state::State;
// use crate::utils::merge_json;
// use crate::utils::Rect;
// use crate::view::View;
// use crate::Store;

// // pub struct Node<'a> {
// //     state: &'a mut State,
// //     // view: &'a mut View,
// //     node_model: Arc<NodeModel>,
// //     pub node_id: NodeId,
// // }

// // impl<'a> Node<'a> {
// //     pub fn new(
// //         state: &'a mut State,
// //         /* view: &'a mut View, */
// //         node_model: Arc<NodeModel>,
// //         node_id: NodeId,
// //     ) -> Self {
// //         Self {
// //             state,
// //             // view,
// //             node_model,
// //             node_id: crate::model::NodeId::default(),
// //         }
// //     }

// //     pub fn move_to(self, start_coords: Coords, coords: Coords) -> Result<Node<'a>, Error> {
// //         let mut node_model = self.node_model;

// //         node_model.x = /*node_model.origin_x + */coords.x - start_coords.x;
// //         node_model.y = /*node_model.origin_y +*/ coords.y - start_coords.y;

// //         Ok(self)
// //     }
// //     pub fn update_view() {}
// // }

// impl Store {
//     // /// METHOD WHICH RETURN A NODE
//     // pub fn get_node(&mut self, node_id: NodeId) -> Result<Node<'_>, Error> {
//     //     let node = self.state.unwrap().model().nodes().get(&node_id).unwrap();

//     //     Ok(Node::new(
//     //         &mut self.state.unwrap(),
//     //         // &mut self.view,
//     //         Arc::new(*node),
//     //         node_id,
//     //     ))
//     // }

// }
