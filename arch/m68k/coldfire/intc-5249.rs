/*
 * intc2.c  -- support for the 2nd INTC controller of the 5249
 *
 * (C) Copyright 2009, Greg Ungerer <gerg@snapgear.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation.
use crate::linux::types::u32;

extern "C" {
    fn mcf_read32(addr: u32) -> u32;
    fn mcf_write32(value: u32, addr: u32);
    fn irq_set_chip(irq: i32, chip: *mut irq_chip);
    fn irq_set_handler(irq: i32, handler: unsafe extern "C" fn(*mut irq_data));
    fn handle_edge_irq(data: *mut irq_data);
}

#[repr(C)]
pub struct irq_data {
    pub irq: i32,
}

#[repr(C)]
pub struct irq_chip {
    pub name: *const u8,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
}

extern "C" {
    static MCFSIM2_GPIOINTENABLE: u32;
    static MCFSIM2_GPIOINTCLEAR: u32;
    static MCF_IRQ_GPIO0: i32;
    static MCF_IRQ_GPIO7: i32;
}

unsafe extern "C" fn intc2_irq_gpio_mask(d: *mut irq_data) {
    let mut imr: u32;
    imr = mcf_read32(MCFSIM2_GPIOINTENABLE);
    imr &= !(0x1u32 << ((*d).irq - MCF_IRQ_GPIO0));
    mcf_write32(imr, MCFSIM2_GPIOINTENABLE);
}

unsafe extern "C" fn intc2_irq_gpio_unmask(d: *mut irq_data) {
    let mut imr: u32;
    imr = mcf_read32(MCFSIM2_GPIOINTENABLE);
    imr |= 0x1u32 << ((*d).irq - MCF_IRQ_GPIO0);
    mcf_write32(imr, MCFSIM2_GPIOINTENABLE);
}

unsafe extern "C" fn intc2_irq_gpio_ack(d: *mut irq_data) {
    mcf_write32(0x1u32 << ((*d).irq - MCF_IRQ_GPIO0), MCFSIM2_GPIOINTCLEAR);
}

static mut intc2_irq_gpio_chip: irq_chip = irq_chip {
    name: b"CF-INTC2\0".as_ptr(),
    irq_mask: Some(intc2_irq_gpio_mask),
    irq_unmask: Some(intc2_irq_gpio_unmask),
    irq_ack: Some(intc2_irq_gpio_ack),
};

unsafe extern "C" fn mcf_intc2_init() -> i32 {
    let mut irq: i32;

    /* GPIO interrupt sources */
    irq = MCF_IRQ_GPIO0;
    while irq <= MCF_IRQ_GPIO7 {
        irq_set_chip(irq, &raw mut intc2_irq_gpio_chip);
        irq_set_handler(irq, handle_edge_irq);
        irq += 1;
    }

    0
}

// arch_initcall(mcf_intc2_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
