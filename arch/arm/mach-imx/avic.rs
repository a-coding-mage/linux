// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2004-2007 Freescale Semiconductor, Inc. All Rights Reserved.
 * Copyright 2008 Juergen Beisert, kernel@pengutronix.de
 */

// C dependencies retained as external symbols supplied by other files.

const AVIC_INTCNTL: usize = 0x00;
const AVIC_NIMASK: usize = 0x04;
const AVIC_INTENNUM: usize = 0x08;
const AVIC_INTDISNUM: usize = 0x0C;
const AVIC_INTENABLEH: usize = 0x10;
const AVIC_INTENABLEL: usize = 0x14;
const AVIC_INTTYPEH: usize = 0x18;
const AVIC_INTTYPEL: usize = 0x1C;
const fn avic_nipriority(x: usize) -> usize { 0x20 + 4 * (7 - x) }
const AVIC_NIVECSR: usize = 0x40;
const AVIC_FIVECSR: usize = 0x44;
const AVIC_INTSRCH: usize = 0x48;
const AVIC_INTSRCL: usize = 0x4C;
const AVIC_INTFRCH: usize = 0x50;
const AVIC_INTFRCL: usize = 0x54;
const AVIC_NIPNDH: usize = 0x58;
const AVIC_NIPNDL: usize = 0x5C;
const AVIC_FIPNDH: usize = 0x60;
const AVIC_FIPNDL: usize = 0x64;

const AVIC_NUM_IRQS: usize = 64;
const MX25_CCM_LPIMR0: usize = 0x68;
const MX25_CCM_LPIMR1: usize = 0x6C;

static mut avic_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut mx25_ccm_base: *mut core::ffi::c_void = core::ptr::null_mut();
static mut domain: *mut irq_domain = core::ptr::null_mut();

#[cfg(feature = "CONFIG_FIQ")]
unsafe fn avic_set_irq_fiq(hwirq: u32, irq_type: u32) -> i32 {
    let mut irqt: u32;
    if hwirq >= AVIC_NUM_IRQS as u32 { return -22; }
    if hwirq < (AVIC_NUM_IRQS / 2) as u32 {
        irqt = imx_readl(avic_base.add(AVIC_INTTYPEL)) & !(1u32 << hwirq);
        imx_writel(irqt | (((irq_type != 0) as u32) << hwirq), avic_base.add(AVIC_INTTYPEL));
    } else {
        let hwirq = hwirq - (AVIC_NUM_IRQS / 2) as u32;
        irqt = imx_readl(avic_base.add(AVIC_INTTYPEH)) & !(1u32 << hwirq);
        imx_writel(irqt | (((irq_type != 0) as u32) << hwirq), avic_base.add(AVIC_INTTYPEH));
    }
    0
}

static mut avic_extra_irq: mxc_extra_irq = mxc_extra_irq {
    #[cfg(feature = "CONFIG_FIQ")]
    set_irq_fiq: Some(avic_set_irq_fiq),
};

#[cfg(feature = "CONFIG_PM")]
static mut avic_saved_mask_reg: [u32; 2] = [0; 2];

