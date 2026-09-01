// SPDX-License-Identifier: GPL-2.0
//
// Mediatek ALSA SoC AFE platform driver for 6797
//
// Copyright (c) 2018 MediaTek Inc.
// Author: KaiChieh Chuang <kaichieh.chuang@mediatek.com>

// C dependencies translated as external Rust dependencies:
// linux/delay.h, linux/module.h, linux/mfd/syscon.h, linux/of.h,
// linux/of_address.h, linux/pm_runtime.h,
// mt6797-afe-common.h, mt6797-afe-clk.h, mt6797-interconnection.h,
// mt6797-reg.h, ../common/mtk-afe-platform-driver.h,
// ../common/mtk-afe-fe-dai.h.

use core::ffi::{c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const MTK_AFE_RATE_8K: c_uint = 0;
const MTK_AFE_RATE_11K: c_uint = 1;
const MTK_AFE_RATE_12K: c_uint = 2;
const MTK_AFE_RATE_384K: c_uint = 3;
const MTK_AFE_RATE_16K: c_uint = 4;
const MTK_AFE_RATE_22K: c_uint = 5;
const MTK_AFE_RATE_24K: c_uint = 6;
const MTK_AFE_RATE_130K: c_uint = 7;
const MTK_AFE_RATE_32K: c_uint = 8;
const MTK_AFE_RATE_44K: c_uint = 9;
const MTK_AFE_RATE_48K: c_uint = 10;
const MTK_AFE_RATE_88K: c_uint = 11;
const MTK_AFE_RATE_96K: c_uint = 12;
const MTK_AFE_RATE_174K: c_uint = 13;
const MTK_AFE_RATE_192K: c_uint = 14;
const MTK_AFE_RATE_260K: c_uint = 15;

const MTK_AFE_DAI_MEMIF_RATE_8K: c_uint = 0;
const MTK_AFE_DAI_MEMIF_RATE_16K: c_uint = 1;
const MTK_AFE_DAI_MEMIF_RATE_32K: c_uint = 2;

const MTK_AFE_PCM_RATE_8K: c_uint = 0;
const MTK_AFE_PCM_RATE_16K: c_uint = 1;
const MTK_AFE_PCM_RATE_32K: c_uint = 2;
const MTK_AFE_PCM_RATE_48K: c_uint = 3;

pub unsafe extern "C" fn mt6797_general_rate_transform(
    dev: *mut device,
    rate: c_uint,
) -> c_uint {
    match rate {
        8000 => MTK_AFE_RATE_8K,
        11025 => MTK_AFE_RATE_11K,
        12000 => MTK_AFE_RATE_12K,
        16000 => MTK_AFE_RATE_16K,
        22050 => MTK_AFE_RATE_22K,
        24000 => MTK_AFE_RATE_24K,
        32000 => MTK_AFE_RATE_32K,
        44100 => MTK_AFE_RATE_44K,
        48000 => MTK_AFE_RATE_48K,
        88200 => MTK_AFE_RATE_88K,
        96000 => MTK_AFE_RATE_96K,
        130000 => MTK_AFE_RATE_130K,
        176400 => MTK_AFE_RATE_174K,
        192000 => MTK_AFE_RATE_192K,
        260000 => MTK_AFE_RATE_260K,
        _ => {
            dev_warn!(
                dev,
                "%s(), rate %u invalid, use %d!!!\n",
                "mt6797_general_rate_transform",
                rate,
                MTK_AFE_RATE_48K
            );
            MTK_AFE_RATE_48K
        }
    }
}

unsafe extern "C" fn dai_memif_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
    match rate {
        8000 => MTK_AFE_DAI_MEMIF_RATE_8K,
        16000 => MTK_AFE_DAI_MEMIF_RATE_16K,
        32000 => MTK_AFE_DAI_MEMIF_RATE_32K,
        _ => {
            dev_warn!(
                dev,
                "%s(), rate %u invalid, use %d!!!\n",
                "dai_memif_rate_transform",
                rate,
                MTK_AFE_DAI_MEMIF_RATE_16K
            );
            MTK_AFE_DAI_MEMIF_RATE_16K
        }
    }
}

pub unsafe extern "C" fn mt6797_rate_transform(
    dev: *mut device,
    rate: c_uint,
    aud_blk: c_int,
) -> c_uint {
    match aud_blk {
        MT6797_MEMIF_DAI | MT6797_MEMIF_MOD_DAI => dai_memif_rate_transform(dev, rate),
        _ => mt6797_general_rate_transform(dev, rate),
    }
}

