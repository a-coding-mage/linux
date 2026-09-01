// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// C dependencies:
// #include <linux/bitfield.h>
// #include <sound/pcm_params.h>
// #include <sound/soc.h>
// #include <sound/soc-dai.h>
// #include <dt-bindings/sound/meson-aiu.h>
// #include "aiu.h"
// #include "meson-codec-glue.h"

use core::ffi::{c_char, c_int, c_uint, c_ulong};
use core::ptr;

const fn bit(nr: c_uint) -> c_uint {
    1u32 << nr
}

const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const fn field_shift(mask: c_uint) -> c_uint {
    mask.trailing_zeros()
}

const fn field_prep(mask: c_uint, val: c_uint) -> c_uint {
    (val << field_shift(mask)) & mask
}

const CTRL_DIN_EN: c_uint = 15;
const CTRL_CLK_INV: c_uint = bit(14);
const CTRL_LRCLK_INV: c_uint = bit(13);
const CTRL_I2S_IN_BCLK_SRC: c_uint = bit(11);
const CTRL_DIN_LRCLK_SRC_SHIFT: c_uint = 6;
const CTRL_DIN_LRCLK_SRC: c_uint = 0x3 << CTRL_DIN_LRCLK_SRC_SHIFT;
const CTRL_BCLK_MCLK_SRC: c_uint = genmask(5, 4);
const CTRL_DIN_SKEW: c_uint = genmask(3, 2);
const CTRL_I2S_OUT_LANE_SRC: c_uint = 0;

const AIU_ACODEC_OUT_CHMAX: c_uint = 2;

const EINVAL: c_int = 22;
const SND_SOC_NOPM: c_int = -1;
const AIU_ACODEC_CTRL: c_uint = 0;
const AIU_ACODEC: c_uint = 0;
const CTRL_I2S: usize = 1;
const CTRL_PCM: usize = 2;
const CTRL_OUT: usize = 3;

const SNDRV_PCM_FMTBIT_S16_LE: c_ulong = 1 << 0;
const SNDRV_PCM_FMTBIT_S20_3LE: c_ulong = 1 << 1;
const SNDRV_PCM_FMTBIT_S24_3LE: c_ulong = 1 << 2;
const SNDRV_PCM_FMTBIT_S24_LE: c_ulong = 1 << 3;
const SNDRV_PCM_FMTBIT_S32_LE: c_ulong = 1 << 4;

const AIU_ACODEC_CTRL_FORMATS: c_ulong = SNDRV_PCM_FMTBIT_S16_LE
    | SNDRV_PCM_FMTBIT_S20_3LE
    | SNDRV_PCM_FMTBIT_S24_3LE
    | SNDRV_PCM_FMTBIT_S24_LE
    | SNDRV_PCM_FMTBIT_S32_LE;

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
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
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_phandle_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_params {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
}

