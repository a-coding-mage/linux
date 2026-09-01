// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// Dependencies from the original C includes:
// linux/module.h, linux/of_platform.h, linux/regmap.h,
// sound/soc.h, sound/soc-dai.h, and "axg-tdm-formatter.h".

const TDMOUT_CTRL0: u32 = 0x00;
const TDMOUT_CTRL0_BITNUM_MASK: u32 = GENMASK(4, 0);
const fn TDMOUT_CTRL0_BITNUM(x: u32) -> u32 {
    x << 0
}
const TDMOUT_CTRL0_SLOTNUM_MASK: u32 = GENMASK(9, 5);
const fn TDMOUT_CTRL0_SLOTNUM(x: u32) -> u32 {
    x << 5
}
const TDMOUT_CTRL0_INIT_BITNUM_MASK: u32 = GENMASK(19, 15);
const fn TDMOUT_CTRL0_INIT_BITNUM(x: u32) -> u32 {
    x << 15
}
const TDMOUT_CTRL0_ENABLE: u32 = BIT(31);
const TDMOUT_CTRL0_RST_OUT: u32 = BIT(29);
const TDMOUT_CTRL0_RST_IN: u32 = BIT(28);
const TDMOUT_CTRL1: u32 = 0x04;
const TDMOUT_CTRL1_TYPE_MASK: u32 = GENMASK(6, 4);
const fn TDMOUT_CTRL1_TYPE(x: u32) -> u32 {
    x << 4
}
const SM1_TDMOUT_CTRL1_GAIN_EN: u32 = 7;
const TDMOUT_CTRL1_MSB_POS_MASK: u32 = GENMASK(12, 8);
const fn TDMOUT_CTRL1_MSB_POS(x: u32) -> u32 {
    x << 8
}
const TDMOUT_CTRL1_SEL_SHIFT: u32 = 24;
const TDMOUT_CTRL1_GAIN_EN: u32 = 26;
const TDMOUT_CTRL1_WS_INV: u32 = BIT(28);
const TDMOUT_SWAP: u32 = 0x08;
const TDMOUT_MASK0: u32 = 0x0c;
const TDMOUT_MASK1: u32 = 0x10;
const TDMOUT_MASK2: u32 = 0x14;
const TDMOUT_MASK3: u32 = 0x18;
const TDMOUT_STAT: u32 = 0x1c;
const TDMOUT_GAIN0: u32 = 0x20;
const TDMOUT_GAIN1: u32 = 0x24;
const TDMOUT_MUTE_VAL: u32 = 0x28;
const TDMOUT_MUTE0: u32 = 0x2c;
const TDMOUT_MUTE1: u32 = 0x30;
const TDMOUT_MUTE2: u32 = 0x34;
const TDMOUT_MUTE3: u32 = 0x38;
const TDMOUT_MASK_VAL: u32 = 0x3c;

static axg_tdmout_regmap_cfg: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    max_register: TDMOUT_MASK_VAL,
};

unsafe fn axg_tdmout_get_be(w: *mut snd_soc_dapm_widget) -> *mut snd_soc_dai {
    let mut p: *mut snd_soc_dapm_path;
    let mut be: *mut snd_soc_dai;

    snd_soc_dapm_widget_for_each_sink_path!(w, p, {
        if !(*p).connect {
            continue;
        }

        if (*(*p).sink).id == snd_soc_dapm_dai_in {
            return (*(*p).sink).priv as *mut snd_soc_dai;
        }

        be = axg_tdmout_get_be((*p).sink);
        if !be.is_null() {
            return be;
        }
    });

    core::ptr::null_mut()
}

unsafe fn axg_tdmout_get_tdm_stream(w: *mut snd_soc_dapm_widget) -> *mut axg_tdm_stream {
    let be: *mut snd_soc_dai = axg_tdmout_get_be(w);

    if be.is_null() {
        return core::ptr::null_mut();
    }

    snd_soc_dai_dma_data_get_playback(be)
}

