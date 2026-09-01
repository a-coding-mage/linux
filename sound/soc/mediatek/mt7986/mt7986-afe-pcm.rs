// SPDX-License-Identifier: GPL-2.0
/*
 * MediaTek ALSA SoC AFE platform driver for MT7986
 *
 * Copyright (c) 2023 MediaTek Inc.
 * Authors: Vic Wu <vic.wu@mediatek.com>
 *          Maso Huang <maso.huang@mediatek.com>
 */

// C dependencies:
// linux/clk.h, linux/delay.h, linux/module.h, linux/of.h,
// linux/of_address.h, linux/pm_runtime.h
// "mt7986-afe-common.h", "mt7986-reg.h",
// "../common/mtk-afe-platform-driver.h", "../common/mtk-afe-fe-dai.h"

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

const MTK_AFE_RATE_8K: c_uint = 0;
const MTK_AFE_RATE_11K: c_uint = 1;
const MTK_AFE_RATE_12K: c_uint = 2;
const MTK_AFE_RATE_16K: c_uint = 4;
const MTK_AFE_RATE_22K: c_uint = 5;
const MTK_AFE_RATE_24K: c_uint = 6;
const MTK_AFE_RATE_32K: c_uint = 8;
const MTK_AFE_RATE_44K: c_uint = 9;
const MTK_AFE_RATE_48K: c_uint = 10;
const MTK_AFE_RATE_88K: c_uint = 13;
const MTK_AFE_RATE_96K: c_uint = 14;
const MTK_AFE_RATE_176K: c_uint = 17;
const MTK_AFE_RATE_192K: c_uint = 18;

const CLK_INFRA_AUD_BUS_CK: usize = 0;
const CLK_INFRA_AUD_26M_CK: usize = 1;
const CLK_INFRA_AUD_L_CK: usize = 2;
const CLK_INFRA_AUD_AUD_CK: usize = 3;
const CLK_INFRA_AUD_EG2_CK: usize = 4;
const CLK_NUM: usize = 5;

static AUD_BUS_CK: &[u8] = b"aud_bus_ck\0";
static AUD_26M_CK: &[u8] = b"aud_26m_ck\0";
static AUD_L_CK: &[u8] = b"aud_l_ck\0";
static AUD_AUD_CK: &[u8] = b"aud_aud_ck\0";
static AUD_EG2_CK: &[u8] = b"aud_eg2_ck\0";

static aud_clks: [*const c_char; CLK_NUM] = [
    AUD_BUS_CK.as_ptr() as *const c_char,
    AUD_26M_CK.as_ptr() as *const c_char,
    AUD_L_CK.as_ptr() as *const c_char,
    AUD_AUD_CK.as_ptr() as *const c_char,
    AUD_EG2_CK.as_ptr() as *const c_char,
];

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_bulk_data {
    pub id: *const c_char,
    pub clk: *mut c_void,
}

#[repr(C)]
pub struct list_head {
    pub next: *mut list_head,
    pub prev: *mut list_head,
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_memif_data {
    pub name: *const c_char,
    pub id: c_int,
    pub reg_ofs_base: c_uint,
    pub reg_ofs_cur: c_uint,
    pub reg_ofs_end: c_uint,
    pub reg_ofs_base_msb: c_uint,
    pub reg_ofs_cur_msb: c_uint,
    pub reg_ofs_end_msb: c_uint,
    pub fs_reg: c_uint,
    pub fs_shift: c_uint,
    pub fs_maskbit: c_uint,
    pub mono_reg: c_uint,
    pub mono_shift: c_uint,
    pub enable_reg: c_uint,
    pub enable_shift: c_uint,
    pub hd_reg: c_uint,
    pub hd_shift: c_uint,
    pub hd_align_reg: c_uint,
    pub hd_align_mshift: c_uint,
    pub pbuf_reg: c_uint,
    pub pbuf_shift: c_uint,
    pub minlen_reg: c_uint,
    pub minlen_shift: c_uint,
}

#[repr(C)]
pub struct mtk_base_irq_data {
    pub id: c_int,
    pub irq_cnt_reg: c_uint,
    pub irq_cnt_shift: c_uint,
    pub irq_cnt_maskbit: c_uint,
    pub irq_fs_reg: c_uint,
    pub irq_fs_shift: c_uint,
    pub irq_fs_maskbit: c_uint,
    pub irq_en_reg: c_uint,
    pub irq_en_shift: c_uint,
    pub irq_clr_reg: c_uint,
    pub irq_clr_shift: c_uint,
}

#[repr(C)]
pub struct mtk_base_afe_memif {
    pub data: *const mtk_base_memif_data,
    pub irq_usage: c_int,
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct mtk_base_afe_irq {
    pub irq_data: *const mtk_base_irq_data,
}

#[repr(C)]
pub struct mtk_base_afe_dai {
    pub list: list_head,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub num_dai_drivers: c_int,
    pub dapm_widgets: *const snd_soc_dapm_widget,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
}

#[repr(C)]
pub struct mtk_base_afe {
    pub platform_priv: *mut mt7986_afe_private,
    pub dev: *mut device,
    pub base_addr: *mut c_void,
    pub regmap: *mut regmap,
    pub memif_size: c_int,
    pub memif: *mut mtk_base_afe_memif,
    pub irq_alloc_lock: mutex,
    pub irqs_size: c_int,
    pub irqs: *mut mtk_base_afe_irq,
    pub sub_dais: list_head,
    pub mtk_afe_hardware: *const snd_pcm_hardware,
    pub memif_fs: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_uint) -> c_int>,
    pub irq_fs: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_uint) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub num_dai_drivers: c_int,
}

