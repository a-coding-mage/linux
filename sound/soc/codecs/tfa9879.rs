// SPDX-License-Identifier: GPL-2.0+
//
// tfa9879.c  --  driver for NXP Semiconductors TFA9879
//
// Copyright (C) 2014 Axentia Technologies AB
// Author: Peter Rosin <peda@axentia.se>

// Translated from C. Kernel, ASoC, regmap, TLV, and tfa9879.h symbols are
// external dependencies supplied by the surrounding repository.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct soc_enum {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub idle_bias_on: c_uint,
    pub use_pmdown_time: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub mute_stream: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_int, c_int) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub no_capture_mute: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct i2c_client {
    pub dev: device,
}

#[repr(C)]
pub struct i2c_device_id {
    pub name: *const c_char,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct i2c_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut i2c_client) -> c_int>,
    pub id_table: *const i2c_device_id,
}

#[repr(C)]
struct tfa9879_priv {
    regmap: *mut regmap,
    lsb_justified: c_int,
}

unsafe extern "C" {
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn i2c_set_clientdata(i2c: *mut i2c_client, data: *mut c_void);
    fn devm_regmap_init_i2c(i2c: *mut i2c_client, config: *const regmap_config) -> *mut regmap;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

unsafe extern "C" fn tfa9879_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let tfa9879 = snd_soc_component_get_drvdata(component) as *mut tfa9879_priv;
    let fs: c_int;
    let mut i2s_set: c_int = 0;

    match params_rate(params) {
        8000 => fs = TFA9879_I2S_FS_8000 as c_int,
        11025 => fs = TFA9879_I2S_FS_11025 as c_int,
        12000 => fs = TFA9879_I2S_FS_12000 as c_int,
        16000 => fs = TFA9879_I2S_FS_16000 as c_int,
        22050 => fs = TFA9879_I2S_FS_22050 as c_int,
        24000 => fs = TFA9879_I2S_FS_24000 as c_int,
        32000 => fs = TFA9879_I2S_FS_32000 as c_int,
        44100 => fs = TFA9879_I2S_FS_44100 as c_int,
        48000 => fs = TFA9879_I2S_FS_48000 as c_int,
        64000 => fs = TFA9879_I2S_FS_64000 as c_int,
        88200 => fs = TFA9879_I2S_FS_88200 as c_int,
        96000 => fs = TFA9879_I2S_FS_96000 as c_int,
        _ => return -EINVAL,
    }

    match params_width(params) {
        16 => i2s_set = TFA9879_I2S_SET_LSB_J_16 as c_int,
        24 => i2s_set = TFA9879_I2S_SET_LSB_J_24 as c_int,
        _ => return -EINVAL,
    }

    if (*tfa9879).lsb_justified != 0 {
        snd_soc_component_update_bits(
            component,
            TFA9879_SERIAL_INTERFACE_1,
            TFA9879_I2S_SET_MASK,
            ((i2s_set as c_uint) << TFA9879_I2S_SET_SHIFT) as c_uint,
        );
    }

    snd_soc_component_update_bits(
        component,
        TFA9879_SERIAL_INTERFACE_1,
        TFA9879_I2S_FS_MASK,
        ((fs as c_uint) << TFA9879_I2S_FS_SHIFT) as c_uint,
    );
    0
}

unsafe extern "C" fn tfa9879_mute_stream(
    dai: *mut snd_soc_dai,
    mute: c_int,
    _direction: c_int,
) -> c_int {
    let component = (*dai).component;

    snd_soc_component_update_bits(
        component,
        TFA9879_MISC_CONTROL,
        TFA9879_S_MUTE_MASK,
        (((mute != 0) as c_uint) << TFA9879_S_MUTE_SHIFT) as c_uint,
    );

    0
}

unsafe extern "C" fn tfa9879_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    let component = (*dai).component;
    let tfa9879 = snd_soc_component_get_drvdata(component) as *mut tfa9879_priv;
    let i2s_set: c_int;
    let sck_pol: c_int;

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        SND_SOC_DAIFMT_NB_NF => sck_pol = TFA9879_SCK_POL_NORMAL as c_int,
        SND_SOC_DAIFMT_IB_NF => sck_pol = TFA9879_SCK_POL_INVERSE as c_int,
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S => {
            (*tfa9879).lsb_justified = 0;
            i2s_set = TFA9879_I2S_SET_I2S_24 as c_int;
        }
        SND_SOC_DAIFMT_LEFT_J => {
            (*tfa9879).lsb_justified = 0;
            i2s_set = TFA9879_I2S_SET_MSB_J_24 as c_int;
        }
        SND_SOC_DAIFMT_RIGHT_J => {
            (*tfa9879).lsb_justified = 1;
            i2s_set = TFA9879_I2S_SET_LSB_J_24 as c_int;
        }
        _ => return -EINVAL,
    }

    snd_soc_component_update_bits(
        component,
        TFA9879_SERIAL_INTERFACE_1,
        TFA9879_SCK_POL_MASK,
        ((sck_pol as c_uint) << TFA9879_SCK_POL_SHIFT) as c_uint,
    );
    snd_soc_component_update_bits(
        component,
        TFA9879_SERIAL_INTERFACE_1,
        TFA9879_I2S_SET_MASK,
        ((i2s_set as c_uint) << TFA9879_I2S_SET_SHIFT) as c_uint,
    );
    0
}

