// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// C includes translated as external dependencies:
// <linux/bitfield.h>, <sound/pcm_params.h>, <sound/soc.h>,
// <sound/soc-dai.h>, <dt-bindings/sound/meson-aiu.h>, "aiu.h",
// and "meson-codec-glue.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong};
use core::ptr;

pub const CTRL_CLK_SEL: c_uint = 0x3;
pub const CTRL_DATA_SEL_SHIFT: c_uint = 4;
pub const CTRL_DATA_SEL: c_uint = 0x3 << CTRL_DATA_SEL_SHIFT;

extern "C" {
    static AIU_HDMI_CLK_DATA_CTRL: c_uint;
    static CTRL_I2S: usize;
    static CTRL_PCM: usize;
    static CTRL_OUT: usize;
    static AIU_HDMI: c_uint;

    static SNDRV_PCM_FMTBIT_S16_LE: u64;
    static SNDRV_PCM_FMTBIT_S20_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_3LE: u64;
    static SNDRV_PCM_FMTBIT_S24_LE: u64;
    static SNDRV_PCM_FMTBIT_S32_LE: u64;

    static SND_SOC_NOPM: c_int;

    fn snd_soc_dapm_kcontrol_to_component(
        kcontrol: *mut snd_kcontrol,
    ) -> *mut snd_soc_component;
    fn snd_soc_dapm_kcontrol_to_dapm(
        kcontrol: *mut snd_kcontrol,
    ) -> *mut snd_soc_dapm_context;
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
        mux: c_int,
        e: *mut soc_enum,
        update: *mut snd_soc_dapm_update,
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

    fn meson_codec_glue_input_dai_probe(dai: *mut snd_soc_dai) -> c_int;
    fn meson_codec_glue_input_dai_remove(dai: *mut snd_soc_dai) -> c_int;
    fn meson_codec_glue_input_hw_params(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        dai: *mut snd_soc_dai,
    ) -> c_int;
    fn meson_codec_glue_input_set_fmt(dai: *mut snd_soc_dai, fmt: c_uint) -> c_int;
    fn meson_codec_glue_output_startup(
        substream: *mut snd_pcm_substream,
        dai: *mut snd_soc_dai,
    ) -> c_int;

    fn aiu_of_xlate_dai_name(
        component: *mut snd_soc_component,
        args: *const of_phandle_args,
        dai_name: *mut *const c_char,
        id: c_uint,
    ) -> c_int;

    fn snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: c_ulong,
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
pub struct snd_soc_dapm_update {
    _private: [u8; 0],
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

#[repr(C)]
pub struct of_phandle_args {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_enum {
    pub reg: c_uint,
    pub shift_l: c_uint,
    pub items: c_uint,
    pub texts: *const *const c_char,
}

#[repr(C)]
pub struct snd_ctl_elem_value_enumerated {
    pub item: [c_uint; 4],
}

#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub iface: c_uint,
    pub name: *const c_char,
    pub info: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_info) -> c_int>,
    pub get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    pub private_value: c_ulong,
}

#[repr(C)]
pub struct snd_ctl_elem_info {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_uchar,
    pub invert: c_uchar,
    pub kcontrol_news: *const snd_kcontrol_new,
    pub num_kcontrols: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
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
    // CONFIG_DEBUG_FS: debugfs_prefix = "hdmi".
    pub debugfs_prefix: *const c_char,
}

pub type c_uchar = u8;

const EINVAL: c_int = 22;

const AIU_CODEC_CTRL_MUX_TEXT_DISABLED: &[u8] = b"DISABLED\0";
const AIU_CODEC_CTRL_MUX_TEXT_PCM: &[u8] = b"PCM\0";
const AIU_CODEC_CTRL_MUX_TEXT_I2S: &[u8] = b"I2S\0";

static aiu_codec_ctrl_mux_texts: [*const c_char; 3] = [
    AIU_CODEC_CTRL_MUX_TEXT_DISABLED.as_ptr() as *const c_char,
    AIU_CODEC_CTRL_MUX_TEXT_PCM.as_ptr() as *const c_char,
    AIU_CODEC_CTRL_MUX_TEXT_I2S.as_ptr() as *const c_char,
];

const HDMI_SOURCE: &[u8] = b"HDMI Source\0";
const HDMI_CTRL_SRC: &[u8] = b"HDMI CTRL SRC\0";
const HDMI_I2S_IN_PLAYBACK: &[u8] = b"HDMI I2S IN Playback\0";
const HDMI_PCM_IN_PLAYBACK: &[u8] = b"HDMI PCM IN Playback\0";
const HDMI_OUT_CAPTURE: &[u8] = b"HDMI OUT Capture\0";
const CODEC_CTRL_HDMI_I2S_IN: &[u8] = b"CODEC CTRL HDMI I2S IN\0";
const CODEC_CTRL_HDMI_PCM_IN: &[u8] = b"CODEC CTRL HDMI PCM IN\0";
const CODEC_CTRL_HDMI_OUT: &[u8] = b"CODEC CTRL HDMI OUT\0";
const AIU_HDMI_CODEC_CONTROL: &[u8] = b"AIU HDMI Codec Control\0";
const DEBUGFS_PREFIX_HDMI: &[u8] = b"hdmi\0";

const fn field_prep(mask: c_uint, val: c_uint) -> c_uint {
    (val << mask.trailing_zeros()) & mask
}

unsafe extern "C" fn aiu_codec_ctrl_mux_put_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component: *mut snd_soc_component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let dapm: *mut snd_soc_dapm_context = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let e: *mut soc_enum = (*kcontrol).private_value as *mut soc_enum;
    let mux: c_uint;
    let changed: c_uint;

    if (*ucontrol).value.enumerated.item[0] >= (*e).items {
        return -EINVAL;
    }

    mux = snd_soc_enum_item_to_val(e, (*ucontrol).value.enumerated.item[0]);
    changed = snd_soc_component_test_bits(
        component,
        (*e).reg,
        CTRL_DATA_SEL,
        field_prep(CTRL_DATA_SEL, mux),
    );

    if changed == 0 {
        return 0;
    }

    /* Force disconnect of the mux while updating */
    snd_soc_dapm_mux_update_power(dapm, kcontrol, 0, ptr::null_mut(), ptr::null_mut());

    /* Reset the source first */
    snd_soc_component_update_bits(
        component,
        (*e).reg,
        CTRL_CLK_SEL | CTRL_DATA_SEL,
        field_prep(CTRL_CLK_SEL, 0) | field_prep(CTRL_DATA_SEL, 0),
    );

    /* Set the appropriate source */
    snd_soc_component_update_bits(
        component,
        (*e).reg,
        CTRL_CLK_SEL | CTRL_DATA_SEL,
        field_prep(CTRL_CLK_SEL, mux) | field_prep(CTRL_DATA_SEL, mux),
    );

    snd_soc_dapm_mux_update_power(dapm, kcontrol, mux as c_int, e, ptr::null_mut());

    return 1;
}

