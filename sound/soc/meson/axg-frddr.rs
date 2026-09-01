// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

/*
 * This driver implements the frontend playback DAI of AXG and G12A based SoCs
 */

// Dependencies from the original C source:
// linux/bitfield.h, linux/clk.h, linux/regmap.h, linux/module.h,
// linux/of_platform.h, sound/pcm_params.h, sound/soc.h, sound/soc-dai.h,
// and "axg-fifo.h".

const CTRL0_FRDDR_PP_MODE: u32 = BIT(30);
const CTRL0_SEL1_EN_SHIFT: u32 = 3;
const CTRL0_SEL2_SHIFT: u32 = 4;
const CTRL0_SEL2_EN_SHIFT: u32 = 7;
const CTRL0_SEL3_SHIFT: u32 = 8;
const CTRL0_SEL3_EN_SHIFT: u32 = 11;
const CTRL1_FRDDR_FORCE_FINISH: u32 = BIT(12);
const CTRL2_SEL1_SHIFT: u32 = 0;
const CTRL2_SEL1_EN_SHIFT: u32 = 4;
const CTRL2_SEL2_SHIFT: u32 = 8;
const CTRL2_SEL2_EN_SHIFT: u32 = 12;
const CTRL2_SEL3_SHIFT: u32 = 16;
const CTRL2_SEL3_EN_SHIFT: u32 = 20;

unsafe fn g12a_frddr_dai_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let fifo: *mut axg_fifo = snd_soc_dai_get_drvdata(dai) as *mut axg_fifo;

    /* Reset the read pointer to the FIFO_INIT_ADDR */
    regmap_update_bits(
        (*fifo).map,
        FIFO_CTRL1,
        CTRL1_FRDDR_FORCE_FINISH,
        0,
    );
    regmap_update_bits(
        (*fifo).map,
        FIFO_CTRL1,
        CTRL1_FRDDR_FORCE_FINISH,
        CTRL1_FRDDR_FORCE_FINISH,
    );
    regmap_update_bits(
        (*fifo).map,
        FIFO_CTRL1,
        CTRL1_FRDDR_FORCE_FINISH,
        0,
    );

    0
}

unsafe fn axg_frddr_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let fifo: *mut axg_fifo = snd_soc_dai_get_drvdata(dai) as *mut axg_fifo;
    let period: u32;
    let depth: u32;
    let val: u32;

    period = params_period_bytes(params);

    /* Trim the FIFO depth if the period is small to improve latency */
    depth = min(period, (*fifo).depth);
    val = (depth / AXG_FIFO_BURST) - 1;
    regmap_update_bits(
        (*fifo).map,
        FIFO_CTRL1,
        CTRL1_FRDDR_DEPTH,
        FIELD_PREP(CTRL1_FRDDR_DEPTH, val),
    );

    0
}

unsafe fn axg_frddr_dai_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    let fifo: *mut axg_fifo = snd_soc_dai_get_drvdata(dai) as *mut axg_fifo;
    let ret: core::ffi::c_int;

    /* Enable pclk to access registers and clock the fifo ip */
    ret = clk_prepare_enable((*fifo).pclk);
    if ret != 0 {
        return ret;
    }

    /* Apply single buffer mode to the interface */
    regmap_update_bits((*fifo).map, FIFO_CTRL0, CTRL0_FRDDR_PP_MODE, 0);

    0
}

unsafe fn axg_frddr_dai_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let fifo: *mut axg_fifo = snd_soc_dai_get_drvdata(dai) as *mut axg_fifo;

    clk_disable_unprepare((*fifo).pclk);
}

unsafe fn axg_frddr_pcm_new(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> core::ffi::c_int {
    axg_fifo_pcm_new(rtd, SNDRV_PCM_STREAM_PLAYBACK)
}

static axg_frddr_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    hw_params: Some(axg_frddr_dai_hw_params),
    startup: Some(axg_frddr_dai_startup),
    shutdown: Some(axg_frddr_dai_shutdown),
    pcm_new: Some(axg_frddr_pcm_new),
    ..unsafe { core::mem::zeroed() }
};

