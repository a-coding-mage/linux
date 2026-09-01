// SPDX-License-Identifier: GPL-2.0-only
//
// nau8325.c -- Nuvoton NAU8325 audio codec driver
//
// Copyright 2023 Nuvoton Technology Crop.
// Author: Seven Lee <WTLI@nuvoton.com>
//	   David Lin <CTLIN0@nuvoton.com>
//

use core::ptr;

/* Range of Master Clock MCLK (Hz) */
const MASTER_CLK_MAX: u32 = 49152000;
const MASTER_CLK_MIN: u32 = 2048000;

/* scaling for MCLK source */
const CLK_PROC_BYPASS: i32 = -1;

/* the maximum CLK_DAC */
const CLK_DA_AD_MAX: u32 = 6144000;

/* from MCLK input */
const MCLK_SRC: i32 = 4;

static mclk_n1_div: [nau8325_src_attr; 3] = [
    nau8325_src_attr { param: 1, val: 0x0 },
    nau8325_src_attr { param: 2, val: 0x1 },
    nau8325_src_attr { param: 3, val: 0x2 },
];

/* over sampling rate */
static osr_dac_sel: [nau8325_osr_attr; 5] = [
    nau8325_osr_attr { osr: 64, clk_src: 2 }, /* OSR 64, SRC 1/4 */
    nau8325_osr_attr { osr: 256, clk_src: 0 }, /* OSR 256, SRC 1 */
    nau8325_osr_attr { osr: 128, clk_src: 1 }, /* OSR 128, SRC 1/2 */
    nau8325_osr_attr { osr: 0, clk_src: 0 },
    nau8325_osr_attr { osr: 32, clk_src: 3 }, /* OSR 32, SRC 1/8 */
];

static mclk_n2_div: [nau8325_src_attr; 5] = [
    nau8325_src_attr { param: 0, val: 0x0 },
    nau8325_src_attr { param: 1, val: 0x1 },
    nau8325_src_attr { param: 2, val: 0x2 },
    nau8325_src_attr { param: 3, val: 0x3 },
    nau8325_src_attr { param: 4, val: 0x4 },
];

static mclk_n3_mult: [nau8325_src_attr; 4] = [
    nau8325_src_attr { param: 0, val: 0x1 },
    nau8325_src_attr { param: 1, val: 0x2 },
    nau8325_src_attr { param: 2, val: 0x3 },
    nau8325_src_attr { param: 3, val: 0x4 },
];

/* Sample Rate and MCLK_SRC selections */
static target_srate_table: [nau8325_srate_attr; 9] = [
    /* { FS, range, max, { MCLK source }} */
    nau8325_srate_attr { fs: 48000, range: 2, max: true, mclk_src: [12288000, 19200000, 24000000] },
    nau8325_srate_attr { fs: 16000, range: 1, max: false, mclk_src: [4096000, 6400000, 8000000] },
    nau8325_srate_attr { fs: 8000, range: 0, max: false, mclk_src: [2048000, 3200000, 4000000] },
    nau8325_srate_attr { fs: 44100, range: 2, max: true, mclk_src: [11289600, 17640000, 22050000] },
    nau8325_srate_attr { fs: 64000, range: 3, max: false, mclk_src: [16384000, 25600000, 32000000] },
    nau8325_srate_attr { fs: 96000, range: 3, max: true, mclk_src: [24576000, 38400000, 48000000] },
    nau8325_srate_attr { fs: 12000, range: 0, max: true, mclk_src: [3072000, 4800000, 6000000] },
    nau8325_srate_attr { fs: 24000, range: 1, max: true, mclk_src: [6144000, 9600000, 12000000] },
    nau8325_srate_attr { fs: 32000, range: 2, max: false, mclk_src: [8192000, 12800000, 16000000] },
];

static nau8325_reg_defaults: [reg_default; 35] = [
    reg_default { reg: NAU8325_R00_HARDWARE_RST, def: 0x0000 },
    reg_default { reg: NAU8325_R01_SOFTWARE_RST, def: 0x0000 },
    reg_default { reg: NAU8325_R03_CLK_CTRL, def: 0x0000 },
    reg_default { reg: NAU8325_R04_ENA_CTRL, def: 0x0000 },
    reg_default { reg: NAU8325_R05_INTERRUPT_CTRL, def: 0x007f },
    reg_default { reg: NAU8325_R09_IRQOUT, def: 0x0000 },
    reg_default { reg: NAU8325_R0A_IO_CTRL, def: 0x0000 },
    reg_default { reg: NAU8325_R0B_PDM_CTRL, def: 0x0000 },
    reg_default { reg: NAU8325_R0C_TDM_CTRL, def: 0x0000 },
    reg_default { reg: NAU8325_R0D_I2S_PCM_CTRL1, def: 0x000a },
    reg_default { reg: NAU8325_R0E_I2S_PCM_CTRL2, def: 0x0000 },
    reg_default { reg: NAU8325_R0F_L_TIME_SLOT, def: 0x0000 },
    reg_default { reg: NAU8325_R10_R_TIME_SLOT, def: 0x0000 },
    reg_default { reg: NAU8325_R11_HPF_CTRL, def: 0x0000 },
    reg_default { reg: NAU8325_R12_MUTE_CTRL, def: 0x0000 },
    reg_default { reg: NAU8325_R13_DAC_VOLUME, def: 0xf3f3 },
    reg_default { reg: NAU8325_R29_DAC_CTRL1, def: 0x0081 },
    reg_default { reg: NAU8325_R2A_DAC_CTRL2, def: 0x0000 },
    reg_default { reg: NAU8325_R2C_ALC_CTRL1, def: 0x000e },
    reg_default { reg: NAU8325_R2D_ALC_CTRL2, def: 0x8400 },
    reg_default { reg: NAU8325_R2E_ALC_CTRL3, def: 0x0000 },
    reg_default { reg: NAU8325_R2F_ALC_CTRL4, def: 0x003f },
    reg_default { reg: NAU8325_R40_CLK_DET_CTRL, def: 0xa801 },
    reg_default { reg: NAU8325_R50_MIXER_CTRL, def: 0x0000 },
    reg_default { reg: NAU8325_R55_MISC_CTRL, def: 0x0000 },
    reg_default { reg: NAU8325_R60_BIAS_ADJ, def: 0x0000 },
    reg_default { reg: NAU8325_R61_ANALOG_CONTROL_1, def: 0x0000 },
    reg_default { reg: NAU8325_R62_ANALOG_CONTROL_2, def: 0x0000 },
    reg_default { reg: NAU8325_R63_ANALOG_CONTROL_3, def: 0x0000 },
    reg_default { reg: NAU8325_R64_ANALOG_CONTROL_4, def: 0x0000 },
    reg_default { reg: NAU8325_R65_ANALOG_CONTROL_5, def: 0x0000 },
    reg_default { reg: NAU8325_R66_ANALOG_CONTROL_6, def: 0x0000 },
    reg_default { reg: NAU8325_R69_CLIP_CTRL, def: 0x0000 },
    reg_default { reg: NAU8325_R73_RDAC, def: 0x0008 },
];

