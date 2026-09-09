// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012-2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// External kernel declarations and architecture helpers are supplied by other files.

unsafe fn __sysreg_save_vel2_state(vcpu: *mut kvm_vcpu) {
    // These registers are common with EL1
    __vcpu_assign_sys_reg(vcpu, PAR_EL1, read_sysreg(par_el1));
    __vcpu_assign_sys_reg(vcpu, TPIDR_EL1, read_sysreg(tpidr_el1));
    __vcpu_assign_sys_reg(vcpu, ESR_EL2, read_sysreg_el1(SYS_ESR));
    __vcpu_assign_sys_reg(vcpu, AFSR0_EL2, read_sysreg_el1(SYS_AFSR0));
    __vcpu_assign_sys_reg(vcpu, AFSR1_EL2, read_sysreg_el1(SYS_AFSR1));
    __vcpu_assign_sys_reg(vcpu, FAR_EL2, read_sysreg_el1(SYS_FAR));
    __vcpu_assign_sys_reg(vcpu, MAIR_EL2, read_sysreg_el1(SYS_MAIR));
    __vcpu_assign_sys_reg(vcpu, VBAR_EL2, read_sysreg_el1(SYS_VBAR));
    __vcpu_assign_sys_reg(vcpu, CONTEXTIDR_EL2, read_sysreg_el1(SYS_CONTEXTIDR));
    __vcpu_assign_sys_reg(vcpu, AMAIR_EL2, read_sysreg_el1(SYS_AMAIR));

    /*
     * In VHE mode those registers are compatible between EL1 and EL2,
     * and the guest uses the _EL1 versions on the CPU naturally.
     * So we save them into their _EL2 versions here.
     * For nVHE mode we trap accesses to those registers, so our
     * _EL2 copy in sys_regs[] is always up-to-date and we don't need to
     * save anything here.
     */
    if vcpu_el2_e2h_is_set(vcpu) {
        let mut val: u64;
        if cpus_have_final_cap(ARM64_HAS_NV2P1) {
            __vcpu_assign_sys_reg(vcpu, CPTR_EL2, read_sysreg_el1(SYS_CPACR));
        }
        __vcpu_assign_sys_reg(vcpu, SCTLR_EL2, read_sysreg_el1(SYS_SCTLR));
        __vcpu_assign_sys_reg(vcpu, TTBR0_EL2, read_sysreg_el1(SYS_TTBR0));
        __vcpu_assign_sys_reg(vcpu, TTBR1_EL2, read_sysreg_el1(SYS_TTBR1));
        __vcpu_assign_sys_reg(vcpu, TCR_EL2, read_sysreg_el1(SYS_TCR));
        if ctxt_has_tcrx(&(*vcpu).arch.ctxt) {
            __vcpu_assign_sys_reg(vcpu, TCR2_EL2, read_sysreg_el1(SYS_TCR2));
            if ctxt_has_s1pie(&(*vcpu).arch.ctxt) {
                __vcpu_assign_sys_reg(vcpu, PIRE0_EL2, read_sysreg_el1(SYS_PIRE0));
                __vcpu_assign_sys_reg(vcpu, PIR_EL2, read_sysreg_el1(SYS_PIR));
            }
            if ctxt_has_s1poe(&(*vcpu).arch.ctxt) {
                __vcpu_assign_sys_reg(vcpu, POR_EL2, read_sysreg_el1(SYS_POR));
            }
        }
        val = read_sysreg_el1(SYS_CNTKCTL);
        if !cpus_have_final_cap(ARM64_HAS_NV2P1) {
            val &= CNTKCTL_VALID_BITS;
            __vcpu_rmw_sys_reg(vcpu, CNTHCTL_EL2, &=, !CNTKCTL_VALID_BITS);
            __vcpu_rmw_sys_reg(vcpu, CNTHCTL_EL2, |=, val);
        } else {
            __vcpu_assign_sys_reg(vcpu, CNTHCTL_EL2, val);
        }
    }
    __vcpu_assign_sys_reg(vcpu, SP_EL2, read_sysreg(sp_el1));
    __vcpu_assign_sys_reg(vcpu, ELR_EL2, read_sysreg_el1(SYS_ELR));
    __vcpu_assign_sys_reg(vcpu, SPSR_EL2, read_sysreg_el1(SYS_SPSR));
    if ctxt_has_sctlr2(&(*vcpu).arch.ctxt) {
        __vcpu_assign_sys_reg(vcpu, SCTLR2_EL2, read_sysreg_el1(SYS_SCTLR2));
    }
}

