// SPDX-License-Identifier: GPL-2.0
/*
 *  Mediatek ALSA SoC AFE platform driver for 8189
 *
 *  Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 *
 *  Source-level Rust translation of mt8189-afe-pcm.c.
 *  C include dependencies are intentionally left as external Rust items.
 */

type c_int = i32;
type c_uint = u32;
type c_ulong = usize;
type c_void = core::ffi::c_void;
type u32_t = u32;
type u64_t = u64;
type bool_t = bool;
type irqreturn_t = c_int;
type dai_register_cb = Option<unsafe extern "C" fn(*mut mtk_base_afe) -> c_int>;

extern "C" {
    static mut mt8189_dai_adda_register: unsafe extern "C" fn(*mut mtk_base_afe) -> c_int;
    static mut mt8189_dai_i2s_register: unsafe extern "C" fn(*mut mtk_base_afe) -> c_int;
    static mut mt8189_dai_pcm_register: unsafe extern "C" fn(*mut mtk_base_afe) -> c_int;
    static mut mt8189_dai_tdm_register: unsafe extern "C" fn(*mut mtk_base_afe) -> c_int;
}

unsafe fn mt8189_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
    match rate {
        8000 => MTK_AFE_IPM2P0_RATE_8K,
        11025 => MTK_AFE_IPM2P0_RATE_11K,
        12000 => MTK_AFE_IPM2P0_RATE_12K,
        16000 => MTK_AFE_IPM2P0_RATE_16K,
        22050 => MTK_AFE_IPM2P0_RATE_22K,
        24000 => MTK_AFE_IPM2P0_RATE_24K,
        32000 => MTK_AFE_IPM2P0_RATE_32K,
        44100 => MTK_AFE_IPM2P0_RATE_44K,
        48000 => MTK_AFE_IPM2P0_RATE_48K,
        88200 => MTK_AFE_IPM2P0_RATE_88K,
        96000 => MTK_AFE_IPM2P0_RATE_96K,
        176400 => MTK_AFE_IPM2P0_RATE_176K,
        192000 => MTK_AFE_IPM2P0_RATE_192K,
        /* not support 260K */
        352800 => MTK_AFE_IPM2P0_RATE_352K,
        384000 => MTK_AFE_IPM2P0_RATE_384K,
        _ => {
            dev_warn!(dev, "rate %u invalid, use %d!!!\n", rate, MTK_AFE_IPM2P0_RATE_48K);
            MTK_AFE_IPM2P0_RATE_48K
        }
    }
}

#[inline]
fn calculate_cm_update(rate: c_uint, ch: c_uint) -> c_uint {
    (((26000000u32 / rate) - 10) / (ch / 2)) - 1
}

unsafe fn mt8189_set_cm(
    afe: *mut mtk_base_afe,
    id: c_int,
    update: bool,
    swap: bool,
    ch: c_uint,
) -> c_int {
    let afe_priv = (*afe).platform_priv as *mut mt8189_afe_private;
    let rate = (*afe_priv).cm_rate[id as usize];
    let rate_val = mt8189_rate_transform((*afe).dev, rate);
    let update_val = if update { calculate_cm_update(rate, ch) } else { 0x64 };
    let reg = AFE_CM0_CON0 + 0x10 * id;

    dev_dbg!(
        (*afe).dev,
        "%s()-0, CM%d, rate %d, update %d, swap %d, ch %d\n",
        c_str!("mt8189_set_cm"),
        id,
        rate,
        update,
        swap,
        ch
    );

    /* update cnt */
    regmap_update_bits(
        (*afe).regmap,
        reg,
        AFE_CM_UPDATE_CNT_MASK << AFE_CM_UPDATE_CNT_SFT,
        update_val << AFE_CM_UPDATE_CNT_SFT,
    );

    /* rate */
    regmap_update_bits(
        (*afe).regmap,
        reg,
        AFE_CM_1X_EN_SEL_FS_MASK << AFE_CM_1X_EN_SEL_FS_SFT,
        rate_val << AFE_CM_1X_EN_SEL_FS_SFT,
    );

    /* ch num */
    regmap_update_bits(
        (*afe).regmap,
        reg,
        AFE_CM_CH_NUM_MASK << AFE_CM_CH_NUM_SFT,
        (ch - 1) << AFE_CM_CH_NUM_SFT,
    );

    /* swap */
    regmap_update_bits(
        (*afe).regmap,
        reg,
        AFE_CM_BYTE_SWAP_MASK << AFE_CM_BYTE_SWAP_SFT,
        (swap as c_uint) << AFE_CM_BYTE_SWAP_SFT,
    );

    0
}

unsafe fn mt8189_enable_cm_bypass(afe: *mut mtk_base_afe, id: c_int, en: bool) -> c_int {
    regmap_update_bits(
        (*afe).regmap,
        AFE_CM0_CON0 + 0x10 * id,
        AFE_CM_BYPASS_MODE_MASK << AFE_CM_BYPASS_MODE_SFT,
        (en as c_uint) << AFE_CM_BYPASS_MODE_SFT,
    )
}

unsafe fn mt8189_fe_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let runtime = (*substream).runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let memif_num = (*cpu_dai).id;
    let memif = &mut *(*afe).memif.add(memif_num as usize);
    let mtk_afe_hardware = (*afe).mtk_afe_hardware;
    let mut ret: c_int;

    dev_dbg!((*afe).dev, "%s(), memif_num: %d.\n", c_str!("mt8189_fe_startup"), memif_num);
    memif.substream = substream;

    snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 16);
    snd_soc_set_runtime_hwparams(substream, mtk_afe_hardware);

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_warn!((*afe).dev, "snd_pcm_hw_constraint_integer failed\n");
    }

    /* dynamic allocate irq to memif */
    if memif.irq_usage < 0 {
        let irq_id = mtk_dynamic_irq_acquire(afe);
        if irq_id != (*afe).irqs_size {
            /* link */
            memif.irq_usage = irq_id;
        } else {
            dev_err!((*afe).dev, "%s() error: no more asys irq\n", c_str!("mt8189_fe_startup"));
            ret = -EBUSY;
        }
    }

    ret
}

unsafe fn mt8189_fe_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let memif_num = (*cpu_dai).id;
    let memif = &mut *(*afe).memif.add(memif_num as usize);
    let irq_id = memif.irq_usage;

    dev_dbg!((*afe).dev, "%s(), memif_num: %d.\n", c_str!("mt8189_fe_shutdown"), memif_num);
    memif.substream = core::ptr::null_mut();

    if !memif.const_irq {
        mtk_dynamic_irq_release(afe, irq_id);
        memif.irq_usage = -1;
        memif.substream = core::ptr::null_mut();
    }
}

unsafe fn mt8189_fe_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8189_afe_private;
    let id = (*dai).id;
    let cm = match id {
        MT8189_MEMIF_VUL8 | MT8189_MEMIF_VUL_CM0 => CM0,
        MT8189_MEMIF_VUL9 | MT8189_MEMIF_VUL_CM1 => CM1,
        _ => CM0,
    };

    (*afe_priv).cm_rate[cm as usize] = params_rate(params);
    (*afe_priv).cm_channels = params_channels(params);

    mtk_afe_fe_hw_params(substream, params, dai)
}

