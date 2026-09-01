// SPDX-License-Identifier: GPL-2.0-only
/*
 * Cirrus Logic CS48L32 audio DSP.
 *
 * Copyright (C) 2016-2018, 2020, 2022, 2025 Cirrus Logic, Inc. and
 *               Cirrus Logic International Semiconductor Ltd.
 */

// C dependencies removed from executable Rust:
// <linux/bits.h>, <sound/soc.h>, "wm_adsp.h".

pub const CS48L32_SILICON_ID: u32 = 0x48a32;

pub const CS48L32_32K_MCLK1: u32 = 0;

pub const CS48L32_SFT_RESET_MAGIC: u32 = 0x5a000000;
pub const CS48L32_SOFT_RESET_US: u32 = 2000;
pub const CS48L32_HARD_RESET_MIN_US: u32 = 1000;

pub const CS48L32_SEEN_BOOT_DONE: u32 = 1u32 << 0;
pub const CS48L32_BOOT_TIMEOUT_US: u32 = 25000;

pub const CS48L32_ASP_ENABLES1: u32 = 0x00;
pub const CS48L32_ASP_CONTROL1: u32 = 0x04;
pub const CS48L32_ASP_CONTROL2: u32 = 0x08;
pub const CS48L32_ASP_CONTROL3: u32 = 0x0c;
pub const CS48L32_ASP_FRAME_CONTROL1: u32 = 0x10;
pub const CS48L32_ASP_FRAME_CONTROL2: u32 = 0x14;
pub const CS48L32_ASP_FRAME_CONTROL5: u32 = 0x20;
pub const CS48L32_ASP_FRAME_CONTROL6: u32 = 0x24;
pub const CS48L32_ASP_DATA_CONTROL1: u32 = 0x30;
pub const CS48L32_ASP_DATA_CONTROL5: u32 = 0x40;
pub const CS48L32_SYSCLK_RATE_6MHZ: u32 = 0;
pub const CS48L32_SYSCLK_RATE_12MHZ: u32 = 1;
pub const CS48L32_SYSCLK_RATE_24MHZ: u32 = 2;
pub const CS48L32_SYSCLK_RATE_49MHZ: u32 = 3;
pub const CS48L32_SYSCLK_RATE_98MHZ: u32 = 4;
pub const CS48L32_FLLHJ_INT_MAX_N: u32 = 1023;
pub const CS48L32_FLLHJ_INT_MIN_N: u32 = 1;
pub const CS48L32_FLLHJ_FRAC_MAX_N: u32 = 255;
pub const CS48L32_FLLHJ_FRAC_MIN_N: u32 = 2;
pub const CS48L32_FLLHJ_LP_INT_MODE_THRESH: u32 = 100000;
pub const CS48L32_FLLHJ_LOW_THRESH: u32 = 192000;
pub const CS48L32_FLLHJ_MID_THRESH: u32 = 1152000;
pub const CS48L32_FLLHJ_MAX_THRESH: u32 = 13000000;
pub const CS48L32_FLLHJ_LOW_GAINS: u32 = 0x23f0;
pub const CS48L32_FLLHJ_MID_GAINS: u32 = 0x22f2;
pub const CS48L32_FLLHJ_HIGH_GAINS: u32 = 0x21f0;
pub const CS48L32_FLL_MAX_FOUT: u32 = 50000000;
pub const CS48L32_FLL_MAX_REFDIV: u32 = 8;
pub const CS48L32_FLL_CONTROL1_OFFS: u32 = 0x00;
pub const CS48L32_FLL_CONTROL2_OFFS: u32 = 0x04;
pub const CS48L32_FLL_CONTROL3_OFFS: u32 = 0x08;
pub const CS48L32_FLL_CONTROL4_OFFS: u32 = 0x0c;
pub const CS48L32_FLL_CONTROL5_OFFS: u32 = 0x10;
pub const CS48L32_FLL_CONTROL6_OFFS: u32 = 0x14;
pub const CS48L32_FLL_DIGITAL_TEST2_OFFS: u32 = 0x34;
pub const CS48L32_FLL_GPIO_CLOCK_OFFS: u32 = 0xa0;
pub const CS48L32_DSP_CLOCK_FREQ_OFFS: u32 = 0x00000;
pub const CS48L32_ASP_FMT_DSP_MODE_A: u32 = 0;
pub const CS48L32_ASP_FMT_DSP_MODE_B: u32 = 1;
pub const CS48L32_ASP_FMT_I2S_MODE: u32 = 2;
pub const CS48L32_ASP_FMT_LEFT_JUSTIFIED_MODE: u32 = 3;
pub const CS48L32_HALO_SAMPLE_RATE_RX1: u32 = 0x00080;
pub const CS48L32_HALO_SAMPLE_RATE_TX1: u32 = 0x00280;
pub const CS48L32_HALO_DSP_RATE_MASK: u32 = 0x1f;

