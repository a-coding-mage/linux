/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Register Map - Based on AN888_SMUforIEEE_SynchEther_82P33xxx_RevH.pdf
 *
 * Copyright (C) 2021 Integrated Device Technology, Inc., a Renesas Company.
 */

pub const fn reg_addr(page: u32, offset: u32) -> u32 {
    (page << 0x7) | (offset & 0x7f)
}

/* Register address */
pub const DPLL1_TOD_CNFG: u32 = 0x134;
pub const DPLL2_TOD_CNFG: u32 = 0x1B4;

pub const DPLL1_TOD_STS: u32 = 0x10B;
pub const DPLL2_TOD_STS: u32 = 0x18B;

pub const DPLL1_TOD_TRIGGER: u32 = 0x115;
pub const DPLL2_TOD_TRIGGER: u32 = 0x195;

pub const DPLL1_OPERATING_MODE_CNFG: u32 = 0x120;
pub const DPLL2_OPERATING_MODE_CNFG: u32 = 0x1A0;

pub const DPLL1_HOLDOVER_FREQ_CNFG: u32 = 0x12C;
pub const DPLL2_HOLDOVER_FREQ_CNFG: u32 = 0x1AC;

pub const DPLL1_PHASE_OFFSET_CNFG: u32 = 0x143;
pub const DPLL2_PHASE_OFFSET_CNFG: u32 = 0x1C3;

pub const DPLL1_SYNC_EDGE_CNFG: u32 = 0x140;
pub const DPLL2_SYNC_EDGE_CNFG: u32 = 0x1C0;

pub const DPLL1_INPUT_MODE_CNFG: u32 = 0x116;
pub const DPLL2_INPUT_MODE_CNFG: u32 = 0x196;

pub const DPLL1_OPERATING_STS: u32 = 0x102;
pub const DPLL2_OPERATING_STS: u32 = 0x182;

pub const DPLL1_CURRENT_FREQ_STS: u32 = 0x103;
pub const DPLL2_CURRENT_FREQ_STS: u32 = 0x183;

pub const REG_SOFT_RESET: u32 = 0X381;

pub const fn out_mux_cnfg(outn: u32) -> u32 {
    reg_addr(0x6, 0xC * outn)
}

pub const fn tod_trigger(wr_trig: u32, rd_trig: u32) -> u32 {
    ((wr_trig & 0xf) << 4) | (rd_trig & 0xf)
}

/* Register bit definitions */
pub const SYNC_TOD: u32 = 1 << 1;
pub const PH_OFFSET_EN: u32 = 1 << 7;
pub const SQUELCH_ENABLE: u32 = 1 << 5;

/* Bit definitions for the DPLL_MODE register */
pub const PLL_MODE_SHIFT: u32 = 0;
pub const PLL_MODE_MASK: u32 = 0x1F;
pub const COMBO_MODE_EN: u32 = 1 << 5;
pub const COMBO_MODE_SHIFT: u32 = 6;
pub const COMBO_MODE_MASK: u32 = 0x3;

/* Bit definitions for DPLL_OPERATING_STS register */
pub const OPERATING_STS_MASK: u32 = 0x7;
pub const OPERATING_STS_SHIFT: u32 = 0x0;

/* Bit definitions for DPLL_TOD_TRIGGER register */
pub const READ_TRIGGER_MASK: u32 = 0xF;
pub const READ_TRIGGER_SHIFT: u32 = 0x0;
pub const WRITE_TRIGGER_MASK: u32 = 0xF0;
pub const WRITE_TRIGGER_SHIFT: u32 = 0x4;

/* Bit definitions for REG_SOFT_RESET register */
pub const SOFT_RESET_EN: u32 = 1 << 7;

#[repr(i32)]
pub enum pll_mode {
    PLL_MODE_MIN = 0,
    PLL_MODE_AUTOMATIC = PLL_MODE_MIN as isize,
    PLL_MODE_FORCE_FREERUN = 1,
    PLL_MODE_FORCE_HOLDOVER = 2,
    PLL_MODE_FORCE_LOCKED = 4,
    PLL_MODE_FORCE_PRE_LOCKED2 = 5,
    PLL_MODE_FORCE_PRE_LOCKED = 6,
    PLL_MODE_FORCE_LOST_PHASE = 7,
    PLL_MODE_DCO = 10,
    PLL_MODE_WPH = 18,
    PLL_MODE_MAX = PLL_MODE_WPH as isize,
}

#[repr(i32)]
pub enum hw_tod_trig_sel {
    HW_TOD_TRIG_SEL_MIN = 0,
    HW_TOD_TRIG_SEL_NO_WRITE = HW_TOD_TRIG_SEL_MIN as isize,
    HW_TOD_TRIG_SEL_NO_READ = HW_TOD_TRIG_SEL_MIN as isize,
    HW_TOD_TRIG_SEL_SYNC_SEL = 1,
    HW_TOD_TRIG_SEL_IN12 = 2,
    HW_TOD_TRIG_SEL_IN13 = 3,
    HW_TOD_TRIG_SEL_IN14 = 4,
    HW_TOD_TRIG_SEL_TOD_PPS = 5,
    HW_TOD_TRIG_SEL_TIMER_INTERVAL = 6,
    HW_TOD_TRIG_SEL_MSB_PHASE_OFFSET_CNFG = 7,
    HW_TOD_TRIG_SEL_MSB_HOLDOVER_FREQ_CNFG = 8,
    HW_TOD_WR_TRIG_SEL_MSB_TOD_CNFG = 9,
    HW_TOD_RD_TRIG_SEL_LSB_TOD_STS = HW_TOD_WR_TRIG_SEL_MSB_TOD_CNFG as isize,
    WR_TRIG_SEL_MAX = HW_TOD_WR_TRIG_SEL_MSB_TOD_CNFG as isize,
}

/// @brief Enumerated type listing DPLL operational modes
#[repr(i32)]
pub enum dpll_state {
    DPLL_STATE_FREERUN = 1,
    DPLL_STATE_HOLDOVER = 2,
    DPLL_STATE_LOCKED = 4,
    DPLL_STATE_PRELOCKED2 = 5,
    DPLL_STATE_PRELOCKED = 6,
    DPLL_STATE_LOSTPHASE = 7,
    DPLL_STATE_MAX,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
