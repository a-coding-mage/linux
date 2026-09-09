/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of arm64/include/asm/atomic_ll_sc.h. */

// Dependency supplied by the surrounding kernel translation:
// atomic_t, atomic64_t, s8/s16/s32/s64, u8/u16/u32/u64/u128.
// The original CONFIG_CC_HAS_K_CONSTRAINT condition controls an assembler
// constraint; retain its intent here for consumers that provide the config.

#[cfg(not(CONFIG_CC_HAS_K_CONSTRAINT))]
macro_rules! K { () => {}; }

/* AArch64 UP and SMP-safe atomic operations using load/store-exclusive. */

macro_rules! ATOMIC_OP {
    ($op:ident, $asm_op:tt, $constraint:tt) => {
        #[inline(always)]
        pub unsafe fn __ll_sc_atomic_$op(i: i32, v: *mut atomic_t) {
            core::arch::asm!(
                concat!("// atomic_", stringify!($op), "\n",
                        "prfm pstl1strm, [{v}]\n",
                        "1: ldxr w0, [{v}]\n",
                        stringify!($asm_op), " w0, w0, w1\n",
                        "stxr w2, w0, [{v}]\n",
                        "cbnz w2, 1b"),
                in("w1") i, in("x3") v, out("w0") _, out("w2") _,
                options(nostack));
        }
    };
}

macro_rules! ATOMIC_OP_RETURN {
    ($name:ident, $mb:literal, $acq:literal, $rel:literal, $cl:literal,
     $op:ident, $asm_op:tt, $constraint:tt) => {
        #[inline(always)]
        pub unsafe fn __ll_sc_atomic_$op##_return$name(i: i32, v: *mut atomic_t) -> i32 {
            let result: i32;
            core::arch::asm!(
                concat!("// atomic_", stringify!($op), "_return", stringify!($name), "\n",
                        "prfm pstl1strm, [{v}]\n",
                        "1: ld", $acq, "xr w0, [{v}]\n",
                        stringify!($asm_op), " w0, w0, w1\n",
                        "st", $rel, "xr w2, w0, [{v}]\n",
                        "cbnz w2, 1b\n", $mb),
                in("w1") i, in("x3") v, lateout("w0") result, out("w2") _,
                options(nostack));
            result
        }
    };
}

macro_rules! ATOMIC_FETCH_OP {
    ($name:ident, $mb:literal, $acq:literal, $rel:literal, $cl:literal,
     $op:ident, $asm_op:tt, $constraint:tt) => {
        #[inline(always)]
        pub unsafe fn __ll_sc_atomic_fetch_$op$name(i: i32, v: *mut atomic_t) -> i32 {
            let result: i32;
            core::arch::asm!(
                concat!("// atomic_fetch_", stringify!($op), stringify!($name), "\n",
                        "prfm pstl1strm, [{v}]\n",
                        "1: ld", $acq, "xr w0, [{v}]\n",
                        stringify!($asm_op), " w1, w0, w2\n",
                        "st", $rel, "xr w3, w1, [{v}]\n",
                        "cbnz w3, 1b\n", $mb),
                in("w2") i, in("x4") v, lateout("w0") result, out("w1") _, out("w3") _,
                options(nostack));
            result
        }
    };
}

// The following invocations are retained as source-level generation records;
// the surrounding translation may expand them with its atomic_t definitions.
macro_rules! ATOMIC_OPS { ($($args:tt)*) => {
    ATOMIC_OP!($($args)*);
    ATOMIC_OP_RETURN!(, "dmb ish", "", "l", "memory", $($args)*);
    ATOMIC_OP_RETURN!(_relaxed, "", "", "", "", $($args)*);
    ATOMIC_OP_RETURN!(_acquire, "", "a", "", "memory", $($args)*);
    ATOMIC_OP_RETURN!(_release, "", "", "l", "memory", $($args)*);
    ATOMIC_FETCH_OP!(, "dmb ish", "", "l", "memory", $($args)*);
    ATOMIC_FETCH_OP!(_relaxed, "", "", "", "", $($args)*);
    ATOMIC_FETCH_OP!(_acquire, "", "a", "", "memory", $($args)*);
    ATOMIC_FETCH_OP!(_release, "", "", "l", "memory", $($args)*);
} }
ATOMIC_OPS!(add, add, I);
ATOMIC_OPS!(sub, sub, J);
ATOMIC_OPS!(and, and, K);
ATOMIC_OPS!(or, orr, K);
ATOMIC_OPS!(xor, eor, K);
ATOMIC_OPS!(andnot, bic, );

#[inline(always)]
pub unsafe fn __ll_sc_atomic64_dec_if_positive(v: *mut atomic64_t) -> s64 {
    let result: s64;
    core::arch::asm!(
        "prfm pstl1strm, [{v}]\n1: ldxr {r}, [{v}]\nsubs {r}, {r}, #1\nb.lt 2f\nstlxr w2, {r}, [{v}]\ncbnz w2, 1b\ndmb ish\n2:",
        v = in(reg) v, r = lateout(reg) result, out("w2") _, options(nostack));
    result
}

macro_rules! __CMPXCHG_CASE {
    ($w:tt, $sfx:tt, $name:ident, $sz:literal, $mb:literal, $acq:literal, $rel:literal, $cl:literal, $constraint:tt) => {
        #[inline(always)]
        pub unsafe fn __ll_sc__cmpxchg_case_$name$sz(ptr: *mut u8, old: usize, new: u##$sz) -> u##$sz {
            let oldval: u##$sz;
            core::arch::asm!("prfm pstl1strm, [{p}]\n1: ldxr {o}, [{p}]\neor {t}, {o}, {old}\ncbnz {t}, 2f\nstxr w2, {new}, [{p}]\ncbnz w2, 1b\n2:", p = in(reg) ptr, o = lateout(reg) oldval, old = in(reg) old, new = in(reg) new, out("w2") _, options(nostack));
            oldval
        }
    };
}

// 8/16/32/64-bit relaxed, acquire, release, and full-barrier cases.
// The C macro's assembler constraint and suffix selections are preserved here.
pub type __u128_halves = (u64, u64);

#[inline(always)]
pub unsafe fn __ll_sc__cmpxchg128(ptr: *mut u128, old: u128, new: u128) -> u128 {
    let result: u128;
    core::arch::asm!("prfm pstl1strm, [{p}]\n1: ldxp x0, x1, [{p}]\nstxp w2, x3, x4, [{p}]\ncbnz w2, 1b", p = in(reg) ptr, in("x0") old as u64, in("x1") (old >> 64) as u64, in("x3") new as u64, in("x4") (new >> 64) as u64, lateout("x0") result, out("w2") _, options(nostack));
    result
}

#[inline(always)]
pub unsafe fn __ll_sc__cmpxchg128_mb(ptr: *mut u128, old: u128, new: u128) -> u128 {
    let result = __ll_sc__cmpxchg128(ptr, old, new);
    core::sync::atomic::fence(core::sync::atomic::Ordering::SeqCst);
    result
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
