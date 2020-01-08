#![allow(dead_code)]

use std::sync::mpsc;
use mpsc::Receiver;

use crate::quantum::Key;

static mut QKD_SESSION: QkdSession = QkdSession { is_server: false, key_length: 0, key_handle: [5; 64], rx: None };

pub struct QkdSession {
    is_server: bool,
    key_length: usize,
    key_handle: [u8; 64],
    rx: Option<Receiver<Key>>
}

fn get_static_qkd_session() -> &'static mut QkdSession {
    unsafe { &mut QKD_SESSION }
}

pub fn set_is_server(b: bool) {
    get_static_qkd_session().set_is_server(b);
}

pub fn get_is_server() -> bool {
    get_static_qkd_session().get_is_server()
}

pub fn set_key_length(len: usize) {
    get_static_qkd_session().set_key_length(len);
}

pub fn get_key_length() -> usize {
    get_static_qkd_session().get_key_length()
}

pub fn set_rx(rx: Receiver<Key>) {
    get_static_qkd_session().set_rx(rx);
}

pub fn get_rx() -> &'static Receiver<Key> {
    get_static_qkd_session().get_rx()
}

impl QkdSession {
    fn set_is_server(&mut self, b: bool) {
        self.is_server = b;
    }

    fn get_is_server(&self) -> bool {
        self.is_server
    }

    fn set_key_length(&mut self, len: usize) {
        self.key_length = len;
    }

    fn get_key_length(&self) -> usize {
        self.key_length
    }

    fn set_rx(&mut self, rx: Receiver<Key>) {
        self.rx = Some(rx);
    }

    fn get_rx(&self) -> &Receiver<Key> {
        if let Some (rx) = &self.rx {
            rx
        } else {
            panic!("rx is None");
        }
    }
}