unsafe fn mt8189_fe_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let runtime = (*substream).runtime;
    let afe = snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let id = (*cpu_dai).id;
    let memif = &mut *(*afe).memif.add(id as usize);
    let irq_id = memif.irq_usage;
    let irqs = &mut *(*afe).irqs.add(irq_id as usize);
    let irq_data = irqs.irq_data;
    let counter = (*runtime).period_size as c_uint;
    let rate = (*runtime).rate;
    let mut tmp_reg: c_uint = 0;
    let mut fs: c_int;
    let mut ret: c_int;

    dev_dbg!(
        (*afe).dev,
        "%s(), %s cmd %d, irq_id %d, dai_id %d\n",
        c_str!("mt8189_fe_trigger"),
        (*memif.data).name,
        cmd,
        irq_id,
        id
    );

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            ret = mtk_memif_set_enable(afe, id);
            if ret != 0 {
                dev_err!((*afe).dev, "id %d, memif enable fail.\n", id);
                return ret;
            }

            /*
             * for small latency record
             * ul memif need read some data before irq enable
             * the context of this triger ops is atmoic, so it cannot sleep
             */
            if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
                if ((*runtime).period_size * 1000) / rate <= 10 {
                    udelay(300);
                }
            }

            regmap_update_bits(
                (*afe).regmap,
                (*irq_data).irq_cnt_reg,
                (*irq_data).irq_cnt_maskbit << (*irq_data).irq_cnt_shift,
                counter << (*irq_data).irq_cnt_shift,
            );

            /* set irq fs */
            fs = ((*afe).irq_fs.unwrap())(substream, rate);
            if fs < 0 {
                return -EINVAL;
            }

            if (*irq_data).irq_fs_reg >= 0 {
                regmap_update_bits(
                    (*afe).regmap,
                    (*irq_data).irq_fs_reg,
                    (*irq_data).irq_fs_maskbit << (*irq_data).irq_fs_shift,
                    fs << (*irq_data).irq_fs_shift,
                );
            }

            /* enable interrupt */
            regmap_update_bits(
                (*afe).regmap,
                (*irq_data).irq_en_reg,
                1 << (*irq_data).irq_en_shift,
                1 << (*irq_data).irq_en_shift,
            );

            0
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            ret = mtk_memif_set_disable(afe, id);
            if ret != 0 {
                dev_warn!((*afe).dev, "id %d, memif disable fail\n", id);
            }

            /* disable interrupt */
            regmap_update_bits(
                (*afe).regmap,
                (*irq_data).irq_en_reg,
                1 << (*irq_data).irq_en_shift,
                0 << (*irq_data).irq_en_shift,
            );

            /*
             * clear pending IRQ, if the register read as one, there is no
             * need to write one to clear operation.
             */
            regmap_read((*afe).regmap, (*irq_data).irq_clr_reg, &mut tmp_reg);
            regmap_update_bits(
                (*afe).regmap,
                (*irq_data).irq_clr_reg,
                AFE_IRQ_CLR_CFG_MASK_SFT | AFE_IRQ_MISS_FLAG_CLR_CFG_MASK_SFT,
                tmp_reg ^ (AFE_IRQ_CLR_CFG_MASK_SFT | AFE_IRQ_MISS_FLAG_CLR_CFG_MASK_SFT),
            );

            ret
        }
        _ => -EINVAL,
    }
}

unsafe fn mt8189_memif_fs(substream: *mut snd_pcm_substream, rate: c_uint) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let mut afe: *mut mtk_base_afe = core::ptr::null_mut();

    if component.is_null() {
        return -EINVAL;
    }

    afe = snd_soc_component_get_drvdata(component) as *mut mtk_base_afe;
    if afe.is_null() {
        return -EINVAL;
    }

    mt8189_rate_transform((*afe).dev, rate) as c_int
}

unsafe fn mt8189_get_dai_fs(afe: *mut mtk_base_afe, _dai_id: c_int, rate: c_uint) -> c_int {
    mt8189_rate_transform((*afe).dev, rate) as c_int
}

unsafe fn mt8189_irq_fs(substream: *mut snd_pcm_substream, rate: c_uint) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let mut afe: *mut mtk_base_afe = core::ptr::null_mut();

    if component.is_null() {
        return -EINVAL;
    }
    afe = snd_soc_component_get_drvdata(component) as *mut mtk_base_afe;

    mt8189_rate_transform((*afe).dev, rate) as c_int
}

unsafe fn mt8189_get_memif_pbuf_size(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;

    if ((*runtime).period_size * 1000) / (*runtime).rate > 10 {
        return MT8189_MEMIF_PBUF_SIZE_256_BYTES;
    }

    MT8189_MEMIF_PBUF_SIZE_32_BYTES
}

static mt8189_afe_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    period_bytes_min: 96,
    period_bytes_max: 4 * 48 * 1024,
    periods_min: 2,
    periods_max: 256,
    buffer_bytes_max: 256 * 1024,
    fifo_size: 0,
};

static mt8189_memif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt8189_fe_startup),
    shutdown: Some(mt8189_fe_shutdown),
    hw_params: Some(mt8189_fe_hw_params),
    hw_free: Some(mtk_afe_fe_hw_free),
    prepare: Some(mtk_afe_fe_prepare),
    trigger: Some(mt8189_fe_trigger),
};

const MTK_PCM_RATES: c_uint = SNDRV_PCM_RATE_8000_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;
const MTK_PCM_DAI_RATES: c_uint =
    SNDRV_PCM_RATE_8000 | SNDRV_PCM_RATE_16000 | SNDRV_PCM_RATE_32000 | SNDRV_PCM_RATE_48000;
const MTK_PCM_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

