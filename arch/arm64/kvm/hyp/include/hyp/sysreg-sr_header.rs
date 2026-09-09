// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012-2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// Dependencies supplied by the surrounding kernel translation unit.

unsafe fn ctxt_to_vcpu(ctxt: *mut kvm_cpu_context) -> *mut kvm_vcpu {
    let mut vcpu = (*ctxt).__hyp_running_vcpu;

    if vcpu.is_null() {
        vcpu = container_of(ctxt, kvm_vcpu, arch.ctxt);
    }

    vcpu
}

unsafe fn ctxt_is_guest(ctxt: *mut kvm_cpu_context) -> bool {
    host_data_ptr(host_ctxt) != ctxt
}

unsafe fn ctxt_mdscr_el1(ctxt: *mut kvm_cpu_context) -> *mut u64 {
    let vcpu = ctxt_to_vcpu(ctxt);

    if ctxt_is_guest(ctxt) && kvm_host_owns_debug_regs(vcpu) {
        return &mut (*vcpu).arch.external_mdscr_el1;
    }

    ctxt_sys_reg_mut(ctxt, MDSCR_EL1)
}

unsafe fn ctxt_midr_el1(ctxt: *mut kvm_cpu_context) -> u64 {
    let kvm = kern_hyp_va((*ctxt_to_vcpu(ctxt)).kvm);

    if !(ctxt_is_guest(ctxt)
        && test_bit(KVM_ARCH_FLAG_WRITABLE_IMP_ID_REGS, &(*kvm).arch.flags))
    {
        return read_cpuid_id();
    }

    kvm_read_vm_id_reg(kvm, SYS_MIDR_EL1)
}

unsafe fn __sysreg_save_common_state(ctxt: *mut kvm_cpu_context) {
    *ctxt_mdscr_el1(ctxt) = read_sysreg(mdscr_el1);

    // POR_EL0 can affect uaccess, so must be saved/restored early.
    if ctxt_has_s1poe(ctxt) {
        *ctxt_sys_reg_mut(ctxt, POR_EL0) = read_sysreg_s(SYS_POR_EL0);
    }
}

unsafe fn __sysreg_save_user_state(ctxt: *mut kvm_cpu_context) {
    *ctxt_sys_reg_mut(ctxt, TPIDR_EL0) = read_sysreg(tpidr_el0);
    *ctxt_sys_reg_mut(ctxt, TPIDRRO_EL0) = read_sysreg(tpidrro_el0);
}

unsafe fn ctxt_has_mte(ctxt: *mut kvm_cpu_context) -> bool {
    let vcpu = ctxt_to_vcpu(ctxt);
    kvm_has_mte(kern_hyp_va((*vcpu).kvm))
}

unsafe fn ctxt_has_s1pie(ctxt: *mut kvm_cpu_context) -> bool {
    if !cpus_have_final_cap(ARM64_HAS_S1PIE) {
        return false;
    }
    let vcpu = ctxt_to_vcpu(ctxt);
    kvm_has_s1pie(kern_hyp_va((*vcpu).kvm))
}

unsafe fn ctxt_has_tcrx(ctxt: *mut kvm_cpu_context) -> bool {
    if !cpus_have_final_cap(ARM64_HAS_TCR2) {
        return false;
    }
    let vcpu = ctxt_to_vcpu(ctxt);
    kvm_has_tcr2(kern_hyp_va((*vcpu).kvm))
}

unsafe fn ctxt_has_s1poe(ctxt: *mut kvm_cpu_context) -> bool {
    if !system_supports_poe() {
        return false;
    }
    let vcpu = ctxt_to_vcpu(ctxt);
    kvm_has_s1poe(kern_hyp_va((*vcpu).kvm))
}

unsafe fn ctxt_has_ras(ctxt: *mut kvm_cpu_context) -> bool {
    if !cpus_have_final_cap(ARM64_HAS_RAS_EXTN) {
        return false;
    }
    let vcpu = ctxt_to_vcpu(ctxt);
    kvm_has_ras(kern_hyp_va((*vcpu).kvm))
}

unsafe fn ctxt_has_sctlr2(ctxt: *mut kvm_cpu_context) -> bool {
    if !cpus_have_final_cap(ARM64_HAS_SCTLR2) {
        return false;
    }
    let vcpu = ctxt_to_vcpu(ctxt);
    kvm_has_sctlr2(kern_hyp_va((*vcpu).kvm))
}