static mt6797_afe_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    period_bytes_min: 256,
    period_bytes_max: 4 * 48 * 1024,
    periods_min: 2,
    periods_max: 256,
    buffer_bytes_max: 8 * 48 * 1024,
    fifo_size: 0,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn mt6797_memif_fs(
    substream: *mut snd_pcm_substream,
    rate: c_uint,
) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let component: *mut snd_soc_component = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(component) as *mut mtk_base_afe;
    let id: c_int = (*snd_soc_rtd_to_cpu(rtd, 0)).id;

    mt6797_rate_transform((*afe).dev, rate, id) as c_int
}

unsafe extern "C" fn mt6797_irq_fs(substream: *mut snd_pcm_substream, rate: c_uint) -> c_int {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let component: *mut snd_soc_component = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let afe: *mut mtk_base_afe = snd_soc_component_get_drvdata(component) as *mut mtk_base_afe;

    mt6797_general_rate_transform((*afe).dev, rate) as c_int
}

const MTK_PCM_RATES: c_uint = SNDRV_PCM_RATE_8000_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

const MTK_PCM_DAI_RATES: c_uint =
    SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000;

const MTK_PCM_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

static mut mt6797_memif_dai_driver: [snd_soc_dai_driver; 8] = [
    /* FE DAIs: memory intefaces to CPU */
    snd_soc_dai_driver {
        name: c_str!("DL1"),
        id: MT6797_MEMIF_DL1,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("DL1"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &mtk_afe_fe_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("DL2"),
        id: MT6797_MEMIF_DL2,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("DL2"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &mtk_afe_fe_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("DL3"),
        id: MT6797_MEMIF_DL3,
        playback: snd_soc_pcm_stream {
            stream_name: c_str!("DL3"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &mtk_afe_fe_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("UL1"),
        id: MT6797_MEMIF_VUL12,
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("UL1"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &mtk_afe_fe_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("UL2"),
        id: MT6797_MEMIF_AWB,
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("UL2"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &mtk_afe_fe_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("UL3"),
        id: MT6797_MEMIF_VUL,
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("UL3"),
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &mtk_afe_fe_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("UL_MONO_1"),
        id: MT6797_MEMIF_MOD_DAI,
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("UL_MONO_1"),
            channels_min: 1,
            channels_max: 1,
            rates: MTK_PCM_DAI_RATES,
            formats: MTK_PCM_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &mtk_afe_fe_ops },
        ..unsafe { core::mem::zeroed() }
    },
    snd_soc_dai_driver {
        name: c_str!("UL_MONO_2"),
        id: MT6797_MEMIF_DAI,
        capture: snd_soc_pcm_stream {
            stream_name: c_str!("UL_MONO_2"),
            channels_min: 1,
            channels_max: 1,
            rates: MTK_PCM_DAI_RATES,
            formats: MTK_PCM_FORMATS,
            ..unsafe { core::mem::zeroed() }
        },
        ops: unsafe { &mtk_afe_fe_ops },
        ..unsafe { core::mem::zeroed() }
    },
];

/* dma widget & routes*/
static memif_ul1_ch1_mix: [snd_kcontrol_new; 1] = [SOC_DAPM_SINGLE_AUTODISABLE!(
    "ADDA_UL_CH1",
    AFE_CONN21,
    I_ADDA_UL_CH1,
    1,
    0
)];

static memif_ul1_ch2_mix: [snd_kcontrol_new; 1] = [SOC_DAPM_SINGLE_AUTODISABLE!(
    "ADDA_UL_CH2",
    AFE_CONN22,
    I_ADDA_UL_CH2,
    1,
    0
)];

static memif_ul2_ch1_mix: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN5, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH1", AFE_CONN5, I_DL1_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH1", AFE_CONN5, I_DL2_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH1", AFE_CONN5, I_DL3_CH1, 1, 0),
];

static memif_ul2_ch2_mix: [snd_kcontrol_new; 4] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN6, I_ADDA_UL_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL1_CH2", AFE_CONN6, I_DL1_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL2_CH2", AFE_CONN6, I_DL2_CH2, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("DL3_CH2", AFE_CONN6, I_DL3_CH2, 1, 0),
];

static memif_ul3_ch1_mix: [snd_kcontrol_new; 1] = [SOC_DAPM_SINGLE_AUTODISABLE!(
    "ADDA_UL_CH1",
    AFE_CONN9,
    I_ADDA_UL_CH1,
    1,
    0
)];

static memif_ul3_ch2_mix: [snd_kcontrol_new; 1] = [SOC_DAPM_SINGLE_AUTODISABLE!(
    "ADDA_UL_CH2",
    AFE_CONN10,
    I_ADDA_UL_CH2,
    1,
    0
)];

static memif_ul_mono_1_mix: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN12, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN12, I_ADDA_UL_CH2, 1, 0),
];

static memif_ul_mono_2_mix: [snd_kcontrol_new; 2] = [
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH1", AFE_CONN11, I_ADDA_UL_CH1, 1, 0),
    SOC_DAPM_SINGLE_AUTODISABLE!("ADDA_UL_CH2", AFE_CONN11, I_ADDA_UL_CH2, 1, 0),
];

static mt6797_memif_widgets: [snd_soc_dapm_widget; 8] = [
    /* memif */
    SND_SOC_DAPM_MIXER!(
        "UL1_CH1",
        SND_SOC_NOPM,
        0,
        0,
        memif_ul1_ch1_mix.as_ptr(),
        memif_ul1_ch1_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "UL1_CH2",
        SND_SOC_NOPM,
        0,
        0,
        memif_ul1_ch2_mix.as_ptr(),
        memif_ul1_ch2_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "UL2_CH1",
        SND_SOC_NOPM,
        0,
        0,
        memif_ul2_ch1_mix.as_ptr(),
        memif_ul2_ch1_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "UL2_CH2",
        SND_SOC_NOPM,
        0,
        0,
        memif_ul2_ch2_mix.as_ptr(),
        memif_ul2_ch2_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "UL3_CH1",
        SND_SOC_NOPM,
        0,
        0,
        memif_ul3_ch1_mix.as_ptr(),
        memif_ul3_ch1_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "UL3_CH2",
        SND_SOC_NOPM,
        0,
        0,
        memif_ul3_ch2_mix.as_ptr(),
        memif_ul3_ch2_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "UL_MONO_1_CH1",
        SND_SOC_NOPM,
        0,
        0,
        memif_ul_mono_1_mix.as_ptr(),
        memif_ul_mono_1_mix.len()
    ),
    SND_SOC_DAPM_MIXER!(
        "UL_MONO_2_CH1",
        SND_SOC_NOPM,
        0,
        0,
        memif_ul_mono_2_mix.as_ptr(),
        memif_ul_mono_2_mix.len()
    ),
];

static mt6797_memif_routes: [snd_soc_dapm_route; 18] = [
    /* capture */
    snd_soc_dapm_route { sink: c_str!("UL1"), control: ptr::null(), source: c_str!("UL1_CH1"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL1"), control: ptr::null(), source: c_str!("UL1_CH2"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL1_CH1"), control: c_str!("ADDA_UL_CH1"), source: c_str!("ADDA Capture"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL1_CH2"), control: c_str!("ADDA_UL_CH2"), source: c_str!("ADDA Capture"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL2"), control: ptr::null(), source: c_str!("UL2_CH1"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL2"), control: ptr::null(), source: c_str!("UL2_CH2"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL2_CH1"), control: c_str!("ADDA_UL_CH1"), source: c_str!("ADDA Capture"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL2_CH2"), control: c_str!("ADDA_UL_CH2"), source: c_str!("ADDA Capture"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL3"), control: ptr::null(), source: c_str!("UL3_CH1"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL3"), control: ptr::null(), source: c_str!("UL3_CH2"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL3_CH1"), control: c_str!("ADDA_UL_CH1"), source: c_str!("ADDA Capture"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL3_CH2"), control: c_str!("ADDA_UL_CH2"), source: c_str!("ADDA Capture"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL_MONO_1"), control: ptr::null(), source: c_str!("UL_MONO_1_CH1"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL_MONO_1_CH1"), control: c_str!("ADDA_UL_CH1"), source: c_str!("ADDA Capture"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL_MONO_1_CH1"), control: c_str!("ADDA_UL_CH2"), source: c_str!("ADDA Capture"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL_MONO_2"), control: ptr::null(), source: c_str!("UL_MONO_2_CH1"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL_MONO_2_CH1"), control: c_str!("ADDA_UL_CH1"), source: c_str!("ADDA Capture"), ..unsafe { core::mem::zeroed() } },
    snd_soc_dapm_route { sink: c_str!("UL_MONO_2_CH1"), control: c_str!("ADDA_UL_CH2"), source: c_str!("ADDA Capture"), ..unsafe { core::mem::zeroed() } },
];

static mt6797_afe_pcm_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c_str!("mt6797-afe-pcm-dai"),
    ..unsafe { core::mem::zeroed() }
};

