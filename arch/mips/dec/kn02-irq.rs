// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *	DECstation 5000/200 (KN02) Control and Status Register
 *	interrupts.
 *
 *	Copyright (c) 2002, 2003, 2005  Maciej W. Rozycki
 */

// Linux kernel dependencies: init, irq, types, and asm/dec/kn02.

use core::ffi::c_int;

#[repr(C)]
pub struct irq_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct irq_chip {
    pub name: *const u8,
    pub irq_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
}

unsafe extern "C" {
    fn CKSEG1ADDR(address: usize) -> usize;
    fn iob();
    fn irq_set_chip_and_handler(
        irq: c_int,
        chip: *mut irq_chip,
        handler: unsafe extern "C" fn(),
    );
    fn handle_level_irq();
    fn irq_data_irq(d: *mut irq_data) -> c_int;
}

unsafe extern "C" {
    static KN02_SLOT_BASE: usize;
    static KN02_CSR: usize;
    static KN02_CSR_IOINTEN: u32;
    static KN02_IRQ_LINES: c_int;
}

/*
 * Bits 7:0 of the Control Register are write-only -- the
 * corresponding bits of the Status Register have a different
 * meaning.  Hence we use a cache.  It speeds up things a bit
 * as well.
 *
 * There is no default value -- it has to be initialized.
 */
#[no_mangle]
pub static mut cached_kn02_csr: u32 = 0;

static mut kn02_irq_base: c_int = 0;

unsafe extern "C" fn unmask_kn02_irq(d: *mut irq_data) {
    let csr = CKSEG1ADDR(KN02_SLOT_BASE + KN02_CSR) as *mut u32;

    cached_kn02_csr |= 1u32 << (irq_data_irq(d) - kn02_irq_base + 16);
    core::ptr::write_volatile(csr, cached_kn02_csr);
}

unsafe extern "C" fn mask_kn02_irq(d: *mut irq_data) {
    let csr = CKSEG1ADDR(KN02_SLOT_BASE + KN02_CSR) as *mut u32;

    cached_kn02_csr &= !(1u32 << (irq_data_irq(d) - kn02_irq_base + 16));
    core::ptr::write_volatile(csr, cached_kn02_csr);
}

unsafe extern "C" fn ack_kn02_irq(d: *mut irq_data) {
    mask_kn02_irq(d);
    iob();
}

static mut kn02_irq_type: irq_chip = irq_chip {
    name: b"KN02-CSR\0".as_ptr(),
    irq_ack: Some(ack_kn02_irq),
    irq_mask: Some(mask_kn02_irq),
    irq_mask_ack: Some(ack_kn02_irq),
    irq_unmask: Some(unmask_kn02_irq),
};

pub unsafe extern "C" fn init_kn02_irqs(base: c_int) {
    let csr = CKSEG1ADDR(KN02_SLOT_BASE + KN02_CSR) as *mut u32;
    let mut i: c_int;

    /* Mask interrupts. */
    cached_kn02_csr &= !KN02_CSR_IOINTEN;
    core::ptr::write_volatile(csr, cached_kn02_csr);
    iob();

    i = base;
    while i < base + KN02_IRQ_LINES {
        irq_set_chip_and_handler(i, &raw mut kn02_irq_type, handle_level_irq);
        i += 1;
    }

    kn02_irq_base = base;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
