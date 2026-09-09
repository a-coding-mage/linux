/* SPDX-License-Identifier: GPL-2.0 */
// Translated from clk.h. C header guards and includes are intentionally omitted.

use core::ffi::{c_char, c_void};

pub const APBC_NO_BUS_CTRL: u32 = 1 << 0;
pub const APBC_POWER_CTRL: u32 = 1 << 1;

/* Clock type "factor" */
#[repr(C)]
pub struct mmp_clk_factor_masks {
    pub factor: u32,
    pub num_mask: u32,
    pub den_mask: u32,
    pub num_shift: u32,
    pub den_shift: u32,
    pub enable_mask: u32,
}

#[repr(C)]
pub struct mmp_clk_factor {
    pub hw: clk_hw,
    pub base: *mut c_void,
    pub masks: *mut mmp_clk_factor_masks,
    pub ftbl: *mut u32_fract,
    pub ftbl_cnt: u32,
    pub lock: *mut spinlock_t,
}

extern "C" {
    pub fn mmp_clk_register_factor(
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        base: *mut c_void,
        masks: *mut mmp_clk_factor_masks,
        ftbl: *mut u32_fract,
        ftbl_cnt: u32,
        lock: *mut spinlock_t,
    ) -> *mut clk;
}

/* Clock type "mix" */
pub const fn mmp_clk_bits_mask(width: u32, shift: u32) -> u32 {
    ((1u32.wrapping_shl(width)) .wrapping_sub(1)).wrapping_shl(shift)
}
pub const fn mmp_clk_bits_get_val(data: u32, width: u32, shift: u32) -> u32 {
    (data & mmp_clk_bits_mask(width, shift)) >> shift
}
pub const fn mmp_clk_bits_set_val(val: u32, width: u32, shift: u32) -> u32 {
    val.wrapping_shl(shift) & mmp_clk_bits_mask(width, shift)
}

pub const MMP_CLK_MIX_TYPE_V1: u32 = 0;
pub const MMP_CLK_MIX_TYPE_V2: u32 = 1;
pub const MMP_CLK_MIX_TYPE_V3: u32 = 2;

#[repr(C)]
pub struct mmp_clk_mix_reg_info {
    pub reg_clk_ctrl: *mut c_void,
    pub reg_clk_sel: *mut c_void,
    pub width_div: u8,
    pub shift_div: u8,
    pub width_mux: u8,
    pub shift_mux: u8,
    pub bit_fc: u8,
}

#[repr(C)]
pub struct mmp_clk_mix_clk_table {
    pub rate: c_ulong,
    pub parent_index: u8,
    pub divisor: u32,
    pub valid: u32,
}

#[repr(C)]
pub struct mmp_clk_mix_config {
    pub reg_info: mmp_clk_mix_reg_info,
    pub table: *mut mmp_clk_mix_clk_table,
    pub table_size: u32,
    pub mux_table: *mut u32,
    pub div_table: *mut clk_div_table,
    pub div_flags: u8,
    pub mux_flags: u8,
}

#[repr(C)]
pub struct mmp_clk_mix {
    pub hw: clk_hw,
    pub reg_info: mmp_clk_mix_reg_info,
    pub table: *mut mmp_clk_mix_clk_table,
    pub mux_table: *mut u32,
    pub div_table: *mut clk_div_table,
    pub table_size: u32,
    pub div_flags: u8,
    pub mux_flags: u8,
    pub type_: u32,
    pub lock: *mut spinlock_t,
}

extern "C" {
    pub static mmp_clk_mix_ops: clk_ops;
    pub fn mmp_clk_register_mix(dev: *mut device, name: *const c_char,
        parent_names: *const *const c_char, num_parents: u8, flags: c_ulong,
        config: *mut mmp_clk_mix_config, lock: *mut spinlock_t) -> *mut clk;
}

/* Clock type "gate". MMP private gate */
pub const MMP_CLK_GATE_NEED_DELAY: u32 = 1 << 0;

#[repr(C)]
pub struct mmp_clk_gate {
    pub hw: clk_hw,
    pub reg: *mut c_void,
    pub mask: u32,
    pub val_enable: u32,
    pub val_disable: u32,
    pub flags: u32,
    pub lock: *mut spinlock_t,
}

extern "C" {
    pub static mmp_clk_gate_ops: clk_ops;
    pub fn mmp_clk_register_gate(dev: *mut device, name: *const c_char,
        parent_name: *const c_char, flags: c_ulong, reg: *mut c_void,
        mask: u32, val_enable: u32, val_disable: u32, gate_flags: u32,
        lock: *mut spinlock_t) -> *mut clk;
    pub fn mmp_clk_register_apbc(name: *const c_char, parent_name: *const c_char,
        base: *mut c_void, delay: u32, apbc_flags: u32,
        lock: *mut spinlock_t) -> *mut clk;
    pub fn mmp_clk_register_apmu(name: *const c_char, parent_name: *const c_char,
        base: *mut c_void, enable_mask: u32, lock: *mut spinlock_t) -> *mut clk;
}