static memif_data: [mtk_base_memif_data; MT6797_MEMIF_NUM as usize] = designated_array! {
    [MT6797_MEMIF_DL1] = mtk_base_memif_data {
        name: c_str!("DL1"), id: MT6797_MEMIF_DL1, reg_ofs_base: AFE_DL1_BASE,
        reg_ofs_cur: AFE_DL1_CUR, fs_reg: AFE_DAC_CON1, fs_shift: DL1_MODE_SFT,
        fs_maskbit: DL1_MODE_MASK, mono_reg: AFE_DAC_CON1, mono_shift: DL1_DATA_SFT,
        enable_reg: AFE_DAC_CON0, enable_shift: DL1_ON_SFT, hd_reg: AFE_MEMIF_HD_MODE,
        hd_shift: DL1_HD_SFT, agent_disable_reg: -1, msb_reg: -1, ..unsafe { core::mem::zeroed() }
    },
    [MT6797_MEMIF_DL2] = mtk_base_memif_data {
        name: c_str!("DL2"), id: MT6797_MEMIF_DL2, reg_ofs_base: AFE_DL2_BASE,
        reg_ofs_cur: AFE_DL2_CUR, fs_reg: AFE_DAC_CON1, fs_shift: DL2_MODE_SFT,
        fs_maskbit: DL2_MODE_MASK, mono_reg: AFE_DAC_CON1, mono_shift: DL2_DATA_SFT,
        enable_reg: AFE_DAC_CON0, enable_shift: DL2_ON_SFT, hd_reg: AFE_MEMIF_HD_MODE,
        hd_shift: DL2_HD_SFT, agent_disable_reg: -1, msb_reg: -1, ..unsafe { core::mem::zeroed() }
    },
    [MT6797_MEMIF_DL3] = mtk_base_memif_data {
        name: c_str!("DL3"), id: MT6797_MEMIF_DL3, reg_ofs_base: AFE_DL3_BASE,
        reg_ofs_cur: AFE_DL3_CUR, fs_reg: AFE_DAC_CON0, fs_shift: DL3_MODE_SFT,
        fs_maskbit: DL3_MODE_MASK, mono_reg: AFE_DAC_CON1, mono_shift: DL3_DATA_SFT,
        enable_reg: AFE_DAC_CON0, enable_shift: DL3_ON_SFT, hd_reg: AFE_MEMIF_HD_MODE,
        hd_shift: DL3_HD_SFT, agent_disable_reg: -1, msb_reg: -1, ..unsafe { core::mem::zeroed() }
    },
    [MT6797_MEMIF_VUL] = mtk_base_memif_data {
        name: c_str!("VUL"), id: MT6797_MEMIF_VUL, reg_ofs_base: AFE_VUL_BASE,
        reg_ofs_cur: AFE_VUL_CUR, fs_reg: AFE_DAC_CON1, fs_shift: VUL_MODE_SFT,
        fs_maskbit: VUL_MODE_MASK, mono_reg: AFE_DAC_CON1, mono_shift: VUL_DATA_SFT,
        enable_reg: AFE_DAC_CON0, enable_shift: VUL_ON_SFT, hd_reg: AFE_MEMIF_HD_MODE,
        hd_shift: VUL_HD_SFT, agent_disable_reg: -1, msb_reg: -1, ..unsafe { core::mem::zeroed() }
    },
    [MT6797_MEMIF_AWB] = mtk_base_memif_data {
        name: c_str!("AWB"), id: MT6797_MEMIF_AWB, reg_ofs_base: AFE_AWB_BASE,
        reg_ofs_cur: AFE_AWB_CUR, fs_reg: AFE_DAC_CON1, fs_shift: AWB_MODE_SFT,
        fs_maskbit: AWB_MODE_MASK, mono_reg: AFE_DAC_CON1, mono_shift: AWB_DATA_SFT,
        enable_reg: AFE_DAC_CON0, enable_shift: AWB_ON_SFT, hd_reg: AFE_MEMIF_HD_MODE,
        hd_shift: AWB_HD_SFT, agent_disable_reg: -1, msb_reg: -1, ..unsafe { core::mem::zeroed() }
    },
    [MT6797_MEMIF_VUL12] = mtk_base_memif_data {
        name: c_str!("VUL12"), id: MT6797_MEMIF_VUL12, reg_ofs_base: AFE_VUL_D2_BASE,
        reg_ofs_cur: AFE_VUL_D2_CUR, fs_reg: AFE_DAC_CON0, fs_shift: VUL_DATA2_MODE_SFT,
        fs_maskbit: VUL_DATA2_MODE_MASK, mono_reg: AFE_DAC_CON0, mono_shift: VUL_DATA2_DATA_SFT,
        enable_reg: AFE_DAC_CON0, enable_shift: VUL_DATA2_ON_SFT, hd_reg: AFE_MEMIF_HD_MODE,
        hd_shift: VUL_DATA2_HD_SFT, agent_disable_reg: -1, msb_reg: -1, ..unsafe { core::mem::zeroed() }
    },
    [MT6797_MEMIF_DAI] = mtk_base_memif_data {
        name: c_str!("DAI"), id: MT6797_MEMIF_DAI, reg_ofs_base: AFE_DAI_BASE,
        reg_ofs_cur: AFE_DAI_CUR, fs_reg: AFE_DAC_CON0, fs_shift: DAI_MODE_SFT,
        fs_maskbit: DAI_MODE_MASK, mono_reg: -1, mono_shift: 0, enable_reg: AFE_DAC_CON0,
        enable_shift: DAI_ON_SFT, hd_reg: AFE_MEMIF_HD_MODE, hd_shift: DAI_HD_SFT,
        agent_disable_reg: -1, msb_reg: -1, ..unsafe { core::mem::zeroed() }
    },
    [MT6797_MEMIF_MOD_DAI] = mtk_base_memif_data {
        name: c_str!("MOD_DAI"), id: MT6797_MEMIF_MOD_DAI, reg_ofs_base: AFE_MOD_DAI_BASE,
        reg_ofs_cur: AFE_MOD_DAI_CUR, fs_reg: AFE_DAC_CON1, fs_shift: MOD_DAI_MODE_SFT,
        fs_maskbit: MOD_DAI_MODE_MASK, mono_reg: -1, mono_shift: 0, enable_reg: AFE_DAC_CON0,
        enable_shift: MOD_DAI_ON_SFT, hd_reg: AFE_MEMIF_HD_MODE, hd_shift: MOD_DAI_HD_SFT,
        agent_disable_reg: -1, msb_reg: -1, ..unsafe { core::mem::zeroed() }
    },
};

