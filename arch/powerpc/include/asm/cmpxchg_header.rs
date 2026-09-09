/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of the PowerPC atomic exchange and compare/exchange header.
// The original include and preprocessor guards are represented by this file's
// compilation context; kernel-provided types, barriers, and build assertions
// remain external dependencies.

#[cfg(feature = "kernel")]
const fn bitoff_cal(size: usize, off: usize) -> usize {
    #[cfg(target_endian = "big")]
    { (core::mem::size_of::<u32>() - size - off) * 8 }
    #[cfg(not(target_endian = "big"))]
    { off * 8 }
}

#[cfg(feature = "kernel")]
macro_rules! xchg_gen {
    ($name:ident, $ty:ty, $clobber:literal) => {
        #[inline(always)]
        pub unsafe fn $name(p: *mut core::ffi::c_void, mut val: u32) -> u32 {
            let off = (p as usize) % core::mem::size_of::<u32>();
            let bitoff = bitoff_cal(core::mem::size_of::<$ty>(), off);
            let q = (p as *mut u8).sub(off) as *mut u32;
            val <<= bitoff;
            let prev_mask: u32 = (<$ty>::MAX as u32) << bitoff;
            let mut prev: u32;
            let mut tmp: u32;
            core::arch::asm!(
                "1: lwarx {prev},0,{ptr}",
                "andc {tmp},{prev},{mask}",
                "or {tmp},{tmp},{val}",
                "stwcx. {tmp},0,{ptr}",
                "bne- 1b",
                prev = out(reg) prev, tmp = out(reg) tmp,
                ptr = in(reg) q, val = in(reg) val, mask = in(reg) prev_mask,
                options(nostack),
            );
            prev >> bitoff
        }
    };
}

#[cfg(feature = "kernel")]
macro_rules! cmpxchg_gen {
    ($name:ident, $ty:ty) => {
        #[inline(always)]
        pub unsafe fn $name(p: *mut core::ffi::c_void, mut old: u32, mut new: u32) -> u32 {
            let off = (p as usize) % core::mem::size_of::<u32>();
            let bitoff = bitoff_cal(core::mem::size_of::<$ty>(), off);
            let q = (p as *mut u8).sub(off) as *mut u32;
            old <<= bitoff; new <<= bitoff;
            let mask: u32 = (<$ty>::MAX as u32) << bitoff;
            let mut prev: u32; let mut tmp: u32;
            core::arch::asm!(
                "1: lwarx {prev},0,{ptr}",
                "and {tmp},{prev},{mask}", "cmpw 0,{tmp},{old}", "bne- 2f",
                "andc {tmp},{prev},{mask}", "or {tmp},{tmp},{new}",
                "stwcx. {tmp},0,{ptr}", "bne- 1b", "2:",
                prev = out(reg) prev, tmp = out(reg) tmp,
                ptr = in(reg) q, old = in(reg) old, new = in(reg) new,
                mask = in(reg) mask, options(nostack),
            );
            prev >> bitoff
        }
    };
}

#[cfg(feature = "kernel")]
macro_rules! simple_xchg {
    ($name:ident, $t:ty, $load:literal, $store:literal) => {
        #[inline(always)]
        pub unsafe fn $name(p: *mut $t, val: usize) -> usize {
            let mut prev: usize;
            core::arch::asm!("1: {load} {prev},0,{ptr}", "{store} {val},0,{ptr}", "bne- 1b",
                load=$load, store=$store, prev=out(reg) prev, ptr=in(reg) p, val=in(reg) val,
                options(nostack)); prev
        }
    };
}

#[cfg(feature = "kernel")]
simple_xchg!(__xchg_u8_local, u8, "lbarx", "stbcx.");
#[cfg(feature = "kernel")]
simple_xchg!(__xchg_u8_relaxed, u8, "lbarx", "stbcx.");
#[cfg(feature = "kernel")]
simple_xchg!(__xchg_u16_local, u16, "lharx", "sthcx.");
#[cfg(feature = "kernel")]
simple_xchg!(__xchg_u16_relaxed, u16, "lharx", "sthcx.");
#[cfg(feature = "kernel")]
simple_xchg!(__xchg_u32_local, u32, "lwarx", "stwcx.");
#[cfg(feature = "kernel")]
simple_xchg!(__xchg_u32_relaxed, u32, "lwarx", "stwcx.");

