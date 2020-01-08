use std::collections::HashMap;
// use std::ffi::CString;
use std::os::raw::c_char;

use std::sync::mpsc;
use mpsc::Sender;
use mpsc::Receiver;

mod session;
mod quantum;
use quantum::Key;



#[repr(C)]
pub struct qos_t {
    requested_length: u32,
    max_bps: u32,
    priority: u32,
    timeout: u32
}

// const IP_ADDR_MAX_LEN: usize = 16;

// #[repr(C)]
// pub struct ip_address_t {
//     length: u8,
//     address: [u8; IP_ADDR_MAX_LEN]
// }

// impl ip_address_t {
//     fn get_address(&self) -> String {
//         let mut s = String::from_utf8_lossy(&self.address).to_string();
//         s.pop();
//         s
//     }
// }



fn key_handle_is_null(handle: *mut u8) -> bool {
    for i in 0..64 {
        unsafe {
            if *handle.offset(i) != 0 {
                return false;
            }
        }
    }
    true
}

fn print_key_handle(handle: *mut u8) {
    for i in 0..64 {
        unsafe {
            let u = *handle.offset(i);
            print!("{:x?}", u);
        }
    }
    println!("");
}

fn write_key_handle(handle: *mut u8) {
    for i in 0..64 {
        unsafe {
            std::ptr::write(handle.offset(i), 3);
        }
    }
}


#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn QKD_open(destination: *mut c_char, qos: qos_t, key_handle_ptr: *mut u8) -> u32 {
    println!("QKD_OPEN");

    // ignore destination for now

    println!("qos req lengt: {}", qos.requested_length);
    println!("qos max bps: {}", qos.max_bps);
    println!("qos priority: {}", qos.priority);
    println!("qos timeout: {}", qos.timeout);

    session::set_key_length(qos.requested_length as usize);
    println!("key_handle: {:x?}", key_handle_ptr);
    
    
    print_key_handle(key_handle_ptr);
    if key_handle_is_null(key_handle_ptr) {
        session::set_is_server(true);

        write_key_handle(key_handle_ptr);
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

fn copy_key_to_buffer(key: &Key, buffer: *mut u8) {
    println!("copying {} bytes of key into buffer", key.value.len());
    let str_slice = unsafe {
        std::slice::from_raw_parts_mut(buffer, key.value.len())
    };
    str_slice[..key.value.len()].copy_from_slice(&key.value);
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn QKD_get_key(key_handle: *mut u8, key_buffer: *mut u8) -> u32 {
    println!("QKD_GET_KEY");

    // let key_length = session::get_key_length();
    // let mut key = Key::from(key_buffer, key_length);

    let key = session::get_rx().recv().unwrap();
    copy_key_to_buffer(&key, key_buffer);

    // key.value = vec![123; key_length];
    
    println!("key: {:?}", key.to_string());

    0
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn QKD_close(key_handle: *mut u8) -> u32 {
    println!("QKD_CLOSE");

    let mut map: HashMap<Vec<u8>, u32> = HashMap::new();
    map.insert(vec![0; 64], 0);
    map.insert(vec![0, 1], 1);
    map.insert(vec![0; 63], 2);

    if let Some(value) = map.get(&vec![0; 63]) {
        println!("value: {}", value);
    }

    0
}

#[no_mangle]
#[allow(non_snake_case)]
#[allow(unused_variables)]
pub extern fn QKD_init(am_server: bool) -> u32 {
    0
}