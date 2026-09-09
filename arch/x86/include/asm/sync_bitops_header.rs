/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Copyright 1992, Linus Torvalds.
 */

/*
 * These have to be done with inline assembly: that way the bit-setting
 * is guaranteed to be atomic. All bit operations return 0 if the bit
 * was cleared before the operation and != 0 if it was not.
 *
 * bit 0 is the LSB of addr; bit 32 is the LSB of (addr+1).
 */

/// sync_set_bit - Atomically set a bit in memory
/// @nr: the bit to set
/// @addr: the address to start counting from
///
/// This function is atomic and may not be reordered. See __set_bit()
/// if you do not require the atomic guarantees.
///
/// Note that @nr may be almost arbitrarily large; this function is not
/// restricted to acting on a single-word quantity.
#[inline]
pub unsafe fn sync_set_bit(nr: libc::c_long, addr: *mut libc::c_ulong) {
    core::arch::asm!(
        "lock bts {nr}, [{addr}]",
        nr = in(reg) nr,
        addr = in(reg) addr,
        options(nostack, preserves_flags)
    );
}

/// sync_clear_bit - Clears a bit in memory
/// @nr: Bit to clear
/// @addr: Address to start counting from
///
/// sync_clear_bit() is atomic and may not be reordered. However, it does
/// not contain a memory barrier, so if it is used for locking purposes,
/// you should call smp_mb__before_atomic() and/or smp_mb__after_atomic()
/// in order to ensure changes are visible on other processors.
#[inline]
pub unsafe fn sync_clear_bit(nr: libc::c_long, addr: *mut libc::c_ulong) {
    core::arch::asm!(
        "lock btr {nr}, [{addr}]",
        nr = in(reg) nr,
        addr = in(reg) addr,
        options(nostack, preserves_flags)
    );
}

/// sync_change_bit - Toggle a bit in memory
/// @nr: Bit to change
/// @addr: Address to start counting from
///
/// sync_change_bit() is atomic and may not be reordered.
/// Note that @nr may be almost arbitrarily large; this function is not
/// restricted to acting on a single-word quantity.
#[inline]
pub unsafe fn sync_change_bit(nr: libc::c_long, addr: *mut libc::c_ulong) {
    core::arch::asm!(
        "lock btc {nr}, [{addr}]",
        nr = in(reg) nr,
        addr = in(reg) addr,
        options(nostack, preserves_flags)
    );
}

/// sync_test_and_set_bit - Set a bit and return its old value
/// @nr: Bit to set
/// @addr: Address to count from
///
/// This operation is atomic and cannot be reordered.
/// It also implies a memory barrier.
#[inline]
pub unsafe fn sync_test_and_set_bit(nr: libc::c_long, addr: *mut libc::c_ulong) -> bool {
    let old: u8;
    core::arch::asm!(
        "lock bts {nr}, [{addr}]",
        "setc {old}",
        nr = in(reg) nr,
        addr = in(reg) addr,
        old = out(reg_byte) old,
        options(nostack)
    );
    old != 0
}

/// sync_test_and_clear_bit - Clear a bit and return its old value
#[inline]
pub unsafe fn sync_test_and_clear_bit(nr: libc::c_long, addr: *mut libc::c_ulong) -> libc::c_int {
    let old: u8;
    core::arch::asm!(
        "lock btr {nr}, [{addr}]",
        "setc {old}",
        nr = in(reg) nr,
        addr = in(reg) addr,
        old = out(reg_byte) old,
        options(nostack)
    );
    old as libc::c_int
}

/// sync_test_and_change_bit - Change a bit and return its old value
#[inline]
pub unsafe fn sync_test_and_change_bit(nr: libc::c_long, addr: *mut libc::c_ulong) -> libc::c_int {
    let old: u8;
    core::arch::asm!(
        "lock btc {nr}, [{addr}]",
        "setc {old}",
        nr = in(reg) nr,
        addr = in(reg) addr,
        old = out(reg_byte) old,
        options(nostack)
    );
    old as libc::c_int
}

/* Equivalent of: #define sync_test_bit(nr, addr) test_bit(nr, addr) */
#[macro_export]
macro_rules! sync_test_bit {
    ($nr:expr, $addr:expr) => {
        test_bit($nr, $addr)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
