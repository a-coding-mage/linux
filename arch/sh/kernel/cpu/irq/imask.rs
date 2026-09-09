// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/irq/imask.c
 *
 * Copyright (C) 1999, 2000  Niibe Yutaka
 *
 * Simple interrupt handling using IMASK of SR register.
 *
 */
/* NOTE: Will not work on level 15 */

// The following declarations are supplied by the surrounding kernel.
use core::ffi::c_uint;

/* Bitmap of IRQ masked */
const IMASK_PRIORITY: usize = 15;

static mut imask_mask: u64 = 0;
static mut interrupt_priority: i32 = 0;

#[inline]
unsafe fn set_interrupt_registers(ip: i32) {
    /* The original implementation uses SH inline assembly to update SR.IMASK.
     * Preserve the operation and its conditional intent for the SH backend. */
    #[cfg(target_arch = "sh")]
    core::arch::asm!(
        "stc sr, {dummy}",
        "and #0xf0, {dummy}",
        "shlr2 {dummy}",
        "cmp/eq #0x3c, {dummy}",
        "bt/s 1f",
        "stc sr, {dummy}",
        "and {mask}, {dummy}",
        "or {priority}, {dummy}",
        "ldc {dummy}, sr",
        "1:",
        dummy = lateout(reg) _,
        mask = in(reg) !0xf0u32,
        priority = in(reg) (ip << 4),
        options(nostack)
    );
    #[cfg(not(target_arch = "sh"))]
    {
        let _ = ip;
    }
}

unsafe fn mask_imask_irq(data: *mut irq_data) {
    let irq = (*data).irq as usize;

    imask_mask &= !(1u64 << irq);
    if interrupt_priority < (IMASK_PRIORITY - irq) as i32 {
        interrupt_priority = (IMASK_PRIORITY - irq) as i32;
    }
    set_interrupt_registers(interrupt_priority);
}

unsafe fn unmask_imask_irq(data: *mut irq_data) {
    let irq = (*data).irq as usize;

    imask_mask |= 1u64 << irq;
    interrupt_priority = (IMASK_PRIORITY - find_first_zero_bit(imask_mask, IMASK_PRIORITY)) as i32;
    set_interrupt_registers(interrupt_priority);
}

#[repr(C)]
pub struct irq_data {
    pub irq: c_uint,
}

#[repr(C)]
struct irq_chip {
    name: *const u8,
    irq_mask: Option<unsafe fn(*mut irq_data)>,
    irq_unmask: Option<unsafe fn(*mut irq_data)>,
    irq_mask_ack: Option<unsafe fn(*mut irq_data)>,
}

static imask_irq_chip: irq_chip = irq_chip {
    name: b"SR.IMASK\0".as_ptr(),
    irq_mask: Some(mask_imask_irq),
    irq_unmask: Some(unmask_imask_irq),
    irq_mask_ack: Some(mask_imask_irq),
};

extern "C" {
    fn irq_set_chip_and_handler_name(
        irq: c_uint,
        chip: *const irq_chip,
        handler: unsafe extern "C" fn(),
        name: *const u8,
    );
    fn handle_level_irq();
}

#[inline]
unsafe fn find_first_zero_bit(bitmap: u64, nbits: usize) -> usize {
    for bit in 0..nbits {
        if (bitmap & (1u64 << bit)) == 0 {
            return bit;
        }
    }
    nbits
}

pub unsafe fn make_imask_irq(irq: c_uint) {
    irq_set_chip_and_handler_name(
        irq,
        &imask_irq_chip,
        handle_level_irq,
        b"level\0".as_ptr(),
    );
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
