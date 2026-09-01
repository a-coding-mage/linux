// SPDX-License-Identifier: GPL-2.0-only
/*
 * PCM3168A codec driver
 *
 * Copyright (C) 2015 Imagination Technologies Ltd.
 *
 * Author: Damien Horsley <Damien.Horsley@imgtec.com>
 */

/* Dependencies from the original C includes:
 * linux/clk.h, linux/delay.h, linux/gpio/consumer.h, linux/module.h,
 * linux/pm_runtime.h, linux/regulator/consumer.h, sound/pcm_params.h,
 * sound/soc.h, sound/tlv.h, and pcm3168a.h.
 */

const PCM3168A_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_LE;

const PCM3168A_FMT_I2S: u32 = 0x0;
const PCM3168A_FMT_LEFT_J: u32 = 0x1;
const PCM3168A_FMT_RIGHT_J: u32 = 0x2;
const PCM3168A_FMT_RIGHT_J_16: u32 = 0x3;
const PCM3168A_FMT_DSP_A: u32 = 0x4;
const PCM3168A_FMT_DSP_B: u32 = 0x5;
const PCM3168A_FMT_I2S_TDM: u32 = 0x6;
const PCM3168A_FMT_LEFT_J_TDM: u32 = 0x7;

static pcm3168a_supply_names: [&'static str; 6] = [
    "VDD1",
    "VDD2",
    "VCCAD1",
    "VCCAD2",
    "VCCDA1",
    "VCCDA2",
];

const PCM3168A_DAI_DAC: usize = 0;
const PCM3168A_DAI_ADC: usize = 1;

/* ADC/DAC side parameters */
#[repr(C)]
struct pcm3168a_io_params {
    provider_mode: bool,
    format: u32,
    tdm_slots: core::ffi::c_int,
    tdm_mask: u32,
    slot_width: core::ffi::c_int,
}

#[repr(C)]
struct pcm3168a_priv {
    supplies: [regulator_bulk_data; pcm3168a_supply_names.len()],
    regmap: *mut regmap,
    scki: *mut clk,
    gpio_rst: *mut gpio_desc,
    sysclk: core::ffi::c_ulong,

    io_params: [pcm3168a_io_params; 2],
    dai_drv: [snd_soc_dai_driver; 2],
}

