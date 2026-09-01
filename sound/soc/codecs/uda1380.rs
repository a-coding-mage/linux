// SPDX-License-Identifier: GPL-2.0-only
/*
 * uda1380.rs - Philips UDA1380 ALSA SoC audio driver
 *
 * Copyright (c) 2007-2009 Philipp Zabel <philipp.zabel@gmail.com>
 *
 * Modified by Richard Purdie <richard@openedhand.com> to fit into SoC
 * codec model.
 *
 * Copyright (c) 2005 Giorgio Padrin <giorgio@mandarinlogiq.org>
 * Copyright 2005 Openedhand Ltd.
 */

// Depends on Linux kernel, ALSA SoC, TLV, I2C, GPIO, workqueue, and
// "uda1380.h" definitions supplied by the surrounding repository.

pub const UDA1380_DAC_CLK_SYSCLK: u32 = 0;
pub const UDA1380_DAC_CLK_WSPLL: u32 = 1;

/* codec private data */
#[repr(C)]
pub struct uda1380_priv {
    pub component: *mut snd_soc_component,
    pub dac_clk: libc::c_uint,
    pub work: work_struct,
    pub i2c: *mut i2c_client,
    pub power: *mut gpio_desc,
    pub reset: *mut gpio_desc,
    pub reg_cache: __IncompleteArrayField<u16>,
}

/*
 * uda1380 register cache
 */
static uda1380_reg: [u16; UDA1380_CACHEREGNUM as usize] = [
    0x0502, 0x0000, 0x0000, 0x3f3f,
    0x0202, 0x0000, 0x0000, 0x0000,
    0x0000, 0x0000, 0x0000, 0x0000,
    0x0000, 0x0000, 0x0000, 0x0000,
    0x0000, 0xff00, 0x0000, 0x4800,
    0x0000, 0x0000, 0x0000, 0x0000,
    0x0000, 0x0000, 0x0000, 0x0000,
    0x0000, 0x0000, 0x0000, 0x0000,
    0x0000, 0x8000, 0x0002, 0x0000,
];

static mut uda1380_cache_dirty: libc::c_ulong = 0;

/*
 * read uda1380 register cache
 */
unsafe fn uda1380_read_reg_cache(
    component: *mut snd_soc_component,
    reg: libc::c_uint,
) -> libc::c_uint {
    let uda1380 = snd_soc_component_get_drvdata(component) as *mut uda1380_priv;
    let cache = (*uda1380).reg_cache.as_mut_ptr();

    if reg == UDA1380_RESET {
        return 0;
    }
    if reg >= UDA1380_CACHEREGNUM {
        return -1i32 as libc::c_uint;
    }
    *cache.add(reg as usize) as libc::c_uint
}

/*
 * write uda1380 register cache
 */
unsafe fn uda1380_write_reg_cache(
    component: *mut snd_soc_component,
    reg: u16,
    value: libc::c_uint,
) {
    let uda1380 = snd_soc_component_get_drvdata(component) as *mut uda1380_priv;
    let cache = (*uda1380).reg_cache.as_mut_ptr();

    if (reg as libc::c_uint) >= UDA1380_CACHEREGNUM {
        return;
    }
    if reg >= 0x10 && (*cache.add(reg as usize) as libc::c_uint) != value {
        set_bit((reg - 0x10) as libc::c_int, &raw mut uda1380_cache_dirty);
    }
    *cache.add(reg as usize) = value as u16;
}

/*
 * write to the UDA1380 register space
 */