#[cfg(feature = "CONFIG_PM")]
unsafe fn avic_irq_suspend(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let ct = (*gc).chip_types;
    let idx = ((*d).hwirq >> 5) as usize;
    avic_saved_mask_reg[idx] = imx_readl(avic_base.add((*ct).regs.mask as usize));
    imx_writel((*gc).wake_active, avic_base.add((*ct).regs.mask as usize));
    if !mx25_ccm_base.is_null() {
        let offs = if (*d).hwirq < (AVIC_NUM_IRQS / 2) as u32 { MX25_CCM_LPIMR0 } else { MX25_CCM_LPIMR1 };
        imx_writel(!(*gc).wake_active, mx25_ccm_base.add(offs));
    }
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn avic_irq_resume(d: *mut irq_data) {
    let gc = irq_data_get_irq_chip_data(d);
    let ct = (*gc).chip_types;
    let idx = ((*d).hwirq >> 5) as usize;
    imx_writel(avic_saved_mask_reg[idx], avic_base.add((*ct).regs.mask as usize));
    if !mx25_ccm_base.is_null() {
        let offs = if (*d).hwirq < (AVIC_NUM_IRQS / 2) as u32 { MX25_CCM_LPIMR0 } else { MX25_CCM_LPIMR1 };
        imx_writel(0xffff_ffff, mx25_ccm_base.add(offs));
    }
}

// Without CONFIG_PM, the C macros define avic_irq_suspend and avic_irq_resume as NULL.

unsafe fn avic_init_gc(idx: i32, mut irq_start: i32) {
    let gc = irq_alloc_generic_chip("mxc-avic", 1, irq_start, avic_base, handle_level_irq);
    (*gc).private = &mut avic_extra_irq;
    (*gc).wake_enabled = IRQ_MSK(32);
    let ct = (*gc).chip_types;
    (*ct).chip.irq_mask = Some(irq_gc_mask_clr_bit);
    (*ct).chip.irq_unmask = Some(irq_gc_mask_set_bit);
    (*ct).chip.irq_ack = Some(irq_gc_mask_clr_bit);
    (*ct).chip.irq_set_wake = Some(irq_gc_set_wake);
    (*ct).regs.mask = if idx == 0 { AVIC_INTENABLEL as u32 } else { AVIC_INTENABLEH as u32 };
    (*ct).regs.ack = (*ct).regs.mask;
    irq_setup_generic_chip(gc, IRQ_MSK(32), 0, IRQ_NOREQUEST, 0);
}

unsafe fn avic_handle_irq(_regs: *mut pt_regs) {
    loop {
        let nivector = imx_readl(avic_base.add(AVIC_NIVECSR)) >> 16;
        if nivector == 0xffff { break; }
        generic_handle_domain_irq(domain, nivector);
    }
}

unsafe fn mxc_init_irq(irqbase: *mut core::ffi::c_void) {
    avic_base = irqbase;
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,imx25-ccm");
    mx25_ccm_base = of_iomap(np, 0);
    of_node_put(np);
    if !mx25_ccm_base.is_null() {
        imx_writel(0xffff_ffff, mx25_ccm_base.add(MX25_CCM_LPIMR0));
        imx_writel(0xffff_ffff, mx25_ccm_base.add(MX25_CCM_LPIMR1));
    }
    imx_writel(0, avic_base.add(AVIC_INTCNTL));
    imx_writel(0x1f, avic_base.add(AVIC_NIMASK));
    imx_writel(0, avic_base.add(AVIC_INTENABLEH));
    imx_writel(0, avic_base.add(AVIC_INTENABLEL));
    imx_writel(0, avic_base.add(AVIC_INTTYPEH));
    imx_writel(0, avic_base.add(AVIC_INTTYPEL));
    let mut irq_base = irq_alloc_descs(-1, 0, AVIC_NUM_IRQS as i32, numa_node_id());
    WARN_ON(irq_base < 0);
    let np = of_find_compatible_node(core::ptr::null_mut(), core::ptr::null_mut(), "fsl,avic");
    domain = irq_domain_create_legacy(of_fwnode_handle(np), AVIC_NUM_IRQS as u32, irq_base, 0, &irq_domain_simple_ops, core::ptr::null_mut());
    of_node_put(np);
    WARN_ON(domain.is_null());
    for i in 0..(AVIC_NUM_IRQS / 32) { avic_init_gc(i as i32, irq_base + (i as i32) * 32); }
    for i in 0..8 { imx_writel(0, avic_base.add(avic_nipriority(i))); }
    set_handle_irq(avic_handle_irq);
    #[cfg(feature = "CONFIG_FIQ")]
    init_FIQ(FIQ_START);
    printk(KERN_INFO, "MXC IRQ initialized\n");
}

unsafe fn imx_avic_init(node: *mut device_node, _parent: *mut device_node) -> i32 {
    let avic_base_local = of_iomap(node, 0);
    BUG_ON(avic_base_local.is_null());
    mxc_init_irq(avic_base_local);
    0
}

// IRQCHIP_DECLARE(imx_avic, "fsl,avic", imx_avic_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
