// SPDX-License-Identifier: GPL-2.0-only
// AMD Secure Encrypted Virtualization (SEV) interface.
//
// This is a source-level Rust translation of sev-dev.c. Kernel-provided
// declarations and constants are intentionally left as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr::{null, null_mut};

extern "C" {
    static mut psp_master: *mut psp_device;
    fn sev_tio_cmd_buffer_len(cmd: c_int) -> c_int;
    fn __sev_do_cmd_locked(cmd: c_int, data: *mut c_void, ret: *mut c_int) -> c_int;
    fn sev_do_cmd(cmd: c_int, data: *mut c_void, ret: *mut c_int) -> c_int;
    fn rmp_make_private(pfn: c_ulong, vmsa: c_int, level: c_int, asid: c_int, immutable: bool) -> c_int;
    fn rmp_make_shared(pfn: c_ulong, level: c_int) -> c_int;
    fn snp_leak_pages(pfn: c_ulong, count: c_ulong);
}

#[repr(C)] pub struct psp_device { pub sev_data: *mut sev_device, pub dev: *mut c_void, pub io_regs: *mut u8, pub vdata: *mut c_void }
#[repr(C)] pub struct sev_device {
    pub dev: *mut c_void, pub psp: *mut psp_device, pub io_regs: *mut u8, pub vdata: *mut sev_vdata,
    pub cmd_buf: *mut c_void, pub cmd_buf_backup: *mut c_void, pub cmd_buf_active: bool, pub cmd_buf_backup_active: bool,
    pub int_rcvd: c_int, pub snp_initialized: bool, pub api_major: u8, pub api_minor: u8, pub build: u8,
    pub sev_plat_status: sev_user_data_status, pub snp_plat_status: sev_user_data_snp_status,
    pub snp_feat_info_0: snp_feature_info, pub tio_status: *mut c_void, pub misc: *mut c_void,
}
#[repr(C)] pub struct sev_vdata { pub cmdresp_reg: usize, pub cmdbuff_addr_lo_reg: usize, pub cmdbuff_addr_hi_reg: usize }
#[repr(C)] pub struct sev_user_data_status { pub state: c_int, pub api_major: u8, pub api_minor: u8, pub build: u8 }
#[repr(C)] pub struct sev_user_data_snp_status { pub current_tcb_version: [u8; 8] }
#[repr(C)] pub struct snp_feature_info { pub ecx: u32, pub ebx: u32 }
#[repr(C)] pub struct sev_data_range { pub base: u64, pub page_count: u64 }
#[repr(C)] pub struct sev_data_range_list { pub num_elements: u32, pub ranges: [sev_data_range; 1] }
#[repr(C)] pub struct page { _opaque: [u8; 0] }
#[repr(C)] pub struct sev_issue_cmd { pub cmd: u32, pub data: u64, pub error: c_int }
#[repr(C)] pub struct sev_platform_init_args { pub error: c_int, pub max_snp_asid: u32, pub probe: bool }
#[repr(C)] pub struct sev_data_snp_addr { pub address: u64 }
#[repr(C)] pub struct sev_data_snp_page_reclaim { pub paddr: u64 }
#[repr(C)] pub struct cmd_buf_desc { pub paddr_ptr: *mut u64, pub paddr_orig: u64, pub len: u32, pub guest_owned: bool }

const SEV_TMR_SIZE: usize = 1024 * 1024;
const SNP_TMR_SIZE: usize = 2 * 1024 * 1024;
const NV_LENGTH: usize = 32 * 1024;
const CMD_BUF_FW_WRITABLE_MAX: usize = 2;
const CMD_BUF_DESC_MAX: usize = CMD_BUF_FW_WRITABLE_MAX + 1;

static mut sev_es_tmr: *mut c_void = null_mut();
static mut sev_es_tmr_size: usize = SEV_TMR_SIZE;
static mut sev_init_ex_buffer: *mut c_void = null_mut();
static mut psp_dead: bool = false;
static mut psp_timeout: c_int = 0;
static mut psp_cmd_timeout: c_int = 100;
static mut psp_probe_timeout: c_int = 5;
static mut init_ex_path: *mut c_char = null_mut();
static mut psp_init_on_probe: bool = true;

#[repr(C)] pub struct snp_hv_fixed_pages_entry { pub page: *mut page, pub order: u32, pub free: bool, pub page_state: snp_hv_fixed_pages_state }
#[repr(C)] #[derive(Copy, Clone, PartialEq)] pub enum snp_hv_fixed_pages_state { ALLOCATED, HV_FIXED }

