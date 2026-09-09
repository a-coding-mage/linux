// SPDX-License-Identifier: GPL-2.0
// Copyright IBM Corp. 2007

use core::ffi::{c_char, c_void};

pub const SCLP_CHP_INFO_MASK_SIZE: usize = 32;
pub const EARLY_SCCB_SIZE: usize = PAGE_SIZE;
pub const SCLP_MAX_CORES: usize = 512;
// 144 + 16 * SCLP_MAX_CORES + 2 * (SCLP_MAX_CORES - 1)
pub const EXT_SCCB_READ_SCP: usize = 3 * PAGE_SIZE;
// 24 + 16 * SCLP_MAX_CORES
pub const EXT_SCCB_READ_CPU: usize = 3 * PAGE_SIZE;

pub const SCLP_ERRNOTIFY_AQ_RESET: u32 = 0;
pub const SCLP_ERRNOTIFY_AQ_REPAIR: u32 = 1;
pub const SCLP_ERRNOTIFY_AQ_INFO_LOG: u32 = 2;
pub const SCLP_ERRNOTIFY_AQ_OPTICS_DATA: u32 = 3;
pub const SCLP_ERRNOTIFY_AQ_NVME_SMART_LOG: u32 = 4;
pub const SCLP_ERRNOTIFY_AQ_ADAPTER_INITIALIZED: u32 = 5;
pub const SCLP_ERRNOTIFY_AQ_RECOVERABLE_ERROR: u32 = 6;
pub const SCLP_ERRNOTIFY_AQ_TELEMETRY_DATA: u32 = 7;

#[repr(C)]
pub struct sclp_chp_info {
    pub recognized: [u8; SCLP_CHP_INFO_MASK_SIZE],
    pub standby: [u8; SCLP_CHP_INFO_MASK_SIZE],
    pub configured: [u8; SCLP_CHP_INFO_MASK_SIZE],
}

pub const LOADPARM_LEN: usize = 8;

#[repr(C)]
pub struct sclp_ipl_info {
    pub is_valid: core::ffi::c_int,
    pub has_dump: core::ffi::c_int,
    pub loadparm: [c_char; LOADPARM_LEN],
}

#[repr(C, packed)]
pub struct sclp_core_entry {
    pub core_id: u8,
    pub reserved0: u8,
    // C bitfields: unnamed 4 bits, sief2, skey, unnamed 2 bits, unnamed 2 bits,
    // gpere, siif, sigpif, unnamed 3 bits.
    pub bitfields0: u8,
    pub bitfields1: u8,
    pub reserved2: [u8; 3],
    // C bitfields: unnamed 2 bits, ib, cei, unnamed 4 bits.
    pub bitfields2: u8,
    pub reserved3: [u8; 6],
    pub type_: u8,
    pub reserved1: u8,
}

#[repr(C)]
pub struct sclp_core_info {
    pub configured: u32,
    pub standby: u32,
    pub combined: u32,
    pub core: [sclp_core_entry; SCLP_MAX_CORES],
}

#[repr(C)]
pub struct sclp_info {
    // 35 one-bit unsigned-char bitfields, occupying five bytes in C.
    pub feature_bits: [u8; 5],
    pub ibc: u32,
    pub mtid: u32,
    pub mtid_cp: u32,
    pub mtid_prev: u32,
    pub rzm: usize,
    pub rnmax: usize,
    pub hamax: usize,
    pub max_cores: u32,
    pub hsa_size: usize,
    pub facilities: usize,
    pub hmfai: u32,
}

pub static mut sclp: sclp_info = unsafe { core::mem::zeroed() };

#[repr(C, packed)]
pub struct sccb_header {
    pub length: u16,
    pub function_code: u8,
    pub control_mask: [u8; 3],
    pub response_code: u16,
}

#[repr(C, packed)]
pub struct evbuf_header {
    pub length: u16,
    pub type_: u8,
    pub flags: u8,
    pub _reserved: u16,
}

#[repr(C, packed)]
pub struct err_notify_evbuf {
    pub header: evbuf_header,
    pub action: u8,
    pub atype: u8,
    pub fh: u32,
    pub fid: u32,
    pub data: [u8; 0],
}

#[repr(C, packed)]
pub struct err_notify_sccb {
    pub header: sccb_header,
    pub evbuf: err_notify_evbuf,
}

#[repr(C, packed)]
pub struct zpci_report_error_header {
    pub version: u8,
    pub action: u8,
    pub length: u16,
    pub data: [u8; 0],
}

extern "C" {
    pub static mut sclp_early_sccb: *mut c_char;

    pub fn sclp_early_adjust_va();
    pub fn sclp_early_set_buffer(sccb: *mut c_void);
    pub fn sclp_early_read_info() -> core::ffi::c_int;
    pub fn sclp_early_read_storage_info() -> core::ffi::c_int;
    pub fn sclp_early_get_core_info(info: *mut sclp_core_info) -> core::ffi::c_int;
    pub fn sclp_early_get_ipl_info(info: *mut sclp_ipl_info);
    pub fn sclp_early_detect();
    pub fn sclp_early_detect_machine_features();
    pub fn sclp_early_printk(s: *const c_char);
    pub fn __sclp_early_printk(s: *const c_char, len: u32);
    pub fn sclp_emergency_printk(s: *const c_char);

    pub fn sclp_init() -> core::ffi::c_int;
    pub fn sclp_early_get_memsize(mem: *mut usize) -> core::ffi::c_int;
    pub fn sclp_early_get_hsa_size(hsa_size: *mut usize) -> core::ffi::c_int;
    pub fn _sclp_get_core_info(info: *mut sclp_core_info) -> core::ffi::c_int;
    pub fn sclp_core_configure(core: u8) -> core::ffi::c_int;
    pub fn sclp_core_deconfigure(core: u8) -> core::ffi::c_int;
    pub fn sclp_sdias_blk_count() -> core::ffi::c_int;
    pub fn sclp_sdias_copy(dest: *mut c_void, blk_num: core::ffi::c_int, nr_blks: core::ffi::c_int) -> core::ffi::c_int;
    pub fn sclp_chp_configure(chpid: crate::chpid::chp_id) -> core::ffi::c_int;
    pub fn sclp_chp_deconfigure(chpid: crate::chpid::chp_id) -> core::ffi::c_int;
    pub fn sclp_chp_read_info(info: *mut sclp_chp_info) -> core::ffi::c_int;
    pub fn sclp_pci_configure(fid: u32) -> core::ffi::c_int;
    pub fn sclp_pci_deconfigure(fid: u32) -> core::ffi::c_int;
    pub fn sclp_ap_configure(apid: u32) -> core::ffi::c_int;
    pub fn sclp_ap_deconfigure(apid: u32) -> core::ffi::c_int;
    pub fn sclp_pci_report(report: *mut zpci_report_error_header, fh: u32, fid: u32) -> core::ffi::c_int;
    pub fn memcpy_hsa_iter(iter: *mut crate::uio::iov_iter, src: usize, count: usize) -> usize;
    pub fn sclp_ocf_cpc_name_copy(dst: *mut c_char);
}

#[inline]
pub unsafe fn sclp_get_core_info(info: *mut sclp_core_info, early: core::ffi::c_int) -> core::ffi::c_int {
    if early != 0 {
        sclp_early_get_core_info(info)
    } else {
        _sclp_get_core_info(info)
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
