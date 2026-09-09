/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Based on arch/arm/include/asm/barrier.h
 *
 * Copyright (C) 2012 ARM Ltd.
 * Copyright (C) 2013 Regents of the University of California
 * Copyright (C) 2017 SiFive
 */

/* The C header guard and __ASSEMBLER__ condition are intentionally omitted. */

/* These barriers need to enforce ordering on both devices or memory. */
macro_rules! __mb { () => { RISCV_FENCE!(iorw, iorw) }; }
macro_rules! __rmb { () => { RISCV_FENCE!(ir, ir) }; }
macro_rules! __wmb { () => { RISCV_FENCE!(ow, ow) }; }

/* These barriers do not need to enforce ordering on devices, just memory. */
macro_rules! __smp_mb { () => { RISCV_FENCE!(rw, rw) }; }
macro_rules! __smp_rmb { () => { RISCV_FENCE!(r, r) }; }
macro_rules! __smp_wmb { () => { RISCV_FENCE!(w, w) }; }

/*
 * This is a very specific barrier: it is currently only used in two places in
 * the kernel, both in the scheduler. The "critical section is RCsc"
 * guarantee mandates a barrier on RISC-V. The AQ/RL pair provides a RCpc
 * critical section, but the ordering is only enforced on one lock, so this is
 * a full fence.
 *
 * Since writeX may be called from preemptive regions, the predecessor set
 * includes "o" to ensure device writes are visible before scheduling on a new
 * hart. This is upgraded to a full IO fence to avoid IO crossing a scheduling
 * boundary.
 */
macro_rules! smp_mb__after_spinlock { () => { RISCV_FENCE!(iorw, iorw) }; }

macro_rules! __smp_store_release {
    ($p:expr, $v:expr) => {{
        compiletime_assert_atomic_type!(*$p);
        RISCV_FENCE!(rw, w);
        WRITE_ONCE!(*$p, $v);
    }};
}

macro_rules! __smp_load_acquire {
    ($p:expr) => {{
        let ___p1 = READ_ONCE!(*$p);
        compiletime_assert_atomic_type!(*$p);
        RISCV_FENCE!(r, rw);
        ___p1
    }};
}

/* CONFIG_RISCV_ISA_ZAWRS controls whether this macro is available. */
#[cfg(CONFIG_RISCV_ISA_ZAWRS)]
macro_rules! smp_cond_load_relaxed {
    ($ptr:expr, $cond_expr:expr) => {{
        let __PTR = $ptr;
        let mut VAL;
        loop {
            VAL = READ_ONCE!(*__PTR);
            if $cond_expr {
                break;
            }
            __cmpwait_relaxed!($ptr, VAL);
        }
        VAL
    }};
}

/* The generic barrier declarations are supplied by asm-generic/barrier.h. */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
