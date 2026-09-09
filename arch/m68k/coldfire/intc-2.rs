/*
 * intc-2.c
 *
 * General interrupt controller code for the many ColdFire cores that use
 * interrupt controllers with 63 interrupt sources, organized as 56 fully-
 * programmable + 7 fixed-level interrupt sources. This includes the 523x
 * family, the 5270, 5271, 5274, 5275, and the 528x family which have two such
 * controllers, and the 547x and 548x families which have only one of them.
 *
 * The external 7 fixed interrupts are part of the Edge Port unit of these
 * ColdFire parts. They can be configured as level or edge triggered.
 *
 * (C) Copyright 2009-2011, Greg Ungerer <gerg@snapgear.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation.

const fn mcfsim_icr_level(l: u8) -> u8 { l << 3 }
const fn mcfsim_icr_pri(p: u8) -> u8 { p }

const EINT0: u32 = 64;
const EINT1: u32 = 65;
const EINT7: u32 = 71;

// #ifdef MCFICM_INTC1
const NR_VECS: u32 = 128;
// #else
// const NR_VECS: u32 = 64;
// #endif

#[repr(C)]
pub struct irq_data { pub irq: u32 }

extern "C" {
    static MCFICM_INTC0: usize;
    static MCFICM_INTC1: usize;
    static MCFINTC_IMRH: usize;
    static MCFINTC_IMRL: usize;
    static MCFINTC_ICR0: usize;
    static MCFEPORT_EPFR: usize;
    static MCFEPORT_EPDDR: usize;
    static MCFEPORT_EPIER: usize;
    static MCFEPORT_EPPAR: usize;
    static MCFINT_VECBASE: u32;
    fn mcf_read32(addr: usize) -> u32;
    fn mcf_write32(val: u32, addr: usize);
    fn mcf_read8(addr: usize) -> u8;
    fn mcf_write8(val: u8, addr: usize);
    fn mcf_read16(addr: usize) -> u16;
    fn mcf_write16(val: u16, addr: usize);
    fn irq_set_handler(irq: u32, handler: unsafe extern "C" fn(*mut irq_data));
    fn irq_set_chip(irq: u32, chip: *const irq_chip);
    fn irq_set_irq_type(irq: u32, ty: u32);
    static handle_edge_irq: unsafe extern "C" fn(*mut irq_data);
    static handle_level_irq: unsafe extern "C" fn(*mut irq_data);
}

const IRQ_TYPE_EDGE_RISING: u32 = 1;
const IRQ_TYPE_EDGE_FALLING: u32 = 2;
const IRQ_TYPE_EDGE_BOTH: u32 = 3;
const IRQ_TYPE_LEVEL_HIGH: u32 = 4;

#[repr(C)]
pub struct irq_chip {
    pub name: *const u8,
    pub irq_startup: Option<unsafe extern "C" fn(*mut irq_data) -> u32>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_set_type: Option<unsafe extern "C" fn(*mut irq_data, u32) -> i32>,
}

unsafe extern "C" fn intc_irq_mask(d: *mut irq_data) {
    let irq = (*d).irq - MCFINT_VECBASE;
    let mut imraddr: usize;
    let imrbit: u32;

    // #ifdef MCFICM_INTC1
    imraddr = if (irq & 0x40) != 0 { MCFICM_INTC1 } else { MCFICM_INTC0 };
    // #else
    // imraddr = MCFICM_INTC0;
    // #endif
    imraddr += if (irq & 0x20) != 0 { MCFINTC_IMRH } else { MCFINTC_IMRL };
    imrbit = 0x1u32 << (irq & 0x1f);

    let val = mcf_read32(imraddr);
    mcf_write32(val | imrbit, imraddr);
}

unsafe extern "C" fn intc_irq_unmask(d: *mut irq_data) {
    let irq = (*d).irq - MCFINT_VECBASE;
    let mut imraddr: usize;
    let mut imrbit: u32;

    // #ifdef MCFICM_INTC1
    imraddr = if (irq & 0x40) != 0 { MCFICM_INTC1 } else { MCFICM_INTC0 };
    // #else
    // imraddr = MCFICM_INTC0;
    // #endif
    imraddr += if (irq & 0x20) != 0 { MCFINTC_IMRH } else { MCFINTC_IMRL };
    imrbit = 0x1u32 << (irq & 0x1f);

    /* Don't set the "maskall" bit! */
    if (irq & 0x20) == 0 { imrbit |= 0x1; }

    let val = mcf_read32(imraddr);
    mcf_write32(val & !imrbit, imraddr);
}