unsafe fn __sysreg_save_el1_state(ctxt: *mut kvm_cpu_context) {
    *ctxt_sys_reg_mut(ctxt, SCTLR_EL1) = read_sysreg_el1(SYS_SCTLR);
    *ctxt_sys_reg_mut(ctxt, CPACR_EL1) = read_sysreg_el1(SYS_CPACR);
    *ctxt_sys_reg_mut(ctxt, TTBR0_EL1) = read_sysreg_el1(SYS_TTBR0);
    *ctxt_sys_reg_mut(ctxt, TTBR1_EL1) = read_sysreg_el1(SYS_TTBR1);
    *ctxt_sys_reg_mut(ctxt, TCR_EL1) = read_sysreg_el1(SYS_TCR);
    if ctxt_has_tcrx(ctxt) {
        *ctxt_sys_reg_mut(ctxt, TCR2_EL1) = read_sysreg_el1(SYS_TCR2);
        if ctxt_has_s1pie(ctxt) {
            *ctxt_sys_reg_mut(ctxt, PIR_EL1) = read_sysreg_el1(SYS_PIR);
            *ctxt_sys_reg_mut(ctxt, PIRE0_EL1) = read_sysreg_el1(SYS_PIRE0);
        }
        if ctxt_has_s1poe(ctxt) {
            *ctxt_sys_reg_mut(ctxt, POR_EL1) = read_sysreg_el1(SYS_POR);
        }
    }
    *ctxt_sys_reg_mut(ctxt, ESR_EL1) = read_sysreg_el1(SYS_ESR);
    *ctxt_sys_reg_mut(ctxt, AFSR0_EL1) = read_sysreg_el1(SYS_AFSR0);
    *ctxt_sys_reg_mut(ctxt, AFSR1_EL1) = read_sysreg_el1(SYS_AFSR1);
    *ctxt_sys_reg_mut(ctxt, FAR_EL1) = read_sysreg_el1(SYS_FAR);
    *ctxt_sys_reg_mut(ctxt, MAIR_EL1) = read_sysreg_el1(SYS_MAIR);
    *ctxt_sys_reg_mut(ctxt, VBAR_EL1) = read_sysreg_el1(SYS_VBAR);
    *ctxt_sys_reg_mut(ctxt, CONTEXTIDR_EL1) = read_sysreg_el1(SYS_CONTEXTIDR);
    *ctxt_sys_reg_mut(ctxt, AMAIR_EL1) = read_sysreg_el1(SYS_AMAIR);
    *ctxt_sys_reg_mut(ctxt, CNTKCTL_EL1) = read_sysreg_el1(SYS_CNTKCTL);
    *ctxt_sys_reg_mut(ctxt, PAR_EL1) = read_sysreg_par();
    *ctxt_sys_reg_mut(ctxt, TPIDR_EL1) = read_sysreg(tpidr_el1);
    if ctxt_has_mte(ctxt) {
        *ctxt_sys_reg_mut(ctxt, TFSR_EL1) = read_sysreg_el1(SYS_TFSR);
        *ctxt_sys_reg_mut(ctxt, TFSRE0_EL1) = read_sysreg_s(SYS_TFSRE0_EL1);
    }
    *ctxt_sys_reg_mut(ctxt, SP_EL1) = read_sysreg(sp_el1);
    *ctxt_sys_reg_mut(ctxt, ELR_EL1) = read_sysreg_el1(SYS_ELR);
    *ctxt_sys_reg_mut(ctxt, SPSR_EL1) = read_sysreg_el1(SYS_SPSR);
    if ctxt_has_sctlr2(ctxt) {
        *ctxt_sys_reg_mut(ctxt, SCTLR2_EL1) = read_sysreg_el1(SYS_SCTLR2);
    }
    /* Retrieve L2's HCR_EL2, and save it for future use */
    if is_nested_nv3_ctxt(ctxt_to_vcpu(ctxt)) {
        *ctxt_sys_reg_mut(ctxt, NVHCR_EL2) = read_sysreg_s(SYS_NVHCR_EL2);
    }
}

unsafe fn __sysreg_save_el2_return_state(ctxt: *mut kvm_cpu_context) {
    (*ctxt).regs.pc = read_sysreg_el2(SYS_ELR);
    /* Guest PSTATE gets saved at guest fixup time in all cases. */
    if !has_vhe() && !(*ctxt).__hyp_running_vcpu.is_null() {
        (*ctxt).regs.pstate = read_sysreg_el2(SYS_SPSR);
    }
    if !cpus_have_final_cap(ARM64_HAS_RAS_EXTN) {
        return;
    }
    if !vserror_state_is_nested(ctxt_to_vcpu(ctxt)) {
        *ctxt_sys_reg_mut(ctxt, DISR_EL1) = read_sysreg_s(SYS_VDISR_EL2);
    } else if ctxt_has_ras(ctxt) {
        *ctxt_sys_reg_mut(ctxt, VDISR_EL2) = read_sysreg_s(SYS_VDISR_EL2);
    }
}

