// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek 8365 ALSA SoC Audio DAI DMIC Control
 *
 * Copyright (c) 2024 MediaTek Inc.
 * Authors: Jia Zeng <jia.zeng@mediatek.com>
 *          Alexandre Mergnat <amergnat@baylibre.com>
 */

/* Dependencies from:
 * <linux/bitops.h>
 * <linux/regmap.h>
 * <sound/pcm_params.h>
 * "mt8365-afe-clk.h"
 * "mt8365-afe-common.h"
 */

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::ptr;

#[repr(C)]
struct mt8365_dmic_data {
    two_wire_mode: bool,
    clk_phase_sel_ch1: c_uint,
    clk_phase_sel_ch2: c_uint,
    iir_on: bool,
    irr_mode: c_uint,
    dmic_mode: c_uint,
    dmic_channel: c_uint,
}

#[repr(C)]
struct list_head {
    next: *mut list_head,
    prev: *mut list_head,
}

#[repr(C)]
struct device {
    of_node: *mut device_node,
}

#[repr(C)]
struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct mtk_base_afe {
    regmap: *mut regmap,
    dev: *mut device,
    platform_priv: *mut c_void,
    sub_dais: list_head,
}

#[repr(C)]
struct mt8365_afe_private {
    dai_priv: [*mut c_void; 0],
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai {
    symmetric_rate: c_uint,
    symmetric_channels: c_uint,
}

#[repr(C)]
struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    capture: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_widget {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dapm_route {
    sink: *const c_char,
    control: *const c_char,
    source: *const c_char,
}

#[repr(C)]
struct mtk_base_afe_dai {
    list: list_head,
    dai_drivers: *mut snd_soc_dai_driver,
    num_dai_drivers: c_uint,
    controls: *const snd_kcontrol_new,
    num_controls: c_uint,
    dapm_widgets: *const snd_soc_dapm_widget,
    num_dapm_widgets: c_uint,
    dapm_routes: *const snd_soc_dapm_route,
    num_dapm_routes: c_uint,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

const fn array_size<T, const N: usize>(_: &[T; N]) -> c_uint {
    N as c_uint
}

const fn field_prep(mask: c_uint, val: c_uint) -> c_uint {
    (val << mask.trailing_zeros()) & mask
}

extern "C" {
    static AFE_DMIC3_UL_SRC_CON0: c_uint;
    static AFE_DMIC2_UL_SRC_CON0: c_uint;
    static AFE_DMIC1_UL_SRC_CON0: c_uint;
    static AFE_DMIC0_UL_SRC_CON0: c_uint;
    static AFE_ADDA_UL_DL_CON0: c_uint;
    static AFE_ADDA_UL_DL_DMIC_CLKDIV_ON: c_uint;
    static DMIC_TOP_CON_CH1_ON: c_uint;
    static DMIC_TOP_CON_CH2_ON: c_uint;
    static DMIC_TOP_CON_SRC_ON: c_uint;
    static DMIC_TOP_CON_SDM3_LEVEL_MODE: c_uint;
    static DMIC_TOP_CON_TWO_WIRE_MODE: c_uint;
    static DMIC_TOP_CON_CK_PHASE_SEL_CH1: c_uint;
    static DMIC_TOP_CON_CK_PHASE_SEL_CH2: c_uint;
    static DMIC_TOP_CON_VOICE_MODE_48K: c_uint;
    static DMIC_TOP_CON_VOICE_MODE_32K: c_uint;
    static DMIC_TOP_CON_VOICE_MODE_16K: c_uint;
    static DMIC_TOP_CON_VOICE_MODE_8K: c_uint;
    static DMIC_TOP_CON_CONFIG_MASK: c_uint;
    static DMIC_TOP_CON_IIR_ON: c_uint;
    static MT8365_AFE_IO_DMIC: usize;
    static MT8365_TOP_CG_DMIC0_ADC: c_int;
    static MT8365_TOP_CG_DMIC1_ADC: c_int;
    static MT8365_TOP_CG_DMIC2_ADC: c_int;
    static MT8365_TOP_CG_DMIC3_ADC: c_int;
    static SNDRV_PCM_RATE_16000: c_uint;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_uint;

    fn mt8365_dai_enable_adda_on(afe: *mut mtk_base_afe);
    fn mt8365_dai_disable_adda_on(afe: *mut mtk_base_afe);
    fn mt8365_afe_enable_main_clk(afe: *mut mtk_base_afe);
    fn mt8365_afe_disable_main_clk(afe: *mut mtk_base_afe);
    fn mt8365_afe_enable_top_cg(afe: *mut mtk_base_afe, cg: c_int);
    fn mt8365_afe_disable_top_cg(afe: *mut mtk_base_afe, cg: c_int);
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn usleep_range(min: c_uint, max: c_uint);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_property_read_u32_array(
        np: *mut device_node,
        propname: *const c_char,
        out_values: *mut c_uint,
        sz: usize,
    ) -> c_int;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
}

unsafe fn get_chan_reg(channel: c_uint) -> c_int {
    match channel {
        8 | 7 => AFE_DMIC3_UL_SRC_CON0 as c_int,
        6 | 5 => AFE_DMIC2_UL_SRC_CON0 as c_int,
        4 | 3 => AFE_DMIC1_UL_SRC_CON0 as c_int,
        2 | 1 => AFE_DMIC0_UL_SRC_CON0 as c_int,
        _ => -EINVAL,
    }
}

/* DAI Drivers */

unsafe fn audio_dmic_adda_enable(afe: *mut mtk_base_afe) {
    mt8365_dai_enable_adda_on(afe);
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_UL_DL_CON0,
        AFE_ADDA_UL_DL_DMIC_CLKDIV_ON,
        AFE_ADDA_UL_DL_DMIC_CLKDIV_ON,
    );
}

unsafe fn audio_dmic_adda_disable(afe: *mut mtk_base_afe) {
    regmap_update_bits(
        (*afe).regmap,
        AFE_ADDA_UL_DL_CON0,
        AFE_ADDA_UL_DL_DMIC_CLKDIV_ON,
        !AFE_ADDA_UL_DL_DMIC_CLKDIV_ON,
    );
    mt8365_dai_disable_adda_on(afe);
}

unsafe extern "C" fn mt8365_dai_enable_dmic(
    afe: *mut mtk_base_afe,
    _substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) {
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let dmic_data = (*afe_priv).dai_priv[MT8365_AFE_IO_DMIC] as *mut mt8365_dmic_data;
    let val_mask: c_uint;
    let reg = get_chan_reg((*dmic_data).dmic_channel);

    if reg < 0 {
        return;
    }

    /* val and mask will be always same to enable */
    val_mask = DMIC_TOP_CON_CH1_ON | DMIC_TOP_CON_CH2_ON | DMIC_TOP_CON_SRC_ON;

    regmap_update_bits((*afe).regmap, reg as c_uint, val_mask, val_mask);
}

unsafe extern "C" fn mt8365_dai_disable_dmic(
    afe: *mut mtk_base_afe,
    _substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) {
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let dmic_data = (*afe_priv).dai_priv[MT8365_AFE_IO_DMIC] as *mut mt8365_dmic_data;
    let mask: c_uint;
    let reg = get_chan_reg((*dmic_data).dmic_channel);

    if reg < 0 {
        return;
    }

    dev_dbg(
        (*afe).dev,
        c"%s dmic_channel %d\n".as_ptr(),
        c"mt8365_dai_disable_dmic".as_ptr(),
        (*dmic_data).dmic_channel,
    );

    mask = DMIC_TOP_CON_CH1_ON
        | DMIC_TOP_CON_CH2_ON
        | DMIC_TOP_CON_SRC_ON
        | DMIC_TOP_CON_SDM3_LEVEL_MODE;

    /* Set all masked values to 0 */
    regmap_update_bits((*afe).regmap, reg as c_uint, mask, 0);
}

unsafe fn mt8365_dai_configure_dmic(
    afe: *mut mtk_base_afe,
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let dmic_data = (*afe_priv).dai_priv[MT8365_AFE_IO_DMIC] as *mut mt8365_dmic_data;
    let two_wire_mode = (*dmic_data).two_wire_mode;
    let clk_phase_sel_ch1 = (*dmic_data).clk_phase_sel_ch1;
    let clk_phase_sel_ch2 = (*dmic_data).clk_phase_sel_ch2;
    let mut val: c_uint = 0;
    let rate = (*dai).symmetric_rate;
    let reg = get_chan_reg((*dai).symmetric_channels);

    if reg < 0 {
        return -EINVAL;
    }

    (*dmic_data).dmic_channel = (*dai).symmetric_channels;

    val |= DMIC_TOP_CON_SDM3_LEVEL_MODE;

    if two_wire_mode {
        val |= DMIC_TOP_CON_TWO_WIRE_MODE;
    } else {
        val |= field_prep(DMIC_TOP_CON_CK_PHASE_SEL_CH1, clk_phase_sel_ch1);
        val |= field_prep(DMIC_TOP_CON_CK_PHASE_SEL_CH2, clk_phase_sel_ch2);
    }

    match rate {
        48000 => {
            val |= DMIC_TOP_CON_VOICE_MODE_48K;
        }
        32000 => {
            val |= DMIC_TOP_CON_VOICE_MODE_32K;
        }
        16000 => {
            val |= DMIC_TOP_CON_VOICE_MODE_16K;
        }
        8000 => {
            val |= DMIC_TOP_CON_VOICE_MODE_8K;
        }
        _ => return -EINVAL,
    }

    regmap_update_bits((*afe).regmap, reg as c_uint, DMIC_TOP_CON_CONFIG_MASK, val);

    0
}

unsafe extern "C" fn mt8365_dai_dmic_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);

