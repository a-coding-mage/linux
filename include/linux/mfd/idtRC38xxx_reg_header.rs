/* SPDX-License-Identifier: GPL-2.0+ */
/* Register Map - Based on PolarBear_CSRs.RevA.xlsx (2023-04-21) */

#![allow(non_camel_case_types, non_upper_case_globals, dead_code)]

macro_rules! BIT { ($n:expr) => { 1u32 << ($n) }; }
macro_rules! GENMASK { ($hi:expr, $lo:expr) => { (((1u32 << (($hi) - ($lo) + 1)) - 1) << ($lo)) }; }
macro_rules! GENMASK_ULL { ($hi:expr, $lo:expr) => { (((1u64 << (($hi) - ($lo) + 1)) - 1) << ($lo)) }; }

pub const SOFT_RESET_CTRL: u32 = 0x15;
pub const MISC_CTRL: u32 = 0x14;
pub const APLL_REINIT: u32 = BIT!(1);
pub const APLL_REINIT_VFC3A: u32 = BIT!(2);
pub const DEVICE_ID: u32 = 0x2;
pub const DEVICE_ID_MASK: u32 = 0x1000;
pub const DEVICE_ID_SHIFT: u32 = 12;

pub const FOD_0: u32 = 0x300;
pub const FOD_0_VFC3A: u32 = 0x400;
pub const FOD_1: u32 = 0x340;
pub const FOD_1_VFC3A: u32 = 0x440;
pub const FOD_2: u32 = 0x380;
pub const FOD_2_VFC3A: u32 = 0x480;

pub const TDC_CTRL: u32 = 0x44a;
pub const TDC_ENABLE_CTRL: u32 = 0x169;
pub const TDC_DAC_CAL_CTRL: u32 = 0x16a;
pub const TDC_EN: u32 = BIT!(0);
pub const TDC_DAC_RECAL_REQ: u32 = BIT!(1);
pub const TDC_DAC_RECAL_REQ_VFC3A: u32 = BIT!(0);
pub const TDC_FB_DIV_INT_CNFG: u32 = 0x442;
pub const TDC_FB_DIV_INT_CNFG_VFC3A: u32 = 0x162;
pub const TDC_FB_DIV_INT_MASK: u32 = GENMASK!(7, 0);
pub const TDC_REF_DIV_CNFG: u32 = 0x443;
pub const TDC_REF_DIV_CNFG_VFC3A: u32 = 0x163;
pub const TDC_REF_DIV_CONFIG_MASK: u32 = GENMASK!(2, 0);

pub const TIME_CLOCK_SRC: u32 = 0xa01;
pub const TIME_CLOCK_COUNT: u32 = 0xa00;
pub const TIME_CLOCK_COUNT_MASK: u32 = GENMASK!(5, 0);
pub const SUB_SYNC_GEN_CNFG: u32 = 0xa04;
pub const TOD_COUNTER_READ_REQ: u32 = 0xa5f;
pub const TOD_COUNTER_READ_REQ_VFC3A: u32 = 0x6df;
pub const TOD_SYNC_LOAD_VAL_CTRL: u32 = 0xa10;
pub const TOD_SYNC_LOAD_VAL_CTRL_VFC3A: u32 = 0x690;
pub const SYNC_COUNTER_MASK: u64 = GENMASK_ULL!(51, 0);
pub const SUB_SYNC_COUNTER_MASK: u32 = GENMASK!(30, 0);
pub const TOD_SYNC_LOAD_REQ_CTRL: u32 = 0xa21;
pub const TOD_SYNC_LOAD_REQ_CTRL_VFC3A: u32 = 0x6a1;
pub const SYNC_LOAD_ENABLE: u32 = BIT!(1);
pub const SUB_SYNC_LOAD_ENABLE: u32 = BIT!(0);
pub const SYNC_LOAD_REQ: u32 = BIT!(0);

