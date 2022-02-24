use serde::{Deserialize, Serialize};
use std::collections::HashSet;

use crate::model::{InputId, NodeId, OutputId, PortId};

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
    //StartCommandInput(String),
    //ModifyCommandInput(String),
    //ApplyCommandInput(String),
    //CancelCommandInput,
}
