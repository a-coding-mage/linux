// SPDX-License-Identifier: GPL-2.0-only
/*
 * Fault injection for both 32 and 64bit guests.
 *
 * Copyright (C) 2012,2013 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 *
 * Based on arch/arm/kvm/emulate.c
 * Copyright (C) 2012 - Virtual Open Systems and Columbia University
 * Author: Christoffer Dall <c.dall@virtualopensystems.com>
 */

unsafe fn exception_target_el(vcpu: *mut kvm_vcpu) -> u32 {
    if likely(!vcpu_has_nv(vcpu)) { return PSR_MODE_EL1h; }
    match *vcpu_cpsr(vcpu) & PSR_MODE_MASK {
        PSR_MODE_EL2h | PSR_MODE_EL2t => PSR_MODE_EL2h,
        PSR_MODE_EL1h | PSR_MODE_EL1t => PSR_MODE_EL1h,
        PSR_MODE_EL0t => if vcpu_el2_tge_is_set(vcpu) { PSR_MODE_EL2h } else { PSR_MODE_EL1h },
        _ => { BUG(); unreachable!() }
    }
}

unsafe fn exception_esr_elx(vcpu: *mut kvm_vcpu) -> vcpu_sysreg {
    if exception_target_el(vcpu) == PSR_MODE_EL2h { ESR_EL2 } else { ESR_EL1 }
}

unsafe fn exception_far_elx(vcpu: *mut kvm_vcpu) -> vcpu_sysreg {
    if exception_target_el(vcpu) == PSR_MODE_EL2h { FAR_EL2 } else { FAR_EL1 }
}

unsafe fn pend_sync_exception(vcpu: *mut kvm_vcpu) {
    kvm_pend_exception(vcpu, if exception_target_el(vcpu) == PSR_MODE_EL1h { EXCEPT_AA64_EL1_SYNC } else { EXCEPT_AA64_EL2_SYNC });
}

unsafe fn pend_serror_exception(vcpu: *mut kvm_vcpu) {
    kvm_pend_exception(vcpu, if exception_target_el(vcpu) == PSR_MODE_EL1h { EXCEPT_AA64_EL1_SERR } else { EXCEPT_AA64_EL2_SERR });
}

unsafe fn __effective_sctlr2_bit(vcpu: *mut kvm_vcpu, idx: u32) -> bool {
    if !kvm_has_sctlr2((*vcpu).kvm) || (is_nested_ctxt(vcpu) && !(__vcpu_sys_reg(vcpu, HCRX_EL2) & HCRX_EL2_SCTLR2En) != 0) { return false; }
    let sctlr2 = if exception_target_el(vcpu) == PSR_MODE_EL1h { vcpu_read_sys_reg(vcpu, SCTLR2_EL1) } else { vcpu_read_sys_reg(vcpu, SCTLR2_EL2) };
    (sctlr2 & BIT(idx)) != 0
}

unsafe fn effective_sctlr2_ease(vcpu: *mut kvm_vcpu) -> bool { __effective_sctlr2_bit(vcpu, SCTLR2_EL1_EASE_SHIFT) }
unsafe fn effective_sctlr2_nmea(vcpu: *mut kvm_vcpu) -> bool { __effective_sctlr2_bit(vcpu, SCTLR2_EL1_NMEA_SHIFT) }

