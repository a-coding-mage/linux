/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright (C) 2013 Broadcom Corporation */
/* Copyright 2013 Linaro Limited */

// Linux dependencies are supplied by the surrounding translation unit.

pub const BILLION: u32 = 1_000_000_000;
pub const PARENT_COUNT_MAX: u32 = u8::MAX as u32;
pub const BAD_CLK_INDEX: u8 = u8::MAX;
pub const BAD_SCALED_DIV_VALUE: u64 = u64::MAX;

#[allow(non_camel_case_types)]
#[repr(C)]
pub enum bcm_clk_type { bcm_clk_none, bcm_clk_bus, bcm_clk_core, bcm_clk_peri }

#[repr(C)]
pub struct bcm_clk_policy { pub offset: u32, pub bit: u32 }

#[repr(C)]
pub struct bcm_clk_gate {
    pub offset: u32, pub status_bit: u32, pub en_bit: u32,
    pub hw_sw_sel_bit: u32, pub flags: u32,
}

pub const BCM_CLK_GATE_FLAGS_EXISTS: u32 = 1 << 0;
pub const BCM_CLK_GATE_FLAGS_HW: u32 = 1 << 1;
pub const BCM_CLK_GATE_FLAGS_SW: u32 = 1 << 2;
pub const BCM_CLK_GATE_FLAGS_NO_DISABLE: u32 = 1 << 3;
pub const BCM_CLK_GATE_FLAGS_SW_MANAGED: u32 = 1 << 4;
pub const BCM_CLK_GATE_FLAGS_ENABLED: u32 = 1 << 5;

#[repr(C)]
pub struct bcm_clk_hyst { pub offset: u32, pub en_bit: u32, pub val_bit: u32 }

#[repr(C)]
pub union bcm_clk_div_u {
    pub s: bcm_clk_div_variable,
    pub fixed: u32,
}
#[repr(C)]
pub struct bcm_clk_div_variable {
    pub offset: u32, pub shift: u32, pub width: u32, pub frac_width: u32,
    pub scaled_div: u64,
}
#[repr(C)]
pub struct bcm_clk_div { pub u: bcm_clk_div_u, pub flags: u32 }

pub const BCM_CLK_DIV_FLAGS_EXISTS: u32 = 1 << 0;
pub const BCM_CLK_DIV_FLAGS_FIXED: u32 = 1 << 1;

#[repr(C)]
pub struct bcm_clk_sel {
    pub offset: u32, pub shift: u32, pub width: u32,
    pub parent_count: u32, pub parent_sel: *mut u32, pub clk_index: u8,
}

#[repr(C)]
pub struct bcm_clk_trig { pub offset: u32, pub bit: u32, pub flags: u32 }
pub const BCM_CLK_TRIG_FLAGS_EXISTS: u32 = 1 << 0;

#[macro_export] macro_rules! gate_exists { ($g:expr) => { flag_test!($g, GATE, EXISTS) }; }
#[macro_export] macro_rules! gate_is_enabled { ($g:expr) => { flag_test!($g, GATE, ENABLED) }; }
#[macro_export] macro_rules! gate_is_hw_controllable { ($g:expr) => { flag_test!($g, GATE, HW) }; }
#[macro_export] macro_rules! gate_is_sw_controllable { ($g:expr) => { flag_test!($g, GATE, SW) }; }
#[macro_export] macro_rules! gate_is_sw_managed { ($g:expr) => { flag_test!($g, GATE, SW_MANAGED) }; }
#[macro_export] macro_rules! gate_is_no_disable { ($g:expr) => { flag_test!($g, GATE, NO_DISABLE) }; }
#[macro_export] macro_rules! gate_flip_enabled { ($g:expr) => { flag_flip!($g, GATE, ENABLED) }; }
#[macro_export] macro_rules! divider_exists { ($d:expr) => { flag_test!($d, DIV, EXISTS) }; }
#[macro_export] macro_rules! divider_is_fixed { ($d:expr) => { flag_test!($d, DIV, FIXED) }; }
#[macro_export] macro_rules! trigger_exists { ($t:expr) => { flag_test!($t, TRIG, EXISTS) }; }
#[macro_export] macro_rules! policy_exists { ($p:expr) => { $p.offset != 0 }; }
#[macro_export] macro_rules! selector_exists { ($s:expr) => { $s.width != 0 }; }
#[macro_export] macro_rules! hyst_exists { ($h:expr) => { $h.offset != 0 }; }

#[macro_export] macro_rules! frac_divider { ($offset:expr, $shift:expr, $width:expr, $frac:expr) => { bcm_clk_div { u: bcm_clk_div_u { s: bcm_clk_div_variable { offset: $offset, shift: $shift, width: $width, frac_width: $frac, scaled_div: BAD_SCALED_DIV_VALUE } }, flags: BCM_CLK_DIV_FLAGS_EXISTS } }; }
#[macro_export] macro_rules! clocks { ($($x:expr),* $(,)?) => { [$($x,)* core::ptr::null()] }; }
#[macro_export] macro_rules! no_clocks { () => { [core::ptr::null()] }; }
#[macro_export] macro_rules! ccu_lvm_en { ($offset:expr, $bit:expr) => { bcm_lvm_en { offset: $offset, bit: $bit } }; }
#[macro_export] macro_rules! ccu_policy_ctl { ($offset:expr, $go:expr, $ac:expr, $atl:expr) => { bcm_policy_ctl { offset: $offset, go_bit: $go, ac_bit: $ac, atl_bit: $atl } }; }

