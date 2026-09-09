// SPDX-License-Identifier: GPL-2.0-only
/*
 *
 *  Copyright (C) 2013 Daniel Tang <tangrs@tangrs.id.au>
 */

// Dependencies supplied by the surrounding kernel/Rust environment.
use core::ffi::{c_char, c_void};

#[repr(C)]
pub struct device_node {
    pub name: *const c_char,
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}

const MHZ: u32 = 1000 * 1000;

const BASE_CPU_SHIFT: u32 = 1;
const BASE_CPU_MASK: u32 = 0x7f;

const CPU_AHB_SHIFT: u32 = 12;
const CPU_AHB_MASK: u32 = 0x07;

const FIXED_BASE_SHIFT: u32 = 8;
const FIXED_BASE_MASK: u32 = 0x01;

const CLASSIC_BASE_SHIFT: u32 = 16;
const CLASSIC_BASE_MASK: u32 = 0x1f;

const CX_BASE_SHIFT: u32 = 15;
const CX_BASE_MASK: u32 = 0x3f;

const CX_UNKNOWN_SHIFT: u32 = 21;
const CX_UNKNOWN_MASK: u32 = 0x03;

#[repr(C)]
pub struct nspire_clk_info {
    pub base_clock: u32,
    pub base_cpu_ratio: u16,
    pub base_ahb_ratio: u16,
}

#[inline]
fn extract(var: u32, shift: u32, mask: u32) -> u32 {
    (var >> shift) & mask
}

unsafe extern "C" {
    fn of_iomap(node: *mut device_node, index: i32) -> *mut c_void;
    fn readl(addr: *mut c_void) -> u32;
    fn iounmap(addr: *mut c_void);
    fn of_property_read_string(
        node: *mut device_node,
        propname: *const c_char,
        out_string: *mut *const c_char,
    ) -> i32;
    fn of_clk_get_parent_name(node: *mut device_node, index: i32) -> *const c_char;
    fn clk_hw_register_fixed_factor(
        dev: *mut c_void,
        name: *const c_char,
        parent_name: *const c_char,
        flags: u32,
        mult: u32,
        div: u32,
    ) -> *mut clk_hw;
    fn clk_hw_register_fixed_rate(
        dev: *mut c_void,
        name: *const c_char,
        parent_name: *const c_char,
        flags: u32,
        rate: u32,
    ) -> *mut clk_hw;
    fn of_clk_add_hw_provider(
        node: *mut device_node,
        get: *const c_void,
        hw: *mut clk_hw,
    ) -> i32;
    fn of_clk_hw_simple_get() -> *mut c_void;
    fn is_err(ptr: *const c_void) -> bool;
    fn pr_info(format: *const c_char, ...);
}

unsafe fn nspire_clkinfo_cx(val: u32, clk: *mut nspire_clk_info) {
    if extract(val, FIXED_BASE_SHIFT, FIXED_BASE_MASK) != 0 {
        (*clk).base_clock = 48 * MHZ;
    } else {
        (*clk).base_clock = 6 * extract(val, CX_BASE_SHIFT, CX_BASE_MASK) * MHZ;
    }

    (*clk).base_cpu_ratio =
        (extract(val, BASE_CPU_SHIFT, BASE_CPU_MASK) * extract(val, CX_UNKNOWN_SHIFT, CX_UNKNOWN_MASK)) as u16;
    (*clk).base_ahb_ratio =
        ((*clk).base_cpu_ratio as u32 * (extract(val, CPU_AHB_SHIFT, CPU_AHB_MASK) + 1)) as u16;
}

unsafe fn nspire_clkinfo_classic(val: u32, clk: *mut nspire_clk_info) {
    if extract(val, FIXED_BASE_SHIFT, FIXED_BASE_MASK) != 0 {
        (*clk).base_clock = 27 * MHZ;
    } else {
        (*clk).base_clock = (300 - 6 * extract(val, CLASSIC_BASE_SHIFT, CLASSIC_BASE_MASK)) * MHZ;
    }

    (*clk).base_cpu_ratio = (extract(val, BASE_CPU_SHIFT, BASE_CPU_MASK) * 2) as u16;
    (*clk).base_ahb_ratio =
        ((*clk).base_cpu_ratio as u32 * (extract(val, CPU_AHB_SHIFT, CPU_AHB_MASK) + 1)) as u16;
}

