extern crate mqtt3;
use std::env;
use std::net::TcpStream;
use std::io::{Read, Write, BufReader, BufWriter};
use std::process::exit;
use mqtt3::{MqttRead, MqttWrite, Packet, Connect, Publish, Protocol, QoS, PacketIdentifier};
use std::sync::Arc;

fn main() {
    println!("Hello, Rust Publish client!");

    let ref address = "sungura1-angani-ke-host.africastalking.com:1883";
    println!("Connecting to {}", address);
    let mut stream  =  TcpStream::connect(address.as_str()).unwrap();
    let mut reader  =  BufReader::new(stream.try_clone().unwrap());
    let mut writer  =  BufWriter::new(stream.try_clone().unwrap());

    let connect  = Packet::Connect(Box::new(Connect{
        protocol: Protocol::MQTT(4),
        keep_alive: 30,
        client_id: "rust-pub".to_ownned(),
        clean_session: true,
        last_will: None,
        username: "RustPub".to_ownned(),
        password: None
    }));

    println!("Connection Status: {:?}", connect);
    writer.write_packet(&connect);
    writer.flush();
    let packet = reader.read_packet().unwrap();
    println!("Packet stat: {:?}", packet);

    let publish = Packet::Publish(Box::new(Publish{
        dup: false,
        qos: QOS::AtLeastOnce,
        retain: false,
        topic_name: "rust/clients/pub".to_ownned(),
        pid: Some(PacketIdentifier(10)),
        payload: Arc::new("Rust Client".to_string().into_bytes())
    }));

    println!("Publish status: {:?}", publish);
    writer.write_packet(&publish);
    writer.flush();
    let packet = reader.read_packet().unwrap();
    println!("Packet Status: {:?}", packet);
}
