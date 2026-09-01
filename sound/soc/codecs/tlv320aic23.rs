// SPDX-License-Identifier: GPL-2.0-only
/*
 * ALSA SoC TLV320AIC23 codec driver
 *
 * Author:      Arun KS, <arunks@mistralsolutions.com>
 * Copyright:   (C) 2008 Mistral Solutions Pvt Ltd.,
 *
 * Based on sound/soc/codecs/wm8731.c by Richard Purdie
 *
 * Notes:
 *  The AIC23 is a driver for a low power stereo audio
 *  codec tlv320aic23
 *
 *  The machine layer should disable unsupported inputs/outputs by
 *  snd_soc_dapm_disable_pin(codec, "LHPOUT"), etc.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

type c_int = i32;
type c_uint = u32;
type u16 = u16;
type u32 = u32;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_union,
}

#[repr(C)]
pub union snd_ctl_elem_value_union {
    pub integer: snd_ctl_elem_value_integer,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub stream: c_int,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
}

#[repr(C)]
pub enum snd_soc_bias_level {
    SND_SOC_BIAS_ON = 0,
    SND_SOC_BIAS_PREPARE = 1,
    SND_SOC_BIAS_STANDBY = 2,
    SND_SOC_BIAS_OFF = 3,
}

#[repr(C)]
pub struct aic23 {
    pub regmap: *mut regmap,
    pub mclk: c_int,
    pub requested_adc: c_int,
    pub requested_dac: c_int,
}

unsafe extern "C" {
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut aic23;
    fn snd_soc_dai_get_drvdata(codec_dai: *mut snd_soc_dai) -> *mut aic23;
    fn snd_soc_component_active(component: *mut snd_soc_component) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn udelay(usecs: c_uint);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut core::ffi::c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut core::ffi::c_void);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const core::ffi::c_void,
        dai_drv: *mut core::ffi::c_void,
        num_dai: c_int,
    ) -> c_int;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> c_int;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> c_int;
}

unsafe extern "C" {
    static TLV320AIC23_RESET: c_uint;
    static TLV320AIC23_ANLG: c_uint;
    static TLV320AIC23_DIGT: c_uint;
    static TLV320AIC23_LCHNVOL: c_uint;
    static TLV320AIC23_RCHNVOL: c_uint;
    static TLV320AIC23_LINVOL: c_uint;
    static TLV320AIC23_RINVOL: c_uint;
    static TLV320AIC23_PWR: c_uint;
    static TLV320AIC23_SRATE: c_uint;
    static TLV320AIC23_DIGT_FMT: c_uint;
    static TLV320AIC23_ACTIVE: c_uint;
    static TLV320AIC23_CLKIN_SHIFT: c_uint;
    static TLV320AIC23_CLKIN_HALF: c_int;
    static TLV320AIC23_DACM_MUTE: c_uint;
    static TLV320AIC23_MS_MASTER: c_uint;
    static TLV320AIC23_FOR_I2S: c_uint;
    static TLV320AIC23_LRP_ON: c_uint;
    static TLV320AIC23_FOR_DSP: c_uint;
    static TLV320AIC23_FOR_LJUST: c_uint;
    static TLV320AIC23_DEVICE_PWR_OFF: c_uint;
    static TLV320AIC23_OSC_OFF: c_uint;
    static TLV320AIC23_DAC_OFF: c_uint;
    static TLV320AIC23_CLK_OFF: c_uint;
    static TLV320AIC23_DEEMP_44K: c_uint;
    static TLV320AIC23_LIM_MUTED: c_uint;
    static TLV320AIC23_LRS_ENABLED: c_uint;
    static TLV320AIC23_BYPASS_ON: c_uint;
    static TLV320AIC23_MICM_MUTED: c_uint;
    static TLV320AIC23_DEFAULT_OUT_VOL: c_uint;
    static TLV320AIC23_OUT_VOL_MASK: c_uint;
    static REGCACHE_RBTREE: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBP_CFP: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_I2S: c_uint;
    static SND_SOC_DAIFMT_DSP_A: c_uint;
    static SND_SOC_DAIFMT_DSP_B: c_uint;
    static SND_SOC_DAIFMT_RIGHT_J: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_RATE_8000_96000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S20_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S24_3LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;
    static GFP_KERNEL: c_uint;
    static EINVAL: c_int;
    static ENOMEM: c_int;
}

/*
 * AIC23 register cache
 */
