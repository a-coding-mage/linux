/* SPDX-License-Identifier: GPL-2.0 */

// Forward declarations from C header.
#[repr(C)]
pub struct termios {
    _private: [u8; 0],
}

#[repr(C)]
pub struct winsize {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn get_term_dimensions(ws: *mut winsize);
    pub fn set_term_quiet_input(old: *mut termios);
}