#[repr(C)]
pub struct mt7986_afe_private {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,
    pub pm_runtime_bypass_reg_ctl: bool,
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub buffer_bytes_max: usize,
    pub fifo_size: c_uint,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const c_void,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    _private: [u8; 0],
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
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub reg_stride: c_int,
    pub val_bits: c_int,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub max_register: c_uint,
    pub num_reg_defaults_raw: c_uint,
}

pub type irqreturn_t = c_uint;
pub type dai_register_cb = Option<unsafe extern "C" fn(*mut mtk_base_afe) -> c_int>;

extern "C" {
    static mtk_afe_fe_ops: c_void;
    static mtk_afe_pcm_platform: snd_soc_component_driver;

    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtdcom_lookup(rtd: *mut snd_soc_pcm_runtime, name: *const c_char) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_clk_bulk_get(dev: *mut device, num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn dev_get_drvdata(dev: *mut device) -> *mut mtk_base_afe;
    fn clk_bulk_disable_unprepare(num_clks: c_int, clks: *mut clk_bulk_data);
    fn clk_bulk_prepare_enable(num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
    fn pm_runtime_get_sync(dev: *mut device) -> c_int;
    fn pm_runtime_put_sync(dev: *mut device) -> c_int;
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn mutex_init(lock: *mut mutex);
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_int,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_uint,
        name: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn mt7986_dai_etdm_register(afe: *mut mtk_base_afe) -> c_int;
    fn mtk_afe_combine_sub_dai(afe: *mut mtk_base_afe) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
}

// External constants supplied by translated headers or other files.
extern "C" {
    static AFE_PCM_NAME: *const c_char;
}

const MTK_PCM_RATES: c_uint = SNDRV_PCM_RATE_8000_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

const MTK_PCM_FORMATS: u64 =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

#[no_mangle]
pub unsafe extern "C" fn mt7986_afe_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
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
        176400 => MTK_AFE_RATE_176K,
        192000 => MTK_AFE_RATE_192K,
        _ => {
            dev_warn(
                dev,
                b"%s(), rate %u invalid, using %d!!!\n\0".as_ptr() as *const c_char,
                b"mt7986_afe_rate_transform\0".as_ptr() as *const c_char,
                rate,
                MTK_AFE_RATE_48K,
            );
            MTK_AFE_RATE_48K
        }
    }
}

static mt7986_afe_hardware: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP | SNDRV_PCM_INFO_INTERLEAVED | SNDRV_PCM_INFO_MMAP_VALID,
    formats: SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    period_bytes_min: 256,
    period_bytes_max: 4 * 48 * 1024,
    periods_min: 2,
    periods_max: 256,
    buffer_bytes_max: 8 * 48 * 1024,
    fifo_size: 0,
};

unsafe extern "C" fn mt7986_memif_fs(substream: *mut snd_pcm_substream, rate: c_uint) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let afe = snd_soc_component_get_drvdata(component);

    mt7986_afe_rate_transform((*afe).dev, rate) as c_int
}

