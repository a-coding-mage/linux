// SPDX-License-Identifier: GPL-2.0
//
// Socionext UniPhier AIO ALSA common driver.
//
// Copyright (c) 2016-2018 Socionext Inc.

// Dependencies: linux/bitfield, linux/kernel, sound/core, sound/pcm, sound/pcm_params, sound/soc
// Local dependencies: aio.h, aio-reg.h

fn rb_cnt(wr: u64, rd: u64, len: u64) -> u64 {
    if rd <= wr {
        wr - rd
    } else {
        len - (rd - wr)
    }
}

fn rb_cnt_to_end(wr: u64, rd: u64, len: u64) -> u64 {
    if rd <= wr {
        wr - rd
    } else {
        len - rd
    }
}

fn rb_space(wr: u64, rd: u64, len: u64) -> u64 {
    if rd <= wr {
        len - (wr - rd) - 8
    } else {
        rd - wr - 8
    }
}

fn rb_space_to_end(wr: u64, rd: u64, len: u64) -> u64 {
    if rd > wr {
        rd - wr - 8
    } else if rd > 0 {
        len - wr
    } else {
        len - wr - 8
    }
}

pub fn aio_rb_cnt(sub: &uniphier_aio_sub) -> u64 {
    rb_cnt(sub.wr_offs, sub.rd_offs, sub.compr_bytes)
}

pub fn aio_rbt_cnt_to_end(sub: &uniphier_aio_sub) -> u64 {
    rb_cnt_to_end(sub.wr_offs, sub.rd_offs, sub.compr_bytes)
}

pub fn aio_rb_space(sub: &uniphier_aio_sub) -> u64 {
    rb_space(sub.wr_offs, sub.rd_offs, sub.compr_bytes)
}

pub fn aio_rb_space_to_end(sub: &uniphier_aio_sub) -> u64 {
    rb_space_to_end(sub.wr_offs, sub.rd_offs, sub.compr_bytes)
}

/// aio_iecout_set_enable - setup IEC output via SoC glue
/// @chip: the AIO chip pointer
/// @enable: false to stop the output, true to start
///
/// Set enabled or disabled S/PDIF signal output to out of SoC via AOnIEC pins.
/// This function need to call at driver startup.
///
/// The regmap of SoC glue is specified by 'socionext,syscon' optional property
/// of DT. This function has no effect if no property.
pub fn aio_iecout_set_enable(chip: &mut uniphier_aio_chip, enable: bool) {
    let r = match &chip.regmap_sg {
        Some(regmap) => regmap,
        None => return,
    };

    regmap_write(r, SG_AOUTEN, if enable { !0 } else { 0 });
}

/// aio_chip_set_pll - set frequency to audio PLL
/// @chip: the AIO chip pointer
/// @pll_id: PLL
/// @freq: frequency in Hz, 0 is ignored
///
/// Sets frequency of audio PLL. This function can be called anytime,
/// but it takes time till PLL is locked.
///
/// Return: Zero if successful, otherwise a negative value on error.
pub fn aio_chip_set_pll(chip: &mut uniphier_aio_chip, pll_id: i32, freq: u32) -> i32 {
    let dev = &chip.pdev.dev;
    let r = &chip.regmap;
    let shift: i32;
    let v: u32;

    if freq == 0 {
        return 0;
    }

    shift = match pll_id {
        AUD_PLL_A1 => 0,
        AUD_PLL_F1 => 1,
        AUD_PLL_A2 => 2,
        AUD_PLL_F2 => 3,
        _ => {
            dev_err(dev, "PLL({}) not supported\n", pll_id);
            return -EINVAL;
        }
    };

    v = match freq {
        36864000 => A2APLLCTR1_APLLX_36MHZ,
        33868800 => A2APLLCTR1_APLLX_33MHZ,
        _ => {
            dev_err(dev, "PLL frequency not supported({})\n", freq);
            return -EINVAL;
        }
    };
    chip.plls[pll_id as usize].freq = freq;

    regmap_update_bits(r, A2APLLCTR1, A2APLLCTR1_APLLX_MASK << shift,
                       v << shift);

    0
}

/// aio_chip_init - initialize AIO whole settings
/// @chip: the AIO chip pointer
///
/// Sets AIO fixed and whole device settings to AIO.
/// This function need to call once at driver startup.
///
/// The register area that is changed by this function is shared by all
/// modules of AIO. But there is not race condition since this function
/// has always set the same initialize values.
pub fn aio_chip_init(chip: &uniphier_aio_chip) {
    let r = &chip.regmap;

    regmap_update_bits(r, A2APLLCTR0,
                       A2APLLCTR0_APLLXPOW_MASK,
                       A2APLLCTR0_APLLXPOW_PWON);

    regmap_update_bits(r, A2EXMCLKSEL0,
                       A2EXMCLKSEL0_EXMCLK_MASK,
                       A2EXMCLKSEL0_EXMCLK_OUTPUT);

    regmap_update_bits(r, A2AIOINPUTSEL, A2AIOINPUTSEL_RXSEL_MASK,
                       A2AIOINPUTSEL_RXSEL_PCMI1_HDMIRX1 |
                       A2AIOINPUTSEL_RXSEL_PCMI2_SIF |
                       A2AIOINPUTSEL_RXSEL_PCMI3_EVEA |
                       A2AIOINPUTSEL_RXSEL_IECI1_HDMIRX1);

    if chip.chip_spec.addr_ext {
        regmap_update_bits(r, CDA2D_TEST, CDA2D_TEST_DDR_MODE_MASK,
                           CDA2D_TEST_DDR_MODE_EXTON0);
    } else {
        regmap_update_bits(r, CDA2D_TEST, CDA2D_TEST_DDR_MODE_MASK,
                           CDA2D_TEST_DDR_MODE_EXTOFF1);
    }
}

