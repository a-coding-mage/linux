/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from kvm_nested.h; dependencies are supplied by other headers. */

#[inline]
pub unsafe fn vcpu_has_nv(vcpu: *const kvm_vcpu) -> bool {
    !__is_defined(__KVM_NVHE_HYPERVISOR__) &&
        cpus_have_final_cap(ARM64_HAS_NESTED_VIRT) &&
        vcpu_has_feature(vcpu, KVM_ARM_VCPU_HAS_EL2)
}

#[inline]
pub fn tcr_el2_ps_to_tcr_el1_ips(tcr_el2: u64) -> u64 {
    ((tcr_el2 & TCR_EL2_PS_MASK) >> TCR_EL2_PS_SHIFT) << TCR_IPS_SHIFT
}

#[inline]
pub fn translate_tcr_el2_to_tcr_el1(tcr: u64) -> u64 {
    TCR_EPD1_MASK |
        if tcr & TCR_EL2_DS != 0 { TCR_DS } else { 0 } |
        if tcr & TCR_EL2_TBI != 0 { TCR_TBI0 } else { 0 } |
        tcr_el2_ps_to_tcr_el1_ips(tcr) |
        (tcr & TCR_EL2_TG0_MASK) | (tcr & TCR_EL2_ORGN0_MASK) |
        (tcr & TCR_EL2_IRGN0_MASK) | (tcr & TCR_EL2_T0SZ_MASK)
}

#[inline]
pub fn translate_cptr_el2_to_cpacr_el1(cptr_el2: u64) -> u64 {
    let mut cpacr_el1 = CPACR_EL1_RES1;
    if cptr_el2 & CPTR_EL2_TTA != 0 { cpacr_el1 |= CPACR_EL1_TTA; }
    if cptr_el2 & CPTR_EL2_TFP == 0 { cpacr_el1 |= CPACR_EL1_FPEN; }
    if cptr_el2 & CPTR_EL2_TZ == 0 { cpacr_el1 |= CPACR_EL1_ZEN; }
    cpacr_el1 | (cptr_el2 & (CPTR_EL2_TCPAC | CPTR_EL2_TAM))
}

#[inline]
pub fn translate_sctlr_el2_to_sctlr_el1(mut val: u64) -> u64 {
    val &= SCTLR_ELx_M | SCTLR_ELx_A | SCTLR_ELx_C | SCTLR_ELx_SA |
        SCTLR_ELx_I | SCTLR_ELx_IESB | SCTLR_ELx_WXN | SCTLR_ELx_EE;
    val | SCTLR_EL1_RES1
}

#[inline]
pub fn translate_ttbr0_el2_to_ttbr0_el1(ttbr0: u64) -> u64 { ttbr0 & !0xffff_0000_0000_0000u64 }