unsafe extern "C" fn mt7986_irq_fs(substream: *mut snd_pcm_substream, rate: c_uint) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME);
    let afe = snd_soc_component_get_drvdata(component);

    mt7986_afe_rate_transform((*afe).dev, rate) as c_int
}

static mut mt7986_memif_dai_driver: [snd_soc_dai_driver; 2] = [
    /* FE DAIs: memory intefaces to CPU */
    snd_soc_dai_driver {
        name: b"DL1\0".as_ptr() as *const c_char,
        id: MT7986_MEMIF_DL1,
        playback: snd_soc_pcm_stream {
            stream_name: b"DL1\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
        },
        capture: snd_soc_pcm_stream {
            stream_name: null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        ops: unsafe { &mtk_afe_fe_ops as *const c_void },
    },
    snd_soc_dai_driver {
        name: b"UL1\0".as_ptr() as *const c_char,
        id: MT7986_MEMIF_VUL12,
        playback: snd_soc_pcm_stream {
            stream_name: null(),
            channels_min: 0,
            channels_max: 0,
            rates: 0,
            formats: 0,
        },
        capture: snd_soc_pcm_stream {
            stream_name: b"UL1\0".as_ptr() as *const c_char,
            channels_min: 1,
            channels_max: 2,
            rates: MTK_PCM_RATES,
            formats: MTK_PCM_FORMATS,
        },
        ops: unsafe { &mtk_afe_fe_ops as *const c_void },
    },
];

// SOC_DAPM_SINGLE_AUTODISABLE initializers are provided by ALSA headers in C.
static o018_mix: [snd_kcontrol_new; 1] = [SOC_DAPM_SINGLE_AUTODISABLE!(
    "I150_Switch",
    AFE_CONN018_4,
    22,
    1,
    0
)];

static o019_mix: [snd_kcontrol_new; 1] = [SOC_DAPM_SINGLE_AUTODISABLE!(
    "I151_Switch",
    AFE_CONN019_4,
    23,
    1,
    0
)];

static mt7986_memif_widgets: [snd_soc_dapm_widget; 4] = [
    /* DL */
    SND_SOC_DAPM_MIXER!("I032", SND_SOC_NOPM, 0, 0, null(), 0),
    SND_SOC_DAPM_MIXER!("I033", SND_SOC_NOPM, 0, 0, null(), 0),
    /* UL */
    SND_SOC_DAPM_MIXER!("O018", SND_SOC_NOPM, 0, 0, o018_mix.as_ptr(), o018_mix.len()),
    SND_SOC_DAPM_MIXER!("O019", SND_SOC_NOPM, 0, 0, o019_mix.as_ptr(), o019_mix.len()),
];

static mt7986_memif_routes: [snd_soc_dapm_route; 6] = [
    snd_soc_dapm_route { sink: b"I032\0".as_ptr() as *const c_char, control: null(), source: b"DL1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"I033\0".as_ptr() as *const c_char, control: null(), source: b"DL1\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"UL1\0".as_ptr() as *const c_char, control: null(), source: b"O018\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"UL1\0".as_ptr() as *const c_char, control: null(), source: b"O019\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"O018\0".as_ptr() as *const c_char, control: b"I150_Switch\0".as_ptr() as *const c_char, source: b"I150\0".as_ptr() as *const c_char },
    snd_soc_dapm_route { sink: b"O019\0".as_ptr() as *const c_char, control: b"I151_Switch\0".as_ptr() as *const c_char, source: b"I151\0".as_ptr() as *const c_char },
];

static mt7986_afe_pcm_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"mt7986-afe-pcm-dai\0".as_ptr() as *const c_char,
};

