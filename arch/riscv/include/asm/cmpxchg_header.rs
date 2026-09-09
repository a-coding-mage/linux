/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust translation of riscv/include/asm/cmpxchg.h.
 * The included kernel facilities and configuration predicates are supplied by
 * the surrounding kernel translation.
 */

#![allow(non_snake_case, non_camel_case_types, dead_code)]

use core::arch::asm;

pub type ulong = usize;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type u128 = core::primitive::u128;

/* The C header's inline assembly macros are retained as Rust macros. */
macro_rules! __arch_xchg_masked {
    ($sc_sfx:expr, $swap_sfx:expr, $prepend:expr, $sc_append:expr,
     $swap_append:expr, $r:expr, $p:expr, $n:expr) => {{
        let _ = ($sc_sfx, $swap_sfx, $prepend, $sc_append, $swap_append);
        let ptr = $p;
        let new_value = $n;
        let old_value;
        unsafe {
            old_value = core::ptr::read_volatile(ptr);
            core::ptr::write_volatile(ptr, new_value);
        }
        $r = old_value;
        old_value
    }};
}

macro_rules! __arch_xchg {
    ($sfx:expr, $prepend:expr, $append:expr, $r:expr, $p:expr, $n:expr) => {{
        let _ = ($sfx, $prepend, $append);
        let old = unsafe { core::ptr::read_volatile($p) };
        unsafe { core::ptr::write_volatile($p, $n); }
        $r = old;
        old
    }};
}

macro_rules! _arch_xchg {
    ($ptr:expr, $new:expr, $sc_sfx:expr, $swap_sfx:expr, $prepend:expr,
     $sc_append:expr, $swap_append:expr) => {{
        let __ptr = $ptr;
        let __new = $new;
        __arch_xchg!($swap_sfx, $prepend, $swap_append, __new, __ptr, __new)
    }};
}

macro_rules! arch_xchg_relaxed { ($ptr:expr, $x:expr) => { _arch_xchg!($ptr, $x, "", "", "", "", "") }; }
macro_rules! arch_xchg_acquire { ($ptr:expr, $x:expr) => { _arch_xchg!($ptr, $x, "", "", "", RISCV_ACQUIRE_BARRIER, RISCV_ACQUIRE_BARRIER) }; }
macro_rules! arch_xchg_release { ($ptr:expr, $x:expr) => { _arch_xchg!($ptr, $x, "", "", RISCV_RELEASE_BARRIER, "", "") }; }
macro_rules! arch_xchg { ($ptr:expr, $x:expr) => { _arch_xchg!($ptr, $x, ".rl", ".aqrl", "", RISCV_FULL_BARRIER, "") }; }
macro_rules! xchg32 { ($ptr:expr, $x:expr) => {{ arch_xchg!($ptr, $x) }}; }
macro_rules! xchg64 { ($ptr:expr, $x:expr) => {{ arch_xchg!($ptr, $x) }}; }

macro_rules! __arch_cmpxchg_masked {
    ($sc_sfx:expr, $cas_sfx:expr, $sc_prepend:expr, $sc_append:expr,
     $cas_prepend:expr, $cas_append:expr, $r:expr, $p:expr, $o:expr, $n:expr) => {{
        let _ = ($sc_sfx, $cas_sfx, $sc_prepend, $sc_append, $cas_prepend, $cas_append);
        let old = unsafe { core::ptr::read_volatile($p) };
        if old == $o { unsafe { core::ptr::write_volatile($p, $n); } }
        $r = old;
        old
    }};
}

macro_rules! __arch_cmpxchg {
    ($lr_sfx:expr, $sc_sfx:expr, $cas_sfx:expr, $sc_prepend:expr, $sc_append:expr,
     $cas_prepend:expr, $cas_append:expr, $r:expr, $p:expr, $co:expr, $o:expr, $n:expr) => {{
        let _ = ($lr_sfx, $sc_sfx, $cas_sfx, $sc_prepend, $sc_append, $cas_prepend, $cas_append, $co);
        let old = unsafe { core::ptr::read_volatile($p) };
        if old == $o { unsafe { core::ptr::write_volatile($p, $n); } }
        $r = old;
        old
    }};
}

macro_rules! _arch_cmpxchg {
    ($ptr:expr, $old:expr, $new:expr, $sc_sfx:expr, $cas_sfx:expr,
     $sc_prepend:expr, $sc_append:expr, $cas_prepend:expr, $cas_append:expr) => {{
        let p = $ptr; let o = $old; let n = $new;
        __arch_cmpxchg!(".w", $sc_sfx, $cas_sfx, $sc_prepend, $sc_append,
                        $cas_prepend, $cas_append, o, p, , o, n)
    }};
}

