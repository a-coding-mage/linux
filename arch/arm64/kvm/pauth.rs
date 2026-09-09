// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2024 - Google LLC
 * Author: Marc Zyngier <maz@kernel.org>
 *
 * Primitive PAuth emulation for ERETAA/ERETAB.
 *
 * This code assumes that is is run from EL2, and that it is part of
 * the emulation of ERETAx for a guest hypervisor. That's a lot of
 * baked-in assumptions and shortcuts.
 *
 * Do no reuse for anything else!
 */

// Dependencies supplied by the surrounding kernel/KVM and architecture code.

unsafe fn compute_pac(vcpu: *mut kvm_vcpu, ptr: u64, ikey: ptrauth_key) -> u64 {
    let mut gkey: ptrauth_key;
    let mut mod_: u64;
    let mut pac: u64 = 0;

    preempt_disable();

    if !vcpu_get_flag(vcpu, SYSREGS_ON_CPU) {
        mod_ = __vcpu_sys_reg(vcpu, SP_EL2);
    } else {
        mod_ = read_sysreg(sp_el1);
    }

    gkey.lo = read_sysreg_s(SYS_APGAKEYLO_EL1);
    gkey.hi = read_sysreg_s(SYS_APGAKEYHI_EL1);

    __ptrauth_key_install_nosync(APGA, ikey);
    isb();

    // PACGA Xd, Xn, Xm
    pacga(&mut pac, ptr, mod_);
    isb();

    __ptrauth_key_install_nosync(APGA, gkey);

    preempt_enable();

    /* PAC in the top 32bits */
    pac
}

unsafe fn effective_tbi(vcpu: *mut kvm_vcpu, bit55: bool) -> bool {
    let tcr: u64 = vcpu_read_sys_reg(vcpu, TCR_EL2);
    let tbi: u64;
    let tbid: u64;

    /*
     * Since we are authenticating an instruction address, we have
     * to take TBID into account. If E2H==0, ignore VA[55], as
     * TCR_EL2 only has a single TBI/TBID. If VA[55] was set in
     * this case, this is likely a guest bug...
     */
    if !vcpu_el2_e2h_is_set(vcpu) {
        tbi = tcr & BIT(20);
        tbid = tcr & BIT(29);
    } else if bit55 {
        tbi = tcr & TCR_TBI1;
        tbid = tcr & TCR_TBID1;
    } else {
        tbi = tcr & TCR_TBI0;
        tbid = tcr & TCR_TBID0;
    }

    tbi != 0 && tbid == 0
}

unsafe fn compute_bottom_pac(vcpu: *mut kvm_vcpu, bit55: bool) -> i32 {
    const MAXTXSZ: i32 = 39; // Revisit these two values once
    const MINTXSZ: i32 = 16; // (if) we support TTST/LVA/LVA2
    let tcr: u64 = vcpu_read_sys_reg(vcpu, TCR_EL2);
    let txsz: i32;

    if !vcpu_el2_e2h_is_set(vcpu) || !bit55 {
        txsz = FIELD_GET(TCR_T0SZ_MASK, tcr) as i32;
    } else {
        txsz = FIELD_GET(TCR_T1SZ_MASK, tcr) as i32;
    }

    64 - txsz.clamp(MINTXSZ, MAXTXSZ)
}

unsafe fn compute_pac_mask(vcpu: *mut kvm_vcpu, bit55: bool) -> u64 {
    let bottom_pac: i32 = compute_bottom_pac(vcpu, bit55);
    let mut mask: u64 = GENMASK(54, bottom_pac as u32);

    if !effective_tbi(vcpu, bit55) {
        mask |= GENMASK(63, 56);
    }

    mask
}

unsafe fn to_canonical_addr(_vcpu: *mut kvm_vcpu, ptr: u64, mask: u64) -> u64 {
    let bit55 = (ptr & BIT(55)) != 0;

    if bit55 {
        ptr | mask
    } else {
        ptr & !mask
    }
}

unsafe fn corrupt_addr(vcpu: *mut kvm_vcpu, mut ptr: u64) -> u64 {
    let bit55 = (ptr & BIT(55)) != 0;
    let mask: u64;
    let error_code: u64;
    let shift: u32;

    if effective_tbi(vcpu, bit55) {
        mask = GENMASK(54, 53);
        shift = 53;
    } else {
        mask = GENMASK(62, 61);
        shift = 61;
    }

    if esr_iss_is_eretab(kvm_vcpu_get_esr(vcpu)) {
        error_code = 2u64 << shift;
    } else {
        error_code = 1u64 << shift;
    }

    ptr &= !mask;
    ptr |= error_code;

    ptr
}

/*
 * Authenticate an ERETAA/ERETAB instruction, returning true if the
 * authentication succeeded and false otherwise. In all cases, *elr
 * contains the VA to ERET to. Potential exception injection is left
 * to the caller.
 */
pub unsafe fn kvm_auth_eretax(vcpu: *mut kvm_vcpu, elr: *mut u64) -> bool {
    let sctlr: u64 = vcpu_read_sys_reg(vcpu, SCTLR_EL2);
    let esr: u64 = kvm_vcpu_get_esr(vcpu);
    let ptr: u64;
    let cptr: u64;
    let pac: u64;
    let mask: u64;
    let ikey: ptrauth_key;

    *elr = vcpu_read_sys_reg(vcpu, ELR_EL2);
    ptr = *elr;

    /* We assume we're already in the context of an ERETAx */
    if esr_iss_is_eretab(esr) {
        if sctlr & SCTLR_EL1_EnIB == 0 {
            return true;
        }

        ikey.lo = __vcpu_sys_reg(vcpu, APIBKEYLO_EL1);
        ikey.hi = __vcpu_sys_reg(vcpu, APIBKEYHI_EL1);
    } else {
        if sctlr & SCTLR_EL1_EnIA == 0 {
            return true;
        }

        ikey.lo = __vcpu_sys_reg(vcpu, APIAKEYLO_EL1);
        ikey.hi = __vcpu_sys_reg(vcpu, APIAKEYHI_EL1);
    }

    mask = compute_pac_mask(vcpu, (ptr & BIT(55)) != 0);
    cptr = to_canonical_addr(vcpu, ptr, mask);

    pac = compute_pac(vcpu, cptr, ikey);

    /*
     * Slightly deviate from the pseudocode: if we have a PAC
     * match with the signed pointer, then it must be good.
     * Anything after this point is pure error handling.
     */
    if (pac & mask == ptr & mask) {
        *elr = cptr;
        return true;
    }

    /*
     * Authentication failed, corrupt the canonical address if
     * PAuth2 isn't implemented, or some XORing if it is.
     */
    let cptr = if !kvm_has_pauth((*vcpu).kvm, PAuth2) {
        corrupt_addr(vcpu, cptr)
    } else {
        ptr ^ (pac & mask)
    };

    *elr = cptr;
    false
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
