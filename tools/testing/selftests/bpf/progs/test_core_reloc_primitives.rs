// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// C dependencies: <linux/bpf.h>, <stdint.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>.

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem;

unsafe extern "C" {
    fn bpf_core_read(dst: *mut c_void, sz: u32, src: *const c_void) -> c_long;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [c_char; 4] = [
    b'G' as c_char,
    b'P' as c_char,
    b'L' as c_char,
    0 as c_char,
];

#[repr(C)]
pub struct Data {
    pub in_: [c_char; 256],
    pub out: [c_char; 256],
}

#[no_mangle]
pub static mut data: Data = Data {
    in_: [0; 256],
    out: [0; 256],
};

#[repr(C)]
#[derive(Copy, Clone)]
pub enum core_reloc_primitives_enum {
    A = 0,
    B = 1,
}

#[repr(C)]
pub struct core_reloc_primitives {
    pub a: c_char,
    pub b: c_int,
    pub c: core_reloc_primitives_enum,
    pub d: *mut c_void,
    pub f: Option<unsafe extern "C" fn(*const c_char) -> c_int>,
}

unsafe fn CORE_READ<T>(dst: *mut T, src: *const T) -> c_long {
    unsafe {
        bpf_core_read(
            dst as *mut c_void,
            mem::size_of::<T>() as u32,
            src as *const c_void,
        )
    }
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn test_core_primitives(ctx: *mut c_void) -> c_int {
    let in_: *mut core_reloc_primitives =
        unsafe { &raw mut data.in_ } as *mut _ as *mut core_reloc_primitives;
    let out: *mut core_reloc_primitives =
        unsafe { &raw mut data.out } as *mut _ as *mut core_reloc_primitives;

    if unsafe { CORE_READ(&raw mut (*out).a, &raw const (*in_).a) } != 0
        || unsafe { CORE_READ(&raw mut (*out).b, &raw const (*in_).b) } != 0
        || unsafe { CORE_READ(&raw mut (*out).c, &raw const (*in_).c) } != 0
        || unsafe { CORE_READ(&raw mut (*out).d, &raw const (*in_).d) } != 0
        || unsafe { CORE_READ(&raw mut (*out).f, &raw const (*in_).f) } != 0
    {
        return 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
