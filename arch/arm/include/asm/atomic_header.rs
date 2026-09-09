/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of arch/arm/include/asm/atomic.h. */

// Dependencies supplied by the surrounding kernel translation.
use core::arch::asm;

#[cfg(feature = "linux_kernel")]
extern "C" {
    fn prefetchw<T>(p: *const T);
    fn smp_mb();
    fn raw_local_irq_save(flags: *mut c_ulong);
    fn raw_local_irq_restore(flags: c_ulong);
}

// `atomic_t`, `s64`, `c_ulong`, and `likely` are supplied by other headers.

#[inline(always)]
pub unsafe fn arch_atomic_read(v: *const atomic_t) -> i32 { core::ptr::read_volatile(&(*v).counter) }
#[inline(always)]
pub unsafe fn arch_atomic_set(v: *mut atomic_t, i: i32) { core::ptr::write_volatile(&mut (*v).counter, i) }

#[cfg(target_arch = "arm")]
macro_rules! atomic_op {
    ($op:ident, $expr:expr) => {
        #[inline]
        pub unsafe fn $op(i: i32, v: *mut atomic_t) {
            prefetchw(&(*v).counter);
            let mut result: i32;
            let mut tmp: usize;
            asm!("1: ldrex {r}, [{p}]\n {ins}\n strex {t}, {r}, [{p}]\n teq {t}, #0\n bne 1b",
                 r = out(reg) result, t = out(reg) tmp, p = in(reg) &mut (*v).counter,
                 ins = const stringify!($expr), options(nostack));
            let _ = (result, tmp, i);
        }
    };
}

#[inline]
pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) { atomic_update32(v, i, 0); }
#[inline]
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) { atomic_update32(v, i, 1); }
#[inline]
pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) { atomic_update32(v, i, 2); }
#[inline]
pub unsafe fn arch_atomic_andnot(i: i32, v: *mut atomic_t) { atomic_update32(v, i, 3); }
#[inline]
pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) { atomic_update32(v, i, 4); }
#[inline]
pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) { atomic_update32(v, i, 5); }

#[inline]
unsafe fn atomic_update32(v: *mut atomic_t, i: i32, op: i32) {
    prefetchw(&(*v).counter);
    loop {
        let old = core::ptr::read_volatile(&(*v).counter);
        let new = match op { 0 => old.wrapping_add(i), 1 => old.wrapping_sub(i), 2 => old & i,
            3 => old & !i, 4 => old | i, _ => old ^ i };
        if core::ptr::compare_exchange_weak(&mut (*v).counter, old, new, core::sync::atomic::Ordering::Relaxed, core::sync::atomic::Ordering::Relaxed).is_ok() { return; }
    }
}

#[inline]
pub unsafe fn arch_atomic_add_return_relaxed(i: i32, v: *mut atomic_t) -> i32 { atomic_fetch_update32(v, i, 0, true) }
#[inline]
pub unsafe fn arch_atomic_sub_return_relaxed(i: i32, v: *mut atomic_t) -> i32 { atomic_fetch_update32(v, i, 1, true) }
#[inline]
pub unsafe fn arch_atomic_fetch_add_relaxed(i: i32, v: *mut atomic_t) -> i32 { atomic_fetch_update32(v, i, 0, false) }
#[inline]
pub unsafe fn arch_atomic_fetch_sub_relaxed(i: i32, v: *mut atomic_t) -> i32 { atomic_fetch_update32(v, i, 1, false) }
#[inline]
unsafe fn atomic_fetch_update32(v: *mut atomic_t, i: i32, op: i32, ret_new: bool) -> i32 {
    loop { let old = core::ptr::read_volatile(&(*v).counter); let new = if op == 0 { old.wrapping_add(i) } else { old.wrapping_sub(i) };
        if core::ptr::compare_exchange_weak(&mut (*v).counter, old, new, core::sync::atomic::Ordering::Relaxed, core::sync::atomic::Ordering::Relaxed).is_ok() { return if ret_new { new } else { old }; } }
}

#[inline]
pub unsafe fn arch_atomic_cmpxchg_relaxed(v: *mut atomic_t, old: i32, new: i32) -> i32 {
    let current = core::ptr::read_volatile(&(*v).counter);
    if current == old { let _ = core::ptr::compare_exchange(&mut (*v).counter, old, new, core::sync::atomic::Ordering::Relaxed, core::sync::atomic::Ordering::Relaxed); }
    current
}

#[repr(C)]
pub struct atomic64_t { pub counter: i64 }
pub const fn atomic64_init(i: i64) -> atomic64_t { atomic64_t { counter: i } }

#[inline] pub unsafe fn arch_atomic64_read(v: *const atomic64_t) -> i64 { core::ptr::read_volatile(&(*v).counter) }
#[inline] pub unsafe fn arch_atomic64_set(v: *mut atomic64_t, i: i64) { core::ptr::write_volatile(&mut (*v).counter, i) }
#[inline] pub unsafe fn arch_atomic64_add(i: i64, v: *mut atomic64_t) { atomic_update64(v, i, 0); }
#[inline] pub unsafe fn arch_atomic64_sub(i: i64, v: *mut atomic64_t) { atomic_update64(v, i, 1); }
#[inline] pub unsafe fn arch_atomic64_and(i: i64, v: *mut atomic64_t) { atomic_update64(v, i, 2); }
#[inline] pub unsafe fn arch_atomic64_andnot(i: i64, v: *mut atomic64_t) { atomic_update64(v, i, 3); }
#[inline] pub unsafe fn arch_atomic64_or(i: i64, v: *mut atomic64_t) { atomic_update64(v, i, 4); }
#[inline] pub unsafe fn arch_atomic64_xor(i: i64, v: *mut atomic64_t) { atomic_update64(v, i, 5); }
#[inline] unsafe fn atomic_update64(v: *mut atomic64_t, i: i64, op: i32) { loop { let o=(*v).counter; let n=match op {0=>o.wrapping_add(i),1=>o.wrapping_sub(i),2=>o&i,3=>o&!i,4=>o|i,_=>o^i}; if (*v).counter==o { (*v).counter=n; return; } } }
#[inline] pub unsafe fn arch_atomic64_cmpxchg_relaxed(v: *mut atomic64_t, old: i64, new: i64) -> i64 { let o=(*v).counter; if o==old { (*v).counter=new; } o }
#[inline] pub unsafe fn arch_atomic64_xchg_relaxed(v: *mut atomic64_t, new: i64) -> i64 { let o=(*v).counter; (*v).counter=new; o }
#[inline] pub unsafe fn arch_atomic64_dec_if_positive(v: *mut atomic64_t) -> i64 { let o=(*v).counter; if o>=0 { (*v).counter=o-1; o-1 } else { o } }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
