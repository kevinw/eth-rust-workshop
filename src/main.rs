extern crate web3;
use web3::futures::Future;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{thread, time};

use web3::types::{H160, U256};


pub struct AddressInfo {
    pub address:H160,
    pub balance:U256,
}

fn main() {
    let ipc_socket = "/Users/kevin/Library/Application Support/io.parity.ethereum/jsonrpc.ipc";
    let (_el, ipc) = web3::transports::Ipc::new(ipc_socket).unwrap();
    let web3 = web3::Web3::new(ipc);

    let mut addrs = vec![];

    for line_result in BufReader::new(File::open("addresses.txt").unwrap()).lines() {
        let line = line_result.unwrap();
        if line.is_empty() {
            continue;
        }

        let address_str = line.clone();
        let addr:H160 = line.parse().unwrap();
        addrs.push(AddressInfo { address: addr, balance: 666.into() });
    }

    loop {
        for ref mut addr_entry in &mut addrs {
            let addr = addr_entry.address;
            let balance = web3.eth().balance(addr, None).wait().unwrap();
            if balance != addr_entry.balance {
                println!("address {:?} has: {:?} ETH", addr, balance);
                addr_entry.balance = balance;
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}
