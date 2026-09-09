/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * include/media/si476x-platform.h -- Platform data specific definitions
 *
 * Copyright (C) 2013 Andrey Smirnov
 *
 * Author: Andrey Smirnov <andrew.smirnov@gmail.com>
 */

// It is possible to select one of the four addresses using pins A0
// and A1 on SI476x
pub const SI476X_I2C_ADDR_1: u32 = 0x60;
pub const SI476X_I2C_ADDR_2: u32 = 0x61;
pub const SI476X_I2C_ADDR_3: u32 = 0x62;
pub const SI476X_I2C_ADDR_4: u32 = 0x63;

#[repr(i32)]
pub enum si476x_iqclk_config { SI476X_IQCLK_NOOP = 0, SI476X_IQCLK_TRISTATE = 1, SI476X_IQCLK_IQ = 21 }
#[repr(i32)]
pub enum si476x_iqfs_config { SI476X_IQFS_NOOP = 0, SI476X_IQFS_TRISTATE = 1, SI476X_IQFS_IQ = 21 }
#[repr(i32)]
pub enum si476x_iout_config { SI476X_IOUT_NOOP = 0, SI476X_IOUT_TRISTATE = 1, SI476X_IOUT_OUTPUT = 22 }
#[repr(i32)]
pub enum si476x_qout_config { SI476X_QOUT_NOOP = 0, SI476X_QOUT_TRISTATE = 1, SI476X_QOUT_OUTPUT = 22 }
#[repr(i32)]
pub enum si476x_dclk_config { SI476X_DCLK_NOOP = 0, SI476X_DCLK_TRISTATE = 1, SI476X_DCLK_DAUDIO = 10 }
#[repr(i32)]
pub enum si476x_dfs_config { SI476X_DFS_NOOP = 0, SI476X_DFS_TRISTATE = 1, SI476X_DFS_DAUDIO = 10 }
#[repr(i32)]
pub enum si476x_dout_config { SI476X_DOUT_NOOP = 0, SI476X_DOUT_TRISTATE = 1, SI476X_DOUT_I2S_OUTPUT = 12, SI476X_DOUT_I2S_INPUT = 13 }
#[repr(i32)]
pub enum si476x_xout_config { SI476X_XOUT_NOOP = 0, SI476X_XOUT_TRISTATE = 1, SI476X_XOUT_I2S_INPUT = 13, SI476X_XOUT_MODE_SELECT = 23 }
#[repr(i32)]
pub enum si476x_icin_config { SI476X_ICIN_NOOP = 0, SI476X_ICIN_TRISTATE = 1, SI476X_ICIN_GPO1_HIGH = 2, SI476X_ICIN_GPO1_LOW = 3, SI476X_ICIN_IC_LINK = 30 }
#[repr(i32)]
pub enum si476x_icip_config { SI476X_ICIP_NOOP = 0, SI476X_ICIP_TRISTATE = 1, SI476X_ICIP_GPO2_HIGH = 2, SI476X_ICIP_GPO2_LOW = 3, SI476X_ICIP_IC_LINK = 30 }
#[repr(i32)]
pub enum si476x_icon_config { SI476X_ICON_NOOP = 0, SI476X_ICON_TRISTATE = 1, SI476X_ICON_I2S = 10, SI476X_ICON_IC_LINK = 30 }
#[repr(i32)]
pub enum si476x_icop_config { SI476X_ICOP_NOOP = 0, SI476X_ICOP_TRISTATE = 1, SI476X_ICOP_I2S = 10, SI476X_ICOP_IC_LINK = 30 }
#[repr(i32)]
pub enum si476x_lrout_config { SI476X_LROUT_NOOP = 0, SI476X_LROUT_TRISTATE = 1, SI476X_LROUT_AUDIO = 2, SI476X_LROUT_MPX = 3 }
#[repr(i32)]
pub enum si476x_intb_config { SI476X_INTB_NOOP = 0, SI476X_INTB_TRISTATE = 1, SI476X_INTB_DAUDIO = 10, SI476X_INTB_IRQ = 40 }
#[repr(i32)]
pub enum si476x_a1_config { SI476X_A1_NOOP = 0, SI476X_A1_TRISTATE = 1, SI476X_A1_IRQ = 40 }

