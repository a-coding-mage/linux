/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from arch/x86/include/asm/barrier.h. */

/* Depends on: asm/alternative.h, asm/nops.h */

/*
 * Force strict CPU ordering.
 * And yes, this might be required on UP too when we're talking
 * to devices.
 */

#[cfg(CONFIG_X86_32)]
#[inline(always)]
pub fn mb() {
    unsafe {
        core::arch::asm!(
            ALTERNATIVE!("lock addl $0,-4(%esp)", "mfence", "ft"),
            ft = const X86_FEATURE_XMM2,
            options(att_syntax)
        )
    }
}
#[cfg(CONFIG_X86_32)]
#[inline(always)]
pub fn rmb() {
    unsafe {
        core::arch::asm!(
            ALTERNATIVE!("lock addl $0,-4(%esp)", "lfence", "ft"),
            ft = const X86_FEATURE_XMM2,
            options(att_syntax)
        )
    }
}
#[cfg(CONFIG_X86_32)]
#[inline(always)]
pub fn wmb() {
    unsafe {
        core::arch::asm!(
            ALTERNATIVE!("lock addl $0,-4(%esp)", "sfence", "ft"),
            ft = const X86_FEATURE_XMM,
            options(att_syntax)
        )
    }
}

#[cfg(not(CONFIG_X86_32))]
#[inline(always)]
pub fn __mb() {
    unsafe { core::arch::asm!("mfence", options(nostack, preserves_flags)) }
}
#[cfg(not(CONFIG_X86_32))]
#[inline(always)]
pub fn __rmb() {
    unsafe { core::arch::asm!("lfence", options(nostack, preserves_flags)) }
}
#[cfg(not(CONFIG_X86_32))]
#[inline(always)]
pub fn __wmb() {
    unsafe { core::arch::asm!("sfence", options(nostack, preserves_flags)) }
}

/**
 * array_index_mask_nospec() - generate a mask that is ~0UL when the
 * 	bounds check succeeds and 0 otherwise
 * @idx: array element index
 * @sz: number of elements in array
 *
 * Returns:
 *     0 - (@idx < @sz)
 */
#[inline(always)]
pub fn array_index_mask_nospec(idx: kernel::ffi::c_ulong, sz: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong {
    let mask: kernel::ffi::c_ulong;
    unsafe {
        core::arch::asm!(
            "cmp {sz}, {idx}",
            "sbb {mask}, {mask}",
            mask = out(reg) mask,
            sz = in(reg) sz,
            idx = in(reg) idx,
            options(att_syntax, nomem, nostack)
        )
    }
    mask
}

/* Prevent speculative execution past this barrier. */
#[inline(always)]
pub fn barrier_nospec() {
    unsafe {
        core::arch::asm!(
            ALTERNATIVE!("", "lfence", "ft"),
            ft = const X86_FEATURE_LFENCE_RDTSC,
            options(att_syntax, nostack, preserves_flags)
        )
    }
}

#[inline(always)]
pub fn __dma_rmb() {
    barrier()
}
#[inline(always)]
pub fn __dma_wmb() {
    barrier()
}

#[inline(always)]
pub fn __smp_mb() {
    #[cfg(CONFIG_X86_32)]
    unsafe {
        core::arch::asm!("lock addl $0,-4(%esp)", options(att_syntax))
    }
    #[cfg(not(CONFIG_X86_32))]
    unsafe {
        core::arch::asm!("lock addl $0,-4(%rsp)", options(att_syntax))
    }
}

#[inline(always)]
pub fn __smp_rmb() {
    dma_rmb()
}
#[inline(always)]
pub fn __smp_wmb() {
    barrier()
}
#[macro_export]
macro_rules! __smp_store_mb {
    ($var:expr, $value:expr) => {{
        let _ = xchg!(&raw mut $var, $value);
    }};
}

#[macro_export]
macro_rules! __smp_store_release {
    ($p:expr, $v:expr) => {{
        let __p = $p;
        __compiletime_assert_atomic_type(__p);
        barrier();
        WRITE_ONCE!(*__p, $v);
    }};
}

#[macro_export]
macro_rules! __smp_load_acquire {
    ($p:expr) => {{
        let __p = $p;
        let ___p1 = READ_ONCE!(*__p);
        __compiletime_assert_atomic_type(__p);
        barrier();
        ___p1
    }};
}

/* Atomic operations are already serializing on x86 */
#[inline(always)]
pub fn __smp_mb__before_atomic() {}
#[inline(always)]
pub fn __smp_mb__after_atomic() {}

/* Writing to CR3 provides a full memory barrier in switch_mm(). */
#[inline(always)]
pub fn smp_mb__after_switch_mm() {}

/* Depends on: asm-generic/barrier.h */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
