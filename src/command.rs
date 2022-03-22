use std::cmp;
use std::collections::{HashMap, HashSet};

use sunshine_solana::commands::simple::branch;
use sunshine_solana::commands::simple::http_request;
use sunshine_solana::commands::simple::ipfs_nft_upload;
use sunshine_solana::commands::simple::ipfs_upload;
use sunshine_solana::commands::simple::json_extract;
use sunshine_solana::commands::simple::json_insert;
use sunshine_solana::commands::solana;
use sunshine_solana::commands::solana::add_pubkey;
use sunshine_solana::commands::solana::create_account;
use sunshine_solana::commands::solana::create_token;
use sunshine_solana::commands::solana::generate_keypair;
use sunshine_solana::commands::solana::generate_keypair::Arg;
use sunshine_solana::commands::solana::get_balance;
use sunshine_solana::commands::solana::mint_token;
use sunshine_solana::commands::solana::nft::approve_use_authority;
use sunshine_solana::commands::solana::nft::arweave_nft_upload;
use sunshine_solana::commands::solana::nft::arweave_upload;
use sunshine_solana::commands::solana::nft::create_master_edition;
use sunshine_solana::commands::solana::nft::create_metadata_accounts;
use sunshine_solana::commands::solana::nft::get_left_uses;
use sunshine_solana::commands::solana::nft::update_metadata_accounts;
use sunshine_solana::commands::solana::nft::utilize;
use sunshine_solana::commands::solana::nft::{self, arweave_bundlr};
use sunshine_solana::commands::solana::request_airdrop;
use sunshine_solana::commands::solana::transfer;
use sunshine_solana::commands::solana::transfer_solana;

use sunshine_solana::{commands::simple::Command as SimpleCommand, CommandConfig};

use crate::model::NodeDimensions;

/*
pub struct Commands {
    commands: BTreeMap<&'static str, &'static dyn FnMut(Context<'_>)>,
}

pub struct Context<'a> {
    state: &'a mut Model,
}

impl Commands {
    pub fn new() -> Self {
        /*let commands: &[(&'static str, &'static dyn FnMut(Context<'_>))] =
            &[("print", &Self::print), ("const", &Self::r#const)];
        Self {
            commands: commands.iter().map(|&(key, value)| (key, value)).collect(),
        }*/
        todo!()
    }
}*/

pub const INPUT_SIZE: i64 = 50;
pub const HEADER_SIZE: i64 = 30;

pub const COMMANDS: &'static [&'static dyn DynCommand] = &[
    &PrintCommand,
    &ConstCommand,
    &JsonExtractCommand,
    &JsonInsertCommand,
    &HttpRequestCommand,
    &IpfsUploadCommand,
    &IpfsNftUploadCommand,
    &WaitCommand,
    &BranchCommand,
    // Solana
    &CreateTokenCommand,
    &AddPubkeyCommand,
    &CreateAccountCommand,
    &GenerateKeypairCommand,
    &MintTokenCommand,
    &TransferCommand,
    &TransferSolanaCommand,
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
    &ArweaveNftUploadCommand,
    &ArweaveBundlrCommand,
];

// TODO: Build once on initialization
pub fn commands_map() -> HashMap<&'static str, &'static dyn DynCommand> {
    COMMANDS
        .iter()
        .map(|&command| (command.command_name(), command))
        .collect()
}

