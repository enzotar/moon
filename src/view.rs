use std::collections::{HashMap, HashSet};

use crate::command::*;
use crate::model::GraphEntry;

#[derive(rid::Config)]
#[rid::model]
#[derive(Clone, Debug)]
#[rid::structs(
    NodeView,
    Camera,
    Selection,
    Command,
    WidgetTextCommand,
    EdgeView,
    GraphEntry
)]
pub struct View {
    pub nodes: HashMap<String, NodeView>,
    pub flow_edges: HashMap<String, EdgeView>,
    pub selected_node_ids: Vec<String>,
    pub selection: Selection, // TODO Implement
    pub command: Command,     // not used
    pub text_commands: Vec<WidgetTextCommand>,
    pub graph_list: Vec<GraphEntry>,
    pub highlighted: Vec<String>,
    pub viewport: Camera,
}

#[derive(rid::Config, Clone, Debug, Default)]
#[rid::model]
#[rid::structs(NodeChange)]
pub struct LastViewChanges {
    pub changed_nodes_ids: HashMap<String, NodeChange>, /*NodeChangeKind*/
    // RefreshNode
    // pub is_nodes_changed: bool,
    pub changed_flow_edges_ids: Vec<String>,
    pub is_selected_node_ids_changed: bool,
    pub is_selection_changed: bool,
    pub is_command_changed: bool,
    pub is_text_commands_changed: bool,
    pub is_graph_list_changed: bool,
    pub is_highlighted_changed: bool,
    pub is_viewport_changed: bool,
}

#[derive(rid::Config, Clone, Copy, Debug, Hash, PartialEq)]
#[rid::model]
pub struct Camera {
    pub x: i64,     // multiplied by 4294967296
    pub y: i64,     // multiplied by 4294967296
    pub scale: i64, // multiplied by 4294967296
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            scale: 4294967296,
        }
    }
}

#[rid::model]
#[derive(rid::Config, Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[rid::enums(NodeChangeKind)]
pub struct NodeChange {
    pub kind: NodeChangeKind,
}

#[rid::model]
#[derive(rid::Config, Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum NodeChangeKind {
    Added,
    Removed,
    Modified,
}

pub trait CommandView: crate::command::Command {
    const VIEW_TYPE: NodeViewType;
}

pub trait DynCommandView: DynCommand + std::fmt::Debug {
    fn view_type(&self) -> NodeViewType;
}

impl<T: CommandView + std::fmt::Debug> DynCommandView for T {
    fn view_type(&self) -> NodeViewType {
        T::VIEW_TYPE
    }
}

impl CommandView for PrintCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::Print;
}

impl CommandView for ConstCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::Const;
}

impl CommandView for JsonExtractCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::JsonExtract;
}
impl CommandView for HttpRequestCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::HttpRequest;
}
impl CommandView for IpfsUploadCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::IpfsUpload;
}
impl CommandView for CreateTokenCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::CreateToken;
}
impl CommandView for AddPubkeyCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::AddPubkey;
}
impl CommandView for CreateAccountCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::CreateAccount;
}
impl CommandView for GenerateKeypairCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::GenerateKeypair;
}
impl CommandView for MintTokenCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::MintToken;
}
impl CommandView for TransferCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::Transfer;
}
impl CommandView for RequestAirdropCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::RequestAirdrop;
}
impl CommandView for GetBalanceCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::GetBalance;
}
impl CommandView for CreateMetadataAccountsCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::CreateMetadataAccounts;
}
impl CommandView for CreateMasterEditionCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::CreateMasterEdition;
}
impl CommandView for UpdateMetadataAccountsCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::UpdateMetadataAccounts;
}
impl CommandView for UtilizeCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::Utilize;
}
impl CommandView for ApproveUseAuthorityCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::ApproveUseAuthority;
}
impl CommandView for GetLeftUsesCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::GetLeftUses;
}
impl CommandView for ArweaveUploadCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::ArweaveUpload;
}

// TODO: list all commands
pub const VIEW_COMMANDS: &'static [&'static dyn DynCommandView] = &[
    &PrintCommand,
    &ConstCommand,
    &JsonExtractCommand,
    &HttpRequestCommand,
    &IpfsUploadCommand,
    // Solana
    &CreateTokenCommand,
    &AddPubkeyCommand,
    &CreateAccountCommand,
    &GenerateKeypairCommand,
    &MintTokenCommand,
    &TransferCommand,
    &RequestAirdropCommand,
    &GetBalanceCommand,
    // NFTs
    &CreateMetadataAccountsCommand,
    &CreateMasterEditionCommand,
    &UpdateMetadataAccountsCommand,
    &UtilizeCommand,
    &ApproveUseAuthorityCommand,
    &GetLeftUsesCommand,
    &ArweaveUploadCommand,
];