static aiu_hdmi_ctrl_mux_enum: soc_enum = soc_enum {
    reg: unsafe { AIU_HDMI_CLK_DATA_CTRL },
    shift_l: CTRL_DATA_SEL_SHIFT,
    items: 3,
    texts: aiu_codec_ctrl_mux_texts.as_ptr(),
};

static aiu_hdmi_ctrl_mux: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0,
    name: HDMI_SOURCE.as_ptr() as *const c_char,
    info: None,
    get: Some(snd_soc_dapm_get_enum_double),
    put: Some(aiu_codec_ctrl_mux_put_enum),
    private_value: (&aiu_hdmi_ctrl_mux_enum as *const soc_enum) as c_ulong,
};

static aiu_hdmi_ctrl_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget {
    id: 0,
    name: HDMI_CTRL_SRC.as_ptr() as *const c_char,
    reg: unsafe { SND_SOC_NOPM },
    shift: 0,
    invert: 0,
    kcontrol_news: &aiu_hdmi_ctrl_mux,
    num_kcontrols: 1,
}];

static aiu_codec_ctrl_input_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(meson_codec_glue_input_dai_probe),
    remove: Some(meson_codec_glue_input_dai_remove),
    hw_params: Some(meson_codec_glue_input_hw_params),
    set_fmt: Some(meson_codec_glue_input_set_fmt),
    startup: None,
};

static aiu_codec_ctrl_output_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(meson_codec_glue_output_startup),
    hw_params: None,
    set_fmt: None,
    probe: None,
    remove: None,
};

unsafe fn aiu_codec_ctrl_formats() -> u64 {
    SNDRV_PCM_FMTBIT_S16_LE
        | SNDRV_PCM_FMTBIT_S20_3LE
        | SNDRV_PCM_FMTBIT_S24_3LE
        | SNDRV_PCM_FMTBIT_S24_LE
        | SNDRV_PCM_FMTBIT_S32_LE
}

unsafe fn aiu_codec_ctrl_stream(stream_name: *const c_char) -> snd_soc_pcm_stream {
    snd_soc_pcm_stream {
        stream_name,
        channels_min: 1,
        channels_max: 8,
        rate_min: 5512,
        rate_max: 192000,
        formats: aiu_codec_ctrl_formats(),
    }
}

