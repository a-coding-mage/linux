/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (c) 2016 Maxime Ripard. All rights reserved. */

// Dependencies supplied by the Linux clock provider and sunxi CCU headers.

#[repr(C)]
pub struct CcuDivInternal {
    pub shift: u8,
    pub width: u8,
    pub max: u32,
    pub offset: u32,
    pub flags: u32,
    pub table: *mut ClkDivTable,
}

#[macro_export]
macro_rules! _SUNXI_CCU_DIV_TABLE_FLAGS {
    ($shift:expr, $width:expr, $table:expr, $flags:expr) => {
        CcuDivInternal { shift: $shift, width: $width, flags: $flags, table: $table, max: 0, offset: 0 }
    };
}
#[macro_export]
macro_rules! _SUNXI_CCU_DIV_TABLE { ($shift:expr, $width:expr, $table:expr) => { _SUNXI_CCU_DIV_TABLE_FLAGS!($shift, $width, $table, 0) }; }
#[macro_export]
macro_rules! _SUNXI_CCU_DIV_OFFSET_MAX_FLAGS {
    ($shift:expr, $width:expr, $off:expr, $max:expr, $flags:expr) => {
        CcuDivInternal { shift: $shift, width: $width, flags: $flags, max: $max, offset: $off, table: core::ptr::null_mut() }
    };
}
#[macro_export]
macro_rules! _SUNXI_CCU_DIV_MAX_FLAGS { ($shift:expr, $width:expr, $max:expr, $flags:expr) => { _SUNXI_CCU_DIV_OFFSET_MAX_FLAGS!($shift, $width, 1, $max, $flags) }; }
#[macro_export]
macro_rules! _SUNXI_CCU_DIV_FLAGS { ($shift:expr, $width:expr, $flags:expr) => { _SUNXI_CCU_DIV_MAX_FLAGS!($shift, $width, 0, $flags) }; }
#[macro_export]
macro_rules! _SUNXI_CCU_DIV_MAX { ($shift:expr, $width:expr, $max:expr) => { _SUNXI_CCU_DIV_MAX_FLAGS!($shift, $width, $max, 0) }; }
#[macro_export]
macro_rules! _SUNXI_CCU_DIV_OFFSET { ($shift:expr, $width:expr, $offset:expr) => { _SUNXI_CCU_DIV_OFFSET_MAX_FLAGS!($shift, $width, $offset, 0, 0) }; }
#[macro_export]
macro_rules! _SUNXI_CCU_DIV { ($shift:expr, $width:expr) => { _SUNXI_CCU_DIV_FLAGS!($shift, $width, 0) }; }

#[repr(C)]
pub struct CcuDiv {
    pub enable: u32,
    pub div: CcuDivInternal,
    pub mux: CcuMuxInternal,
    pub common: CcuCommon,
    pub fixed_post_div: core::ffi::c_uint,
}

// The following constructor macros preserve the original C initializer shapes.
#[macro_export]
macro_rules! SUNXI_CCU_DIV_TABLE_WITH_GATE { ($s:ident,$name:expr,$parent:expr,$reg:expr,$shift:expr,$width:expr,$table:expr,$gate:expr,$flags:expr) => { let $s = CcuDiv { div: _SUNXI_CCU_DIV_TABLE!($shift,$width,$table), enable: $gate, common: CcuCommon { reg: $reg, hw: CLK_HW_INIT!($name,$parent,&ccu_div_ops,$flags), ..Default::default() }, mux: Default::default(), fixed_post_div: 0 }; }; }
#[macro_export]
macro_rules! SUNXI_CCU_DIV_TABLE { ($($args:tt)*) => { SUNXI_CCU_DIV_TABLE_WITH_GATE!($($args)*, 0) }; }
#[macro_export]
macro_rules! SUNXI_CCU_DIV_TABLE_HW { ($s:ident,$name:expr,$parent:expr,$reg:expr,$shift:expr,$width:expr,$table:expr,$flags:expr) => { let $s = CcuDiv { div: _SUNXI_CCU_DIV_TABLE!($shift,$width,$table), common: CcuCommon { reg: $reg, hw: CLK_HW_INIT_HW!($name,$parent,&ccu_div_ops,$flags), ..Default::default() }, enable: 0, mux: Default::default(), fixed_post_div: 0 }; }; }

#[macro_export]
macro_rules! SUNXI_CCU_M_WITH_MUX_TABLE_GATE { ($s:ident,$name:expr,$parents:expr,$table:expr,$reg:expr,$ms:expr,$mw:expr,$xs:expr,$xw:expr,$gate:expr,$flags:expr) => { let $s = CcuDiv { enable:$gate, div:_SUNXI_CCU_DIV!($ms,$mw), mux:_SUNXI_CCU_MUX_TABLE!($xs,$xw,$table), common:CcuCommon { reg:$reg, hw:CLK_HW_INIT_PARENTS!($name,$parents,&ccu_div_ops,$flags), ..Default::default() }, fixed_post_div:0 }; }; }
#[macro_export]
macro_rules! SUNXI_CCU_M_WITH_MUX_TABLE_GATE_CLOSEST { ($s:ident,$name:expr,$parents:expr,$table:expr,$reg:expr,$ms:expr,$mw:expr,$xs:expr,$xw:expr,$gate:expr,$flags:expr) => { let $s = CcuDiv { enable:$gate, div:_SUNXI_CCU_DIV_FLAGS!($ms,$mw,CLK_DIVIDER_ROUND_CLOSEST), mux:_SUNXI_CCU_MUX_TABLE!($xs,$xw,$table), common:CcuCommon { reg:$reg, hw:CLK_HW_INIT_PARENTS!($name,$parents,&ccu_div_ops,$flags), features:CCU_FEATURE_CLOSEST_RATE, ..Default::default() }, fixed_post_div:0 }; }; }

