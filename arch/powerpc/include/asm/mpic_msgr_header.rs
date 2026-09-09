/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2011-2012, Meador Inge, Mentor Graphics Corporation.
 */

/*
 * C header guard: _ASM_MPIC_MSGR_H
 *
 * Dependencies supplied by the surrounding kernel translation are intentionally
 * left external: raw_spinlock_t, out_be32, in_be32, and
 * get_hard_smp_processor_id.
 */

#[repr(C)]
pub struct mpic_msgr {
    pub base: *mut u32,
    pub mer: *mut u32,
    pub irq: i32,
    pub in_use: u8,
    pub lock: raw_spinlock_t,
    pub num: i32,
}

/* Get a message register
 *
 * @reg_num: the MPIC message register to get
 *
 * A pointer to the message register is returned.  If
 * the message register asked for is already in use, then
 * EBUSY is returned.  If the number given is not associated
 * with an actual message register, then ENODEV is returned.
 * Successfully getting the register marks it as in use.
 */
extern "C" {
    pub fn mpic_msgr_get(reg_num: u32) -> *mut mpic_msgr;
}

/* Relinquish a message register
 *
 * @msgr: the message register to return
 *
 * Disables the given message register and marks it as free.
 * After this call has completed successully the message
 * register is available to be acquired by a call to
 * mpic_msgr_get.
 */
extern "C" {
    pub fn mpic_msgr_put(msgr: *mut mpic_msgr);
}

/* Enable a message register
 *
 * @msgr: the message register to enable
 *
 * The given message register is enabled for sending
 * messages.
 */
extern "C" {
    pub fn mpic_msgr_enable(msgr: *mut mpic_msgr);
}

/* Disable a message register
 *
 * @msgr: the message register to disable
 *
 * The given message register is disabled for sending
 * messages.
 */
extern "C" {
    pub fn mpic_msgr_disable(msgr: *mut mpic_msgr);
}

/* Write a message to a message register
 *
 * @msgr: the message register to write to
 * @message: the message to write
 *
 * The given 32-bit message is written to the given message
 * register.  Writing to an enabled message registers fires
 * an interrupt.
 */
#[inline]
pub unsafe fn mpic_msgr_write(msgr: *mut mpic_msgr, message: u32) {
    out_be32((*msgr).base, message);
}

/* Read a message from a message register
 *
 * @msgr: the message register to read from
 *
 * Returns the 32-bit value currently in the given message register.
 * Upon reading the register any interrupts for that register are
 * cleared.
 */
#[inline]
pub unsafe fn mpic_msgr_read(msgr: *mut mpic_msgr) -> u32 {
    in_be32((*msgr).base)
}

/* Clear a message register
 *
 * @msgr: the message register to clear
 *
 * Clears any interrupts associated with the given message register.
 */
#[inline]
pub unsafe fn mpic_msgr_clear(msgr: *mut mpic_msgr) {
    let _ = mpic_msgr_read(msgr);
}

/* Set the destination CPU for the message register
 *
 * @msgr: the message register whose destination is to be set
 * @cpu_num: the Linux CPU number to bind the message register to
 *
 * Note that the CPU number given is the CPU number used by the kernel
 * and *not* the actual hardware CPU number.
 */
#[inline]
pub unsafe fn mpic_msgr_set_destination(msgr: *mut mpic_msgr, cpu_num: u32) {
    out_be32((*msgr).base, 1u32 << get_hard_smp_processor_id(cpu_num));
}

/* Get the IRQ number for the message register
 * @msgr: the message register whose IRQ is to be returned
 *
 * Returns the IRQ number associated with the given message register.
 * 0 is returned if this message register is not capable of receiving
 * interrupts.  What message register can and cannot receive interrupts is
 * specified in the device tree for the system.
 */
#[inline]
pub unsafe fn mpic_msgr_get_irq(msgr: *mut mpic_msgr) -> i32 {
    (*msgr).irq
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
