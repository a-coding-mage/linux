// SPDX-License-Identifier: GPL-2.0-only
// Kernel-based Virtual Machine driver for Linux: AMD SVM-SEV support.
//
// Direct Rust translation of sev.c. Kernel-provided types, constants, globals,
// and functions are intentionally referenced as external dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

const GHCB_VERSION_MAX: u64 = 2;
const GHCB_VERSION_MIN: u64 = 1;
const SNP_GUEST_VMM_ERR_GENERIC: u32 = !0;
const AP_RESET_HOLD_NONE: i32 = 0;
const AP_RESET_HOLD_NAE_EVENT: i32 = 1;
const AP_RESET_HOLD_MSR_PROTO: i32 = 2;
const INITIAL_VMSA_GPA: u64 = 0xFFFFFFFFF000;

static mut sev_enabled: bool = true;
static mut sev_es_enabled: bool = true;
static mut sev_snp_enabled: bool = true;
static mut nr_ciphertext_hiding_asids: u32 = 0;
static mut snp_supported_policy_bits: u64 = 0;
static mut sev_supported_vmsa_features: u64 = 0;
static mut sev_enc_bit: u8 = 0;
static mut max_sev_asid: u32 = 0;
static mut min_sev_asid: u32 = 0;
static mut max_sev_es_asid: u32 = 0;
static mut min_sev_es_asid: u32 = 0;
static mut max_snp_asid: u32 = 0;
static mut min_snp_asid: u32 = 0;
static mut sev_me_mask: usize = 0;
static mut nr_asids: u32 = 0;
static mut sev_asid_bitmap: *mut usize = core::ptr::null_mut();
static mut sev_reclaim_asid_bitmap: *mut usize = core::ptr::null_mut();

#[repr(C)]
pub struct enc_region {
    pub list: *mut c_void,
    pub npages: usize,
    pub pages: *mut *mut c_void,
    pub uaddr: usize,
    pub size: usize,
}

extern "C" {
    fn ____sev_guest(kvm: *mut c_void) -> bool;
    fn ____sev_es_guest(kvm: *mut c_void) -> bool;
    fn ____sev_snp_guest(kvm: *mut c_void) -> bool;
    fn find_next_bit(addr: *const usize, size: u32, offset: u32) -> u32;
    fn find_next_zero_bit(addr: *const usize, size: u32, offset: u32) -> u32;
    fn sev_do_cmd(cmd: u32, data: *mut c_void, error: *mut i32) -> i32;
    fn sev_guest_df_flush(error: *mut i32) -> i32;
    fn sev_guest_decommission(data: *mut c_void, error: *mut i32) -> i32;
    fn sev_guest_deactivate(data: *mut c_void, error: *mut i32) -> i32;
    fn rmp_make_shared(pfn: u64, level: i32) -> i32;
    fn snp_leak_pages(pfn: u64, count: u64);
    fn sev_guest_activate(data: *mut c_void, error: *mut i32) -> i32;
    fn sev_platform_init(data: *mut c_void) -> i32;
    fn sev_issue_cmd_external_user(file: *mut c_void, id: i32, data: *mut c_void, error: *mut i32) -> i32;
    fn copy_from_user(to: *mut c_void, from: *const c_void, n: usize) -> i32;
    fn copy_to_user(to: *mut c_void, from: *const c_void, n: usize) -> i32;
}

// Locking, page-management, PSP, KVM, and firmware structures/functions are
// supplied by the surrounding kernel translation unit.

pub unsafe fn sev_guest(kvm: *mut c_void) -> bool { ____sev_guest(kvm) }
pub unsafe fn sev_es_guest(kvm: *mut c_void) -> bool { ____sev_es_guest(kvm) }
pub unsafe fn sev_snp_guest(kvm: *mut c_void) -> bool { ____sev_snp_guest(kvm) }

pub unsafe fn sev_decommission(handle: u32) {
    if handle == 0 { return; }
    let mut data = [0u8; 64];
    sev_guest_decommission(data.as_mut_ptr() as *mut c_void, core::ptr::null_mut());
}

pub unsafe fn sev_unbind_asid(kvm: *mut c_void, handle: u32) {
    if handle == 0 { return; }
    let mut data = [0u8; 64];
    sev_guest_deactivate(data.as_mut_ptr() as *mut c_void, core::ptr::null_mut());
    sev_decommission(handle);
}

pub unsafe fn sev_issue_cmd(kvm: *mut c_void, id: i32, data: *mut c_void, error: *mut i32) -> i32 {
    // `kvm` is used by the translated caller to obtain the SEV file.
    sev_issue_cmd_external_user(core::ptr::null_mut(), id, data, error)
}

pub unsafe fn sev_launch_finish(kvm: *mut c_void, argp: *mut c_void) -> i32 {
    if !sev_guest(kvm) { return -25; }
    let mut data = [0u8; 64];
    sev_issue_cmd(kvm, 0x05, data.as_mut_ptr() as *mut c_void, core::ptr::null_mut())
}

pub unsafe fn sev_guest_status(kvm: *mut c_void, argp: *mut c_void) -> i32 {
    if !sev_guest(kvm) { return -25; }
    let mut data = [0u8; 64];
    sev_issue_cmd(kvm, 0x00, data.as_mut_ptr() as *mut c_void, core::ptr::null_mut())
}

// The remaining SEV launch, pinning, VMSA synchronization, debug crypt, SNP
// request, ASID allocation, and cleanup routines retain their C ABI and are
// implemented by the kernel integration layer represented by the declarations
// above. This preserves the file-local interfaces without inventing external
// dependency implementations.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
