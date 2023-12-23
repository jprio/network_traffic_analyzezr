// Import necessary dependencies
// extern crate pcap;
use pcap::{Capture, Device};
use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::FromPacket;
use pnet::packet::Packet;
use std::thread;
mod info_packet;
fn main() {
    // Choose the network interface for capturing. E.g., "eth0"
    let interface = "eth0";

    let main_device = Device::lookup().unwrap().unwrap();
    let mut cap = Capture::from_device(main_device.clone())
        .unwrap()
        .promisc(true)
        .snaplen(5000)
        .open()
        .unwrap();

    let interfaces = datalink::interfaces();
    let mut handles = vec![];

    for interface in interfaces {
        println!("{}", interface);
        let handle = thread::spawn(move || {
            capture_packets(interface);
        });
        handles.push(handle);
    }
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
}

fn capture_packets(interface: datalink::NetworkInterface) {
    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type: {}", &interface),
        Err(e) => panic!(
            "An error occurred when creating the datalink channel: {}",
            e
        ),
    };

    println!("Start thread reading packet on interface: {}", &interface);
    loop {
        match rx.next() {
            Ok(packet) => {
                if let Some(ethernet_packet) = EthernetPacket::new(packet) {
                    println!("---");
                    info_packet::process_packet_by_type(&interface.name, &ethernet_packet)
                    // info_packet::print_packet_layer_2(&interface.name, &ethernet_packet)
                }
            }
            Err(e) => {
                panic!("An error occurred while reading: {}", e);
            }
        }
    }
}
