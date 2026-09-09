// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 Google LLC
 * Author: Fuad Tabba <tabba@google.com>
 */

// C headers provide the architecture constants, types, and external functions
// referenced below.

pub static mut id_aa64pfr0_el1_sys_val: u64 = 0;
pub static mut id_aa64pfr1_el1_sys_val: u64 = 0;
pub static mut id_aa64pfr2_el1_sys_val: u64 = 0;
pub static mut id_aa64isar0_el1_sys_val: u64 = 0;
pub static mut id_aa64isar1_el1_sys_val: u64 = 0;
pub static mut id_aa64isar2_el1_sys_val: u64 = 0;
pub static mut id_aa64mmfr0_el1_sys_val: u64 = 0;
pub static mut id_aa64mmfr1_el1_sys_val: u64 = 0;
pub static mut id_aa64mmfr2_el1_sys_val: u64 = 0;
pub static mut id_aa64smfr0_el1_sys_val: u64 = 0;

#[repr(C)]
pub struct pvm_ftr_bits {
    pub sign: bool,
    pub shift: u8,
    pub width: u8,
    pub max_val: u8,
    pub vm_supported: Option<unsafe extern "C" fn(*const kvm) -> bool>,
}

unsafe fn vm_has_ptrauth(kvm: *const kvm) -> bool {
    if !IS_ENABLED(CONFIG_ARM64_PTR_AUTH) { return false; }
    (cpus_have_final_cap(ARM64_HAS_ADDRESS_AUTH) || cpus_have_final_cap(ARM64_HAS_GENERIC_AUTH))
        && kvm_vcpu_has_feature(kvm, KVM_ARM_VCPU_PTRAUTH_GENERIC)
}

unsafe fn vm_has_sve(kvm: *const kvm) -> bool {
    system_supports_sve() && kvm_vcpu_has_feature(kvm, KVM_ARM_VCPU_SVE)
}

