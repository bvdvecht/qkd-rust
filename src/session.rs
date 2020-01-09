#![allow(dead_code)]

use std::sync::mpsc;
use mpsc::Receiver;

use crate::quantum::Key;
use crate::keyhandle::KeyHandle;

// singleton QkdSession
static mut QKD_SESSION: Option<QkdSession> = None;

pub struct QkdSession {
    is_server: bool,
    key_length: usize,
    key_handle: KeyHandle,
    rx: Receiver<Key>
}

fn get_qkd_session() -> &'static mut QkdSession {
    unsafe {
        if let Some(session) = &mut QKD_SESSION {
            session
        } else {
            panic!("trying to get session but it does not exist");
        }
    }
}

pub fn create_qkd_session(is_server: bool, key_length: usize, key_handle: KeyHandle, rx: Receiver<Key>) {
    unsafe {
        QKD_SESSION = Some(QkdSession {is_server, key_length, key_handle, rx});
    }
}

pub fn get_is_server() -> bool {
    get_qkd_session().get_is_server()
}

pub fn get_key_length() -> usize {
    get_qkd_session().get_key_length()
}

pub fn get_rx() -> &'static Receiver<Key> {
    get_qkd_session().get_rx()
}

impl QkdSession {
    fn get_is_server(&self) -> bool {
        self.is_server
    }

    fn get_key_length(&self) -> usize {
        self.key_length
    }

    fn get_rx(&self) -> &Receiver<Key> {
        &self.rx
    }
}