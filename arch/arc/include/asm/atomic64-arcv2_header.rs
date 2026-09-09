/* SPDX-License-Identifier: GPL-2.0-only */
/* ARCv2 64-bit exclusive load/store atomic operations. */

use core::arch::asm;

pub type s64 = i64;
pub type u64 = core::primitive::u64;

#[repr(C, align(8))]
pub struct atomic64_t {
    pub counter: s64,
}

#[inline]
pub const fn ATOMIC64_INIT(a: s64) -> atomic64_t {
    atomic64_t { counter: a }
}

unsafe extern "C" {
    fn smp_mb();
}

#[inline]
pub unsafe fn arch_atomic64_read(v: *const atomic64_t) -> s64 {
    let mut val: s64;
    asm!("ldd {0}, [{1}]", out(reg) val, in(reg) core::ptr::addr_of!((*v).counter));
    val
}

#[inline]
pub unsafe fn arch_atomic64_set(v: *mut atomic64_t, a: s64) {
    asm!("std {0}, [{1}]", in(reg) a, in(reg) core::ptr::addr_of_mut!((*v).counter), options(nostack));
}

macro_rules! atomic64_op {
    ($name:ident, $lo:literal, $hi:literal) => {
        #[inline]
        pub unsafe fn $name(a: s64, v: *mut atomic64_t) {
            let mut val: s64;
            asm!(
                "1:", "llockd {0}, [{1}]", $lo, $hi,
                "scondd {0}, [{1}]", "bnz 1b",
                out(reg) val, in(reg) core::ptr::addr_of_mut!((*v).counter), in(reg) a,
            );
        }
    };
}

macro_rules! atomic64_op_return {
    ($name:ident, $lo:literal, $hi:literal) => {
        #[inline]
        pub unsafe fn $name(a: s64, v: *mut atomic64_t) -> s64 {
            let mut val: s64;
            asm!("1:", "llockd {0}, [{1}]", $lo, $hi, "scondd {0}, [{1}]", "bnz 1b",
                out(reg) val, in(reg) core::ptr::addr_of_mut!((*v).counter), in(reg) a);
            val
        }
    };
}

macro_rules! atomic64_fetch_op {
    ($name:ident, $lo:literal, $hi:literal) => {
        #[inline]
        pub unsafe fn $name(a: s64, v: *mut atomic64_t) -> s64 {
            let mut orig: s64;
            let mut val: s64;
            asm!("1:", "llockd {0}, [{2}]", $lo, $hi, "scondd {1}, [{2}]", "bnz 1b",
                out(reg) orig, out(reg) val, in(reg) core::ptr::addr_of_mut!((*v).counter), in(reg) a);
            orig
        }
    };
}

atomic64_op!(arch_atomic64_add, "add.f {0}, {0}, {2}", "adc {0}, {0}, {2}");
atomic64_op_return!(arch_atomic64_add_return_relaxed, "add.f {0}, {0}, {2}", "adc {0}, {0}, {2}");
atomic64_fetch_op!(arch_atomic64_fetch_add_relaxed, "add.f {1}, {0}, {3}", "adc {1}, {0}, {3}");
atomic64_op!(arch_atomic64_sub, "sub.f {0}, {0}, {2}", "sbc {0}, {0}, {2}");
atomic64_op_return!(arch_atomic64_sub_return_relaxed, "sub.f {0}, {0}, {2}", "sbc {0}, {0}, {2}");
atomic64_fetch_op!(arch_atomic64_fetch_sub_relaxed, "sub.f {1}, {0}, {3}", "sbc {1}, {0}, {3}");
atomic64_op!(arch_atomic64_and, "and {0}, {0}, {2}", "and {0}, {0}, {2}");
atomic64_fetch_op!(arch_atomic64_fetch_and_relaxed, "and {1}, {0}, {3}", "and {1}, {0}, {3}");
atomic64_op!(arch_atomic64_andnot, "bic {0}, {0}, {2}", "bic {0}, {0}, {2}");
atomic64_fetch_op!(arch_atomic64_fetch_andnot_relaxed, "bic {1}, {0}, {3}", "bic {1}, {0}, {3}");
atomic64_op!(arch_atomic64_or, "or {0}, {0}, {2}", "or {0}, {0}, {2}");
atomic64_fetch_op!(arch_atomic64_fetch_or_relaxed, "or {1}, {0}, {3}", "or {1}, {0}, {3}");
atomic64_op!(arch_atomic64_xor, "xor {0}, {0}, {2}", "xor {0}, {0}, {2}");
atomic64_fetch_op!(arch_atomic64_fetch_xor_relaxed, "xor {1}, {0}, {3}", "xor {1}, {0}, {3}");

#[inline]
pub unsafe fn __arch_cmpxchg64_relaxed(ptr: *mut core::ffi::c_void, old: u64, new: u64) -> u64 {
    let mut prev: u64;
    asm!("1: llockd {0}, [{1}]", "brne {0}, {2}, 2f", "scondd {3}, [{1}]", "bnz 1b", "2:",
        out(reg) prev, in(reg) ptr, in(reg) old, in(reg) new);
    prev
}

#[inline]
pub unsafe fn arch_atomic64_xchg(ptr: *mut atomic64_t, new: s64) -> s64 {
    smp_mb();
    let mut prev: s64;
    asm!("1: llockd {0}, [{1}]", "scondd {2}, [{1}]", "bnz 1b", "2:",
        out(reg) prev, in(reg) ptr, in(reg) new);
    smp_mb();
    prev
}

#[inline]
pub unsafe fn arch_atomic64_dec_if_positive(v: *mut atomic64_t) -> s64 {
    smp_mb();
    let mut val: s64;
    asm!("1: llockd {0}, [{1}]", "sub.f {0}, {0}, 1", "sub.c {0}, {0}, 1", "brlt {0}, 0, 2f", "scondd {0}, [{1}]", "bnz 1b", "2:",
        out(reg) val, in(reg) core::ptr::addr_of_mut!((*v).counter));
    smp_mb();
    val
}

#[inline]
pub unsafe fn arch_atomic64_fetch_add_unless(v: *mut atomic64_t, a: s64, u: s64) -> s64 {
    smp_mb();
    let mut old: s64;
    let mut temp: s64;
    asm!("1: llockd {0}, [{2}]", "brne {0}, {4}, 2f", "breq.d {0}, {4}, 3f", "2:",
        "add.f {1}, {0}, {3}", "adc {1}, {0}, {3}", "scondd {1}, [{2}]", "bnz 1b", "3:",
        out(reg) old, out(reg) temp, in(reg) core::ptr::addr_of_mut!((*v).counter), in(reg) a, in(reg) u);
    smp_mb();
    old
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
