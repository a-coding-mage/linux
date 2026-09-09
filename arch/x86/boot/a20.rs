// SPDX-License-Identifier: GPL-2.0-only
/* -*- linux-c -*- ------------------------------------------------------- *
 *
 *   Copyright (C) 1991, 1992 Linus Torvalds
 *   Copyright 2007-2008 rPath, Inc. - All Rights Reserved
 *   Copyright 2009 Intel Corporation; author H. Peter Anvin
 *
 * ----------------------------------------------------------------------- */

/* Enable A20 gate (return -1 on failure) */

// Declarations supplied by boot.h.
extern "C" {
    fn io_delay();
    fn inb(port: u16) -> u8;
    fn outb(value: u8, port: u16);
    fn set_fs(value: u16);
    fn set_gs(value: u16);
    fn rdfs32(addr: u32) -> i32;
    fn wrfs32(value: i32, addr: u32);
    fn rdgs32(addr: u32) -> i32;
    fn initregs(regs: *mut biosregs);
    fn intcall(vector: u8, regs: *mut biosregs, result: *mut biosregs);
}

#[repr(C)]
struct biosregs {
    ax: u16,
}

const MAX_8042_LOOPS: i32 = 100000;
const MAX_8042_FF: i32 = 32;

unsafe fn empty_8042() -> i32 {
    let mut status: u8;
    let mut loops = MAX_8042_LOOPS;
    let mut ffs = MAX_8042_FF;

    while loops > 0 {
        loops -= 1;
        io_delay();

        status = inb(0x64);
        if status == 0xff {
            /* FF is a plausible, but very unlikely status */
            ffs -= 1;
            if ffs == 0 {
                return -1; /* Assume no KBC present */
            }
        }
        if status & 1 != 0 {
            /* Read and discard input data */
            io_delay();
            let _ = inb(0x60);
        } else if status & 2 == 0 {
            /* Buffers empty, finished! */
            return 0;
        }
    }

    -1
}

/* Returns nonzero if the A20 line is enabled.  The memory address
   used as a test is the int $0x80 vector, which should be safe. */

const A20_TEST_ADDR: u32 = 4 * 0x80;
const A20_TEST_SHORT: i32 = 32;
const A20_TEST_LONG: i32 = 2097152; /* 2^21 */

unsafe fn a20_test(mut loops: i32) -> i32 {
    let mut ok = 0;
    let saved: i32;
    let mut ctr: i32;

    set_fs(0x0000);
    set_gs(0xffff);

    saved = rdfs32(A20_TEST_ADDR);
    ctr = saved;

    while loops > 0 {
        loops -= 1;
        ctr += 1;
        wrfs32(ctr, A20_TEST_ADDR);
        io_delay(); /* Serialize and make delay constant */
        ok = rdgs32(A20_TEST_ADDR + 0x10) ^ ctr;
        if ok != 0 {
            break;
        }
    }

    wrfs32(saved, A20_TEST_ADDR);
    ok
}

/* Quick test to see if A20 is already enabled */
unsafe fn a20_test_short() -> i32 {
    a20_test(A20_TEST_SHORT)
}

/* Longer test that actually waits for A20 to come on line; this
   is useful when dealing with the KBC or other slow external circuitry. */
unsafe fn a20_test_long() -> i32 {
    a20_test(A20_TEST_LONG)
}

unsafe fn enable_a20_bios() {
    let mut ireg = biosregs { ax: 0 };

    initregs(&mut ireg);
    ireg.ax = 0x2401;
    intcall(0x15, &mut ireg, core::ptr::null_mut());
}

unsafe fn enable_a20_kbc() {
    empty_8042();

    outb(0xd1, 0x64); /* Command write */
    empty_8042();

    outb(0xdf, 0x60); /* A20 on */
    empty_8042();

    outb(0xff, 0x64); /* Null command, but UHCI wants it */
    empty_8042();
}

unsafe fn enable_a20_fast() {
    let mut port_a: u8;

    port_a = inb(0x92); /* Configuration port A */
    port_a |= 0x02; /* Enable A20 */
    port_a &= !0x01; /* Do not reset machine */
    outb(port_a, 0x92);
}

/* Actual routine to enable A20; return 0 on ok, -1 on failure */

const A20_ENABLE_LOOPS: i32 = 255; /* Number of times to try */

pub unsafe fn enable_a20() -> i32 {
    let mut loops = A20_ENABLE_LOOPS;
    let mut kbc_err: i32;

    while loops > 0 {
        loops -= 1;
        /* First, check to see if A20 is already enabled
           (legacy free, etc.) */
        if a20_test_short() != 0 {
            return 0;
        }

        /* Next, try the BIOS (INT 0x15, AX=0x2401) */
        enable_a20_bios();
        if a20_test_short() != 0 {
            return 0;
        }

        /* Try enabling A20 through the keyboard controller */
        kbc_err = empty_8042();

        if a20_test_short() != 0 {
            return 0; /* BIOS worked, but with delayed reaction */
        }

        if kbc_err == 0 {
            enable_a20_kbc();
            if a20_test_long() != 0 {
                return 0;
            }
        }

        /* Finally, try enabling the "fast A20 gate" */
        enable_a20_fast();
        if a20_test_long() != 0 {
            return 0;
        }
    }

    -1
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
