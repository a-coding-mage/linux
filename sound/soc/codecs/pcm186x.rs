// SPDX-License-Identifier: GPL-2.0
/*
 * Texas Instruments PCM186x Universal Audio ADC
 *
 * Copyright (C) 2015-2017 Texas Instruments Incorporated - https://www.ti.com
 *	Andreas Dannenberg <dannenberg@ti.com>
 *	Andrew F. Davis <afd@ti.com>
 */

// Dependencies in the original C file:
// linux/module.h, linux/moduleparam.h, linux/init.h, linux/delay.h,
// linux/pm.h, linux/regulator/consumer.h, linux/regmap.h, linux/slab.h,
// sound/core.h, sound/pcm.h, sound/pcm_params.h, sound/soc.h,
// sound/jack.h, sound/initval.h, sound/tlv.h, and "pcm186x.h".

pub static pcm186x_supply_names: [&'static str; 3] = [
    "avdd",  /* Analog power supply. Connect to 3.3-V supply. */
    "dvdd",  /* Digital power supply. Connect to 3.3-V supply. */
    "iovdd", /* I/O power supply. Connect to 3.3-V or 1.8-V. */
];

pub const PCM186x_NUM_SUPPLIES: usize = pcm186x_supply_names.len();

#[repr(C)]
pub struct pcm186x_priv {
    pub regmap: *mut regmap,
    pub supplies: [regulator_bulk_data; PCM186x_NUM_SUPPLIES],
    pub sysclk: c_uint,
    pub tdm_offset: c_uint,
    pub is_tdm_mode: bool,
    pub is_provider_mode: bool,
}

static_decl_tlv_db_scale!(pcm186x_pga_tlv, -1200, 50, 0);

pub static pcm1863_snd_controls: [snd_kcontrol_new; 1] = [
    SOC_DOUBLE_R_S_TLV!(
        "ADC Capture Volume",
        PCM186X_PGA_VAL_CH1_L,
        PCM186X_PGA_VAL_CH1_R,
        0,
        -24,
        80,
        7,
        0,
        pcm186x_pga_tlv
    ),
];

pub static pcm1865_snd_controls: [snd_kcontrol_new; 2] = [
    SOC_DOUBLE_R_S_TLV!(
        "ADC1 Capture Volume",
        PCM186X_PGA_VAL_CH1_L,
        PCM186X_PGA_VAL_CH1_R,
        0,
        -24,
        80,
        7,
        0,
        pcm186x_pga_tlv
    ),
    SOC_DOUBLE_R_S_TLV!(
        "ADC2 Capture Volume",
        PCM186X_PGA_VAL_CH2_L,
        PCM186X_PGA_VAL_CH2_R,
        0,
        -24,
        80,
        7,
        0,
        pcm186x_pga_tlv
    ),
];

pub static pcm186x_adc_input_channel_sel_value: [c_uint; 19] = [
    0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07,
    0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f,
    0x10, 0x20, 0x30,
];

pub static pcm186x_adcl_input_channel_sel_text: [&'static str; 19] = [
    "No Select",
    "VINL1[SE]", /* Default for ADC1L */
    "VINL2[SE]", /* Default for ADC2L */
    "VINL2[SE] + VINL1[SE]",
    "VINL3[SE]",
    "VINL3[SE] + VINL1[SE]",
    "VINL3[SE] + VINL2[SE]",
    "VINL3[SE] + VINL2[SE] + VINL1[SE]",
    "VINL4[SE]",
    "VINL4[SE] + VINL1[SE]",
    "VINL4[SE] + VINL2[SE]",
    "VINL4[SE] + VINL2[SE] + VINL1[SE]",
    "VINL4[SE] + VINL3[SE]",
    "VINL4[SE] + VINL3[SE] + VINL1[SE]",
    "VINL4[SE] + VINL3[SE] + VINL2[SE]",
    "VINL4[SE] + VINL3[SE] + VINL2[SE] + VINL1[SE]",
    "{VIN1P, VIN1M}[DIFF]",
    "{VIN4P, VIN4M}[DIFF]",
    "{VIN1P, VIN1M}[DIFF] + {VIN4P, VIN4M}[DIFF]",
];