unsafe extern "C" fn nau8325_readable_reg(_dev: *mut device, reg: u32) -> bool {
    match reg {
        NAU8325_R02_DEVICE_ID..=NAU8325_R06_INT_CLR_STATUS
        | NAU8325_R09_IRQOUT..=NAU8325_R13_DAC_VOLUME
        | NAU8325_R1D_DEBUG_READ1
        | NAU8325_R1F_DEBUG_READ2
        | NAU8325_R22_DEBUG_READ3
        | NAU8325_R29_DAC_CTRL1..=NAU8325_R2A_DAC_CTRL2
        | NAU8325_R2C_ALC_CTRL1..=NAU8325_R2F_ALC_CTRL4
        | NAU8325_R40_CLK_DET_CTRL
        | NAU8325_R49_TEST_STATUS..=NAU8325_R4A_ANALOG_READ
        | NAU8325_R50_MIXER_CTRL
        | NAU8325_R55_MISC_CTRL
        | NAU8325_R60_BIAS_ADJ..=NAU8325_R66_ANALOG_CONTROL_6
        | NAU8325_R69_CLIP_CTRL
        | NAU8325_R73_RDAC => true,
        _ => false,
    }
}

unsafe extern "C" fn nau8325_writeable_reg(_dev: *mut device, reg: u32) -> bool {
    match reg {
        NAU8325_R00_HARDWARE_RST..=NAU8325_R01_SOFTWARE_RST
        | NAU8325_R03_CLK_CTRL..=NAU8325_R06_INT_CLR_STATUS
        | NAU8325_R09_IRQOUT..=NAU8325_R13_DAC_VOLUME
        | NAU8325_R29_DAC_CTRL1..=NAU8325_R2A_DAC_CTRL2
        | NAU8325_R2C_ALC_CTRL1..=NAU8325_R2F_ALC_CTRL4
        | NAU8325_R40_CLK_DET_CTRL
        | NAU8325_R50_MIXER_CTRL
        | NAU8325_R55_MISC_CTRL
        | NAU8325_R60_BIAS_ADJ..=NAU8325_R66_ANALOG_CONTROL_6
        | NAU8325_R69_CLIP_CTRL
        | NAU8325_R73_RDAC => true,
        _ => false,
    }
}

unsafe extern "C" fn nau8325_volatile_reg(_dev: *mut device, reg: u32) -> bool {
    match reg {
        NAU8325_R00_HARDWARE_RST..=NAU8325_R02_DEVICE_ID
        | NAU8325_R06_INT_CLR_STATUS
        | NAU8325_R1D_DEBUG_READ1
        | NAU8325_R1F_DEBUG_READ2
        | NAU8325_R22_DEBUG_READ3
        | NAU8325_R4A_ANALOG_READ => true,
        _ => false,
    }
}

static nau8325_dac_oversampl_texts: [*const c_char; 4] = [
    c"64".as_ptr(),
    c"256".as_ptr(),
    c"128".as_ptr(),
    c"32".as_ptr(),
];

static nau8325_dac_oversampl_values: [u32; 4] = [0, 1, 2, 4];

static nau8325_dac_oversampl_enum: soc_enum = SOC_VALUE_ENUM_SINGLE(
    NAU8325_R29_DAC_CTRL1,
    NAU8325_DAC_OVERSAMPLE_SFT,
    0x7,
    nau8325_dac_oversampl_texts.len(),
    nau8325_dac_oversampl_texts.as_ptr(),
    nau8325_dac_oversampl_values.as_ptr(),
);

static dac_vol_tlv: [u32; 4] = DECLARE_TLV_DB_MINMAX_MUTE(-8000, 600);