pub const CS48L32_PDMCLK_SRC_IN1_PDMCLK: u32 = 0x0;
pub const CS48L32_PDMCLK_SRC_IN2_PDMCLK: u32 = 0x1;
pub const CS48L32_PDMCLK_SRC_IN3_PDMCLK: u32 = 0x2;
pub const CS48L32_PDMCLK_SRC_IN4_PDMCLK: u32 = 0x3;
pub const CS48L32_PDMCLK_SRC_AUXPDM1_CLK: u32 = 0x8;
pub const CS48L32_PDMCLK_SRC_AUXPDM2_CLK: u32 = 0x9;

pub const CS48L32_MAX_DAI: usize = 6;
pub const CS48L32_MAX_INPUT: usize = 4;
pub const CS48L32_MAX_ANALOG_INPUT: usize = 2;
pub const CS48L32_MAX_IN_MUX_WAYS: usize = 2;
pub const CS48L32_MAX_ASP: usize = 2;

pub const CS48L32_EQ_BLOCK_SZ: usize = 60;
pub const CS48L32_N_EQ_BLOCKS: usize = 4;

pub const CS48L32_DSP_N_RX_CHANNELS: usize = 8;
pub const CS48L32_DSP_N_TX_CHANNELS: usize = 8;

pub const CS48L32_LHPF_MAX_COEFF: u32 = 4095;
pub const CS48L32_EQ_MAX_COEFF: u32 = 4095;

macro_rules! CS48L32_MIXER_CONTROLS {
    ($name:expr, $base:expr) => {
        SOC_SINGLE_RANGE_TLV!(concat!($name, " Input 1 Volume"), $base,
            CS48L32_MIXER_VOL_SHIFT, 0x20, 0x50, 0, cs48l32_mixer_tlv),
        SOC_SINGLE_RANGE_TLV!(concat!($name, " Input 2 Volume"), $base + 4,
            CS48L32_MIXER_VOL_SHIFT, 0x20, 0x50, 0, cs48l32_mixer_tlv),
        SOC_SINGLE_RANGE_TLV!(concat!($name, " Input 3 Volume"), $base + 8,
            CS48L32_MIXER_VOL_SHIFT, 0x20, 0x50, 0, cs48l32_mixer_tlv),
        SOC_SINGLE_RANGE_TLV!(concat!($name, " Input 4 Volume"), $base + 12,
            CS48L32_MIXER_VOL_SHIFT, 0x20, 0x50, 0, cs48l32_mixer_tlv)
    };
}

macro_rules! CS48L32_MUX_ENUM_DECL {
    ($name:ident, $reg:expr) => {
        SOC_VALUE_ENUM_SINGLE_DECL!($name, $reg, 0, CS48L32_MIXER_SRC_MASK,
            cs48l32_mixer_texts, cs48l32_mixer_values)
    };
}

// C token-pasting macros retained by intent. A direct Rust macro expansion of
// name##_mux/name##_enum requires an external identifier-concatenation helper.
// CS48L32_MUX_CTL_DECL(name):
// const struct snd_kcontrol_new name##_mux = SOC_DAPM_ENUM("Route", name##_enum)
// CS48L32_MUX_ENUMS(name, base_reg):
// static CS48L32_MUX_ENUM_DECL(name##_enum, base_reg);
// static CS48L32_MUX_CTL_DECL(name)
// CS48L32_MIXER_ENUMS(name, base_reg):
// CS48L32_MUX_ENUMS(name##_in1, base_reg);
// CS48L32_MUX_ENUMS(name##_in2, base_reg + 4);
// CS48L32_MUX_ENUMS(name##_in3, base_reg + 8);
// CS48L32_MUX_ENUMS(name##_in4, base_reg + 12)

macro_rules! CS48L32_MUX {
    ($name:expr, $ctrl:expr) => {
        SND_SOC_DAPM_MUX!($name, SND_SOC_NOPM, 0, 0, $ctrl)
    };
}

// C token-pasting in name##_mux/name##_inN_mux is preserved as intent here.
// CS48L32_MUX_WIDGETS(name, name_str): CS48L32_MUX(name_str " Input 1", &name##_mux)
// CS48L32_MIXER_WIDGETS(name, name_str):
// CS48L32_MUX(name_str " Input 1", &name##_in1_mux), ...
// SND_SOC_DAPM_MIXER(name_str " Mixer", SND_SOC_NOPM, 0, 0, NULL, 0)

