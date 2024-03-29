<<<<<<< HEAD
use std::collections::HashMap;

use crate::command::*;
use crate::model::{GraphEntry, SolanaNet};

// #[derive(rid::Config)]
=======
use std::collections::{HashMap, HashSet};

use crate::command::*;
use crate::model::GraphEntry;

#[derive(rid::Config)]
>>>>>>> master
#[rid::model]
#[derive(Clone, Debug)]
#[rid::structs(
    NodeView,
    Camera,
    Selection,
    Command,
    WidgetTextCommand,
    EdgeView,
<<<<<<< HEAD
    GraphEntry,
    BookmarkView,
    DebugData
)]
#[rid::enums(SolanaNet)]
pub struct View {
    pub graph_entry: GraphEntry,
    pub nodes: HashMap<String, NodeView>,
    pub flow_edges: HashMap<String, EdgeView>,
    pub selected_node_ids: Vec<String>,
    pub selected_command_ids: Vec<String>,
=======
    GraphEntry
)]
pub struct View {
    pub nodes: HashMap<String, NodeView>,
    pub flow_edges: HashMap<String, EdgeView>,
    pub selected_node_ids: Vec<String>,
>>>>>>> master
    pub selection: Selection, // TODO Implement
    pub command: Command,     // not used
    pub text_commands: Vec<WidgetTextCommand>,
    pub graph_list: Vec<GraphEntry>,
    pub highlighted: Vec<String>,
<<<<<<< HEAD
    pub transform: Camera,
    pub transform_screenshot: Camera,
    pub bookmarks: HashMap<String, BookmarkView>,
    pub solana_net: SolanaNet,
    pub ui_state_debug: DebugData,
}

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
#[rid::model]
pub struct DebugData {
    pub ui_state: String,
    pub mapping_kind: String,
    pub selected_node_ids: String,
}

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
#[rid::model]
pub struct Ratio {
    pub numer: i64,
    pub denom: u64,
}

#[derive(Clone, Debug, Default)]
=======
    pub viewport: Camera,
}

#[derive(rid::Config, Clone, Debug, Default)]
>>>>>>> master
#[rid::model]
#[rid::structs(NodeChange)]
pub struct LastViewChanges {
    pub changed_nodes_ids: HashMap<String, NodeChange>, /*NodeChangeKind*/
<<<<<<< HEAD
=======
    // RefreshNode
    // pub is_nodes_changed: bool,
>>>>>>> master
    pub changed_flow_edges_ids: Vec<String>,
    pub is_selected_node_ids_changed: bool,
    pub is_selection_changed: bool,
    pub is_command_changed: bool,
    pub is_text_commands_changed: bool,
    pub is_graph_list_changed: bool,
    pub is_highlighted_changed: bool,
<<<<<<< HEAD
    pub is_transform_changed: bool,
    pub is_transform_screenshot_changed: bool,
    pub is_graph_changed: bool,
    pub is_bookmark_changed: bool,
}

impl From<i64> for Ratio {
    fn from(numer: i64) -> Self {
        Self { numer, denom: 1 }
    }
}

impl From<f64> for Ratio {
    fn from(value: f64) -> Self {
        Self {
            numer: (value * 4294967296.0) as i64,
            denom: 4294967296,
        }
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq)]
#[rid::model]
#[rid::structs(Ratio)]
pub struct Camera {
    pub x: Ratio,
    pub y: Ratio,
    pub scale: Ratio,
=======
    pub is_viewport_changed: bool,
}

#[derive(rid::Config, Clone, Copy, Debug, Hash, PartialEq)]
#[rid::model]
pub struct Camera {
    pub x: i64,     // multiplied by 4294967296
    pub y: i64,     // multiplied by 4294967296
    pub scale: i64, // multiplied by 4294967296
>>>>>>> master
}

impl Default for Camera {
    fn default() -> Self {
        Self {
<<<<<<< HEAD
            x: Ratio::from(0),
            y: Ratio::from(0),
            scale: Ratio::from(1),
=======
            x: 0,
            y: 0,
            scale: 4294967296,
>>>>>>> master
        }
    }
}