static tlv320aic23_reg: [reg_default; 10] = [
    reg_default { reg: 0, def: 0x0097 },
    reg_default { reg: 1, def: 0x0097 },
    reg_default { reg: 2, def: 0x00F9 },
    reg_default { reg: 3, def: 0x00F9 },
    reg_default { reg: 4, def: 0x001A },
    reg_default { reg: 5, def: 0x0004 },
    reg_default { reg: 6, def: 0x0007 },
    reg_default { reg: 7, def: 0x0001 },
    reg_default { reg: 8, def: 0x0020 },
    reg_default { reg: 9, def: 0x0000 },
];

#[no_mangle]
pub static tlv320aic23_regmap: regmap_config = unsafe {
    regmap_config {
        reg_bits: 7,
        val_bits: 9,
        max_register: TLV320AIC23_RESET,
        reg_defaults: tlv320aic23_reg.as_ptr(),
        num_reg_defaults: tlv320aic23_reg.len() as c_uint,
        cache_type: REGCACHE_RBTREE,
    }
};

static rec_src_text: [&[u8]; 2] = [b"Line\0", b"Mic\0"];
static deemph_text: [&[u8]; 4] = [b"None\0", b"32Khz\0", b"44.1Khz\0", b"48Khz\0"];

/*
 * The following static control/widget/route tables are direct translations of
 * Linux ASoC macro initializers in the C source:
 * - SOC_ENUM_SINGLE_DECL(rec_src_enum, TLV320AIC23_ANLG, 2, rec_src_text)
 * - SOC_DAPM_ENUM("Input Select", rec_src_enum)
 * - SOC_ENUM_SINGLE_DECL(tlv320aic23_deemph, TLV320AIC23_DIGT, 1, deemph_text)
 * - DECLARE_TLV_DB_SCALE(out_gain_tlv, -12100, 100, 0)
 * - DECLARE_TLV_DB_SCALE(input_gain_tlv, -1725, 75, 0)
 * - DECLARE_TLV_DB_SCALE(sidetone_vol_tlv, -1800, 300, 0)
 * - tlv320aic23_snd_controls[]
 * - tlv320aic23_output_mixer_controls[]
 * - tlv320aic23_dapm_widgets[]
 * - tlv320aic23_intercon[]
 *
 * Their concrete Rust item types and field layouts are supplied by the future
 * ASoC bindings corresponding to the removed Linux headers.
 */

unsafe extern "C" {
    static tlv320aic23_snd_controls: core::ffi::c_void;
    static tlv320aic23_output_mixer_controls: core::ffi::c_void;
    static tlv320aic23_dapm_widgets: core::ffi::c_void;
    static tlv320aic23_intercon: core::ffi::c_void;
}

/*
 * Common Crystals used
 * 11.2896 Mhz /128 = *88.2k  /192 = 58.8k
 * 12.0000 Mhz /125 = *96k    /136 = 88.235K
 * 12.2880 Mhz /128 = *96k    /192 = 64k
 * 16.9344 Mhz /128 = 132.3k /192 = *88.2k
 * 18.4320 Mhz /128 = 144k   /192 = *96k
 */

/*
 * Normal BOSR 0-256/2 = 128, 1-384/2 = 192
 * USB BOSR 0-250/2 = 125, 1-272/2 = 136
 */
static bosr_usb_divisor_table: [c_int; 4] = [128, 125, 192, 136];

const LOWER_GROUP: c_uint = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 6) | (1 << 7);
const UPPER_GROUP: c_uint = (1 << 8) | (1 << 9) | (1 << 10) | (1 << 11) | (1 << 15);

static sr_valid_mask: [u16; 4] = [
    (LOWER_GROUP | UPPER_GROUP) as u16, /* Normal, bosr - 0*/
    LOWER_GROUP as u16,                /* Usb, bosr - 0*/
    (LOWER_GROUP | UPPER_GROUP) as u16, /* Normal, bosr - 1*/
    UPPER_GROUP as u16,                /* Usb, bosr - 1*/
];