static memif_data: [mtk_base_memif_data; MT7986_MEMIF_NUM as usize] = [
    mtk_base_memif_data {
        name: b"DL1\0".as_ptr() as *const c_char,
        id: MT7986_MEMIF_DL1,
        reg_ofs_base: AFE_DL0_BASE,
        reg_ofs_cur: AFE_DL0_CUR,
        reg_ofs_end: AFE_DL0_END,
        reg_ofs_base_msb: AFE_DL0_BASE_MSB,
        reg_ofs_cur_msb: AFE_DL0_CUR_MSB,
        reg_ofs_end_msb: AFE_DL0_END_MSB,
        fs_reg: AFE_DL0_CON0,
        fs_shift: DL0_MODE_SFT,
        fs_maskbit: DL0_MODE_MASK,
        mono_reg: AFE_DL0_CON0,
        mono_shift: DL0_MONO_SFT,
        enable_reg: AFE_DL0_CON0,
        enable_shift: DL0_ON_SFT,
        hd_reg: AFE_DL0_CON0,
        hd_shift: DL0_HD_MODE_SFT,
        hd_align_reg: AFE_DL0_CON0,
        hd_align_mshift: DL0_HALIGN_SFT,
        pbuf_reg: AFE_DL0_CON0,
        pbuf_shift: DL0_PBUF_SIZE_SFT,
        minlen_reg: AFE_DL0_CON0,
        minlen_shift: DL0_MINLEN_SFT,
    },
    mtk_base_memif_data {
        name: b"VUL12\0".as_ptr() as *const c_char,
        id: MT7986_MEMIF_VUL12,
        reg_ofs_base: AFE_VUL0_BASE,
        reg_ofs_cur: AFE_VUL0_CUR,
        reg_ofs_end: AFE_VUL0_END,
        reg_ofs_base_msb: AFE_VUL0_BASE_MSB,
        reg_ofs_cur_msb: AFE_VUL0_CUR_MSB,
        reg_ofs_end_msb: AFE_VUL0_END_MSB,
        fs_reg: AFE_VUL0_CON0,
        fs_shift: VUL0_MODE_SFT,
        fs_maskbit: VUL0_MODE_MASK,
        mono_reg: AFE_VUL0_CON0,
        mono_shift: VUL0_MONO_SFT,
        enable_reg: AFE_VUL0_CON0,
        enable_shift: VUL0_ON_SFT,
        hd_reg: AFE_VUL0_CON0,
        hd_shift: VUL0_HD_MODE_SFT,
        hd_align_reg: AFE_VUL0_CON0,
        hd_align_mshift: VUL0_HALIGN_SFT,
        pbuf_reg: 0,
        pbuf_shift: 0,
        minlen_reg: 0,
        minlen_shift: 0,
    },
];

static irq_data: [mtk_base_irq_data; MT7986_IRQ_NUM as usize] = [
    mtk_base_irq_data {
        id: MT7986_IRQ_0,
        irq_cnt_reg: AFE_IRQ0_MCU_CFG1,
        irq_cnt_shift: AFE_IRQ_CNT_SHIFT,
        irq_cnt_maskbit: AFE_IRQ_CNT_MASK,
        irq_fs_reg: AFE_IRQ0_MCU_CFG0,
        irq_fs_shift: IRQ_MCU_MODE_SFT,
        irq_fs_maskbit: IRQ_MCU_MODE_MASK,
        irq_en_reg: AFE_IRQ0_MCU_CFG0,
        irq_en_shift: IRQ_MCU_ON_SFT,
        irq_clr_reg: AFE_IRQ_MCU_CLR,
        irq_clr_shift: IRQ0_MCU_CLR_SFT,
    },
    mtk_base_irq_data {
        id: MT7986_IRQ_1,
        irq_cnt_reg: AFE_IRQ1_MCU_CFG1,
        irq_cnt_shift: AFE_IRQ_CNT_SHIFT,
        irq_cnt_maskbit: AFE_IRQ_CNT_MASK,
        irq_fs_reg: AFE_IRQ1_MCU_CFG0,
        irq_fs_shift: IRQ_MCU_MODE_SFT,
        irq_fs_maskbit: IRQ_MCU_MODE_MASK,
        irq_en_reg: AFE_IRQ1_MCU_CFG0,
        irq_en_shift: IRQ_MCU_ON_SFT,
        irq_clr_reg: AFE_IRQ_MCU_CLR,
        irq_clr_shift: IRQ1_MCU_CLR_SFT,
    },
    mtk_base_irq_data {
        id: MT7986_IRQ_2,
        irq_cnt_reg: AFE_IRQ2_MCU_CFG1,
        irq_cnt_shift: AFE_IRQ_CNT_SHIFT,
        irq_cnt_maskbit: AFE_IRQ_CNT_MASK,
        irq_fs_reg: AFE_IRQ2_MCU_CFG0,
        irq_fs_shift: IRQ_MCU_MODE_SFT,
        irq_fs_maskbit: IRQ_MCU_MODE_MASK,
        irq_en_reg: AFE_IRQ2_MCU_CFG0,
        irq_en_shift: IRQ_MCU_ON_SFT,
        irq_clr_reg: AFE_IRQ_MCU_CLR,
        irq_clr_shift: IRQ2_MCU_CLR_SFT,
    },
];

