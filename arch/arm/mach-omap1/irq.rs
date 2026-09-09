/*
 * linux/arch/arm/mach-omap1/irq.c
 *
 * Interrupt handler for all OMAP boards
 *
 * Copyright (C) 2004 Nokia Corporation
 * Written by Tony Lindgren <tony@atomide.com>
 * Major cleanups by Juha Yrjölä <juha.yrjola@nokia.com>
 *
 * Completely re-written to support various OMAP chips with bank specific
 * interrupt handlers.
 *
 * Some snippets of the code taken from the older OMAP interrupt handler
 * Copyright (C) 2001 RidgeRun, Inc. Greg Lonnon <glonnon@ridgerun.com>
 *
 * GPIO interrupt handler moved to gpio.c by Juha Yrjola
 */

// C headers and local headers provide the external kernel, architecture, and
// OMAP symbols referenced below.

const IRQ_BANK = |irq: i32| irq >> 5;
const IRQ_BIT = |irq: i32| irq & 0x1f;

#[repr(C)]
struct omap_irq_bank {
    base_reg: ::core::ffi::c_ulong,
    va: *mut ::core::ffi::c_void,
    trigger_map: ::core::ffi::c_ulong,
    wake_enable: ::core::ffi::c_ulong,
}

static mut omap_l2_irq: u32 = 0;
static mut irq_bank_count: u32 = 0;
static mut irq_banks: *mut omap_irq_bank = ::core::ptr::null_mut();
static mut domain: *mut irq_domain = ::core::ptr::null_mut();

#[inline]
unsafe fn irq_bank_readl(bank: i32, offset: i32) -> u32 {
    readl_relaxed((*irq_banks.add(bank as usize)).va.cast::<u8>().add(offset as usize))
}

#[inline]
unsafe fn irq_bank_writel(value: ::core::ffi::c_ulong, bank: i32, offset: i32) {
    writel_relaxed(value, (*irq_banks.add(bank as usize)).va.cast::<u8>().add(offset as usize));
}

unsafe fn omap_ack_irq(irq: i32) {
    if irq > 31 {
        writel_relaxed(0x1, (*irq_banks.add(1)).va.cast::<u8>().add(IRQ_CONTROL_REG_OFFSET as usize));
    }
    writel_relaxed(0x1, (*irq_banks).va.cast::<u8>().add(IRQ_CONTROL_REG_OFFSET as usize));
}

unsafe fn omap_mask_ack_irq(d: *mut irq_data) {
    let ct = irq_data_get_chip_type(d);
    ((*ct).chip.irq_mask.unwrap())(d);
    omap_ack_irq((*d).irq);
}

/*
 * Allows tuning the IRQ type and priority
 *
 * NOTE: There is currently no OMAP fiq handler for Linux. Read the
 * mailing list threads on FIQ handlers if you are planning to add a FIQ
 * handler for OMAP.
 */
unsafe fn omap_irq_set_cfg(mut irq: i32, mut fiq: i32, priority: i32, trigger: i32) {
    let bank: i32 = IRQ_BANK(irq);
    fiq = if bank != 0 { 0 } else { fiq & 0x1 };
    let val: ::core::ffi::c_ulong = (fiq | ((priority & 0x1f) << 2) | ((trigger & 0x1) << 1)) as _;
    let offset = IRQ_ILR0_REG_OFFSET + IRQ_BIT(irq) * 0x4;
    irq_bank_writel(val, bank, offset);
}

#[cfg(CONFIG_ARCH_OMAP15XX)]
static mut omap1510_irq_banks: [omap_irq_bank; 2] = [
    omap_irq_bank { base_reg: OMAP_IH1_BASE, va: ::core::ptr::null_mut(), trigger_map: 0xb3febfff, wake_enable: 0 },
    omap_irq_bank { base_reg: OMAP_IH2_BASE, va: ::core::ptr::null_mut(), trigger_map: 0xffbfffed, wake_enable: 0 },
];
#[cfg(CONFIG_ARCH_OMAP15XX)]
static mut omap310_irq_banks: [omap_irq_bank; 2] = [
    omap_irq_bank { base_reg: OMAP_IH1_BASE, va: ::core::ptr::null_mut(), trigger_map: 0xb3faefc3, wake_enable: 0 },
    omap_irq_bank { base_reg: OMAP_IH2_BASE, va: ::core::ptr::null_mut(), trigger_map: 0x65b3c061, wake_enable: 0 },
];

#[cfg(CONFIG_ARCH_OMAP16XX)]
static mut omap1610_irq_banks: [omap_irq_bank; 4] = [
    omap_irq_bank { base_reg: OMAP_IH1_BASE, va: ::core::ptr::null_mut(), trigger_map: 0xb3fefe8f, wake_enable: 0 },
    omap_irq_bank { base_reg: OMAP_IH2_BASE, va: ::core::ptr::null_mut(), trigger_map: 0xfdb7c1fd, wake_enable: 0 },
    omap_irq_bank { base_reg: OMAP_IH2_BASE + 0x100, va: ::core::ptr::null_mut(), trigger_map: 0xffffb7ff, wake_enable: 0 },
    omap_irq_bank { base_reg: OMAP_IH2_BASE + 0x200, va: ::core::ptr::null_mut(), trigger_map: 0xffffffff, wake_enable: 0 },
];

