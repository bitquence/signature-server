use ethers::{
    abi::{Address, Int},
    signers::LocalWallet,
};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

pub type SignerWallet = Arc<LocalWallet>;

#[derive(Deserialize, Serialize, Debug)]
pub struct SignatureRequest {
    pub address: Address,
    pub quantity: Int,
}

#[derive(Serialize, Debug)]
pub struct SignatureResponse {
    
}