/// aio_init - initialize AIO substream
/// @sub: the AIO substream pointer
///
/// Sets fixed settings of each AIO substreams.
/// This function need to call once at substream startup.
///
/// Return: Zero if successful, otherwise a negative value on error.
pub fn aio_init(sub: &uniphier_aio_sub) -> i32 {
    let dev = &sub.aio.chip.pdev.dev;
    let r = &sub.aio.chip.regmap;

    regmap_write(r, A2RBNMAPCTR0(sub.swm.rb.hw),
                 MAPCTR0_EN | sub.swm.rb.map);
    regmap_write(r, A2CHNMAPCTR0(sub.swm.ch.hw),
                 MAPCTR0_EN | sub.swm.ch.map);

    match sub.swm.type_ {
        PORT_TYPE_I2S | PORT_TYPE_SPDIF | PORT_TYPE_EVE => {
            if sub.swm.dir == PORT_DIR_INPUT {
                regmap_write(r, A2IIFNMAPCTR0(sub.swm.iif.hw),
                             MAPCTR0_EN | sub.swm.iif.map);
                regmap_write(r, A2IPORTNMAPCTR0(sub.swm.iport.hw),
                             MAPCTR0_EN | sub.swm.iport.map);
            } else {
                regmap_write(r, A2OIFNMAPCTR0(sub.swm.oif.hw),
                             MAPCTR0_EN | sub.swm.oif.map);
                regmap_write(r, A2OPORTNMAPCTR0(sub.swm.oport.hw),
                             MAPCTR0_EN | sub.swm.oport.map);
            }
        }
        PORT_TYPE_CONV => {
            regmap_write(r, A2OIFNMAPCTR0(sub.swm.oif.hw),
                         MAPCTR0_EN | sub.swm.oif.map);
            regmap_write(r, A2OPORTNMAPCTR0(sub.swm.oport.hw),
                         MAPCTR0_EN | sub.swm.oport.map);
            regmap_write(r, A2CHNMAPCTR0(sub.swm.och.hw),
                         MAPCTR0_EN | sub.swm.och.map);
            regmap_write(r, A2IIFNMAPCTR0(sub.swm.iif.hw),
                         MAPCTR0_EN | sub.swm.iif.map);
        }
        _ => {
            dev_err(dev, "Unknown port type {}.\n", sub.swm.type_);
            return -EINVAL;
        }
    }

    0
}

/// aio_port_reset - reset AIO port block
/// @sub: the AIO substream pointer
///
/// Resets the digital signal input/output port block of AIO.
pub fn aio_port_reset(sub: &uniphier_aio_sub) {
    let r = &sub.aio.chip.regmap;

    if sub.swm.dir == PORT_DIR_OUTPUT {
        regmap_write(r, AOUTRSTCTR0, 1u32 << sub.swm.oport.map);
        regmap_write(r, AOUTRSTCTR1, 1u32 << sub.swm.oport.map);
    } else {
        regmap_update_bits(r, IPORTMXRSTCTR(sub.swm.iport.map),
                           IPORTMXRSTCTR_RSTPI_MASK,
                           IPORTMXRSTCTR_RSTPI_RESET);
        regmap_update_bits(r, IPORTMXRSTCTR(sub.swm.iport.map),
                           IPORTMXRSTCTR_RSTPI_MASK,
                           IPORTMXRSTCTR_RSTPI_RELEASE);
    }
}

/// aio_port_set_ch - set channels of LPCM
/// @sub: the AIO substream pointer, PCM substream only
///
/// Set suitable slot selecting to input/output port block of AIO.
///
/// This function may return error if non-PCM substream.
///
/// Return: Zero if successful, otherwise a negative value on error.
fn aio_port_set_ch(sub: &uniphier_aio_sub) -> i32 {
    let r = &sub.aio.chip.regmap;
    const SLOTSEL_2CH: &[u32] = &[
        0, 0, 0, 0, 0,
    ];
    const SLOTSEL_MULTI: &[u32] = &[
        OPORTMXTYSLOTCTR_SLOTSEL_SLOT0,
        OPORTMXTYSLOTCTR_SLOTSEL_SLOT1,
        OPORTMXTYSLOTCTR_SLOTSEL_SLOT2,
        OPORTMXTYSLOTCTR_SLOTSEL_SLOT3,
        OPORTMXTYSLOTCTR_SLOTSEL_SLOT4,
    ];
    let mode: u32;
    let slotsel: &[u32];

    match params_channels(&sub.params) {
        8 | 6 => {
            mode = OPORTMXTYSLOTCTR_MODE;
            slotsel = SLOTSEL_MULTI;
        }
        2 => {
            mode = 0;
            slotsel = SLOTSEL_2CH;
        }
        _ => {
            return -EINVAL;
        }
    }

    for i in 0..AUD_MAX_SLOTSEL {
        regmap_update_bits(r, OPORTMXTYSLOTCTR(sub.swm.oport.map, i as i32),
                           OPORTMXTYSLOTCTR_MODE, mode);
        regmap_update_bits(r, OPORTMXTYSLOTCTR(sub.swm.oport.map, i as i32),
                           OPORTMXTYSLOTCTR_SLOTSEL_MASK, slotsel[i]);
    }

    0
}

