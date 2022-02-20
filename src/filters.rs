use super::{
    handler,
    model::{SignatureRequest, SignerWallet},
};
use warp::Filter;

// for future addition of new routes
pub fn all(
    signer: SignerWallet,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    sign(signer)
}

/// POST /sign with JSON `{address: "0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045", quantity: "2"}`
pub fn sign(
    signer: SignerWallet,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path!("sign")
        .and(warp::post())
        .and(json_body())
        .and(with_signer(signer))
        .and_then(handler::serve_signature)
}

pub fn with_signer(
    signer: SignerWallet,
) -> impl Filter<Extract = (SignerWallet,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || signer.clone())
}

pub fn json_body() -> impl Filter<Extract = (SignatureRequest,), Error = warp::Rejection> + Clone {
    // When accepting a body, we want a JSON body
    // (and to reject huge payloads)...
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
