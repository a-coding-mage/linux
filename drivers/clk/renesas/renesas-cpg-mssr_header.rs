/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Renesas Clock Pulse Generator / Module Standby and Software Reset
 *
 * Copyright (C) 2015 Glider bvba
 */

// Dependency: linux/notifier.h

/*
 * Definitions of CPG Core Clocks
 *
 * These include:
 *   - Clock outputs exported to DT
 *   - External input clocks
 *   - Internal CPG clocks
 */

#[repr(C)]
pub union CpgCoreClkParent {
    pub parent_names: *const *const core::ffi::c_char,
    pub dtable: *const ClkDivTable,
}

#[repr(C)]
pub struct CpgCoreClk {
    /* Common */
    pub name: *const core::ffi::c_char,
    pub id: core::ffi::c_uint,
    pub type_: core::ffi::c_uint,
    /* Depending on type */
    pub parent: core::ffi::c_uint, /* Core Clocks only */
    pub div: core::ffi::c_uint,
    pub mult: core::ffi::c_uint,
    pub offset: core::ffi::c_uint,
    pub parent_or_dtable: CpgCoreClkParent,
    pub conf: u32,
    pub flag: u16,
    pub mux_flags: u8,
    pub num_parents: u8,
}

/**
 * struct cpg_mssr_pub - data shared with device-specific clk registration code
 *
 * @base0: CPG/MSSR register block base0 address
 * @base1: CPG/MSSR register block base1 address
 * @notifiers: Notifier chain to save/restore clock state for system resume
 * @rmw_lock: protects RMW register accesses
 * @clks: pointer to clocks
 */
#[repr(C)]
pub struct CpgMssrPub {
    pub base0: *mut core::ffi::c_void,
    pub base1: *mut core::ffi::c_void,
    pub notifiers: RawNotifierHead,
    pub rmw_lock: SpinlockT,
    pub clks: *mut *mut Clk,
}

#[repr(C)]
pub struct RawNotifierHead {
    _opaque: [u8; 0],
}
pub struct SpinlockT { _opaque: [u8; 0] }
pub struct Clk { _opaque: [u8; 0] }
pub struct ClkDivTable { _opaque: [u8; 0] }
pub struct Device { _opaque: [u8; 0] }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ClkTypes {
    /* Generic */
    ClkTypeIn,
    ClkTypeFf,
    ClkTypeDiv6p1,
    ClkTypeDiv6Ro,
    ClkTypeFr,

    /* Custom definitions start here */
    ClkTypeCustom,
}

macro_rules! DEF_TYPE {
    ($name:expr, $id:expr, $type_:expr) => { CpgCoreClk { name: $name, id: $id, type_: $type_, parent: 0, div: 0, mult: 0, offset: 0, parent_or_dtable: CpgCoreClkParent { parent_names: core::ptr::null() }, conf: 0, flag: 0, mux_flags: 0, num_parents: 0 } };
}
macro_rules! DEF_BASE {
    ($name:expr, $id:expr, $type_:expr, $parent:expr) => {{ let mut x = DEF_TYPE!($name, $id, $type_); x.parent = $parent; x }};
}
macro_rules! DEF_INPUT { ($name:expr, $id:expr) => { DEF_TYPE!($name, $id, ClkTypes::ClkTypeIn) }; }
macro_rules! DEF_FIXED { ($name:expr, $id:expr, $parent:expr, $div:expr, $mult:expr) => {{ let mut x = DEF_BASE!($name, $id, ClkTypes::ClkTypeFf, $parent); x.div = $div; x.mult = $mult; x }}; }
macro_rules! DEF_DIV6P1 { ($name:expr, $id:expr, $parent:expr, $offset:expr) => {{ let mut x = DEF_BASE!($name, $id, ClkTypes::ClkTypeDiv6p1, $parent); x.offset = $offset; x }}; }
macro_rules! DEF_DIV6_RO { ($name:expr, $id:expr, $parent:expr, $offset:expr, $div:expr) => {{ let mut x = DEF_BASE!($name, $id, ClkTypes::ClkTypeDiv6Ro, $parent); x.offset = $offset; x.div = $div; x.mult = 1; x }}; }
macro_rules! DEF_RATE { ($name:expr, $id:expr, $rate:expr) => {{ let mut x = DEF_TYPE!($name, $id, ClkTypes::ClkTypeFr); x.mult = $rate; x }}; }

/* Definitions of Module Clocks */
#[repr(C)]
pub struct MssrModClk {
    pub name: *const core::ffi::c_char,
    pub id: core::ffi::c_uint,
    pub parent: core::ffi::c_uint, /* Add MOD_CLK_BASE for Module Clocks */
}