unsafe fn inject_abt64(vcpu: *mut kvm_vcpu, is_iabt: bool, addr: c_ulong) {
    let cpsr = *vcpu_cpsr(vcpu); let is_aarch32 = vcpu_mode_is_32bit(vcpu); let mut esr: u64 = 0; let fsc: u64;
    if kvm_vcpu_abt_iss1tw(vcpu) { let hpfar = kvm_vcpu_get_fault_ipa(vcpu); if hpfar == INVALID_GPA { return; } let mut level = 0; if __kvm_find_s1_desc_level(vcpu, addr, hpfar, &mut level) != 0 { return; } WARN_ON_ONCE(level < -1 || level > 3); fsc = ESR_ELx_FSC_SEA_TTW(level); } else { fsc = ESR_ELx_FSC_EXTABT; }
    if effective_sctlr2_ease(vcpu) { pend_serror_exception(vcpu); } else { pend_sync_exception(vcpu); }
    esr |= ESR_ELx_IL;
    esr |= if is_aarch32 || (cpsr & PSR_MODE_MASK) == PSR_MODE_EL0t { ESR_ELx_EC_IABT_LOW << ESR_ELx_EC_SHIFT } else { ESR_ELx_EC_IABT_CUR << ESR_ELx_EC_SHIFT };
    if !is_iabt { esr |= ESR_ELx_EC_DABT_LOW << ESR_ELx_EC_SHIFT; }
    esr |= fsc;
    vcpu_write_sys_reg(vcpu, addr, exception_far_elx(vcpu)); vcpu_write_sys_reg(vcpu, esr, exception_esr_elx(vcpu));
}

pub unsafe fn kvm_inject_sync(vcpu: *mut kvm_vcpu, esr: u64) { pend_sync_exception(vcpu); vcpu_write_sys_reg(vcpu, esr, exception_esr_elx(vcpu)); }
unsafe fn inject_undef64(vcpu: *mut kvm_vcpu) { kvm_inject_sync(vcpu, (ESR_ELx_EC_UNKNOWN << ESR_ELx_EC_SHIFT) | ESR_ELx_IL); }

const DFSR_FSC_EXTABT_LPAE: u32 = 0x10; const DFSR_FSC_EXTABT_NLPAE: u32 = 0x08; const DFSR_LPAE: u32 = BIT(9); const TTBCR_EAE: u64 = BIT(31);
unsafe fn inject_undef32(vcpu: *mut kvm_vcpu) { kvm_pend_exception(vcpu, EXCEPT_AA32_UND); }

unsafe fn inject_abt32(vcpu: *mut kvm_vcpu, is_pabt: bool, addr: u32) {
    let mut far = vcpu_read_sys_reg(vcpu, FAR_EL1); let fsr = if vcpu_read_sys_reg(vcpu, TCR_EL1) & TTBCR_EAE != 0 { DFSR_LPAE | DFSR_FSC_EXTABT_LPAE } else { DFSR_FSC_EXTABT_NLPAE };
    if is_pabt { kvm_pend_exception(vcpu, EXCEPT_AA32_IABT); far &= GENMASK(31, 0); far |= (addr as u64) << 32; vcpu_write_sys_reg(vcpu, fsr, IFSR32_EL2); } else { kvm_pend_exception(vcpu, EXCEPT_AA32_DABT); far &= GENMASK(63, 32); far |= addr as u64; vcpu_write_sys_reg(vcpu, fsr, ESR_EL1); } vcpu_write_sys_reg(vcpu, far, FAR_EL1);
}
unsafe fn __kvm_inject_sea(vcpu: *mut kvm_vcpu, iabt: bool, addr: u64) { if vcpu_el1_is_32bit(vcpu) { inject_abt32(vcpu, iabt, addr as u32); } else { inject_abt64(vcpu, iabt, addr as c_ulong); } }
unsafe fn kvm_sea_target_is_el2(vcpu: *mut kvm_vcpu) -> bool { if __vcpu_sys_reg(vcpu, HCR_EL2) & (HCR_TGE | HCR_TEA) != 0 { return true; } if !vcpu_mode_priv(vcpu) { return false; } (*vcpu_cpsr(vcpu) & PSR_A_BIT != 0) && (__vcpu_sys_reg(vcpu, HCRX_EL2) & HCRX_EL2_TMEA != 0) }
pub unsafe fn kvm_inject_sea(vcpu: *mut kvm_vcpu, iabt: bool, addr: u64) -> i32 { lockdep_assert_held(&(*vcpu).mutex); if is_nested_ctxt(vcpu) && kvm_sea_target_is_el2(vcpu) { return kvm_inject_nested_sea(vcpu, iabt, addr); } __kvm_inject_sea(vcpu, iabt, addr); 1 }