static tfa9879_regs: [reg_default; 21] = [
    reg_default { reg: TFA9879_DEVICE_CONTROL, def: 0x0000 }, /* 0x00 */
    reg_default { reg: TFA9879_SERIAL_INTERFACE_1, def: 0x0a18 }, /* 0x01 */
    reg_default { reg: TFA9879_PCM_IOM2_FORMAT_1, def: 0x0007 }, /* 0x02 */
    reg_default { reg: TFA9879_SERIAL_INTERFACE_2, def: 0x0a18 }, /* 0x03 */
    reg_default { reg: TFA9879_PCM_IOM2_FORMAT_2, def: 0x0007 }, /* 0x04 */
    reg_default { reg: TFA9879_EQUALIZER_A1, def: 0x59dd }, /* 0x05 */
    reg_default { reg: TFA9879_EQUALIZER_A2, def: 0xc63e }, /* 0x06 */
    reg_default { reg: TFA9879_EQUALIZER_B1, def: 0x651a }, /* 0x07 */
    reg_default { reg: TFA9879_EQUALIZER_B2, def: 0xe53e }, /* 0x08 */
    reg_default { reg: TFA9879_EQUALIZER_C1, def: 0x4616 }, /* 0x09 */
    reg_default { reg: TFA9879_EQUALIZER_C2, def: 0xd33e }, /* 0x0a */
    reg_default { reg: TFA9879_EQUALIZER_D1, def: 0x4df3 }, /* 0x0b */
    reg_default { reg: TFA9879_EQUALIZER_D2, def: 0xea3e }, /* 0x0c */
    reg_default { reg: TFA9879_EQUALIZER_E1, def: 0x5ee0 }, /* 0x0d */
    reg_default { reg: TFA9879_EQUALIZER_E2, def: 0xf93e }, /* 0x0e */
    reg_default { reg: TFA9879_BYPASS_CONTROL, def: 0x0093 }, /* 0x0f */
    reg_default { reg: TFA9879_DYNAMIC_RANGE_COMPR, def: 0x92ba }, /* 0x10 */
    reg_default { reg: TFA9879_BASS_TREBLE, def: 0x12a5 }, /* 0x11 */
    reg_default { reg: TFA9879_HIGH_PASS_FILTER, def: 0x0004 }, /* 0x12 */
    reg_default { reg: TFA9879_VOLUME_CONTROL, def: 0x10bd }, /* 0x13 */
    reg_default { reg: TFA9879_MISC_CONTROL, def: 0x0000 }, /* 0x14 */
];

unsafe extern "C" fn tfa9879_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    reg == TFA9879_MISC_STATUS
}

static volume_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-7050, 50, 1);
static tb_gain_tlv: [c_uint; 4] = DECLARE_TLV_DB_SCALE!(-1800, 200, 0);
static tb_freq_text: [*const c_char; 3] = [
    c"Low".as_ptr(),
    c"Mid".as_ptr(),
    c"High".as_ptr(),
];
static treble_freq_enum: soc_enum = SOC_ENUM_SINGLE!(
    TFA9879_BASS_TREBLE,
    TFA9879_F_TRBLE_SHIFT,
    tb_freq_text.len(),
    tb_freq_text.as_ptr()
);
static bass_freq_enum: soc_enum = SOC_ENUM_SINGLE!(
    TFA9879_BASS_TREBLE,
    TFA9879_F_BASS_SHIFT,
    tb_freq_text.len(),
    tb_freq_text.as_ptr()
);

static tfa9879_controls: [snd_kcontrol_new; 5] = [
    SOC_SINGLE_TLV!(
        c"PCM Playback Volume".as_ptr(),
        TFA9879_VOLUME_CONTROL,
        TFA9879_VOL_SHIFT,
        0xbd,
        1,
        volume_tlv.as_ptr()
    ),
    SOC_SINGLE_TLV!(
        c"Treble Volume".as_ptr(),
        TFA9879_BASS_TREBLE,
        TFA9879_G_TRBLE_SHIFT,
        18,
        0,
        tb_gain_tlv.as_ptr()
    ),
    SOC_SINGLE_TLV!(
        c"Bass Volume".as_ptr(),
        TFA9879_BASS_TREBLE,
        TFA9879_G_BASS_SHIFT,
        18,
        0,
        tb_gain_tlv.as_ptr()
    ),
    SOC_ENUM!(c"Treble Corner Freq".as_ptr(), &treble_freq_enum),
    SOC_ENUM!(c"Bass Corner Freq".as_ptr(), &bass_freq_enum),
];

