// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// Depends on the Rust equivalents of:
// linux/module.h, linux/of_platform.h, linux/regmap.h,
// sound/soc.h, sound/soc-dai.h, and "axg-tdm-formatter.h".

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const TDMIN_CTRL: c_uint = 0x00;
const TDMIN_CTRL_ENABLE: c_uint = BIT(31);
const TDMIN_CTRL_I2S_MODE: c_uint = BIT(30);
const TDMIN_CTRL_RST_OUT: c_uint = BIT(29);
const TDMIN_CTRL_RST_IN: c_uint = BIT(28);
const TDMIN_CTRL_WS_INV: c_uint = BIT(25);
const TDMIN_CTRL_SEL_SHIFT: c_uint = 20;
const TDMIN_CTRL_IN_BIT_SKEW_MASK: c_uint = GENMASK(18, 16);
const fn TDMIN_CTRL_IN_BIT_SKEW(x: c_uint) -> c_uint {
    x << 16
}
const TDMIN_CTRL_LSB_FIRST: c_uint = BIT(5);
const TDMIN_CTRL_BITNUM_MASK: c_uint = GENMASK(4, 0);
const fn TDMIN_CTRL_BITNUM(x: c_uint) -> c_uint {
    x << 0
}
const TDMIN_SWAP: c_uint = 0x04;
const TDMIN_MASK0: c_uint = 0x08;
const TDMIN_MASK1: c_uint = 0x0c;
const TDMIN_MASK2: c_uint = 0x10;
const TDMIN_MASK3: c_uint = 0x14;
const TDMIN_STAT: c_uint = 0x18;
const TDMIN_MUTE_VAL: c_uint = 0x1c;
const TDMIN_MUTE0: c_uint = 0x20;
const TDMIN_MUTE1: c_uint = 0x24;
const TDMIN_MUTE2: c_uint = 0x28;
const TDMIN_MUTE3: c_uint = 0x2c;

extern "C" {
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dai_dma_data_get_capture(dai: *mut snd_soc_dai) -> *mut axg_tdm_stream;
    fn axg_tdm_lrclk_invert(fmt: c_uint) -> bool;
    fn axg_tdm_formatter_set_channel_masks(
        map: *mut regmap,
        ts: *mut axg_tdm_stream,
        offset: c_uint,
    ) -> c_int;
    fn axg_tdm_formatter_event(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;
    fn axg_tdm_formatter_probe(pdev: *mut platform_device) -> c_int;
    fn pr_err(fmt: *const c_char, ...);
}

static axg_tdmin_regmap_cfg: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    max_register: TDMIN_MUTE3,
};

static axg_tdmin_sel_texts: [*const c_char; 16] = [
    b"IN 0\0".as_ptr() as *const c_char,
    b"IN 1\0".as_ptr() as *const c_char,
    b"IN 2\0".as_ptr() as *const c_char,
    b"IN 3\0".as_ptr() as *const c_char,
    b"IN 4\0".as_ptr() as *const c_char,
    b"IN 5\0".as_ptr() as *const c_char,
    b"IN 6\0".as_ptr() as *const c_char,
    b"IN 7\0".as_ptr() as *const c_char,
    b"IN 8\0".as_ptr() as *const c_char,
    b"IN 9\0".as_ptr() as *const c_char,
    b"IN 10\0".as_ptr() as *const c_char,
    b"IN 11\0".as_ptr() as *const c_char,
    b"IN 12\0".as_ptr() as *const c_char,
    b"IN 13\0".as_ptr() as *const c_char,
    b"IN 14\0".as_ptr() as *const c_char,
    b"IN 15\0".as_ptr() as *const c_char,
];

/* Change to special mux control to reset dapm */
SOC_ENUM_SINGLE_DECL!(
    axg_tdmin_sel_enum,
    TDMIN_CTRL,
    TDMIN_CTRL_SEL_SHIFT,
    axg_tdmin_sel_texts
);

static axg_tdmin_in_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM!(b"Input Source\0".as_ptr() as *const c_char, axg_tdmin_sel_enum);

