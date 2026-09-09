// SPDX-License-Identifier: GPL-2.0
/*
 * Handle interrupts from the SRM, assuming no additional weirdness.
 */

// Dependencies supplied by the surrounding kernel translation.
use core::ffi::{c_int, c_ulong, c_void};

#[repr(C)]
pub struct irq_data {
    pub irq: c_ulong,
}

#[repr(C)]
pub struct irq_chip {
    pub name: *const core::ffi::c_char,
    pub irq_unmask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask: Option<unsafe extern "C" fn(*mut irq_data)>,
    pub irq_mask_ack: Option<unsafe extern "C" fn(*mut irq_data)>,
}

#[repr(C)]
pub struct spinlock_t {
    _private: [u8; 0],
}

extern "C" {
    fn spin_lock(lock: *mut spinlock_t);
    fn spin_unlock(lock: *mut spinlock_t);
    fn cserve_ena(irq: c_ulong);
    fn cserve_dis(irq: c_ulong);
    fn irq_set_chip_and_handler(
        irq: c_ulong,
        chip: *mut irq_chip,
        handler: unsafe extern "C" fn(c_int),
    );
    fn irq_set_status_flags(irq: c_ulong, flags: c_ulong);
    fn handle_level_irq(irq: c_int);
    fn handle_irq(irq: c_int);
}

const NR_IRQS: c_ulong = 0; // Supplied by the kernel build configuration.
const IRQ_LEVEL: c_ulong = 0; // Supplied by the kernel headers.

/*
 * Is the palcode SMP safe? In other words: can we call cserve_ena/dis
 * at the same time in multiple CPUs? To be safe I added a spinlock
 * but it can be removed trivially if the palcode is robust against smp.
 */
#[no_mangle]
pub static mut srm_irq_lock: spinlock_t = spinlock_t { _private: [] };

unsafe extern "C" fn srm_enable_irq(d: *mut irq_data) {
    spin_lock(&raw mut srm_irq_lock);
    cserve_ena((*d).irq.wrapping_sub(16));
    spin_unlock(&raw mut srm_irq_lock);
}

unsafe extern "C" fn srm_disable_irq(d: *mut irq_data) {
    spin_lock(&raw mut srm_irq_lock);
    cserve_dis((*d).irq.wrapping_sub(16));
    spin_unlock(&raw mut srm_irq_lock);
}

/* Handle interrupts from the SRM, assuming no additional weirdness.  */
#[no_mangle]
pub static mut srm_irq_type: irq_chip = irq_chip {
    name: b"SRM\0".as_ptr() as *const core::ffi::c_char,
    irq_unmask: Some(srm_enable_irq),
    irq_mask: Some(srm_disable_irq),
    irq_mask_ack: Some(srm_disable_irq),
};

#[no_mangle]
pub unsafe extern "C" fn init_srm_irqs(max: isize, ignore_mask: c_ulong) {
    let mut i: isize;

    if NR_IRQS <= 16 {
        return;
    }
    i = 16;
    while i < max {
        if i < 64 && ((ignore_mask >> (i as u32)) & 1) != 0 {
            i += 1;
            continue;
        }
        irq_set_chip_and_handler(
            i as c_ulong,
            &raw mut srm_irq_type,
            handle_level_irq,
        );
        irq_set_status_flags(i as c_ulong, IRQ_LEVEL);
        i += 1;
    }
}

#[no_mangle]
pub unsafe extern "C" fn srm_device_interrupt(vector: c_ulong) {
    let irq: c_int = ((vector.wrapping_sub(0x800)) >> 4) as c_int;
    handle_irq(irq);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