unsafe fn __sysreg_restore_common_state(ctxt: *mut kvm_cpu_context) {
    write_sysreg(*ctxt_mdscr_el1(ctxt), mdscr_el1);
    // POR_EL0 can affect uaccess, so must be saved/restored early.
    if ctxt_has_s1poe(ctxt) {
        write_sysreg_s(*ctxt_sys_reg(ctxt, POR_EL0), SYS_POR_EL0);
    }
}

unsafe fn __sysreg_restore_user_state(ctxt: *mut kvm_cpu_context) {
    write_sysreg(*ctxt_sys_reg(ctxt, TPIDR_EL0), tpidr_el0);
    write_sysreg(*ctxt_sys_reg(ctxt, TPIDRRO_EL0), tpidrro_el0);
}

unsafe fn __sysreg_restore_el1_state(ctxt: *mut kvm_cpu_context, midr: u64, mpidr: u64) {
    write_sysreg(midr, vpidr_el2);
    write_sysreg(mpidr, vmpidr_el2);
    if has_vhe() || !cpus_have_final_cap(ARM64_WORKAROUND_SPECULATIVE_AT) {
        write_sysreg_el1(*ctxt_sys_reg(ctxt, SCTLR_EL1), SYS_SCTLR);
        write_sysreg_el1(*ctxt_sys_reg(ctxt, TCR_EL1), SYS_TCR);
    } else if (*ctxt).__hyp_running_vcpu.is_null() {
        write_sysreg_el1(*ctxt_sys_reg(ctxt, TCR_EL1) | TCR_EPD1_MASK | TCR_EPD0_MASK, SYS_TCR);
        isb();
    }
    write_sysreg_el1(*ctxt_sys_reg(ctxt, CPACR_EL1), SYS_CPACR);
    write_sysreg_el1(*ctxt_sys_reg(ctxt, TTBR0_EL1), SYS_TTBR0);
    write_sysreg_el1(*ctxt_sys_reg(ctxt, TTBR1_EL1), SYS_TTBR1);
    if ctxt_has_tcrx(ctxt) {
        write_sysreg_el1(*ctxt_sys_reg(ctxt, TCR2_EL1), SYS_TCR2);
        if ctxt_has_s1pie(ctxt) {
            write_sysreg_el1(*ctxt_sys_reg(ctxt, PIR_EL1), SYS_PIR);
            write_sysreg_el1(*ctxt_sys_reg(ctxt, PIRE0_EL1), SYS_PIRE0);
        }
        if ctxt_has_s1poe(ctxt) {
            write_sysreg_el1(*ctxt_sys_reg(ctxt, POR_EL1), SYS_POR);
        }
    }
    write_sysreg_el1(*ctxt_sys_reg(ctxt, ESR_EL1), SYS_ESR);
    write_sysreg_el1(*ctxt_sys_reg(ctxt, AFSR0_EL1), SYS_AFSR0);
    write_sysreg_el1(*ctxt_sys_reg(ctxt, AFSR1_EL1), SYS_AFSR1);
    write_sysreg_el1(*ctxt_sys_reg(ctxt, FAR_EL1), SYS_FAR);
    write_sysreg_el1(*ctxt_sys_reg(ctxt, MAIR_EL1), SYS_MAIR);
    write_sysreg_el1(*ctxt_sys_reg(ctxt, VBAR_EL1), SYS_VBAR);
    write_sysreg_el1(*ctxt_sys_reg(ctxt, CONTEXTIDR_EL1), SYS_CONTEXTIDR);
    write_sysreg_el1(*ctxt_sys_reg(ctxt, AMAIR_EL1), SYS_AMAIR);
    write_sysreg_el1(*ctxt_sys_reg(ctxt, CNTKCTL_EL1), SYS_CNTKCTL);
    write_sysreg(*ctxt_sys_reg(ctxt, PAR_EL1), par_el1);
    write_sysreg(*ctxt_sys_reg(ctxt, TPIDR_EL1), tpidr_el1);
    if ctxt_has_mte(ctxt) {
        write_sysreg_el1(*ctxt_sys_reg(ctxt, TFSR_EL1), SYS_TFSR);
        write_sysreg_s(*ctxt_sys_reg(ctxt, TFSRE0_EL1), SYS_TFSRE0_EL1);
    }
    if !has_vhe() && cpus_have_final_cap(ARM64_WORKAROUND_SPECULATIVE_AT)
        && !(*ctxt).__hyp_running_vcpu.is_null()
    {
        isb();
        write_sysreg_el1(*ctxt_sys_reg(ctxt, SCTLR_EL1), SYS_SCTLR);
        isb();
        write_sysreg_el1(*ctxt_sys_reg(ctxt, TCR_EL1), SYS_TCR);
    }
    write_sysreg(*ctxt_sys_reg(ctxt, SP_EL1), sp_el1);
    write_sysreg_el1(*ctxt_sys_reg(ctxt, ELR_EL1), SYS_ELR);
    write_sysreg_el1(*ctxt_sys_reg(ctxt, SPSR_EL1), SYS_SPSR);
    if ctxt_has_sctlr2(ctxt) {
        write_sysreg_el1(*ctxt_sys_reg(ctxt, SCTLR2_EL1), SYS_SCTLR2);
    }
    /* Publish the L2 view of HCR_EL2 to the HW if L1 is using NV3. */
    if is_nested_nv3_ctxt(ctxt_to_vcpu(ctxt)) {
        write_sysreg_s(*ctxt_sys_reg(ctxt, NVHCR_EL2), SYS_NVHCR_EL2);
    }
}

