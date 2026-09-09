/* SPDX-License-Identifier: GPL-2.0 */

/* Atomic operations that C can't guarantee us. Useful for resource counting etc. */

// Supplied by the translated Linux type and atomic-operation dependencies.
#[repr(C)]
pub struct atomic_t {
    pub counter: core::cell::UnsafeCell<i32>,
}

#[inline(always)]
pub unsafe fn arch_atomic_read(v: *const atomic_t) -> i32 {
    // Deliberately equivalent to the source's __READ_ONCE((v)->counter).
    core::ptr::read_volatile((*v).counter.get())
}

#[inline(always)]
pub unsafe fn arch_atomic_set(v: *mut atomic_t, i: i32) {
    core::ptr::write_volatile((*v).counter.get(), i);
}

#[inline(always)]
pub unsafe fn arch_atomic_add(i: i32, v: *mut atomic_t) {
    core::arch::asm!("lock add dword ptr [{counter}], {value}",
        counter = in(reg) (*v).counter.get(), value = in(reg) i,
        options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic_sub(i: i32, v: *mut atomic_t) {
    core::arch::asm!("lock sub dword ptr [{counter}], {value}",
        counter = in(reg) (*v).counter.get(), value = in(reg) i,
        options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic_sub_and_test(i: i32, v: *mut atomic_t) -> bool {
    let result: u8;
    core::arch::asm!("lock sub dword ptr [{counter}], {value}",
        "sete {result}", counter = in(reg) (*v).counter.get(), value = in(reg) i,
        result = out(reg_byte) result, options(nostack));
    result != 0
}

#[inline(always)]
pub unsafe fn arch_atomic_inc(v: *mut atomic_t) {
    core::arch::asm!("lock inc dword ptr [{counter}]", counter = in(reg) (*v).counter.get(), options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic_dec(v: *mut atomic_t) {
    core::arch::asm!("lock dec dword ptr [{counter}]", counter = in(reg) (*v).counter.get(), options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic_dec_and_test(v: *mut atomic_t) -> bool {
    let result: u8;
    core::arch::asm!("lock dec dword ptr [{counter}]", "sete {result}",
        counter = in(reg) (*v).counter.get(), result = out(reg_byte) result, options(nostack));
    result != 0
}

#[inline(always)]
pub unsafe fn arch_atomic_inc_and_test(v: *mut atomic_t) -> bool {
    let result: u8;
    core::arch::asm!("lock inc dword ptr [{counter}]", "sete {result}",
        counter = in(reg) (*v).counter.get(), result = out(reg_byte) result, options(nostack));
    result != 0
}

#[inline(always)]
pub unsafe fn arch_atomic_add_negative(i: i32, v: *mut atomic_t) -> bool {
    let result: u8;
    core::arch::asm!("lock add dword ptr [{counter}], {value}", "sets {result}",
        counter = in(reg) (*v).counter.get(), value = in(reg) i,
        result = out(reg_byte) result, options(nostack));
    result != 0
}

unsafe extern "C" {
    pub fn xadd(ptr: *mut i32, value: i32) -> i32;
    pub fn arch_cmpxchg(ptr: *mut i32, old: i32, new: i32) -> i32;
    pub fn arch_try_cmpxchg(ptr: *mut i32, old: *mut i32, new: i32) -> bool;
    pub fn arch_xchg(ptr: *mut i32, new: i32) -> i32;
}

#[inline(always)]
pub unsafe fn arch_atomic_add_return(i: i32, v: *mut atomic_t) -> i32 {
    i.wrapping_add(xadd((*v).counter.get(), i))
}

#[inline(always)]
pub unsafe fn arch_atomic_sub_return(i: i32, v: *mut atomic_t) -> i32 {
    arch_atomic_add_return(i.wrapping_neg(), v)
}

#[inline(always)]
pub unsafe fn arch_atomic_fetch_add(i: i32, v: *mut atomic_t) -> i32 {
    xadd((*v).counter.get(), i)
}

#[inline(always)]
pub unsafe fn arch_atomic_fetch_sub(i: i32, v: *mut atomic_t) -> i32 {
    arch_atomic_fetch_add(i.wrapping_neg(), v)
}

#[inline(always)]
pub unsafe fn arch_atomic_cmpxchg(v: *mut atomic_t, old: i32, new: i32) -> i32 {
    arch_cmpxchg((*v).counter.get(), old, new)
}

#[inline(always)]
pub unsafe fn arch_atomic_try_cmpxchg(v: *mut atomic_t, old: *mut i32, new: i32) -> bool {
    arch_try_cmpxchg((*v).counter.get(), old, new)
}

#[inline(always)]
pub unsafe fn arch_atomic_xchg(v: *mut atomic_t, new: i32) -> i32 {
    arch_xchg((*v).counter.get(), new)
}

#[inline(always)]
pub unsafe fn arch_atomic_and(i: i32, v: *mut atomic_t) {
    core::arch::asm!("lock and dword ptr [{counter}], {value}",
        counter = in(reg) (*v).counter.get(), value = in(reg) i, options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic_fetch_and(i: i32, v: *mut atomic_t) -> i32 {
    let mut val = arch_atomic_read(v);
    while !arch_atomic_try_cmpxchg(v, &mut val, val & i) {}
    val
}

#[inline(always)]
pub unsafe fn arch_atomic_or(i: i32, v: *mut atomic_t) {
    core::arch::asm!("lock or dword ptr [{counter}], {value}",
        counter = in(reg) (*v).counter.get(), value = in(reg) i, options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic_fetch_or(i: i32, v: *mut atomic_t) -> i32 {
    let mut val = arch_atomic_read(v);
    while !arch_atomic_try_cmpxchg(v, &mut val, val | i) {}
    val
}

#[inline(always)]
pub unsafe fn arch_atomic_xor(i: i32, v: *mut atomic_t) {
    core::arch::asm!("lock xor dword ptr [{counter}], {value}",
        counter = in(reg) (*v).counter.get(), value = in(reg) i, options(nostack));
}

#[inline(always)]
pub unsafe fn arch_atomic_fetch_xor(i: i32, v: *mut atomic_t) -> i32 {
    let mut val = arch_atomic_read(v);
    while !arch_atomic_try_cmpxchg(v, &mut val, val ^ i) {}
    val
}

// CONFIG_X86_32 selects asm/atomic64_32.h; otherwise asm/atomic64_64.h.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