static nau8325_snd_controls: [snd_kcontrol_new; 9] = [
    SOC_ENUM(c"DAC Oversampling Rate".as_ptr(), &nau8325_dac_oversampl_enum),
    SOC_DOUBLE_TLV(
        c"Speaker Volume".as_ptr(),
        NAU8325_R13_DAC_VOLUME,
        NAU8325_DAC_VOLUME_L_SFT,
        NAU8325_DAC_VOLUME_R_SFT,
        NAU8325_DAC_VOLUME_R_EN,
        0,
        dac_vol_tlv.as_ptr(),
    ),
    SOC_SINGLE(c"ALC Max Gain".as_ptr(), NAU8325_R2C_ALC_CTRL1, NAU8325_ALC_MAXGAIN_SFT, NAU8325_ALC_MAXGAIN_MAX, 0),
    SOC_SINGLE(c"ALC Min Gain".as_ptr(), NAU8325_R2C_ALC_CTRL1, NAU8325_ALC_MINGAIN_SFT, NAU8325_ALC_MINGAIN_MAX, 0),
    SOC_SINGLE(c"ALC Decay Timer".as_ptr(), NAU8325_R2D_ALC_CTRL2, NAU8325_ALC_DCY_SFT, NAU8325_ALC_DCY_MAX, 0),
    SOC_SINGLE(c"ALC Attack Timer".as_ptr(), NAU8325_R2D_ALC_CTRL2, NAU8325_ALC_ATK_SFT, NAU8325_ALC_ATK_MAX, 0),
    SOC_SINGLE(c"ALC Hold Time".as_ptr(), NAU8325_R2D_ALC_CTRL2, NAU8325_ALC_HLD_SFT, NAU8325_ALC_HLD_MAX, 0),
    SOC_SINGLE(c"ALC Target Level".as_ptr(), NAU8325_R2D_ALC_CTRL2, NAU8325_ALC_LVL_SFT, NAU8325_ALC_LVL_MAX, 0),
    SOC_SINGLE(c"ALC Enable Switch".as_ptr(), NAU8325_R2E_ALC_CTRL3, NAU8325_ALC_EN_SFT, 1, 0),
];

unsafe extern "C" fn nau8325_dac_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8325 = snd_soc_component_get_drvdata(component) as *mut nau8325;

    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_update_bits((*nau8325).regmap, NAU8325_R12_MUTE_CTRL, NAU8325_SOFT_MUTE, 0);
            msleep(30);
        }
        SND_SOC_DAPM_PRE_PMD => {
            /* Soft mute the output to prevent the pop noise. */
            regmap_update_bits(
                (*nau8325).regmap,
                NAU8325_R12_MUTE_CTRL,
                NAU8325_SOFT_MUTE,
                NAU8325_SOFT_MUTE,
            );
            msleep(30);
        }
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn nau8325_powerup_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let nau8325 = snd_soc_component_get_drvdata(component) as *mut nau8325;

    if (*nau8325).clock_detection {
        return 0;
    }

    match event {
        SND_SOC_DAPM_POST_PMU => {
            regmap_update_bits(
                (*nau8325).regmap,
                NAU8325_R40_CLK_DET_CTRL,
                NAU8325_PWRUP_DFT,
                NAU8325_PWRUP_DFT,
            );
        }
        SND_SOC_DAPM_POST_PMD => {
            regmap_update_bits((*nau8325).regmap, NAU8325_R40_CLK_DET_CTRL, NAU8325_PWRUP_DFT, 0);
        }
        _ => return -EINVAL,
    }

    0
}

static nau8325_dapm_widgets: [snd_soc_dapm_widget; 6] = [
    SND_SOC_DAPM_SUPPLY(
        c"Power Up".as_ptr(),
        SND_SOC_NOPM,
        0,
        0,
        Some(nau8325_powerup_event),
        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_POST_PMD,
    ),
    SND_SOC_DAPM_DAC_E(
        c"DACL".as_ptr(),
        ptr::null(),
        NAU8325_R04_ENA_CTRL,
        NAU8325_DAC_LEFT_CH_EN_SFT,
        0,
        Some(nau8325_dac_event),
        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD,
    ),
    SND_SOC_DAPM_DAC_E(
        c"DACR".as_ptr(),
        ptr::null(),
        NAU8325_R04_ENA_CTRL,
        NAU8325_DAC_RIGHT_CH_EN_SFT,
        0,
        Some(nau8325_dac_event),
        SND_SOC_DAPM_POST_PMU | SND_SOC_DAPM_PRE_PMD,
    ),
    SND_SOC_DAPM_AIF_IN(c"AIFRX".as_ptr(), c"Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_OUTPUT(c"SPKL".as_ptr()),
    SND_SOC_DAPM_OUTPUT(c"SPKR".as_ptr()),
];

static nau8325_dapm_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: c"DACL".as_ptr(), control: ptr::null(), source: c"Power Up".as_ptr() },
    snd_soc_dapm_route { sink: c"DACR".as_ptr(), control: ptr::null(), source: c"Power Up".as_ptr() },
    snd_soc_dapm_route { sink: c"DACL".as_ptr(), control: ptr::null(), source: c"AIFRX".as_ptr() },
    snd_soc_dapm_route { sink: c"DACR".as_ptr(), control: ptr::null(), source: c"AIFRX".as_ptr() },
    snd_soc_dapm_route { sink: c"SPKL".as_ptr(), control: ptr::null(), source: c"DACL".as_ptr() },
    snd_soc_dapm_route { sink: c"SPKR".as_ptr(), control: ptr::null(), source: c"DACR".as_ptr() },
];

