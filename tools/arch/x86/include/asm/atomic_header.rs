/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of arch/x86/include/asm/atomic.h.
 *
 * Original C dependencies:
 * - <linux/compiler.h>
 * - <linux/types.h>
 * - "rmwcc.h"
 * - <asm/asm.h>
 * - <asm/cmpxchg.h>
 */

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use core::arch::asm;

pub const LOCK_PREFIX: &str = "\n\tlock; ";

macro_rules! ATOMIC_INIT {
    ($i:expr) => {
        atomic_t { counter: $i }
    };
}

/*
 * Atomic operations that C can't guarantee us.  Useful for
 * resource counting etc..
 */

/**
 * atomic_read - read atomic variable
 * @v: pointer of type atomic_t
 *
 * Atomically reads the value of @v.
 */
#[inline]
pub unsafe fn atomic_read(v: *const atomic_t) -> i32 {
    unsafe { core::ptr::read_volatile(core::ptr::addr_of!((*v).counter)) }
}

/**
 * atomic_set - set atomic variable
 * @v: pointer of type atomic_t
 * @i: required value
 *
 * Atomically sets the value of @v to @i.
 */
#[inline]
pub unsafe fn atomic_set(v: *mut atomic_t, i: i32) {
    unsafe {
        (*v).counter = i;
    }
}

/**
 * atomic_inc - increment atomic variable
 * @v: pointer of type atomic_t
 *
 * Atomically increments @v by 1.
 */
#[inline]
pub unsafe fn atomic_inc(v: *mut atomic_t) {
    unsafe {
        asm!(
            "lock; incl dword ptr [{counter}]",
            counter = in(reg) core::ptr::addr_of_mut!((*v).counter),
            options(nostack)
        );
    }
}

/**
 * atomic_dec_and_test - decrement and test
 * @v: pointer of type atomic_t
 *
 * Atomically decrements @v by 1 and
 * returns true if the result is 0, or false for all other
 * cases.
 */
#[inline]
pub unsafe fn atomic_dec_and_test(v: *mut atomic_t) -> i32 {
    let c: u8;

    unsafe {
        asm!(
            "lock; decl dword ptr [{counter}]",
            "sete {c}",
            counter = in(reg) core::ptr::addr_of_mut!((*v).counter),
            c = lateout(reg_byte) c,
            options(nostack)
        );
    }

    c as i32
}

#[inline(always)]
pub unsafe fn atomic_cmpxchg(v: *mut atomic_t, old: i32, r#new: i32) -> i32 {
    unsafe { cmpxchg(core::ptr::addr_of_mut!((*v).counter), old, r#new) }
}

#[inline]
pub unsafe fn test_and_set_bit(nr: isize, addr: *mut usize) -> i32 {
    let c: u8;

    unsafe {
        #[cfg(target_pointer_width = "64")]
        asm!(
            "lock; bts qword ptr [{addr}], {nr}",
            "setc {c}",
            addr = in(reg) addr,
            nr = in(reg) nr,
            c = lateout(reg_byte) c,
            options(nostack)
        );

        #[cfg(target_pointer_width = "32")]
        asm!(
            "lock; bts dword ptr [{addr}], {nr}",
            "setc {c}",
            addr = in(reg) addr,
            nr = in(reg) nr,
            c = lateout(reg_byte) c,
            options(nostack)
        );
    }

    c as i32
}

#[inline]
pub unsafe fn test_and_clear_bit(nr: isize, addr: *mut usize) -> i32 {
    let c: u8;

    unsafe {
        #[cfg(target_pointer_width = "64")]
        asm!(
            "lock; btc qword ptr [{addr}], {nr}",
            "setc {c}",
            addr = in(reg) addr,
            nr = in(reg) nr,
            c = lateout(reg_byte) c,
            options(nostack)
        );

        #[cfg(target_pointer_width = "32")]
        asm!(
            "lock; btc dword ptr [{addr}], {nr}",
            "setc {c}",
            addr = in(reg) addr,
            nr = in(reg) nr,
            c = lateout(reg_byte) c,
            options(nostack)
        );
    }

    c as i32
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