static irq_data: [mtk_base_irq_data; MT6797_IRQ_NUM as usize] = designated_array! {
    [MT6797_IRQ_1] = mtk_base_irq_data {
        id: MT6797_IRQ_1, irq_cnt_reg: AFE_IRQ_MCU_CNT1, irq_cnt_shift: AFE_IRQ_MCU_CNT1_SFT,
        irq_cnt_maskbit: AFE_IRQ_MCU_CNT1_MASK, irq_fs_reg: AFE_IRQ_MCU_CON,
        irq_fs_shift: IRQ1_MCU_MODE_SFT, irq_fs_maskbit: IRQ1_MCU_MODE_MASK,
        irq_en_reg: AFE_IRQ_MCU_CON, irq_en_shift: IRQ1_MCU_ON_SFT,
        irq_clr_reg: AFE_IRQ_MCU_CLR, irq_clr_shift: IRQ1_MCU_CLR_SFT,
        ..unsafe { core::mem::zeroed() }
    },
    [MT6797_IRQ_2] = mtk_base_irq_data {
        id: MT6797_IRQ_2, irq_cnt_reg: AFE_IRQ_MCU_CNT2, irq_cnt_shift: AFE_IRQ_MCU_CNT2_SFT,
        irq_cnt_maskbit: AFE_IRQ_MCU_CNT2_MASK, irq_fs_reg: AFE_IRQ_MCU_CON,
        irq_fs_shift: IRQ2_MCU_MODE_SFT, irq_fs_maskbit: IRQ2_MCU_MODE_MASK,
        irq_en_reg: AFE_IRQ_MCU_CON, irq_en_shift: IRQ2_MCU_ON_SFT,
        irq_clr_reg: AFE_IRQ_MCU_CLR, irq_clr_shift: IRQ2_MCU_CLR_SFT,
        ..unsafe { core::mem::zeroed() }
    },
    [MT6797_IRQ_3] = mtk_base_irq_data {
        id: MT6797_IRQ_3, irq_cnt_reg: AFE_IRQ_MCU_CNT3, irq_cnt_shift: AFE_IRQ_MCU_CNT3_SFT,
        irq_cnt_maskbit: AFE_IRQ_MCU_CNT3_MASK, irq_fs_reg: AFE_IRQ_MCU_CON,
        irq_fs_shift: IRQ3_MCU_MODE_SFT, irq_fs_maskbit: IRQ3_MCU_MODE_MASK,
        irq_en_reg: AFE_IRQ_MCU_CON, irq_en_shift: IRQ3_MCU_ON_SFT,
        irq_clr_reg: AFE_IRQ_MCU_CLR, irq_clr_shift: IRQ3_MCU_CLR_SFT,
        ..unsafe { core::mem::zeroed() }
    },
    [MT6797_IRQ_4] = mtk_base_irq_data {
        id: MT6797_IRQ_4, irq_cnt_reg: AFE_IRQ_MCU_CNT4, irq_cnt_shift: AFE_IRQ_MCU_CNT4_SFT,
        irq_cnt_maskbit: AFE_IRQ_MCU_CNT4_MASK, irq_fs_reg: AFE_IRQ_MCU_CON,
        irq_fs_shift: IRQ4_MCU_MODE_SFT, irq_fs_maskbit: IRQ4_MCU_MODE_MASK,
        irq_en_reg: AFE_IRQ_MCU_CON, irq_en_shift: IRQ4_MCU_ON_SFT,
        irq_clr_reg: AFE_IRQ_MCU_CLR, irq_clr_shift: IRQ4_MCU_CLR_SFT,
        ..unsafe { core::mem::zeroed() }
    },
    [MT6797_IRQ_7] = mtk_base_irq_data {
        id: MT6797_IRQ_7, irq_cnt_reg: AFE_IRQ_MCU_CNT7, irq_cnt_shift: AFE_IRQ_MCU_CNT7_SFT,
        irq_cnt_maskbit: AFE_IRQ_MCU_CNT7_MASK, irq_fs_reg: AFE_IRQ_MCU_CON,
        irq_fs_shift: IRQ7_MCU_MODE_SFT, irq_fs_maskbit: IRQ7_MCU_MODE_MASK,
        irq_en_reg: AFE_IRQ_MCU_CON, irq_en_shift: IRQ7_MCU_ON_SFT,
        irq_clr_reg: AFE_IRQ_MCU_CLR, irq_clr_shift: IRQ7_MCU_CLR_SFT,
        ..unsafe { core::mem::zeroed() }
    },
};

