// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) Meta Platforms, Inc. and affiliates. */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

type __s64 = i64;
type __u32 = u32;

/* From <linux/bpf.h> / <bpf/bpf_helpers.h>. */
const BPF_ANY: u64 = 0;
const BPF_MAP_TYPE_HASH: u32 = 1;

#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn bpf_map_update_elem(
        map: *mut core::ffi::c_void,
        key: *const core::ffi::c_void,
        value: *const core::ffi::c_void,
        flags: u64,
    ) -> i64;
}

/* volatile to force a read */
#[no_mangle]
pub static var1: i32 = 0;

#[no_mangle]
pub static mut var2: i32 = 1;

#[repr(C)]
pub struct var3_t {
    pub var3_1: i32,
    pub var3_2: __s64,
}

#[no_mangle]
pub static mut var3: var3_t = var3_t {
    var3_1: 0,
    var3_2: 0,
};

#[no_mangle]
pub static mut libout1: i32 = 0;

/* extern volatile bool CONFIG_BPF_SYSCALL __kconfig; */
unsafe extern "C" {
    static CONFIG_BPF_SYSCALL: bool;
}

#[no_mangle]
pub static mut var4: [i32; 4] = [0; 4];

/* __weak int var5 SEC(".data"); */
#[no_mangle]
#[link_section = ".data"]
pub static mut var5: i32 = 0;

/* Fully contained within library extern-and-definition */
unsafe extern "C" {
    static mut var6: i32;
}

#[no_mangle]
#[link_section = ".data.custom"]
pub static mut var7: i32 = 0;

#[no_mangle]
pub static mut fn_ptr: Option<unsafe extern "C" fn() -> i32> = None;

#[repr(C)]
pub struct bpf_map_def_hash_u32_u32_16 {
    pub type_: u32,
    pub key_size: u32,
    pub value_size: u32,
    pub max_entries: u32,
}

#[no_mangle]
#[link_section = ".maps"]
pub static mut map1: bpf_map_def_hash_u32_u32_16 = bpf_map_def_hash_u32_u32_16 {
    type_: BPF_MAP_TYPE_HASH,
    key_size: core::mem::size_of::<__u32>() as u32,
    value_size: core::mem::size_of::<__u32>() as u32,
    max_entries: 16,
};

unsafe extern "C" {
    #[link_section = ".maps"]
    static mut map2: bpf_map_def_hash_u32_u32_16;
}

#[no_mangle]
pub unsafe extern "C" fn lib_routine() -> i32 {
    let key: __u32 = 1;
    let value: __u32 = 2;

    let _ = core::ptr::addr_of!(CONFIG_BPF_SYSCALL).read_volatile();
    bpf_map_update_elem(
        core::ptr::addr_of_mut!(map2).cast::<core::ffi::c_void>(),
        core::ptr::addr_of!(key).cast::<core::ffi::c_void>(),
        core::ptr::addr_of!(value).cast::<core::ffi::c_void>(),
        BPF_ANY,
    );

    libout1 = core::ptr::addr_of!(var1).read_volatile()
        + core::ptr::addr_of!(var2).read_volatile()
        + var3.var3_1
        + var3.var3_2 as i32
        + var5
        + var6;
    libout1
}

#[no_mangle]
#[link_section = "perf_event"]
pub unsafe extern "C" fn lib_perf_handler(_ctx: *mut pt_regs) -> i32 {
    0
}

#[no_mangle]
#[link_section = "license"]
pub static mut LICENSE: [u8; 4] = *b"GPL\0";

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
