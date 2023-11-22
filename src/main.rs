use pnet::datalink::{self, DataLinkReceiver};
use pnet::packet::ethernet::{EtherTypes, Ethernet, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use pnet::util::MacAddr;
use pnet::datalink::Channel::Ethernet as DataLinkEthernet;
use std::io;

fn main() -> Result<(), io::Error> {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == "enp2s0")
        .expect("Network interface not found");

    println!("interface name: {:?}", interface);

    let (_sender, mut receiver) = match datalink::channel(&interface, Default::default()) {
        Ok(DataLinkEthernet(_tx, rx)) => (_tx, rx),
        _ => {
            eprintln!("Failed to open datalink channel");
            return Err(io::Error::new(io::ErrorKind::Other, "Failed to open datalink channel"));
        }
    };

    println!("Listening for incoming packets on interface: {}", interface.name);

    loop {
        match receiver.next() {
            Ok(packet) => {
                // Process and analyze the captured packet here
                println!("Received a packet: {:?}", packet);
            }
            Err(err) => {
                eprintln!("Error capturing packet: {}", err);
            }
        }
    }
    }