static pcm3168a_roll_off: [&'static str; 2] = ["Sharp", "Slow"];

SOC_ENUM_SINGLE_DECL!(pcm3168a_d1_roll_off, PCM3168A_DAC_OP_FLT,
    PCM3168A_DAC_FLT_SHIFT, pcm3168a_roll_off);
SOC_ENUM_SINGLE_DECL!(pcm3168a_d2_roll_off, PCM3168A_DAC_OP_FLT,
    PCM3168A_DAC_FLT_SHIFT + 1, pcm3168a_roll_off);
SOC_ENUM_SINGLE_DECL!(pcm3168a_d3_roll_off, PCM3168A_DAC_OP_FLT,
    PCM3168A_DAC_FLT_SHIFT + 2, pcm3168a_roll_off);
SOC_ENUM_SINGLE_DECL!(pcm3168a_d4_roll_off, PCM3168A_DAC_OP_FLT,
    PCM3168A_DAC_FLT_SHIFT + 3, pcm3168a_roll_off);

static pcm3168a_volume_type: [&'static str; 2] = [
    "Individual", "Master + Individual",
];

SOC_ENUM_SINGLE_DECL!(pcm3168a_dac_volume_type, PCM3168A_DAC_ATT_DEMP_ZF,
    PCM3168A_DAC_ATMDDA_SHIFT, pcm3168a_volume_type);

static pcm3168a_att_speed_mult: [&'static str; 2] = ["2048", "4096"];

SOC_ENUM_SINGLE_DECL!(pcm3168a_dac_att_mult, PCM3168A_DAC_ATT_DEMP_ZF,
    PCM3168A_DAC_ATSPDA_SHIFT, pcm3168a_att_speed_mult);

static pcm3168a_demp: [&'static str; 4] = [
    "Disabled", "48khz", "44.1khz", "32khz",
];

SOC_ENUM_SINGLE_DECL!(pcm3168a_dac_demp, PCM3168A_DAC_ATT_DEMP_ZF,
    PCM3168A_DAC_DEMP_SHIFT, pcm3168a_demp);

static pcm3168a_zf_func: [&'static str; 6] = [
    "DAC 1/2/3/4 AND", "DAC 1/2/3/4 OR", "DAC 1/2/3 AND",
    "DAC 1/2/3 OR", "DAC 4 AND", "DAC 4 OR",
];

SOC_ENUM_SINGLE_DECL!(pcm3168a_dac_zf_func, PCM3168A_DAC_ATT_DEMP_ZF,
    PCM3168A_DAC_AZRO_SHIFT, pcm3168a_zf_func);

static pcm3168a_pol: [&'static str; 2] = ["Active High", "Active Low"];

SOC_ENUM_SINGLE_DECL!(pcm3168a_dac_zf_pol, PCM3168A_DAC_ATT_DEMP_ZF,
    PCM3168A_DAC_ATSPDA_SHIFT, pcm3168a_pol);

static pcm3168a_con: [&'static str; 2] = ["Differential", "Single-Ended"];

SOC_ENUM_DOUBLE_DECL!(pcm3168a_adc1_con, PCM3168A_ADC_SEAD, 0, 1, pcm3168a_con);
SOC_ENUM_DOUBLE_DECL!(pcm3168a_adc2_con, PCM3168A_ADC_SEAD, 2, 3, pcm3168a_con);
SOC_ENUM_DOUBLE_DECL!(pcm3168a_adc3_con, PCM3168A_ADC_SEAD, 4, 5, pcm3168a_con);

SOC_ENUM_SINGLE_DECL!(pcm3168a_adc_volume_type, PCM3168A_ADC_ATT_OVF,
    PCM3168A_ADC_ATMDAD_SHIFT, pcm3168a_volume_type);

SOC_ENUM_SINGLE_DECL!(pcm3168a_adc_att_mult, PCM3168A_ADC_ATT_OVF,
    PCM3168A_ADC_ATSPAD_SHIFT, pcm3168a_att_speed_mult);

SOC_ENUM_SINGLE_DECL!(pcm3168a_adc_ov_pol, PCM3168A_ADC_ATT_OVF,
    PCM3168A_ADC_OVFP_SHIFT, pcm3168a_pol);

/* -100db to 0db, register values 0-54 cause mute */
static pcm3168a_dac_tlv: _ = DECLARE_TLV_DB_SCALE!(-10050, 50, 1);

/* -100db to 20db, register values 0-14 cause mute */
static pcm3168a_adc_tlv: _ = DECLARE_TLV_DB_SCALE!(-10050, 50, 1);

static pcm3168a_snd_controls: [snd_kcontrol_new; 38] = [
    SOC_SINGLE!("DAC Power-Save Switch", PCM3168A_DAC_PWR_MST_FMT, PCM3168A_DAC_PSMDA_SHIFT, 1, 1),
    SOC_ENUM!("DAC1 Digital Filter roll-off", pcm3168a_d1_roll_off),
    SOC_ENUM!("DAC2 Digital Filter roll-off", pcm3168a_d2_roll_off),
    SOC_ENUM!("DAC3 Digital Filter roll-off", pcm3168a_d3_roll_off),
    SOC_ENUM!("DAC4 Digital Filter roll-off", pcm3168a_d4_roll_off),
    SOC_DOUBLE!("DAC1 Invert Switch", PCM3168A_DAC_INV, 0, 1, 1, 0),
    SOC_DOUBLE!("DAC2 Invert Switch", PCM3168A_DAC_INV, 2, 3, 1, 0),
    SOC_DOUBLE!("DAC3 Invert Switch", PCM3168A_DAC_INV, 4, 5, 1, 0),
    SOC_DOUBLE!("DAC4 Invert Switch", PCM3168A_DAC_INV, 6, 7, 1, 0),
    SOC_ENUM!("DAC Volume Control Type", pcm3168a_dac_volume_type),
    SOC_ENUM!("DAC Volume Rate Multiplier", pcm3168a_dac_att_mult),
    SOC_ENUM!("DAC De-Emphasis", pcm3168a_dac_demp),
    SOC_ENUM!("DAC Zero Flag Function", pcm3168a_dac_zf_func),
    SOC_ENUM!("DAC Zero Flag Polarity", pcm3168a_dac_zf_pol),
    SOC_SINGLE_RANGE_TLV!("Master Playback Volume", PCM3168A_DAC_VOL_MASTER, 0, 54, 255, 0, pcm3168a_dac_tlv),
    SOC_DOUBLE_R_RANGE_TLV!("DAC1 Playback Volume", PCM3168A_DAC_VOL_CHAN_START, PCM3168A_DAC_VOL_CHAN_START + 1, 0, 54, 255, 0, pcm3168a_dac_tlv),
    SOC_DOUBLE_R_RANGE_TLV!("DAC2 Playback Volume", PCM3168A_DAC_VOL_CHAN_START + 2, PCM3168A_DAC_VOL_CHAN_START + 3, 0, 54, 255, 0, pcm3168a_dac_tlv),
    SOC_DOUBLE_R_RANGE_TLV!("DAC3 Playback Volume", PCM3168A_DAC_VOL_CHAN_START + 4, PCM3168A_DAC_VOL_CHAN_START + 5, 0, 54, 255, 0, pcm3168a_dac_tlv),
    SOC_DOUBLE_R_RANGE_TLV!("DAC4 Playback Volume", PCM3168A_DAC_VOL_CHAN_START + 6, PCM3168A_DAC_VOL_CHAN_START + 7, 0, 54, 255, 0, pcm3168a_dac_tlv),
    SOC_SINGLE!("ADC1 High-Pass Filter Switch", PCM3168A_ADC_PWR_HPFB, PCM3168A_ADC_BYP_SHIFT, 1, 1),
    SOC_SINGLE!("ADC2 High-Pass Filter Switch", PCM3168A_ADC_PWR_HPFB, PCM3168A_ADC_BYP_SHIFT + 1, 1, 1),
    SOC_SINGLE!("ADC3 High-Pass Filter Switch", PCM3168A_ADC_PWR_HPFB, PCM3168A_ADC_BYP_SHIFT + 2, 1, 1),
    SOC_ENUM!("ADC1 Connection Type", pcm3168a_adc1_con),
    SOC_ENUM!("ADC2 Connection Type", pcm3168a_adc2_con),
    SOC_ENUM!("ADC3 Connection Type", pcm3168a_adc3_con),
    SOC_DOUBLE!("ADC1 Invert Switch", PCM3168A_ADC_INV, 0, 1, 1, 0),
    SOC_DOUBLE!("ADC2 Invert Switch", PCM3168A_ADC_INV, 2, 3, 1, 0),
    SOC_DOUBLE!("ADC3 Invert Switch", PCM3168A_ADC_INV, 4, 5, 1, 0),
    SOC_DOUBLE!("ADC1 Mute Switch", PCM3168A_ADC_MUTE, 0, 1, 1, 0),
    SOC_DOUBLE!("ADC2 Mute Switch", PCM3168A_ADC_MUTE, 2, 3, 1, 0),
    SOC_DOUBLE!("ADC3 Mute Switch", PCM3168A_ADC_MUTE, 4, 5, 1, 0),
    SOC_ENUM!("ADC Volume Control Type", pcm3168a_adc_volume_type),
    SOC_ENUM!("ADC Volume Rate Multiplier", pcm3168a_adc_att_mult),
    SOC_ENUM!("ADC Overflow Flag Polarity", pcm3168a_adc_ov_pol),
    SOC_SINGLE_RANGE_TLV!("Master Capture Volume", PCM3168A_ADC_VOL_MASTER, 0, 14, 255, 0, pcm3168a_adc_tlv),
    SOC_DOUBLE_R_RANGE_TLV!("ADC1 Capture Volume", PCM3168A_ADC_VOL_CHAN_START, PCM3168A_ADC_VOL_CHAN_START + 1, 0, 14, 255, 0, pcm3168a_adc_tlv),
    SOC_DOUBLE_R_RANGE_TLV!("ADC2 Capture Volume", PCM3168A_ADC_VOL_CHAN_START + 2, PCM3168A_ADC_VOL_CHAN_START + 3, 0, 14, 255, 0, pcm3168a_adc_tlv),
    SOC_DOUBLE_R_RANGE_TLV!("ADC3 Capture Volume", PCM3168A_ADC_VOL_CHAN_START + 4, PCM3168A_ADC_VOL_CHAN_START + 5, 0, 14, 255, 0, pcm3168a_adc_tlv),
];

static pcm3168a_dapm_widgets: [snd_soc_dapm_widget; 21] = [
    SND_SOC_DAPM_DAC!("DAC1", "Playback", PCM3168A_DAC_OP_FLT, PCM3168A_DAC_OPEDA_SHIFT, 1),
    SND_SOC_DAPM_DAC!("DAC2", "Playback", PCM3168A_DAC_OP_FLT, PCM3168A_DAC_OPEDA_SHIFT + 1, 1),
    SND_SOC_DAPM_DAC!("DAC3", "Playback", PCM3168A_DAC_OP_FLT, PCM3168A_DAC_OPEDA_SHIFT + 2, 1),
    SND_SOC_DAPM_DAC!("DAC4", "Playback", PCM3168A_DAC_OP_FLT, PCM3168A_DAC_OPEDA_SHIFT + 3, 1),
    SND_SOC_DAPM_OUTPUT!("AOUT1L"),
    SND_SOC_DAPM_OUTPUT!("AOUT1R"),
    SND_SOC_DAPM_OUTPUT!("AOUT2L"),
    SND_SOC_DAPM_OUTPUT!("AOUT2R"),
    SND_SOC_DAPM_OUTPUT!("AOUT3L"),
    SND_SOC_DAPM_OUTPUT!("AOUT3R"),
    SND_SOC_DAPM_OUTPUT!("AOUT4L"),
    SND_SOC_DAPM_OUTPUT!("AOUT4R"),
    SND_SOC_DAPM_ADC!("ADC1", "Capture", PCM3168A_ADC_PWR_HPFB, PCM3168A_ADC_PSVAD_SHIFT, 1),
    SND_SOC_DAPM_ADC!("ADC2", "Capture", PCM3168A_ADC_PWR_HPFB, PCM3168A_ADC_PSVAD_SHIFT + 1, 1),
    SND_SOC_DAPM_ADC!("ADC3", "Capture", PCM3168A_ADC_PWR_HPFB, PCM3168A_ADC_PSVAD_SHIFT + 2, 1),
    SND_SOC_DAPM_INPUT!("AIN1L"),
    SND_SOC_DAPM_INPUT!("AIN1R"),
    SND_SOC_DAPM_INPUT!("AIN2L"),
    SND_SOC_DAPM_INPUT!("AIN2R"),
    SND_SOC_DAPM_INPUT!("AIN3L"),
    SND_SOC_DAPM_INPUT!("AIN3R"),
];

static pcm3168a_dapm_routes: [snd_soc_dapm_route; 14] = [
    /* Playback */
    snd_soc_dapm_route { sink: "AOUT1L", control: core::ptr::null(), source: "DAC1" },
    snd_soc_dapm_route { sink: "AOUT1R", control: core::ptr::null(), source: "DAC1" },
    snd_soc_dapm_route { sink: "AOUT2L", control: core::ptr::null(), source: "DAC2" },
    snd_soc_dapm_route { sink: "AOUT2R", control: core::ptr::null(), source: "DAC2" },
    snd_soc_dapm_route { sink: "AOUT3L", control: core::ptr::null(), source: "DAC3" },
    snd_soc_dapm_route { sink: "AOUT3R", control: core::ptr::null(), source: "DAC3" },
    snd_soc_dapm_route { sink: "AOUT4L", control: core::ptr::null(), source: "DAC4" },
    snd_soc_dapm_route { sink: "AOUT4R", control: core::ptr::null(), source: "DAC4" },
    /* Capture */
    snd_soc_dapm_route { sink: "ADC1", control: core::ptr::null(), source: "AIN1L" },
    snd_soc_dapm_route { sink: "ADC1", control: core::ptr::null(), source: "AIN1R" },
    snd_soc_dapm_route { sink: "ADC2", control: core::ptr::null(), source: "AIN2L" },
    snd_soc_dapm_route { sink: "ADC2", control: core::ptr::null(), source: "AIN2R" },
    snd_soc_dapm_route { sink: "ADC3", control: core::ptr::null(), source: "AIN3L" },
    snd_soc_dapm_route { sink: "ADC3", control: core::ptr::null(), source: "AIN3R" },
];

static mut pcm3168a_scki_ratios: [u32; 6] = [768, 512, 384, 256, 192, 128];

const PCM3168A_NUM_SCKI_RATIOS_DAC: usize = 6;
const PCM3168A_NUM_SCKI_RATIOS_ADC: usize = 4;

const PCM3168A_MAX_SYSCLK: u32 = 36864000;

unsafe fn pcm3168a_reset(pcm3168a: *mut pcm3168a_priv) -> core::ffi::c_int {
    let mut ret: core::ffi::c_int;

    ret = regmap_write((*pcm3168a).regmap, PCM3168A_RST_SMODE, 0);
    if ret != 0 {
        return ret;
    }

    /* Internal reset is de-asserted after 3846 SCKI cycles */
    msleep(DIV_ROUND_UP!(3846 * 1000, (*pcm3168a).sysclk));

    return regmap_write((*pcm3168a).regmap, PCM3168A_RST_SMODE,
        PCM3168A_MRST_MASK | PCM3168A_SRST_MASK);
}

unsafe fn pcm3168a_mute(
    dai: *mut snd_soc_dai,
    mute: core::ffi::c_int,
    direction: core::ffi::c_int,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let pcm3168a: *mut pcm3168a_priv = snd_soc_component_get_drvdata(component);

    regmap_write((*pcm3168a).regmap, PCM3168A_DAC_MUTE, if mute != 0 { 0xff } else { 0 });

    return 0;
}

unsafe fn pcm3168a_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: core::ffi::c_int,
    freq: u32,
    dir: core::ffi::c_int,
) -> core::ffi::c_int {
    let pcm3168a: *mut pcm3168a_priv = snd_soc_component_get_drvdata((*dai).component);
    let mut ret: core::ffi::c_int;

    /*
     * Some sound card sets 0 Hz as reset,
     * but it is impossible to set. Ignore it here
     */
    if freq == 0 {
        return 0;
    }

    if freq > PCM3168A_MAX_SYSCLK {
        return -EINVAL;
    }

    ret = clk_set_rate((*pcm3168a).scki, freq);
    if ret != 0 {
        return ret;
    }

    (*pcm3168a).sysclk = freq as core::ffi::c_ulong;

    return 0;
}

unsafe fn pcm3168a_update_fixup_pcm_stream(dai: *mut snd_soc_dai) {
    let component: *mut snd_soc_component = (*dai).component;
    let pcm3168a: *mut pcm3168a_priv = snd_soc_component_get_drvdata(component);
    let io_params: *mut pcm3168a_io_params = &mut (*pcm3168a).io_params[(*dai).id as usize];
    let mut formats: u64 = SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_LE;
    let mut channel_max: u32 = if (*dai).id as usize == PCM3168A_DAI_DAC { 8 } else { 6 };

    if (*io_params).format == SND_SOC_DAIFMT_RIGHT_J {
        /* S16_LE is only supported in RIGHT_J mode */
        formats |= SNDRV_PCM_FMTBIT_S16_LE;

        /*
         * If multi DIN/DOUT is not selected, RIGHT_J can only support
         * two channels (no TDM support)
         */
        if (*io_params).tdm_slots != 2 {
            channel_max = 2;
        }
    }

    if (*dai).id as usize == PCM3168A_DAI_DAC {
        (*(*dai).driver).playback.channels_max = channel_max;
        (*(*dai).driver).playback.formats = formats;
    } else {
        (*(*dai).driver).capture.channels_max = channel_max;
        (*(*dai).driver).capture.formats = formats;
    }
}

unsafe fn pcm3168a_set_dai_fmt(dai: *mut snd_soc_dai, format: u32) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let pcm3168a: *mut pcm3168a_priv = snd_soc_component_get_drvdata(component);
    let io_params: *mut pcm3168a_io_params = &mut (*pcm3168a).io_params[(*dai).id as usize];
    let provider_mode: bool;

    match format & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_RIGHT_J |
        SND_SOC_DAIFMT_DSP_A | SND_SOC_DAIFMT_DSP_B => {}
        _ => {
            dev_err!((*component).dev, "unsupported dai format\n");
            return -EINVAL;
        }
    }

    match format & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {
            provider_mode = false;
        }
        SND_SOC_DAIFMT_CBP_CFP => {
            provider_mode = true;
        }
        _ => {
            dev_err!((*component).dev, "unsupported provider mode\n");
            return -EINVAL;
        }
    }

    match format & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        _ => {
            return -EINVAL;
        }
    }

    (*io_params).provider_mode = provider_mode;
    (*io_params).format = format & SND_SOC_DAIFMT_FORMAT_MASK;

    pcm3168a_update_fixup_pcm_stream(dai);

    return 0;
}