unsafe fn nau8325_srate_clk_apply(
    nau8325: *mut nau8325,
    srate_table: *const nau8325_srate_attr,
    n1_sel: i32,
    mclk_mult_sel: i32,
    n2_sel: i32,
) -> i32 {
    if srate_table.is_null()
        || n2_sel < 0
        || n2_sel as usize >= mclk_n2_div.len()
        || n1_sel < 0
        || n1_sel as usize >= mclk_n1_div.len()
    {
        dev_dbg((*nau8325).dev, c"The CLK isn't supported.".as_ptr());
        return -EINVAL;
    }

    regmap_update_bits(
        (*nau8325).regmap,
        NAU8325_R40_CLK_DET_CTRL,
        NAU8325_REG_SRATE_MASK | NAU8325_REG_DIV_MAX,
        ((*srate_table).range << NAU8325_REG_SRATE_SFT)
            | if (*srate_table).max { NAU8325_REG_DIV_MAX } else { 0 },
    );
    regmap_update_bits(
        (*nau8325).regmap,
        NAU8325_R03_CLK_CTRL,
        NAU8325_MCLK_SRC_MASK,
        mclk_n2_div[n2_sel as usize].val,
    );
    regmap_update_bits(
        (*nau8325).regmap,
        NAU8325_R03_CLK_CTRL,
        NAU8325_CLK_MUL_SRC_MASK,
        mclk_n1_div[n1_sel as usize].val << NAU8325_CLK_MUL_SRC_SFT,
    );

    if mclk_mult_sel != CLK_PROC_BYPASS {
        regmap_update_bits(
            (*nau8325).regmap,
            NAU8325_R03_CLK_CTRL,
            NAU8325_MCLK_SEL_MASK,
            mclk_n3_mult[mclk_mult_sel as usize].val << NAU8325_MCLK_SEL_SFT,
        );
    } else {
        regmap_update_bits((*nau8325).regmap, NAU8325_R03_CLK_CTRL, NAU8325_MCLK_SEL_MASK, 0);
    }

    match mclk_mult_sel {
        2 => regmap_update_bits(
            (*nau8325).regmap,
            NAU8325_R65_ANALOG_CONTROL_5,
            NAU8325_MCLK4XEN_EN,
            NAU8325_MCLK4XEN_EN,
        ),
        3 => regmap_update_bits(
            (*nau8325).regmap,
            NAU8325_R65_ANALOG_CONTROL_5,
            NAU8325_MCLK4XEN_EN | NAU8325_MCLK8XEN_EN,
            NAU8325_MCLK4XEN_EN | NAU8325_MCLK8XEN_EN,
        ),
        _ => regmap_update_bits(
            (*nau8325).regmap,
            NAU8325_R65_ANALOG_CONTROL_5,
            NAU8325_MCLK4XEN_EN | NAU8325_MCLK8XEN_EN,
            0,
        ),
    };

    0
}

unsafe fn nau8325_clksrc_n2(
    _nau8325: *mut nau8325,
    srate_table: *const nau8325_srate_attr,
    mclk: i32,
    n2_sel: *mut i32,
) -> i32 {
    let mut ratio = NAU8325_MCLK_FS_RATIO_NUM;
    let mut i = 0usize;

    while i < mclk_n2_div.len() {
        let mclk_src = mclk >> mclk_n2_div[i].param;
        if (*srate_table).mclk_src[NAU8325_MCLK_FS_RATIO_256 as usize] as i32 == mclk_src {
            ratio = NAU8325_MCLK_FS_RATIO_256;
            break;
        } else if (*srate_table).mclk_src[NAU8325_MCLK_FS_RATIO_400 as usize] as i32 == mclk_src {
            ratio = NAU8325_MCLK_FS_RATIO_400;
            break;
        } else if (*srate_table).mclk_src[NAU8325_MCLK_FS_RATIO_500 as usize] as i32 == mclk_src {
            ratio = NAU8325_MCLK_FS_RATIO_500;
            break;
        }
        i += 1;
    }
    if ratio != NAU8325_MCLK_FS_RATIO_NUM {
        *n2_sel = i as i32;
    }

    ratio
}

fn target_srate_attribute(srate: i32) -> *const nau8325_srate_attr {
    let mut i = 0usize;

    while i < target_srate_table.len() {
        if target_srate_table[i].fs == srate {
            break;
        }
        i += 1;
    }

    if i == target_srate_table.len() {
        return ptr::null();
    }

    &target_srate_table[i]
}