unsafe fn axg_tdmin_get_be(w: *mut snd_soc_dapm_widget) -> *mut snd_soc_dai {
    let mut p: *mut snd_soc_dapm_path;
    let mut be: *mut snd_soc_dai;

    snd_soc_dapm_widget_for_each_source_path!(w, p, {
        if !(*p).connect {
            continue;
        }

        if (*(*p).source).id == snd_soc_dapm_dai_out {
            return (*(*p).source).priv as *mut snd_soc_dai;
        }

        be = axg_tdmin_get_be((*p).source);
        if !be.is_null() {
            return be;
        }
    });

    ptr::null_mut()
}

unsafe fn axg_tdmin_get_tdm_stream(w: *mut snd_soc_dapm_widget) -> *mut axg_tdm_stream {
    let be: *mut snd_soc_dai = axg_tdmin_get_be(w);

    if be.is_null() {
        return ptr::null_mut();
    }

    snd_soc_dai_dma_data_get_capture(be)
}

unsafe fn axg_tdmin_enable(map: *mut regmap) {
    /* Apply both reset */
    regmap_update_bits(map, TDMIN_CTRL, TDMIN_CTRL_RST_OUT | TDMIN_CTRL_RST_IN, 0);

    /* Clear out reset before in reset */
    regmap_update_bits(map, TDMIN_CTRL, TDMIN_CTRL_RST_OUT, TDMIN_CTRL_RST_OUT);
    regmap_update_bits(map, TDMIN_CTRL, TDMIN_CTRL_RST_IN, TDMIN_CTRL_RST_IN);

    /* Actually enable tdmin */
    regmap_update_bits(map, TDMIN_CTRL, TDMIN_CTRL_ENABLE, TDMIN_CTRL_ENABLE);
}

unsafe fn axg_tdmin_disable(map: *mut regmap) {
    regmap_update_bits(map, TDMIN_CTRL, TDMIN_CTRL_ENABLE, 0);
}

unsafe fn axg_tdmin_prepare(
    map: *mut regmap,
    quirks: *const axg_tdm_formatter_hw,
    ts: *mut axg_tdm_stream,
) -> c_int {
    let mut val: c_uint;
    let mut skew: c_uint = (*quirks).skew_offset;

    /* Set stream skew */
    match (*(*ts).iface).fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_DSP_A => {
            skew = skew.wrapping_add(1);
        }
        SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_DSP_B => {}
        _ => {
            pr_err(
                b"Unsupported format: %u\n\0".as_ptr() as *const c_char,
                (*(*ts).iface).fmt & SND_SOC_DAIFMT_FORMAT_MASK,
            );
            return -EINVAL;
        }
    }

    val = TDMIN_CTRL_IN_BIT_SKEW(skew);

    /* Set stream format mode */
    match (*(*ts).iface).fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_RIGHT_J => {
            val |= TDMIN_CTRL_I2S_MODE;
        }
        _ => {}
    }

    /* If the sample clock is inverted, invert it back for the formatter */
    if axg_tdm_lrclk_invert((*(*ts).iface).fmt) {
        val |= TDMIN_CTRL_WS_INV;
    }

    /* Set the slot width */
    val |= TDMIN_CTRL_BITNUM((*(*ts).iface).slot_width - 1);

    /*
     * The following also reset LSB_FIRST which result in the formatter
     * placing the first bit received at bit 31
     */
    regmap_update_bits(
        map,
        TDMIN_CTRL,
        TDMIN_CTRL_IN_BIT_SKEW_MASK
            | TDMIN_CTRL_WS_INV
            | TDMIN_CTRL_I2S_MODE
            | TDMIN_CTRL_LSB_FIRST
            | TDMIN_CTRL_BITNUM_MASK,
        val,
    );

    /* Set static swap mask configuration */
    regmap_write(map, TDMIN_SWAP, 0x76543210);

    axg_tdm_formatter_set_channel_masks(map, ts, TDMIN_MASK0)
}