unsafe fn pcm3168a_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: u32,
    rx_mask: u32,
    slots: core::ffi::c_int,
    slot_width: core::ffi::c_int,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let pcm3168a: *mut pcm3168a_priv = snd_soc_component_get_drvdata(component);
    let io_params: *mut pcm3168a_io_params = &mut (*pcm3168a).io_params[(*dai).id as usize];

    if tx_mask >= (1u32 << slots) || rx_mask >= (1u32 << slots) {
        dev_err!((*component).dev,
            "Bad tdm mask tx: 0x%08x rx: 0x%08x slots %d\n",
            tx_mask, rx_mask, slots);
        return -EINVAL;
    }

    if slot_width != 0 && slot_width != 16 && slot_width != 24 && slot_width != 32 {
        dev_err!((*component).dev, "Unsupported slot_width %d\n", slot_width);
        return -EINVAL;
    }

    (*io_params).tdm_slots = slots;
    (*io_params).slot_width = slot_width;
    /* Ignore the not relevant mask for the DAI/direction */
    if (*dai).id as usize == PCM3168A_DAI_DAC {
        (*io_params).tdm_mask = tx_mask;
    } else {
        (*io_params).tdm_mask = rx_mask;
    }

    pcm3168a_update_fixup_pcm_stream(dai);

    return 0;
}

