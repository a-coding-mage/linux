/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * MFD internals for Cirrus Logic Madera codecs
 *
 * Copyright (C) 2015-2018 Cirrus Logic
 */

// External dependencies supplied by other translated files.
pub enum regmap {}
pub enum device {}
pub enum regulator {}
pub enum regulator_bulk_data {}
pub enum regmap_irq_chip_data {}
pub enum clk_bulk_data {}
pub enum snd_soc_dapm_context {}
pub enum mutex {}
pub enum blocking_notifier_head {}
pub struct madera_pdata;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum madera_type {
    // 0 is reserved for indicating failure to identify.
    CS47L35 = 1,
    CS47L85 = 2,
    CS47L90 = 3,
    CS47L91 = 4,
    CS47L92 = 5,
    CS47L93 = 6,
    WM1840 = 7,
    CS47L15 = 8,
    CS42L92 = 9,
}

pub const MADERA_MCLK1: u32 = 0;
pub const MADERA_MCLK2: u32 = 1;
pub const MADERA_MCLK3: u32 = 2;
pub const MADERA_NUM_MCLK: u32 = 3;

pub const MADERA_MAX_CORE_SUPPLIES: usize = 2;
pub const MADERA_MAX_GPIOS: usize = 40;

pub const CS47L15_NUM_GPIOS: usize = 15;
pub const CS47L35_NUM_GPIOS: usize = 16;
pub const CS47L85_NUM_GPIOS: usize = 40;
pub const CS47L90_NUM_GPIOS: usize = 38;
pub const CS47L92_NUM_GPIOS: usize = 16;

pub const MADERA_MAX_MICBIAS: usize = 4;
pub const MADERA_MAX_HP_OUTPUT: usize = 3;

// Notifier events
pub const MADERA_NOTIFY_VOICE_TRIGGER: u32 = 0x1;
pub const MADERA_NOTIFY_HPDET: u32 = 0x2;
pub const MADERA_NOTIFY_MICDET: u32 = 0x4;