macro_rules! CS48L32_MUX_ROUTES {
    ($widget:expr, $name:expr) => {
        { $widget, core::ptr::null(), concat!($name, " Input 1") },
        CS48L32_MIXER_INPUT_ROUTES!(concat!($name, " Input 1"))
    };
}

macro_rules! CS48L32_MIXER_ROUTES {
    ($widget:expr, $name:expr) => {
        { $widget, core::ptr::null(), concat!($name, " Mixer") },
        { concat!($name, " Mixer"), core::ptr::null(), concat!($name, " Input 1") },
        { concat!($name, " Mixer"), core::ptr::null(), concat!($name, " Input 2") },
        { concat!($name, " Mixer"), core::ptr::null(), concat!($name, " Input 3") },
        { concat!($name, " Mixer"), core::ptr::null(), concat!($name, " Input 4") },
        CS48L32_MIXER_INPUT_ROUTES!(concat!($name, " Input 1")),
        CS48L32_MIXER_INPUT_ROUTES!(concat!($name, " Input 2")),
        CS48L32_MIXER_INPUT_ROUTES!(concat!($name, " Input 3")),
        CS48L32_MIXER_INPUT_ROUTES!(concat!($name, " Input 4"))
    };
}

macro_rules! CS48L32_DSP_ROUTES_1_8_SYSCLK {
    ($name:expr) => {
        { $name, core::ptr::null(), concat!($name, " Preloader") },
        { $name, core::ptr::null(), "SYSCLK" },
        { concat!($name, " Preload"), core::ptr::null(), concat!($name, " Preloader") },
        CS48L32_MIXER_ROUTES!($name, concat!($name, "RX1")),
        CS48L32_MIXER_ROUTES!($name, concat!($name, "RX2")),
        CS48L32_MIXER_ROUTES!($name, concat!($name, "RX3")),
        CS48L32_MIXER_ROUTES!($name, concat!($name, "RX4")),
        CS48L32_MIXER_ROUTES!($name, concat!($name, "RX5")),
        CS48L32_MIXER_ROUTES!($name, concat!($name, "RX6")),
        CS48L32_MIXER_ROUTES!($name, concat!($name, "RX7")),
        CS48L32_MIXER_ROUTES!($name, concat!($name, "RX8"))
    };
}

macro_rules! CS48L32_DSP_ROUTES_1_8 {
    ($name:expr) => {
        { $name, core::ptr::null(), "DSPCLK" },
        CS48L32_DSP_ROUTES_1_8_SYSCLK!($name)
    };
}

macro_rules! CS48L32_RATE_CONTROL {
    ($name:expr, $domain:expr) => {
        SOC_ENUM!($name, cs48l32_sample_rate[($domain) - 1])
    };
}

macro_rules! CS48L32_RATE_ENUM {
    ($name:expr, $enum_:expr) => {
        SOC_ENUM_EXT!($name, $enum_, snd_soc_get_enum_double, cs48l32_rate_put)
    };
}

macro_rules! CS48L32_DSP_RATE_CONTROL {
    ($name:expr, $num:expr) => {
        SOC_ENUM_EXT!(concat!($name, " Rate"), cs48l32_dsp_rate_enum[$num],
            cs48l32_dsp_rate_get, cs48l32_dsp_rate_put)
    };
}

// CS48L32_EQ_COEFF_CONTROL and derived EQ coefficient macros use C compound
// literals, designated initializers, stringification, and token pasting.
// Their source-level forms are preserved in comments for the future ALSA Rust
// binding layer that supplies snd_kcontrol_new and related constructors.
// CS48L32_EQ_REG_NAME_PASTER(eq, band, type): CS48L32_ ## eq ## _ ## band ## _ ## type
// CS48L32_EQ_BAND_COEFF_CONTROLS(name, band): A/B/C/PG controls for one EQ band.
// CS48L32_EQ_COEFF_CONTROLS(name): BAND1..BAND5 coefficient controls.
// CS48L32_LHPF_CONTROL(xname, xbase): snd_kcontrol_new byte control with soc_bytes.

/* these have a subseq number so they run after SYSCLK and DSPCLK widgets */
macro_rules! CS48L32_DSP_FREQ_WIDGET_EV {
    ($name:expr, $num:expr, $event:expr) => {
        SND_SOC_DAPM_SUPPLY_S!(concat!($name, "FREQ"), 100, SND_SOC_NOPM, $num, 0,
            $event, SND_SOC_DAPM_POST_PMU)
    };
}

pub const CS48L32_RATES: u32 = SNDRV_PCM_RATE_KNOT;

