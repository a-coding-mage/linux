/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Generic barrier definitions translated from the C header. */

/* Dependencies supplied by other translated units: barrier, kcsan_*,
 * __smp_*, READ_ONCE/WRITE_ONCE, compiletime_assert_atomic_type, and
 * cpu_relax. Build-time CONFIG_SMP and architecture-provided variants are
 * preserved below as conditional intent. */

#[inline(always)]
pub unsafe fn nop() {
    core::arch::asm!("nop", options(nostack, preserves_flags));
}

#[macro_export]
macro_rules! mb { () => { barrier() }; }
#[macro_export]
macro_rules! rmb { () => { mb!() }; }
#[macro_export]
macro_rules! wmb { () => { mb!() }; }
#[macro_export]
macro_rules! dma_mb { () => { mb!() }; }
#[macro_export]
macro_rules! dma_rmb { () => { rmb!() }; }
#[macro_export]
macro_rules! dma_wmb { () => { wmb!() }; }

#[macro_export]
macro_rules! __smp_mb { () => { mb!() }; }
#[macro_export]
macro_rules! __smp_rmb { () => { rmb!() }; }
#[macro_export]
macro_rules! __smp_wmb { () => { wmb!() }; }

/* CONFIG_SMP selects the following SMP definitions in the original header. */
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! smp_mb { () => {{ kcsan_mb(); __smp_mb!(); }}; }
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! smp_mb { () => { barrier() }; }
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! smp_rmb { () => {{ kcsan_rmb(); __smp_rmb!(); }}; }
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! smp_rmb { () => { barrier() }; }
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! smp_wmb { () => {{ kcsan_wmb(); __smp_wmb!(); }}; }
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! smp_wmb { () => { barrier() }; }

#[macro_export]
macro_rules! __smp_store_mb { ($var:expr, $value:expr) => {{ WRITE_ONCE!($var, $value); __smp_mb!(); }}; }
#[macro_export]
macro_rules! __smp_mb__before_atomic { () => { __smp_mb!() }; }
#[macro_export]
macro_rules! __smp_mb__after_atomic { () => { __smp_mb!() }; }
#[macro_export]
macro_rules! __smp_store_release { ($p:expr, $v:expr) => {{ __smp_mb!(); WRITE_ONCE!(*$p, $v); }}; }
#[macro_export]
macro_rules! __smp_load_acquire { ($p:expr) => {{ let ___p1 = READ_ONCE!(*$p); __smp_mb!(); ___p1 }}; }

#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! smp_store_mb { ($var:expr, $value:expr) => {{ kcsan_mb(); __smp_store_mb!($var, $value); }}; }
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! smp_store_mb { ($var:expr, $value:expr) => {{ WRITE_ONCE!($var, $value); barrier(); }}; }
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! smp_mb__before_atomic { () => {{ kcsan_mb(); __smp_mb__before_atomic!(); }}; }
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! smp_mb__before_atomic { () => { barrier() }; }
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! smp_mb__after_atomic { () => {{ kcsan_mb(); __smp_mb__after_atomic!(); }}; }
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! smp_mb__after_atomic { () => { barrier() }; }
#[cfg(feature = "CONFIG_SMP")]
#[macro_export]
macro_rules! smp_store_release { ($p:expr, $v:expr) => {{ kcsan_release(); __smp_store_release!($p, $v); }}; }
#[cfg(not(feature = "CONFIG_SMP"))]
#[macro_export]
macro_rules! smp_store_release { ($p:expr, $v:expr) => {{ barrier(); WRITE_ONCE!(*$p, $v); }}; }
#[macro_export]
macro_rules! smp_load_acquire { ($p:expr) => { __smp_load_acquire!($p) }; }

#[macro_export]
macro_rules! virt_mb { () => {{ kcsan_mb(); __smp_mb!(); }}; }
#[macro_export]
macro_rules! virt_rmb { () => {{ kcsan_rmb(); __smp_rmb!(); }}; }
#[macro_export]
macro_rules! virt_wmb { () => {{ kcsan_wmb(); __smp_wmb!(); }}; }
#[macro_export]
macro_rules! virt_store_mb { ($var:expr, $value:expr) => {{ kcsan_mb(); __smp_store_mb!($var, $value); }}; }
#[macro_export]
macro_rules! virt_mb__before_atomic { () => {{ kcsan_mb(); __smp_mb__before_atomic!(); }}; }
#[macro_export]
macro_rules! virt_mb__after_atomic { () => {{ kcsan_mb(); __smp_mb__after_atomic!(); }}; }
#[macro_export]
macro_rules! virt_store_release { ($p:expr, $v:expr) => {{ kcsan_release(); __smp_store_release!($p, $v); }}; }
#[macro_export]
macro_rules! virt_load_acquire { ($p:expr) => { __smp_load_acquire!($p) }; }

#[macro_export]
macro_rules! smp_acquire__after_ctrl_dep { () => { smp_rmb!() }; }

#[macro_export]
macro_rules! smp_cond_load_relaxed {
    ($ptr:expr, $cond:expr) => {{
        let __ptr = $ptr;
        loop {
            let val = READ_ONCE!(*__ptr);
            if $cond { break val; }
            cpu_relax();
        }
    }};
}
#[macro_export]
macro_rules! smp_cond_load_acquire {
    ($ptr:expr, $cond:expr) => {{
        let val = smp_cond_load_relaxed!($ptr, $cond);
        smp_acquire__after_ctrl_dep!();
        val
    }};
}

#[macro_export]
macro_rules! pmem_wmb { () => { wmb!() }; }
#[macro_export]
macro_rules! io_stop_wc { () => {{}}; }
#[macro_export]
macro_rules! smp_mb__after_switch_mm { () => { smp_mb!() }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