macro_rules! MT8189_FE_DAI_PLAYBACK {
    ($name:ident, $id:expr, $max_ch:expr) => {
        snd_soc_dai_driver {
            name: stringify!($name).as_ptr() as *const i8,
            id: $id,
            playback: snd_soc_pcm_stream {
                stream_name: stringify!($name).as_ptr() as *const i8,
                channels_min: 1,
                channels_max: $max_ch,
                rates: MTK_PCM_RATES,
                formats: MTK_PCM_FORMATS,
                ..unsafe { core::mem::zeroed() }
            },
            ops: &mt8189_memif_dai_ops,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

macro_rules! MT8189_FE_DAI_CAPTURE {
    ($name:ident, $id:expr, $max_ch:expr) => {
        snd_soc_dai_driver {
            name: stringify!($name).as_ptr() as *const i8,
            id: $id,
            capture: snd_soc_pcm_stream {
                stream_name: stringify!($name).as_ptr() as *const i8,
                channels_min: 1,
                channels_max: $max_ch,
                rates: MTK_PCM_RATES,
                formats: MTK_PCM_FORMATS,
                ..unsafe { core::mem::zeroed() }
            },
            ops: &mt8189_memif_dai_ops,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

static mut mt8189_memif_dai_driver: [snd_soc_dai_driver; 31] = [
    /* FE DAIs: memory interfaces to CPU */
    /* Playback */
    MT8189_FE_DAI_PLAYBACK!(DL0, MT8189_MEMIF_DL0, 2),
    MT8189_FE_DAI_PLAYBACK!(DL1, MT8189_MEMIF_DL1, 2),
    MT8189_FE_DAI_PLAYBACK!(DL2, MT8189_MEMIF_DL2, 2),
    MT8189_FE_DAI_PLAYBACK!(DL3, MT8189_MEMIF_DL3, 2),
    MT8189_FE_DAI_PLAYBACK!(DL4, MT8189_MEMIF_DL4, 2),
    MT8189_FE_DAI_PLAYBACK!(DL5, MT8189_MEMIF_DL5, 2),
    MT8189_FE_DAI_PLAYBACK!(DL6, MT8189_MEMIF_DL6, 2),
    MT8189_FE_DAI_PLAYBACK!(DL7, MT8189_MEMIF_DL7, 2),
    MT8189_FE_DAI_PLAYBACK!(DL8, MT8189_MEMIF_DL8, 2),
    MT8189_FE_DAI_PLAYBACK!(DL23, MT8189_MEMIF_DL23, 2),
    MT8189_FE_DAI_PLAYBACK!(DL24, MT8189_MEMIF_DL24, 2),
    MT8189_FE_DAI_PLAYBACK!(DL25, MT8189_MEMIF_DL25, 2),
    MT8189_FE_DAI_PLAYBACK!(DL_24CH, MT8189_MEMIF_DL_24CH, 8),
    MT8189_FE_DAI_PLAYBACK!(HDMI, MT8189_MEMIF_HDMI, 8),
    /* Capture */
    MT8189_FE_DAI_CAPTURE!(UL0, MT8189_MEMIF_VUL0, 2),
    MT8189_FE_DAI_CAPTURE!(UL1, MT8189_MEMIF_VUL1, 2),
    MT8189_FE_DAI_CAPTURE!(UL2, MT8189_MEMIF_VUL2, 2),
    MT8189_FE_DAI_CAPTURE!(UL3, MT8189_MEMIF_VUL3, 2),
    MT8189_FE_DAI_CAPTURE!(UL4, MT8189_MEMIF_VUL4, 2),
    MT8189_FE_DAI_CAPTURE!(UL5, MT8189_MEMIF_VUL5, 2),
    MT8189_FE_DAI_CAPTURE!(UL6, MT8189_MEMIF_VUL6, 2),
    MT8189_FE_DAI_CAPTURE!(UL7, MT8189_MEMIF_VUL7, 2),
    MT8189_FE_DAI_CAPTURE!(UL8, MT8189_MEMIF_VUL8, 2),
    MT8189_FE_DAI_CAPTURE!(UL9, MT8189_MEMIF_VUL9, 16),
    MT8189_FE_DAI_CAPTURE!(UL10, MT8189_MEMIF_VUL10, 2),
    MT8189_FE_DAI_CAPTURE!(UL24, MT8189_MEMIF_VUL24, 2),
    MT8189_FE_DAI_CAPTURE!(UL25, MT8189_MEMIF_VUL25, 2),
    MT8189_FE_DAI_CAPTURE!(UL_CM0, MT8189_MEMIF_VUL_CM0, 8),
    MT8189_FE_DAI_CAPTURE!(UL_CM1, MT8189_MEMIF_VUL_CM1, 16),
    MT8189_FE_DAI_CAPTURE!(UL_ETDM_IN0, MT8189_MEMIF_ETDM_IN0, 2),
    MT8189_FE_DAI_CAPTURE!(UL_ETDM_IN1, MT8189_MEMIF_ETDM_IN1, 2),
];

unsafe fn ul_cm0_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8189_afe_private;
    let channels = (*afe_priv).cm_channels;

    dev_dbg!(
        (*afe).dev,
        "%s(), event 0x%x, name %s, channels %d\n",
        c_str!("ul_cm0_event"),
        event,
        (*w).name,
        channels
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8189_enable_cm_bypass(afe, CM0, false);
            mt8189_set_cm(afe, CM0, true, false, channels);
            regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, PDN_CM0_MASK_SFT, 0 << PDN_CM0_SFT);
        }
        SND_SOC_DAPM_PRE_PMD => {
            mt8189_enable_cm_bypass(afe, CM0, true);
            regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, PDN_CM0_MASK_SFT, 1 << PDN_CM0_SFT);
        }
        _ => {}
    }

    0
}

unsafe fn ul_cm1_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt) as *mut mtk_base_afe;
    let afe_priv = (*afe).platform_priv as *mut mt8189_afe_private;
    let channels = (*afe_priv).cm_channels;

    dev_dbg!(
        (*afe).dev,
        "%s(), event 0x%x, name %s, channels %d\n",
        c_str!("ul_cm1_event"),
        event,
        (*w).name,
        channels
    );

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8189_enable_cm_bypass(afe, CM1, false);
            mt8189_set_cm(afe, CM1, true, false, channels);
            regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, PDN_CM1_MASK_SFT, 0 << PDN_CM1_SFT);
        }
        SND_SOC_DAPM_POST_PMD => {
            mt8189_enable_cm_bypass(afe, CM1, true);
            regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, PDN_CM1_MASK_SFT, 1 << PDN_CM1_SFT);
        }
        _ => {}
    }

    0
}

/*
 * dma widget & routes
 * The mixer controls and routes are by no means fully implemented,
 * only the ones that are intended to be used are, as other wise a fully
 * interconnected switch bar mixer would introduce way too many unused
 * controls.
 *
 * The following static DAPM control/widget/route arrays are a direct
 * translation of the source declarations using external ALSA SoC helper
 * macros as Rust macro invocations:
 *   memif_ul0_ch1_mix through memif_ul_cm1_ch16_mix
 *   cm0_mux_texts, cm1_mux_map_texts
 *   ul_cm0_mux_map_enum, ul_cm1_mux_map_enum
 *   ul_cm0_mux_control, ul_cm1_mux_control
 *   mt8189_memif_widgets
 *   mt8189_memif_routes
 */
include_dapm_declarations_from_c_translation!();

macro_rules! MT8189_DL_MEMIF {
    ($id:ident) => {
        mtk_base_memif_data {
            name: stringify!($id).as_ptr() as *const i8,
            id: concat_idents!(MT8189_MEMIF_, $id),
            reg_ofs_base: concat_idents!(AFE_, $id, _BASE),
            reg_ofs_cur: concat_idents!(AFE_, $id, _CUR),
            reg_ofs_end: concat_idents!(AFE_, $id, _END),
            reg_ofs_base_msb: concat_idents!(AFE_, $id, _BASE_MSB),
            reg_ofs_cur_msb: concat_idents!(AFE_, $id, _CUR_MSB),
            reg_ofs_end_msb: concat_idents!(AFE_, $id, _END_MSB),
            fs_reg: concat_idents!(AFE_, $id, _CON0),
            fs_shift: concat_idents!($id, _SEL_FS_SFT),
            fs_maskbit: concat_idents!($id, _SEL_FS_MASK),
            mono_reg: concat_idents!(AFE_, $id, _CON0),
            mono_shift: concat_idents!($id, _MONO_SFT),
            enable_reg: concat_idents!(AFE_, $id, _CON0),
            enable_shift: concat_idents!($id, _ON_SFT),
            hd_reg: concat_idents!(AFE_, $id, _CON0),
            hd_shift: concat_idents!($id, _HD_MODE_SFT),
            hd_align_reg: concat_idents!(AFE_, $id, _CON0),
            hd_align_mshift: concat_idents!($id, _HALIGN_SFT),
            agent_disable_reg: -1,
            agent_disable_shift: -1,
            msb_reg: -1,
            msb_shift: -1,
            pbuf_reg: concat_idents!(AFE_, $id, _CON0),
            pbuf_mask: concat_idents!($id, _PBUF_SIZE_MASK),
            pbuf_shift: concat_idents!($id, _PBUF_SIZE_SFT),
            minlen_reg: concat_idents!(AFE_, $id, _CON0),
            minlen_mask: concat_idents!($id, _MINLEN_MASK),
            minlen_shift: concat_idents!($id, _MINLEN_SFT),
            ..unsafe { core::mem::zeroed() }
        }
    };
}

macro_rules! MT8189_MULTI_DL_MEMIF {
    ($id:ident) => {
        mtk_base_memif_data {
            name: stringify!($id).as_ptr() as *const i8,
            id: concat_idents!(MT8189_MEMIF_, $id),
            reg_ofs_base: concat_idents!(AFE_, $id, _BASE),
            reg_ofs_cur: concat_idents!(AFE_, $id, _CUR),
            reg_ofs_end: concat_idents!(AFE_, $id, _END),
            reg_ofs_base_msb: concat_idents!(AFE_, $id, _BASE_MSB),
            reg_ofs_cur_msb: concat_idents!(AFE_, $id, _CUR_MSB),
            reg_ofs_end_msb: concat_idents!(AFE_, $id, _END_MSB),
            fs_reg: concat_idents!(AFE_, $id, _CON0),
            fs_shift: concat_idents!($id, _SEL_FS_SFT),
            fs_maskbit: concat_idents!($id, _SEL_FS_MASK),
            mono_reg: -1,
            mono_shift: -1,
            enable_reg: concat_idents!(AFE_, $id, _CON0),
            enable_shift: concat_idents!($id, _ON_SFT),
            hd_reg: concat_idents!(AFE_, $id, _CON0),
            hd_shift: concat_idents!($id, _HD_MODE_SFT),
            hd_align_reg: concat_idents!(AFE_, $id, _CON0),
            hd_align_mshift: concat_idents!($id, _HALIGN_SFT),
            agent_disable_reg: -1,
            agent_disable_shift: -1,
            msb_reg: -1,
            msb_shift: -1,
            pbuf_reg: concat_idents!(AFE_, $id, _CON0),
            pbuf_mask: concat_idents!($id, _PBUF_SIZE_MASK),
            pbuf_shift: concat_idents!($id, _PBUF_SIZE_SFT),
            minlen_reg: concat_idents!(AFE_, $id, _CON0),
            minlen_mask: concat_idents!($id, _MINLEN_MASK),
            minlen_shift: concat_idents!($id, _MINLEN_SFT),
            ch_num_reg: concat_idents!(AFE_, $id, _CON0),
            ch_num_maskbit: concat_idents!($id, _NUM_MASK),
            ch_num_shift: concat_idents!($id, _NUM_SFT),
            ..unsafe { core::mem::zeroed() }
        }
    };
}

