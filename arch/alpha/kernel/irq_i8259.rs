// SPDX-License-Identifier: GPL-2.0
/*
 *      linux/arch/alpha/kernel/irq_i8259.c
 *
 * This is the 'legacy' 8259A Programmable Interrupt Controller,
 * present in the majority of PC/AT boxes.
 *
 * Started hacking from linux-2.3.30pre6/arch/i386/kernel/i8259.c.
 */

// Linux and architecture dependencies supplied by other translated files.

/* Note mask bit is true for DISABLED irqs.  */
static mut cached_irq_mask: ::core::ffi::c_uint = 0xffff;
// DEFINE_RAW_SPINLOCK(i8259_irq_lock);
extern "C" {
    static mut i8259_irq_lock: RawSpinLock;
}

unsafe fn i8259_update_irq_hw(mut irq: ::core::ffi::c_uint, mut mask: ::core::ffi::c_ulong) {
    let mut port: ::core::ffi::c_int = 0x21;
    if irq & 8 != 0 {
        mask >>= 8;
    }
    if irq & 8 != 0 {
        port = 0xA1;
    }
    outb(mask as u8, port as u16);
}

pub unsafe extern "C" fn i8259a_enable_irq(d: *mut irq_data) {
    let mut flags: ::core::ffi::c_ulong = 0;

    raw_spin_lock_irqsave(&mut i8259_irq_lock, &mut flags);
    let irq = (*d).irq;
    cached_irq_mask &= !(1 << irq);
    i8259_update_irq_hw(irq, cached_irq_mask as ::core::ffi::c_ulong);
    raw_spin_unlock_irqrestore(&mut i8259_irq_lock, flags);
}

unsafe fn __i8259a_disable_irq(irq: ::core::ffi::c_uint) {
    cached_irq_mask |= 1 << irq;
    i8259_update_irq_hw(irq, cached_irq_mask as ::core::ffi::c_ulong);
}

pub unsafe extern "C" fn i8259a_disable_irq(d: *mut irq_data) {
    let mut flags: ::core::ffi::c_ulong = 0;

    raw_spin_lock_irqsave(&mut i8259_irq_lock, &mut flags);
    __i8259a_disable_irq((*d).irq);
    raw_spin_unlock_irqrestore(&mut i8259_irq_lock, flags);
}

pub unsafe extern "C" fn i8259a_mask_and_ack_irq(d: *mut irq_data) {
    let mut irq = (*d).irq;
    let mut flags: ::core::ffi::c_ulong = 0;

    raw_spin_lock_irqsave(&mut i8259_irq_lock, &mut flags);
    __i8259a_disable_irq(irq);

    /* Ack the interrupt making it the lowest priority.  */
    if irq >= 8 {
        outb((0xE0 | (irq - 8)) as u8, 0xa0);
        irq = 2;
    }
    outb((0xE0 | irq) as u8, 0x20);
    raw_spin_unlock_irqrestore(&mut i8259_irq_lock, flags);
}

#[repr(C)]
pub struct irq_chip {
    pub name: *const ::core::ffi::c_char,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
}

#[no_mangle]
pub static mut i8259a_irq_type: irq_chip = irq_chip {
    name: b"XT-PIC\0".as_ptr() as *const ::core::ffi::c_char,
    irq_unmask: Some(i8259a_enable_irq),
    irq_mask: Some(i8259a_disable_irq),
    irq_mask_ack: Some(i8259a_mask_and_ack_irq),
};

pub unsafe extern "C" fn init_i8259a_irqs() {
    outb(0xff, 0x21);
    outb(0xff, 0xA1);

    let mut i: ::core::ffi::c_long = 0;
    while i < 16 {
        irq_set_chip_and_handler(i as ::core::ffi::c_uint, &mut i8259a_irq_type, handle_level_irq);
        i += 1;
    }

    if request_irq(2, no_action, 0, b"cascade\0".as_ptr() as *const ::core::ffi::c_char, ::core::ptr::null_mut()) != 0 {
        pr_err(b"Failed to request irq 2 (cascade)\n\0".as_ptr() as *const ::core::ffi::c_char);
    }
}

/* CONFIG_ALPHA_POLARIS is intentionally left out here. */

#[cfg(any(CONFIG_ALPHA_GENERIC, CONFIG_ALPHA_CIA, CONFIG_ALPHA_PYXIS, CONFIG_ALPHA_TITAN, CONFIG_ALPHA_TSUNAMI, CONFIG_ALPHA_IRONGATE))]
pub unsafe extern "C" fn isa_device_interrupt(_vector: ::core::ffi::c_ulong) {
    /* Generate a PCI interrupt acknowledge cycle and handle its vector. */
    let mut j = core::ptr::read_volatile(IACK_SC as *const ::core::ffi::c_int);
    j &= 0xff;
    handle_irq(j as ::core::ffi::c_uint);
}

#[cfg(any(CONFIG_ALPHA_GENERIC, not(any(CONFIG_ALPHA_CIA, CONFIG_ALPHA_PYXIS, CONFIG_ALPHA_TITAN, CONFIG_ALPHA_TSUNAMI, CONFIG_ALPHA_IRONGATE))))]
pub unsafe extern "C" fn isa_no_iack_sc_device_interrupt(_vector: ::core::ffi::c_ulong) {
    let mut pic: ::core::ffi::c_ulong = (inb(0x20) as ::core::ffi::c_ulong) | ((inb(0xA0) as ::core::ffi::c_ulong) << 8);
    pic &= 0xFFFB;

    while pic != 0 {
        let j = (!pic).trailing_zeros();
        pic &= pic.wrapping_sub(1);
        handle_irq(j as ::core::ffi::c_uint);
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