#[repr(C)]
pub struct si476x_pinmux {
    pub dclk: si476x_dclk_config,
    pub dfs: si476x_dfs_config,
    pub dout: si476x_dout_config,
    pub xout: si476x_xout_config,
    pub iqclk: si476x_iqclk_config,
    pub iqfs: si476x_iqfs_config,
    pub iout: si476x_iout_config,
    pub qout: si476x_qout_config,
    pub icin: si476x_icin_config,
    pub icip: si476x_icip_config,
    pub icon: si476x_icon_config,
    pub icop: si476x_icop_config,
    pub lrout: si476x_lrout_config,
    pub intb: si476x_intb_config,
    pub a1: si476x_a1_config,
}

#[repr(i32)]
pub enum si476x_ibias6x { SI476X_IBIAS6X_OTHER = 0, SI476X_IBIAS6X_RCVR1_NON_4MHZ_CLK = 1 }
#[repr(i32)]
pub enum si476x_xstart { SI476X_XSTART_MULTIPLE_TUNER = 0x11, SI476X_XSTART_NORMAL = 0x77 }
#[repr(i32)]
pub enum si476x_freq { SI476X_FREQ_4_MHZ = 0, SI476X_FREQ_37P209375_MHZ = 1, SI476X_FREQ_36P4_MHZ = 2, SI476X_FREQ_37P8_MHZ = 3 }
#[repr(i32)]
pub enum si476x_xmode { SI476X_XMODE_CRYSTAL_RCVR1 = 1, SI476X_XMODE_EXT_CLOCK = 2, SI476X_XMODE_CRYSTAL_RCVR2_3 = 3 }
#[repr(i32)]
pub enum si476x_xbiashc { SI476X_XBIASHC_SINGLE_RECEIVER = 0, SI476X_XBIASHC_MULTIPLE_RECEIVER = 1 }
#[repr(i32)]
pub enum si476x_xbias { SI476X_XBIAS_RCVR2_3 = 0, SI476X_XBIAS_4MHZ_RCVR1 = 3, SI476X_XBIAS_RCVR1 = 7 }
#[repr(i32)]
pub enum si476x_func { SI476X_FUNC_BOOTLOADER = 0, SI476X_FUNC_FM_RECEIVER = 1, SI476X_FUNC_AM_RECEIVER = 2, SI476X_FUNC_WB_RECEIVER = 3 }

/**
 * @xcload: Selects the amount of additional on-chip capacitance to
 *          be connected between XTAL1 and gnd and between XTAL2 and
 *          GND. One half of the capacitance value shown here is the
 *          additional load capacitance presented to the xtal. The
 *          minimum step size is 0.277 pF. Recommended value is 0x28
 *          but it will be layout dependent. Range is 0–0x3F i.e.
 *          (0–16.33 pF)
 * @ctsien: enable CTSINT(interrupt request when CTS condition
 *          arises) when set
 * @intsel: when set A1 pin becomes the interrupt pin; otherwise,
 *          INTB is the interrupt pin
 * @func:   selects the boot function of the device. I.e.
 *          SI476X_BOOTLOADER  - Boot loader
 *          SI476X_FM_RECEIVER - FM receiver
 *          SI476X_AM_RECEIVER - AM receiver
 *          SI476X_WB_RECEIVER - Weatherband receiver
 * @freq:   oscillator's crystal frequency:
 *          SI476X_XTAL_37P209375_MHZ - 37.209375 Mhz
 *          SI476X_XTAL_36P4_MHZ      - 36.4 Mhz
 *          SI476X_XTAL_37P8_MHZ      - 37.8 Mhz
 */
#[repr(C)]
pub struct si476x_power_up_args {
    pub ibias6x: si476x_ibias6x,
    pub xstart: si476x_xstart,
    pub xcload: u8,
    pub fastboot: bool,
    pub xbiashc: si476x_xbiashc,
    pub xbias: si476x_xbias,
    pub func: si476x_func,
    pub freq: si476x_freq,
    pub xmode: si476x_xmode,
}

/** enum si476x_phase_diversity_mode - possbile phase diversity modes
 * for SI4764/5/6/7 chips.
 */
#[repr(i32)]
pub enum si476x_phase_diversity_mode {
    SI476X_PHDIV_DISABLED = 0,
    SI476X_PHDIV_PRIMARY_COMBINING = 1,
    SI476X_PHDIV_PRIMARY_ANTENNA = 2,
    SI476X_PHDIV_SECONDARY_ANTENNA = 3,
    SI476X_PHDIV_SECONDARY_COMBINING = 5,
}

/* Platform dependent definition */
#[repr(C)]
pub struct si476x_platform_data {
    pub power_up_parameters: si476x_power_up_args,
    pub diversity_mode: si476x_phase_diversity_mode,
    pub pinmux: si476x_pinmux,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
