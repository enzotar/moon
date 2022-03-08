use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]

pub enum CommandKind {
    // Solana
    AddPubkey,
    CreateAccount,
    CreateToken,
    DeleteKeypair,
    DeletePubkey,
    GenerateKeypair,
    GetBalance,
    MintToken,
    RequestAirdrop,
    Transfer,
    // NFT
    ApproveUseAuthority,
    CreateMasterAccount,
    CreateMetadataAccount,
    GetLeftUses,
    UpdateMetadaAccount,
    Utilize,
    //
    HttpRequest,
    JsonExtract,
}
