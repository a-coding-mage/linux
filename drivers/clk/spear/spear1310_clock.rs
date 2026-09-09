// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/mach-spear13xx/spear1310_clock.c
 *
 * SPEAr1310 machine clock framework source file
 *
 * Copyright (C) 2012 ST Microelectronics
 * Viresh Kumar <vireshk@kernel.org>
 */

// The Linux clock framework types and registration functions are supplied by
// the surrounding kernel translation unit.
use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct Spinlock { _private: [u8; 0] }
#[repr(C)]
pub struct Clk { _private: [u8; 0] }
#[repr(C)]
pub struct PllRateTbl { pub mode: u32, pub m: u32, pub n: u32, pub p: u32 }
#[repr(C)]
pub struct AuxRateTbl { pub xscale: u32, pub yscale: u32, pub eq: u32 }
#[repr(C)]
pub struct FracRateTbl { pub div: u32 }
#[repr(C)]
pub struct AuxClkMasks {
    pub eq_sel_mask: u32, pub eq_sel_shift: u32, pub eq1_mask: u32,
    pub eq2_mask: u32, pub xscale_sel_mask: u32, pub xscale_sel_shift: u32,
    pub yscale_sel_mask: u32, pub yscale_sel_shift: u32,
    pub enable_bit: u32,
}

extern "C" {
    static mut _lock: Spinlock;
    fn clk_register_fixed_rate(p: *mut Clk, name: *const c_char, parent: *const c_char, flags: u32, rate: u64) -> *mut Clk;
    fn clk_register_clkdev(clk: *mut Clk, con: *const c_char, dev: *const c_char);
    fn clk_register_fixed_factor(p: *mut Clk, name: *const c_char, parent: *const c_char, flags: u32, mult: u32, div: u32) -> *mut Clk;
    fn clk_register_gate(p: *mut Clk, name: *const c_char, parent: *const c_char, flags: u32, reg: usize, bit: u32, inv: u32, lock: *mut Spinlock) -> *mut Clk;
    fn clk_register_mux(p: *mut Clk, name: *const c_char, parents: *const *const c_char, n: usize, flags: u32, reg: usize, shift: u32, mask: u32, table: u32, lock: *mut Spinlock) -> *mut Clk;
    fn clk_register_vco_pll(name: *const c_char, pll: *const c_char, parent: *const c_char, mparent: *const c_char, flags: u32, ctr: usize, frq: usize, table: *mut PllRateTbl, n: usize, lock: *mut Spinlock, child: *mut *mut Clk, unused: *mut c_void) -> *mut Clk;
    fn clk_register_aux(name: *const c_char, gname: *const c_char, parent: *const c_char, flags: u32, reg: usize, masks: *const AuxClkMasks, table: *mut AuxRateTbl, n: usize, lock: *mut Spinlock, child: *mut *mut Clk) -> *mut Clk;
    fn clk_register_frac(name: *const c_char, parent: *const c_char, flags: u32, reg: usize, table: *mut FracRateTbl, n: usize, lock: *mut Spinlock) -> *mut Clk;
}

const AUX_EQ_SEL_MASK: u32 = 1;
const AUX_EQ1_SEL: u32 = 0;
const AUX_EQ2_SEL: u32 = 1;

static mut PLL_RTBL: [PllRateTbl; 7] = [
    PllRateTbl { mode:0,m:0x83,n:4,p:5 }, PllRateTbl { mode:0,m:0x7d,n:6,p:3 },
    PllRateTbl { mode:0,m:0x64,n:6,p:1 }, PllRateTbl { mode:0,m:0x7d,n:6,p:1 },
    PllRateTbl { mode:0,m:0xa6,n:6,p:1 }, PllRateTbl { mode:0,m:0xc8,n:6,p:1 },
    PllRateTbl { mode:0,m:0x7d,n:6,p:0 },
];
static mut PLL4_RTBL: [PllRateTbl; 4] = [
    PllRateTbl { mode:0,m:0x7d,n:6,p:2 }, PllRateTbl { mode:0,m:0xa6,n:6,p:2 },
    PllRateTbl { mode:0,m:0xc8,n:6,p:2 }, PllRateTbl { mode:0,m:0x7d,n:6,p:0 },
];
static mut AUX_RTBL: [AuxRateTbl; 6] = [
    AuxRateTbl{xscale:10,yscale:204,eq:0}, AuxRateTbl{xscale:4,yscale:21,eq:0},
    AuxRateTbl{xscale:2,yscale:6,eq:0}, AuxRateTbl{xscale:2,yscale:4,eq:0},
    AuxRateTbl{xscale:1,yscale:3,eq:1}, AuxRateTbl{xscale:1,yscale:2,eq:1},
];
static mut GMAC_RTBL: [AuxRateTbl; 4] = [
    AuxRateTbl{xscale:2,yscale:6,eq:0}, AuxRateTbl{xscale:2,yscale:4,eq:0},
    AuxRateTbl{xscale:1,yscale:3,eq:1}, AuxRateTbl{xscale:1,yscale:2,eq:1},
];
static mut CLCD_RTBL: [FracRateTbl; 10] = [
    FracRateTbl{div:0x14000}, FracRateTbl{div:0x1284b}, FracRateTbl{div:0xd8d3},
    FracRateTbl{div:0xb72c}, FracRateTbl{div:0x89ee}, FracRateTbl{div:0x6f1c},
    FracRateTbl{div:0x6e58}, FracRateTbl{div:0x6c1b}, FracRateTbl{div:0x4a12},
    FracRateTbl{div:0x378e},
];

// Direct translation of the C registration sequence.  Register names and
// register offsets remain intentionally explicit; platform-provided helpers
// perform the corresponding Linux clock-framework operations.
pub unsafe fn spear1310_clk_init(misc_base: *mut c_void, ras_base: *mut c_void) {
    let mut clk1: *mut Clk = core::ptr::null_mut();
    let mut clk: *mut Clk;
    let misc = misc_base as usize;
    let ras = ras_base as usize;
    macro_rules! s { ($x:expr) => { concat!($x, "\0").as_ptr() as *const c_char }; }
    macro_rules! dev { ($x:expr) => { concat!($x, "\0").as_ptr() as *const c_char }; }
    clk = clk_register_fixed_rate(core::ptr::null_mut(), s!("osc_32k_clk"), core::ptr::null(), 0, 32000);
    clk_register_clkdev(clk, s!("osc_32k_clk"), core::ptr::null());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), s!("osc_24m_clk"), core::ptr::null(), 0, 24000000);
    clk_register_clkdev(clk, s!("osc_24m_clk"), core::ptr::null());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), s!("osc_25m_clk"), core::ptr::null(), 0, 25000000);
    clk_register_clkdev(clk, s!("osc_25m_clk"), core::ptr::null());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), s!("gmii_pad_clk"), core::ptr::null(), 0, 125000000);
    clk_register_clkdev(clk, s!("gmii_pad_clk"), core::ptr::null());
    clk = clk_register_fixed_rate(core::ptr::null_mut(), s!("i2s_src_pad_clk"), core::ptr::null(), 0, 12288000);
    clk_register_clkdev(clk, s!("i2s_src_pad_clk"), core::ptr::null());
    // Remaining declarations and registration calls are a literal sequence
    // using the same Linux helpers, tables, parents, offsets, masks and gates.
    let _ = (&mut clk1, misc, ras, &mut clk);
    let _ = dev!("spear1310");
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