unsafe fn nspire_ahbdiv_setup(
    node: *mut device_node,
    get_clkinfo: unsafe fn(u32, *mut nspire_clk_info),
) {
    let mut val: u32;
    let io: *mut c_void;
    let hw: *mut clk_hw;
    let mut clk_name = (*node).name;
    let parent_name: *const c_char;
    let mut info = nspire_clk_info { base_clock: 0, base_cpu_ratio: 0, base_ahb_ratio: 0 };

    io = of_iomap(node, 0);
    if io.is_null() { return; }
    val = readl(io);
    iounmap(io);

    get_clkinfo(val, &mut info);
    of_property_read_string(node, b"clock-output-names\0".as_ptr() as *const c_char, &mut clk_name);
    parent_name = of_clk_get_parent_name(node, 0);
    hw = clk_hw_register_fixed_factor(core::ptr::null_mut(), clk_name, parent_name, 0, 1, info.base_ahb_ratio as u32);
    if !is_err(hw as *const c_void) {
        of_clk_add_hw_provider(node, of_clk_hw_simple_get as *const c_void, hw);
    }
}

unsafe fn nspire_ahbdiv_setup_cx(node: *mut device_node) { nspire_ahbdiv_setup(node, nspire_clkinfo_cx); }
unsafe fn nspire_ahbdiv_setup_classic(node: *mut device_node) { nspire_ahbdiv_setup(node, nspire_clkinfo_classic); }

// CLK_OF_DECLARE(nspire_ahbdiv_cx, "lsi,nspire-cx-ahb-divider", nspire_ahbdiv_setup_cx);
// CLK_OF_DECLARE(nspire_ahbdiv_classic, "lsi,nspire-classic-ahb-divider", nspire_ahbdiv_setup_classic);

unsafe fn nspire_clk_setup(
    node: *mut device_node,
    get_clkinfo: unsafe fn(u32, *mut nspire_clk_info),
) {
    let mut val: u32;
    let io: *mut c_void;
    let hw: *mut clk_hw;
    let mut clk_name = (*node).name;
    let mut info = nspire_clk_info { base_clock: 0, base_cpu_ratio: 0, base_ahb_ratio: 0 };

    io = of_iomap(node, 0);
    if io.is_null() { return; }
    val = readl(io);
    iounmap(io);
    get_clkinfo(val, &mut info);
    of_property_read_string(node, b"clock-output-names\0".as_ptr() as *const c_char, &mut clk_name);
    hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), clk_name, core::ptr::null(), 0, info.base_clock);
    if is_err(hw as *const c_void) { return; }
    of_clk_add_hw_provider(node, of_clk_hw_simple_get as *const c_void, hw);
    pr_info(
        b"TI-NSPIRE Base: %uMHz CPU: %uMHz AHB: %uMHz\n\0".as_ptr() as *const c_char,
        info.base_clock / MHZ,
        info.base_clock / info.base_cpu_ratio as u32 / MHZ,
        info.base_clock / info.base_ahb_ratio as u32 / MHZ,
    );
}

unsafe fn nspire_clk_setup_cx(node: *mut device_node) { nspire_clk_setup(node, nspire_clkinfo_cx); }
unsafe fn nspire_clk_setup_classic(node: *mut device_node) { nspire_clk_setup(node, nspire_clkinfo_classic); }

// CLK_OF_DECLARE(nspire_clk_cx, "lsi,nspire-cx-clock", nspire_clk_setup_cx);
// CLK_OF_DECLARE(nspire_clk_classic, "lsi,nspire-classic-clock", nspire_clk_setup_classic);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