    mt8365_afe_enable_main_clk(afe);

    mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_DMIC0_ADC);
    mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_DMIC1_ADC);
    mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_DMIC2_ADC);
    mt8365_afe_enable_top_cg(afe, MT8365_TOP_CG_DMIC3_ADC);

    audio_dmic_adda_enable(afe);

    0
}

unsafe extern "C" fn mt8365_dai_dmic_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let afe = snd_soc_dai_get_drvdata(dai);

    mt8365_dai_disable_dmic(afe, substream, dai);
    audio_dmic_adda_disable(afe);
    /* HW Request delay 125us before CG off */
    usleep_range(125, 300);
    mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_DMIC3_ADC);
    mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_DMIC2_ADC);
    mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_DMIC1_ADC);
    mt8365_afe_disable_top_cg(afe, MT8365_TOP_CG_DMIC0_ADC);

    mt8365_afe_disable_main_clk(afe);
}

unsafe extern "C" fn mt8365_dai_dmic_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai);

    mt8365_dai_configure_dmic(afe, substream, dai);
    mt8365_dai_enable_dmic(afe, substream, dai);

    0
}

static mt8365_afe_dmic_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt8365_dai_dmic_startup),
    shutdown: Some(mt8365_dai_dmic_shutdown),
    prepare: Some(mt8365_dai_dmic_prepare),
};

