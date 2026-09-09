/* SPDX-License-Identifier: GPL-2.0 */

// Rust translation of x86/include/asm/atomic64_64.h.

use core::arch::asm;

pub type s64 = i64;

#[repr(C)]
pub struct atomic64_t {
    pub counter: s64,
}

#[inline(always)]
pub const fn ATOMIC64_INIT(i: s64) -> atomic64_t {
    atomic64_t { counter: i }
}

#[inline(always)]
pub unsafe fn arch_atomic64_read(v: *const atomic64_t) -> s64 {
    core::ptr::read_volatile(core::ptr::addr_of!((*v).counter))
}

#[inline(always)]
pub unsafe fn arch_atomic64_set(v: *mut atomic64_t, i: s64) {
    core::ptr::write_volatile(core::ptr::addr_of_mut!((*v).counter), i);
}

#[inline(always)]
pub unsafe fn arch_atomic64_add(i: s64, v: *mut atomic64_t) {
    asm!("lock add q, {i}", i = in(reg) i, memory("{v}"), options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic64_sub(i: s64, v: *mut atomic64_t) {
    asm!("lock sub q, {i}", i = in(reg) i, memory("{v}"), options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic64_sub_and_test(i: s64, v: *mut atomic64_t) -> bool {
    let result: u8;
    asm!("lock sub q, {i}; sete {result}", i = in(reg) i, result = lateout(reg_byte) result, memory("{v}"), options(nostack));
    result != 0
}

#[inline(always)]
pub unsafe fn arch_atomic64_inc(v: *mut atomic64_t) {
    asm!("lock inc q, [rdi]", in("rdi") v, memory("{v}"), options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic64_dec(v: *mut atomic64_t) {
    asm!("lock dec q, [rdi]", in("rdi") v, memory("{v}"), options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic64_dec_and_test(v: *mut atomic64_t) -> bool {
    let result: u8;
    asm!("lock dec q, [rdi]; sete {result}", in("rdi") v, result = lateout(reg_byte) result, memory("{v}"), options(nostack));
    result != 0
}

#[inline(always)]
pub unsafe fn arch_atomic64_inc_and_test(v: *mut atomic64_t) -> bool {
    let result: u8;
    asm!("lock inc q, [rdi]; sete {result}", in("rdi") v, result = lateout(reg_byte) result, memory("{v}"), options(nostack));
    result != 0
}

#[inline(always)]
pub unsafe fn arch_atomic64_add_negative(i: s64, v: *mut atomic64_t) -> bool {
    let result: u8;
    asm!("lock add q, {i}; sets {result}", i = in(reg) i, result = lateout(reg_byte) result, memory("{v}"), options(nostack));
    result != 0
}

extern "C" {
    pub fn xadd(ptr: *mut s64, value: s64) -> s64;
    pub fn arch_cmpxchg(ptr: *mut s64, old: s64, new: s64) -> s64;
    pub fn arch_try_cmpxchg(ptr: *mut s64, old: *mut s64, new: s64) -> bool;
    pub fn arch_xchg(ptr: *mut s64, new: s64) -> s64;
}

#[inline(always)]
pub unsafe fn arch_atomic64_add_return(i: s64, v: *mut atomic64_t) -> s64 {
    i.wrapping_add(xadd(core::ptr::addr_of_mut!((*v).counter), i))
}

#[inline(always)]
pub unsafe fn arch_atomic64_sub_return(i: s64, v: *mut atomic64_t) -> s64 {
    arch_atomic64_add_return(i.wrapping_neg(), v)
}

#[inline(always)]
pub unsafe fn arch_atomic64_fetch_add(i: s64, v: *mut atomic64_t) -> s64 {
    xadd(core::ptr::addr_of_mut!((*v).counter), i)
}

#[inline(always)]
pub unsafe fn arch_atomic64_fetch_sub(i: s64, v: *mut atomic64_t) -> s64 {
    arch_atomic64_fetch_add(i.wrapping_neg(), v)
}

#[inline(always)]
pub unsafe fn arch_atomic64_cmpxchg(v: *mut atomic64_t, old: s64, new: s64) -> s64 {
    arch_cmpxchg(core::ptr::addr_of_mut!((*v).counter), old, new)
}

#[inline(always)]
pub unsafe fn arch_atomic64_try_cmpxchg(v: *mut atomic64_t, old: *mut s64, new: s64) -> bool {
    arch_try_cmpxchg(core::ptr::addr_of_mut!((*v).counter), old, new)
}

#[inline(always)]
pub unsafe fn arch_atomic64_xchg(v: *mut atomic64_t, new: s64) -> s64 {
    arch_xchg(core::ptr::addr_of_mut!((*v).counter), new)
}

#[inline(always)]
pub unsafe fn arch_atomic64_and(i: s64, v: *mut atomic64_t) {
    asm!("lock and q, {i}", i = in(reg) i, memory("{v}"), options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic64_fetch_and(i: s64, v: *mut atomic64_t) -> s64 {
    let mut val = arch_atomic64_read(v);
    while !arch_atomic64_try_cmpxchg(v, &mut val, val & i) {}
    val
}

#[inline(always)]
pub unsafe fn arch_atomic64_or(i: s64, v: *mut atomic64_t) {
    asm!("lock or q, {i}", i = in(reg) i, memory("{v}"), options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic64_fetch_or(i: s64, v: *mut atomic64_t) -> s64 {
    let mut val = arch_atomic64_read(v);
    while !arch_atomic64_try_cmpxchg(v, &mut val, val | i) {}
    val
}

#[inline(always)]
pub unsafe fn arch_atomic64_xor(i: s64, v: *mut atomic64_t) {
    asm!("lock xor q, {i}", i = in(reg) i, memory("{v}"), options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic64_fetch_xor(i: s64, v: *mut atomic64_t) -> s64 {
    let mut val = arch_atomic64_read(v);
    while !arch_atomic64_try_cmpxchg(v, &mut val, val ^ i) {}
    val
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