unsafe fn nau8325_clksrc_choose(
    nau8325: *mut nau8325,
    srate_table: *mut *const nau8325_srate_attr,
    n1_sel: *mut i32,
    mult_sel: *mut i32,
    n2_sel: *mut i32,
) -> i32 {
    let mut mclk_max = 0;
    let mut ratio_sel = 0;
    let mut n2_max = 0;

    if (*nau8325).mclk == 0 || (*nau8325).fs == 0 {
        dev_dbg(
            (*nau8325).dev,
            c"The MCLK %d is invalid. It can't get MCLK_SRC of 256/400/500 FS (%d)".as_ptr(),
            (*nau8325).mclk,
            (*nau8325).fs,
        );
        return -EINVAL;
    }

    /* select sampling rate and MCLK_SRC */
    *srate_table = target_srate_attribute((*nau8325).fs);
    if (*srate_table).is_null() {
        dev_dbg(
            (*nau8325).dev,
            c"The MCLK %d is invalid. It can't get MCLK_SRC of 256/400/500 FS (%d)".as_ptr(),
            (*nau8325).mclk,
            (*nau8325).fs,
        );
        return -EINVAL;
    }

    /* First check clock from MCLK directly, decide N2 for MCLK_SRC.
     * If not good, consider 1/N1 and Multiplier.
     */
    let mut ratio = nau8325_clksrc_n2(nau8325, *srate_table, (*nau8325).mclk as i32, n2_sel);
    if ratio != NAU8325_MCLK_FS_RATIO_NUM {
        *n1_sel = 0;
        *mult_sel = CLK_PROC_BYPASS;
        *n2_sel = MCLK_SRC;
    } else {
        /* Get MCLK_SRC through 1/N, Multiplier, and then 1/N2. */
        let mut i = 0usize;
        while i < mclk_n1_div.len() {
            let mut j = 0usize;
            while j < mclk_n3_mult.len() {
                let mut mclk = ((*nau8325).mclk as i32) << mclk_n3_mult[j].param;
                mclk = mclk / mclk_n1_div[i].param;
                ratio = nau8325_clksrc_n2(nau8325, *srate_table, mclk, n2_sel);
                if ratio != NAU8325_MCLK_FS_RATIO_NUM && (mclk_max < mclk || i as i32 > *n1_sel) {
                    mclk_max = mclk;
                    n2_max = *n2_sel;
                    *n1_sel = i as i32;
                    *mult_sel = j as i32;
                    ratio_sel = ratio;
                    break;
                }
                j += 1;
            }
            if mclk_max != 0 {
                break;
            }
            i += 1;
        }
        if mclk_max != 0 {
            *n2_sel = n2_max;
            ratio = ratio_sel;
        } else {
            dev_dbg(
                (*nau8325).dev,
                c"The MCLK %d is invalid. It can't get MCLK_SRC of 256/400/500 FS (%d)".as_ptr(),
                (*nau8325).mclk,
                (*nau8325).fs,
            );
            return -EINVAL;
        }
    }

    dev_dbg(
        (*nau8325).dev,
        c"nau8325->fs=%d,range=0x%x, %s, (n1,mu,n2,dmu):(%d,%d,%d), MCLK_SRC=%uHz (%d)".as_ptr(),
        (*nau8325).fs,
        (**srate_table).range,
        if (**srate_table).max { c"MAX".as_ptr() } else { c"MIN".as_ptr() },
        if *n1_sel == CLK_PROC_BYPASS { CLK_PROC_BYPASS } else { mclk_n1_div[*n1_sel as usize].param },
        if *mult_sel == CLK_PROC_BYPASS { CLK_PROC_BYPASS } else { 1 << mclk_n3_mult[*mult_sel as usize].param },
        1 << mclk_n2_div[*n2_sel as usize].param,
        (**srate_table).mclk_src[ratio as usize],
        (**srate_table).mclk_src[ratio as usize] / (*nau8325).fs as u32,
    );

    0
}

unsafe fn nau8325_clock_config(nau8325: *mut nau8325) -> i32 {
    let mut srate_table: *const nau8325_srate_attr = ptr::null();
    let mut n1_sel = 0;
    let mut mult_sel = 0;
    let mut n2_sel = 0;

    let mut ret = nau8325_clksrc_choose(nau8325, &mut srate_table, &mut n1_sel, &mut mult_sel, &mut n2_sel);
    if ret != 0 {
        return ret;
    }

    ret = nau8325_srate_clk_apply(nau8325, srate_table, n1_sel, mult_sel, n2_sel);
    if ret != 0 {
        return ret;
    }

    0
}

unsafe fn nau8325_get_osr(nau8325: *mut nau8325) -> *const nau8325_osr_attr {
    let mut osr: u32 = 0;

    regmap_read((*nau8325).regmap, NAU8325_R29_DAC_CTRL1, &mut osr);
    osr &= NAU8325_DAC_OVERSAMPLE_MASK;
    if osr as usize >= osr_dac_sel.len() {
        return ptr::null();
    }

    &osr_dac_sel[osr as usize]
}

unsafe extern "C" fn nau8325_dai_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> i32 {
    let component = (*dai).component;
    let nau8325 = snd_soc_component_get_drvdata(component) as *mut nau8325;
    let osr = nau8325_get_osr(nau8325);

    if osr.is_null() || (*osr).osr == 0 {
        return -EINVAL;
    }

    snd_pcm_hw_constraint_minmax(
        (*substream).runtime,
        SNDRV_PCM_HW_PARAM_RATE,
        0,
        CLK_DA_AD_MAX / (*osr).osr,
    )
}

unsafe extern "C" fn nau8325_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let component = (*dai).component;
    let nau8325 = snd_soc_component_get_drvdata(component) as *mut nau8325;
    let mut val_len: u32 = 0;
    let mut ret: i32;

    (*nau8325).fs = params_rate(params);
    let osr = nau8325_get_osr(nau8325);
    if osr.is_null() || (*osr).osr == 0 || (*nau8325).fs as u32 * (*osr).osr > CLK_DA_AD_MAX {
        ret = -EINVAL;
        return ret;
    }
    regmap_update_bits(
        (*nau8325).regmap,
        NAU8325_R03_CLK_CTRL,
        NAU8325_CLK_DAC_SRC_MASK,
        (*osr).clk_src << NAU8325_CLK_DAC_SRC_SFT,
    );

    ret = nau8325_clock_config(nau8325);
    if ret != 0 {
        return ret;
    }

    match params_width(params) {
        16 => val_len |= NAU8325_I2S_DL_16,
        20 => val_len |= NAU8325_I2S_DL_20,
        24 => val_len |= NAU8325_I2S_DL_24,
        32 => val_len |= NAU8325_I2S_DL_32,
        _ => {
            ret = -EINVAL;
            return ret;
        }
    }

    regmap_update_bits((*nau8325).regmap, NAU8325_R0D_I2S_PCM_CTRL1, NAU8325_I2S_DL_MASK, val_len);

    0
}