#[rid::model]
<<<<<<< HEAD
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
=======
#[derive(rid::Config, Clone, Copy, Debug, Eq, Hash, PartialEq)]
>>>>>>> master
#[rid::enums(NodeChangeKind)]
pub struct NodeChange {
    pub kind: NodeChangeKind,
}

#[rid::model]
<<<<<<< HEAD
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
=======
#[derive(rid::Config, Clone, Copy, Debug, Eq, Hash, PartialEq)]
>>>>>>> master
pub enum NodeChangeKind {
    Added,
    Removed,
    Modified,
<<<<<<< HEAD
    AddedOrModified, // temporary
=======
>>>>>>> master
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
<<<<<<< HEAD
impl CommandView for JsonInsertCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::JsonInsert;
}
=======
>>>>>>> master
impl CommandView for HttpRequestCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::HttpRequest;
}
impl CommandView for IpfsUploadCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::IpfsUpload;
}
<<<<<<< HEAD

impl CommandView for IpfsNftUploadCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::IpfsNftUpload;
}
impl CommandView for BranchCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::Branch;
}

impl CommandView for WaitCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::Wait;
}
impl CommandView for CreateMintAccountCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::CreateMintAccount;
}
// impl CommandView for AddPubkeyCommand {
//     const VIEW_TYPE: NodeViewType = NodeViewType::AddPubkey;
// }
impl CommandView for CreateTokenAccountCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::CreateTokenAccount;
=======
impl CommandView for CreateTokenCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::CreateToken;
}
impl CommandView for AddPubkeyCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::AddPubkey;
}
impl CommandView for CreateAccountCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::CreateAccount;
>>>>>>> master
}
impl CommandView for GenerateKeypairCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::GenerateKeypair;
}
impl CommandView for MintTokenCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::MintToken;
}
<<<<<<< HEAD
impl CommandView for TransferTokenCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::TransferToken;
}

impl CommandView for TransferSolanaCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::TransferSolana;
=======
impl CommandView for TransferCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::Transfer;
>>>>>>> master
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
<<<<<<< HEAD

impl CommandView for VerifyCollectionCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::VerifyCollection;
}
impl CommandView for ApproveCollectionAuthorityCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::ApproveCollectionAuthority;
}
impl CommandView for SignMetadataCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::SignMetadata;
}
=======
>>>>>>> master
impl CommandView for UtilizeCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::Utilize;
}
impl CommandView for ApproveUseAuthorityCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::ApproveUseAuthority;
}
impl CommandView for GetLeftUsesCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::GetLeftUses;
}
<<<<<<< HEAD
// impl CommandView for ArweaveUploadCommand {
//     const VIEW_TYPE: NodeViewType = NodeViewType::ArweaveUpload;
// }
// impl CommandView for ArweaveNftUploadCommand {
//     const VIEW_TYPE: NodeViewType = NodeViewType::ArweaveNftUpload;
// }
impl CommandView for ArweaveNftUploadCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::ArweaveNftUpload;
}

impl CommandView for ArweaveFileUploadCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::ArweaveFileUpload;
=======
impl CommandView for ArweaveUploadCommand {
    const VIEW_TYPE: NodeViewType = NodeViewType::ArweaveUpload;
>>>>>>> master
}

