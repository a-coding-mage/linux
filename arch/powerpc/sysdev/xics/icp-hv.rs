// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2011 IBM Corporation.
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented in this file.

#[inline]
unsafe fn icp_hv_get_xirr(cppr: u8) -> u32 {
    let mut retbuf: [c_ulong; PLPAR_HCALL_BUFSIZE] = [0; PLPAR_HCALL_BUFSIZE];
    let rc: c_long;
    let mut ret: u32 = XICS_IRQ_SPURIOUS;

    rc = plpar_hcall(H_XIRR, retbuf.as_mut_ptr(), cppr);
    if rc == H_SUCCESS {
        ret = retbuf[0] as u32;
    } else {
        pr_err!("{}: bad return code xirr cppr=0x{:x} returned {}\n", "icp_hv_get_xirr", cppr, rc);
        WARN_ON_ONCE!(1);
    }

    ret
}

#[inline]
unsafe fn icp_hv_set_cppr(value: u8) {
    let rc: c_long = plpar_hcall_norets(H_CPPR, value);
    if rc != H_SUCCESS {
        pr_err!("{}: bad return code cppr cppr=0x{:x} returned {}\n", "icp_hv_set_cppr", value, rc);
        WARN_ON_ONCE!(1);
    }
}

#[inline]
unsafe fn icp_hv_set_xirr(value: u32) {
    let rc: c_long = plpar_hcall_norets(H_EOI, value);
    if rc != H_SUCCESS {
        pr_err!("{}: bad return code eoi xirr=0x{:x} returned {}\n", "icp_hv_set_xirr", value, rc);
        WARN_ON_ONCE!(1);
        icp_hv_set_cppr((value >> 24) as u8);
    }
}

#[inline]
unsafe fn icp_hv_set_qirr(n_cpu: c_int, value: u8) {
    let hw_cpu: c_int = get_hard_smp_processor_id(n_cpu);

    /* Make sure all previous accesses are ordered before IPI sending */
    mb();
    let rc: c_long = plpar_hcall_norets(H_IPI, hw_cpu, value);
    if rc != H_SUCCESS {
        pr_err!("{}: bad return code qirr cpu={} hw_cpu={} mfrr=0x{:x} returned {}\n", "icp_hv_set_qirr", n_cpu, hw_cpu, value, rc);
        WARN_ON_ONCE!(1);
    }
}

unsafe fn icp_hv_eoi(d: *mut irq_data) {
    let hw_irq: u32 = irqd_to_hwirq(d) as u32;

    iosync();
    icp_hv_set_xirr((xics_pop_cppr() << 24) | hw_irq);
}

unsafe fn icp_hv_teardown_cpu() {
    let cpu: c_int = smp_processor_id();

    /* Clear any pending IPI */
    icp_hv_set_qirr(cpu, 0xff);
}

unsafe fn icp_hv_flush_ipi() {
    /* We take the ipi irq but and never return so we
     * need to EOI the IPI, but want to leave our priority 0
     *
     * should we check all the other interrupts too?
     * should we be flagging idle loop instead?
     * or creating some task to be scheduled?
     */

    icp_hv_set_xirr((0x00 << 24) | XICS_IPI);
}

unsafe fn icp_hv_get_irq() -> c_uint {
    let xirr: u32 = icp_hv_get_xirr(xics_cppr_top());
    let vec: u32 = xirr & 0x00ffffff;
    let irq: c_uint;

    if vec == XICS_IRQ_SPURIOUS {
        return 0;
    }

    irq = irq_find_mapping(xics_host, vec);
    if likely!(irq != 0) {
        xics_push_cppr(vec);
        return irq;
    }

    /* We don't have a linux mapping, so have rtas mask it. */
    xics_mask_unknown_vec(vec);

    /* We might learn about it later, so EOI it */
    icp_hv_set_xirr(xirr);

    0
}

unsafe fn icp_hv_set_cpu_priority(cppr: u8) {
    xics_set_base_cppr(cppr);
    icp_hv_set_cppr(cppr);
    iosync();
}

#[cfg(CONFIG_SMP)]
unsafe fn icp_hv_cause_ipi(cpu: c_int) {
    icp_hv_set_qirr(cpu, IPI_PRIORITY);
}

#[cfg(CONFIG_SMP)]
unsafe fn icp_hv_ipi_action(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    let cpu: c_int = smp_processor_id();

    icp_hv_set_qirr(cpu, 0xff);

    smp_ipi_demux()
}

static icp_hv_ops: icp_ops = icp_ops {
    get_irq: Some(icp_hv_get_irq),
    eoi: Some(icp_hv_eoi),
    set_priority: Some(icp_hv_set_cpu_priority),
    teardown_cpu: Some(icp_hv_teardown_cpu),
    flush_ipi: Some(icp_hv_flush_ipi),
    #[cfg(CONFIG_SMP)]
    ipi_action: Some(icp_hv_ipi_action),
    #[cfg(CONFIG_SMP)]
    cause_ipi: Some(icp_hv_cause_ipi),
};

#[init]
unsafe fn icp_hv_init() -> c_int {
    let mut np: *mut device_node;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), c"ibm,ppc-xicp".as_ptr());
    if np.is_null() {
        np = of_find_node_by_type(core::ptr::null_mut(), c"PowerPC-External-Interrupt-Presentation".as_ptr());
    }
    if np.is_null() {
        return -ENODEV;
    }

    icp_ops = &icp_hv_ops;

    of_node_put(np);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