static mut axg_frddr_dai_drv: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c_str!("FRDDR"),
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("Playback"),
        channels_min: 1,
        channels_max: AXG_FIFO_CH_MAX,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 5515,
        rate_max: 768000,
        formats: AXG_FIFO_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &axg_frddr_ops,
    ..unsafe { core::mem::zeroed() }
};

static axg_frddr_sel_texts: [*const core::ffi::c_char; 8] = [
    c_str!("OUT 0"),
    c_str!("OUT 1"),
    c_str!("OUT 2"),
    c_str!("OUT 3"),
    c_str!("OUT 4"),
    c_str!("OUT 5"),
    c_str!("OUT 6"),
    c_str!("OUT 7"),
];

static axg_frddr_sel_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(FIFO_CTRL0, CTRL0_SEL_SHIFT, axg_frddr_sel_texts);

static axg_frddr_out_demux: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Output Sink", axg_frddr_sel_enum);

static axg_frddr_dapm_widgets: [snd_soc_dapm_widget; 9] = [
    SND_SOC_DAPM_DEMUX!("SINK SEL", SND_SOC_NOPM, 0, 0, &axg_frddr_out_demux),
    SND_SOC_DAPM_AIF_OUT!("OUT 0", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 1", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 2", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 3", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 4", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 5", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 6", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 7", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
];

static axg_frddr_dapm_routes: [snd_soc_dapm_route; 9] = [
    snd_soc_dapm_route { sink: c_str!("SINK SEL"), control: core::ptr::null(), source: c_str!("Playback") },
    snd_soc_dapm_route { sink: c_str!("OUT 0"), control: c_str!("OUT 0"), source: c_str!("SINK SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 1"), control: c_str!("OUT 1"), source: c_str!("SINK SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 2"), control: c_str!("OUT 2"), source: c_str!("SINK SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 3"), control: c_str!("OUT 3"), source: c_str!("SINK SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 4"), control: c_str!("OUT 4"), source: c_str!("SINK SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 5"), control: c_str!("OUT 5"), source: c_str!("SINK SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 6"), control: c_str!("OUT 6"), source: c_str!("SINK SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 7"), control: c_str!("OUT 7"), source: c_str!("SINK SEL") },
];

static axg_frddr_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: axg_frddr_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&axg_frddr_dapm_widgets),
    dapm_routes: axg_frddr_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&axg_frddr_dapm_routes),
    open: Some(axg_fifo_pcm_open),
    close: Some(axg_fifo_pcm_close),
    hw_params: Some(axg_fifo_pcm_hw_params),
    hw_free: Some(axg_fifo_pcm_hw_free),
    pointer: Some(axg_fifo_pcm_pointer),
    trigger: Some(axg_fifo_pcm_trigger),
    legacy_dai_naming: 1,
    ..unsafe { core::mem::zeroed() }
};

static axg_frddr_match_data: axg_fifo_match_data = axg_fifo_match_data {
    field_threshold: REG_FIELD!(FIFO_CTRL1, 16, 23),
    component_drv: &axg_frddr_component_drv,
    dai_drv: unsafe { &axg_frddr_dai_drv },
};

static g12a_frddr_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    prepare: Some(g12a_frddr_dai_prepare),
    hw_params: Some(axg_frddr_dai_hw_params),
    startup: Some(axg_frddr_dai_startup),
    shutdown: Some(axg_frddr_dai_shutdown),
    pcm_new: Some(axg_frddr_pcm_new),
    ..unsafe { core::mem::zeroed() }
};

static mut g12a_frddr_dai_drv: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c_str!("FRDDR"),
    playback: snd_soc_pcm_stream {
        stream_name: c_str!("Playback"),
        channels_min: 1,
        channels_max: AXG_FIFO_CH_MAX,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 5515,
        rate_max: 768000,
        formats: AXG_FIFO_FORMATS,
        ..unsafe { core::mem::zeroed() }
    },
    ops: &g12a_frddr_ops,
    ..unsafe { core::mem::zeroed() }
};

