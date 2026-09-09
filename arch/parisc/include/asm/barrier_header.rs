/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: <asm/alternative.h> and <asm-generic/barrier.h> supply the
// alternative-instruction machinery and generic barrier definitions.
// The original header is excluded when compiling assembly (__ASSEMBLER__).

/// The synchronize caches instruction is a nop on systems in which all
/// memory references are performed in order.
#[inline(always)]
pub unsafe fn synchronize_caches() {
    core::arch::asm!("sync", options(nostack, preserves_flags));
}

// CONFIG_SMP selects the synchronize_caches implementation; otherwise these
// barriers map to the generic compiler barrier supplied by asm-generic/barrier.h.
#[cfg(CONFIG_SMP)]
#[inline(always)]
pub unsafe fn mb() {
    synchronize_caches();
}

#[cfg(not(CONFIG_SMP))]
#[inline(always)]
pub fn mb() {
    // Equivalent to barrier() from asm-generic/barrier.h.
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
}

#[inline(always)]
pub unsafe fn rmb() {
    mb();
}

#[inline(always)]
pub unsafe fn wmb() {
    mb();
}

#[inline(always)]
pub unsafe fn dma_rmb() {
    mb();
}

#[inline(always)]
pub unsafe fn dma_wmb() {
    mb();
}

#[inline(always)]
pub unsafe fn __smp_mb() {
    mb();
}

#[inline(always)]
pub unsafe fn __smp_rmb() {
    mb();
}

#[inline(always)]
pub unsafe fn __smp_wmb() {
    mb();
}

// The C macros below use typeof, compiletime_assert_atomic_type, and
// architecture-specific PA-RISC acquire/release loads and stores.  They are
// retained as Rust macros to preserve their call-site type inference and
// pointer behavior.
#[macro_export]
macro_rules! __smp_store_release {
    ($p:expr, $v:expr) => {{
        let __p = $p;
        let __v = $v;
        match core::mem::size_of_val(unsafe { &*__p }) {
            1 => core::arch::asm!("stb,ma {0},0({1})", in(reg) __v as u8, in(reg) __p, options(nostack)),
            2 => core::arch::asm!("sth,ma {0},0({1})", in(reg) __v as u16, in(reg) __p, options(nostack)),
            4 => core::arch::asm!("stw,ma {0},0({1})", in(reg) __v as u32, in(reg) __p, options(nostack)),
            8 if cfg!(CONFIG_64BIT) => core::arch::asm!("std,ma {0},0({1})", in(reg) __v as u64, in(reg) __p, options(nostack)),
            _ => (),
        }
    }};
}

#[macro_export]
macro_rules! __smp_load_acquire {
    ($p:expr) => {{
        let __p = $p;
        let mut __u = core::mem::MaybeUninit::uninit();
        match core::mem::size_of_val(unsafe { &*__p }) {
            1 => core::arch::asm!("ldb,ma 0({1}),{0}", out(reg) *(unsafe { __u.as_mut_ptr() as *mut u8 }), in(reg) __p, options(nostack)),
            2 => core::arch::asm!("ldh,ma 0({1}),{0}", out(reg) *(unsafe { __u.as_mut_ptr() as *mut u16 }), in(reg) __p, options(nostack)),
            4 => core::arch::asm!("ldw,ma 0({1}),{0}", out(reg) *(unsafe { __u.as_mut_ptr() as *mut u32 }), in(reg) __p, options(nostack)),
            8 if cfg!(CONFIG_64BIT) => core::arch::asm!("ldd,ma 0({1}),{0}", out(reg) *(unsafe { __u.as_mut_ptr() as *mut u64 }), in(reg) __p, options(nostack)),
            _ => (),
        }
        unsafe { __u.assume_init() }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