/// aio_port_set_rate - set sampling rate of LPCM
/// @sub: the AIO substream pointer, PCM substream only
/// @rate: Sampling rate in Hz.
///
/// Set suitable I2S format settings to input/output port block of AIO.
/// Parameter is specified by hw_params().
///
/// This function may return error if non-PCM substream.
///
/// Return: Zero if successful, otherwise a negative value on error.
fn aio_port_set_rate(sub: &uniphier_aio_sub, rate: i32) -> i32 {
    let r = &sub.aio.chip.regmap;
    let dev = &sub.aio.chip.pdev.dev;
    let v: u32;

    if sub.swm.dir == PORT_DIR_OUTPUT {
        v = match rate {
            8000 => OPORTMXCTR1_FSSEL_8,
            11025 => OPORTMXCTR1_FSSEL_11_025,
            12000 => OPORTMXCTR1_FSSEL_12,
            16000 => OPORTMXCTR1_FSSEL_16,
            22050 => OPORTMXCTR1_FSSEL_22_05,
            24000 => OPORTMXCTR1_FSSEL_24,
            32000 => OPORTMXCTR1_FSSEL_32,
            44100 => OPORTMXCTR1_FSSEL_44_1,
            48000 => OPORTMXCTR1_FSSEL_48,
            88200 => OPORTMXCTR1_FSSEL_88_2,
            96000 => OPORTMXCTR1_FSSEL_96,
            176400 => OPORTMXCTR1_FSSEL_176_4,
            192000 => OPORTMXCTR1_FSSEL_192,
            _ => {
                dev_err(dev, "Rate not supported({})\n", rate);
                return -EINVAL;
            }
        };

        regmap_update_bits(r, OPORTMXCTR1(sub.swm.oport.map),
                           OPORTMXCTR1_FSSEL_MASK, v);
    } else {
        v = match rate {
            8000 => IPORTMXCTR1_FSSEL_8,
            11025 => IPORTMXCTR1_FSSEL_11_025,
            12000 => IPORTMXCTR1_FSSEL_12,
            16000 => IPORTMXCTR1_FSSEL_16,
            22050 => IPORTMXCTR1_FSSEL_22_05,
            24000 => IPORTMXCTR1_FSSEL_24,
            32000 => IPORTMXCTR1_FSSEL_32,
            44100 => IPORTMXCTR1_FSSEL_44_1,
            48000 => IPORTMXCTR1_FSSEL_48,
            88200 => IPORTMXCTR1_FSSEL_88_2,
            96000 => IPORTMXCTR1_FSSEL_96,
            176400 => IPORTMXCTR1_FSSEL_176_4,
            192000 => IPORTMXCTR1_FSSEL_192,
            _ => {
                dev_err(dev, "Rate not supported({})\n", rate);
                return -EINVAL;
            }
        };

        regmap_update_bits(r, IPORTMXCTR1(sub.swm.iport.map),
                           IPORTMXCTR1_FSSEL_MASK, v);
    }

    0
}

/// aio_port_set_fmt - set format of I2S data
/// @sub: the AIO substream pointer, PCM substream only
/// This parameter has no effect if substream is I2S or PCM.
///
/// Set suitable I2S format settings to input/output port block of AIO.
/// Parameter is specified by set_fmt().
///
/// This function may return error if non-PCM substream.
///
/// Return: Zero if successful, otherwise a negative value on error.
fn aio_port_set_fmt(sub: &uniphier_aio_sub) -> i32 {
    let r = &sub.aio.chip.regmap;
    let dev = &sub.aio.chip.pdev.dev;
    let mut v: u32;

    if sub.swm.dir == PORT_DIR_OUTPUT {
        v = match sub.aio.fmt {
            SND_SOC_DAIFMT_LEFT_J => OPORTMXCTR1_I2SLRSEL_LEFT,
            SND_SOC_DAIFMT_RIGHT_J => OPORTMXCTR1_I2SLRSEL_RIGHT,
            SND_SOC_DAIFMT_I2S => OPORTMXCTR1_I2SLRSEL_I2S,
            _ => {
                dev_err(dev, "Format is not supported({})\n",
                        sub.aio.fmt);
                return -EINVAL;
            }
        };

        v |= OPORTMXCTR1_OUTBITSEL_24;
        regmap_update_bits(r, OPORTMXCTR1(sub.swm.oport.map),
                           OPORTMXCTR1_I2SLRSEL_MASK |
                           OPORTMXCTR1_OUTBITSEL_MASK, v);
    } else {
        v = match sub.aio.fmt {
            SND_SOC_DAIFMT_LEFT_J => IPORTMXCTR1_LRSEL_LEFT,
            SND_SOC_DAIFMT_RIGHT_J => IPORTMXCTR1_LRSEL_RIGHT,
            SND_SOC_DAIFMT_I2S => IPORTMXCTR1_LRSEL_I2S,
            _ => {
                dev_err(dev, "Format is not supported({})\n",
                        sub.aio.fmt);
                return -EINVAL;
            }
        };

        v |= IPORTMXCTR1_OUTBITSEL_24 |
            IPORTMXCTR1_CHSEL_ALL;
        regmap_update_bits(r, IPORTMXCTR1(sub.swm.iport.map),
                           IPORTMXCTR1_LRSEL_MASK |
                           IPORTMXCTR1_OUTBITSEL_MASK |
                           IPORTMXCTR1_CHSEL_MASK, v);
    }

    0
}

