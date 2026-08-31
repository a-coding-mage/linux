// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/compiler.h>, <linux/zalloc.h>, "../disasm.h"

use core::ffi::{c_char, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct e_machine_and_e_flags {
    _data: [u8; 0],
}

#[repr(C)]
pub struct objdump {
    pub comment_char: c_char,
}

#[repr(C)]
pub struct arch {
    pub name: *const c_char,
    pub id: e_machine_and_e_flags,
    pub objdump: objdump,
}

extern "C" {
    fn zalloc(size: usize) -> *mut c_void;
}

#[no_mangle]
pub unsafe extern "C" fn arch__new_arc(
    id: *const e_machine_and_e_flags,
    cpuid: *const c_char,
) -> *const arch {
    let _ = cpuid;
    let arch = zalloc(size_of::<arch>()) as *mut arch;

    if arch.is_null() {
        return ptr::null();
    }

    (*arch).name = b"arc\0".as_ptr() as *const c_char;
    ptr::copy_nonoverlapping(id, &mut (*arch).id, 1);
    (*arch).objdump.comment_char = b';' as c_char;
    arch
}
