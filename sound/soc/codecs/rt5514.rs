// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt5514.rs  --  RT5514 ALSA SoC audio codec driver
 *
 * Copyright 2015 Realtek Semiconductor Corp.
 * Author: Oder Chiou <oder_chiou@realtek.com>
 */

/* Translated from rt5514.c. C include dependencies are expected to be supplied
 * by the surrounding kernel/ASoC Rust binding layer.
 */

static RT5514_I2C_PATCH: &[reg_sequence] = &[
    reg_sequence { reg: 0x1800101c, def: 0x00000000 },
    reg_sequence { reg: 0x18001100, def: 0x0000031f },
    reg_sequence { reg: 0x18001104, def: 0x00000007 },
    reg_sequence { reg: 0x18001108, def: 0x00000000 },
    reg_sequence { reg: 0x1800110c, def: 0x00000000 },
    reg_sequence { reg: 0x18001110, def: 0x00000000 },
    reg_sequence { reg: 0x18001114, def: 0x00000001 },
    reg_sequence { reg: 0x18001118, def: 0x00000000 },
    reg_sequence { reg: 0x18002f08, def: 0x00000006 },
    reg_sequence { reg: 0x18002f00, def: 0x00055149 },
    reg_sequence { reg: 0x18002f00, def: 0x0005514b },
    reg_sequence { reg: 0x18002f00, def: 0x00055149 },
    reg_sequence { reg: 0xfafafafa, def: 0x00000001 },
    reg_sequence { reg: 0x18002f10, def: 0x00000001 },
    reg_sequence { reg: 0x18002f10, def: 0x00000000 },
    reg_sequence { reg: 0x18002f10, def: 0x00000001 },
    reg_sequence { reg: 0xfafafafa, def: 0x00000001 },
    reg_sequence { reg: 0x18002000, def: 0x000010ec },
    reg_sequence { reg: 0xfafafafa, def: 0x00000000 },
];

static RT5514_PATCH: &[reg_sequence] = &[
    reg_sequence { reg: RT5514_DIG_IO_CTRL, def: 0x00000040 },
    reg_sequence { reg: RT5514_CLK_CTRL1, def: 0x38020041 },
    reg_sequence { reg: RT5514_SRC_CTRL, def: 0x44000eee },
    reg_sequence { reg: RT5514_ANA_CTRL_LDO10, def: 0x00028604 },
    reg_sequence { reg: RT5514_ANA_CTRL_ADCFED, def: 0x00000800 },
    reg_sequence { reg: RT5514_ASRC_IN_CTRL1, def: 0x00000003 },
    reg_sequence { reg: RT5514_DOWNFILTER0_CTRL3, def: 0x10000342 },
    reg_sequence { reg: RT5514_DOWNFILTER1_CTRL3, def: 0x10000342 },
];

static RT5514_REG: &[reg_default] = &[
    reg_default { reg: RT5514_RESET, def: 0x00000000 },
    reg_default { reg: RT5514_PWR_ANA1, def: 0x00808880 },
    reg_default { reg: RT5514_PWR_ANA2, def: 0x00220000 },
    reg_default { reg: RT5514_I2S_CTRL1, def: 0x00000330 },
    reg_default { reg: RT5514_I2S_CTRL2, def: 0x20000000 },
    reg_default { reg: RT5514_VAD_CTRL6, def: 0xc00007d2 },
    reg_default { reg: RT5514_EXT_VAD_CTRL, def: 0x80000080 },
    reg_default { reg: RT5514_DIG_IO_CTRL, def: 0x00000040 },
    reg_default { reg: RT5514_PAD_CTRL1, def: 0x00804000 },
    reg_default { reg: RT5514_DMIC_DATA_CTRL, def: 0x00000005 },
    reg_default { reg: RT5514_DIG_SOURCE_CTRL, def: 0x00000002 },
    reg_default { reg: RT5514_SRC_CTRL, def: 0x44000eee },
    reg_default { reg: RT5514_DOWNFILTER2_CTRL1, def: 0x0000882f },
    reg_default { reg: RT5514_PLL_SOURCE_CTRL, def: 0x00000004 },
    reg_default { reg: RT5514_CLK_CTRL1, def: 0x38020041 },
    reg_default { reg: RT5514_CLK_CTRL2, def: 0x00000000 },
    reg_default { reg: RT5514_PLL3_CALIB_CTRL1, def: 0x00400200 },
    reg_default { reg: RT5514_PLL3_CALIB_CTRL5, def: 0x40220012 },
    reg_default { reg: RT5514_DELAY_BUF_CTRL1, def: 0x7fff006a },
    reg_default { reg: RT5514_DELAY_BUF_CTRL3, def: 0x00000000 },
    reg_default { reg: RT5514_ASRC_IN_CTRL1, def: 0x00000003 },
    reg_default { reg: RT5514_DOWNFILTER0_CTRL1, def: 0x00020c2f },
    reg_default { reg: RT5514_DOWNFILTER0_CTRL2, def: 0x00020c2f },
    reg_default { reg: RT5514_DOWNFILTER0_CTRL3, def: 0x10000342 },
    reg_default { reg: RT5514_DOWNFILTER1_CTRL1, def: 0x00020c2f },
    reg_default { reg: RT5514_DOWNFILTER1_CTRL2, def: 0x00020c2f },
    reg_default { reg: RT5514_DOWNFILTER1_CTRL3, def: 0x10000342 },
    reg_default { reg: RT5514_ANA_CTRL_LDO10, def: 0x00028604 },
    reg_default { reg: RT5514_ANA_CTRL_LDO18_16, def: 0x02000345 },
    reg_default { reg: RT5514_ANA_CTRL_ADC12, def: 0x0000a2a8 },
    reg_default { reg: RT5514_ANA_CTRL_ADC21, def: 0x00001180 },
    reg_default { reg: RT5514_ANA_CTRL_ADC22, def: 0x0000aaa8 },
    reg_default { reg: RT5514_ANA_CTRL_ADC23, def: 0x00151427 },
    reg_default { reg: RT5514_ANA_CTRL_MICBST, def: 0x00002000 },
    reg_default { reg: RT5514_ANA_CTRL_ADCFED, def: 0x00000800 },
    reg_default { reg: RT5514_ANA_CTRL_INBUF, def: 0x00000143 },
    reg_default { reg: RT5514_ANA_CTRL_VREF, def: 0x00008d50 },
    reg_default { reg: RT5514_ANA_CTRL_PLL3, def: 0x0000000e },
    reg_default { reg: RT5514_ANA_CTRL_PLL1_1, def: 0x00000000 },
    reg_default { reg: RT5514_ANA_CTRL_PLL1_2, def: 0x00030220 },
    reg_default { reg: RT5514_DMIC_LP_CTRL, def: 0x00000000 },
    reg_default { reg: RT5514_MISC_CTRL_DSP, def: 0x00000000 },
    reg_default { reg: RT5514_DSP_CTRL1, def: 0x00055149 },
    reg_default { reg: RT5514_DSP_CTRL3, def: 0x00000006 },
    reg_default { reg: RT5514_DSP_CTRL4, def: 0x00000001 },
    reg_default { reg: RT5514_VENDOR_ID1, def: 0x00000001 },
    reg_default { reg: RT5514_VENDOR_ID2, def: 0x10ec5514 },
];