unsafe fn omap1_handle_irq(_regs: *mut pt_regs) {
    let l1 = (*irq_banks).va.cast::<u8>();
    let l2 = (*irq_banks.add(1)).va.cast::<u8>();
    let mut irqnr: u32;
    loop {
        irqnr = readl_relaxed(l1.add(IRQ_ITR_REG_OFFSET as usize));
        irqnr &= !(readl_relaxed(l1.add(IRQ_MIR_REG_OFFSET as usize)) & 0xffffffff);
        if irqnr == 0 { break; }
        irqnr = readl_relaxed(l1.add(IRQ_SIR_FIQ_REG_OFFSET as usize));
        if irqnr == 0 {
            irqnr = readl_relaxed(l1.add(IRQ_SIR_IRQ_REG_OFFSET as usize));
            if irqnr == omap_l2_irq {
                irqnr = readl_relaxed(l2.add(IRQ_SIR_IRQ_REG_OFFSET as usize));
                if irqnr != 0 { irqnr += 32; }
            }
        }
        if irqnr != 0 { generic_handle_domain_irq(domain, irqnr); } else { break; }
    }
}

unsafe fn omap_alloc_gc(base: *mut ::core::ffi::c_void, irq_start: u32, num: u32) {
    let gc = irq_alloc_generic_chip("MPU\0".as_ptr() as _, 1, irq_start, base, handle_level_irq);
    let ct = (*gc).chip_types;
    (*ct).chip.irq_ack = Some(omap_mask_ack_irq);
    (*ct).chip.irq_mask = Some(irq_gc_mask_set_bit);
    (*ct).chip.irq_unmask = Some(irq_gc_mask_clr_bit);
    (*ct).chip.irq_set_wake = Some(irq_gc_set_wake);
    (*ct).regs.mask = IRQ_MIR_REG_OFFSET;
    irq_setup_generic_chip(gc, IRQ_MSK(num), IRQ_GC_INIT_MASK_CACHE, IRQ_NOREQUEST | IRQ_NOPROBE, 0);
}

unsafe fn omap1_init_irq() {
    let mut ct: *mut irq_chip_type;
    let mut d: *mut irq_data = ::core::ptr::null_mut();
    let mut i: i32;
    let mut j: i32;
    let mut irq_base: i32;
    let mut nr_irqs: usize;

    #[cfg(CONFIG_ARCH_OMAP15XX)] {
        if cpu_is_omap1510() { irq_banks = omap1510_irq_banks.as_mut_ptr(); irq_bank_count = 2; }
        if cpu_is_omap310() { irq_banks = omap310_irq_banks.as_mut_ptr(); irq_bank_count = 2; }
    }
    #[cfg(CONFIG_ARCH_OMAP16XX)] {
        if cpu_is_omap16xx() { irq_banks = omap1610_irq_banks.as_mut_ptr(); irq_bank_count = 4; }
    }
    i = 0;
    while i < irq_bank_count as i32 {
        (*irq_banks.add(i as usize)).va = ioremap((*irq_banks.add(i as usize)).base_reg, 0xff);
        if WARN_ON((*irq_banks.add(i as usize)).va.is_null()) { return; }
        i += 1;
    }
    nr_irqs = irq_bank_count as usize * 32;
    irq_base = irq_alloc_descs(-1, 0, nr_irqs, 0);
    if irq_base < 0 { pr_warn!("Couldn't allocate IRQ numbers\n"); irq_base = 0; }
    omap_l2_irq = irq_base as u32;
    omap_l2_irq -= NR_IRQS_LEGACY;
    domain = irq_domain_create_legacy(::core::ptr::null_mut(), nr_irqs, irq_base as u32, 0, &irq_domain_simple_ops, ::core::ptr::null_mut());
    pr_info!("Total of {} interrupts in {} interrupt banks\n", nr_irqs, irq_bank_count);
    i = 0;
    while i < irq_bank_count as i32 { irq_bank_writel(!0x0, i, IRQ_MIR_REG_OFFSET); irq_bank_writel(0x0, i, IRQ_ITR_REG_OFFSET); i += 1; }
    irq_bank_writel(0x03, 0, IRQ_CONTROL_REG_OFFSET);
    irq_bank_writel(0x03, 1, IRQ_CONTROL_REG_OFFSET);
    i = 0;
    while i < irq_bank_count as i32 {
        j = i * 32;
        while j < (i + 1) * 32 {
            let irq_trigger = ((*irq_banks.add(i as usize)).trigger_map >> IRQ_BIT(j)) as i32;
            omap_irq_set_cfg(j, 0, 0, irq_trigger);
            irq_clear_status_flags(j as u32, IRQ_NOREQUEST);
            j += 1;
        }
        omap_alloc_gc((*irq_banks.add(i as usize)).va, irq_base as u32 + i as u32 * 32, 32);
        i += 1;
    }
    d = irq_get_irq_data(irq_find_mapping(domain, omap_l2_irq));
    if !d.is_null() { ct = irq_data_get_chip_type(d); ((*ct).chip.irq_unmask.unwrap())(d); }
    set_handle_irq(omap1_handle_irq);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