unsafe fn pcm3168a_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let pcm3168a: *mut pcm3168a_priv = snd_soc_component_get_drvdata(component);
    let io_params: *mut pcm3168a_io_params = &mut (*pcm3168a).io_params[(*dai).id as usize];
    let provider_mode: bool;
    let tdm_mode: bool;
    let mut format: u32;
    let mut reg: u32;
    let mut mask: u32;
    let mut ms: u32;
    let mut ms_shift: u32;
    let mut fmt: u32;
    let mut fmt_shift: u32;
    let mut ratio: u32;
    let mut tdm_slots: u32;
    let mut i: core::ffi::c_int;
    let mut num_scki_ratios: core::ffi::c_int;
    let mut slot_width: core::ffi::c_int;

    if (*dai).id as usize == PCM3168A_DAI_DAC {
        num_scki_ratios = PCM3168A_NUM_SCKI_RATIOS_DAC as core::ffi::c_int;
        reg = PCM3168A_DAC_PWR_MST_FMT;
        mask = PCM3168A_DAC_MSDA_MASK | PCM3168A_DAC_FMT_MASK;
        ms_shift = PCM3168A_DAC_MSDA_SHIFT;
        fmt_shift = PCM3168A_DAC_FMT_SHIFT;
    } else {
        num_scki_ratios = PCM3168A_NUM_SCKI_RATIOS_ADC as core::ffi::c_int;
        reg = PCM3168A_ADC_MST_FMT;
        mask = PCM3168A_ADC_MSAD_MASK | PCM3168A_ADC_FMTAD_MASK;
        ms_shift = PCM3168A_ADC_MSAD_SHIFT;
        fmt_shift = PCM3168A_ADC_FMTAD_SHIFT;
    }

    provider_mode = (*io_params).provider_mode;

    if provider_mode {
        ratio = ((*pcm3168a).sysclk / params_rate(params) as core::ffi::c_ulong) as u32;

        i = 0;
        while i < num_scki_ratios {
            if pcm3168a_scki_ratios[i as usize] == ratio {
                break;
            }
            i += 1;
        }

        if i == num_scki_ratios {
            dev_err!((*component).dev, "unsupported sysclk ratio\n");
            return -EINVAL;
        }

        ms = (i + 1) as u32;
    } else {
        ms = 0;
    }

    format = (*io_params).format;

    if (*io_params).slot_width != 0 {
        slot_width = (*io_params).slot_width;
    } else {
        slot_width = params_width(params);
    }

    match slot_width {
        16 => {
            if provider_mode || format != SND_SOC_DAIFMT_RIGHT_J {
                dev_err!((*component).dev, "16-bit slots are supported only for consumer mode using right justified\n");
                return -EINVAL;
            }
        }
        24 => {
            if !provider_mode &&
                (format == SND_SOC_DAIFMT_DSP_A || format == SND_SOC_DAIFMT_DSP_B) {
                dev_err!((*component).dev, "24-bit slots not supported in consumer mode using DSP\n");
                return -EINVAL;
            }
        }
        32 => {}
        _ => {
            dev_err!((*component).dev, "unsupported frame size: %d\n", slot_width);
            return -EINVAL;
        }
    }

    if (*io_params).tdm_slots != 0 {
        tdm_slots = (*io_params).tdm_slots as u32;
    } else {
        tdm_slots = params_channels(params);
    }

    /*
     * Switch the codec to TDM mode when more than 2 TDM slots are needed
     * for the stream.
     * If pcm3168a->tdm_slots is not set or set to more than 2 (8/6 usually)
     * then DIN1/DOUT1 is used in TDM mode.
     * If pcm3168a->tdm_slots is set to 2 then DIN1/2/3/4 and DOUT1/2/3 is
     * used in normal mode, no need to switch to TDM modes.
     */
    tdm_mode = tdm_slots > 2;

    if tdm_mode {
        match format {
            SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_DSP_A |
            SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_DSP_B => {}
            _ => {
                dev_err!((*component).dev,
                    "TDM is supported under DSP/I2S/Left_J only\n");
                return -EINVAL;
            }
        }
    }

    match format {
        SND_SOC_DAIFMT_I2S => {
            fmt = if tdm_mode { PCM3168A_FMT_I2S_TDM } else { PCM3168A_FMT_I2S };
        }
        SND_SOC_DAIFMT_LEFT_J => {
            fmt = if tdm_mode { PCM3168A_FMT_LEFT_J_TDM } else { PCM3168A_FMT_LEFT_J };
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            fmt = if slot_width == 16 { PCM3168A_FMT_RIGHT_J_16 } else { PCM3168A_FMT_RIGHT_J };
        }
        SND_SOC_DAIFMT_DSP_A => {
            fmt = if tdm_mode { PCM3168A_FMT_I2S_TDM } else { PCM3168A_FMT_DSP_A };
        }
        SND_SOC_DAIFMT_DSP_B => {
            fmt = if tdm_mode { PCM3168A_FMT_LEFT_J_TDM } else { PCM3168A_FMT_DSP_B };
        }
        _ => {
            return -EINVAL;
        }
    }

    regmap_update_bits((*pcm3168a).regmap, reg, mask, (ms << ms_shift) | (fmt << fmt_shift));

    return 0;
}