macro_rules! MT8189_UL_MEMIF {
    ($id:ident, $fs_shift:expr, $fs_maskbit:expr, $mono_shift:expr) => {
        mtk_base_memif_data {
            name: stringify!($id).as_ptr() as *const i8,
            id: concat_idents!(MT8189_MEMIF_, $id),
            reg_ofs_base: concat_idents!(AFE_, $id, _BASE),
            reg_ofs_cur: concat_idents!(AFE_, $id, _CUR),
            reg_ofs_end: concat_idents!(AFE_, $id, _END),
            reg_ofs_base_msb: concat_idents!(AFE_, $id, _BASE_MSB),
            reg_ofs_cur_msb: concat_idents!(AFE_, $id, _CUR_MSB),
            reg_ofs_end_msb: concat_idents!(AFE_, $id, _END_MSB),
            fs_reg: concat_idents!(AFE_, $id, _CON0),
            fs_shift: $fs_shift,
            fs_maskbit: $fs_maskbit,
            mono_reg: concat_idents!(AFE_, $id, _CON0),
            mono_shift: $mono_shift,
            enable_reg: concat_idents!(AFE_, $id, _CON0),
            enable_shift: concat_idents!($id, _ON_SFT),
            hd_reg: concat_idents!(AFE_, $id, _CON0),
            hd_shift: concat_idents!($id, _HD_MODE_SFT),
            hd_align_reg: concat_idents!(AFE_, $id, _CON0),
            hd_align_mshift: concat_idents!($id, _HALIGN_SFT),
            agent_disable_reg: -1,
            agent_disable_shift: -1,
            msb_reg: -1,
            msb_shift: -1,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

/* For convenience with macros: missing register fields */
const HDMI_SEL_FS_SFT: c_int = -1;
const HDMI_SEL_FS_MASK: c_int = -1;

/* For convenience with macros: register name differences */
const AFE_HDMI_BASE: c_int = AFE_HDMI_OUT_BASE;
const AFE_HDMI_CUR: c_int = AFE_HDMI_OUT_CUR;
const AFE_HDMI_END: c_int = AFE_HDMI_OUT_END;
const AFE_HDMI_BASE_MSB: c_int = AFE_HDMI_OUT_BASE_MSB;
const AFE_HDMI_CUR_MSB: c_int = AFE_HDMI_OUT_CUR_MSB;
const AFE_HDMI_END_MSB: c_int = AFE_HDMI_OUT_END_MSB;
const AFE_HDMI_CON0: c_int = AFE_HDMI_OUT_CON0;
const HDMI_ON_SFT: c_int = HDMI_OUT_ON_SFT;
const HDMI_HD_MODE_SFT: c_int = HDMI_OUT_HD_MODE_SFT;
const HDMI_HALIGN_SFT: c_int = HDMI_OUT_HALIGN_SFT;
const HDMI_PBUF_SIZE_MASK: c_int = HDMI_OUT_PBUF_SIZE_MASK;
const HDMI_PBUF_SIZE_SFT: c_int = HDMI_OUT_PBUF_SIZE_SFT;
const HDMI_MINLEN_MASK: c_int = HDMI_OUT_MINLEN_MASK;
const HDMI_MINLEN_SFT: c_int = HDMI_OUT_MINLEN_SFT;
const HDMI_NUM_MASK: c_int = HDMI_CH_NUM_MASK;
const HDMI_NUM_SFT: c_int = HDMI_CH_NUM_SFT;

static memif_data: [mtk_base_memif_data; MT8189_MEMIF_NUM as usize] =
    memif_data_from_c_translation!();

macro_rules! MT8189_AFE_IRQ {
    ($id:expr) => {
        mtk_base_irq_data {
            id: concat_idents!(MT8189_IRQ_, $id),
            irq_cnt_reg: concat_idents!(AFE_IRQ, $id, _MCU_CFG1),
            irq_cnt_shift: AFE_IRQ_CNT_SHIFT,
            irq_cnt_maskbit: AFE_IRQ_CNT_MASK,
            irq_fs_reg: concat_idents!(AFE_IRQ, $id, _MCU_CFG0),
            irq_fs_shift: concat_idents!(AFE_IRQ, $id, _MCU_FS_SFT),
            irq_fs_maskbit: concat_idents!(AFE_IRQ, $id, _MCU_FS_MASK),
            irq_en_reg: concat_idents!(AFE_IRQ, $id, _MCU_CFG0),
            irq_en_shift: concat_idents!(AFE_IRQ, $id, _MCU_ON_SFT),
            irq_clr_reg: concat_idents!(AFE_IRQ, $id, _MCU_CFG1),
            irq_clr_shift: concat_idents!(AFE_IRQ, $id, _CLR_CFG_SFT),
            ..unsafe { core::mem::zeroed() }
        }
    };
}

macro_rules! MT8189_AFE_TDM_IRQ {
    ($id:expr) => {
        mtk_base_irq_data {
            id: MT8189_CUS_IRQ_TDM,
            irq_cnt_reg: AFE_CUSTOM_IRQ0_MCU_CFG1,
            irq_cnt_shift: AFE_CUSTOM_IRQ0_MCU_CNT_SFT,
            irq_cnt_maskbit: AFE_CUSTOM_IRQ0_MCU_CNT_MASK,
            irq_fs_reg: -1,
            irq_fs_shift: -1,
            irq_fs_maskbit: -1,
            irq_en_reg: AFE_CUSTOM_IRQ0_MCU_CFG0,
            irq_en_shift: AFE_CUSTOM_IRQ0_MCU_ON_SFT,
            irq_clr_reg: AFE_CUSTOM_IRQ0_MCU_CFG1,
            irq_clr_shift: AFE_CUSTOM_IRQ0_CLR_CFG_SFT,
            ..unsafe { core::mem::zeroed() }
        }
    };
}

static irq_data: [mtk_base_irq_data; MT8189_IRQ_NUM as usize] = irq_data_from_c_translation!();

static memif_irq_usage: [c_int; MT8189_MEMIF_NUM as usize] = memif_irq_usage_from_c_translation!();

unsafe fn mt8189_is_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    /* these auto-gen reg has read-only bit, so put it as volatile */
    /* volatile reg cannot be cached, so cannot be set when power off */
    match reg {
        AUDIO_TOP_CON0
        | AUDIO_TOP_CON1
        | AUDIO_TOP_CON2
        | AUDIO_TOP_CON3
        | AUDIO_TOP_CON4
        | AFE_APLL1_TUNER_MON0
        | AFE_APLL2_TUNER_MON0
        | AFE_SPM_CONTROL_ACK
        | AUDIO_TOP_IP_VERSION
        | AUDIO_ENGEN_CON0_MON
        | AFE_CONNSYS_I2S_IPM_VER_MON
        | AFE_CONNSYS_I2S_MON
        | AFE_PCM_INTF_MON
        | AFE_PCM_TOP_IP_VERSION
        | AFE_IRQ_MCU_STATUS
        | AFE_CUSTOM_IRQ_MCU_STATUS
        | AFE_IRQ_MCU_MON0
        | AFE_IRQ_MCU_MON1
        | AFE_IRQ_MCU_MON2
        | AFE_IRQ0_CNT_MON
        | AFE_IRQ1_CNT_MON
        | AFE_IRQ2_CNT_MON
        | AFE_IRQ3_CNT_MON
        | AFE_IRQ4_CNT_MON
        | AFE_IRQ5_CNT_MON
        | AFE_IRQ6_CNT_MON
        | AFE_IRQ7_CNT_MON
        | AFE_IRQ8_CNT_MON
        | AFE_IRQ9_CNT_MON
        | AFE_IRQ10_CNT_MON
        | AFE_IRQ11_CNT_MON
        | AFE_IRQ12_CNT_MON
        | AFE_IRQ13_CNT_MON
        | AFE_IRQ14_CNT_MON
        | AFE_IRQ15_CNT_MON
        | AFE_IRQ16_CNT_MON
        | AFE_IRQ17_CNT_MON
        | AFE_IRQ18_CNT_MON
        | AFE_IRQ19_CNT_MON
        | AFE_IRQ20_CNT_MON
        | AFE_IRQ21_CNT_MON
        | AFE_IRQ22_CNT_MON
        | AFE_IRQ23_CNT_MON
        | AFE_IRQ24_CNT_MON
        | AFE_IRQ25_CNT_MON
        | AFE_IRQ26_CNT_MON
        | AFE_CM0_MON
        | AFE_CM0_IP_VERSION
        | AFE_CM1_MON
        | AFE_CM1_IP_VERSION
        | AFE_ADDA_UL0_SRC_DEBUG_MON0
        | AFE_ADDA_UL0_SRC_MON0
        | AFE_ADDA_UL0_SRC_MON1
        | AFE_ADDA_UL0_IP_VERSION
        | AFE_ADDA_DMIC0_SRC_DEBUG_MON0
        | AFE_ADDA_DMIC0_SRC_MON0
        | AFE_ADDA_DMIC0_SRC_MON1
        | AFE_ADDA_DMIC0_IP_VERSION
        | AFE_ADDA_DMIC1_SRC_DEBUG_MON0
        | AFE_ADDA_DMIC1_SRC_MON0
        | AFE_ADDA_DMIC1_SRC_MON1
        | AFE_ADDA_DMIC1_IP_VERSION
        | AFE_MTKAIF_IPM_VER_MON
        | AFE_MTKAIF_MON
        | AFE_AUD_PAD_TOP_MON
        | AFE_ADDA_MTKAIFV4_MON0
        | AFE_ADDA_MTKAIFV4_MON1
        | AFE_ADDA6_MTKAIFV4_MON0
        | ETDM_IN0_MON
        | ETDM_IN1_MON
        | ETDM_OUT0_MON
        | ETDM_OUT1_MON
        | ETDM_OUT4_MON
        | AFE_CONN_MON0
        | AFE_CONN_MON1
        | AFE_CONN_MON2
        | AFE_CONN_MON3
        | AFE_CONN_MON4
        | AFE_CONN_MON5
        | AFE_CBIP_SLV_DECODER_MON0
        | AFE_CBIP_SLV_DECODER_MON1
        | AFE_CBIP_SLV_MUX_MON0
        | AFE_CBIP_SLV_MUX_MON1
        | AFE_DL0_CUR_MSB
        | AFE_DL0_CUR
        | AFE_DL0_RCH_MON
        | AFE_DL0_LCH_MON
        | AFE_DL1_CUR_MSB
        | AFE_DL1_CUR
        | AFE_DL1_RCH_MON
        | AFE_DL1_LCH_MON
        | AFE_DL2_CUR_MSB
        | AFE_DL2_CUR
        | AFE_DL2_RCH_MON
        | AFE_DL2_LCH_MON
        | AFE_DL3_CUR_MSB
        | AFE_DL3_CUR
        | AFE_DL3_RCH_MON
        | AFE_DL3_LCH_MON
        | AFE_DL4_CUR_MSB
        | AFE_DL4_CUR
        | AFE_DL4_RCH_MON
        | AFE_DL4_LCH_MON
        | AFE_DL5_CUR_MSB
        | AFE_DL5_CUR
        | AFE_DL5_RCH_MON
        | AFE_DL5_LCH_MON
        | AFE_DL6_CUR_MSB
        | AFE_DL6_CUR
        | AFE_DL6_RCH_MON
        | AFE_DL6_LCH_MON
        | AFE_DL7_CUR_MSB
        | AFE_DL7_CUR
        | AFE_DL7_RCH_MON
        | AFE_DL7_LCH_MON
        | AFE_DL8_CUR_MSB
        | AFE_DL8_CUR
        | AFE_DL8_RCH_MON
        | AFE_DL8_LCH_MON
        | AFE_DL_24CH_CUR_MSB
        | AFE_DL_24CH_CUR
        | AFE_DL23_CUR_MSB
        | AFE_DL23_CUR
        | AFE_DL23_RCH_MON
        | AFE_DL23_LCH_MON
        | AFE_DL24_CUR_MSB
        | AFE_DL24_CUR
        | AFE_DL24_RCH_MON
        | AFE_DL24_LCH_MON
        | AFE_DL25_CUR_MSB
        | AFE_DL25_CUR
        | AFE_DL25_RCH_MON
        | AFE_DL25_LCH_MON
        | AFE_VUL0_CUR_MSB
        | AFE_VUL0_CUR
        | AFE_VUL1_CUR_MSB
        | AFE_VUL1_CUR
        | AFE_VUL2_CUR_MSB
        | AFE_VUL2_CUR
        | AFE_VUL3_CUR_MSB
        | AFE_VUL3_CUR
        | AFE_VUL4_CUR_MSB
        | AFE_VUL4_CUR
        | AFE_VUL5_CUR_MSB
        | AFE_VUL5_CUR
        | AFE_VUL6_CUR_MSB
        | AFE_VUL6_CUR
        | AFE_VUL7_CUR_MSB
        | AFE_VUL7_CUR
        | AFE_VUL8_CUR_MSB
        | AFE_VUL8_CUR
        | AFE_VUL9_CUR_MSB
        | AFE_VUL9_CUR
        | AFE_VUL10_CUR_MSB
        | AFE_VUL10_CUR
        | AFE_VUL24_CUR_MSB
        | AFE_VUL24_CUR
        | AFE_VUL25_CUR_MSB
        | AFE_VUL25_CUR
        | AFE_VUL_CM0_CUR_MSB
        | AFE_VUL_CM0_CUR
        | AFE_VUL_CM1_CUR_MSB
        | AFE_VUL_CM1_CUR
        | AFE_ETDM_IN0_CUR_MSB
        | AFE_ETDM_IN0_CUR
        | AFE_ETDM_IN1_CUR_MSB
        | AFE_ETDM_IN1_CUR
        | AFE_HDMI_OUT_CUR_MSB
        | AFE_HDMI_OUT_CUR
        | AFE_HDMI_OUT_END
        | AFE_HDMI_OUT_MON0
        | AFE_PROT_SIDEBAND0_MON
        | AFE_PROT_SIDEBAND1_MON
        | AFE_PROT_SIDEBAND2_MON
        | AFE_PROT_SIDEBAND3_MON
        | AFE_DOMAIN_SIDEBAND0_MON
        | AFE_DOMAIN_SIDEBAND1_MON
        | AFE_DOMAIN_SIDEBAND2_MON
        | AFE_DOMAIN_SIDEBAND3_MON
        | AFE_DOMAIN_SIDEBAND4_MON
        | AFE_DOMAIN_SIDEBAND5_MON
        | AFE_DOMAIN_SIDEBAND6_MON
        | AFE_DOMAIN_SIDEBAND7_MON
        | AFE_DOMAIN_SIDEBAND8_MON
        | AFE_DOMAIN_SIDEBAND9_MON
        | AFE_PCM0_INTF_CON1_MASK_MON
        | AFE_CONNSYS_I2S_CON_MASK_MON
        | AFE_MTKAIF0_CFG0_MASK_MON
        | AFE_MTKAIF1_CFG0_MASK_MON
        | AFE_ADDA_UL0_SRC_CON0_MASK_MON
        | AFE_ASRC_NEW_CON0
        | AFE_ASRC_NEW_CON6
        | AFE_ASRC_NEW_CON8
        | AFE_ASRC_NEW_CON9
        | AFE_ASRC_NEW_CON12
        | AFE_ASRC_NEW_IP_VERSION
        | AFE_GASRC0_NEW_CON0
        | AFE_GASRC0_NEW_CON6
        | AFE_GASRC0_NEW_CON8
        | AFE_GASRC0_NEW_CON9
        | AFE_GASRC0_NEW_CON10
        | AFE_GASRC0_NEW_CON11
        | AFE_GASRC0_NEW_CON12
        | AFE_GASRC0_NEW_IP_VERSION
        | AFE_GASRC1_NEW_CON0
        | AFE_GASRC1_NEW_CON6
        | AFE_GASRC1_NEW_CON8
        | AFE_GASRC1_NEW_CON9
        | AFE_GASRC1_NEW_CON12
        | AFE_GASRC1_NEW_IP_VERSION
        | AFE_GASRC2_NEW_CON0
        | AFE_GASRC2_NEW_CON6
        | AFE_GASRC2_NEW_CON8
        | AFE_GASRC2_NEW_CON9
        | AFE_GASRC2_NEW_CON12
        | AFE_GASRC2_NEW_IP_VERSION
        | AFE_GAIN0_CUR_L
        | AFE_GAIN0_CUR_R
        | AFE_GAIN1_CUR_L
        | AFE_GAIN1_CUR_R
        | AFE_GAIN2_CUR_L
        | AFE_GAIN2_CUR_R
        | AFE_GAIN3_CUR_L
        | AFE_GAIN3_CUR_R
        | AFE_IRQ_MCU_EN
        | AFE_CUSTOM_IRQ_MCU_EN
        | AFE_IRQ_MCU_DSP_EN
        | AFE_IRQ_MCU_DSP2_EN
        | AFE_DL5_CON0
        | AFE_DL6_CON0
        | AFE_DL23_CON0
        | AFE_DL_24CH_CON0
        | AFE_VUL1_CON0
        | AFE_VUL3_CON0
        | AFE_VUL4_CON0
        | AFE_VUL5_CON0
        | AFE_VUL9_CON0
        | AFE_VUL25_CON0
        | AFE_IRQ0_MCU_CFG0
        | AFE_IRQ1_MCU_CFG0
        | AFE_IRQ2_MCU_CFG0
        | AFE_IRQ3_MCU_CFG0
        | AFE_IRQ4_MCU_CFG0
        | AFE_IRQ5_MCU_CFG0
        | AFE_IRQ6_MCU_CFG0
        | AFE_IRQ7_MCU_CFG0
        | AFE_IRQ8_MCU_CFG0
        | AFE_IRQ9_MCU_CFG0
        | AFE_IRQ10_MCU_CFG0
        | AFE_IRQ11_MCU_CFG0
        | AFE_IRQ12_MCU_CFG0
        | AFE_IRQ13_MCU_CFG0
        | AFE_IRQ14_MCU_CFG0
        | AFE_IRQ15_MCU_CFG0
        | AFE_IRQ16_MCU_CFG0
        | AFE_IRQ17_MCU_CFG0
        | AFE_IRQ18_MCU_CFG0
        | AFE_IRQ19_MCU_CFG0
        | AFE_IRQ20_MCU_CFG0
        | AFE_IRQ21_MCU_CFG0
        | AFE_IRQ22_MCU_CFG0
        | AFE_IRQ23_MCU_CFG0
        | AFE_IRQ24_MCU_CFG0
        | AFE_IRQ25_MCU_CFG0
        | AFE_IRQ26_MCU_CFG0
        | AFE_CUSTOM_IRQ0_MCU_CFG0
        | AFE_IRQ0_MCU_CFG1
        | AFE_IRQ1_MCU_CFG1
        | AFE_IRQ2_MCU_CFG1
        | AFE_IRQ3_MCU_CFG1
        | AFE_IRQ4_MCU_CFG1
        | AFE_IRQ5_MCU_CFG1
        | AFE_IRQ6_MCU_CFG1
        | AFE_IRQ7_MCU_CFG1
        | AFE_IRQ8_MCU_CFG1
        | AFE_IRQ9_MCU_CFG1
        | AFE_IRQ10_MCU_CFG1
        | AFE_IRQ11_MCU_CFG1
        | AFE_IRQ12_MCU_CFG1
        | AFE_IRQ13_MCU_CFG1
        | AFE_IRQ14_MCU_CFG1
        | AFE_IRQ15_MCU_CFG1
        | AFE_IRQ16_MCU_CFG1
        | AFE_IRQ17_MCU_CFG1
        | AFE_IRQ18_MCU_CFG1
        | AFE_IRQ19_MCU_CFG1
        | AFE_IRQ20_MCU_CFG1
        | AFE_IRQ21_MCU_CFG1
        | AFE_IRQ22_MCU_CFG1
        | AFE_IRQ23_MCU_CFG1
        | AFE_IRQ24_MCU_CFG1
        | AFE_IRQ25_MCU_CFG1
        | AFE_IRQ26_MCU_CFG1
        | AFE_CUSTOM_IRQ0_MCU_CFG1
        /* for vow using */
        | AFE_IRQ_MCU_SCP_EN
        | AFE_VUL_CM0_BASE_MSB
        | AFE_VUL_CM0_BASE
        | AFE_VUL_CM0_END_MSB
        | AFE_VUL_CM0_END
        | AFE_VUL_CM0_CON0 => true,
        _ => false,
    }
}

static mt8189_afe_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    volatile_reg: Some(mt8189_is_volatile_reg),
    max_register: AFE_MAX_REGISTER,
    num_reg_defaults_raw: AFE_MAX_REGISTER,
    cache_type: REGCACHE_FLAT,
    ..unsafe { core::mem::zeroed() }
};

unsafe extern "C" fn mt8189_afe_irq_handler(_irq_id: c_int, dev: *mut c_void) -> irqreturn_t {
    let afe = dev as *mut mtk_base_afe;
    let mut irq: *mut mtk_base_afe_irq;
    let mut status: u32_t = 0;
    let mut status_mcu: u32_t;
    let mut mcu_en: u32_t = 0;
    let mut cus_status: u32_t = 0;
    let mut cus_status_mcu: u32_t;
    let mut cus_mcu_en: u32_t = 0;
    let mut tmp_reg: u32_t = 0;
    let ret: c_int;
    let cus_ret: c_int;
    let mut i: c_int;
    let mut ts64: timespec64 = core::mem::zeroed();
    let mut t1: u64_t;
    let mut t2: u64_t;
    /* one interrupt period = 5ms */
    const timeout_limit: u64_t = 5000000;

    /* get irq that is sent to MCU */
    regmap_read((*afe).regmap, AFE_IRQ_MCU_EN, &mut mcu_en);
    regmap_read((*afe).regmap, AFE_CUSTOM_IRQ_MCU_EN, &mut cus_mcu_en);

    ret = regmap_read((*afe).regmap, AFE_IRQ_MCU_STATUS, &mut status);
    cus_ret = regmap_read((*afe).regmap, AFE_CUSTOM_IRQ_MCU_STATUS, &mut cus_status);
    /* only care IRQ which is sent to MCU */
    status_mcu = status & mcu_en & AFE_IRQ_STATUS_BITS;
    cus_status_mcu = cus_status & cus_mcu_en & AFE_IRQ_STATUS_BITS;
    if (ret != 0 || status_mcu == 0) && (cus_ret != 0 || cus_status_mcu == 0) {
        dev_err!(
            (*afe).dev,
            "%s(), irq status err, ret %d, 0x%x:0x%x:0x%x:0x%x\n",
            c_str!("mt8189_afe_irq_handler"),
            ret,
            status,
            mcu_en,
            cus_status_mcu,
            cus_mcu_en
        );
        return IRQ_NONE;
    }

    ktime_get_ts64(&mut ts64);
    t1 = ktime_get_ns();

    i = 0;
    while i < MT8189_MEMIF_NUM {
        let memif = &mut *(*afe).memif.add(i as usize);

        if memif.substream.is_null() {
            i += 1;
            continue;
        }

        if memif.irq_usage < 0 {
            i += 1;
            continue;
        }
        irq = (*afe).irqs.add(memif.irq_usage as usize);

        if i == MT8189_MEMIF_HDMI {
            if (cus_status_mcu & BIT((*(*irq).irq_data).id)) != 0 {
                snd_pcm_period_elapsed(memif.substream);
            }
        } else if (status_mcu & BIT((*(*irq).irq_data).id)) != 0 {
            snd_pcm_period_elapsed(memif.substream);
        }
        i += 1;
    }

    ktime_get_ts64(&mut ts64);
    t2 = ktime_get_ns();
    t2 = t2 - t1; /* in ns (10^9) */

    if t2 > timeout_limit {
        dev_warn!((*afe).dev, "IRQ handler exceeded time limit by %llu ns\n", t2 - timeout_limit);
    }

    /* clear irq */
    i = 0;
    while i < MT8189_IRQ_NUM {
        if (((cus_status_mcu & BIT(irq_data[i as usize].id)) != 0) && i == MT8189_IRQ_31)
            || (((status_mcu & BIT(irq_data[i as usize].id)) != 0) && i != MT8189_IRQ_31)
        {
            regmap_read((*afe).regmap, irq_data[i as usize].irq_clr_reg, &mut tmp_reg);
            regmap_update_bits(
                (*afe).regmap,
                irq_data[i as usize].irq_clr_reg,
                AFE_IRQ_CLR_CFG_MASK_SFT | AFE_IRQ_MISS_FLAG_CLR_CFG_MASK_SFT,
                tmp_reg ^ (AFE_IRQ_CLR_CFG_MASK_SFT | AFE_IRQ_MISS_FLAG_CLR_CFG_MASK_SFT),
            );
        }
        i += 1;
    }

    IRQ_HANDLED
}

unsafe extern "C" fn mt8189_afe_runtime_suspend(dev: *mut device) -> c_int {
    let afe = dev_get_drvdata(dev) as *mut mtk_base_afe;
    let mut value: c_uint = 0;
    let mut tmp_reg: c_uint = 0;
    let mut ret: c_int;
    let mut i: c_int;

    if (*afe).regmap.is_null() {
        dev_warn!((*afe).dev, "%s() skip regmap\n", c_str!("mt8189_afe_runtime_suspend"));
        mt8189_afe_disable_reg_rw_clk(afe);
        return 0;
    }

    /* disable AFE */
    mt8189_afe_disable_main_clock(afe);

    ret = regmap_read_poll_timeout!(
        (*afe).regmap,
        AUDIO_ENGEN_CON0_MON,
        value,
        (value & AUDIO_ENGEN_MON_SFT) == 0,
        20,
        1 * 1000 * 1000
    );
    dev_dbg!((*afe).dev, "%s() read_poll ret %d\n", c_str!("mt8189_afe_runtime_suspend"), ret);
    if ret != 0 {
        dev_warn!((*afe).dev, "%s(), ret %d\n", c_str!("mt8189_afe_runtime_suspend"), ret);
    }

    /* make sure all irq status are cleared */
    i = 0;
    while i < MT8189_IRQ_NUM {
        regmap_read((*afe).regmap, irq_data[i as usize].irq_clr_reg, &mut tmp_reg);
        regmap_update_bits(
            (*afe).regmap,
            irq_data[i as usize].irq_clr_reg,
            AFE_IRQ_CLR_CFG_MASK_SFT | AFE_IRQ_MISS_FLAG_CLR_CFG_MASK_SFT,
            tmp_reg ^ (AFE_IRQ_CLR_CFG_MASK_SFT | AFE_IRQ_MISS_FLAG_CLR_CFG_MASK_SFT),
        );
        i += 1;
    }

    /* reset audio 26M request */
    regmap_update_bits((*afe).regmap, AFE_SPM_CONTROL_REQ, 0x1, 0x0);

    /* cache only */
    regcache_cache_only((*afe).regmap, true);
    regcache_mark_dirty((*afe).regmap);

    mt8189_afe_disable_reg_rw_clk(afe);
    0
}

unsafe extern "C" fn mt8189_afe_runtime_resume(dev: *mut device) -> c_int {
    let afe = dev_get_drvdata(dev) as *mut mtk_base_afe;
    let ret = mt8189_afe_enable_reg_rw_clk(afe);
    if ret != 0 {
        return ret;
    }

    if (*afe).regmap.is_null() {
        dev_warn!((*afe).dev, "skip regmap\n");
        return 0;
    }

    regcache_cache_only((*afe).regmap, false);
    regcache_sync((*afe).regmap);

    /* set audio 26M request */
    regmap_update_bits((*afe).regmap, AFE_SPM_CONTROL_REQ, 0x1, 0x1);
    regmap_update_bits((*afe).regmap, AFE_CBIP_CFG0, 0x1, 0x1);

    /* force cpu use 8_24 format when writing 32bit data */
    regmap_update_bits((*afe).regmap, AFE_MEMIF_CON0, CPU_HD_ALIGN_MASK_SFT, 0 << CPU_HD_ALIGN_SFT);

    /* enable AFE */
    mt8189_afe_enable_main_clock(afe);

    0
}

unsafe extern "C" fn mt8189_afe_component_probe(component: *mut snd_soc_component) -> c_int {
    let afe = snd_soc_component_get_drvdata(component) as *mut mtk_base_afe;
    let ret = pm_runtime_resume_and_get((*afe).dev);
    if ret != 0 {
        return dev_err_probe((*afe).dev, ret, "failed to resume device\n");
    }

    mtk_afe_add_sub_dai_control(component);
    pm_runtime_put_sync((*afe).dev);

    0
}

unsafe extern "C" fn mt8189_afe_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    /* set the wait_for_avail to 2 sec*/
    (*substream).wait_time = msecs_to_jiffies(2 * 1000);
    0
}

