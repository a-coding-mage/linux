// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2019 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>
//
// Translated from C implementation source. Original dependencies:
// <linux/bitfield.h>, <linux/clk.h>, <linux/module.h>,
// <sound/pcm_params.h>, <linux/regmap.h>, <linux/reset.h>,
// <sound/soc.h>, <sound/soc-dai.h>,
// <dt-bindings/sound/meson-g12a-tohdmitx.h>, "meson-codec-glue.h"

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

const EINVAL: c_int = 22;

const G12A_TOHDMITX_DRV_NAME: *const c_char = b"g12a-tohdmitx\0".as_ptr() as *const c_char;

const TOHDMITX_CTRL0: c_uint = 0x0;
const CTRL0_ENABLE_SHIFT: c_uint = 31;
const CTRL0_I2S_DAT_SEL_SHIFT: c_uint = 12;
const CTRL0_I2S_DAT_SEL: c_uint = 0x3 << CTRL0_I2S_DAT_SEL_SHIFT;
const CTRL0_I2S_LRCLK_SEL: c_uint = genmask(9, 8);
const CTRL0_I2S_BLK_CAP_INV: c_uint = bit(7);
const CTRL0_I2S_BCLK_O_INV: c_uint = bit(6);
const CTRL0_I2S_BCLK_SEL: c_uint = genmask(5, 4);
const CTRL0_SPDIF_CLK_CAP_INV: c_uint = bit(3);
const CTRL0_SPDIF_CLK_O_INV: c_uint = bit(2);
const CTRL0_SPDIF_SEL_SHIFT: c_uint = 1;
const CTRL0_SPDIF_SEL: c_uint = 0x1 << CTRL0_SPDIF_SEL_SHIFT;
const CTRL0_SPDIF_CLK_SEL: c_uint = bit(0);

const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 0;
const SNDRV_PCM_FMTBIT_S20_3LE: u64 = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_3LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 3;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 4;

const TOHDMITX_I2S_IN_A: c_int = 0;
const TOHDMITX_I2S_IN_B: c_int = 1;
const TOHDMITX_I2S_IN_C: c_int = 2;
const TOHDMITX_I2S_OUT: c_int = 3;
const TOHDMITX_SPDIF_IN_A: c_int = 4;
const TOHDMITX_SPDIF_IN_B: c_int = 5;
const TOHDMITX_SPDIF_OUT: c_int = 6;

const SND_SOC_NOPM: c_int = -1;

const fn bit(nr: c_uint) -> c_uint {
    1u32 << nr
}

const fn genmask(high: c_uint, low: c_uint) -> c_uint {
    ((!0u32) << low) & ((!0u32) >> (31 - high))
}