static mt6797_afe_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: AFE_MAX_REGISTER,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn mt6797_afe_irq_handler(irq_id: c_int, dev: *mut c_void) -> irqreturn_t {
    let afe: *mut mtk_base_afe = dev as *mut mtk_base_afe;
    let mut irq: *mut mtk_base_afe_irq;
    let mut status: c_uint = 0;
    let mut mcu_en: c_uint = 0;
    let mut ret: c_int;
    let mut i: c_int;
    let mut irq_ret: irqreturn_t = IRQ_HANDLED;

    /* get irq that is sent to MCU */
    regmap_read((*afe).regmap, AFE_IRQ_MCU_EN, &mut mcu_en);

    ret = regmap_read((*afe).regmap, AFE_IRQ_MCU_STATUS, &mut status);
    if ret != 0 || (status & mcu_en) == 0 {
        dev_err!(
            (*afe).dev,
            "%s(), irq status err, ret %d, status 0x%x, mcu_en 0x%x\n",
            "mt6797_afe_irq_handler",
            ret,
            status,
            mcu_en
        );

        /* only clear IRQ which is sent to MCU */
        status = mcu_en & AFE_IRQ_STATUS_BITS;

        irq_ret = IRQ_NONE;
        goto_err_irq!();
    }

    i = 0;
    while i < MT6797_MEMIF_NUM {
        let memif: *mut mtk_base_afe_memif = (*afe).memif.add(i as usize);

        if (*memif).substream.is_null() {
            i += 1;
            continue;
        }

        irq = (*afe).irqs.add((*memif).irq_usage as usize);

        if (status & (1u32 << (*(*irq).irq_data).irq_en_shift)) != 0 {
            snd_pcm_period_elapsed((*memif).substream);
        }

        i += 1;
    }

    /* err_irq: clear irq */
    regmap_write((*afe).regmap, AFE_IRQ_MCU_CLR, status & AFE_IRQ_STATUS_BITS);

    irq_ret
}