/// aio_port_set_clk - set clock and divider of AIO port block
/// @sub: the AIO substream pointer
///
/// Set suitable PLL clock divider and relational settings to
/// input/output port block of AIO. Parameters are specified by
/// set_sysclk() and set_pll().
///
/// Return: Zero if successful, otherwise a negative value on error.
fn aio_port_set_clk(sub: &uniphier_aio_sub) -> i32 {
    let chip = &sub.aio.chip;
    let dev = &sub.aio.chip.pdev.dev;
    let r = &sub.aio.chip.regmap;
    const V_PLL: &[u32] = &[
        OPORTMXCTR2_ACLKSEL_A1, OPORTMXCTR2_ACLKSEL_F1,
        OPORTMXCTR2_ACLKSEL_A2, OPORTMXCTR2_ACLKSEL_F2,
        OPORTMXCTR2_ACLKSEL_A2PLL,
        OPORTMXCTR2_ACLKSEL_RX1,
    ];
    const V_DIV: &[u32] = &[
        OPORTMXCTR2_DACCKSEL_1_2, OPORTMXCTR2_DACCKSEL_1_3,
        OPORTMXCTR2_DACCKSEL_1_1, OPORTMXCTR2_DACCKSEL_2_3,
    ];
    let v: u32;

    if sub.swm.dir == PORT_DIR_OUTPUT {
        if sub.swm.type_ == PORT_TYPE_I2S {
            if sub.aio.pll_out >= V_PLL.len() {
                dev_err(dev, "PLL({}) is invalid\n",
                        sub.aio.pll_out);
                return -EINVAL;
            }
            if sub.aio.plldiv >= V_DIV.len() {
                dev_err(dev, "PLL divider({}) is invalid\n",
                        sub.aio.plldiv);
                return -EINVAL;
            }

            v = V_PLL[sub.aio.pll_out] |
                OPORTMXCTR2_MSSEL_MASTER |
                V_DIV[sub.aio.plldiv];

            v |= match chip.plls[sub.aio.pll_out].freq {
                0 | 36864000 | 33868800 => OPORTMXCTR2_EXTLSIFSSEL_36,
                _ => OPORTMXCTR2_EXTLSIFSSEL_24,
            };
        } else if sub.swm.type_ == PORT_TYPE_EVE {
            v = OPORTMXCTR2_ACLKSEL_A2PLL |
                OPORTMXCTR2_MSSEL_MASTER |
                OPORTMXCTR2_EXTLSIFSSEL_36 |
                OPORTMXCTR2_DACCKSEL_1_2;
        } else if sub.swm.type_ == PORT_TYPE_SPDIF {
            if sub.aio.pll_out >= V_PLL.len() {
                dev_err(dev, "PLL({}) is invalid\n",
                        sub.aio.pll_out);
                return -EINVAL;
            }
            v = V_PLL[sub.aio.pll_out] |
                OPORTMXCTR2_MSSEL_MASTER |
                OPORTMXCTR2_DACCKSEL_1_2;

            v |= match chip.plls[sub.aio.pll_out].freq {
                0 | 36864000 | 33868800 => OPORTMXCTR2_EXTLSIFSSEL_36,
                _ => OPORTMXCTR2_EXTLSIFSSEL_24,
            };
        } else {
            v = OPORTMXCTR2_ACLKSEL_A1 |
                OPORTMXCTR2_MSSEL_MASTER |
                OPORTMXCTR2_EXTLSIFSSEL_36 |
                OPORTMXCTR2_DACCKSEL_1_2;
        }
        regmap_write(r, OPORTMXCTR2(sub.swm.oport.map), v);
    } else {
        v = IPORTMXCTR2_ACLKSEL_A1 |
            IPORTMXCTR2_MSSEL_SLAVE |
            IPORTMXCTR2_EXTLSIFSSEL_36 |
            IPORTMXCTR2_DACCKSEL_1_2;
        regmap_write(r, IPORTMXCTR2(sub.swm.iport.map), v);
    }

    0
}

/// aio_port_set_param - set parameters of AIO port block
/// @sub: the AIO substream pointer
/// @pass_through: Zero if sound data is LPCM, otherwise if data is not LPCM.
/// This parameter has no effect if substream is I2S or PCM.
/// @params: hardware parameters of ALSA
///
/// Set suitable setting to input/output port block of AIO to process the
/// specified in params.
///
/// Return: Zero if successful, otherwise a negative value on error.
pub fn aio_port_set_param(sub: &uniphier_aio_sub, pass_through: i32,
                          params: &snd_pcm_hw_params) -> i32 {
    let r = &sub.aio.chip.regmap;
    let rate: u32;
    let v: u32;
    let mut ret: i32;

    if pass_through == 0 {
        if sub.swm.type_ == PORT_TYPE_EVE ||
            sub.swm.type_ == PORT_TYPE_CONV {
            rate = 48000;
        } else {
            rate = params_rate(params);
        }

        ret = aio_port_set_ch(sub);
        if ret != 0 {
            return ret;
        }

        ret = aio_port_set_rate(sub, rate as i32);
        if ret != 0 {
            return ret;
        }

        ret = aio_port_set_fmt(sub);
        if ret != 0 {
            return ret;
        }
    }

    ret = aio_port_set_clk(sub);
    if ret != 0 {
        return ret;
    }

    if sub.swm.dir == PORT_DIR_OUTPUT {
        let v = if pass_through != 0 {
            OPORTMXCTR3_SRCSEL_STREAM |
            OPORTMXCTR3_VALID_STREAM
        } else {
            OPORTMXCTR3_SRCSEL_PCM |
            OPORTMXCTR3_VALID_PCM
        };

        let v = v | OPORTMXCTR3_IECTHUR_IECOUT |
            OPORTMXCTR3_PMSEL_PAUSE |
            OPORTMXCTR3_PMSW_MUTE_OFF;
        regmap_write(r, OPORTMXCTR3(sub.swm.oport.map), v);
    } else {
        regmap_write(r, IPORTMXACLKSEL0EX(sub.swm.iport.map),
                     IPORTMXACLKSEL0EX_ACLKSEL0EX_INTERNAL);
        regmap_write(r, IPORTMXEXNOE(sub.swm.iport.map),
                     IPORTMXEXNOE_PCMINOE_INPUT);
    }

    0
}

/// aio_port_set_enable - start or stop of AIO port block
/// @sub: the AIO substream pointer
/// @enable: zero to stop the block, otherwise to start
///
/// Start or stop the signal input/output port block of AIO.
pub fn aio_port_set_enable(sub: &uniphier_aio_sub, enable: i32) {
    let r = &sub.aio.chip.regmap;

    if sub.swm.dir == PORT_DIR_OUTPUT {
        regmap_write(r, OPORTMXPATH(sub.swm.oport.map),
                     sub.swm.oif.map);

        regmap_update_bits(r, OPORTMXMASK(sub.swm.oport.map),
                           OPORTMXMASK_IUDXMSK_MASK |
                           OPORTMXMASK_IUXCKMSK_MASK |
                           OPORTMXMASK_DXMSK_MASK |
                           OPORTMXMASK_XCKMSK_MASK,
                           OPORTMXMASK_IUDXMSK_OFF |
                           OPORTMXMASK_IUXCKMSK_OFF |
                           OPORTMXMASK_DXMSK_OFF |
                           OPORTMXMASK_XCKMSK_OFF);

        if enable != 0 {
            regmap_write(r, AOUTENCTR0, 1u32 << sub.swm.oport.map);
        } else {
            regmap_write(r, AOUTENCTR1, 1u32 << sub.swm.oport.map);
        }
    } else {
        regmap_update_bits(r, IPORTMXMASK(sub.swm.iport.map),
                           IPORTMXMASK_IUXCKMSK_MASK |
                           IPORTMXMASK_XCKMSK_MASK,
                           IPORTMXMASK_IUXCKMSK_OFF |
                           IPORTMXMASK_XCKMSK_OFF);

        if enable != 0 {
            regmap_update_bits(r,
                               IPORTMXCTR2(sub.swm.iport.map),
                               IPORTMXCTR2_REQEN_MASK,
                               IPORTMXCTR2_REQEN_ENABLE);
        } else {
            regmap_update_bits(r,
                               IPORTMXCTR2(sub.swm.iport.map),
                               IPORTMXCTR2_REQEN_MASK,
                               IPORTMXCTR2_REQEN_DISABLE);
        }
    }
}

