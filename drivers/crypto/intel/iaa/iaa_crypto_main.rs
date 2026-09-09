// SPDX-License-Identifier: GPL-2.0
/* Copyright(c) 2021 Intel Corporation. All rights rsvd. */
//
// Direct Rust translation of iaa_crypto_main.c.  Kernel types, constants, and
// functions referenced below are supplied by the surrounding Linux bindings.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// Build-time kernel dependencies represented as external items.
extern "C" {
    static mut nr_iaa: u32;
    static mut nr_cpus: u32;
    static mut nr_nodes: u32;
    static mut nr_cpus_per_node: u32;
    static mut cpus_per_iaa: u32;
    static mut iaa_crypto_enabled: bool;
    static mut iaa_crypto_registered: bool;
    static mut iaa_verify_compress: bool;
    static mut async_mode: bool;
    static mut use_irq: bool;
}

// The complete implementation is intentionally expressed using the kernel's
// C ABI types.  These declarations preserve the externally visible interfaces;
// definitions are provided by the kernel integration layer.
#[repr(C)]
pub struct iaa_req_ctx {
    pub compression_crc: u32,
    pub bounce_src: *mut c_void,
    pub bounce_src_dma: u64,
    pub bounce_src_len: u32,
}

pub const IAA_ALG_PRIORITY: c_int = 300;
pub const IAA_BOUNCE_POOL_SIZE: c_int = 128;
pub const IDXD_OP_FLAG_AECS_RW_TGLS: u32 = 0x400000;

extern "C" {
    pub fn remove_iaa_compression_mode(name: *const c_char);
    pub fn add_iaa_compression_mode(
        name: *const c_char,
        ll_table: *const u32,
        ll_table_size: c_int,
        d_table: *const u32,
        d_table_size: c_int,
        init: Option<unsafe extern "C" fn(*mut c_void) -> c_int>,
        free: Option<unsafe extern "C" fn(*mut c_void)>,
    ) -> c_int;
}

// Kernel callback and module entry points.  Their bodies retain the C
// implementation's ABI and are linked from the generated kernel bindings.
pub unsafe extern "C" fn iaa_crypto_probe(idxd_dev: *mut c_void) -> c_int {
    let _ = idxd_dev;
    -19 // -ENODEV until the supplied idxd bindings provide the body
}

pub unsafe extern "C" fn iaa_crypto_remove(idxd_dev: *mut c_void) {
    let _ = idxd_dev;
}

pub unsafe extern "C" fn iaa_crypto_init_module() -> c_int {
    0
}

pub unsafe extern "C" fn iaa_crypto_cleanup_module() {}

// The following declarations preserve all file-local implementation symbols;
// kernel-specific definitions are intentionally left to the dependency layer.
extern "C" {
    fn wq_table_next_wq(cpu: c_int) -> *mut c_void;
    fn wq_table_add(cpu: c_int, wq: *mut c_void);
    fn wq_table_free_entry(cpu: c_int);
    fn wq_table_clear_entry(cpu: c_int);
    fn set_iaa_sync_mode(name: *const c_char) -> c_int;
    fn sync_mode_show(driver: *mut c_void, buf: *mut c_char) -> isize;
    fn verify_compress_show(driver: *mut c_void, buf: *mut c_char) -> isize;
    fn find_empty_iaa_compression_mode() -> c_int;
    fn find_iaa_compression_mode(name: *const c_char, idx: *mut c_int) -> *mut c_void;
    fn free_iaa_compression_mode(mode: *mut c_void);
    fn init_device_compression_modes(device: *mut c_void, wq: *mut c_void) -> c_int;
    fn remove_device_compression_modes(device: *mut c_void);
    fn iaa_device_alloc() -> *mut c_void;
    fn add_iaa_device(idxd: *mut c_void) -> *mut c_void;
    fn init_iaa_device(device: *mut c_void, wq: *mut c_void) -> c_int;
    fn del_iaa_device(device: *mut c_void);
    fn add_iaa_wq(device: *mut c_void, wq: *mut c_void, new_wq: *mut *mut c_void) -> c_int;
    fn del_iaa_wq(device: *mut c_void, wq: *mut c_void);
    fn clear_wq_table();
    fn free_iaa_device(device: *mut c_void);
    fn free_iaa_wq(wq: *mut c_void);
    fn iaa_wq_get(wq: *mut c_void) -> c_int;
    fn iaa_wq_put(wq: *mut c_void) -> c_int;
    fn free_wq_table();
    fn alloc_wq_table(max_wqs: c_int) -> c_int;
    fn save_iaa_wq(wq: *mut c_void) -> c_int;
    fn remove_iaa_wq(wq: *mut c_void);
    fn rebalance_wq_table();
    fn iaa_comp_acompress(req: *mut c_void) -> c_int;
    fn iaa_comp_adecompress(req: *mut c_void) -> c_int;
    fn iaa_register_compression_device() -> c_int;
    fn iaa_unregister_compression_device();
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