#[repr(C)]
pub struct mmp_clk_unit {
    pub nr_clks: u32,
    pub clk_table: *mut *mut clk,
    pub clk_data: clk_onecell_data,
}

#[repr(C)]
pub struct mmp_param_fixed_rate_clk {
    pub id: u32, pub name: *mut c_char, pub parent_name: *const c_char,
    pub flags: c_ulong, pub fixed_rate: c_ulong,
}
extern "C" { pub fn mmp_register_fixed_rate_clks(unit: *mut mmp_clk_unit, clks: *mut mmp_param_fixed_rate_clk, size: i32); }

#[repr(C)]
pub struct mmp_param_fixed_factor_clk {
    pub id: u32, pub name: *mut c_char, pub parent_name: *const c_char,
    pub mult: c_ulong, pub div: c_ulong, pub flags: c_ulong,
}
extern "C" { pub fn mmp_register_fixed_factor_clks(unit: *mut mmp_clk_unit, clks: *mut mmp_param_fixed_factor_clk, size: i32); }

#[repr(C)]
pub struct mmp_param_general_gate_clk {
    pub id: u32, pub name: *const c_char, pub parent_name: *const c_char,
    pub flags: c_ulong, pub offset: c_ulong, pub bit_idx: u8,
    pub gate_flags: u8, pub lock: *mut spinlock_t,
}
extern "C" { pub fn mmp_register_general_gate_clks(unit: *mut mmp_clk_unit, clks: *mut mmp_param_general_gate_clk, base: *mut c_void, size: i32); }

#[repr(C)]
pub struct mmp_param_gate_clk {
    pub id: u32, pub name: *mut c_char, pub parent_name: *const c_char,
    pub flags: c_ulong, pub offset: c_ulong, pub mask: u32,
    pub val_enable: u32, pub val_disable: u32, pub gate_flags: u32,
    pub lock: *mut spinlock_t,
}
extern "C" { pub fn mmp_register_gate_clks(unit: *mut mmp_clk_unit, clks: *mut mmp_param_gate_clk, base: *mut c_void, size: i32); }

#[repr(C)]
pub struct mmp_param_mux_clk {
    pub id: u32, pub name: *mut c_char, pub parent_name: *const *const c_char,
    pub num_parents: u8, pub flags: c_ulong, pub offset: c_ulong,
    pub shift: u8, pub width: u8, pub mux_flags: u8, pub lock: *mut spinlock_t,
}
extern "C" { pub fn mmp_register_mux_clks(unit: *mut mmp_clk_unit, clks: *mut mmp_param_mux_clk, base: *mut c_void, size: i32); }

#[repr(C)]
pub struct mmp_param_div_clk {
    pub id: u32, pub name: *mut c_char, pub parent_name: *const c_char,
    pub flags: c_ulong, pub offset: c_ulong, pub shift: u8,
    pub width: u8, pub div_flags: u8, pub lock: *mut spinlock_t,
}
extern "C" { pub fn mmp_register_div_clks(unit: *mut mmp_clk_unit, clks: *mut mmp_param_div_clk, base: *mut c_void, size: i32); }

#[repr(C)]
pub struct mmp_param_pll_clk {
    pub id: u32, pub name: *mut c_char, pub default_rate: c_ulong,
    pub enable_offset: c_ulong, pub enable: u32, pub offset: c_ulong,
    pub shift: u8, pub input_rate: c_ulong, pub postdiv_offset: c_ulong,
    pub postdiv_shift: c_ulong,
}
extern "C" { pub fn mmp_register_pll_clks(unit: *mut mmp_clk_unit, clks: *mut mmp_param_pll_clk, base: *mut c_void, size: i32); }

pub const fn define_mix_reg_info(w_d: u8, s_d: u8, w_m: u8, s_m: u8, fc: u8) -> mmp_clk_mix_reg_info {
    mmp_clk_mix_reg_info { reg_clk_ctrl: core::ptr::null_mut(), reg_clk_sel: core::ptr::null_mut(), width_div: w_d, shift_div: s_d, width_mux: w_m, shift_mux: s_m, bit_fc: fc }
}

extern "C" {
    pub fn mmp_clk_init(np: *mut device_node, unit: *mut mmp_clk_unit, nr_clks: i32);
    pub fn mmp_clk_add(unit: *mut mmp_clk_unit, id: u32, clk: *mut clk);
    pub fn mmp_pm_domain_register(name: *const c_char, reg: *mut c_void,
        power_on: u32, reset: u32, clock_enable: u32, flags: u32,
        lock: *mut spinlock_t) -> *mut generic_pm_domain;
}

pub const MMP_PM_DOMAIN_NO_DISABLE: u32 = 1 << 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
