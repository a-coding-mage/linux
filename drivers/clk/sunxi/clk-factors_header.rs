/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation.

pub const SUNXI_FACTORS_NOT_APPLICABLE: u32 = 0;

#[repr(C)]
pub struct clk_factors_config {
    pub nshift: u8,
    pub nwidth: u8,
    pub kshift: u8,
    pub kwidth: u8,
    pub mshift: u8,
    pub mwidth: u8,
    pub pshift: u8,
    pub pwidth: u8,
    pub n_start: u8,
}

#[repr(C)]
pub struct factors_request {
    pub rate: libc::c_ulong,
    pub parent_rate: libc::c_ulong,
    pub parent_index: u8,
    pub n: u8,
    pub k: u8,
    pub m: u8,
    pub p: u8,
}

#[repr(C)]
pub struct factors_data {
    pub enable: libc::c_int,
    pub mux: libc::c_int,
    pub muxmask: libc::c_int,
    pub table: *const clk_factors_config,
    pub getter: Option<unsafe extern "C" fn(req: *mut factors_request)>,
    pub recalc: Option<unsafe extern "C" fn(req: *mut factors_request)>,
    pub name: *const libc::c_char,
}

#[repr(C)]
pub struct clk_factors {
    pub hw: clk_hw,
    pub reg: *mut libc::c_void,
    pub config: *const clk_factors_config,
    pub get_factors: Option<unsafe extern "C" fn(req: *mut factors_request)>,
    pub recalc: Option<unsafe extern "C" fn(req: *mut factors_request)>,
    pub lock: *mut spinlock_t,
    /* for cleanup */
    pub mux: *mut clk_mux,
    pub gate: *mut clk_gate,
}

unsafe extern "C" {
    pub fn sunxi_factors_register(
        node: *mut device_node,
        data: *const factors_data,
        lock: *mut spinlock_t,
        reg: *mut libc::c_void,
    ) -> *mut clk;

    pub fn sunxi_factors_register_critical(
        node: *mut device_node,
        data: *const factors_data,
        lock: *mut spinlock_t,
        reg: *mut libc::c_void,
    ) -> *mut clk;

    pub fn sunxi_factors_unregister(node: *mut device_node, clk: *mut clk);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
