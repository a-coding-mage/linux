// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// C includes translated as external dependency intent:
// <linux/bpf.h>, <stdint.h>, <bpf/bpf_helpers.h>, <bpf/bpf_core_read.h>

#[link_section = "license"]
#[no_mangle]
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
pub struct core_reloc_arrays_output {
    pub a2: i32,
    pub a3: i32,
    pub b123: i8,
    pub c1c: i32,
    pub d00d: i32,
    pub f01c: i32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct core_reloc_arrays_substruct {
    pub c: i32,
    pub d: i32,
}

#[repr(C)]
pub struct core_reloc_arrays {
    pub a: [i32; 5],
    pub b: [[[i8; 4]; 3]; 2],
    pub c: [core_reloc_arrays_substruct; 3],
    pub d: [[core_reloc_arrays_substruct; 2]; 1],
    // Flexible array member in C: struct core_reloc_arrays_substruct f[][2];
    pub f: [[core_reloc_arrays_substruct; 2]; 0],
}

extern "C" {
    pub fn bpf_core_read(dst: *mut core::ffi::c_void, sz: u32, src: *const core::ffi::c_void) -> i64;
}

// #define CORE_READ(dst, src) bpf_core_read(dst, sizeof(*(dst)), src)
unsafe fn CORE_READ<T>(dst: *mut T, src: *const T) -> i64 {
    bpf_core_read(
        dst as *mut core::ffi::c_void,
        core::mem::size_of::<T>() as u32,
        src as *const core::ffi::c_void,
    )
}

// SEC("raw_tracepoint/sys_enter")
#[link_section = "raw_tracepoint/sys_enter"]
#[no_mangle]
pub unsafe extern "C" fn test_core_arrays(ctx: *mut core::ffi::c_void) -> i32 {
    let in_: *mut core_reloc_arrays = core::ptr::addr_of_mut!(data.in_) as *mut core_reloc_arrays;
    let out: *mut core_reloc_arrays_output =
        core::ptr::addr_of_mut!(data.out) as *mut core_reloc_arrays_output;
    let mut a: *mut i32;

    let _ = ctx;

    if CORE_READ(
        core::ptr::addr_of_mut!((*out).a2),
        core::ptr::addr_of!((*in_).a[2]),
    ) != 0
    {
        return 1;
    }
    if CORE_READ(
        core::ptr::addr_of_mut!((*out).b123),
        core::ptr::addr_of!((*in_).b[1][2][3]),
    ) != 0
    {
        return 1;
    }
    if CORE_READ(
        core::ptr::addr_of_mut!((*out).c1c),
        core::ptr::addr_of!((*in_).c[1].c),
    ) != 0
    {
        return 1;
    }
    if CORE_READ(
        core::ptr::addr_of_mut!((*out).d00d),
        core::ptr::addr_of!((*in_).d[0][0].d),
    ) != 0
    {
        return 1;
    }
    if CORE_READ(
        core::ptr::addr_of_mut!((*out).f01c),
        core::ptr::addr_of!((*in_).f) as *const core_reloc_arrays_substruct,
    ) != 0
    {
        return 1;
    }

    // __builtin_preserve_access_index(({ in->a; }))
    a = core::ptr::addr_of_mut!((*in_).a) as *mut i32;
    (*out).a3 = *a.add(0) + *a.add(1) + *a.add(2) + *a.add(3);

    return 0;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
