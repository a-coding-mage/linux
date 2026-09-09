/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Arizona MFD internals
 *
 * Copyright 2012 Wolfson Microelectronics plc
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// Dependencies supplied by the surrounding kernel translation are intentionally
// referenced here rather than reimplemented.

pub const ARIZONA_MAX_CORE_SUPPLIES: usize = 2;

pub const ARIZONA_MCLK1: u32 = 0;
pub const ARIZONA_MCLK2: u32 = 1;
pub const ARIZONA_NUM_MCLK: usize = 2;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum arizona_type {
    WM5102 = 1,
    WM5110 = 2,
    WM8997 = 3,
    WM8280 = 4,
    WM8998 = 5,
    WM1814 = 6,
    WM1831 = 7,
    CS47L24 = 8,
}

pub const ARIZONA_IRQ_GP1: u32 = 0;
pub const ARIZONA_IRQ_GP2: u32 = 1;
pub const ARIZONA_IRQ_GP3: u32 = 2;
pub const ARIZONA_IRQ_GP4: u32 = 3;
pub const ARIZONA_IRQ_GP5_FALL: u32 = 4;
pub const ARIZONA_IRQ_GP5_RISE: u32 = 5;
pub const ARIZONA_IRQ_JD_FALL: u32 = 6;
pub const ARIZONA_IRQ_JD_RISE: u32 = 7;
pub const ARIZONA_IRQ_DSP1_RAM_RDY: u32 = 8;
pub const ARIZONA_IRQ_DSP2_RAM_RDY: u32 = 9;
pub const ARIZONA_IRQ_DSP3_RAM_RDY: u32 = 10;
pub const ARIZONA_IRQ_DSP4_RAM_RDY: u32 = 11;
pub const ARIZONA_IRQ_DSP_IRQ1: u32 = 12;
pub const ARIZONA_IRQ_DSP_IRQ2: u32 = 13;
pub const ARIZONA_IRQ_DSP_IRQ3: u32 = 14;
pub const ARIZONA_IRQ_DSP_IRQ4: u32 = 15;
pub const ARIZONA_IRQ_DSP_IRQ5: u32 = 16;
pub const ARIZONA_IRQ_DSP_IRQ6: u32 = 17;
pub const ARIZONA_IRQ_DSP_IRQ7: u32 = 18;
pub const ARIZONA_IRQ_DSP_IRQ8: u32 = 19;
pub const ARIZONA_IRQ_SPK_OVERHEAT_WARN: u32 = 20;
pub const ARIZONA_IRQ_SPK_OVERHEAT: u32 = 21;
pub const ARIZONA_IRQ_MICDET: u32 = 22;
pub const ARIZONA_IRQ_HPDET: u32 = 23;
pub const ARIZONA_IRQ_WSEQ_DONE: u32 = 24;
pub const ARIZONA_IRQ_DRC2_SIG_DET: u32 = 25;
pub const ARIZONA_IRQ_DRC1_SIG_DET: u32 = 26;
pub const ARIZONA_IRQ_ASRC2_LOCK: u32 = 27;
pub const ARIZONA_IRQ_ASRC1_LOCK: u32 = 28;
pub const ARIZONA_IRQ_UNDERCLOCKED: u32 = 29;
pub const ARIZONA_IRQ_OVERCLOCKED: u32 = 30;
pub const ARIZONA_IRQ_FLL2_LOCK: u32 = 31;
pub const ARIZONA_IRQ_FLL1_LOCK: u32 = 32;
pub const ARIZONA_IRQ_CLKGEN_ERR: u32 = 33;
pub const ARIZONA_IRQ_CLKGEN_ERR_ASYNC: u32 = 34;
pub const ARIZONA_IRQ_ASRC_CFG_ERR: u32 = 35;
pub const ARIZONA_IRQ_AIF3_ERR: u32 = 36;
pub const ARIZONA_IRQ_AIF2_ERR: u32 = 37;
pub const ARIZONA_IRQ_AIF1_ERR: u32 = 38;
pub const ARIZONA_IRQ_CTRLIF_ERR: u32 = 39;
pub const ARIZONA_IRQ_MIXER_DROPPED_SAMPLES: u32 = 40;
pub const ARIZONA_IRQ_ASYNC_CLK_ENA_LOW: u32 = 41;
pub const ARIZONA_IRQ_SYSCLK_ENA_LOW: u32 = 42;
pub const ARIZONA_IRQ_ISRC1_CFG_ERR: u32 = 43;
pub const ARIZONA_IRQ_ISRC2_CFG_ERR: u32 = 44;
pub const ARIZONA_IRQ_BOOT_DONE: u32 = 45;
pub const ARIZONA_IRQ_DCS_DAC_DONE: u32 = 46;
pub const ARIZONA_IRQ_DCS_HP_DONE: u32 = 47;
pub const ARIZONA_IRQ_FLL2_CLOCK_OK: u32 = 48;
pub const ARIZONA_IRQ_FLL1_CLOCK_OK: u32 = 49;
pub const ARIZONA_IRQ_MICD_CLAMP_RISE: u32 = 50;
pub const ARIZONA_IRQ_MICD_CLAMP_FALL: u32 = 51;
pub const ARIZONA_IRQ_HP3R_DONE: u32 = 52;
pub const ARIZONA_IRQ_HP3L_DONE: u32 = 53;
pub const ARIZONA_IRQ_HP2R_DONE: u32 = 54;
pub const ARIZONA_IRQ_HP2L_DONE: u32 = 55;
pub const ARIZONA_IRQ_HP1R_DONE: u32 = 56;
pub const ARIZONA_IRQ_HP1L_DONE: u32 = 57;
pub const ARIZONA_IRQ_ISRC3_CFG_ERR: u32 = 58;
pub const ARIZONA_IRQ_DSP_SHARED_WR_COLL: u32 = 59;
pub const ARIZONA_IRQ_SPK_SHUTDOWN: u32 = 60;
pub const ARIZONA_IRQ_SPK1R_SHORT: u32 = 61;
pub const ARIZONA_IRQ_SPK1L_SHORT: u32 = 62;
pub const ARIZONA_IRQ_HP3R_SC_NEG: u32 = 63;
pub const ARIZONA_IRQ_HP3R_SC_POS: u32 = 64;
pub const ARIZONA_IRQ_HP3L_SC_NEG: u32 = 65;
pub const ARIZONA_IRQ_HP3L_SC_POS: u32 = 66;
pub const ARIZONA_IRQ_HP2R_SC_NEG: u32 = 67;
pub const ARIZONA_IRQ_HP2R_SC_POS: u32 = 68;
pub const ARIZONA_IRQ_HP2L_SC_NEG: u32 = 69;
pub const ARIZONA_IRQ_HP2L_SC_POS: u32 = 70;
pub const ARIZONA_IRQ_HP1R_SC_NEG: u32 = 71;
pub const ARIZONA_IRQ_HP1R_SC_POS: u32 = 72;
pub const ARIZONA_IRQ_HP1L_SC_NEG: u32 = 73;
pub const ARIZONA_IRQ_HP1L_SC_POS: u32 = 74;
pub const ARIZONA_NUM_IRQ: usize = 75;

