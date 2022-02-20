use super::model::{Signature, SignatureRequest, SignerWallet};
use ethers::{abi::Tokenizable, signers::Signer, types::Signature as EthersSignature};
use sha3::{Digest, Keccak256};

pub async fn serve_signature(
    request: SignatureRequest,
    signer: SignerWallet,
) -> Result<impl warp::Reply, warp::Rejection> {
    log::debug!("signature requested: {request:?}");

    let hashed = {
        // We ABI encode the contents of the request
        let tokens = [request.address.into_token()];
        let message = ethers::abi::encode(&tokens);

        // ... then hash them ...
        Keccak256::digest(&message[..])
    };

    // ... and then sign a message containing the hashed message
    // (syntactic trick to destructure the Signature struct)
    let EthersSignature { r, s, v } = signer.sign_message(hashed).await.unwrap();

    let signed = Signature { r, s, v };

    log::debug!("signature created: {signed:?}");

    Ok(warp::reply::json(&signed))
}