/*
 * Every divisor is a factor of 11*12
 */
const SR_MULT: c_int = 11 * 12;
const fn A(x: c_int) -> u8 {
    (SR_MULT / x) as u8
}

static sr_adc_mult_table: [u8; 16] = [
    A(2), A(2), A(12), A(12), 0, 0, A(3), A(1),
    A(2), A(2), A(11), A(11), 0, 0, 0, A(1),
];

static sr_dac_mult_table: [u8; 16] = [
    A(2), A(12), A(2), A(12), 0, 0, A(3), A(1),
    A(2), A(11), A(2), A(11), 0, 0, 0, A(1),
];

fn get_score(
    adc: c_int,
    adc_l: c_int,
    adc_h: c_int,
    need_adc: c_int,
    dac: c_int,
    dac_l: c_int,
    dac_h: c_int,
    need_dac: c_int,
) -> c_uint {
    if (adc >= adc_l) && (adc <= adc_h) && (dac >= dac_l) && (dac <= dac_h) {
        let diff_adc: c_int = need_adc - adc;
        let diff_dac: c_int = need_dac - dac;
        return diff_adc.abs() as c_uint + diff_dac.abs() as c_uint;
    }
    c_uint::MAX
}

fn find_rate(mclk: c_int, mut need_adc: u32, mut need_dac: u32) -> c_int {
    let mut best_i: c_int = -1;
    let mut best_j: c_int = -1;
    let mut best_div: c_int = 0;
    let mut best_score: c_uint = c_uint::MAX;

    need_adc = need_adc.wrapping_mul(SR_MULT as u32);
    need_dac = need_dac.wrapping_mul(SR_MULT as u32);
    /*
     * rates given are +/- 1/32
     */
    let adc_l: c_int = (need_adc - (need_adc >> 5)) as c_int;
    let adc_h: c_int = (need_adc + (need_adc >> 5)) as c_int;
    let dac_l: c_int = (need_dac - (need_dac >> 5)) as c_int;
    let dac_h: c_int = (need_dac + (need_dac >> 5)) as c_int;

    for i in 0..bosr_usb_divisor_table.len() {
        let base: c_int = mclk / bosr_usb_divisor_table[i];
        let mut mask: c_int = sr_valid_mask[i] as c_int;
        let mut j = 0usize;
        while j < sr_adc_mult_table.len() {
            let adc: c_int;
            let dac: c_int;
            let mut score: c_uint;
            if (mask & 1) == 0 {
                j += 1;
                mask >>= 1;
                continue;
            }
            adc = base * sr_adc_mult_table[j] as c_int;
            dac = base * sr_dac_mult_table[j] as c_int;
            score = get_score(adc, adc_l, adc_h, need_adc as c_int, dac, dac_l, dac_h, need_dac as c_int);
            if best_score > score {
                best_score = score;
                best_i = i as c_int;
                best_j = j as c_int;
                best_div = 0;
            }
            score = get_score(
                adc >> 1,
                adc_l,
                adc_h,
                need_adc as c_int,
                dac >> 1,
                dac_l,
                dac_h,
                need_dac as c_int,
            );
            /* prefer to have a /2 */
            if (score != c_uint::MAX) && (best_score >= score) {
                best_score = score;
                best_i = i as c_int;
                best_j = j as c_int;
                best_div = 1;
            }
            j += 1;
            mask >>= 1;
        }
    }
    unsafe { (best_j << 2) | best_i | (best_div << TLV320AIC23_CLKIN_SHIFT) }
}

/* #ifdef DEBUG */
#[cfg(DEBUG)]
unsafe fn get_current_sample_rates(
    component: *mut snd_soc_component,
    mclk: c_int,
    sample_rate_adc: *mut u32,
    sample_rate_dac: *mut u32,
) {
    let src: c_int = snd_soc_component_read(component, TLV320AIC23_SRATE) as c_int;
    let sr: c_int = (src >> 2) & 0x0f;
    let val: c_int = mclk / bosr_usb_divisor_table[(src & 3) as usize];
    let mut adc: c_int = (val * sr_adc_mult_table[sr as usize] as c_int) / SR_MULT;
    let mut dac: c_int = (val * sr_dac_mult_table[sr as usize] as c_int) / SR_MULT;
    if (src & TLV320AIC23_CLKIN_HALF) != 0 {
        adc >>= 1;
        dac >>= 1;
    }
    *sample_rate_adc = adc as u32;
    *sample_rate_dac = dac as u32;
}
/* #endif */