pub const CS48L32_FORMATS: u32 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

macro_rules! CS48L32_MIXER_INPUT_ROUTES {
    ($name:expr) => {
        { $name, "Tone Generator 1", "Tone Generator 1" },
        { $name, "Tone Generator 2", "Tone Generator 2" },
        { $name, "Noise Generator", "Noise Generator" },
        { $name, "IN1L", "IN1L PGA" },
        { $name, "IN1R", "IN1R PGA" },
        { $name, "IN2L", "IN2L PGA" },
        { $name, "IN2R", "IN2R PGA" },
        { $name, "ASP1RX1", "ASP1RX1" },
        { $name, "ASP1RX2", "ASP1RX2" },
        { $name, "ASP1RX3", "ASP1RX3" },
        { $name, "ASP1RX4", "ASP1RX4" },
        { $name, "ASP1RX5", "ASP1RX5" },
        { $name, "ASP1RX6", "ASP1RX6" },
        { $name, "ASP1RX7", "ASP1RX7" },
        { $name, "ASP1RX8", "ASP1RX8" },
        { $name, "ASP2RX1", "ASP2RX1" },
        { $name, "ASP2RX2", "ASP2RX2" },
        { $name, "ASP2RX3", "ASP2RX3" },
        { $name, "ASP2RX4", "ASP2RX4" },
        { $name, "ISRC1DEC1", "ISRC1DEC1" },
        { $name, "ISRC1DEC2", "ISRC1DEC2" },
        { $name, "ISRC1DEC3", "ISRC1DEC3" },
        { $name, "ISRC1DEC4", "ISRC1DEC4" },
        { $name, "ISRC1INT1", "ISRC1INT1" },
        { $name, "ISRC1INT2", "ISRC1INT2" },
        { $name, "ISRC1INT3", "ISRC1INT3" },
        { $name, "ISRC1INT4", "ISRC1INT4" },
        { $name, "ISRC2DEC1", "ISRC2DEC1" },
        { $name, "ISRC2DEC2", "ISRC2DEC2" },
        { $name, "ISRC2INT1", "ISRC2INT1" },
        { $name, "ISRC2INT2", "ISRC2INT2" },
        { $name, "ISRC3DEC1", "ISRC3DEC1" },
        { $name, "ISRC3DEC2", "ISRC3DEC2" },
        { $name, "ISRC3INT1", "ISRC3INT1" },
        { $name, "ISRC3INT2", "ISRC3INT2" },
        { $name, "EQ1", "EQ1" },
        { $name, "EQ2", "EQ2" },
        { $name, "EQ3", "EQ3" },
        { $name, "EQ4", "EQ4" },
        { $name, "DRC1L", "DRC1L" },
        { $name, "DRC1R", "DRC1R" },
        { $name, "DRC2L", "DRC2L" },
        { $name, "DRC2R", "DRC2R" },
        { $name, "LHPF1", "LHPF1" },
        { $name, "LHPF2", "LHPF2" },
        { $name, "LHPF3", "LHPF3" },
        { $name, "LHPF4", "LHPF4" },
        { $name, "Ultrasonic 1", "Ultrasonic 1" },
        { $name, "Ultrasonic 2", "Ultrasonic 2" },
        { $name, "DSP1.1", "DSP1" },
        { $name, "DSP1.2", "DSP1" },
        { $name, "DSP1.3", "DSP1" },
        { $name, "DSP1.4", "DSP1" },
        { $name, "DSP1.5", "DSP1" },
        { $name, "DSP1.6", "DSP1" },
        { $name, "DSP1.7", "DSP1" },
        { $name, "DSP1.8", "DSP1" }
    };
}

#[repr(C)]
pub struct cs48l32_enum {
    pub mixer_enum: soc_enum,
    pub val: core::ffi::c_int,
}

#[repr(C)]
pub struct cs48l32_eq_control {
    pub reg: core::ffi::c_uint,
    pub shift: core::ffi::c_uint,
    pub block_base: core::ffi::c_uint,
    pub max: core::ffi::c_uint,
}

#[repr(C)]
pub struct cs48l32_dai_priv {
    pub clk: core::ffi::c_int,
    pub constraint: snd_pcm_hw_constraint_list,
}

#[repr(C)]
pub struct cs48l32_dsp_power_reg_block {
    pub start: core::ffi::c_uint,
    pub end: core::ffi::c_uint,
}

#[repr(C)]
pub struct cs48l32_dsp_power_regs {
    pub pwd: *const core::ffi::c_uint,
    pub n_pwd: core::ffi::c_uint,
    pub ext: *const cs48l32_dsp_power_reg_block,
    pub n_ext: core::ffi::c_uint,
}

