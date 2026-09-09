/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/arch/arm/mach-omap1/clock.h
 *
 *  Copyright (C) 2004 - 2005, 2009 Nokia corporation
 *  Written by Tuukka Tikkanen <tuukka.tikkanen@elektrobit.com>
 *  Based on clocks.h by Tony Lindgren, Gordon McNutt and RidgeRun, Inc
 */

// Dependencies supplied by the surrounding kernel translation.
pub struct module;
pub struct clk_lookup {
    pub dev_id: *const core::ffi::c_char,
    pub con_id: *const core::ffi::c_char,
    pub clk_hw: *mut clk_hw,
}
pub struct clk_hw;
pub struct clk_ops;

pub struct omap_clk {
    pub cpu: u16,
    pub lk: clk_lookup,
}

#[macro_export]
macro_rules! CLK {
    ($dev:expr, $con:expr, $ck:expr, $cp:expr) => {
        omap_clk {
            cpu: $cp,
            lk: clk_lookup { dev_id: $dev, con_id: $con, clk_hw: $ck },
        }
    };
}

/* Platform flags for the clkdev-OMAP integration code */
pub const CK_310: u32 = 1 << 0;
pub const CK_7XX: u32 = 1 << 1; /* 7xx, 850 */
pub const CK_1510: u32 = 1 << 2;
pub const CK_16XX: u32 = 1 << 3; /* 16xx, 17xx, 5912 */
pub const CK_1710: u32 = 1 << 4; /* 1710 extra for rate selection */

pub struct clkops {
    pub enable: Option<unsafe extern "C" fn(clk: *mut omap1_clk) -> i32>,
    pub disable: Option<unsafe extern "C" fn(clk: *mut omap1_clk)>,
}

pub const ENABLE_REG_32BIT: u8 = 1 << 0; /* Use 32-bit access */
pub const CLOCK_IDLE_CONTROL: u8 = 1 << 1;
pub const CLOCK_NO_IDLE_PARENT: u8 = 1 << 2;

pub struct omap1_clk {
    pub hw: clk_hw,
    pub ops: *const clkops,
    pub rate: usize,
    pub enable_reg: *mut core::ffi::c_void,
    pub recalc: Option<unsafe extern "C" fn(*mut omap1_clk, usize) -> usize>,
    pub set_rate: Option<unsafe extern "C" fn(*mut omap1_clk, usize, usize) -> i32>,
    pub round_rate: Option<unsafe extern "C" fn(*mut omap1_clk, usize, *mut usize) -> isize>,
    pub init: Option<unsafe extern "C" fn(*mut omap1_clk) -> i32>,
    pub enable_bit: u8,
    pub fixed_div: u8,
    pub flags: u8,
    pub rate_offset: u8,
}

/* Equivalent to container_of(_hw, struct omap1_clk, hw). */
#[macro_export]
macro_rules! to_omap1_clk {
    ($hw:expr) => {
        unsafe { &mut *((($hw as *mut u8).sub(core::mem::offset_of!(omap1_clk, hw))) as *mut omap1_clk) }
    };
}