static pcm3168a_dai_formats: [u64; 2] = [
    /*
     * First Priority
     */
    SND_SOC_POSSIBLE_DAIFMT_I2S | SND_SOC_POSSIBLE_DAIFMT_LEFT_J,
    /*
     * Second Priority
     *
     * These have picky limitation.
     * see
     *	pcm3168a_hw_params()
     */
    SND_SOC_POSSIBLE_DAIFMT_I2S |
    SND_SOC_POSSIBLE_DAIFMT_LEFT_J |
    SND_SOC_POSSIBLE_DAIFMT_RIGHT_J |
    SND_SOC_POSSIBLE_DAIFMT_DSP_A |
    SND_SOC_POSSIBLE_DAIFMT_DSP_B,
];

static pcm3168a_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_fmt: Some(pcm3168a_set_dai_fmt),
    set_sysclk: Some(pcm3168a_set_dai_sysclk),
    hw_params: Some(pcm3168a_hw_params),
    mute_stream: Some(pcm3168a_mute),
    set_tdm_slot: Some(pcm3168a_set_tdm_slot),
    no_capture_mute: 1,
    auto_selectable_formats: pcm3168a_dai_formats.as_ptr(),
    num_auto_selectable_formats: pcm3168a_dai_formats.len(),
};

static mut pcm3168a_dais: [snd_soc_dai_driver; 2] = [
    snd_soc_dai_driver {
        name: "pcm3168a-dac",
        id: PCM3168A_DAI_DAC as core::ffi::c_int,
        playback: snd_soc_pcm_stream {
            stream_name: "Playback",
            channels_min: 1,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_8000_192000,
            formats: PCM3168A_FORMATS,
        },
        ops: &pcm3168a_dai_ops,
    },
    snd_soc_dai_driver {
        name: "pcm3168a-adc",
        id: PCM3168A_DAI_ADC as core::ffi::c_int,
        capture: snd_soc_pcm_stream {
            stream_name: "Capture",
            channels_min: 1,
            channels_max: 6,
            rates: SNDRV_PCM_RATE_8000_96000,
            formats: PCM3168A_FORMATS,
        },
        ops: &pcm3168a_dai_ops,
    },
];

