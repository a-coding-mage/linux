/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Force strict CPU ordering.
 * And yes, this might be required on UP too when we're talking
 * to devices.
 */

/* CONFIG_X86_32 selects the legacy lock-add barriers in the C header. */
#[cfg(target_arch = "x86")]
#[inline(always)]
pub unsafe fn mb() {
    core::arch::asm!("lock addl $0, -4(%esp)", options(nostack, preserves_flags));
}

#[cfg(target_arch = "x86")]
#[inline(always)]
pub unsafe fn rmb() {
    core::arch::asm!("lock addl $0, -4(%esp)", options(nostack, preserves_flags));
}

#[cfg(target_arch = "x86")]
#[inline(always)]
pub unsafe fn wmb() {
    core::arch::asm!("lock addl $0, -4(%esp)", options(nostack, preserves_flags));
}

#[cfg(not(target_arch = "x86"))]
#[inline(always)]
pub unsafe fn __mb() {
    core::arch::asm!("mfence", options(nostack));
}

#[cfg(not(target_arch = "x86"))]
#[inline(always)]
pub unsafe fn __rmb() {
    core::arch::asm!("lfence", options(nostack));
}

#[cfg(not(target_arch = "x86"))]
#[inline(always)]
pub unsafe fn __wmb() {
    core::arch::asm!("sfence", options(nostack));
}

/**
 * array_index_mask_nospec() - generate a mask that is !0 when the
 * bounds check succeeds and 0 otherwise.
 *
 * Returns 0 when idx < sz.
 */
#[macro_export]
macro_rules! array_index_mask_nospec {
    ($idx:expr, $sz:expr) => {{
        let __idx = ($idx) + ($sz);
        let __sz = ($sz);
        let mut __mask: usize;
        unsafe {
            core::arch::asm!(
                "cmp {sz}, {idx}; sbb {mask}, {mask}",
                sz = in(reg) __sz,
                idx = in(reg) __idx,
                mask = lateout(reg) __mask,
                options(nostack)
            );
        }
        __mask
    }};
}

/* Prevent speculative execution past this barrier. */
#[macro_export]
macro_rules! barrier_nospec {
    () => {{
        #[cfg(target_feature = "lfence")]
        unsafe { core::arch::asm!("lfence", options(nostack)); }
    }};
}

#[macro_export]
macro_rules! __dma_rmb {
    () => { barrier!() };
}

#[macro_export]
macro_rules! __dma_wmb {
    () => { barrier!() };
}

#[macro_export]
macro_rules! __smp_mb {
    () => {{
        unsafe { core::arch::asm!("lock addl $0, -4(%rsp)", options(nostack)); }
    }};
}

#[macro_export]
macro_rules! __smp_rmb {
    () => { dma_rmb!() };
}

#[macro_export]
macro_rules! __smp_wmb {
    () => { barrier!() };
}

#[macro_export]
macro_rules! __smp_store_mb {
    ($var:expr, $value:expr) => {{ let _ = xchg!($var, $value); }};
}

#[macro_export]
macro_rules! __smp_store_release {
    ($p:expr, $v:expr) => {{
        compiletime_assert_atomic_type!(*$p);
        barrier!();
        WRITE_ONCE!(*$p, $v);
    }};
}

#[macro_export]
macro_rules! __smp_load_acquire {
    ($p:expr) => {{
        let ___p1 = READ_ONCE!(*$p);
        compiletime_assert_atomic_type!(*$p);
        barrier!();
        ___p1
    }};
}

/* Atomic operations are already serializing on x86. */
#[macro_export]
macro_rules! __smp_mb__before_atomic { () => {}; }

#[macro_export]
macro_rules! __smp_mb__after_atomic { () => {}; }

/* Writing to CR3 provides a full memory barrier in switch_mm(). */
#[macro_export]
macro_rules! smp_mb__after_switch_mm { () => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