pub static pcm186x_adcr_input_channel_sel_text: [&'static str; 19] = [
    "No Select",
    "VINR1[SE]", /* Default for ADC1R */
    "VINR2[SE]", /* Default for ADC2R */
    "VINR2[SE] + VINR1[SE]",
    "VINR3[SE]",
    "VINR3[SE] + VINR1[SE]",
    "VINR3[SE] + VINR2[SE]",
    "VINR3[SE] + VINR2[SE] + VINR1[SE]",
    "VINR4[SE]",
    "VINR4[SE] + VINR1[SE]",
    "VINR4[SE] + VINR2[SE]",
    "VINR4[SE] + VINR2[SE] + VINR1[SE]",
    "VINR4[SE] + VINR3[SE]",
    "VINR4[SE] + VINR3[SE] + VINR1[SE]",
    "VINR4[SE] + VINR3[SE] + VINR2[SE]",
    "VINR4[SE] + VINR3[SE] + VINR2[SE] + VINR1[SE]",
    "{VIN2P, VIN2M}[DIFF]",
    "{VIN3P, VIN3M}[DIFF]",
    "{VIN2P, VIN2M}[DIFF] + {VIN3P, VIN3M}[DIFF]",
];

pub static pcm186x_adc_input_channel_sel: [soc_enum; 4] = [
    SOC_VALUE_ENUM_SINGLE!(
        PCM186X_ADC1_INPUT_SEL_L, 0, PCM186X_ADC_INPUT_SEL_MASK,
        pcm186x_adcl_input_channel_sel_text.len(),
        pcm186x_adcl_input_channel_sel_text,
        pcm186x_adc_input_channel_sel_value
    ),
    SOC_VALUE_ENUM_SINGLE!(
        PCM186X_ADC1_INPUT_SEL_R, 0, PCM186X_ADC_INPUT_SEL_MASK,
        pcm186x_adcr_input_channel_sel_text.len(),
        pcm186x_adcr_input_channel_sel_text,
        pcm186x_adc_input_channel_sel_value
    ),
    SOC_VALUE_ENUM_SINGLE!(
        PCM186X_ADC2_INPUT_SEL_L, 0, PCM186X_ADC_INPUT_SEL_MASK,
        pcm186x_adcl_input_channel_sel_text.len(),
        pcm186x_adcl_input_channel_sel_text,
        pcm186x_adc_input_channel_sel_value
    ),
    SOC_VALUE_ENUM_SINGLE!(
        PCM186X_ADC2_INPUT_SEL_R, 0, PCM186X_ADC_INPUT_SEL_MASK,
        pcm186x_adcr_input_channel_sel_text.len(),
        pcm186x_adcr_input_channel_sel_text,
        pcm186x_adc_input_channel_sel_value
    ),
];

pub static pcm186x_adc_mux_controls: [snd_kcontrol_new; 4] = [
    SOC_DAPM_ENUM!("ADC1 Left Input", pcm186x_adc_input_channel_sel[0]),
    SOC_DAPM_ENUM!("ADC1 Right Input", pcm186x_adc_input_channel_sel[1]),
    SOC_DAPM_ENUM!("ADC2 Left Input", pcm186x_adc_input_channel_sel[2]),
    SOC_DAPM_ENUM!("ADC2 Right Input", pcm186x_adc_input_channel_sel[3]),
];

pub static pcm1863_dapm_widgets: [snd_soc_dapm_widget; 11] = [
    SND_SOC_DAPM_INPUT!("VINL1"),
    SND_SOC_DAPM_INPUT!("VINR1"),
    SND_SOC_DAPM_INPUT!("VINL2"),
    SND_SOC_DAPM_INPUT!("VINR2"),
    SND_SOC_DAPM_INPUT!("VINL3"),
    SND_SOC_DAPM_INPUT!("VINR3"),
    SND_SOC_DAPM_INPUT!("VINL4"),
    SND_SOC_DAPM_INPUT!("VINR4"),
    SND_SOC_DAPM_MUX!("ADC Left Capture Source", SND_SOC_NOPM, 0, 0, &pcm186x_adc_mux_controls[0]),
    SND_SOC_DAPM_MUX!("ADC Right Capture Source", SND_SOC_NOPM, 0, 0, &pcm186x_adc_mux_controls[1]),
    /*
     * Put the codec into SLEEP mode when not in use, allowing the
     * Energysense mechanism to operate.
     */
    SND_SOC_DAPM_ADC!("ADC", "HiFi Capture", PCM186X_POWER_CTRL, 1, 1),
];