unsafe fn rt5514_enable_dsp_prepare(rt5514: *mut rt5514_priv) {
    /* Reset */
    regmap_write((*rt5514).i2c_regmap, 0x18002000, 0x000010ec);
    /* LDO_I_limit */
    regmap_write((*rt5514).i2c_regmap, 0x18002200, 0x00028604);
    /* I2C bypass enable */
    regmap_write((*rt5514).i2c_regmap, 0xfafafafa, 0x00000001);
    /* mini-core reset */
    regmap_write((*rt5514).i2c_regmap, 0x18002f00, 0x0005514b);
    regmap_write((*rt5514).i2c_regmap, 0x18002f00, 0x00055149);
    /* I2C bypass disable */
    regmap_write((*rt5514).i2c_regmap, 0xfafafafa, 0x00000000);
    /* PIN config */
    regmap_write((*rt5514).i2c_regmap, 0x18002070, 0x00000040);
    /* PLL3(QN)=RCOSC*(10+2) */
    regmap_write((*rt5514).i2c_regmap, 0x18002240, 0x0000000a);
    /* PLL3 source=RCOSC, fsi=rt_clk */
    regmap_write((*rt5514).i2c_regmap, 0x18002100, 0x0000000b);
    /* Power on RCOSC, pll3 */
    regmap_write((*rt5514).i2c_regmap, 0x18002004, 0x00808b81);
    /* DSP clk source = pll3, ENABLE DSP clk */
    regmap_write((*rt5514).i2c_regmap, 0x18002f08, 0x00000005);
    /* Enable DSP clk auto switch */
    regmap_write((*rt5514).i2c_regmap, 0x18001114, 0x00000001);
    /* Reduce DSP power */
    regmap_write((*rt5514).i2c_regmap, 0x18001118, 0x00000001);
}

unsafe fn rt5514_volatile_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        RT5514_VENDOR_ID1 | RT5514_VENDOR_ID2 => true,
        _ => false,
    }
}

unsafe fn rt5514_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        RT5514_RESET | RT5514_PWR_ANA1 | RT5514_PWR_ANA2 | RT5514_I2S_CTRL1 |
        RT5514_I2S_CTRL2 | RT5514_VAD_CTRL6 | RT5514_EXT_VAD_CTRL |
        RT5514_DIG_IO_CTRL | RT5514_PAD_CTRL1 | RT5514_DMIC_DATA_CTRL |
        RT5514_DIG_SOURCE_CTRL | RT5514_SRC_CTRL | RT5514_DOWNFILTER2_CTRL1 |
        RT5514_PLL_SOURCE_CTRL | RT5514_CLK_CTRL1 | RT5514_CLK_CTRL2 |
        RT5514_PLL3_CALIB_CTRL1 | RT5514_PLL3_CALIB_CTRL5 |
        RT5514_DELAY_BUF_CTRL1 | RT5514_DELAY_BUF_CTRL3 |
        RT5514_ASRC_IN_CTRL1 | RT5514_DOWNFILTER0_CTRL1 |
        RT5514_DOWNFILTER0_CTRL2 | RT5514_DOWNFILTER0_CTRL3 |
        RT5514_DOWNFILTER1_CTRL1 | RT5514_DOWNFILTER1_CTRL2 |
        RT5514_DOWNFILTER1_CTRL3 | RT5514_ANA_CTRL_LDO10 |
        RT5514_ANA_CTRL_LDO18_16 | RT5514_ANA_CTRL_ADC12 |
        RT5514_ANA_CTRL_ADC21 | RT5514_ANA_CTRL_ADC22 |
        RT5514_ANA_CTRL_ADC23 | RT5514_ANA_CTRL_MICBST |
        RT5514_ANA_CTRL_ADCFED | RT5514_ANA_CTRL_INBUF |
        RT5514_ANA_CTRL_VREF | RT5514_ANA_CTRL_PLL3 |
        RT5514_ANA_CTRL_PLL1_1 | RT5514_ANA_CTRL_PLL1_2 |
        RT5514_DMIC_LP_CTRL | RT5514_MISC_CTRL_DSP | RT5514_DSP_CTRL1 |
        RT5514_DSP_CTRL3 | RT5514_DSP_CTRL4 | RT5514_VENDOR_ID1 |
        RT5514_VENDOR_ID2 => true,
        _ => false,
    }
}

unsafe fn rt5514_i2c_readable_register(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == (RT5514_DSP_MAPPING | RT5514_RESET) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_PWR_ANA1) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_PWR_ANA2) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_I2S_CTRL1) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_I2S_CTRL2) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_VAD_CTRL6) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_EXT_VAD_CTRL) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DIG_IO_CTRL) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_PAD_CTRL1) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DMIC_DATA_CTRL) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DIG_SOURCE_CTRL) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_SRC_CTRL) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DOWNFILTER2_CTRL1) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_PLL_SOURCE_CTRL) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_CLK_CTRL1) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_CLK_CTRL2) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_PLL3_CALIB_CTRL1) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_PLL3_CALIB_CTRL5) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DELAY_BUF_CTRL1) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DELAY_BUF_CTRL3) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ASRC_IN_CTRL1) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DOWNFILTER0_CTRL1) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DOWNFILTER0_CTRL2) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DOWNFILTER0_CTRL3) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DOWNFILTER1_CTRL1) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DOWNFILTER1_CTRL2) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DOWNFILTER1_CTRL3) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ANA_CTRL_LDO10) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ANA_CTRL_LDO18_16) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ANA_CTRL_ADC12) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ANA_CTRL_ADC21) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ANA_CTRL_ADC22) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ANA_CTRL_ADC23) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ANA_CTRL_MICBST) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ANA_CTRL_ADCFED) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ANA_CTRL_INBUF) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ANA_CTRL_VREF) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ANA_CTRL_PLL3) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ANA_CTRL_PLL1_1) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_ANA_CTRL_PLL1_2) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DMIC_LP_CTRL) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_MISC_CTRL_DSP) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DSP_CTRL1) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DSP_CTRL3) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_DSP_CTRL4) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_VENDOR_ID1) => true,
        x if x == (RT5514_DSP_MAPPING | RT5514_VENDOR_ID2) => true,
        _ => false,
    }
}

/* {-3, 0, +3, +4.5, +7.5, +9.5, +12, +14, +17} dB */
static BST_TLV: &[c_uint] = DECLARE_TLV_DB_RANGE!(
    0, 2, TLV_DB_SCALE_ITEM!(-300, 300, 0),
    3, 3, TLV_DB_SCALE_ITEM!(450, 0, 0),
    4, 4, TLV_DB_SCALE_ITEM!(750, 0, 0),
    5, 5, TLV_DB_SCALE_ITEM!(950, 0, 0),
    6, 6, TLV_DB_SCALE_ITEM!(1200, 0, 0),
    7, 7, TLV_DB_SCALE_ITEM!(1400, 0, 0),
    8, 8, TLV_DB_SCALE_ITEM!(1700, 0, 0)
);

static ADC_VOL_TLV: &[c_uint] = DECLARE_TLV_DB_SCALE!(-1725, 75, 0);

unsafe fn rt5514_dsp_voice_wake_up_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let rt5514 = snd_soc_component_get_drvdata(component) as *mut rt5514_priv;

    (*ucontrol).value.integer.value[0] = (*rt5514).dsp_enabled as _;
    0
}

