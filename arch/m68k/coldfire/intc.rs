/*
 * intc.c  -- support for the old ColdFire interrupt controller
 *
 * (C) Copyright 2009, Greg Ungerer <gerg@snapgear.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

/* C dependencies supplied by the surrounding kernel translation. */

pub static mut mcf_irq2imr: [u8; NR_IRQS as usize] = [0; NR_IRQS as usize];

/* Define the minimum and maximum external interrupt numbers. */
const EIRQ1: i32 = 25;
const EIRQ7: i32 = 31;

#[cfg(MCFSIM_IMR_IS_16BITS)]
pub unsafe fn mcf_setimr(index: i32) {
    let imr: u16 = mcf_read16(MCFSIM_IMR);
    mcf_write16(imr | (0x1u16 << index), MCFSIM_IMR);
}

#[cfg(MCFSIM_IMR_IS_16BITS)]
pub unsafe fn mcf_clrimr(index: i32) {
    let imr: u16 = mcf_read16(MCFSIM_IMR);
    mcf_write16(imr & !(0x1u16 << index), MCFSIM_IMR);
}

#[cfg(MCFSIM_IMR_IS_16BITS)]
unsafe fn mcf_maskimr(mask: u32) {
    let mut imr: u16 = mcf_read16(MCFSIM_IMR);
    imr |= mask as u16;
    mcf_write16(imr, MCFSIM_IMR);
}

#[cfg(not(MCFSIM_IMR_IS_16BITS))]
pub unsafe fn mcf_setimr(index: i32) {
    let imr: u32 = mcf_read32(MCFSIM_IMR);
    mcf_write32(imr | (0x1u32 << index), MCFSIM_IMR);
}

#[cfg(not(MCFSIM_IMR_IS_16BITS))]
pub unsafe fn mcf_clrimr(index: i32) {
    let imr: u32 = mcf_read32(MCFSIM_IMR);
    mcf_write32(imr & !(0x1u32 << index), MCFSIM_IMR);
}

#[cfg(not(MCFSIM_IMR_IS_16BITS))]
unsafe fn mcf_maskimr(mask: u32) {
    let mut imr: u32 = mcf_read32(MCFSIM_IMR);
    imr |= mask;
    mcf_write32(imr, MCFSIM_IMR);
}

pub unsafe fn mcf_autovector(irq: i32) {
    #[cfg(MCFSIM_AVR)]
    if (irq >= EIRQ1) && (irq <= EIRQ7) {
        let mut avec: u8 = mcf_read8(MCFSIM_AVR);
        avec |= 0x1u8 << (irq - EIRQ1 + 1);
        mcf_write8(avec, MCFSIM_AVR);
    }
}

unsafe fn intc_irq_mask(d: *mut irq_data) {
    let irq = (*d).irq as usize;
    if mcf_irq2imr[irq] != 0 {
        mcf_setimr(mcf_irq2imr[irq] as i32);
    }
}

unsafe fn intc_irq_unmask(d: *mut irq_data) {
    let irq = (*d).irq as usize;
    if mcf_irq2imr[irq] != 0 {
        mcf_clrimr(mcf_irq2imr[irq] as i32);
    }
}

unsafe fn intc_irq_set_type(_d: *mut irq_data, _type: u32) -> i32 {
    0
}

static mut intc_irq_chip: irq_chip = irq_chip {
    name: c"CF-INTC".as_ptr(),
    irq_mask: Some(intc_irq_mask),
    irq_unmask: Some(intc_irq_unmask),
    irq_set_type: Some(intc_irq_set_type),
};

pub unsafe fn init_IRQ() {
    mcf_maskimr(0xffffffff);

    let mut irq: i32 = 0;
    while irq < NR_IRQS {
        irq_set_chip(irq, &raw mut intc_irq_chip);
        irq_set_irq_type(irq, IRQ_TYPE_LEVEL_HIGH);
        irq_set_handler(irq, handle_level_irq);
        irq += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
