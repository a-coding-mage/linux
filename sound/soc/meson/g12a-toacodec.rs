// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// C dependencies translated as external Rust dependencies:
// linux/bitfield.h, linux/clk.h, linux/module.h, sound/pcm_params.h,
// linux/regmap.h, linux/regulator/consumer.h, linux/reset.h,
// sound/soc.h, sound/soc-dai.h, dt-bindings/sound/meson-g12a-toacodec.h,
// axg-tdm.h, meson-codec-glue.h

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::ptr;

const G12A_TOACODEC_DRV_NAME: &[u8] = b"g12a-toacodec\0";

const TOACODEC_CTRL0: c_uint = 0x0;
const CTRL0_ENABLE_SHIFT: c_uint = 31;
const CTRL0_DAT_SEL_SM1_MSB: c_uint = 19;
const CTRL0_DAT_SEL_SM1_LSB: c_uint = 18;
const CTRL0_DAT_SEL_MSB: c_uint = 15;
const CTRL0_DAT_SEL_LSB: c_uint = 14;
const CTRL0_LANE_SEL_SM1: c_uint = 16;
const CTRL0_LANE_SEL: c_uint = 12;
const CTRL0_LRCLK_SEL_SM1_MSB: c_uint = 14;
const CTRL0_LRCLK_SEL_SM1_LSB: c_uint = 12;
const CTRL0_LRCLK_SEL_MSB: c_uint = 9;
const CTRL0_LRCLK_SEL_LSB: c_uint = 8;
const CTRL0_LRCLK_INV_SM1: c_uint = bit(10);
const CTRL0_BLK_CAP_INV_SM1: c_uint = bit(9);
const CTRL0_BLK_CAP_INV: c_uint = bit(7);
const CTRL0_BCLK_O_INV_SM1: c_uint = bit(8);
const CTRL0_BCLK_O_INV: c_uint = bit(6);
const CTRL0_BCLK_SEL_SM1_MSB: c_uint = 6;
const CTRL0_BCLK_SEL_MSB: c_uint = 5;
const CTRL0_BCLK_SEL_LSB: c_uint = 4;
const CTRL0_MCLK_SEL: c_uint = genmask(2, 0);

const TOACODEC_OUT_CHMAX: c_uint = 2;

const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const SND_SOC_NOPM: c_int = 0;
const AXG_TDM_FORMATS: u64 = 0;
const TOACODEC_IN_A: c_int = 0;
const TOACODEC_IN_B: c_int = 1;
const TOACODEC_IN_C: c_int = 2;
const TOACODEC_OUT: c_int = 3;

const fn bit(nr: c_uint) -> c_uint {
    1_u32 << nr
}

const fn genmask(h: c_uint, l: c_uint) -> c_uint {
    (!0_u32 << l) & (!0_u32 >> (31 - h))
}

const fn field_prep(mask: c_uint, val: c_uint) -> c_uint {
    (val << mask.trailing_zeros()) & mask
}

