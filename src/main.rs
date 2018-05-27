extern crate web3;
//extern crate tokio_core;

//use tokio_core::reactor;
use web3::futures::Future;
use web3::types::BlockNumber;

fn main() {
    let ipc_socket = "/Users/kevin/Library/Application Support/io.parity.ethereum/jsonrpc.ipc";
    let (_el, ipc) = web3::transports::Ipc::new(ipc_socket).unwrap();

    let web3 = web3::Web3::new(ipc);

    let block = web3.eth().block_with_txs(
        BlockNumber::Latest.into(),
    ).wait().unwrap();

    let address = "0x00a329c0648769A73afAc7F9381E08FB43dBEA72".into();

    let balance = web3.eth().balance(address, None).wait().unwrap();

    println!("address {:?} has: {:?} ETH", address, balance);
    /*
    {
        let address = "0x00a329c0648769A73afAc7F9381E08FB43dBEA72".parse().unwrap();
        let nonce = web3.eth().transaction_count(address, None).wait().unwrap();
    println!("Number of transactions stent from {:?}: {:?}", address, nonce);
    }
    */
}