// TODO: Build once on initialization
pub fn commands_view_map() -> HashMap<&'static str, &'static dyn DynCommandView> {
    VIEW_COMMANDS
        .iter()
        .map(|&command| (command.command_name(), command))
        .collect()
}

#[test]
fn commands_equal_view_commands() {
    assert_eq!(COMMANDS.len(), VIEW_COMMANDS.len());
    assert!(COMMANDS
        .iter()
        .zip(VIEW_COMMANDS.iter())
        .all(|(command, view_command)| { command.command_name() == view_command.command_name() }));
}

pub fn generate_default_text_commands() -> Vec<WidgetTextCommand> {
    COMMANDS
        .iter()
        .map(|command| WidgetTextCommand {
            command_name: command.command_name().to_owned(),
            widget_name: command.widget_name().to_owned(),
            inputs: command
                .inputs()
                .iter()
                .map(|input| TextCommandInput {
                    name: input.name.to_owned(),
                    acceptable_kinds: input
                        .acceptable_types
                        .iter()
                        .map(|&value| value.to_owned())
                        .collect(),
                })
                .collect(),
            outputs: command
                .outputs()
                .iter()
                .map(|output| TextCommandOutput {
                    name: output.name.to_owned(),
                    kind: output.r#type.to_owned(),
                })
                .collect(),
        })
        .collect()
}

impl Default for View {
    fn default() -> Self {
        Self {
            nodes: HashMap::default(),
            flow_edges: HashMap::default(),
            selected_node_ids: Vec::default(),
            selection: Selection::default(),
            command: Default::default(),
            text_commands: generate_default_text_commands(),
            graph_list: Vec::default(),
            highlighted: Vec::default(),
            viewport: Camera::default(),
        }
    }
}

#[derive(rid::Config, Debug, Default, Clone, Eq, PartialEq)]
#[rid::model]
pub struct Selection {
    pub is_active: bool,
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
}

// not used
#[derive(rid::Config, Debug, Default, Clone, Eq, PartialEq)]
#[rid::model]
pub struct Command {
    pub is_active: bool,
    pub command: String,
}

#[derive(rid::Config, Debug, Clone, Eq, PartialEq)]
#[rid::model]
#[rid::structs(TextCommandInput, TextCommandOutput)]
pub struct WidgetTextCommand {
    pub command_name: String,
    pub widget_name: String,
    pub inputs: Vec<TextCommandInput>,
    pub outputs: Vec<TextCommandOutput>,
}

#[derive(rid::Config, Debug, Clone, Eq, PartialEq)]
#[rid::model]
pub struct TextCommandInput {
    pub name: String,
    pub acceptable_kinds: Vec<String>,
}

#[derive(rid::Config, Debug, Clone, Eq, PartialEq)]
#[rid::model]
pub struct TextCommandOutput {
    pub name: String,
    pub kind: String,
}

#[derive(rid::Config, Debug, Clone, Eq, PartialEq)]
#[rid::model]
#[rid::structs(EdgeView)]
#[rid::enums(NodeViewType)]
pub struct NodeView {
    pub index: i64, // only for input output nodes
    pub parent_id: String,
    pub origin_x: i64, // position of node before movements or after moved
    pub origin_y: i64,
    pub x: i64,
    pub y: i64,
    pub height: i64,
    pub width: i64,
    pub text: String,
    pub outbound_edges: HashMap<String, EdgeView>, // not include flow edges
    pub widget_type: NodeViewType,                 // FIXME, pub NodeType
    pub flow_inbound_edges: Vec<String>,
    pub flow_outbound_edges: Vec<String>,
    pub success: String,
}

#[rid::model]
#[derive(rid::Config, Debug, Clone, Eq, PartialEq)]
pub enum NodeViewType {
    Data,
    WidgetBlock,
    WidgetTextInput,
    DummyEdgeHandle,
    WidgetInput,
    WidgetOutput,
    //
    Print,
    Const,
    JsonExtract,
    HttpRequest,
    IpfsUpload,
    //
    CreateToken,
    AddPubkey,
    CreateAccount,
    GenerateKeypair,
    MintToken,
    Transfer,
    RequestAirdrop,
    GetBalance,
    //
    CreateMetadataAccounts,
    CreateMasterEdition,
    UpdateMetadataAccounts,
    Utilize,
    ApproveUseAuthority,
    GetLeftUses,
    ArweaveUpload,
}

#[derive(rid::Config, Clone, Debug, Eq, Hash, PartialEq)]
#[rid::model]
#[rid::enums(ViewEdgeType)]
pub struct EdgeView {
    pub from: String,
    pub to: String,
    pub edge_type: ViewEdgeType,
    pub from_coords_x: i64,
    pub from_coords_y: i64,
    pub to_coords_x: i64,
    pub to_coords_y: i64,
}

#[derive(rid::Config, Clone, Debug, Eq, Hash, PartialEq)]
#[rid::model]
pub enum ViewEdgeType {
    Child,
    Data,
    Flow,
}
