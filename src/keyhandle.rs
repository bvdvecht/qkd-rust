const KEY_HANDLE_LENGTH: isize = 64;

pub struct KeyHandle(*mut u8);

impl KeyHandle {
    pub fn from(ptr: *mut u8) -> KeyHandle {
        KeyHandle(ptr)
    }

    pub fn is_null(&self) -> bool {
        println!("checking if key handle is null...");

        for i in 0..KEY_HANDLE_LENGTH {
            unsafe {
                if *self.0.offset(i) != 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn print(&self) {
        for i in 0..KEY_HANDLE_LENGTH {
            unsafe {
                let u = *self.0.offset(i);
                print!("{:x?}", u);
            }
        }
        println!("");
    }


    pub fn set_value(&self) {
        // for now, set fixed value
        for i in 0..KEY_HANDLE_LENGTH {
            unsafe {
                std::ptr::write(self.0.offset(i), 3);
            }
        }
    }
}