unsafe extern "C" fn mt7986_is_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    /*
     * Those auto-gen regs are read-only, so put it as volatile because
     * volatile registers cannot be cached, which means that they cannot
     * be set when power is off
     */
    match reg {
        AFE_DL0_CUR_MSB | AFE_DL0_CUR | AFE_DL0_RCH_MON | AFE_DL0_LCH_MON
        | AFE_VUL0_CUR_MSB | AFE_VUL0_CUR | AFE_IRQ_MCU_STATUS | AFE_MEMIF_RD_MON
        | AFE_MEMIF_WR_MON => true,
        _ => false,
    }
}

static mt7986_afe_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    volatile_reg: Some(mt7986_is_volatile_reg),
    max_register: AFE_MAX_REGISTER,
    num_reg_defaults_raw: (AFE_MAX_REGISTER / 4) + 1,
};

unsafe extern "C" fn mt7986_init_clock(afe: *mut mtk_base_afe) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let mut ret: c_int;
    let mut i: c_int;

    (*afe_priv).clks = devm_kcalloc(
        (*afe).dev,
        CLK_NUM,
        size_of::<clk_bulk_data>(),
        GFP_KERNEL,
    ) as *mut clk_bulk_data;
    if (*afe_priv).clks.is_null() {
        return -ENOMEM;
    }
    (*afe_priv).num_clks = CLK_NUM as c_int;

    i = 0;
    while i < (*afe_priv).num_clks {
        (*(*afe_priv).clks.add(i as usize)).id = aud_clks[i as usize];
        i += 1;
    }

    ret = devm_clk_bulk_get((*afe).dev, (*afe_priv).num_clks, (*afe_priv).clks);
    if ret != 0 {
        return dev_err_probe(
            (*afe).dev,
            ret,
            b"Failed to get clocks\n\0".as_ptr() as *const c_char,
        );
    }

    0
}

unsafe extern "C" fn mt7986_afe_irq_handler(irq_id: c_int, dev: *mut c_void) -> irqreturn_t {
    let afe = dev as *mut mtk_base_afe;
    let mut irq: *mut mtk_base_afe_irq;
    let mut mcu_en: c_uint = 0;
    let mut status: c_uint = 0;
    let status_mcu: c_uint;
    let mut i: c_int;
    let ret: c_int;
    let mut irq_ret: irqreturn_t = IRQ_HANDLED;

    /* get irq that is sent to MCU */
    regmap_read((*afe).regmap, AFE_IRQ_MCU_EN, &mut mcu_en);

    ret = regmap_read((*afe).regmap, AFE_IRQ_MCU_STATUS, &mut status);
    /* only care IRQ which is sent to MCU */
    status_mcu = status & mcu_en & AFE_IRQ_STATUS_BITS;

    if ret != 0 || status_mcu == 0 {
        dev_err(
            (*afe).dev,
            b"%s(), irq status err, ret %d, status 0x%x, mcu_en 0x%x\n\0".as_ptr()
                as *const c_char,
            b"mt7986_afe_irq_handler\0".as_ptr() as *const c_char,
            ret,
            status,
            mcu_en,
        );

        irq_ret = IRQ_NONE;
    } else {
        i = 0;
        while i < MT7986_MEMIF_NUM {
            let memif = (*afe).memif.add(i as usize);

            if (*memif).substream.is_null() {
                i += 1;
                continue;
            }

            if (*memif).irq_usage < 0 {
                i += 1;
                continue;
            }

            irq = (*afe).irqs.add((*memif).irq_usage as usize);

            if (status_mcu & (1u32 << (*(*irq).irq_data).irq_en_shift)) != 0 {
                snd_pcm_period_elapsed((*memif).substream);
            }

            i += 1;
        }
    }

    /* clear irq */
    regmap_write((*afe).regmap, AFE_IRQ_MCU_CLR, status_mcu);

    irq_ret
}

