// SPDX-License-Identifier: GPL-2.0
/*
 * arch/sh/kernel/cpu/sh4a/ubc.c
 *
 * On-chip UBC support for SH-4A CPUs.
 *
 * Copyright (C) 2009 - 2010  Paul Mundt
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

#[repr(C)]
pub struct arch_hw_breakpoint {
    pub len: u32,
    pub type_: u32,
    pub address: u32,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct sh_ubc {
    pub name: *const i8,
    pub num_events: i32,
    pub trap_nr: i32,
    pub enable: Option<unsafe extern "C" fn(*mut arch_hw_breakpoint, i32)>,
    pub disable: Option<unsafe extern "C" fn(*mut arch_hw_breakpoint, i32)>,
    pub enable_all: Option<unsafe extern "C" fn(usize)>,
    pub disable_all: Option<unsafe extern "C" fn()>,
    pub active_mask: Option<unsafe extern "C" fn() -> usize>,
    pub triggered_mask: Option<unsafe extern "C" fn() -> usize>,
    pub clear_triggered_mask: Option<unsafe extern "C" fn(usize)>,
    pub clk: *mut clk,
}

extern "C" {
    fn __raw_writel(value: u32, address: usize);
    fn __raw_readl(address: usize) -> u32;
    fn clk_get(dev: *mut c_void, name: *const i8) -> *mut clk;
    fn clk_enable(clock: *mut clk);
    fn clk_disable(clock: *mut clk);
    fn register_sh_ubc(ubc: *mut sh_ubc) -> i32;
}

const fn ubc_cbr(idx: usize) -> usize { 0xff20_0000 + (0x20 * idx) }
const fn ubc_crr(idx: usize) -> usize { 0xff20_0004 + (0x20 * idx) }
const fn ubc_car(idx: usize) -> usize { 0xff20_0008 + (0x20 * idx) }
const fn ubc_camr(idx: usize) -> usize { 0xff20_000c + (0x20 * idx) }

const UBC_CCMFR: usize = 0xff20_0600;
const UBC_CBCR: usize = 0xff20_0620;
const UBC_CRR_PCB: u32 = 1 << 1;
const UBC_CRR_BIE: u32 = 1 << 0;
const UBC_CBR_CE: u32 = 1 << 0;

unsafe extern "C" fn sh4a_ubc_enable(info: *mut arch_hw_breakpoint, idx: i32) {
    __raw_writel(UBC_CBR_CE | (*info).len | (*info).type_, ubc_cbr(idx as usize));
    __raw_writel((*info).address, ubc_car(idx as usize));
}

unsafe extern "C" fn sh4a_ubc_disable(_info: *mut arch_hw_breakpoint, idx: i32) {
    __raw_writel(0, ubc_cbr(idx as usize));
    __raw_writel(0, ubc_car(idx as usize));
}

unsafe extern "C" fn sh4a_ubc_enable_all(mask: usize) {
    let mut i = 0;
    while i < sh4a_ubc.num_events {
        if mask & (1usize << i) != 0 {
            __raw_writel(__raw_readl(ubc_cbr(i as usize)) | UBC_CBR_CE, ubc_cbr(i as usize));
        }
        i += 1;
    }
}

unsafe extern "C" fn sh4a_ubc_disable_all() {
    let mut i = 0;
    while i < sh4a_ubc.num_events {
        __raw_writel(__raw_readl(ubc_cbr(i as usize)) & !UBC_CBR_CE, ubc_cbr(i as usize));
        i += 1;
    }
}

unsafe extern "C" fn sh4a_ubc_active_mask() -> usize {
    let mut active = 0usize;
    let mut i = 0;
    while i < sh4a_ubc.num_events {
        if __raw_readl(ubc_cbr(i as usize)) & UBC_CBR_CE != 0 {
            active |= 1usize << i;
        }
        i += 1;
    }
    active
}

unsafe extern "C" fn sh4a_ubc_triggered_mask() -> usize { __raw_readl(UBC_CCMFR) as usize }

unsafe extern "C" fn sh4a_ubc_clear_triggered_mask(mask: usize) {
    __raw_writel(__raw_readl(UBC_CCMFR) & !(mask as u32), UBC_CCMFR);
}

static mut sh4a_ubc: sh_ubc = sh_ubc {
    name: b"SH-4A\0".as_ptr() as *const i8,
    num_events: 2,
    trap_nr: 0x1e0,
    enable: Some(sh4a_ubc_enable),
    disable: Some(sh4a_ubc_disable),
    enable_all: Some(sh4a_ubc_enable_all),
    disable_all: Some(sh4a_ubc_disable_all),
    active_mask: Some(sh4a_ubc_active_mask),
    triggered_mask: Some(sh4a_ubc_triggered_mask),
    clear_triggered_mask: Some(sh4a_ubc_clear_triggered_mask),
    clk: core::ptr::null_mut(),
};

unsafe extern "C" fn sh4a_ubc_init() -> i32 {
    let mut ubc_iclk = clk_get(core::ptr::null_mut(), b"ubc0\0".as_ptr() as *const i8);
    // The UBC MSTP bit is optional; ignore it if it cannot be found.
    if ubc_iclk as usize == usize::MAX { ubc_iclk = core::ptr::null_mut(); }
    clk_enable(ubc_iclk);
    __raw_writel(0, UBC_CBCR);
    let mut i = 0;
    while i < sh4a_ubc.num_events {
        __raw_writel(0, ubc_camr(i as usize));
        __raw_writel(0, ubc_cbr(i as usize));
        __raw_writel(UBC_CRR_BIE | UBC_CRR_PCB, ubc_crr(i as usize));
        let _ = __raw_readl(ubc_crr(i as usize));
        i += 1;
    }
    clk_disable(ubc_iclk);
    sh4a_ubc.clk = ubc_iclk;
    register_sh_ubc(&mut sh4a_ubc)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
