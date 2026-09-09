/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright IBM Corp. 1999, 2011
 *
 * Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>,
 */

// C header dependencies are supplied by the surrounding kernel translation.

extern "C" {
    pub fn __cmpxchg_called_with_bad_pointer();
    pub fn __xchg_called_with_bad_pointer();
}

#[inline(always)]
unsafe fn __cs_asm(ptr: u64, mut old: u32, new: u32) -> u32 {
    core::arch::asm!(
        "cs {old}, {new}, {ptr}",
        old = inout(reg) old,
        new = in(reg) new,
        ptr = in(reg) ptr,
        options(nostack)
    );
    old
}

#[inline(always)]
unsafe fn __csg_asm(ptr: u64, mut old: u64, new: u64) -> u64 {
    core::arch::asm!(
        "csg {old}, {new}, {ptr}",
        old = inout(reg) old,
        new = in(reg) new,
        ptr = in(reg) ptr,
        options(nostack)
    );
    old
}

#[repr(C)]
union __CmpxchgOld32 {
    b: [u8; 4],
    w: u32,
}

#[repr(C)]
union __CmpxchgOld16 {
    b: [u16; 2],
    w: u32,
}

#[inline]
unsafe fn __arch_cmpxchg1(mut ptr: u64, old: u8, new: u8) -> u8 {
    let i = (ptr & 3) as usize;
    ptr &= !0x3;
    let mut prev = core::ptr::read_volatile(ptr as *const u32);
    loop {
        let mut old32 = __CmpxchgOld32 { w: prev };
        if old32.b[i] != old { return old32.b[i]; }
        let mut new32 = __CmpxchgOld32 { w: old32.w };
        new32.b[i] = new;
        prev = __cs_asm(ptr, old32.w, new32.w);
        if prev == old32.w { return old; }
    }
}

#[inline]
unsafe fn __arch_cmpxchg2(mut ptr: u64, old: u16, new: u16) -> u16 {
    let i = ((ptr & 3) >> 1) as usize;
    ptr &= !0x3;
    let mut prev = core::ptr::read_volatile(ptr as *const u32);
    loop {
        let mut old32 = __CmpxchgOld16 { w: prev };
        if old32.b[i] != old { return old32.b[i]; }
        let mut new32 = __CmpxchgOld16 { w: old32.w };
        new32.b[i] = new;
        prev = __cs_asm(ptr, old32.w, new32.w);
        if prev == old32.w { return old; }
    }
}

#[inline(always)]
unsafe fn __arch_cmpxchg(ptr: u64, old: u64, new: u64, size: usize) -> u64 {
    match size {
        1 => __arch_cmpxchg1(ptr, (old & 0xff) as u8, (new & 0xff) as u8) as u64,
        2 => __arch_cmpxchg2(ptr, (old & 0xffff) as u16, (new & 0xffff) as u16) as u64,
        4 => __cs_asm(ptr, old as u32, new as u32) as u64,
        8 => __csg_asm(ptr, old, new),
        _ => { __cmpxchg_called_with_bad_pointer(); old }
    }
}

macro_rules! arch_cmpxchg {
    ($ptr:expr, $o:expr, $n:expr) => {
        __arch_cmpxchg($ptr as *mut _ as u64, $o as u64, $n as u64,
                       core::mem::size_of_val(&*$ptr)) as _
    };
}
pub(crate) use arch_cmpxchg;
macro_rules! arch_cmpxchg64 { ($($t:tt)*) => { arch_cmpxchg!($($t)*) }; }
macro_rules! arch_cmpxchg_local { ($($t:tt)*) => { arch_cmpxchg!($($t)*) }; }
macro_rules! arch_cmpxchg64_local { ($($t:tt)*) => { arch_cmpxchg!($($t)*) }; }
pub(crate) use {arch_cmpxchg64, arch_cmpxchg_local, arch_cmpxchg64_local};

// The C implementation has separate asm-flag-output and fallback variants.
// This fallback preserves the behavior when that build-time feature is absent.
macro_rules! arch_try_cmpxchg {
    ($ptr:expr, $oldp:expr, $new:expr) => {{
        let __old = *($oldp);
        let __prev = arch_cmpxchg!($ptr, __old, $new);
        if __prev != __old { *($oldp) = __prev; }
        __prev == __old
    }};
}
pub(crate) use arch_try_cmpxchg;
macro_rules! arch_try_cmpxchg64 { ($($t:tt)*) => { arch_try_cmpxchg!($($t)*) }; }
macro_rules! arch_try_cmpxchg_local { ($($t:tt)*) => { arch_try_cmpxchg!($($t)*) }; }
macro_rules! arch_try_cmpxchg64_local { ($($t:tt)*) => { arch_try_cmpxchg!($($t)*) }; }
pub(crate) use {arch_try_cmpxchg64, arch_try_cmpxchg_local, arch_try_cmpxchg64_local};

#[inline]
unsafe fn __arch_xchg1(mut ptr: u64, x: u8) -> u8 {
    let shift = ((3 ^ (ptr & 3)) << 3) as u32;
    ptr &= !0x3;
    let mask = !(0xffu32 << shift);
    let mut old = core::ptr::read_volatile(ptr as *const u32);
    loop {
        let new = (old & mask) | ((x as u32) << shift);
        if arch_try_cmpxchg!(ptr as *mut u32, &mut old, new) { return old >> shift as u32 as u8; }
    }
}

#[inline]
unsafe fn __arch_xchg2(mut ptr: u64, x: u16) -> u16 {
    let shift = ((2 ^ (ptr & 2)) << 3) as u32;
    ptr &= !0x3;
    let mask = !(0xffffu32 << shift);
    let mut old = core::ptr::read_volatile(ptr as *const u32);
    loop {
        let new = (old & mask) | ((x as u32) << shift);
        if arch_try_cmpxchg!(ptr as *mut u32, &mut old, new) { return (old >> shift) as u16; }
    }
}

#[inline(always)]
unsafe fn __arch_xchg(ptr: u64, x: u64, size: usize) -> u64 {
    match size {
        1 => __arch_xchg1(ptr, (x & 0xff) as u8) as u64,
        2 => __arch_xchg2(ptr, (x & 0xffff) as u16) as u64,
        4 => { let mut old = core::ptr::read_volatile(ptr as *const u32); while !arch_try_cmpxchg!(ptr as *mut u32, &mut old, x as u32) {} old as u64 }
        8 => { let mut old = core::ptr::read_volatile(ptr as *const u64); while !arch_try_cmpxchg!(ptr as *mut u64, &mut old, x) {} old }
        _ => { __xchg_called_with_bad_pointer(); x }
    }
}

macro_rules! arch_xchg { ($ptr:expr, $x:expr) => { __arch_xchg($ptr as *mut _ as u64, $x as u64, core::mem::size_of_val(&*$ptr)) as _ }; }
pub(crate) use arch_xchg;

#[inline(always)]
pub const fn system_has_cmpxchg128() -> i32 { 1 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
