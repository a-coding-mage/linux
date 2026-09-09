/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *	include/asm-mips/dec/prom.h
 *
 *	DECstation PROM interface.
 *
 *	Copyright (C) 2002, 2026  Maciej W. Rozycki
 *
 *	Based on arch/mips/dec/prom/prom.h by the Anonymous.
 */

// C dependencies: linux/types.h and asm/addrspace.h.

/*
 * PMAX/3MAX PROM entry points for DS2100/3100's and DS5000/2xx's.
 * Many of these will work for MIPSen as well!
 */
// CKSEG1ADDR(0x1fc00000), the PROM base address.
pub const VEC_RESET: *mut u64 = 0x1fc0_0000 as *mut u64;

#[inline]
pub const fn pmax_prom_entry(x: usize) -> *mut u64 {
    VEC_RESET.wrapping_add(x)
}

pub const PMAX_PROM_HALT: *mut u64 = pmax_prom_entry(2);
pub const PMAX_PROM_AUTOBOOT: *mut u64 = pmax_prom_entry(5);
pub const PMAX_PROM_OPEN: *mut u64 = pmax_prom_entry(6);
pub const PMAX_PROM_READ: *mut u64 = pmax_prom_entry(7);
pub const PMAX_PROM_CLOSE: *mut u64 = pmax_prom_entry(10);
pub const PMAX_PROM_LSEEK: *mut u64 = pmax_prom_entry(11);
pub const PMAX_PROM_GETCHAR: *mut u64 = pmax_prom_entry(12);
pub const PMAX_PROM_PUTCHAR: *mut u64 = pmax_prom_entry(13);
pub const PMAX_PROM_GETS: *mut u64 = pmax_prom_entry(15);
pub const PMAX_PROM_PRINTF: *mut u64 = pmax_prom_entry(17);
pub const PMAX_PROM_GETENV: *mut u64 = pmax_prom_entry(33);

/* Magic number indicating REX PROM available on DECstation. */
pub const REX_PROM_MAGIC: u32 = 0x3046_4354;

#[inline]
pub const fn prom_is_rex(magic: u32) -> bool {
    // CONFIG_CPU_R3000 is a build-time condition; non-R3000 builds return true.
    !cfg!(feature = "CONFIG_CPU_R3000") || magic == REX_PROM_MAGIC
}

/* 3MIN/MAXINE PROM entry points. */
pub const REX_PROM_GETBITMAP: usize = 0x84 / 4;
pub const REX_PROM_GETCHAR: usize = 0x24 / 4;
pub const REX_PROM_GETENV: usize = 0x64 / 4;
pub const REX_PROM_GETSYSID: usize = 0x80 / 4;
pub const REX_PROM_GETTCINFO: usize = 0xa4 / 4;
pub const REX_PROM_PRINTF: usize = 0x30 / 4;
pub const REX_PROM_SLOTADDR: usize = 0x6c / 4;
pub const REX_PROM_BOOTINIT: usize = 0x54 / 4;
pub const REX_PROM_BOOTREAD: usize = 0x58 / 4;
pub const REX_PROM_CLEARCACHE: usize = 0x7c / 4;

/* Used by rex_getbitmap(). */
#[repr(C)]
pub struct memmap {
    pub pagesize: i32,
    pub bitmap: [u8; 0],
}