// calculate node height based on the max ports
fn calculate_node_height(command: impl DynCommand) -> i64 {
    let max_port_count = cmp::max(command.inputs().len(), command.outputs().len()) as i64;
    (max_port_count * INPUT_SIZE + 30) as i64
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandInput {
    pub name: &'static str,
    pub type_bounds: &'static [TypeBound],
    // command_type
    // inputtable
}

impl CommandInput {
    pub const fn new(name: &'static str, type_bounds: &'static [TypeBound]) -> Self {
        Self { name, type_bounds }
    }

    pub fn acceptable_types(&self) -> HashSet<&'static str> {
        let mut type_bounds = self.type_bounds.iter();
        if let Some(type_bound) = type_bounds.next() {
            let mut acceptable_types: HashSet<_> = type_bound.types.iter().copied().collect();
            for type_bound in self.type_bounds {
                let other_acceptable_types: HashSet<_> = type_bound.types.iter().copied().collect();
                acceptable_types = acceptable_types
                    .intersection(&other_acceptable_types)
                    .copied()
                    .collect();
            }
            acceptable_types
        } else {
            HashSet::new()
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommandOutput {
    pub name: &'static str,
    pub r#type: &'static str,
}

impl CommandOutput {
    pub const fn new(name: &'static str, r#type: &'static str) -> Self {
        Self { name, r#type }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct TypeBound {
    pub name: &'static str,
    pub types: &'static [&'static str],
}

pub trait Command {
    const COMMAND_NAME: &'static str;
    const WIDGET_NAME: &'static str;
    const INPUTS: &'static [CommandInput];
    const OUTPUTS: &'static [CommandOutput];
    fn dimensions() -> NodeDimensions; // TODO: Move to CommandView
    fn config() -> CommandConfig;
}

pub trait DynCommand: std::fmt::Debug {
    fn command_name(&self) -> &'static str;
    fn widget_name(&self) -> &'static str;
    fn inputs(&self) -> &'static [CommandInput];
    fn outputs(&self) -> &'static [CommandOutput];
    fn dimensions(&self) -> NodeDimensions; // TODO: Move to DynCommandView
    fn config(&self) -> CommandConfig;
}

impl<T: Command + std::fmt::Debug> DynCommand for T {
    fn command_name(&self) -> &'static str {
        T::COMMAND_NAME
    }

    fn widget_name(&self) -> &'static str {
        T::WIDGET_NAME
    }

    fn inputs(&self) -> &'static [CommandInput] {
        T::INPUTS
    }

    fn outputs(&self) -> &'static [CommandOutput] {
        T::OUTPUTS
    }

    fn dimensions(&self) -> NodeDimensions {
        T::dimensions()
    }

    fn config(&self) -> CommandConfig {
        T::config()
    }
}

// BASIC

#[derive(Copy, Clone, Debug)]
pub struct PrintCommand;

#[derive(Copy, Clone, Debug)]
pub struct ConstCommand;

#[derive(Copy, Clone, Debug)]
pub struct JsonExtractCommand;

#[derive(Copy, Clone, Debug)]
pub struct JsonInsertCommand;

#[derive(Copy, Clone, Debug)]
pub struct HttpRequestCommand;

#[derive(Copy, Clone, Debug)]
pub struct IpfsUploadCommand;

#[derive(Copy, Clone, Debug)]
pub struct IpfsNftUploadCommand;

#[derive(Copy, Clone, Debug)]
pub struct BranchCommand;

#[derive(Copy, Clone, Debug)]
pub struct WaitCommand;

// SOLANA

#[derive(Copy, Clone, Debug)]
pub struct CreateTokenCommand;

#[derive(Copy, Clone, Debug)]
pub struct AddPubkeyCommand;

#[derive(Copy, Clone, Debug)]
pub struct CreateAccountCommand;

#[derive(Copy, Clone, Debug)]
pub struct GenerateKeypairCommand;

#[derive(Copy, Clone, Debug)]
pub struct MintTokenCommand;

#[derive(Copy, Clone, Debug)]
pub struct TransferCommand;

#[derive(Copy, Clone, Debug)]
pub struct TransferSolanaCommand;

#[derive(Copy, Clone, Debug)]
pub struct RequestAirdropCommand;

#[derive(Copy, Clone, Debug)]
pub struct GetBalanceCommand;

// METAPLEX

#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountsCommand;

#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionCommand;

#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountsCommand;

#[derive(Copy, Clone, Debug)]
pub struct UtilizeCommand;

#[derive(Copy, Clone, Debug)]
pub struct ApproveUseAuthorityCommand;

#[derive(Copy, Clone, Debug)]
pub struct GetLeftUsesCommand;

#[derive(Copy, Clone, Debug)]
pub struct ArweaveUploadCommand;

#[derive(Copy, Clone, Debug)]
pub struct ArweaveNftUploadCommand;

#[derive(Copy, Clone, Debug)]
pub struct ArweaveBundlrCommand;

