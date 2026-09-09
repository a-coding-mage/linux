// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *    Architecture-specific kernel symbols
 *
 *    Copyright (C) 2000-2001 Richard Hirst <rhirst with parisc-linux.org>
 *    Copyright (C) 2001 Dave Kennedy
 *    Copyright (C) 2001 Paul Bame <bame at parisc-linux.org>
 *    Copyright (C) 2001-2003 Grant Grundler <grundler with parisc-linux.org>
 *    Copyright (C) 2002-2003 Matthew Wilcox <willy at parisc-linux.org>
 *    Copyright (C) 2002 Randolph Chung <tausq at parisc-linux.org>
 *    Copyright (C) 2002-2007 Helge Deller <deller with parisc-linux.org>
 */

// Kernel headers provide these declarations and the EXPORT_SYMBOL! macro.
extern "C" {
    pub static mut memset: unsafe extern "C" fn(*mut core::ffi::c_void, i32, usize) -> *mut core::ffi::c_void;
    pub fn __xchg8();
    pub fn __xchg32();
    pub fn __cmpxchg_u8();
    pub fn __cmpxchg_u16();
    pub fn __cmpxchg_u32();
    pub fn __cmpxchg_u64();
    pub fn lclear_user();

    #[link_name = "$global$"]
    pub static mut global: i32;

    #[link_name = "$$divI"] pub fn div_i();
    #[link_name = "$$divU"] pub fn div_u();
    #[link_name = "$$remI"] pub fn rem_i();
    #[link_name = "$$remU"] pub fn rem_u();
    #[link_name = "$$mulI"] pub fn mul_i();
    #[link_name = "$$divU_3"] pub fn div_u_3();
    #[link_name = "$$divU_5"] pub fn div_u_5();
    #[link_name = "$$divU_6"] pub fn div_u_6();
    #[link_name = "$$divU_9"] pub fn div_u_9();
    #[link_name = "$$divU_10"] pub fn div_u_10();
    #[link_name = "$$divU_12"] pub fn div_u_12();
    #[link_name = "$$divU_7"] pub fn div_u_7();
    #[link_name = "$$divU_14"] pub fn div_u_14();
    #[link_name = "$$divU_15"] pub fn div_u_15();
    #[link_name = "$$divI_3"] pub fn div_i_3();
    #[link_name = "$$divI_5"] pub fn div_i_5();
    #[link_name = "$$divI_6"] pub fn div_i_6();
    #[link_name = "$$divI_7"] pub fn div_i_7();
    #[link_name = "$$divI_9"] pub fn div_i_9();
    #[link_name = "$$divI_10"] pub fn div_i_10();
    #[link_name = "$$divI_12"] pub fn div_i_12();
    #[link_name = "$$divI_14"] pub fn div_i_14();
    #[link_name = "$$divI_15"] pub fn div_i_15();

    pub fn __ashrdi3();
    pub fn __ashldi3();
    pub fn __lshrdi3();
    pub fn __muldi3();
    pub fn __ucmpdi2();
    pub fn __canonicalize_funcptr_for_compare(_: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    pub fn clear_page_asm();
    pub fn copy_page_asm();
}

EXPORT_SYMBOL!(memset);
EXPORT_SYMBOL!(__xchg8);
EXPORT_SYMBOL!(__xchg32);
EXPORT_SYMBOL!(__cmpxchg_u8);
EXPORT_SYMBOL!(__cmpxchg_u16);
EXPORT_SYMBOL!(__cmpxchg_u32);
EXPORT_SYMBOL!(__cmpxchg_u64);
#[cfg(feature = "CONFIG_SMP")]
EXPORT_SYMBOL!(__atomic_hash);
#[cfg(feature = "CONFIG_64BIT")]
EXPORT_SYMBOL!(__xchg64);
EXPORT_SYMBOL!(lclear_user);

#[cfg(not(feature = "CONFIG_64BIT"))]
EXPORT_SYMBOL!(global);

EXPORT_SYMBOL!(div_i);
EXPORT_SYMBOL!(div_u);
EXPORT_SYMBOL!(rem_i);
EXPORT_SYMBOL!(rem_u);
EXPORT_SYMBOL!(mul_i);
EXPORT_SYMBOL!(div_u_3);
EXPORT_SYMBOL!(div_u_5);
EXPORT_SYMBOL!(div_u_6);
EXPORT_SYMBOL!(div_u_9);
EXPORT_SYMBOL!(div_u_10);
EXPORT_SYMBOL!(div_u_12);
EXPORT_SYMBOL!(div_u_7);
EXPORT_SYMBOL!(div_u_14);
EXPORT_SYMBOL!(div_u_15);
EXPORT_SYMBOL!(div_i_3);
EXPORT_SYMBOL!(div_i_5);
EXPORT_SYMBOL!(div_i_6);
EXPORT_SYMBOL!(div_i_7);
EXPORT_SYMBOL!(div_i_9);
EXPORT_SYMBOL!(div_i_10);
EXPORT_SYMBOL!(div_i_12);
EXPORT_SYMBOL!(div_i_14);
EXPORT_SYMBOL!(div_i_15);
EXPORT_SYMBOL!(__ashrdi3);
EXPORT_SYMBOL!(__ashldi3);
EXPORT_SYMBOL!(__lshrdi3);
EXPORT_SYMBOL!(__muldi3);
EXPORT_SYMBOL!(__ucmpdi2);
EXPORT_SYMBOL!(__canonicalize_funcptr_for_compare);

#[cfg(feature = "CONFIG_64BIT")]
extern "C" {
    pub fn __divdi3();
    pub fn __udivdi3();
    pub fn __umoddi3();
    pub fn __moddi3();
}
#[cfg(feature = "CONFIG_64BIT")]
EXPORT_SYMBOL!(__divdi3);
#[cfg(feature = "CONFIG_64BIT")]
EXPORT_SYMBOL!(__udivdi3);
#[cfg(feature = "CONFIG_64BIT")]
EXPORT_SYMBOL!(__umoddi3);
#[cfg(feature = "CONFIG_64BIT")]
EXPORT_SYMBOL!(__moddi3);

#[cfg(not(feature = "CONFIG_64BIT"))]
extern "C" {
    #[link_name = "$$dyncall"] pub fn dyncall();
}
#[cfg(not(feature = "CONFIG_64BIT"))]
EXPORT_SYMBOL!(dyncall);

#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
extern "C" { pub fn _mcount(); }
#[cfg(feature = "CONFIG_FUNCTION_TRACER")]
EXPORT_SYMBOL!(_mcount);

// from pacache.S -- needed for clear/copy_page
EXPORT_SYMBOL!(clear_page_asm);
EXPORT_SYMBOL!(copy_page_asm);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
