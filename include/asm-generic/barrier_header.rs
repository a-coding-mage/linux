/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Generic barrier definitions.
 *
 * It should be possible to use these on really simple architectures,
 * but it serves more as a starting point for new ports.
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

/* Depends on: linux/compiler.h, linux/kcsan-checks.h, asm/rwonce.h */

/*
 * Each C "#ifndef name" default below is compiled only for architectures whose
 * <asm/barrier.h> does not define `name` itself (the cfg lists mirror the
 * arch headers), so the arch definition is the only one in scope.
 */

#[cfg(not(any(CONFIG_ARM, CONFIG_CSKY, CONFIG_OPENRISC, CONFIG_RISCV, CONFIG_X86)))]
#[inline(always)]
pub fn nop() {
    unsafe { core::arch::asm!("nop", options(nomem, nostack, preserves_flags)) }
}

/*
 * Architectures that want generic instrumentation can define __ prefixed
 * variants of all barriers.
 */

#[cfg(any(CONFIG_ARM64, CONFIG_PPC, CONFIG_RISCV, CONFIG_S390, all(CONFIG_X86, not(CONFIG_X86_32)), CONFIG_XTENSA))]
#[inline(always)]
pub fn mb() {
    kcsan_mb!();
    __mb();
}

#[cfg(any(CONFIG_ARM64, CONFIG_PPC, CONFIG_RISCV, CONFIG_S390, all(CONFIG_X86, not(CONFIG_X86_32)), CONFIG_XTENSA))]
#[inline(always)]
pub fn rmb() {
    kcsan_rmb!();
    __rmb();
}

#[cfg(any(CONFIG_ARM64, CONFIG_PPC, CONFIG_RISCV, CONFIG_S390, all(CONFIG_X86, not(CONFIG_X86_32)), CONFIG_XTENSA))]
#[inline(always)]
pub fn wmb() {
    kcsan_wmb!();
    __wmb();
}

#[cfg(CONFIG_ARM64)]
#[inline(always)]
pub fn dma_mb() {
    kcsan_mb!();
    __dma_mb();
}

#[cfg(any(CONFIG_ARM64, CONFIG_PPC, CONFIG_S390, CONFIG_X86))]
#[inline(always)]
pub fn dma_rmb() {
    kcsan_rmb!();
    __dma_rmb();
}

#[cfg(any(CONFIG_ARM64, CONFIG_PPC, CONFIG_S390, CONFIG_X86))]
#[inline(always)]
pub fn dma_wmb() {
    kcsan_wmb!();
    __dma_wmb();
}

/*
 * Force strict CPU ordering. And yes, this is required on UP too when we're
 * talking to devices.
 *
 * Fall back to compiler barriers if nothing better is provided.
 */

#[cfg(not(any(
    CONFIG_ARM64, CONFIG_PPC, CONFIG_RISCV, CONFIG_S390, CONFIG_X86, CONFIG_XTENSA,
    CONFIG_ALPHA, CONFIG_ARC, CONFIG_ARM, CONFIG_CSKY, CONFIG_LOONGARCH, CONFIG_MICROBLAZE,
    CONFIG_MIPS, CONFIG_OPENRISC, CONFIG_PARISC, CONFIG_SUPERH, CONFIG_SPARC64
)))]
#[inline(always)]
pub fn mb() {
    barrier()
}

#[cfg(not(any(
    CONFIG_ARM64, CONFIG_PPC, CONFIG_RISCV, CONFIG_S390, CONFIG_X86, CONFIG_XTENSA,
    CONFIG_ALPHA, CONFIG_ARC, CONFIG_ARM, CONFIG_LOONGARCH, CONFIG_MIPS, CONFIG_PARISC,
    CONFIG_SUPERH, CONFIG_SPARC64
)))]
#[inline(always)]
pub fn rmb() {
    mb()
}

#[cfg(not(any(
    CONFIG_ARM64, CONFIG_PPC, CONFIG_RISCV, CONFIG_S390, CONFIG_X86, CONFIG_XTENSA,
    CONFIG_ALPHA, CONFIG_ARC, CONFIG_ARM, CONFIG_LOONGARCH, CONFIG_MIPS, CONFIG_PARISC,
    CONFIG_SUPERH, CONFIG_SPARC64
)))]
#[inline(always)]
pub fn wmb() {
    mb()
}

#[cfg(not(CONFIG_ARM64))]
#[inline(always)]
pub fn dma_mb() {
    mb()
}

#[cfg(not(any(CONFIG_ARM64, CONFIG_PPC, CONFIG_S390, CONFIG_X86, CONFIG_ARM, CONFIG_PARISC)))]
#[inline(always)]
pub fn dma_rmb() {
    rmb()
}

#[cfg(not(any(CONFIG_ARM64, CONFIG_PPC, CONFIG_S390, CONFIG_X86, CONFIG_ARM, CONFIG_PARISC)))]
#[inline(always)]
pub fn dma_wmb() {
    wmb()
}

