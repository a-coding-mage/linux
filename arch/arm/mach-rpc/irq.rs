// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the surrounding kernel translation unit.

const STAT: usize = 0x00;
const REQ: usize = 0x04;
const CLR: usize = 0x04;
const MASK: usize = 0x08;

static IRQ_PRIO_H: [u8; 256] = [
     0, 8, 9, 8,10,10,10,10,11,11,11,11,10,10,10,10,
    12, 8, 9, 8,10,10,10,10,11,11,11,11,10,10,10,10,
    13,13,13,13,10,10,10,10,11,11,11,11,10,10,10,10,
    13,13,13,13,10,10,10,10,11,11,11,11,10,10,10,10,
    14,14,14,14,10,10,10,10,11,11,11,11,10,10,10,10,
    14,14,14,14,10,10,10,10,11,11,11,11,10,10,10,10,
    13,13,13,13,10,10,10,10,11,11,11,11,10,10,10,10,
    13,13,13,13,10,10,10,10,11,11,11,11,10,10,10,10,
    15,15,15,15,10,10,10,10,11,11,11,11,10,10,10,10,
    15,15,15,15,10,10,10,10,11,11,11,11,10,10,10,10,
    13,13,13,13,10,10,10,10,11,11,11,11,10,10,10,10,
    13,13,13,13,10,10,10,10,11,11,11,11,10,10,10,10,
    15,15,15,15,10,10,10,10,11,11,11,11,10,10,10,10,
    15,15,15,15,10,10,10,10,11,11,11,11,10,10,10,10,
    13,13,13,13,10,10,10,10,11,11,11,11,10,10,10,10,
    13,13,13,13,10,10,10,10,11,11,11,11,10,10,10,10,
];

static IRQ_PRIO_D: [u8; 256] = [
     0,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    20,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    21,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    21,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    22,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    22,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    21,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    21,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    23,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    23,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    21,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    21,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    22,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    22,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    21,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
    21,16,17,16,18,16,17,16,19,16,17,16,18,16,17,16,
];

static IRQ_PRIO_L: [u8; 256] = [
     0, 0, 1, 0, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3,
     4, 0, 1, 0, 2, 2, 2, 2, 3, 3, 3, 3, 3, 3, 3, 3,
     5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
     5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
     6, 6, 6, 6, 6, 6, 6, 6, 3, 3, 3, 3, 3, 3, 3, 3,
     6, 6, 6, 6, 6, 6, 6, 6, 3, 3, 3, 3, 3, 3, 3, 3,
     5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
     5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5,
     7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
     7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
     7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
     7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
     7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
     7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
     7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
     7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7, 7,
];

unsafe fn iomd_get_irq_nr() -> i32 {
    let mut irq: i32;
    let mut reg: u8;
    reg = readb((IOC_BASE + IOMD_IRQREQB) as *const core::ffi::c_void);
    irq = IRQ_PRIO_H[reg as usize] as i32;
    if irq != 0 { return irq; }
    reg = readb((IOC_BASE + IOMD_DMAREQ) as *const core::ffi::c_void);
    irq = IRQ_PRIO_D[reg as usize] as i32;
    if irq != 0 { return irq; }
    reg = readb((IOC_BASE + IOMD_IRQREQA) as *const core::ffi::c_void);
    irq = IRQ_PRIO_L[reg as usize] as i32;
    if irq != 0 { return irq; }
    0
}

unsafe fn iomd_handle_irq(regs: *mut pt_regs) {
    let mut irq: i32;
    loop {
        irq = iomd_get_irq_nr();
        if irq != 0 { generic_handle_irq(irq as u32); } else { break; }
    }
}

unsafe fn iomd_get_base(d: *mut irq_data) -> *mut core::ffi::c_void {
    irq_data_get_irq_chip_data(d) as *mut core::ffi::c_void
}

unsafe fn iomd_set_base_mask(irq: u32, base: *mut core::ffi::c_void, mask: u32) {
    let d = irq_get_irq_data(irq);
    (*d).mask = mask;
    irq_set_chip_data(irq, base as *mut core::ffi::c_void);
}

