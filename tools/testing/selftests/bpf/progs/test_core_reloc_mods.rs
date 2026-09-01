// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019 Facebook

// C dependencies: <linux/bpf.h>, <stdint.h>, <bpf/bpf_helpers.h>,
// and <bpf/bpf_core_read.h>.

use core::ffi::c_void;

extern "C" {
    fn bpf_core_read(dst: *mut c_void, sz: u32, src: *const c_void) -> i64;
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
pub struct core_reloc_mods_output {
    pub a: i32,
    pub b: i32,
    pub c: i32,
    pub d: i32,
    pub e: i32,
    pub f: i32,
    pub g: i32,
    pub h: i32,
}

pub type int_t = i32;
pub type char_ptr_t = *const i8;
pub type arr_t = [i32; 7];

#[repr(C)]
pub struct core_reloc_mods_substruct {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct core_reloc_mods_substruct_t {
    pub x: i32,
    pub y: i32,
}

#[repr(C)]
pub struct core_reloc_mods {
    pub a: i32,
    pub b: int_t,
    pub c: *mut i8,
    pub d: char_ptr_t,
    pub e: [i32; 3],
    pub f: arr_t,
    pub g: core_reloc_mods_substruct,
    pub h: core_reloc_mods_substruct_t,
}

#[cfg(target_endian = "little")]
unsafe fn CORE_READ<D, S>(dst: *mut D, src: *const S) -> i64 {
    bpf_core_read(
        dst as *mut c_void,
        core::mem::size_of::<D>() as u32,
        src as *const c_void,
    )
}

#[cfg(not(target_endian = "little"))]
unsafe fn CORE_READ<D, S>(dst: *mut D, src: *const S) -> i64 {
    let dst_sz = core::mem::size_of::<D>();
    let src_sz = core::mem::size_of::<S>();
    let __sz = if dst_sz < src_sz { dst_sz } else { src_sz };

    bpf_core_read(
        (dst as *mut i8).add(dst_sz - __sz) as *mut c_void,
        __sz as u32,
        (src as *const i8).add(src_sz - __sz) as *const c_void,
    )
}

#[no_mangle]
#[link_section = "raw_tracepoint/sys_enter"]
pub unsafe extern "C" fn test_core_mods(ctx: *mut c_void) -> i32 {
    let in_: *mut core_reloc_mods = data.in_.as_mut_ptr() as *mut core_reloc_mods;
    let out: *mut core_reloc_mods_output = data.out.as_mut_ptr() as *mut core_reloc_mods_output;

    if CORE_READ(&mut (*out).a as *mut i32, &(*in_).a as *const i32) != 0
        || CORE_READ(&mut (*out).b as *mut i32, &(*in_).b as *const int_t) != 0
        || CORE_READ(&mut (*out).c as *mut i32, &(*in_).c as *const *mut i8) != 0
        || CORE_READ(&mut (*out).d as *mut i32, &(*in_).d as *const char_ptr_t) != 0
        || CORE_READ(&mut (*out).e as *mut i32, &(*in_).e[2] as *const i32) != 0
        || CORE_READ(&mut (*out).f as *mut i32, &(*in_).f[1] as *const i32) != 0
        || CORE_READ(&mut (*out).g as *mut i32, &(*in_).g.x as *const i32) != 0
        || CORE_READ(&mut (*out).h as *mut i32, &(*in_).h.y as *const i32) != 0
    {
        return 1;
    }

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
