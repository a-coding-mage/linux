/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2012 ARM Ltd. */

// Translated from arm64/include/asm/futex.h.  The Linux kernel types,
// constants, accessors, and LSUI/exception-table primitives are supplied by
// the including translation unit.

pub const FUTEX_MAX_LOOPS: u32 = 128;

// LLSC_FUTEX_ATOMIC_OP expands to the five helpers below.  Their bodies are
// ARM inline assembly using ldxr/stlxr, exception tables, and privileged
// uaccess; those kernel-provided operations have no file-local Rust mapping.
pub unsafe fn __llsc_futex_atomic_add(_oparg: i32, _uaddr: *mut u32, _oval: *mut i32) -> i32 { unimplemented!("ARM64 LLSC inline assembly") }
pub unsafe fn __llsc_futex_atomic_or(_oparg: i32, _uaddr: *mut u32, _oval: *mut i32) -> i32 { unimplemented!("ARM64 LLSC inline assembly") }
pub unsafe fn __llsc_futex_atomic_and(_oparg: i32, _uaddr: *mut u32, _oval: *mut i32) -> i32 { unimplemented!("ARM64 LLSC inline assembly") }
pub unsafe fn __llsc_futex_atomic_eor(_oparg: i32, _uaddr: *mut u32, _oval: *mut i32) -> i32 { unimplemented!("ARM64 LLSC inline assembly") }
pub unsafe fn __llsc_futex_atomic_set(_oparg: i32, _uaddr: *mut u32, _oval: *mut i32) -> i32 { unimplemented!("ARM64 LLSC inline assembly") }

pub unsafe fn __llsc_futex_cmpxchg(_uaddr: *mut u32, _oldval: u32, _newval: u32, _oval: *mut u32) -> i32 {
    unimplemented!("ARM64 LLSC inline assembly")
}

// CONFIG_ARM64_LSUI conditionally provides these helpers.  The condition is
// retained here because its value is supplied by the kernel build.
#[cfg(CONFIG_ARM64_LSUI)]
pub unsafe fn __lsui_futex_atomic_add(_oparg: i32, _uaddr: *mut u32, _oval: *mut i32) -> i32 { unimplemented!("ARM64 LSUI inline assembly") }
#[cfg(CONFIG_ARM64_LSUI)]
pub unsafe fn __lsui_futex_atomic_or(_oparg: i32, _uaddr: *mut u32, _oval: *mut i32) -> i32 { unimplemented!("ARM64 LSUI inline assembly") }
#[cfg(CONFIG_ARM64_LSUI)]
pub unsafe fn __lsui_futex_atomic_andnot(_oparg: i32, _uaddr: *mut u32, _oval: *mut i32) -> i32 { unimplemented!("ARM64 LSUI inline assembly") }
#[cfg(CONFIG_ARM64_LSUI)]
pub unsafe fn __lsui_futex_atomic_set(_oparg: i32, _uaddr: *mut u32, _oval: *mut i32) -> i32 { unimplemented!("ARM64 LSUI inline assembly") }

#[cfg(CONFIG_ARM64_LSUI)]
pub unsafe fn __lsui_cmpxchg64(_uaddr: *mut u64, _oldval: *mut u64, _newval: u64) -> i32 { unimplemented!("ARM64 LSUI inline assembly") }

#[cfg(CONFIG_ARM64_LSUI)]
pub unsafe fn __lsui_cmpxchg32(uaddr: *mut u32, oldval: *mut u32, newval: u32) -> i32 {
    let uaddr64 = (uaddr as usize & !(core::mem::size_of::<u64>() - 1)) as *mut u64;
    let futex_pos = ((uaddr as usize & (core::mem::size_of::<u64>() - 1)) != 0) as usize;
    let other_pos = 1usize - futex_pos;
    let mut orig64 = [0u32; 2];
    let mut oval64;
    let mut nval64;
    orig64[futex_pos] = *oldval;
    if !crate::get_user(&mut orig64[other_pos], uaddr64.cast::<u32>().add(other_pos)) {
        return -14;
    }
    oval64 = orig64;
    nval64 = orig64;
    nval64[futex_pos] = newval;
    if __lsui_cmpxchg64(uaddr64, oval64.as_mut_ptr().cast::<u64>(), nval64.as_ptr().cast::<u64>()) != 0 { return -14; }
    *oldval = oval64[futex_pos];
    if oval64 != orig64 { -11 } else { 0 }
}

