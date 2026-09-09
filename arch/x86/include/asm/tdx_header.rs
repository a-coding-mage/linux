/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2021-2022 Intel Corporation */

// Translated from the C header. Required types and symbols are supplied by
// the surrounding kernel translation.

pub const TDX_ERROR: u64 = 1u64 << 63;
pub const TDX_NON_RECOVERABLE: u64 = 1u64 << 62;
pub const TDX_SW_ERROR: u64 = TDX_ERROR | (((1u64 << 8) - 1) << 40);
pub const TDX_SEAMCALL_VMFAILINVALID: u64 = TDX_SW_ERROR | 0xFFFF0000u64;

pub const TDX_SEAMCALL_GP: u64 = TDX_SW_ERROR | X86_TRAP_GP as u64;
pub const TDX_SEAMCALL_UD: u64 = TDX_SW_ERROR | X86_TRAP_UD as u64;

pub const TDX_SUCCESS: u64 = 0u64;
pub const TDX_RND_NO_ENTROPY: u64 = 0x8000020300000000u64;

pub const TDX_FEATURES0_TD_PRESERVING: u64 = 1u64 << 1;
pub const TDX_FEATURES0_NO_RBP_MOD: u64 = 1u64 << 18;

pub const TDX_VERSION_FMT: &str = "%u.%u.%02u";

#[repr(C)]
pub struct ve_info {
    pub exit_reason: u64,
    pub exit_qual: u64,
    /* Guest Linear (virtual) Address */
    pub gla: u64,
    /* Guest Physical Address */
    pub gpa: u64,
    pub instr_len: u32,
    pub instr_info: u32,
}

// CONFIG_INTEL_TDX_GUEST
extern "C" {
    pub fn tdx_early_init();
    pub fn tdx_get_ve_info(ve: *mut ve_info);
    pub fn tdx_handle_virt_exception(regs: *mut pt_regs, ve: *mut ve_info) -> bool;
    pub fn tdx_halt();
    pub fn tdx_early_handle_ve(regs: *mut pt_regs) -> bool;
    pub fn tdx_mcall_get_report0(reportdata: *mut u8, tdreport: *mut u8) -> i32;
    pub fn tdx_mcall_extend_rtmr(index: u8, data: *mut u8) -> i32;
    pub fn tdx_hcall_get_quote(buf: *mut u8, size: usize) -> u64;
    pub fn tdx_dump_attributes(td_attr: u64);
    pub fn tdx_dump_td_ctls(td_ctls: u64);
}

// When CONFIG_INTEL_TDX_GUEST is disabled, the C header provides empty/falsy
// inline definitions for the guest initialization, halt, and #VE entrypoint.

// CONFIG_KVM_GUEST && CONFIG_INTEL_TDX_GUEST
extern "C" {
    pub fn tdx_kvm_hypercall(
        nr: core::ffi::c_uint,
        p1: core::ffi::c_ulong,
        p2: core::ffi::c_ulong,
        p3: core::ffi::c_ulong,
        p4: core::ffi::c_ulong,
    ) -> core::ffi::c_long;
}

// CONFIG_INTEL_TDX_HOST
extern "C" {
    pub fn tdx_init();
    pub fn tdx_cpu_enable() -> i32;
    pub fn tdx_dump_mce_info(m: *mut mce) -> *const core::ffi::c_char;
    pub fn tdx_get_sysinfo() -> *const tdx_sys_info;
    pub fn tdx_guest_keyid_alloc() -> i32;
    pub fn tdx_get_nr_guest_keyids() -> u32;
    pub fn tdx_guest_keyid_free(keyid: core::ffi::c_uint);
    pub fn tdx_quirk_reset_paddr(base: core::ffi::c_ulong, size: core::ffi::c_ulong);
}

#[inline]
pub unsafe fn tdx_supports_runtime_update(sysinfo: *const tdx_sys_info) -> bool {
    (*sysinfo).features.tdx_features0 & TDX_FEATURES0_TD_PRESERVING != 0
}