#[repr(i32)]
pub enum lpf_mode { LPF_DISABLED = 0, LPF_WP = 1, LPF_HOLDOVER = 2, LPF_WF = 3, LPF_INVALID = 4 }
pub const LPF_MODE_CNFG: u32 = 0xa80;
pub const LPF_MODE_CNFG_VFC3A: u32 = 0x700;
pub const LPF_CTRL: u32 = 0xa98;
pub const LPF_CTRL_VFC3A: u32 = 0x718;
pub const LPF_EN: u32 = BIT!(0);
pub const LPF_BW_CNFG: u32 = 0xa81;
pub const LPF_BW_SHIFT: u32 = GENMASK!(7, 3);
pub const LPF_BW_MULT: u32 = GENMASK!(2, 0);
pub const LPF_BW_SHIFT_DEFAULT: u32 = 0xb;
pub const LPF_BW_MULT_DEFAULT: u32 = 0x0;
pub const LPF_BW_SHIFT_1PPS: u32 = 0x5;
pub const LPF_WR_PHASE_CTRL: u32 = 0xaa8;
pub const LPF_WR_PHASE_CTRL_VFC3A: u32 = 0x728;
pub const LPF_WR_FREQ_CTRL: u32 = 0xab0;
pub const LPF_WR_FREQ_CTRL_VFC3A: u32 = 0x730;

pub const TIME_CLOCK_TDC_FANOUT_CNFG: u32 = 0xB00;
pub const TIME_SYNC_TO_TDC_EN: u32 = BIT!(0);
pub const SIG1_MUX_SEL_MASK: u32 = GENMASK!(7, 4);
pub const SIG2_MUX_SEL_MASK: u32 = GENMASK!(11, 8);
#[repr(i32)]
pub enum tdc_mux_sel { REF0 = 0, REF1 = 1, REF2 = 2, REF3 = 3, REF_CLK5 = 4, REF_CLK6 = 5, DPLL_FB_TO_TDC = 6, DPLL_FB_DIVIDED_TO_TDC = 7, TIME_CLK_DIVIDED = 8, TIME_SYNC = 9 }
pub const TIME_CLOCK_MEAS_CNFG: u32 = 0xB04;
pub const TDC_MEAS_MODE: u32 = BIT!(0);
#[repr(i32)]
pub enum tdc_meas_mode { CONTINUOUS = 0, ONE_SHOT = 1, MEAS_MODE_INVALID = 2 }
pub const TIME_CLOCK_MEAS_DIV_CNFG: u32 = 0xB08;
pub const TIME_REF_DIV_MASK: u32 = GENMASK!(29, 24);
pub const TIME_CLOCK_MEAS_CTRL: u32 = 0xB10;
pub const TDC_MEAS_EN: u32 = BIT!(0);
pub const TDC_MEAS_START: u32 = BIT!(1);
pub const TDC_FIFO_READ_REQ: u32 = 0xB2F;
pub const TDC_FIFO_READ: u32 = 0xB30;
pub const COARSE_MEAS_MASK: u64 = GENMASK_ULL!(39, 13);
pub const FINE_MEAS_MASK: u32 = GENMASK!(12, 0);
pub const TDC_FIFO_CTRL: u32 = 0xB12;
pub const FIFO_CLEAR: u32 = BIT!(0);
pub const TDC_FIFO_STS: u32 = 0xB38;
pub const FIFO_FULL: u32 = BIT!(1);
pub const FIFO_EMPTY: u32 = BIT!(0);
pub const TDC_FIFO_EVENT: u32 = 0xB39;
pub const FIFO_OVERRUN: u32 = BIT!(1);

pub const MAX_REFERENCE_INDEX: u32 = 3;
pub const MAX_NUM_REF_PRIORITY: u32 = 4;
pub const MAX_DPLL_INDEX: u32 = 2;
pub const DPLL_STS: u32 = 0x580;
pub const DPLL_STS_VFC3A: u32 = 0x571;
pub const DPLL_STATE_STS_MASK: u32 = 0x70;
pub const DPLL_STATE_STS_SHIFT: u32 = 4;
pub const DPLL_REF_SEL_STS_MASK: u32 = 0x6;
pub const DPLL_REF_SEL_STS_SHIFT: u32 = 1;
pub const DPLL_REF_PRIORITY_CNFG: u32 = 0x502;
pub const DPLL_REFX_PRIORITY_DISABLE_MASK: u32 = 0xf;
pub const DPLL_REF0_PRIORITY_ENABLE_AND_SET_MASK: u32 = 0x31;
pub const DPLL_REF1_PRIORITY_ENABLE_AND_SET_MASK: u32 = 0xc2;
pub const DPLL_REF2_PRIORITY_ENABLE_AND_SET_MASK: u32 = 0x304;
pub const DPLL_REF3_PRIORITY_ENABLE_AND_SET_MASK: u32 = 0xc08;
pub const DPLL_REF0_PRIORITY_SHIFT: u32 = 4;
pub const DPLL_REF1_PRIORITY_SHIFT: u32 = 6;
pub const DPLL_REF2_PRIORITY_SHIFT: u32 = 8;
pub const DPLL_REF3_PRIORITY_SHIFT: u32 = 10;
#[repr(i32)]
pub enum dpll_state { DPLL_STATE_MIN = 0, DPLL_STATE_FREERUN = 0, DPLL_STATE_LOCKED = 1, DPLL_STATE_HOLDOVER = 2, DPLL_STATE_WRITE_FREQUENCY = 3, DPLL_STATE_ACQUIRE = 4, DPLL_STATE_HITLESS_SWITCH = 5, DPLL_STATE_MAX = 5 }

