use super::model::{SignatureRequest, SignerWallet};
use ethers::{abi::Tokenizable, signers::Signer};

pub async fn serve_signature(
    request: SignatureRequest,
    signer: SignerWallet,
) -> Result<impl warp::Reply, std::convert::Infallible> {
    let tokens = [request.address.into_token(), request.quantity.into_token()];
    let message = ethers::abi::encode(&tokens);

    let signed = signer.sign_message(message).await.unwrap();

    Ok(warp::reply::json(&signed))
}