static pcm3168a_reg_default: [reg_default; 31] = [
    reg_default { reg: PCM3168A_RST_SMODE, def: PCM3168A_MRST_MASK | PCM3168A_SRST_MASK },
    reg_default { reg: PCM3168A_DAC_PWR_MST_FMT, def: 0x00 },
    reg_default { reg: PCM3168A_DAC_OP_FLT, def: 0x00 },
    reg_default { reg: PCM3168A_DAC_INV, def: 0x00 },
    reg_default { reg: PCM3168A_DAC_MUTE, def: 0x00 },
    reg_default { reg: PCM3168A_DAC_ZERO, def: 0x00 },
    reg_default { reg: PCM3168A_DAC_ATT_DEMP_ZF, def: 0x00 },
    reg_default { reg: PCM3168A_DAC_VOL_MASTER, def: 0xff },
    reg_default { reg: PCM3168A_DAC_VOL_CHAN_START, def: 0xff },
    reg_default { reg: PCM3168A_DAC_VOL_CHAN_START + 1, def: 0xff },
    reg_default { reg: PCM3168A_DAC_VOL_CHAN_START + 2, def: 0xff },
    reg_default { reg: PCM3168A_DAC_VOL_CHAN_START + 3, def: 0xff },
    reg_default { reg: PCM3168A_DAC_VOL_CHAN_START + 4, def: 0xff },
    reg_default { reg: PCM3168A_DAC_VOL_CHAN_START + 5, def: 0xff },
    reg_default { reg: PCM3168A_DAC_VOL_CHAN_START + 6, def: 0xff },
    reg_default { reg: PCM3168A_DAC_VOL_CHAN_START + 7, def: 0xff },
    reg_default { reg: PCM3168A_ADC_SMODE, def: 0x00 },
    reg_default { reg: PCM3168A_ADC_MST_FMT, def: 0x00 },
    reg_default { reg: PCM3168A_ADC_PWR_HPFB, def: 0x00 },
    reg_default { reg: PCM3168A_ADC_SEAD, def: 0x00 },
    reg_default { reg: PCM3168A_ADC_INV, def: 0x00 },
    reg_default { reg: PCM3168A_ADC_MUTE, def: 0x00 },
    reg_default { reg: PCM3168A_ADC_OV, def: 0x00 },
    reg_default { reg: PCM3168A_ADC_ATT_OVF, def: 0x00 },
    reg_default { reg: PCM3168A_ADC_VOL_MASTER, def: 0xd3 },
    reg_default { reg: PCM3168A_ADC_VOL_CHAN_START, def: 0xd3 },
    reg_default { reg: PCM3168A_ADC_VOL_CHAN_START + 1, def: 0xd3 },
    reg_default { reg: PCM3168A_ADC_VOL_CHAN_START + 2, def: 0xd3 },
    reg_default { reg: PCM3168A_ADC_VOL_CHAN_START + 3, def: 0xd3 },
    reg_default { reg: PCM3168A_ADC_VOL_CHAN_START + 4, def: 0xd3 },
    reg_default { reg: PCM3168A_ADC_VOL_CHAN_START + 5, def: 0xd3 },
];