unsafe fn axg_tdmout_enable(map: *mut regmap) {
    /* Apply both reset */
    regmap_update_bits(
        map,
        TDMOUT_CTRL0,
        TDMOUT_CTRL0_RST_OUT | TDMOUT_CTRL0_RST_IN,
        0,
    );

    /* Clear out reset before in reset */
    regmap_update_bits(
        map,
        TDMOUT_CTRL0,
        TDMOUT_CTRL0_RST_OUT,
        TDMOUT_CTRL0_RST_OUT,
    );
    regmap_update_bits(map, TDMOUT_CTRL0, TDMOUT_CTRL0_RST_IN, TDMOUT_CTRL0_RST_IN);

    /* Actually enable tdmout */
    regmap_update_bits(
        map,
        TDMOUT_CTRL0,
        TDMOUT_CTRL0_ENABLE,
        TDMOUT_CTRL0_ENABLE,
    );
}

unsafe fn axg_tdmout_disable(map: *mut regmap) {
    regmap_update_bits(map, TDMOUT_CTRL0, TDMOUT_CTRL0_ENABLE, 0);
}

unsafe fn axg_tdmout_prepare(
    map: *mut regmap,
    quirks: *const axg_tdm_formatter_hw,
    ts: *mut axg_tdm_stream,
) -> i32 {
    let mut val: u32;
    let mut skew: u32 = (*quirks).skew_offset;

    /* Set the stream skew */
    match (*(*ts).iface).fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        SND_SOC_DAIFMT_I2S | SND_SOC_DAIFMT_DSP_A => {}

        SND_SOC_DAIFMT_LEFT_J | SND_SOC_DAIFMT_DSP_B => {
            skew += 1;
        }

        _ => {
            pr_err!(
                "Unsupported format: %u\n",
                (*(*ts).iface).fmt & SND_SOC_DAIFMT_FORMAT_MASK
            );
            return -EINVAL;
        }
    }

    val = TDMOUT_CTRL0_INIT_BITNUM(skew);

    /* Set the slot width */
    val |= TDMOUT_CTRL0_BITNUM((*(*ts).iface).slot_width - 1);

    /* Set the slot number */
    val |= TDMOUT_CTRL0_SLOTNUM((*(*ts).iface).slots - 1);

    regmap_update_bits(
        map,
        TDMOUT_CTRL0,
        TDMOUT_CTRL0_INIT_BITNUM_MASK | TDMOUT_CTRL0_BITNUM_MASK | TDMOUT_CTRL0_SLOTNUM_MASK,
        val,
    );

    /* Set the sample width */
    val = TDMOUT_CTRL1_MSB_POS((*ts).width - 1);

    /* FIFO data are arranged in chunks of 64bits */
    match (*ts).physical_width {
        8 => {
            /* 8 samples of 8 bits */
            val |= TDMOUT_CTRL1_TYPE(0);
        }
        16 => {
            /* 4 samples of 16 bits - right justified */
            val |= TDMOUT_CTRL1_TYPE(2);
        }
        32 => {
            /* 2 samples of 32 bits - right justified */
            val |= TDMOUT_CTRL1_TYPE(4);
        }
        _ => {
            pr_err!("Unsupported physical width: %u\n", (*ts).physical_width);
            return -EINVAL;
        }
    }

    /* If the sample clock is inverted, invert it back for the formatter */
    if axg_tdm_lrclk_invert((*(*ts).iface).fmt) {
        val |= TDMOUT_CTRL1_WS_INV;
    }

    regmap_update_bits(
        map,
        TDMOUT_CTRL1,
        TDMOUT_CTRL1_TYPE_MASK | TDMOUT_CTRL1_MSB_POS_MASK | TDMOUT_CTRL1_WS_INV,
        val,
    );

    /* Set static swap mask configuration */
    regmap_write(map, TDMOUT_SWAP, 0x76543210);

    axg_tdm_formatter_set_channel_masks(map, ts, TDMOUT_MASK0)
}

static axg_tdmout_controls: [snd_kcontrol_new; 5] = [
    SOC_DOUBLE!("Lane 0 Volume", TDMOUT_GAIN0, 0, 8, 255, 0),
    SOC_DOUBLE!("Lane 1 Volume", TDMOUT_GAIN0, 16, 24, 255, 0),
    SOC_DOUBLE!("Lane 2 Volume", TDMOUT_GAIN1, 0, 8, 255, 0),
    SOC_DOUBLE!("Lane 3 Volume", TDMOUT_GAIN1, 16, 24, 255, 0),
    SOC_SINGLE!("Gain Enable Switch", TDMOUT_CTRL1, TDMOUT_CTRL1_GAIN_EN, 1, 0),
];