#[cfg(all(feature = "kernel", target_pointer_width = "64"))]
simple_xchg!(__xchg_u64_local, u64, "ldarx", "stdcx.");
#[cfg(all(feature = "kernel", target_pointer_width = "64"))]
simple_xchg!(__xchg_u64_relaxed, u64, "ldarx", "stdcx.");

#[cfg(feature = "kernel")]
xchg_gen!(__xchg_u8_local_generic, u8, "memory");
#[cfg(feature = "kernel")]
xchg_gen!(__xchg_u8_relaxed_generic, u8, "cc");
#[cfg(feature = "kernel")]
xchg_gen!(__xchg_u16_local_generic, u16, "memory");
#[cfg(feature = "kernel")]
xchg_gen!(__xchg_u16_relaxed_generic, u16, "cc");

#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u8_generic, u8);
#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u16_generic, u16);

// The following operations preserve the C API's size dispatch and failure
// behavior. Unsupported sizes correspond to BUILD_BUG_ON_MSG in the header.
#[cfg(feature = "kernel")]
#[inline(always)]
pub unsafe fn __xchg_local(ptr: *mut core::ffi::c_void, x: usize, size: u32) -> usize {
    match size { 1 => __xchg_u8_local(ptr as *mut u8, x), 2 => __xchg_u16_local(ptr as *mut u16, x),
        4 => __xchg_u32_local(ptr as *mut u32, x),
        #[cfg(target_pointer_width = "64")] 8 => __xchg_u64_local(ptr as *mut u64, x),
        _ => panic!("Unsupported size for __xchg_local") }
}

#[cfg(feature = "kernel")]
#[inline(always)]
pub unsafe fn __xchg_relaxed(ptr: *mut core::ffi::c_void, x: usize, size: u32) -> usize {
    match size { 1 => __xchg_u8_relaxed(ptr as *mut u8, x), 2 => __xchg_u16_relaxed(ptr as *mut u16, x),
        4 => __xchg_u32_relaxed(ptr as *mut u32, x),
        #[cfg(target_pointer_width = "64")] 8 => __xchg_u64_relaxed(ptr as *mut u64, x),
        _ => panic!("Unsupported size for __xchg_relaxed") }
}

#[cfg(feature = "kernel")]
#[macro_export]
macro_rules! arch_xchg_local { ($ptr:expr, $x:expr) => {{ let _x_ = $x; $crate::__xchg_local($ptr as *mut _, _x_ as usize, core::mem::size_of_val(unsafe { &*$ptr }) as u32) }} }
#[cfg(feature = "kernel")]
#[macro_export]
macro_rules! arch_xchg_relaxed { ($ptr:expr, $x:expr) => {{ let _x_ = $x; $crate::__xchg_relaxed($ptr as *mut _, _x_ as usize, core::mem::size_of_val(unsafe { &*$ptr }) as u32) }} }

// Compare-and-exchange variants retain the original PowerPC reservation loop;
// barrier macros supplied by asm/synch.h are intentionally external.
#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u8, u8);
#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u8_local, u8);
#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u8_relaxed, u8);
#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u8_acquire, u8);
#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u16, u16);
#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u16_local, u16);
#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u16_relaxed, u16);
#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u16_acquire, u16);
#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u32, u32);
#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u32_local, u32);
#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u32_relaxed, u32);
#[cfg(feature = "kernel")]
cmpxchg_gen!(__cmpxchg_u32_acquire, u32);

#[cfg(all(feature = "kernel", target_pointer_width = "64"))]
cmpxchg_gen!(__cmpxchg_u64, u64);
#[cfg(all(feature = "kernel", target_pointer_width = "64"))]
cmpxchg_gen!(__cmpxchg_u64_local, u64);
#[cfg(all(feature = "kernel", target_pointer_width = "64"))]
cmpxchg_gen!(__cmpxchg_u64_relaxed, u64);
#[cfg(all(feature = "kernel", target_pointer_width = "64"))]
cmpxchg_gen!(__cmpxchg_u64_acquire, u64);