/// aio_port_get_volume - get volume of AIO port block
/// @sub: the AIO substream pointer
///
/// Return: current volume, range is 0x0000 - 0xffff
pub fn aio_port_get_volume(sub: &uniphier_aio_sub) -> i32 {
    let r = &sub.aio.chip.regmap;
    let mut v: u32 = 0;

    regmap_read(r, OPORTMXTYVOLGAINSTATUS(sub.swm.oport.map, 0), &mut v);

    FIELD_GET(OPORTMXTYVOLGAINSTATUS_CUR_MASK, v) as i32
}

/// aio_port_set_volume - set volume of AIO port block
/// @sub: the AIO substream pointer
/// @vol: target volume, range is 0x0000 - 0xffff.
///
/// Change digital volume and perfome fade-out/fade-in effect for specified
/// output slot of port. Gained PCM value can calculate as the following:
///   Gained = Original * vol / 0x4000
pub fn aio_port_set_volume(sub: &uniphier_aio_sub, vol: i32) {
    let r = &sub.aio.chip.regmap;
    let oport_map = sub.swm.oport.map;
    let cur: i32;
    let diff: i32;
    let mut slope: i32 = 0;
    let fs: u32;

    if sub.swm.dir == PORT_DIR_INPUT {
        return;
    }

    cur = aio_port_get_volume(sub);
    diff = (vol - cur).abs();
    fs = params_rate(&sub.params);
    if fs != 0 {
        slope = (diff / (AUD_VOL_FADE_TIME as i32) * 1000 / (fs as i32));
    }
    slope = core::cmp::max(1, slope);

    regmap_update_bits(r, OPORTMXTYVOLPARA1(oport_map, 0),
                       OPORTMXTYVOLPARA1_SLOPEU_MASK, (slope as u32) << 16);
    regmap_update_bits(r, OPORTMXTYVOLPARA2(oport_map, 0),
                       OPORTMXTYVOLPARA2_TARGET_MASK, vol as u32);

    if cur < vol {
        regmap_update_bits(r, OPORTMXTYVOLPARA2(oport_map, 0),
                           OPORTMXTYVOLPARA2_FADE_MASK,
                           OPORTMXTYVOLPARA2_FADE_FADEIN);
    } else {
        regmap_update_bits(r, OPORTMXTYVOLPARA2(oport_map, 0),
                           OPORTMXTYVOLPARA2_FADE_MASK,
                           OPORTMXTYVOLPARA2_FADE_FADEOUT);
    }

    regmap_write(r, AOUTFADECTR0, 1u32 << oport_map);
}

/// aio_if_set_param - set parameters of AIO DMA I/F block
/// @sub: the AIO substream pointer
/// @pass_through: Zero if sound data is LPCM, otherwise if data is not LPCM.
/// This parameter has no effect if substream is I2S or PCM.
///
/// Set suitable setting to DMA interface block of AIO to process the
/// specified in settings.
///
/// Return: Zero if successful, otherwise a negative value on error.
pub fn aio_if_set_param(sub: &uniphier_aio_sub, pass_through: i32) -> i32 {
    let r = &sub.aio.chip.regmap;
    let memfmt: u32;
    let v: u32;

    if sub.swm.dir == PORT_DIR_OUTPUT {
        let v = if pass_through != 0 {
            PBOUTMXCTR0_ENDIAN_0123 |
            PBOUTMXCTR0_MEMFMT_STREAM
        } else {
            memfmt = match params_channels(&sub.params) {
                2 => PBOUTMXCTR0_MEMFMT_2CH,
                6 => PBOUTMXCTR0_MEMFMT_6CH,
                8 => PBOUTMXCTR0_MEMFMT_8CH,
                _ => return -EINVAL,
            };
            PBOUTMXCTR0_ENDIAN_3210 | memfmt
        };

        regmap_write(r, PBOUTMXCTR0(sub.swm.oif.map), v);
        regmap_write(r, PBOUTMXCTR1(sub.swm.oif.map), 0);
    } else {
        regmap_write(r, PBINMXCTR(sub.swm.iif.map),
                     PBINMXCTR_NCONNECT_CONNECT |
                     PBINMXCTR_INOUTSEL_IN |
                     ((sub.swm.iport.map as u32) << PBINMXCTR_PBINSEL_SHIFT) |
                     PBINMXCTR_ENDIAN_3210 |
                     PBINMXCTR_MEMFMT_D0);
    }

    0
}