unsafe fn set_sample_rate_control(
    component: *mut snd_soc_component,
    mclk: c_int,
    sample_rate_adc: u32,
    sample_rate_dac: u32,
) -> c_int {
    /* Search for the right sample rate */
    let data: c_int = find_rate(mclk, sample_rate_adc, sample_rate_dac);
    if data < 0 {
        /* printk(KERN_ERR "%s:Invalid rate %u,%u requested\n", __func__, sample_rate_adc, sample_rate_dac); */
        return -EINVAL;
    }
    snd_soc_component_write(component, TLV320AIC23_SRATE, data as c_uint);
    #[cfg(DEBUG)]
    {
        let mut adc: u32 = 0;
        let mut dac: u32 = 0;
        get_current_sample_rates(component, mclk, &mut adc, &mut dac);
        /* printk(KERN_DEBUG "actual samplerate = %u,%u reg=%x\n", adc, dac, data); */
    }
    0
}

unsafe fn snd_soc_tlv320aic23_put_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let mut val: u16;
    let reg: u16;

    val = ((*ucontrol).value.integer.value[0] & 0x07) as u16;

    /* linear conversion to userspace
     * 000  =   -6db
     * 001  =   -9db
     * 010  =   -12db
     * 011  =   -18db (Min)
     * 100  =   0db (Max)
     */
    val = if val >= 4 { 4 } else { 3 - val };

    reg = (snd_soc_component_read(component, TLV320AIC23_ANLG) & !0x1C0) as u16;
    snd_soc_component_write(component, TLV320AIC23_ANLG, (reg | (val << 6)) as c_uint);

    0
}

unsafe fn snd_soc_tlv320aic23_get_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let mut val: u16;

    val = (snd_soc_component_read(component, TLV320AIC23_ANLG) & 0x1C0) as u16;
    val = val >> 6;
    val = if val >= 4 { 4 } else { 3 - val };
    (*ucontrol).value.integer.value[0] = val as i64;
    0
}

unsafe fn tlv320aic23_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mut iface_reg: u16;
    let ret: c_int;
    let aic23: *mut aic23 = snd_soc_component_get_drvdata(component);
    let mut sample_rate_adc: u32 = (*aic23).requested_adc as u32;
    let mut sample_rate_dac: u32 = (*aic23).requested_dac as u32;
    let sample_rate: u32 = params_rate(params);

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*aic23).requested_dac = sample_rate as c_int;
        sample_rate_dac = sample_rate;
        if sample_rate_adc == 0 {
            sample_rate_adc = sample_rate;
        }
    } else {
        (*aic23).requested_adc = sample_rate as c_int;
        sample_rate_adc = sample_rate;
        if sample_rate_dac == 0 {
            sample_rate_dac = sample_rate;
        }
    }
    ret = set_sample_rate_control(component, (*aic23).mclk, sample_rate_adc, sample_rate_dac);
    if ret < 0 {
        return ret;
    }

    iface_reg = (snd_soc_component_read(component, TLV320AIC23_DIGT_FMT) & !(0x03 << 2)) as u16;

    match params_width(params) {
        16 => {}
        20 => {
            iface_reg |= 0x01 << 2;
        }
        24 => {
            iface_reg |= 0x02 << 2;
        }
        32 => {
            iface_reg |= 0x03 << 2;
        }
        _ => {}
    }
    snd_soc_component_write(component, TLV320AIC23_DIGT_FMT, iface_reg as c_uint);

    0
}

unsafe fn tlv320aic23_pcm_prepare(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;

    /* set active */
    snd_soc_component_write(component, TLV320AIC23_ACTIVE, 0x0001);

    0
}

