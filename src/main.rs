use ethers::signers::LocalWallet;
use sha3::{Digest, Keccak256};
use std::env;
use std::str::FromStr;
use std::sync::Arc;
use warp::Filter;

mod filters;
mod handler;
mod model;

#[tokio::main]
async fn main() {
    if env::var_os("SIGNING_KEY").is_none() {
        // Set `SIGNING_KEY=keccak256("s1gning_key")` if a signing key was not provided
        env::set_var(
            "SIGNING_KEY",
            "d4b9e7ae8585ef740d9fa79ed53eb63d59bff149fdb8c20527ad62e1e0fbba50", // keccak256("s1gning_key")
        );
    }

    if env::var_os("RUST_LOG").is_none() {
        // Set `RUST_LOG=signature_server=debug` to see debug logs,
        // this only shows access logs.
        env::set_var("SIGNING_KEY", "signature_server=debug");
    }

    pretty_env_logger::init();

    let private_key = env::var("SIGNING_KEY").unwrap();
    let signer = LocalWallet::from_str(&private_key).unwrap();

    let api = filters::all(Arc::new(signer));

    // View access logs by setting `RUST_LOG=signature_server`.
    let routes = api.with(warp::log("signature_server"));

    // Start up the server...
    warp::serve(routes).run(([0; 4], 3030)).await;
}

#[cfg(test)]
mod tests {
    use warp::http::StatusCode;
    use warp::test::request;

    use ethers::signers::LocalWallet;
    use std::{str::FromStr, sync::Arc};

    use super::{filters, model::SignatureRequest};

    #[tokio::test]
    async fn test_bad_method() {
        let private_key = hex::encode([3; 32]);
        let signer = LocalWallet::from_str(&private_key).unwrap();
        let api = filters::sign(Arc::new(signer));

        let res = request()
            .method("PUT")
            .path("/sign")
            .json(&make_body())
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
    
    #[tokio::test]
    async fn test_sign_and_recover() {
        let private_key = hex::encode([3; 32]);
        let signer = LocalWallet::from_str(&private_key).unwrap();
        let api = filters::sign(Arc::new(signer));

        let res = request()
            .method("POST")
            .path("/sign")
            .json(&make_body())
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::OK);
    }

    fn make_body() -> SignatureRequest {
        let address = [
            145u8, 178, 111, 255, 255, 179, 37, 225, 63, 30, 245, 146, 176, 147, 54, 150, 9, 128,
            68, 175,
        ];
        SignatureRequest {
            address: address.into(),
            quantity: 2i8.into(),
        }
    }
}