#[repr(C)]
pub struct arizona {
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub r#type: arizona_type,
    pub rev: u32,
    pub num_core_supplies: i32,
    pub core_supplies: [regulator_bulk_data; ARIZONA_MAX_CORE_SUPPLIES],
    pub dcvdd: *mut regulator,
    pub has_fully_powered_off: bool,
    pub pdata: arizona_pdata,
    pub external_dcvdd: u32,
    pub irq: i32,
    pub virq: *mut irq_domain,
    pub aod_irq_chip: *mut regmap_irq_chip_data,
    pub irq_chip: *mut regmap_irq_chip_data,
    pub hpdet_clamp: bool,
    pub hp_ena: u32,
    pub clk_lock: mutex,
    pub clk32k_ref: i32,
    pub mclk: [*mut clk; ARIZONA_NUM_MCLK],
    pub ctrlif_error: bool,
    pub dapm: *mut snd_soc_dapm_context,
    pub tdm_width: [i32; ARIZONA_MAX_AIF],
    pub tdm_slots: [i32; ARIZONA_MAX_AIF],
    pub dac_comp_coeff: u16,
    pub dac_comp_enabled: u8,
    pub dac_comp_lock: mutex,
    pub notifier: blocking_notifier_head,
}

#[inline]
pub unsafe fn arizona_call_notifiers(arizona: *mut arizona, event: c_ulong, data: *mut c_void) -> i32 {
    blocking_notifier_call_chain(&mut (*arizona).notifier, event, data)
}

extern "C" {
    pub fn arizona_clk32k_enable(arizona: *mut arizona) -> i32;
    pub fn arizona_clk32k_disable(arizona: *mut arizona) -> i32;
    pub fn arizona_request_irq(arizona: *mut arizona, irq: i32, name: *mut c_char, handler: irq_handler_t, data: *mut c_void) -> i32;
    pub fn arizona_free_irq(arizona: *mut arizona, irq: i32, data: *mut c_void);
    pub fn arizona_set_irq_wake(arizona: *mut arizona, irq: i32, on: i32) -> i32;
    #[cfg(feature = "CONFIG_MFD_WM5102")]
    pub fn wm5102_patch(arizona: *mut arizona) -> i32;
    pub fn wm5110_patch(arizona: *mut arizona) -> i32;
    pub fn cs47l24_patch(arizona: *mut arizona) -> i32;
    pub fn wm8997_patch(arizona: *mut arizona) -> i32;
    pub fn wm8998_patch(arizona: *mut arizona) -> i32;
}

#[cfg(not(feature = "CONFIG_MFD_WM5102"))]
#[inline]
pub unsafe fn wm5102_patch(_arizona: *mut arizona) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
