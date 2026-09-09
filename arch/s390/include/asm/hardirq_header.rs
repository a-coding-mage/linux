/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  S390 version
 *    Copyright IBM Corp. 1999, 2000
 *    Author(s): Martin Schwidefsky (schwidefsky@de.ibm.com),
 *               Denis Joseph Barrow (djbarrow@de.ibm.com,barrow_dj@yahoo.com)
 *
 *  Derived from "include/asm-i386/hardirq.h"
 */

// Dependency supplied by asm/lowcore.h in the C source.

#[macro_export]
macro_rules! local_softirq_pending {
    () => {{ unsafe { (*$crate::get_lowcore()).softirq_pending } }};
}

#[macro_export]
macro_rules! set_softirq_pending {
    ($x:expr) => {{ unsafe { (*$crate::get_lowcore()).softirq_pending = $x; }} };
}

#[macro_export]
macro_rules! or_softirq_pending {
    ($x:expr) => {{ unsafe { (*$crate::get_lowcore()).softirq_pending |= $x; }} };
}

pub const __ARCH_IRQ_STAT: bool = true;
pub const __ARCH_IRQ_EXIT_IRQS_DISABLED: bool = true;

/// Acknowledge an unexpected interrupt vector.
#[inline]
pub unsafe fn ack_bad_irq(irq: ::core::ffi::c_uint) {
    // KERN_CRIT and printk are supplied by the surrounding kernel dependencies.
    printk(KERN_CRIT, b"unexpected IRQ trap at vector %02x\n\0".as_ptr().cast(), irq);
}

unsafe extern "C" {
    fn printk(fmt: *const ::core::ffi::c_char, ...) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