unsafe fn __sysreg_restore_vel2_state(vcpu: *mut kvm_vcpu) {
    let mut val: u64;
    write_sysreg(__vcpu_sys_reg(vcpu, PAR_EL1), par_el1);
    write_sysreg(__vcpu_sys_reg(vcpu, TPIDR_EL1), tpidr_el1);
    write_sysreg(ctxt_midr_el1(&(*vcpu).arch.ctxt), vpidr_el2);
    write_sysreg(__vcpu_sys_reg(vcpu, MPIDR_EL1), vmpidr_el2);
    write_sysreg_el1(__vcpu_sys_reg(vcpu, MAIR_EL2), SYS_MAIR);
    write_sysreg_el1(__vcpu_sys_reg(vcpu, VBAR_EL2), SYS_VBAR);
    write_sysreg_el1(__vcpu_sys_reg(vcpu, CONTEXTIDR_EL2), SYS_CONTEXTIDR);
    write_sysreg_el1(__vcpu_sys_reg(vcpu, AMAIR_EL2), SYS_AMAIR);
    if vcpu_el2_e2h_is_set(vcpu) {
        write_sysreg_el1(__vcpu_sys_reg(vcpu, SCTLR_EL2), SYS_SCTLR);
        write_sysreg_el1(__vcpu_sys_reg(vcpu, CPTR_EL2), SYS_CPACR);
        write_sysreg_el1(__vcpu_sys_reg(vcpu, TTBR0_EL2), SYS_TTBR0);
        write_sysreg_el1(__vcpu_sys_reg(vcpu, TTBR1_EL2), SYS_TTBR1);
        write_sysreg_el1(__vcpu_sys_reg(vcpu, TCR_EL2), SYS_TCR);
        write_sysreg_el1(__vcpu_sys_reg(vcpu, CNTHCTL_EL2), SYS_CNTKCTL);
    } else {
        val = translate_sctlr_el2_to_sctlr_el1(__vcpu_sys_reg(vcpu, SCTLR_EL2));
        write_sysreg_el1(val, SYS_SCTLR);
        val = translate_cptr_el2_to_cpacr_el1(__vcpu_sys_reg(vcpu, CPTR_EL2));
        write_sysreg_el1(val, SYS_CPACR);
        val = translate_ttbr0_el2_to_ttbr0_el1(__vcpu_sys_reg(vcpu, TTBR0_EL2));
        write_sysreg_el1(val, SYS_TTBR0);
        val = translate_tcr_el2_to_tcr_el1(__vcpu_sys_reg(vcpu, TCR_EL2));
        write_sysreg_el1(val, SYS_TCR);
    }
    if ctxt_has_tcrx(&(*vcpu).arch.ctxt) {
        write_sysreg_el1(__vcpu_sys_reg(vcpu, TCR2_EL2), SYS_TCR2);
        if ctxt_has_s1pie(&(*vcpu).arch.ctxt) {
            write_sysreg_el1(__vcpu_sys_reg(vcpu, PIR_EL2), SYS_PIR);
            write_sysreg_el1(__vcpu_sys_reg(vcpu, PIRE0_EL2), SYS_PIRE0);
        }
        if ctxt_has_s1poe(&(*vcpu).arch.ctxt) {
            write_sysreg_el1(__vcpu_sys_reg(vcpu, POR_EL2), SYS_POR);
        }
    }
    write_sysreg_el1(__vcpu_sys_reg(vcpu, ESR_EL2), SYS_ESR);
    write_sysreg_el1(__vcpu_sys_reg(vcpu, AFSR0_EL2), SYS_AFSR0);
    write_sysreg_el1(__vcpu_sys_reg(vcpu, AFSR1_EL2), SYS_AFSR1);
    write_sysreg_el1(__vcpu_sys_reg(vcpu, FAR_EL2), SYS_FAR);
    write_sysreg(__vcpu_sys_reg(vcpu, SP_EL2), sp_el1);
    write_sysreg_el1(__vcpu_sys_reg(vcpu, ELR_EL2), SYS_ELR);
    write_sysreg_el1(__vcpu_sys_reg(vcpu, SPSR_EL2), SYS_SPSR);
    if ctxt_has_sctlr2(&(*vcpu).arch.ctxt) {
        write_sysreg_el1(__vcpu_sys_reg(vcpu, SCTLR2_EL2), SYS_SCTLR2);
    }
}

