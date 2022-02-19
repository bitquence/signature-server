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
