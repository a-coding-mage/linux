// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * arch/powerpc/sysdev/uic.c
 *
 * IBM PowerPC 4xx Universal Interrupt Controller
 *
 * Copyright 2007 David Gibson <dwg@au1.ibm.com>, IBM Corporation.
 */

// Linux and PowerPC dependencies supplied by the surrounding kernel.

const NR_UIC_INTS: u32 = 32;

const UIC_SR: i32 = 0x0;
const UIC_ER: i32 = 0x2;
const UIC_CR: i32 = 0x3;
const UIC_PR: i32 = 0x4;
const UIC_TR: i32 = 0x5;
const UIC_MSR: i32 = 0x6;
const UIC_VR: i32 = 0x7;
const UIC_VCR: i32 = 0x8;

static mut primary_uic: *mut uic = core::ptr::null_mut();

#[repr(C)]
struct uic {
    index: i32,
    dcrbase: i32,
    lock: raw_spinlock_t,
    /* The remapper for this UIC */
    irqhost: *mut irq_domain,
}

unsafe fn uic_unmask_irq(d: *mut irq_data) {
    let uic = irq_data_get_irq_chip_data(d);
    let src: u32 = irqd_to_hwirq(d);
    let mut flags: usize = 0;
    let sr: u32 = 1u32 << (31 - src);
    raw_spin_lock_irqsave(&mut (*uic).lock, &mut flags);
    /* ack level-triggered interrupts here */
    if irqd_is_level_type(d) {
        mtdcr((*uic).dcrbase + UIC_SR, sr);
    }
    let mut er: u32 = mfdcr((*uic).dcrbase + UIC_ER);
    er |= sr;
    mtdcr((*uic).dcrbase + UIC_ER, er);
    raw_spin_unlock_irqrestore(&mut (*uic).lock, flags);
}

unsafe fn uic_mask_irq(d: *mut irq_data) {
    let uic = irq_data_get_irq_chip_data(d);
    let src: u32 = irqd_to_hwirq(d);
    let mut flags: usize = 0;
    raw_spin_lock_irqsave(&mut (*uic).lock, &mut flags);
    let mut er: u32 = mfdcr((*uic).dcrbase + UIC_ER);
    er &= !(1u32 << (31 - src));
    mtdcr((*uic).dcrbase + UIC_ER, er);
    raw_spin_unlock_irqrestore(&mut (*uic).lock, flags);
}

unsafe fn uic_ack_irq(d: *mut irq_data) {
    let uic = irq_data_get_irq_chip_data(d);
    let src: u32 = irqd_to_hwirq(d);
    let mut flags: usize = 0;
    raw_spin_lock_irqsave(&mut (*uic).lock, &mut flags);
    mtdcr((*uic).dcrbase + UIC_SR, 1u32 << (31 - src));
    raw_spin_unlock_irqrestore(&mut (*uic).lock, flags);
}

unsafe fn uic_mask_ack_irq(d: *mut irq_data) {
    let uic = irq_data_get_irq_chip_data(d);
    let src: u32 = irqd_to_hwirq(d);
    let mut flags: usize = 0;
    let sr: u32 = 1u32 << (31 - src);
    raw_spin_lock_irqsave(&mut (*uic).lock, &mut flags);
    let mut er: u32 = mfdcr((*uic).dcrbase + UIC_ER);
    er &= !sr;
    mtdcr((*uic).dcrbase + UIC_ER, er);
    /* On the UIC, acking (i.e. clearing the SR bit)
     * a level irq will have no effect if the interrupt
     * is still asserted by the device, even if
     * the interrupt is already masked. Therefore
     * we only ack the egde interrupts here, while
     * level interrupts are ack'ed after the actual
     * isr call in the uic_unmask_irq()
     */
    if !irqd_is_level_type(d) {
        mtdcr((*uic).dcrbase + UIC_SR, sr);
    }
    raw_spin_unlock_irqrestore(&mut (*uic).lock, flags);
}

unsafe fn uic_set_irq_type(d: *mut irq_data, flow_type: u32) -> i32 {
    let uic = irq_data_get_irq_chip_data(d);
    let src: u32 = irqd_to_hwirq(d);
    let mut flags: usize = 0;
    let (trigger, polarity): (u32, u32);
    match flow_type & IRQ_TYPE_SENSE_MASK {
        IRQ_TYPE_NONE => { uic_mask_irq(d); return 0; }
        IRQ_TYPE_EDGE_RISING => { trigger = 1; polarity = 1; }
        IRQ_TYPE_EDGE_FALLING => { trigger = 1; polarity = 0; }
        IRQ_TYPE_LEVEL_HIGH => { trigger = 0; polarity = 1; }
        IRQ_TYPE_LEVEL_LOW => { trigger = 0; polarity = 0; }
        _ => return -EINVAL,
    }
    let mask: u32 = !(1u32 << (31 - src));
    raw_spin_lock_irqsave(&mut (*uic).lock, &mut flags);
    let mut tr: u32 = mfdcr((*uic).dcrbase + UIC_TR);
    let mut pr: u32 = mfdcr((*uic).dcrbase + UIC_PR);
    tr = (tr & mask) | (trigger << (31 - src));
    pr = (pr & mask) | (polarity << (31 - src));
    mtdcr((*uic).dcrbase + UIC_PR, pr);
    mtdcr((*uic).dcrbase + UIC_TR, tr);
    mtdcr((*uic).dcrbase + UIC_SR, !mask);
    raw_spin_unlock_irqrestore(&mut (*uic).lock, flags);
    0
}