unsafe fn uda1380_write(
    component: *mut snd_soc_component,
    reg: libc::c_uint,
    value: libc::c_uint,
) -> libc::c_int {
    let uda1380 = snd_soc_component_get_drvdata(component) as *mut uda1380_priv;
    let mut data: [u8; 3] = [0; 3];
    let val: libc::c_uint;
    let mut ret: libc::c_int;

    /* data is
     *   data[0] is register offset
     *   data[1] is MS byte
     *   data[2] is LS byte
     */
    data[0] = reg as u8;
    data[1] = ((value & 0xff00) >> 8) as u8;
    data[2] = (value & 0x00ff) as u8;

    uda1380_write_reg_cache(component, reg as u16, value);

    /* the interpolator & decimator regs must only be written when the
     * codec DAI is active.
     */
    if snd_soc_component_active(component) == 0 && reg >= UDA1380_MVOL {
        return 0;
    }
    pr_debug!("uda1380: hw write %x val %x\n", reg, value);

    ret = i2c_master_send((*uda1380).i2c, data.as_mut_ptr(), 3);
    if ret != 3 {
        let err = if ret < 0 { ret } else { -EIO };
        dev_err!((*component).dev, "write failed: %pe\n", ERR_PTR(err));
        return err;
    }

    ret = i2c_master_send((*uda1380).i2c, data.as_mut_ptr(), 1);
    if ret != 1 {
        let err = if ret < 0 { ret } else { -EIO };
        dev_err!((*component).dev, "send address failed: %pe\n", ERR_PTR(err));
        return err;
    }

    ret = i2c_master_recv((*uda1380).i2c, data.as_mut_ptr(), 2);
    if ret != 2 {
        let err = if ret < 0 { ret } else { -EIO };
        dev_err!((*component).dev, "read failed: %pe\n", ERR_PTR(err));
        return err;
    }

    val = ((data[0] as libc::c_uint) << 8) | data[1] as libc::c_uint;
    if val != value {
        dev_err!(
            (*component).dev,
            "read back val %x (expected %x)\n",
            val,
            value
        );
        return -EIO;
    }

    if reg >= 0x10 {
        clear_bit((reg - 0x10) as libc::c_int, &raw mut uda1380_cache_dirty);
    }

    0
}

unsafe fn uda1380_sync_cache(component: *mut snd_soc_component) {
    let uda1380 = snd_soc_component_get_drvdata(component) as *mut uda1380_priv;
    let mut reg: libc::c_int;
    let mut data: [u8; 3] = [0; 3];
    let cache = (*uda1380).reg_cache.as_mut_ptr();

    /* Sync reg_cache with the hardware */
    reg = 0;
    while reg < UDA1380_MVOL as libc::c_int {
        data[0] = reg as u8;
        data[1] = ((*cache.add(reg as usize) & 0xff00) >> 8) as u8;
        data[2] = (*cache.add(reg as usize) & 0x00ff) as u8;
        if i2c_master_send((*uda1380).i2c, data.as_mut_ptr(), 3) != 3 {
            dev_err!(
                (*component).dev,
                "%s: write to reg 0x%x failed\n",
                __func__,
                reg
            );
        }
        reg += 1;
    }
}

unsafe fn uda1380_reset(component: *mut snd_soc_component) -> libc::c_int {
    let uda1380 = snd_soc_component_get_drvdata(component) as *mut uda1380_priv;

    if !(*uda1380).reset.is_null() {
        gpiod_set_value((*uda1380).reset, 1);
        mdelay(1);
        gpiod_set_value((*uda1380).reset, 0);
    } else {
        let mut data: [u8; 3] = [0; 3];

        data[0] = UDA1380_RESET as u8;
        data[1] = 0;
        data[2] = 0;

        if i2c_master_send((*uda1380).i2c, data.as_mut_ptr(), 3) != 3 {
            dev_err!((*component).dev, "%s: failed\n", __func__);
            return -EIO;
        }
    }

    0
}

unsafe fn uda1380_flush_work(work: *mut work_struct) {
    let uda1380 = container_of!(work, uda1380_priv, work);
    let uda1380_component = (*uda1380).component;
    let mut bit: libc::c_int = 0;
    let mut reg: libc::c_int;

    while bit < (UDA1380_CACHEREGNUM - 0x10) as libc::c_int {
        if test_bit(bit, &raw const uda1380_cache_dirty) != 0 {
            reg = 0x10 + bit;
            pr_debug!(
                "uda1380: flush reg %x val %x:\n",
                reg,
                uda1380_read_reg_cache(uda1380_component, reg as libc::c_uint)
            );
            uda1380_write(
                uda1380_component,
                reg as libc::c_uint,
                uda1380_read_reg_cache(uda1380_component, reg as libc::c_uint),
            );
            clear_bit(bit, &raw mut uda1380_cache_dirty);
        }
        bit += 1;
    }
}

/* declarations of ALSA reg_elem_REAL controls */
static uda1380_deemp: [&str; 5] = [
    "None",
    "32kHz",
    "44.1kHz",
    "48kHz",
    "96kHz",
];
static uda1380_input_sel: [&str; 4] = [
    "Line",
    "Mic + Line R",
    "Line L",
    "Mic",
];
static uda1380_output_sel: [&str; 2] = [
    "DAC",
    "Analog Mixer",
];
static uda1380_spf_mode: [&str; 4] = [
    "Flat",
    "Minimum1",
    "Minimum2",
    "Maximum",
];
static uda1380_capture_sel: [&str; 2] = [
    "ADC",
    "Digital Mixer",
];
static uda1380_sel_ns: [&str; 2] = [
    "3rd-order",
    "5th-order",
];
static uda1380_mix_control: [&str; 4] = [
    "off",
    "PCM only",
    "before sound processing",
    "after sound processing",
];
static uda1380_sdet_setting: [&str; 4] = [
    "3200",
    "4800",
    "9600",
    "19200",
];
static uda1380_os_setting: [&str; 3] = [
    "single-speed",
    "double-speed (no mixing)",
    "quad-speed (no mixing)",
];