pub unsafe fn sysreg_save_host_state_vhe(ctxt: *mut kvm_cpu_context) { __sysreg_save_common_state(ctxt); }
pub unsafe fn sysreg_save_guest_state_vhe(ctxt: *mut kvm_cpu_context) { __sysreg_save_common_state(ctxt); __sysreg_save_el2_return_state(ctxt); }
pub unsafe fn sysreg_restore_host_state_vhe(ctxt: *mut kvm_cpu_context) { __sysreg_restore_common_state(ctxt); }
pub unsafe fn sysreg_restore_guest_state_vhe(ctxt: *mut kvm_cpu_context) { __sysreg_restore_common_state(ctxt); __sysreg_restore_el2_return_state(ctxt); }

unsafe fn __mpam_guest_load() {
    let mask = MPAM0_EL1_PARTID_D | MPAM0_EL1_PARTID_I | MPAM0_EL1_PMG_D | MPAM0_EL1_PMG_I;
    if system_supports_mpam() {
        let val = (read_sysreg_s(SYS_MPAM0_EL1) & mask) | MPAM1_EL1_MPAMEN;
        write_sysreg_el1(val, SYS_MPAM1);
    }
}

pub unsafe fn __vcpu_load_switch_sysregs(vcpu: *mut kvm_vcpu) {
    let guest_ctxt = &mut (*vcpu).arch.ctxt as *mut kvm_cpu_context;
    let host_ctxt: *mut kvm_cpu_context;
    let mut midr: u64;
    let mut mpidr: u64;
    host_ctxt = host_data_ptr(host_ctxt);
    __sysreg_save_user_state(host_ctxt);
    if vcpu_has_nv(vcpu) { dsb(nsh); }
    __sysreg32_restore_state(vcpu);
    __sysreg_restore_user_state(guest_ctxt);
    __mpam_guest_load();
    if unlikely(is_hyp_ctxt(vcpu)) {
        __sysreg_restore_vel2_state(vcpu);
    } else {
        if vcpu_has_nv(vcpu) {
            midr = ctxt_sys_reg(guest_ctxt, VPIDR_EL2);
            mpidr = ctxt_sys_reg(guest_ctxt, VMPIDR_EL2);
        } else {
            midr = ctxt_midr_el1(guest_ctxt);
            mpidr = ctxt_sys_reg(guest_ctxt, MPIDR_EL1);
        }
        __sysreg_restore_el1_state(guest_ctxt, midr, mpidr);
    }
    vcpu_set_flag(vcpu, SYSREGS_ON_CPU);
}

pub unsafe fn __vcpu_put_switch_sysregs(vcpu: *mut kvm_vcpu) {
    let guest_ctxt = &mut (*vcpu).arch.ctxt as *mut kvm_cpu_context;
    let host_ctxt: *mut kvm_cpu_context;
    host_ctxt = host_data_ptr(host_ctxt);
    if unlikely(is_hyp_ctxt(vcpu)) { __sysreg_save_vel2_state(vcpu); } else { __sysreg_save_el1_state(guest_ctxt); }
    __sysreg_save_user_state(guest_ctxt);
    __sysreg32_save_state(vcpu);
    __sysreg_restore_user_state(host_ctxt);
    vcpu_clear_flag(vcpu, SYSREGS_ON_CPU);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