#[repr(C)]
pub struct peri_clk_data {
    pub policy: bcm_clk_policy, pub gate: bcm_clk_gate, pub hyst: bcm_clk_hyst,
    pub pre_trig: bcm_clk_trig, pub pre_div: bcm_clk_div,
    pub trig: bcm_clk_trig, pub div: bcm_clk_div, pub sel: bcm_clk_sel,
    pub clocks: [*const core::ffi::c_char; 0],
}

#[repr(C)]
pub union kona_clk_u { pub data: *mut core::ffi::c_void, pub peri: *mut peri_clk_data }
#[repr(C)]
pub struct kona_clk {
    pub hw: clk_hw, pub init_data: clk_init_data, pub ccu: *mut ccu_data,
    pub type_: bcm_clk_type, pub u: kona_clk_u,
}

#[repr(C)]
pub struct bcm_lvm_en { pub offset: u32, pub bit: u32 }
#[repr(C)]
pub struct bcm_policy_ctl {
    pub offset: u32, pub go_bit: u32, pub atl_bit: u32, pub ac_bit: u32,
}
#[repr(C)]
pub struct ccu_policy { pub enable: bcm_lvm_en, pub control: bcm_policy_ctl }

#[repr(C)]
pub struct ccu_data {
    pub base: *mut core::ffi::c_void, pub lock: spinlock_t, pub write_enabled: bool,
    pub policy: ccu_policy, pub node: *mut device_node, pub clk_num: usize,
    pub name: *const core::ffi::c_char, pub range: u32,
    pub kona_clks: [kona_clk; 0],
}

// Utility macros for object flag management.
#[macro_export] macro_rules! flag { ($ty:ident, $flag:ident) => { concat_idents::concat_idents!(BCM_CLK_, $ty, _FLAGS_, $flag) }; }
#[macro_export] macro_rules! flag_set { ($obj:expr, $ty:ident, $flag:ident) => { $obj.flags |= flag!($ty, $flag) }; }
#[macro_export] macro_rules! flag_clear { ($obj:expr, $ty:ident, $flag:ident) => { $obj.flags &= !flag!($ty, $flag) }; }
#[macro_export] macro_rules! flag_flip { ($obj:expr, $ty:ident, $flag:ident) => { $obj.flags ^= flag!($ty, $flag) }; }
#[macro_export] macro_rules! flag_test { ($obj:expr, $ty:ident, $flag:ident) => { ($obj.flags & flag!($ty, $flag)) != 0 }; }

#[macro_export] macro_rules! policy { ($offset:expr, $bit:expr) => { bcm_clk_policy { offset: $offset, bit: $bit } }; }
#[macro_export] macro_rules! hyst { ($offset:expr, $en_bit:expr, $val_bit:expr) => { bcm_clk_hyst { offset: $offset, en_bit: $en_bit, val_bit: $val_bit } }; }
#[macro_export] macro_rules! fixed_divider { ($value:expr) => { bcm_clk_div { u: bcm_clk_div_u { fixed: $value }, flags: BCM_CLK_DIV_FLAGS_EXISTS | BCM_CLK_DIV_FLAGS_FIXED } }; }
#[macro_export] macro_rules! divider { ($offset:expr, $shift:expr, $width:expr) => { bcm_clk_div { u: bcm_clk_div_u { s: bcm_clk_div_variable { offset: $offset, shift: $shift, width: $width, frac_width: 0, scaled_div: BAD_SCALED_DIV_VALUE } }, flags: BCM_CLK_DIV_FLAGS_EXISTS } }; }
#[macro_export] macro_rules! selector { ($offset:expr, $shift:expr, $width:expr) => { bcm_clk_sel { offset: $offset, shift: $shift, width: $width, parent_count: 0, parent_sel: core::ptr::null_mut(), clk_index: BAD_CLK_INDEX } }; }
#[macro_export] macro_rules! trigger { ($offset:expr, $bit:expr) => { bcm_clk_trig { offset: $offset, bit: $bit, flags: BCM_CLK_TRIG_FLAGS_EXISTS } }; }

extern "C" {
    pub static kona_peri_clk_ops: clk_ops;
    pub fn scaled_div_max(div: *mut bcm_clk_div) -> u64;
    pub fn kona_dt_ccu_setup(ccu: *mut ccu_data, node: *mut device_node);
    pub fn kona_ccu_init(ccu: *mut ccu_data) -> bool;
}

// External kernel types referenced by this header.
extern "C" { type clk_hw; type clk_init_data; type clk_ops; type spinlock_t; type device_node; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