static uda1380_deemp_enum: [soc_enum; 2] = [
    SOC_ENUM_SINGLE!(UDA1380_DEEMP, 8, ARRAY_SIZE!(uda1380_deemp), uda1380_deemp),
    SOC_ENUM_SINGLE!(UDA1380_DEEMP, 0, ARRAY_SIZE!(uda1380_deemp), uda1380_deemp),
];
SOC_ENUM_SINGLE_DECL!(uda1380_input_sel_enum, UDA1380_ADC, 2, uda1380_input_sel); /* SEL_MIC, SEL_LNA */
SOC_ENUM_SINGLE_DECL!(uda1380_output_sel_enum, UDA1380_PM, 7, uda1380_output_sel); /* R02_EN_AVC */
SOC_ENUM_SINGLE_DECL!(uda1380_spf_enum, UDA1380_MODE, 14, uda1380_spf_mode); /* M */
SOC_ENUM_SINGLE_DECL!(uda1380_capture_sel_enum, UDA1380_IFACE, 6, uda1380_capture_sel); /* SEL_SOURCE */
SOC_ENUM_SINGLE_DECL!(uda1380_sel_ns_enum, UDA1380_MIXER, 14, uda1380_sel_ns); /* SEL_NS */
SOC_ENUM_SINGLE_DECL!(uda1380_mix_enum, UDA1380_MIXER, 12, uda1380_mix_control); /* MIX, MIX_POS */
SOC_ENUM_SINGLE_DECL!(uda1380_sdet_enum, UDA1380_MIXER, 4, uda1380_sdet_setting); /* SD_VALUE */
SOC_ENUM_SINGLE_DECL!(uda1380_os_enum, UDA1380_MIXER, 0, uda1380_os_setting); /* OS */

/*
 * from -48 dB in 1.5 dB steps (mute instead of -49.5 dB)
 */
DECLARE_TLV_DB_SCALE!(amix_tlv, -4950, 150, 1);

/*
 * from -78 dB in 1 dB steps (3 dB steps, really. LSB are ignored),
 * from -66 dB in 0.5 dB steps (2 dB steps, really) and
 * from -52 dB in 0.25 dB steps
 */
static mvol_tlv: DECLARE_TLV_DB_RANGE!() = DECLARE_TLV_DB_RANGE!(
    0, 15, TLV_DB_SCALE_ITEM!(-8200, 100, 1),
    16, 43, TLV_DB_SCALE_ITEM!(-6600, 50, 0),
    44, 252, TLV_DB_SCALE_ITEM!(-5200, 25, 0)
);

/*
 * from -72 dB in 1.5 dB steps (6 dB steps really),
 * from -66 dB in 0.75 dB steps (3 dB steps really),
 * from -60 dB in 0.5 dB steps (2 dB steps really) and
 * from -46 dB in 0.25 dB steps
 */
static vc_tlv: DECLARE_TLV_DB_RANGE!() = DECLARE_TLV_DB_RANGE!(
    0, 7, TLV_DB_SCALE_ITEM!(-7800, 150, 1),
    8, 15, TLV_DB_SCALE_ITEM!(-6600, 75, 0),
    16, 43, TLV_DB_SCALE_ITEM!(-6000, 50, 0),
    44, 228, TLV_DB_SCALE_ITEM!(-4600, 25, 0)
);

/* from 0 to 6 dB in 2 dB steps if SPF mode != flat */
DECLARE_TLV_DB_SCALE!(tr_tlv, 0, 200, 0);

/* from 0 to 24 dB in 2 dB steps, if SPF mode == maximum, otherwise cuts
 * off at 18 dB max) */
DECLARE_TLV_DB_SCALE!(bb_tlv, 0, 200, 0);

/* from -63 to 24 dB in 0.5 dB steps (-128...48) */
DECLARE_TLV_DB_SCALE!(dec_tlv, -6400, 50, 1);

/* from 0 to 24 dB in 3 dB steps */
DECLARE_TLV_DB_SCALE!(pga_tlv, 0, 300, 0);

