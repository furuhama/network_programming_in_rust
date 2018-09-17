use std::env;
use pnet::datalink;
use pnet::datalink::NetworkInterface;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::{EtherTypes, EthernetPacket};
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::Packet;
use ipnetwork::IpNetwork::{V4, V6};

pub fn interfaces() {
    println!("index\tname\tprotocol\tip / prefix");
    for interface in datalink::interfaces().into_iter() {
        for ip in interface.ips.into_iter() {
            match ip {
                V4(network) => {
                    println!("{}\t{}\tipv4\t\t{} / {}",
                             interface.index,
                             interface.name,
                             network.ip(),
                             network.prefix());
                },
                V6(network) => {
                    println!("{}\t{}\tipv6\t\t{} / {}",
                             interface.index,
                             interface.name,
                             network.ip(),
                             network.prefix());
                },
            }
        }
    }
}

pub fn run() {
    let interface_name = env::args().nth(1).unwrap();

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .filter(|iface: &NetworkInterface| iface.name == interface_name)
        .next()
        .expect("Error getting interface");

    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => {
            panic!("An error occurred when creating the datalink channel: {}", e);
        }
    };

    loop {
        match rx.next() {
            Ok(packet) => {
                let packet = EthernetPacket::new(packet).unwrap();
                handle_packet(&packet);
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}

fn handle_packet(ethernet: &EthernetPacket) {
    match ethernet.get_ethertype() {
        EtherTypes::Ipv4 => {
            let header = Ipv4Packet::new(ethernet.payload());

            if let Some(header) = header {
                match header.get_next_level_protocol() {
                    IpNextHeaderProtocols::Tcp => {
                        let tcp = TcpPacket::new(header.payload());

                        if let Some(tcp) = tcp {
                            println!(
                                "Got a TCP packet {}:{} to {}:{}",
                                header.get_source(),
                                tcp.get_source(),
                                header.get_destination(),
                                tcp.get_destination()
                            );
                        }
                    }
                    _ => println!("Ignoring non TCP packet"),
                }
            }
        }
        _ => println!("Ignoring non IPv4 packet"),
    }
}
