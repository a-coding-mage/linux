// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2008 IBM Corp.
 *
 * Based on arch/powerpc/platforms/pasemi/idle.c:
 * Copyright (C) 2006-2007 PA Semi, Inc
 *
 * Added by: Jerone Young <jyoung5@us.ibm.com>
 */

// C dependencies: linux/of.h, linux/kernel.h, and asm/machdep.h.

extern "C" {
    static mut ppc_md: PpcMd;
    fn mfmsr() -> c_ulong;
    fn mtmsr(value: c_ulong);
    fn isync();
    fn strcmp(left: *const c_char, right: *const c_char) -> c_int;
}

type c_char = i8;
type c_int = i32;
type c_ulong = usize;

#[repr(C)]
struct PpcMd {
    power_save: Option<unsafe extern "C" fn()>,
}

const MSR_WE: c_ulong = 1 << 18;
const MSR_EE: c_ulong = 1 << 15;
const MSR_CE: c_ulong = 1 << 17;
const MSR_DE: c_ulong = 1 << 9;

static mut mode_spin: c_int = 0;

unsafe extern "C" fn ppc44x_idle() {
    let msr_save: c_ulong;

    msr_save = mfmsr();
    /* set wait state MSR */
    mtmsr(msr_save | MSR_WE | MSR_EE | MSR_CE | MSR_DE);
    isync();
    /* return to initial state */
    mtmsr(msr_save);
    isync();
}

unsafe extern "C" fn ppc44x_idle_init() -> c_int {
    if mode_spin == 0 {
        /* If we are not setting spin mode
           then we set to wait mode */
        ppc_md.power_save = Some(ppc44x_idle);
    }

    0
}

// arch_initcall(ppc44x_idle_init);

unsafe extern "C" fn idle_param(p: *mut c_char) -> c_int {
    if strcmp(b"spin\0".as_ptr() as *const c_char, p) == 0 {
        mode_spin = 1;
        ppc_md.power_save = None;
    }

    0
}

// early_param("idle", idle_param);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
