// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * OpenRISC or32_ksyms.c
 *
 * Linux architectural port borrowing liberally from similar works of
 * others.  All original copyrights apply as per the original source
 * declaration.
 *
 * Modifications for the OpenRISC architecture:
 * Copyright (C) 2003 Matjaz Breskvar <phoenix@bsemi.com>
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// Dependencies supplied by the Linux kernel and OpenRISC architecture.
// The original source includes:
// linux/export.h, linux/elfcore.h, linux/sched.h, linux/in6.h,
// linux/interrupt.h, linux/vmalloc.h, linux/semaphore.h, linux/pgtable.h,
// asm/processor.h, linux/uaccess.h, asm/checksum.h, asm/io.h,
// asm/hardirq.h, and asm/delay.h.

// `DECLARE_EXPORT(name)` in the source declares an external void function and
// exports it with `EXPORT_SYMBOL(name)`.
extern "C" {
    pub fn __udivsi3();
    pub fn __divsi3();
    pub fn __umodsi3();
    pub fn __modsi3();
    pub fn __muldi3();
    pub fn __ashrdi3();
    pub fn __ashldi3();
    pub fn __lshrdi3();
    pub fn __ucmpdi2();

    // EXPORT_SYMBOL declarations from the source.
    pub fn __copy_tofrom_user();
    pub fn __clear_user();
    pub fn memset();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