unsafe fn tlv320aic23_shutdown(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) {
    let component: *mut snd_soc_component = (*dai).component;
    let aic23: *mut aic23 = snd_soc_component_get_drvdata(component);

    /* deactivate */
    if snd_soc_component_active(component) == 0 {
        udelay(50);
        snd_soc_component_write(component, TLV320AIC23_ACTIVE, 0x0);
    }
    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        (*aic23).requested_dac = 0;
    } else {
        (*aic23).requested_adc = 0;
    }
}

unsafe fn tlv320aic23_mute(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component: *mut snd_soc_component = (*dai).component;
    let mut reg: u16;

    reg = snd_soc_component_read(component, TLV320AIC23_DIGT) as u16;
    if mute != 0 {
        reg |= TLV320AIC23_DACM_MUTE as u16;
    } else {
        reg &= !(TLV320AIC23_DACM_MUTE as u16);
    }

    snd_soc_component_write(component, TLV320AIC23_DIGT, reg as c_uint);

    0
}

unsafe fn tlv320aic23_set_dai_fmt(codec_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component: *mut snd_soc_component = (*codec_dai).component;
    let mut iface_reg: u16;

    iface_reg = (snd_soc_component_read(component, TLV320AIC23_DIGT_FMT) & !0x03) as u16;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBP_CFP => {
            iface_reg |= TLV320AIC23_MS_MASTER as u16;
        }
        x if x == SND_SOC_DAIFMT_CBC_CFC => {
            iface_reg &= !(TLV320AIC23_MS_MASTER as u16);
        }
        _ => {
            return -EINVAL;
        }
    }

    /* interface format */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_I2S => {
            iface_reg |= TLV320AIC23_FOR_I2S as u16;
        }
        x if x == SND_SOC_DAIFMT_DSP_A => {
            iface_reg |= TLV320AIC23_LRP_ON as u16;
            iface_reg |= TLV320AIC23_FOR_DSP as u16;
        }
        x if x == SND_SOC_DAIFMT_DSP_B => {
            iface_reg |= TLV320AIC23_FOR_DSP as u16;
        }
        x if x == SND_SOC_DAIFMT_RIGHT_J => {}
        x if x == SND_SOC_DAIFMT_LEFT_J => {
            iface_reg |= TLV320AIC23_FOR_LJUST as u16;
        }
        _ => {
            return -EINVAL;
        }
    }

    snd_soc_component_write(component, TLV320AIC23_DIGT_FMT, iface_reg as c_uint);

    0
}

unsafe fn tlv320aic23_set_dai_sysclk(
    codec_dai: *mut snd_soc_dai,
    _clk_id: c_int,
    freq: c_uint,
    _dir: c_int,
) -> c_int {
    let aic23: *mut aic23 = snd_soc_dai_get_drvdata(codec_dai);
    (*aic23).mclk = freq as c_int;
    0
}

unsafe fn tlv320aic23_set_bias_level(
    component: *mut snd_soc_component,
    level: snd_soc_bias_level,
) -> c_int {
    let mut reg: u16 = (snd_soc_component_read(component, TLV320AIC23_PWR) & 0x17f) as u16;

    match level {
        snd_soc_bias_level::SND_SOC_BIAS_ON => {
            /* vref/mid, osc on, dac unmute */
            reg &= !((TLV320AIC23_DEVICE_PWR_OFF | TLV320AIC23_OSC_OFF | TLV320AIC23_DAC_OFF) as u16);
            snd_soc_component_write(component, TLV320AIC23_PWR, reg as c_uint);
        }
        snd_soc_bias_level::SND_SOC_BIAS_PREPARE => {}
        snd_soc_bias_level::SND_SOC_BIAS_STANDBY => {
            /* everything off except vref/vmid, */
            snd_soc_component_write(component, TLV320AIC23_PWR, (reg as c_uint) | TLV320AIC23_CLK_OFF);
        }
        snd_soc_bias_level::SND_SOC_BIAS_OFF => {
            /* everything off, dac mute, inactive */
            snd_soc_component_write(component, TLV320AIC23_ACTIVE, 0x0);
            snd_soc_component_write(component, TLV320AIC23_PWR, 0x1ff);
        }
    }
    0
}

