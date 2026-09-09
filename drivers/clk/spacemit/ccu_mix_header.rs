/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2024 SpacemiT Technology Co. Ltd
 * Copyright (c) 2024-2025 Haylen Chu <heylenay@4d2.org>
 */

// Linux clock-provider and ccu_common dependencies are supplied externally.

#[repr(C)]
pub struct CcuGateConfig {
    pub mask: u32,
    pub inverted: bool,
}

#[repr(C)]
pub struct CcuFactorConfig {
    pub div: u32,
    pub mul: u32,
}

#[repr(C)]
pub struct CcuMuxConfig {
    pub shift: u8,
    pub width: u8,
}

#[repr(C)]
pub struct CcuDivConfig {
    pub shift: u8,
    pub width: u8,
}

#[repr(C)]
pub struct CcuMix {
    pub factor: CcuFactorConfig,
    pub gate: CcuGateConfig,
    pub div: CcuDivConfig,
    pub mux: CcuMuxConfig,
    pub common: CcuCommon,
}

#[macro_export]
macro_rules! CCU_GATE_INIT { ($mask:expr) => { CcuGateConfig { mask: $mask, inverted: false } }; }
#[macro_export]
macro_rules! CCU_FACTOR_INIT { ($div:expr, $mul:expr) => { CcuFactorConfig { div: $div, mul: $mul } }; }
#[macro_export]
macro_rules! CCU_MUX_INIT { ($shift:expr, $width:expr) => { CcuMuxConfig { shift: $shift, width: $width } }; }
#[macro_export]
macro_rules! CCU_DIV_INIT { ($shift:expr, $width:expr) => { CcuDivConfig { shift: $shift, width: $width } }; }
#[macro_export]
macro_rules! CCU_GATE_FLAGS_INIT { ($mask:expr, $inverted:expr) => { CcuGateConfig { mask: $mask, inverted: $inverted } }; }

#[macro_export]
macro_rules! CCU_PARENT_HW { ($parent:expr) => { ClkParentData { hw: &$parent.common.hw } }; }
#[macro_export]
macro_rules! CCU_PARENT_NAME { ($name:ident) => { ClkParentData { fw_name: stringify!($name) } }; }

// The following definition macros preserve the corresponding C initializers.
#[macro_export]
macro_rules! CCU_MIX_INITHW { ($name:ident, $parent:expr, $ops:expr, $flags:expr) => {
    hw: ClkHw { init: &ClkInitData { flags: $flags, name: stringify!($name), parent_data: &[CCU_PARENT_HW!($parent)], num_parents: 1, ops: &$ops } }
}; }
#[macro_export]
macro_rules! CCU_MIX_INITHW_PARENTS { ($name:ident, $parents:expr, $ops:expr, $flags:expr) => {
    hw: ClkHw { init: &clk_hw_init_parents_data(stringify!($name), $parents, &$ops, $flags) }
}; }