/// aio_oport_set_stream_type - set parameters of AIO playback port block
/// @sub: the AIO substream pointer
/// @pc: Pc type of IEC61937
///
/// Set special setting to output port block of AIO to output the stream
/// via S/PDIF.
///
/// Return: Zero if successful, otherwise a negative value on error.
pub fn aio_oport_set_stream_type(sub: &uniphier_aio_sub,
                                 pc: IEC61937_PC) -> i32 {
    let r = &sub.aio.chip.regmap;
    let repet: u32 = 0;
    let pause: u32 = OPORTMXPAUDAT_PAUSEPC_CMN;
    let mut repet = repet;
    let mut pause = pause;

    match pc {
        IEC61937_PC_AC3 => {
            repet = OPORTMXREPET_STRLENGTH_AC3 |
                OPORTMXREPET_PMLENGTH_AC3;
            pause |= OPORTMXPAUDAT_PAUSEPD_AC3;
        }
        IEC61937_PC_MPA => {
            repet = OPORTMXREPET_STRLENGTH_MPA |
                OPORTMXREPET_PMLENGTH_MPA;
            pause |= OPORTMXPAUDAT_PAUSEPD_MPA;
        }
        IEC61937_PC_MP3 => {
            repet = OPORTMXREPET_STRLENGTH_MP3 |
                OPORTMXREPET_PMLENGTH_MP3;
            pause |= OPORTMXPAUDAT_PAUSEPD_MP3;
        }
        IEC61937_PC_DTS1 => {
            repet = OPORTMXREPET_STRLENGTH_DTS1 |
                OPORTMXREPET_PMLENGTH_DTS1;
            pause |= OPORTMXPAUDAT_PAUSEPD_DTS1;
        }
        IEC61937_PC_DTS2 => {
            repet = OPORTMXREPET_STRLENGTH_DTS2 |
                OPORTMXREPET_PMLENGTH_DTS2;
            pause |= OPORTMXPAUDAT_PAUSEPD_DTS2;
        }
        IEC61937_PC_DTS3 => {
            repet = OPORTMXREPET_STRLENGTH_DTS3 |
                OPORTMXREPET_PMLENGTH_DTS3;
            pause |= OPORTMXPAUDAT_PAUSEPD_DTS3;
        }
        IEC61937_PC_AAC => {
            repet = OPORTMXREPET_STRLENGTH_AAC |
                OPORTMXREPET_PMLENGTH_AAC;
            pause |= OPORTMXPAUDAT_PAUSEPD_AAC;
        }
        IEC61937_PC_PAUSE => {
        }
    }

    let ret = regmap_write(r, OPORTMXREPET(sub.swm.oport.map), repet);
    if ret != 0 {
        return ret;
    }

    let ret = regmap_write(r, OPORTMXPAUDAT(sub.swm.oport.map), pause);
    if ret != 0 {
        return ret;
    }

    0
}

/// aio_src_reset - reset AIO SRC block
/// @sub: the AIO substream pointer
///
/// Resets the digital signal input/output port with sampling rate converter
/// block of AIO.
/// This function has no effect if substream is not supported rate converter.
pub fn aio_src_reset(sub: &uniphier_aio_sub) {
    let r = &sub.aio.chip.regmap;

    if sub.swm.dir != PORT_DIR_OUTPUT {
        return;
    }

    regmap_write(r, AOUTSRCRSTCTR0, 1u32 << sub.swm.oport.map);
    regmap_write(r, AOUTSRCRSTCTR1, 1u32 << sub.swm.oport.map);
}

/// aio_src_set_param - set parameters of AIO SRC block
/// @sub: the AIO substream pointer
/// @params: hardware parameters of ALSA
///
/// Set suitable setting to input/output port with sampling rate converter
/// block of AIO to process the specified in params.
/// This function has no effect if substream is not supported rate converter.
///
/// Return: Zero if successful, otherwise a negative value on error.
pub fn aio_src_set_param(sub: &uniphier_aio_sub,
                         params: &snd_pcm_hw_params) -> i32 {
    let r = &sub.aio.chip.regmap;
    let v: u32;
    let mut ret: i32;

    if sub.swm.dir != PORT_DIR_OUTPUT {
        return 0;
    }

    ret = regmap_write(r, OPORTMXSRC1CTR(sub.swm.oport.map),
                       OPORTMXSRC1CTR_THMODE_SRC |
                       OPORTMXSRC1CTR_SRCPATH_CALC |
                       OPORTMXSRC1CTR_SYNC_ASYNC |
                       OPORTMXSRC1CTR_FSIIPSEL_INNER |
                       OPORTMXSRC1CTR_FSISEL_ACLK);
    if ret != 0 {
        return ret;
    }

    let v = match params_rate(params) {
        44100 => {
            OPORTMXRATE_I_ACLKSEL_APLLA2 |
            OPORTMXRATE_I_MCKSEL_33 |
            OPORTMXRATE_I_FSSEL_44_1
        }
        32000 => {
            OPORTMXRATE_I_ACLKSEL_APLLA1 |
            OPORTMXRATE_I_MCKSEL_36 |
            OPORTMXRATE_I_FSSEL_32
        }
        _ => {
            OPORTMXRATE_I_ACLKSEL_APLLA1 |
            OPORTMXRATE_I_MCKSEL_36 |
            OPORTMXRATE_I_FSSEL_48
        }
    };

    ret = regmap_write(r, OPORTMXRATE_I(sub.swm.oport.map),
                       v | OPORTMXRATE_I_ACLKSRC_APLL |
                       OPORTMXRATE_I_LRCKSTP_STOP);
    if ret != 0 {
        return ret;
    }

    ret = regmap_update_bits(r, OPORTMXRATE_I(sub.swm.oport.map),
                             OPORTMXRATE_I_LRCKSTP_MASK,
                             OPORTMXRATE_I_LRCKSTP_START);
    if ret != 0 {
        return ret;
    }

    0
}

pub fn aio_srcif_set_param(sub: &uniphier_aio_sub) -> i32 {
    let r = &sub.aio.chip.regmap;

    regmap_write(r, PBINMXCTR(sub.swm.iif.map),
                 PBINMXCTR_NCONNECT_CONNECT |
                 PBINMXCTR_INOUTSEL_OUT |
                 ((sub.swm.oport.map as u32) << PBINMXCTR_PBINSEL_SHIFT) |
                 PBINMXCTR_ENDIAN_3210 |
                 PBINMXCTR_MEMFMT_D0);

    0
}