unsafe fn rt5514_calibration(rt5514: *mut rt5514_priv, on: bool) -> c_int {
    if on {
        regmap_write((*rt5514).regmap, RT5514_ANA_CTRL_PLL3, 0x0000000a);
        regmap_update_bits((*rt5514).regmap, RT5514_PLL_SOURCE_CTRL, 0xf, 0xa);
        regmap_update_bits((*rt5514).regmap, RT5514_PWR_ANA1, 0x301, 0x301);
        regmap_write(
            (*rt5514).regmap,
            RT5514_PLL3_CALIB_CTRL4,
            0x80000000 | (*rt5514).pll3_cal_value,
        );
        regmap_write((*rt5514).regmap, RT5514_PLL3_CALIB_CTRL1, 0x8bb80800);
        regmap_update_bits(
            (*rt5514).regmap,
            RT5514_PLL3_CALIB_CTRL5,
            0xc0000000,
            0x80000000,
        );
        regmap_update_bits(
            (*rt5514).regmap,
            RT5514_PLL3_CALIB_CTRL5,
            0xc0000000,
            0xc0000000,
        );
    } else {
        regmap_update_bits(
            (*rt5514).regmap,
            RT5514_PLL3_CALIB_CTRL5,
            0xc0000000,
            0x40000000,
        );
        regmap_update_bits((*rt5514).regmap, RT5514_PWR_ANA1, 0x301, 0);
        regmap_update_bits((*rt5514).regmap, RT5514_PLL_SOURCE_CTRL, 0xf, 0x4);
    }

    0
}

unsafe fn rt5514_dsp_voice_wake_up_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_kcontrol_chip(kcontrol);
    let dapm = snd_soc_component_to_dapm(component);
    let rt5514 = snd_soc_component_get_drvdata(component) as *mut rt5514_priv;
    let mut fw: *const firmware = core::ptr::null();
    let mut buf = [0u8; 8];

    if (*ucontrol).value.integer.value[0] == (*rt5514).dsp_enabled as _ {
        return 0;
    }

    if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
        (*rt5514).dsp_enabled = (*ucontrol).value.integer.value[0] as _;

        if (*rt5514).dsp_enabled != 0 {
            if !(*rt5514).pdata.dsp_calib_clk_name.is_null() && !IS_ERR((*rt5514).dsp_calib_clk) {
                if clk_set_rate((*rt5514).dsp_calib_clk, (*rt5514).pdata.dsp_calib_clk_rate) != 0 {
                    dev_err((*component).dev, c_str!("Can't set rate for mclk"));
                }

                if clk_prepare_enable((*rt5514).dsp_calib_clk) != 0 {
                    dev_err((*component).dev, c_str!("Can't enable dsp_calib_clk"));
                }

                rt5514_calibration(rt5514, true);
                msleep(20);

                /* If CONFIG_SND_SOC_RT5514_SPI is enabled:
                 * rt5514_spi_burst_read(RT5514_PLL3_CALIB_CTRL6 | RT5514_DSP_MAPPING, buf, sizeof(buf));
                 * otherwise log the missing SPI driver and clear buf.
                 */
                if cfg!(CONFIG_SND_SOC_RT5514_SPI) {
                    rt5514_spi_burst_read(
                        RT5514_PLL3_CALIB_CTRL6 | RT5514_DSP_MAPPING,
                        buf.as_mut_ptr(),
                        buf.len(),
                    );
                } else {
                    dev_err(
                        (*component).dev,
                        c_str!("There is no SPI driver for loading the firmware\n"),
                    );
                    memset(buf.as_mut_ptr() as *mut c_void, 0, buf.len());
                }

                (*rt5514).pll3_cal_value = (buf[0] as c_uint)
                    | ((buf[1] as c_uint) << 8)
                    | ((buf[2] as c_uint) << 16)
                    | ((buf[3] as c_uint) << 24);

                rt5514_calibration(rt5514, false);
                clk_disable_unprepare((*rt5514).dsp_calib_clk);
            }

            rt5514_enable_dsp_prepare(rt5514);

            request_firmware(&mut fw, RT5514_FIRMWARE1, (*component).dev);
            if !fw.is_null() {
                if cfg!(CONFIG_SND_SOC_RT5514_SPI) {
                    rt5514_spi_burst_write(
                        0x4ff60000,
                        (*fw).data,
                        (((*fw).size / 8) + 1) * 8,
                    );
                } else {
                    dev_err(
                        (*component).dev,
                        c_str!("There is no SPI driver for loading the firmware\n"),
                    );
                }
                release_firmware(fw);
                fw = core::ptr::null();
            }

            request_firmware(&mut fw, RT5514_FIRMWARE2, (*component).dev);
            if !fw.is_null() {
                if cfg!(CONFIG_SND_SOC_RT5514_SPI) {
                    rt5514_spi_burst_write(
                        0x4ffc0000,
                        (*fw).data,
                        (((*fw).size / 8) + 1) * 8,
                    );
                } else {
                    dev_err(
                        (*component).dev,
                        c_str!("There is no SPI driver for loading the firmware\n"),
                    );
                }
                release_firmware(fw);
                fw = core::ptr::null();
            }

            /* DSP run */
            regmap_write((*rt5514).i2c_regmap, 0x18002f00, 0x00055148);

            if !(*rt5514).pdata.dsp_calib_clk_name.is_null() && !IS_ERR((*rt5514).dsp_calib_clk) {
                msleep(20);
                regmap_write((*rt5514).i2c_regmap, 0x1800211c, (*rt5514).pll3_cal_value);
                regmap_write((*rt5514).i2c_regmap, 0x18002124, 0x00220012);
                regmap_write((*rt5514).i2c_regmap, 0x18002124, 0x80220042);
                regmap_write((*rt5514).i2c_regmap, 0x18002124, 0xe0220042);
            }
        } else {
            regmap_multi_reg_write((*rt5514).i2c_regmap, RT5514_I2C_PATCH.as_ptr(), RT5514_I2C_PATCH.len());
            regcache_mark_dirty((*rt5514).regmap);
            regcache_sync((*rt5514).regmap);
        }
    }

    1
}

static RT5514_SND_CONTROLS: &[snd_kcontrol_new] = &[
    SOC_DOUBLE_TLV!("MIC Boost Volume", RT5514_ANA_CTRL_MICBST, RT5514_SEL_BSTL_SFT, RT5514_SEL_BSTR_SFT, 8, 0, BST_TLV),
    SOC_DOUBLE_R_TLV!("ADC1 Capture Volume", RT5514_DOWNFILTER0_CTRL1, RT5514_DOWNFILTER0_CTRL2, RT5514_AD_GAIN_SFT, 63, 0, ADC_VOL_TLV),
    SOC_DOUBLE_R_TLV!("ADC2 Capture Volume", RT5514_DOWNFILTER1_CTRL1, RT5514_DOWNFILTER1_CTRL2, RT5514_AD_GAIN_SFT, 63, 0, ADC_VOL_TLV),
    SOC_SINGLE_EXT!("DSP Voice Wake Up", SND_SOC_NOPM, 0, 1, 0, rt5514_dsp_voice_wake_up_get, rt5514_dsp_voice_wake_up_put),
];

/* ADC Mixer*/
static RT5514_STO1_ADC_L_MIX: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("DMIC Switch", RT5514_DOWNFILTER0_CTRL1, RT5514_AD_DMIC_MIX_BIT, 1, 1),
    SOC_DAPM_SINGLE!("ADC Switch", RT5514_DOWNFILTER0_CTRL1, RT5514_AD_AD_MIX_BIT, 1, 1),
];

static RT5514_STO1_ADC_R_MIX: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("DMIC Switch", RT5514_DOWNFILTER0_CTRL2, RT5514_AD_DMIC_MIX_BIT, 1, 1),
    SOC_DAPM_SINGLE!("ADC Switch", RT5514_DOWNFILTER0_CTRL2, RT5514_AD_AD_MIX_BIT, 1, 1),
];

