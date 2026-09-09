// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright 2014 Chen-Yu Tsai
 *
 * Chen-Yu Tsai <wens@csie.org>
 */

// Dependencies supplied by the surrounding kernel translation.

#[repr(C)]
pub struct factors_request {
    pub rate: u32,
    pub parent_rate: u32,
    pub n: u32,
    pub m: u32,
    pub p: u32,
}

#[repr(C)]
pub struct clk_factors_config {
    pub mshift: u32,
    pub mwidth: u32,
    pub nshift: u32,
    pub nwidth: u32,
    pub pshift: u32,
    pub pwidth: u32,
}

#[repr(C)]
pub struct factors_data {
    pub enable: u32,
    pub mux: u32,
    pub muxmask: u32,
    pub table: *const clk_factors_config,
    pub getter: unsafe extern "C" fn(*mut factors_request),
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spinlock {
    _private: [u8; 0],
}

extern "C" {
    fn of_io_request_and_map(node: *mut device_node, index: i32, name: *const i8) -> *mut core::ffi::c_void;
    fn of_node_full_name(node: *mut device_node) -> *const i8;
    fn sunxi_factors_register(
        node: *mut device_node,
        data: *const factors_data,
        lock: *mut spinlock,
        reg: *mut core::ffi::c_void,
    );
    fn sunxi_factors_register_critical(
        node: *mut device_node,
        data: *const factors_data,
        lock: *mut spinlock,
        reg: *mut core::ffi::c_void,
    );
    fn pr_err(format: *const i8, ...);
}

unsafe extern "C" fn sun9i_a80_get_pll4_factors(req: *mut factors_request) {
    let req = &mut *req;
    let mut n: i32;
    let mut m: i32 = 1;
    let mut p: i32 = 1;

    // Normalize value to a 6 MHz multiple (24 MHz / 4)
    n = ((req.rate + 6_000_000 - 1) / 6_000_000) as i32;

    // If n is too large switch to steps of 12 MHz
    if n > 255 {
        m = 0;
        n = (n + 1) / 2;
    }

    // If n is still too large switch to steps of 24 MHz
    if n > 255 {
        p = 0;
        n = (n + 1) / 2;
    }

    // n must be between 12 and 255
    if n > 255 {
        n = 255;
    } else if n < 12 {
        n = 12;
    }

    req.rate = (((24_000_000i32 * n) >> p) / (m + 1)) as u32;
    req.n = n as u32;
    req.m = m as u32;
    req.p = p as u32;
}

static sun9i_a80_pll4_config: clk_factors_config = clk_factors_config {
    mshift: 18, mwidth: 1, nshift: 8, nwidth: 8, pshift: 16, pwidth: 1,
};

static sun9i_a80_pll4_data: factors_data = factors_data {
    enable: 31, mux: 0, muxmask: 0, table: &sun9i_a80_pll4_config,
    getter: sun9i_a80_get_pll4_factors,
};
static mut sun9i_a80_pll4_lock: spinlock = spinlock { _private: [] };

unsafe extern "C" fn sun9i_a80_pll4_setup(node: *mut device_node) {
    let reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if reg.is_null() {
        pr_err(b"Could not get registers for a80-pll4-clk: %pOFn\0".as_ptr() as *const i8, node);
        return;
    }
    sunxi_factors_register(node, &sun9i_a80_pll4_data, &mut sun9i_a80_pll4_lock, reg);
}

unsafe extern "C" fn sun9i_a80_get_gt_factors(req: *mut factors_request) {
    let req = &mut *req;
    if req.parent_rate < req.rate { req.rate = req.parent_rate; }
    let mut div = (req.parent_rate + req.rate - 1) / req.rate;
    if div > 4 { div = 4; }
    req.rate = req.parent_rate / div;
    req.m = div;
}

static sun9i_a80_gt_config: clk_factors_config = clk_factors_config { mshift: 0, mwidth: 2, nshift: 0, nwidth: 0, pshift: 0, pwidth: 0 };
static sun9i_a80_gt_data: factors_data = factors_data { enable: 0, mux: 24, muxmask: (1 << 1) | (1 << 0), table: &sun9i_a80_gt_config, getter: sun9i_a80_get_gt_factors };
static mut sun9i_a80_gt_lock: spinlock = spinlock { _private: [] };

unsafe extern "C" fn sun9i_a80_gt_setup(node: *mut device_node) {
    let reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if reg.is_null() { pr_err(b"Could not get registers for a80-gt-clk: %pOFn\0".as_ptr() as *const i8, node); return; }
    sunxi_factors_register_critical(node, &sun9i_a80_gt_data, &mut sun9i_a80_gt_lock, reg);
}

unsafe extern "C" fn sun9i_a80_get_ahb_factors(req: *mut factors_request) {
    let req = &mut *req;
    if req.parent_rate < req.rate { req.rate = req.parent_rate; }
    let mut p = (32 - ((req.parent_rate + req.rate - 1) / req.rate).leading_zeros()) as u32;
    if p > 3 { p = 3; }
    req.rate = req.parent_rate >> p;
    req.p = p;
}

static sun9i_a80_ahb_config: clk_factors_config = clk_factors_config { mshift: 0, mwidth: 0, nshift: 0, nwidth: 0, pshift: 0, pwidth: 2 };
static sun9i_a80_ahb_data: factors_data = factors_data { enable: 0, mux: 24, muxmask: 3, table: &sun9i_a80_ahb_config, getter: sun9i_a80_get_ahb_factors };
static mut sun9i_a80_ahb_lock: spinlock = spinlock { _private: [] };

unsafe extern "C" fn sun9i_a80_ahb_setup(node: *mut device_node) {
    let reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if reg.is_null() { pr_err(b"Could not get registers for a80-ahb-clk: %pOFn\0".as_ptr() as *const i8, node); return; }
    sunxi_factors_register(node, &sun9i_a80_ahb_data, &mut sun9i_a80_ahb_lock, reg);
}

static sun9i_a80_apb0_data: factors_data = factors_data { enable: 0, mux: 24, muxmask: 1, table: &sun9i_a80_ahb_config, getter: sun9i_a80_get_ahb_factors };
static mut sun9i_a80_apb0_lock: spinlock = spinlock { _private: [] };
unsafe extern "C" fn sun9i_a80_apb0_setup(node: *mut device_node) {
    let reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if reg.is_null() { pr_err(b"Could not get registers for a80-apb0-clk: %pOFn\0".as_ptr() as *const i8, node); return; }
    sunxi_factors_register(node, &sun9i_a80_apb0_data, &mut sun9i_a80_apb0_lock, reg);
}

unsafe extern "C" fn sun9i_a80_get_apb1_factors(req: *mut factors_request) {
    let req = &mut *req;
    if req.parent_rate < req.rate { req.rate = req.parent_rate; }
    let mut div = (req.parent_rate + req.rate - 1) / req.rate;
    if div > 256 { div = 256; }
    req.p = 32 - div.leading_zeros();
    req.m = (req.parent_rate >> req.p) - 1;
    req.rate = (req.parent_rate >> req.p) / (req.m + 1);
}

static sun9i_a80_apb1_config: clk_factors_config = clk_factors_config { mshift: 0, mwidth: 5, nshift: 0, nwidth: 0, pshift: 16, pwidth: 2 };
static sun9i_a80_apb1_data: factors_data = factors_data { enable: 0, mux: 24, muxmask: 1, table: &sun9i_a80_apb1_config, getter: sun9i_a80_get_apb1_factors };
static mut sun9i_a80_apb1_lock: spinlock = spinlock { _private: [] };
unsafe extern "C" fn sun9i_a80_apb1_setup(node: *mut device_node) {
    let reg = of_io_request_and_map(node, 0, of_node_full_name(node));
    if reg.is_null() { pr_err(b"Could not get registers for a80-apb1-clk: %pOFn\0".as_ptr() as *const i8, node); return; }
    sunxi_factors_register(node, &sun9i_a80_apb1_data, &mut sun9i_a80_apb1_lock, reg);
}

// CLK_OF_DECLARE registrations:
// sun9i_a80_pll4, "allwinner,sun9i-a80-pll4-clk", sun9i_a80_pll4_setup
// sun9i_a80_gt, "allwinner,sun9i-a80-gt-clk", sun9i_a80_gt_setup
// sun9i_a80_ahb, "allwinner,sun9i-a80-ahb-clk", sun9i_a80_ahb_setup
// sun9i_a80_apb0, "allwinner,sun9i-a80-apb0-clk", sun9i_a80_apb0_setup
// sun9i_a80_apb1, "allwinner,sun9i-a80-apb1-clk", sun9i_a80_apb1_setup

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