impl Command for PrintCommand {
    const COMMAND_NAME: &'static str = "print";
    const WIDGET_NAME: &'static str = "Print";
    const INPUTS: &'static [CommandInput] = &[CommandInput::new("print", &[PRINTABLE])];
    const OUTPUTS: &'static [CommandOutput] = &[];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: 150,
            width: 450,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::Print)
    }
}

/*pub enum Value {
    Integer(i64),
    Keypair(WrappedKeypair),
    String(String),
    NodeId(Uuid),
    DeletedNode(Uuid),
    Pubkey(Pubkey),
    Success(Signature),
    Balance(u64),
    U8(u8),
    U16(u16),
    U64(u64),
    F32(f32),
    F64(f64),
    Bool(bool),
    StringOpt(Option<String>),
    Empty,
    NodeIdOpt(Option<NodeId>),
    NftCreators(Vec<NftCreator>),
    MetadataAccountData(MetadataAccountData),
} */

impl Command for ConstCommand {
    const COMMAND_NAME: &'static str = "const";
    const WIDGET_NAME: &'static str = "Const";
    const INPUTS: &'static [CommandInput] = &[];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("output", "String")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: 300,
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::Const(sunshine_solana::Value::String(
            "".to_string(),
        )))
    }
}

impl Command for JsonExtractCommand {
    const COMMAND_NAME: &'static str = "json_extract";
    const WIDGET_NAME: &'static str = "JsonExtract";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("path", &[STRING]),
        CommandInput::new("json", &[JSON]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("value", "String")]; //FIXME any value
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: 300,
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::JsonExtract(json_extract::JsonExtract {
            path: None,
            json: None,
        }))
    }
}

impl Command for JsonInsertCommand {
    const COMMAND_NAME: &'static str = "json_insert";
    const WIDGET_NAME: &'static str = "JsonInsert";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("path", &[STRING]),
        CommandInput::new("json", &[JSON]),
        CommandInput::new("value", &[STRING]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("json", "String")]; //FIXME any value
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: 300,
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::JsonInsert(json_insert::JsonInsert {
            path: None,
            json: None,
            value: None,
        }))
    }
}

impl Command for HttpRequestCommand {
    const COMMAND_NAME: &'static str = "http_request";
    const WIDGET_NAME: &'static str = "HttpRequest";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("method", &[STRING]),
        CommandInput::new("url", &[STRING]),
        CommandInput::new("auth_token", &[STRING]),
        CommandInput::new("json_body", &[JSON]),
        CommandInput::new("headers", &[JSON]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("resp_body", "String")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::HttpRequest(http_request::HttpRequest {
            method: None,
            url: None,
            auth_token: None,
            json_body: None,
            headers: None,
        }))
    }
}

impl Command for IpfsUploadCommand {
    const COMMAND_NAME: &'static str = "ipfs_upload";
    const WIDGET_NAME: &'static str = "IpfsUpload";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("pinata_url", &[STRING]),
        CommandInput::new("pinata_jwt", &[STRING]),
        CommandInput::new("file_path", &[STRING]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("image_cid", "String")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::IpfsUpload(ipfs_upload::IpfsUpload {
            pinata_url: None,
            pinata_jwt: None,
            file_path: None,
        }))
    }
}

impl Command for IpfsNftUploadCommand {
    const COMMAND_NAME: &'static str = "ipfs_nft_upload";
    const WIDGET_NAME: &'static str = "IpfsNftUpload";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("pinata_url", &[STRING]),
        CommandInput::new("pinata_jwt", &[STRING]),
        CommandInput::new("metadata", &[NFT_METADATA]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("metadata_cid", "String"),
        CommandOutput::new("metadata", "NftMetadata"),
        CommandOutput::new("metadata_url", "String"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::IpfsNftUpload(
            ipfs_nft_upload::IpfsNftUpload {
                pinata_url: None,
                pinata_jwt: None,
                metadata: None,
            },
        ))
    }
}

impl Command for BranchCommand {
    const COMMAND_NAME: &'static str = "branch";
    const WIDGET_NAME: &'static str = "Branch";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("operator", &[STRING]), //change to operator
        CommandInput::new("a", &[NUMBER]),
        CommandInput::new("b", &[NUMBER]), //anytype
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("__true_branch", "Empty"),
        CommandOutput::new("__false_branch", "Empty"), // TODO
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::Branch(branch::Branch {
            a: None,
            b: None,
            operator: None,
        }))
    }
}

