/*
 * intc2.c  -- support for the 2nd INTC controller of the 525x
 *
 * (C) Copyright 2012, Steven King <sfking@fdwdc.com>
 * (C) Copyright 2009, Greg Ungerer <gerg@snapgear.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

use crate::{
    handle_edge_irq, irq_data, irq_chip, irq_set_chip, irq_set_handler,
    irqd_get_trigger_type, mcf_read32, mcf_write32, MCFINTC2_INTBASE,
    MCFINTC2_VECBASE, MCF_IRQ_GPIO0, MCFSIM2_GPIOINTCLEAR,
    MCFSIM2_GPIOINTENABLE, IRQ_TYPE_EDGE_BOTH, IRQ_TYPE_EDGE_FALLING,
    IRQ_TYPE_EDGE_RISING,
};

unsafe fn intc2_irq_gpio_mask(d: *mut irq_data) {
    let mut imr: u32 = mcf_read32(MCFSIM2_GPIOINTENABLE);
    let trigger_type: u32 = irqd_get_trigger_type(d);
    let irq: i32 = (*d).irq - MCF_IRQ_GPIO0;

    if trigger_type & IRQ_TYPE_EDGE_RISING != 0 {
        imr &= !(0x001u32 << irq);
    }
    if trigger_type & IRQ_TYPE_EDGE_FALLING != 0 {
        imr &= !(0x100u32 << irq);
    }
    mcf_write32(imr, MCFSIM2_GPIOINTENABLE);
}

unsafe fn intc2_irq_gpio_unmask(d: *mut irq_data) {
    let mut imr: u32 = mcf_read32(MCFSIM2_GPIOINTENABLE);
    let trigger_type: u32 = irqd_get_trigger_type(d);
    let irq: i32 = (*d).irq - MCF_IRQ_GPIO0;

    if trigger_type & IRQ_TYPE_EDGE_RISING != 0 {
        imr |= 0x001u32 << irq;
    }
    if trigger_type & IRQ_TYPE_EDGE_FALLING != 0 {
        imr |= 0x100u32 << irq;
    }
    mcf_write32(imr, MCFSIM2_GPIOINTENABLE);
}

unsafe fn intc2_irq_gpio_ack(d: *mut irq_data) {
    let mut imr: u32 = 0;
    let trigger_type: u32 = irqd_get_trigger_type(d);
    let irq: i32 = (*d).irq - MCF_IRQ_GPIO0;

    if trigger_type & IRQ_TYPE_EDGE_RISING != 0 {
        imr |= 0x001u32 << irq;
    }
    if trigger_type & IRQ_TYPE_EDGE_FALLING != 0 {
        imr |= 0x100u32 << irq;
    }
    mcf_write32(imr, MCFSIM2_GPIOINTCLEAR);
}

unsafe fn intc2_irq_gpio_set_type(_d: *mut irq_data, f: u32) -> i32 {
    if f & !IRQ_TYPE_EDGE_BOTH != 0 {
        return -22; // -EINVAL
    }
    0
}

static mut intc2_irq_gpio_chip: irq_chip = irq_chip {
    name: "CF-INTC2",
    irq_mask: Some(intc2_irq_gpio_mask),
    irq_unmask: Some(intc2_irq_gpio_unmask),
    irq_ack: Some(intc2_irq_gpio_ack),
    irq_set_type: Some(intc2_irq_gpio_set_type),
};

unsafe fn mcf_intc2_init() -> i32 {
    let mut irq: i32;

    /* set the interrupt base for the second interrupt controller */
    mcf_write32(MCFINTC2_VECBASE, MCFINTC2_INTBASE);

    /* GPIO interrupt sources */
    irq = MCF_IRQ_GPIO0;
    while irq <= MCF_IRQ_GPIO6 {
        irq_set_chip(irq, &raw mut intc2_irq_gpio_chip);
        irq_set_handler(irq, handle_edge_irq);
        irq += 1;
    }

    0
}

// arch_initcall(mcf_intc2_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