static mut mtk_dai_dmic_driver: [snd_soc_dai_driver; 1] = [snd_soc_dai_driver {
    name: c"DMIC".as_ptr(),
    id: unsafe { MT8365_AFE_IO_DMIC as c_int },
    capture: snd_soc_pcm_stream {
        stream_name: c"DMIC Capture".as_ptr(),
        channels_min: 1,
        channels_max: 8,
        rates: unsafe { SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE },
    },
    ops: &mt8365_afe_dmic_ops,
}];

/* DAI Controls */

/* Values for 48kHz mode */
static iir_mode_src: [*const c_char; 6] = [
    c"SW custom".as_ptr(),
    c"5Hz".as_ptr(),
    c"10Hz".as_ptr(),
    c"25Hz".as_ptr(),
    c"50Hz".as_ptr(),
    c"65Hz".as_ptr(),
];

/* static SOC_ENUM_SINGLE_DECL(iir_mode, AFE_DMIC0_UL_SRC_CON0, 7, iir_mode_src); */

/* static const struct snd_kcontrol_new mtk_dai_dmic_controls[] = {
 *     SOC_SINGLE("DMIC IIR Switch", AFE_DMIC0_UL_SRC_CON0, DMIC_TOP_CON_IIR_ON, 1, 0),
 *     SOC_ENUM("DMIC IIR Mode", iir_mode),
 * };
 */