pub static pcm1865_dapm_widgets: [snd_soc_dapm_widget; 14] = [
    SND_SOC_DAPM_INPUT!("VINL1"),
    SND_SOC_DAPM_INPUT!("VINR1"),
    SND_SOC_DAPM_INPUT!("VINL2"),
    SND_SOC_DAPM_INPUT!("VINR2"),
    SND_SOC_DAPM_INPUT!("VINL3"),
    SND_SOC_DAPM_INPUT!("VINR3"),
    SND_SOC_DAPM_INPUT!("VINL4"),
    SND_SOC_DAPM_INPUT!("VINR4"),
    SND_SOC_DAPM_MUX!("ADC1 Left Capture Source", SND_SOC_NOPM, 0, 0, &pcm186x_adc_mux_controls[0]),
    SND_SOC_DAPM_MUX!("ADC1 Right Capture Source", SND_SOC_NOPM, 0, 0, &pcm186x_adc_mux_controls[1]),
    SND_SOC_DAPM_MUX!("ADC2 Left Capture Source", SND_SOC_NOPM, 0, 0, &pcm186x_adc_mux_controls[2]),
    SND_SOC_DAPM_MUX!("ADC2 Right Capture Source", SND_SOC_NOPM, 0, 0, &pcm186x_adc_mux_controls[3]),
    /*
     * Put the codec into SLEEP mode when not in use, allowing the
     * Energysense mechanism to operate.
     */
    SND_SOC_DAPM_ADC!("ADC1", "HiFi Capture 1", PCM186X_POWER_CTRL, 1, 1),
    SND_SOC_DAPM_ADC!("ADC2", "HiFi Capture 2", PCM186X_POWER_CTRL, 1, 1),
];

pub static pcm1863_dapm_routes: [snd_soc_dapm_route; 18] = [
    snd_soc_dapm_route { sink: "ADC Left Capture Source", control: None, source: "VINL1" },
    snd_soc_dapm_route { sink: "ADC Left Capture Source", control: None, source: "VINR1" },
    snd_soc_dapm_route { sink: "ADC Left Capture Source", control: None, source: "VINL2" },
    snd_soc_dapm_route { sink: "ADC Left Capture Source", control: None, source: "VINR2" },
    snd_soc_dapm_route { sink: "ADC Left Capture Source", control: None, source: "VINL3" },
    snd_soc_dapm_route { sink: "ADC Left Capture Source", control: None, source: "VINR3" },
    snd_soc_dapm_route { sink: "ADC Left Capture Source", control: None, source: "VINL4" },
    snd_soc_dapm_route { sink: "ADC Left Capture Source", control: None, source: "VINR4" },
    snd_soc_dapm_route { sink: "ADC", control: None, source: "ADC Left Capture Source" },
    snd_soc_dapm_route { sink: "ADC Right Capture Source", control: None, source: "VINL1" },
    snd_soc_dapm_route { sink: "ADC Right Capture Source", control: None, source: "VINR1" },
    snd_soc_dapm_route { sink: "ADC Right Capture Source", control: None, source: "VINL2" },
    snd_soc_dapm_route { sink: "ADC Right Capture Source", control: None, source: "VINR2" },
    snd_soc_dapm_route { sink: "ADC Right Capture Source", control: None, source: "VINL3" },
    snd_soc_dapm_route { sink: "ADC Right Capture Source", control: None, source: "VINR3" },
    snd_soc_dapm_route { sink: "ADC Right Capture Source", control: None, source: "VINL4" },
    snd_soc_dapm_route { sink: "ADC Right Capture Source", control: None, source: "VINR4" },
    snd_soc_dapm_route { sink: "ADC", control: None, source: "ADC Right Capture Source" },
];

