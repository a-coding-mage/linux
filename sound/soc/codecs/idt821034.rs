// SPDX-License-Identifier: GPL-2.0
//
// IDT821034 ALSA SoC driver
//
// Copyright 2022 CS GROUP France
//
// Author: Herve Codina <herve.codina@bootlin.com>

const IDT821034_NB_CHANNEL: usize = 4;

#[repr(C)]
struct idt821034_amp {
    gain: u16,
    is_muted: bool,
}

#[repr(C)]
struct idt821034_cache_ch {
    power: u8,
    tx_slot: u8,
    rx_slot: u8,
    slic_conf: u8,
    slic_control: u8,
}

#[repr(C)]
struct idt821034_cache {
    codec_conf: u8,
    ch: [idt821034_cache_ch; IDT821034_NB_CHANNEL],
}

#[repr(C)]
struct idt821034_amps_ch {
    amp_out: idt821034_amp,
    amp_in: idt821034_amp,
}

#[repr(C)]
struct idt821034_amps {
    ch: [idt821034_amps_ch; IDT821034_NB_CHANNEL],
}

#[repr(C)]
struct idt821034 {
    spi: *mut spi_device,
    mutex: mutex,
    spi_tx_buf: u8, /* Cannot use stack area for SPI (dma-safe memory) */
    spi_rx_buf: u8, /* Cannot use stack area for SPI (dma-safe memory) */
    cache: idt821034_cache,
    amps: idt821034_amps,
    max_ch_playback: i32,
    max_ch_capture: i32,
    gpio_chip: gpio_chip,
}

unsafe fn idt821034_8bit_write(idt821034: *mut idt821034, val: u8) -> i32 {
    let mut xfer: [spi_transfer; 2] = [
        spi_transfer {
            tx_buf: core::ptr::addr_of_mut!((*idt821034).spi_tx_buf) as *const core::ffi::c_void,
            len: 1,
            ..unsafe { core::mem::zeroed() }
        },
        spi_transfer {
            cs_off: 1,
            tx_buf: core::ptr::addr_of_mut!((*idt821034).spi_tx_buf) as *const core::ffi::c_void,
            len: 1,
            ..unsafe { core::mem::zeroed() }
        },
    ];

    (*idt821034).spi_tx_buf = val;

    dev_vdbg!(&mut (*(*idt821034).spi).dev, "spi xfer wr 0x%x\n", val);

    spi_sync_transfer((*idt821034).spi, xfer.as_mut_ptr(), 2)
}

unsafe fn idt821034_2x8bit_write(idt821034: *mut idt821034, val1: u8, val2: u8) -> i32 {
    let mut ret: i32;

    ret = idt821034_8bit_write(idt821034, val1);
    if ret != 0 {
        return ret;
    }
    idt821034_8bit_write(idt821034, val2)
}

unsafe fn idt821034_8bit_read(idt821034: *mut idt821034, valw: u8, valr: *mut u8) -> i32 {
    let mut xfer: [spi_transfer; 2] = [
        spi_transfer {
            tx_buf: core::ptr::addr_of_mut!((*idt821034).spi_tx_buf) as *const core::ffi::c_void,
            rx_buf: core::ptr::addr_of_mut!((*idt821034).spi_rx_buf) as *mut core::ffi::c_void,
            len: 1,
            ..unsafe { core::mem::zeroed() }
        },
        spi_transfer {
            cs_off: 1,
            tx_buf: core::ptr::addr_of_mut!((*idt821034).spi_tx_buf) as *const core::ffi::c_void,
            len: 1,
            ..unsafe { core::mem::zeroed() }
        },
    ];
    let mut ret: i32;

    (*idt821034).spi_tx_buf = valw;

    ret = spi_sync_transfer((*idt821034).spi, xfer.as_mut_ptr(), 2);
    if ret != 0 {
        return ret;
    }

    *valr = (*idt821034).spi_rx_buf;

    dev_vdbg!(
        &mut (*(*idt821034).spi).dev,
        "spi xfer wr 0x%x, rd 0x%x\n",
        valw,
        *valr
    );

    0
}

/* Available mode for the programming sequence */
const fn IDT821034_MODE_CODEC(ch: u8) -> u8 {
    0x80 | (ch << 2)
}
const fn IDT821034_MODE_SLIC(ch: u8) -> u8 {
    0xD0 | (ch << 2)
}
const fn IDT821034_MODE_GAIN(ch: u8) -> u8 {
    0xC0 | (ch << 2)
}

/* Power values that can be used in 'power' (can be ORed) */
const IDT821034_CONF_PWRUP_TX: u8 = BIT!(1); /* from analog input to PCM */
const IDT821034_CONF_PWRUP_RX: u8 = BIT!(0); /* from PCM to analog output */

unsafe fn idt821034_set_channel_power(idt821034: *mut idt821034, ch: u8, power: u8) -> i32 {
    let mut conf: u8;
    let mut ret: i32;

    dev_dbg!(
        &mut (*(*idt821034).spi).dev,
        "set_channel_power(%u, 0x%x)\n",
        ch,
        power
    );

    conf = IDT821034_MODE_CODEC(ch) | (*idt821034).cache.codec_conf;

    if power & IDT821034_CONF_PWRUP_RX != 0 {
        ret = idt821034_2x8bit_write(
            idt821034,
            conf | IDT821034_CONF_PWRUP_RX,
            (*idt821034).cache.ch[ch as usize].rx_slot,
        );
        if ret != 0 {
            return ret;
        }
    }
    if power & IDT821034_CONF_PWRUP_TX != 0 {
        ret = idt821034_2x8bit_write(
            idt821034,
            conf | IDT821034_CONF_PWRUP_TX,
            (*idt821034).cache.ch[ch as usize].tx_slot,
        );
        if ret != 0 {
            return ret;
        }
    }
    if power & (IDT821034_CONF_PWRUP_TX | IDT821034_CONF_PWRUP_RX) == 0 {
        ret = idt821034_2x8bit_write(idt821034, conf, 0);
        if ret != 0 {
            return ret;
        }
    }

    (*idt821034).cache.ch[ch as usize].power = power;

    0
}