// Forward declarations supplied by other files in C:
// struct cs48l32;
// struct cs48l32_codec;
// struct spi_device;

#[repr(C)]
pub struct cs48l32_fll_cfg {
    pub n: core::ffi::c_int,
    pub theta: core::ffi::c_uint,
    pub lambda: core::ffi::c_uint,
    pub refdiv: core::ffi::c_int,
    pub fratio: core::ffi::c_int,
    pub gain: core::ffi::c_int,
    pub alt_gain: core::ffi::c_int,
}

#[repr(C)]
pub struct cs48l32_fll {
    pub codec: *mut cs48l32_codec,
    pub id: core::ffi::c_int,
    pub base: core::ffi::c_uint,

    pub sts_addr: core::ffi::c_uint,
    pub sts_mask: core::ffi::c_uint,
    pub fout: core::ffi::c_uint,
    pub ref_src: core::ffi::c_int,
    pub ref_freq: core::ffi::c_uint,

    pub ref_cfg: cs48l32_fll_cfg,
}

#[repr(C)]
pub struct cs48l32_codec {
    pub dsp: wm_adsp, /* must be first */
    pub core: cs48l32,
    pub sysclk: core::ffi::c_int,
    pub dspclk: core::ffi::c_int,
    pub dai: [cs48l32_dai_priv; CS48L32_MAX_DAI],
    pub fll: cs48l32_fll,

    pub in_up_pending: core::ffi::c_uint,
    pub in_vu_reg: core::ffi::c_uint,

    pub rate_lock: mutex,

    pub dsp_dma_rates: [u8; CS48L32_DSP_N_RX_CHANNELS + CS48L32_DSP_N_TX_CHANNELS],

    pub in_type: [[u8; CS48L32_MAX_IN_MUX_WAYS]; CS48L32_MAX_ANALOG_INPUT],
    pub pdm_sup: [u8; CS48L32_MAX_ANALOG_INPUT],
    pub tdm_width: [u8; CS48L32_MAX_ASP],
    pub tdm_slots: [u8; CS48L32_MAX_ASP],

    pub eq_mode: [core::ffi::c_uint; CS48L32_N_EQ_BLOCKS],
    pub eq_coefficients: [[__be16; CS48L32_EQ_BLOCK_SZ / 2]; CS48L32_N_EQ_BLOCKS],

    pub dsp_power_regs: *const cs48l32_dsp_power_regs,
}

macro_rules! cs48l32_fll_err {
    ($_fll:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        dev_err!((*(*$_fll).codec).core.dev, concat!("FLL%d: ", $fmt), (*$_fll).id $(, $args)*)
    };
}

macro_rules! cs48l32_fll_warn {
    ($_fll:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        dev_warn!((*(*$_fll).codec).core.dev, concat!("FLL%d: ", $fmt), (*$_fll).id $(, $args)*)
    };
}

macro_rules! cs48l32_fll_dbg {
    ($_fll:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        dev_dbg!((*(*$_fll).codec).core.dev, concat!("FLL%d: ", $fmt), (*$_fll).id $(, $args)*)
    };
}

macro_rules! cs48l32_asp_err {
    ($_dai:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        dev_err!((*(*$_dai).component).dev, concat!("ASP%d: ", $fmt), (*$_dai).id $(, $args)*)
    };
}

macro_rules! cs48l32_asp_warn {
    ($_dai:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        dev_warn!((*(*$_dai).component).dev, concat!("ASP%d: ", $fmt), (*$_dai).id $(, $args)*)
    };
}

macro_rules! cs48l32_asp_dbg {
    ($_dai:expr, $fmt:expr $(, $args:expr)* $(,)?) => {
        dev_dbg!((*(*$_dai).component).dev, concat!("ASP%d: ", $fmt), (*$_dai).id $(, $args)*)
    };
}

unsafe extern "C" {
    pub fn cs48l32_apply_patch(cs48l32: *mut cs48l32) -> core::ffi::c_int;
    pub fn cs48l32_create_regmap(spi: *mut spi_device, cs48l32: *mut cs48l32) -> core::ffi::c_int;
    pub fn cs48l32_enable_asp1_pins(cs48l32_codec: *mut cs48l32_codec) -> core::ffi::c_int;
    pub fn cs48l32_enable_asp2_pins(cs48l32_codec: *mut cs48l32_codec) -> core::ffi::c_int;
    pub fn cs48l32_micvdd_voltage_index(voltage: u32) -> core::ffi::c_int;
    pub fn cs48l32_micbias1_voltage_index(voltage: u32) -> core::ffi::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