const fn field_prep(mask: c_uint, val: c_uint) -> c_uint {
    (val << mask.trailing_zeros()) & mask
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 4],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub shift_r: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_uint,
    pub max: c_uint,
    pub invert: c_uint,
    pub enum_: *const soc_enum,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_uint,
    pub invert: c_uint,
    pub kcontrol: *const snd_kcontrol_new,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub endianness: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub val_bits: c_uint,
    pub reg_stride: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai {
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

unsafe extern "C" {
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn snd_soc_component_test_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        value: c_uint,
    ) -> c_uint;
    fn snd_soc_dapm_mux_update_power(
        dapm: *mut snd_soc_dapm_context,
        kcontrol: *mut snd_kcontrol,
        mux: c_uint,
        e: *mut soc_enum,
        update: *mut c_void,
    );
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        value: c_uint,
    ) -> c_int;
    fn snd_soc_dapm_get_enum_double(kcontrol: *mut snd_kcontrol, ucontrol: *mut snd_ctl_elem_value) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, value: c_uint) -> c_int;

    fn meson_codec_glue_input_dai_probe(dai: *mut snd_soc_dai) -> c_int;
    fn meson_codec_glue_input_dai_remove(dai: *mut snd_soc_dai) -> c_int;
    fn meson_codec_glue_input_hw_params(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    fn meson_codec_glue_input_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn meson_codec_glue_output_startup(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int;

    fn device_reset(dev: *mut device) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
}

static G12A_TOHDMITX_I2S_MUX_TEXTS: [*const c_char; 3] = [
    b"I2S A\0".as_ptr() as *const c_char,
    b"I2S B\0".as_ptr() as *const c_char,
    b"I2S C\0".as_ptr() as *const c_char,
];

unsafe extern "C" fn g12a_tohdmitx_i2s_mux_put_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = unsafe { snd_soc_dapm_kcontrol_to_component(kcontrol) };
    let dapm = unsafe { snd_soc_dapm_kcontrol_to_dapm(kcontrol) };
    let e = unsafe { (*kcontrol).private_value as *mut soc_enum };
    let mux: c_uint;
    let changed: c_uint;

    if unsafe { (*ucontrol).value.enumerated.item[0] >= (*e).items } {
        return -EINVAL;
    }

    mux = unsafe { snd_soc_enum_item_to_val(e, (*ucontrol).value.enumerated.item[0]) };
    changed = unsafe {
        snd_soc_component_test_bits(
            component,
            (*e).reg,
            CTRL0_I2S_DAT_SEL,
            field_prep(CTRL0_I2S_DAT_SEL, mux),
        )
    };

    if changed == 0 {
        return 0;
    }

    /* Force disconnect of the mux while updating */
    unsafe { snd_soc_dapm_mux_update_power(dapm, kcontrol, 0, ptr::null_mut(), ptr::null_mut()) };

    unsafe {
        snd_soc_component_update_bits(
            component,
            (*e).reg,
            CTRL0_I2S_DAT_SEL | CTRL0_I2S_LRCLK_SEL | CTRL0_I2S_BCLK_SEL,
            field_prep(CTRL0_I2S_DAT_SEL, mux)
                | field_prep(CTRL0_I2S_LRCLK_SEL, mux)
                | field_prep(CTRL0_I2S_BCLK_SEL, mux),
        )
    };

    unsafe { snd_soc_dapm_mux_update_power(dapm, kcontrol, mux, e, ptr::null_mut()) };

    1
}

static G12A_TOHDMITX_I2S_MUX_ENUM: soc_enum = soc_enum {
    reg: TOHDMITX_CTRL0,
    shift_l: CTRL0_I2S_DAT_SEL_SHIFT,
    shift_r: CTRL0_I2S_DAT_SEL_SHIFT,
    items: G12A_TOHDMITX_I2S_MUX_TEXTS.len() as c_uint,
    texts: G12A_TOHDMITX_I2S_MUX_TEXTS.as_ptr(),
};

static G12A_TOHDMITX_I2S_MUX: snd_kcontrol_new = snd_kcontrol_new {
    name: b"I2S Source\0".as_ptr() as *const c_char,
    reg: 0,
    shift: 0,
    max: 0,
    invert: 0,
    enum_: &G12A_TOHDMITX_I2S_MUX_ENUM,
    get: Some(snd_soc_dapm_get_enum_double),
    put: Some(g12a_tohdmitx_i2s_mux_put_enum),
};

static G12A_TOHDMITX_SPDIF_MUX_TEXTS: [*const c_char; 2] = [
    b"SPDIF A\0".as_ptr() as *const c_char,
    b"SPDIF B\0".as_ptr() as *const c_char,
];