pub fn aio_srcch_set_param(sub: &uniphier_aio_sub) -> i32 {
    let r = &sub.aio.chip.regmap;

    regmap_write(r, CDA2D_CHMXCTRL1(sub.swm.och.map),
                 CDA2D_CHMXCTRL1_INDSIZE_INFINITE);

    regmap_write(r, CDA2D_CHMXSRCAMODE(sub.swm.och.map),
                 CDA2D_CHMXAMODE_ENDIAN_3210 |
                 CDA2D_CHMXAMODE_AUPDT_FIX |
                 CDA2D_CHMXAMODE_TYPE_NORMAL);

    regmap_write(r, CDA2D_CHMXDSTAMODE(sub.swm.och.map),
                 CDA2D_CHMXAMODE_ENDIAN_3210 |
                 CDA2D_CHMXAMODE_AUPDT_INC |
                 CDA2D_CHMXAMODE_TYPE_RING |
                 ((sub.swm.och.map as u32) << CDA2D_CHMXAMODE_RSSEL_SHIFT));

    0
}

pub fn aio_srcch_set_enable(sub: &uniphier_aio_sub, enable: i32) {
    let r = &sub.aio.chip.regmap;
    let v: u32;

    if enable != 0 {
        v = CDA2D_STRT0_STOP_START;
    } else {
        v = CDA2D_STRT0_STOP_STOP;
    }

    regmap_write(r, CDA2D_STRT0,
                 v | (1u32 << sub.swm.och.map));
}

pub fn aiodma_ch_set_param(sub: &uniphier_aio_sub) -> i32 {
    let r = &sub.aio.chip.regmap;
    let v: u32;

    regmap_write(r, CDA2D_CHMXCTRL1(sub.swm.ch.map),
                 CDA2D_CHMXCTRL1_INDSIZE_INFINITE);

    v = CDA2D_CHMXAMODE_ENDIAN_3210 |
        CDA2D_CHMXAMODE_AUPDT_INC |
        CDA2D_CHMXAMODE_TYPE_NORMAL |
        ((sub.swm.rb.map as u32) << CDA2D_CHMXAMODE_RSSEL_SHIFT);
    if sub.swm.dir == PORT_DIR_OUTPUT {
        regmap_write(r, CDA2D_CHMXSRCAMODE(sub.swm.ch.map), v);
    } else {
        regmap_write(r, CDA2D_CHMXDSTAMODE(sub.swm.ch.map), v);
    }

    0
}

pub fn aiodma_ch_set_enable(sub: &uniphier_aio_sub, enable: i32) {
    let r = &sub.aio.chip.regmap;

    if enable != 0 {
        regmap_write(r, CDA2D_STRT0,
                     CDA2D_STRT0_STOP_START | (1u32 << sub.swm.ch.map));

        regmap_update_bits(r, INTRBIM(0),
                           1u32 << sub.swm.rb.map,
                           1u32 << sub.swm.rb.map);
    } else {
        regmap_write(r, CDA2D_STRT0,
                     CDA2D_STRT0_STOP_STOP | (1u32 << sub.swm.ch.map));

        regmap_update_bits(r, INTRBIM(0),
                           1u32 << sub.swm.rb.map,
                           0);
    }
}

fn aiodma_rb_get_rp(sub: &uniphier_aio_sub) -> u64 {
    let r = &sub.aio.chip.regmap;
    let mut pos_u: u32;
    let mut pos_l: u32 = 0;
    let mut i: i32;

    regmap_write(r, CDA2D_RDPTRLOAD,
                 CDA2D_RDPTRLOAD_LSFLAG_STORE | (1u32 << sub.swm.rb.map));
    for i in 0..6 {
        regmap_read(r, CDA2D_RBMXRDPTR(sub.swm.rb.map), &mut pos_l);
    }

    regmap_read(r, CDA2D_RBMXRDPTR(sub.swm.rb.map), &mut pos_l);
    regmap_read(r, CDA2D_RBMXRDPTRU(sub.swm.rb.map), &mut pos_u);
    pos_u = FIELD_GET(CDA2D_RBMXPTRU_PTRU_MASK, pos_u);

    ((pos_u as u64) << 32) | (pos_l as u64)
}

fn aiodma_rb_set_rp(sub: &uniphier_aio_sub, pos: u64) {
    let r = &sub.aio.chip.regmap;
    let mut tmp: u32 = 0;
    let mut i: i32;

    regmap_write(r, CDA2D_RBMXRDPTR(sub.swm.rb.map), pos as u32);
    regmap_write(r, CDA2D_RBMXRDPTRU(sub.swm.rb.map), (pos >> 32) as u32);
    regmap_write(r, CDA2D_RDPTRLOAD, 1u32 << sub.swm.rb.map);
    for i in 0..6 {
        regmap_read(r, CDA2D_RBMXRDPTR(sub.swm.rb.map), &mut tmp);
    }
}

fn aiodma_rb_get_wp(sub: &uniphier_aio_sub) -> u64 {
    let r = &sub.aio.chip.regmap;
    let mut pos_u: u32;
    let mut pos_l: u32 = 0;
    let mut i: i32;

    regmap_write(r, CDA2D_WRPTRLOAD,
                 CDA2D_WRPTRLOAD_LSFLAG_STORE | (1u32 << sub.swm.rb.map));
    for i in 0..6 {
        regmap_read(r, CDA2D_RBMXWRPTR(sub.swm.rb.map), &mut pos_l);
    }

    regmap_read(r, CDA2D_RBMXWRPTR(sub.swm.rb.map), &mut pos_l);
    regmap_read(r, CDA2D_RBMXWRPTRU(sub.swm.rb.map), &mut pos_u);
    pos_u = FIELD_GET(CDA2D_RBMXPTRU_PTRU_MASK, pos_u);

    ((pos_u as u64) << 32) | (pos_l as u64)
}