pub static pcm1865_dapm_routes: [snd_soc_dapm_route; 36] = [
    snd_soc_dapm_route { sink: "ADC1 Left Capture Source", control: None, source: "VINL1" },
    snd_soc_dapm_route { sink: "ADC1 Left Capture Source", control: None, source: "VINR1" },
    snd_soc_dapm_route { sink: "ADC1 Left Capture Source", control: None, source: "VINL2" },
    snd_soc_dapm_route { sink: "ADC1 Left Capture Source", control: None, source: "VINR2" },
    snd_soc_dapm_route { sink: "ADC1 Left Capture Source", control: None, source: "VINL3" },
    snd_soc_dapm_route { sink: "ADC1 Left Capture Source", control: None, source: "VINR3" },
    snd_soc_dapm_route { sink: "ADC1 Left Capture Source", control: None, source: "VINL4" },
    snd_soc_dapm_route { sink: "ADC1 Left Capture Source", control: None, source: "VINR4" },
    snd_soc_dapm_route { sink: "ADC1", control: None, source: "ADC1 Left Capture Source" },
    snd_soc_dapm_route { sink: "ADC1 Right Capture Source", control: None, source: "VINL1" },
    snd_soc_dapm_route { sink: "ADC1 Right Capture Source", control: None, source: "VINR1" },
    snd_soc_dapm_route { sink: "ADC1 Right Capture Source", control: None, source: "VINL2" },
    snd_soc_dapm_route { sink: "ADC1 Right Capture Source", control: None, source: "VINR2" },
    snd_soc_dapm_route { sink: "ADC1 Right Capture Source", control: None, source: "VINL3" },
    snd_soc_dapm_route { sink: "ADC1 Right Capture Source", control: None, source: "VINR3" },
    snd_soc_dapm_route { sink: "ADC1 Right Capture Source", control: None, source: "VINL4" },
    snd_soc_dapm_route { sink: "ADC1 Right Capture Source", control: None, source: "VINR4" },
    snd_soc_dapm_route { sink: "ADC1", control: None, source: "ADC1 Right Capture Source" },
    snd_soc_dapm_route { sink: "ADC2 Left Capture Source", control: None, source: "VINL1" },
    snd_soc_dapm_route { sink: "ADC2 Left Capture Source", control: None, source: "VINR1" },
    snd_soc_dapm_route { sink: "ADC2 Left Capture Source", control: None, source: "VINL2" },
    snd_soc_dapm_route { sink: "ADC2 Left Capture Source", control: None, source: "VINR2" },
    snd_soc_dapm_route { sink: "ADC2 Left Capture Source", control: None, source: "VINL3" },
    snd_soc_dapm_route { sink: "ADC2 Left Capture Source", control: None, source: "VINR3" },
    snd_soc_dapm_route { sink: "ADC2 Left Capture Source", control: None, source: "VINL4" },
    snd_soc_dapm_route { sink: "ADC2 Left Capture Source", control: None, source: "VINR4" },
    snd_soc_dapm_route { sink: "ADC2", control: None, source: "ADC2 Left Capture Source" },
    snd_soc_dapm_route { sink: "ADC2 Right Capture Source", control: None, source: "VINL1" },
    snd_soc_dapm_route { sink: "ADC2 Right Capture Source", control: None, source: "VINR1" },
    snd_soc_dapm_route { sink: "ADC2 Right Capture Source", control: None, source: "VINL2" },
    snd_soc_dapm_route { sink: "ADC2 Right Capture Source", control: None, source: "VINR2" },
    snd_soc_dapm_route { sink: "ADC2 Right Capture Source", control: None, source: "VINL3" },
    snd_soc_dapm_route { sink: "ADC2 Right Capture Source", control: None, source: "VINR3" },
    snd_soc_dapm_route { sink: "ADC2 Right Capture Source", control: None, source: "VINL4" },
    snd_soc_dapm_route { sink: "ADC2 Right Capture Source", control: None, source: "VINR4" },
    snd_soc_dapm_route { sink: "ADC2", control: None, source: "ADC2 Right Capture Source" },
];

