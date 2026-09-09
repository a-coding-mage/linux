/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2013, 2018, The Linux Foundation. All rights reserved. */

/* Dependencies supplied by the surrounding kernel translation. */

#[macro_export]
macro_rules! F {
    ($f:expr, $s:expr, $h:expr, $m:expr, $n:expr) => {
        { ($f), ($s), (2 * ($h) - 1), ($m), ($n) }
    };
}

#[repr(C)]
pub struct freq_tbl {
    pub freq: ::core::ffi::c_ulong,
    pub src: u8,
    pub pre_div: u8,
    pub m: u16,
    pub n: u16,
}

#[macro_export]
macro_rules! C {
    ($s:expr, $h:expr, $m:expr, $n:expr) => {
        { ($s), (2 * ($h) - 1), ($m), ($n) }
    };
}

#[macro_export]
macro_rules! FM {
    ($f:expr, $confs:expr) => {
        { ($f), ($confs).len(), ($confs) }
    };
}

#[macro_export]
macro_rules! FMS {
    ($f:expr, $s:expr, $h:expr, $m:expr, $n:expr) => {
        { ($f), 1, &[C!($s, $h, $m, $n)] }
    };
}

#[repr(C)]
pub struct freq_conf {
    pub src: u8,
    pub pre_div: u8,
    pub m: u16,
    pub n: u16,
}

#[repr(C)]
pub struct freq_multi_tbl {
    pub freq: ::core::ffi::c_ulong,
    pub num_confs: usize,
    pub confs: *const freq_conf,
}

/** struct mn - M/N:D counter */
#[repr(C)]
pub struct mn {
    pub mnctr_en_bit: u8,
    pub mnctr_reset_bit: u8,
    pub mnctr_mode_shift: u8,
    pub n_val_shift: u8,
    pub m_val_shift: u8,
    pub width: u8,
    pub reset_in_cc: bool,
}

pub const MNCTR_MODE_DUAL: u32 = 0x2;
pub const MNCTR_MODE_MASK: u32 = 0x3;

/** struct pre_div - pre-divider */
#[repr(C)]
pub struct pre_div {
    pub pre_div_shift: u8,
    pub pre_div_width: u8,
}

/** struct src_sel - source selector */
#[repr(C)]
pub struct src_sel {
    pub src_sel_shift: u8,
    pub parent_map: *const parent_map,
}

pub const SRC_SEL_MASK: u32 = 0x7;

#[repr(C)]
pub struct clk_rcg {
    pub ns_reg: u32,
    pub md_reg: u32,
    pub mn: mn,
    pub p: pre_div,
    pub s: src_sel,
    pub freq_tbl: *const freq_tbl,
    pub clkr: clk_regmap,
}

extern "C" {
    pub static clk_rcg_ops: clk_ops;
    pub static clk_rcg_floor_ops: clk_ops;
    pub static clk_rcg_bypass_ops: clk_ops;
    pub static clk_rcg_bypass2_ops: clk_ops;
    pub static clk_rcg_pixel_ops: clk_ops;
    pub static clk_rcg_esc_ops: clk_ops;
    pub static clk_rcg_lcc_ops: clk_ops;
}

#[macro_export]
macro_rules! to_clk_rcg {
    ($hw:expr) => { container_of!(to_clk_regmap!($hw), clk_rcg, clkr) };
}

#[repr(C)]
pub struct clk_dyn_rcg {
    pub ns_reg: [u32; 2],
    pub md_reg: [u32; 2],
    pub bank_reg: u32,
    pub mux_sel_bit: u8,
    pub mn: [mn; 2],
    pub p: [pre_div; 2],
    pub s: [src_sel; 2],
    pub freq_tbl: *const freq_tbl,
    pub clkr: clk_regmap,
}

extern "C" { pub static clk_dyn_rcg_ops: clk_ops; }

#[macro_export]
macro_rules! to_clk_dyn_rcg {
    ($hw:expr) => { container_of!(to_clk_regmap!($hw), clk_dyn_rcg, clkr) };
}

#[repr(C)]
pub union clk_rcg2_freq_table {
    pub freq_tbl: *const freq_tbl,
    pub freq_multi_tbl: *const freq_multi_tbl,
}

#[repr(C)]
pub struct clk_rcg2 {
    pub cmd_rcgr: u32,
    pub mnd_width: u8,
    pub hid_width: u8,
    pub safe_src_index: u8,
    pub parent_map: *const parent_map,
    pub freq_table: clk_rcg2_freq_table,
    pub clkr: clk_regmap,
    pub cfg_off: u8,
    pub parked_cfg: u32,
    pub hw_clk_ctrl: bool,
}

#[macro_export]
macro_rules! to_clk_rcg2 {
    ($hw:expr) => { container_of!(to_clk_regmap!($hw), clk_rcg2, clkr) };
}

#[repr(C)]
pub struct clk_rcg2_gfx3d {
    pub div: u8,
    pub rcg: clk_rcg2,
    pub hws: *mut *mut clk_hw,
}

#[macro_export]
macro_rules! to_clk_rcg2_gfx3d {
    ($hw:expr) => { container_of!(to_clk_rcg2!($hw), clk_rcg2_gfx3d, rcg) };
}

extern "C" {
    pub static clk_rcg2_ops: clk_ops;
    pub static clk_rcg2_gp_ops: clk_ops;
    pub static clk_rcg2_floor_ops: clk_ops;
    pub static clk_rcg2_fm_ops: clk_ops;
    pub static clk_rcg2_mux_closest_ops: clk_ops;
    pub static clk_edp_pixel_ops: clk_ops;
    pub static clk_byte_ops: clk_ops;
    pub static clk_byte2_ops: clk_ops;
    pub static clk_pixel_ops: clk_ops;
    pub static clk_gfx3d_ops: clk_ops;
    pub static clk_rcg2_shared_ops: clk_ops;
    pub static clk_rcg2_shared_floor_ops: clk_ops;
    pub static clk_rcg2_shared_no_init_park_ops: clk_ops;
    pub static clk_dp_ops: clk_ops;
}

#[repr(C)]
pub struct clk_rcg_dfs_data {
    pub rcg: *mut clk_rcg2,
    pub init: *mut clk_init_data,
}

#[macro_export]
macro_rules! DEFINE_RCG_DFS {
    ($r:ident) => { clk_rcg_dfs_data { rcg: &mut $r, init: &mut $r##_init } };
}

extern "C" {
    pub fn qcom_cc_register_rcg_dfs(
        regmap: *mut regmap,
        rcgs: *const clk_rcg_dfs_data,
        len: usize,
    ) -> ::core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