static mut aiu_hdmi_ctrl_dai_drv: [snd_soc_dai_driver; 3] = unsafe {
    let mut dai = [
        snd_soc_dai_driver {
            name: ptr::null(),
            playback: snd_soc_pcm_stream {
                stream_name: ptr::null(),
                channels_min: 0,
                channels_max: 0,
                rate_min: 0,
                rate_max: 0,
                formats: 0,
            },
            capture: snd_soc_pcm_stream {
                stream_name: ptr::null(),
                channels_min: 0,
                channels_max: 0,
                rate_min: 0,
                rate_max: 0,
                formats: 0,
            },
            ops: ptr::null(),
        },
        snd_soc_dai_driver {
            name: ptr::null(),
            playback: snd_soc_pcm_stream {
                stream_name: ptr::null(),
                channels_min: 0,
                channels_max: 0,
                rate_min: 0,
                rate_max: 0,
                formats: 0,
            },
            capture: snd_soc_pcm_stream {
                stream_name: ptr::null(),
                channels_min: 0,
                channels_max: 0,
                rate_min: 0,
                rate_max: 0,
                formats: 0,
            },
            ops: ptr::null(),
        },
        snd_soc_dai_driver {
            name: ptr::null(),
            playback: snd_soc_pcm_stream {
                stream_name: ptr::null(),
                channels_min: 0,
                channels_max: 0,
                rate_min: 0,
                rate_max: 0,
                formats: 0,
            },
            capture: snd_soc_pcm_stream {
                stream_name: ptr::null(),
                channels_min: 0,
                channels_max: 0,
                rate_min: 0,
                rate_max: 0,
                formats: 0,
            },
            ops: ptr::null(),
        },
    ];

    dai[CTRL_I2S] = snd_soc_dai_driver {
        name: CODEC_CTRL_HDMI_I2S_IN.as_ptr() as *const c_char,
        playback: aiu_codec_ctrl_stream(HDMI_I2S_IN_PLAYBACK.as_ptr() as *const c_char),
        capture: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rate_min: 0,
            rate_max: 0,
            formats: 0,
        },
        ops: &aiu_codec_ctrl_input_ops,
    };
    dai[CTRL_PCM] = snd_soc_dai_driver {
        name: CODEC_CTRL_HDMI_PCM_IN.as_ptr() as *const c_char,
        playback: aiu_codec_ctrl_stream(HDMI_PCM_IN_PLAYBACK.as_ptr() as *const c_char),
        capture: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rate_min: 0,
            rate_max: 0,
            formats: 0,
        },
        ops: &aiu_codec_ctrl_input_ops,
    };
    dai[CTRL_OUT] = snd_soc_dai_driver {
        name: CODEC_CTRL_HDMI_OUT.as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: ptr::null(),
            channels_min: 0,
            channels_max: 0,
            rate_min: 0,
            rate_max: 0,
            formats: 0,
        },
        capture: aiu_codec_ctrl_stream(HDMI_OUT_CAPTURE.as_ptr() as *const c_char),
        ops: &aiu_codec_ctrl_output_ops,
    };

    dai
};

static aiu_hdmi_ctrl_routes: [snd_soc_dapm_route; 3] = [
    snd_soc_dapm_route {
        sink: HDMI_CTRL_SRC.as_ptr() as *const c_char,
        control: AIU_CODEC_CTRL_MUX_TEXT_I2S.as_ptr() as *const c_char,
        source: HDMI_I2S_IN_PLAYBACK.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: HDMI_CTRL_SRC.as_ptr() as *const c_char,
        control: AIU_CODEC_CTRL_MUX_TEXT_PCM.as_ptr() as *const c_char,
        source: HDMI_PCM_IN_PLAYBACK.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: HDMI_OUT_CAPTURE.as_ptr() as *const c_char,
        control: ptr::null(),
        source: HDMI_CTRL_SRC.as_ptr() as *const c_char,
    },
];

unsafe extern "C" fn aiu_hdmi_of_xlate_dai_name(
    component: *mut snd_soc_component,
    args: *const of_phandle_args,
    dai_name: *mut *const c_char,
) -> c_int {
    return aiu_of_xlate_dai_name(component, args, dai_name, AIU_HDMI);
}

static aiu_hdmi_ctrl_component: snd_soc_component_driver = snd_soc_component_driver {
    name: AIU_HDMI_CODEC_CONTROL.as_ptr() as *const c_char,
    dapm_widgets: aiu_hdmi_ctrl_widgets.as_ptr(),
    num_dapm_widgets: aiu_hdmi_ctrl_widgets.len() as c_uint,
    dapm_routes: aiu_hdmi_ctrl_routes.as_ptr(),
    num_dapm_routes: aiu_hdmi_ctrl_routes.len() as c_uint,
    of_xlate_dai_name: Some(aiu_hdmi_of_xlate_dai_name),
    endianness: 1,
    // CONFIG_DEBUG_FS
    debugfs_prefix: DEBUGFS_PREFIX_HDMI.as_ptr() as *const c_char,
};

#[no_mangle]
pub unsafe extern "C" fn aiu_hdmi_ctrl_register_component(dev: *mut device) -> c_int {
    return snd_soc_register_component(
        dev,
        &aiu_hdmi_ctrl_component,
        aiu_hdmi_ctrl_dai_drv.as_mut_ptr(),
        aiu_hdmi_ctrl_dai_drv.len() as c_int,
    );
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