pub unsafe extern "C" fn pcm186x_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let priv_: *mut pcm186x_priv = snd_soc_component_get_drvdata(component) as *mut pcm186x_priv;
    let rate: c_uint = params_rate(params);
    let format: snd_pcm_format_t = params_format(params);
    let width: c_uint = params_width(params);
    let channels: c_uint = params_channels(params);
    let mut div_lrck: c_uint;
    let div_bck: c_uint;
    let mut tdm_tx_sel: u8 = 0;
    let mut pcm_cfg: u8 = 0;

    dev_dbg!((*component).dev, "%s() rate=%u format=0x%x width=%u channels=%u\n", __func__, rate, format, width, channels);

    match width {
        16 => {
            pcm_cfg = ((PCM186X_PCM_CFG_RX_WLEN_16 << PCM186X_PCM_CFG_RX_WLEN_SHIFT)
                | (PCM186X_PCM_CFG_TX_WLEN_16 << PCM186X_PCM_CFG_TX_WLEN_SHIFT)) as u8;
        }
        20 => {
            pcm_cfg = ((PCM186X_PCM_CFG_RX_WLEN_20 << PCM186X_PCM_CFG_RX_WLEN_SHIFT)
                | (PCM186X_PCM_CFG_TX_WLEN_20 << PCM186X_PCM_CFG_TX_WLEN_SHIFT)) as u8;
        }
        24 => {
            pcm_cfg = ((PCM186X_PCM_CFG_RX_WLEN_24 << PCM186X_PCM_CFG_RX_WLEN_SHIFT)
                | (PCM186X_PCM_CFG_TX_WLEN_24 << PCM186X_PCM_CFG_TX_WLEN_SHIFT)) as u8;
        }
        32 => {
            pcm_cfg = ((PCM186X_PCM_CFG_RX_WLEN_32 << PCM186X_PCM_CFG_RX_WLEN_SHIFT)
                | (PCM186X_PCM_CFG_TX_WLEN_32 << PCM186X_PCM_CFG_TX_WLEN_SHIFT)) as u8;
        }
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(
        component,
        PCM186X_PCM_CFG,
        PCM186X_PCM_CFG_RX_WLEN_MASK | PCM186X_PCM_CFG_TX_WLEN_MASK,
        pcm_cfg as c_uint,
    );

    div_lrck = width.wrapping_mul(channels);

    if (*priv_).is_tdm_mode {
        /* Select TDM transmission data */
        match channels {
            2 => tdm_tx_sel = PCM186X_TDM_TX_SEL_2CH as u8,
            4 => tdm_tx_sel = PCM186X_TDM_TX_SEL_4CH as u8,
            6 => tdm_tx_sel = PCM186X_TDM_TX_SEL_6CH as u8,
            _ => return -EINVAL,
        }

        snd_soc_component_update_bits(
            component,
            PCM186X_TDM_TX_SEL,
            PCM186X_TDM_TX_SEL_MASK,
            tdm_tx_sel as c_uint,
        );

        /* In DSP/TDM mode, the LRCLK divider must be 256 */
        div_lrck = 256;

        /* Configure 1/256 duty cycle for LRCK */
        snd_soc_component_update_bits(
            component,
            PCM186X_PCM_CFG,
            PCM186X_PCM_CFG_TDM_LRCK_MODE,
            PCM186X_PCM_CFG_TDM_LRCK_MODE,
        );
    }

    /* Only configure clock dividers in provider mode. */
    if (*priv_).is_provider_mode {
        div_bck = (*priv_).sysclk / div_lrck.wrapping_mul(rate);

        dev_dbg!((*component).dev, "%s() master_clk=%u div_bck=%u div_lrck=%u\n", __func__, (*priv_).sysclk, div_bck, div_lrck);

        snd_soc_component_write(component, PCM186X_BCK_DIV, div_bck.wrapping_sub(1));
        snd_soc_component_write(component, PCM186X_LRK_DIV, div_lrck.wrapping_sub(1));
    }

    0
}

