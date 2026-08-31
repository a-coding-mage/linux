// SPDX-License-Identifier: GPL-2.0
// C dependencies: <stdio.h>, <dis-asm.h>

use core::ffi::c_int;
use core::mem::MaybeUninit;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
pub struct disassemble_info {
    _private: [u8; 0],
}

unsafe extern "C" {
    static mut stdout: *mut FILE;

    fn init_disassemble_info(
        info: *mut disassemble_info,
        stream: *mut FILE,
        fprintf_func: Option<unsafe extern "C" fn()>,
        fprintf_styled_func: Option<unsafe extern "C" fn()>,
    );
}

pub unsafe fn main() -> c_int {
    let mut info = MaybeUninit::<disassemble_info>::uninit();

    unsafe {
        init_disassemble_info(info.as_mut_ptr(), stdout, None, None);
    }

    0
}