#[cfg(feature = "kernel")]
#[inline(always)]
pub unsafe fn __cmpxchg(ptr: *mut core::ffi::c_void, old: usize, new: usize, size: u32) -> usize {
    match size { 1 => __cmpxchg_u8(ptr as *mut _, old as u32, new as u32),
        2 => __cmpxchg_u16(ptr as *mut _, old as u32, new as u32),
        4 => __cmpxchg_u32(ptr as *mut _, old as u32, new as u32),
        #[cfg(target_pointer_width = "64")] 8 => __cmpxchg_u64(ptr as *mut _, old, new),
        _ => panic!("Unsupported size for __cmpxchg") }
}

#[cfg(feature = "kernel")]
#[inline(always)]
pub unsafe fn __cmpxchg_local(ptr: *mut core::ffi::c_void, old: usize, new: usize, size: u32) -> usize {
    match size { 1 => __cmpxchg_u8_local(ptr as *mut _, old as u32, new as u32),
        2 => __cmpxchg_u16_local(ptr as *mut _, old as u32, new as u32),
        4 => __cmpxchg_u32_local(ptr as *mut _, old as u32, new as u32),
        #[cfg(target_pointer_width = "64")] 8 => __cmpxchg_u64_local(ptr as *mut _, old, new),
        _ => panic!("Unsupported size for __cmpxchg_local") }
}

#[cfg(feature = "kernel")]
#[inline(always)]
pub unsafe fn __cmpxchg_relaxed(ptr: *mut core::ffi::c_void, old: usize, new: usize, size: u32) -> usize {
    match size { 1 => __cmpxchg_u8_relaxed(ptr as *mut _, old as u32, new as u32),
        2 => __cmpxchg_u16_relaxed(ptr as *mut _, old as u32, new as u32),
        4 => __cmpxchg_u32_relaxed(ptr as *mut _, old as u32, new as u32),
        #[cfg(target_pointer_width = "64")] 8 => __cmpxchg_u64_relaxed(ptr as *mut _, old, new),
        _ => panic!("Unsupported size for __cmpxchg_relaxed") }
}

#[cfg(feature = "kernel")]
#[inline(always)]
pub unsafe fn __cmpxchg_acquire(ptr: *mut core::ffi::c_void, old: usize, new: usize, size: u32) -> usize {
    match size { 1 => __cmpxchg_u8_acquire(ptr as *mut _, old as u32, new as u32),
        2 => __cmpxchg_u16_acquire(ptr as *mut _, old as u32, new as u32),
        4 => __cmpxchg_u32_acquire(ptr as *mut _, old as u32, new as u32),
        #[cfg(target_pointer_width = "64")] 8 => __cmpxchg_u64_acquire(ptr as *mut _, old, new),
        _ => panic!("Unsupported size for __cmpxchg_acquire") }
}

#[cfg(feature = "kernel")]
#[macro_export]
macro_rules! arch_cmpxchg { ($p:expr, $o:expr, $n:expr) => {{ $crate::__cmpxchg($p as *mut _, $o as usize, $n as usize, core::mem::size_of_val(unsafe { &*$p }) as u32) }} }
#[cfg(feature = "kernel")]
#[macro_export]
macro_rules! arch_cmpxchg_local { ($p:expr, $o:expr, $n:expr) => {{ $crate::__cmpxchg_local($p as *mut _, $o as usize, $n as usize, core::mem::size_of_val(unsafe { &*$p }) as u32) }} }
#[cfg(feature = "kernel")]
#[macro_export]
macro_rules! arch_cmpxchg_relaxed { ($p:expr, $o:expr, $n:expr) => {{ $crate::__cmpxchg_relaxed($p as *mut _, $o as usize, $n as usize, core::mem::size_of_val(unsafe { &*$p }) as u32) }} }
#[cfg(feature = "kernel")]
#[macro_export]
macro_rules! arch_cmpxchg_acquire { ($p:expr, $o:expr, $n:expr) => {{ $crate::__cmpxchg_acquire($p as *mut _, $o as usize, $n as usize, core::mem::size_of_val(unsafe { &*$p }) as u32) }} }

// On 32-bit PowerPC the original header imports asm-generic/cmpxchg-local.h
// for arch_cmpxchg64_local; that external generic implementation remains so.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