pub unsafe extern "C" fn pcm186x_set_fmt(dai: *mut snd_soc_dai, format: c_uint) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let priv_: *mut pcm186x_priv = snd_soc_component_get_drvdata(component) as *mut pcm186x_priv;
    let mut clk_ctrl: u8 = 0;
    let mut pcm_cfg: u8 = 0;

    dev_dbg!((*component).dev, "%s() format=0x%x\n", __func__, format);

    match format & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBP_CFP => {
            if (*priv_).sysclk == 0 {
                dev_err!((*component).dev, "operating in provider mode requires sysclock to be configured\n");
                return -EINVAL;
            }
            clk_ctrl |= PCM186X_CLK_CTRL_MST_MODE as u8;
            (*priv_).is_provider_mode = true;
        }
        SND_SOC_DAIFMT_CBC_CFC => {
            (*priv_).is_provider_mode = false;
        }
        _ => {
            dev_err!((*component).dev, "Invalid DAI master/slave interface\n");
            return -EINVAL;
        }
    }

    /* set interface polarity */
    match format & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        _ => {
            dev_err!((*component).dev, "Inverted DAI clocks not supported\n");
            return -EINVAL;
        }
    }

    /* set interface format */
    match format & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            pcm_cfg = PCM186X_PCM_CFG_FMT_I2S as u8;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            pcm_cfg = PCM186X_PCM_CFG_FMT_LEFTJ as u8;
        }
        SND_SOC_DAIFMT_DSP_A => {
            (*priv_).tdm_offset = (*priv_).tdm_offset.wrapping_add(1);
            /* fallthrough */
            /*
             * DSP_A uses the same basic config as DSP_B
             * except we need to shift the TDM output by one BCK cycle
             */
            (*priv_).is_tdm_mode = true;
            pcm_cfg = PCM186X_PCM_CFG_FMT_TDM as u8;
        }
        SND_SOC_DAIFMT_DSP_B => {
            (*priv_).is_tdm_mode = true;
            pcm_cfg = PCM186X_PCM_CFG_FMT_TDM as u8;
        }
        _ => {
            dev_err!((*component).dev, "Invalid DAI format\n");
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, PCM186X_CLK_CTRL, PCM186X_CLK_CTRL_MST_MODE, clk_ctrl as c_uint);
    snd_soc_component_write(component, PCM186X_TDM_TX_OFFSET, (*priv_).tdm_offset);
    snd_soc_component_update_bits(component, PCM186X_PCM_CFG, PCM186X_PCM_CFG_FMT_MASK, pcm_cfg as c_uint);

    0
}

pub unsafe extern "C" fn pcm186x_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let priv_: *mut pcm186x_priv = snd_soc_component_get_drvdata(component) as *mut pcm186x_priv;
    let first_slot: c_uint;
    let last_slot: c_uint;
    let tdm_offset: c_uint;

    dev_dbg!((*component).dev, "%s() tx_mask=0x%x rx_mask=0x%x slots=%d slot_width=%d\n", __func__, tx_mask, rx_mask, slots, slot_width);

    if tx_mask == 0 {
        dev_err!((*component).dev, "tdm tx mask must not be 0\n");
        return -EINVAL;
    }

    first_slot = __ffs(tx_mask);
    last_slot = __fls(tx_mask);

    if last_slot.wrapping_sub(first_slot) != hweight32(tx_mask).wrapping_sub(1) {
        dev_err!((*component).dev, "tdm tx mask must be contiguous\n");
        return -EINVAL;
    }

    tdm_offset = first_slot.wrapping_mul(slot_width as c_uint);

    if tdm_offset > 255 {
        dev_err!((*component).dev, "tdm tx slot selection out of bounds\n");
        return -EINVAL;
    }

    (*priv_).tdm_offset = tdm_offset;

    0
}

pub unsafe extern "C" fn pcm186x_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let priv_: *mut pcm186x_priv = snd_soc_component_get_drvdata(component) as *mut pcm186x_priv;

    dev_dbg!((*component).dev, "%s() clk_id=%d freq=%u dir=%d\n", __func__, clk_id, freq, dir);

    (*priv_).sysclk = freq;

    0
}

