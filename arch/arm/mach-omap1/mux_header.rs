/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Table of the Omap register configurations for the FUNC_MUX and
 * PULL_DWN combinations.
 *
 * C header dependency: <linux/soc/ti/omap1-mux.h>
 */

use core::ffi::{c_char, c_int};

pub const PU_PD_SEL_NA: u32 = 0; // No pu_pd reg available
pub const PULL_DWN_CTRL_NA: u32 = 0; // No pull-down control needed

/* The C preprocessor selects the debug form when CONFIG_OMAP_MUX_DEBUG is set.
 * The register-name fields are represented directly by the corresponding
 * Rust macros below; register constants are supplied by the including code.
 */
#[cfg(feature = "CONFIG_OMAP_MUX_DEBUG")]
#[macro_export]
macro_rules! MUX_REG {
    ($reg:ident, $mode_offset:expr, $mode:expr) => {
        mux_reg_name: Some(concat!("FUNC_MUX_CTRL_", stringify!($reg))),
        mux_reg: FUNC_MUX_CTRL_$reg,
        mask_offset: $mode_offset,
        mask: $mode,
    };
}

#[cfg(not(feature = "CONFIG_OMAP_MUX_DEBUG"))]
#[macro_export]
macro_rules! MUX_REG {
    ($reg:expr, $mode_offset:expr, $mode:expr) => {
        mux_reg: $reg,
        mask_offset: $mode_offset,
        mask: $mode,
    };
}

#[macro_export]
macro_rules! PULL_REG {
    ($reg:expr, $bit:expr, $status:expr) => {
        pull_reg: $reg,
        pull_bit: $bit,
        pull_val: $status,
    };
}

#[macro_export]
macro_rules! PU_PD_REG {
    ($reg:expr, $status:expr) => {
        pu_pd_reg: $reg,
        pu_pd_val: $status,
    };
}

#[macro_export]
macro_rules! MUX_REG_7XX {
    ($reg:expr, $mode_offset:expr, $mode:expr) => {
        mux_reg: $reg,
        mask_offset: $mode_offset,
        mask: $mode,
    };
}

#[macro_export]
macro_rules! PULL_REG_7XX {
    ($reg:expr, $bit:expr, $status:expr) => {
        pull_reg: $reg,
        pull_bit: $bit,
        pull_val: $status,
    };
}

/* C's MUX_CFG and MUX_CFG_7XX expand to struct-initializer entries. */
#[macro_export]
macro_rules! MUX_CFG {
    ($desc:expr, $mux_reg:expr, $mode_offset:expr, $mode:expr,
     $pull_reg:expr, $pull_bit:expr, $pull_status:expr,
     $pu_pd_reg:expr, $pu_pd_status:expr, $debug_status:expr) => {
        pin_config {
            name: $desc,
            debug: $debug_status,
            mux_reg: $mux_reg,
            mask_offset: $mode_offset,
            mask: $mode,
            pull_name: None,
            pull_reg: $pull_reg,
            pull_val: $pull_status,
            pull_bit: $pull_bit,
            pu_pd_name: None,
            pu_pd_reg: $pu_pd_reg,
            pu_pd_val: $pu_pd_status,
            mux_reg_name: None,
        }
    };
}

#[macro_export]
macro_rules! MUX_CFG_7XX {
    ($desc:expr, $mux_reg:expr, $mode_offset:expr, $mode:expr,
     $pull_bit:expr, $pull_status:expr, $debug_status:expr) => {
        pin_config {
            name: $desc,
            debug: $debug_status,
            mux_reg: $mux_reg,
            mask_offset: $mode_offset,
            mask: $mode,
            pull_name: None,
            pull_reg: $mux_reg,
            pull_val: $pull_status,
            pull_bit: $pull_bit,
            pu_pd_name: None,
            pu_pd_reg: PU_PD_SEL_NA,
            pu_pd_val: 0,
            mux_reg_name: None,
        }
    };
}

#[repr(C)]
pub struct pin_config {
    pub name: *mut c_char,
    pub mux_reg: u32,
    pub debug: u8,
    pub mask_offset: u8,
    pub mask: u8,
    pub pull_name: Option<*const c_char>,
    pub pull_reg: u32,
    pub pull_val: u8,
    pub pull_bit: u8,
    pub pu_pd_name: Option<*const c_char>,
    pub pu_pd_reg: u32,
    pub pu_pd_val: u8,
    #[cfg(any(feature = "CONFIG_OMAP_MUX_DEBUG", feature = "CONFIG_OMAP_MUX_WARNINGS"))]
    pub mux_reg_name: Option<*const c_char>,
}

#[repr(C)]
pub struct omap_mux_cfg {
    pub pins: *mut pin_config,
    pub size: usize,
    pub cfg_reg: Option<unsafe extern "C" fn(*const pin_config) -> c_int>,
}

#[cfg(feature = "CONFIG_OMAP_MUX")]
unsafe extern "C" {
    pub fn omap1_mux_init() -> c_int;
    pub fn omap_mux_register(cfg: *mut omap_mux_cfg) -> c_int;
}

#[cfg(not(feature = "CONFIG_OMAP_MUX"))]
#[inline]
pub const unsafe fn omap1_mux_init() -> c_int { 0 }

unsafe extern "C" {
    pub fn omap2_mux_init() -> c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