#[repr(C)]
struct regmap_field {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_context {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct snd_kcontrol {
    private_value: usize,
}

#[repr(C)]
struct snd_ctl_elem_value {
    value: snd_ctl_elem_value_value,
}

#[repr(C)]
union snd_ctl_elem_value_value {
    enumerated: snd_ctl_elem_value_enumerated,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct snd_ctl_elem_value_enumerated {
    item: [c_uint; 4],
}

#[repr(C)]
struct soc_enum {
    reg: c_uint,
    shift_l: c_uint,
    shift_r: c_uint,
    items: c_uint,
    texts: *const *const c_char,
    values: *const c_uint,
    mask: c_uint,
}

#[repr(C)]
struct g12a_toacodec {
    field_dat_sel: *mut regmap_field,
    field_lrclk_sel: *mut regmap_field,
    field_bclk_sel: *mut regmap_field,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct reg_field {
    reg: c_uint,
    lsb: c_uint,
    msb: c_uint,
    id_size: c_uint,
    id_offset: c_uint,
}

#[repr(C)]
struct g12a_toacodec_match_data {
    component_drv: *const snd_soc_component_driver,
    field_dat_sel: reg_field,
    field_lrclk_sel: reg_field,
    field_bclk_sel: reg_field,
}

#[repr(C)]
struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rate_min: c_uint,
    rate_max: c_uint,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_kcontrol_new {
    iface: c_uint,
    name: *const c_char,
    info: *const c_void,
    get: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    put: Option<unsafe extern "C" fn(*mut snd_kcontrol, *mut snd_ctl_elem_value) -> c_int>,
    private_value: usize,
}

#[repr(C)]
struct snd_soc_dapm_widget {
    id: c_int,
    name: *const c_char,
    reg: c_int,
    shift: c_uint,
    invert: c_uint,
    kcontrol_news: *const snd_kcontrol_new,
    num_kcontrols: c_int,
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct snd_soc_component_driver {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
    endianness: c_uint,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_uint,
    val_bits: c_uint,
    reg_stride: c_uint,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

#[repr(C)]
struct meson_codec_glue_input {
    params: meson_codec_glue_params,
}

#[repr(C)]
struct meson_codec_glue_params {
    channels_min: c_uint,
    channels_max: c_uint,
}

unsafe extern "C" {
    fn snd_soc_dapm_kcontrol_to_component(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn snd_soc_dapm_kcontrol_to_dapm(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_dapm_context;
    fn snd_soc_enum_item_to_val(e: *mut soc_enum, item: c_uint) -> c_uint;
    fn regmap_field_read(field: *mut regmap_field, val: *mut c_uint) -> c_int;
    fn snd_soc_dapm_mux_update_power(
        dapm: *mut snd_soc_dapm_context,
        kcontrol: *mut snd_kcontrol,
        mux: c_uint,
        e: *mut soc_enum,
        update: *mut c_void,
    ) -> c_int;
    fn regmap_field_write(field: *mut regmap_field, val: c_uint) -> c_int;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
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
    fn snd_soc_component_write(c: *mut snd_soc_component, reg: c_uint, val: c_uint) -> c_int;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn device_reset(dev: *mut device) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_regmap_field_alloc(
        dev: *mut device,
        regmap: *mut regmap,
        field: reg_field,
    ) -> *mut regmap_field;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

unsafe fn is_err<T>(ptr: *const T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn ptr_err<T>(ptr: *const T) -> c_int {
    ptr as isize as c_int
}

const fn reg_field(reg: c_uint, lsb: c_uint, msb: c_uint) -> reg_field {
    reg_field {
        reg,
        lsb,
        msb,
        id_size: 0,
        id_offset: 0,
    }
}

static G12A_TOACODEC_MUX_TEXT_0: &[u8] = b"I2S A\0";
static G12A_TOACODEC_MUX_TEXT_1: &[u8] = b"I2S B\0";
static G12A_TOACODEC_MUX_TEXT_2: &[u8] = b"I2S C\0";

static G12A_TOACODEC_MUX_TEXTS: [*const c_char; 3] = [
    G12A_TOACODEC_MUX_TEXT_0.as_ptr() as *const c_char,
    G12A_TOACODEC_MUX_TEXT_1.as_ptr() as *const c_char,
    G12A_TOACODEC_MUX_TEXT_2.as_ptr() as *const c_char,
];

unsafe extern "C" fn g12a_toacodec_mux_put_enum(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> c_int {
    let component = snd_soc_dapm_kcontrol_to_component(kcontrol);
    let priv_ = snd_soc_component_get_drvdata(component) as *mut g12a_toacodec;
    let dapm = snd_soc_dapm_kcontrol_to_dapm(kcontrol);
    let e = (*kcontrol).private_value as *mut soc_enum;
    let mut reg: c_uint = 0;

    if (*ucontrol).value.enumerated.item[0] >= (*e).items {
        return -EINVAL;
    }

    let mux = snd_soc_enum_item_to_val(e, (*ucontrol).value.enumerated.item[0]);
    regmap_field_read((*priv_).field_dat_sel, &mut reg);

    if mux == reg {
        return 0;
    }

    /* Force disconnect of the mux while updating */
    snd_soc_dapm_mux_update_power(dapm, kcontrol, 0, ptr::null_mut(), ptr::null_mut());

    regmap_field_write((*priv_).field_dat_sel, mux);
    regmap_field_write((*priv_).field_lrclk_sel, mux);
    regmap_field_write((*priv_).field_bclk_sel, mux);

    /*
     * FIXME:
     * On this soc, the glue gets the MCLK directly from the clock
     * controller instead of going the through the TDM interface.
     *
     * Here we assume interface A uses clock A, etc ... While it is
     * true for now, it could be different. Instead the glue should
     * find out the clock used by the interface and select the same
     * source. For that, we will need regmap backed clock mux which
     * is a work in progress
     */
    snd_soc_component_update_bits(
        component,
        (*e).reg,
        CTRL0_MCLK_SEL,
        field_prep(CTRL0_MCLK_SEL, mux),
    );

    snd_soc_dapm_mux_update_power(dapm, kcontrol, mux, e, ptr::null_mut());

    1
}

static G12A_TOACODEC_MUX_ENUM: soc_enum = soc_enum {
    reg: TOACODEC_CTRL0,
    shift_l: CTRL0_DAT_SEL_LSB,
    shift_r: CTRL0_DAT_SEL_LSB,
    items: 3,
    texts: G12A_TOACODEC_MUX_TEXTS.as_ptr(),
    values: ptr::null(),
    mask: 3,
};

static SM1_TOACODEC_MUX_ENUM: soc_enum = soc_enum {
    reg: TOACODEC_CTRL0,
    shift_l: CTRL0_DAT_SEL_SM1_LSB,
    shift_r: CTRL0_DAT_SEL_SM1_LSB,
    items: 3,
    texts: G12A_TOACODEC_MUX_TEXTS.as_ptr(),
    values: ptr::null(),
    mask: 3,
};

static G12A_TOACODEC_MUX_NAME: &[u8] = b"Source\0";
static G12A_TOACODEC_MUX: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0,
    name: G12A_TOACODEC_MUX_NAME.as_ptr() as *const c_char,
    info: ptr::null(),
    get: Some(snd_soc_dapm_get_enum_double),
    put: Some(g12a_toacodec_mux_put_enum),
    private_value: &G12A_TOACODEC_MUX_ENUM as *const soc_enum as usize,
};

static SM1_TOACODEC_MUX: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0,
    name: G12A_TOACODEC_MUX_NAME.as_ptr() as *const c_char,
    info: ptr::null(),
    get: Some(snd_soc_dapm_get_enum_double),
    put: Some(g12a_toacodec_mux_put_enum),
    private_value: &SM1_TOACODEC_MUX_ENUM as *const soc_enum as usize,
};

static SWITCH_NAME: &[u8] = b"Switch\0";
static LANE_SELECT_NAME: &[u8] = b"Lane Select\0";
static SRC_NAME: &[u8] = b"SRC\0";
static OUT_EN_NAME: &[u8] = b"OUT EN\0";
static OUT_CAPTURE_NAME: &[u8] = b"OUT Capture\0";
static IN_A_PLAYBACK_NAME: &[u8] = b"IN A Playback\0";
static IN_B_PLAYBACK_NAME: &[u8] = b"IN B Playback\0";
static IN_C_PLAYBACK_NAME: &[u8] = b"IN C Playback\0";
static IN_A_NAME: &[u8] = b"IN A\0";
static IN_B_NAME: &[u8] = b"IN B\0";
static IN_C_NAME: &[u8] = b"IN C\0";
static OUT_NAME: &[u8] = b"OUT\0";

static G12A_TOACODEC_OUT_ENABLE: snd_kcontrol_new = snd_kcontrol_new {
    iface: 0,
    name: SWITCH_NAME.as_ptr() as *const c_char,
    info: ptr::null(),
    get: None,
    put: None,
    private_value: ((TOACODEC_CTRL0 as usize) << 24)
        | ((CTRL0_ENABLE_SHIFT as usize) << 16)
        | ((1_usize) << 8),
};

static G12A_TOACODEC_WIDGETS: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget {
        id: 0,
        name: SRC_NAME.as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol_news: &G12A_TOACODEC_MUX,
        num_kcontrols: 1,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: OUT_EN_NAME.as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol_news: &G12A_TOACODEC_OUT_ENABLE,
        num_kcontrols: 1,
    },
];

static SM1_TOACODEC_WIDGETS: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget {
        id: 0,
        name: SRC_NAME.as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol_news: &SM1_TOACODEC_MUX,
        num_kcontrols: 1,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: OUT_EN_NAME.as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol_news: &G12A_TOACODEC_OUT_ENABLE,
        num_kcontrols: 1,
    },
];

unsafe extern "C" fn g12a_toacodec_input_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let ret = meson_codec_glue_input_hw_params(substream, params, dai);
    if ret != 0 {
        return ret;
    }

    /* The glue will provide 1 lane out of the 4 to the output */
    let data = meson_codec_glue_input_get_data(dai);
    (*data).params.channels_min = core::cmp::min(TOACODEC_OUT_CHMAX, (*data).params.channels_min);
    (*data).params.channels_max = core::cmp::min(TOACODEC_OUT_CHMAX, (*data).params.channels_max);

    0
}

static G12A_TOACODEC_INPUT_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(meson_codec_glue_input_dai_probe),
    remove: Some(meson_codec_glue_input_dai_remove),
    hw_params: Some(g12a_toacodec_input_hw_params),
    set_fmt: Some(meson_codec_glue_input_set_fmt),
    startup: None,
};

static G12A_TOACODEC_OUTPUT_OPS: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: None,
    remove: None,
    hw_params: None,
    set_fmt: None,
    startup: Some(meson_codec_glue_output_startup),
};

const fn toacodec_stream(stream_name: *const c_char, xchmax: c_uint) -> snd_soc_pcm_stream {
    snd_soc_pcm_stream {
        stream_name,
        channels_min: 1,
        channels_max: xchmax,
        rate_min: 5512,
        rate_max: 192000,
        formats: AXG_TDM_FORMATS,
    }
}

static IN_A_PLAYBACK_STREAM_NAME: &[u8] = b"IN A Playback\0";
static IN_B_PLAYBACK_STREAM_NAME: &[u8] = b"IN B Playback\0";
static IN_C_PLAYBACK_STREAM_NAME: &[u8] = b"IN C Playback\0";
static OUT_CAPTURE_STREAM_NAME: &[u8] = b"OUT Capture\0";

static mut G12A_TOACODEC_DAI_DRV: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver {
        name: IN_A_NAME.as_ptr() as *const c_char,
        id: TOACODEC_IN_A,
        playback: toacodec_stream(IN_A_PLAYBACK_STREAM_NAME.as_ptr() as *const c_char, 8),
        capture: toacodec_stream(ptr::null(), 0),
        ops: &G12A_TOACODEC_INPUT_OPS,
    },
    snd_soc_dai_driver {
        name: IN_B_NAME.as_ptr() as *const c_char,
        id: TOACODEC_IN_B,
        playback: toacodec_stream(IN_B_PLAYBACK_STREAM_NAME.as_ptr() as *const c_char, 8),
        capture: toacodec_stream(ptr::null(), 0),
        ops: &G12A_TOACODEC_INPUT_OPS,
    },
    snd_soc_dai_driver {
        name: IN_C_NAME.as_ptr() as *const c_char,
        id: TOACODEC_IN_C,
        playback: toacodec_stream(IN_C_PLAYBACK_STREAM_NAME.as_ptr() as *const c_char, 8),
        capture: toacodec_stream(ptr::null(), 0),
        ops: &G12A_TOACODEC_INPUT_OPS,
    },
    snd_soc_dai_driver {
        name: OUT_NAME.as_ptr() as *const c_char,
        id: TOACODEC_OUT,
        playback: toacodec_stream(ptr::null(), 0),
        capture: toacodec_stream(OUT_CAPTURE_STREAM_NAME.as_ptr() as *const c_char, TOACODEC_OUT_CHMAX),
        ops: &G12A_TOACODEC_OUTPUT_OPS,
    },
];

unsafe extern "C" fn g12a_toacodec_component_probe(c: *mut snd_soc_component) -> c_int {
    /* Initialize the static clock parameters */
    snd_soc_component_write(c, TOACODEC_CTRL0, CTRL0_BLK_CAP_INV)
}

unsafe extern "C" fn sm1_toacodec_component_probe(c: *mut snd_soc_component) -> c_int {
    /* Initialize the static clock parameters */
    snd_soc_component_write(c, TOACODEC_CTRL0, CTRL0_BLK_CAP_INV_SM1)
}

static G12A_TOACODEC_ROUTES: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route {
        sink: SRC_NAME.as_ptr() as *const c_char,
        control: G12A_TOACODEC_MUX_TEXT_0.as_ptr() as *const c_char,
        source: IN_A_PLAYBACK_NAME.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: SRC_NAME.as_ptr() as *const c_char,
        control: G12A_TOACODEC_MUX_TEXT_1.as_ptr() as *const c_char,
        source: IN_B_PLAYBACK_NAME.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: SRC_NAME.as_ptr() as *const c_char,
        control: G12A_TOACODEC_MUX_TEXT_2.as_ptr() as *const c_char,
        source: IN_C_PLAYBACK_NAME.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: OUT_EN_NAME.as_ptr() as *const c_char,
        control: SWITCH_NAME.as_ptr() as *const c_char,
        source: SRC_NAME.as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: OUT_CAPTURE_NAME.as_ptr() as *const c_char,
        control: ptr::null(),
        source: OUT_EN_NAME.as_ptr() as *const c_char,
    },
];

static G12A_TOACODEC_CONTROLS: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    iface: 0,
    name: LANE_SELECT_NAME.as_ptr() as *const c_char,
    info: ptr::null(),
    get: None,
    put: None,
    private_value: ((TOACODEC_CTRL0 as usize) << 24) | ((CTRL0_LANE_SEL as usize) << 16) | 3,
}];