impl Command for WaitCommand {
    const COMMAND_NAME: &'static str = "wait";
    const WIDGET_NAME: &'static str = "Wait";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("wait", &[STRING]),  //any type
        CommandInput::new("value", &[STRING]), // any type
    ];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("value", "String")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Simple(SimpleCommand::Wait)
    }
}

impl Command for CreateTokenCommand {
    const COMMAND_NAME: &'static str = "create_token";
    const WIDGET_NAME: &'static str = "CreateToken";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("fee_payer", &[KEYPAIR]), // , "NodeId"
        CommandInput::new("decimals", &[NUMBER]),
        CommandInput::new("authority", &[KEYPAIR]),
        CommandInput::new("token", &[KEYPAIR]),
        CommandInput::new("memo", &[STRING]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("token", "Keypair"),
        CommandOutput::new("signature", "String"),
        CommandOutput::new("fee_payer", "Keypair"),
        CommandOutput::new("authority", "Keypair"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::CreateToken(create_token::CreateToken {
            fee_payer: None,
            decimals: None,
            authority: None,
            token: None,
            memo: None,
        }))
    }
}

impl Command for AddPubkeyCommand {
    const COMMAND_NAME: &'static str = "add_pubkey";
    const WIDGET_NAME: &'static str = "AddPubkey";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("name", &[STRING]),
        CommandInput::new("pubkey", &[PUBKEY]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("pubkey", "Pubkey")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::AddPubkey(add_pubkey::AddPubkey {
            name: None,
            pubkey: None,
        }))
    }
}

impl Command for CreateAccountCommand {
    const COMMAND_NAME: &'static str = "create_account";
    const WIDGET_NAME: &'static str = "CreateAccount";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("owner", &[KEYPAIR]),
        CommandInput::new("fee_payer", &[KEYPAIR]),
        CommandInput::new("token", &[KEYPAIR]),
        CommandInput::new("account", &[KEYPAIR]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String"),
        CommandOutput::new("token", "Pubkey"),
        CommandOutput::new("owner", "Pubkey"),
        CommandOutput::new("fee_payer", "Keypair"),
        CommandOutput::new("account", "Keypair"), // conditional output
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::CreateAccount(create_account::CreateAccount {
            owner: None,
            fee_payer: None,
            token: None,
            account: None,
        }))
    }
}

impl Command for GenerateKeypairCommand {
    const COMMAND_NAME: &'static str = "generate_keypair";
    const WIDGET_NAME: &'static str = "GenerateKeypair";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("seed_phrase", &[STRING]),
        CommandInput::new("private_key", &[STRING]), //base58 str
        CommandInput::new("passphrase", &[STRING]),
        CommandInput::new("save", &[STRING]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("pubkey", "Pubkey"),
        CommandOutput::new("keypair", "Keypair"),
        CommandOutput::new("node_id", "NodeId"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::GenerateKeypair(
            generate_keypair::GenerateKeypair {
                seed_phrase: Arg::None,
                passphrase: None,
                save: Arg::None,
                private_key: Arg::None,
            },
        ))
    }
}

impl Command for MintTokenCommand {
    const COMMAND_NAME: &'static str = "mint_token";
    const WIDGET_NAME: &'static str = "MintToken";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("token", &[KEYPAIR]),
        CommandInput::new("recipient", &[PUBKEY]),
        CommandInput::new("mint_authority", &[KEYPAIR]),
        CommandInput::new("amount", &[NUMBER]),
        CommandInput::new("fee_payer", &[KEYPAIR]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("token", "Pubkey"),
        CommandOutput::new("signature", "String"),
        CommandOutput::new("fee_payer", "Keypair"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::MintToken(mint_token::MintToken {
            token: None,
            recipient: None,
            mint_authority: None,
            amount: None,
            fee_payer: None,
        }))
    }
}

