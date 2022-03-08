use std::{
    default,
    sync::{Arc, RwLock},
};

use futures::executor::block_on;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value as JsonValue};
use sunshine_core::{
    msg::{Action, CreateEdge, EdgeId as IndraEdgeId, MutateKind},
    store::Datastore,
};
use sunshine_indra::store::generate_uuid_v1;
use uuid::Uuid;

use crate::{
    model::{EdgeId, EdgeModel, Model, NodeId, NodeModel},
    utils::merge_json,
};

// pub struct DataNode {
//     pub node_id: String,
// }

/*
#[derive(Clone, Debug)]
pub struct Edge {
    pub id: String,
    pub from: NodeId,
    pub to: NodeId,
    pub properties: EdgeProperties,
}
*//*
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct EdgeProperties {
    pub edge_type: EdgeType,
    //
}*/