unsafe extern "C" fn mt6797_afe_runtime_suspend(dev: *mut device) -> c_int {
    let afe: *mut mtk_base_afe = dev_get_drvdata(dev) as *mut mtk_base_afe;
    let mut afe_on_retm: c_uint = 0;
    let mut retry: c_int = 0;

    /* disable AFE */
    regmap_update_bits((*afe).regmap, AFE_DAC_CON0, AFE_ON_MASK_SFT, 0x0);
    loop {
        regmap_read((*afe).regmap, AFE_DAC_CON0, &mut afe_on_retm);
        if (afe_on_retm & AFE_ON_RETM_MASK_SFT) == 0 {
            break;
        }

        udelay(10);
        retry += 1;
        if retry >= 100000 {
            break;
        }
    }

    if retry != 0 {
        dev_warn!(
            (*afe).dev,
            "%s(), retry %d\n",
            "mt6797_afe_runtime_suspend",
            retry
        );
    }

    /* make sure all irq status are cleared */
    regmap_update_bits((*afe).regmap, AFE_IRQ_MCU_CLR, 0xffff, 0xffff);

    mt6797_afe_disable_clock(afe)
}

unsafe extern "C" fn mt6797_afe_runtime_resume(dev: *mut device) -> c_int {
    let afe: *mut mtk_base_afe = dev_get_drvdata(dev) as *mut mtk_base_afe;
    let mut ret: c_int;

    ret = mt6797_afe_enable_clock(afe);
    if ret != 0 {
        return ret;
    }

    /* irq signal to mcu only */
    regmap_write((*afe).regmap, AFE_IRQ_MCU_EN, AFE_IRQ_MCU_EN_MASK_SFT);

    /* force all memif use normal mode */
    regmap_update_bits((*afe).regmap, AFE_MEMIF_HDALIGN, 0x7ff << 16, 0x7ff << 16);
    /* force cpu use normal mode when access sram data */
    regmap_update_bits((*afe).regmap, AFE_MEMIF_MSB, CPU_COMPACT_MODE_MASK_SFT, 0);
    /* force cpu use 8_24 format when writing 32bit data */
    regmap_update_bits((*afe).regmap, AFE_MEMIF_MSB, CPU_HD_ALIGN_MASK_SFT, 0);

    /* set all output port to 24bit */
    regmap_update_bits((*afe).regmap, AFE_CONN_24BIT, 0x3fffffff, 0x3fffffff);

    /* enable AFE */
    regmap_update_bits(
        (*afe).regmap,
        AFE_DAC_CON0,
        AFE_ON_MASK_SFT,
        0x1 << AFE_ON_SFT,
    );

    0
}

