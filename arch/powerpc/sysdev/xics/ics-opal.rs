// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ICS backend for OPAL managed interrupts.
 *
 * Copyright 2011 IBM Corp.
 */

// #include dependencies are supplied by the surrounding kernel translation.

unsafe fn ics_opal_mangle_server(server: i32) -> i32 {
    /* No link for now */
    server << 2
}

unsafe fn ics_opal_unmangle_server(server: i32) -> i32 {
    /* No link for now */
    server >> 2
}

unsafe fn ics_opal_unmask_irq(d: *mut irq_data) {
    let hw_irq = irqd_to_hwirq(d) as u32;
    let mut rc: i64;
    let mut server: i32;

    pr_devel!("ics-hal: unmask virq %d [hw 0x%x]\n", (*d).irq, hw_irq);

    if hw_irq == XICS_IPI || hw_irq == XICS_IRQ_SPURIOUS { return; }

    server = xics_get_irq_server((*d).irq, irq_data_get_affinity_mask(d), 0);
    server = ics_opal_mangle_server(server);

    rc = opal_set_xive(hw_irq, server, DEFAULT_PRIORITY);
    if rc != OPAL_SUCCESS {
        pr_err!("{}: opal_set_xive(irq={} [hw 0x{:x}] server={:x}) error {}\n",
                "ics_opal_unmask_irq", (*d).irq, hw_irq, server, rc);
    }
}

unsafe fn ics_opal_startup(d: *mut irq_data) -> u32 {
    ics_opal_unmask_irq(d);
    0
}

unsafe fn ics_opal_mask_real_irq(hw_irq: u32) {
    let server = ics_opal_mangle_server(xics_default_server);
    let rc: i64;

    if hw_irq == XICS_IPI { return; }

    /* Have to set XIVE to 0xff to be able to remove a slot */
    rc = opal_set_xive(hw_irq, server, 0xff);
    if rc != OPAL_SUCCESS {
        pr_err!("{}: opal_set_xive(0xff) irq={} returned {}\n",
                "ics_opal_mask_real_irq", hw_irq, rc);
    }
}

unsafe fn ics_opal_mask_irq(d: *mut irq_data) {
    let hw_irq = irqd_to_hwirq(d) as u32;

    pr_devel!("ics-hal: mask virq {} [hw 0x{:x}]\n", (*d).irq, hw_irq);

    if hw_irq == XICS_IPI || hw_irq == XICS_IRQ_SPURIOUS { return; }
    ics_opal_mask_real_irq(hw_irq);
}

unsafe fn ics_opal_set_affinity(d: *mut irq_data, cpumask: *const cpumask, _force: bool) -> i32 {
    let hw_irq = irqd_to_hwirq(d) as u32;
    let mut oserver: __be16 = 0;
    let mut server: i16;
    let mut priority: i8 = 0;
    let mut rc: i64;
    let wanted_server: i32;

    if hw_irq == XICS_IPI || hw_irq == XICS_IRQ_SPURIOUS { return -1; }

    rc = opal_get_xive(hw_irq, &mut oserver, &mut priority);
    if rc != OPAL_SUCCESS {
        pr_err!("{}: opal_get_xive(irq={} [hw 0x{:x}]) error {}\n",
                "ics_opal_set_affinity", (*d).irq, hw_irq, rc);
        return -1;
    }

    wanted_server = xics_get_irq_server((*d).irq, cpumask, 1);
    if wanted_server < 0 {
        pr_warn!("{}: No online cpus in the mask for irq {}\n",
                 "ics_opal_set_affinity", (*d).irq);
        return -1;
    }
    server = ics_opal_mangle_server(wanted_server) as i16;

    pr_debug!("ics-hal: set-affinity irq {} [hw 0x{:x}] server: 0x{:x}/0x{:x}\n",
              (*d).irq, hw_irq, wanted_server, server);

    rc = opal_set_xive(hw_irq, server, priority);
    if rc != OPAL_SUCCESS {
        pr_err!("{}: opal_set_xive(irq={} [hw 0x{:x}] server={:x}) error {}\n",
                "ics_opal_set_affinity", (*d).irq, hw_irq, server, rc);
        return -1;
    }
    IRQ_SET_MASK_OK
}

static mut ics_opal_irq_chip: irq_chip = irq_chip {
    name: "OPAL ICS",
    irq_startup: Some(ics_opal_startup),
    irq_mask: Some(ics_opal_mask_irq),
    irq_unmask: Some(ics_opal_unmask_irq),
    irq_eoi: None, /* Patched at init time */
    irq_set_affinity: Some(ics_opal_set_affinity),
    irq_set_type: Some(xics_set_irq_type),
    irq_retrigger: Some(xics_retrigger),
};

unsafe fn ics_opal_host_match(_ics: *mut ics, _node: *mut device_node) -> i32 { 1 }

unsafe fn ics_opal_check(_ics: *mut ics, hw_irq: u32) -> i32 {
    let mut server: __be16 = 0;
    let mut priority: i8 = 0;
    if WARN_ON!(hw_irq == XICS_IPI || hw_irq == XICS_IRQ_SPURIOUS) { return -EINVAL; }
    /* Check if HAL knows about this interrupt */
    if opal_get_xive(hw_irq, &mut server, &mut priority) != OPAL_SUCCESS { return -ENXIO; }
    0
}

unsafe fn ics_opal_mask_unknown(_ics: *mut ics, vec: u64) {
    let mut server: __be16 = 0;
    let mut priority: i8 = 0;
    /* Check if HAL knows about this interrupt */
    if opal_get_xive(vec as u32, &mut server, &mut priority) != OPAL_SUCCESS { return; }
    ics_opal_mask_real_irq(vec as u32);
}

unsafe fn ics_opal_get_server(_ics: *mut ics, vec: u64) -> i64 {
    let mut server: __be16 = 0;
    let mut priority: i8 = 0;
    /* Check if HAL knows about this interrupt */
    if opal_get_xive(vec as u32, &mut server, &mut priority) != OPAL_SUCCESS { return -1; }
    ics_opal_unmangle_server(be16_to_cpu(server) as i32) as i64
}

/* Only one global & state struct ics */
static mut ics_hal: ics = ics {
    check: Some(ics_opal_check),
    mask_unknown: Some(ics_opal_mask_unknown),
    get_server: Some(ics_opal_get_server),
    host_match: Some(ics_opal_host_match),
    chip: &mut ics_opal_irq_chip,
};

unsafe fn ics_opal_init() -> i32 {
    if !firmware_has_feature(FW_FEATURE_OPAL) { return -ENODEV; }

    /* We need to patch our irq chip's EOI to point to the
     * right ICP
     */
    ics_opal_irq_chip.irq_eoi = Some((*icp_ops).eoi);

    /* Register ourselves */
    xics_register_ics(&mut ics_hal);
    pr_info!("ICS OPAL backend registered\n");
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