// Feature restrictions for protected VMs. Constants are supplied by the
// architecture headers; the entries retain the C table's ordering and values.
macro_rules! feat { ($id:ident, $fld:ident, $max:ident) => { pvm_ftr_bits { sign: $id##_$fld##_SIGNED, shift: $id##_$fld##_SHIFT, width: $id##_$fld##_WIDTH, max_val: $id##_$fld##_$max, vm_supported: None } }; }
macro_rules! feat_fn { ($id:ident, $fld:ident, $max:ident, $f:ident) => { pvm_ftr_bits { sign: $id##_$fld##_SIGNED, shift: $id##_$fld##_SHIFT, width: $id##_$fld##_WIDTH, max_val: $id##_$fld##_$max, vm_supported: Some($f) } }; }
macro_rules! feat_enum { ($id:ident, $fld:ident, $max:ident) => { pvm_ftr_bits { sign: false, shift: $id##_$fld##_SHIFT, width: $id##_$fld##_WIDTH, max_val: $id##_$fld##_$max, vm_supported: None } }; }

// The token-pasting forms above mirror the C feature macros and are resolved
// by the surrounding architecture bindings.
static pvmid_aa64pfr0: &[pvm_ftr_bits] = &[
    feat!(ID_AA64PFR0_EL1, EL0, IMP), feat!(ID_AA64PFR0_EL1, EL1, IMP), feat!(ID_AA64PFR0_EL1, EL2, IMP), feat!(ID_AA64PFR0_EL1, EL3, IMP),
    feat!(ID_AA64PFR0_EL1, FP, FP16), feat!(ID_AA64PFR0_EL1, AdvSIMD, FP16), feat!(ID_AA64PFR0_EL1, GIC, IMP),
    feat_fn!(ID_AA64PFR0_EL1, SVE, IMP, vm_has_sve), feat!(ID_AA64PFR0_EL1, RAS, IMP), feat!(ID_AA64PFR0_EL1, DIT, IMP),
    feat!(ID_AA64PFR0_EL1, CSV2, IMP), feat!(ID_AA64PFR0_EL1, CSV3, IMP),
];
static pvmid_aa64pfr1: &[pvm_ftr_bits] = &[feat!(ID_AA64PFR1_EL1, BT, IMP), feat!(ID_AA64PFR1_EL1, SSBS, SSBS2), feat_enum!(ID_AA64PFR1_EL1, MTE_frac, NI)];
static pvmid_aa64pfr2: &[pvm_ftr_bits] = &[feat!(ID_AA64PFR2_EL1, GCIE, NI)];
static pvmid_aa64mmfr0: &[pvm_ftr_bits] = &[feat_enum!(ID_AA64MMFR0_EL1, PARANGE, 40), feat_enum!(ID_AA64MMFR0_EL1, ASIDBITS, 16), feat!(ID_AA64MMFR0_EL1, BIGEND, IMP), feat!(ID_AA64MMFR0_EL1, SNSMEM, IMP), feat!(ID_AA64MMFR0_EL1, BIGENDEL0, IMP), feat!(ID_AA64MMFR0_EL1, EXS, IMP)];
static pvmid_aa64mmfr1: &[pvm_ftr_bits] = &[feat!(ID_AA64MMFR1_EL1, HAFDBS, DBM), feat_enum!(ID_AA64MMFR1_EL1, VMIDBits, 16), feat!(ID_AA64MMFR1_EL1, HPDS, HPDS2), feat!(ID_AA64MMFR1_EL1, PAN, PAN3), feat!(ID_AA64MMFR1_EL1, SpecSEI, IMP), feat!(ID_AA64MMFR1_EL1, ETS, IMP), feat!(ID_AA64MMFR1_EL1, CMOW, IMP)];
static pvmid_aa64mmfr2: &[pvm_ftr_bits] = &[feat!(ID_AA64MMFR2_EL1, CnP, IMP), feat!(ID_AA64MMFR2_EL1, UAO, IMP), feat!(ID_AA64MMFR2_EL1, IESB, IMP), feat!(ID_AA64MMFR2_EL1, AT, IMP), feat!(ID_AA64MMFR2_EL1, IDS, IMP), feat!(ID_AA64MMFR2_EL1, TTL, IMP), feat!(ID_AA64MMFR2_EL1, BBM, 2), feat!(ID_AA64MMFR2_EL1, E0PD, IMP)];
static pvmid_aa64isar1: &[pvm_ftr_bits] = &[feat!(ID_AA64ISAR1_EL1, DPB, DPB2), feat_fn!(ID_AA64ISAR1_EL1, APA, PAuth, vm_has_ptrauth), feat_fn!(ID_AA64ISAR1_EL1, API, PAuth, vm_has_ptrauth), feat!(ID_AA64ISAR1_EL1, JSCVT, IMP), feat!(ID_AA64ISAR1_EL1, FCMA, IMP), feat!(ID_AA64ISAR1_EL1, LRCPC, LRCPC3), feat!(ID_AA64ISAR1_EL1, GPA, IMP), feat!(ID_AA64ISAR1_EL1, GPI, IMP), feat!(ID_AA64ISAR1_EL1, FRINTTS, IMP), feat!(ID_AA64ISAR1_EL1, SB, IMP), feat!(ID_AA64ISAR1_EL1, SPECRES, COSP_RCTX), feat!(ID_AA64ISAR1_EL1, BF16, EBF16), feat!(ID_AA64ISAR1_EL1, DGH, IMP), feat!(ID_AA64ISAR1_EL1, I8MM, IMP)];
static pvmid_aa64isar2: &[pvm_ftr_bits] = &[feat_fn!(ID_AA64ISAR2_EL1, GPA3, IMP, vm_has_ptrauth), feat_fn!(ID_AA64ISAR2_EL1, APA3, PAuth, vm_has_ptrauth), feat!(ID_AA64ISAR2_EL1, ATS1A, IMP)];

unsafe fn get_restricted_features(vcpu: *const kvm_vcpu, sys_reg_val: u64, restrictions: &[pvm_ftr_bits]) -> u64 {
    let mut val = 0u64;
    for f in restrictions {
        let supported = f.vm_supported.map_or(true, |x| x((*vcpu).kvm));
        let min_signed = (1u64 << f.width) - 1;
        let sign_bit = 1u64 << (f.width - 1);
        let mask = GENMASK_ULL(f.width as u32 + f.shift as u32 - 1, f.shift as u32);
        let sys_val = (sys_reg_val & mask) >> f.shift;
        let pvm_max = f.max_val as u64;
        if !supported { val |= (if f.sign { min_signed } else { 0 }) << f.shift; }
        else if f.sign && (sys_val >= sign_bit || pvm_max >= sign_bit) { val |= core::cmp::max(sys_val, pvm_max) << f.shift; }
        else { val |= core::cmp::min(sys_val, pvm_max) << f.shift; }
    }
    val
}

unsafe fn pvm_calc_id_reg(vcpu: *const kvm_vcpu, id: u32) -> u64 {
    match id {
        SYS_ID_AA64PFR0_EL1 => get_restricted_features(vcpu, id_aa64pfr0_el1_sys_val, pvmid_aa64pfr0),
        SYS_ID_AA64PFR1_EL1 => get_restricted_features(vcpu, id_aa64pfr1_el1_sys_val, pvmid_aa64pfr1),
        SYS_ID_AA64PFR2_EL1 => get_restricted_features(vcpu, id_aa64pfr2_el1_sys_val, pvmid_aa64pfr2),
        SYS_ID_AA64ISAR0_EL1 => id_aa64isar0_el1_sys_val,
        SYS_ID_AA64ISAR1_EL1 => get_restricted_features(vcpu, id_aa64isar1_el1_sys_val, pvmid_aa64isar1),
        SYS_ID_AA64ISAR2_EL1 => get_restricted_features(vcpu, id_aa64isar2_el1_sys_val, pvmid_aa64isar2),
        SYS_ID_AA64MMFR0_EL1 => get_restricted_features(vcpu, id_aa64mmfr0_el1_sys_val, pvmid_aa64mmfr0),
        SYS_ID_AA64MMFR1_EL1 => get_restricted_features(vcpu, id_aa64mmfr1_el1_sys_val, pvmid_aa64mmfr1),
        SYS_ID_AA64MMFR2_EL1 => get_restricted_features(vcpu, id_aa64mmfr2_el1_sys_val, pvmid_aa64mmfr2),
        SYS_ID_AA64DFR0_EL1 => ID_AA64DFR0_EL1_NONZERO_NI,
        SYS_ID_AA64MMFR4_EL1 => ID_AA64MMFR4_EL1_NONZERO_NI,
        _ => 0,
    }
}

unsafe fn inject_sync64(vcpu: *mut kvm_vcpu, esr: u64) {
    *vcpu_pc(vcpu) = read_sysreg_el2(SYS_ELR); *vcpu_cpsr(vcpu) = read_sysreg_el2(SYS_SPSR);
    __vcpu_assign_sys_reg(vcpu, VBAR_EL1, read_sysreg_el1(SYS_VBAR));
    __vcpu_assign_sys_reg(vcpu, SCTLR_EL1, read_sysreg_el1(SYS_SCTLR));
    kvm_pend_exception(vcpu, EXCEPT_AA64_EL1_SYNC); __kvm_adjust_pc(vcpu);
    write_sysreg_el1(esr, SYS_ESR); write_sysreg_el1(read_sysreg_el2(SYS_ELR), SYS_ELR); write_sysreg_el1(read_sysreg_el2(SYS_SPSR), SYS_SPSR);
    write_sysreg_el2(*vcpu_pc(vcpu), SYS_ELR); write_sysreg_el2(*vcpu_cpsr(vcpu), SYS_SPSR);
}

unsafe fn inject_undef64(vcpu: *mut kvm_vcpu) { inject_sync64(vcpu, (ESR_ELx_EC_UNKNOWN << ESR_ELx_EC_SHIFT) | ESR_ELx_IL); }

unsafe fn read_id_reg(vcpu: *const kvm_vcpu, r: *const sys_reg_desc) -> u64 {
    let kvm = (*vcpu).kvm; let reg = reg_to_encoding(r);
    if WARN_ON_ONCE(!test_bit(KVM_ARCH_FLAG_ID_REGS_INITIALIZED, &(*kvm).arch.flags)) { return 0; }
    if reg >= sys_reg(3,0,0,1,0) && reg <= sys_reg(3,0,0,7,7) { return (*kvm).arch.id_regs[IDREG_IDX(reg)]; }
    0
}

unsafe fn pvm_access_raz_wi(_vcpu: *mut kvm_vcpu, p: *mut sys_reg_params, _r: *const sys_reg_desc) -> bool { if !(*p).is_write { (*p).regval = 0; } true }
unsafe fn pvm_access_id_aarch32(vcpu: *mut kvm_vcpu, p: *mut sys_reg_params, r: *const sys_reg_desc) -> bool { if (*p).is_write { inject_undef64(vcpu); return false; } pvm_access_raz_wi(vcpu,p,r) }
unsafe fn pvm_access_id_aarch64(vcpu: *mut kvm_vcpu, p: *mut sys_reg_params, r: *const sys_reg_desc) -> bool { if (*p).is_write { inject_undef64(vcpu); return false; } (*p).regval = read_id_reg(vcpu,r); true }
unsafe fn pvm_gic_read_sre(_vcpu: *mut kvm_vcpu, p: *mut sys_reg_params, _r: *const sys_reg_desc) -> bool { if !(*p).is_write { (*p).regval = ICC_SRE_EL1_DIB | ICC_SRE_EL1_DFB | ICC_SRE_EL1_SRE; } true }
unsafe fn pvm_idst_access(vcpu: *mut kvm_vcpu, _p: *mut sys_reg_params, _r: *const sys_reg_desc) -> bool { if kvm_has_feat((*vcpu).kvm, ID_AA64MMFR2_EL1, IDS, IMP) { inject_sync64(vcpu,kvm_vcpu_get_esr(vcpu)); } else { inject_undef64(vcpu); } false }

// Architected descriptor table. The descriptor initializers are supplied by
// the system-register bindings and preserve the C table's sorted order.
pub static pvm_sys_reg_descs: &[sys_reg_desc] = &[
    SYS_DESC_ACCESS!(SYS_DBGBVRn_EL1(0), pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_DBGBCRn_EL1(0), pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_DBGWVRn_EL1(0), pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_DBGWCRn_EL1(0), pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_MDSCR_EL1, pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_OSLAR_EL1, pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_OSLSR_EL1, pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_OSDLR_EL1, pvm_access_raz_wi),
    SYS_DESC_HOST!(SYS_REVIDR_EL1),
    SYS_DESC_ACCESS!(SYS_ID_PFR0_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_PFR1_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_DFR0_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_AFR0_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_MMFR0_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_MMFR1_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_MMFR2_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_MMFR3_EL1,pvm_access_id_aarch32),
    SYS_DESC_ACCESS!(SYS_ID_ISAR0_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_ISAR1_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_ISAR2_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_ISAR3_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_ISAR4_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_ISAR5_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_MMFR4_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_ISAR6_EL1,pvm_access_id_aarch32),
    SYS_DESC_ACCESS!(SYS_MVFR0_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_MVFR1_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_MVFR2_EL1,pvm_access_id_aarch32), SYS_DESC_ID_UNALLOC!(3,3), SYS_DESC_ACCESS!(SYS_ID_PFR2_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_DFR1_EL1,pvm_access_id_aarch32), SYS_DESC_ACCESS!(SYS_ID_MMFR5_EL1,pvm_access_id_aarch32), SYS_DESC_ID_UNALLOC!(3,7),
    SYS_DESC_ACCESS!(SYS_ID_AA64PFR0_EL1,pvm_access_id_aarch64), SYS_DESC_ACCESS!(SYS_ID_AA64PFR1_EL1,pvm_access_id_aarch64), SYS_DESC_ACCESS!(SYS_ID_AA64PFR2_EL1,pvm_access_id_aarch64), SYS_DESC_ID_UNALLOC!(4,3), SYS_DESC_ACCESS!(SYS_ID_AA64ZFR0_EL1,pvm_access_id_aarch64), SYS_DESC_ID_UNALLOC!(4,5), SYS_DESC_ID_UNALLOC!(4,6), SYS_DESC_ID_UNALLOC!(4,7), SYS_DESC_ACCESS!(SYS_ID_AA64DFR0_EL1,pvm_access_id_aarch64), SYS_DESC_ACCESS!(SYS_ID_AA64DFR1_EL1,pvm_access_id_aarch64), SYS_DESC_ID_UNALLOC!(5,2), SYS_DESC_ID_UNALLOC!(5,3), SYS_DESC_ACCESS!(SYS_ID_AA64AFR0_EL1,pvm_access_id_aarch64), SYS_DESC_ACCESS!(SYS_ID_AA64AFR1_EL1,pvm_access_id_aarch64), SYS_DESC_ID_UNALLOC!(5,6), SYS_DESC_ID_UNALLOC!(5,7), SYS_DESC_ACCESS!(SYS_ID_AA64ISAR0_EL1,pvm_access_id_aarch64), SYS_DESC_ACCESS!(SYS_ID_AA64ISAR1_EL1,pvm_access_id_aarch64), SYS_DESC_ACCESS!(SYS_ID_AA64ISAR2_EL1,pvm_access_id_aarch64), SYS_DESC_ID_UNALLOC!(6,3), SYS_DESC_ID_UNALLOC!(6,4), SYS_DESC_ID_UNALLOC!(6,5), SYS_DESC_ID_UNALLOC!(6,6), SYS_DESC_ID_UNALLOC!(6,7), SYS_DESC_ACCESS!(SYS_ID_AA64MMFR0_EL1,pvm_access_id_aarch64), SYS_DESC_ACCESS!(SYS_ID_AA64MMFR1_EL1,pvm_access_id_aarch64), SYS_DESC_ACCESS!(SYS_ID_AA64MMFR2_EL1,pvm_access_id_aarch64), SYS_DESC_ID_UNALLOC!(7,3), SYS_DESC_ID_UNALLOC!(7,4), SYS_DESC_ID_UNALLOC!(7,5), SYS_DESC_ID_UNALLOC!(7,6), SYS_DESC_ID_UNALLOC!(7,7),
    SYS_DESC_HOST!(SYS_ICC_PMR_EL1), SYS_DESC_ACCESS!(SYS_ERRIDR_EL1,pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_ERRSELR_EL1,pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_ERXFR_EL1,pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_ERXCTLR_EL1,pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_ERXSTATUS_EL1,pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_ERXADDR_EL1,pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_ERXMISC0_EL1,pvm_access_raz_wi), SYS_DESC_ACCESS!(SYS_ERXMISC1_EL1,pvm_access_raz_wi),
    SYS_DESC_HOST!(SYS_ICC_DIR_EL1), SYS_DESC_HOST!(SYS_ICC_RPR_EL1), SYS_DESC_HOST!(SYS_ICC_SGI1R_EL1), SYS_DESC_HOST!(SYS_ICC_ASGI1R_EL1), SYS_DESC_HOST!(SYS_ICC_SGI0R_EL1), SYS_DESC_HOST!(SYS_ICC_CTLR_EL1), SYS_DESC_ACCESS!(SYS_ICC_SRE_EL1,pvm_gic_read_sre), SYS_DESC_HOST!(SYS_CCSIDR_EL1), SYS_DESC_HOST!(SYS_CLIDR_EL1), SYS_DESC_ACCESS!(SYS_CCSIDR2_EL1,pvm_idst_access), SYS_DESC_ACCESS!(SYS_GMID_EL1,pvm_idst_access), SYS_DESC_ACCESS!(SYS_SMIDR_EL1,pvm_idst_access), SYS_DESC_HOST!(SYS_AIDR_EL1), SYS_DESC_HOST!(SYS_CSSELR_EL1), SYS_DESC_HOST!(SYS_CTR_EL0), SYS_DESC_HOST!(SYS_CNTP_TVAL_EL0), SYS_DESC_HOST!(SYS_CNTP_CTL_EL0), SYS_DESC_HOST!(SYS_CNTP_CVAL_EL0),
];

pub unsafe fn kvm_init_pvm_id_regs(vcpu: *mut kvm_vcpu) { let kvm=(*vcpu).kvm; let ka=&mut (*kvm).arch; hyp_assert_lock_held(&vm_table_lock); if test_bit(KVM_ARCH_FLAG_ID_REGS_INITIALIZED,&(*kvm).arch.flags) { return; } let mut r=sys_reg(3,0,0,4,0); while r<=sys_reg(3,0,0,7,7) { ka.id_regs[IDREG_IDX(r)]=pvm_calc_id_reg(vcpu,r); r+=sys_reg(0,0,0,0,1); } set_bit(KVM_ARCH_FLAG_ID_REGS_INITIALIZED,&mut (*kvm).arch.flags); }
pub unsafe fn kvm_check_pvm_sysreg_table() -> i32 { for i in 1..pvm_sys_reg_descs.len() { if cmp_sys_reg(&pvm_sys_reg_descs[i-1],&pvm_sys_reg_descs[i])>=0 { return 1; } } 0 }
pub unsafe fn kvm_handle_pvm_sysreg(vcpu:*mut kvm_vcpu, _exit_code:*mut u64)->bool { let esr=kvm_vcpu_get_esr(vcpu); let rt=kvm_vcpu_sys_get_rt(vcpu); let mut params=esr_sys64_to_params(esr); params.regval=vcpu_get_reg(vcpu,rt); let r=find_reg(&params,pvm_sys_reg_descs); if r.is_null(){inject_undef64(vcpu);return true;} if (*r).access.is_none(){return false;} if ((*r).access.unwrap())(vcpu,&mut params,r){__kvm_skip_instr(vcpu);} if !params.is_write{vcpu_set_reg(vcpu,rt,params.regval);} true }
pub unsafe fn kvm_handle_pvm_restricted(vcpu:*mut kvm_vcpu,_exit_code:*mut u64)->bool{inject_undef64(vcpu);true}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