/* Function pointers as read from a PROM's callback vector. */
extern "C" {
    pub static mut __rex_bootinit: Option<unsafe extern "C" fn() -> i32>;
    pub static mut __rex_bootread: Option<unsafe extern "C" fn() -> i32>;
    pub static mut __rex_getbitmap: Option<unsafe extern "C" fn(*mut memmap) -> i32>;
    pub static mut __rex_slot_address: Option<unsafe extern "C" fn(i32) -> *mut usize>;
    pub static mut __rex_gettcinfo: Option<unsafe extern "C" fn() -> *mut core::ffi::c_void>;
    pub static mut __rex_getsysid: Option<unsafe extern "C" fn() -> i32>;
    pub static mut __rex_clear_cache: Option<unsafe extern "C" fn()>;
    pub static mut __prom_getchar: Option<unsafe extern "C" fn() -> i32>;
    pub static mut __prom_getenv: Option<unsafe extern "C" fn(*mut i8) -> *mut i8>;
    pub static mut __prom_printf: Option<unsafe extern "C" fn(*mut i8, ...) -> i32>;
    pub static mut __pmax_open: Option<unsafe extern "C" fn(*mut i8, i32) -> i32>;
    pub static mut __pmax_lseek: Option<unsafe extern "C" fn(i32, isize, i32) -> i32>;
    pub static mut __pmax_read: Option<unsafe extern "C" fn(i32, *mut core::ffi::c_void, i32) -> i32>;
    pub static mut __pmax_close: Option<unsafe extern "C" fn(i32) -> i32>;
}

// The CONFIG_64BIT branch uses an O32 ABI dispatcher supplied elsewhere.
#[cfg(feature = "CONFIG_64BIT")]
pub const O32_STK_SIZE: usize = 512;

#[cfg(feature = "CONFIG_64BIT")]
extern "C" {
    pub static mut o32_stk: [usize; O32_STK_SIZE];
}

#[cfg(feature = "CONFIG_64BIT")]
#[inline]
pub unsafe fn o32_get_stk() -> *mut usize {
    // Equivalent to __builtin_frame_address(0); target-specific frame access
    // is supplied by the eventual MIPS backend.
    let fp: isize = 0;
    if fp != fp as i32 as isize {
        o32_stk.as_mut_ptr().wrapping_add(O32_STK_SIZE)
    } else {
        core::ptr::null_mut()
    }
}

#[cfg(feature = "CONFIG_64BIT")]
pub unsafe fn rex_bootinit() -> i32 { __rex_bootinit.unwrap()() }
#[cfg(feature = "CONFIG_64BIT")]
pub unsafe fn rex_bootread() -> i32 { __rex_bootread.unwrap()() }
#[cfg(feature = "CONFIG_64BIT")]
pub unsafe fn rex_getbitmap(x: *mut memmap) -> i32 { __rex_getbitmap.unwrap()(x) }
#[cfg(feature = "CONFIG_64BIT")]
pub unsafe fn rex_slot_address(x: i32) -> *mut usize { __rex_slot_address.unwrap()(x) }
#[cfg(feature = "CONFIG_64BIT")]
pub unsafe fn rex_gettcinfo() -> *mut core::ffi::c_void { __rex_gettcinfo.unwrap()() }
#[cfg(feature = "CONFIG_64BIT")]
pub unsafe fn rex_getsysid() -> i32 { __rex_getsysid.unwrap()() }
#[cfg(feature = "CONFIG_64BIT")]
pub unsafe fn rex_clear_cache() { __rex_clear_cache.unwrap()() }
#[cfg(feature = "CONFIG_64BIT")]
pub unsafe fn prom_getchar() -> i32 { __prom_getchar.unwrap()() }
#[cfg(feature = "CONFIG_64BIT")]
pub unsafe fn prom_getenv(x: *mut i8) -> *mut i8 { __prom_getenv.unwrap()(x) }

#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __rex_bootinit as rex_bootinit;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __rex_bootread as rex_bootread;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __rex_getbitmap as rex_getbitmap;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __rex_slot_address as rex_slot_address;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __rex_gettcinfo as rex_gettcinfo;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __rex_getsysid as rex_getsysid;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __rex_clear_cache as rex_clear_cache;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __prom_getchar as prom_getchar;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __prom_getenv as prom_getenv;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __prom_printf as prom_printf;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __pmax_open as pmax_open;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __pmax_lseek as pmax_lseek;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __pmax_read as pmax_read;
#[cfg(not(feature = "CONFIG_64BIT"))]
pub use __pmax_close as pmax_close;

extern "C" {
    pub fn prom_meminit(memsize: u32);
    pub fn prom_identify_arch(machine: u32);
    pub fn prom_init_cmdline(argc: i32, argv: *mut i32, cmdline: u32);
    pub fn register_prom_console();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