#[cfg(not(any(
    CONFIG_ARM64, CONFIG_ARM, CONFIG_CSKY, CONFIG_LOONGARCH, CONFIG_MIPS, CONFIG_PARISC,
    CONFIG_PPC, CONFIG_RISCV, CONFIG_S390, CONFIG_SUPERH, CONFIG_X86, CONFIG_XTENSA
)))]
#[inline(always)]
pub fn __smp_mb() {
    mb()
}

#[cfg(not(any(
    CONFIG_ARM64, CONFIG_ARM, CONFIG_CSKY, CONFIG_LOONGARCH, CONFIG_MIPS, CONFIG_PARISC,
    CONFIG_PPC, CONFIG_RISCV, CONFIG_S390, CONFIG_SUPERH, CONFIG_X86, CONFIG_XTENSA
)))]
#[inline(always)]
pub fn __smp_rmb() {
    rmb()
}

#[cfg(not(any(
    CONFIG_ARM64, CONFIG_ARM, CONFIG_CSKY, CONFIG_LOONGARCH, CONFIG_MIPS, CONFIG_PARISC,
    CONFIG_PPC, CONFIG_RISCV, CONFIG_S390, CONFIG_SUPERH, CONFIG_X86, CONFIG_XTENSA
)))]
#[inline(always)]
pub fn __smp_wmb() {
    wmb()
}

#[cfg(CONFIG_SMP)]
#[inline(always)]
pub fn smp_mb() {
    kcsan_mb!();
    __smp_mb();
}

#[cfg(CONFIG_SMP)]
#[inline(always)]
pub fn smp_rmb() {
    kcsan_rmb!();
    __smp_rmb();
}

#[cfg(CONFIG_SMP)]
#[inline(always)]
pub fn smp_wmb() {
    kcsan_wmb!();
    __smp_wmb();
}

#[cfg(not(CONFIG_SMP))]
#[inline(always)]
pub fn smp_mb() {
    barrier()
}

#[cfg(not(CONFIG_SMP))]
#[inline(always)]
pub fn smp_rmb() {
    barrier()
}

#[cfg(not(CONFIG_SMP))]
#[inline(always)]
pub fn smp_wmb() {
    barrier()
}

#[cfg(not(any(CONFIG_LOONGARCH, CONFIG_SUPERH, CONFIG_X86)))]
#[macro_export]
macro_rules! __smp_store_mb {
    ($var:expr, $value:expr) => {{
        WRITE_ONCE!($var, $value);
        __smp_mb();
    }};
}

#[cfg(not(any(CONFIG_LOONGARCH, CONFIG_MIPS, CONFIG_S390, CONFIG_SPARC64, CONFIG_X86, CONFIG_XTENSA)))]
#[inline(always)]
pub fn __smp_mb__before_atomic() {
    __smp_mb()
}

#[cfg(not(any(CONFIG_LOONGARCH, CONFIG_MIPS, CONFIG_S390, CONFIG_SPARC64, CONFIG_X86, CONFIG_XTENSA)))]
#[inline(always)]
pub fn __smp_mb__after_atomic() {
    __smp_mb()
}

#[cfg(not(any(
    CONFIG_ARM64, CONFIG_LOONGARCH, CONFIG_PPC, CONFIG_PARISC, CONFIG_RISCV, CONFIG_S390,
    CONFIG_SPARC64, CONFIG_X86
)))]
#[macro_export]
macro_rules! __smp_store_release {
    ($p:expr, $v:expr) => {{
        let __p = $p;
        __compiletime_assert_atomic_type(__p);
        __smp_mb();
        WRITE_ONCE!(*__p, $v);
    }};
}