#[repr(C)]
pub struct meson_codec_glue_input {
    pub params: snd_soc_dai_params,
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub shift_r: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
    pub values: *const c_uint,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
    pub private_value: c_ulong,
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
    pub kcontrol_news: *const snd_kcontrol_new,
    pub num_kcontrols: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
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
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: c_uint,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub of_xlate_dai_name: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *const of_phandle_args,
            *mut *const c_char,
        ) -> c_int,
    >,
    pub endianness: c_uint,
    // Present in C only when CONFIG_DEBUG_FS is enabled.
    pub debugfs_prefix: *const c_char,
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
        update: *mut core::ffi::c_void,
    );
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        value: c_uint,
    ) -> c_int;
    fn snd_soc_dapm_get_enum_double(
        kcontrol: *mut snd_kcontrol,
        ucontrol: *mut snd_ctl_elem_value,
    ) -> c_int;
    fn meson_codec_glue_input_hw_params(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    fn meson_codec_glue_input_get_data(dai: *mut snd_soc_dai) -> *mut meson_codec_glue_input;
    fn meson_codec_glue_input_dai_probe(dai: *mut snd_soc_dai) -> c_int;
    fn meson_codec_glue_input_dai_remove(dai: *mut snd_soc_dai) -> c_int;
    fn meson_codec_glue_input_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn meson_codec_glue_output_startup(
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    fn aiu_of_xlate_dai_name(
        component: *mut snd_soc_component,
        args: *const of_phandle_args,
        dai_name: *mut *const c_char,
        component_id: c_uint,
    ) -> c_int;
    fn snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

static AIU_ACODEC_CTRL_MUX_TEXTS: [*const c_char; 3] = [
    b"DISABLED\0".as_ptr() as *const c_char,
    b"I2S\0".as_ptr() as *const c_char,
    b"PCM\0".as_ptr() as *const c_char,
];

unsafe extern "C" fn aiu_acodec_ctrl_mux_put_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let mux: c_uint;
    let changed: c_uint;

    if (*ucontrol).value.enumerated.item[0] >= (*e).items {
        return -EINVAL;
    }

    mux = snd_soc_enum_item_to_val(e, (*ucontrol).value.enumerated.item[0]);
    changed = snd_soc_component_test_bits(
        component,
        (*e).reg,
        CTRL_DIN_LRCLK_SRC,
        field_prep(CTRL_DIN_LRCLK_SRC, mux),
    );

    if changed == 0 {
        return 0;
    }

    /* Force disconnect of the mux while updating */
    snd_soc_dapm_mux_update_power(dapm, kcontrol, 0, ptr::null_mut(), ptr::null_mut());

    snd_soc_component_update_bits(
        component,
        (*e).reg,
        CTRL_DIN_LRCLK_SRC | CTRL_BCLK_MCLK_SRC,
        field_prep(CTRL_DIN_LRCLK_SRC, mux) | field_prep(CTRL_BCLK_MCLK_SRC, mux),
    );

    snd_soc_dapm_mux_update_power(dapm, kcontrol, mux, e, ptr::null_mut());

    1
}

static AIU_ACODEC_CTRL_MUX_ENUM: soc_enum = soc_enum {
    reg: AIU_ACODEC_CTRL,
    shift_l: CTRL_DIN_LRCLK_SRC_SHIFT,
    shift_r: CTRL_DIN_LRCLK_SRC_SHIFT,
    items: AIU_ACODEC_CTRL_MUX_TEXTS.len() as c_uint,
    texts: AIU_ACODEC_CTRL_MUX_TEXTS.as_ptr(),
    values: ptr::null(),
};

static AIU_ACODEC_CTRL_MUX: snd_kcontrol_new = snd_kcontrol_new {
    name: b"ACodec Source\0".as_ptr() as *const c_char,
    private_value: &AIU_ACODEC_CTRL_MUX_ENUM as *const soc_enum as c_ulong,
    get: Some(snd_soc_dapm_get_enum_double),
    put: Some(aiu_acodec_ctrl_mux_put_enum),
};

static AIU_ACODEC_CTRL_OUT_ENABLE: snd_kcontrol_new = snd_kcontrol_new {
    name: b"Switch\0".as_ptr() as *const c_char,
    private_value: 0,
    get: None,
    put: None,
};

static AIU_ACODEC_CTRL_WIDGETS: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget {
        id: 0,
        name: b"ACODEC SRC\0".as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol_news: &AIU_ACODEC_CTRL_MUX,
        num_kcontrols: 1,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: b"ACODEC OUT EN\0".as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol_news: &AIU_ACODEC_CTRL_OUT_ENABLE,
        num_kcontrols: 1,
    },
];

unsafe extern "C" fn aiu_acodec_ctrl_input_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let data: *mut meson_codec_glue_input;
    let ret: c_int;

    ret = meson_codec_glue_input_hw_params(substream, params, dai);
    if ret != 0 {
        return ret;
    }

    /* The glue will provide 1 lane out of the 4 to the output */
    data = meson_codec_glue_input_get_data(dai);
    (*data).params.channels_min = AIU_ACODEC_OUT_CHMAX.min((*data).params.channels_min);
    (*data).params.channels_max = AIU_ACODEC_OUT_CHMAX.min((*data).params.channels_max);

    0
}

static AIU_ACODEC_CTRL_INPUT_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(meson_codec_glue_input_dai_probe),
    remove: Some(meson_codec_glue_input_dai_remove),
    hw_params: Some(aiu_acodec_ctrl_input_hw_params),
    set_fmt: Some(meson_codec_glue_input_set_fmt),
    startup: None,
};

static AIU_ACODEC_CTRL_OUTPUT_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: None,
    remove: None,
    hw_params: None,
    set_fmt: None,
    startup: Some(meson_codec_glue_output_startup),
};

const fn aiu_acodec_stream(
    stream_name: *const c_char,
    xchmax: c_uint,
) -> snd_soc_pcm_stream {
    snd_soc_pcm_stream {
        stream_name,
        channels_min: 1,
        channels_max: xchmax,
        rate_min: 5512,
        rate_max: 192000,
        formats: AIU_ACODEC_CTRL_FORMATS,
    }
}

