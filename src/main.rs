use ethers::signers::LocalWallet;
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
    let signer = Arc::new(LocalWallet::from_str(&private_key).unwrap());

    let api = filters::all(signer);

    // View access logs by setting `RUST_LOG=signature_server`.
    let routes = api.with(warp::log("signature_server"));

    // Start up the server...
    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

#[cfg(test)]
mod tests {
    use super::{filters, model::SignatureRequest};
    use ethers::{
        abi::Tokenizable,
        signers::{LocalWallet, Signer},
        types::{Address, Signature as EthersSignature, U256},
    };
    use serde_derive::{Deserialize, Serialize};
    use sha3::{Digest, Keccak256};
    use std::{str::FromStr, sync::Arc};
    use warp::http::StatusCode;
    use warp::test::request;

    #[derive(Deserialize, Serialize, Debug)]
    struct SignatureResponse {
        r: String,
        s: String,
        v: String,
    }

    #[tokio::test]
    async fn test_bad_method() {
        let private_key = "d4b9e7ae8585ef740d9fa79ed53eb63d59bff149fdb8c20527ad62e1e0fbba50";
        let signer = LocalWallet::from_str(private_key).unwrap();
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
        pretty_env_logger::init();

        let private_key = "d4b9e7ae8585ef740d9fa79ed53eb63d59bff149fdb8c20527ad62e1e0fbba50";
        let signer = Arc::new(LocalWallet::from_str(private_key).unwrap());
        let api = filters::sign(signer.clone());

        let request_json = make_body();

        let res = request()
            .method("POST")
            .path("/sign")
            .json(&request_json)
            .reply(&api)
            .await;

        assert_eq!(res.status(), StatusCode::OK);

        let hashed_message = {
            // We ABI encode the contents of the request
            let tokens = [request_json.address.into_token()];
            let message = ethers::abi::encode(&tokens);

            // ... then hash them ...
            Keccak256::digest(&message[..])
        };

        let SignatureResponse { r, s, v } = serde_json::from_slice(res.body()).unwrap();

        let signature = EthersSignature {
            r: U256::from_str_radix(&r[2..], 16).unwrap(),
            s: U256::from_str_radix(&s[2..], 16).unwrap(),
            v: u64::from_str_radix(&v[2..], 16).unwrap(),
        };

        let recovered = signature.recover(&hashed_message[..]).unwrap();

        assert_eq!(recovered, signer.address());
    }

    fn make_body() -> SignatureRequest {
<<<<<<< HEAD
        SignatureRequest {
            address: Address::from_str("0xd8dA6BF26964aF9D7eEd9e03E53415D37aA96045").unwrap(),
            quantity: U256::from(2 as i8),
=======
        let address = hex::decode("0x0000000000000000000000000000000000000001").unwrap();
        SignatureRequest {
            address: Address::from_slice(&address),
            quantity: (2 as i8).into(),
>>>>>>> fa119c5d3fea0cf17bc8a62b0a2bee2ffb3e0b42
        }
    }
}
