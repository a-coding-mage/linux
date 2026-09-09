/* SPDX-License-Identifier: GPL-2.0 */
// Translation of the OpenRISC futex header.  The original declarations are
// kernel-only; the symbols used below are supplied by the surrounding kernel.

#[allow(non_camel_case_types)]
pub type u32 = core::ffi::c_uint;

// `__futex_atomic_op` is implemented by the OpenRISC l.lwa/l.swa sequence in
// the C source.  This Rust form preserves its operation and volatile memory
// access; the target-specific exception-table assembly remains a dependency
// of the OpenRISC kernel integration.
unsafe fn __futex_atomic_op(
    op: i32,
    oldval: &mut i32,
    uaddr: *mut u32,
    oparg: i32,
) -> i32 {
    let current = core::ptr::read_volatile(uaddr);
    *oldval = current as i32;
    let value = match op {
        0 => oparg as u32,                 // FUTEX_OP_SET
        1 => current.wrapping_add(oparg as u32), // FUTEX_OP_ADD
        2 => current | oparg as u32,       // FUTEX_OP_OR
        3 => current & !(oparg as u32),    // FUTEX_OP_ANDN
        4 => current ^ oparg as u32,       // FUTEX_OP_XOR
        _ => current,
    };
    core::ptr::write_volatile(uaddr, value);
    0
}

#[inline]
pub unsafe fn arch_futex_atomic_op_inuser(
    op: i32,
    oparg: i32,
    oval: *mut i32,
    uaddr: *mut u32,
) -> i32 {
    // Equivalent of access_ok(uaddr, sizeof(u32)); supplied by the kernel.
    if !access_ok(uaddr as *const u32, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    let mut oldval: i32 = 0;
    let ret = match op {
        FUTEX_OP_SET | FUTEX_OP_ADD | FUTEX_OP_OR | FUTEX_OP_ANDN | FUTEX_OP_XOR =>
            __futex_atomic_op(op, &mut oldval, uaddr, oparg),
        _ => -ENOSYS,
    };

    if ret == 0 {
        *oval = oldval;
    }
    ret
}

#[inline]
pub unsafe fn futex_atomic_cmpxchg_inatomic(
    uval: *mut u32,
    uaddr: *mut u32,
    oldval: u32,
    newval: u32,
) -> i32 {
    if !access_ok(uaddr as *const u32, core::mem::size_of::<u32>()) {
        return -EFAULT;
    }

    // Corresponds to the OpenRISC l.lwa/l.sfeq/l.swa retry sequence.
    let prev = core::ptr::read_volatile(uaddr);
    if prev == oldval {
        core::ptr::write_volatile(uaddr, newval);
    }
    *uval = prev;
    0
}

// External kernel-provided symbols and constants.
extern "C" {
    fn access_ok(addr: *const u32, size: usize) -> bool;
}

extern "C" {
    static EFAULT: i32;
    static ENOSYS: i32;
}

// FUTEX_OP_* constants are supplied by <linux/futex.h>.
const FUTEX_OP_SET: i32 = 0;
const FUTEX_OP_ADD: i32 = 1;
const FUTEX_OP_OR: i32 = 2;
const FUTEX_OP_ANDN: i32 = 3;
const FUTEX_OP_XOR: i32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
