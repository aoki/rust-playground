use pnet::packet::{ip::IpNextHeaderProtocols::Tcp, tcp::TcpFlags};
use std::net::Ipv4Addr;

enum ScagnType {
    Syn = TcpFlags::SYN as isize,
    Fin = TcpFlags::FIN as isize,
    Xmax = (TcpFlags::FIN | TcpFlags::URG | TcpFlags::PSH) as isize,
    Null = 0,
}

struct PacketInfo {
    my_ipaddr: Ipv4Addr,
    target_ipaddr: Ipv4Addr,
    my_port: u16,
    maximum_port: u16,
    scan_type: ScanType,
}

fn main() {
    println!("Hello, world!");
}
