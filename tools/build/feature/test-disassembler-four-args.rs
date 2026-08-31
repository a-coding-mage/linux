// SPDX-License-Identifier: GPL-2.0
// C dependencies: <bfd.h>, <dis-asm.h>

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct bfd {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn bfd_openr(filename: *const c_char, target: *const c_char) -> *mut bfd;
    pub fn bfd_get_arch(abfd: *mut bfd) -> c_int;
    pub fn bfd_big_endian(abfd: *mut bfd) -> c_int;
    pub fn bfd_get_mach(abfd: *mut bfd) -> c_uint;
    pub fn disassembler(
        arch: c_int,
        big: c_int,
        mach: c_uint,
        abfd: *mut bfd,
    ) -> *mut c_void;
}

fn main() {
    unsafe {
        let abfd: *mut bfd = bfd_openr(core::ptr::null(), core::ptr::null());

        disassembler(
            bfd_get_arch(abfd),
            bfd_big_endian(abfd),
            bfd_get_mach(abfd),
            abfd,
        );
    }
}