unsafe fn pcm3168a_readable_register(dev: *mut device, reg: u32) -> bool {
    if reg >= PCM3168A_RST_SMODE {
        return true;
    } else {
        return false;
    }
}

unsafe fn pcm3168a_volatile_register(dev: *mut device, reg: u32) -> bool {
    match reg {
        PCM3168A_RST_SMODE | PCM3168A_DAC_ZERO | PCM3168A_ADC_OV => {
            return true;
        }
        _ => {
            return false;
        }
    }
}

unsafe fn pcm3168a_writeable_register(dev: *mut device, reg: u32) -> bool {
    if reg < PCM3168A_RST_SMODE {
        return false;
    }

    match reg {
        PCM3168A_DAC_ZERO | PCM3168A_ADC_OV => {
            return false;
        }
        _ => {
            return true;
        }
    }
}

pub static pcm3168a_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,

    max_register: PCM3168A_ADC_VOL_CHAN_START + 5,
    reg_defaults: pcm3168a_reg_default.as_ptr(),
    num_reg_defaults: pcm3168a_reg_default.len(),
    readable_reg: Some(pcm3168a_readable_register),
    volatile_reg: Some(pcm3168a_volatile_register),
    writeable_reg: Some(pcm3168a_writeable_register),
    cache_type: REGCACHE_FLAT,
};
EXPORT_SYMBOL_GPL!(pcm3168a_regmap);

static pcm3168a_driver: snd_soc_component_driver = snd_soc_component_driver {
    controls: pcm3168a_snd_controls.as_ptr(),
    num_controls: pcm3168a_snd_controls.len(),
    dapm_widgets: pcm3168a_dapm_widgets.as_ptr(),
    num_dapm_widgets: pcm3168a_dapm_widgets.len(),
    dapm_routes: pcm3168a_dapm_routes.as_ptr(),
    num_dapm_routes: pcm3168a_dapm_routes.len(),
    use_pmdown_time: 1,
    endianness: 1,
};

pub unsafe fn pcm3168a_probe(dev: *mut device, regmap: *mut regmap) -> core::ffi::c_int {
    let mut pcm3168a: *mut pcm3168a_priv;
    let mut ret: core::ffi::c_int;
    let mut i: core::ffi::c_int;

    pcm3168a = devm_kzalloc(dev, core::mem::size_of::<pcm3168a_priv>(), GFP_KERNEL);
    if pcm3168a == core::ptr::null_mut() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, pcm3168a);

    /*
     * Request the reset (connected to RST pin) gpio line as non exclusive
     * as the same reset line might be connected to multiple pcm3168a codec
     *
     * The RST is low active, we want the GPIO line to be high initially, so
     * request the initial level to LOW which in practice means DEASSERTED:
     * The deasserted level of GPIO_ACTIVE_LOW is HIGH.
     */
    (*pcm3168a).gpio_rst = devm_gpiod_get_optional(dev, "reset",
        GPIOD_OUT_LOW | GPIOD_FLAGS_BIT_NONEXCLUSIVE);
    if IS_ERR((*pcm3168a).gpio_rst) {
        return dev_err_probe(dev, PTR_ERR((*pcm3168a).gpio_rst),
            "failed to acquire RST gpio\n");
    }

    (*pcm3168a).scki = devm_clk_get_optional(dev, "scki");
    if IS_ERR((*pcm3168a).scki) {
        return dev_err_probe(dev, PTR_ERR((*pcm3168a).scki),
            "failed to acquire clock 'scki'\n");
    }

    ret = clk_prepare_enable((*pcm3168a).scki);
    if ret != 0 {
        dev_err!(dev, "Failed to enable mclk: %d\n", ret);
        return ret;
    }

    (*pcm3168a).sysclk = clk_get_rate((*pcm3168a).scki);
    /* Fallback to the default if no clk entry available. */
    if (*pcm3168a).sysclk == 0 {
        (*pcm3168a).sysclk = 24576000;
    }

    i = 0;
    while i < (*pcm3168a).supplies.len() as core::ffi::c_int {
        (*pcm3168a).supplies[i as usize].supply = pcm3168a_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, (*pcm3168a).supplies.len(), (*pcm3168a).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err_probe(dev, ret, "failed to request supplies\n");
        clk_disable_unprepare((*pcm3168a).scki);
        return ret;
    }

    ret = regulator_bulk_enable((*pcm3168a).supplies.len(), (*pcm3168a).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err!(dev, "failed to enable supplies: %d\n", ret);
        clk_disable_unprepare((*pcm3168a).scki);
        return ret;
    }

    (*pcm3168a).regmap = regmap;
    if IS_ERR((*pcm3168a).regmap) {
        ret = PTR_ERR((*pcm3168a).regmap);
        dev_err!(dev, "failed to allocate regmap: %d\n", ret);
        regulator_bulk_disable((*pcm3168a).supplies.len(), (*pcm3168a).supplies.as_mut_ptr());
        clk_disable_unprepare((*pcm3168a).scki);
        return ret;
    }

    if (*pcm3168a).gpio_rst != core::ptr::null_mut() {
        /*
         * The device is taken out from reset via GPIO line, wait for
         * 3846 SCKI clock cycles for the internal reset de-assertion
         */
        msleep(DIV_ROUND_UP!(3846 * 1000, (*pcm3168a).sysclk));
    } else {
        ret = pcm3168a_reset(pcm3168a);
        if ret != 0 {
            dev_err!(dev, "Failed to reset device: %d\n", ret);
            regulator_bulk_disable((*pcm3168a).supplies.len(), (*pcm3168a).supplies.as_mut_ptr());
            clk_disable_unprepare((*pcm3168a).scki);
            return ret;
        }
    }

    pm_runtime_set_active(dev);
    pm_runtime_enable(dev);

    memcpy((*pcm3168a).dai_drv.as_mut_ptr(), pcm3168a_dais.as_ptr(), core::mem::size_of_val(&(*pcm3168a).dai_drv));
    ret = devm_snd_soc_register_component(dev, &pcm3168a_driver,
        (*pcm3168a).dai_drv.as_mut_ptr(), (*pcm3168a).dai_drv.len());
    if ret != 0 {
        dev_err!(dev, "failed to register component: %d\n", ret);
        regulator_bulk_disable((*pcm3168a).supplies.len(), (*pcm3168a).supplies.as_mut_ptr());
        clk_disable_unprepare((*pcm3168a).scki);
        return ret;
    }

    return 0;
}
EXPORT_SYMBOL_GPL!(pcm3168a_probe);