#[repr(C)]
pub struct tdx_td {
    /* TD root structure: */
    pub tdr_page: *mut page,
    pub tdcs_nr_pages: i32,
    /* TD control structure: */
    pub tdcs_pages: *mut *mut page,
    /* Size of `tdcx_pages` in struct tdx_vp */
    pub tdcx_nr_pages: i32,
}

#[repr(C)]
pub struct tdx_vp {
    /* TDVP root page */
    pub tdvpr_page: *mut page,
    /* precalculated page_to_phys(tdvpr_page) for use in noinstr code */
    pub tdvpr_pa: phys_addr_t,
    /* TD vCPU control structure: */
    pub tdcx_pages: *mut *mut page,
}

extern "C" {
    pub fn tdx_sys_disable();
    pub fn tdh_vp_enter(vp: *mut tdx_vp, args: *mut tdx_module_args) -> u64;
    pub fn tdh_mng_addcx(td: *mut tdx_td, tdcs_page: *mut page) -> u64;
    pub fn tdh_mem_page_add(td: *mut tdx_td, gpa: u64, pfn: kvm_pfn_t, source: *mut page, ext_err1: *mut u64, ext_err2: *mut u64) -> u64;
    pub fn tdh_mem_sept_add(td: *mut tdx_td, gpa: u64, level: pg_level, page: *mut page, ext_err1: *mut u64, ext_err2: *mut u64) -> u64;
    pub fn tdh_vp_addcx(vp: *mut tdx_vp, tdcx_page: *mut page) -> u64;
    pub fn tdh_mem_page_aug(td: *mut tdx_td, gpa: u64, level: pg_level, pfn: kvm_pfn_t, ext_err1: *mut u64, ext_err2: *mut u64) -> u64;
    pub fn tdh_mem_range_block(td: *mut tdx_td, gpa: u64, level: pg_level, ext_err1: *mut u64, ext_err2: *mut u64) -> u64;
    pub fn tdh_mng_key_config(td: *mut tdx_td) -> u64;
    pub fn tdh_mng_create(td: *mut tdx_td, hkid: u16) -> u64;
    pub fn tdh_vp_create(td: *mut tdx_td, vp: *mut tdx_vp) -> u64;
    pub fn tdh_mng_rd(td: *mut tdx_td, field: u64, data: *mut u64) -> u64;
    pub fn tdh_mr_extend(td: *mut tdx_td, gpa: u64, ext_err1: *mut u64, ext_err2: *mut u64) -> u64;
    pub fn tdh_mr_finalize(td: *mut tdx_td) -> u64;
    pub fn tdh_vp_flush(vp: *mut tdx_vp) -> u64;
    pub fn tdh_mng_vpflushdone(td: *mut tdx_td) -> u64;
    pub fn tdh_mng_key_freeid(td: *mut tdx_td) -> u64;
    pub fn tdh_mng_init(td: *mut tdx_td, td_params: u64, extended_err: *mut u64) -> u64;
    pub fn tdh_vp_init(vp: *mut tdx_vp, initial_rcx: u64, x2apicid: u32) -> u64;
    pub fn tdh_vp_rd(vp: *mut tdx_vp, field: u64, data: *mut u64) -> u64;
    pub fn tdh_vp_wr(vp: *mut tdx_vp, field: u64, data: u64, mask: u64) -> u64;
    pub fn tdh_phymem_page_reclaim(page: *mut page, tdx_pt: *mut u64, tdx_owner: *mut u64, tdx_size: *mut u64) -> u64;
    pub fn tdh_mem_track(tdr: *mut tdx_td) -> u64;
    pub fn tdh_mem_page_remove(td: *mut tdx_td, gpa: u64, level: pg_level, ext_err1: *mut u64, ext_err2: *mut u64) -> u64;
    pub fn tdh_phymem_cache_wb(resume: bool) -> u64;
    pub fn tdh_phymem_page_wbinvd_tdr(td: *mut tdx_td) -> u64;
    pub fn tdh_phymem_page_wbinvd_hkid(hkid: u64, pfn: kvm_pfn_t) -> u64;
}

// External declarations supplied by included headers.
extern "C" {
    static X86_TRAP_GP: i32;
    static X86_TRAP_UD: i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