impl Command for TransferCommand {
    const COMMAND_NAME: &'static str = "transfer";
    const WIDGET_NAME: &'static str = "Transfer";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("fee_payer", &[KEYPAIR]),
        CommandInput::new("token", &[PUBKEY]),
        CommandInput::new("amount", &[NUMBER]),
        CommandInput::new("recipient", &[PUBKEY]),
        CommandInput::new("sender", &[KEYPAIR]),
        CommandInput::new("sender_owner", &[KEYPAIR]),
        CommandInput::new("allow_unfunded", &[BOOL]), //TODO update name to allow_unfunded_recipient
        CommandInput::new("fund_recipient", &[BOOL]),
        CommandInput::new("memo", &[STRING]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("sender_owner", "Pubkey"),
        CommandOutput::new("recipient_account", "Pubkey"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Transfer(transfer::Transfer {
            fee_payer: None,
            token: None,
            amount: None,
            recipient: None,
            sender: None,
            sender_owner: None,
            allow_unfunded: None,
            fund_recipient: None,
            memo: None,
        }))
    }
}

impl Command for TransferSolanaCommand {
    const COMMAND_NAME: &'static str = "transfer_solana";
    const WIDGET_NAME: &'static str = "TransferSolana";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("sender", &[KEYPAIR]),
        CommandInput::new("recipient", &[PUBKEY]),
        CommandInput::new("amount", &[NUMBER]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String"),
        CommandOutput::new("sender", "Keypair"),
        CommandOutput::new("recipient", "Pubkey"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::TransferSolana(
            transfer_solana::TransferSolana {
                sender: None,
                recipient: None,
                amount: None,
            },
        ))
    }
}

impl Command for RequestAirdropCommand {
    const COMMAND_NAME: &'static str = "request_airdrop";
    const WIDGET_NAME: &'static str = "RequestAirdrop";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("pubkey", &[PUBKEY]),
        CommandInput::new("amount", &[NUMBER]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("signature", "String")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::RequestAirdrop(
            request_airdrop::RequestAirdrop {
                pubkey: None,
                amount: None,
            },
        ))
    }
}

impl Command for GetBalanceCommand {
    const COMMAND_NAME: &'static str = "get_balance";
    const WIDGET_NAME: &'static str = "GetBalance";
    const INPUTS: &'static [CommandInput] = &[CommandInput::new("pubkey", &[PUBKEY])];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("balance", "Number")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::GetBalance(get_balance::GetBalance {
            pubkey: None,
        }))
    }
}

// METAPLEX

impl Command for CreateMetadataAccountsCommand {
    const COMMAND_NAME: &'static str = "create_metadata_accounts";
    const WIDGET_NAME: &'static str = "CreateMetadataAccounts";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("token", &[PUBKEY]),
        CommandInput::new("token_authority", &[PUBKEY]),
        CommandInput::new("fee_payer", &[KEYPAIR]),
        CommandInput::new("update_authority", &[KEYPAIR]),
        CommandInput::new("uri", &[STRING]),
        CommandInput::new("metadata", &[NFT_METADATA]),
        CommandInput::new("update_authority_is_signer", &[BOOL]),
        CommandInput::new("is_mutable", &[BOOL]),
        CommandInput::new("uses", &[NTF_USES]), //multi arg input
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String"),
        CommandOutput::new("fee_payer", "Keypair"),
        CommandOutput::new("token", "Pubkey"),
        CommandOutput::new("metadata_pubkey", "Pubkey"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::CreateMetadataAccounts(
            create_metadata_accounts::CreateMetadataAccounts {
                token: None,
                token_authority: None,
                fee_payer: None,
                update_authority: None,
                uri: None,
                metadata: None,
                update_authority_is_signer: None,
                is_mutable: None,
                uses: None,
            },
        )))
    }
}

impl Command for CreateMasterEditionCommand {
    const COMMAND_NAME: &'static str = "create_master_edition";
    const WIDGET_NAME: &'static str = "CreateMasterEdition";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("token", &[PUBKEY]),
        CommandInput::new("token_authority", &[PUBKEY]),
        CommandInput::new("fee_payer", &[KEYPAIR]),
        CommandInput::new("update_authority", &[KEYPAIR]),
        CommandInput::new("max_supply", &[NUMBER]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String"),
        CommandOutput::new("fee_payer", "Keypair"),
        CommandOutput::new("token", "Pubkey"),
        CommandOutput::new("metadata_pubkey", "Pubkey"),
        CommandOutput::new("master_edition_pubkey", "Pubkey"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::CreateMasterEdition(
            create_master_edition::CreateMasterEdition {
                token: None,
                token_authority: None,
                fee_payer: None,
                update_authority: None,
                max_supply: solana::nft::create_master_edition::Arg::Some(None), // TODO double check
            },
        )))
    }
}