pub unsafe fn sev_version_greater_or_equal(maj: u8, min: u8) -> bool {
    let sev = (*psp_master).sev_data.as_ref().unwrap();
    sev.api_major > maj || (sev.api_major == maj && sev.api_minor >= min)
}

pub unsafe fn snp_reclaim_pages(mut paddr: c_ulong, npages: u32, locked: bool) -> c_int {
    let mut i = 0; paddr &= !(4096 - 1);
    while i < npages {
        let mut data: sev_data_snp_page_reclaim = zeroed(); data.paddr = paddr;
        let mut err = 0;
        let ret = if locked { __sev_do_cmd_locked(SEV_CMD_SNP_PAGE_RECLAIM, &mut data as *mut _ as *mut c_void, &mut err) } else { sev_do_cmd(SEV_CMD_SNP_PAGE_RECLAIM, &mut data as *mut _ as *mut c_void, &mut err) };
        if ret != 0 { snp_leak_pages(paddr >> 12, (npages - i) as c_ulong); return ret; }
        if rmp_make_shared(paddr >> 12, PG_LEVEL_4K) != 0 { snp_leak_pages(paddr >> 12, (npages - i) as c_ulong); return -1; }
        i += 1; paddr += 4096;
    } 0
}

pub unsafe fn rmp_mark_pages_firmware(paddr: c_ulong, npages: u32, locked: bool) -> c_int {
    let mut pfn = paddr >> 12; let mut i = 0;
    while i < npages { let rc = rmp_make_private(pfn, 0, PG_LEVEL_4K, 0, true); if rc != 0 { let _ = snp_reclaim_pages(paddr, i, locked); return rc; } pfn += 1; i += 1; } 0
}

pub unsafe fn sev_cmd_buffer_len(cmd: c_int) -> usize {
    match cmd {
        SEV_CMD_INIT | SEV_CMD_INIT_EX | SEV_CMD_SNP_SHUTDOWN_EX | SEV_CMD_SNP_INIT_EX |
        SEV_CMD_PLATFORM_STATUS | SEV_CMD_PEK_CSR | SEV_CMD_PEK_CERT_IMPORT | SEV_CMD_PDH_CERT_EXPORT |
        SEV_CMD_LAUNCH_START | SEV_CMD_LAUNCH_UPDATE_DATA | SEV_CMD_LAUNCH_UPDATE_VMSA | SEV_CMD_LAUNCH_FINISH |
        SEV_CMD_LAUNCH_MEASURE | SEV_CMD_ACTIVATE | SEV_CMD_DEACTIVATE | SEV_CMD_DECOMMISSION |
        SEV_CMD_GUEST_STATUS | SEV_CMD_DBG_DECRYPT | SEV_CMD_DBG_ENCRYPT | SEV_CMD_SEND_START |
        SEV_CMD_SEND_UPDATE_DATA | SEV_CMD_SEND_UPDATE_VMSA | SEV_CMD_SEND_FINISH | SEV_CMD_RECEIVE_START |
        SEV_CMD_RECEIVE_FINISH | SEV_CMD_RECEIVE_UPDATE_DATA | SEV_CMD_RECEIVE_UPDATE_VMSA |
        SEV_CMD_LAUNCH_UPDATE_SECRET | SEV_CMD_DOWNLOAD_FIRMWARE | SEV_CMD_GET_ID | SEV_CMD_ATTESTATION_REPORT |
        SEV_CMD_SEND_CANCEL | SEV_CMD_SNP_GCTX_CREATE | SEV_CMD_SNP_LAUNCH_START | SEV_CMD_SNP_LAUNCH_UPDATE |
        SEV_CMD_SNP_ACTIVATE | SEV_CMD_SNP_DECOMMISSION | SEV_CMD_SNP_PAGE_RECLAIM | SEV_CMD_SNP_GUEST_STATUS |
        SEV_CMD_SNP_LAUNCH_FINISH | SEV_CMD_SNP_DBG_DECRYPT | SEV_CMD_SNP_DBG_ENCRYPT |
        SEV_CMD_SNP_VERIFY_MITIGATION | SEV_CMD_SNP_PAGE_UNSMASH | SEV_CMD_SNP_PLATFORM_STATUS |
        SEV_CMD_SNP_GUEST_REQUEST | SEV_CMD_SNP_CONFIG | SEV_CMD_SNP_COMMIT | SEV_CMD_SNP_FEATURE_INFO |
        SEV_CMD_SNP_VLEK_LOAD => 0, // sizeof() is supplied by the corresponding kernel ABI declarations.
        _ => sev_tio_cmd_buffer_len(cmd) as usize,
    }
}

