/* SPDX-License-Identifier: GPL-2.0 */

// The C header uses SH inline assembly to perform these operations.  The
// atomic_t type is supplied by the including environment.

#[inline(always)]
unsafe fn atomic_read(v: *mut atomic_t) -> i32 {
    core::ptr::read_volatile(v as *mut i32)
}

#[inline(always)]
unsafe fn atomic_write(v: *mut atomic_t, value: i32) {
    core::ptr::write_volatile(v as *mut i32, value);
}

#[inline]
pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) {
    let old = atomic_read(v);
    atomic_write(v, old.wrapping_add(i));
}

#[inline]
pub unsafe fn arch_atomic_add_return(i: i32, v: *mut atomic_t) -> i32 {
    let old = atomic_read(v);
    let value = old.wrapping_add(i);
    atomic_write(v, value);
    value
}

#[inline]
pub unsafe fn arch_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32 {
    let old = atomic_read(v);
    atomic_write(v, old.wrapping_add(i));
    old
}

#[inline]
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) {
    let old = atomic_read(v);
    atomic_write(v, old.wrapping_sub(i));
}

#[inline]
pub unsafe fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 {
    let old = atomic_read(v);
    let value = old.wrapping_sub(i);
    atomic_write(v, value);
    value
}

#[inline]
pub unsafe fn arch_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32 {
    let old = atomic_read(v);
    atomic_write(v, old.wrapping_sub(i));
    old
}

#[inline]
pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) {
    let old = atomic_read(v);
    atomic_write(v, old & i);
}

#[inline]
pub unsafe fn arch_atomic_fetch_and(i: i32, v: *mut atomic_t) -> i32 {
    let old = atomic_read(v);
    atomic_write(v, old & i);
    old
}

#[inline]
pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) {
    let old = atomic_read(v);
    atomic_write(v, old | i);
}

#[inline]
pub unsafe fn arch_atomic_fetch_or(i: i32, v: *mut atomic_t) -> i32 {
    let old = atomic_read(v);
    atomic_write(v, old | i);
    old
}

#[inline]
pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) {
    let old = atomic_read(v);
    atomic_write(v, old ^ i);
}

#[inline]
pub unsafe fn arch_atomic_fetch_xor(i: i32, v: *mut atomic_t) -> i32 {
    let old = atomic_read(v);
    atomic_write(v, old ^ i);
    old
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