unsafe extern "C" fn mt6797_dai_memif_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mt6797_memif_dai_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mt6797_memif_dai_driver.len() as c_int;

    (*dai).dapm_widgets = mt6797_memif_widgets.as_ptr();
    (*dai).num_dapm_widgets = mt6797_memif_widgets.len() as c_int;
    (*dai).dapm_routes = mt6797_memif_routes.as_ptr();
    (*dai).num_dapm_routes = mt6797_memif_routes.len() as c_int;
    0
}

type dai_register_cb = unsafe extern "C" fn(*mut mtk_base_afe) -> c_int;
static dai_register_cbs: [dai_register_cb; 4] = [
    mt6797_dai_adda_register,
    mt6797_dai_pcm_register,
    mt6797_dai_hostless_register,
    mt6797_dai_memif_register,
];

unsafe extern "C" fn mt6797_afe_pcm_dev_probe(pdev: *mut platform_device) -> c_int {
    let afe: *mut mtk_base_afe;
    let mut afe_priv: *mut mt6797_afe_private;
    let dev: *mut device;
    let mut i: c_int;
    let mut irq_id: c_int;
    let mut ret: c_int;

    afe = devm_kzalloc(&mut (*pdev).dev, size_of::<mtk_base_afe>(), GFP_KERNEL) as *mut mtk_base_afe;
    if afe.is_null() {
        return -ENOMEM;
    }

    (*afe).platform_priv = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<mt6797_afe_private>(),
        GFP_KERNEL,
    );
    if (*afe).platform_priv.is_null() {
        return -ENOMEM;
    }

    afe_priv = (*afe).platform_priv as *mut mt6797_afe_private;
    (*afe).dev = &mut (*pdev).dev;
    dev = (*afe).dev;

    /* initial audio related clock */
    ret = mt6797_init_clock(afe);
    if ret != 0 {
        dev_err!(dev, "init clock error\n");
        return ret;
    }

    /* regmap init */
    (*afe).base_addr = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*afe).base_addr) {
        return PTR_ERR((*afe).base_addr);
    }

    (*afe).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, (*afe).base_addr, &mt6797_afe_regmap_config);
    if IS_ERR((*afe).regmap) {
        return PTR_ERR((*afe).regmap);
    }

    /* init memif */
    (*afe).memif_size = MT6797_MEMIF_NUM;
    (*afe).memif = devm_kcalloc(
        dev,
        (*afe).memif_size as usize,
        size_of::<mtk_base_afe_memif>(),
        GFP_KERNEL,
    ) as *mut mtk_base_afe_memif;
    if (*afe).memif.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*afe).memif_size {
        (*(*afe).memif.add(i as usize)).data = &memif_data[i as usize];
        (*(*afe).memif.add(i as usize)).irq_usage = -1;
        i += 1;
    }

    mutex_init(&mut (*afe).irq_alloc_lock);

    /* irq initialize */
    (*afe).irqs_size = MT6797_IRQ_NUM;
    (*afe).irqs = devm_kcalloc(
        dev,
        (*afe).irqs_size as usize,
        size_of::<mtk_base_afe_irq>(),
        GFP_KERNEL,
    ) as *mut mtk_base_afe_irq;
    if (*afe).irqs.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*afe).irqs_size {
        (*(*afe).irqs.add(i as usize)).irq_data = &irq_data[i as usize];
        i += 1;
    }

    /* request irq */
    irq_id = platform_get_irq(pdev, 0);
    if irq_id < 0 {
        return irq_id;
    }

    ret = devm_request_irq(
        dev,
        irq_id,
        Some(mt6797_afe_irq_handler),
        IRQF_TRIGGER_NONE,
        c_str!("asys-isr"),
        afe as *mut c_void,
    );
    if ret != 0 {
        dev_err!(dev, "could not request_irq for asys-isr\n");
        return ret;
    }

    /* init sub_dais */
    INIT_LIST_HEAD(&mut (*afe).sub_dais);

    i = 0;
    while (i as usize) < dai_register_cbs.len() {
        ret = dai_register_cbs[i as usize](afe);
        if ret != 0 {
            dev_warn!(
                (*afe).dev,
                "dai register i %d fail, ret %d\n",
                i,
                ret
            );
            return ret;
        }
        i += 1;
    }

    /* init dai_driver and component_driver */
    ret = mtk_afe_combine_sub_dai(afe);
    if ret != 0 {
        dev_warn!(
            (*afe).dev,
            "mtk_afe_combine_sub_dai fail, ret %d\n",
            ret
        );
        return ret;
    }

    (*afe).mtk_afe_hardware = &mt6797_afe_hardware;
    (*afe).memif_fs = Some(mt6797_memif_fs);
    (*afe).irq_fs = Some(mt6797_irq_fs);

    (*afe).runtime_resume = Some(mt6797_afe_runtime_resume);
    (*afe).runtime_suspend = Some(mt6797_afe_runtime_suspend);

    platform_set_drvdata(pdev, afe as *mut c_void);

    pm_runtime_enable(dev);
    if !pm_runtime_enabled(dev) {
        pm_runtime_disable(dev);
        return ret;
    }
    pm_runtime_get_sync(&mut (*pdev).dev);

    /* register component */
    ret = devm_snd_soc_register_component(dev, &mtk_afe_pcm_platform, ptr::null_mut(), 0);
    if ret != 0 {
        dev_warn!(dev, "err_platform\n");
        pm_runtime_disable(dev);
        return ret;
    }

    ret = devm_snd_soc_register_component(
        (*afe).dev,
        &mt6797_afe_pcm_dai_component,
        (*afe).dai_drivers,
        (*afe).num_dai_drivers,
    );
    if ret != 0 {
        dev_warn!(dev, "err_dai_component\n");
        pm_runtime_disable(dev);
        return ret;
    }

    0
}

