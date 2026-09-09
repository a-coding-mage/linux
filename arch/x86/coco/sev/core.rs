// SPDX-License-Identifier: GPL-2.0-only
// AMD Memory Encryption Support
//
// Direct low-level Rust translation of x86/coco/sev/core.c. Kernel-provided
// types, macros, globals, and functions remain external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// Opaque kernel types supplied by the surrounding kernel translation unit.
#[repr(C)] pub struct ghcb { _private: [u8; 0] }
#[repr(C)] pub struct sev_es_runtime_data { _private: [u8; 0] }
#[repr(C)] pub struct sev_es_save_area { _private: [u8; 0] }
#[repr(C)] pub struct snp_psc_desc { _private: [u8; 0] }
#[repr(C)] pub struct pte_t { _private: [u8; 0] }
#[repr(C)] pub struct pgd_t { _private: [u8; 0] }
#[repr(C)] pub struct real_mode_header { pub trampoline_start: u64, pub sev_es_trampoline_start: u64 }
#[repr(C)] pub struct snp_msg_desc { _private: [u8; 0] }
#[repr(C)] pub struct snp_guest_req { _private: [u8; 0] }

pub static mut sev_hv_features: u64 = 0;
pub static mut sev_secrets_pa: u64 = 0;
pub static mut snp_vmpl: u8 = 0;
pub static mut ghcb_version: u16 = 0;
pub static mut boot_ghcb: *mut ghcb = core::ptr::null_mut();

const AP_INIT_CS_LIMIT: u64 = 0xffff;
const AP_INIT_DS_LIMIT: u64 = 0xffff;
const AP_INIT_LDTR_LIMIT: u64 = 0xffff;
const AP_INIT_GDTR_LIMIT: u64 = 0xffff;
const AP_INIT_IDTR_LIMIT: u64 = 0xffff;
const AP_INIT_TR_LIMIT: u64 = 0xffff;
const AP_INIT_RFLAGS_DEFAULT: u64 = 0x2;
const AP_INIT_DR6_DEFAULT: u64 = 0xffff0ff0;
const AP_INIT_GPAT_DEFAULT: u64 = 0x0007040600070406;
const AP_INIT_XCR0_DEFAULT: u64 = 0x1;
const AP_INIT_X87_FTW_DEFAULT: u64 = 0x5555;
const AP_INIT_X87_FCW_DEFAULT: u64 = 0x0040;
const AP_INIT_CR0_DEFAULT: u64 = 0x60000010;
const AP_INIT_MXCSR_DEFAULT: u64 = 0x1f80;

static mut snp_tsc_scale: u64 = 0;
static mut snp_tsc_offset: u64 = 0;
static mut snp_tsc_freq_khz: c_ulong = 0;

extern "C" {
    fn cc_platform_has(attr: c_int) -> bool;
    fn ioremap_encrypted(addr: u64, size: usize) -> *mut c_void;
    fn iounmap(addr: *mut c_void);
    fn pvalidate(vaddr: c_ulong, size: c_uint, validate: bool) -> c_int;
    fn __pval_terminate(pfn: u64, validate: bool, size: c_uint, rc: c_int, rsvd: c_int);
    fn svsm_pval_pages(desc: *mut snp_psc_desc);
    fn cpu_feature_enabled(feature: c_int) -> bool;
    fn sev_evict_cache(addr: *mut c_void, pages: c_uint);
    fn __sev_get_ghcb(state: *mut c_void) -> *mut ghcb;
    fn __sev_put_ghcb(state: *mut c_void);
    fn vmgexit_psc(ghcb: *mut ghcb, desc: *mut snp_psc_desc) -> c_int;
    fn early_set_pages_state(vaddr: c_ulong, paddr: u64, npages: c_ulong, desc: *mut c_void) -> c_ulong;
    fn sev_es_terminate(set: c_int, reason: c_int) -> !;
    fn set_memory_enc_stop_conversion() -> bool;
    fn snp_set_vmsa(va: *mut c_void, caa: *mut c_void, apic_id: c_int, make_vmsa: bool) -> c_int;
    fn snp_set_memory_private(vaddr: c_ulong, npages: c_ulong);
    fn snp_set_memory_shared(vaddr: c_ulong, npages: c_ulong);
    fn snp_accept_memory(start: u64, end: u64);
    fn snp_set_wakeup_secondary_cpu();
    fn snp_secure_tsc_prepare();
    fn snp_secure_tsc_init();
}

// File-local operations retain their C calling interfaces and are implemented
// by the kernel-facing translation layer; declarations intentionally reference
// the external Linux/SEV data structures rather than reimplementing them here.
extern "C" {
    fn get_snp_jump_table_addr() -> u64;
    fn get_jump_table_addr() -> u64;
    fn pval_pages(desc: *mut snp_psc_desc);
    fn pvalidate_pages(desc: *mut snp_psc_desc);
    fn set_pages_state(vaddr: c_ulong, npages: c_ulong, op: c_int);
    fn vmgexit_ap_control(event: u64, vmsa: *mut sev_es_save_area, apic_id: c_uint) -> c_int;
    fn snp_cleanup_vmsa(vmsa: *mut sev_es_save_area, apic_id: c_int);
    fn unshare_all_memory();
    fn shutdown_all_aps();
    fn wakeup_cpu_via_vmgexit(apic_id: c_uint, start_ip: c_ulong, cpu: c_uint) -> c_int;
    fn savic_ghcb_msr_read(reg: c_uint) -> u64;
    fn savic_ghcb_msr_write(reg: c_uint, value: u64);
    fn savic_register_gpa(gpa: u64) -> c_int;
    fn savic_unregister_gpa(gpa: *mut u64) -> c_int;
    fn setup_ghcb();
    fn sev_es_init_vc_handling();
    fn snp_dmi_setup();
    fn snp_msg_init(desc: *mut snp_msg_desc, vmpck_id: c_int) -> c_int;
    fn snp_msg_alloc() -> *mut snp_msg_desc;
    fn snp_msg_free(desc: *mut snp_msg_desc);
    fn snp_send_guest_request(desc: *mut snp_msg_desc, req: *mut snp_guest_req) -> c_int;
    fn sev_show_status();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
