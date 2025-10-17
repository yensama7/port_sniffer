use std::env;
use std::net::{IpAddr, TcpStream};
use std::str::FromStr;
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::io::{self, Write};


const MAX: u16 = 65535; // max port to sniff
struct Arguments{
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments{
    fn new(args: &[String]) -> Result<Arguments, &'static str>{
        if args.len() < 2{
            return Err("not enough arguments");
        }else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let f = args[1].clone(); // assigns f to the first argument passed
        if let Ok(ipaddr) = IpAddr::from_str(&f){ // checks if the first argument is the ip address
            return Ok(Arguments { flag: String::from(""),ipaddr, threads: 4}); // uses 4 as the default thread is none is specified
        }else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2{ // checks if the -h is the fist argument and if the arguments are just 2
                println!("Usage: -j to select how many threads you want\
                \r\n       -h or -help to show this help message ");
                return Err("help");
            }else if flag.contains("-h") || flag.contains("--help") { // if -h and other arguments are passed
                return Err("too many arguments");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]){ //gets ip address from argument
                    Ok(s) => s, // returns the ip address
                    Err(_) => return Err("not a valid IPADDR: must be ipv4 or ipv6"),
                };
                let threads = match args[2].parse::<u16>(){ // parses the thread_number to u16
                    Ok(s) => s, // returns the thread_number
                    Err(_) => return Err("failed to parse thread number"),
                };

                return Ok(Arguments{threads, flag, ipaddr});
            }
            else {
                return Err("invalid arguments");
            }
        }
    }
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)){ // connects to the ports
            Ok(_) => { // if open
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap(); // send sto the reciever
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads{
            break;
        }
        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect(); // collects args from the cli
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0); // exits the process when the help is shown
        } else{
            eprintln!("{} problem parsing arguments: {}",program,err); // prints out the errors
            process::exit(0);
        }
    });

    let num_threads = arguments.threads; // gets the threads
    let addr = arguments.ipaddr; // gets the ip address
    let (tx, rx) = channel(); //instantiating a channel, where tx -> transmitter, rx -> reciever

    for i in 0..num_threads{
        let tx = tx.clone(); // each thread has it's own transmitter

        thread::spawn(move || {
            scan(tx, i, addr, num_threads); // calls the scan function
        }); // spawns a thread
    }

    let mut out = vec![];
    drop(tx); // drops the transaction
    for p in rx {
        out.push(p); // iterates over the reciever and pushes the ports to the vector
    }
    println!("");
    out.sort(); // sorts the vector

    for v in out {
        println!("{} is open", v); // iterates over the vector and prints out the open ports
    }
}