unsafe extern "C" fn mt8189_afe_pcm_free(_component: *mut snd_soc_component, pcm: *mut snd_pcm) {
    snd_pcm_lib_preallocate_free_for_all(pcm);
}

static mt8189_afe_component: snd_soc_component_driver = snd_soc_component_driver {
    name: AFE_PCM_NAME,
    probe: Some(mt8189_afe_component_probe),
    pcm_new: Some(mtk_afe_pcm_new),
    pcm_free: Some(mt8189_afe_pcm_free),
    open: Some(mt8189_afe_pcm_open),
    pointer: Some(mtk_afe_pcm_pointer),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn mt8189_dai_memif_register(afe: *mut mtk_base_afe) -> c_int {
    let dai = devm_kzalloc((*afe).dev, core::mem::size_of::<mtk_base_afe_dai>(), GFP_KERNEL)
        as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mt8189_memif_dai_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mt8189_memif_dai_driver.len() as c_int;
    (*dai).dapm_widgets = mt8189_memif_widgets.as_ptr();
    (*dai).num_dapm_widgets = mt8189_memif_widgets.len() as c_int;
    (*dai).dapm_routes = mt8189_memif_routes.as_ptr();
    (*dai).num_dapm_routes = mt8189_memif_routes.len() as c_int;

    0
}

static dai_register_cbs: [dai_register_cb; 5] = [
    Some(mt8189_dai_adda_register),
    Some(mt8189_dai_i2s_register),
    Some(mt8189_dai_pcm_register),
    Some(mt8189_dai_tdm_register),
    Some(mt8189_dai_memif_register),
];

static mt8189_cg_patch: [reg_sequence; 1] = [reg_sequence {
    reg: AUDIO_TOP_CON4,
    def: 0x361c,
    delay_us: 0,
}];

unsafe extern "C" fn mt8189_afe_release_reserved_mem(data: *mut c_void) {
    of_reserved_mem_device_release(data);
}

unsafe extern "C" fn mt8189_afe_pcm_dev_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let mut i: c_int;
    let mut tmp_reg: c_uint = 0;
    let irq_id: c_int;
    let afe: *mut mtk_base_afe;
    let mut afe_priv: *mut mt8189_afe_private;
    let dev = &mut (*pdev).dev as *mut device;

    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(34));
    if ret != 0 {
        return ret;
    }

    ret = of_reserved_mem_device_init(dev);
    if ret != 0 {
        dev_warn!(dev, "failed to assign memory region: %d\n", ret);
    } else {
        ret = devm_add_action_or_reset(dev, Some(mt8189_afe_release_reserved_mem), dev as *mut c_void);
        if ret != 0 {
            return ret;
        }
    }

    afe = devm_kzalloc(dev, core::mem::size_of::<mtk_base_afe>(), GFP_KERNEL) as *mut mtk_base_afe;
    if afe.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, afe as *mut c_void);

    (*afe).platform_priv =
        devm_kzalloc(dev, core::mem::size_of::<mt8189_afe_private>(), GFP_KERNEL) as *mut c_void;
    if (*afe).platform_priv.is_null() {
        return -ENOMEM;
    }

    afe_priv = (*afe).platform_priv as *mut mt8189_afe_private;
    (*afe).dev = dev;

    (*afe).base_addr = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*afe).base_addr) {
        return dev_err_probe(dev, PTR_ERR((*afe).base_addr), "AFE base_addr not found\n");
    }

    /* init audio related clock */
    ret = mt8189_init_clock(afe);
    if ret != 0 {
        return dev_err_probe(dev, ret, "init clock error.\n");
    }

    /* init memif */
    /* IPM2.0 no need banding */
    (*afe).memif_32bit_supported = 1;
    (*afe).memif_size = MT8189_MEMIF_NUM;
    (*afe).memif = devm_kcalloc(
        dev,
        (*afe).memif_size as usize,
        core::mem::size_of::<mtk_base_afe_memif>(),
        GFP_KERNEL,
    ) as *mut mtk_base_afe_memif;

    if (*afe).memif.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < (*afe).memif_size {
        (*(*afe).memif.add(i as usize)).data = &memif_data[i as usize];
        (*(*afe).memif.add(i as usize)).irq_usage = memif_irq_usage[i as usize];
        (*(*afe).memif.add(i as usize)).const_irq = 1;
        i += 1;
    }

    mutex_init(&mut (*afe).irq_alloc_lock);

    /* init irq */
    (*afe).irqs_size = MT8189_IRQ_NUM;
    (*afe).irqs = devm_kcalloc(
        dev,
        (*afe).irqs_size as usize,
        core::mem::size_of::<mtk_base_afe_irq>(),
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
        return dev_err_probe(dev, irq_id, "no irq found");
    }

    ret = devm_request_irq(
        dev,
        irq_id,
        Some(mt8189_afe_irq_handler),
        IRQF_TRIGGER_NONE,
        "Afe_ISR_Handle",
        afe as *mut c_void,
    );
    if ret != 0 {
        return dev_err_probe(dev, ret, "could not request_irq for Afe_ISR_Handle\n");
    }

    /* init sub_dais */
    INIT_LIST_HEAD(&mut (*afe).sub_dais);

    i = 0;
    while (i as usize) < dai_register_cbs.len() {
        ret = (dai_register_cbs[i as usize].unwrap())(afe);
        if ret != 0 {
            return dev_err_probe(dev, ret, "dai register i %d fail\n", i);
        }
        i += 1;
    }

    /* init dai_driver and component_driver */
    ret = mtk_afe_combine_sub_dai(afe);
    if ret != 0 {
        return dev_err_probe(dev, ret, "mtk_afe_combine_sub_dai fail\n");
    }

    /* others */
    (*afe).mtk_afe_hardware = &mt8189_afe_hardware;
    (*afe).memif_fs = Some(mt8189_memif_fs);
    (*afe).irq_fs = Some(mt8189_irq_fs);
    (*afe).get_dai_fs = Some(mt8189_get_dai_fs);
    (*afe).get_memif_pbuf_size = Some(mt8189_get_memif_pbuf_size);

    (*afe).runtime_resume = Some(mt8189_afe_runtime_resume);
    (*afe).runtime_suspend = Some(mt8189_afe_runtime_suspend);

    ret = devm_pm_runtime_enable(dev);
    if ret != 0 {
        return ret;
    }

    /*
     * Audio device is part of genpd. Registering it as a syscore device
     * ensure the proper power-on sequence of the AFE device.
     */
    dev_pm_syscore_device(dev, true);

    /* enable clock for regcache get default value from hw */
    ret = pm_runtime_resume_and_get(dev);
    if ret != 0 {
        return dev_err_probe(dev, ret, "failed to resume device\n");
    }

    (*afe).regmap = devm_regmap_init_mmio(dev, (*afe).base_addr, &mt8189_afe_regmap_config);
    if IS_ERR((*afe).regmap) {
        ret = PTR_ERR((*afe).regmap);
        pm_runtime_put_sync(dev);
        return ret;
    }

    ret = regmap_register_patch((*afe).regmap, mt8189_cg_patch.as_ptr(), mt8189_cg_patch.len());
    if ret < 0 {
        dev_err!(dev, "Failed to apply cg patch\n");
        pm_runtime_put_sync(dev);
        return ret;
    }

    regmap_read((*afe).regmap, AFE_IRQ_MCU_EN, &mut tmp_reg);
    regmap_write((*afe).regmap, AFE_IRQ_MCU_EN, 0xffffffff);
    regmap_read((*afe).regmap, AFE_IRQ_MCU_EN, &mut tmp_reg);

    pm_runtime_put_sync(dev);

    regcache_cache_only((*afe).regmap, true);
    regcache_mark_dirty((*afe).regmap);

    /* register component */
    ret = devm_snd_soc_register_component(
        dev,
        &mt8189_afe_component,
        (*afe).dai_drivers,
        (*afe).num_dai_drivers,
    );
    if ret != 0 {
        dev_err!(dev, "afe component err: %d\n", ret);
        return ret;
    }

    0
}

