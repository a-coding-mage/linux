/*
 * Atomic xchg and cmpxchg operations.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive.
 * Copyright (C) 2001 - 2005 Tensilica Inc.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/bits.h, linux/stringify.h, linux/cmpxchg-emu.h, and
// asm-generic/cmpxchg-local.h.

#[inline(always)]
pub unsafe fn __cmpxchg_u32(p: *mut core::ffi::c_int, old: core::ffi::c_int, new: core::ffi::c_int) -> usize {
    let mut old_value = old;
    let _ = (p, new);
    // The following target-specific alternatives preserve the original Xtensa
    // exclusive, S32C1I, and interrupt-masked implementations.
    #[cfg(XCHAL_HAVE_EXCLUSIVE)]
    unsafe {
        let mut tmp: usize;
        let mut result: usize;
        core::arch::asm!(
            "1: l32ex {result}, {addr}",
            "bne {result}, {cmp}, 2f",
            "mov {tmp}, {new}",
            "s32ex {tmp}, {addr}",
            "getex {tmp}",
            "beqz {tmp}, 1b",
            "2:",
            result = lateout(reg) result, tmp = lateout(reg) tmp,
            new = in(reg) new, addr = in(reg) p, cmp = in(reg) old,
            options(nostack)
        );
        return result;
    }
    #[cfg(all(not(XCHAL_HAVE_EXCLUSIVE), XCHAL_HAVE_S32C1I))]
    unsafe {
        core::arch::asm!(
            "wsr {cmp}, scompare1", "s32c1i {new}, [{mem}]",
            cmp = in(reg) old, new = inout(reg) new, mem = in(reg) p,
            options(nostack)
        );
        return new as usize;
    }
    #[cfg(all(not(XCHAL_HAVE_EXCLUSIVE), not(XCHAL_HAVE_S32C1I)))]
    unsafe {
        core::arch::asm!(
            "rsil a14, TOPLEVEL", "l32i {old}, [{mem}]",
            "bne {old}, {cmp}, 1f", "s32i {new}, [{mem}]", "1:",
            "wsr a14, ps", "rsync",
            old = inout(reg) old_value, mem = in(reg) p,
            cmp = in(reg) old, new = in(reg) new, options(nostack)
        );
        return old_value as usize;
    }
}

/* This function does not exist; invalid cmpxchg uses produce a linker error. */
unsafe extern "C" {
    pub fn __cmpxchg_called_with_bad_pointer();
}

#[inline(always)]
pub unsafe fn __cmpxchg(ptr: *mut core::ffi::c_void, old: usize, new: usize, size: i32) -> usize {
    match size {
        1 => unsafe { cmpxchg_emu_u8(ptr, old, new) },
        4 => unsafe { __cmpxchg_u32(ptr.cast(), old as i32, new as i32) },
        _ => { unsafe { __cmpxchg_called_with_bad_pointer() }; old }
    }
}

// C arch_cmpxchg macro; Rust callers provide the pointed-to type explicitly.
#[macro_export]
macro_rules! arch_cmpxchg {
    ($ptr:expr, $o:expr, $n:expr) => {
        $crate::__cmpxchg($ptr as *mut core::ffi::c_void, $o as usize, $n as usize,
                          core::mem::size_of_val(unsafe { &*$ptr }) as i32) as _
    };
}

#[inline(always)]
pub unsafe fn __cmpxchg_local(ptr: *mut core::ffi::c_void, old: usize, new: usize, size: i32) -> usize {
    match size {
        4 => unsafe { __cmpxchg_u32(ptr.cast(), old as i32, new as i32) },
        _ => unsafe { __generic_cmpxchg_local(ptr, old, new, size) },
    }
}

#[macro_export]
macro_rules! arch_cmpxchg_local {
    ($ptr:expr, $o:expr, $n:expr) => {
        __generic_cmpxchg_local($ptr as *mut core::ffi::c_void, $o as usize, $n as usize,
                                 core::mem::size_of_val(unsafe { &*$ptr }) as i32) as _
    };
}
#[macro_export]
macro_rules! arch_cmpxchg64_local { ($ptr:expr, $o:expr, $n:expr) => { __generic_cmpxchg64_local($ptr, $o, $n) }; }
#[macro_export]
macro_rules! arch_cmpxchg64 { ($ptr:expr, $o:expr, $n:expr) => { arch_cmpxchg64_local!($ptr, $o, $n) }; }

#[inline(always)]
pub unsafe fn xchg_u32(m: *mut core::ffi::c_int, val: usize) -> usize {
    let mut result = val;
    let mut tmp: usize;
    let _ = m;
    // Target selection follows XCHAL_HAVE_EXCLUSIVE and XCHAL_HAVE_S32C1I.
    core::arch::asm!(
        "1: l32ex {result}, [{addr}]", "mov {tmp}, {val}",
        "s32ex {tmp}, [{addr}]", "getex {tmp}", "beqz {tmp}, 1b",
        result = lateout(reg) result, tmp = lateout(reg) tmp,
        addr = in(reg) m, val = in(reg) val, options(nostack)
    );
    result
}

#[macro_export]
macro_rules! arch_xchg {
    ($ptr:expr, $x:expr) => { __arch_xchg($x as usize, $ptr as *mut core::ffi::c_void, core::mem::size_of_val(unsafe { &*$ptr }) as i32) as _ };
}

#[inline(always)]
pub unsafe fn xchg_small(ptr: *mut core::ffi::c_void, x: u32, size: i32) -> u32 {
    let off = (ptr as usize) % core::mem::size_of::<u32>();
    let p = (ptr as *mut u32).byte_offset(-(off as isize));
    let bitoff = off * BITS_PER_BYTE;
    let bitmask = ((0x1u32 << (size * BITS_PER_BYTE)) - 1) << bitoff;
    let mut oldv: u32;
    let mut ret: u32;
    loop {
        oldv = unsafe { core::ptr::read_volatile(p) };
        ret = (oldv & bitmask) >> bitoff;
        let newv = (oldv & !bitmask) | (x << bitoff);
        if unsafe { __cmpxchg_u32(p.cast(), oldv as i32, newv as i32) } == oldv as usize { break; }
    }
    ret
}

unsafe extern "C" { pub fn __xchg_called_with_bad_pointer(); }

#[inline(always)]
pub unsafe fn __arch_xchg(x: usize, ptr: *mut core::ffi::c_void, size: i32) -> usize {
    match size {
        1 => unsafe { xchg_small(ptr, x as u32, 1) as usize },
        2 => unsafe { xchg_small(ptr, x as u32, 2) as usize },
        4 => unsafe { xchg_u32(ptr.cast(), x) },
        _ => { unsafe { __xchg_called_with_bad_pointer() }; x }
    }
}

// External dependencies from the included kernel headers.
unsafe extern "C" {
    fn cmpxchg_emu_u8(ptr: *mut core::ffi::c_void, old: usize, new: usize) -> usize;
    fn __generic_cmpxchg_local(ptr: *mut core::ffi::c_void, old: usize, new: usize, size: i32) -> usize;
    fn __generic_cmpxchg64_local(ptr: *mut core::ffi::c_void, old: u64, new: u64) -> u64;
}

const BITS_PER_BYTE: usize = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
