/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 1999, 2016
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>,
 *            Denis Joseph Barrow,
 *            Arnd Bergmann,
 */

// Dependencies supplied by the surrounding kernel translation:
// atomic_t, atomic64_t, __atomic*, __atomic64*, arch_xchg, arch_cmpxchg,
// and arch_try_cmpxchg.

#[inline(always)]
pub unsafe fn arch_atomic_read(v: *const atomic_t) -> i32 { __atomic_read(&(*v).counter) }

#[inline(always)]
pub unsafe fn arch_atomic_set(v: *mut atomic_t, i: i32) { __atomic_set(&mut (*v).counter, i) }

#[inline(always)]
pub unsafe fn arch_atomic_add_return(i: i32, v: *mut atomic_t) -> i32 {
    __atomic_add_barrier(i, &mut (*v).counter).wrapping_add(i)
}

#[inline(always)]
pub unsafe fn arch_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32 {
    __atomic_add_barrier(i, &mut (*v).counter)
}

#[inline(always)]
pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) { __atomic_add(i, &mut (*v).counter) }

#[inline(always)]
pub unsafe fn arch_atomic_inc(v: *mut atomic_t) { __atomic_add_const(1, &mut (*v).counter) }

#[inline(always)]
pub unsafe fn arch_atomic_dec(v: *mut atomic_t) { __atomic_add_const(-1, &mut (*v).counter) }

#[inline(always)]
pub unsafe fn arch_atomic_sub_and_test(i: i32, v: *mut atomic_t) -> bool {
    __atomic_add_and_test_barrier(i.wrapping_neg(), &mut (*v).counter)
}

#[inline(always)]
pub unsafe fn arch_atomic_dec_and_test(v: *mut atomic_t) -> bool {
    __atomic_add_const_and_test_barrier(-1, &mut (*v).counter)
}

#[inline(always)]
pub unsafe fn arch_atomic_inc_and_test(v: *mut atomic_t) -> bool {
    __atomic_add_const_and_test_barrier(1, &mut (*v).counter)
}

#[inline(always)]
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) { arch_atomic_add(i.wrapping_neg(), v) }
#[inline(always)]
pub unsafe fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_add_return(i.wrapping_neg(), v) }
#[inline(always)]
pub unsafe fn arch_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32 { arch_atomic_fetch_add(i.wrapping_neg(), v) }

#[inline(always)]
pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) { __atomic_and(i, &mut (*v).counter) }
#[inline(always)]
pub unsafe fn arch_atomic_fetch_and(i: i32, v: *mut atomic_t) -> i32 { __atomic_and_barrier(i, &mut (*v).counter) }
#[inline(always)]
pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) { __atomic_or(i, &mut (*v).counter) }
#[inline(always)]
pub unsafe fn arch_atomic_fetch_or(i: i32, v: *mut atomic_t) -> i32 { __atomic_or_barrier(i, &mut (*v).counter) }
#[inline(always)]
pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) { __atomic_xor(i, &mut (*v).counter) }
#[inline(always)]
pub unsafe fn arch_atomic_fetch_xor(i: i32, v: *mut atomic_t) -> i32 { __atomic_xor_barrier(i, &mut (*v).counter) }

#[inline(always)]
pub unsafe fn arch_atomic_xchg(v: *mut atomic_t, new: i32) -> i32 { arch_xchg(&mut (*v).counter, new) }
#[inline(always)]
pub unsafe fn arch_atomic_cmpxchg(v: *mut atomic_t, old: i32, new: i32) -> i32 { arch_cmpxchg(&mut (*v).counter, old, new) }
#[inline(always)]
pub unsafe fn arch_atomic_try_cmpxchg(v: *mut atomic_t, old: *mut i32, new: i32) -> bool { arch_try_cmpxchg(&mut (*v).counter, old, new) }

// ATOMIC64_INIT(i) = { (i) }

