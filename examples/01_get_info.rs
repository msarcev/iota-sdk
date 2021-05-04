// Copyright 2021 IOTA Stiftung
// SPDX-License-Identifier: Apache-2.0

//! cargo run --example 01_get_info --release

use iota_client::Client;

/// In this example we will get information about the node

#[tokio::main]
async fn main() {
    // Create a client instance
    let iota = Client::builder()
        .with_node("https://api.lb-0.testnet.chrysalis2.com") // Insert your node URL here
        // .with_node_auth("https://somechrysalisiotanode.com", "name", "password") // Optional authentication
        .unwrap()
        .finish()
        .await
        .unwrap();

    let info = iota.get_info().await.unwrap();
    println!("Node Info: {:?}", info);
}