static axg_tdmout_sel_texts: [&'static str; 3] = ["IN 0", "IN 1", "IN 2"];

SOC_ENUM_SINGLE_DECL!(
    axg_tdmout_sel_enum,
    TDMOUT_CTRL1,
    TDMOUT_CTRL1_SEL_SHIFT,
    axg_tdmout_sel_texts
);

static axg_tdmout_in_mux: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Input Source", axg_tdmout_sel_enum);

static axg_tdmout_dapm_widgets: [snd_soc_dapm_widget; 6] = [
    SND_SOC_DAPM_AIF_IN!("IN 0", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("IN 1", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("IN 2", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX!("SRC SEL", SND_SOC_NOPM, 0, 0, &axg_tdmout_in_mux),
    SND_SOC_DAPM_PGA_E!(
        "ENC",
        SND_SOC_NOPM,
        0,
        0,
        core::ptr::null(),
        0,
        axg_tdm_formatter_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD
    ),
    SND_SOC_DAPM_AIF_OUT!("OUT", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
];

static axg_tdmout_dapm_routes: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route {
        sink: "SRC SEL",
        control: "IN 0",
        source: "IN 0",
    },
    snd_soc_dapm_route {
        sink: "SRC SEL",
        control: "IN 1",
        source: "IN 1",
    },
    snd_soc_dapm_route {
        sink: "SRC SEL",
        control: "IN 2",
        source: "IN 2",
    },
    snd_soc_dapm_route {
        sink: "ENC",
        control: core::ptr::null(),
        source: "SRC SEL",
    },
    snd_soc_dapm_route {
        sink: "OUT",
        control: core::ptr::null(),
        source: "ENC",
    },
];

static axg_tdmout_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    controls: axg_tdmout_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&axg_tdmout_controls),
    dapm_widgets: axg_tdmout_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&axg_tdmout_dapm_widgets),
    dapm_routes: axg_tdmout_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&axg_tdmout_dapm_routes),
};

static axg_tdmout_ops: axg_tdm_formatter_ops = axg_tdm_formatter_ops {
    get_stream: Some(axg_tdmout_get_tdm_stream),
    prepare: Some(axg_tdmout_prepare),
    enable: Some(axg_tdmout_enable),
    disable: Some(axg_tdmout_disable),
};

static axg_tdmout_quirks: axg_tdm_formatter_hw = axg_tdm_formatter_hw { skew_offset: 1 };

static axg_tdmout_drv: axg_tdm_formatter_driver = axg_tdm_formatter_driver {
    component_drv: &axg_tdmout_component_drv,
    regmap_cfg: &axg_tdmout_regmap_cfg,
    ops: &axg_tdmout_ops,
    quirks: &axg_tdmout_quirks,
};

static g12a_tdmout_quirks: axg_tdm_formatter_hw = axg_tdm_formatter_hw { skew_offset: 2 };

static g12a_tdmout_drv: axg_tdm_formatter_driver = axg_tdm_formatter_driver {
    component_drv: &axg_tdmout_component_drv,
    regmap_cfg: &axg_tdmout_regmap_cfg,
    ops: &axg_tdmout_ops,
    quirks: &g12a_tdmout_quirks,
};

static sm1_tdmout_controls: [snd_kcontrol_new; 5] = [
    SOC_DOUBLE!("Lane 0 Volume", TDMOUT_GAIN0, 0, 8, 255, 0),
    SOC_DOUBLE!("Lane 1 Volume", TDMOUT_GAIN0, 16, 24, 255, 0),
    SOC_DOUBLE!("Lane 2 Volume", TDMOUT_GAIN1, 0, 8, 255, 0),
    SOC_DOUBLE!("Lane 3 Volume", TDMOUT_GAIN1, 16, 24, 255, 0),
    SOC_SINGLE!(
        "Gain Enable Switch",
        TDMOUT_CTRL1,
        SM1_TDMOUT_CTRL1_GAIN_EN,
        1,
        0
    ),
];