unsafe extern "C" fn intc_irq_ack(d: *mut irq_data) {
    let irq = (*d).irq;
    mcf_write8((0x1u32 << (irq - EINT0)) as u8, MCFEPORT_EPFR);
}

static mut intc_intpri: u8 = mcfsim_icr_level(6) | mcfsim_icr_pri(6);

unsafe extern "C" fn intc_irq_startup(d: *mut irq_data) -> u32 {
    let mut irq = (*d).irq - MCFINT_VECBASE;
    let mut icraddr: usize;

    // #ifdef MCFICM_INTC1
    icraddr = if (irq & 0x40) != 0 { MCFICM_INTC1 } else { MCFICM_INTC0 };
    // #else
    // icraddr = MCFICM_INTC0;
    // #endif
    icraddr += MCFINTC_ICR0 + (irq & 0x3f) as usize;
    if mcf_read8(icraddr) == 0 {
        mcf_write8(intc_intpri, icraddr);
        intc_intpri = intc_intpri.wrapping_sub(1);
    }

    irq = (*d).irq;
    if irq >= EINT1 && irq <= EINT7 {
        let line = irq - EINT0;
        let v = mcf_read8(MCFEPORT_EPDDR);
        mcf_write8(v & !((0x1u32 << line) as u8), MCFEPORT_EPDDR);
        let v = mcf_read8(MCFEPORT_EPIER);
        mcf_write8(v | ((0x1u32 << line) as u8), MCFEPORT_EPIER);
    }

    intc_irq_unmask(d);
    0
}

unsafe extern "C" fn intc_irq_set_type(d: *mut irq_data, ty: u32) -> i32 {
    let mut irq = (*d).irq;
    let tb: u16 = match ty {
        IRQ_TYPE_EDGE_RISING => 0x1,
        IRQ_TYPE_EDGE_FALLING => 0x2,
        IRQ_TYPE_EDGE_BOTH => 0x3,
        _ => 0,
    };
    if tb != 0 { irq_set_handler(irq, handle_edge_irq); }
    irq -= EINT0;
    let pa = mcf_read16(MCFEPORT_EPPAR);
    let pa = (pa & !((0x3u16) << (irq * 2))) | (tb << (irq * 2));
    mcf_write16(pa, MCFEPORT_EPPAR);
    0
}

static mut intc_irq_chip: irq_chip = irq_chip {
    name: b"CF-INTC\0".as_ptr(), irq_startup: Some(intc_irq_startup),
    irq_mask: Some(intc_irq_mask), irq_unmask: Some(intc_irq_unmask),
    irq_ack: None, irq_set_type: None,
};

static mut intc_irq_chip_edge_port: irq_chip = irq_chip {
    name: b"CF-INTC-EP\0".as_ptr(), irq_startup: Some(intc_irq_startup),
    irq_mask: Some(intc_irq_mask), irq_unmask: Some(intc_irq_unmask),
    irq_ack: Some(intc_irq_ack), irq_set_type: Some(intc_irq_set_type),
};

pub unsafe extern "C" fn init_IRQ() {
    mcf_write32(0x1, MCFICM_INTC0 + MCFINTC_IMRL);
    // #ifdef MCFICM_INTC1
    mcf_write32(0x1, MCFICM_INTC1 + MCFINTC_IMRL);
    // #endif
    let mut irq = MCFINT_VECBASE;
    while irq < MCFINT_VECBASE + NR_VECS {
        if irq >= EINT1 && irq <= EINT7 {
            irq_set_chip(irq, &raw const intc_irq_chip_edge_port);
        } else {
            irq_set_chip(irq, &raw const intc_irq_chip);
        }
        irq_set_irq_type(irq, IRQ_TYPE_LEVEL_HIGH);
        irq_set_handler(irq, handle_level_irq);
        irq += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