static axg_tdmin_dapm_widgets: [snd_soc_dapm_widget; 19] = [
    SND_SOC_DAPM_AIF_IN!(b"IN 0\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 1\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 2\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 3\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 4\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 5\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 6\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 7\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 8\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 9\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 10\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 11\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 12\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 13\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 14\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!(b"IN 15\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX!(b"SRC SEL\0".as_ptr() as *const c_char, SND_SOC_NOPM, 0, 0, &axg_tdmin_in_mux),
    SND_SOC_DAPM_PGA_E!(
        b"DEC\0".as_ptr() as *const c_char,
        SND_SOC_NOPM,
        0,
        0,
        ptr::null(),
        0,
        axg_tdm_formatter_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD
    ),
    SND_SOC_DAPM_AIF_OUT!(b"OUT\0".as_ptr() as *const c_char, ptr::null(), 0, SND_SOC_NOPM, 0, 0),
];

static axg_tdmin_dapm_routes: [snd_soc_dapm_route; 18] = [
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 0\0".as_ptr() as *const c_char, source: b"IN 0\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 1\0".as_ptr() as *const c_char, source: b"IN 1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 2\0".as_ptr() as *const c_char, source: b"IN 2\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 3\0".as_ptr() as *const c_char, source: b"IN 3\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 4\0".as_ptr() as *const c_char, source: b"IN 4\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 5\0".as_ptr() as *const c_char, source: b"IN 5\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 6\0".as_ptr() as *const c_char, source: b"IN 6\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 7\0".as_ptr() as *const c_char, source: b"IN 7\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 8\0".as_ptr() as *const c_char, source: b"IN 8\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 9\0".as_ptr() as *const c_char, source: b"IN 9\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 10\0".as_ptr() as *const c_char, source: b"IN 10\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 11\0".as_ptr() as *const c_char, source: b"IN 11\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 12\0".as_ptr() as *const c_char, source: b"IN 12\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 13\0".as_ptr() as *const c_char, source: b"IN 13\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 14\0".as_ptr() as *const c_char, source: b"IN 14\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"SRC SEL\0".as_ptr() as *const c_char, control: b"IN 15\0".as_ptr() as *const c_char, source: b"IN 15\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"DEC\0".as_ptr() as *const c_char, control: ptr::null(), source: b"SRC SEL\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"OUT\0".as_ptr() as *const c_char, control: ptr::null(), source: b"DEC\0".as_ptr() as *const c_char },
];

static axg_tdmin_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: axg_tdmin_dapm_widgets.as_ptr(),
    num_dapm_widgets: axg_tdmin_dapm_widgets.len() as c_uint,
    dapm_routes: axg_tdmin_dapm_routes.as_ptr(),
    num_dapm_routes: axg_tdmin_dapm_routes.len() as c_uint,
};

static axg_tdmin_ops: axg_tdm_formatter_ops = axg_tdm_formatter_ops {
    get_stream: Some(axg_tdmin_get_tdm_stream),
    prepare: Some(axg_tdmin_prepare),
    enable: Some(axg_tdmin_enable),
    disable: Some(axg_tdmin_disable),
};

static axg_tdmin_quirks: axg_tdm_formatter_hw = axg_tdm_formatter_hw { skew_offset: 3 };

static axg_tdmin_drv: axg_tdm_formatter_driver = axg_tdm_formatter_driver {
    component_drv: &axg_tdmin_component_drv,
    regmap_cfg: &axg_tdmin_regmap_cfg,
    ops: &axg_tdmin_ops,
    quirks: &axg_tdmin_quirks,
};

static axg_tdmin_of_match: [of_device_id; 4] = [
    of_device_id {
        compatible: b"amlogic,axg-tdmin\0".as_ptr() as *const c_char,
        data: &axg_tdmin_drv as *const axg_tdm_formatter_driver as *const c_void,
    },
    of_device_id {
        compatible: b"amlogic,g12a-tdmin\0".as_ptr() as *const c_char,
        data: &axg_tdmin_drv as *const axg_tdm_formatter_driver as *const c_void,
    },
    of_device_id {
        compatible: b"amlogic,sm1-tdmin\0".as_ptr() as *const c_char,
        data: &axg_tdmin_drv as *const axg_tdm_formatter_driver as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
MODULE_DEVICE_TABLE!(of, axg_tdmin_of_match);

static mut axg_tdmin_pdrv: platform_driver = platform_driver {
    probe: Some(axg_tdm_formatter_probe),
    driver: device_driver {
        name: b"axg-tdmin\0".as_ptr() as *const c_char,
        of_match_table: axg_tdmin_of_match.as_ptr(),
    },
};
module_platform_driver!(axg_tdmin_pdrv);

MODULE_DESCRIPTION!(b"Amlogic AXG TDM input formatter driver\0".as_ptr() as *const c_char);
MODULE_AUTHOR!(b"Jerome Brunet <jbrunet@baylibre.com>\0".as_ptr() as *const c_char);
MODULE_LICENSE!(b"GPL v2\0".as_ptr() as *const c_char);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
