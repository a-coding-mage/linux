/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: symbols from <asm/debug.h> are supplied by other files.

extern "C" {
    pub static mut pci_debug_msg_id: *mut debug_info_t;
    pub static mut pci_debug_err_id: *mut debug_info_t;
}

#[macro_export]
macro_rules! zpci_dbg {
    ($imp:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        unsafe {
            debug_sprintf_event(
                pci_debug_msg_id,
                $imp,
                $fmt $(, $args)*
            )
        }
    };
}

#[macro_export]
macro_rules! zpci_err {
    ($($text:tt)*) => {{
        let mut debug_buffer = [0u8; 16];
        unsafe {
            snprintf(debug_buffer.as_mut_ptr(), 16, $($text)*);
            debug_text_event(
                pci_debug_err_id,
                0,
                debug_buffer.as_mut_ptr() as *mut core::ffi::c_char,
            );
        }
    }};
}

#[inline]
pub unsafe fn zpci_err_hex_level(level: i32, addr: *mut core::ffi::c_void, len: i32) {
    unsafe {
        debug_event(pci_debug_err_id, level, addr, len);
    }
}

#[inline]
pub unsafe fn zpci_err_hex(addr: *mut core::ffi::c_void, len: i32) {
    unsafe {
        zpci_err_hex_level(0, addr, len);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
