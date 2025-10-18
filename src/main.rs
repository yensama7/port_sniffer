use std::sync::mpsc::channel;
use tokio::task;
use ip_sniffer::{arguments, scan};

#[tokio::main]
async fn main() {
    let opts = arguments().run(); // collects all the command line values and puts them in the struct

    let (tx, rx) = channel(); //instantiating a channel, where tx -> transmitter, rx -> receiver

    for i in opts.start_port..opts.end_port {
        let tx = tx.clone(); // each thread has its own transmitter

        task::spawn(async move {
            scan(tx, i, opts.ipaddr).await; // calls the scan function
        });
    }

    let mut out = vec![];
    drop(tx); // drops the transaction
    for p in rx {
        out.push(p); // iterates over the receiver and pushes the ports to the vector
    }
    println!();

    out.sort(); // sorts the vector

    for v in out {
        println!("{} is open", v); // iterates over the vector and prints out the open ports
    }
}