unsafe fn idt821034_get_channel_power(idt821034: *mut idt821034, ch: u8) -> u8 {
    (*idt821034).cache.ch[ch as usize].power
}

/* Codec configuration values that can be used in 'codec_conf' (can be ORed) */
const IDT821034_CONF_ALAW_MODE: u8 = BIT!(5);
const IDT821034_CONF_DELAY_MODE: u8 = BIT!(4);

unsafe fn idt821034_set_codec_conf(idt821034: *mut idt821034, codec_conf: u8) -> i32 {
    let mut conf: u8;
    let mut ts: u8;
    let mut ret: i32;

    dev_dbg!(
        &mut (*(*idt821034).spi).dev,
        "set_codec_conf(0x%x)\n",
        codec_conf
    );

    /* codec conf fields are common to all channel.
     * Arbitrary use of channel 0 for this configuration.
     */

    /* Set Configuration Register */
    conf = IDT821034_MODE_CODEC(0) | codec_conf;

    /* Update conf value and timeslot register value according
     * to cache values
     */
    if (*idt821034).cache.ch[0].power & IDT821034_CONF_PWRUP_RX != 0 {
        conf |= IDT821034_CONF_PWRUP_RX;
        ts = (*idt821034).cache.ch[0].rx_slot;
    } else if (*idt821034).cache.ch[0].power & IDT821034_CONF_PWRUP_TX != 0 {
        conf |= IDT821034_CONF_PWRUP_TX;
        ts = (*idt821034).cache.ch[0].tx_slot;
    } else {
        ts = 0x00;
    }

    /* Write configuration register and time-slot register */
    ret = idt821034_2x8bit_write(idt821034, conf, ts);
    if ret != 0 {
        return ret;
    }

    (*idt821034).cache.codec_conf = codec_conf;
    0
}

unsafe fn idt821034_get_codec_conf(idt821034: *mut idt821034) -> u8 {
    (*idt821034).cache.codec_conf
}

/* Channel direction values that can be used in 'ch_dir' (can be ORed) */
const IDT821034_CH_RX: u8 = BIT!(0); /* from PCM to analog output */
const IDT821034_CH_TX: u8 = BIT!(1); /* from analog input to PCM */

unsafe fn idt821034_set_channel_ts(
    idt821034: *mut idt821034,
    ch: u8,
    ch_dir: u8,
    ts_num: u8,
) -> i32 {
    let mut conf: u8;
    let mut ret: i32;

    dev_dbg!(
        &mut (*(*idt821034).spi).dev,
        "set_channel_ts(%u, 0x%x, %d)\n",
        ch,
        ch_dir,
        ts_num
    );

    conf = IDT821034_MODE_CODEC(ch) | (*idt821034).cache.codec_conf;

    if ch_dir & IDT821034_CH_RX != 0 {
        if (*idt821034).cache.ch[ch as usize].power & IDT821034_CONF_PWRUP_RX != 0 {
            ret = idt821034_2x8bit_write(idt821034, conf | IDT821034_CONF_PWRUP_RX, ts_num);
            if ret != 0 {
                return ret;
            }
        }
        (*idt821034).cache.ch[ch as usize].rx_slot = ts_num;
    }
    if ch_dir & IDT821034_CH_TX != 0 {
        if (*idt821034).cache.ch[ch as usize].power & IDT821034_CONF_PWRUP_TX != 0 {
            ret = idt821034_2x8bit_write(idt821034, conf | IDT821034_CONF_PWRUP_TX, ts_num);
            if ret != 0 {
                return ret;
            }
        }
        (*idt821034).cache.ch[ch as usize].tx_slot = ts_num;
    }

    0
}

/* SLIC direction values that can be used in 'slic_dir' (can be ORed) */
const IDT821034_SLIC_IO1_IN: u8 = BIT!(1);
const IDT821034_SLIC_IO0_IN: u8 = BIT!(0);

unsafe fn idt821034_set_slic_conf(idt821034: *mut idt821034, ch: u8, slic_dir: u8) -> i32 {
    let mut conf: u8;
    let mut ret: i32;

    dev_dbg!(
        &mut (*(*idt821034).spi).dev,
        "set_slic_conf(%u, 0x%x)\n",
        ch,
        slic_dir
    );

    conf = IDT821034_MODE_SLIC(ch) | slic_dir;
    ret = idt821034_2x8bit_write(
        idt821034,
        conf,
        (*idt821034).cache.ch[ch as usize].slic_control,
    );
    if ret != 0 {
        return ret;
    }

    (*idt821034).cache.ch[ch as usize].slic_conf = slic_dir;

    0
}

unsafe fn idt821034_get_slic_conf(idt821034: *mut idt821034, ch: u8) -> u8 {
    (*idt821034).cache.ch[ch as usize].slic_conf
}

unsafe fn idt821034_write_slic_raw(idt821034: *mut idt821034, ch: u8, slic_raw: u8) -> i32 {
    let mut conf: u8;
    let mut ret: i32;

    dev_dbg!(
        &mut (*(*idt821034).spi).dev,
        "write_slic_raw(%u, 0x%x)\n",
        ch,
        slic_raw
    );

    /*
     * On write, slic_raw is mapped as follow :
     *   b4: O_4
     *   b3: O_3
     *   b2: O_2
     *   b1: I/O_1
     *   b0: I/O_0
     */

    conf = IDT821034_MODE_SLIC(ch) | (*idt821034).cache.ch[ch as usize].slic_conf;
    ret = idt821034_2x8bit_write(idt821034, conf, slic_raw);
    if ret != 0 {
        return ret;
    }

    (*idt821034).cache.ch[ch as usize].slic_control = slic_raw;
    0
}

unsafe fn idt821034_get_written_slic_raw(idt821034: *mut idt821034, ch: u8) -> u8 {
    (*idt821034).cache.ch[ch as usize].slic_control
}

