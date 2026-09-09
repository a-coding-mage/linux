/* SPDX-License-Identifier: GPL-2.0 */
// Rust translation of linux/clk-provider.h.  External kernel types and
// functions are intentionally left as unresolved dependencies.

#![allow(non_camel_case_types, non_snake_case, dead_code)]

pub const CLK_SET_RATE_GATE: u64 = 1 << 0;
pub const CLK_SET_PARENT_GATE: u64 = 1 << 1;
pub const CLK_SET_RATE_PARENT: u64 = 1 << 2;
pub const CLK_IGNORE_UNUSED: u64 = 1 << 3;
pub const CLK_GET_RATE_NOCACHE: u64 = 1 << 6;
pub const CLK_SET_RATE_NO_REPARENT: u64 = 1 << 7;
pub const CLK_GET_ACCURACY_NOCACHE: u64 = 1 << 8;
pub const CLK_RECALC_NEW_RATES: u64 = 1 << 9;
pub const CLK_SET_RATE_UNGATE: u64 = 1 << 10;
pub const CLK_IS_CRITICAL: u64 = 1 << 11;
pub const CLK_OPS_PARENT_ENABLE: u64 = 1 << 12;
pub const CLK_DUTY_CYCLE_PARENT: u64 = 1 << 13;

#[repr(C)] pub struct clk;
#[repr(C)] pub struct clk_core;
#[repr(C)] pub struct device;
#[repr(C)] pub struct device_node;
#[repr(C)] pub struct dentry;
#[repr(C)] pub struct spinlock_t;
#[repr(C)] pub struct of_phandle_args;

pub type u8 = ::core::primitive::u8;
pub type u16 = ::core::primitive::u16;
pub type u32 = ::core::primitive::u32;
pub type ulong = usize;

#[repr(C)] pub struct clk_rate_request {
    pub core: *mut clk_core, pub rate: ulong, pub min_rate: ulong,
    pub max_rate: ulong, pub best_parent_rate: ulong,
    pub best_parent_hw: *mut clk_hw,
}
#[repr(C)] pub struct clk_duty { pub num: u32, pub den: u32 }
#[repr(C)] pub enum clk_ssc_method { CLK_SPREAD_NO, CLK_SPREAD_CENTER, CLK_SPREAD_UP, CLK_SPREAD_DOWN }
#[repr(C)] pub struct clk_spread_spectrum { pub modfreq_hz: u32, pub spread_bp: u32, pub method: clk_ssc_method }

pub type PrepareFn = unsafe extern "C" fn(*mut clk_hw) -> i32;
pub type VoidFn = unsafe extern "C" fn(*mut clk_hw);
pub type RateFn = unsafe extern "C" fn(*mut clk_hw, ulong) -> ulong;
pub type DetermineFn = unsafe extern "C" fn(*mut clk_hw, *mut clk_rate_request) -> i32;
pub type ParentFn = unsafe extern "C" fn(*mut clk_hw, u8) -> i32;
pub type GetParentFn = unsafe extern "C" fn(*mut clk_hw) -> u8;
pub type SetRateFn = unsafe extern "C" fn(*mut clk_hw, ulong, ulong) -> i32;