static mtk_dai_dmic_controls: [snd_kcontrol_new; 2] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

/* DAI widget */

/* static const struct snd_soc_dapm_widget mtk_dai_dmic_widgets[] = {
 *     SND_SOC_DAPM_INPUT("DMIC In"),
 * };
 */
static mtk_dai_dmic_widgets: [snd_soc_dapm_widget; 1] = [snd_soc_dapm_widget { _private: [] }];

/* DAI route */

static mtk_dai_dmic_routes: [snd_soc_dapm_route; 9] = [
    snd_soc_dapm_route {
        sink: c"I14".as_ptr(),
        control: ptr::null(),
        source: c"DMIC Capture".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"I15".as_ptr(),
        control: ptr::null(),
        source: c"DMIC Capture".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"I16".as_ptr(),
        control: ptr::null(),
        source: c"DMIC Capture".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"I17".as_ptr(),
        control: ptr::null(),
        source: c"DMIC Capture".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"I18".as_ptr(),
        control: ptr::null(),
        source: c"DMIC Capture".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"I19".as_ptr(),
        control: ptr::null(),
        source: c"DMIC Capture".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"I20".as_ptr(),
        control: ptr::null(),
        source: c"DMIC Capture".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"I21".as_ptr(),
        control: ptr::null(),
        source: c"DMIC Capture".as_ptr(),
    },
    snd_soc_dapm_route {
        sink: c"DMIC Capture".as_ptr(),
        control: ptr::null(),
        source: c"DMIC In".as_ptr(),
    },
];

unsafe fn init_dmic_priv_data(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8365_afe_private;
    let dmic_priv: *mut mt8365_dmic_data;
    let np = (*(*afe).dev).of_node;
    let mut temps: [c_uint; 4] = [0; 4];
    let ret: c_int;

    dmic_priv = devm_kzalloc(
        (*afe).dev,
        core::mem::size_of::<mt8365_dmic_data>(),
        GFP_KERNEL,
    ) as *mut mt8365_dmic_data;
    if dmic_priv.is_null() {
        return -ENOMEM;
    }

    ret = of_property_read_u32_array(
        np,
        c"mediatek,dmic-mode".as_ptr(),
        &mut temps[0],
        1,
    );
    if ret == 0 {
        (*dmic_priv).two_wire_mode = temps[0] != 0;
    }

    if !(*dmic_priv).two_wire_mode {
        (*dmic_priv).clk_phase_sel_ch1 = 0;
        (*dmic_priv).clk_phase_sel_ch2 = 4;
    }

    (*afe_priv).dai_priv[MT8365_AFE_IO_DMIC] = dmic_priv as *mut c_void;
    0
}

#[no_mangle]
pub unsafe extern "C" fn mt8365_dai_dmic_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc(
        (*afe).dev,
        core::mem::size_of::<mtk_base_afe_dai>(),
        GFP_KERNEL,
    ) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);
    (*dai).dai_drivers = unsafe { mtk_dai_dmic_driver.as_mut_ptr() };
    (*dai).num_dai_drivers = array_size(unsafe { &mtk_dai_dmic_driver });
    (*dai).controls = mtk_dai_dmic_controls.as_ptr();
    (*dai).num_controls = array_size(&mtk_dai_dmic_controls);
    (*dai).dapm_widgets = mtk_dai_dmic_widgets.as_ptr();
    (*dai).num_dapm_widgets = array_size(&mtk_dai_dmic_widgets);
    (*dai).dapm_routes = mtk_dai_dmic_routes.as_ptr();
    (*dai).num_dapm_routes = array_size(&mtk_dai_dmic_routes);
    init_dmic_priv_data(afe)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
