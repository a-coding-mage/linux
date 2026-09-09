// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

// Dependencies supplied by the surrounding kernel translation unit:
// asm/kvm_asm.h, asm/kvm_emulate.h, asm/kvm_hyp.h, and asm/kvm_mmu.h.

unsafe fn __fault_safe_to_translate(esr: u64) -> bool {
    let fsc = esr & ESR_ELx_FSC;

    if esr_fsc_is_sea_ttw(esr) || esr_fsc_is_secc_ttw(esr) {
        return false;
    }

    !(fsc == ESR_ELx_FSC_EXTABT && (esr & ESR_ELx_FnV) != 0)
}

unsafe fn __translate_far_to_hpfar(far: u64, hpfar: *mut u64) -> bool {
    let ret: i32;
    let par: u64;
    let tmp: u64;

    /*
     * Resolve the IPA the hard way using the guest VA.
     *
     * Stage-1 translation already validated the memory access
     * rights. As such, we can use the EL1 translation regime, and
     * don't have to distinguish between EL0 and EL1 access.
     *
     * We do need to save/restore PAR_EL1 though, as we haven't
     * saved the guest context yet, and we may return early...
     */
    par = read_sysreg_par();
    ret = if system_supports_poe() {
        __kvm_at(OP_AT_S1E1A, far)
    } else {
        __kvm_at(OP_AT_S1E1R, far)
    };
    tmp = if ret == 0 {
        read_sysreg_par()
    } else {
        SYS_PAR_EL1_F /* back to the guest */
    };
    write_sysreg(par, par_el1);

    if (tmp & SYS_PAR_EL1_F) != 0 {
        return false; /* Translation failed, back to guest */
    }

    /* Convert PAR to HPFAR format */
    *hpfar = PAR_TO_HPFAR(tmp);
    true
}

/*
 * Checks for the conditions when HPFAR_EL2 is written, per ARM ARM R_FKLWR.
 */
unsafe fn __hpfar_valid(esr: u64) -> bool {
    /*
     * CPUs affected by ARM erratum #834220 may incorrectly report
     * a stage-2 translation fault when a stage-1 permission fault occurs.
     *
     * Re-walk the page tables to determine if a stage-1 fault actually
     * occurred.
     */
    if cpus_have_final_cap(ARM64_WORKAROUND_834220)
        && esr_fsc_is_translation_fault(esr)
    {
        return false;
    }

    if esr_fsc_is_translation_fault(esr) || esr_fsc_is_access_flag_fault(esr) {
        return true;
    }

    if (esr & ESR_ELx_S1PTW) != 0 && esr_fsc_is_permission_fault(esr) {
        return true;
    }

    esr_fsc_is_addr_sz_fault(esr)
}

unsafe fn __get_fault_info(esr: u64, fault: *mut kvm_vcpu_fault_info) -> bool {
    let hpfar: u64;

    (*fault).far_el2 = read_sysreg_el2(SYS_FAR);
    (*fault).hpfar_el2 = 0;

    if __hpfar_valid(esr) {
        hpfar = read_sysreg(hpfar_el2);
    } else if !__fault_safe_to_translate(esr) {
        return true;
    } else if !__translate_far_to_hpfar((*fault).far_el2, &mut hpfar as *mut u64) {
        return false;
    }

    /*
     * Hijack HPFAR_EL2.NS (RES0 in Non-secure) to indicate a valid
     * HPFAR value.
     */
    (*fault).hpfar_el2 = hpfar | HPFAR_EL2_NS;
    true
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