unsafe extern "C" fn g12a_tohdmitx_spdif_mux_put_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = unsafe { snd_soc_dapm_kcontrol_to_component(kcontrol) };
    let dapm = unsafe { snd_soc_dapm_kcontrol_to_dapm(kcontrol) };
    let e = unsafe { (*kcontrol).private_value as *mut soc_enum };
    let mux: c_uint;
    let changed: c_uint;

    if unsafe { (*ucontrol).value.enumerated.item[0] >= (*e).items } {
        return -EINVAL;
    }

    mux = unsafe { snd_soc_enum_item_to_val(e, (*ucontrol).value.enumerated.item[0]) };
    changed = unsafe {
        snd_soc_component_test_bits(
            component,
            TOHDMITX_CTRL0,
            CTRL0_SPDIF_SEL,
            field_prep(CTRL0_SPDIF_SEL, mux),
        )
    };

    if changed == 0 {
        return 0;
    }

    /* Force disconnect of the mux while updating */
    unsafe { snd_soc_dapm_mux_update_power(dapm, kcontrol, 0, ptr::null_mut(), ptr::null_mut()) };

    unsafe {
        snd_soc_component_update_bits(
            component,
            TOHDMITX_CTRL0,
            CTRL0_SPDIF_SEL | CTRL0_SPDIF_CLK_SEL,
            field_prep(CTRL0_SPDIF_SEL, mux) | field_prep(CTRL0_SPDIF_CLK_SEL, mux),
        )
    };

    unsafe { snd_soc_dapm_mux_update_power(dapm, kcontrol, mux, e, ptr::null_mut()) };

    1
}

static G12A_TOHDMITX_SPDIF_MUX_ENUM: soc_enum = soc_enum {
    reg: TOHDMITX_CTRL0,
    shift_l: CTRL0_SPDIF_SEL_SHIFT,
    shift_r: CTRL0_SPDIF_SEL_SHIFT,
    items: G12A_TOHDMITX_SPDIF_MUX_TEXTS.len() as c_uint,
    texts: G12A_TOHDMITX_SPDIF_MUX_TEXTS.as_ptr(),
};

static G12A_TOHDMITX_SPDIF_MUX: snd_kcontrol_new = snd_kcontrol_new {
    name: b"SPDIF Source\0".as_ptr() as *const c_char,
    reg: 0,
    shift: 0,
    max: 0,
    invert: 0,
    enum_: &G12A_TOHDMITX_SPDIF_MUX_ENUM,
    get: Some(snd_soc_dapm_get_enum_double),
    put: Some(g12a_tohdmitx_spdif_mux_put_enum),
};

static G12A_TOHDMITX_OUT_ENABLE: snd_kcontrol_new = snd_kcontrol_new {
    name: b"Switch\0".as_ptr() as *const c_char,
    reg: TOHDMITX_CTRL0 as c_int,
    shift: CTRL0_ENABLE_SHIFT,
    max: 1,
    invert: 0,
    enum_: ptr::null(),
    get: None,
    put: None,
};

static G12A_TOHDMITX_WIDGETS: [snd_soc_dapm_widget; 4] = [
    snd_soc_dapm_widget {
        id: 0,
        name: b"I2S SRC\0".as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol: &G12A_TOHDMITX_I2S_MUX,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: b"I2S OUT EN\0".as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol: &G12A_TOHDMITX_OUT_ENABLE,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: b"SPDIF SRC\0".as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol: &G12A_TOHDMITX_SPDIF_MUX,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: b"SPDIF OUT EN\0".as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol: &G12A_TOHDMITX_OUT_ENABLE,
    },
];

static G12A_TOHDMITX_INPUT_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(meson_codec_glue_input_dai_probe),
    remove: Some(meson_codec_glue_input_dai_remove),
    hw_params: Some(meson_codec_glue_input_hw_params),
    set_fmt: Some(meson_codec_glue_input_set_fmt),
    startup: None,
};

static G12A_TOHDMITX_OUTPUT_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: None,
    remove: None,
    hw_params: None,
    set_fmt: None,
    startup: Some(meson_codec_glue_output_startup),
};

const TOHDMITX_SPDIF_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S20_3LE | SNDRV_PCM_FMTBIT_S24_3LE | SNDRV_PCM_FMTBIT_S24_LE;

const TOHDMITX_I2S_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

const fn tohdmitx_stream(stream_name: *const c_char, xfmt: u64, xchmax: c_uint) -> snd_soc_pcm_stream {
    snd_soc_pcm_stream {
        stream_name,
        channels_min: 1,
        channels_max: xchmax,
        rate_min: 8000,
        rate_max: 192000,
        formats: xfmt,
    }
}