pub const LOSMON_STS_0: u32 = 0x81e; pub const LOSMON_STS_0_VFC3A: u32 = 0x18e;
pub const LOSMON_STS_1: u32 = 0x82e; pub const LOSMON_STS_1_VFC3A: u32 = 0x19e;
pub const LOSMON_STS_2: u32 = 0x83e; pub const LOSMON_STS_2_VFC3A: u32 = 0x1ae;
pub const LOSMON_STS_3: u32 = 0x84e; pub const LOSMON_STS_3_VFC3A: u32 = 0x1be;
pub const LOS_STS_MASK: u32 = 0x1;
pub const FREQMON_STS_0: u32 = 0x874; pub const FREQMON_STS_0_VFC3A: u32 = 0x1d4;
pub const FREQMON_STS_1: u32 = 0x894; pub const FREQMON_STS_1_VFC3A: u32 = 0x1f4;
pub const FREQMON_STS_2: u32 = 0x8b4; pub const FREQMON_STS_2_VFC3A: u32 = 0x214;
pub const FREQMON_STS_3: u32 = 0x8d4; pub const FREQMON_STS_3_VFC3A: u32 = 0x234;
pub const FREQ_FAIL_STS_SHIFT: u32 = 31;
pub const TIME_CLK_FREQ_ADDR: u16 = 0xffa0;
pub const XTAL_FREQ_ADDR: u16 = 0xffa1;

#[macro_export]
macro_rules! IDTFC3_FW_REG { ($fw:expr, $ver:expr, $reg:ident, $reg_ver:ident) => { if $fw < $ver { $reg } else { $reg_ver } }; }
#[macro_export]
macro_rules! IDTFC3_FW_FIELD { ($fw:expr, $ver:expr, $field:ident, $field_ver:ident) => { if $fw < $ver { $field } else { $field_ver } }; }
#[repr(i32)]
pub enum fw_version { V_DEFAULT = 0, VFC3W = 1, VFC3A = 2 }
#[repr(i32)]
pub enum Freq { FREQ_MIN = 0, FREQ_25M = 1, FREQ_49_152M = 2, FREQ_50M = 3, FREQ_100M = 4, FREQ_125M = 5, FREQ_250M = 6, FREQ_MAX = 7 }

#[repr(C)]
pub struct idtfc3_hw_param { pub xtal_freq: u32, pub time_clk_freq: u32 }
#[repr(C, packed)]
pub struct idtfc3_fwrc { pub hiaddr: u8, pub loaddr: u8, pub value: u8, pub reserved: u8 }

#[inline]
pub unsafe fn idtfc3_default_hw_param(hw_param: *mut idtfc3_hw_param) {
    (*hw_param).xtal_freq = 49152000;
    (*hw_param).time_clk_freq = 25000000;
}

#[inline]
pub unsafe fn idtfc3_set_hw_param(hw_param: *mut idtfc3_hw_param, addr: u16, val: u8) -> i32 {
    const EINVAL: i32 = 22;
    const EFAULT: i32 = 14;
    if addr == XTAL_FREQ_ADDR {
        match val { x if x == Freq::FREQ_49_152M as u8 => (*hw_param).xtal_freq = 49152000, x if x == Freq::FREQ_50M as u8 => (*hw_param).xtal_freq = 50000000, _ => return -EINVAL }
    } else if addr == TIME_CLK_FREQ_ADDR {
        match val { x if x == Freq::FREQ_25M as u8 => (*hw_param).time_clk_freq = 25000000, x if x == Freq::FREQ_50M as u8 => (*hw_param).time_clk_freq = 50000000, x if x == Freq::FREQ_100M as u8 => (*hw_param).time_clk_freq = 100000000, x if x == Freq::FREQ_125M as u8 => (*hw_param).time_clk_freq = 125000000, x if x == Freq::FREQ_250M as u8 => (*hw_param).time_clk_freq = 250000000, _ => return -EINVAL }
    } else { return -EFAULT; }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