extern "C" {
    pub fn forward_smc_trap(vcpu: *mut kvm_vcpu) -> bool;
    pub fn forward_debug_exception(vcpu: *mut kvm_vcpu) -> bool;
    pub fn kvm_init_nested(kvm: *mut kvm);
    pub fn kvm_vcpu_init_nested(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_init_nested_s2_mmu(mmu: *mut kvm_s2_mmu);
    pub fn lookup_s2_mmu(vcpu: *mut kvm_vcpu) -> *mut kvm_s2_mmu;
    pub fn kvm_s2_mmu_iterate_by_vmid(kvm: *mut kvm, vmid: u16, info: *const tlbi_info,
        cb: Option<unsafe extern "C" fn(*mut kvm_s2_mmu, *const tlbi_info)>);
    pub fn kvm_vcpu_load_hw_mmu(vcpu: *mut kvm_vcpu);
    pub fn kvm_vcpu_put_hw_mmu(vcpu: *mut kvm_vcpu);
    pub fn check_nested_vcpu_requests(vcpu: *mut kvm_vcpu);
    pub fn kvm_nested_flush_hwstate(vcpu: *mut kvm_vcpu);
    pub fn kvm_nested_sync_hwstate(vcpu: *mut kvm_vcpu);
    pub fn kvm_nested_setup_mdcr_el2(vcpu: *mut kvm_vcpu);
}

#[repr(C)]
pub struct kvm_s2_trans { pub output: phys_addr_t, pub block_size: c_ulong, pub writable: bool, pub readable: bool, pub level: i32, pub esr: u32, pub desc: u64 }

#[inline] pub unsafe fn kvm_s2_trans_output(t: *mut kvm_s2_trans) -> phys_addr_t { (*t).output }
#[inline] pub unsafe fn kvm_s2_trans_size(t: *mut kvm_s2_trans) -> c_ulong { (*t).block_size }
#[inline] pub unsafe fn kvm_s2_trans_esr(t: *mut kvm_s2_trans) -> u32 { (*t).esr }
#[inline] pub unsafe fn kvm_s2_trans_readable(t: *mut kvm_s2_trans) -> bool { (*t).readable }
#[inline] pub unsafe fn kvm_s2_trans_writable(t: *mut kvm_s2_trans) -> bool { (*t).writable }

extern "C" { pub fn kvm_has_xnx(kvm: *mut kvm) -> bool; }

#[inline]
pub unsafe fn kvm_s2_trans_exec_el0(kvm: *mut kvm, trans: *mut kvm_s2_trans) -> bool {
    let mut xn = (( (*trans).desc & KVM_PTE_LEAF_ATTR_HI_S2_XN) >> KVM_PTE_LEAF_ATTR_HI_S2_XN_SHIFT) as u8;
    if !kvm_has_xnx(kvm) { xn &= 0b10; }
    xn == 0b00 || xn == 0b01
}
#[inline]
pub unsafe fn kvm_s2_trans_exec_el1(kvm: *mut kvm, trans: *mut kvm_s2_trans) -> bool {
    let mut xn = (( (*trans).desc & KVM_PTE_LEAF_ATTR_HI_S2_XN) >> KVM_PTE_LEAF_ATTR_HI_S2_XN_SHIFT) as u8;
    if !kvm_has_xnx(kvm) { xn &= 0b10; }
    xn == 0b00 || xn == 0b11
}

extern "C" {
    pub fn kvm_walk_nested_s2(vcpu: *mut kvm_vcpu, gipa: phys_addr_t, result: *mut kvm_s2_trans) -> i32;
    pub fn kvm_s2_handle_perm_fault(vcpu: *mut kvm_vcpu, trans: *mut kvm_s2_trans) -> i32;
    pub fn kvm_inject_s2_fault(vcpu: *mut kvm_vcpu, esr_el2: u64) -> i32;
    pub fn kvm_nested_s2_wp(kvm: *mut kvm);
    pub fn kvm_nested_s2_unmap(kvm: *mut kvm, may_block: bool);
    pub fn kvm_nested_s2_flush(kvm: *mut kvm);
    pub fn compute_tlb_inval_range(mmu: *mut kvm_s2_mmu, val: u64) -> c_ulong;
    pub fn kvm_init_nv_sysregs(vcpu: *mut kvm_vcpu) -> i32;
    pub fn limit_nv_id_reg(kvm: *mut kvm, reg: u32, val: u64) -> u64;
}

#[repr(C)] pub enum trans_regime { TR_EL10, TR_EL20, TR_EL2 }
#[repr(C)] pub struct s1_walk_info;
#[repr(C)] pub struct s1_walk_context { pub wi: *mut s1_walk_info, pub table_ipa: u64, pub level: i32 }
#[repr(C)] pub struct s1_walk_filter { pub fn_: Option<unsafe extern "C" fn(*mut s1_walk_context, *mut c_void) -> i32>, pub priv_: *mut c_void }
#[repr(C)] pub struct s1_walk_info { pub filter: *mut s1_walk_filter, pub baddr: u64, pub regime: trans_regime, pub max_oa_bits: c_uint, pub pgshift: c_uint, pub txsz: c_uint, pub sl: i32, pub sh: u8, pub as_el0: bool, pub hpd: bool, pub e0poe: bool, pub poe: bool, pub pan: bool, pub be: bool, pub s2: bool, pub pa52bit: bool, pub ha: bool }

#[repr(C)] pub struct s1_walk_result { pub desc: u64, pub pa: u64, pub level: i8, pub APTable: u8, pub nG: bool, pub asid: u16, pub UXNTable: bool, pub PXNTable: bool, pub uwxn: bool, pub uov: bool, pub ur: bool, pub uw: bool, pub ux: bool, pub pwxn: bool, pub pov: bool, pub pr: bool, pub pw: bool, pub px: bool, pub fst: u8, pub ptw: bool, pub s2: bool, pub failed: bool }

pub const S1_MMU_DISABLED: i32 = -127;
#[inline] pub unsafe fn fail_s1_walk(wr: *mut s1_walk_result, fst: u8, s1ptw: bool) { (*wr).fst=fst; (*wr).ptw=s1ptw; (*wr).s2=s1ptw; (*wr).failed=true; }
#[inline] pub unsafe fn s1_walk_translated(wr: *mut s1_walk_result) -> bool { (*wr).level != S1_MMU_DISABLED as i8 }

extern "C" {
    pub fn __kvm_translate_va(vcpu: *mut kvm_vcpu, wi: *mut s1_walk_info, wr: *mut s1_walk_result, va: u64) -> i32;
    pub fn __kvm_find_s1_desc_level(vcpu: *mut kvm_vcpu, va: u64, ipa: u64, level: *mut i32) -> i32;
    pub fn kvm_vcpu_allocate_vncr_tlb(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_handle_vncr_abort(vcpu: *mut kvm_vcpu) -> i32;
    pub fn kvm_handle_s1e2_tlbi(vcpu: *mut kvm_vcpu, inst: u32, val: u64);
    pub fn get_asid_by_regime(vcpu: *mut kvm_vcpu, regime: trans_regime) -> u16;
    pub fn __kvm_at_swap_desc(kvm: *mut kvm, ipa: gpa_t, old: u64, new: u64) -> i32;
}

#[inline]
pub unsafe fn ps_to_output_size(ps: c_uint, pa52bit: bool) -> c_uint {
    match ps { 0 => 32, 1 => 36, 2 => 40, 3 => 42, 4 => 44, 5 => 48,
        6 if pa52bit => 52, _ => 48 }
}

#[inline]
pub unsafe fn kvm_encode_nested_level(trans: *mut kvm_s2_trans) -> u64 {
    FIELD_PREP(KVM_NV_GUEST_MAP_SZ, (*trans).level as u64)
}

#[inline]
pub unsafe fn decode_range_tlbi(val: u64, range: *mut u64, asid: *mut u16) -> u64 {
    let tg = (val >> 46) & 3;
    let shift: u32 = match tg { 1 => 12, 2 => 14, _ => 16 };
    let mut base = (val & ((1u64 << 37) - 1)) << shift;
    base = base.wrapping_shl(16).wrapping_shr(16);
    if !asid.is_null() { *asid = ((val & TLBIR_ASID_MASK) >> TLBIR_ASID_SHIFT) as u16; }
    let scale = (val >> 44) & 3;
    let num = (val >> 39) & 0x1f;
    *range = __TLBI_RANGE_PAGES(num, scale) << shift;
    if base & (1u64 << 48) == 0 { *range = core::cmp::min(*range, (1u64 << 48).wrapping_sub(base)); }
    else { *range = core::cmp::min(*range, (!base).wrapping_add(1)); }
    base
}

#[cfg(not(CONFIG_ARM64_PTR_AUTH))]
#[inline]
pub unsafe fn kvm_auth_eretax(_vcpu: *mut kvm_vcpu, elr: *mut u64) -> bool {
    WARN_ON_ONCE(1); *elr = 0xbad9_acc0_debad_badu64; false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
