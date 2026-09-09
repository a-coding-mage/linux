// SPDX-License-Identifier: GPL-2.0
/*
 * AMD Encrypted Register State Support
 *
 * Author: Joerg Roedel <jroedel@suse.de>
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct Ghcb {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PscDesc {
    pub page_state: u64,
    pub svsm_caa: *mut SvsmCa,
    pub svsm_caa_pa: u64,
}

#[repr(C)]
pub struct SvsmCa {
    _private: [u8; 0],
}

#[repr(C)]
pub struct BootParams {
    pub cc_blob_address: u32,
}

#[repr(C)]
pub struct CcBlobSevInfo {
    pub magic: u32,
    pub cpuid_phys: u64,
    pub cpuid_len: u64,
}

#[repr(C)]
pub struct Msr {
    pub q: u64,
}

extern "C" {
    static mut sev_status: u64;
    static mut sev_snp_needs_sfw: bool;
    static mut sme_me_mask: u64;
    static mut boot_params_ptr: *mut BootParams;

    fn set_page_decrypted(addr: usize) -> i32;
    fn set_page_encrypted(addr: usize) -> i32;
    fn set_page_non_present(addr: usize) -> i32;
    fn sev_insn_decode_init();
    fn sev_es_check_cpu_features() -> bool;
    fn sev_es_wr_ghcb_msr(addr: u64);
    fn vc_ghcb_invalidate(ghcb: *mut Ghcb);
    fn ghcb_set_sw_exit_code(ghcb: *mut Ghcb, code: u64);
    fn ghcb_set_sw_exit_info_1(ghcb: *mut Ghcb, value: u64);
    fn ghcb_set_sw_exit_info_2(ghcb: *mut Ghcb, value: u64);
    fn sev_es_terminate(set: u32, reason: u32) -> !;
    fn efi_get_conf_table(bp: *mut BootParams, pa: *mut usize, len: *mut u32) -> i32;
    fn efi_find_vendor_table(bp: *mut BootParams, pa: usize, len: u32, guid: u64) -> *mut CcBlobSevInfo;
    fn find_cc_blob_setup_data(bp: *mut BootParams) -> *mut CcBlobSevInfo;
    fn setup_cpuid_table(info: *mut CcBlobSevInfo);
    fn svsm_setup_ca(info: *mut CcBlobSevInfo, ghcb: *const Ghcb);
    fn rip_rel_ptr<T>(ptr: *const T) -> *const T;
    fn native_cpuid(eax: *mut u32, ebx: *mut u32, ecx: *mut u32, edx: *mut u32);
    fn sev_es_negotiate_protocol() -> bool;
    fn get_hv_features() -> u64;
    fn raw_rdmsr(msr: u32, value: *mut Msr);
    fn kernel_add_identity_map(start: u64, end: u64);
    fn sev_verify_cbit(top_level_pgt: usize);
    fn __page_state_change(start: u64, end: u64, desc: *mut PscDesc);
    fn error(message: *const u8) -> !;
}

static mut boot_ghcb_page: Ghcb = Ghcb { _private: [] };
pub static mut boot_ghcb: *mut Ghcb = core::ptr::null_mut();
pub static mut snp_vmpl: u8 = 0;
pub static mut ghcb_version: u16 = 0;
pub static mut boot_svsm_caa_pa: u64 = 0;

const PAGE_SIZE: u64 = 4096;
const PAGE_MASK: usize = !(PAGE_SIZE as usize - 1);
const SNP_PAGE_STATE_PRIVATE: u64 = 1;
const SNP_PAGE_STATE_SHARED: u64 = 2;
const MSR_AMD64_SEV_ENABLED: u64 = 1 << 0;
const MSR_AMD64_SEV_ES_ENABLED: u64 = 1 << 3;
const MSR_AMD64_SEV_SNP_ENABLED: u64 = 1 << 4;
const MSR_AMD64_SNP_DEBUG_SWAP: u64 = 1 << 6;
const MSR_AMD64_SNP_SECURE_TSC: u64 = 1 << 10;
const MSR_AMD64_SNP_SECURE_AVIC: u64 = 1 << 11;
const MSR_AMD64_SNP_VTOM: u64 = 1 << 3;
const MSR_AMD64_SNP_REFLECT_VC: u64 = 1 << 4;
const MSR_AMD64_SNP_RESTRICTED_INJ: u64 = 1 << 5;
const MSR_AMD64_SNP_ALT_INJ: u64 = 1 << 6;
const MSR_AMD64_SNP_VMPL_SSS: u64 = 1 << 7;
const MSR_AMD64_SNP_VMGEXIT_PARAM: u64 = 1 << 12;
const MSR_AMD64_SNP_VMSA_REG_PROT: u64 = 1 << 13;
const MSR_AMD64_SNP_RESERVED_BIT13: u64 = 1 << 13;
const MSR_AMD64_SNP_RESERVED_BIT15: u64 = 1 << 15;
const MSR_AMD64_SNP_RESERVED_BITS19_22: u64 = 0xF << 19;
const MSR_AMD64_SNP_RESERVED_MASK: u64 = 0;
const GHCB_HV_FT_SNP: u64 = 1 << 0;
const GHCB_HV_FT_SNP_MULTI_VMPL: u64 = 1 << 1;
const GHCB_SNP_UNSUPPORTED: u32 = 0;
const GHCB_SEV_ES_PROT_UNSUPPORTED: u32 = 0;
const GHCB_TERM_NOT_VMPL0: u32 = 0;
const SEV_TERM_SET_GEN: u32 = 0;
const SEV_TERM_SET_LINUX: u32 = 0;
const CC_BLOB_SEV_HDR_MAGIC: u32 = 0;
const MSR_AMD64_SEV: u32 = 0;
const MSR_SVSM_CAA: u32 = 0;

unsafe fn sev_snp_enabled() -> bool { sev_status & MSR_AMD64_SEV_SNP_ENABLED != 0 }

pub unsafe fn snp_set_page_private(paddr: u64) {
    let mut d = PscDesc { page_state: SNP_PAGE_STATE_PRIVATE, svsm_caa: boot_svsm_caa_pa as *mut SvsmCa, svsm_caa_pa: boot_svsm_caa_pa };
    if !sev_snp_enabled() { return; }
    __page_state_change(paddr, paddr, &mut d);
}

pub unsafe fn snp_set_page_shared(paddr: u64) {
    let mut d = PscDesc { page_state: SNP_PAGE_STATE_SHARED, svsm_caa: boot_svsm_caa_pa as *mut SvsmCa, svsm_caa_pa: boot_svsm_caa_pa };
    if !sev_snp_enabled() { return; }
    __page_state_change(paddr, paddr, &mut d);
}

pub unsafe fn early_setup_ghcb() -> bool {
    if set_page_decrypted((&raw mut boot_ghcb_page) as usize) != 0 { return false; }
    core::ptr::write_bytes((&raw mut boot_ghcb_page) as *mut u8, 0, core::mem::size_of::<Ghcb>());
    boot_ghcb = &raw mut boot_ghcb_page;
    sev_insn_decode_init();
    if sev_snp_enabled() { snp_register_ghcb_early((&raw const boot_ghcb_page) as usize as u64); }
    true
}

pub unsafe fn snp_accept_memory(start: u64, end: u64) {
    let mut d = PscDesc { page_state: SNP_PAGE_STATE_PRIVATE, svsm_caa: boot_svsm_caa_pa as *mut SvsmCa, svsm_caa_pa: boot_svsm_caa_pa };
    let mut pa = start;
    while pa < end { __page_state_change(pa, pa, &mut d); pa = pa.wrapping_add(PAGE_SIZE); }
}

pub unsafe fn sev_es_shutdown_ghcb() {
    if boot_ghcb.is_null() { return; }
    if !sev_es_check_cpu_features() { error(b"SEV-ES CPU Features missing.\0".as_ptr()); }
    boot_ghcb = core::ptr::null_mut();
    if set_page_encrypted((&raw mut boot_ghcb_page) as usize) != 0 { error(b"Can't map GHCB page encrypted\0".as_ptr()); }
    if set_page_non_present((&raw mut boot_ghcb_page) as usize) != 0 { error(b"Can't unmap GHCB page\0".as_ptr()); }
}

unsafe fn sev_es_ghcb_terminate(ghcb: *mut Ghcb, set: u32, reason: u32, exit_info_2: u64) -> ! {
    let exit_info_1 = ((set as u64) << 32) | reason as u64;
    vc_ghcb_invalidate(ghcb);
    ghcb_set_sw_exit_code(ghcb, 0);
    ghcb_set_sw_exit_info_1(ghcb, exit_info_1);
    ghcb_set_sw_exit_info_2(ghcb, exit_info_2);
    sev_es_wr_ghcb_msr(ghcb as u64);
    core::arch::asm!("vmgexit");
    loop { core::arch::asm!("hlt", options(nomem, nostack, preserves_flags)); }
}

pub unsafe fn sev_es_check_ghcb_fault(address: usize) -> bool {
    (address & PAGE_MASK) == (&raw const boot_ghcb_page) as usize
}

const SNP_FEATURES_IMPL_REQ: u64 = MSR_AMD64_SNP_VTOM | MSR_AMD64_SNP_REFLECT_VC | MSR_AMD64_SNP_RESTRICTED_INJ | MSR_AMD64_SNP_ALT_INJ | MSR_AMD64_SNP_DEBUG_SWAP | MSR_AMD64_SNP_VMPL_SSS | MSR_AMD64_SNP_SECURE_TSC | MSR_AMD64_SNP_VMGEXIT_PARAM | MSR_AMD64_SNP_VMSA_REG_PROT | MSR_AMD64_SNP_RESERVED_BIT13 | MSR_AMD64_SNP_RESERVED_BIT15 | MSR_AMD64_SNP_SECURE_AVIC | MSR_AMD64_SNP_RESERVED_BITS19_22 | MSR_AMD64_SNP_RESERVED_MASK;
const SNP_FEATURES_IMPL: u64 = MSR_AMD64_SNP_DEBUG_SWAP | MSR_AMD64_SNP_SECURE_TSC | MSR_AMD64_SNP_SECURE_AVIC;

pub unsafe fn snp_get_unsupported_features(status: u64) -> u64 { if status & MSR_AMD64_SEV_SNP_ENABLED == 0 { 0 } else { status & SNP_FEATURES_IMPL_REQ & !SNP_FEATURES_IMPL } }

pub unsafe fn snp_check_features() {
    let unsupported = snp_get_unsupported_features(sev_status);
    if unsupported != 0 {
        if ghcb_version < 2 || (boot_ghcb.is_null() && !early_setup_ghcb()) { sev_es_terminate(SEV_TERM_SET_GEN, GHCB_SNP_UNSUPPORTED); }
        sev_es_ghcb_terminate(boot_ghcb, SEV_TERM_SET_GEN, GHCB_SNP_UNSUPPORTED, unsupported);
    }
}

unsafe fn find_cc_blob_efi(bp: *mut BootParams) -> *mut CcBlobSevInfo {
    let mut pa = 0usize; let mut len = 0u32;
    if efi_get_conf_table(bp, &mut pa, &mut len) != 0 { return core::ptr::null_mut(); }
    efi_find_vendor_table(bp, pa, len, 0)
}

unsafe fn find_cc_blob(bp: *mut BootParams) -> *mut CcBlobSevInfo {
    let mut info = find_cc_blob_efi(bp);
    if info.is_null() { info = find_cc_blob_setup_data(bp); if info.is_null() { return core::ptr::null_mut(); } }
    if (*info).magic != CC_BLOB_SEV_HDR_MAGIC { sev_es_terminate(SEV_TERM_SET_GEN, GHCB_SNP_UNSUPPORTED); }
    info
}

unsafe fn early_snp_init(bp: *mut BootParams) -> bool {
    if bp.is_null() { return false; }
    let info = find_cc_blob(bp); if info.is_null() { return false; }
    setup_cpuid_table(info); svsm_setup_ca(info, rip_rel_ptr(&boot_ghcb_page));
    (*bp).cc_blob_address = info as usize as u32; true
}

unsafe fn sev_check_cpu_support() -> i32 {
    let (mut eax, mut ebx, mut ecx, mut edx) = (0x80000000u32, 0, 0, 0); native_cpuid(&mut eax, &mut ebx, &mut ecx, &mut edx);
    if eax < 0x8000001f { return -19; }
    eax = 0x8000001f; ecx = 0; native_cpuid(&mut eax, &mut ebx, &mut ecx, &mut edx);
    if eax & (1 << 1) == 0 { return -19; }
    sev_snp_needs_sfw = ebx & (1 << 31) == 0; (ebx & 0x3f) as i32
}

pub unsafe fn sev_enable(bp: *mut BootParams) {
    if !bp.is_null() { (*bp).cc_blob_address = 0; }
    if sev_check_cpu_support() < 0 { return; }
    let snp = early_snp_init(bp); let bitpos = sev_check_cpu_support();
    if bitpos < 0 { if snp { error(b"SEV-SNP support indicated by CC blob, but not CPUID.\0".as_ptr()); } return; }
    let mut m = Msr { q: 0 }; raw_rdmsr(MSR_AMD64_SEV, &mut m); sev_status = m.q;
    if sev_status & MSR_AMD64_SEV_ENABLED == 0 { return; }
    if sev_status & MSR_AMD64_SEV_ES_ENABLED != 0 && !sev_es_negotiate_protocol() { sev_es_terminate(SEV_TERM_SET_GEN, GHCB_SEV_ES_PROT_UNSUPPORTED); }
    if sev_status & MSR_AMD64_SEV_SNP_ENABLED != 0 {
        let hv = get_hv_features(); if hv & GHCB_HV_FT_SNP == 0 { sev_es_terminate(SEV_TERM_SET_GEN, GHCB_SNP_UNSUPPORTED); }
        if snp_vmpl != 0 && hv & GHCB_HV_FT_SNP_MULTI_VMPL == 0 { sev_es_terminate(SEV_TERM_SET_LINUX, GHCB_TERM_NOT_VMPL0); }
    }
    if snp && sev_status & MSR_AMD64_SEV_SNP_ENABLED == 0 { error(b"SEV-SNP supported indicated by CC blob, but not SEV status MSR.\0".as_ptr()); }
    sme_me_mask = 1u64 << bitpos;
}

pub unsafe fn sev_get_status() -> u64 { if sev_check_cpu_support() < 0 { 0 } else { let mut m = Msr { q: 0 }; raw_rdmsr(MSR_AMD64_SEV, &mut m); m.q } }

pub unsafe fn sev_prep_identity_maps(top_level_pgt: usize) {
    if sev_snp_enabled() { let pa = (*boot_params_ptr).cc_blob_address as u64; kernel_add_identity_map(pa, pa + core::mem::size_of::<CcBlobSevInfo>() as u64); let info = pa as *const CcBlobSevInfo; kernel_add_identity_map((*info).cpuid_phys, (*info).cpuid_phys + (*info).cpuid_len); }
    sev_verify_cbit(top_level_pgt);
}

pub unsafe fn early_is_sevsnp_guest() -> bool {
    static mut SEVSNP: bool = false;
    if SEVSNP { return true; } if sev_get_status() & MSR_AMD64_SEV_SNP_ENABLED == 0 { return false; } SEVSNP = true;
    if snp_vmpl == 0 { let (mut eax, mut ebx, mut ecx, mut edx) = (0x8000001f, 0, 0, 0); native_cpuid(&mut eax, &mut ebx, &mut ecx, &mut edx); if eax & (1 << 28) != 0 { let mut m = Msr { q: 0 }; raw_rdmsr(MSR_SVSM_CAA, &mut m); boot_svsm_caa_pa = m.q; snp_vmpl = u8::MAX; } }
    true
}

extern "C" { fn snp_register_ghcb_early(gpa: u64); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