// TODO: list all commands
pub const VIEW_COMMANDS: &'static [&'static dyn DynCommandView] = &[
    &PrintCommand,
    &ConstCommand,
    &JsonExtractCommand,
<<<<<<< HEAD
    &JsonInsertCommand,
    &HttpRequestCommand,
    &IpfsUploadCommand,
    &IpfsNftUploadCommand,
    &WaitCommand,
    &BranchCommand,
    // Solana
    &CreateMintAccountCommand,
    // &AddPubkeyCommand,
    &CreateTokenAccountCommand,
    &GenerateKeypairCommand,
    &MintTokenCommand,
    &TransferTokenCommand,
    &TransferSolanaCommand,
=======
    &HttpRequestCommand,
    &IpfsUploadCommand,
    // Solana
    &CreateTokenCommand,
    &AddPubkeyCommand,
    &CreateAccountCommand,
    &GenerateKeypairCommand,
    &MintTokenCommand,
    &TransferCommand,
>>>>>>> master
    &RequestAirdropCommand,
    &GetBalanceCommand,
    // NFTs
    &CreateMetadataAccountsCommand,
    &CreateMasterEditionCommand,
    &UpdateMetadataAccountsCommand,
<<<<<<< HEAD
    &VerifyCollectionCommand,
    &ApproveCollectionAuthorityCommand,
    &SignMetadataCommand,
    &UtilizeCommand,
    &ApproveUseAuthorityCommand,
    &GetLeftUsesCommand,
    // &ArweaveUploadCommand,
    // &ArweaveNftUploadCommand,
    &ArweaveNftUploadCommand,
    &ArweaveFileUploadCommand,
=======
    &UtilizeCommand,
    &ApproveUseAuthorityCommand,
    &GetLeftUsesCommand,
    &ArweaveUploadCommand,
>>>>>>> master
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
<<<<<<< HEAD
                        .acceptable_types()
                        .iter()
                        .map(|&value| value.to_owned())
                        .collect(),
                    required: input.required.to_owned(),
                    tooltip: input.tooltip.to_owned(),
                    has_default: input.has_default.to_owned(),
                    default_value: input.default_value.to_owned(),
=======
                        .acceptable_types
                        .iter()
                        .map(|&value| value.to_owned())
                        .collect(),
>>>>>>> master
                })
                .collect(),
            outputs: command
                .outputs()
                .iter()
                .map(|output| TextCommandOutput {
                    name: output.name.to_owned(),
                    kind: output.r#type.to_owned(),
<<<<<<< HEAD
                    passthrough: output.passthrough.to_owned(),
                    tooltip: output.tooltip.to_owned(),
                })
                .collect(),
            description: command.description().to_owned(),
            availability: command
                .availability()
                .iter()
                .map(|solana_net| match solana_net {
                    SolanaNet::Devnet => "devnet".to_string(),
                    SolanaNet::Testnet => "testnet".to_string(),
                    SolanaNet::Mainnet => "mainnet".to_string(),
=======
>>>>>>> master
                })
                .collect(),
        })
        .collect()
}

impl Default for View {
    fn default() -> Self {
        Self {
<<<<<<< HEAD
            graph_entry: GraphEntry::default(),
            nodes: HashMap::default(),
            flow_edges: HashMap::default(),
            selected_node_ids: Vec::default(),
            selected_command_ids: Vec::default(),
=======
            nodes: HashMap::default(),
            flow_edges: HashMap::default(),
            selected_node_ids: Vec::default(),
>>>>>>> master
            selection: Selection::default(),
            command: Default::default(),
            text_commands: generate_default_text_commands(),
            graph_list: Vec::default(),
            highlighted: Vec::default(),
<<<<<<< HEAD
            transform: Camera::default(),
            transform_screenshot: Camera::default(),
            bookmarks: HashMap::default(),
            solana_net: SolanaNet::Devnet,
            ui_state_debug: DebugData::default(),
=======
            viewport: Camera::default(),
>>>>>>> master
        }
    }
}

<<<<<<< HEAD
#[derive(Debug, Default, Clone, Eq, PartialEq)]
=======
#[derive(rid::Config, Debug, Default, Clone, Eq, PartialEq)]
>>>>>>> master
#[rid::model]
pub struct Selection {
    pub is_active: bool,
    pub x1: i64,
    pub y1: i64,
    pub x2: i64,
    pub y2: i64,
}

// not used
<<<<<<< HEAD
#[derive(Debug, Default, Clone, Eq, PartialEq)]
=======
#[derive(rid::Config, Debug, Default, Clone, Eq, PartialEq)]
>>>>>>> master
#[rid::model]
pub struct Command {
    pub is_active: bool,
    pub command: String,
}

