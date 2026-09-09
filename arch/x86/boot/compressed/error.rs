// SPDX-License-Identifier: GPL-2.0
/*
 * Callers outside of misc.c need access to the error reporting routines,
 * but the *_putstr() functions need to stay in misc.c because of how
 * memcpy() and memmove() are defined for the compressed boot environment.
 */

use core::ffi::{c_char, c_int};

// Supplied by misc.c and the error-reporting interface.
unsafe extern "C" {
    fn error_putstr(m: *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn warn(m: *const c_char) {
    error_putstr(b"\n\n\0".as_ptr() as *const c_char);
    error_putstr(m);
    error_putstr(b"\n\n\0".as_ptr() as *const c_char);
}

#[no_mangle]
pub unsafe extern "C" fn error(m: *mut c_char) {
    warn(m as *const c_char);
    error_putstr(b" -- System halted\0".as_ptr() as *const c_char);

    loop {
        core::arch::asm!("hlt");
    }
}

/* EFI libstub provides vsnprintf(). */
#[cfg(CONFIG_EFI_STUB)]
mod efi_stub {
    use super::{c_char, c_int, error};

    // C va_list and the variadic vsnprintf() are supplied by the EFI libstub.
    #[repr(C)]
    pub struct va_list {
        _private: [u8; 0],
    }

    unsafe extern "C" {
        fn vsnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    }

    #[no_mangle]
    pub unsafe extern "C" fn panic(fmt: *const c_char, ...) {
        static mut BUF: [c_char; 1024] = [0; 1024];
        let mut args: va_list = va_list { _private: [] };
        let len: c_int;

        // Corresponds to va_start(args, fmt).
        core::arch::asm!("", inout("rdi") args, options(nostack, preserves_flags));
        len = vsnprintf(BUF.as_mut_ptr(), BUF.len(), fmt, args);
        // Corresponds to va_end(args).
        core::arch::asm!("", inout("rdi") args, options(nostack, preserves_flags));

        if len != 0 && BUF[(len - 1) as usize] == b'\n' as c_char {
            BUF[(len - 1) as usize] = 0;
        }

        error(BUF.as_mut_ptr());
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