static mut AIU_ACODEC_CTRL_DAI_DRV: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver {
        name: ptr::null(),
        playback: aiu_acodec_stream(ptr::null(), 0),
        capture: aiu_acodec_stream(ptr::null(), 0),
        ops: ptr::null(),
    },
    snd_soc_dai_driver {
        name: b"ACODEC CTRL ACODEC I2S IN\0".as_ptr() as *const c_char,
        playback: aiu_acodec_stream(b"ACODEC I2S IN Playback\0".as_ptr() as *const c_char, 8),
        capture: aiu_acodec_stream(ptr::null(), 0),
        ops: &AIU_ACODEC_CTRL_INPUT_OPS,
    },
    snd_soc_dai_driver {
        name: b"ACODEC CTRL ACODEC PCM IN\0".as_ptr() as *const c_char,
        playback: aiu_acodec_stream(b"ACODEC PCM IN Playback\0".as_ptr() as *const c_char, 8),
        capture: aiu_acodec_stream(ptr::null(), 0),
        ops: &AIU_ACODEC_CTRL_INPUT_OPS,
    },
    snd_soc_dai_driver {
        name: b"ACODEC CTRL ACODEC OUT\0".as_ptr() as *const c_char,
        playback: aiu_acodec_stream(ptr::null(), 0),
        capture: aiu_acodec_stream(
            b"ACODEC OUT Capture\0".as_ptr() as *const c_char,
            AIU_ACODEC_OUT_CHMAX,
        ),
        ops: &AIU_ACODEC_CTRL_OUTPUT_OPS,
    },
];

static AIU_ACODEC_CTRL_ROUTES: [snd_soc_dapm_route; 4] = [
    snd_soc_dapm_route {
        sink: b"ACODEC SRC\0".as_ptr() as *const c_char,
        control: b"I2S\0".as_ptr() as *const c_char,
        source: b"ACODEC I2S IN Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ACODEC SRC\0".as_ptr() as *const c_char,
        control: b"PCM\0".as_ptr() as *const c_char,
        source: b"ACODEC PCM IN Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ACODEC OUT EN\0".as_ptr() as *const c_char,
        control: b"Switch\0".as_ptr() as *const c_char,
        source: b"ACODEC SRC\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"ACODEC OUT Capture\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"ACODEC OUT EN\0".as_ptr() as *const c_char,
    },
];

static AIU_ACODEC_CTRL_CONTROLS: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    name: b"ACODEC I2S Lane Select\0".as_ptr() as *const c_char,
    private_value: 0,
    get: None,
    put: None,
}];

unsafe extern "C" fn aiu_acodec_of_xlate_dai_name(
    component: *mut snd_soc_component,
    args: *const of_phandle_args,
    dai_name: *mut *const c_char,
) -> c_int {
    aiu_of_xlate_dai_name(component, args, dai_name, AIU_ACODEC)
}

unsafe extern "C" fn aiu_acodec_ctrl_component_probe(
    component: *mut snd_soc_component,
) -> c_int {
    /*
     * NOTE: Din Skew setting
     * According to the documentation, the following update adds one delay
     * to the din line. Without this, the output saturates. This happens
     * regardless of the link format (i2s or left_j) so it is not clear what
     * it actually does but it seems to be required
     */
    snd_soc_component_update_bits(
        component,
        AIU_ACODEC_CTRL,
        CTRL_DIN_SKEW,
        field_prep(CTRL_DIN_SKEW, 2),
    );

    0
}

static AIU_ACODEC_CTRL_COMPONENT: snd_soc_component_driver = snd_soc_component_driver {
    name: b"AIU Internal DAC Codec Control\0".as_ptr() as *const c_char,
    probe: Some(aiu_acodec_ctrl_component_probe),
    controls: AIU_ACODEC_CTRL_CONTROLS.as_ptr(),
    num_controls: AIU_ACODEC_CTRL_CONTROLS.len() as c_uint,
    dapm_widgets: AIU_ACODEC_CTRL_WIDGETS.as_ptr(),
    num_dapm_widgets: AIU_ACODEC_CTRL_WIDGETS.len() as c_uint,
    dapm_routes: AIU_ACODEC_CTRL_ROUTES.as_ptr(),
    num_dapm_routes: AIU_ACODEC_CTRL_ROUTES.len() as c_uint,
    of_xlate_dai_name: Some(aiu_acodec_of_xlate_dai_name),
    endianness: 1,
    // #ifdef CONFIG_DEBUG_FS
    debugfs_prefix: b"acodec\0".as_ptr() as *const c_char,
    // #endif
};

#[no_mangle]
pub unsafe extern "C" fn aiu_acodec_ctrl_register_component(dev: *mut device) -> c_int {
    snd_soc_register_component(
        dev,
        &AIU_ACODEC_CTRL_COMPONENT,
        AIU_ACODEC_CTRL_DAI_DRV.as_mut_ptr(),
        AIU_ACODEC_CTRL_DAI_DRV.len() as c_int,
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
