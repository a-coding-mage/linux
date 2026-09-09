/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	linux/arch/alpha/kernel/irq_impl.h
 *
 *	Copyright (C) 1995 Linus Torvalds
 *	Copyright (C) 1998, 2000 Richard Henderson
 *
 * This file contains declarations and inline functions for interfacing
 * with the IRQ handling routines in irq.c.
 */

// Dependencies supplied by the Linux IRQ, interrupt, and profiling interfaces.

pub const RTC_IRQ: i32 = 8;

extern "C" {
    pub fn isa_device_interrupt(irq: ::core::ffi::c_ulong);
    pub fn isa_no_iack_sc_device_interrupt(irq: ::core::ffi::c_ulong);
    pub fn srm_device_interrupt(irq: ::core::ffi::c_ulong);
    pub fn pyxis_device_interrupt(irq: ::core::ffi::c_ulong);

    pub fn init_srm_irqs(a: ::core::ffi::c_long, b: ::core::ffi::c_ulong);
    pub fn init_pyxis_irqs(a: ::core::ffi::c_ulong);
    pub fn init_rtc_irq(handler: irq_handler_t);

    pub fn common_init_isa_dma();

    pub fn i8259a_enable_irq(d: *mut irq_data);
    pub fn i8259a_disable_irq(d: *mut irq_data);
    pub fn i8259a_mask_and_ack_irq(d: *mut irq_data);
    pub static mut i8259a_irq_type: irq_chip;
    pub fn init_i8259a_irqs();

    pub fn handle_irq(irq: ::core::ffi::c_int);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
