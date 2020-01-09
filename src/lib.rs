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


#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn QKD_open(destination: *mut c_char, qos: QoS, key_handle_ptr: *mut u8) -> u32 {
    println!("QKD_OPEN");

    // ignore destination for now

    session::set_key_length(qos.requested_length as usize);
    println!("qos req lengt: {}", qos.requested_length);

    let key_handle = KeyHandle::from(key_handle_ptr);
    println!("key_handle: {:x?}", key_handle_ptr);
    key_handle.print();

    if key_handle.is_null() {
        session::set_is_server(true);
        key_handle.set_value();
    }

    let (tx, rx): (Sender<Key>, Receiver<Key>) = mpsc::channel();
    session::set_rx(rx);

    println!("spawning qkd generator...");
    quantum::spawn_qkd_generator(tx);
    println!("spawned");

    println!("QKD_open end");

    0
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn QKD_connect_nonblock(key_handle: *mut u8) -> u32 {
    println!("QKD_CONNECT_NONBLOCK");

    0
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn QKD_connect_blocking(key_handle: *mut u8, timeout: u32) -> u32 {
    println!("QKD_CONNECT_BLOCKING");

    0
}


#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn QKD_get_key(key_handle: *mut u8, key_buffer: *mut u8) -> u32 {
    println!("QKD_GET_KEY");

    let key = session::get_rx().recv().unwrap();
    println!("key: {:?}", key.to_string());
    
    key.write_to_buffer(key_buffer);

    println!("QKD_get_key end");

    0
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn QKD_close(key_handle: *mut u8) -> u32 {
    println!("QKD_CLOSE");

    0
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn QKD_init(am_server: bool) -> u32 {
    0
}