#[macro_export]
macro_rules! SUNXI_CCU_M_WITH_MUX_GATE { ($($a:tt)*) => { SUNXI_CCU_M_WITH_MUX_TABLE_GATE!($($a)*, core::ptr::null_mut(), 0) }; }
#[macro_export]
macro_rules! SUNXI_CCU_M_WITH_MUX_GATE_CLOSEST { ($($a:tt)*) => { SUNXI_CCU_M_WITH_MUX_TABLE_GATE_CLOSEST!($($a)*, core::ptr::null_mut(), 0) }; }
#[macro_export]
macro_rules! SUNXI_CCU_M_WITH_MUX { ($($a:tt)*) => { SUNXI_CCU_M_WITH_MUX_TABLE_GATE!($($a)*, core::ptr::null_mut(), 0, 0) }; }
#[macro_export]
macro_rules! SUNXI_CCU_M_WITH_GATE { ($s:ident,$name:expr,$parent:expr,$reg:expr,$ms:expr,$mw:expr,$gate:expr,$flags:expr) => { let $s = CcuDiv { enable:$gate, div:_SUNXI_CCU_DIV!($ms,$mw), common:CcuCommon { reg:$reg, hw:CLK_HW_INIT!($name,$parent,&ccu_div_ops,$flags), ..Default::default() }, mux:Default::default(), fixed_post_div:0 }; }; }
#[macro_export]
macro_rules! SUNXI_CCU_M { ($($a:tt)*) => { SUNXI_CCU_M_WITH_GATE!($($a)*, 0) }; }
#[macro_export]
macro_rules! SUNXI_CCU_M_DATA_WITH_MUX_GATE { ($s:ident,$name:expr,$parents:expr,$reg:expr,$ms:expr,$mw:expr,$xs:expr,$xw:expr,$gate:expr,$flags:expr) => { let $s = CcuDiv { enable:$gate, div:_SUNXI_CCU_DIV!($ms,$mw), mux:_SUNXI_CCU_MUX!($xs,$xw), common:CcuCommon { reg:$reg, hw:CLK_HW_INIT_PARENTS_DATA!($name,$parents,&ccu_div_ops,$flags), ..Default::default() }, fixed_post_div:0 }; }; }
#[macro_export]
macro_rules! SUNXI_CCU_M_DATA_WITH_MUX { ($($a:tt)*) => { SUNXI_CCU_M_DATA_WITH_MUX_GATE!($($a)*, 0) }; }
#[macro_export]
macro_rules! SUNXI_CCU_M_HW_WITH_MUX_GATE { ($s:ident,$name:expr,$parents:expr,$reg:expr,$ms:expr,$mw:expr,$xs:expr,$xw:expr,$gate:expr,$flags:expr) => { let $s = CcuDiv { enable:$gate, div:_SUNXI_CCU_DIV!($ms,$mw), mux:_SUNXI_CCU_MUX!($xs,$xw), common:CcuCommon { reg:$reg, hw:CLK_HW_INIT_PARENTS_HW!($name,$parents,&ccu_div_ops,$flags), ..Default::default() }, fixed_post_div:0 }; }; }
#[macro_export]
macro_rules! SUNXI_CCU_M_HWS_WITH_GATE { ($s:ident,$name:expr,$parent:expr,$reg:expr,$ms:expr,$mw:expr,$gate:expr,$flags:expr) => { let $s = CcuDiv { enable:$gate, div:_SUNXI_CCU_DIV!($ms,$mw), common:CcuCommon { reg:$reg, hw:CLK_HW_INIT_HWS!($name,$parent,&ccu_div_ops,$flags), ..Default::default() }, mux:Default::default(), fixed_post_div:0 }; }; }
#[macro_export]
macro_rules! SUNXI_CCU_M_HWS { ($($a:tt)*) => { SUNXI_CCU_M_HWS_WITH_GATE!($($a)*, 0) }; }
#[macro_export]
macro_rules! SUNXI_CCU_P_DATA_WITH_MUX_GATE { ($s:ident,$name:expr,$parents:expr,$reg:expr,$ms:expr,$mw:expr,$xs:expr,$xw:expr,$gate:expr,$flags:expr) => { let $s = CcuDiv { enable:$gate, div:_SUNXI_CCU_DIV_FLAGS!($ms,$mw,CLK_DIVIDER_POWER_OF_TWO), mux:_SUNXI_CCU_MUX!($xs,$xw), common:CcuCommon { reg:$reg, hw:CLK_HW_INIT_PARENTS_DATA!($name,$parents,&ccu_div_ops,$flags), ..Default::default() }, fixed_post_div:0 }; }; }

pub unsafe fn hw_to_ccu_div(hw: *mut ClkHw) -> *mut CcuDiv {
    let common = hw_to_ccu_common(hw);
    container_of!(common, CcuDiv, common)
}

unsafe extern "C" {
    pub static ccu_div_ops: ClkOps;
    pub static ccu_rodiv_ops: ClkOps;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