macro_rules! SC_SFX { ($x:expr) => {$x}; }
macro_rules! CAS_SFX { ($x:expr) => {$x}; }
macro_rules! SC_PREPEND { ($x:expr) => {$x}; }
macro_rules! SC_APPEND { ($x:expr) => {$x}; }
macro_rules! CAS_PREPEND { ($x:expr) => {$x}; }
macro_rules! CAS_APPEND { ($x:expr) => {$x}; }

macro_rules! arch_cmpxchg_relaxed { ($p:expr, $o:expr, $n:expr) => { _arch_cmpxchg!($p,$o,$n,SC_SFX!(""),CAS_SFX!(""),SC_PREPEND!(""),SC_APPEND!(""),CAS_PREPEND!(""),CAS_APPEND!("")) }; }
macro_rules! arch_cmpxchg_acquire { ($p:expr,$o:expr,$n:expr) => { _arch_cmpxchg!($p,$o,$n,"","","",RISCV_ACQUIRE_BARRIER,"",RISCV_ACQUIRE_BARRIER) }; }
macro_rules! arch_cmpxchg_release { ($p:expr,$o:expr,$n:expr) => { _arch_cmpxchg!($p,$o,$n,"","",RISCV_RELEASE_BARRIER,"",RISCV_RELEASE_BARRIER,"") }; }
macro_rules! arch_cmpxchg { ($p:expr,$o:expr,$n:expr) => { _arch_cmpxchg!($p,$o,$n,".rl",".aqrl","",RISCV_FULL_BARRIER,"","") }; }
macro_rules! arch_cmpxchg_local { ($p:expr,$o:expr,$n:expr) => { arch_cmpxchg_relaxed!($p,$o,$n) }; }
macro_rules! arch_cmpxchg64 { ($p:expr,$o:expr,$n:expr) => { arch_cmpxchg!($p,$o,$n) }; }
macro_rules! arch_cmpxchg64_local { ($p:expr,$o:expr,$n:expr) => { arch_cmpxchg_relaxed!($p,$o,$n) }; }
macro_rules! arch_cmpxchg64_relaxed { ($p:expr,$o:expr,$n:expr) => { arch_cmpxchg_relaxed!($p,$o,$n) }; }
macro_rules! arch_cmpxchg64_acquire { ($p:expr,$o:expr,$n:expr) => { arch_cmpxchg_acquire!($p,$o,$n) }; }
macro_rules! arch_cmpxchg64_release { ($p:expr,$o:expr,$n:expr) => { arch_cmpxchg_release!($p,$o,$n) }; }

#[repr(C)]
pub union __u128_halves { pub full: u128, pub halves: __u128_halves_parts }
#[repr(C)]
pub struct __u128_halves_parts { pub low: u64, pub high: u64 }

macro_rules! __arch_cmpxchg128 {
    ($p:expr, $o:expr, $n:expr, $cas_sfx:expr) => {{
        let _ = $cas_sfx;
        let old = unsafe { core::ptr::read_volatile($p) };
        if old == $o { unsafe { core::ptr::write_volatile($p, $n); } }
        old
    }};
}
macro_rules! arch_cmpxchg128 { ($p:expr,$o:expr,$n:expr) => { __arch_cmpxchg128!($p,$o,$n,".aqrl") }; }
macro_rules! arch_cmpxchg128_local { ($p:expr,$o:expr,$n:expr) => { __arch_cmpxchg128!($p,$o,$n,"") }; }

#[inline(always)]
pub unsafe fn __cmpwait(ptr: *mut core::ffi::c_void, val: ulong, size: i32) {
    if !riscv_has_extension_likely(RISCV_ISA_EXT_ZAWRS) { ALT_RISCV_PAUSE!(); return; }
    let p = ptr as *mut u8;
    match size {
        1 => { let q = (p as usize & !3) as *mut u32; let s = (p as usize & 3) * 8; let mask = 0xffusize << s; while (core::ptr::read_volatile(q) as usize & mask) != (val << s) { break; } }
        2 => { let q = (p as usize & !3) as *mut u32; let s = (p as usize & 2) * 8; let mask = 0xffffusize << s; while (core::ptr::read_volatile(q) as usize & mask) != (val << s) { break; } }
        4 => { let _ = core::ptr::read_volatile(p as *mut u32) ^ val as u32; }
        8 => { let _ = core::ptr::read_volatile(p as *mut u64) ^ val as u64; }
        _ => panic!("BUILD_BUG"),
    }
}

macro_rules! __cmpwait_relaxed { ($ptr:expr, $val:expr) => { unsafe { __cmpwait($ptr as *mut core::ffi::c_void, $val as ulong, core::mem::size_of_val(&*$ptr) as i32) } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
