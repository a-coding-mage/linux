/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright(c) 2021 Intel Corporation. All rights rsvd. */

// C header dependencies are supplied by other translation units.

pub const IDXD_SUBDRIVER_NAME: &str = "crypto";

pub const IAA_DECOMP_ENABLE: u32 = 1u32 << 0;
pub const IAA_DECOMP_FLUSH_OUTPUT: u32 = 1u32 << 1;
pub const IAA_DECOMP_CHECK_FOR_EOB: u32 = 1u32 << 2;
pub const IAA_DECOMP_STOP_ON_EOB: u32 = 1u32 << 3;
pub const IAA_DECOMP_SUPPRESS_OUTPUT: u32 = 1u32 << 9;

pub const IAA_COMP_FLUSH_OUTPUT: u32 = 1u32 << 1;
pub const IAA_COMP_APPEND_EOB: u32 = 1u32 << 2;

pub const IAA_COMPLETION_TIMEOUT: i32 = 1000000;

pub const IAA_ANALYTICS_ERROR: u32 = 0x0a;
pub const IAA_ERROR_DECOMP_BUF_OVERFLOW: u32 = 0x0b;
pub const IAA_ERROR_COMP_BUF_OVERFLOW: u32 = 0x19;
pub const IAA_ERROR_WATCHDOG_EXPIRED: u32 = 0x24;

pub const IAA_COMP_MODES_MAX: usize = 2;

pub const FIXED_HDR: u32 = 0x2;
pub const FIXED_HDR_SIZE: usize = 3;

pub const IAA_COMP_FLAGS: u32 = IAA_COMP_FLUSH_OUTPUT | IAA_COMP_APPEND_EOB;
pub const IAA_DECOMP_FLAGS: u32 = IAA_DECOMP_ENABLE
    | IAA_DECOMP_FLUSH_OUTPUT
    | IAA_DECOMP_CHECK_FOR_EOB
    | IAA_DECOMP_STOP_ON_EOB;

/* Representation of IAA workqueue */
#[repr(C)]
pub struct iaa_wq {
    pub list: list_head,
    pub wq: *mut idxd_wq,
    pub ref_: i32,
    pub remove: bool,
    pub iaa_device: *mut iaa_device,
    pub comp_calls: atomic64_t,
    pub comp_bytes: atomic64_t,
    pub decomp_calls: atomic64_t,
    pub decomp_bytes: atomic64_t,
}

#[repr(C)]
pub struct iaa_device_compression_mode {
    pub name: *const core::ffi::c_char,
    pub aecs_comp_table: *mut aecs_comp_table_record,
    pub aecs_comp_table_dma_addr: dma_addr_t,
}

/* Representation of IAA device with wqs, populated by probe */
#[repr(C)]
pub struct iaa_device {
    pub list: list_head,
    pub idxd: *mut idxd_device,
    pub compression_modes: [*mut iaa_device_compression_mode; IAA_COMP_MODES_MAX],
    pub n_wq: i32,
    pub wqs: list_head,
    pub comp_calls: atomic64_t,
    pub comp_bytes: atomic64_t,
    pub decomp_calls: atomic64_t,
    pub decomp_bytes: atomic64_t,
}

#[repr(C)]
pub struct wq_table_entry {
    pub wqs: *mut *mut idxd_wq,
    pub max_wqs: i32,
    pub n_wqs: i32,
    pub cur_wq: i32,
}

pub const IAA_AECS_ALIGN: usize = 32;

/*
 * Analytics Engine Configuration and State (AECS) contains parameters and
 * internal state of the analytics engine.
 */
#[repr(C, packed)]
pub struct aecs_comp_table_record {
    pub crc: u32,
    pub xor_checksum: u32,
    pub reserved0: [u32; 5],
    pub num_output_accum_bits: u32,
    pub output_accum: [u8; 256],
    pub ll_sym: [u32; 286],
    pub reserved1: u32,
    pub reserved2: u32,
    pub d_sym: [u32; 30],
    pub reserved_padding: [u32; 2],
}

extern "C" {
    pub fn iaa_aecs_init_fixed() -> i32;
    pub fn iaa_aecs_cleanup_fixed();
}

pub type iaa_dev_comp_init_fn_t = Option<unsafe extern "C" fn(mode: *mut iaa_device_compression_mode) -> i32>;
pub type iaa_dev_comp_free_fn_t = Option<unsafe extern "C" fn(mode: *mut iaa_device_compression_mode) -> i32>;

#[repr(C)]
pub struct iaa_compression_mode {
    pub name: *const core::ffi::c_char,
    pub ll_table: *mut u32,
    pub ll_table_size: i32,
    pub d_table: *mut u32,
    pub d_table_size: i32,
    pub init: iaa_dev_comp_init_fn_t,
    pub free: iaa_dev_comp_free_fn_t,
}

extern "C" {
    pub fn add_iaa_compression_mode(
        name: *const core::ffi::c_char,
        ll_table: *const u32,
        ll_table_size: i32,
        d_table: *const u32,
        d_table_size: i32,
        init: iaa_dev_comp_init_fn_t,
        free: iaa_dev_comp_free_fn_t,
    ) -> i32;

    pub fn remove_iaa_compression_mode(name: *const core::ffi::c_char);
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum iaa_mode {
    IAA_MODE_FIXED,
}

#[repr(C)]
pub struct iaa_compression_ctx {
    pub mode: iaa_mode,
    pub verify_compress: bool,
    pub async_mode: bool,
    pub use_irq: bool,
}

extern "C" {
    pub static mut iaa_devices: list_head;
    pub static mut iaa_devices_lock: mutex;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