pub static pcm186x_selectable_formats: u64 =
    SND_SOC_POSSIBLE_DAIFMT_I2S
        | SND_SOC_POSSIBLE_DAIFMT_LEFT_J
        | SND_SOC_POSSIBLE_DAIFMT_DSP_A
        | SND_SOC_POSSIBLE_DAIFMT_DSP_B
        | SND_SOC_POSSIBLE_DAIFMT_NB_NF;

pub static pcm186x_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_sysclk: Some(pcm186x_set_dai_sysclk),
    set_tdm_slot: Some(pcm186x_set_tdm_slot),
    set_fmt: Some(pcm186x_set_fmt),
    hw_params: Some(pcm186x_hw_params),
    auto_selectable_formats: &pcm186x_selectable_formats,
    num_auto_selectable_formats: 1,
};

pub static mut pcm1863_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: "pcm1863-aif",
    capture: snd_soc_pcm_stream {
        stream_name: "Capture",
        channels_min: 1,
        channels_max: 2,
        rates: PCM186X_RATES,
        formats: PCM186X_FORMATS,
    },
    ops: &pcm186x_dai_ops,
};

pub static mut pcm1865_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: "pcm1865-aif",
    capture: snd_soc_pcm_stream {
        stream_name: "Capture",
        channels_min: 1,
        channels_max: 4,
        rates: PCM186X_RATES,
        formats: PCM186X_FORMATS,
    },
    ops: &pcm186x_dai_ops,
};

pub unsafe extern "C" fn pcm186x_power_on(component: *mut snd_soc_component) -> c_int {
    let priv_: *mut pcm186x_priv = snd_soc_component_get_drvdata(component) as *mut pcm186x_priv;
    let mut ret: c_int = 0;

    ret = regulator_bulk_enable((*priv_).supplies.len(), (*priv_).supplies.as_mut_ptr());
    if ret != 0 {
        return ret;
    }

    regcache_cache_only((*priv_).regmap, false);
    ret = regcache_sync((*priv_).regmap);
    if ret != 0 {
        dev_err!((*component).dev, "Failed to restore cache\n");
        regcache_cache_only((*priv_).regmap, true);
        regulator_bulk_disable((*priv_).supplies.len(), (*priv_).supplies.as_mut_ptr());
        return ret;
    }

    snd_soc_component_update_bits(component, PCM186X_POWER_CTRL, PCM186X_PWR_CTRL_PWRDN, 0);

    0
}

pub unsafe extern "C" fn pcm186x_power_off(component: *mut snd_soc_component) -> c_int {
    let priv_: *mut pcm186x_priv = snd_soc_component_get_drvdata(component) as *mut pcm186x_priv;

    snd_soc_component_update_bits(
        component,
        PCM186X_POWER_CTRL,
        PCM186X_PWR_CTRL_PWRDN,
        PCM186X_PWR_CTRL_PWRDN,
    );

    regcache_cache_only((*priv_).regmap, true);

    regulator_bulk_disable((*priv_).supplies.len(), (*priv_).supplies.as_mut_ptr())
}

pub unsafe extern "C" fn pcm186x_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let dapm: *mut snd_soc_dapm_context = snd_soc_component_to_dapm(component);

    dev_dbg!((*component).dev, "## %s: %d -> %d\n", __func__, snd_soc_dapm_get_bias_level(dapm), level);

    match level {
        SND_SOC_BIAS_ON => {}
        SND_SOC_BIAS_PREPARE => {}
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                pcm186x_power_on(component);
            }
        }
        SND_SOC_BIAS_OFF => {
            pcm186x_power_off(component);
        }
    }

    0
}

pub static soc_codec_dev_pcm1863: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(pcm186x_set_bias_level),
    controls: pcm1863_snd_controls.as_ptr(),
    num_controls: pcm1863_snd_controls.len(),
    dapm_widgets: pcm1863_dapm_widgets.as_ptr(),
    num_dapm_widgets: pcm1863_dapm_widgets.len(),
    dapm_routes: pcm1863_dapm_routes.as_ptr(),
    num_dapm_routes: pcm1863_dapm_routes.len(),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