unsafe extern "C" fn nau8325_set_fmt(dai: *mut snd_soc_dai, fmt: u32) -> i32 {
    let component = (*dai).component;
    let nau8325 = snd_soc_component_get_drvdata(component) as *mut nau8325;
    let mut ctrl1_val: u32 = 0;

    match fmt & SND_SOC_DAIFMT_MASTER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_IB_NF => ctrl1_val |= NAU8325_I2S_BP_INV,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => ctrl1_val |= NAU8325_I2S_DF_I2S,
        SND_SOC_DAIFMT_LEFT_J => ctrl1_val |= NAU8325_I2S_DF_LEFT,
        SND_SOC_DAIFMT_RIGHT_J => ctrl1_val |= NAU8325_I2S_DF_RIGTH,
        SND_SOC_DAIFMT_DSP_A => ctrl1_val |= NAU8325_I2S_DF_PCM_AB,
        SND_SOC_DAIFMT_DSP_B => {
            ctrl1_val |= NAU8325_I2S_DF_PCM_AB;
            ctrl1_val |= NAU8325_I2S_PCMB_EN;
        }
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*nau8325).regmap,
        NAU8325_R0D_I2S_PCM_CTRL1,
        NAU8325_I2S_DF_MASK | NAU8325_I2S_BP_MASK | NAU8325_I2S_PCMB_EN,
        ctrl1_val,
    );

    0
}

unsafe extern "C" fn nau8325_set_sysclk(
    component: *mut snd_soc_component,
    _clk_id: i32,
    _source: i32,
    freq: u32,
    _dir: i32,
) -> i32 {
    let nau8325 = snd_soc_component_get_drvdata(component) as *mut nau8325;

    if freq < MASTER_CLK_MIN || freq > MASTER_CLK_MAX {
        dev_dbg((*nau8325).dev, c"MCLK exceeds the range, MCLK:%d".as_ptr(), freq);
        return -EINVAL;
    }

    (*nau8325).mclk = freq;
    dev_dbg((*nau8325).dev, c"MCLK %dHz".as_ptr(), (*nau8325).mclk);

    0
}

static nau8325_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    set_sysclk: Some(nau8325_set_sysclk),
    suspend_bias_off: true,
    controls: nau8325_snd_controls.as_ptr(),
    num_controls: nau8325_snd_controls.len() as u32,
    dapm_widgets: nau8325_dapm_widgets.as_ptr(),
    num_dapm_widgets: nau8325_dapm_widgets.len() as u32,
    dapm_routes: nau8325_dapm_routes.as_ptr(),
    num_dapm_routes: nau8325_dapm_routes.len() as u32,
    ..unsafe { core::mem::zeroed() }
};

static nau8325_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(nau8325_dai_startup),
    hw_params: Some(nau8325_hw_params),
    set_fmt: Some(nau8325_set_fmt),
    ..unsafe { core::mem::zeroed() }
};

const NAU8325_RATES: u32 = SNDRV_PCM_RATE_8000_96000;
const NAU8325_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_3LE;

static mut nau8325_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: NAU8325_CODEC_DAI,
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 1,
        channels_max: 2,
        rates: NAU8325_RATES,
        formats: NAU8325_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &nau8325_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

