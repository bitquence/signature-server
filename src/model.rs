use ethers::{
    abi::{Address, Int},
    signers::LocalWallet,
    types::U256,
};
use serde_derive::{Deserialize, Serialize};
use std::sync::Arc;

pub type SignerWallet = Arc<LocalWallet>;

#[derive(Deserialize, Serialize, Debug)]
pub struct SignatureRequest {
    pub address: Address,
    pub quantity: Int,
}

use serde::ser::{Serialize, SerializeStruct};

#[derive(Deserialize, Debug)]
pub struct Signature {
    pub r: U256,
    pub s: U256,
    pub v: u64,
}

impl Serialize for Signature {
    /// Format a Signature instance's fields into hex strings using the `std::fmt::LowerHex` trait like so:
    ///
    /// Signature {
    ///   r: 61902298917516051060705547168949512437165012983460353662508826366911472179362,
    ///   s: 1121937554060273997485664851212476653638899951043563377215104812973964031867,
    ///   v: 28
    /// }
    ///
    /// into:
    ///
    /// Signature {
    ///   r: "0x88db75a6dd8c10003df62d9da14649b538c09e33b1fd93c26243e461619f98a2",
    ///   s: "0x27afe80c31764918e45e9c50196f7caeba8de339db8ad75243af4a46a96777b",
    ///   v: "0x1c"
    /// }
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let Signature { r, s, v } = self;
        let mut state = serializer.serialize_struct("Signature", 3)?;

        // NOTE: remove `#` from the format call to remove the `0x` prefix
        state.serialize_field("r", &format!("{r:#x}")[..])?;
        state.serialize_field("s", &format!("{s:#x}")[..])?;
        state.serialize_field("v", &format!("{v:#x}")[..])?;

        state.end()
    }
}
