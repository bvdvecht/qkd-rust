#![allow(non_snake_case)]
#![allow(unused_variables)]

use std::os::raw::c_char;
use std::sync::mpsc;
use mpsc::Sender;
use mpsc::Receiver;

mod session;

mod quantum;
use quantum::Key;

mod keyhandle;
use keyhandle::KeyHandle;


#[repr(C)]
pub struct QoS {
    requested_length: u32,
    max_bps: u32,
    priority: u32,
    timeout: u32
}


// QKD API

#[no_mangle]
pub extern fn QKD_open(destination: *mut c_char, qos: QoS, key_handle_ptr: *mut u8) -> u32 {
    println!("QKD_OPEN");

    // ignore destination for now

    println!("qos req lengt: {}", qos.requested_length);
    println!("key_handle: {:x?}", key_handle_ptr);

    let key_handle = KeyHandle::from(key_handle_ptr);
    key_handle.print();

    // create channels for communication between this and qkd threads
    let (tx, rx): (Sender<Key>, Receiver<Key>) = mpsc::channel();

    if key_handle.is_null() {
        key_handle.set_value();
        session::create_qkd_session(true, qos.requested_length as usize, key_handle, rx);
    } else {
        session::create_qkd_session(false, qos.requested_length as usize, key_handle, rx)
    }

    println!("spawning qkd generator...");
    quantum::spawn_qkd_thread(tx);
    println!("spawned");

    println!("QKD_open end");

    0
}

#[no_mangle]
pub extern fn QKD_connect_nonblock(key_handle: *mut u8) -> u32 {
    println!("QKD_CONNECT_NONBLOCK");
    0
}

#[no_mangle]
pub extern fn QKD_connect_blocking(key_handle: *mut u8, timeout: u32) -> u32 {
    println!("QKD_CONNECT_BLOCKING");
    0
}


#[no_mangle]
pub extern fn QKD_get_key(key_handle: *mut u8, key_buffer: *mut u8) -> u32 {
    println!("QKD_GET_KEY");

    // wait for key from qkd thread
    let key = session::get_rx().recv().unwrap();
    println!("key: {:?}", key.to_string());
    
    // copy to buffer
    key.write_to_buffer(key_buffer);

    println!("QKD_get_key end");

    0
}

#[no_mangle]
pub extern fn QKD_close(key_handle: *mut u8) -> u32 {
    println!("QKD_CLOSE");
    0
}

#[no_mangle]
pub extern fn QKD_init(am_server: bool) -> u32 {
    0
}