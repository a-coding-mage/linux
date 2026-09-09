/*
 * intc-simr.c
 *
 * Interrupt controller code for the ColdFire 5208, 5207 & 532x parts.
 *
 * (C) Copyright 2009-2011, Greg Ungerer <gerg@snapgear.com>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

// Linux and ColdFire declarations supplied by the surrounding kernel.

#[cfg(feature = "CONFIG_M520x")]
const EINT0: u32 = 64;
#[cfg(feature = "CONFIG_M520x")]
const EINT1: u32 = 65;
#[cfg(feature = "CONFIG_M520x")]
const EINT4: u32 = 66;
#[cfg(feature = "CONFIG_M520x")]
const EINT7: u32 = 67;

#[cfg(feature = "CONFIG_M520x")]
static IRQEBITMAP: [u32; 4] = [0, 1, 4, 7];

#[cfg(feature = "CONFIG_M520x")]
#[inline]
unsafe fn irq2ebit(irq: u32) -> u32 {
    IRQEBITMAP[(irq - EINT0) as usize]
}

#[cfg(not(feature = "CONFIG_M520x"))]
const EINT0: u32 = 64;
#[cfg(not(feature = "CONFIG_M520x"))]
const EINT1: u32 = 65;
#[cfg(not(feature = "CONFIG_M520x"))]
const EINT7: u32 = 71;

#[cfg(not(feature = "CONFIG_M520x"))]
#[inline]
unsafe fn irq2ebit(irq: u32) -> u32 {
    irq - EINT0
}

extern "C" {
    fn mcf_write8(value: u8, address: usize);
    fn mcf_read8(address: usize) -> u8;
    fn mcf_write16(value: u16, address: usize);
    fn mcf_read16(address: usize) -> u16;
    fn irq_set_handler(irq: u32, handler: unsafe extern "C" fn());
    fn irq_set_chip(irq: i32, chip: *mut irq_chip);
    fn irq_set_irq_type(irq: i32, irq_type: u32);
    static mut MCFINT_VECBASE: u32;
    static mut MCFINTC0_SIMR: usize;
    static mut MCFINTC1_SIMR: usize;
    static mut MCFINTC2_SIMR: usize;
    static mut MCFINTC0_CIMR: usize;
    static mut MCFINTC1_CIMR: usize;
    static mut MCFINTC2_CIMR: usize;
    static mut MCFINTC0_ICR0: usize;
    static mut MCFINTC1_ICR0: usize;
    static mut MCFINTC2_ICR0: usize;
    static mut MCFEPORT_EPFR: usize;
    static mut MCFEPORT_EPDDR: usize;
    static mut MCFEPORT_EPIER: usize;
    static mut MCFEPORT_EPPAR: usize;
}

#[repr(C)]
pub struct irq_data { pub irq: u32 }

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
    if MCFINTC2_SIMR != 0 && irq > 127 { mcf_write8((irq - 128) as u8, MCFINTC2_SIMR); }
    else if MCFINTC1_SIMR != 0 && irq > 63 { mcf_write8((irq - 64) as u8, MCFINTC1_SIMR); }
    else { mcf_write8(irq as u8, MCFINTC0_SIMR); }
}

unsafe extern "C" fn intc_irq_unmask(d: *mut irq_data) {
    let irq = (*d).irq - MCFINT_VECBASE;
    if MCFINTC2_CIMR != 0 && irq > 127 { mcf_write8((irq - 128) as u8, MCFINTC2_CIMR); }
    else if MCFINTC1_CIMR != 0 && irq > 63 { mcf_write8((irq - 64) as u8, MCFINTC1_CIMR); }
    else { mcf_write8(irq as u8, MCFINTC0_CIMR); }
}

unsafe extern "C" fn intc_irq_ack(d: *mut irq_data) {
    mcf_write8((1u8).wrapping_shl(irq2ebit((*d).irq)), MCFEPORT_EPFR);
}

unsafe extern "C" fn intc_irq_startup(d: *mut irq_data) -> u32 {
    let mut irq = (*d).irq;
    if irq >= EINT1 && irq <= EINT7 {
        let ebit = irq2ebit(irq);
        let v = mcf_read8(MCFEPORT_EPDDR);
        mcf_write8(v & !(1u8 << ebit), MCFEPORT_EPDDR);
        let v = mcf_read8(MCFEPORT_EPIER);
        mcf_write8(v | (1u8 << ebit), MCFEPORT_EPIER);
    }
    irq -= MCFINT_VECBASE;
    if MCFINTC2_ICR0 != 0 && irq > 127 { mcf_write8(5, MCFINTC2_ICR0 + (irq - 128) as usize); }
    else if MCFINTC1_ICR0 != 0 && irq > 63 { mcf_write8(5, MCFINTC1_ICR0 + (irq - 64) as usize); }
    else { mcf_write8(5, MCFINTC0_ICR0 + irq as usize); }
    intc_irq_unmask(d); 0
}

unsafe extern "C" fn intc_irq_set_type(d: *mut irq_data, type_: u32) -> i32 {
    let tb: u16 = match type_ { IRQ_TYPE_EDGE_RISING => 1, IRQ_TYPE_EDGE_FALLING => 2, IRQ_TYPE_EDGE_BOTH => 3, _ => 0 };
    if tb != 0 { irq_set_handler((*d).irq, handle_edge_irq); }
    let ebit = irq2ebit((*d).irq) * 2;
    let pa = (mcf_read16(MCFEPORT_EPPAR) & !(3u16 << ebit)) | (tb << ebit);
    mcf_write16(pa, MCFEPORT_EPPAR); 0
}

static mut intc_irq_chip: irq_chip = irq_chip { name: b"CF-INTC\0".as_ptr(), irq_startup: Some(intc_irq_startup), irq_mask: Some(intc_irq_mask), irq_unmask: Some(intc_irq_unmask), irq_ack: None, irq_set_type: None };
static mut intc_irq_chip_edge_port: irq_chip = irq_chip { name: b"CF-INTC-EP\0".as_ptr(), irq_startup: Some(intc_irq_startup), irq_mask: Some(intc_irq_mask), irq_unmask: Some(intc_irq_unmask), irq_ack: Some(intc_irq_ack), irq_set_type: Some(intc_irq_set_type) };

const IRQ_TYPE_EDGE_RISING: u32 = 1;
const IRQ_TYPE_EDGE_FALLING: u32 = 2;
const IRQ_TYPE_EDGE_BOTH: u32 = 3;
const IRQ_TYPE_LEVEL_HIGH: u32 = 4;
extern "C" { fn handle_edge_irq(); fn handle_level_irq(); }

pub unsafe extern "C" fn init_IRQ() {
    mcf_write8(0xff, MCFINTC0_SIMR);
    if MCFINTC1_SIMR != 0 { mcf_write8(0xff, MCFINTC1_SIMR); }
    if MCFINTC2_SIMR != 0 { mcf_write8(0xff, MCFINTC2_SIMR); }
    let eirq = MCFINT_VECBASE + 64 + if MCFINTC1_ICR0 != 0 { 64 } else { 0 } + if MCFINTC2_ICR0 != 0 { 64 } else { 0 };
    let mut irq = MCFINT_VECBASE;
    while irq < eirq {
        if irq >= EINT1 && irq <= EINT7 { irq_set_chip(irq as i32, &mut intc_irq_chip_edge_port); }
        else { irq_set_chip(irq as i32, &mut intc_irq_chip); }
        irq_set_irq_type(irq as i32, IRQ_TYPE_LEVEL_HIGH);
        irq_set_handler(irq, handle_level_irq);
        irq += 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
