// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// C dependencies: <linux/bpf.h>, <stdint.h>, <bpf/bpf_helpers.h>,
// <bpf/bpf_core_read.h>

extern "C" {
    fn bpf_core_read(dst: *mut core::ffi::c_void, sz: u32, src: *const core::ffi::c_void) -> i64;
}

#[link_section = "license"]
#[no_mangle]
pub static mut _license: [u8; 4] = *b"GPL\0";

#[repr(C)]
pub struct Data {
    pub in_: [u8; 256],
    pub out: [u8; 256],
}

#[no_mangle]
pub static mut data: Data = Data {
    in_: [0; 256],
    out: [0; 256],
};

#[repr(C)]
pub struct core_reloc_ptr_as_arr {
    pub a: i32,
}

unsafe fn CORE_READ<T>(dst: *mut T, src: *const core::ffi::c_void) -> i64 {
    bpf_core_read(
        dst as *mut core::ffi::c_void,
        core::mem::size_of::<T>() as u32,
        src,
    )
}

#[link_section = "raw_tracepoint/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn test_core_ptr_as_arr(ctx: *mut core::ffi::c_void) -> i32 {
    let in_: *mut core_reloc_ptr_as_arr = core::ptr::addr_of_mut!(data.in_) as *mut core_reloc_ptr_as_arr;
    let out: *mut core_reloc_ptr_as_arr = core::ptr::addr_of_mut!(data.out) as *mut core_reloc_ptr_as_arr;

    let _ = ctx;

    if CORE_READ(
        core::ptr::addr_of_mut!((*out).a),
        core::ptr::addr_of!((*in_.add(2)).a) as *const core::ffi::c_void,
    ) != 0
    {
        return 1;
    }

    return 0;
}