#[macro_export]
macro_rules! CCU_GATE_DEFINE { ($name:ident, $parent:expr, $reg_ctrl:expr, $mask_gate:expr, $flags:expr) => {
    static mut $name: CcuMix = CcuMix { gate: CCU_GATE_INIT!($mask_gate), common: CcuCommon { reg_ctrl: $reg_ctrl, CCU_MIX_INITHW!($name, $parent, spacemit_ccu_gate_ops, $flags) } };
}; }
#[macro_export]
macro_rules! CCU_FACTOR_DEFINE { ($name:ident, $parent:expr, $div:expr, $mul:expr) => {
    static mut $name: CcuMix = CcuMix { factor: CCU_FACTOR_INIT!($div, $mul), common: CcuCommon { CCU_MIX_INITHW!($name, $parent, spacemit_ccu_factor_ops, 0) } };
}; }
#[macro_export]
macro_rules! CCU_MUX_DEFINE { ($name:ident, $parents:expr, $reg_ctrl:expr, $shift:expr, $width:expr, $flags:expr) => {
    static mut $name: CcuMix = CcuMix { mux: CCU_MUX_INIT!($shift, $width), common: CcuCommon { reg_ctrl: $reg_ctrl, CCU_MIX_INITHW_PARENTS!($name, $parents, spacemit_ccu_mux_ops, $flags) } };
}; }
#[macro_export]
macro_rules! CCU_DIV_DEFINE { ($name:ident, $parent:expr, $reg_ctrl:expr, $shift:expr, $width:expr, $flags:expr) => {
    static mut $name: CcuMix = CcuMix { div: CCU_DIV_INIT!($shift, $width), common: CcuCommon { reg_ctrl: $reg_ctrl, CCU_MIX_INITHW!($name, $parent, spacemit_ccu_div_ops, $flags) } };
}; }
#[macro_export]
macro_rules! CCU_GATE_FLAGS_DEFINE { ($name:ident, $parent:expr, $reg_ctrl:expr, $mask:expr, $inv:expr, $flags:expr) => {
    static mut $name: CcuMix = CcuMix { gate: CCU_GATE_FLAGS_INIT!($mask, $inv), common: CcuCommon { reg_ctrl: $reg_ctrl, CCU_MIX_INITHW!($name, $parent, spacemit_ccu_gate_ops, $flags) } };
}; }
#[macro_export]
macro_rules! CCU_FACTOR_GATE_FLAGS_DEFINE { ($name:ident, $parent:expr, $reg_ctrl:expr, $mask:expr, $div:expr, $mul:expr, $flags:expr) => {
    static mut $name: CcuMix { gate: CCU_GATE_INIT!($mask), factor: CCU_FACTOR_INIT!($div, $mul), common: CcuCommon { reg_ctrl: $reg_ctrl, CCU_MIX_INITHW!($name, $parent, spacemit_ccu_factor_gate_ops, $flags) } }
}; }
#[macro_export]
macro_rules! CCU_FACTOR_GATE_DEFINE { ($name:ident, $parent:expr, $reg_ctrl:expr, $mask:expr, $div:expr, $mul:expr) => { CCU_FACTOR_GATE_FLAGS_DEFINE!($name, $parent, $reg_ctrl, $mask, $div, $mul, 0) }; }
#[macro_export]
macro_rules! CCU_MUX_GATE_DEFINE { ($name:ident, $parents:expr, $reg_ctrl:expr, $shift:expr, $width:expr, $mask:expr, $flags:expr) => {
    static mut $name: CcuMix = CcuMix { gate: CCU_GATE_INIT!($mask), mux: CCU_MUX_INIT!($shift, $width), common: CcuCommon { reg_ctrl: $reg_ctrl, CCU_MIX_INITHW_PARENTS!($name, $parents, spacemit_ccu_mux_gate_ops, $flags) } }
}; }
#[macro_export]
macro_rules! CCU_DIV_GATE_DEFINE { ($name:ident, $parent:expr, $reg_ctrl:expr, $shift:expr, $width:expr, $mask:expr, $flags:expr) => {
    static mut $name: CcuMix = CcuMix { gate: CCU_GATE_INIT!($mask), div: CCU_DIV_INIT!($shift, $width), common: CcuCommon { reg_ctrl: $reg_ctrl, CCU_MIX_INITHW!($name, $parent, spacemit_ccu_div_gate_ops, $flags) } }
}; }
#[macro_export]
macro_rules! CCU_MUX_DIV_GATE_DEFINE { ($name:ident, $parents:expr, $reg_ctrl:expr, $mshift:expr, $mwidth:expr, $muxshift:expr, $muxwidth:expr, $mask:expr, $flags:expr) => {
    static mut $name: CcuMix { gate: CCU_GATE_INIT!($mask), div: CCU_DIV_INIT!($mshift, $mwidth), mux: CCU_MUX_INIT!($muxshift, $muxwidth), common: CcuCommon { reg_ctrl: $reg_ctrl, CCU_MIX_INITHW_PARENTS!($name, $parents, spacemit_ccu_mux_div_gate_ops, $flags) } }
}; }
#[macro_export]
macro_rules! CCU_MUX_DIV_GATE_SPLIT_FC_DEFINE { ($name:ident, $parents:expr, $reg_ctrl:expr, $reg_fc:expr, $mshift:expr, $mwidth:expr, $mask_fc:expr, $muxshift:expr, $muxwidth:expr, $mask:expr, $flags:expr) => {
    static mut $name: CcuMix { gate: CCU_GATE_INIT!($mask), div: CCU_DIV_INIT!($mshift, $mwidth), mux: CCU_MUX_INIT!($muxshift, $muxwidth), common: CcuCommon { reg_ctrl: $reg_ctrl, reg_fc: $reg_fc, mask_fc: $mask_fc, CCU_MIX_INITHW_PARENTS!($name, $parents, spacemit_ccu_mux_div_gate_ops, $flags) } }
}; }
#[macro_export]
macro_rules! CCU_MUX_DIV_GATE_FC_DEFINE { ($($arg:tt)*) => { CCU_MUX_DIV_GATE_SPLIT_FC_DEFINE!($($arg)*) }; }
#[macro_export]
macro_rules! CCU_MUX_DIV_FC_DEFINE { ($name:ident, $parents:expr, $reg:expr, $mshift:expr, $mwidth:expr, $mask_fc:expr, $muxshift:expr, $muxwidth:expr, $flags:expr) => {
    static mut $name: CcuMix { div: CCU_DIV_INIT!($mshift, $mwidth), mux: CCU_MUX_INIT!($muxshift, $muxwidth), common: CcuCommon { reg_ctrl: $reg, reg_fc: $reg, mask_fc: $mask_fc, CCU_MIX_INITHW_PARENTS!($name, $parents, spacemit_ccu_mux_div_ops, $flags) } }
}; }
#[macro_export]
macro_rules! CCU_MUX_FC_DEFINE { ($name:ident, $parents:expr, $reg:expr, $mask_fc:expr, $shift:expr, $width:expr, $flags:expr) => {
    static mut $name: CcuMix { mux: CCU_MUX_INIT!($shift, $width), common: CcuCommon { reg_ctrl: $reg, reg_fc: $reg, mask_fc: $mask_fc, CCU_MIX_INITHW_PARENTS!($name, $parents, spacemit_ccu_mux_ops, $flags) } }
}; }

extern "C" {
    pub static spacemit_ccu_gate_ops: ClkOps;
    pub static spacemit_ccu_factor_ops: ClkOps;
    pub static spacemit_ccu_mux_ops: ClkOps;
    pub static spacemit_ccu_div_ops: ClkOps;
    pub static spacemit_ccu_factor_gate_ops: ClkOps;
    pub static spacemit_ccu_div_gate_ops: ClkOps;
    pub static spacemit_ccu_mux_gate_ops: ClkOps;
    pub static spacemit_ccu_mux_div_ops: ClkOps;
    pub static spacemit_ccu_mux_div_gate_ops: ClkOps;
}

pub unsafe fn hw_to_ccu_mix(hw: *mut ClkHw) -> *mut CcuMix {
    let common = hw_to_ccu_common(hw);
    (common as *mut u8).sub(core::mem::offset_of!(CcuMix, common)) as *mut CcuMix
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
