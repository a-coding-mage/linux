// SPDX-License-Identifier: GPL-2.0
//! Compressed-kernel diagnostics and halt path.
use crate::bindings;
use core::ffi::c_char;

#[no_mangle]
pub(crate) unsafe extern "C" fn warn(message: *const c_char) {
    // SAFETY: callers provide a terminated string; __putstr is the boot console.
    unsafe {
        bindings::__putstr(c"\n\n".as_ptr());
        bindings::__putstr(message);
        bindings::__putstr(c"\n\n".as_ptr());
    }
}

#[no_mangle]
pub(crate) unsafe extern "C" fn error(message: *mut c_char) -> ! {
    // SAFETY: error preserves the original reporting and x86 halt loop.
    unsafe {
        warn(message);
        bindings::__putstr(c" -- System halted".as_ptr());
        loop {
            core::arch::asm!("hlt", options(nomem, nostack));
        }
    }
}

#[cfg(CONFIG_EFI_STUB)]
extern "C" {
    fn vsnprintf(
        buffer: *mut c_char,
        size: usize,
        format: *const c_char,
        args: core::ffi::VaList<'_, '_>,
    ) -> core::ffi::c_int;
}

#[cfg(CONFIG_EFI_STUB)]
#[no_mangle]
pub(crate) unsafe extern "C" fn panic(format: *const c_char, mut args: ...) -> ! {
    static mut BUFFER: [c_char; 1024] = [0; 1024];
    // SAFETY: rustc supplies the actual SysV va_list ABI for this variadic
    // function; the libstub function consumes it before the list is dropped.
    unsafe {
        let buffer = core::ptr::addr_of_mut!(BUFFER).cast::<c_char>();
        let len = vsnprintf(buffer, 1024, format, args.as_va_list());
        // Preserve C's return-value/last-byte check and pointer ordering.
        if len != 0 && *buffer.offset(len.wrapping_sub(1) as isize) == b'\n' as c_char {
            *buffer.offset(len.wrapping_sub(1) as isize) = 0;
        }
        error(buffer)
    }
}