pub unsafe fn pcm3168a_remove(dev: *mut device) {
    let pcm3168a: *mut pcm3168a_priv = dev_get_drvdata(dev);

    /*
     * The RST is low active, we want the GPIO line to be low when the
     * driver is removed, so set level to 1 which in practice means
     * ASSERTED:
     * The asserted level of GPIO_ACTIVE_LOW is LOW.
     */
    gpiod_set_value_cansleep((*pcm3168a).gpio_rst, 1);

    pm_runtime_disable(dev);
    if !pm_runtime_status_suspended(dev) {
        regulator_bulk_disable((*pcm3168a).supplies.len(), (*pcm3168a).supplies.as_mut_ptr());
        clk_disable_unprepare((*pcm3168a).scki);
    }
}
EXPORT_SYMBOL_GPL!(pcm3168a_remove);

unsafe fn pcm3168a_rt_resume(dev: *mut device) -> core::ffi::c_int {
    let pcm3168a: *mut pcm3168a_priv = dev_get_drvdata(dev);
    let mut ret: core::ffi::c_int;

    ret = clk_prepare_enable((*pcm3168a).scki);
    if ret != 0 {
        dev_err!(dev, "Failed to enable mclk: %d\n", ret);
        return ret;
    }

    ret = regulator_bulk_enable((*pcm3168a).supplies.len(), (*pcm3168a).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err!(dev, "Failed to enable supplies: %d\n", ret);
        clk_disable_unprepare((*pcm3168a).scki);
        return ret;
    }

    ret = pcm3168a_reset(pcm3168a);
    if ret != 0 {
        dev_err!(dev, "Failed to reset device: %d\n", ret);
        regulator_bulk_disable((*pcm3168a).supplies.len(), (*pcm3168a).supplies.as_mut_ptr());
        clk_disable_unprepare((*pcm3168a).scki);
        return ret;
    }

    regcache_cache_only((*pcm3168a).regmap, false);

    regcache_mark_dirty((*pcm3168a).regmap);

    ret = regcache_sync((*pcm3168a).regmap);
    if ret != 0 {
        dev_err!(dev, "Failed to sync regmap: %d\n", ret);
        regulator_bulk_disable((*pcm3168a).supplies.len(), (*pcm3168a).supplies.as_mut_ptr());
        clk_disable_unprepare((*pcm3168a).scki);
        return ret;
    }

    return 0;
}

unsafe fn pcm3168a_rt_suspend(dev: *mut device) -> core::ffi::c_int {
    let pcm3168a: *mut pcm3168a_priv = dev_get_drvdata(dev);

    regcache_cache_only((*pcm3168a).regmap, true);

    regulator_bulk_disable((*pcm3168a).supplies.len(), (*pcm3168a).supplies.as_mut_ptr());
    clk_disable_unprepare((*pcm3168a).scki);

    return 0;
}

EXPORT_GPL_DEV_PM_OPS!(pcm3168a_pm_ops, {
    RUNTIME_PM_OPS!(pcm3168a_rt_suspend, pcm3168a_rt_resume, core::ptr::null_mut());
    SYSTEM_SLEEP_PM_OPS!(pm_runtime_force_suspend, pm_runtime_force_resume);
});

MODULE_DESCRIPTION!("PCM3168A codec driver");
MODULE_AUTHOR!("Damien Horsley <Damien.Horsley@imgtec.com>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