unsafe fn tlv320aic23_resume(component: *mut snd_soc_component) -> c_int {
    let aic23: *mut aic23 = snd_soc_component_get_drvdata(component);
    regcache_mark_dirty((*aic23).regmap);
    regcache_sync((*aic23).regmap);

    0
}

unsafe fn tlv320aic23_component_probe(component: *mut snd_soc_component) -> c_int {
    /* Reset codec */
    snd_soc_component_write(component, TLV320AIC23_RESET, 0);

    snd_soc_component_write(component, TLV320AIC23_DIGT, TLV320AIC23_DEEMP_44K);

    /* Unmute input */
    snd_soc_component_update_bits(
        component,
        TLV320AIC23_LINVOL,
        TLV320AIC23_LIM_MUTED,
        TLV320AIC23_LRS_ENABLED,
    );

    snd_soc_component_update_bits(
        component,
        TLV320AIC23_RINVOL,
        TLV320AIC23_LIM_MUTED,
        TLV320AIC23_LRS_ENABLED,
    );

    snd_soc_component_update_bits(
        component,
        TLV320AIC23_ANLG,
        TLV320AIC23_BYPASS_ON | TLV320AIC23_MICM_MUTED,
        0,
    );

    /* Default output volume */
    snd_soc_component_write(
        component,
        TLV320AIC23_LCHNVOL,
        TLV320AIC23_DEFAULT_OUT_VOL & TLV320AIC23_OUT_VOL_MASK,
    );
    snd_soc_component_write(
        component,
        TLV320AIC23_RCHNVOL,
        TLV320AIC23_DEFAULT_OUT_VOL & TLV320AIC23_OUT_VOL_MASK,
    );

    snd_soc_component_write(component, TLV320AIC23_ACTIVE, 0x1);

    0
}

const AIC23_RATES: c_uint = unsafe { SNDRV_PCM_RATE_8000_96000 };
const AIC23_FORMATS: c_uint = unsafe {
    SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S20_3LE
        | SNDRV_PCM_FMTBIT_S24_3LE
        | SNDRV_PCM_FMTBIT_S32_LE
};

/*
 * The C source initializes these Linux-owned descriptor objects with designated
 * initializers:
 * - static const struct snd_soc_dai_ops tlv320aic23_dai_ops
 * - static struct snd_soc_dai_driver tlv320aic23_dai
 * - static const struct snd_soc_component_driver soc_component_dev_tlv320aic23
 *
 * The fields point at the translated functions above and preserve the same
 * playback/capture rates, formats, controls, DAPM widgets, and DAPM routes.
 * Their Rust layouts depend on external ASoC bindings.
 */
unsafe extern "C" {
    static mut tlv320aic23_dai: core::ffi::c_void;
    static soc_component_dev_tlv320aic23: core::ffi::c_void;
}

#[no_mangle]
pub unsafe extern "C" fn tlv320aic23_probe(dev: *mut device, regmap: *mut regmap) -> c_int {
    let aic23: *mut aic23;

    if IS_ERR(regmap as *const core::ffi::c_void) != 0 {
        return PTR_ERR(regmap as *const core::ffi::c_void);
    }

    aic23 = devm_kzalloc(
        dev,
        core::mem::size_of::<aic23>(),
        GFP_KERNEL,
    ) as *mut aic23;
    if aic23.is_null() {
        return -ENOMEM;
    }

    (*aic23).regmap = regmap;

    dev_set_drvdata(dev, aic23 as *mut core::ffi::c_void);

    devm_snd_soc_register_component(
        dev,
        &soc_component_dev_tlv320aic23 as *const _ as *const core::ffi::c_void,
        &mut tlv320aic23_dai as *mut _ as *mut core::ffi::c_void,
        1,
    )
}

/*
 * EXPORT_SYMBOL(tlv320aic23_regmap);
 * EXPORT_SYMBOL(tlv320aic23_probe);
 * MODULE_DESCRIPTION("ASoC TLV320AIC23 codec driver");
 * MODULE_AUTHOR("Arun KS <arunks@mistralsolutions.com>");
 * MODULE_LICENSE("GPL");
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