/* from 0 to 30 dB in 2 dB steps */
DECLARE_TLV_DB_SCALE!(vga_tlv, 0, 200, 0);

static uda1380_snd_controls: [snd_kcontrol_new; 28] = [
    SOC_DOUBLE_TLV!("Analog Mixer Volume", UDA1380_AMIX, 0, 8, 44, 1, amix_tlv), /* AVCR, AVCL */
    SOC_DOUBLE_TLV!("Master Playback Volume", UDA1380_MVOL, 0, 8, 252, 1, mvol_tlv), /* MVCL, MVCR */
    SOC_SINGLE_TLV!("ADC Playback Volume", UDA1380_MIXVOL, 8, 228, 1, vc_tlv), /* VC2 */
    SOC_SINGLE_TLV!("PCM Playback Volume", UDA1380_MIXVOL, 0, 228, 1, vc_tlv), /* VC1 */
    SOC_ENUM!("Sound Processing Filter", uda1380_spf_enum), /* M */
    SOC_DOUBLE_TLV!("Tone Control - Treble", UDA1380_MODE, 4, 12, 3, 0, tr_tlv), /* TRL, TRR */
    SOC_DOUBLE_TLV!("Tone Control - Bass", UDA1380_MODE, 0, 8, 15, 0, bb_tlv), /* BBL, BBR */
    /**/ SOC_SINGLE!("Master Playback Switch", UDA1380_DEEMP, 14, 1, 1), /* MTM */
    SOC_SINGLE!("ADC Playback Switch", UDA1380_DEEMP, 11, 1, 1), /* MT2 from decimation filter */
    SOC_ENUM!("ADC Playback De-emphasis", uda1380_deemp_enum[0]), /* DE2 */
    SOC_SINGLE!("PCM Playback Switch", UDA1380_DEEMP, 3, 1, 1), /* MT1, from digital data input */
    SOC_ENUM!("PCM Playback De-emphasis", uda1380_deemp_enum[1]), /* DE1 */
    SOC_SINGLE!("DAC Polarity inverting Switch", UDA1380_MIXER, 15, 1, 0), /* DA_POL_INV */
    SOC_ENUM!("Noise Shaper", uda1380_sel_ns_enum), /* SEL_NS */
    SOC_ENUM!("Digital Mixer Signal Control", uda1380_mix_enum), /* MIX_POS, MIX */
    SOC_SINGLE!("Silence Detector Switch", UDA1380_MIXER, 6, 1, 0), /* SDET_ON */
    SOC_ENUM!("Silence Detector Setting", uda1380_sdet_enum), /* SD_VALUE */
    SOC_ENUM!("Oversampling Input", uda1380_os_enum), /* OS */
    SOC_DOUBLE_S8_TLV!("ADC Capture Volume", UDA1380_DEC, -128, 48, dec_tlv), /* ML_DEC, MR_DEC */
    /**/ SOC_SINGLE!("ADC Capture Switch", UDA1380_PGA, 15, 1, 1), /* MT_ADC */
    SOC_DOUBLE_TLV!("Line Capture Volume", UDA1380_PGA, 0, 8, 8, 0, pga_tlv), /* PGA_GAINCTRLL, PGA_GAINCTRLR */
    SOC_SINGLE!("ADC Polarity inverting Switch", UDA1380_ADC, 12, 1, 0), /* ADCPOL_INV */
    SOC_SINGLE_TLV!("Mic Capture Volume", UDA1380_ADC, 8, 15, 0, vga_tlv), /* VGA_CTRL */
    SOC_SINGLE!("DC Filter Bypass Switch", UDA1380_ADC, 1, 1, 0), /* SKIP_DCFIL (before decimator) */
    SOC_SINGLE!("DC Filter Enable Switch", UDA1380_ADC, 0, 1, 0), /* EN_DCFIL (at output of decimator) */
    SOC_SINGLE!("AGC Timing", UDA1380_AGC, 8, 7, 0), /* TODO: enum, see table 62 */
    SOC_SINGLE!("AGC Target level", UDA1380_AGC, 2, 3, 1), /* AGC_LEVEL */
    /* -5.5, -8, -11.5, -14 dBFS */
    SOC_SINGLE!("AGC Switch", UDA1380_AGC, 0, 1, 0),
];

/* Input mux */
static uda1380_input_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", uda1380_input_sel_enum);

/* Output mux */
static uda1380_output_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", uda1380_output_sel_enum);

/* Capture mux */
static uda1380_capture_mux_control: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Route", uda1380_capture_sel_enum);

