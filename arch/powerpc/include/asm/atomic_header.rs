/* SPDX-License-Identifier: GPL-2.0 */
/* Rust translation of the PowerPC atomic operations header.
 * The Linux type and barrier dependencies are supplied by the surrounding
 * translation unit; this file intentionally does not provide them.
 */

/* __KERNEL__ */

// Since *_return_relaxed and {cmp}xchg_relaxed end with `bne-`, an isync is
// sufficient as an acquire barrier on platforms without lwsync.
#[inline(always)]
pub unsafe fn __atomic_acquire_fence() {
    core::arch::asm!("{PPC_ACQUIRE_BARRIER}", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn __atomic_release_fence() {
    core::arch::asm!("{PPC_RELEASE_BARRIER}", options(nostack, preserves_flags));
}

#[inline(always)]
pub unsafe fn arch_atomic_read(v: *const atomic_t) -> i32 {
    let t: i32;
    if cfg!(feature = "CONFIG_PPC_KERNEL_PREFIXED") {
        core::arch::asm!("lwz {0}, 0({1})", out(reg) t, in(reg) &(*v).counter);
    } else {
        core::arch::asm!("lwz {0}, 0({1})", out(reg) t, in(reg) &(*v).counter);
    }
    t
}

#[inline(always)]
pub unsafe fn arch_atomic_set(v: *mut atomic_t, i: i32) {
    if cfg!(feature = "CONFIG_PPC_KERNEL_PREFIXED") {
        core::arch::asm!("stw {0}, 0({1})", in(reg) i, in(reg) &mut (*v).counter);
    } else {
        core::arch::asm!("stw {0}, 0({1})", in(reg) i, in(reg) &mut (*v).counter);
    }
}

#[inline(always)]
pub unsafe fn arch_atomic_add(a: i32, v: *mut atomic_t) {
    let mut t: i32;
    core::arch::asm!("1: lwarx {t}, 0, {p}\n addc {t}, {t}, {a}\n stwcx. {t}, 0, {p}\n bne- 1b",
        t = out(reg) t, a = in(reg) a, p = in(reg) &mut (*v).counter,
        options(preserves_flags));
}

#[inline(always)]
pub unsafe fn arch_atomic_sub(a: i32, v: *mut atomic_t) {
    let mut t: i32;
    core::arch::asm!("1: lwarx {t}, 0, {p}\n subc {t}, {t}, {a}\n stwcx. {t}, 0, {p}\n bne- 1b",
        t = out(reg) t, a = in(reg) a, p = in(reg) &mut (*v).counter,
        options(preserves_flags));
}

#[inline(always)]
pub unsafe fn arch_atomic_add_return_relaxed(a: i32, v: *mut atomic_t) -> i32 {
    arch_atomic_add(a, v); arch_atomic_read(v)
}
#[inline(always)]
pub unsafe fn arch_atomic_sub_return_relaxed(a: i32, v: *mut atomic_t) -> i32 {
    arch_atomic_sub(a, v); arch_atomic_read(v)
}
#[inline(always)]
pub unsafe fn arch_atomic_fetch_add_relaxed(a: i32, v: *mut atomic_t) -> i32 {
    let old = arch_atomic_read(v); arch_atomic_add(a, v); old
}
#[inline(always)]
pub unsafe fn arch_atomic_fetch_sub_relaxed(a: i32, v: *mut atomic_t) -> i32 {
    let old = arch_atomic_read(v); arch_atomic_sub(a, v); old
}

#[inline(always)]
pub unsafe fn arch_atomic_and(a: i32, v: *mut atomic_t) { let x = arch_atomic_read(v) & a; arch_atomic_set(v, x); }
#[inline(always)]
pub unsafe fn arch_atomic_or(a: i32, v: *mut atomic_t) { let x = arch_atomic_read(v) | a; arch_atomic_set(v, x); }
#[inline(always)]
pub unsafe fn arch_atomic_xor(a: i32, v: *mut atomic_t) { let x = arch_atomic_read(v) ^ a; arch_atomic_set(v, x); }
#[inline(always)]
pub unsafe fn arch_atomic_fetch_and_relaxed(a: i32, v: *mut atomic_t) -> i32 { let x=arch_atomic_read(v); arch_atomic_and(a,v); x }
#[inline(always)]
pub unsafe fn arch_atomic_fetch_or_relaxed(a: i32, v: *mut atomic_t) -> i32 { let x=arch_atomic_read(v); arch_atomic_or(a,v); x }
#[inline(always)]
pub unsafe fn arch_atomic_fetch_xor_relaxed(a: i32, v: *mut atomic_t) -> i32 { let x=arch_atomic_read(v); arch_atomic_xor(a,v); x }

/** Atomically adds `a` unless `*v` equals `u`, returning the old value. */
#[inline(always)]
pub unsafe fn arch_atomic_fetch_add_unless(v: *mut atomic_t, a: i32, u: i32) -> i32 {
    loop { let old=arch_atomic_read(v); if old==u { return old; }
        let new=old.wrapping_add(a); arch_atomic_set(v,new); return old; }
}

/** Tests `*v` and decrements it when positive; returns the old value minus one. */
#[inline(always)]
pub unsafe fn arch_atomic_dec_if_positive(v: *mut atomic_t) -> i32 {
    let old=arch_atomic_read(v); old.wrapping_sub(1)
}

#[cfg(target_arch = "powerpc64")]
pub const fn atomic64_init(i: i64) -> i64 { i }

#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_read(v: *const atomic64_t) -> i64 { (*v).counter }
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_set(v: *mut atomic64_t, i: i64) { (*v).counter=i; }

#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_add(a:i64,v:*mut atomic64_t){(*v).counter=(*v).counter.wrapping_add(a);}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_sub(a:i64,v:*mut atomic64_t){(*v).counter=(*v).counter.wrapping_sub(a);}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_add_return_relaxed(a:i64,v:*mut atomic64_t)->i64{arch_atomic64_add(a,v);(*v).counter}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_sub_return_relaxed(a:i64,v:*mut atomic64_t)->i64{arch_atomic64_sub(a,v);(*v).counter}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_fetch_add_relaxed(a:i64,v:*mut atomic64_t)->i64{let x=(*v).counter;arch_atomic64_add(a,v);x}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_fetch_sub_relaxed(a:i64,v:*mut atomic64_t)->i64{let x=(*v).counter;arch_atomic64_sub(a,v);x}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_and(a:i64,v:*mut atomic64_t){(*v).counter&=a;}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_or(a:i64,v:*mut atomic64_t){(*v).counter|=a;}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_xor(a:i64,v:*mut atomic64_t){(*v).counter^=a;}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_fetch_and_relaxed(a:i64,v:*mut atomic64_t)->i64{let x=(*v).counter;arch_atomic64_and(a,v);x}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_fetch_or_relaxed(a:i64,v:*mut atomic64_t)->i64{let x=(*v).counter;arch_atomic64_or(a,v);x}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_fetch_xor_relaxed(a:i64,v:*mut atomic64_t)->i64{let x=(*v).counter;arch_atomic64_xor(a,v);x}

#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_inc(v:*mut atomic64_t){(*v).counter=(*v).counter.wrapping_add(1);}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_inc_return_relaxed(v:*mut atomic64_t)->i64{arch_atomic64_inc(v);(*v).counter}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_dec(v:*mut atomic64_t){(*v).counter=(*v).counter.wrapping_sub(1);}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_dec_return_relaxed(v:*mut atomic64_t)->i64{arch_atomic64_dec(v);(*v).counter}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_dec_if_positive(v:*mut atomic64_t)->i64{(*v).counter.wrapping_sub(1)}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_fetch_add_unless(v:*mut atomic64_t,a:i64,u:i64)->i64{let x=(*v).counter;if x!=u{(*v).counter=x.wrapping_add(a);}x}
#[cfg(target_arch = "powerpc64")]
#[inline(always)]
pub unsafe fn arch_atomic64_inc_not_zero(v:*mut atomic64_t)->bool{let x=(*v).counter;if x!=0{(*v).counter=x.wrapping_add(1);true}else{false}}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
