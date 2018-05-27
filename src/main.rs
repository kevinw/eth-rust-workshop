extern crate web3;
//extern crate tokio_core;

//use tokio_core::reactor;
use web3::futures::Future;
//use web3::types::BlockNumber;

use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let ipc_socket = "/Users/kevin/Library/Application Support/io.parity.ethereum/jsonrpc.ipc";
    let (_el, ipc) = web3::transports::Ipc::new(ipc_socket).unwrap();

    let web3 = web3::Web3::new(ipc);

    /*
    let block = web3.eth().block_with_txs(
        BlockNumber::Latest.into(),
    ).wait().unwrap();
    */

    for line_result in BufReader::new(File::open("addresses.txt").unwrap()).lines() {
        let line = line_result.unwrap();
        if line.is_empty() {
            continue;
        }

        let address_str = line.clone();
        let balance = web3.eth().balance(line.parse().unwrap(), None).wait().unwrap();
        println!("address {:?} has: {:?} ETH", address_str, balance);
    }
}