static RT5514_STO2_ADC_L_MIX: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("DMIC Switch", RT5514_DOWNFILTER1_CTRL1, RT5514_AD_DMIC_MIX_BIT, 1, 1),
    SOC_DAPM_SINGLE!("ADC Switch", RT5514_DOWNFILTER1_CTRL1, RT5514_AD_AD_MIX_BIT, 1, 1),
];

static RT5514_STO2_ADC_R_MIX: &[snd_kcontrol_new] = &[
    SOC_DAPM_SINGLE!("DMIC Switch", RT5514_DOWNFILTER1_CTRL2, RT5514_AD_DMIC_MIX_BIT, 1, 1),
    SOC_DAPM_SINGLE!("ADC Switch", RT5514_DOWNFILTER1_CTRL2, RT5514_AD_AD_MIX_BIT, 1, 1),
];

/* DMIC Source */
static RT5514_DMIC_SRC: &[*const c_char] = &[c_str!("DMIC1"), c_str!("DMIC2")];

static RT5514_STEREO1_DMIC_ENUM: soc_enum =
    SOC_ENUM_SINGLE_DECL!(RT5514_DIG_SOURCE_CTRL, RT5514_AD0_DMIC_INPUT_SEL_SFT, RT5514_DMIC_SRC);
static RT5514_STO1_DMIC_MUX: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Stereo1 DMIC Source", RT5514_STEREO1_DMIC_ENUM);
static RT5514_STEREO2_DMIC_ENUM: soc_enum =
    SOC_ENUM_SINGLE_DECL!(RT5514_DIG_SOURCE_CTRL, RT5514_AD1_DMIC_INPUT_SEL_SFT, RT5514_DMIC_SRC);
static RT5514_STO2_DMIC_MUX: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Stereo2 DMIC Source", RT5514_STEREO2_DMIC_ENUM);

/**
 * rt5514_calc_dmic_clk - Calculate the frequency divider parameter of dmic.
 *
 * @component: only used for dev_warn
 * @rate: base clock rate.
 *
 * Choose divider parameter that gives the highest possible DMIC frequency in
 * 1MHz - 3MHz range.
 */
unsafe fn rt5514_calc_dmic_clk(component: *mut snd_soc_component, rate: c_int) -> c_int {
    static DIV: [c_int; 8] = [2, 3, 4, 8, 12, 16, 24, 32];

    if rate < 1000000 * DIV[0] {
        pr_warn(c_str!("Base clock rate %d is too low\n"), rate);
        return -EINVAL;
    }

    for i in 0..DIV.len() {
        /* find divider that gives DMIC frequency below 3.072MHz */
        if 3072000 * DIV[i] >= rate {
            return i as c_int;
        }
    }

    dev_warn((*component).dev, c_str!("Base clock rate %d is too high\n"), rate);
    -EINVAL
}

unsafe fn rt5514_set_dmic_clk(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    _event: c_int,
) -> c_int {
    let component = snd_soc_dapm_to_component((*w).dapm);
    let rt5514 = snd_soc_component_get_drvdata(component) as *mut rt5514_priv;
    let idx = rt5514_calc_dmic_clk(component, (*rt5514).sysclk);

    if idx < 0 {
        dev_err((*component).dev, c_str!("Failed to set DMIC clock\n"));
    } else {
        regmap_update_bits(
            (*rt5514).regmap,
            RT5514_CLK_CTRL1,
            RT5514_CLK_DMIC_OUT_SEL_MASK,
            (idx as c_uint) << RT5514_CLK_DMIC_OUT_SEL_SFT,
        );
    }

    if (*rt5514).pdata.dmic_init_delay != 0 {
        msleep((*rt5514).pdata.dmic_init_delay);
    }

    idx
}

unsafe fn rt5514_is_sys_clk_from_pll(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let component = snd_soc_dapm_to_component((*source).dapm);
    let rt5514 = snd_soc_component_get_drvdata(component) as *mut rt5514_priv;

    if (*rt5514).sysclk_src == RT5514_SCLK_S_PLL1 { 1 } else { 0 }
}

unsafe fn rt5514_i2s_use_asrc(
    source: *mut snd_soc_dapm_widget,
    _sink: *mut snd_soc_dapm_widget,
) -> c_int {
    let component = snd_soc_dapm_to_component((*source).dapm);
    let rt5514 = snd_soc_component_get_drvdata(component) as *mut rt5514_priv;

    ((*rt5514).sysclk > (*rt5514).lrck * 384) as c_int
}

