use std::thread;
use std::sync::mpsc;
use mpsc::Sender;

use std::net::Ipv4Addr;
use cqc::hdr::CommHdr;
use qkd_rs::Cqc;

use crate::session;

fn server_hdr() -> CommHdr {
    CommHdr {
        remote_app_id: 10,
        remote_port: 8004,
        remote_node: u32::from(Ipv4Addr::new(127, 0, 0, 1))
    }
}

pub struct Key {
    pub value: Vec<u8>
}

unsafe impl Send for Key {}

impl Key {
    pub fn from(length: usize) -> Key {
        Key {
            value: vec![0; length]
        }
    }

    pub fn to_string(&self) -> &[u8] {
        &self.value
    }
}

fn server_thread(tx: Sender<Key>) {
    println!("starting server thread");
    let key_length = session::get_key_length();
    let mut key = Key::from(key_length);

    let cqc = Cqc::new(10, "localhost", 8004);

    for i in 0..key_length {
        let id = cqc.recv_epr(false);
        let outcome = cqc.measure_qubit(id, false);
        print!("{:x?}", outcome as u8);
        key.value[i] = outcome as u8;
    }
    println!("");
    println!("generated key: {:x?}", key.to_string());

    tx.send(key).expect("could not send key through channel");
}

fn client_thread(tx: Sender<Key>) {
    println!("starting client thread");
    let key_length = session::get_key_length();
    let mut key = Key::from(key_length);

    let cqc = Cqc::new(10, "localhost", 8001);

    for i in 0..key_length {
        let id = cqc.create_epr(server_hdr(), false);
        let outcome = cqc.measure_qubit(id, false);
        print!("{:x?}", outcome as u8);
        key.value[i] = outcome as u8;
    }
    println!("");
    println!("generated key: {:x?}", key.to_string());

    tx.send(key).unwrap();
}

pub fn spawn_qkd_generator(tx: Sender<Key>) {
    if session::get_is_server() {
        thread::spawn(move || {
            server_thread(tx);
        });
    } else {
        thread::spawn(move || {
            client_thread(tx);
        });
    }
}