static mut G12A_TOHDMITX_DAI_DRV: [snd_soc_dai_driver; 7] = [
    snd_soc_dai_driver {
        name: b"I2S IN A\0".as_ptr() as *const c_char,
        id: TOHDMITX_I2S_IN_A,
        playback: tohdmitx_stream(b"I2S IN A Playback\0".as_ptr() as *const c_char, TOHDMITX_I2S_FORMATS, 8),
        capture: tohdmitx_stream(ptr::null(), 0, 0),
        ops: &G12A_TOHDMITX_INPUT_OPS,
    },
    snd_soc_dai_driver {
        name: b"I2S IN B\0".as_ptr() as *const c_char,
        id: TOHDMITX_I2S_IN_B,
        playback: tohdmitx_stream(b"I2S IN B Playback\0".as_ptr() as *const c_char, TOHDMITX_I2S_FORMATS, 8),
        capture: tohdmitx_stream(ptr::null(), 0, 0),
        ops: &G12A_TOHDMITX_INPUT_OPS,
    },
    snd_soc_dai_driver {
        name: b"I2S IN C\0".as_ptr() as *const c_char,
        id: TOHDMITX_I2S_IN_C,
        playback: tohdmitx_stream(b"I2S IN C Playback\0".as_ptr() as *const c_char, TOHDMITX_I2S_FORMATS, 8),
        capture: tohdmitx_stream(ptr::null(), 0, 0),
        ops: &G12A_TOHDMITX_INPUT_OPS,
    },
    snd_soc_dai_driver {
        name: b"I2S OUT\0".as_ptr() as *const c_char,
        id: TOHDMITX_I2S_OUT,
        playback: tohdmitx_stream(ptr::null(), 0, 0),
        capture: tohdmitx_stream(b"I2S OUT Capture\0".as_ptr() as *const c_char, TOHDMITX_I2S_FORMATS, 8),
        ops: &G12A_TOHDMITX_OUTPUT_OPS,
    },
    snd_soc_dai_driver {
        name: b"SPDIF IN A\0".as_ptr() as *const c_char,
        id: TOHDMITX_SPDIF_IN_A,
        playback: tohdmitx_stream(b"SPDIF IN A Playback\0".as_ptr() as *const c_char, TOHDMITX_SPDIF_FORMATS, 2),
        capture: tohdmitx_stream(ptr::null(), 0, 0),
        ops: &G12A_TOHDMITX_INPUT_OPS,
    },
    snd_soc_dai_driver {
        name: b"SPDIF IN B\0".as_ptr() as *const c_char,
        id: TOHDMITX_SPDIF_IN_B,
        playback: tohdmitx_stream(b"SPDIF IN B Playback\0".as_ptr() as *const c_char, TOHDMITX_SPDIF_FORMATS, 2),
        capture: tohdmitx_stream(ptr::null(), 0, 0),
        ops: &G12A_TOHDMITX_INPUT_OPS,
    },
    snd_soc_dai_driver {
        name: b"SPDIF OUT\0".as_ptr() as *const c_char,
        id: TOHDMITX_SPDIF_OUT,
        playback: tohdmitx_stream(ptr::null(), 0, 0),
        capture: tohdmitx_stream(b"SPDIF OUT Capture\0".as_ptr() as *const c_char, TOHDMITX_SPDIF_FORMATS, 2),
        ops: &G12A_TOHDMITX_OUTPUT_OPS,
    },
];

unsafe extern "C" fn g12a_tohdmi_component_probe(c: *mut snd_soc_component) -> c_int {
    /* Initialize the static clock parameters */
    unsafe { snd_soc_component_write(c, TOHDMITX_CTRL0, CTRL0_I2S_BLK_CAP_INV | CTRL0_SPDIF_CLK_CAP_INV) }
}