static RT5514_DAPM_WIDGETS: &[snd_soc_dapm_widget] = &[
    /* Input Lines */
    SND_SOC_DAPM_INPUT!("DMIC1L"),
    SND_SOC_DAPM_INPUT!("DMIC1R"),
    SND_SOC_DAPM_INPUT!("DMIC2L"),
    SND_SOC_DAPM_INPUT!("DMIC2R"),
    SND_SOC_DAPM_INPUT!("AMICL"),
    SND_SOC_DAPM_INPUT!("AMICR"),
    SND_SOC_DAPM_PGA!("DMIC1", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("DMIC2", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!("DMIC CLK", 1, SND_SOC_NOPM, 0, 0, rt5514_set_dmic_clk, SND_SOC_DAPM_PRE_PMU),
    SND_SOC_DAPM_SUPPLY!("ADC CLK", RT5514_CLK_CTRL1, RT5514_CLK_AD_ANA1_EN_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("LDO18 IN", RT5514_PWR_ANA1, RT5514_POW_LDO18_IN_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("LDO18 ADC", RT5514_PWR_ANA1, RT5514_POW_LDO18_ADC_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("LDO21", RT5514_PWR_ANA1, RT5514_POW_LDO21_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("BG LDO18 IN", RT5514_PWR_ANA1, RT5514_POW_BG_LDO18_IN_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("BG LDO21", RT5514_PWR_ANA1, RT5514_POW_BG_LDO21_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("BG MBIAS", RT5514_PWR_ANA2, RT5514_POW_BG_MBIAS_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("MBIAS", RT5514_PWR_ANA2, RT5514_POW_MBIAS_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("VREF2", RT5514_PWR_ANA2, RT5514_POW_VREF2_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("VREF1", RT5514_PWR_ANA2, RT5514_POW_VREF1_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADC Power", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("LDO16L", RT5514_PWR_ANA2, RT5514_POWL_LDO16_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADC1L", RT5514_PWR_ANA2, RT5514_POW_ADC1_L_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("BSTL2", RT5514_PWR_ANA2, RT5514_POW2_BSTL_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("BSTL", RT5514_PWR_ANA2, RT5514_POW_BSTL_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADCFEDL", RT5514_PWR_ANA2, RT5514_POW_ADCFEDL_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADCL Power", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("LDO16R", RT5514_PWR_ANA2, RT5514_POWR_LDO16_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADC1R", RT5514_PWR_ANA2, RT5514_POW_ADC1_R_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("BSTR2", RT5514_PWR_ANA2, RT5514_POW2_BSTR_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("BSTR", RT5514_PWR_ANA2, RT5514_POW_BSTR_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADCFEDR", RT5514_PWR_ANA2, RT5514_POW_ADCFEDR_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("ADCR Power", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("PLL1 LDO ENABLE", RT5514_ANA_CTRL_PLL1_2, RT5514_EN_LDO_PLL1_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("PLL1 LDO", RT5514_PWR_ANA2, RT5514_POW_PLL1_LDO_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("PLL1", RT5514_PWR_ANA2, RT5514_POW_PLL1_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!("ASRC AD1", 1, RT5514_CLK_CTRL2, RT5514_CLK_AD0_ASRC_EN_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY_S!("ASRC AD2", 1, RT5514_CLK_CTRL2, RT5514_CLK_AD1_ASRC_EN_BIT, 0, core::ptr::null(), 0),
    /* ADC Mux */
    SND_SOC_DAPM_MUX!("Stereo1 DMIC Mux", SND_SOC_NOPM, 0, 0, &RT5514_STO1_DMIC_MUX),
    SND_SOC_DAPM_MUX!("Stereo2 DMIC Mux", SND_SOC_NOPM, 0, 0, &RT5514_STO2_DMIC_MUX),
    /* ADC Mixer */
    SND_SOC_DAPM_SUPPLY!("adc stereo1 filter", RT5514_CLK_CTRL1, RT5514_CLK_AD0_EN_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_SUPPLY!("adc stereo2 filter", RT5514_CLK_CTRL1, RT5514_CLK_AD1_EN_BIT, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_MIXER!("Sto1 ADC MIXL", SND_SOC_NOPM, 0, 0, RT5514_STO1_ADC_L_MIX, RT5514_STO1_ADC_L_MIX.len()),
    SND_SOC_DAPM_MIXER!("Sto1 ADC MIXR", SND_SOC_NOPM, 0, 0, RT5514_STO1_ADC_R_MIX, RT5514_STO1_ADC_R_MIX.len()),
    SND_SOC_DAPM_MIXER!("Sto2 ADC MIXL", SND_SOC_NOPM, 0, 0, RT5514_STO2_ADC_L_MIX, RT5514_STO2_ADC_L_MIX.len()),
    SND_SOC_DAPM_MIXER!("Sto2 ADC MIXR", SND_SOC_NOPM, 0, 0, RT5514_STO2_ADC_R_MIX, RT5514_STO2_ADC_R_MIX.len()),
    SND_SOC_DAPM_ADC!("Stereo1 ADC MIXL", core::ptr::null(), RT5514_DOWNFILTER0_CTRL1, RT5514_AD_AD_MUTE_BIT, 1),
    SND_SOC_DAPM_ADC!("Stereo1 ADC MIXR", core::ptr::null(), RT5514_DOWNFILTER0_CTRL2, RT5514_AD_AD_MUTE_BIT, 1),
    SND_SOC_DAPM_ADC!("Stereo2 ADC MIXL", core::ptr::null(), RT5514_DOWNFILTER1_CTRL1, RT5514_AD_AD_MUTE_BIT, 1),
    SND_SOC_DAPM_ADC!("Stereo2 ADC MIXR", core::ptr::null(), RT5514_DOWNFILTER1_CTRL2, RT5514_AD_AD_MUTE_BIT, 1),
    /* ADC PGA */
    SND_SOC_DAPM_PGA!("Stereo1 ADC MIX", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    SND_SOC_DAPM_PGA!("Stereo2 ADC MIX", SND_SOC_NOPM, 0, 0, core::ptr::null(), 0),
    /* Audio Interface */
    SND_SOC_DAPM_AIF_OUT!("AIF1TX", "AIF1 Capture", 0, SND_SOC_NOPM, 0, 0),
];

static RT5514_DAPM_ROUTES: &[snd_soc_dapm_route] = &[
    route!("DMIC1", NULL, "DMIC1L"), route!("DMIC1", NULL, "DMIC1R"),
    route!("DMIC2", NULL, "DMIC2L"), route!("DMIC2", NULL, "DMIC2R"),
    route!("DMIC1L", NULL, "DMIC CLK"), route!("DMIC1R", NULL, "DMIC CLK"),
    route!("DMIC2L", NULL, "DMIC CLK"), route!("DMIC2R", NULL, "DMIC CLK"),
    route!("Stereo1 DMIC Mux", "DMIC1", "DMIC1"),
    route!("Stereo1 DMIC Mux", "DMIC2", "DMIC2"),
    route!("Sto1 ADC MIXL", "DMIC Switch", "Stereo1 DMIC Mux"),
    route!("Sto1 ADC MIXL", "ADC Switch", "AMICL"),
    route!("Sto1 ADC MIXR", "DMIC Switch", "Stereo1 DMIC Mux"),
    route!("Sto1 ADC MIXR", "ADC Switch", "AMICR"),
    route!("ADC Power", NULL, "LDO18 IN"), route!("ADC Power", NULL, "LDO18 ADC"),
    route!("ADC Power", NULL, "LDO21"), route!("ADC Power", NULL, "BG LDO18 IN"),
    route!("ADC Power", NULL, "BG LDO21"), route!("ADC Power", NULL, "BG MBIAS"),
    route!("ADC Power", NULL, "MBIAS"), route!("ADC Power", NULL, "VREF2"),
    route!("ADC Power", NULL, "VREF1"),
    route!("ADCL Power", NULL, "LDO16L"), route!("ADCL Power", NULL, "ADC1L"),
    route!("ADCL Power", NULL, "BSTL2"), route!("ADCL Power", NULL, "BSTL"),
    route!("ADCL Power", NULL, "ADCFEDL"),
    route!("ADCR Power", NULL, "LDO16R"), route!("ADCR Power", NULL, "ADC1R"),
    route!("ADCR Power", NULL, "BSTR2"), route!("ADCR Power", NULL, "BSTR"),
    route!("ADCR Power", NULL, "ADCFEDR"),
    route!("AMICL", NULL, "ADC CLK"), route!("AMICL", NULL, "ADC Power"),
    route!("AMICL", NULL, "ADCL Power"), route!("AMICR", NULL, "ADC CLK"),
    route!("AMICR", NULL, "ADC Power"), route!("AMICR", NULL, "ADCR Power"),
    route!("PLL1 LDO", NULL, "PLL1 LDO ENABLE"), route!("PLL1", NULL, "PLL1 LDO"),
    route!("Stereo1 ADC MIXL", NULL, "Sto1 ADC MIXL"),
    route!("Stereo1 ADC MIXR", NULL, "Sto1 ADC MIXR"),
    route!("Stereo1 ADC MIX", NULL, "Stereo1 ADC MIXL"),
    route!("Stereo1 ADC MIX", NULL, "Stereo1 ADC MIXR"),
    route!("Stereo1 ADC MIX", NULL, "adc stereo1 filter"),
    route_with_check!("adc stereo1 filter", NULL, "PLL1", rt5514_is_sys_clk_from_pll),
    route_with_check!("adc stereo1 filter", NULL, "ASRC AD1", rt5514_i2s_use_asrc),
    route!("Stereo2 DMIC Mux", "DMIC1", "DMIC1"),
    route!("Stereo2 DMIC Mux", "DMIC2", "DMIC2"),
    route!("Sto2 ADC MIXL", "DMIC Switch", "Stereo2 DMIC Mux"),
    route!("Sto2 ADC MIXL", "ADC Switch", "AMICL"),
    route!("Sto2 ADC MIXR", "DMIC Switch", "Stereo2 DMIC Mux"),
    route!("Sto2 ADC MIXR", "ADC Switch", "AMICR"),
    route!("Stereo2 ADC MIXL", NULL, "Sto2 ADC MIXL"),
    route!("Stereo2 ADC MIXR", NULL, "Sto2 ADC MIXR"),
    route!("Stereo2 ADC MIX", NULL, "Stereo2 ADC MIXL"),
    route!("Stereo2 ADC MIX", NULL, "Stereo2 ADC MIXR"),
    route!("Stereo2 ADC MIX", NULL, "adc stereo2 filter"),
    route_with_check!("adc stereo2 filter", NULL, "PLL1", rt5514_is_sys_clk_from_pll),
    route_with_check!("adc stereo2 filter", NULL, "ASRC AD2", rt5514_i2s_use_asrc),
    route!("AIF1TX", NULL, "Stereo1 ADC MIX"),
    route!("AIF1TX", NULL, "Stereo2 ADC MIX"),
];

unsafe fn rt5514_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let rt5514 = snd_soc_component_get_drvdata(component) as *mut rt5514_priv;
    let mut val_len: c_uint = 0;

    (*rt5514).lrck = params_rate(params);
    let pre_div = rl6231_get_clk_info((*rt5514).sysclk, (*rt5514).lrck);
    if pre_div < 0 {
        dev_err((*component).dev, c_str!("Unsupported clock setting\n"));
        return -EINVAL;
    }

    let frame_size = snd_soc_params_to_frame_size(params);
    if frame_size < 0 {
        dev_err((*component).dev, c_str!("Unsupported frame size: %d\n"), frame_size);
        return -EINVAL;
    }

    let bclk_ms = (frame_size > 32) as c_int;
    (*rt5514).bclk = (*rt5514).lrck * (32 << bclk_ms);

    dev_dbg((*dai).dev, c_str!("bclk is %dHz and lrck is %dHz\n"), (*rt5514).bclk, (*rt5514).lrck);
    dev_dbg((*dai).dev, c_str!("bclk_ms is %d and pre_div is %d for iis %d\n"), bclk_ms, pre_div, (*dai).id);

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {}
        SNDRV_PCM_FORMAT_S20_3LE => val_len = RT5514_I2S_DL_20,
        SNDRV_PCM_FORMAT_S24_LE => val_len = RT5514_I2S_DL_24,
        SNDRV_PCM_FORMAT_S8 => val_len = RT5514_I2S_DL_8,
        _ => return -EINVAL,
    }

    regmap_update_bits((*rt5514).regmap, RT5514_I2S_CTRL1, RT5514_I2S_DL_MASK, val_len);
    regmap_update_bits(
        (*rt5514).regmap,
        RT5514_CLK_CTRL1,
        RT5514_CLK_AD_ANA1_SEL_MASK,
        ((pre_div + 1) as c_uint) << RT5514_CLK_AD_ANA1_SEL_SFT,
    );
    regmap_update_bits(
        (*rt5514).regmap,
        RT5514_CLK_CTRL2,
        RT5514_CLK_SYS_DIV_OUT_MASK | RT5514_SEL_ADC_OSR_MASK,
        ((pre_div as c_uint) << RT5514_CLK_SYS_DIV_OUT_SFT)
            | ((pre_div as c_uint) << RT5514_SEL_ADC_OSR_SFT),
    );

    0
}

unsafe fn rt5514_set_dai_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let rt5514 = snd_soc_component_get_drvdata(component) as *mut rt5514_priv;
    let mut reg_val: c_uint = 0;

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => {}
        SND_SOC_DAIFMT_NB_IF => reg_val |= RT5514_I2S_LR_INV,
        SND_SOC_DAIFMT_IB_NF => reg_val |= RT5514_I2S_BP_INV,
        SND_SOC_DAIFMT_IB_IF => reg_val |= RT5514_I2S_BP_INV | RT5514_I2S_LR_INV,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {}
        SND_SOC_DAIFMT_LEFT_J => reg_val |= RT5514_I2S_DF_LEFT,
        SND_SOC_DAIFMT_DSP_A => reg_val |= RT5514_I2S_DF_PCM_A,
        SND_SOC_DAIFMT_DSP_B => reg_val |= RT5514_I2S_DF_PCM_B,
        _ => return -EINVAL,
    }

    regmap_update_bits(
        (*rt5514).regmap,
        RT5514_I2S_CTRL1,
        RT5514_I2S_DF_MASK | RT5514_I2S_BP_MASK | RT5514_I2S_LR_MASK,
        reg_val,
    );

    0
}

unsafe fn rt5514_set_dai_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let component = (*dai).component;
    let rt5514 = snd_soc_component_get_drvdata(component) as *mut rt5514_priv;
    let mut reg_val: c_uint = 0;

    if freq == (*rt5514).sysclk as c_uint && clk_id == (*rt5514).sysclk_src {
        return 0;
    }

    match clk_id {
        RT5514_SCLK_S_MCLK => reg_val |= RT5514_CLK_SYS_PRE_SEL_MCLK,
        RT5514_SCLK_S_PLL1 => reg_val |= RT5514_CLK_SYS_PRE_SEL_PLL,
        _ => {
            dev_err((*component).dev, c_str!("Invalid clock id (%d)\n"), clk_id);
            return -EINVAL;
        }
    }

    regmap_update_bits((*rt5514).regmap, RT5514_CLK_CTRL2, RT5514_CLK_SYS_PRE_SEL_MASK, reg_val);
    (*rt5514).sysclk = freq as _;
    (*rt5514).sysclk_src = clk_id;

    dev_dbg((*dai).dev, c_str!("Sysclk is %dHz and clock id is %d\n"), freq, clk_id);
    0
}

unsafe fn rt5514_set_dai_pll(
    dai: *mut snd_soc_dai,
    _pll_id: c_int,
    source: c_int,
    freq_in: c_uint,
    freq_out: c_uint,
) -> c_int {
    let component = (*dai).component;
    let rt5514 = snd_soc_component_get_drvdata(component) as *mut rt5514_priv;
    let mut pll_code: rl6231_pll_code = core::mem::zeroed();

    if freq_in == 0 || freq_out == 0 {
        dev_dbg((*component).dev, c_str!("PLL disabled\n"));
        (*rt5514).pll_in = 0;
        (*rt5514).pll_out = 0;
        regmap_update_bits(
            (*rt5514).regmap,
            RT5514_CLK_CTRL2,
            RT5514_CLK_SYS_PRE_SEL_MASK,
            RT5514_CLK_SYS_PRE_SEL_MCLK,
        );
        return 0;
    }

    if source == (*rt5514).pll_src && freq_in == (*rt5514).pll_in && freq_out == (*rt5514).pll_out {
        return 0;
    }

    match source {
        RT5514_PLL1_S_MCLK => regmap_update_bits(
            (*rt5514).regmap,
            RT5514_PLL_SOURCE_CTRL,
            RT5514_PLL_1_SEL_MASK,
            RT5514_PLL_1_SEL_MCLK,
        ),
        RT5514_PLL1_S_BCLK => regmap_update_bits(
            (*rt5514).regmap,
            RT5514_PLL_SOURCE_CTRL,
            RT5514_PLL_1_SEL_MASK,
            RT5514_PLL_1_SEL_SCLK,
        ),
        _ => {
            dev_err((*component).dev, c_str!("Unknown PLL source %d\n"), source);
            return -EINVAL;
        }
    };

    let ret = rl6231_pll_calc(freq_in, freq_out, &mut pll_code);
    if ret < 0 {
        dev_err((*component).dev, c_str!("Unsupported input clock %d\n"), freq_in);
        return ret;
    }

    dev_dbg(
        (*component).dev,
        c_str!("bypass=%d m=%d n=%d k=%d\n"),
        pll_code.m_bp,
        if pll_code.m_bp != 0 { 0 } else { pll_code.m_code },
        pll_code.n_code,
        pll_code.k_code,
    );

    regmap_write(
        (*rt5514).regmap,
        RT5514_ANA_CTRL_PLL1_1,
        (pll_code.k_code << RT5514_PLL_K_SFT)
            | (pll_code.n_code << RT5514_PLL_N_SFT)
            | ((if pll_code.m_bp != 0 { 0 } else { pll_code.m_code }) << RT5514_PLL_M_SFT),
    );
    regmap_update_bits(
        (*rt5514).regmap,
        RT5514_ANA_CTRL_PLL1_2,
        RT5514_PLL_M_BP,
        pll_code.m_bp << RT5514_PLL_M_BP_SFT,
    );

    (*rt5514).pll_in = freq_in;
    (*rt5514).pll_out = freq_out;
    (*rt5514).pll_src = source;
    0
}

unsafe fn rt5514_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: c_uint,
    rx_mask: c_uint,
    slots: c_int,
    slot_width: c_int,
) -> c_int {
    let component = (*dai).component;
    let rt5514 = snd_soc_component_get_drvdata(component) as *mut rt5514_priv;
    let mut val: c_uint = 0;
    let mut val2: c_uint = 0;

    if rx_mask != 0 || tx_mask != 0 {
        val |= RT5514_TDM_MODE;
    }

    match tx_mask {
        0x3 => val2 |= RT5514_TDM_DOCKING_MODE | RT5514_TDM_DOCKING_VALID_CH2 | RT5514_TDM_DOCKING_START_SLOT0,
        0x30 => val2 |= RT5514_TDM_DOCKING_MODE | RT5514_TDM_DOCKING_VALID_CH2 | RT5514_TDM_DOCKING_START_SLOT4,
        0xf => val2 |= RT5514_TDM_DOCKING_MODE | RT5514_TDM_DOCKING_VALID_CH4 | RT5514_TDM_DOCKING_START_SLOT0,
        0xf0 => val2 |= RT5514_TDM_DOCKING_MODE | RT5514_TDM_DOCKING_VALID_CH4 | RT5514_TDM_DOCKING_START_SLOT4,
        _ => {}
    }

    match slots {
        4 => val |= RT5514_TDMSLOT_SEL_RX_4CH | RT5514_TDMSLOT_SEL_TX_4CH,
        6 => val |= RT5514_TDMSLOT_SEL_RX_6CH | RT5514_TDMSLOT_SEL_TX_6CH,
        8 => val |= RT5514_TDMSLOT_SEL_RX_8CH | RT5514_TDMSLOT_SEL_TX_8CH,
        2 | _ => {}
    }

    match slot_width {
        20 => val |= RT5514_CH_LEN_RX_20 | RT5514_CH_LEN_TX_20,
        24 => val |= RT5514_CH_LEN_RX_24 | RT5514_CH_LEN_TX_24,
        25 => val |= RT5514_TDM_MODE2,
        32 => val |= RT5514_CH_LEN_RX_32 | RT5514_CH_LEN_TX_32,
        16 | _ => {}
    }

    regmap_update_bits(
        (*rt5514).regmap,
        RT5514_I2S_CTRL1,
        RT5514_TDM_MODE
            | RT5514_TDMSLOT_SEL_RX_MASK
            | RT5514_TDMSLOT_SEL_TX_MASK
            | RT5514_CH_LEN_RX_MASK
            | RT5514_CH_LEN_TX_MASK
            | RT5514_TDM_MODE2,
        val,
    );

    regmap_update_bits(
        (*rt5514).regmap,
        RT5514_I2S_CTRL2,
        RT5514_TDM_DOCKING_MODE | RT5514_TDM_DOCKING_VALID_CH_MASK | RT5514_TDM_DOCKING_START_MASK,
        val2,
    );

    0
}

unsafe fn rt5514_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let rt5514 = snd_soc_component_get_drvdata(component) as *mut rt5514_priv;
    let dapm = snd_soc_component_to_dapm(component);
    let mut ret: c_int;

    match level {
        SND_SOC_BIAS_PREPARE => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_ON {
                clk_disable_unprepare((*rt5514).mclk);
            } else {
                ret = clk_prepare_enable((*rt5514).mclk);
                if ret != 0 {
                    return ret;
                }
            }
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                /*
                 * If the DSP is enabled in start of recording, the DSP
                 * should be disabled, and sync back to normal recording
                 * settings to make sure recording properly.
                 */
                if (*rt5514).dsp_enabled != 0 {
                    ret = regmap_multi_reg_write(
                        (*rt5514).i2c_regmap,
                        RT5514_I2C_PATCH.as_ptr(),
                        RT5514_I2C_PATCH.len(),
                    );
                    if ret != 0 {
                        return ret;
                    }

                    regcache_mark_dirty((*rt5514).regmap);
                    ret = regcache_sync((*rt5514).regmap);
                    if ret != 0 {
                        return ret;
                    }

                    (*rt5514).dsp_enabled = 0;
                }
            }
        }
        _ => {}
    }

    0
}

unsafe fn rt5514_probe(component: *mut snd_soc_component) -> c_int {
    let rt5514 = snd_soc_component_get_drvdata(component) as *mut rt5514_priv;
    let pdev = to_platform_device((*component).dev);

    (*rt5514).mclk = devm_clk_get_optional((*component).dev, c_str!("mclk"));
    if IS_ERR((*rt5514).mclk) {
        return PTR_ERR((*rt5514).mclk) as c_int;
    }

    if !(*rt5514).pdata.dsp_calib_clk_name.is_null() {
        (*rt5514).dsp_calib_clk = devm_clk_get(&mut (*pdev).dev, (*rt5514).pdata.dsp_calib_clk_name);
        if PTR_ERR((*rt5514).dsp_calib_clk) == -EPROBE_DEFER as isize {
            return -EPROBE_DEFER;
        }
    }

    (*rt5514).component = component;
    (*rt5514).pll3_cal_value = 0x0078b000;
    0
}

unsafe fn rt5514_i2c_read(context: *mut c_void, reg: c_uint, val: *mut c_uint) -> c_int {
    let client = context as *mut i2c_client;
    let rt5514 = i2c_get_clientdata(client) as *mut rt5514_priv;

    regmap_read((*rt5514).i2c_regmap, reg | RT5514_DSP_MAPPING, val);
    0
}

unsafe fn rt5514_i2c_write(context: *mut c_void, reg: c_uint, val: c_uint) -> c_int {
    let client = context as *mut i2c_client;
    let rt5514 = i2c_get_clientdata(client) as *mut rt5514_priv;

    regmap_write((*rt5514).i2c_regmap, reg | RT5514_DSP_MAPPING, val);
    0
}

const RT5514_STEREO_RATES: c_uint = SNDRV_PCM_RATE_8000_192000;
const RT5514_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S8;

static RT5514_AIF_DAI_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(rt5514_hw_params),
    set_fmt: Some(rt5514_set_dai_fmt),
    set_sysclk: Some(rt5514_set_dai_sysclk),
    set_pll: Some(rt5514_set_dai_pll),
    set_tdm_slot: Some(rt5514_set_tdm_slot),
    ..unsafe { core::mem::zeroed() }
};

static mut RT5514_DAI: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c_str!("rt5514-aif1"),
    id: 0,
    capture: snd_soc_pcm_stream {
        stream_name: c_str!("AIF1 Capture"),
        channels_min: 1,
        channels_max: 4,
        rates: RT5514_STEREO_RATES,
        formats: RT5514_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &RT5514_AIF_DAI_OPS,
    ..unsafe { core::mem::zeroed() }
}];

static SOC_COMPONENT_DEV_RT5514: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(rt5514_probe),
    set_bias_level: Some(rt5514_set_bias_level),
    controls: RT5514_SND_CONTROLS.as_ptr(),
    num_controls: RT5514_SND_CONTROLS.len(),
    dapm_widgets: RT5514_DAPM_WIDGETS.as_ptr(),
    num_dapm_widgets: RT5514_DAPM_WIDGETS.len(),
    dapm_routes: RT5514_DAPM_ROUTES.as_ptr(),
    num_dapm_routes: RT5514_DAPM_ROUTES.len(),
    use_pmdown_time: 1,
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

static RT5514_I2C_REGMAP: regmap_config = regmap_config {
    name: c_str!("i2c"),
    reg_bits: 32,
    val_bits: 32,
    readable_reg: Some(rt5514_i2c_readable_register),
    cache_type: REGCACHE_NONE,
    ..unsafe { core::mem::zeroed() }
};

static RT5514_REGMAP: regmap_config = regmap_config {
    reg_bits: 16,
    val_bits: 32,
    max_register: RT5514_VENDOR_ID2,
    volatile_reg: Some(rt5514_volatile_register),
    readable_reg: Some(rt5514_readable_register),
    reg_read: Some(rt5514_i2c_read),
    reg_write: Some(rt5514_i2c_write),
    cache_type: REGCACHE_MAPLE,
    reg_defaults: RT5514_REG.as_ptr(),
    num_reg_defaults: RT5514_REG.len(),
    use_single_read: true,
    use_single_write: true,
    ..unsafe { core::mem::zeroed() }
};

static RT5514_I2C_ID: &[i2c_device_id] = &[
    i2c_device_id { name: c_str!("rt5514"), ..unsafe { core::mem::zeroed() } },
    i2c_device_id { ..unsafe { core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(i2c, RT5514_I2C_ID);

/* CONFIG_OF */
static RT5514_OF_MATCH: &[of_device_id] = &[
    of_device_id { compatible: c_str!("realtek,rt5514"), ..unsafe { core::mem::zeroed() } },
    of_device_id { ..unsafe { core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(of, RT5514_OF_MATCH);

/* CONFIG_ACPI */
static RT5514_ACPI_MATCH: &[acpi_device_id] = &[
    acpi_device_id { id: c_str!("10EC5514"), ..unsafe { core::mem::zeroed() } },
    acpi_device_id { ..unsafe { core::mem::zeroed() } },
];
MODULE_DEVICE_TABLE!(acpi, RT5514_ACPI_MATCH);

unsafe fn rt5514_parse_dp(rt5514: *mut rt5514_priv, dev: *mut device) -> c_int {
    device_property_read_u32(
        dev,
        c_str!("realtek,dmic-init-delay-ms"),
        &mut (*rt5514).pdata.dmic_init_delay,
    );
    device_property_read_string(
        dev,
        c_str!("realtek,dsp-calib-clk-name"),
        &mut (*rt5514).pdata.dsp_calib_clk_name,
    );
    device_property_read_u32(
        dev,
        c_str!("realtek,dsp-calib-clk-rate"),
        &mut (*rt5514).pdata.dsp_calib_clk_rate,
    );

    0
}

unsafe fn rt5514_i2c_resume(dev: *mut device) -> c_int {
    let rt5514 = dev_get_drvdata(dev) as *mut rt5514_priv;
    let mut val: c_uint = 0;

    /*
     * Add a bogus read to avoid rt5514's confusion after s2r in case it
     * saw glitches on the i2c lines and thought the other side sent a
     * start bit.
     */
    regmap_read((*rt5514).regmap, RT5514_VENDOR_ID2, &mut val);

    0
}

unsafe fn rt5514_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let pdata = dev_get_platdata(&mut (*i2c).dev) as *mut rt5514_platform_data;
    let mut ret: c_int;
    let mut val: c_uint = !0;

    let rt5514 = devm_kzalloc(
        &mut (*i2c).dev,
        core::mem::size_of::<rt5514_priv>(),
        GFP_KERNEL,
    ) as *mut rt5514_priv;
    if rt5514.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, rt5514 as *mut c_void);

    if !pdata.is_null() {
        (*rt5514).pdata = *pdata;
    } else {
        rt5514_parse_dp(rt5514, &mut (*i2c).dev);
    }

    (*rt5514).i2c_regmap = devm_regmap_init_i2c(i2c, &RT5514_I2C_REGMAP);
    if IS_ERR((*rt5514).i2c_regmap) {
        ret = PTR_ERR((*rt5514).i2c_regmap) as c_int;
        dev_err(&mut (*i2c).dev, c_str!("Failed to allocate register map: %d\n"), ret);
        return ret;
    }

    (*rt5514).regmap = devm_regmap_init(&mut (*i2c).dev, core::ptr::null(), i2c as *mut c_void, &RT5514_REGMAP);
    if IS_ERR((*rt5514).regmap) {
        ret = PTR_ERR((*rt5514).regmap) as c_int;
        dev_err(&mut (*i2c).dev, c_str!("Failed to allocate register map: %d\n"), ret);
        return ret;
    }

    /*
     * The rt5514 can get confused if the i2c lines glitch together, as
     * can happen at bootup as regulators are turned off and on.  If it's
     * in this glitched state the first i2c read will fail, so we'll give
     * it one change to retry.
     */
    ret = regmap_read((*rt5514).regmap, RT5514_VENDOR_ID2, &mut val);
    if ret != 0 || val != RT5514_DEVICE_ID {
        ret = regmap_read((*rt5514).regmap, RT5514_VENDOR_ID2, &mut val);
    }
    if ret != 0 || val != RT5514_DEVICE_ID {
        dev_err(&mut (*i2c).dev, c_str!("Device with ID register %x is not rt5514\n"), val);
        return -ENODEV;
    }

    ret = regmap_multi_reg_write((*rt5514).i2c_regmap, RT5514_I2C_PATCH.as_ptr(), RT5514_I2C_PATCH.len());
    if ret != 0 {
        dev_warn(&mut (*i2c).dev, c_str!("Failed to apply i2c_regmap patch: %d\n"), ret);
    }

    ret = regmap_register_patch((*rt5514).regmap, RT5514_PATCH.as_ptr(), RT5514_PATCH.len());
    if ret != 0 {
        dev_warn(&mut (*i2c).dev, c_str!("Failed to apply regmap patch: %d\n"), ret);
    }

    devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &SOC_COMPONENT_DEV_RT5514,
        RT5514_DAI.as_mut_ptr(),
        RT5514_DAI.len(),
    )
}

static RT5514_I2_PM_OPS: dev_pm_ops = dev_pm_ops {
    /* SYSTEM_SLEEP_PM_OPS(NULL, rt5514_i2c_resume) */
    resume: Some(rt5514_i2c_resume),
    ..unsafe { core::mem::zeroed() }
};

static mut RT5514_I2C_DRIVER: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c_str!("rt5514"),
        acpi_match_table: ACPI_PTR!(RT5514_ACPI_MATCH),
        of_match_table: of_match_ptr!(RT5514_OF_MATCH),
        pm: pm_ptr!(&RT5514_I2_PM_OPS),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(rt5514_i2c_probe),
    id_table: RT5514_I2C_ID.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};
module_i2c_driver!(RT5514_I2C_DRIVER);

MODULE_DESCRIPTION!("ASoC RT5514 driver");
MODULE_AUTHOR!("Oder Chiou <oder_chiou@realtek.com>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
