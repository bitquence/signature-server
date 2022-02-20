# ECDSA Signature Server

Simple REST API written in Rust using the [warp](https://docs.rs/warp/) crate used for serving ECDSA signatures to prevent automation software from minting NFTs in bulk.

Please keep in mind that this server assumes that you are caching users who have already used a signature in contract storage by their addresses.  

Heavily inspired by [warp's todo example API](https://github.com/seanmonstar/warp/blob/master/examples/todos.rs).

### Todo List
- [x] Types that resemble `ethers`'s Signature type with deserialization to convert fields into hex strings 
- [ ] Error handling using rejections instead of `Result::unwrap`
- [ ] Whitelisted user support using ECDSA or Merkle Trees
- [ ] Nonces / Database for invalidating signatures after on-chain use