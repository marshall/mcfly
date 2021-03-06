use libc;

extern {
    pub fn ioctl(fd: i8, request: u32, arg: *const u8) -> i8;
}

pub fn use_tiocsti(string: &String) {
    for byte in string.as_bytes() {
        let a: *const u8 = byte;
        if unsafe { ioctl(0, libc::TIOCSTI as u32, a) } < 0 {
            panic!("Error encountered when calling ioctl");
        }
    }
}
