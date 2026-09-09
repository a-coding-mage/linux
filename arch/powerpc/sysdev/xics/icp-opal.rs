// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2016 IBM Corporation.
 */

unsafe fn icp_opal_teardown_cpu() {
    let hw_cpu: i32 = hard_smp_processor_id();

    /* Clear any pending IPI */
    opal_int_set_mfrr(hw_cpu, 0xff);
}

unsafe fn icp_opal_flush_ipi() {
    /*
     * We take the ipi irq but and never return so we need to EOI the IPI,
     * but want to leave our priority 0.
     *
     * Should we check all the other interrupts too?
     * Should we be flagging idle loop instead?
     * Or creating some task to be scheduled?
     */
    if opal_int_eoi((0x00 << 24) | XICS_IPI) > 0 {
        force_external_irq_replay();
    }
}

unsafe fn icp_opal_get_xirr() -> u32 {
    let mut kvm_xirr: u32;
    let mut hw_xirr: __be32 = 0;
    let rc: i64;

    /* Handle an interrupt latched by KVM first */
    kvm_xirr = kvmppc_get_xics_latch();
    if kvm_xirr != 0 {
        return kvm_xirr;
    }

    /* Then ask OPAL */
    rc = opal_int_get_xirr(&mut hw_xirr, false);
    if rc < 0 {
        return 0;
    }
    be32_to_cpu(hw_xirr)
}

unsafe fn icp_opal_get_irq() -> u32 {
    let xirr: u32 = icp_opal_get_xirr();
    let vec: u32 = xirr & 0x00ffffff;
    if vec == XICS_IRQ_SPURIOUS {
        return 0;
    }

    let irq: u32 = irq_find_mapping(xics_host, vec);
    if likely(irq != 0) {
        xics_push_cppr(vec);
        return irq;
    }

    /* We don't have a linux mapping, so have rtas mask it. */
    xics_mask_unknown_vec(vec);

    /* We might learn about it later, so EOI it */
    if opal_int_eoi(xirr) > 0 {
        force_external_irq_replay();
    }

    0
}

unsafe fn icp_opal_set_cpu_priority(mut cppr: u8) {
    /*
     * Here be dragons. The caller has asked to allow only IPI's and not
     * external interrupts. But OPAL XIVE doesn't support that. So instead
     * of allowing no interrupts allow all. That's still not right, but
     * currently the only caller who does this is xics_migrate_irqs_away()
     * and it works in that case.
     */
    if cppr >= DEFAULT_PRIORITY {
        cppr = LOWEST_PRIORITY;
    }

    xics_set_base_cppr(cppr);
    opal_int_set_cppr(cppr);
    iosync();
}

unsafe fn icp_opal_eoi(d: *mut irq_data) {
    let hw_irq: u32 = irqd_to_hwirq(d) as u32;
    let rc: i64;

    iosync();
    rc = opal_int_eoi((xics_pop_cppr() << 24) | hw_irq);

    /*
     * EOI tells us whether there are more interrupts to fetch.
     *
     * Some HW implementations might not be able to send us another
     * external interrupt in that case, so we force a replay.
     */
    if rc > 0 {
        force_external_irq_replay();
    }
}

#[cfg(CONFIG_SMP)]
unsafe fn icp_opal_cause_ipi(cpu: i32) {
    let hw_cpu: i32 = get_hard_smp_processor_id(cpu);

    kvmppc_set_host_ipi(cpu);
    opal_int_set_mfrr(hw_cpu, IPI_PRIORITY);
}

#[cfg(CONFIG_SMP)]
unsafe fn icp_opal_ipi_action(_irq: i32, _dev_id: *mut core::ffi::c_void) -> irqreturn_t {
    let cpu: i32 = smp_processor_id();

    kvmppc_clear_host_ipi(cpu);
    opal_int_set_mfrr(get_hard_smp_processor_id(cpu), 0xff);

    smp_ipi_demux()
}

/*
 * Called when an interrupt is received on an off-line CPU to
 * clear the interrupt, so that the CPU can go back to nap mode.
 */
#[cfg(CONFIG_SMP)]
pub unsafe fn icp_opal_flush_interrupt() {
    let mut xirr: u32;
    let mut vec: u32;

    loop {
        xirr = icp_opal_get_xirr();
        vec = xirr & 0x00ffffff;
        if vec == XICS_IRQ_SPURIOUS {
            break;
        }
        if vec == XICS_IPI {
            /* Clear pending IPI */
            let cpu: i32 = smp_processor_id();
            kvmppc_clear_host_ipi(cpu);
            opal_int_set_mfrr(get_hard_smp_processor_id(cpu), 0xff);
        } else {
            pr_err!("XICS: hw interrupt 0x{:x} to offline cpu, disabling\n", vec);
            xics_mask_unknown_vec(vec);
        }

        /* EOI the interrupt */
        if opal_int_eoi(xirr) <= 0 {
            break;
        }
    }
}

static icp_opal_ops: icp_ops = icp_ops {
    get_irq: icp_opal_get_irq,
    eoi: icp_opal_eoi,
    set_priority: icp_opal_set_cpu_priority,
    teardown_cpu: icp_opal_teardown_cpu,
    flush_ipi: icp_opal_flush_ipi,
    #[cfg(CONFIG_SMP)]
    ipi_action: icp_opal_ipi_action,
    #[cfg(CONFIG_SMP)]
    cause_ipi: icp_opal_cause_ipi,
};

pub unsafe fn icp_opal_init() -> i32 {
    let np: *mut device_node;

    np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "ibm,opal-intc");
    if np.is_null() {
        return -ENODEV;
    }

    icp_ops = &icp_opal_ops;

    printk!("XICS: Using OPAL ICP fallbacks\n");

    of_node_put(np);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