#[repr(C)] pub struct clk_ops {
    pub prepare: Option<PrepareFn>, pub unprepare: Option<VoidFn>,
    pub is_prepared: Option<PrepareFn>, pub unprepare_unused: Option<VoidFn>,
    pub enable: Option<PrepareFn>, pub disable: Option<VoidFn>,
    pub is_enabled: Option<PrepareFn>, pub disable_unused: Option<VoidFn>,
    pub save_context: Option<PrepareFn>, pub restore_context: Option<VoidFn>,
    pub recalc_rate: Option<RateFn>, pub determine_rate: Option<DetermineFn>,
    pub set_parent: Option<ParentFn>, pub get_parent: Option<GetParentFn>,
    pub set_rate: Option<SetRateFn>, pub set_rate_and_parent: Option<unsafe extern "C" fn(*mut clk_hw, ulong, ulong, u8)->i32>,
    pub set_spread_spectrum: Option<unsafe extern "C" fn(*mut clk_hw, *const clk_spread_spectrum)->i32>,
    pub recalc_accuracy: Option<RateFn>, pub get_phase: Option<PrepareFn>,
    pub set_phase: Option<unsafe extern "C" fn(*mut clk_hw,i32)->i32>,
    pub get_duty_cycle: Option<unsafe extern "C" fn(*mut clk_hw,*mut clk_duty)->i32>,
    pub set_duty_cycle: Option<unsafe extern "C" fn(*mut clk_hw,*mut clk_duty)->i32>,
    pub init: Option<PrepareFn>, pub terminate: Option<VoidFn>,
    pub debug_init: Option<unsafe extern "C" fn(*mut clk_hw,*mut dentry)>,
}
#[repr(C)] pub struct clk_parent_data { pub hw:*const clk_hw, pub fw_name:*const i8, pub name:*const i8, pub index:i32 }
#[repr(C)] pub struct clk_init_data { pub name:*const i8, pub ops:*const clk_ops, pub parent_names:*const *const i8, pub parent_data:*const clk_parent_data, pub parent_hws:*const *const clk_hw, pub num_parents:u8, pub flags:ulong }
#[repr(C)] pub struct clk_hw { pub core:*mut clk_core, pub clk:*mut clk, pub init:*const clk_init_data }
#[repr(C)] pub struct clk_fixed_rate { pub hw:clk_hw, pub fixed_rate:ulong, pub fixed_accuracy:ulong, pub flags:ulong }
#[repr(C)] pub struct clk_gate { pub hw:clk_hw, pub reg:*mut u8, pub bit_idx:u8, pub flags:u8, pub lock:*mut spinlock_t }
#[repr(C)] pub struct clk_div_table { pub val:u32, pub div:u32 }
#[repr(C)] pub struct clk_divider { pub hw:clk_hw, pub reg:*mut u8, pub shift:u8, pub width:u8, pub flags:u16, pub table:*const clk_div_table, pub lock:*mut spinlock_t }
#[repr(C)] pub struct clk_mux { pub hw:clk_hw, pub reg:*mut u8, pub table:*const u32, pub mask:u32, pub shift:u8, pub flags:u8, pub lock:*mut spinlock_t }
#[repr(C)] pub struct clk_fixed_factor { pub hw:clk_hw, pub mult:u32, pub div:u32, pub acc:ulong, pub flags:u32 }
#[repr(C)] pub struct clk_fractional_divider { pub hw:clk_hw, pub reg:*mut u8, pub mshift:u8, pub mwidth:u8, pub nshift:u8, pub nwidth:u8, pub flags:u8, pub approximation:Option<unsafe extern "C" fn(*mut clk_hw,ulong,*mut ulong,*mut ulong,*mut ulong)>, pub lock:*mut spinlock_t }
#[repr(C)] pub struct clk_multiplier { pub hw:clk_hw, pub reg:*mut u8, pub shift:u8, pub width:u8, pub flags:u8, pub lock:*mut spinlock_t }
#[repr(C)] pub struct clk_composite { pub hw:clk_hw, pub ops:clk_ops, pub mux_hw:*mut clk_hw, pub rate_hw:*mut clk_hw, pub gate_hw:*mut clk_hw, pub mux_ops:*const clk_ops, pub rate_ops:*const clk_ops, pub gate_ops:*const clk_ops }
#[repr(C)] pub struct clk_onecell_data { pub clks:*mut *mut clk, pub clk_num:u32 }

pub const CLK_FIXED_RATE_PARENT_ACCURACY:u64=1<<0; pub const CLK_GATE_SET_TO_DISABLE:u8=1<<0; pub const CLK_GATE_HIWORD_MASK:u8=1<<1; pub const CLK_GATE_BIG_ENDIAN:u8=1<<2;
pub const CLK_DIVIDER_ONE_BASED:u64=1<<0; pub const CLK_DIVIDER_POWER_OF_TWO:u64=1<<1; pub const CLK_DIVIDER_ALLOW_ZERO:u64=1<<2; pub const CLK_DIVIDER_HIWORD_MASK:u64=1<<3; pub const CLK_DIVIDER_ROUND_CLOSEST:u64=1<<4; pub const CLK_DIVIDER_READ_ONLY:u64=1<<5; pub const CLK_DIVIDER_MAX_AT_ZERO:u64=1<<6; pub const CLK_DIVIDER_BIG_ENDIAN:u64=1<<7; pub const CLK_DIVIDER_EVEN_INTEGERS:u64=1<<8;
pub const CLK_MUX_INDEX_ONE:u8=1<<0; pub const CLK_MUX_INDEX_BIT:u8=1<<1; pub const CLK_MUX_HIWORD_MASK:u8=1<<2; pub const CLK_MUX_READ_ONLY:u8=1<<3; pub const CLK_MUX_ROUND_CLOSEST:u8=1<<4; pub const CLK_MUX_BIG_ENDIAN:u8=1<<5;
pub const CLK_FIXED_FACTOR_FIXED_ACCURACY:u32=1<<0; pub const CLK_FRAC_DIVIDER_ZERO_BASED:u8=1<<0; pub const CLK_FRAC_DIVIDER_BIG_ENDIAN:u8=1<<1; pub const CLK_FRAC_DIVIDER_POWER_OF_TWO_PS:u8=1<<2; pub const CLK_MULTIPLIER_ZERO_BYPASS:u8=1<<0; pub const CLK_MULTIPLIER_ROUND_CLOSEST:u8=1<<1; pub const CLK_MULTIPLIER_BIG_ENDIAN:u8=1<<2;

// The remaining header content consists of C preprocessor registration macros
// and external kernel API declarations. Their interfaces are preserved below.
extern "C" { pub fn clk_hw_init_rate_request(hw:*const clk_hw, req:*mut clk_rate_request, rate:ulong); pub fn clk_hw_forward_rate_request(core:*const clk_hw, old_req:*const clk_rate_request, parent:*const clk_hw, req:*mut clk_rate_request, parent_rate:ulong); pub fn clk_register(dev:*mut device, hw:*mut clk_hw)->*mut clk; pub fn clk_unregister(clk:*mut clk); pub fn clk_hw_unregister(hw:*mut clk_hw); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