#[inline(always)]
pub unsafe fn arch_atomic64_read(v: *const atomic64_t) -> i64 { __atomic64_read(&(*v).counter as *const _ as *mut _) }
#[inline(always)]
pub unsafe fn arch_atomic64_set(v: *mut atomic64_t, i: i64) { __atomic64_set(&mut (*v).counter as *mut _ as *mut _, i) }
#[inline(always)]
pub unsafe fn arch_atomic64_add_return(i: i64, v: *mut atomic64_t) -> i64 { __atomic64_add_barrier(i, &mut (*v).counter as *mut _ as *mut _).wrapping_add(i) }
#[inline(always)]
pub unsafe fn arch_atomic64_fetch_add(i: i64, v: *mut atomic64_t) -> i64 { __atomic64_add_barrier(i, &mut (*v).counter as *mut _ as *mut _) }
#[inline(always)]
pub unsafe fn arch_atomic64_add(i: i64, v: *mut atomic64_t) { __atomic64_add(i, &mut (*v).counter as *mut _ as *mut _) }
#[inline(always)]
pub unsafe fn arch_atomic64_inc(v: *mut atomic64_t) { __atomic64_add_const(1, &mut (*v).counter as *mut _ as *mut _) }
#[inline(always)]
pub unsafe fn arch_atomic64_dec(v: *mut atomic64_t) { __atomic64_add_const(-1, &mut (*v).counter as *mut _ as *mut _) }
#[inline(always)]
pub unsafe fn arch_atomic64_sub_and_test(i: i64, v: *mut atomic64_t) -> bool { __atomic64_add_and_test_barrier(i.wrapping_neg(), &mut (*v).counter as *mut _ as *mut _) }
#[inline(always)]
pub unsafe fn arch_atomic64_dec_and_test(v: *mut atomic64_t) -> bool { __atomic64_add_const_and_test_barrier(-1, &mut (*v).counter as *mut _ as *mut _) }
#[inline(always)]
pub unsafe fn arch_atomic64_inc_and_test(v: *mut atomic64_t) -> bool { __atomic64_add_const_and_test_barrier(1, &mut (*v).counter as *mut _ as *mut _) }
#[inline(always)]
pub unsafe fn arch_atomic64_xchg(v: *mut atomic64_t, new: i64) -> i64 { arch_xchg(&mut (*v).counter, new) }
#[inline(always)]
pub unsafe fn arch_atomic64_cmpxchg(v: *mut atomic64_t, old: i64, new: i64) -> i64 { arch_cmpxchg(&mut (*v).counter, old, new) }
#[inline(always)]
pub unsafe fn arch_atomic64_try_cmpxchg(v: *mut atomic64_t, old: *mut i64, new: i64) -> bool { arch_try_cmpxchg(&mut (*v).counter, old, new) }

#[inline(always)]
pub unsafe fn arch_atomic64_and(i: i64, v: *mut atomic64_t) { __atomic64_and(i, &mut (*v).counter as *mut _ as *mut _) }
#[inline(always)]
pub unsafe fn arch_atomic64_fetch_and(i: i64, v: *mut atomic64_t) -> i64 { __atomic64_and_barrier(i, &mut (*v).counter as *mut _ as *mut _) }
#[inline(always)]
pub unsafe fn arch_atomic64_or(i: i64, v: *mut atomic64_t) { __atomic64_or(i, &mut (*v).counter as *mut _ as *mut _) }
#[inline(always)]
pub unsafe fn arch_atomic64_fetch_or(i: i64, v: *mut atomic64_t) -> i64 { __atomic64_or_barrier(i, &mut (*v).counter as *mut _ as *mut _) }
#[inline(always)]
pub unsafe fn arch_atomic64_xor(i: i64, v: *mut atomic64_t) { __atomic64_xor(i, &mut (*v).counter as *mut _ as *mut _) }
#[inline(always)]
pub unsafe fn arch_atomic64_fetch_xor(i: i64, v: *mut atomic64_t) -> i64 { __atomic64_xor_barrier(i, &mut (*v).counter as *mut _ as *mut _) }

#[inline(always)]
pub unsafe fn arch_atomic64_sub_return(i: i64, v: *mut atomic64_t) -> i64 { arch_atomic64_add_return(i.wrapping_neg(), v) }
#[inline(always)]
pub unsafe fn arch_atomic64_fetch_sub(i: i64, v: *mut atomic64_t) -> i64 { arch_atomic64_fetch_add(i.wrapping_neg(), v) }
#[inline(always)]
pub unsafe fn arch_atomic64_sub(i: i64, v: *mut atomic64_t) { arch_atomic64_add(i.wrapping_neg(), v) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
