/* SPDX-License-Identifier: GPL-2.0 */
/* Translated from arch/x86/include/asm/bitops.h. */

/*
 * Copyright 1992, Linus Torvalds.
 *
 * Note: inlines with more than a single statement should be marked
 * __always_inline to avoid problems with older gcc's inlining heuristics.
 */

/* Depends on: linux/compiler.h, asm/alternative.h, asm/rmwcc.h, asm/barrier.h */

#[cfg(not(CONFIG_64BIT))]
pub const _BITOPS_LONG_SHIFT: u32 = 5;
#[cfg(CONFIG_64BIT)]
pub const _BITOPS_LONG_SHIFT: u32 = 6;

#[allow(non_snake_case)]
#[inline(always)]
pub const fn BIT_64(n: u32) -> u64 {
    1u64 << n
}

/*
 * These have to be done with inline assembly: that way the bit-setting
 * is guaranteed to be atomic. All bit operations return 0 if the bit
 * was cleared before the operation and != 0 if it was not.
 *
 * bit 0 is the LSB of addr; bit 32 is the LSB of (addr+1).
 *
 * The C versions switch to a byte-mask form ("orb"/"andb"/"xorb") when @nr is
 * a compile-time constant; Rust cannot observe that, so every call uses the
 * bit-string form below, which has the same effect on memory.
 */

/* __ASM_SIZE(op): the long-sized instruction. */
#[cfg(CONFIG_64BIT)]
macro_rules! __bitops_asm_size {
    ($op:literal) => {
        concat!($op, "q")
    };
}
#[cfg(not(CONFIG_64BIT))]
macro_rules! __bitops_asm_size {
    ($op:literal) => {
        concat!($op, "l")
    };
}

/* LOCK_PREFIX __ASM_SIZE(op) " %1,%0" on the long at @addr. */
macro_rules! __bitops_rmw {
    ($op:literal, $nr:expr, $addr:expr) => {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), __bitops_asm_size!($op), " {nr}, ({addr})"),
            nr = in(reg) $nr,
            addr = in(reg) $addr,
            options(att_syntax, nostack)
        )
    };
}

/* GEN_BINARY_RMWcc(LOCK_PREFIX __ASM_SIZE(op), *addr, c, "Ir", nr) */
macro_rules! __bitops_rmw_cc {
    ($op:literal, $nr:expr, $addr:expr) => {{
        let oldbit: u8;
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), __bitops_asm_size!($op), " {nr}, ({addr})"),
            "setc {oldbit}",
            nr = in(reg) $nr,
            addr = in(reg) $addr,
            oldbit = out(reg_byte) oldbit,
            options(att_syntax, nostack)
        );
        oldbit != 0
    }};
}

/* __ASM_SIZE(op) " %2,%1" with "=@ccc" (oldbit): the non-atomic form. */
macro_rules! __bitops_rmw_cc_nonatomic {
    ($op:literal, $nr:expr, $addr:expr) => {{
        let oldbit: u8;
        core::arch::asm!(
            concat!(__bitops_asm_size!($op), " {nr}, ({addr})"),
            "setc {oldbit}",
            nr = in(reg) $nr,
            addr = in(reg) $addr,
            oldbit = out(reg_byte) oldbit,
            options(att_syntax, nostack)
        );
        oldbit != 0
    }};
}

#[inline(always)]
pub unsafe fn arch_set_bit(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) {
    unsafe { __bitops_rmw!("bts", nr, addr) }
}