unsafe extern "C" fn mt7986_afe_runtime_suspend(dev: *mut device) -> c_int {
    let afe = dev_get_drvdata(dev);
    let afe_priv = (*afe).platform_priv;

    if !(*afe).regmap.is_null() && !(*afe_priv).pm_runtime_bypass_reg_ctl {
        /* disable clk*/
        regmap_update_bits((*afe).regmap, AUDIO_TOP_CON4, 0x3fff, 0x3fff);
        regmap_update_bits((*afe).regmap, AUDIO_ENGEN_CON0, AUD_APLL2_EN_MASK, 0);
        regmap_update_bits((*afe).regmap, AUDIO_ENGEN_CON0, AUD_26M_EN_MASK, 0);

        /* make sure all irq status are cleared, twice intended */
        regmap_update_bits((*afe).regmap, AFE_IRQ_MCU_CLR, 0xffff, 0xffff);
    }

    clk_bulk_disable_unprepare((*afe_priv).num_clks, (*afe_priv).clks);

    0
}

unsafe extern "C" fn mt7986_afe_runtime_resume(dev: *mut device) -> c_int {
    let afe = dev_get_drvdata(dev);
    let afe_priv = (*afe).platform_priv;
    let ret: c_int;

    ret = clk_bulk_prepare_enable((*afe_priv).num_clks, (*afe_priv).clks);
    if ret != 0 {
        return dev_err_probe(
            (*afe).dev,
            ret,
            b"Failed to enable clocks\n\0".as_ptr() as *const c_char,
        );
    }

    if (*afe).regmap.is_null() || (*afe_priv).pm_runtime_bypass_reg_ctl {
        return 0;
    }

    /* enable clk*/
    regmap_update_bits((*afe).regmap, AUDIO_TOP_CON4, 0x3fff, 0);
    regmap_update_bits(
        (*afe).regmap,
        AUDIO_ENGEN_CON0,
        AUD_APLL2_EN_MASK,
        AUD_APLL2_EN,
    );
    regmap_update_bits(
        (*afe).regmap,
        AUDIO_ENGEN_CON0,
        AUD_26M_EN_MASK,
        AUD_26M_EN,
    );

    0
}

unsafe extern "C" fn mt7986_dai_memif_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, size_of::<mtk_base_afe_dai>(), GFP_KERNEL)
        as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mt7986_memif_dai_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mt7986_memif_dai_driver.len() as c_int;

    (*dai).dapm_widgets = mt7986_memif_widgets.as_ptr();
    (*dai).num_dapm_widgets = mt7986_memif_widgets.len() as c_int;
    (*dai).dapm_routes = mt7986_memif_routes.as_ptr();
    (*dai).num_dapm_routes = mt7986_memif_routes.len() as c_int;

    0
}

static dai_register_cbs: [dai_register_cb; 2] = [
    Some(mt7986_dai_etdm_register),
    Some(mt7986_dai_memif_register),
];