static tfa9879_dapm_widgets: [snd_soc_dapm_widget; 5] = [
    SND_SOC_DAPM_AIF_IN!(c"AIFINL".as_ptr(), c"Playback".as_ptr(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(c"AIFINR".as_ptr(), c"Playback".as_ptr(), 1, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_DAC!(c"DAC".as_ptr(), ptr::null(), TFA9879_DEVICE_CONTROL, TFA9879_OPMODE_SHIFT, 0),
    SND_SOC_DAPM_OUTPUT!(c"LINEOUT".as_ptr()),
    SND_SOC_DAPM_SUPPLY!(
        c"POWER".as_ptr(),
        TFA9879_DEVICE_CONTROL,
        TFA9879_POWERUP_SHIFT,
        0,
        ptr::null(),
        0
    ),
];

static tfa9879_dapm_routes: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route { sink: c"DAC".as_ptr(), control: ptr::null(), source: c"AIFINL".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC".as_ptr(), control: ptr::null(), source: c"AIFINR".as_ptr() },
    snd_soc_dapm_route { sink: c"LINEOUT".as_ptr(), control: ptr::null(), source: c"DAC".as_ptr() },
    snd_soc_dapm_route { sink: c"DAC".as_ptr(), control: ptr::null(), source: c"POWER".as_ptr() },
];

static tfa9879_component: snd_soc_component_driver = snd_soc_component_driver {
    controls: tfa9879_controls.as_ptr(),
    num_controls: tfa9879_controls.len() as c_uint,
    dapm_widgets: tfa9879_dapm_widgets.as_ptr(),
    num_dapm_widgets: tfa9879_dapm_widgets.len() as c_uint,
    dapm_routes: tfa9879_dapm_routes.as_ptr(),
    num_dapm_routes: tfa9879_dapm_routes.len() as c_uint,
    idle_bias_on: 1,
    use_pmdown_time: 1,
    endianness: 1,
};

static tfa9879_regmap: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 16,
    volatile_reg: Some(tfa9879_volatile_reg),
    max_register: TFA9879_MISC_STATUS,
    reg_defaults: tfa9879_regs.as_ptr(),
    num_reg_defaults: tfa9879_regs.len() as c_uint,
    cache_type: REGCACHE_RBTREE,
};

static tfa9879_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(tfa9879_hw_params),
    mute_stream: Some(tfa9879_mute_stream),
    set_fmt: Some(tfa9879_set_fmt),
    no_capture_mute: 1,
};

const TFA9879_RATES: c_uint = SNDRV_PCM_RATE_8000_96000;

const TFA9879_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE;

static mut tfa9879_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"tfa9879-hifi".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: TFA9879_RATES,
        formats: TFA9879_FORMATS,
    },
    ops: &tfa9879_dai_ops,
};

unsafe extern "C" fn tfa9879_i2c_probe(i2c: *mut i2c_client) -> c_int {
    let tfa9879: *mut tfa9879_priv;
    let mut i: c_int;

    tfa9879 = devm_kzalloc(
        &mut (*i2c).dev,
        size_of::<tfa9879_priv>(),
        GFP_KERNEL,
    ) as *mut tfa9879_priv;
    if tfa9879.is_null() {
        return -ENOMEM;
    }

    i2c_set_clientdata(i2c, tfa9879 as *mut c_void);

    (*tfa9879).regmap = devm_regmap_init_i2c(i2c, &tfa9879_regmap);
    if IS_ERR((*tfa9879).regmap as *const c_void) {
        return PTR_ERR((*tfa9879).regmap as *const c_void);
    }

    /* Ensure the device is in reset state */
    i = 0;
    while (i as usize) < tfa9879_regs.len() {
        regmap_write(
            (*tfa9879).regmap,
            tfa9879_regs[i as usize].reg,
            tfa9879_regs[i as usize].def,
        );
        i += 1;
    }

    devm_snd_soc_register_component(
        &mut (*i2c).dev,
        &tfa9879_component,
        &raw mut tfa9879_dai,
        1,
    )
}

static tfa9879_i2c_id: [i2c_device_id; 2] = [
    i2c_device_id { name: c"tfa9879".as_ptr() },
    i2c_device_id { name: ptr::null() },
];
// MODULE_DEVICE_TABLE(i2c, tfa9879_i2c_id);

static tfa9879_of_match: [of_device_id; 2] = [
    of_device_id { compatible: c"nxp,tfa9879".as_ptr() },
    of_device_id { compatible: ptr::null() },
];
// MODULE_DEVICE_TABLE(of, tfa9879_of_match);

static mut tfa9879_i2c_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: c"tfa9879".as_ptr(),
        of_match_table: tfa9879_of_match.as_ptr(),
    },
    probe: Some(tfa9879_i2c_probe),
    id_table: tfa9879_i2c_id.as_ptr(),
};

module_i2c_driver!(tfa9879_i2c_driver);

// MODULE_DESCRIPTION("ASoC NXP Semiconductors TFA9879 driver");
// MODULE_AUTHOR("Peter Rosin <peda@axentia.se>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