static uda1380_dapm_widgets: [snd_soc_dapm_widget; 18] = [
    SND_SOC_DAPM_MUX!("Input Mux", SND_SOC_NOPM, 0, 0, &uda1380_input_mux_control),
    SND_SOC_DAPM_MUX!("Output Mux", SND_SOC_NOPM, 0, 0, &uda1380_output_mux_control),
    SND_SOC_DAPM_MUX!("Capture Mux", SND_SOC_NOPM, 0, 0, &uda1380_capture_mux_control),
    SND_SOC_DAPM_PGA!("Left PGA", UDA1380_PM, 3, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Right PGA", UDA1380_PM, 1, 0, NULL, 0),
    SND_SOC_DAPM_PGA!("Mic LNA", UDA1380_PM, 4, 0, NULL, 0),
    SND_SOC_DAPM_ADC!("Left ADC", "Left Capture", UDA1380_PM, 2, 0),
    SND_SOC_DAPM_ADC!("Right ADC", "Right Capture", UDA1380_PM, 0, 0),
    SND_SOC_DAPM_INPUT!("VINM"),
    SND_SOC_DAPM_INPUT!("VINL"),
    SND_SOC_DAPM_INPUT!("VINR"),
    SND_SOC_DAPM_MIXER!("Analog Mixer", UDA1380_PM, 6, 0, NULL, 0),
    SND_SOC_DAPM_OUTPUT!("VOUTLHP"),
    SND_SOC_DAPM_OUTPUT!("VOUTRHP"),
    SND_SOC_DAPM_OUTPUT!("VOUTL"),
    SND_SOC_DAPM_OUTPUT!("VOUTR"),
    SND_SOC_DAPM_DAC!("DAC", "Playback", UDA1380_PM, 10, 0),
    SND_SOC_DAPM_PGA!("HeadPhone Driver", UDA1380_PM, 13, 0, NULL, 0),
];

static uda1380_dapm_routes: [snd_soc_dapm_route; 24] = [
    /* output mux */
    snd_soc_dapm_route { sink: "HeadPhone Driver", control: NULL, source: "Output Mux" },
    snd_soc_dapm_route { sink: "VOUTR", control: NULL, source: "Output Mux" },
    snd_soc_dapm_route { sink: "VOUTL", control: NULL, source: "Output Mux" },

    snd_soc_dapm_route { sink: "Analog Mixer", control: NULL, source: "VINR" },
    snd_soc_dapm_route { sink: "Analog Mixer", control: NULL, source: "VINL" },
    snd_soc_dapm_route { sink: "Analog Mixer", control: NULL, source: "DAC" },

    snd_soc_dapm_route { sink: "Output Mux", control: "DAC", source: "DAC" },
    snd_soc_dapm_route { sink: "Output Mux", control: "Analog Mixer", source: "Analog Mixer" },

    /* {"DAC", "Digital Mixer", "I2S" } */

    /* headphone driver */
    snd_soc_dapm_route { sink: "VOUTLHP", control: NULL, source: "HeadPhone Driver" },
    snd_soc_dapm_route { sink: "VOUTRHP", control: NULL, source: "HeadPhone Driver" },

    /* input mux */
    snd_soc_dapm_route { sink: "Left ADC", control: NULL, source: "Input Mux" },
    snd_soc_dapm_route { sink: "Input Mux", control: "Mic", source: "Mic LNA" },
    snd_soc_dapm_route { sink: "Input Mux", control: "Mic + Line R", source: "Mic LNA" },
    snd_soc_dapm_route { sink: "Input Mux", control: "Line L", source: "Left PGA" },
    snd_soc_dapm_route { sink: "Input Mux", control: "Line", source: "Left PGA" },

    /* right input */
    snd_soc_dapm_route { sink: "Right ADC", control: "Mic + Line R", source: "Right PGA" },
    snd_soc_dapm_route { sink: "Right ADC", control: "Line", source: "Right PGA" },

    /* inputs */
    snd_soc_dapm_route { sink: "Mic LNA", control: NULL, source: "VINM" },
    snd_soc_dapm_route { sink: "Left PGA", control: NULL, source: "VINL" },
    snd_soc_dapm_route { sink: "Right PGA", control: NULL, source: "VINR" },
];

unsafe fn uda1380_set_dai_fmt_both(
    codec_dai: *mut snd_soc_dai,
    fmt: libc::c_uint,
) -> libc::c_int {
    let component = (*codec_dai).component;
    let mut iface: libc::c_int;

    /* set up DAI based upon fmt */
    iface = uda1380_read_reg_cache(component, UDA1380_IFACE) as libc::c_int;
    iface &= !(R01_SFORI_MASK | R01_SIM | R01_SFORO_MASK);

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => iface |= R01_SFORI_I2S | R01_SFORO_I2S,
        SND_SOC_DAIFMT_LSB => iface |= R01_SFORI_LSB16 | R01_SFORO_LSB16,
        SND_SOC_DAIFMT_MSB => iface |= R01_SFORI_MSB | R01_SFORO_MSB,
        _ => {}
    }

    /* DATAI is consumer only */
    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC {
        return -EINVAL;
    }

    uda1380_write_reg_cache(component, UDA1380_IFACE as u16, iface as libc::c_uint);

    0
}