unsafe extern "C" fn mt8189_afe_pcm_dev_remove(pdev: *mut platform_device) {
    let afe = platform_get_drvdata(pdev) as *mut mtk_base_afe;
    let dev = &mut (*pdev).dev as *mut device;

    if !pm_runtime_status_suspended(dev) {
        mt8189_afe_runtime_suspend(dev);
    }

    mt8189_afe_disable_main_clock(afe);
    /* disable afe clock */
    mt8189_afe_disable_reg_rw_clk(afe);
}

static mt8189_afe_pcm_dt_match: [of_device_id; 2] = [
    of_device_id {
        compatible: "mediatek,mt8189-afe-pcm\0".as_ptr() as *const i8,
        ..unsafe { core::mem::zeroed() }
    },
    of_device_id {
        ..unsafe { core::mem::zeroed() }
    },
];
MODULE_DEVICE_TABLE!(of, mt8189_afe_pcm_dt_match);

static mt8189_afe_pm_ops: dev_pm_ops =
    SET_RUNTIME_PM_OPS!(mt8189_afe_runtime_suspend, mt8189_afe_runtime_resume, None);

static mut mt8189_afe_pcm_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: "mt8189-afe-pcm\0".as_ptr() as *const i8,
        of_match_table: mt8189_afe_pcm_dt_match.as_ptr(),
        pm: &mt8189_afe_pm_ops,
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(mt8189_afe_pcm_dev_probe),
    remove: Some(mt8189_afe_pcm_dev_remove),
    ..unsafe { core::mem::zeroed() }
};
module_platform_driver!(mt8189_afe_pcm_driver);

MODULE_DESCRIPTION!("Mediatek ALSA SoC AFE platform driver for 8189");
MODULE_AUTHOR!("Darren Ye <darren.ye@mediatek.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