impl Command for UpdateMetadataAccountsCommand {
    const COMMAND_NAME: &'static str = "update_metadata_accounts";
    const WIDGET_NAME: &'static str = "UpdateMetadataAccounts";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("token", &[PUBKEY]),
        CommandInput::new("fee_payer", &[KEYPAIR]),
        CommandInput::new("update_authority", &[KEYPAIR]),
        CommandInput::new("new_update_authority", &[KEYPAIR]),
        CommandInput::new("data", &[METADATA_ACCOUNT]), // multi arg
        CommandInput::new("primary_sale_happened", &[BOOL]),
        CommandInput::new("is_mutable", &[BOOL]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String"),
        CommandOutput::new("fee_payer", "Keypair"),
        CommandOutput::new("token", "Pubkey"),
        CommandOutput::new("metadata_pubkey", "Pubkey"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::UpdateMetadataAccounts(
            update_metadata_accounts::UpdateMetadataAccounts {
                token: None,
                fee_payer: None,
                update_authority: None,
                new_update_authority: None,
                data: None,
                primary_sale_happened: None,
                is_mutable: None,
            },
        )))
    }
}

impl Command for UtilizeCommand {
    const COMMAND_NAME: &'static str = "utilize";
    const WIDGET_NAME: &'static str = "Utilize";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("token_account", &[PUBKEY]),
        CommandInput::new("token", &[PUBKEY]),
        CommandInput::new("use_authority_record_pda", &[PUBKEY]),
        CommandInput::new("use_authority", &[KEYPAIR]),
        CommandInput::new("fee_payer", &[KEYPAIR]),
        CommandInput::new("owner", &[PUBKEY]),
        CommandInput::new("burner", &[PUBKEY]),
        CommandInput::new("number_of_uses", &[NUMBER]), //u64
    ];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("signature", "String")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::Utilize(utilize::Utilize {
            token_account: None,
            token: None,
            use_authority_record_pda: None,
            use_authority: None,
            fee_payer: None,
            owner: None,
            burner: None,
            number_of_uses: None,
        })))
    }
}

impl Command for ApproveUseAuthorityCommand {
    const COMMAND_NAME: &'static str = "approve_use_authority";
    const WIDGET_NAME: &'static str = "ApproveUseAuthority";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("user", &[PUBKEY]),
        CommandInput::new("owner", &[KEYPAIR]),
        CommandInput::new("fee_payer", &[KEYPAIR]),
        CommandInput::new("token_account", &[PUBKEY]),
        CommandInput::new("token", &[PUBKEY]),
        CommandInput::new("burner", &[PUBKEY]),
        CommandInput::new("number_of_uses", &[NUMBER]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("signature", "String"),
        CommandOutput::new("use_authority_record_pubkey", "Pubkey"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::ApproveUseAuthority(
            approve_use_authority::ApproveUseAuthority {
                user: None,
                owner: None,
                fee_payer: None,
                token_account: None,
                token: None,
                burner: None,
                number_of_uses: None,
            },
        )))
    }
}

impl Command for GetLeftUsesCommand {
    const COMMAND_NAME: &'static str = "get_left_uses";
    const WIDGET_NAME: &'static str = "GetLeftUses";
    const INPUTS: &'static [CommandInput] = &[CommandInput::new("token", &[PUBKEY])];
    const OUTPUTS: &'static [CommandOutput] = &[CommandOutput::new("left_uses", "Number")];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::GetLeftUses(
            get_left_uses::GetLeftUses { token: None },
        )))
    }
}

impl Command for ArweaveUploadCommand {
    const COMMAND_NAME: &'static str = "arweave_upload";
    const WIDGET_NAME: &'static str = "ArweaveUpload";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("fee_payer", &[KEYPAIR]),
        CommandInput::new("reward_mult", &[NUMBER]), //f32
        CommandInput::new("file_path", &[STRING]),
        CommandInput::new("arweave_key_path", &[STRING]),
        CommandInput::new("pay_with_solana", &[BOOL]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("fee_payer", "Keypair"),
        CommandOutput::new("file_uri", "String"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::ArweaveUpload(
            arweave_upload::ArweaveUpload {
                fee_payer: None,
                reward_mult: None,
                file_path: None,
                arweave_key_path: None,
                pay_with_solana: None,
            },
        )))
    }
}