static g12a_frddr_sel1_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(FIFO_CTRL0, CTRL0_SEL_SHIFT, axg_frddr_sel_texts);
static g12a_frddr_sel2_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(FIFO_CTRL0, CTRL0_SEL2_SHIFT, axg_frddr_sel_texts);
static g12a_frddr_sel3_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(FIFO_CTRL0, CTRL0_SEL3_SHIFT, axg_frddr_sel_texts);

static g12a_frddr_out1_demux: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Output Src 1", g12a_frddr_sel1_enum);
static g12a_frddr_out2_demux: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Output Src 2", g12a_frddr_sel2_enum);
static g12a_frddr_out3_demux: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Output Src 3", g12a_frddr_sel3_enum);

static g12a_frddr_out1_enable: snd_kcontrol_new =
    SOC_DAPM_SINGLE_AUTODISABLE!("Switch", FIFO_CTRL0, CTRL0_SEL1_EN_SHIFT, 1, 0);
static g12a_frddr_out2_enable: snd_kcontrol_new =
    SOC_DAPM_SINGLE_AUTODISABLE!("Switch", FIFO_CTRL0, CTRL0_SEL2_EN_SHIFT, 1, 0);
static g12a_frddr_out3_enable: snd_kcontrol_new =
    SOC_DAPM_SINGLE_AUTODISABLE!("Switch", FIFO_CTRL0, CTRL0_SEL3_EN_SHIFT, 1, 0);

