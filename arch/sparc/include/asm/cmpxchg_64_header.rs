/* SPDX-License-Identifier: GPL-2.0 */
/* 64-bit atomic xchg() and cmpxchg() definitions.
 *
 * Copyright (C) 1996, 1997, 2000 David S. Miller (davem@redhat.com)
 */

// Original header guard: __ARCH_SPARC64_CMPXCHG__

#[inline]
pub unsafe fn __cmpxchg_u32(m: *mut core::ffi::c_int, old: core::ffi::c_int, mut new: core::ffi::c_int) -> u64 {
    core::arch::asm!("cas [{m}], {old}, {new}", m = in(reg) m, old = in(reg) old, new = inout(reg) new, options(nostack, preserves_flags));
    new as u64
}

#[inline]
pub unsafe fn xchg32(m: *mut u32, mut val: u32) -> u64 {
    let mut tmp1: u64;
    let mut tmp2: u64;
    core::arch::asm!(
        "mov {val}, {tmp1}",
        "1:",
        "lduw [{m}], {tmp2}",
        "cas [{m}], {tmp2}, {val}",
        "cmp {tmp2}, {val}",
        "bne,a,pn %icc, 1b",
        " mov {tmp1}, {val}",
        val = inout(reg) val,
        tmp1 = lateout(reg) tmp1,
        tmp2 = lateout(reg) tmp2,
        m = in(reg) m,
        options(nostack)
    );
    val as u64
}

#[inline]
pub unsafe fn xchg64(m: *mut u64, mut val: u64) -> u64 {
    let mut tmp1: u64;
    let mut tmp2: u64;
    core::arch::asm!(
        "mov {val}, {tmp1}", "1:", "ldx [{m}], {tmp2}",
        "casx [{m}], {tmp2}, {val}", "cmp {tmp2}, {val}",
        "bne,a,pn %xcc, 1b", " mov {tmp1}, {val}",
        val = inout(reg) val, tmp1 = lateout(reg) tmp1,
        tmp2 = lateout(reg) tmp2, m = in(reg) m, options(nostack)
    );
    val
}

#[inline]
pub unsafe fn xchg16(m: *mut u16, val: u16) -> u64 {
    let maddr = m as usize;
    let bit_shift = (((maddr & 2) ^ 2) << 3) as u32;
    let mask = 0xffffu32 << bit_shift;
    let ptr = (maddr & !2) as *mut u32;
    let mut load32 = core::ptr::read_volatile(ptr);
    let mut old32;
    loop {
        old32 = load32;
        let new32 = (load32 & !mask) | ((val as u32) << bit_shift);
        load32 = __cmpxchg_u32(ptr.cast(), old32 as i32, new32 as i32) as u32;
        if load32 == old32 { break; }
    }
    ((load32 & mask) >> bit_shift) as u64
}

extern "C" { pub fn __xchg_called_with_bad_pointer(); }

#[inline(always)]
pub unsafe fn __arch_xchg(x: u64, ptr: *mut core::ffi::c_void, size: i32) -> u64 {
    match size { 2 => xchg16(ptr.cast(), x as u16), 4 => xchg32(ptr.cast(), x as u32), 8 => xchg64(ptr.cast(), x), _ => { __xchg_called_with_bad_pointer(); x } }
}

#[inline]
pub unsafe fn __cmpxchg_u64(m: *mut core::ffi::c_long, old: u64, mut new: u64) -> u64 {
    core::arch::asm!("casx [{m}], {old}, {new}", m = in(reg) m, old = in(reg) old, new = inout(reg) new, options(nostack, preserves_flags));
    new
}

#[inline]
pub unsafe fn __cmpxchg_u8(m: *mut u8, old: u8, new: u8) -> u64 {
    let maddr = m as usize;
    let bit_shift = (((maddr & 3) ^ 3) << 3) as u32;
    let mask = 0xffu32 << bit_shift;
    let ptr = (maddr & !3) as *mut u32;
    let mut load32 = core::ptr::read_volatile(ptr);
    loop {
        let new32 = (load32 & !mask) | ((new as u32) << bit_shift);
        let old32 = (load32 & !mask) | ((old as u32) << bit_shift);
        load32 = __cmpxchg_u32(ptr.cast(), old32 as i32, new32 as i32) as u32;
        if load32 == old32 { return old as u64; }
        let load = (load32 & mask) >> bit_shift;
        if load != old as u32 { return load as u64; }
    }
}

extern "C" { pub fn __cmpxchg_called_with_bad_pointer(); }

#[inline]
pub unsafe fn __cmpxchg(ptr: *mut core::ffi::c_void, old: u64, new: u64, size: i32) -> u64 {
    match size { 1 => __cmpxchg_u8(ptr.cast(), old as u8, new as u8), 4 => __cmpxchg_u32(ptr.cast(), old as i32, new as i32), 8 => __cmpxchg_u64(ptr.cast(), old, new), _ => { __cmpxchg_called_with_bad_pointer(); old } }
}

extern "C" { pub fn __generic_cmpxchg_local(ptr: *mut core::ffi::c_void, old: u64, new: u64, size: i32) -> u64; }

#[inline]
pub unsafe fn __cmpxchg_local(ptr: *mut core::ffi::c_void, old: u64, new: u64, size: i32) -> u64 {
    match size { 4 | 8 => __cmpxchg(ptr, old, new, size), _ => __generic_cmpxchg_local(ptr, old, new, size) }
}

#[macro_export]
macro_rules! arch_xchg { ($ptr:expr, $x:expr) => {{
    unsafe { $crate::__arch_xchg($x as u64, ($ptr) as *mut core::ffi::c_void, core::mem::size_of_val(&*$ptr) as i32) }
}}; }

#[macro_export]
macro_rules! arch_cmpxchg { ($ptr:expr, $o:expr, $n:expr) => {{
    unsafe { $crate::__cmpxchg(($ptr) as *mut core::ffi::c_void, $o as u64, $n as u64, core::mem::size_of_val(&*$ptr) as i32) as _ }
}}; }

#[macro_export]
macro_rules! arch_cmpxchg_local { ($ptr:expr, $o:expr, $n:expr) => {{
    unsafe { $crate::__cmpxchg_local(($ptr) as *mut core::ffi::c_void, $o as u64, $n as u64, core::mem::size_of_val(&*$ptr) as i32) as _ }
}}; }

#[macro_export]
macro_rules! arch_cmpxchg64_local { ($ptr:expr, $o:expr, $n:expr) => {{
    const _: () = assert!(core::mem::size_of::<u64>() == 8);
    arch_cmpxchg_local!($ptr, $o, $n)
}}; }

#[macro_export]
macro_rules! arch_cmpxchg64 { ($ptr:expr, $o:expr, $n:expr) => {
    arch_cmpxchg64_local!($ptr, $o, $n)
}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