/* Convert from sparse base-100 to packed index space */
#[inline] pub const fn mod_clk_pack(x: core::ffi::c_uint) -> core::ffi::c_uint { x - (x / 100) * (100 - 32) }
#[inline] pub const fn mod_clk_id(x: core::ffi::c_uint) -> core::ffi::c_uint { MOD_CLK_BASE + mod_clk_pack(x) }
macro_rules! DEF_MOD { ($name:expr, $mod_:expr, $parent:expr) => { MssrModClk { name: $name, id: mod_clk_id($mod_), parent: $parent } }; }

/* Convert from sparse base-10 to packed index space */
#[inline] pub const fn mod_clk_pack_10(x: core::ffi::c_uint) -> core::ffi::c_uint { (x / 10) * 32 + (x % 10) }
#[inline] pub const fn mod_clk_id_10(x: core::ffi::c_uint) -> core::ffi::c_uint { MOD_CLK_BASE + mod_clk_pack_10(x) }
macro_rules! DEF_MOD_STB { ($name:expr, $mod_:expr, $parent:expr) => { MssrModClk { name: $name, id: mod_clk_id_10($mod_), parent: $parent } }; }

pub enum DeviceNode {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ClkRegLayout {
    ClkRegLayoutRcarGen2AndGen3 = 0,
    ClkRegLayoutRzA,
    ClkRegLayoutRcarGen4,
    ClkRegLayoutRzT2h,
}

#[repr(C)]
pub struct CpgMssrInfo {
    pub early_core_clks: *const CpgCoreClk,
    pub num_early_core_clks: core::ffi::c_uint,
    pub early_mod_clks: *const MssrModClk,
    pub num_early_mod_clks: core::ffi::c_uint,
    pub core_clks: *const CpgCoreClk,
    pub num_core_clks: core::ffi::c_uint,
    pub last_dt_core_clk: core::ffi::c_uint,
    pub num_total_core_clks: core::ffi::c_uint,
    pub reg_layout: ClkRegLayout,
    pub mod_clks: *const MssrModClk,
    pub num_mod_clks: core::ffi::c_uint,
    pub num_hw_mod_clks: core::ffi::c_uint,
    pub crit_mod_clks: *const core::ffi::c_uint,
    pub num_crit_mod_clks: core::ffi::c_uint,
    pub core_pm_clks: *const core::ffi::c_uint,
    pub num_core_pm_clks: core::ffi::c_uint,
    pub init: Option<unsafe extern "C" fn(*mut Device) -> core::ffi::c_int>,
    pub cpg_clk_register: Option<unsafe extern "C" fn(*mut Device, *const CpgCoreClk, *const CpgMssrInfo, *mut CpgMssrPub) -> *mut Clk>,
}

extern "C" {
    pub static r7s9210_cpg_mssr_info: CpgMssrInfo;
    pub static r8a7742_cpg_mssr_info: CpgMssrInfo;
    pub static r8a7743_cpg_mssr_info: CpgMssrInfo;
    pub static r8a7745_cpg_mssr_info: CpgMssrInfo;
    pub static r8a77470_cpg_mssr_info: CpgMssrInfo;
    pub static r8a774a1_cpg_mssr_info: CpgMssrInfo;
    pub static r8a774b1_cpg_mssr_info: CpgMssrInfo;
    pub static r8a774c0_cpg_mssr_info: CpgMssrInfo;
    pub static r8a774e1_cpg_mssr_info: CpgMssrInfo;
    pub static r8a7790_cpg_mssr_info: CpgMssrInfo;
    pub static r8a7791_cpg_mssr_info: CpgMssrInfo;
    pub static r8a7792_cpg_mssr_info: CpgMssrInfo;
    pub static r8a7794_cpg_mssr_info: CpgMssrInfo;
    pub static r8a7795_cpg_mssr_info: CpgMssrInfo;
    pub static r8a7796_cpg_mssr_info: CpgMssrInfo;
    pub static r8a77965_cpg_mssr_info: CpgMssrInfo;
    pub static r8a77970_cpg_mssr_info: CpgMssrInfo;
    pub static r8a77980_cpg_mssr_info: CpgMssrInfo;
    pub static r8a77990_cpg_mssr_info: CpgMssrInfo;
    pub static r8a77995_cpg_mssr_info: CpgMssrInfo;
    pub static r8a779a0_cpg_mssr_info: CpgMssrInfo;
    pub static r8a779f0_cpg_mssr_info: CpgMssrInfo;
    pub static r8a779g0_cpg_mssr_info: CpgMssrInfo;
    pub static r8a779h0_cpg_mssr_info: CpgMssrInfo;
    pub static r9a09g077_cpg_mssr_info: CpgMssrInfo;

    pub fn cpg_mssr_early_init(np: *mut DeviceNode, info: *const CpgMssrInfo);
    pub fn mssr_mod_nullify(mod_clks: *mut MssrModClk, num_mod_clks: core::ffi::c_uint, clks: *const core::ffi::c_uint, n: core::ffi::c_uint);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