static g12a_frddr_dapm_widgets: [snd_soc_dapm_widget; 17] = [
    SND_SOC_DAPM_AIF_OUT!("SRC 1", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("SRC 2", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("SRC 3", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_SWITCH!("SRC 1 EN", SND_SOC_NOPM, 0, 0, &g12a_frddr_out1_enable),
    SND_SOC_DAPM_SWITCH!("SRC 2 EN", SND_SOC_NOPM, 0, 0, &g12a_frddr_out2_enable),
    SND_SOC_DAPM_SWITCH!("SRC 3 EN", SND_SOC_NOPM, 0, 0, &g12a_frddr_out3_enable),
    SND_SOC_DAPM_DEMUX!("SINK 1 SEL", SND_SOC_NOPM, 0, 0, &g12a_frddr_out1_demux),
    SND_SOC_DAPM_DEMUX!("SINK 2 SEL", SND_SOC_NOPM, 0, 0, &g12a_frddr_out2_demux),
    SND_SOC_DAPM_DEMUX!("SINK 3 SEL", SND_SOC_NOPM, 0, 0, &g12a_frddr_out3_demux),
    SND_SOC_DAPM_AIF_OUT!("OUT 0", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 1", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 2", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 3", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 4", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 5", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 6", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 7", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
];

static g12a_frddr_dapm_routes: [snd_soc_dapm_route; 33] = [
    snd_soc_dapm_route { sink: c_str!("SRC 1"), control: core::ptr::null(), source: c_str!("Playback") },
    snd_soc_dapm_route { sink: c_str!("SRC 2"), control: core::ptr::null(), source: c_str!("Playback") },
    snd_soc_dapm_route { sink: c_str!("SRC 3"), control: core::ptr::null(), source: c_str!("Playback") },
    snd_soc_dapm_route { sink: c_str!("SRC 1 EN"), control: c_str!("Switch"), source: c_str!("SRC 1") },
    snd_soc_dapm_route { sink: c_str!("SRC 2 EN"), control: c_str!("Switch"), source: c_str!("SRC 2") },
    snd_soc_dapm_route { sink: c_str!("SRC 3 EN"), control: c_str!("Switch"), source: c_str!("SRC 3") },
    snd_soc_dapm_route { sink: c_str!("SINK 1 SEL"), control: core::ptr::null(), source: c_str!("SRC 1 EN") },
    snd_soc_dapm_route { sink: c_str!("SINK 2 SEL"), control: core::ptr::null(), source: c_str!("SRC 2 EN") },
    snd_soc_dapm_route { sink: c_str!("SINK 3 SEL"), control: core::ptr::null(), source: c_str!("SRC 3 EN") },
    snd_soc_dapm_route { sink: c_str!("OUT 0"), control: c_str!("OUT 0"), source: c_str!("SINK 1 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 1"), control: c_str!("OUT 1"), source: c_str!("SINK 1 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 2"), control: c_str!("OUT 2"), source: c_str!("SINK 1 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 3"), control: c_str!("OUT 3"), source: c_str!("SINK 1 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 4"), control: c_str!("OUT 4"), source: c_str!("SINK 1 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 5"), control: c_str!("OUT 5"), source: c_str!("SINK 1 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 6"), control: c_str!("OUT 6"), source: c_str!("SINK 1 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 7"), control: c_str!("OUT 7"), source: c_str!("SINK 1 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 0"), control: c_str!("OUT 0"), source: c_str!("SINK 2 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 1"), control: c_str!("OUT 1"), source: c_str!("SINK 2 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 2"), control: c_str!("OUT 2"), source: c_str!("SINK 2 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 3"), control: c_str!("OUT 3"), source: c_str!("SINK 2 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 4"), control: c_str!("OUT 4"), source: c_str!("SINK 2 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 5"), control: c_str!("OUT 5"), source: c_str!("SINK 2 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 6"), control: c_str!("OUT 6"), source: c_str!("SINK 2 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 7"), control: c_str!("OUT 7"), source: c_str!("SINK 2 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 0"), control: c_str!("OUT 0"), source: c_str!("SINK 3 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 1"), control: c_str!("OUT 1"), source: c_str!("SINK 3 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 2"), control: c_str!("OUT 2"), source: c_str!("SINK 3 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 3"), control: c_str!("OUT 3"), source: c_str!("SINK 3 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 4"), control: c_str!("OUT 4"), source: c_str!("SINK 3 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 5"), control: c_str!("OUT 5"), source: c_str!("SINK 3 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 6"), control: c_str!("OUT 6"), source: c_str!("SINK 3 SEL") },
    snd_soc_dapm_route { sink: c_str!("OUT 7"), control: c_str!("OUT 7"), source: c_str!("SINK 3 SEL") },
];

static g12a_frddr_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: g12a_frddr_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&g12a_frddr_dapm_widgets),
    dapm_routes: g12a_frddr_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&g12a_frddr_dapm_routes),
    open: Some(axg_fifo_pcm_open),
    close: Some(axg_fifo_pcm_close),
    hw_params: Some(g12a_fifo_pcm_hw_params),
    hw_free: Some(axg_fifo_pcm_hw_free),
    pointer: Some(axg_fifo_pcm_pointer),
    trigger: Some(axg_fifo_pcm_trigger),
    legacy_dai_naming: 1,
    ..unsafe { core::mem::zeroed() }
};

static g12a_frddr_match_data: axg_fifo_match_data = axg_fifo_match_data {
    field_threshold: REG_FIELD!(FIFO_CTRL1, 16, 23),
    component_drv: &g12a_frddr_component_drv,
    dai_drv: unsafe { &g12a_frddr_dai_drv },
};

/* On SM1, the output selection in on CTRL2 */
static sm1_frddr_out1_enable: snd_kcontrol_new =
    SOC_DAPM_SINGLE_AUTODISABLE!("Switch", FIFO_CTRL2, CTRL2_SEL1_EN_SHIFT, 1, 0);
static sm1_frddr_out2_enable: snd_kcontrol_new =
    SOC_DAPM_SINGLE_AUTODISABLE!("Switch", FIFO_CTRL2, CTRL2_SEL2_EN_SHIFT, 1, 0);
static sm1_frddr_out3_enable: snd_kcontrol_new =
    SOC_DAPM_SINGLE_AUTODISABLE!("Switch", FIFO_CTRL2, CTRL2_SEL3_EN_SHIFT, 1, 0);

static sm1_frddr_sel1_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(FIFO_CTRL2, CTRL2_SEL1_SHIFT, axg_frddr_sel_texts);
static sm1_frddr_sel2_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(FIFO_CTRL2, CTRL2_SEL2_SHIFT, axg_frddr_sel_texts);
static sm1_frddr_sel3_enum: soc_enum =
    SOC_ENUM_SINGLE_DECL!(FIFO_CTRL2, CTRL2_SEL3_SHIFT, axg_frddr_sel_texts);

static sm1_frddr_out1_demux: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Output Src 1", sm1_frddr_sel1_enum);
static sm1_frddr_out2_demux: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Output Src 2", sm1_frddr_sel2_enum);
static sm1_frddr_out3_demux: snd_kcontrol_new =
    SOC_DAPM_ENUM!("Output Src 3", sm1_frddr_sel3_enum);

static sm1_frddr_dapm_widgets: [snd_soc_dapm_widget; 17] = [
    SND_SOC_DAPM_AIF_OUT!("SRC 1", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("SRC 2", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("SRC 3", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_SWITCH!("SRC 1 EN", SND_SOC_NOPM, 0, 0, &sm1_frddr_out1_enable),
    SND_SOC_DAPM_SWITCH!("SRC 2 EN", SND_SOC_NOPM, 0, 0, &sm1_frddr_out2_enable),
    SND_SOC_DAPM_SWITCH!("SRC 3 EN", SND_SOC_NOPM, 0, 0, &sm1_frddr_out3_enable),
    SND_SOC_DAPM_DEMUX!("SINK 1 SEL", SND_SOC_NOPM, 0, 0, &sm1_frddr_out1_demux),
    SND_SOC_DAPM_DEMUX!("SINK 2 SEL", SND_SOC_NOPM, 0, 0, &sm1_frddr_out2_demux),
    SND_SOC_DAPM_DEMUX!("SINK 3 SEL", SND_SOC_NOPM, 0, 0, &sm1_frddr_out3_demux),
    SND_SOC_DAPM_AIF_OUT!("OUT 0", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 1", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 2", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 3", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 4", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 5", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 6", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
    SND_SOC_DAPM_AIF_OUT!("OUT 7", core::ptr::null(), 0, SND_SOC_NOPM, 0, 0),
];

static sm1_frddr_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    dapm_widgets: sm1_frddr_dapm_widgets.as_ptr(),
    num_dapm_widgets: ARRAY_SIZE(&sm1_frddr_dapm_widgets),
    dapm_routes: g12a_frddr_dapm_routes.as_ptr(),
    num_dapm_routes: ARRAY_SIZE(&g12a_frddr_dapm_routes),
    open: Some(axg_fifo_pcm_open),
    close: Some(axg_fifo_pcm_close),
    hw_params: Some(g12a_fifo_pcm_hw_params),
    hw_free: Some(axg_fifo_pcm_hw_free),
    pointer: Some(axg_fifo_pcm_pointer),
    trigger: Some(axg_fifo_pcm_trigger),
    legacy_dai_naming: 1,
    ..unsafe { core::mem::zeroed() }
};

static sm1_frddr_match_data: axg_fifo_match_data = axg_fifo_match_data {
    field_threshold: REG_FIELD!(FIFO_CTRL1, 16, 23),
    component_drv: &sm1_frddr_component_drv,
    dai_drv: unsafe { &g12a_frddr_dai_drv },
};

static axg_frddr_of_match: [of_device_id; 4] = [
    of_device_id {
        compatible: c_str!("amlogic,axg-frddr"),
        data: &axg_frddr_match_data as *const _ as *const core::ffi::c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: c_str!("amlogic,g12a-frddr"),
        data: &g12a_frddr_match_data as *const _ as *const core::ffi::c_void,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        compatible: c_str!("amlogic,sm1-frddr"),
        data: &sm1_frddr_match_data as *const _ as *const core::ffi::c_void,
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
MODULE_DEVICE_TABLE!(of, axg_frddr_of_match);

static mut axg_frddr_pdrv: platform_driver = platform_driver {
    probe: Some(axg_fifo_probe),
    driver: device_driver {
        name: c_str!("axg-frddr"),
        of_match_table: axg_frddr_of_match.as_ptr(),
        ..unsafe { core::mem::zeroed() }
    },
    ..unsafe { core::mem::zeroed() }
};
module_platform_driver!(axg_frddr_pdrv);

MODULE_DESCRIPTION!("Amlogic AXG/G12A playback fifo driver");
MODULE_AUTHOR!("Jerome Brunet <jbrunet@baylibre.com>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
