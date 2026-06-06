use std::env;
use std::net::{SocketAddr, TcpStream, ToSocketAddrs}; // https://doc.rust-lang.org/std/net/index.html
use std::process;
use std::time::Duration;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <host> <port>", args[0]);
        process::exit(1);
    }

    let host = &args[1];
    let port: u16 = match args[2].parse() {
        Ok(p) if (1..=65535).contains(&p) => p,
        _ => {
            eprintln!("Invalid port: '{}'. Must be 1-65535.", args[2]);
            process::exit(1);
        }
    };

    // Building the Main Logic
    let target = format!("{host}:{port}");
    let addr: SocketAddr = match target.to_socket_addrs() {
        Ok(mut iter) => match iter.next() {
            Some(a) => a,
            None => {
                eprintln!("Could not resolve {target}");
                process::exit(2);
            }
        },
        Err(e) => {
            eprintln!("Resolution error for {target}: {e}");
            process::exit(2);
        }
    };

    // Now Checking for Port
    let timeout = Duration::from_millis(1000);
    let status = match TcpStream::connect_timeout(&addr, timeout) {
        Ok(_) => "OPEN",
        Err(_) => "CLOSED",
    };

    println!("{host}:{port} {status}");
}