unsafe fn kvm_inject_nested_excl_atomic(vcpu: *mut kvm_vcpu, addr: u64) -> i32 { let esr = FIELD_PREP(ESR_ELx_EC_MASK, ESR_ELx_EC_DABT_LOW) | FIELD_PREP(ESR_ELx_FSC, ESR_ELx_FSC_EXCL_ATOMIC) | ESR_ELx_IL; vcpu_write_sys_reg(vcpu, addr, FAR_EL2); kvm_inject_nested_sync(vcpu, esr) }
pub unsafe fn kvm_inject_dabt_excl_atomic(vcpu: *mut kvm_vcpu, addr: u64) -> i32 { if is_nested_ctxt(vcpu) && vcpu_read_sys_reg(vcpu, HCR_EL2) & HCR_VM != 0 { return kvm_inject_nested_excl_atomic(vcpu, addr); } __kvm_inject_sea(vcpu, false, addr); let mut esr = vcpu_read_sys_reg(vcpu, exception_esr_elx(vcpu)); esr &= !ESR_ELx_FSC; esr |= ESR_ELx_FSC_EXCL_ATOMIC; vcpu_write_sys_reg(vcpu, esr, exception_esr_elx(vcpu)); 1 }

pub unsafe fn kvm_inject_size_fault(vcpu: *mut kvm_vcpu) { let mut addr = kvm_vcpu_get_fault_ipa(vcpu); addr |= FAR_TO_FIPA_OFFSET(kvm_vcpu_get_hfar(vcpu)); __kvm_inject_sea(vcpu, kvm_vcpu_trap_is_iabt(vcpu), addr); if vcpu_el1_is_32bit(vcpu) && vcpu_read_sys_reg(vcpu, TCR_EL1) & TTBCR_EAE == 0 { return; } let mut esr = vcpu_read_sys_reg(vcpu, exception_esr_elx(vcpu)); esr &= !GENMASK_ULL(5, 0); vcpu_write_sys_reg(vcpu, esr, exception_esr_elx(vcpu)); }
pub unsafe fn kvm_inject_undefined(vcpu: *mut kvm_vcpu) { if vcpu_el1_is_32bit(vcpu) { inject_undef32(vcpu); } else { inject_undef64(vcpu); } }
unsafe fn serror_is_masked(vcpu: *mut kvm_vcpu) -> bool { (*vcpu_cpsr(vcpu) & PSR_A_BIT != 0) && !effective_sctlr2_nmea(vcpu) }
unsafe fn kvm_serror_target_is_el2(vcpu: *mut kvm_vcpu) -> bool { if is_hyp_ctxt(vcpu) || vcpu_el2_amo_is_set(vcpu) { return true; } if __vcpu_sys_reg(vcpu, HCRX_EL2) & HCRX_EL2_TMEA == 0 { return false; } if vcpu_mode_priv(vcpu) { return *vcpu_cpsr(vcpu) & PSR_A_BIT != 0; } serror_is_masked(vcpu) }
unsafe fn kvm_serror_undeliverable_at_el2(vcpu: *mut kvm_vcpu) -> bool { !(vcpu_el2_tge_is_set(vcpu) || vcpu_el2_amo_is_set(vcpu)) }
pub unsafe fn kvm_inject_serror_esr(vcpu: *mut kvm_vcpu, mut esr: u64) -> i32 { lockdep_assert_held(&(*vcpu).mutex); if is_nested_ctxt(vcpu) && kvm_serror_target_is_el2(vcpu) { return kvm_inject_nested_serror(vcpu, esr); } if vcpu_is_el2(vcpu) && kvm_serror_undeliverable_at_el2(vcpu) { vcpu_set_vsesr(vcpu, esr); vcpu_set_flag(vcpu, NESTED_SERROR_PENDING); return 1; } if !serror_is_masked(vcpu) { pend_serror_exception(vcpu); esr |= FIELD_PREP(ESR_ELx_EC_MASK, ESR_ELx_EC_SERROR) | ESR_ELx_IL; vcpu_write_sys_reg(vcpu, esr, exception_esr_elx(vcpu)); return 1; } vcpu_set_vsesr(vcpu, esr & ESR_ELx_ISS_MASK); *vcpu_hcr(vcpu) |= HCR_VSE; 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