static sm1_tdmout_sel_texts: [&'static str; 5] = ["IN 0", "IN 1", "IN 2", "IN 3", "IN 4"];

SOC_ENUM_SINGLE_DECL!(
    sm1_tdmout_sel_enum,
    TDMOUT_CTRL1,
    TDMOUT_CTRL1_SEL_SHIFT,
    sm1_tdmout_sel_texts
);

static sm1_tdmout_in_mux: snd_kcontrol_new = SOC_DAPM_ENUM!("Input Source", sm1_tdmout_sel_enum);

static sm1_tdmout_dapm_widgets: [snd_soc_dapm_widget; 8] = [
    SND_SOC_DAPM_AIF_IN!("IN 0", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("IN 1", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("IN 2", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("IN 3", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_IN!("IN 4", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_MUX!("SRC SEL", SND_SOC_NOPM, 0, 0, &sm1_tdmout_in_mux),
    SND_SOC_DAPM_PGA_E!(
        "ENC",
        SND_SOC_NOPM,
        0,
        0,
        core::ptr::null(),
        0,
        axg_tdm_formatter_event,
        SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD
    ),
    SND_SOC_DAPM_AIF_OUT!("OUT", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
];

static sm1_tdmout_dapm_routes: [snd_soc_dapm_route; 7] = [
    snd_soc_dapm_route {
        sink: "SRC SEL",
        control: "IN 0",
        source: "IN 0",
    },
    snd_soc_dapm_route {
        sink: "SRC SEL",
        control: "IN 1",
        source: "IN 1",
    },
    snd_soc_dapm_route {
        sink: "SRC SEL",
        control: "IN 2",
        source: "IN 2",
    },
    snd_soc_dapm_route {
        sink: "SRC SEL",
        control: "IN 3",
        source: "IN 3",
    },
    snd_soc_dapm_route {
        sink: "SRC SEL",
        control: "IN 4",
        source: "IN 4",
    },
    snd_soc_dapm_route {
        sink: "ENC",
        control: core::ptr::null(),
        source: "SRC SEL",
    },
    snd_soc_dapm_route {
        sink: "OUT",
        control: core::ptr::null(),
        source: "ENC",
    },
];

static sm1_tdmout_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    controls: sm1_tdmout_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&sm1_tdmout_controls),
    dapm_widgets: sm1_tdmout_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&sm1_tdmout_dapm_widgets),
    dapm_routes: sm1_tdmout_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&sm1_tdmout_dapm_routes),
};

static sm1_tdmout_quirks: axg_tdm_formatter_hw = axg_tdm_formatter_hw { skew_offset: 2 };

static sm1_tdmout_drv: axg_tdm_formatter_driver = axg_tdm_formatter_driver {
    component_drv: &sm1_tdmout_component_drv,
    regmap_cfg: &axg_tdmout_regmap_cfg,
    ops: &axg_tdmout_ops,
    quirks: &sm1_tdmout_quirks,
};

static axg_tdmout_of_match: [of_device_id; 4] = [
    of_device_id {
        compatible: "amlogic,axg-tdmout",
        data: &axg_tdmout_drv as *const axg_tdm_formatter_driver as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: "amlogic,g12a-tdmout",
        data: &g12a_tdmout_drv as *const axg_tdm_formatter_driver as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: "amlogic,sm1-tdmout",
        data: &sm1_tdmout_drv as *const axg_tdm_formatter_driver as *const core::ffi::c_void,
    },
    of_device_id {},
];
MODULE_DEVICE_TABLE!(of, axg_tdmout_of_match);

static mut axg_tdmout_pdrv: platform_driver = platform_driver {
    probe: Some(axg_tdm_formatter_probe),
    driver: device_driver {
        name: "axg-tdmout",
        of_match_table: axg_tdmout_of_match.as_ptr(),
    },
};
module_platform_driver!(axg_tdmout_pdrv);

MODULE_DESCRIPTION!("Amlogic AXG TDM output formatter driver");
MODULE_AUTHOR!("Jerome Brunet <jbrunet@baylibre.com>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