unsafe fn idt821034_read_slic_raw(idt821034: *mut idt821034, ch: u8, slic_raw: *mut u8) -> i32 {
    let mut val: u8;
    let mut ret: i32;

    /*
     * On read, slic_raw is mapped as follow :
     *   b7: I/O_0
     *   b6: I/O_1
     *   b5: O_2
     *   b4: O_3
     *   b3: O_4
     *   b2: I/O1_0, I/O_0 from channel 1 (no matter ch value)
     *   b1: I/O2_0, I/O_0 from channel 2 (no matter ch value)
     *   b2: I/O3_0, I/O_0 from channel 3 (no matter ch value)
     */

    val = IDT821034_MODE_SLIC(ch) | (*idt821034).cache.ch[ch as usize].slic_conf;
    ret = idt821034_8bit_write(idt821034, val);
    if ret != 0 {
        return ret;
    }

    ret = idt821034_8bit_read(
        idt821034,
        (*idt821034).cache.ch[ch as usize].slic_control,
        slic_raw,
    );
    if ret != 0 {
        return ret;
    }

    dev_dbg!(
        &mut (*(*idt821034).spi).dev,
        "read_slic_raw(%i) 0x%x\n",
        ch,
        *slic_raw
    );

    0
}

/* Gain type values that can be used in 'gain_type' (cannot be ORed) */
const IDT821034_GAIN_RX: u8 = 0 << 1; /* from PCM to analog output */
const IDT821034_GAIN_TX: u8 = 1 << 1; /* from analog input to PCM */

unsafe fn idt821034_set_gain_channel(
    idt821034: *mut idt821034,
    ch: u8,
    gain_type: u8,
    gain_val: u16,
) -> i32 {
    let mut conf: u8;
    let mut ret: i32;

    dev_dbg!(
        &mut (*(*idt821034).spi).dev,
        "set_gain_channel(%u, 0x%x, 0x%x-%d)\n",
        ch,
        gain_type,
        gain_val,
        gain_val
    );

    /*
     * The gain programming coefficients should be calculated as:
     *   Transmit : Coeff_X = round [ gain_X0dB x gain_X ]
     *   Receive: Coeff_R = round [ gain_R0dB x gain_R ]
     * where:
     *   gain_X0dB = 1820;
     *   gain_X is the target gain;
     *   Coeff_X should be in the range of 0 to 8192.
     *   gain_R0dB = 2506;
     *   gain_R is the target gain;
     *   Coeff_R should be in the range of 0 to 8192.
     *
     * A gain programming coefficient is 14-bit wide and in binary format.
     * The 7 Most Significant Bits of the coefficient is called
     * GA_MSB_Transmit for transmit path, or is called GA_MSB_Receive for
     * receive path; The 7 Least Significant Bits of the coefficient is
     * called GA_LSB_ Transmit for transmit path, or is called
     * GA_LSB_Receive for receive path.
     *
     * An example is given below to clarify the calculation of the
     * coefficient. To program a +3 dB gain in transmit path and a -3.5 dB
     * gain in receive path:
     *
     * Linear Code of +3dB = 10^(3/20)= 1.412537545
     * Coeff_X = round (1820 x 1.412537545) = 2571
     *                                      = 0b001010_00001011
     * GA_MSB_Transmit = 0b0010100
     * GA_LSB_Transmit = 0b0001011
     *
     * Linear Code of -3.5dB = 10^(-3.5/20) = 0.668343917
     * Coeff_R= round (2506 x 0.668343917) = 1675
     *                                     = 0b0001101_0001011
     * GA_MSB_Receive = 0b0001101
     * GA_LSB_Receive = 0b0001011
     */

    conf = IDT821034_MODE_GAIN(ch) | gain_type;

    ret = idt821034_2x8bit_write(idt821034, conf | 0x00, (gain_val & 0x007F) as u8);
    if ret != 0 {
        return ret;
    }

    ret = idt821034_2x8bit_write(idt821034, conf | 0x01, ((gain_val >> 7) & 0x7F) as u8);
    if ret != 0 {
        return ret;
    }

    0
}

/* Id helpers used in controls and dapm */
const IDT821034_DIR_OUT: u32 = 1 << 3;
const IDT821034_DIR_IN: u32 = 0 << 3;
const fn IDT821034_ID(ch: u32, dir: u32) -> u32 {
    (ch & 0x03) | dir
}
const fn IDT821034_ID_OUT(ch: u32) -> u32 {
    IDT821034_ID(ch, IDT821034_DIR_OUT)
}
const fn IDT821034_ID_IN(ch: u32) -> u32 {
    IDT821034_ID(ch, IDT821034_DIR_IN)
}

const fn IDT821034_ID_GET_CHAN(id: u32) -> u8 {
    (id & 0x03) as u8
}
const fn IDT821034_ID_GET_DIR(id: u32) -> u32 {
    id & (1 << 3)
}
const fn IDT821034_ID_IS_OUT(id: u32) -> bool {
    IDT821034_ID_GET_DIR(id) == IDT821034_DIR_OUT
}

unsafe fn idt821034_kctrl_gain_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let idt821034: *mut idt821034 = snd_soc_component_get_drvdata(component);
    let min: i32 = (*mc).min;
    let max: i32 = (*mc).max;
    let mask: u32 = (1u32 << fls(max)) - 1;
    let invert: u32 = (*mc).invert;
    let mut val: i32 = 0;
    let ch: u8;

    ch = IDT821034_ID_GET_CHAN((*mc).reg as u32);

    scoped_guard!(mutex, &mut (*idt821034).mutex, {
        if IDT821034_ID_IS_OUT((*mc).reg as u32) {
            val = (*idt821034).amps.ch[ch as usize].amp_out.gain as i32;
        } else {
            val = (*idt821034).amps.ch[ch as usize].amp_in.gain as i32;
        }
    });

    (*ucontrol).value.integer.value[0] = (val as u32 & mask) as _;
    if invert != 0 {
        (*ucontrol).value.integer.value[0] = max as _ - (*ucontrol).value.integer.value[0];
    } else {
        (*ucontrol).value.integer.value[0] = (*ucontrol).value.integer.value[0] - min as _;
    }

    0
}

