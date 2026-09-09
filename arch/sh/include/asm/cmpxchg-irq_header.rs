/* SPDX-License-Identifier: GPL-2.0 */
// Translated from __ASM_SH_CMPXCHG_IRQ_H.
// The original header includes <linux/irqflags.h>; the irq helpers below are
// supplied by that dependency.

extern "C" {
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
}

pub unsafe fn xchg_u32(m: *mut u32, val: usize) -> usize {
    let mut flags: usize = 0;
    local_irq_save(&mut flags as *mut usize);
    let retval = core::ptr::read_volatile(m) as usize;
    core::ptr::write_volatile(m, val as u32);
    local_irq_restore(flags);
    retval
}

pub unsafe fn xchg_u16(m: *mut u16, val: usize) -> usize {
    let mut flags: usize = 0;
    local_irq_save(&mut flags as *mut usize);
    let retval = core::ptr::read_volatile(m) as usize;
    core::ptr::write_volatile(m, val as u16);
    local_irq_restore(flags);
    retval
}

pub unsafe fn xchg_u8(m: *mut u8, val: usize) -> usize {
    let mut flags: usize = 0;
    local_irq_save(&mut flags as *mut usize);
    let retval = core::ptr::read_volatile(m) as usize;
    core::ptr::write_volatile(m, (val & 0xff) as u8);
    local_irq_restore(flags);
    retval
}

pub unsafe fn __cmpxchg_u32(m: *mut i32, old: usize, new: usize) -> u32 {
    let mut flags: usize = 0;
    local_irq_save(&mut flags as *mut usize);
    let retval = core::ptr::read_volatile(m) as u32;
    if retval as usize == old {
        core::ptr::write_volatile(m, new as i32);
    }
    local_irq_restore(flags); // implies memory barrier
    retval
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
