use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::model::{InputId, NodeId, OutputId, PortId};

<<<<<<< HEAD
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub struct Coords {
    pub x: f64,
    pub y: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Unselect,
    SelectNode(NodeId),
    AddOrRemoveNodeToSelection(NodeId),
=======
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Serialize, Deserialize)]
pub struct Coords {
    pub x: i64,
    pub y: i64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Event {
    Unselect,
    SelectNode(NodeId),
    AddNodeToSelection(NodeId),
>>>>>>> master
    CreateNode(Coords),
    EditNode(NodeId),
    RemoveNodes(HashSet<NodeId>),
    //
    MaybeStartSelection(Coords),
    NotASelection,
    StartSelection(Coords, Coords),
    ContinueSelection(Coords, Coords), // start_coords, new_coords
    EndSelection(Coords, Coords),      // start_coords, new_coords
    CancelSelection,
    //
<<<<<<< HEAD
    MaybeStartTransformMove(Coords),
    NotATransformMove,
    StartTransformMove(Coords, Coords), // start_cords, new_coords
    ContinueTransformMove(Coords, Coords), // start_cords, new_coords
    EndTransformMove(Coords, Coords),   // start_cords, new_coords
    CancelTransformMove,
    //
    MaybeStartNodeMove(NodeId, Coords),
    NotANodeMove,
    StartNodeMove(NodeId, Coords, Coords),
=======
    MaybeStartViewportMove(Coords),
    NotAViewportMove,
    StartViewportMove(Coords, Coords),    // start_cords, new_coords
    ContinueViewportMove(Coords, Coords), // last_cords, new_coords
    EndViewportMove(Coords, Coords),      // last_cords, new_coords
    CancelViewportMove,
    //
    MaybeStartNodeMove(NodeId, Coords),
    NotANodeMove,
    StartNodeMove(Coords, Coords),
>>>>>>> master
    ContinueNodeMove(Coords, Coords),
    EndNodeMove(Coords, Coords),
    CancelNodeMove,
    //
    MaybeStartEdge(PortId),
    NotAEdge,
    StartEdge(PortId, Coords),
    ContinueEdge(PortId, Coords),
    EndEdge(InputId, OutputId),
    CancelEdge(PortId),
    //
<<<<<<< HEAD
    // x, y, multiplier
    ScrollZoom(f64, f64, f64),
    ScrollMoveScreen(f64, f64),
    //
    //StartCommandInput(String),
    //ModifyCommandInput(String),
    //ApplyCommandInput(String),
    //CancelCommandInput
=======
    //StartCommandInput(String),
    //ModifyCommandInput(String),
    //ApplyCommandInput(String),
    //CancelCommandInput,
>>>>>>> master
}