unsafe fn idt821034_kctrl_gain_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let idt821034: *mut idt821034 = snd_soc_component_get_drvdata(component);
    let mut amp: *mut idt821034_amp;
    let min: i32 = (*mc).min;
    let max: i32 = (*mc).max;
    let mask: u32 = (1u32 << fls(max)) - 1;
    let invert: u32 = (*mc).invert;
    let mut val: u32;
    let mut ret: i32;
    let gain_type: u8;
    let ch: u8;

    val = (*ucontrol).value.integer.value[0] as u32;
    if val > (max - min) as u32 {
        return -EINVAL;
    }

    if invert != 0 {
        val = (max as u32 - val) & mask;
    } else {
        val = (val + min as u32) & mask;
    }

    ch = IDT821034_ID_GET_CHAN((*mc).reg as u32);

    guard!(mutex, &mut (*idt821034).mutex);

    if IDT821034_ID_IS_OUT((*mc).reg as u32) {
        amp = &mut (*idt821034).amps.ch[ch as usize].amp_out;
        gain_type = IDT821034_GAIN_RX;
    } else {
        amp = &mut (*idt821034).amps.ch[ch as usize].amp_in;
        gain_type = IDT821034_GAIN_TX;
    }

    if (*amp).gain as u32 == val {
        return 0;
    }

    if !(*amp).is_muted {
        ret = idt821034_set_gain_channel(idt821034, ch, gain_type, val as u16);
        if ret != 0 {
            return ret;
        }
    }

    (*amp).gain = val as u16;

    1
}

unsafe fn idt821034_kctrl_mute_get(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let idt821034: *mut idt821034 = snd_soc_component_get_drvdata(component);
    let id: i32 = (*kcontrol).private_value as i32;
    let mut is_muted: bool = false;
    let ch: u8;

    ch = IDT821034_ID_GET_CHAN(id as u32);

    scoped_guard!(mutex, &mut (*idt821034).mutex, {
        is_muted = if IDT821034_ID_IS_OUT(id as u32) {
            (*idt821034).amps.ch[ch as usize].amp_out.is_muted
        } else {
            (*idt821034).amps.ch[ch as usize].amp_in.is_muted
        };
    });

    (*ucontrol).value.integer.value[0] = (!is_muted) as _;

    0
}

unsafe fn idt821034_kctrl_mute_put(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let idt821034: *mut idt821034 = snd_soc_component_get_drvdata(component);
    let id: i32 = (*kcontrol).private_value as i32;
    let mut amp: *mut idt821034_amp;
    let is_mute: bool;
    let gain_type: u8;
    let mut ret: i32;
    let ch: u8;

    ch = IDT821034_ID_GET_CHAN(id as u32);
    is_mute = (*ucontrol).value.integer.value[0] == 0;

    guard!(mutex, &mut (*idt821034).mutex);

    if IDT821034_ID_IS_OUT(id as u32) {
        amp = &mut (*idt821034).amps.ch[ch as usize].amp_out;
        gain_type = IDT821034_GAIN_RX;
    } else {
        amp = &mut (*idt821034).amps.ch[ch as usize].amp_in;
        gain_type = IDT821034_GAIN_TX;
    }

    if (*amp).is_muted == is_mute {
        return 0;
    }

    ret = idt821034_set_gain_channel(
        idt821034,
        ch,
        gain_type,
        if is_mute { 0 } else { (*amp).gain },
    );
    if ret != 0 {
        return ret;
    }

    (*amp).is_muted = is_mute;

    1
}

static_tlv_db_linear!(idt821034_gain_in, -300, 1300);
const IDT821034_GAIN_IN_MIN_RAW: u16 = 1288; /* -3.0 dB -> 10^(-3.0/20.0) * 1820 = 1288 */
const IDT821034_GAIN_IN_MAX_RAW: u16 = 8130; /* 13.0 dB -> 10^(13.0/20.0) * 1820 = 8130 */
const IDT821034_GAIN_IN_INIT_RAW: u16 = 1820; /* 0dB -> 10^(0/20) * 1820 = 1820 */

static_tlv_db_linear!(idt821034_gain_out, -1300, 300);
const IDT821034_GAIN_OUT_MIN_RAW: u16 = 561; /* -13.0 dB -> 10^(-13.0/20.0) * 2506 = 561 */
const IDT821034_GAIN_OUT_MAX_RAW: u16 = 3540; /* 3.0 dB -> 10^(3.0/20.0) * 2506 = 3540 */
const IDT821034_GAIN_OUT_INIT_RAW: u16 = 2506; /* 0dB -> 10^(0/20) * 2506 = 2506 */

