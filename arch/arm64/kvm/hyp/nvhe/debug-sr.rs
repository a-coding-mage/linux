// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 */

unsafe fn __debug_save_spe() {
    let pmscr_el1: *mut u64;
    let pmblimitr_el1: *mut u64;

    pmscr_el1 = host_data_ptr(host_debug_state.pmscr_el1);
    pmblimitr_el1 = host_data_ptr(host_debug_state.pmblimitr_el1);

    /*
     * At this point, we know that this CPU implements
     * SPE and is available to the host.
     * Check if the host is actually using it ?
     */
    *pmblimitr_el1 = read_sysreg_s(SYS_PMBLIMITR_EL1);
    if (*pmblimitr_el1 & BIT(PMBLIMITR_EL1_E_SHIFT)) == 0 {
        return;
    }

    /* Yes; save the control register and disable data generation */
    *pmscr_el1 = read_sysreg_el1(SYS_PMSCR);
    write_sysreg_el1(0, SYS_PMSCR);
    isb();

    /* Now drain all buffered data to memory */
    psb_csync();
    dsb(nsh);

    /* And disable the profiling buffer */
    write_sysreg_s(0, SYS_PMBLIMITR_EL1);
    isb();
}

unsafe fn __debug_restore_spe() {
    let pmblimitr_el1: u64 = *host_data_ptr(host_debug_state.pmblimitr_el1);

    if (pmblimitr_el1 & BIT(PMBLIMITR_EL1_E_SHIFT)) == 0 {
        return;
    }

    /* The host page table is installed, but not yet synchronised */
    isb();

    /* Re-enable the profiling buffer. */
    write_sysreg_s(pmblimitr_el1, SYS_PMBLIMITR_EL1);
    isb();

    /* Re-enable data generation */
    write_sysreg_el1(*host_data_ptr(host_debug_state.pmscr_el1), SYS_PMSCR);
}

unsafe fn __trace_do_switch(saved_trfcr: *mut u64, new_trfcr: u64) {
    *saved_trfcr = read_sysreg_el1(SYS_TRFCR);
    write_sysreg_el1(new_trfcr, SYS_TRFCR);
}

unsafe fn __trace_drain_and_disable() {
    let trblimitr_el1: *mut u64 = host_data_ptr(host_debug_state.trblimitr_el1);
    let needs_drain: bool = if is_protected_kvm_enabled() {
        host_data_test_flag(HAS_TRBE)
    } else {
        host_data_test_flag(TRBE_ENABLED)
    };

    if !needs_drain {
        *trblimitr_el1 = 0;
        return;
    }

    *trblimitr_el1 = read_sysreg_s(SYS_TRBLIMITR_EL1);
    if *trblimitr_el1 & TRBLIMITR_EL1_E != 0 {
        /*
         * The host has enabled the Trace Buffer Unit so we have
         * to beat the CPU with a stick until it stops accessing
         * memory.
         */

        /* First, ensure that our prior write to TRFCR has stuck. */
        isb();

        /* Now synchronise with the trace and drain the buffer. */
        tsb_csync();
        dsb(nsh);

        /*
         * With no more trace being generated, we can disable the
         * Trace Buffer Unit.
         */
        write_sysreg_s(0, SYS_TRBLIMITR_EL1);
        if cpus_have_final_cap(ARM64_WORKAROUND_2064142) {
            /*
             * Some CPUs are so good, we have to drain 'em
             * twice.
             */
            tsb_csync();
            dsb(nsh);
        }

        /*
         * Ensure that the Trace Buffer Unit is disabled before
         * we start mucking with the stage-2 and trap
         * configuration.
         */
        isb();
    }
}

unsafe fn __trace_needs_switch() -> bool {
    host_data_test_flag(TRBE_ENABLED) || host_data_test_flag(EL1_TRACING_CONFIGURED)
}

unsafe fn __trace_switch_to_guest() {
    /* Unsupported with TRBE so disable */
    if host_data_test_flag(TRBE_ENABLED) {
        *host_data_ptr(trfcr_while_in_guest) = 0;
    }

    __trace_do_switch(
        host_data_ptr(host_debug_state.trfcr_el1),
        *host_data_ptr(trfcr_while_in_guest),
    );
    __trace_drain_and_disable();
}

unsafe fn __trace_switch_to_host() {
    let trblimitr_el1: u64 = *host_data_ptr(host_debug_state.trblimitr_el1);

    if trblimitr_el1 & TRBLIMITR_EL1_E != 0 {
        /* Re-enable the Trace Buffer Unit for the host. */
        write_sysreg_s(trblimitr_el1, SYS_TRBLIMITR_EL1);
        isb();
        if cpus_have_final_cap(ARM64_WORKAROUND_2038923) {
            /*
             * Make sure the unit is re-enabled before we
             * poke TRFCR.
             */
            isb();
        }
    }

    __trace_do_switch(
        host_data_ptr(trfcr_while_in_guest),
        *host_data_ptr(host_debug_state.trfcr_el1),
    );
}

unsafe fn __debug_save_brbe() {
    let brbcr_el1: *mut u64 = host_data_ptr(host_debug_state.brbcr_el1);

    *brbcr_el1 = 0;

    /* Check if the BRBE is enabled */
    if read_sysreg_el1(SYS_BRBCR) & (BRBCR_ELx_E0BRE | BRBCR_ELx_ExBRE) == 0 {
        return;
    }

    /*
     * Prohibit branch record generation while we are in guest.
     * Since access to BRBCR_EL1 is trapped, the guest can't
     * modify the filtering set by the host.
     */
    *brbcr_el1 = read_sysreg_el1(SYS_BRBCR);
    write_sysreg_el1(0, SYS_BRBCR);
}

unsafe fn __debug_restore_brbe() {
    let brbcr_el1: u64 = *host_data_ptr(host_debug_state.brbcr_el1);

    if brbcr_el1 == 0 {
        return;
    }

    /* Restore BRBE controls */
    write_sysreg_el1(brbcr_el1, SYS_BRBCR);
}

pub unsafe fn __debug_save_host_buffers_nvhe(vcpu: *mut kvm_vcpu) {
    /* Disable and flush SPE data generation */
    if host_data_test_flag(HAS_SPE) {
        __debug_save_spe();
    }

    /* Disable BRBE branch records */
    if host_data_test_flag(HAS_BRBE) {
        __debug_save_brbe();
    }

    if __trace_needs_switch() {
        __trace_switch_to_guest();
    }
}

pub unsafe fn __debug_switch_to_guest(vcpu: *mut kvm_vcpu) {
    __debug_switch_to_guest_common(vcpu);
}

pub unsafe fn __debug_restore_host_buffers_nvhe(vcpu: *mut kvm_vcpu) {
    if host_data_test_flag(HAS_SPE) {
        __debug_restore_spe();
    }
    if host_data_test_flag(HAS_BRBE) {
        __debug_restore_brbe();
    }
    if __trace_needs_switch() {
        __trace_switch_to_host();
    }
}

pub unsafe fn __debug_switch_to_host(vcpu: *mut kvm_vcpu) {
    __debug_switch_to_host_common(vcpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