<<<<<<< HEAD
#[derive(Debug, Clone, Eq, PartialEq)]
=======
#[derive(rid::Config, Debug, Clone, Eq, PartialEq)]
>>>>>>> master
#[rid::model]
#[rid::structs(TextCommandInput, TextCommandOutput)]
pub struct WidgetTextCommand {
    pub command_name: String,
    pub widget_name: String,
<<<<<<< HEAD
    pub description: String,
    pub inputs: Vec<TextCommandInput>,
    pub outputs: Vec<TextCommandOutput>,
    pub availability: Vec<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
=======
    pub inputs: Vec<TextCommandInput>,
    pub outputs: Vec<TextCommandOutput>,
}

#[derive(rid::Config, Debug, Clone, Eq, PartialEq)]
>>>>>>> master
#[rid::model]
pub struct TextCommandInput {
    pub name: String,
    pub acceptable_kinds: Vec<String>,
<<<<<<< HEAD
    pub required: bool,
    pub tooltip: String,
    pub has_default: bool,
    pub default_value: String,
=======
>>>>>>> master
}

#[derive(rid::Config, Debug, Clone, Eq, PartialEq)]
#[rid::model]
pub struct TextCommandOutput {
    pub name: String,
    pub kind: String,
<<<<<<< HEAD
    pub passthrough: bool,
    pub tooltip: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
#[rid::model]
#[rid::structs(EdgeView)]
#[rid::enums(NodeViewType, RunStateView)]
=======
}

#[derive(rid::Config, Debug, Clone, Eq, PartialEq)]
#[rid::model]
#[rid::structs(EdgeView)]
#[rid::enums(NodeViewType)]
>>>>>>> master
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
<<<<<<< HEAD
    pub run_state: RunStateView,
    pub elapsed_time: u64,
    pub error: String,
    pub print_output: String,
    pub additional_data: String,
    pub required: bool,
    pub tooltip: String,
    pub type_bounds: String,
    pub passthrough: bool,
    pub default_value: String,
    pub has_default: bool,
}

#[rid::model]
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum RunStateView {
    WaitingInputs,
    Running,
    Failed,
    Success,
    NotRunning,
    Canceled,
}

#[rid::model]
#[derive(Debug, Clone, Eq, PartialEq)]
=======
    pub success: String,
}

#[rid::model]
#[derive(rid::Config, Debug, Clone, Eq, PartialEq)]
>>>>>>> master
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
<<<<<<< HEAD
    JsonInsert,
    HttpRequest,
    IpfsUpload,
    IpfsNftUpload,
    Wait,
    Branch,
    //
    CreateMintAccount,
    // AddPubkey,
    CreateTokenAccount,
    GenerateKeypair,
    MintToken,
    TransferToken,
    TransferSolana,
=======
    HttpRequest,
    IpfsUpload,
    //
    CreateToken,
    AddPubkey,
    CreateAccount,
    GenerateKeypair,
    MintToken,
    Transfer,
>>>>>>> master
    RequestAirdrop,
    GetBalance,
    //
    CreateMetadataAccounts,
    CreateMasterEdition,
    UpdateMetadataAccounts,
<<<<<<< HEAD
    VerifyCollection,
    ApproveCollectionAuthority,
    SignMetadata,
    Utilize,
    ApproveUseAuthority,
    GetLeftUses,
    // ArweaveUpload,
    // ArweaveNftUpload,
    ArweaveNftUpload,
    ArweaveFileUpload,
}

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
=======
    Utilize,
    ApproveUseAuthority,
    GetLeftUses,
    ArweaveUpload,
}

#[derive(rid::Config, Clone, Debug, Eq, Hash, PartialEq)]
>>>>>>> master
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

<<<<<<< HEAD
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
=======
#[derive(rid::Config, Clone, Debug, Eq, Hash, PartialEq)]
>>>>>>> master
#[rid::model]
pub enum ViewEdgeType {
    Child,
    Data,
    Flow,
}
<<<<<<< HEAD

#[derive(Debug, Clone, Eq, PartialEq)]
#[rid::model]
pub struct BookmarkView {
    pub name: String,
    pub nodes: Vec<String>,
}
=======
>>>>>>> master