/* Read the VCPU state's PSTATE, but translate (v)EL2 to EL1. */
unsafe fn to_hw_pstate(ctxt: *const kvm_cpu_context) -> u64 {
    let mut mode = (*ctxt).regs.pstate & (PSR_MODE_MASK | PSR_MODE32_BIT);
    match mode {
        PSR_MODE_EL2t => mode = PSR_MODE_EL1t,
        PSR_MODE_EL2h => mode = PSR_MODE_EL1h,
        _ => {}
    }
    ((*ctxt).regs.pstate & !(PSR_MODE_MASK | PSR_MODE32_BIT)) | mode
}

unsafe fn __sysreg_restore_el2_return_state(ctxt: *mut kvm_cpu_context) {
    let mut pstate = to_hw_pstate(ctxt);
    let mode = pstate & PSR_AA32_MODE_MASK;
    let vdisr;
    if mode & PSR_MODE32_BIT == 0 && mode >= PSR_MODE_EL2t {
        pstate = PSR_MODE_EL2h | PSR_IL_BIT;
    }
    write_sysreg_el2((*ctxt).regs.pc, SYS_ELR);
    write_sysreg_el2(pstate, SYS_SPSR);
    if !cpus_have_final_cap(ARM64_HAS_RAS_EXTN) {
        return;
    }
    if !vserror_state_is_nested(ctxt_to_vcpu(ctxt)) {
        vdisr = *ctxt_sys_reg(ctxt, DISR_EL1);
    } else if ctxt_has_ras(ctxt) {
        vdisr = *ctxt_sys_reg(ctxt, VDISR_EL2);
    } else {
        vdisr = 0;
    }
    write_sysreg_s(vdisr, SYS_VDISR_EL2);
}

unsafe fn __sysreg32_save_state(vcpu: *mut kvm_vcpu) {
    if !vcpu_el1_is_32bit(vcpu) { return; }
    (*vcpu).arch.ctxt.spsr_abt = read_sysreg(spsr_abt);
    (*vcpu).arch.ctxt.spsr_und = read_sysreg(spsr_und);
    (*vcpu).arch.ctxt.spsr_irq = read_sysreg(spsr_irq);
    (*vcpu).arch.ctxt.spsr_fiq = read_sysreg(spsr_fiq);
    __vcpu_assign_sys_reg(vcpu, DACR32_EL2, read_sysreg(dacr32_el2));
    __vcpu_assign_sys_reg(vcpu, IFSR32_EL2, read_sysreg(ifsr32_el2));
    if has_vhe() || kvm_debug_regs_in_use(vcpu) {
        __vcpu_assign_sys_reg(vcpu, DBGVCR32_EL2, read_sysreg(dbgvcr32_el2));
    }
}

unsafe fn __sysreg32_restore_state(vcpu: *mut kvm_vcpu) {
    if !vcpu_el1_is_32bit(vcpu) { return; }
    write_sysreg((*vcpu).arch.ctxt.spsr_abt, spsr_abt);
    write_sysreg((*vcpu).arch.ctxt.spsr_und, spsr_und);
    write_sysreg((*vcpu).arch.ctxt.spsr_irq, spsr_irq);
    write_sysreg((*vcpu).arch.ctxt.spsr_fiq, spsr_fiq);
    write_sysreg(__vcpu_sys_reg(vcpu, DACR32_EL2), dacr32_el2);
    write_sysreg(__vcpu_sys_reg(vcpu, IFSR32_EL2), ifsr32_el2);
    if has_vhe() || kvm_debug_regs_in_use(vcpu) {
        write_sysreg(__vcpu_sys_reg(vcpu, DBGVCR32_EL2), dbgvcr32_el2);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
