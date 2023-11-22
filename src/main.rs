use pnet::datalink::Channel::Ethernet as DataLinkEthernet;
use pnet::datalink::{self, DataLinkReceiver, DataLinkSender};
use pnet::packet::ethernet::{EtherTypes, MutableEthernetPacket};
use pnet::packet::{MutablePacket, Packet};
use pnet::util::MacAddr;
use std::io;

fn main() -> Result<(), io::Error> {

    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == "enp2s0")
        .expect("Network interface not found"); // Replace "eth0" with the name of the desired network interface

    println!("interface name {:?}", interface);
    let (mut sender, mut receiver) = match datalink::channel(&interface, Default::default()) {
        Ok(DataLinkEthernet(tx, rx)) => (tx, rx),
        _ => {
            eprintln!("Failed to open datalink channel");
            return Err(io::Error::new(
                io::ErrorKind::Other,
                "Failed to open datalink channel",
            ));
        }
    };

    let data = "hello world".as_bytes().to_vec();

    // Create an Ethernet frame with your data
    let mut ethernet_frame_buffer = vec![0u8; data.len() + 14]; // Adjust the frame size as needed

    let mut ethernet_frame = MutableEthernetPacket::new(&mut ethernet_frame_buffer)
        .expect("Failed to create Ethernet frame");
    ethernet_frame.set_destination(MacAddr::broadcast());
    ethernet_frame.set_source(interface.mac.expect("No MAC address available"));
    ethernet_frame.set_ethertype(EtherTypes::Ipv6); // Adjust the EtherType as needed

    // Set the payload data manually
    let payload = ethernet_frame.payload_mut();
    payload.copy_from_slice(&data);

    // Send the Ethernet frame using the sender

    sender.send_to(ethernet_frame.packet(), None);
    println!("{:?}", ethernet_frame.packet());
    // }
    match receiver.next() {
        Ok(packet) => {
            print!("received acknowledgment{:?}", packet);
        }
        Err(err) => {
            eprintln!("\nError capturing packet : {}", err);
        }
    }

    Ok(())
}
