// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/mach-pxa/irq.c
 *
 *  Generic PXA IRQ handling
 *
 *  Author: Nicolas Pitre
 *  Created: Jun 15, 2001
 *  Copyright: MontaVista Software Inc.
 */

// Kernel dependencies supplied by the surrounding translation unit/build.

const ICIP: usize = 0x000;
const ICMR: usize = 0x004;
const ICLR: usize = 0x008;
const ICFR: usize = 0x00c;
const ICPR: usize = 0x010;
const ICCR: usize = 0x014;
const ICHP: usize = 0x018;

#[inline]
const fn ipr(i: usize) -> usize {
    if i < 32 {
        0x01c + (i << 2)
    } else if i < 64 {
        0x0b0 + ((i - 32) << 2)
    } else {
        0x144 + ((i - 64) << 2)
    }
}

const ICHP_VAL_IRQ: u32 = 1 << 31;
const ICHP_IRQ_MASK: u32 = 0x7fff;
const IPR_VALID: u32 = 1 << 31;
const MAX_INTERNAL_IRQS: usize = 128;

/* This is for peripheral IRQs internal to the PXA chip. */
static mut pxa_irq_base: *mut u8 = core::ptr::null_mut();
static mut pxa_internal_irq_nr: i32 = 0;
static mut cpu_has_ipr: bool = false;
static mut pxa_irq_domain: *mut irq_domain = core::ptr::null_mut();

#[inline]
unsafe fn irq_base(i: usize) -> *mut u8 {
    static PHYS_BASE_OFFSET: [usize; 3] = [0x0, 0x9c, 0x130];
    pxa_irq_base.add(PHYS_BASE_OFFSET[i])
}

pub unsafe fn pxa_mask_irq(d: *mut irq_data) {
    let base = irq_data_get_irq_chip_data(d);
    let irq = irqd_to_hwirq(d);
    let mut icmr = __raw_readl(base.add(ICMR));
    icmr &= !(1u32 << (irq & 0x1f));
    __raw_writel(icmr, base.add(ICMR));
}

pub unsafe fn pxa_unmask_irq(d: *mut irq_data) {
    let base = irq_data_get_irq_chip_data(d);
    let irq = irqd_to_hwirq(d);
    let mut icmr = __raw_readl(base.add(ICMR));
    icmr |= 1u32 << (irq & 0x1f);
    __raw_writel(icmr, base.add(ICMR));
}

static mut pxa_internal_irq_chip: irq_chip = irq_chip {
    name: "SC" as *const str,
    irq_ack: Some(pxa_mask_irq),
    irq_mask: Some(pxa_mask_irq),
    irq_unmask: Some(pxa_unmask_irq),
    ..irq_chip::zeroed()
};

pub unsafe extern "C" fn icip_handle_irq(regs: *mut pt_regs) {
    loop {
        let icip = __raw_readl(pxa_irq_base.add(ICIP));
        let icmr = __raw_readl(pxa_irq_base.add(ICMR));
        let mask = icip & icmr;
        if mask == 0 {
            break;
        }
        handle_IRQ(PXA_IRQ(31 - mask.leading_zeros()), regs);
    }
}

pub unsafe extern "C" fn ichp_handle_irq(regs: *mut pt_regs) {
    loop {
        let ichp: u32;
        core::arch::asm!("mrc p6, 0, {0}, c5, c0, 0", out(reg) ichp);
        if (ichp & ICHP_VAL_IRQ) == 0 {
            break;
        }
        handle_IRQ(PXA_IRQ((ichp >> 16) & ICHP_IRQ_MASK), regs);
    }
}

unsafe fn pxa_irq_map(_h: *mut irq_domain, virq: u32, hw: usize) -> i32 {
    let base = irq_base(hw / 32);
    if cpu_has_ipr {
        __raw_writel((hw as u32) | IPR_VALID, pxa_irq_base.add(ipr(hw)));
    }
    irq_set_chip_and_handler(virq, &mut pxa_internal_irq_chip, handle_level_irq);
    irq_set_chip_data(virq, base);
    0
}

static mut pxa_irq_ops: irq_domain_ops = irq_domain_ops {
    map: Some(pxa_irq_map),
    xlate: Some(irq_domain_xlate_onecell),
    ..irq_domain_ops::zeroed()
};