// GPIO Function Definitions
pub const MADERA_GP_FN_ALTERNATE: u32 = 0x00;
pub const MADERA_GP_FN_GPIO: u32 = 0x01;
pub const MADERA_GP_FN_DSP_GPIO: u32 = 0x02;
pub const MADERA_GP_FN_IRQ1: u32 = 0x03;
pub const MADERA_GP_FN_IRQ2: u32 = 0x04;
pub const MADERA_GP_FN_FLL1_CLOCK: u32 = 0x10;
pub const MADERA_GP_FN_FLL2_CLOCK: u32 = 0x11;
pub const MADERA_GP_FN_FLL3_CLOCK: u32 = 0x12;
pub const MADERA_GP_FN_FLLAO_CLOCK: u32 = 0x13;
pub const MADERA_GP_FN_FLL1_LOCK: u32 = 0x18;
pub const MADERA_GP_FN_FLL2_LOCK: u32 = 0x19;
pub const MADERA_GP_FN_FLL3_LOCK: u32 = 0x1A;
pub const MADERA_GP_FN_FLLAO_LOCK: u32 = 0x1B;
pub const MADERA_GP_FN_OPCLK_OUT: u32 = 0x40;
pub const MADERA_GP_FN_OPCLK_ASYNC_OUT: u32 = 0x41;
pub const MADERA_GP_FN_PWM1: u32 = 0x48;
pub const MADERA_GP_FN_PWM2: u32 = 0x49;
pub const MADERA_GP_FN_SPDIF_OUT: u32 = 0x4C;
pub const MADERA_GP_FN_HEADPHONE_DET: u32 = 0x50;
pub const MADERA_GP_FN_MIC_DET: u32 = 0x58;
pub const MADERA_GP_FN_DRC1_SIGNAL_DETECT: u32 = 0x80;
pub const MADERA_GP_FN_DRC2_SIGNAL_DETECT: u32 = 0x81;
pub const MADERA_GP_FN_ASRC1_IN1_LOCK: u32 = 0x88;
pub const MADERA_GP_FN_ASRC1_IN2_LOCK: u32 = 0x89;
pub const MADERA_GP_FN_ASRC2_IN1_LOCK: u32 = 0x8A;
pub const MADERA_GP_FN_ASRC2_IN2_LOCK: u32 = 0x8B;
pub const MADERA_GP_FN_DSP_IRQ1: u32 = 0xA0;
pub const MADERA_GP_FN_DSP_IRQ2: u32 = 0xA1;
pub const MADERA_GP_FN_DSP_IRQ3: u32 = 0xA2;
pub const MADERA_GP_FN_DSP_IRQ4: u32 = 0xA3;
pub const MADERA_GP_FN_DSP_IRQ5: u32 = 0xA4;
pub const MADERA_GP_FN_DSP_IRQ6: u32 = 0xA5;
pub const MADERA_GP_FN_DSP_IRQ7: u32 = 0xA6;
pub const MADERA_GP_FN_DSP_IRQ8: u32 = 0xA7;
pub const MADERA_GP_FN_DSP_IRQ9: u32 = 0xA8;
pub const MADERA_GP_FN_DSP_IRQ10: u32 = 0xA9;
pub const MADERA_GP_FN_DSP_IRQ11: u32 = 0xAA;
pub const MADERA_GP_FN_DSP_IRQ12: u32 = 0xAB;
pub const MADERA_GP_FN_DSP_IRQ13: u32 = 0xAC;
pub const MADERA_GP_FN_DSP_IRQ14: u32 = 0xAD;
pub const MADERA_GP_FN_DSP_IRQ15: u32 = 0xAE;
pub const MADERA_GP_FN_DSP_IRQ16: u32 = 0xAF;
pub const MADERA_GP_FN_HPOUT1L_SC: u32 = 0xB0;
pub const MADERA_GP_FN_HPOUT1R_SC: u32 = 0xB1;
pub const MADERA_GP_FN_HPOUT2L_SC: u32 = 0xB2;
pub const MADERA_GP_FN_HPOUT2R_SC: u32 = 0xB3;
pub const MADERA_GP_FN_HPOUT3L_SC: u32 = 0xB4;
pub const MADERA_GP_FN_HPOUT4R_SC: u32 = 0xB5;
pub const MADERA_GP_FN_SPKOUTL_SC: u32 = 0xB6;
pub const MADERA_GP_FN_SPKOUTR_SC: u32 = 0xB7;
pub const MADERA_GP_FN_HPOUT1L_ENA: u32 = 0xC0;
pub const MADERA_GP_FN_HPOUT1R_ENA: u32 = 0xC1;
pub const MADERA_GP_FN_HPOUT2L_ENA: u32 = 0xC2;
pub const MADERA_GP_FN_HPOUT2R_ENA: u32 = 0xC3;
pub const MADERA_GP_FN_HPOUT3L_ENA: u32 = 0xC4;
pub const MADERA_GP_FN_HPOUT4R_ENA: u32 = 0xC5;
pub const MADERA_GP_FN_SPKOUTL_ENA: u32 = 0xC6;
pub const MADERA_GP_FN_SPKOUTR_ENA: u32 = 0xC7;
pub const MADERA_GP_FN_HPOUT1L_DIS: u32 = 0xD0;
pub const MADERA_GP_FN_HPOUT1R_DIS: u32 = 0xD1;
pub const MADERA_GP_FN_HPOUT2L_DIS: u32 = 0xD2;
pub const MADERA_GP_FN_HPOUT2R_DIS: u32 = 0xD3;
pub const MADERA_GP_FN_HPOUT3L_DIS: u32 = 0xD4;
pub const MADERA_GP_FN_HPOUT4R_DIS: u32 = 0xD5;
pub const MADERA_GP_FN_SPKOUTL_DIS: u32 = 0xD6;
pub const MADERA_GP_FN_SPKOUTR_DIS: u32 = 0xD7;
pub const MADERA_GP_FN_SPK_SHUTDOWN: u32 = 0xE0;
pub const MADERA_GP_FN_SPK_OVH_SHUTDOWN: u32 = 0xE1;
pub const MADERA_GP_FN_SPK_OVH_WARN: u32 = 0xE2;
pub const MADERA_GP_FN_TIMER1_STATUS: u32 = 0x140;
pub const MADERA_GP_FN_TIMER2_STATUS: u32 = 0x141;
pub const MADERA_GP_FN_TIMER3_STATUS: u32 = 0x142;
pub const MADERA_GP_FN_TIMER4_STATUS: u32 = 0x143;
pub const MADERA_GP_FN_TIMER5_STATUS: u32 = 0x144;
pub const MADERA_GP_FN_TIMER6_STATUS: u32 = 0x145;
pub const MADERA_GP_FN_TIMER7_STATUS: u32 = 0x146;
pub const MADERA_GP_FN_TIMER8_STATUS: u32 = 0x147;
pub const MADERA_GP_FN_EVENTLOG1_FIFO_STS: u32 = 0x150;
pub const MADERA_GP_FN_EVENTLOG2_FIFO_STS: u32 = 0x151;
pub const MADERA_GP_FN_EVENTLOG3_FIFO_STS: u32 = 0x152;
pub const MADERA_GP_FN_EVENTLOG4_FIFO_STS: u32 = 0x153;
pub const MADERA_GP_FN_EVENTLOG5_FIFO_STS: u32 = 0x154;
pub const MADERA_GP_FN_EVENTLOG6_FIFO_STS: u32 = 0x155;
pub const MADERA_GP_FN_EVENTLOG7_FIFO_STS: u32 = 0x156;
pub const MADERA_GP_FN_EVENTLOG8_FIFO_STS: u32 = 0x157;

/*
 * struct madera - internal data shared by the set of Madera drivers
 *
 * This should not be used by anything except child drivers of the Madera MFD
 */
#[repr(C)]
pub struct madera {
    pub regmap: *mut regmap,
    pub regmap_32bit: *mut regmap,
    pub dev: *mut device,
    pub type_: madera_type,
    pub rev: u32,
    pub type_name: *const core::ffi::c_char,
    pub num_core_supplies: i32,
    pub core_supplies: [regulator_bulk_data; MADERA_MAX_CORE_SUPPLIES],
    pub dcvdd: *mut regulator,
    pub internal_dcvdd: bool,
    pub reset_errata: bool,
    pub pdata: madera_pdata,
    pub irq_dev: *mut device,
    pub irq_data: *mut regmap_irq_chip_data,
    pub irq: i32,
    pub mclk: [clk_bulk_data; MADERA_NUM_MCLK as usize],
    pub num_micbias: u32,
    pub num_childbias: [u32; MADERA_MAX_MICBIAS],
    pub dapm: *mut snd_soc_dapm_context,
    pub dapm_ptr_lock: mutex,
    pub hp_ena: u32,
    pub out_clamp: [bool; MADERA_MAX_HP_OUTPUT],
    pub out_shorted: [bool; MADERA_MAX_HP_OUTPUT],
    pub notifier: blocking_notifier_head,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