static G12A_TOHDMITX_ROUTES: [snd_soc_dapm_route; 9] = [
    snd_soc_dapm_route { sink: b"I2S SRC\0".as_ptr() as *const c_char, control: b"I2S A\0".as_ptr() as *const c_char, source: b"I2S IN A Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I2S SRC\0".as_ptr() as *const c_char, control: b"I2S B\0".as_ptr() as *const c_char, source: b"I2S IN B Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I2S SRC\0".as_ptr() as *const c_char, control: b"I2S C\0".as_ptr() as *const c_char, source: b"I2S IN C Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I2S OUT EN\0".as_ptr() as *const c_char, control: b"Switch\0".as_ptr() as *const c_char, source: b"I2S SRC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I2S OUT Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"I2S OUT EN\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPDIF SRC\0".as_ptr() as *const c_char, control: b"SPDIF A\0".as_ptr() as *const c_char, source: b"SPDIF IN A Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPDIF SRC\0".as_ptr() as *const c_char, control: b"SPDIF B\0".as_ptr() as *const c_char, source: b"SPDIF IN B Playback\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPDIF OUT EN\0".as_ptr() as *const c_char, control: b"Switch\0".as_ptr() as *const c_char, source: b"SPDIF SRC\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SPDIF OUT Capture\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SPDIF OUT EN\0".as_ptr() as *const c_char },
];

static G12A_TOHDMITX_COMPONENT_DRV: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(g12a_tohdmi_component_probe),
    dapm_widgets: G12A_TOHDMITX_WIDGETS.as_ptr(),
    num_dapm_widgets: G12A_TOHDMITX_WIDGETS.len() as c_uint,
    dapm_routes: G12A_TOHDMITX_ROUTES.as_ptr(),
    num_dapm_routes: G12A_TOHDMITX_ROUTES.len() as c_uint,
    endianness: 1,
};

static G12A_TOHDMITX_REGMAP_CFG: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
};

static G12A_TOHDMITX_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: b"amlogic,g12a-tohdmitx\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, g12a_tohdmitx_of_match);

unsafe extern "C" fn g12a_tohdmitx_probe(pdev: *mut platform_device) -> c_int {
    let dev = unsafe { &mut (*pdev).dev as *mut device };
    let regs: *mut c_void;
    let map: *mut regmap;
    let mut ret: c_int;

    ret = unsafe { device_reset(dev) };
    if ret != 0 {
        return unsafe { dev_err_probe(dev, ret, b"failed to reset device\n\0".as_ptr() as *const c_char) };
    }

    regs = unsafe { devm_platform_ioremap_resource(pdev, 0) };
    if unsafe { IS_ERR(regs as *const c_void) } {
        return unsafe { PTR_ERR(regs as *const c_void) as c_int };
    }

    map = unsafe { devm_regmap_init_mmio(dev, regs, &G12A_TOHDMITX_REGMAP_CFG) };
    if unsafe { IS_ERR(map as *const c_void) } {
        unsafe {
            dev_err(
                dev,
                b"failed to init regmap: %ld\n\0".as_ptr() as *const c_char,
                PTR_ERR(map as *const c_void),
            )
        };
        return unsafe { PTR_ERR(map as *const c_void) as c_int };
    }

    unsafe {
        devm_snd_soc_register_component(
            dev,
            &G12A_TOHDMITX_COMPONENT_DRV,
            G12A_TOHDMITX_DAI_DRV.as_mut_ptr(),
            G12A_TOHDMITX_DAI_DRV.len() as c_int,
        )
    }
}

static mut G12A_TOHDMITX_PDRV: platform_driver = platform_driver {
    driver: device_driver {
        name: G12A_TOHDMITX_DRV_NAME,
        of_match_table: G12A_TOHDMITX_OF_MATCH.as_ptr(),
    },
    probe: Some(g12a_tohdmitx_probe),
};

// module_platform_driver(g12a_tohdmitx_pdrv);
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_DESCRIPTION("Amlogic G12a To HDMI Tx Control Codec Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
