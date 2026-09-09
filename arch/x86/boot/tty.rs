// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007 rPath, Inc. - All Rights Reserved
 *   Copyright 2009 Intel Corporation; author H. Peter Anvin
 *
 * ----------------------------------------------------------------------- */

/* Very simple screen and serial I/O */

use crate::biosregs;

pub static mut early_serial_base: i32 = 0;

const XMTRDY: u8 = 0x20;
const TXR: i32 = 0; /* Transmit register (WRITE) */
const LSR: i32 = 5; /* Line Status */

extern "C" {
    fn inb(port: i32) -> u8;
    fn outb(value: i32, port: i32);
    fn cpu_relax();
    fn initregs(regs: *mut biosregs);
    fn intcall(int_no: i32, input: *mut biosregs, output: *mut biosregs);
}

/* These functions are in .inittext so they can be used to signal
 * error during initialization. */

#[inline(never)]
#[link_section = ".inittext"]
unsafe fn serial_putchar(ch: i32) {
    let mut timeout: u32 = 0xffff;

    while ((inb(early_serial_base + LSR) & XMTRDY) == 0) && {
        timeout = timeout.wrapping_sub(1);
        timeout != 0
    } {
        cpu_relax();
    }

    outb(ch, early_serial_base + TXR);
}

#[inline(never)]
#[link_section = ".inittext"]
unsafe fn bios_putchar(ch: i32) {
    let mut ireg: biosregs = core::mem::zeroed();

    initregs(&mut ireg);
    ireg.bx = 0x0007;
    ireg.cx = 0x0001;
    ireg.ah = 0x0e;
    ireg.al = ch as _;
    intcall(0x10, &mut ireg, core::ptr::null_mut());
}

#[inline(never)]
#[link_section = ".inittext"]
pub unsafe fn putchar(ch: i32) {
    if ch == '\n' as i32 {
        putchar('\r' as i32); /* \n -> \r\n */
    }

    bios_putchar(ch);

    if early_serial_base != 0 {
        serial_putchar(ch);
    }
}

#[inline(never)]
#[link_section = ".inittext"]
pub unsafe fn puts(mut str_: *const i8) {
    while *str_ != 0 {
        putchar(*str_ as i32);
        str_ = str_.add(1);
    }
}

/* Read the CMOS clock through the BIOS, and return the
 * seconds in BCD. */
unsafe fn gettime() -> u8 {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();

    initregs(&mut ireg);
    ireg.ah = 0x02;
    intcall(0x1a, &mut ireg, &mut oreg);

    oreg.dh
}

/* Read from the keyboard */
pub unsafe fn getchar() -> i32 {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();

    initregs(&mut ireg);
    /* ireg.ah = 0x00; */
    intcall(0x16, &mut ireg, &mut oreg);

    oreg.al as i32
}

unsafe fn kbd_pending() -> bool {
    let mut ireg: biosregs = core::mem::zeroed();
    let mut oreg: biosregs = core::mem::zeroed();

    initregs(&mut ireg);
    ireg.ah = 0x01;
    intcall(0x16, &mut ireg, &mut oreg);

    (oreg.eflags & crate::X86_EFLAGS_ZF) == 0
}

pub unsafe fn kbd_flush() {
    loop {
        if !kbd_pending() {
            break;
        }
        getchar();
    }
}

pub unsafe fn getchar_timeout() -> i32 {
    let mut cnt = 30;
    let mut t0: u8;
    let mut t1: u8;

    t0 = gettime();

    while cnt != 0 {
        if kbd_pending() {
            return getchar();
        }

        t1 = gettime();
        if t0 != t1 {
            cnt -= 1;
            t0 = t1;
        }
    }

    0 /* Timeout! */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