unsafe fn uda1380_set_dai_fmt_playback(
    codec_dai: *mut snd_soc_dai,
    fmt: libc::c_uint,
) -> libc::c_int {
    let component = (*codec_dai).component;
    let mut iface: libc::c_int;

    /* set up DAI based upon fmt */
    iface = uda1380_read_reg_cache(component, UDA1380_IFACE) as libc::c_int;
    iface &= !R01_SFORI_MASK;

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => iface |= R01_SFORI_I2S,
        SND_SOC_DAIFMT_LSB => iface |= R01_SFORI_LSB16,
        SND_SOC_DAIFMT_MSB => iface |= R01_SFORI_MSB,
        _ => {}
    }

    /* DATAI is consumer only */
    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) != SND_SOC_DAIFMT_CBC_CFC {
        return -EINVAL;
    }

    uda1380_write(component, UDA1380_IFACE, iface as libc::c_uint);

    0
}

unsafe fn uda1380_set_dai_fmt_capture(
    codec_dai: *mut snd_soc_dai,
    fmt: libc::c_uint,
) -> libc::c_int {
    let component = (*codec_dai).component;
    let mut iface: libc::c_int;

    /* set up DAI based upon fmt */
    iface = uda1380_read_reg_cache(component, UDA1380_IFACE) as libc::c_int;
    iface &= !(R01_SIM | R01_SFORO_MASK);

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => iface |= R01_SFORO_I2S,
        SND_SOC_DAIFMT_LSB => iface |= R01_SFORO_LSB16,
        SND_SOC_DAIFMT_MSB => iface |= R01_SFORO_MSB,
        _ => {}
    }

    if (fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK) == SND_SOC_DAIFMT_CBP_CFP {
        iface |= R01_SIM;
    }

    uda1380_write(component, UDA1380_IFACE, iface as libc::c_uint);

    0
}

unsafe fn uda1380_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: libc::c_int,
    dai: *mut snd_soc_dai,
) -> libc::c_int {
    let component = (*dai).component;
    let uda1380 = snd_soc_component_get_drvdata(component) as *mut uda1380_priv;
    let mixer = uda1380_read_reg_cache(component, UDA1380_MIXER) as libc::c_int;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            uda1380_write_reg_cache(
                component,
                UDA1380_MIXER as u16,
                (mixer & !R14_SILENCE) as libc::c_uint,
            );
            schedule_work(&raw mut (*uda1380).work);
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            uda1380_write_reg_cache(
                component,
                UDA1380_MIXER as u16,
                (mixer | R14_SILENCE) as libc::c_uint,
            );
            schedule_work(&raw mut (*uda1380).work);
        }
        _ => {}
    }
    0
}

unsafe fn uda1380_pcm_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> libc::c_int {
    let component = (*dai).component;
    let mut clk = uda1380_read_reg_cache(component, UDA1380_CLK) as u16;

    /* set WSPLL power and divider if running from this clock */
    if (clk & R00_DAC_CLK as u16) != 0 {
        let rate = params_rate(params);
        let pm = uda1380_read_reg_cache(component, UDA1380_PM) as u16;
        clk &= !0x3; /* clear SEL_LOOP_DIV */
        match rate {
            6250..=12500 => clk |= 0x0,
            12501..=25000 => clk |= 0x1,
            25001..=50000 => clk |= 0x2,
            50001..=100000 => clk |= 0x3,
            _ => {}
        }
        uda1380_write(component, UDA1380_PM, (R02_PON_PLL as u16 | pm) as libc::c_uint);
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        clk |= (R00_EN_DAC | R00_EN_INT) as u16;
    } else {
        clk |= (R00_EN_ADC | R00_EN_DEC) as u16;
    }

    uda1380_write(component, UDA1380_CLK, clk as libc::c_uint);
    0
}