unsafe fn iomd_irq_mask_ack(d: *mut irq_data) {
    let base = iomd_get_base(d);
    let mask = (*d).mask;
    let val = readb((base as usize + MASK) as *const core::ffi::c_void);
    writeb(val & !(mask as u8), (base as usize + MASK) as *mut core::ffi::c_void);
    writeb(mask as u8, (base as usize + CLR) as *mut core::ffi::c_void);
}

unsafe fn iomd_irq_mask(d: *mut irq_data) {
    let base = iomd_get_base(d);
    let mask = (*d).mask;
    let val = readb((base as usize + MASK) as *const core::ffi::c_void);
    writeb(val & !(mask as u8), (base as usize + MASK) as *mut core::ffi::c_void);
}

unsafe fn iomd_irq_unmask(d: *mut irq_data) {
    let base = iomd_get_base(d);
    let mask = (*d).mask;
    let val = readb((base as usize + MASK) as *const core::ffi::c_void);
    writeb(val | mask as u8, (base as usize + MASK) as *mut core::ffi::c_void);
}

static mut IOMD_CHIP_CLR: irq_chip = irq_chip {
    irq_mask_ack: Some(iomd_irq_mask_ack), irq_mask: Some(iomd_irq_mask), irq_unmask: Some(iomd_irq_unmask),
};
static mut IOMD_CHIP_NOCLR: irq_chip = irq_chip {
    irq_mask_ack: None, irq_mask: Some(iomd_irq_mask), irq_unmask: Some(iomd_irq_unmask),
};

extern "C" {
    static mut rpc_default_fiq_start: u8;
    static mut rpc_default_fiq_end: u8;
}

pub unsafe fn rpc_init_irq() {
    let (mut clr, mut set): (u32, u32);
    iomd_writeb(0, IOMD_IRQMASKA); iomd_writeb(0, IOMD_IRQMASKB);
    iomd_writeb(0, IOMD_FIQMASK); iomd_writeb(0, IOMD_DMAMASK);
    set_fiq_handler(&raw mut rpc_default_fiq_start, (&raw mut rpc_default_fiq_end as usize).wrapping_sub(&raw mut rpc_default_fiq_start as usize));
    set_handle_irq(iomd_handle_irq);
    for irq in 0..NR_IRQS {
        clr = IRQ_NOREQUEST; set = 0;
        if irq <= 6 || (irq >= 9 && irq <= 15) { clr |= IRQ_NOPROBE; }
        if irq == 21 || (irq >= 16 && irq <= 19) || irq == IRQ_KEYBOARDTX { set |= IRQ_NOAUTOEN; }
        match irq {
            0..=7 => { irq_set_chip_and_handler(irq, &raw mut IOMD_CHIP_CLR, handle_level_irq); irq_modify_status(irq, clr, set); iomd_set_base_mask(irq, (IOMD_BASE + IOMD_IRQSTATA) as *mut _, 1u32 << irq); }
            8..=15 => { irq_set_chip_and_handler(irq, &raw mut IOMD_CHIP_NOCLR, handle_level_irq); irq_modify_status(irq, clr, set); iomd_set_base_mask(irq, (IOMD_BASE + IOMD_IRQSTATB) as *mut _, 1u32 << (irq - 8)); }
            16..=21 => { irq_set_chip_and_handler(irq, &raw mut IOMD_CHIP_NOCLR, handle_level_irq); irq_modify_status(irq, clr, set); iomd_set_base_mask(irq, (IOMD_BASE + IOMD_DMASTAT) as *mut _, 1u32 << (irq - 16)); }
            64..=71 => { irq_set_chip(irq, &raw mut IOMD_CHIP_NOCLR); irq_modify_status(irq, clr, set); iomd_set_base_mask(irq, (IOMD_BASE + IOMD_FIQSTAT) as *mut _, 1u32 << (irq - 64)); }
            _ => {}
        }
    }
    init_FIQ(FIQ_START);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