fn aiodma_rb_set_wp(sub: &uniphier_aio_sub, pos: u64) {
    let r = &sub.aio.chip.regmap;
    let mut tmp: u32 = 0;
    let mut i: i32;

    regmap_write(r, CDA2D_RBMXWRPTR(sub.swm.rb.map),
                 lower_32_bits(pos));
    regmap_write(r, CDA2D_RBMXWRPTRU(sub.swm.rb.map),
                 upper_32_bits(pos));
    regmap_write(r, CDA2D_WRPTRLOAD, 1u32 << sub.swm.rb.map);
    for i in 0..6 {
        regmap_read(r, CDA2D_RBMXWRPTR(sub.swm.rb.map), &mut tmp);
    }
}

pub fn aiodma_rb_set_threshold(sub: &uniphier_aio_sub, size: u64, th: u32) -> i32 {
    let r = &sub.aio.chip.regmap;

    if size <= (th as u64) {
        return -EINVAL;
    }

    regmap_write(r, CDA2D_RBMXBTH(sub.swm.rb.map), th);
    regmap_write(r, CDA2D_RBMXRTH(sub.swm.rb.map), th);

    0
}

pub fn aiodma_rb_set_buffer(sub: &mut uniphier_aio_sub, start: u64, end: u64,
                            period: i32) -> i32 {
    let r = &sub.aio.chip.regmap;
    let size = end - start;
    let ret: i32;

    if end < start || period < 0 {
        return -EINVAL;
    }

    regmap_write(r, CDA2D_RBMXCNFG(sub.swm.rb.map), 0);
    regmap_write(r, CDA2D_RBMXBGNADRS(sub.swm.rb.map),
                 lower_32_bits(start));
    regmap_write(r, CDA2D_RBMXBGNADRSU(sub.swm.rb.map),
                 upper_32_bits(start));
    regmap_write(r, CDA2D_RBMXENDADRS(sub.swm.rb.map),
                 lower_32_bits(end));
    regmap_write(r, CDA2D_RBMXENDADRSU(sub.swm.rb.map),
                 upper_32_bits(end));

    regmap_write(r, CDA2D_RBADRSLOAD, 1u32 << sub.swm.rb.map);

    ret = aiodma_rb_set_threshold(sub, size, (2 * period) as u32);
    if ret != 0 {
        return ret;
    }

    if sub.swm.dir == PORT_DIR_OUTPUT {
        aiodma_rb_set_rp(sub, start);
        aiodma_rb_set_wp(sub, end - (period as u64));

        regmap_update_bits(r, CDA2D_RBMXIE(sub.swm.rb.map),
                           CDA2D_RBMXIX_SPACE,
                           CDA2D_RBMXIX_SPACE);
    } else {
        aiodma_rb_set_rp(sub, end - (period as u64));
        aiodma_rb_set_wp(sub, start);

        regmap_update_bits(r, CDA2D_RBMXIE(sub.swm.rb.map),
                           CDA2D_RBMXIX_REMAIN,
                           CDA2D_RBMXIX_REMAIN);
    }

    sub.threshold = (2 * period) as u64;
    sub.rd_offs = 0;
    sub.wr_offs = 0;
    sub.rd_org = 0;
    sub.wr_org = 0;
    sub.rd_total = 0;
    sub.wr_total = 0;

    0
}

pub fn aiodma_rb_sync(sub: &mut uniphier_aio_sub, start: u64, size: u64,
                      period: i32) {
    if sub.swm.dir == PORT_DIR_OUTPUT {
        sub.rd_offs = aiodma_rb_get_rp(sub) - start;

        if sub.use_mmap {
            sub.threshold = (2 * period) as u64;
            aiodma_rb_set_threshold(sub, size, (2 * period) as u32);

            sub.wr_offs = sub.rd_offs - (period as u64);
            if sub.rd_offs < (period as u64) {
                sub.wr_offs += size;
            }
        }
        aiodma_rb_set_wp(sub, sub.wr_offs + start);
    } else {
        sub.wr_offs = aiodma_rb_get_wp(sub) - start;

        if sub.use_mmap {
            sub.threshold = (2 * period) as u64;
            aiodma_rb_set_threshold(sub, size, (2 * period) as u32);

            sub.rd_offs = sub.wr_offs - (period as u64);
            if sub.wr_offs < (period as u64) {
                sub.rd_offs += size;
            }
        }
        aiodma_rb_set_rp(sub, sub.rd_offs + start);
    }

    sub.rd_total += sub.rd_offs - sub.rd_org;
    if sub.rd_offs < sub.rd_org {
        sub.rd_total += size;
    }
    sub.wr_total += sub.wr_offs - sub.wr_org;
    if sub.wr_offs < sub.wr_org {
        sub.wr_total += size;
    }

    sub.rd_org = sub.rd_offs;
    sub.wr_org = sub.wr_offs;
}

pub fn aiodma_rb_is_irq(sub: &uniphier_aio_sub) -> bool {
    let r = &sub.aio.chip.regmap;
    let mut ir: u32 = 0;

    regmap_read(r, CDA2D_RBMXIR(sub.swm.rb.map), &mut ir);

    if sub.swm.dir == PORT_DIR_OUTPUT {
        (ir & CDA2D_RBMXIX_SPACE) != 0
    } else {
        (ir & CDA2D_RBMXIX_REMAIN) != 0
    }
}

pub fn aiodma_rb_clear_irq(sub: &uniphier_aio_sub) {
    let r = &sub.aio.chip.regmap;

    if sub.swm.dir == PORT_DIR_OUTPUT {
        regmap_write(r, CDA2D_RBMXIR(sub.swm.rb.map),
                     CDA2D_RBMXIX_SPACE);
    } else {
        regmap_write(r, CDA2D_RBMXIR(sub.swm.rb.map),
                     CDA2D_RBMXIX_REMAIN);
    }
}

// External type and function declarations required by this module:
// struct uniphier_aio_sub, struct uniphier_aio, struct uniphier_aio_chip
// struct snd_pcm_hw_params, enum IEC61937_PC
// regmap_write, regmap_read, regmap_update_bits
// params_channels, params_rate, dev_err, FIELD_GET
// lower_32_bits, upper_32_bits

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