unsafe fn uda1380_pcm_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let component = (*dai).component;
    let mut clk = uda1380_read_reg_cache(component, UDA1380_CLK) as u16;

    /* shut down WSPLL power if running from this clock */
    if (clk & R00_DAC_CLK as u16) != 0 {
        let pm = uda1380_read_reg_cache(component, UDA1380_PM) as u16;
        uda1380_write(component, UDA1380_PM, ((!R02_PON_PLL as u16) & pm) as libc::c_uint);
    }

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        clk &= !((R00_EN_DAC | R00_EN_INT) as u16);
    } else {
        clk &= !((R00_EN_ADC | R00_EN_DEC) as u16);
    }

    uda1380_write(component, UDA1380_CLK, clk as libc::c_uint);
}

unsafe fn uda1380_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> libc::c_int {
    let dapm = snd_soc_component_to_dapm(component);
    let uda1380 = snd_soc_component_get_drvdata(component) as *mut uda1380_priv;
    let pm = uda1380_read_reg_cache(component, UDA1380_PM) as libc::c_int;
    let mut reg: libc::c_int;

    match level {
        SND_SOC_BIAS_ON | SND_SOC_BIAS_PREPARE => {
            /* ADC, DAC on */
            uda1380_write(component, UDA1380_PM, (R02_PON_BIAS | pm) as libc::c_uint);
        }
        SND_SOC_BIAS_STANDBY => {
            if snd_soc_dapm_get_bias_level(dapm) == SND_SOC_BIAS_OFF {
                if !(*uda1380).power.is_null() {
                    gpiod_set_value((*uda1380).power, 1);
                    mdelay(1);
                    uda1380_reset(component);
                }

                uda1380_sync_cache(component);
            }
            uda1380_write(component, UDA1380_PM, 0x0);
        }
        SND_SOC_BIAS_OFF => {
            if (*uda1380).power.is_null() {
                return 0;
            }

            gpiod_set_value((*uda1380).power, 0);

            /* Mark mixer regs cache dirty to sync them with
             * codec regs on power on.
             */
            reg = UDA1380_MVOL as libc::c_int;
            while reg < UDA1380_CACHEREGNUM as libc::c_int {
                set_bit(reg - 0x10, &raw mut uda1380_cache_dirty);
                reg += 1;
            }
        }
        _ => {}
    }
    0
}

pub const UDA1380_RATES: libc::c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_11025
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_22050
    | SNDRV_PCM_RATE_44100
    | SNDRV_PCM_RATE_48000;

static uda1380_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(uda1380_pcm_hw_params),
    shutdown: Some(uda1380_pcm_shutdown),
    trigger: Some(uda1380_trigger),
    set_fmt: Some(uda1380_set_dai_fmt_both),
};

static uda1380_dai_ops_playback: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(uda1380_pcm_hw_params),
    shutdown: Some(uda1380_pcm_shutdown),
    trigger: Some(uda1380_trigger),
    set_fmt: Some(uda1380_set_dai_fmt_playback),
};

static uda1380_dai_ops_capture: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(uda1380_pcm_hw_params),
    shutdown: Some(uda1380_pcm_shutdown),
    trigger: Some(uda1380_trigger),
    set_fmt: Some(uda1380_set_dai_fmt_capture),
};

static mut uda1380_dai: [snd_soc_dai_driver; 3] = [
    snd_soc_dai_driver {
        name: "uda1380-hifi",
        playback: snd_soc_pcm_stream {
            stream_name: "Playback",
            channels_min: 1,
            channels_max: 2,
            rates: UDA1380_RATES,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        },
        capture: snd_soc_pcm_stream {
            stream_name: "Capture",
            channels_min: 1,
            channels_max: 2,
            rates: UDA1380_RATES,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        },
        ops: &uda1380_dai_ops,
    },
    snd_soc_dai_driver {
        /* playback only - dual interface */
        name: "uda1380-hifi-playback",
        playback: snd_soc_pcm_stream {
            stream_name: "Playback",
            channels_min: 1,
            channels_max: 2,
            rates: UDA1380_RATES,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        },
        ops: &uda1380_dai_ops_playback,
    },
    snd_soc_dai_driver {
        /* capture only - dual interface*/
        name: "uda1380-hifi-capture",
        capture: snd_soc_pcm_stream {
            stream_name: "Capture",
            channels_min: 1,
            channels_max: 2,
            rates: UDA1380_RATES,
            formats: SNDRV_PCM_FMTBIT_S16_LE,
        },
        ops: &uda1380_dai_ops_capture,
    },
];