static SM1_TOACODEC_CONTROLS: [snd_kcontrol_new; 1] = [snd_kcontrol_new {
    iface: 0,
    name: LANE_SELECT_NAME.as_ptr() as *const c_char,
    info: ptr::null(),
    get: None,
    put: None,
    private_value: ((TOACODEC_CTRL0 as usize) << 24) | ((CTRL0_LANE_SEL_SM1 as usize) << 16) | 3,
}];

static G12A_TOACODEC_COMPONENT_DRV: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(g12a_toacodec_component_probe),
    controls: G12A_TOACODEC_CONTROLS.as_ptr(),
    num_controls: G12A_TOACODEC_CONTROLS.len() as c_uint,
    dapm_widgets: G12A_TOACODEC_WIDGETS.as_ptr(),
    num_dapm_widgets: G12A_TOACODEC_WIDGETS.len() as c_uint,
    dapm_routes: G12A_TOACODEC_ROUTES.as_ptr(),
    num_dapm_routes: G12A_TOACODEC_ROUTES.len() as c_uint,
    endianness: 1,
};

static SM1_TOACODEC_COMPONENT_DRV: snd_soc_component_driver = snd_soc_component_driver {
    probe: Some(sm1_toacodec_component_probe),
    controls: SM1_TOACODEC_CONTROLS.as_ptr(),
    num_controls: SM1_TOACODEC_CONTROLS.len() as c_uint,
    dapm_widgets: SM1_TOACODEC_WIDGETS.as_ptr(),
    num_dapm_widgets: SM1_TOACODEC_WIDGETS.len() as c_uint,
    dapm_routes: G12A_TOACODEC_ROUTES.as_ptr(),
    num_dapm_routes: G12A_TOACODEC_ROUTES.len() as c_uint,
    endianness: 1,
};

