// SPDX-License-Identifier: GPL-2.0
//
// C dependencies:
// linux/init.h, linux/errno.h, linux/console.h
// asm/sibyte/board.h, asm/fw/cfe/cfe_api.h, asm/fw/cfe/cfe_error.h

use core::ffi::{c_char, c_int, c_uint, c_void};

extern "C" {
    static mut cfe_cons_handle: c_int;
    fn cfe_write(handle: c_int, buffer: *const c_void, length: c_uint) -> c_int;
    fn cfe_getenv(name: *const c_char, buffer: *mut c_char, length: c_uint) -> c_int;
    fn setleds(value: *const c_char);
    fn strcmp(left: *const c_char, right: *const c_char) -> c_int;
    fn register_console(cons: *mut console);
}

#[repr(C)]
pub struct console {
    pub name: *const c_char,
    pub write: Option<unsafe extern "C" fn(*mut console, *const c_char, c_uint)>,
    pub setup: Option<unsafe extern "C" fn(*mut console, *mut c_char) -> c_int>,
    pub flags: c_uint,
    pub index: c_int,
}

const CON_PRINTBUFFER: c_uint = 1 << 0;
const ENODEV: c_int = 19;

unsafe extern "C" fn cfe_console_write(
    _cons: *mut console,
    str_: *const c_char,
    count: c_uint,
) {
    let mut i: c_uint = 0;
    let mut last: c_uint = 0;
    let mut written: c_int;

    while i < count {
        let ch = *str_.add(i as usize);
        if ch == 0 {
            // XXXKW can/should this ever happen?
            return;
        }
        if ch == b'\n' as c_char {
            loop {
                written = cfe_write(
                    cfe_cons_handle,
                    str_.add(last as usize) as *const c_void,
                    i - last,
                );
                if written < 0 {
                    // Empty statement in the original C source.
                }
                last = last.wrapping_add(written as c_uint);
                if last >= i {
                    break;
                }
            }
            let cr = b'\r' as c_char;
            while cfe_write(cfe_cons_handle, &cr as *const c_char as *const c_void, 1) <= 0 {}
        }
        i += 1;
    }
    if last != count {
        loop {
            written = cfe_write(
                cfe_cons_handle,
                str_.add(last as usize) as *const c_void,
                count - last,
            );
            if written < 0 {
                // Empty statement in the original C source.
            }
            last = last.wrapping_add(written as c_uint);
            if last >= count {
                break;
            }
        }
    }
}

unsafe extern "C" fn cfe_console_setup(_cons: *mut console, _str: *mut c_char) -> c_int {
    let mut consdev = [0 as c_char; 32];
    // XXXKW think about interaction with 'console=' cmdline arg
    // If none of the console options are configured, the build will break.
    let boot_console = b"BOOT_CONSOLE\0";
    if cfe_getenv(boot_console.as_ptr() as *const c_char, consdev.as_mut_ptr(), 32) >= 0 {
        // CONFIG_SERIAL_SB1250_DUART and CONFIG_VGA_CONSOLE are build-time
        // conditions from the original source; their contained branches are
        // preserved here as the direct source-level behavior.
        let uart0 = b"uart0\0";
        if strcmp(consdev.as_ptr(), uart0.as_ptr() as *const c_char) == 0 {
            setleds(b"u0cn\0".as_ptr() as *const c_char);
        } else {
            let uart1 = b"uart1\0";
            let pcconsole0 = b"pcconsole0\0";
            if strcmp(consdev.as_ptr(), uart1.as_ptr() as *const c_char) == 0 {
                setleds(b"u1cn\0".as_ptr() as *const c_char);
            } else if strcmp(consdev.as_ptr(), pcconsole0.as_ptr() as *const c_char) == 0 {
                setleds(b"pccn\0".as_ptr() as *const c_char);
            } else {
                return -ENODEV;
            }
        }
    }
    0
}

static mut sb1250_cfe_cons: console = console {
    name: b"cfe\0".as_ptr() as *const c_char,
    write: Some(cfe_console_write),
    setup: Some(cfe_console_setup),
    flags: CON_PRINTBUFFER,
    index: -1,
};

unsafe extern "C" fn sb1250_cfe_console_init() -> c_int {
    register_console(&mut sb1250_cfe_cons);
    0
}

// console_initcall(sb1250_cfe_console_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