static nau8325_regmap_config: regmap_config = regmap_config {
    reg_bits: NAU8325_REG_ADDR_LEN,
    val_bits: NAU8325_REG_DATA_LEN,
    max_register: NAU8325_REG_MAX,
    readable_reg: Some(nau8325_readable_reg),
    writeable_reg: Some(nau8325_writeable_reg),
    volatile_reg: Some(nau8325_volatile_reg),
    cache_type: REGCACHE_RBTREE,
    reg_defaults: nau8325_reg_defaults.as_ptr(),
    num_reg_defaults: nau8325_reg_defaults.len() as u32,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn nau8325_reset_chip(regmap: *mut regmap) {
    regmap_write(regmap, NAU8325_R00_HARDWARE_RST, 0x0001);
    regmap_write(regmap, NAU8325_R00_HARDWARE_RST, 0x0000);
}

unsafe fn nau8325_software_reset(regmap: *mut regmap) {
    regmap_write(regmap, NAU8325_R01_SOFTWARE_RST, 0x0000);
    regmap_write(regmap, NAU8325_R01_SOFTWARE_RST, 0x0000);
}

unsafe fn nau8325_init_regs(nau8325: *mut nau8325) {
    let regmap = (*nau8325).regmap;
    let dev = (*nau8325).dev;

    /* set ALC parameters */
    regmap_update_bits(regmap, NAU8325_R2C_ALC_CTRL1, NAU8325_ALC_MAXGAIN_MASK, 0x7 << NAU8325_ALC_MAXGAIN_SFT);
    regmap_update_bits(
        regmap,
        NAU8325_R2D_ALC_CTRL2,
        NAU8325_ALC_DCY_MASK | NAU8325_ALC_ATK_MASK | NAU8325_ALC_HLD_MASK,
        (0x5 << NAU8325_ALC_DCY_SFT) | (0x3 << NAU8325_ALC_ATK_SFT) | (0x5 << NAU8325_ALC_HLD_SFT),
    );
    /* Enable ALC to avoid signal distortion when battery low. */
    if (*nau8325).alc_enable {
        regmap_update_bits(regmap, NAU8325_R2E_ALC_CTRL3, NAU8325_ALC_EN, NAU8325_ALC_EN);
    }
    if (*nau8325).clock_detection {
        regmap_update_bits(regmap, NAU8325_R40_CLK_DET_CTRL, NAU8325_CLKPWRUP_DIS | NAU8325_PWRUP_DFT, 0);
    } else {
        regmap_update_bits(
            regmap,
            NAU8325_R40_CLK_DET_CTRL,
            NAU8325_CLKPWRUP_DIS | NAU8325_PWRUP_DFT,
            NAU8325_CLKPWRUP_DIS,
        );
    }
    if (*nau8325).clock_det_data {
        regmap_update_bits(regmap, NAU8325_R40_CLK_DET_CTRL, NAU8325_APWRUP_EN, NAU8325_APWRUP_EN);
    } else {
        regmap_update_bits(regmap, NAU8325_R40_CLK_DET_CTRL, NAU8325_APWRUP_EN, 0);
    }

    /* DAC Reference Voltage Setting */
    match (*nau8325).dac_vref_microvolt {
        1800000 => regmap_update_bits(regmap, NAU8325_R73_RDAC, NAU8325_DACVREFSEL_MASK, 0 << NAU8325_DACVREFSEL_SFT),
        2700000 => regmap_update_bits(regmap, NAU8325_R73_RDAC, NAU8325_DACVREFSEL_MASK, 1 << NAU8325_DACVREFSEL_SFT),
        2880000 => regmap_update_bits(regmap, NAU8325_R73_RDAC, NAU8325_DACVREFSEL_MASK, 2 << NAU8325_DACVREFSEL_SFT),
        3060000 => regmap_update_bits(regmap, NAU8325_R73_RDAC, NAU8325_DACVREFSEL_MASK, 3 << NAU8325_DACVREFSEL_SFT),
        _ => dev_dbg(dev, c"Invalid dac-vref-microvolt %d".as_ptr(), (*nau8325).dac_vref_microvolt),
    }

    /* DAC Reference Voltage Decoupling Capacitors. */
    regmap_update_bits(regmap, NAU8325_R63_ANALOG_CONTROL_3, NAU8325_CLASSD_COARSE_GAIN_MASK, 0x4);
    /* Auto-Att Min Gain 0dB, Class-D N Driver Slew Rate -25%. */
    regmap_update_bits(regmap, NAU8325_R64_ANALOG_CONTROL_4, NAU8325_CLASSD_SLEWN_MASK, 0x7);

    /* VMID Tieoff (VMID Resistor Selection) */
    match (*nau8325).vref_impedance_ohms {
        0 => regmap_update_bits(regmap, NAU8325_R60_BIAS_ADJ, NAU8325_BIAS_VMID_SEL_MASK, 0 << NAU8325_BIAS_VMID_SEL_SFT),
        25000 => regmap_update_bits(regmap, NAU8325_R60_BIAS_ADJ, NAU8325_BIAS_VMID_SEL_MASK, 1 << NAU8325_BIAS_VMID_SEL_SFT),
        125000 => regmap_update_bits(regmap, NAU8325_R60_BIAS_ADJ, NAU8325_BIAS_VMID_SEL_MASK, 2 << NAU8325_BIAS_VMID_SEL_SFT),
        2500 => regmap_update_bits(regmap, NAU8325_R60_BIAS_ADJ, NAU8325_BIAS_VMID_SEL_MASK, 3 << NAU8325_BIAS_VMID_SEL_SFT),
        _ => dev_dbg(dev, c"Invalid vref-impedance-ohms %d".as_ptr(), (*nau8325).vref_impedance_ohms),
    }

    /* enable VMID, BIAS, DAC, DCA CLOCK, Voltage/Current Amps
     */
    regmap_update_bits(
        regmap,
        NAU8325_R61_ANALOG_CONTROL_1,
        NAU8325_DACEN_MASK
            | NAU8325_DACCLKEN_MASK
            | NAU8325_DACEN_R_MASK
            | NAU8325_DACCLKEN_R_MASK
            | NAU8325_CLASSDEN_MASK
            | NAU8325_VMDFSTENB_MASK
            | NAU8325_BIASEN_MASK
            | NAU8325_VMIDEN_MASK,
        (0x1 << NAU8325_DACEN_SFT)
            | (0x1 << NAU8325_DACCLKEN_SFT)
            | (0x1 << NAU8325_DACEN_R_SFT)
            | (0x1 << NAU8325_DACCLKEN_R_SFT)
            | (0x1 << NAU8325_CLASSDEN_SFT)
            | (0x1 << NAU8325_VMDFSTENB_SFT)
            | (0x1 << NAU8325_BIASEN_SFT)
            | 0x3,
    );

    /* Enable ALC to avoid signal distortion when battery low. */
    if (*nau8325).alc_enable {
        regmap_update_bits(regmap, NAU8325_R2E_ALC_CTRL3, NAU8325_ALC_EN, NAU8325_ALC_EN);
    }
    if (*nau8325).clock_det_data {
        regmap_update_bits(regmap, NAU8325_R40_CLK_DET_CTRL, NAU8325_APWRUP_EN, NAU8325_APWRUP_EN);
    } else {
        regmap_update_bits(regmap, NAU8325_R40_CLK_DET_CTRL, NAU8325_APWRUP_EN, 0);
    }
    if (*nau8325).clock_detection {
        regmap_update_bits(regmap, NAU8325_R40_CLK_DET_CTRL, NAU8325_CLKPWRUP_DIS | NAU8325_PWRUP_DFT, 0);
    } else {
        regmap_update_bits(
            regmap,
            NAU8325_R40_CLK_DET_CTRL,
            NAU8325_CLKPWRUP_DIS | NAU8325_PWRUP_DFT,
            NAU8325_CLKPWRUP_DIS,
        );
    }
    regmap_update_bits(regmap, NAU8325_R29_DAC_CTRL1, NAU8325_DAC_OVERSAMPLE_MASK, NAU8325_DAC_OVERSAMPLE_128);
}

unsafe fn nau8325_print_device_properties(nau8325: *mut nau8325) {
    let dev = (*nau8325).dev;

    dev_dbg(dev, c"vref-impedance-ohms:     %d".as_ptr(), (*nau8325).vref_impedance_ohms);
    dev_dbg(dev, c"dac-vref-microvolt:      %d".as_ptr(), (*nau8325).dac_vref_microvolt);
    dev_dbg(dev, c"alc-enable:              %d".as_ptr(), (*nau8325).alc_enable);
    dev_dbg(dev, c"clock-det-data:          %d".as_ptr(), (*nau8325).clock_det_data);
    dev_dbg(dev, c"clock-detection-disable: %d".as_ptr(), (*nau8325).clock_detection);
}

unsafe fn nau8325_read_device_properties(dev: *mut device, nau8325: *mut nau8325) -> i32 {
    (*nau8325).alc_enable = device_property_read_bool(dev, c"nuvoton,alc-enable".as_ptr());
    (*nau8325).clock_det_data = device_property_read_bool(dev, c"nuvoton,clock-det-data".as_ptr());
    (*nau8325).clock_detection = !device_property_read_bool(dev, c"nuvoton,clock-detection-disable".as_ptr());

    let mut ret = device_property_read_u32(
        dev,
        c"nuvoton,vref-impedance-ohms".as_ptr(),
        &mut (*nau8325).vref_impedance_ohms,
    );
    if ret != 0 {
        (*nau8325).vref_impedance_ohms = 125000;
    }
    ret = device_property_read_u32(
        dev,
        c"nuvoton,dac-vref-microvolt".as_ptr(),
        &mut (*nau8325).dac_vref_microvolt,
    );
    if ret != 0 {
        (*nau8325).dac_vref_microvolt = 2880000;
    }

    0
}

unsafe extern "C" fn nau8325_i2c_probe(i2c: *mut i2c_client) -> i32 {
    let dev = &mut (*i2c).dev as *mut device;
    let mut nau8325 = dev_get_platdata(dev) as *mut nau8325;
    let mut value: i32 = 0;
    let mut ret: i32;

    if nau8325.is_null() {
        nau8325 = devm_kzalloc(dev, core::mem::size_of::<nau8325>(), GFP_KERNEL) as *mut nau8325;
        if nau8325.is_null() {
            ret = -ENOMEM;
            return ret;
        }
        ret = nau8325_read_device_properties(dev, nau8325);
        if ret != 0 {
            return ret;
        }
    }
    i2c_set_clientdata(i2c, nau8325 as *mut c_void);

    (*nau8325).regmap = devm_regmap_init_i2c(i2c, &nau8325_regmap_config);
    if IS_ERR((*nau8325).regmap as *const c_void) {
        ret = PTR_ERR((*nau8325).regmap as *const c_void) as i32;
        return ret;
    }
    (*nau8325).dev = dev;
    nau8325_print_device_properties(nau8325);

    nau8325_reset_chip((*nau8325).regmap);
    nau8325_software_reset((*nau8325).regmap);
    ret = regmap_read((*nau8325).regmap, NAU8325_R02_DEVICE_ID, &mut value);
    if ret != 0 {
        dev_dbg(dev, c"Failed to read device id (%d)".as_ptr(), ret);
        return ret;
    }
    nau8325_init_regs(nau8325);

    ret = devm_snd_soc_register_component(dev, &nau8325_component_driver, &mut nau8325_dai, 1);
    ret
}

static nau8325_i2c_ids: [i2c_device_id; 2] = [
    i2c_device_id { name: *b"nau8325\0", driver_data: 0 },
    i2c_device_id { name: [0; I2C_NAME_SIZE], driver_data: 0 },
];
MODULE_DEVICE_TABLE!(i2c, nau8325_i2c_ids);

// CONFIG_OF: Open Firmware device match table.
static nau8325_of_ids: [of_device_id; 2] = [
    of_device_id { compatible: c"nuvoton,nau8325".as_ptr(), ..unsafe { core::mem::zeroed() } },
    of_device_id { ..unsafe { core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(of, nau8325_of_ids);

static mut nau8325_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"nau8325".as_ptr(),
        of_match_table: of_match_ptr(nau8325_of_ids.as_ptr()),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(nau8325_i2c_probe),
    id_table: nau8325_i2c_ids.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};
module_i2c_driver!(nau8325_i2c_driver);

MODULE_DESCRIPTION!(c"ASoC NAU8325 driver");
MODULE_AUTHOR!(c"Seven Lee <WTLI@nuvoton.com>");
MODULE_AUTHOR!(c"David Lin <CTLIN0@nuvoton.com>");
MODULE_LICENSE!(c"GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
