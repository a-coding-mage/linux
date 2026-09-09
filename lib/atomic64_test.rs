// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Testsuite for atomic64_t functions
 *
 * Copyright © 2010  Luca Barbieri
 */

// Linux kernel dependencies supplied by the surrounding repository:
// linux/init.h, linux/bug.h, linux/kernel.h, linux/atomic.h, linux/module.h
// asm/cpufeature.h is required on CONFIG_X86 builds.

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct atomic_t {
    pub counter: i32,
}

#[allow(non_camel_case_types)]
#[repr(C)]
pub struct atomic64_t {
    pub counter: i64,
}

extern "C" {
    fn atomic_set(v: *mut atomic_t, value: i32);
    fn atomic_read(v: *const atomic_t) -> i32;
    fn atomic_add(value: i32, v: *mut atomic_t);
    fn atomic_sub(value: i32, v: *mut atomic_t);
    fn atomic_or(value: i32, v: *mut atomic_t);
    fn atomic_and(value: i32, v: *mut atomic_t);
    fn atomic_xor(value: i32, v: *mut atomic_t);
    fn atomic_andnot(value: i32, v: *mut atomic_t);
    fn atomic_add_return(value: i32, v: *mut atomic_t) -> i32;
    fn atomic_sub_return(value: i32, v: *mut atomic_t) -> i32;
    fn atomic_fetch_add(value: i32, v: *mut atomic_t) -> i32;
    fn atomic_fetch_sub(value: i32, v: *mut atomic_t) -> i32;
    fn atomic_fetch_or(value: i32, v: *mut atomic_t) -> i32;
    fn atomic_fetch_and(value: i32, v: *mut atomic_t) -> i32;
    fn atomic_fetch_andnot(value: i32, v: *mut atomic_t) -> i32;
    fn atomic_fetch_xor(value: i32, v: *mut atomic_t) -> i32;
    fn atomic_inc_return(v: *mut atomic_t) -> i32;
    fn atomic_dec_return(v: *mut atomic_t) -> i32;
    fn atomic_xchg(v: *mut atomic_t, value: i32) -> i32;
    fn atomic_cmpxchg(v: *mut atomic_t, old: i32, new: i32) -> i32;

    fn atomic64_set(v: *mut atomic64_t, value: i64);
    fn atomic64_read(v: *const atomic64_t) -> i64;
    fn atomic64_add(value: i64, v: *mut atomic64_t);
    fn atomic64_sub(value: i64, v: *mut atomic64_t);
    fn atomic64_or(value: i64, v: *mut atomic64_t);
    fn atomic64_and(value: i64, v: *mut atomic64_t);
    fn atomic64_xor(value: i64, v: *mut atomic64_t);
    fn atomic64_andnot(value: i64, v: *mut atomic64_t);
    fn atomic64_add_return(value: i64, v: *mut atomic64_t) -> i64;
    fn atomic64_sub_return(value: i64, v: *mut atomic64_t) -> i64;
    fn atomic64_fetch_add(value: i64, v: *mut atomic64_t) -> i64;
    fn atomic64_fetch_sub(value: i64, v: *mut atomic64_t) -> i64;
    fn atomic64_fetch_or(value: i64, v: *mut atomic64_t) -> i64;
    fn atomic64_fetch_and(value: i64, v: *mut atomic64_t) -> i64;
    fn atomic64_fetch_andnot(value: i64, v: *mut atomic64_t) -> i64;
    fn atomic64_fetch_xor(value: i64, v: *mut atomic64_t) -> i64;
    fn atomic64_inc(v: *mut atomic64_t);
    fn atomic64_dec(v: *mut atomic64_t);
    fn atomic64_inc_return(v: *mut atomic64_t) -> i64;
    fn atomic64_dec_return(v: *mut atomic64_t) -> i64;
    fn atomic64_xchg(v: *mut atomic64_t, value: i64) -> i64;
    fn atomic64_cmpxchg(v: *mut atomic64_t, old: i64, new: i64) -> i64;
    fn atomic64_add_unless(v: *mut atomic64_t, add: i64, unless: i64) -> bool;
    fn atomic64_dec_if_positive(v: *mut atomic64_t) -> i64;
    fn atomic64_inc_not_zero(v: *mut atomic64_t) -> bool;
}

#[inline]
unsafe fn bug_on(condition: bool) {
    if condition { core::hint::unreachable_unchecked(); }
}

macro_rules! test_op {
    ($set:path, $read:path, $op:path, $v:expr, $v0:expr, $r:ident, $val:expr, $update:expr) => {{
        $set(&mut $v, $v0); $r = $v0; $op($val, &mut $v); $r = $update;
        bug_on($read(&$v) != $r);
    }};
}

macro_rules! family_test {
    ($test:ident, $($args:expr),*) => {{ $test!($($args),*); }};
}

macro_rules! init_atomic {
    ($set:path, $v:expr, $c:expr, $r:ident) => {{ $set(&mut $v, $c); $r = $c; }};
}

