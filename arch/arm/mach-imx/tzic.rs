// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C)2004-2010 Freescale Semiconductor, Inc. All Rights Reserved.
 */

// Linux kernel dependencies are supplied by other translation units.

const TZIC_INTCNTL: usize = 0x0000;
const TZIC_INTTYPE: usize = 0x0004;
const TZIC_IMPID: usize = 0x0008;
const TZIC_PRIOMASK: usize = 0x000C;
const TZIC_SYNCCTRL: usize = 0x0010;
const TZIC_DSMINT: usize = 0x0014;
#[inline]
const fn TZIC_INTSEC0(i: usize) -> usize { 0x0080 + (i << 2) }
#[inline]
const fn TZIC_ENSET0(i: usize) -> usize { 0x0100 + (i << 2) }
#[inline]
const fn TZIC_ENCLEAR0(i: usize) -> usize { 0x0180 + (i << 2) }
const TZIC_SRCSET0: usize = 0x0200;
const TZIC_SRCCLAR0: usize = 0x0280;
const TZIC_PRIORITY0: usize = 0x0400;
const TZIC_PND0: usize = 0x0D00;
#[inline]
const fn TZIC_HIPND(i: usize) -> usize { 0x0D80 + (i << 2) }
#[inline]
const fn TZIC_WAKEUP0(i: usize) -> usize { 0x0E00 + (i << 2) }
const TZIC_SWINT: usize = 0x0F00;
const TZIC_ID0: usize = 0x0FD0;

static mut tzic_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut domain: *mut irq_domain = core::ptr::null_mut();

const TZIC_NUM_IRQS: u32 = 128;

#[cfg(feature = "CONFIG_FIQ")]
unsafe fn tzic_set_irq_fiq(hwirq: u32, irq_type: u32) -> i32 {
    let index = hwirq >> 5;
    if index >= 4 { return -22; }
    let mask = 1u32 << (hwirq & 0x1f);
    let mut value = imx_readl(tzic_base.add(TZIC_INTSEC0(index as usize))) | mask;
    if irq_type != 0 { value &= !mask; }
    imx_writel(value, tzic_base.add(TZIC_INTSEC0(index as usize)));
    0
}

// Without CONFIG_FIQ, tzic_set_irq_fiq is NULL.

#[cfg(feature = "CONFIG_PM")]
unsafe fn tzic_irq_suspend(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let idx = (*d).hwirq >> 5;
    imx_writel((*gc).wake_active, tzic_base.add(TZIC_WAKEUP0(idx as usize)));
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn tzic_irq_resume(d: *mut irq_data) {
    let idx = (*d).hwirq >> 5;
    imx_writel(imx_readl(tzic_base.add(TZIC_ENSET0(idx as usize))),
               tzic_base.add(TZIC_WAKEUP0(idx as usize)));
}

// Without CONFIG_PM, tzic_irq_suspend and tzic_irq_resume are NULL.

#[repr(C)]
struct mxc_extra_irq {
    #[cfg(feature = "CONFIG_FIQ")]
    set_irq_fiq: Option<unsafe fn(u32, u32) -> i32>,
}

static mut tzic_extra_irq: mxc_extra_irq = mxc_extra_irq {
    #[cfg(feature = "CONFIG_FIQ")]
    set_irq_fiq: Some(tzic_set_irq_fiq),
};

unsafe fn tzic_init_gc(idx: i32, irq_start: u32) {
    let gc = irq_alloc_generic_chip("tzic\0".as_ptr() as *const _, 1, irq_start,
                                    tzic_base, Some(handle_level_irq));
    (*gc).private = &mut tzic_extra_irq as *mut _ as *mut core::ffi::c_void;
    (*gc).wake_enabled = IRQ_MSK(32);
    let ct = (*gc).chip_types;
    (*ct).chip.irq_mask = Some(irq_gc_mask_disable_reg);
    (*ct).chip.irq_unmask = Some(irq_gc_unmask_enable_reg);
    (*ct).chip.irq_set_wake = Some(irq_gc_set_wake);
    (*ct).chip.irq_suspend = tzic_irq_suspend;
    (*ct).chip.irq_resume = tzic_irq_resume;
    (*ct).regs.disable = TZIC_ENCLEAR0(idx as usize);
    (*ct).regs.enable = TZIC_ENSET0(idx as usize);
    irq_setup_generic_chip(gc, IRQ_MSK(32), 0, IRQ_NOREQUEST, 0);
}

unsafe fn tzic_handle_irq(_regs: *mut pt_regs) {
    let mut handled: i32;
    loop {
        handled = 0;
        for i in 0..4 {
            let mut stat = imx_readl(tzic_base.add(TZIC_HIPND(i))) &
                imx_readl(tzic_base.add(TZIC_INTSEC0(i)));
            while stat != 0 {
                handled = 1;
                let irqofs = 31 - stat.leading_zeros();
                generic_handle_domain_irq(domain, irqofs + i as u32 * 32);
                stat &= !(1u32 << irqofs);
            }
        }
        if handled == 0 { break; }
    }
}

unsafe fn tzic_init_dt(np: *mut device_node, _p: *mut device_node) -> i32 {
    let mut irq_base: i32;
    let mut i: i32;
    tzic_base = of_iomap(np, 0);
    WARN_ON(tzic_base.is_null());
    i = imx_readl(tzic_base.add(TZIC_INTCNTL)) as i32;
    imx_writel(0x80010001, tzic_base.add(TZIC_INTCNTL));
    imx_writel(0x1f, tzic_base.add(TZIC_PRIOMASK));
    imx_writel(0x02, tzic_base.add(TZIC_SYNCCTRL));
    for i in 0..4 { imx_writel(0xffffffff, tzic_base.add(TZIC_INTSEC0(i as usize))); }
    for i in 0..4 { imx_writel(0xffffffff, tzic_base.add(TZIC_ENCLEAR0(i as usize))); }
    irq_base = irq_alloc_descs(-1, 0, TZIC_NUM_IRQS, numa_node_id());
    WARN_ON(irq_base < 0);
    domain = irq_domain_create_legacy(of_fwnode_handle(np), TZIC_NUM_IRQS, irq_base,
                                      0, &irq_domain_simple_ops, core::ptr::null_mut());
    WARN_ON(domain.is_null());
    for i in 0..4 { tzic_init_gc(i, irq_base as u32); irq_base += 32; }
    set_handle_irq(tzic_handle_irq);
    #[cfg(feature = "CONFIG_FIQ")]
    init_FIQ(FIQ_START);
    pr_info("TrustZone Interrupt Controller (TZIC) initialized\0".as_ptr() as *const _);
    0
}

// IRQCHIP_DECLARE(tzic, "fsl,tzic", tzic_init_dt);

pub unsafe fn tzic_enable_wake() -> i32 {
    imx_writel(1, tzic_base.add(TZIC_DSMINT));
    if imx_readl(tzic_base.add(TZIC_DSMINT)) == 0 { return -11; }
    for i in 0..4 {
        imx_writel(imx_readl(tzic_base.add(TZIC_ENSET0(i))),
                   tzic_base.add(TZIC_WAKEUP0(i)));
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