unsafe extern "C" fn mt7986_afe_pcm_dev_probe(pdev: *mut platform_device) -> c_int {
    let mut afe: *mut mtk_base_afe;
    let mut afe_priv: *mut mt7986_afe_private;
    let dev: *mut device;
    let mut i: c_int;
    let irq_id: c_int;
    let mut ret: c_int;

    afe = devm_kzalloc(&mut (*pdev).dev, size_of::<mtk_base_afe>(), GFP_KERNEL)
        as *mut mtk_base_afe;
    if afe.is_null() {
        return -ENOMEM;
    }
    platform_set_drvdata(pdev, afe as *mut c_void);

    (*afe).platform_priv = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<mt7986_afe_private>(),
        GFP_KERNEL,
    ) as *mut mt7986_afe_private;
    if (*afe).platform_priv.is_null() {
        return -ENOMEM;
    }

    afe_priv = (*afe).platform_priv;
    (*afe).dev = &mut (*pdev).dev;
    dev = (*afe).dev;

    (*afe).base_addr = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*afe).base_addr) {
        return PTR_ERR((*afe).base_addr);
    }

    /* initial audio related clock */
    ret = mt7986_init_clock(afe);
    if ret != 0 {
        return dev_err_probe(dev, ret, b"Cannot initialize clocks\n\0".as_ptr() as *const c_char);
    }

    ret = devm_pm_runtime_enable(dev);
    if ret != 0 {
        return ret;
    }

    /* enable clock for regcache get default value from hw */
    (*afe_priv).pm_runtime_bypass_reg_ctl = true;
    pm_runtime_get_sync(&mut (*pdev).dev);

    (*afe).regmap = devm_regmap_init_mmio(
        &mut (*pdev).dev,
        (*afe).base_addr,
        &mt7986_afe_regmap_config,
    );

    pm_runtime_put_sync(&mut (*pdev).dev);
    if IS_ERR((*afe).regmap as *const c_void) {
        return PTR_ERR((*afe).regmap as *const c_void);
    }

    (*afe_priv).pm_runtime_bypass_reg_ctl = false;

    /* init memif */
    (*afe).memif_size = MT7986_MEMIF_NUM;
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
    (*afe).irqs_size = MT7986_IRQ_NUM;
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
        mt7986_afe_irq_handler,
        IRQF_TRIGGER_NONE,
        b"asys-isr\0".as_ptr() as *const c_char,
        afe as *mut c_void,
    );
    if ret != 0 {
        return dev_err_probe(
            dev,
            ret,
            b"Failed to request irq for asys-isr\n\0".as_ptr() as *const c_char,
        );
    }

    /* init sub_dais */
    INIT_LIST_HEAD(&mut (*afe).sub_dais);

    i = 0;
    while (i as usize) < dai_register_cbs.len() {
        ret = dai_register_cbs[i as usize].unwrap()(afe);
        if ret != 0 {
            return dev_err_probe(
                dev,
                ret,
                b"DAI register failed, i: %d\n\0".as_ptr() as *const c_char,
                i,
            );
        }
        i += 1;
    }

    /* init dai_driver and component_driver */
    ret = mtk_afe_combine_sub_dai(afe);
    if ret != 0 {
        return dev_err_probe(
            dev,
            ret,
            b"mtk_afe_combine_sub_dai fail\n\0".as_ptr() as *const c_char,
        );
    }

    (*afe).mtk_afe_hardware = &mt7986_afe_hardware;
    (*afe).memif_fs = Some(mt7986_memif_fs);
    (*afe).irq_fs = Some(mt7986_irq_fs);

    (*afe).runtime_resume = Some(mt7986_afe_runtime_resume);
    (*afe).runtime_suspend = Some(mt7986_afe_runtime_suspend);

    /* register component */
    ret = devm_snd_soc_register_component(&mut (*pdev).dev, &mtk_afe_pcm_platform, null_mut(), 0);
    if ret != 0 {
        return dev_err_probe(dev, ret, b"Cannot register AFE component\n\0".as_ptr() as *const c_char);
    }

    ret = devm_snd_soc_register_component(
        (*afe).dev,
        &mt7986_afe_pcm_dai_component,
        (*afe).dai_drivers,
        (*afe).num_dai_drivers,
    );
    if ret != 0 {
        return dev_err_probe(
            dev,
            ret,
            b"Cannot register PCM DAI component\n\0".as_ptr() as *const c_char,
        );
    }

    0
}

unsafe extern "C" fn mt7986_afe_pcm_dev_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
    if !pm_runtime_status_suspended(&mut (*pdev).dev) {
        mt7986_afe_runtime_suspend(&mut (*pdev).dev);
    }
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

static mt7986_afe_pcm_dt_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"mediatek,mt7986-afe\0".as_ptr() as *const c_char,
    },
    of_device_id {
        /* sentinel */
        compatible: null(),
    },
];
// MODULE_DEVICE_TABLE(of, mt7986_afe_pcm_dt_match);

#[repr(C)]
pub struct dev_pm_ops {
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub idle: *const c_void,
}

static mt7986_afe_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(mt7986_afe_runtime_suspend),
    runtime_resume: Some(mt7986_afe_runtime_resume),
    idle: null(),
};

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

static mut mt7986_afe_pcm_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: b"mt7986-audio\0".as_ptr() as *const c_char,
        of_match_table: mt7986_afe_pcm_dt_match.as_ptr(),
        pm: &mt7986_afe_pm_ops,
    },
    probe: Some(mt7986_afe_pcm_dev_probe),
    remove: Some(mt7986_afe_pcm_dev_remove),
};
// module_platform_driver(mt7986_afe_pcm_driver);

// MODULE_DESCRIPTION("MediaTek SoC AFE platform driver for ALSA MT7986");
// MODULE_AUTHOR("Vic Wu <vic.wu@mediatek.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
