use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{Sender, channel};
use std::io::{self, Write};
use tokio::net::TcpStream;
use tokio::task;
use bpaf::Bpaf;

const MAX: u16 = 65535; // max port to sniff

const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)); //falls back to loopback address inca se address specified fails

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
 pub struct Arguments{
    #[bpaf(long, short, fallback(IPFALLBACK))]
    /// address you want to sniff, must be a valid IPV4 address, falls back to 127.0.0.1
    pub ipaddr: IpAddr,

    #[bpaf(long("start"), short('s'), fallback(1u16), guard(start_port_guard, "Must be greater than 0."))]
    /// start port for the port sniffer, must be greater than 0
    pub start_port: u16,

    #[bpaf(long("end"), short('e'), guard(end_port_guard, "Must be less than or equals to 65535."), fallback(MAX))]
    /// end port for the port sniffer, must be less than or equals to 65535
    pub end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= MAX
}

async fn scan(tx: Sender<u16>, port: u16, addr: IpAddr) {
        match TcpStream::connect(format!("{}:{}",addr, port)).await{ // connects to the ports on that address
            Ok(_) => { // if open
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap(); // send to the receiver
            }
            Err(_) => {}
        }

}

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