static mut uic_irq_chip: irq_chip = irq_chip {
    name: "UIC",
    irq_unmask: Some(uic_unmask_irq),
    irq_mask: Some(uic_mask_irq),
    irq_mask_ack: Some(uic_mask_ack_irq),
    irq_ack: Some(uic_ack_irq),
    irq_set_type: Some(uic_set_irq_type),
};

unsafe fn uic_host_map(h: *mut irq_domain, virq: u32, _hw: irq_hw_number_t) -> i32 {
    let uic = (*h).host_data as *mut uic;
    irq_set_chip_data(virq, uic);
    /* Despite the name, handle_level_irq() works for both level
     * and edge irqs on UIC.  FIXME: check this is correct */
    irq_set_chip_and_handler(virq, &mut uic_irq_chip, handle_level_irq);
    /* Set default irq type */
    irq_set_irq_type(virq, IRQ_TYPE_NONE);
    0
}

static uic_host_ops: irq_domain_ops = irq_domain_ops {
    map: Some(uic_host_map),
    xlate: Some(irq_domain_xlate_twocell),
};

unsafe fn uic_irq_cascade(desc: *mut irq_desc) {
    let chip = irq_desc_get_chip(desc);
    let idata = irq_desc_get_irq_data(desc);
    let uic = irq_desc_get_handler_data(desc) as *mut uic;
    let mut msr: u32;
    let src: i32;
    raw_spin_lock(&mut (*desc).lock);
    if irqd_is_level_type(idata) { ((*chip).irq_mask.unwrap())(idata); }
    else { ((*chip).irq_mask_ack.unwrap())(idata); }
    raw_spin_unlock(&mut (*desc).lock);
    msr = mfdcr((*uic).dcrbase + UIC_MSR);
    if msr != 0 {
        src = 32 - ffs(msr) as i32;
        generic_handle_domain_irq((*uic).irqhost, src as u32);
    }
    raw_spin_lock(&mut (*desc).lock);
    if irqd_is_level_type(idata) { ((*chip).irq_ack.unwrap())(idata); }
    if !irqd_irq_disabled(idata) && (*chip).irq_unmask.is_some() {
        ((*chip).irq_unmask.unwrap())(idata);
    }
    raw_spin_unlock(&mut (*desc).lock);
}

// The remaining device-tree and allocator declarations are supplied externally.
unsafe fn uic_init_one(node: *mut device_node) -> *mut uic {
    BUG_ON(!of_device_is_compatible(node, "ibm,uic"));
    let uic = kzalloc_obj::<uic>();
    if uic.is_null() { return core::ptr::null_mut(); }
    raw_spin_lock_init(&mut (*uic).lock);
    let mut len: i32 = 0;
    let indexp = of_get_property(node, "cell-index", &mut len);
    if indexp.is_null() || len != core::mem::size_of::<u32>() as i32 {
        printk(KERN_ERR, "uic: Device node %pOF has missing or invalid cell-index property\n", node);
        return core::ptr::null_mut();
    }
    (*uic).index = *(indexp as *const u32) as i32;
    let dcrreg = of_get_property(node, "dcr-reg", &mut len);
    if dcrreg.is_null() || len != (2 * core::mem::size_of::<u32>()) as i32 {
        printk(KERN_ERR, "uic: Device node %pOF has missing or invalid dcr-reg property\n", node);
        return core::ptr::null_mut();
    }
    (*uic).dcrbase = *(dcrreg as *const u32) as i32;
    (*uic).irqhost = irq_domain_create_linear(of_fwnode_handle(node), NR_UIC_INTS, &uic_host_ops, uic);
    if (*uic).irqhost.is_null() { return core::ptr::null_mut(); }
    mtdcr((*uic).dcrbase + UIC_ER, 0);
    mtdcr((*uic).dcrbase + UIC_CR, 0);
    mtdcr((*uic).dcrbase + UIC_TR, 0);
    mtdcr((*uic).dcrbase + UIC_SR, 0xffffffff);
    printk("UIC%d (%d IRQ sources) at DCR 0x%x\n", (*uic).index, NR_UIC_INTS, (*uic).dcrbase);
    uic
}

unsafe fn uic_init_tree() {
    let mut np: *mut device_node = core::ptr::null_mut();
    let mut interrupts: *const u32;
    for_each_compatible_node!(np, "ibm,uic") {
        interrupts = of_get_property(np, "interrupts", core::ptr::null_mut());
        if interrupts.is_null() { break; }
    }
    BUG_ON(np.is_null());
    primary_uic = uic_init_one(np);
    if primary_uic.is_null() { panic!("Unable to initialize primary UIC %pOF\n", np); }
    irq_set_default_domain((*primary_uic).irqhost);
    of_node_put(np);
    for_each_compatible_node!(np, "ibm,uic") {
        interrupts = of_get_property(np, "interrupts", core::ptr::null_mut());
        if !interrupts.is_null() {
            let uic = uic_init_one(np);
            if uic.is_null() { panic!("Unable to initialize a secondary UIC %pOF\n", np); }
            let cascade_virq = irq_of_parse_and_map(np, 0);
            irq_set_chained_handler_and_data(cascade_virq, uic_irq_cascade, uic);
            /* FIXME: setup critical cascade?? */
        }
    }
}

/* Return an interrupt vector or 0 if no interrupt is pending. */
unsafe fn uic_get_irq() -> u32 {
    BUG_ON(primary_uic.is_null());
    let msr = mfdcr((*primary_uic).dcrbase + UIC_MSR);
    let src = 32 - ffs(msr) as i32;
    irq_find_mapping((*primary_uic).irqhost, src as u32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