#[cfg(CONFIG_ARM64_LSUI)]
pub unsafe fn __lsui_futex_atomic_and(oparg: i32, uaddr: *mut u32, oval: *mut i32) -> i32 {
    __lsui_futex_atomic_andnot(!oparg, uaddr, oval)
}

#[cfg(CONFIG_ARM64_LSUI)]
pub unsafe fn __lsui_futex_atomic_eor(oparg: i32, uaddr: *mut u32, oval: *mut i32) -> i32 {
    let mut oldval = 0u32;
    if !crate::get_user(&mut oldval, uaddr) { return -14; }
    let mut ret = 0;
    for _ in 0..FUTEX_MAX_LOOPS {
        let newval = oldval ^ oparg as u32;
        ret = __lsui_cmpxchg32(uaddr, &mut oldval, newval);
        if ret != -11 { break; }
    }
    *oval = oldval as i32;
    ret
}

#[cfg(CONFIG_ARM64_LSUI)]
pub unsafe fn __lsui_futex_cmpxchg(uaddr: *mut u32, oldval: u32, newval: u32, oval: *mut u32) -> i32 {
    let mut curval = oldval;
    let mut ret = __lsui_cmpxchg32(uaddr, &mut curval, newval);
    if ret == -11 && curval == oldval { ret = 0; }
    *oval = curval;
    ret
}

// FUTEX_ATOMIC_OP and __lsui_llsc_body are kernel dispatch abstractions.
pub unsafe fn __futex_atomic_add(oparg: i32, uaddr: *mut u32, oval: *mut i32) -> i32 { crate::__lsui_llsc_body_futex_atomic_add(oparg, uaddr, oval) }
pub unsafe fn __futex_atomic_or(oparg: i32, uaddr: *mut u32, oval: *mut i32) -> i32 { crate::__lsui_llsc_body_futex_atomic_or(oparg, uaddr, oval) }
pub unsafe fn __futex_atomic_and(oparg: i32, uaddr: *mut u32, oval: *mut i32) -> i32 { crate::__lsui_llsc_body_futex_atomic_and(oparg, uaddr, oval) }
pub unsafe fn __futex_atomic_eor(oparg: i32, uaddr: *mut u32, oval: *mut i32) -> i32 { crate::__lsui_llsc_body_futex_atomic_eor(oparg, uaddr, oval) }
pub unsafe fn __futex_atomic_set(oparg: i32, uaddr: *mut u32, oval: *mut i32) -> i32 { crate::__lsui_llsc_body_futex_atomic_set(oparg, uaddr, oval) }

pub unsafe fn __futex_cmpxchg(uaddr: *mut u32, oldval: u32, newval: u32, oval: *mut u32) -> i32 {
    crate::__lsui_llsc_body_futex_cmpxchg(uaddr, oldval, newval, oval)
}

pub unsafe fn arch_futex_atomic_op_inuser(op: i32, oparg: i32, oval: *mut i32, uaddr: *mut u32) -> i32 {
    if !crate::access_ok(uaddr.cast(), core::mem::size_of::<u32>()) { return -14; }
    let uaddr = crate::__uaccess_mask_ptr(uaddr);
    match op {
        crate::FUTEX_OP_SET => __futex_atomic_set(oparg, uaddr, oval),
        crate::FUTEX_OP_ADD => __futex_atomic_add(oparg, uaddr, oval),
        crate::FUTEX_OP_OR => __futex_atomic_or(oparg, uaddr, oval),
        crate::FUTEX_OP_ANDN => __futex_atomic_and(!oparg, uaddr, oval),
        crate::FUTEX_OP_XOR => __futex_atomic_eor(oparg, uaddr, oval),
        _ => -38,
    }
}

pub unsafe fn futex_atomic_cmpxchg_inatomic(uval: *mut u32, uaddr: *mut u32, oldval: u32, newval: u32) -> i32 {
    if !crate::access_ok(uaddr.cast(), core::mem::size_of::<u32>()) { return -14; }
    __futex_cmpxchg(crate::__uaccess_mask_ptr(uaddr), oldval, newval, uval)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