pub static soc_codec_dev_pcm1865: snd_soc_component_driver = snd_soc_component_driver {
    set_bias_level: Some(pcm186x_set_bias_level),
    controls: pcm1865_snd_controls.as_ptr(),
    num_controls: pcm1865_snd_controls.len(),
    dapm_widgets: pcm1865_dapm_widgets.as_ptr(),
    num_dapm_widgets: pcm1865_dapm_widgets.len(),
    dapm_routes: pcm1865_dapm_routes.as_ptr(),
    num_dapm_routes: pcm1865_dapm_routes.len(),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

pub unsafe extern "C" fn pcm186x_volatile(dev: *mut device, reg: c_uint) -> bool {
    match reg {
        PCM186X_PAGE
        | PCM186X_DEVICE_STATUS
        | PCM186X_FSAMPLE_STATUS
        | PCM186X_DIV_STATUS
        | PCM186X_CLK_STATUS
        | PCM186X_SUPPLY_STATUS
        | PCM186X_MMAP_STAT_CTRL
        | PCM186X_MMAP_ADDRESS => true,
        _ => false,
    }
}

pub static pcm186x_range: regmap_range_cfg = regmap_range_cfg {
    name: "Pages",
    range_max: PCM186X_MAX_REGISTER,
    selector_reg: PCM186X_PAGE,
    selector_mask: 0xff,
    window_len: PCM186X_PAGE_LEN,
};

pub static pcm186x_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    volatile_reg: Some(pcm186x_volatile),
    ranges: &pcm186x_range,
    num_ranges: 1,
    max_register: PCM186X_MAX_REGISTER,
    cache_type: REGCACHE_RBTREE,
};
EXPORT_SYMBOL_GPL!(pcm186x_regmap);

pub unsafe extern "C" fn pcm186x_probe(
    dev: *mut device,
    type_: pcm186x_type,
    irq: c_int,
    regmap: *mut regmap,
) -> c_int {
    let priv_: *mut pcm186x_priv;
    let mut i: c_int;
    let mut ret: c_int;

    priv_ = devm_kzalloc(dev, core::mem::size_of::<pcm186x_priv>(), GFP_KERNEL) as *mut pcm186x_priv;
    if priv_.is_null() {
        return -ENOMEM;
    }

    dev_set_drvdata(dev, priv_ as *mut c_void);
    (*priv_).regmap = regmap;

    i = 0;
    while (i as usize) < (*priv_).supplies.len() {
        (*priv_).supplies[i as usize].supply = pcm186x_supply_names[i as usize];
        i += 1;
    }

    ret = devm_regulator_bulk_get(dev, (*priv_).supplies.len(), (*priv_).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err!(dev, "failed to request supplies: %d\n", ret);
        return ret;
    }

    ret = regulator_bulk_enable((*priv_).supplies.len(), (*priv_).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err!(dev, "failed enable supplies: %d\n", ret);
        return ret;
    }

    /* Reset device registers for a consistent power-on like state */
    ret = regmap_write(regmap, PCM186X_PAGE, PCM186X_RESET);
    if ret != 0 {
        dev_err!(dev, "failed to write device: %d\n", ret);
        return ret;
    }

    ret = regulator_bulk_disable((*priv_).supplies.len(), (*priv_).supplies.as_mut_ptr());
    if ret != 0 {
        dev_err!(dev, "failed disable supplies: %d\n", ret);
        return ret;
    }

    match type_ {
        PCM1865 | PCM1864 => {
            ret = devm_snd_soc_register_component(dev, &soc_codec_dev_pcm1865, &mut pcm1865_dai, 1);
        }
        PCM1863 | PCM1862 | _ => {
            ret = devm_snd_soc_register_component(dev, &soc_codec_dev_pcm1863, &mut pcm1863_dai, 1);
        }
    }
    if ret != 0 {
        dev_err!(dev, "failed to register CODEC: %d\n", ret);
        return ret;
    }

    0
}
EXPORT_SYMBOL_GPL!(pcm186x_probe);

MODULE_AUTHOR!("Andreas Dannenberg <dannenberg@ti.com>");
MODULE_AUTHOR!("Andrew F. Davis <afd@ti.com>");
MODULE_DESCRIPTION!("PCM186x Universal Audio ADC driver");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
