/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) STMicroelectronics 2022 - All Rights Reserved
 * Author: Gabriel Fernandez <gabriel.fernandez@foss.st.com> for STMicroelectronics.
 */

/* Dependency supplied by the Linux clock-provider headers. */

#[repr(C)]
pub struct stm32_mux_cfg {
    pub offset: u16,
    pub shift: u8,
    pub width: u8,
    pub flags: u8,
    pub table: *mut u32,
    pub ready: u8,
}

#[repr(C)]
pub struct stm32_gate_cfg {
    pub offset: u16,
    pub bit_idx: u8,
    pub set_clr: u8,
}

#[repr(C)]
pub struct stm32_div_cfg {
    pub offset: u16,
    pub shift: u8,
    pub width: u8,
    pub flags: u8,
    pub ready: u8,
    pub table: *const clk_div_table,
}

#[repr(C)]
pub struct stm32_composite_cfg {
    pub mux: i32,
    pub gate: i32,
    pub div: i32,
}

pub const NO_ID: u32 = 0xFFFF_FFFF;
pub const NO_STM32_MUX: u16 = 0xFFFF;
pub const NO_STM32_DIV: u16 = 0xFFFF;
pub const NO_STM32_GATE: u16 = 0xFFFF;

#[repr(C)]
pub struct clock_config {
    pub id: usize,
    pub sec_id: i32,
    pub clock_cfg: *mut core::ffi::c_void,
    pub func: Option<unsafe extern "C" fn(
        dev: *mut device,
        data: *const stm32_rcc_match_data,
        base: *mut core::ffi::c_void,
        lock: *mut spinlock_t,
        cfg: *const clock_config,
    ) -> *mut clk_hw>,
}

#[repr(C)]
pub struct clk_stm32_clock_data {
    pub gate_cpt: *mut u16,
    pub gates: *const stm32_gate_cfg,
    pub muxes: *const stm32_mux_cfg,
    pub dividers: *const stm32_div_cfg,
    pub is_multi_mux: Option<unsafe extern "C" fn(*mut clk_hw) -> *mut clk_hw>,
}

#[repr(C)]
pub struct stm32_rcc_match_data {
    pub hw_clks: *mut clk_hw_onecell_data,
    pub num_clocks: u32,
    pub tab_clocks: *const clock_config,
    pub maxbinding: u32,
    pub clock_data: *mut clk_stm32_clock_data,
    pub reset_data: *mut clk_stm32_reset_data,
    pub check_security: Option<unsafe extern "C" fn(*mut device_node, *mut core::ffi::c_void, *const clock_config) -> i32>,
    pub multi_mux: Option<unsafe extern "C" fn(*mut core::ffi::c_void, *const clock_config) -> i32>,
}

extern "C" {
    pub fn stm32_rcc_init(dev: *mut device, match_data: *const of_device_id, base: *mut core::ffi::c_void) -> i32;
}

pub const MUX_NO_RDY: u8 = 0xFF;
pub const MUX_SAFE: u32 = 1u32 << 7;
pub const DIV_NO_RDY: u8 = 0xFF;

#[repr(C)]
pub struct clk_stm32_mux {
    pub mux_id: u16,
    pub hw: clk_hw,
    pub base: *mut core::ffi::c_void,
    pub clock_data: *mut clk_stm32_clock_data,
    pub lock: *mut spinlock_t,
}

macro_rules! to_clk_stm32_mux { ($hw:expr) => { container_of!($hw, clk_stm32_mux, hw) }; }

#[repr(C)]
pub struct clk_stm32_gate {
    pub gate_id: u16,
    pub hw: clk_hw,
    pub base: *mut core::ffi::c_void,
    pub clock_data: *mut clk_stm32_clock_data,
    pub lock: *mut spinlock_t,
}

macro_rules! to_clk_stm32_gate { ($hw:expr) => { container_of!($hw, clk_stm32_gate, hw) }; }

#[repr(C)]
pub struct clk_stm32_div {
    pub div_id: u16,
    pub hw: clk_hw,
    pub base: *mut core::ffi::c_void,
    pub clock_data: *mut clk_stm32_clock_data,
    pub lock: *mut spinlock_t,
}

macro_rules! to_clk_stm32_divider { ($hw:expr) => { container_of!($hw, clk_stm32_div, hw) }; }

#[repr(C)]
pub struct clk_stm32_composite {
    pub gate_id: u16,
    pub mux_id: u16,
    pub div_id: u16,
    pub hw: clk_hw,
    pub base: *mut core::ffi::c_void,
    pub clock_data: *mut clk_stm32_clock_data,
    pub lock: *mut spinlock_t,
}

macro_rules! to_clk_stm32_composite { ($hw:expr) => { container_of!($hw, clk_stm32_composite, hw) }; }

extern "C" {
    pub static clk_stm32_mux_ops: clk_ops;
    pub static clk_stm32_gate_ops: clk_ops;
    pub static clk_stm32_divider_ops: clk_ops;
    pub static clk_stm32_composite_ops: clk_ops;

    pub fn clk_stm32_mux_register(dev: *mut device, data: *const stm32_rcc_match_data, base: *mut core::ffi::c_void, lock: *mut spinlock_t, cfg: *const clock_config) -> *mut clk_hw;
    pub fn clk_stm32_gate_register(dev: *mut device, data: *const stm32_rcc_match_data, base: *mut core::ffi::c_void, lock: *mut spinlock_t, cfg: *const clock_config) -> *mut clk_hw;
    pub fn clk_stm32_div_register(dev: *mut device, data: *const stm32_rcc_match_data, base: *mut core::ffi::c_void, lock: *mut spinlock_t, cfg: *const clock_config) -> *mut clk_hw;
    pub fn clk_stm32_composite_register(dev: *mut device, data: *const stm32_rcc_match_data, base: *mut core::ffi::c_void, lock: *mut spinlock_t, cfg: *const clock_config) -> *mut clk_hw;
}

macro_rules! STM32_CLOCK_CFG {
    ($binding:expr, $clk:expr, $sec_id:expr, $struct_ty:ty, $register:ident) => {
        clock_config {
            id: $binding as usize,
            sec_id: $sec_id,
            clock_cfg: ($clk as *mut $struct_ty) as *mut core::ffi::c_void,
            func: Some($register),
        }
    };
}

macro_rules! STM32_MUX_CFG {
    ($binding:expr, $clk:expr, $sec_id:expr) => {
        STM32_CLOCK_CFG!($binding, $clk, $sec_id, clk_stm32_mux, clk_stm32_mux_register)
    };
}

macro_rules! STM32_GATE_CFG {
    ($binding:expr, $clk:expr, $sec_id:expr) => {
        STM32_CLOCK_CFG!($binding, $clk, $sec_id, clk_stm32_gate, clk_stm32_gate_register)
    };
}

macro_rules! STM32_DIV_CFG {
    ($binding:expr, $clk:expr, $sec_id:expr) => {
        STM32_CLOCK_CFG!($binding, $clk, $sec_id, clk_stm32_div, clk_stm32_div_register)
    };
}

macro_rules! STM32_COMPOSITE_CFG {
    ($binding:expr, $clk:expr, $sec_id:expr) => {
        STM32_CLOCK_CFG!($binding, $clk, $sec_id, clk_stm32_composite, clk_stm32_composite_register)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