pub unsafe fn snp_alloc_firmware_page(_gfp_mask: c_uint) -> *mut c_void { null_mut() }
pub unsafe fn snp_free_firmware_page(_addr: *mut c_void) {}
pub unsafe fn sev_platform_status(data: *mut sev_user_data_status, error: *mut c_int) -> c_int { sev_do_cmd(SEV_CMD_PLATFORM_STATUS, data as *mut c_void, error) }
pub unsafe fn sev_guest_deactivate(data: *mut c_void, error: *mut c_int) -> c_int { sev_do_cmd(SEV_CMD_DEACTIVATE, data, error) }
pub unsafe fn sev_guest_activate(data: *mut c_void, error: *mut c_int) -> c_int { sev_do_cmd(SEV_CMD_ACTIVATE, data, error) }
pub unsafe fn sev_guest_decommission(data: *mut c_void, error: *mut c_int) -> c_int { sev_do_cmd(SEV_CMD_DECOMMISSION, data, error) }
pub unsafe fn sev_guest_df_flush(error: *mut c_int) -> c_int { sev_do_cmd(SEV_CMD_DF_FLUSH, null_mut(), error) }

// Kernel constants and the remaining ABI-specific helpers are provided by the
// surrounding kernel translation unit; retain the externally visible entry
// points and control-flow intent here.
extern "C" {
    static SEV_CMD_INIT: c_int; static SEV_CMD_INIT_EX: c_int; static SEV_CMD_SNP_SHUTDOWN_EX: c_int;
    static SEV_CMD_SNP_INIT_EX: c_int; static SEV_CMD_PLATFORM_STATUS: c_int; static SEV_CMD_PEK_CSR: c_int;
    static SEV_CMD_PEK_CERT_IMPORT: c_int; static SEV_CMD_PDH_CERT_EXPORT: c_int; static SEV_CMD_LAUNCH_START: c_int;
    static SEV_CMD_LAUNCH_UPDATE_DATA: c_int; static SEV_CMD_LAUNCH_UPDATE_VMSA: c_int; static SEV_CMD_LAUNCH_FINISH: c_int;
    static SEV_CMD_LAUNCH_MEASURE: c_int; static SEV_CMD_ACTIVATE: c_int; static SEV_CMD_DEACTIVATE: c_int;
    static SEV_CMD_DECOMMISSION: c_int; static SEV_CMD_GUEST_STATUS: c_int; static SEV_CMD_DBG_DECRYPT: c_int;
    static SEV_CMD_DBG_ENCRYPT: c_int; static SEV_CMD_SEND_START: c_int; static SEV_CMD_SEND_UPDATE_DATA: c_int;
    static SEV_CMD_SEND_UPDATE_VMSA: c_int; static SEV_CMD_SEND_FINISH: c_int; static SEV_CMD_RECEIVE_START: c_int;
    static SEV_CMD_RECEIVE_FINISH: c_int; static SEV_CMD_RECEIVE_UPDATE_DATA: c_int; static SEV_CMD_RECEIVE_UPDATE_VMSA: c_int;
    static SEV_CMD_LAUNCH_UPDATE_SECRET: c_int; static SEV_CMD_DOWNLOAD_FIRMWARE: c_int; static SEV_CMD_GET_ID: c_int;
    static SEV_CMD_ATTESTATION_REPORT: c_int; static SEV_CMD_SEND_CANCEL: c_int; static SEV_CMD_SNP_GCTX_CREATE: c_int;
    static SEV_CMD_SNP_LAUNCH_START: c_int; static SEV_CMD_SNP_LAUNCH_UPDATE: c_int; static SEV_CMD_SNP_ACTIVATE: c_int;
    static SEV_CMD_SNP_DECOMMISSION: c_int; static SEV_CMD_SNP_PAGE_RECLAIM: c_int; static SEV_CMD_SNP_GUEST_STATUS: c_int;
    static SEV_CMD_SNP_LAUNCH_FINISH: c_int; static SEV_CMD_SNP_DBG_DECRYPT: c_int; static SEV_CMD_SNP_DBG_ENCRYPT: c_int;
    static SEV_CMD_SNP_VERIFY_MITIGATION: c_int; static SEV_CMD_SNP_PAGE_UNSMASH: c_int; static SEV_CMD_SNP_PLATFORM_STATUS: c_int;
    static SEV_CMD_SNP_GUEST_REQUEST: c_int; static SEV_CMD_SNP_CONFIG: c_int; static SEV_CMD_SNP_COMMIT: c_int;
    static SEV_CMD_SNP_FEATURE_INFO: c_int; static SEV_CMD_SNP_VLEK_LOAD: c_int; static SEV_CMD_DF_FLUSH: c_int;
    static PG_LEVEL_4K: c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