unsafe fn uda1380_probe(component: *mut snd_soc_component) -> libc::c_int {
    let uda1380 = snd_soc_component_get_drvdata(component) as *mut uda1380_priv;
    let ret: libc::c_int;

    (*uda1380).component = component;

    if (*uda1380).power.is_null() {
        ret = uda1380_reset(component);
        if ret != 0 {
            return ret;
        }
    }

    INIT_WORK(&raw mut (*uda1380).work, uda1380_flush_work);

    /* set clock input */
    match (*uda1380).dac_clk {
        UDA1380_DAC_CLK_SYSCLK => {
            uda1380_write_reg_cache(component, UDA1380_CLK as u16, 0);
        }
        UDA1380_DAC_CLK_WSPLL => {
            uda1380_write_reg_cache(component, UDA1380_CLK as u16, R00_DAC_CLK as libc::c_uint);
        }
        _ => {}
    }

    0
}

static soc_component_dev_uda1380: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(uda1380_probe),
    read: Some(uda1380_read_reg_cache),
    write: Some(uda1380_write),
    set_bias_level: Some(uda1380_set_bias_level),
    controls: uda1380_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE!(uda1380_snd_controls),
    dapm_widgets: uda1380_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE!(uda1380_dapm_widgets),
    dapm_routes: uda1380_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE!(uda1380_dapm_routes),
    suspend_bias_off: 1,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

unsafe fn uda1380_i2c_probe(i2c: *mut i2c_client) -> libc::c_int {
    let dev = &raw mut (*i2c).dev;
    let uda1380: *mut uda1380_priv;
    let ret: libc::c_int;

    uda1380 = devm_kzalloc(
        &raw mut (*i2c).dev,
        struct_size!(uda1380_priv, reg_cache, ARRAY_SIZE!(uda1380_reg)),
        GFP_KERNEL,
    ) as *mut uda1380_priv;
    if uda1380.is_null() {
        return -ENOMEM;
    }

    memcpy(
        (*uda1380).reg_cache.as_mut_ptr() as *mut libc::c_void,
        uda1380_reg.as_ptr() as *const libc::c_void,
        ARRAY_SIZE!(uda1380_reg) * core::mem::size_of::<u16>(),
    );

    (*uda1380).reset = devm_gpiod_get_optional(dev, "reset", GPIOD_OUT_LOW);
    if IS_ERR((*uda1380).reset as *const libc::c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*uda1380).reset as *const libc::c_void),
            "error obtaining reset GPIO\n",
        );
    }
    gpiod_set_consumer_name((*uda1380).reset, "uda1380 reset");

    (*uda1380).power = devm_gpiod_get_optional(dev, "power", GPIOD_OUT_LOW);
    if IS_ERR((*uda1380).power as *const libc::c_void) {
        return dev_err_probe(
            dev,
            PTR_ERR((*uda1380).power as *const libc::c_void),
            "error obtaining power GPIO\n",
        );
    }
    gpiod_set_consumer_name((*uda1380).power, "uda1380 power");

    /* This is just some default */
    (*uda1380).dac_clk = UDA1380_DAC_CLK_SYSCLK;
    if device_property_match_string(dev, "dac-clk", "wspll") >= 0 {
        (*uda1380).dac_clk = UDA1380_DAC_CLK_WSPLL;
    }

    i2c_set_clientdata(i2c, uda1380 as *mut libc::c_void);
    (*uda1380).i2c = i2c;

    ret = devm_snd_soc_register_component(
        &raw mut (*i2c).dev,
        &soc_component_dev_uda1380,
        uda1380_dai.as_mut_ptr(),
        ARRAY_SIZE!(uda1380_dai),
    );
    ret
}

static uda1380_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: "uda1380" },
    i2c_device_id::default(),
];
MODULE_DEVICE_TABLE!(i2c, uda1380_i2c_id);

static uda1380_of_match: [of_device_id; 2] = [
    of_device_id { compatible: "nxp,uda1380" },
    of_device_id::default(),
];
MODULE_DEVICE_TABLE!(of, uda1380_of_match);

static mut uda1380_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: "uda1380-codec",
        of_match_table: uda1380_of_match.as_ptr(),
    },
    probe: Some(uda1380_i2c_probe),
    id_table: uda1380_i2c_id.as_ptr(),
};

module_i2c_driver!(uda1380_i2c_driver);

MODULE_AUTHOR!("Giorgio Padrin");
MODULE_DESCRIPTION!("Audio support for codec Philips UDA1380");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
