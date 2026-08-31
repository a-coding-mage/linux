// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

use core::ffi::c_void;
use core::mem::size_of;

unsafe extern "C" {
    fn bpf_core_read(dst: *mut c_void, size: u32, src: *const c_void) -> i64;
}

#[no_mangle]
#[link_section = "license"]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct Data {
    pub in_: [i8; 256],
    pub out: [i8; 256],
}

#[no_mangle]
pub static mut data: Data = Data {
    in_: [0; 256],
    out: [0; 256],
};

#[repr(C)]
pub struct core_reloc_ints {
    pub u8_field: u8,
    pub s8_field: i8,
    pub u16_field: u16,
    pub s16_field: i16,
    pub u32_field: u32,
    pub s32_field: i32,
    pub u64_field: u64,
    pub s64_field: i64,
}

unsafe fn CORE_READ<T>(dst: *mut T, src: *const T) -> i64 {
    unsafe {
        bpf_core_read(
            dst as *mut c_void,
            size_of::<T>() as u32,
            src as *const c_void,
        )
    }
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn test_core_ints(ctx: *mut c_void) -> i32 {
    let _ = ctx;
    let in_: *mut core_reloc_ints = unsafe { &raw mut data.in_ as *mut _ as *mut core_reloc_ints };
    let out: *mut core_reloc_ints = unsafe { &raw mut data.out as *mut _ as *mut core_reloc_ints };

    if unsafe {
        CORE_READ(&raw mut (*out).u8_field, &raw const (*in_).u8_field)
            || CORE_READ(&raw mut (*out).s8_field, &raw const (*in_).s8_field)
            || CORE_READ(&raw mut (*out).u16_field, &raw const (*in_).u16_field)
            || CORE_READ(&raw mut (*out).s16_field, &raw const (*in_).s16_field)
            || CORE_READ(&raw mut (*out).u32_field, &raw const (*in_).u32_field)
            || CORE_READ(&raw mut (*out).s32_field, &raw const (*in_).s32_field)
            || CORE_READ(&raw mut (*out).u64_field, &raw const (*in_).u64_field)
            || CORE_READ(&raw mut (*out).s64_field, &raw const (*in_).s64_field)
    } != 0
    {
        return 1;
    }

    0
}