#[cfg(not(any(
    CONFIG_ALPHA, CONFIG_ARM64, CONFIG_LOONGARCH, CONFIG_PPC, CONFIG_PARISC, CONFIG_RISCV,
    CONFIG_S390, CONFIG_SPARC64, CONFIG_X86
)))]
#[macro_export]
macro_rules! __smp_load_acquire {
    ($p:expr) => {{
        let __p = $p;
        let ___p1 = READ_ONCE!(*__p);
        __compiletime_assert_atomic_type(__p);
        __smp_mb();
        ___p1
    }};
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! smp_store_mb {
    ($var:expr, $value:expr) => {{
        kcsan_mb!();
        __smp_store_mb!($var, $value);
    }};
}

#[cfg(CONFIG_SMP)]
#[inline(always)]
pub fn smp_mb__before_atomic() {
    kcsan_mb!();
    __smp_mb__before_atomic();
}

#[cfg(CONFIG_SMP)]
#[inline(always)]
pub fn smp_mb__after_atomic() {
    kcsan_mb!();
    __smp_mb__after_atomic();
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! smp_store_release {
    ($p:expr, $v:expr) => {{
        kcsan_release!();
        __smp_store_release!($p, $v);
    }};
}

#[cfg(CONFIG_SMP)]
#[macro_export]
macro_rules! smp_load_acquire {
    ($p:expr) => {
        __smp_load_acquire!($p)
    };
}

#[cfg(not(CONFIG_SMP))]
#[macro_export]
macro_rules! smp_store_mb {
    ($var:expr, $value:expr) => {{
        WRITE_ONCE!($var, $value);
        barrier();
    }};
}

#[cfg(not(CONFIG_SMP))]
#[inline(always)]
pub fn smp_mb__before_atomic() {
    barrier()
}

#[cfg(not(CONFIG_SMP))]
#[inline(always)]
pub fn smp_mb__after_atomic() {
    barrier()
}

#[cfg(not(CONFIG_SMP))]
#[macro_export]
macro_rules! smp_store_release {
    ($p:expr, $v:expr) => {{
        let __p = $p;
        barrier();
        WRITE_ONCE!(*__p, $v);
    }};
}

#[cfg(not(CONFIG_SMP))]
#[macro_export]
macro_rules! smp_load_acquire {
    ($p:expr) => {{
        let __p = $p;
        let ___p1 = READ_ONCE!(*__p);
        barrier();
        ___p1
    }};
}

/* Barriers for virtual machine guests when talking to an SMP host */
#[inline(always)]
pub fn virt_mb() {
    kcsan_mb!();
    __smp_mb();
}
#[inline(always)]
pub fn virt_rmb() {
    kcsan_rmb!();
    __smp_rmb();
}
#[inline(always)]
pub fn virt_wmb() {
    kcsan_wmb!();
    __smp_wmb();
}
#[macro_export]
macro_rules! virt_store_mb {
    ($var:expr, $value:expr) => {{
        kcsan_mb!();
        __smp_store_mb!($var, $value);
    }};
}
#[inline(always)]
pub fn virt_mb__before_atomic() {
    kcsan_mb!();
    __smp_mb__before_atomic();
}
#[inline(always)]
pub fn virt_mb__after_atomic() {
    kcsan_mb!();
    __smp_mb__after_atomic();
}
#[macro_export]
macro_rules! virt_store_release {
    ($p:expr, $v:expr) => {{
        kcsan_release!();
        __smp_store_release!($p, $v);
    }};
}
#[macro_export]
macro_rules! virt_load_acquire {
    ($p:expr) => {
        __smp_load_acquire!($p)
    };
}

/**
 * smp_acquire__after_ctrl_dep() - Provide ACQUIRE ordering after a control dependency
 *
 * A control dependency provides a LOAD->STORE order, the additional RMB
 * provides LOAD->LOAD order, together they provide LOAD->{LOAD,STORE} order,
 * aka. (load)-ACQUIRE.
 *
 * Architectures that do not do load speculation can have this be barrier().
 */
#[inline(always)]
pub fn smp_acquire__after_ctrl_dep() {
    smp_rmb()
}

/**
 * smp_cond_load_relaxed() - (Spin) wait for cond with no ordering guarantees
 * @ptr: pointer to the variable to wait on
 * @cond: boolean expression to wait for
 *
 * Equivalent to using READ_ONCE() on the condition variable.
 *
 * Due to C lacking lambda expressions we load the value of *ptr into a
 * pre-named variable @VAL to be used in @cond.
 *
 * Rust callers name that variable themselves: `smp_cond_load_relaxed!(p, |VAL| VAL != 0)`.
 */
#[cfg(not(any(CONFIG_ARM64, CONFIG_RISCV)))]
#[macro_export]
macro_rules! smp_cond_load_relaxed {
    ($ptr:expr, |$val:ident| $cond:expr) => {{
        let __PTR = $ptr;
        loop {
            let $val = READ_ONCE!(*__PTR);
            if $cond {
                break $val;
            }
            cpu_relax();
        }
    }};
}

/**
 * smp_cond_load_acquire() - (Spin) wait for cond with ACQUIRE ordering
 * @ptr: pointer to the variable to wait on
 * @cond: boolean expression to wait for
 *
 * Equivalent to using smp_load_acquire() on the condition variable but employs
 * the control dependency of the wait to reduce the barrier on many platforms.
 */
#[cfg(not(CONFIG_ARM64))]
#[macro_export]
macro_rules! smp_cond_load_acquire {
    ($ptr:expr, |$val:ident| $cond:expr) => {{
        let _val = smp_cond_load_relaxed!($ptr, |$val| $cond);
        smp_acquire__after_ctrl_dep();
        _val
    }};
}

/*
 * pmem_wmb() ensures that all stores for which the modification
 * are written to persistent storage by preceding instructions have
 * updated persistent storage before any data  access or data transfer
 * caused by subsequent instructions is initiated.
 */
#[cfg(not(CONFIG_PPC))]
#[inline(always)]
pub fn pmem_wmb() {
    wmb()
}

/*
 * ioremap_wc() maps I/O memory as memory with write-combining attributes. For
 * this kind of memory accesses, the CPU may wait for prior accesses to be
 * merged with subsequent ones. In some situation, such wait is bad for the
 * performance. io_stop_wc() can be used to prevent the merging of
 * write-combining memory accesses before this macro with those after it.
 */
#[cfg(not(CONFIG_ARM64))]
#[inline(always)]
pub fn io_stop_wc() {}

/*
 * Architectures that guarantee an implicit smp_mb() in switch_mm()
 * can override smp_mb__after_switch_mm.
 */
#[cfg(not(CONFIG_X86))]
#[inline(always)]
pub fn smp_mb__after_switch_mm() {
    smp_mb()
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