static G12A_TOACODEC_REGMAP_CFG: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
};

static G12A_TOACODEC_MATCH_DATA: g12a_toacodec_match_data = g12a_toacodec_match_data {
    component_drv: &G12A_TOACODEC_COMPONENT_DRV,
    field_dat_sel: reg_field(TOACODEC_CTRL0, 14, 15),
    field_lrclk_sel: reg_field(TOACODEC_CTRL0, 8, 9),
    field_bclk_sel: reg_field(TOACODEC_CTRL0, 4, 5),
};

static SM1_TOACODEC_MATCH_DATA: g12a_toacodec_match_data = g12a_toacodec_match_data {
    component_drv: &SM1_TOACODEC_COMPONENT_DRV,
    field_dat_sel: reg_field(TOACODEC_CTRL0, 18, 19),
    field_lrclk_sel: reg_field(TOACODEC_CTRL0, 12, 14),
    field_bclk_sel: reg_field(TOACODEC_CTRL0, 4, 6),
};

static G12A_COMPATIBLE: &[u8] = b"amlogic,g12a-toacodec\0";
static SM1_COMPATIBLE: &[u8] = b"amlogic,sm1-toacodec\0";

static G12A_TOACODEC_OF_MATCH: [of_device_id; 3] = [
    of_device_id {
        compatible: G12A_COMPATIBLE.as_ptr() as *const c_char,
        data: &G12A_TOACODEC_MATCH_DATA as *const g12a_toacodec_match_data as *const c_void,
    },
    of_device_id {
        compatible: SM1_COMPATIBLE.as_ptr() as *const c_char,
        data: &SM1_TOACODEC_MATCH_DATA as *const g12a_toacodec_match_data as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, g12a_toacodec_of_match);

unsafe extern "C" fn g12a_toacodec_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let data = device_get_match_data(dev) as *const g12a_toacodec_match_data;
    if data.is_null() {
        dev_err(dev, b"failed to match device\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    let priv_ = devm_kzalloc(dev, core::mem::size_of::<g12a_toacodec>(), GFP_KERNEL)
        as *mut g12a_toacodec;
    if priv_.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, priv_ as *mut c_void);

    let ret = device_reset(dev);
    if ret != 0 {
        return dev_err_probe(dev, ret, b"failed to reset device\n\0".as_ptr() as *const c_char);
    }

    let regs = devm_platform_ioremap_resource(pdev, 0);
    if is_err(regs) {
        return ptr_err(regs);
    }

    let map = devm_regmap_init_mmio(dev, regs, &G12A_TOACODEC_REGMAP_CFG);
    if is_err(map) {
        dev_err(
            dev,
            b"failed to init regmap: %ld\n\0".as_ptr() as *const c_char,
            ptr_err(map) as c_long,
        );
        return ptr_err(map);
    }

    (*priv_).field_dat_sel = devm_regmap_field_alloc(dev, map, (*data).field_dat_sel);
    if is_err((*priv_).field_dat_sel) {
        return ptr_err((*priv_).field_dat_sel);
    }

    (*priv_).field_lrclk_sel = devm_regmap_field_alloc(dev, map, (*data).field_lrclk_sel);
    if is_err((*priv_).field_lrclk_sel) {
        return ptr_err((*priv_).field_lrclk_sel);
    }

    (*priv_).field_bclk_sel = devm_regmap_field_alloc(dev, map, (*data).field_bclk_sel);
    if is_err((*priv_).field_bclk_sel) {
        return ptr_err((*priv_).field_bclk_sel);
    }

    devm_snd_soc_register_component(
        dev,
        (*data).component_drv,
        G12A_TOACODEC_DAI_DRV.as_mut_ptr(),
        G12A_TOACODEC_DAI_DRV.len() as c_int,
    )
}

static mut G12A_TOACODEC_PDRV: platform_driver = platform_driver {
    driver: device_driver {
        name: G12A_TOACODEC_DRV_NAME.as_ptr() as *const c_char,
        of_match_table: G12A_TOACODEC_OF_MATCH.as_ptr(),
    },
    probe: Some(g12a_toacodec_probe),
};

// module_platform_driver(g12a_toacodec_pdrv);
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_DESCRIPTION("Amlogic G12a To Internal DAC Codec Driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
