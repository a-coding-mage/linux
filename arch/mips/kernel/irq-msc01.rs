// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *
 * Copyright (c) 2004 MIPS Inc
 * Author: chris@mips.com
 *
 * Copyright (C) 2004, 06 Ralf Baechle <ralf@linux-mips.org>
 */

// Dependencies supplied by the surrounding kernel translation.

static mut _ICCTRL_MSC: ::core::ffi::c_ulong = 0;
const MSC01_IC_REG_BASE: usize = unsafe { _ICCTRL_MSC as usize };

static mut IRQ_BASE: ::core::ffi::c_uint = 0;

#[inline]
unsafe fn mscic_write(reg: usize, data: u32) {
    ::core::ptr::write_volatile(reg as *mut u32, data);
}

#[inline]
unsafe fn mscic_read(reg: usize, data: &mut u32) {
    *data = ::core::ptr::read_volatile(reg as *const u32);
}

/* mask off an interrupt */
#[inline]
unsafe fn mask_msc_irq(d: *mut irq_data) {
    let irq = (*d).irq;

    if irq < IRQ_BASE + 32 {
        mscic_write(MSC01_IC_DISL, 1u32 << (irq - IRQ_BASE));
    } else {
        mscic_write(MSC01_IC_DISH, 1u32 << (irq - IRQ_BASE - 32));
    }
}

/* unmask an interrupt */
#[inline]
unsafe fn unmask_msc_irq(d: *mut irq_data) {
    let irq = (*d).irq;

    if irq < IRQ_BASE + 32 {
        mscic_write(MSC01_IC_ENAL, 1u32 << (irq - IRQ_BASE));
    } else {
        mscic_write(MSC01_IC_ENAH, 1u32 << (irq - IRQ_BASE - 32));
    }
}

/*
 * Masks and ACKs an IRQ
 */
unsafe fn level_mask_and_ack_msc_irq(d: *mut irq_data) {
    mask_msc_irq(d);
    if !cpu_has_veic {
        mscic_write(MSC01_IC_EOI, 0);
    }
}

/*
 * Masks and ACKs an IRQ
 */
unsafe fn edge_mask_and_ack_msc_irq(d: *mut irq_data) {
    let irq = (*d).irq;

    mask_msc_irq(d);
    if !cpu_has_veic {
        mscic_write(MSC01_IC_EOI, 0);
    } else {
        let mut r: u32 = 0;
        mscic_read(MSC01_IC_SUP + irq * 8, &mut r);
        mscic_write(MSC01_IC_SUP + irq * 8, r | !MSC01_IC_SUP_EDGE_BIT);
        mscic_write(MSC01_IC_SUP + irq * 8, r);
    }
}

/*
 * Interrupt handler for interrupts coming from SOC-it.
 */
pub unsafe fn ll_msc_irq() {
    let mut irq: u32 = 0;

    /* read the interrupt vector register */
    mscic_read(MSC01_IC_VEC, &mut irq);
    if irq < 64 {
        do_IRQ(irq + IRQ_BASE);
    } else {
        /* Ignore spurious interrupt */
    }
}

unsafe fn msc_bind_eic_interrupt(irq: ::core::ffi::c_int, set: ::core::ffi::c_int) {
    mscic_write(
        MSC01_IC_RAMW,
        ((irq as u32) << MSC01_IC_RAMW_ADDR_SHF) | ((set as u32) << MSC01_IC_RAMW_DATA_SHF),
    );
}

static mut MSC_LEVELIRQ_TYPE: irq_chip = irq_chip {
    name: "SOC-it-Level",
    irq_ack: Some(level_mask_and_ack_msc_irq),
    irq_mask: Some(mask_msc_irq),
    irq_mask_ack: Some(level_mask_and_ack_msc_irq),
    irq_unmask: Some(unmask_msc_irq),
    irq_eoi: Some(unmask_msc_irq),
};

static mut MSC_EDGEIRQ_TYPE: irq_chip = irq_chip {
    name: "SOC-it-Edge",
    irq_ack: Some(edge_mask_and_ack_msc_irq),
    irq_mask: Some(mask_msc_irq),
    irq_mask_ack: Some(edge_mask_and_ack_msc_irq),
    irq_unmask: Some(unmask_msc_irq),
    irq_eoi: Some(unmask_msc_irq),
};

pub unsafe fn init_msc_irqs(
    icubase: ::core::ffi::c_ulong,
    irqbase: ::core::ffi::c_uint,
    mut imp: *mut msc_irqmap_t,
    mut nirq: ::core::ffi::c_int,
) {
    _ICCTRL_MSC = ioremap(icubase, 0x40000) as ::core::ffi::c_ulong;

    /* Reset interrupt controller - initialises all registers to 0 */
    mscic_write(MSC01_IC_RST, MSC01_IC_RST_RST_BIT);

    board_bind_eic_interrupt = Some(msc_bind_eic_interrupt);

    while nirq > 0 {
        let n = (*imp).im_irq;

        match (*imp).im_type {
            MSC01_IRQ_EDGE => {
                irq_set_chip_and_handler_name(
                    irqbase + n,
                    &mut MSC_EDGEIRQ_TYPE,
                    handle_edge_irq,
                    "edge",
                );
                if cpu_has_veic {
                    mscic_write(MSC01_IC_SUP + n * 8, MSC01_IC_SUP_EDGE_BIT);
                } else {
                    mscic_write(MSC01_IC_SUP + n * 8, MSC01_IC_SUP_EDGE_BIT | (*imp).im_lvl);
                }
            }
            MSC01_IRQ_LEVEL => {
                irq_set_chip_and_handler_name(
                    irqbase + n,
                    &mut MSC_LEVELIRQ_TYPE,
                    handle_level_irq,
                    "level",
                );
                if cpu_has_veic {
                    mscic_write(MSC01_IC_SUP + n * 8, 0);
                } else {
                    mscic_write(MSC01_IC_SUP + n * 8, (*imp).im_lvl);
                }
            }
            _ => {}
        }

        nirq -= 1;
        imp = imp.add(1);
    }

    IRQ_BASE = irqbase;

    mscic_write(MSC01_IC_GENA, MSC01_IC_GENA_GENA_BIT); /* Enable interrupt generation */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
