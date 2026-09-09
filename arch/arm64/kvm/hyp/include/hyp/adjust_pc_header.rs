// SPDX-License-Identifier: GPL-2.0-only
/*
 * Guest PC manipulation helpers
 *
 * Copyright (C) 2012,2013 - ARM Ltd
 * Copyright (C) 2020 - Google LLC
 * Author: Marc Zyngier <maz@kernel.org>
 */

// Dependencies supplied by the corresponding KVM emulation and host headers
// are intentionally left external to this translation unit.

pub unsafe fn kvm_skip_instr(vcpu: *mut kvm_vcpu) {
    if vcpu_mode_is_32bit(vcpu) {
        kvm_skip_instr32(vcpu);
    } else {
        *vcpu_pc(vcpu) = (*vcpu_pc(vcpu)).wrapping_add(4);
        *vcpu_cpsr(vcpu) &= !PSR_BTYPE_MASK;
    }

    /* advance the singlestep state machine */
    *vcpu_cpsr(vcpu) &= !DBG_SPSR_SS;
}

/*
 * Skip an instruction which has been emulated at hyp while most guest sysregs
 * are live.
 */
pub unsafe fn __kvm_skip_instr(vcpu: *mut kvm_vcpu) {
    *vcpu_pc(vcpu) = read_sysreg_el2(SYS_ELR);
    (*vcpu_gp_regs(vcpu)).pstate = read_sysreg_el2(SYS_SPSR);

    kvm_skip_instr(vcpu);

    write_sysreg_el2((*vcpu_gp_regs(vcpu)).pstate, SYS_SPSR);
    write_sysreg_el2(*vcpu_pc(vcpu), SYS_ELR);
}

/*
 * Skip an instruction while host sysregs are live.
 * Assumes host is always 64-bit.
 */
pub unsafe fn kvm_skip_host_instr() {
    write_sysreg_el2(read_sysreg_el2(SYS_ELR).wrapping_add(4), SYS_ELR);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
