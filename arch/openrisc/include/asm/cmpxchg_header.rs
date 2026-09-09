/*
 * 1,2 and 4 byte cmpxchg and xchg implementations for OpenRISC.
 *
 * Copyright (C) 2014 Stefan Kristiansson <stefan.kristiansson@saunalahti.fi>
 * Copyright (C) 2017 Stafford Horne <shorne@gmail.com>
 *
 * This file is licensed under the terms of the GNU General Public License
 * version 2.  This program is licensed "as is" without any warranty of any
 * kind, whether express or implied.
 *
 * Note:
 * The portable implementations of 1 and 2 byte xchg and cmpxchg using a 4
 * byte cmpxchg is sourced heavily from the sh and mips implementations.
 */

// C header dependencies: linux/bits.h, linux/compiler.h, linux/types.h.
// Build-time __BIG_ENDIAN selects the bit offset calculation below.

pub const __HAVE_ARCH_CMPXCHG: i32 = 1;

#[inline]
pub unsafe fn cmpxchg_u32(ptr: *mut core::ffi::c_void, mut old: usize, new: usize) -> usize {
    core::arch::asm!(
        "1: l.lwa {0}, 0({1})",
        "l.sfeq {0}, {2}",
        "l.bnf 2f",
        " l.nop",
        "l.swa 0({1}), {3}",
        "l.bnf 1b",
        " l.nop",
        "2:",
        inout(reg) old,
        in(reg) ptr,
        in(reg) old,
        in(reg) new,
        options(nostack)
    );
    old
}

#[inline]
pub unsafe fn xchg_u32(ptr: *mut core::ffi::c_void, mut val: usize) -> usize {
    core::arch::asm!(
        "1: l.lwa {0}, 0({1})",
        "l.swa 0({1}), {2}",
        "l.bnf 1b",
        " l.nop",
        inout(reg) val,
        in(reg) ptr,
        in(reg) val,
        options(nostack)
    );
    val
}

#[inline]
pub unsafe fn cmpxchg_small(ptr: *mut core::ffi::c_void, old: u32, new: u32, size: i32) -> u32 {
    let off = (ptr as usize) % core::mem::size_of::<u32>();
    let p = (ptr as *mut u8).sub(off) as *mut u32;
    #[cfg(target_endian = "big")]
    let bitoff = (core::mem::size_of::<u32>() - size as usize - off) * u32::BITS as usize;
    #[cfg(not(target_endian = "big"))]
    let bitoff = off * u32::BITS as usize;
    let bitmask = ((1u32 << (size as u32 * u32::BITS)) - 1) << bitoff;
    let mut load32 = core::ptr::read_volatile(p);
    loop {
        let ret = (load32 & bitmask) >> bitoff;
        if old != ret { return ret; }
        let old32 = (load32 & !bitmask) | (old << bitoff);
        let new32 = (load32 & !bitmask) | (new << bitoff);
        load32 = cmpxchg_u32(p.cast(), old32 as usize, new32 as usize) as u32;
        if load32 == old32 { return old; }
    }
}

/* xchg */
#[inline]
pub unsafe fn xchg_small(ptr: *mut core::ffi::c_void, x: u32, size: i32) -> u32 {
    let off = (ptr as usize) % core::mem::size_of::<u32>();
    let p = (ptr as *mut u8).sub(off) as *mut u32;
    #[cfg(target_endian = "big")]
    let bitoff = (core::mem::size_of::<u32>() - size as usize - off) * u32::BITS as usize;
    #[cfg(not(target_endian = "big"))]
    let bitoff = off * u32::BITS as usize;
    let bitmask = ((1u32 << (size as u32 * u32::BITS)) - 1) << bitoff;
    let (ret, _) = loop {
        let oldv = core::ptr::read_volatile(p);
        let ret = (oldv & bitmask) >> bitoff;
        let newv = (oldv & !bitmask) | (x << bitoff);
        if cmpxchg_u32(p.cast(), oldv as usize, newv as usize) as u32 == oldv { break (ret, oldv); }
    };
    ret
}

/* This function doesn't exist, so invalid cmpxchg sizes produce a linker error. */
unsafe extern "C" { pub fn __cmpxchg_called_with_bad_pointer() -> usize; }

#[inline]
pub unsafe fn __cmpxchg(ptr: *mut core::ffi::c_void, old: usize, new: usize, size: i32) -> usize {
    match size {
        1 | 2 => cmpxchg_small(ptr, old as u32, new as u32, size) as usize,
        4 => cmpxchg_u32(ptr, old, new),
        _ => __cmpxchg_called_with_bad_pointer(),
    }
}

/* This function doesn't exist, so invalidly-sized xchg calls produce a linker error. */
unsafe extern "C" { pub fn __xchg_called_with_bad_pointer() -> usize; }

#[inline]
pub unsafe fn __arch_xchg(ptr: *mut core::ffi::c_void, with: usize, size: i32) -> usize {
    match size {
        1 | 2 => xchg_small(ptr, with as u32, size) as usize,
        4 => xchg_u32(ptr, with),
        _ => __xchg_called_with_bad_pointer(),
    }
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
