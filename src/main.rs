use std::io;
use std::io::Write;
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use bpaf::Bpaf;
use tokio::net::TcpStream;

const MAX: u16 = 65535;
const IPFALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    #[bpaf(long, short, argument("Address"), fallback(IPFALLBACK))]
    pub address: IpAddr,
    #[bpaf(
        long("start"),
        short('s'),
        guard(start_port_guard, "Must be greater than 0"),
        fallback(1u16)
    )]
    pub start_port: u16,
    #[bpaf(
        long("end"),
        short('e'),
        guard(end_port_guard, "Must be less than or equal to 65535"),
        fallback(MAX)
    )]
    pub end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input < MAX
}

async fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, start_port)).await {
        Ok(_) => {
            print!(".");
            io::stdout().flush().expect("failed io flush");
            tx.send(start_port).expect("Failed tx send ");
        },
        Err(_) => {
            // println!("{:?}", e);
        }
    }
}

#[tokio::main]
async  fn main() {
    let opts = arguments().run();
    let (tx,rx) = channel();
    for i in opts.start_port..opts.end_port{
        let tx = tx.clone();
        tokio::spawn(async move{
            scan(tx,i,opts.address).await;
        });
    }
    drop(tx);
    let mut out  = vec![];
    for p in rx{
        out.push(p);
    }
    println!();
    out.sort();
    for v in out{
        println!("{} is open",v);
    }
}