unsafe fn pxa_init_irq_common(
    node: *mut device_node,
    irq_nr: i32,
    fn_: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>,
) {
    pxa_internal_irq_nr = irq_nr;
    pxa_irq_domain = irq_domain_create_legacy(
        of_fwnode_handle(node), irq_nr as u32, PXA_IRQ(0), 0,
        &mut pxa_irq_ops, core::ptr::null_mut(),
    );
    if pxa_irq_domain.is_null() {
        panic!("Unable to add PXA IRQ domain\n");
    }
    irq_set_default_domain(pxa_irq_domain);
    let mut n = 0;
    while n < irq_nr {
        let base = irq_base((n >> 5) as usize);
        __raw_writel(0, base.add(ICMR));
        __raw_writel(0, base.add(ICLR));
        n += 32;
    }
    __raw_writel(1, irq_base(0).add(ICCR));
    pxa_internal_irq_chip.irq_set_wake = fn_;
}

pub unsafe fn pxa_init_irq(
    irq_nr: i32,
    fn_: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>,
) {
    BUG_ON(irq_nr > MAX_INTERNAL_IRQS as i32);
    pxa_irq_base = io_p2v(0x40d00000);
    cpu_has_ipr = !cpu_is_pxa25x();
    pxa_init_irq_common(core::ptr::null_mut(), irq_nr, fn_);
}

#[cfg(feature = "CONFIG_PM")]
static mut saved_icmr: [usize; MAX_INTERNAL_IRQS / 32] = [0; MAX_INTERNAL_IRQS / 32];
#[cfg(feature = "CONFIG_PM")]
static mut saved_ipr: [usize; MAX_INTERNAL_IRQS] = [0; MAX_INTERNAL_IRQS];

#[cfg(feature = "CONFIG_PM")]
unsafe fn pxa_irq_suspend(_data: *mut core::ffi::c_void) -> i32 {
    let count = ((pxa_internal_irq_nr + 31) / 32) as usize;
    for i in 0..count {
        let base = irq_base(i);
        saved_icmr[i] = __raw_readl(base.add(ICMR)) as usize;
        __raw_writel(0, base.add(ICMR));
    }
    if cpu_has_ipr {
        for i in 0..pxa_internal_irq_nr as usize {
            saved_ipr[i] = __raw_readl(pxa_irq_base.add(ipr(i))) as usize;
        }
    }
    0
}

#[cfg(feature = "CONFIG_PM")]
unsafe fn pxa_irq_resume(_data: *mut core::ffi::c_void) {
    let count = ((pxa_internal_irq_nr + 31) / 32) as usize;
    for i in 0..count {
        let base = irq_base(i);
        __raw_writel(saved_icmr[i] as u32, base.add(ICMR));
        __raw_writel(0, base.add(ICLR));
    }
    if cpu_has_ipr {
        for i in 0..pxa_internal_irq_nr as usize {
            __raw_writel(saved_ipr[i] as u32, pxa_irq_base.add(ipr(i)));
        }
    }
    __raw_writel(1, pxa_irq_base.add(ICCR));
}

#[cfg(not(feature = "CONFIG_PM"))]
unsafe fn pxa_irq_suspend(_data: *mut core::ffi::c_void) -> i32 { 0 }
#[cfg(not(feature = "CONFIG_PM"))]
unsafe fn pxa_irq_resume(_data: *mut core::ffi::c_void) {}

static mut pxa_irq_syscore_ops: syscore_ops = syscore_ops {
    suspend: Some(pxa_irq_suspend),
    resume: Some(pxa_irq_resume),
    ..syscore_ops::zeroed()
};

pub static mut pxa_irq_syscore: syscore = syscore { ops: &mut pxa_irq_syscore_ops };

#[cfg(feature = "CONFIG_OF")]
static intc_ids: [of_device_id; 2] = [
    of_device_id { compatible: "marvell,pxa-intc\0" },
    of_device_id::zeroed(),
];

#[cfg(feature = "CONFIG_OF")]
pub unsafe fn pxa_dt_irq_init(
    fn_: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>,
) {
    let node = of_find_matching_node(core::ptr::null_mut(), intc_ids.as_ptr());
    if node.is_null() { pr_err!("Failed to find interrupt controller in arch-pxa\n"); return; }
    let mut res = resource::zeroed();
    if of_property_read_u32(node, "marvell,intc-nr-irqs\0", &mut pxa_internal_irq_nr) != 0 {
        pr_err!("Not found marvell,intc-nr-irqs property\n"); return;
    }
    if of_address_to_resource(node, 0, &mut res) < 0 {
        pr_err!("No registers defined for node\n"); return;
    }
    pxa_irq_base = io_p2v(res.start);
    cpu_has_ipr = of_property_read_bool(node, "marvell,intc-priority\0");
    if irq_alloc_descs(-1, 0, pxa_internal_irq_nr, 0) < 0 {
        pr_err!("Failed to allocate IRQ numbers\n"); return;
    }
    pxa_init_irq_common(node, pxa_internal_irq_nr, fn_);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