impl Command for ArweaveNftUploadCommand {
    const COMMAND_NAME: &'static str = "arweave_nft_upload";
    const WIDGET_NAME: &'static str = "ArweaveNftUpload";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("fee_payer", &[KEYPAIR]),
        CommandInput::new("reward_mult", &[NUMBER]), //f32
        CommandInput::new("arweave_key_path", &[STRING]),
        CommandInput::new("metadata", &[NFT_METADATA]),
        CommandInput::new("pay_with_solana", &[BOOL]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("metadata_url", "String"),
        CommandOutput::new("metadata", "String"),
        CommandOutput::new("fee_payer", "Keypair"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::ArweaveNftUpload(
            arweave_nft_upload::ArweaveNftUpload {
                fee_payer: None,
                reward_mult: None,
                arweave_key_path: None,
                metadata: None,
                pay_with_solana: None,
            },
        )))
    }
}

impl Command for ArweaveBundlrCommand {
    const COMMAND_NAME: &'static str = "arweave_bundlr";
    const WIDGET_NAME: &'static str = "ArweaveBundlr";
    const INPUTS: &'static [CommandInput] = &[
        CommandInput::new("fee_payer", &[KEYPAIR]),
        CommandInput::new("metadata", &[NFT_METADATA]),
        CommandInput::new("fund_bundlr", &[BOOL]),
    ];
    const OUTPUTS: &'static [CommandOutput] = &[
        CommandOutput::new("metadata_url", "String"),
        CommandOutput::new("metadata", "String"),
        CommandOutput::new("fee_payer", "Keypair"),
    ];
    fn dimensions() -> NodeDimensions {
        NodeDimensions {
            height: calculate_node_height(Self),
            width: 300,
        }
    }
    fn config() -> CommandConfig {
        CommandConfig::Solana(solana::Kind::Nft(nft::Command::ArweaveBundlr(
            arweave_bundlr::ArweaveBundlr {
                fee_payer: None,
                metadata: None,
                fund_bundlr: None,
            },
        )))
    }
}

const PRINTABLE: TypeBound = TypeBound {
    name: "Printable",
    types: &[
        "String",
        "Number",
        "Bool",
        "Pubkey",
        "Keypair",
        "NftCreators",
        "NftCollection",
        "NftMetadata",
        "NftUses",
        "MetadataAccount",
        "Json",
    ],
};

const PUBKEY: TypeBound = TypeBound {
    name: "Pubkey",
    types: &["Pubkey"], // TODO if accepts pubkey, also accept keypair
};

const KEYPAIR: TypeBound = TypeBound {
    name: "Keypair",
    types: &["Keypair"],
};

const STRING: TypeBound = TypeBound {
    name: "String",
    types: &["String"],
};

const NUMBER: TypeBound = TypeBound {
    name: "Number",
    types: &["Number"],
};

const BOOL: TypeBound = TypeBound {
    name: "Bool",
    types: &["Bool"],
};

const NFT_CREATORS: TypeBound = TypeBound {
    name: "NftCreators",
    types: &["NftCreators"],
};

const NFT_COLLECTION: TypeBound = TypeBound {
    name: "NftCollection",
    types: &["NftCollection"],
};

const NTF_USES: TypeBound = TypeBound {
    name: "NftUses",
    types: &["NftUses"],
};

const NFT_METADATA: TypeBound = TypeBound {
    name: "NftMetadata",
    types: &["NftMetadata"],
};

const METADATA_ACCOUNT: TypeBound = TypeBound {
    name: "MetadataAccount",
    types: &["MetadataAccount"],
};

const JSON: TypeBound = TypeBound {
    name: "Json",
    types: &["Json"],
};

/*
    const INTEGER: TypeBound = TypeBound {
        name: "Printable",
        types: &[
            "u8",
            "u16",
            "u32",
        ],
    };
    ...
*/