static_snd_kcontrol_new_array!(
    idt821034_controls,
    /* DAC volume control */
    SOC_SINGLE_RANGE_EXT_TLV!("DAC0 Playback Volume", IDT821034_ID_OUT(0), 0, IDT821034_GAIN_OUT_MIN_RAW, IDT821034_GAIN_OUT_MAX_RAW, 0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put, idt821034_gain_out),
    SOC_SINGLE_RANGE_EXT_TLV!("DAC1 Playback Volume", IDT821034_ID_OUT(1), 0, IDT821034_GAIN_OUT_MIN_RAW, IDT821034_GAIN_OUT_MAX_RAW, 0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put, idt821034_gain_out),
    SOC_SINGLE_RANGE_EXT_TLV!("DAC2 Playback Volume", IDT821034_ID_OUT(2), 0, IDT821034_GAIN_OUT_MIN_RAW, IDT821034_GAIN_OUT_MAX_RAW, 0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put, idt821034_gain_out),
    SOC_SINGLE_RANGE_EXT_TLV!("DAC3 Playback Volume", IDT821034_ID_OUT(3), 0, IDT821034_GAIN_OUT_MIN_RAW, IDT821034_GAIN_OUT_MAX_RAW, 0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put, idt821034_gain_out),
    /* DAC mute control */
    SOC_SINGLE_BOOL_EXT!("DAC0 Playback Switch", IDT821034_ID_OUT(0), idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    SOC_SINGLE_BOOL_EXT!("DAC1 Playback Switch", IDT821034_ID_OUT(1), idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    SOC_SINGLE_BOOL_EXT!("DAC2 Playback Switch", IDT821034_ID_OUT(2), idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    SOC_SINGLE_BOOL_EXT!("DAC3 Playback Switch", IDT821034_ID_OUT(3), idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    /* ADC volume control */
    SOC_SINGLE_RANGE_EXT_TLV!("ADC0 Capture Volume", IDT821034_ID_IN(0), 0, IDT821034_GAIN_IN_MIN_RAW, IDT821034_GAIN_IN_MAX_RAW, 0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put, idt821034_gain_in),
    SOC_SINGLE_RANGE_EXT_TLV!("ADC1 Capture Volume", IDT821034_ID_IN(1), 0, IDT821034_GAIN_IN_MIN_RAW, IDT821034_GAIN_IN_MAX_RAW, 0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put, idt821034_gain_in),
    SOC_SINGLE_RANGE_EXT_TLV!("ADC2 Capture Volume", IDT821034_ID_IN(2), 0, IDT821034_GAIN_IN_MIN_RAW, IDT821034_GAIN_IN_MAX_RAW, 0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put, idt821034_gain_in),
    SOC_SINGLE_RANGE_EXT_TLV!("ADC3 Capture Volume", IDT821034_ID_IN(3), 0, IDT821034_GAIN_IN_MIN_RAW, IDT821034_GAIN_IN_MAX_RAW, 0, idt821034_kctrl_gain_get, idt821034_kctrl_gain_put, idt821034_gain_in),
    /* ADC mute control */
    SOC_SINGLE_BOOL_EXT!("ADC0 Capture Switch", IDT821034_ID_IN(0), idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    SOC_SINGLE_BOOL_EXT!("ADC1 Capture Switch", IDT821034_ID_IN(1), idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    SOC_SINGLE_BOOL_EXT!("ADC2 Capture Switch", IDT821034_ID_IN(2), idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
    SOC_SINGLE_BOOL_EXT!("ADC3 Capture Switch", IDT821034_ID_IN(3), idt821034_kctrl_mute_get, idt821034_kctrl_mute_put),
);

unsafe fn idt821034_power_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: i32,
) -> i32 {
    let component: *mut snd_soc_component = snd_soc_dapm_to_component((*w).dapm);
    let idt821034: *mut idt821034 = snd_soc_component_get_drvdata(component);
    let id: u32 = (*w).shift as u32;
    let mut power: u8;
    let mask: u8;
    let ch: u8;

    ch = IDT821034_ID_GET_CHAN(id);
    mask = if IDT821034_ID_IS_OUT(id) {
        IDT821034_CONF_PWRUP_RX
    } else {
        IDT821034_CONF_PWRUP_TX
    };

    guard!(mutex, &mut (*idt821034).mutex);

    power = idt821034_get_channel_power(idt821034, ch);
    if SND_SOC_DAPM_EVENT_ON(event) {
        power |= mask;
    } else {
        power &= !mask;
    }

    idt821034_set_channel_power(idt821034, ch, power)
}

static_snd_soc_dapm_widget_array!(
    idt821034_dapm_widgets,
    SND_SOC_DAPM_DAC_E!("DAC0", "Playback", SND_SOC_NOPM, IDT821034_ID_OUT(0), 0, idt821034_power_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E!("DAC1", "Playback", SND_SOC_NOPM, IDT821034_ID_OUT(1), 0, idt821034_power_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E!("DAC2", "Playback", SND_SOC_NOPM, IDT821034_ID_OUT(2), 0, idt821034_power_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E!("DAC3", "Playback", SND_SOC_NOPM, IDT821034_ID_OUT(3), 0, idt821034_power_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_OUTPUT!("OUT0"),
    SND_SOC_DAPM_OUTPUT!("OUT1"),
    SND_SOC_DAPM_OUTPUT!("OUT2"),
    SND_SOC_DAPM_OUTPUT!("OUT3"),
    SND_SOC_DAPM_DAC_E!("ADC0", "Capture", SND_SOC_NOPM, IDT821034_ID_IN(0), 0, idt821034_power_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E!("ADC1", "Capture", SND_SOC_NOPM, IDT821034_ID_IN(1), 0, idt821034_power_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E!("ADC2", "Capture", SND_SOC_NOPM, IDT821034_ID_IN(2), 0, idt821034_power_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_DAC_E!("ADC3", "Capture", SND_SOC_NOPM, IDT821034_ID_IN(3), 0, idt821034_power_event, SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_POST_PMD),
    SND_SOC_DAPM_INPUT!("IN0"),
    SND_SOC_DAPM_INPUT!("IN1"),
    SND_SOC_DAPM_INPUT!("IN2"),
    SND_SOC_DAPM_INPUT!("IN3"),
);

static_snd_soc_dapm_route_array!(
    idt821034_dapm_routes,
    snd_soc_dapm_route { sink: c_str!("OUT0"), control: core::ptr::null(), source: c_str!("DAC0") },
    snd_soc_dapm_route { sink: c_str!("OUT1"), control: core::ptr::null(), source: c_str!("DAC1") },
    snd_soc_dapm_route { sink: c_str!("OUT2"), control: core::ptr::null(), source: c_str!("DAC2") },
    snd_soc_dapm_route { sink: c_str!("OUT3"), control: core::ptr::null(), source: c_str!("DAC3") },
    snd_soc_dapm_route { sink: c_str!("ADC0"), control: core::ptr::null(), source: c_str!("IN0") },
    snd_soc_dapm_route { sink: c_str!("ADC1"), control: core::ptr::null(), source: c_str!("IN1") },
    snd_soc_dapm_route { sink: c_str!("ADC2"), control: core::ptr::null(), source: c_str!("IN2") },
    snd_soc_dapm_route { sink: c_str!("ADC3"), control: core::ptr::null(), source: c_str!("IN3") },
);

unsafe fn idt821034_dai_set_tdm_slot(
    dai: *mut snd_soc_dai,
    tx_mask: u32,
    rx_mask: u32,
    _slots: i32,
    width: i32,
) -> i32 {
    let idt821034: *mut idt821034 = snd_soc_component_get_drvdata((*dai).component);
    let mut mask: u32;
    let mut slot: u8;
    let mut ret: i32 = 0;
    let mut ch: u8;

    match width {
        0 | 8 => {}
        _ => {
            dev_err!((*dai).dev, "tdm slot width %d not supported\n", width);
            return -EINVAL;
        }
    }

    mask = tx_mask;
    slot = 0;
    ch = 0;
    while mask != 0 && (ch as usize) < IDT821034_NB_CHANNEL {
        if mask & 0x1 != 0 {
            scoped_guard!(mutex, &mut (*idt821034).mutex, {
                ret = idt821034_set_channel_ts(idt821034, ch, IDT821034_CH_RX, slot);
            });
            if ret != 0 {
                dev_err!(
                    (*dai).dev,
                    "ch%u set tx tdm slot failed (%d)\n",
                    ch,
                    ret
                );
                return ret;
            }
            ch += 1;
        }
        mask >>= 1;
        slot = slot.wrapping_add(1);
    }
    if mask != 0 {
        dev_err!(
            (*dai).dev,
            "too much tx slots defined (mask = 0x%x) support max %d\n",
            tx_mask,
            IDT821034_NB_CHANNEL
        );
        return -EINVAL;
    }
    (*idt821034).max_ch_playback = ch as i32;

    mask = rx_mask;
    slot = 0;
    ch = 0;
    while mask != 0 && (ch as usize) < IDT821034_NB_CHANNEL {
        if mask & 0x1 != 0 {
            scoped_guard!(mutex, &mut (*idt821034).mutex, {
                ret = idt821034_set_channel_ts(idt821034, ch, IDT821034_CH_TX, slot);
            });
            if ret != 0 {
                dev_err!(
                    (*dai).dev,
                    "ch%u set rx tdm slot failed (%d)\n",
                    ch,
                    ret
                );
                return ret;
            }
            ch += 1;
        }
        mask >>= 1;
        slot = slot.wrapping_add(1);
    }
    if mask != 0 {
        dev_err!(
            (*dai).dev,
            "too much rx slots defined (mask = 0x%x) support max %d\n",
            rx_mask,
            IDT821034_NB_CHANNEL
        );
        return -EINVAL;
    }
    (*idt821034).max_ch_capture = ch as i32;

    0
}

unsafe fn idt821034_dai_set_fmt(dai: *mut snd_soc_dai, fmt: u32) -> i32 {
    let idt821034: *mut idt821034 = snd_soc_component_get_drvdata((*dai).component);
    let mut conf: u8;

    guard!(mutex, &mut (*idt821034).mutex);

    conf = idt821034_get_codec_conf(idt821034);

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_DSP_A => {
            conf |= IDT821034_CONF_DELAY_MODE;
        }
        SND_SOC_DAIFMT_DSP_B => {
            conf &= !IDT821034_CONF_DELAY_MODE;
        }
        _ => {
            dev_err!(
                (*dai).dev,
                "Unsupported DAI format 0x%x\n",
                fmt & SND_SOC_DAIFMT_FORMAT_MASK
            );
            return -EINVAL;
        }
    }

    idt821034_set_codec_conf(idt821034, conf)
}

unsafe fn idt821034_dai_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> i32 {
    let idt821034: *mut idt821034 = snd_soc_component_get_drvdata((*dai).component);
    let mut conf: u8;

    guard!(mutex, &mut (*idt821034).mutex);

    conf = idt821034_get_codec_conf(idt821034);

    match params_format(params) {
        SNDRV_PCM_FORMAT_A_LAW => {
            conf |= IDT821034_CONF_ALAW_MODE;
        }
        SNDRV_PCM_FORMAT_MU_LAW => {
            conf &= !IDT821034_CONF_ALAW_MODE;
        }
        _ => {
            dev_err!(
                (*dai).dev,
                "Unsupported PCM format 0x%x\n",
                params_format(params)
            );
            return -EINVAL;
        }
    }

    idt821034_set_codec_conf(idt821034, conf)
}

static idt821034_sample_bits: [u32; 1] = [8];

static mut idt821034_sample_bits_constr: snd_pcm_hw_constraint_list =
    snd_pcm_hw_constraint_list {
        list: idt821034_sample_bits.as_ptr(),
        count: ARRAY_SIZE!(idt821034_sample_bits),
    };

unsafe fn idt821034_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> i32 {
    let idt821034: *mut idt821034 = snd_soc_component_get_drvdata((*dai).component);
    let mut max_ch: u32 = 0;
    let mut ret: i32;

    max_ch = if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*idt821034).max_ch_playback as u32
    } else {
        (*idt821034).max_ch_capture as u32
    };

    /*
     * Disable stream support (min = 0, max = 0) if no timeslots were
     * configured otherwise, limit the number of channels to those
     * configured.
     */
    ret = snd_pcm_hw_constraint_minmax(
        (*substream).runtime,
        SNDRV_PCM_HW_PARAM_CHANNELS,
        if max_ch != 0 { 1 } else { 0 },
        max_ch,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_pcm_hw_constraint_list(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_SAMPLE_BITS,
        &mut idt821034_sample_bits_constr,
    );
    if ret != 0 {
        return ret;
    }

    0
}

static idt821034_dai_formats: u64 = SND_SOC_POSSIBLE_DAIFMT_DSP_A | SND_SOC_POSSIBLE_DAIFMT_DSP_B;

static mut idt821034_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(idt821034_dai_startup),
    hw_params: Some(idt821034_dai_hw_params),
    set_tdm_slot: Some(idt821034_dai_set_tdm_slot),
    set_fmt: Some(idt821034_dai_set_fmt),
    auto_selectable_formats: &idt821034_dai_formats,
    num_auto_selectable_formats: 1,
    ..unsafe { core::mem::zeroed() }
};

static mut idt821034_dai_driver: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c_str!("idt821034"),
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("Playback"),
        channels_min: 1,
        channels_max: IDT821034_NB_CHANNEL as u32,
        rates: SNDRV_PCM_RATE_8000,
        formats: SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW,
        ..unsafe { core::mem::zeroed() }
    },
    capture: snd_soc_pcm_stream {
        stream_name: c_str!("Capture"),
        channels_min: 1,
        channels_max: IDT821034_NB_CHANNEL as u32,
        rates: SNDRV_PCM_RATE_8000,
        formats: SNDRV_PCM_FMTBIT_MU_LAW | SNDRV_PCM_FMTBIT_A_LAW,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &mut idt821034_dai_ops,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn idt821034_reset_audio(idt821034: *mut idt821034) -> i32 {
    let mut ret: i32;
    let mut i: u8;

    guard!(mutex, &mut (*idt821034).mutex);

    ret = idt821034_set_codec_conf(idt821034, 0);
    if ret != 0 {
        return ret;
    }

    i = 0;
    while (i as usize) < IDT821034_NB_CHANNEL {
        (*idt821034).amps.ch[i as usize].amp_out.gain = IDT821034_GAIN_OUT_INIT_RAW;
        (*idt821034).amps.ch[i as usize].amp_out.is_muted = false;
        ret = idt821034_set_gain_channel(
            idt821034,
            i,
            IDT821034_GAIN_RX,
            (*idt821034).amps.ch[i as usize].amp_out.gain,
        );
        if ret != 0 {
            return ret;
        }

        (*idt821034).amps.ch[i as usize].amp_in.gain = IDT821034_GAIN_IN_INIT_RAW;
        (*idt821034).amps.ch[i as usize].amp_in.is_muted = false;
        ret = idt821034_set_gain_channel(
            idt821034,
            i,
            IDT821034_GAIN_TX,
            (*idt821034).amps.ch[i as usize].amp_in.gain,
        );
        if ret != 0 {
            return ret;
        }

        ret = idt821034_set_channel_power(idt821034, i, 0);
        if ret != 0 {
            return ret;
        }

        i = i.wrapping_add(1);
    }

    0
}

unsafe fn idt821034_component_probe(component: *mut snd_soc_component) -> i32 {
    let idt821034: *mut idt821034 = snd_soc_component_get_drvdata(component);
    let mut ret: i32;

    /* reset idt821034 audio part*/
    ret = idt821034_reset_audio(idt821034);
    if ret != 0 {
        return ret;
    }

    0
}

static mut idt821034_component_driver: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(idt821034_component_probe),
    controls: idt821034_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(idt821034_controls),
    dapm_widgets: idt821034_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(idt821034_dapm_widgets),
    dapm_routes: idt821034_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(idt821034_dapm_routes),
    endianness: 1,
    ..unsafe { core::mem::zeroed() }
};

const fn IDT821034_GPIO_OFFSET_TO_SLIC_CHANNEL(offset: u32) -> u8 {
    (((offset) / 5) % 4) as u8
}
const fn IDT821034_GPIO_OFFSET_TO_SLIC_MASK(offset: u32) -> u8 {
    BIT!((offset) % 5)
}

unsafe fn idt821034_chip_gpio_set(c: *mut gpio_chip, offset: u32, val: i32) -> i32 {
    let ch: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_CHANNEL(offset);
    let mask: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_MASK(offset);
    let idt821034: *mut idt821034 = gpiochip_get_data(c);
    let mut slic_raw: u8;
    let mut ret: i32;

    guard!(mutex, &mut (*idt821034).mutex);

    slic_raw = idt821034_get_written_slic_raw(idt821034, ch);
    if val != 0 {
        slic_raw |= mask;
    } else {
        slic_raw &= !mask;
    }
    ret = idt821034_write_slic_raw(idt821034, ch, slic_raw);

    if ret != 0 {
        dev_err!(
            &mut (*(*idt821034).spi).dev,
            "set gpio %d (%u, 0x%x) failed (%d)\n",
            offset,
            ch,
            mask,
            ret
        );
    }

    ret
}

unsafe fn idt821034_chip_gpio_get(c: *mut gpio_chip, offset: u32) -> i32 {
    let ch: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_CHANNEL(offset);
    let mask: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_MASK(offset);
    let idt821034: *mut idt821034 = gpiochip_get_data(c);
    let mut slic_raw: u8 = 0;
    let mut ret: i32 = 0;

    scoped_guard!(mutex, &mut (*idt821034).mutex, {
        ret = idt821034_read_slic_raw(idt821034, ch, &mut slic_raw);
    });
    if ret != 0 {
        dev_err!(
            &mut (*(*idt821034).spi).dev,
            "get gpio %d (%u, 0x%x) failed (%d)\n",
            offset,
            ch,
            mask,
            ret
        );
        return ret;
    }

    /*
     * SLIC IOs are read in reverse order compared to write.
     * Reverse the read value here in order to have IO0 at lsb (ie same
     * order as write)
     */
    ((bitrev8(slic_raw) & mask) != 0) as i32
}

unsafe fn idt821034_chip_get_direction(c: *mut gpio_chip, offset: u32) -> i32 {
    let ch: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_CHANNEL(offset);
    let mask: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_MASK(offset);
    let idt821034: *mut idt821034 = gpiochip_get_data(c);
    let slic_dir: u8;

    guard!(mutex, &mut (*idt821034).mutex);
    slic_dir = idt821034_get_slic_conf(idt821034, ch);

    if slic_dir & mask != 0 {
        GPIO_LINE_DIRECTION_IN
    } else {
        GPIO_LINE_DIRECTION_OUT
    }
}

unsafe fn idt821034_chip_direction_input(c: *mut gpio_chip, offset: u32) -> i32 {
    let ch: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_CHANNEL(offset);
    let mask: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_MASK(offset);
    let idt821034: *mut idt821034 = gpiochip_get_data(c);
    let mut slic_conf: u8;
    let mut ret: i32;

    /* Only IO0 and IO1 can be set as input */
    if mask & !(IDT821034_SLIC_IO1_IN | IDT821034_SLIC_IO0_IN) != 0 {
        return -EPERM;
    }

    guard!(mutex, &mut (*idt821034).mutex);

    slic_conf = idt821034_get_slic_conf(idt821034, ch) | mask;

    ret = idt821034_set_slic_conf(idt821034, ch, slic_conf);
    if ret != 0 {
        dev_err!(
            &mut (*(*idt821034).spi).dev,
            "dir in gpio %d (%u, 0x%x) failed (%d)\n",
            offset,
            ch,
            mask,
            ret
        );
    }

    ret
}

unsafe fn idt821034_chip_direction_output(
    c: *mut gpio_chip,
    offset: u32,
    val: i32,
) -> i32 {
    let ch: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_CHANNEL(offset);
    let mask: u8 = IDT821034_GPIO_OFFSET_TO_SLIC_MASK(offset);
    let idt821034: *mut idt821034 = gpiochip_get_data(c);
    let mut slic_conf: u8;
    let mut ret: i32;

    ret = idt821034_chip_gpio_set(c, offset, val);
    if ret != 0 {
        return ret;
    }

    guard!(mutex, &mut (*idt821034).mutex);

    slic_conf = idt821034_get_slic_conf(idt821034, ch) & !mask;

    ret = idt821034_set_slic_conf(idt821034, ch, slic_conf);
    if ret != 0 {
        dev_err!(
            &mut (*(*idt821034).spi).dev,
            "dir out gpio %d (%u, 0x%x) failed (%d)\n",
            offset,
            ch,
            mask,
            ret
        );
    }

    ret
}

unsafe fn idt821034_reset_gpio(idt821034: *mut idt821034) -> i32 {
    let mut ret: i32;
    let mut i: u8;

    guard!(mutex, &mut (*idt821034).mutex);

    /* IO0 and IO1 as input for all channels and output IO set to 0 */
    i = 0;
    while (i as usize) < IDT821034_NB_CHANNEL {
        ret = idt821034_set_slic_conf(
            idt821034,
            i,
            IDT821034_SLIC_IO1_IN | IDT821034_SLIC_IO0_IN,
        );
        if ret != 0 {
            return ret;
        }

        ret = idt821034_write_slic_raw(idt821034, i, 0);
        if ret != 0 {
            return ret;
        }

        i = i.wrapping_add(1);
    }

    0
}

unsafe fn idt821034_gpio_init(idt821034: *mut idt821034) -> i32 {
    let mut ret: i32;

    ret = idt821034_reset_gpio(idt821034);
    if ret != 0 {
        return ret;
    }

    (*idt821034).gpio_chip.owner = THIS_MODULE;
    (*idt821034).gpio_chip.label = dev_name(&mut (*(*idt821034).spi).dev);
    (*idt821034).gpio_chip.parent = &mut (*(*idt821034).spi).dev;
    (*idt821034).gpio_chip.base = -1;
    (*idt821034).gpio_chip.ngpio = 5 * 4; /* 5 GPIOs on 4 channels */
    (*idt821034).gpio_chip.get_direction = Some(idt821034_chip_get_direction);
    (*idt821034).gpio_chip.direction_input = Some(idt821034_chip_direction_input);
    (*idt821034).gpio_chip.direction_output = Some(idt821034_chip_direction_output);
    (*idt821034).gpio_chip.get = Some(idt821034_chip_gpio_get);
    (*idt821034).gpio_chip.set = Some(idt821034_chip_gpio_set);
    (*idt821034).gpio_chip.can_sleep = true;

    devm_gpiochip_add_data(
        &mut (*(*idt821034).spi).dev,
        &mut (*idt821034).gpio_chip,
        idt821034 as *mut core::ffi::c_void,
    )
}

unsafe fn idt821034_spi_probe(spi: *mut spi_device) -> i32 {
    let mut idt821034: *mut idt821034;
    let mut ret: i32;

    (*spi).bits_per_word = 8;
    ret = spi_setup(spi);
    if ret < 0 {
        return ret;
    }

    idt821034 = devm_kzalloc(
        &mut (*spi).dev,
        core::mem::size_of::<idt821034>(),
        GFP_KERNEL,
    ) as *mut idt821034;
    if idt821034.is_null() {
        return -ENOMEM;
    }

    (*idt821034).spi = spi;

    mutex_init(&mut (*idt821034).mutex);

    spi_set_drvdata(spi, idt821034 as *mut core::ffi::c_void);

    ret = devm_snd_soc_register_component(
        &mut (*spi).dev,
        &mut idt821034_component_driver,
        &mut idt821034_dai_driver,
        1,
    );
    if ret != 0 {
        return ret;
    }

    // C conditional: if (IS_ENABLED(CONFIG_GPIOLIB))
    if IS_ENABLED!(CONFIG_GPIOLIB) {
        return idt821034_gpio_init(idt821034);
    }

    0
}

static_of_device_id_array!(
    idt821034_of_match,
    of_device_id { compatible: c_str!("renesas,idt821034"), ..unsafe { core::mem::zeroed() } },
    of_device_id { ..unsafe { core::mem::zeroed() } },
);
MODULE_DEVICE_TABLE!(of, idt821034_of_match);

static_spi_device_id_array!(
    idt821034_id_table,
    spi_device_id { name: c_str!("idt821034"), driver_data: 0 },
    spi_device_id { ..unsafe { core::mem::zeroed() } },
);
MODULE_DEVICE_TABLE!(spi, idt821034_id_table);

static mut idt821034_spi_driver: spi_driver = spi_driver {
    driver: device_driver {
        name: c_str!("idt821034"),
        of_match_table: idt821034_of_match.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    id_table: idt821034_id_table.as_ptr(),
    probe: Some(idt821034_spi_probe),
    ..unsafe { core::mem::zeroed() }
};

module_spi_driver!(idt821034_spi_driver);

MODULE_AUTHOR!("Herve Codina <herve.codina@bootlin.com>");
MODULE_DESCRIPTION!("IDT821034 ALSA SoC driver");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