unsafe extern "C" fn mt6797_afe_pcm_dev_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        mt6797_afe_runtime_suspend(&mut (*pdev).dev);
    }
    pm_runtime_put_sync(&mut (*pdev).dev);
}

static mt6797_afe_pcm_dt_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c_str!("mediatek,mt6797-audio"),
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];
MODULE_DEVICE_TABLE!(of, mt6797_afe_pcm_dt_match);

static mt6797_afe_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(mt6797_afe_runtime_suspend),
    runtime_resume: Some(mt6797_afe_runtime_resume),
    runtime_idle: None,
    ..unsafe { core::mem::zeroed() }
};

static mut mt6797_afe_pcm_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c_str!("mt6797-audio"),
        of_match_table: mt6797_afe_pcm_dt_match.as_ptr(),
        pm: pm_ptr(&mt6797_afe_pm_ops),
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(mt6797_afe_pcm_dev_probe),
    remove: Some(mt6797_afe_pcm_dev_remove),
    ..unsafe { core::mem::zeroed() }
};

module_platform_driver!(mt6797_afe_pcm_driver);

MODULE_DESCRIPTION!("Mediatek ALSA SoC AFE platform driver for 6797");
MODULE_AUTHOR!("KaiChieh Chuang <kaichieh.chuang@mediatek.com>");
MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