#[inline(always)]
pub unsafe fn arch___set_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) {
    unsafe {
        core::arch::asm!(
            concat!(__bitops_asm_size!("bts"), " {nr}, ({addr})"),
            nr = in(reg) nr,
            addr = in(reg) addr,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_clear_bit(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) {
    unsafe { __bitops_rmw!("btr", nr, addr) }
}

#[inline(always)]
pub unsafe fn arch_clear_bit_unlock(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) {
    barrier();
    unsafe { arch_clear_bit(nr, addr) };
}

#[inline(always)]
pub unsafe fn arch___clear_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) {
    unsafe {
        core::arch::asm!(
            concat!(__bitops_asm_size!("btr"), " {nr}, ({addr})"),
            nr = in(reg) nr,
            addr = in(reg) addr,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_xor_unlock_is_negative_byte(
    mask: kernel::ffi::c_ulong,
    addr: *mut kernel::ffi::c_ulong,
) -> bool {
    let negative: u8;
    unsafe {
        core::arch::asm!(
            concat!(LOCK_PREFIX!(), "xorb {mask}, ({addr})"),
            "sets {negative}",
            mask = in(reg_byte) mask as u8,
            addr = in(reg) addr,
            negative = out(reg_byte) negative,
            options(att_syntax, nostack)
        )
    }
    negative != 0
}

#[inline(always)]
pub unsafe fn arch___clear_bit_unlock(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) {
    unsafe { arch___clear_bit(nr as kernel::ffi::c_ulong, addr) }
}

#[inline(always)]
pub unsafe fn arch___change_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) {
    unsafe {
        core::arch::asm!(
            concat!(__bitops_asm_size!("btc"), " {nr}, ({addr})"),
            nr = in(reg) nr,
            addr = in(reg) addr,
            options(att_syntax, nostack)
        )
    }
}

#[inline(always)]
pub unsafe fn arch_change_bit(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) {
    unsafe { __bitops_rmw!("btc", nr, addr) }
}

#[inline(always)]
pub unsafe fn arch_test_and_set_bit(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) -> bool {
    unsafe { __bitops_rmw_cc!("bts", nr, addr) }
}

#[inline(always)]
pub unsafe fn arch_test_and_set_bit_lock(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) -> bool {
    unsafe { arch_test_and_set_bit(nr, addr) }
}

#[inline(always)]
pub unsafe fn arch___test_and_set_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) -> bool {
    unsafe { __bitops_rmw_cc_nonatomic!("bts", nr, addr) }
}

#[inline(always)]
pub unsafe fn arch_test_and_clear_bit(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) -> bool {
    unsafe { __bitops_rmw_cc!("btr", nr, addr) }
}

/*
 * Note: the operation is performed atomically with respect to
 * the local CPU, but not other CPUs. Portable code should not
 * rely on this behaviour.
 * KVM relies on this behaviour on x86 for modifying memory that is also
 * accessed from a hypervisor on the same CPU if running in a VM: don't change
 * this without also updating arch/x86/kernel/kvm.c
 */
#[inline(always)]
pub unsafe fn arch___test_and_clear_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) -> bool {
    unsafe { __bitops_rmw_cc_nonatomic!("btr", nr, addr) }
}

#[inline(always)]
pub unsafe fn arch___test_and_change_bit(nr: kernel::ffi::c_ulong, addr: *mut kernel::ffi::c_ulong) -> bool {
    unsafe { __bitops_rmw_cc_nonatomic!("btc", nr, addr) }
}

#[inline(always)]
pub unsafe fn arch_test_and_change_bit(nr: kernel::ffi::c_long, addr: *mut kernel::ffi::c_ulong) -> bool {
    unsafe { __bitops_rmw_cc!("btc", nr, addr) }
}

#[inline(always)]
pub unsafe fn constant_test_bit(nr: kernel::ffi::c_long, addr: *const kernel::ffi::c_ulong) -> bool {
    let word = unsafe { core::ptr::read_volatile(addr.offset(nr >> _BITOPS_LONG_SHIFT)) };
    ((1 << (nr & (BITS_PER_LONG as kernel::ffi::c_long - 1))) & word) != 0
}

#[inline(always)]
pub unsafe fn constant_test_bit_acquire(nr: kernel::ffi::c_long, addr: *const kernel::ffi::c_ulong) -> bool {
    let oldbit: u8;
    unsafe {
        core::arch::asm!(
            "testb {mask}, ({byte})",
            "setnz {oldbit}",
            mask = in(reg_byte) (1u8 << (nr & 7)),
            byte = in(reg) addr.cast::<u8>().offset(nr >> 3),
            oldbit = out(reg_byte) oldbit,
            options(att_syntax, nostack)
        )
    }
    oldbit != 0
}

#[inline(always)]
pub unsafe fn variable_test_bit(nr: kernel::ffi::c_long, addr: *const kernel::ffi::c_ulong) -> bool {
    let oldbit: u8;
    unsafe {
        core::arch::asm!(
            concat!(__bitops_asm_size!("bt"), " {nr}, ({addr})"),
            "setc {oldbit}",
            nr = in(reg) nr,
            addr = in(reg) addr,
            oldbit = out(reg_byte) oldbit,
            options(att_syntax, nostack)
        )
    }
    oldbit != 0
}

#[inline(always)]
pub unsafe fn arch_test_bit(nr: kernel::ffi::c_ulong, addr: *const kernel::ffi::c_ulong) -> bool {
    unsafe { variable_test_bit(nr as kernel::ffi::c_long, addr) }
}

#[inline(always)]
pub unsafe fn arch_test_bit_acquire(nr: kernel::ffi::c_ulong, addr: *const kernel::ffi::c_ulong) -> bool {
    unsafe { variable_test_bit(nr as kernel::ffi::c_long, addr) }
}

#[inline(always)]
pub fn variable__ffs(word: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong {
    let ret: kernel::ffi::c_ulong;
    unsafe {
        core::arch::asm!(
            "tzcnt {word}, {ret}",
            word = in(reg) word,
            ret = lateout(reg) ret,
            options(att_syntax, pure, nomem, nostack)
        )
    }
    ret
}

/*
 * The functions below are the __builtin_constant_p() branches of the C
 * macros.  They compute the same value as the asm branches and stay usable in
 * const contexts.
 */

/**
 * __ffs - find first set bit in word
 * @word: The word to search
 *
 * Undefined if no bit exists, so code should check against 0 first.
 */
#[inline(always)]
pub const fn __ffs(word: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong {
    word.trailing_zeros() as kernel::ffi::c_ulong
}

#[inline(always)]
pub fn variable_ffz(word: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong {
    variable__ffs(!word)
}

/**
 * ffz - find first zero bit in word
 * @word: The word to search
 *
 * Undefined if no zero exists, so code should check against ~0UL first.
 */
#[inline(always)]
pub const fn ffz(word: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong {
    (!word).trailing_zeros() as kernel::ffi::c_ulong
}

/*
 * __fls: find last set bit in word
 * @word: The word to search
 *
 * Undefined if no set bit exists, so code should check against 0 first.
 */
#[inline(always)]
pub const fn __fls(word: kernel::ffi::c_ulong) -> kernel::ffi::c_ulong {
    (BITS_PER_LONG as u32 - 1).wrapping_sub(word.leading_zeros()) as kernel::ffi::c_ulong
}

/* __KERNEL__ */

#[inline(always)]
pub fn variable_ffs(x: kernel::ffi::c_int) -> kernel::ffi::c_int {
    let mut r: kernel::ffi::c_int = -1;

    /*
     * AMD64 says BSFL won't clobber the dest reg if x==0; Intel64 says the
     * dest reg is undefined if x==0, but their CPU architect says its
     * value is written to set it to the same as before, except that the
     * top 32 bits will be cleared.
     *
     * We cannot do this on 32 bits because at the very least some
     * 486 CPUs did not behave this way.
     */
    unsafe {
        core::arch::asm!(
            "bsfl {x:e}, {r:e}",
            x = in(reg) x,
            r = inout(reg) r,
            options(att_syntax, pure, nomem, nostack)
        )
    }
    r + 1
}

/**
 * ffs - find first set bit in word
 * @x: the word to search
 *
 * This is defined the same way as the libc and compiler builtin ffs
 * routines, therefore differs in spirit from the other bitops.
 *
 * ffs(value) returns 0 if value is 0 or the position of the first
 * set bit if value is nonzero. The first (least significant) bit
 * is at position 1.
 */
#[inline(always)]
pub const fn ffs(x: kernel::ffi::c_int) -> kernel::ffi::c_int {
    if x == 0 { 0 } else { x.trailing_zeros() as kernel::ffi::c_int + 1 }
}

/**
 * fls - find last set bit in word
 * @x: the word to search
 *
 * This is defined in a similar way as the libc and compiler builtin
 * ffs, but returns the position of the most significant set bit.
 *
 * fls(value) returns 0 if value is 0 or the position of the last
 * set bit if value is nonzero. The last (most significant) bit is
 * at position 32.
 */
#[inline(always)]
pub const fn fls(x: kernel::ffi::c_uint) -> kernel::ffi::c_int {
    if x != 0 { 32 - x.leading_zeros() as kernel::ffi::c_int } else { 0 }
}

/**
 * fls64 - find last set bit in a 64-bit word
 * @x: the word to search
 *
 * This is defined in a similar way as the libc and compiler builtin
 * ffsll, but returns the position of the most significant set bit.
 *
 * fls64(value) returns 0 if value is 0 or the position of the last
 * set bit if value is nonzero. The last (most significant) bit is
 * at position 64.
 */
#[cfg(CONFIG_X86_64)]
#[inline(always)]
pub const fn fls64(x: __u64) -> kernel::ffi::c_int {
    if x != 0 { 64 - x.leading_zeros() as kernel::ffi::c_int } else { 0 }
}
/* !CONFIG_X86_64: Depends on: asm-generic/bitops/fls64.h */

/* Depends on: asm-generic/bitops/sched.h, asm/arch_hweight.h,
 * asm-generic/bitops/const_hweight.h, asm-generic/bitops/instrumented-atomic.h,
 * asm-generic/bitops/instrumented-non-atomic.h,
 * asm-generic/bitops/instrumented-lock.h, asm-generic/bitops/le.h,
 * asm-generic/bitops/ext2-atomic-setbit.h */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
