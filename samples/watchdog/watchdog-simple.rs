// SPDX-License-Identifier: GPL-2.0
// C dependencies: stdio.h, stdlib.h, unistd.h, and fcntl.h.

unsafe extern "C" {
    fn open(pathname: *const core::ffi::c_char, flags: i32, ...) -> i32;
    fn perror(s: *const core::ffi::c_char);
    fn exit(status: i32) -> !;
    fn write(fd: i32, buf: *const core::ffi::c_void, count: usize) -> isize;
    fn sleep(seconds: u32) -> u32;
    fn close(fd: i32) -> i32;
}

const O_WRONLY: i32 = 1;
const EXIT_FAILURE: i32 = 1;

fn main() {
    let fd: i32;
    let mut ret: i32 = 0;

    unsafe {
        fd = open(b"/dev/watchdog\0".as_ptr() as *const core::ffi::c_char, O_WRONLY);
        if fd == -1 {
            perror(b"watchdog\0".as_ptr() as *const core::ffi::c_char);
            exit(EXIT_FAILURE);
        }

        loop {
            ret = write(fd, b"\0".as_ptr() as *const core::ffi::c_void, 1) as i32;
            if ret != 1 {
                ret = -1;
                break;
            }
            sleep(10);
        }
        close(fd);
    }

    std::process::exit(ret);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
