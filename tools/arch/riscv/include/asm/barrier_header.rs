// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copied from the kernel sources to tools/arch/riscv:
 *
 * Copyright (C) 2012 ARM Ltd.
 * Copyright (C) 2013 Regents of the University of California
 * Copyright (C) 2017 SiFive
 */

// C header dependencies:
// #include <asm/fence.h>
// #include <linux/compiler.h>

/* These barriers need to enforce ordering on both devices and memory. */
macro_rules! mb {
    () => {
        RISCV_FENCE!(iorw, iorw)
    };
}

macro_rules! rmb {
    () => {
        RISCV_FENCE!(ir, ir)
    };
}

macro_rules! wmb {
    () => {
        RISCV_FENCE!(ow, ow)
    };
}

/* These barriers do not need to enforce ordering on devices, just memory. */
macro_rules! smp_mb {
    () => {
        RISCV_FENCE!(rw, rw)
    };
}

macro_rules! smp_rmb {
    () => {
        RISCV_FENCE!(r, r)
    };
}

macro_rules! smp_wmb {
    () => {
        RISCV_FENCE!(w, w)
    };
}

macro_rules! smp_store_release {
    ($p:expr, $v:expr) => {{
        RISCV_FENCE!(rw, w);
        WRITE_ONCE!(*$p, $v);
    }};
}

macro_rules! smp_load_acquire {
    ($p:expr) => {{
        let ___p1 = READ_ONCE!(*$p);
        RISCV_FENCE!(r, rw);
        ___p1
    }};
}
