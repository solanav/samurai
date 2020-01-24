use std::net::TcpStream;
use crate::types::id::{Id, ID_BYTES};
use crate::network::packet::{Packet, *};
use std::io::Write;

fn send_packet(stream: &mut TcpStream, packet: Packet) {
    if let Err(e) = stream.write(&packet.as_bytes()) {
        println!("Failed to send bytes [{}]", e);
        return;
    }

    if let Err(e) = stream.flush() {
        println!("Failed to flush stream [{}]", e);
        return;
    }
}

pub fn ping(stream: &mut TcpStream) {
    let packet = Packet::new_with_cookie(PING_HEADER, &[0; DATA_SIZE]);
    send_packet(stream, packet);
}

pub fn pong(stream: &mut TcpStream, cookie: u32) {
    let packet = Packet::new(PONG_HEADER, cookie, &[0; DATA_SIZE]);
    send_packet(stream, packet);
}

pub fn find_node(stream: &mut TcpStream, id: &Id) {
    let mut buf = [0u8; DATA_SIZE];
    let id_bytes = id.as_bytes();

    for i in 0..id_bytes.len() {
        buf[i] = id_bytes[i];
    }

    let packet = Packet::new_with_cookie(FINDNODE_HEADER, &buf);
    send_packet(stream, packet);
}

pub fn send_node(stream: &mut TcpStream, cookie: u32, id_list: &Vec<Id>) {
    let mut buf = [0u8; DATA_SIZE];

    let mut j = 0;
    for i in 0..id_list.iter().len() {
        // Careful not to add too many ID
        if i >= DATA_SIZE/ID_BYTES {
            break;
        }

        // Copy ID to the buffer
        for b in id_list[i].as_bytes().iter() {
            buf[j] = *b;
            j += 1;
        }
    }

    let packet = Packet::new(SENDNODE_HEADER, cookie, &buf);
    send_packet(stream, packet);
}

pub fn send_message(stream: &mut TcpStream, msg: &String) {
    let mut buf = [0u8; DATA_SIZE];

    let mut i = 0;
    for b in msg.as_bytes().iter() {
        if i >= 502 {
            break;
        }

        buf[i] = *b;
        i += 1;
    }

    let packet = Packet::new_with_cookie(SENDNODE_HEADER, &buf);
    send_packet(stream, packet);
}

pub fn send_echo(stream: &mut TcpStream, cookie: u32, buf: &[u8; DATA_SIZE]) {
    let packet = Packet::new(SENDNODE_HEADER, cookie, buf);
    send_packet(stream, packet);
}
