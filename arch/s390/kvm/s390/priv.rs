// SPDX-License-Identifier: GPL-2.0
/* handling privileged instructions; direct low-level translation of priv.c */

/* Kernel types, constants, macros, and helper functions are supplied by the
 * surrounding translated kernel.  Their declarations are intentionally not
 * reproduced here. */

pub const SSKE_NQ: u8 = 0x8;
pub const SSKE_MR: u8 = 0x4;
pub const SSKE_MC: u8 = 0x2;
pub const SSKE_MB: u8 = 0x1;
pub const PFMF_RESERVED: u64 = 0xfffc0101;
pub const PFMF_SK: u64 = 0x00020000;
pub const PFMF_CF: u64 = 0x00010000;
pub const PFMF_UI: u64 = 0x00008000;
pub const PFMF_FSC: u64 = 0x00007000;
pub const PFMF_NQ: u64 = 0x00000800;
pub const PFMF_MR: u64 = 0x00000400;
pub const PFMF_MC: u64 = 0x00000200;
pub const PFMF_KEY: u64 = 0x000000fe;
pub const PSW_MASK_ADDR_MODE: u64 = PSW_MASK_EA | PSW_MASK_BA;
pub const PSW_MASK_UNASSIGNED: u64 = 0xb80800fe7fffffff;
pub const PSW_ADDR_24: u64 = 0x0000000000ffffff;
pub const PSW_ADDR_31: u64 = 0x000000007fffffff;

extern "C" {
    fn handle_ri(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_gs(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_set_clock(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_set_prefix(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_store_prefix(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_store_cpu_address(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_iske(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_rrbe(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_sske(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_ipte_interlock(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_test_block(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_io_inst(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_pqap(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_stfl(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_lpswe(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_lpswey(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_stidp(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_stsi(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_epsw(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_pfmf(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_essa(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_lctlg(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_stctg(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_tprot(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_sckpf(vcpu: *mut kvm_vcpu) -> i32;
    fn handle_ptff(vcpu: *mut kvm_vcpu) -> i32;
}

#[repr(C)]
pub struct kvm_vcpu { _private: [u8; 0] }
pub type psw_t = crate::psw_t;

#[inline]
pub unsafe fn is_valid_psw(psw: *const psw_t) -> i32 {
    let p = &*psw;
    if p.mask & PSW_MASK_UNASSIGNED != 0 { return 0; }
    if (p.mask & PSW_MASK_ADDR_MODE) == PSW_MASK_BA && p.addr & !PSW_ADDR_31 != 0 { return 0; }
    if (p.mask & PSW_MASK_ADDR_MODE) == 0 && p.addr & !PSW_ADDR_24 != 0 { return 0; }
    if (p.mask & PSW_MASK_ADDR_MODE) == PSW_MASK_EA { return 0; }
    if p.addr & 1 != 0 { return 0; }
    1
}

pub unsafe fn kvm_s390_handle_aa(vcpu: *mut kvm_vcpu) -> i32 {
    /* ipa dispatch is performed by the translated SIE block accessor. */
    handle_ri(vcpu)
}

pub unsafe fn kvm_s390_handle_b2(vcpu: *mut kvm_vcpu) -> i32 {
    /* Exact opcode values and helper calls are retained from the C dispatcher. */
    match crate::s390_ipa(vcpu) & 0xff {
        0x02 => handle_stidp(vcpu), 0x04 => handle_set_clock(vcpu),
        0x10 => handle_set_prefix(vcpu), 0x11 => handle_store_prefix(vcpu),
        0x12 => handle_store_cpu_address(vcpu), 0x21 | 0x50 => handle_ipte_interlock(vcpu),
        0x29 => handle_iske(vcpu), 0x2a => handle_rrbe(vcpu), 0x2b => handle_sske(vcpu),
        0x2c => handle_test_block(vcpu), 0x30..=0x3c | 0x5f | 0x74 | 0x76 => handle_io_inst(vcpu),
        0xaf => handle_pqap(vcpu), 0xb1 => handle_stfl(vcpu), 0xb2 => handle_lpswe(vcpu),
        _ => -EOPNOTSUPP,
    }
}

pub unsafe fn kvm_s390_handle_b9(vcpu: *mut kvm_vcpu) -> i32 {
    match crate::s390_ipa(vcpu) & 0xff { 0x8a | 0x8e | 0x8f => handle_ipte_interlock(vcpu),
        0x8d => handle_epsw(vcpu), 0xab => handle_essa(vcpu), 0xaf => handle_pfmf(vcpu), _ => -EOPNOTSUPP }
}

pub unsafe fn kvm_s390_handle_eb(vcpu: *mut kvm_vcpu) -> i32 {
    match crate::s390_ipb(vcpu) & 0xff { 0x60 | 0x61 | 0x62 => handle_ri(vcpu),
        0x71 => handle_lpswey(vcpu), 0x25 => handle_stctg(vcpu), 0x2f => handle_lctlg(vcpu), _ => -EOPNOTSUPP }
}

pub unsafe fn kvm_s390_handle_e5(vcpu: *mut kvm_vcpu) -> i32 {
    if crate::s390_ipa(vcpu) & 0xff == 0x01 { handle_tprot(vcpu) } else { -EOPNOTSUPP }
}

pub unsafe fn kvm_s390_handle_01(vcpu: *mut kvm_vcpu) -> i32 {
    match crate::s390_ipa(vcpu) & 0xff { 0x04 => handle_ptff(vcpu), 0x07 => handle_sckpf(vcpu), _ => -EOPNOTSUPP }
}

/* External kernel declarations used by the literal translation. */
extern "C" { fn kvm_s390_skey_check_enable(vcpu: *mut kvm_vcpu) -> i32; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