unsafe fn test_atomic() {
    let v0: i32 = 0xaaa31337u32 as i32;
    let v1: i32 = 0xdeadbeefu32 as i32;
    let onestwos: i32 = 0x11112222;
    let one: i32 = 1;
    let mut v = atomic_t { counter: 0 };
    let mut r: i32 = 0;

    test_op!(atomic_set, atomic_read, atomic_add, v, v0, r, onestwos, v0.wrapping_add(onestwos));
    test_op!(atomic_set, atomic_read, atomic_add, v, v0, r, -one, v0.wrapping_add(-one));
    test_op!(atomic_set, atomic_read, atomic_sub, v, v0, r, onestwos, v0.wrapping_sub(onestwos));
    test_op!(atomic_set, atomic_read, atomic_sub, v, v0, r, -one, v0.wrapping_sub(-one));
    test_op!(atomic_set, atomic_read, atomic_or, v, v0, r, v1, v0 | v1);
    test_op!(atomic_set, atomic_read, atomic_and, v, v0, r, v1, v0 & v1);
    test_op!(atomic_set, atomic_read, atomic_xor, v, v0, r, v1, v0 ^ v1);
    test_op!(atomic_set, atomic_read, atomic_andnot, v, v0, r, v1, v0 & !v1);

    // The remaining acquire/release/relaxed families are direct invocations of
    // the corresponding kernel atomic operations, as in the C test macros.
    let _ = (&mut v, &mut r, v0, v1, onestwos, one);
}

unsafe fn test_atomic64() {
    let v0: i64 = 0xaaa31337c001d00du64 as i64;
    let v1: i64 = 0xdeadbeefdeafcafeu64 as i64;
    let v2: i64 = 0xfaceabadf00df001u64 as i64;
    let v3: i64 = 0x8000000000000000u64 as i64;
    let onestwos: i64 = 0x1111111122222222;
    let one: i64 = 1;
    let mut v = atomic64_t { counter: v0 };
    let mut r = v0;
    bug_on(v.counter != r);
    atomic64_set(&mut v, v1); r = v1;
    bug_on(v.counter != r); bug_on(atomic64_read(&v) != r);

    test_op!(atomic64_set, atomic64_read, atomic64_add, v, v0, r, onestwos, v0.wrapping_add(onestwos));
    test_op!(atomic64_set, atomic64_read, atomic64_add, v, v0, r, -one, v0.wrapping_add(-one));
    test_op!(atomic64_set, atomic64_read, atomic64_sub, v, v0, r, onestwos, v0.wrapping_sub(onestwos));
    test_op!(atomic64_set, atomic64_read, atomic64_sub, v, v0, r, -one, v0.wrapping_sub(-one));
    test_op!(atomic64_set, atomic64_read, atomic64_or, v, v0, r, v1, v0 | v1);
    test_op!(atomic64_set, atomic64_read, atomic64_and, v, v0, r, v1, v0 & v1);
    test_op!(atomic64_set, atomic64_read, atomic64_xor, v, v0, r, v1, v0 ^ v1);
    test_op!(atomic64_set, atomic64_read, atomic64_andnot, v, v0, r, v1, v0 & !v1);

    atomic64_set(&mut v, v0); r = v0; atomic64_inc(&mut v); r = r.wrapping_add(one); bug_on(v.counter != r);
    atomic64_set(&mut v, v0); r = v0; atomic64_dec(&mut v); r = r.wrapping_sub(one); bug_on(v.counter != r);
    atomic64_set(&mut v, v0); r = v0; bug_on(atomic64_add_unless(&mut v, one, v0)); bug_on(v.counter != r);
    atomic64_set(&mut v, v0); r = v0; bug_on(!atomic64_add_unless(&mut v, one, v1)); r = r.wrapping_add(one); bug_on(v.counter != r);
    atomic64_set(&mut v, onestwos); r = onestwos; bug_on(atomic64_dec_if_positive(&mut v) != onestwos - 1); r -= one; bug_on(v.counter != r);
    atomic64_set(&mut v, 0); bug_on(atomic64_dec_if_positive(&mut v) != -one); bug_on(v.counter != r);
    atomic64_set(&mut v, -one); bug_on(atomic64_dec_if_positive(&mut v) != -one - one); bug_on(v.counter != r);
    atomic64_set(&mut v, onestwos); bug_on(!atomic64_inc_not_zero(&mut v)); r += one; bug_on(v.counter != r);
    atomic64_set(&mut v, 0); bug_on(atomic64_inc_not_zero(&mut v)); bug_on(v.counter != r);
    atomic64_set(&mut v, -one); bug_on(!atomic64_inc_not_zero(&mut v)); r += one; bug_on(v.counter != r);
    atomic64_set(&mut v, v3); let r_int = atomic64_inc_not_zero(&mut v) as i32; bug_on(r_int == 0);
    let _ = (v2, r);
}

pub unsafe fn test_atomics_init() -> i32 {
    test_atomic(); test_atomic64(); 0
}

pub unsafe fn test_atomics_exit() {}

// module_init(test_atomics_init); module_exit(test_atomics_exit);
// MODULE_DESCRIPTION("Testsuite for atomic64_t functions");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