extern "C" {
    pub fn propagate_rate(clk: *mut omap1_clk);
    pub fn followparent_recalc(clk: *mut omap1_clk, p_rate: usize) -> usize;
    pub fn omap_fixed_divisor_recalc(clk: *mut omap1_clk, p_rate: usize) -> usize;
    pub static mut dummy_ck: omap1_clk;
    pub fn omap1_clk_init() -> i32;
    pub fn omap1_clk_late_init();
    pub fn omap1_ckctl_recalc(clk: *mut omap1_clk, p_rate: usize) -> usize;
    pub fn omap1_round_sossi_rate(clk: *mut omap1_clk, rate: usize, p_rate: *mut usize) -> isize;
    pub fn omap1_set_sossi_rate(clk: *mut omap1_clk, rate: usize, p_rate: usize) -> i32;
    pub fn omap1_sossi_recalc(clk: *mut omap1_clk, p_rate: usize) -> usize;
    pub fn omap1_ckctl_recalc_dsp_domain(clk: *mut omap1_clk, p_rate: usize) -> usize;
    pub fn omap1_clk_set_rate_dsp_domain(clk: *mut omap1_clk, rate: usize, p_rate: usize) -> i32;
    pub fn omap1_round_uart_rate(clk: *mut omap1_clk, rate: usize, p_rate: *mut usize) -> isize;
    pub fn omap1_set_uart_rate(clk: *mut omap1_clk, rate: usize, p_rate: usize) -> i32;
    pub fn omap1_uart_recalc(clk: *mut omap1_clk, p_rate: usize) -> usize;
    pub fn omap1_set_ext_clk_rate(clk: *mut omap1_clk, rate: usize, p_rate: usize) -> i32;
    pub fn omap1_round_ext_clk_rate(clk: *mut omap1_clk, rate: usize, p_rate: *mut usize) -> isize;
    pub fn omap1_init_ext_clk(clk: *mut omap1_clk) -> i32;
    pub fn omap1_select_table_rate(clk: *mut omap1_clk, rate: usize, p_rate: usize) -> i32;
    pub fn omap1_round_to_table_rate(clk: *mut omap1_clk, rate: usize, p_rate: *mut usize) -> isize;
    pub fn omap1_clk_set_rate_ckctl_arm(clk: *mut omap1_clk, rate: usize, p_rate: usize) -> i32;
    pub fn omap1_clk_round_rate_ckctl_arm(clk: *mut omap1_clk, rate: usize, p_rate: *mut usize) -> isize;
}

pub struct uart_clk {
    pub clk: omap1_clk,
    pub sysc_addr: usize,
}

/* Provide a method for preventing idling some ARM IDLECT clocks */
pub struct arm_idlect1_clk {
    pub clk: omap1_clk,
    pub no_idle_count: usize,
    pub idlect_shift: u8,
}

pub const CKCTL_PERDIV_OFFSET: u32 = 0;
pub const CKCTL_LCDDIV_OFFSET: u32 = 2;
pub const CKCTL_ARMDIV_OFFSET: u32 = 4;
pub const CKCTL_DSPDIV_OFFSET: u32 = 6;
pub const CKCTL_TCDIV_OFFSET: u32 = 8;
pub const CKCTL_DSPMMUDIV_OFFSET: u32 = 10;
pub const EN_DSPCK: u32 = 13;
pub const CKCTL_DSPPERDIV_OFFSET: u32 = 0;
pub const EN_WDTCK: u32 = 0;
pub const EN_XORPCK: u32 = 1;
pub const EN_PERCK: u32 = 2;
pub const EN_LCDCK: u32 = 3;
pub const EN_LBCK: u32 = 4; /* Not on 1610/1710 */
pub const EN_APICK: u32 = 6;
pub const EN_TIMCK: u32 = 7;
pub const DMACK_REQ: u32 = 8;
pub const EN_GPIOCK: u32 = 9; /* Not on 1610/1710 */
pub const EN_CKOUT_ARM: u32 = 11;
pub const EN_OCPI_CK: u32 = 0;
pub const EN_TC1_CK: u32 = 2;
pub const EN_TC2_CK: u32 = 4;
pub const EN_DSPTIMCK: u32 = 5;
pub const SDW_MCLK_INV_BIT: u32 = 2;
pub const USB_MCLK_EN_BIT: u32 = 4;
pub const USB_HOST_HHC_UHOST_EN: u32 = 9;
pub const SWD_ULPD_PLL_CLK_REQ: u32 = 1;
pub const COM_ULPD_PLL_CLK_REQ: u32 = 1;
pub const SWD_CLK_DIV_CTRL_SEL: u32 = 0xfffe0874;
pub const COM_CLK_DIV_CTRL_SEL: u32 = 0xfffe0878;
pub const SOFT_REQ_REG: u32 = 0xfffe0834;
pub const SOFT_REQ_REG2: u32 = 0xfffe0880;

extern "C" {
    pub static mut arm_idlect1_mask: u32;
    pub static mut api_ck_p: *mut omap1_clk;
    pub static mut ck_dpll1_p: *mut omap1_clk;
    pub static mut ck_ref_p: *mut omap1_clk;
    pub static clkops_dspck: clkops;
    pub static clkops_uart_16xx: clkops;
    pub static clkops_generic: clkops;
    pub static mut cpu_mask: u32;
    pub static omap1_clk_null_ops: clk_ops;
    pub static omap1_clk_gate_ops: clk_ops;
    pub static omap1_clk_rate_ops: clk_ops;
    pub static omap1_clk_full_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
