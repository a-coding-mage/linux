// SPDX-License-Identifier: GPL-2.0
/*
 *  Mediatek ALSA SoC AFE platform driver for 8196
 *
 *  Copyright (c) 2025 MediaTek Inc.
 *  Author: Darren Ye <darren.ye@mediatek.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

// C include dependencies translated as external Rust dependencies:
// linux/delay.h, linux/dma-mapping.h, linux/module.h, linux/of.h,
// linux/of_address.h, linux/of_device.h, linux/of_reserved_mem.h,
// linux/pm_runtime.h, linux/regmap.h, sound/pcm.h, sound/soc.h,
// mt8196-afe-clk.h, mt8196-afe-common.h, mt8196-interconnection.h,
// ../common/mtk-afe-fe-dai.h, ../common/mtk-afe-platform-driver.h.

type bool_ = bool;
type u32 = u32;
type u64 = u64;
type irqreturn_t = c_int;
type dma_addr_t = u64;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
    pub wait_time: c_ulong,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub period_size: c_uint,
    pub rate: c_uint,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai {
    pub id: c_int,
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
pub struct snd_pcm {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_context {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub name: *const c_char,
    pub dapm: *mut snd_soc_dapm_context,
}
#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}
#[repr(C)]
pub struct timespec64 {
    _private: [u8; 0],
}
#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: c_uint,
    pub period_bytes_min: c_uint,
    pub period_bytes_max: c_uint,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub buffer_bytes_max: c_uint,
    pub fifo_size: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub hw_params: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int>,
    pub hw_free: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub id: c_int,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dapm_widget_desc {
    _private: [u8; 0],
}
type snd_soc_dapm_widget_item = snd_soc_dapm_widget_desc;

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
}

#[repr(C)]
pub struct mtk_base_memif_data {
    pub name: *const c_char,
    pub id: c_int,
    pub reg_ofs_base: c_int,
    pub reg_ofs_cur: c_int,
    pub reg_ofs_end: c_int,
    pub reg_ofs_base_msb: c_int,
    pub reg_ofs_cur_msb: c_int,
    pub reg_ofs_end_msb: c_int,
    pub fs_reg: c_int,
    pub fs_shift: c_int,
    pub fs_maskbit: c_int,
    pub mono_reg: c_int,
    pub mono_shift: c_int,
    pub enable_reg: c_int,
    pub enable_shift: c_int,
    pub hd_reg: c_int,
    pub hd_shift: c_int,
    pub hd_align_reg: c_int,
    pub hd_align_mshift: c_int,
    pub agent_disable_reg: c_int,
    pub agent_disable_shift: c_int,
    pub msb_reg: c_int,
    pub msb_shift: c_int,
    pub pbuf_reg: c_int,
    pub pbuf_mask: c_int,
    pub pbuf_shift: c_int,
    pub minlen_reg: c_int,
    pub minlen_mask: c_int,
    pub minlen_shift: c_int,
    pub ch_num_reg: c_int,
    pub ch_num_maskbit: c_int,
    pub ch_num_shift: c_int,
}

#[repr(C)]
pub struct mtk_base_irq_data {
    pub id: c_int,
    pub irq_cnt_reg: c_int,
    pub irq_cnt_shift: c_int,
    pub irq_cnt_maskbit: c_int,
    pub irq_fs_reg: c_int,
    pub irq_fs_shift: c_int,
    pub irq_fs_maskbit: c_int,
    pub irq_en_reg: c_int,
    pub irq_en_shift: c_int,
    pub irq_clr_reg: c_int,
    pub irq_clr_shift: c_int,
}

#[repr(C)]
pub struct mtk_base_afe_memif {
    pub substream: *mut snd_pcm_substream,
    pub irq_usage: c_int,
    pub const_irq: c_int,
    pub data: *const mtk_base_memif_data,
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
    pub dapm_widgets: *const snd_soc_dapm_widget_item,
    pub num_dapm_widgets: c_int,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_int,
}
#[repr(C)]
pub struct mt8196_afe_private {
    pub cm_rate: [c_uint; 3],
    pub cm_channels: c_uint,
}
#[repr(C)]
pub struct mtk_base_afe {
    pub platform_priv: *mut mt8196_afe_private,
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub memif: *mut mtk_base_afe_memif,
    pub memif_size: c_int,
    pub memif_32bit_supported: c_int,
    pub irqs: *mut mtk_base_afe_irq,
    pub irqs_size: c_int,
    pub irq_alloc_lock: c_void,
    pub sub_dais: list_head,
    pub dai_drivers: *mut snd_soc_dai_driver,
    pub num_dai_drivers: c_int,
    pub mtk_afe_hardware: *const snd_pcm_hardware,
    pub base_addr: *mut c_void,
    pub memif_fs: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_uint) -> c_int>,
    pub irq_fs: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_uint) -> c_int>,
    pub get_dai_fs: Option<unsafe extern "C" fn(*mut mtk_base_afe, c_int, c_uint) -> c_int>,
    pub get_memif_pbuf_size: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub reg_stride: c_int,
    pub val_bits: c_int,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub max_register: c_uint,
    pub num_reg_defaults_raw: c_uint,
    pub cache_type: c_int,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub pcm_new: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm) -> c_int>,
    pub pcm_free: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm)>,
    pub open: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_int>,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_soc_component, *mut snd_pcm_substream) -> c_ulong>,
}
#[repr(C)]
pub struct reg_sequence {
    pub reg: c_uint,
    pub def: c_uint,
}
type dai_register_cb = Option<unsafe extern "C" fn(*mut mtk_base_afe) -> c_int>;

extern "C" {
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: c_int, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_int, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_int, val: c_uint) -> c_int;
    fn regmap_register_patch(map: *mut regmap, regs: *const reg_sequence, num_regs: c_int) -> c_int;
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn regmap_read_poll_timeout(map: *mut regmap, addr: c_int, val: c_uint, cond: bool, sleep_us: c_int, timeout_us: c_int) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut mtk_base_afe;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, n: c_int) -> *mut snd_soc_dai;
    fn snd_pcm_hw_constraint_step(runtime: *mut snd_pcm_runtime, cond: c_uint, var: c_int, step: c_ulong) -> c_int;
    fn snd_soc_set_runtime_hwparams(substream: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn mtk_dynamic_irq_acquire(afe: *mut mtk_base_afe) -> c_int;
    fn mtk_dynamic_irq_release(afe: *mut mtk_base_afe, irq_id: c_int);
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn mtk_afe_fe_hw_params(substream: *mut snd_pcm_substream, params: *mut snd_pcm_hw_params, dai: *mut snd_soc_dai) -> c_int;
    fn mtk_afe_fe_hw_free(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int;
    fn mtk_afe_fe_prepare(substream: *mut snd_pcm_substream, dai: *mut snd_soc_dai) -> c_int;
    fn mtk_memif_set_enable(afe: *mut mtk_base_afe, id: c_int) -> c_int;
    fn mtk_memif_set_disable(afe: *mut mtk_base_afe, id: c_int) -> c_int;
    fn udelay(usecs: c_ulong);
    fn snd_soc_rtdcom_lookup(rtd: *mut snd_soc_pcm_runtime, name: *const c_char) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut mtk_base_afe;
    fn snd_soc_dapm_to_component(dapm: *mut snd_soc_dapm_context) -> *mut snd_soc_component;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn ktime_get_ts64(ts: *mut timespec64);
    fn ktime_get_ns() -> u64;
    fn dev_get_drvdata(dev: *mut device) -> *mut mtk_base_afe;
    fn pm_runtime_resume_and_get(dev: *mut device) -> c_int;
    fn pm_runtime_put_sync(dev: *mut device) -> c_int;
    fn msecs_to_jiffies(msecs: c_uint) -> c_ulong;
    fn snd_pcm_lib_preallocate_free_for_all(pcm: *mut snd_pcm);
    fn mtk_afe_add_sub_dai_control(component: *mut snd_soc_component);
    fn mtk_afe_pcm_new(component: *mut snd_soc_component, pcm: *mut snd_pcm) -> c_int;
    fn mtk_afe_pcm_pointer(component: *mut snd_soc_component, substream: *mut snd_pcm_substream) -> c_ulong;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: c_uint) -> *mut c_void;
    fn devm_kcalloc(dev: *mut device, n: c_int, size: usize, gfp: c_uint) -> *mut c_void;
    fn list_add(new: *mut list_head, head: *mut list_head);
    fn of_reserved_mem_device_release(data: *mut c_void);
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn of_reserved_mem_device_init(dev: *mut device) -> c_int;
    fn devm_add_action_or_reset(dev: *mut device, action: unsafe extern "C" fn(*mut c_void), data: *mut c_void) -> c_int;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut mtk_base_afe;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn mt8196_init_clock(afe: *mut mtk_base_afe) -> c_int;
    fn mutex_init(lock: *mut c_void);
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(dev: *mut device, irq: c_int, handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t, flags: c_ulong, name: *const c_char, data: *mut c_void) -> c_int;
    fn INIT_LIST_HEAD(list: *mut list_head);
    fn mtk_afe_combine_sub_dai(afe: *mut mtk_base_afe) -> c_int;
    fn mt8196_dai_adda_register(afe: *mut mtk_base_afe) -> c_int;
    fn mt8196_dai_i2s_register(afe: *mut mtk_base_afe) -> c_int;
    fn mt8196_dai_tdm_register(afe: *mut mtk_base_afe) -> c_int;
    fn devm_pm_runtime_enable(dev: *mut device) -> c_int;
    fn dev_pm_syscore_device(dev: *mut device, enable: bool);
    fn devm_regmap_init_mmio(dev: *mut device, regs: *mut c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_snd_soc_register_component(dev: *mut device, cmpnt_drv: *const snd_soc_component_driver, dai_drv: *mut snd_soc_dai_driver, num_dai: c_int) -> c_int;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn mt8196_afe_disable_main_clock(afe: *mut mtk_base_afe);
    fn mt8196_afe_enable_main_clock(afe: *mut mtk_base_afe);
    fn mt8196_afe_disable_reg_rw_clk(afe: *mut mtk_base_afe);
    fn mt8196_afe_enable_reg_rw_clk(afe: *mut mtk_base_afe) -> c_int;
}

const fn BIT(n: c_int) -> c_uint {
    1u32 << n
}
const fn DMA_BIT_MASK(n: c_int) -> u64 {
    if n == 64 { !0u64 } else { (1u64 << n) - 1 }
}

static mt8196_afe_hardware: snd_pcm_hardware = snd_pcm_hardware {
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

unsafe extern "C" fn mt8196_rate_transform(dev: *mut device, rate: c_uint) -> c_uint {
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
            dev_err(dev, b"rate %u invalid, use %d!!!\n\0".as_ptr() as *const c_char, rate, MTK_AFE_IPM2P0_RATE_48K);
            MTK_AFE_IPM2P0_RATE_48K
        }
    }
}

unsafe extern "C" fn mt8196_set_cm(
    afe: *mut mtk_base_afe,
    id: c_int,
    update: bool,
    swap: bool,
    mut ch: c_uint,
) -> c_int {
    let afe_priv = (*afe).platform_priv;
    let rate = (*afe_priv).cm_rate[id as usize];
    let rate_val = mt8196_rate_transform((*afe).dev, rate);
    let ch_pair = ch / 2;
    let update_val: c_uint;
    let reg = AFE_CM0_CON0 + 0x10 * id;

    if update && ch_pair != 0 {
        update_val = (26000000 / rate - 10) / ch_pair - 1;
    } else {
        update_val = 0x64;
    }

    dev_dbg(
        (*afe).dev,
        b"CM%d, rate %d, update %d, swap %d, ch %d, update_val: %d\n\0".as_ptr() as *const c_char,
        id,
        rate,
        update as c_int,
        swap as c_int,
        ch,
        update_val,
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
    ch = ch - 1;
    regmap_update_bits(
        (*afe).regmap,
        reg,
        AFE_CM_CH_NUM_MASK << AFE_CM_CH_NUM_SFT,
        ch << AFE_CM_CH_NUM_SFT,
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

unsafe extern "C" fn mt8196_enable_cm_bypass(afe: *mut mtk_base_afe, id: c_int, en: bool) -> c_int {
    regmap_update_bits(
        (*afe).regmap,
        AFE_CM0_CON0 + 0x10 * id,
        AFE_CM_BYPASS_MODE_MASK << AFE_CM_BYPASS_MODE_SFT,
        (en as c_uint) << AFE_CM_BYPASS_MODE_SFT,
    )
}

unsafe extern "C" fn mt8196_fe_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let afe = snd_soc_dai_get_drvdata(dai);
    let runtime = (*substream).runtime;
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let memif_num = (*cpu_dai).id;
    let memif = (*afe).memif.add(memif_num as usize);
    let mtk_afe_hardware = (*afe).mtk_afe_hardware;
    let mut ret: c_int;

    dev_dbg((*afe).dev, b"memif_num: %d.\n\0".as_ptr() as *const c_char, memif_num);

    (*memif).substream = substream;

    snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_BUFFER_BYTES, 16);

    if memif_num == MT8196_MEMIF_VUL_CM0 {
        snd_pcm_hw_constraint_step((*substream).runtime, 0, SNDRV_PCM_HW_PARAM_PERIOD_BYTES, 16);
    }

    snd_soc_set_runtime_hwparams(substream, mtk_afe_hardware);

    ret = snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS);
    if ret < 0 {
        dev_warn((*afe).dev, b"snd_pcm_hw_constraint_integer failed\n\0".as_ptr() as *const c_char);
    }

    /* dynamic allocate irq to memif */
    if (*memif).irq_usage < 0 {
        let irq_id = mtk_dynamic_irq_acquire(afe);

        if irq_id != (*afe).irqs_size {
            /* link */
            (*memif).irq_usage = irq_id;
        } else {
            dev_err((*afe).dev, b"no more asys irq\n\0".as_ptr() as *const c_char);
            ret = -EBUSY;
        }
    }
    ret
}

unsafe extern "C" fn mt8196_fe_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let rtd = snd_soc_substream_to_rtd(substream);
    let afe = snd_soc_dai_get_drvdata(dai);
    let cpu_dai = snd_soc_rtd_to_cpu(rtd, 0);
    let memif_num = (*cpu_dai).id;
    let memif = (*afe).memif.add(memif_num as usize);
    let irq_id = (*memif).irq_usage;

    dev_dbg((*afe).dev, b"memif_num: %d.\n\0".as_ptr() as *const c_char, memif_num);

    (*memif).substream = ptr::null_mut();

    if (*memif).const_irq == 0 {
        mtk_dynamic_irq_release(afe, irq_id);
        (*memif).irq_usage = -1;
        (*memif).substream = ptr::null_mut();
    }
}

unsafe extern "C" fn mt8196_fe_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let afe = snd_soc_dai_get_drvdata(dai);
    let channels = params_channels(params);
    let afe_priv = (*afe).platform_priv;
    let id = (*snd_soc_rtd_to_cpu(rtd, 0)).id;
    let memif = (*afe).memif.add(id as usize);
    let data = (*memif).data;
    let cm: c_int;

    (*afe_priv).cm_channels = channels;

    /* set channels */
    if (*data).ch_num_shift >= 0 {
        regmap_update_bits(
            (*afe).regmap,
            (*data).ch_num_reg,
            ((*data).ch_num_maskbit << (*data).ch_num_shift) as c_uint,
            channels << (*data).ch_num_shift,
        );
    }

    match id {
        MT8196_MEMIF_VUL8 | MT8196_MEMIF_VUL_CM0 => cm = CM0,
        MT8196_MEMIF_VUL9 | MT8196_MEMIF_VUL_CM1 => cm = CM1,
        MT8196_MEMIF_VUL10 | MT8196_MEMIF_VUL_CM2 => cm = CM2,
        _ => cm = CM0,
    }

    (*afe_priv).cm_rate[cm as usize] = params_rate(params);

    mtk_afe_fe_hw_params(substream, params, dai)
}

unsafe extern "C" fn mt8196_fe_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let runtime = (*substream).runtime;
    let afe = snd_soc_dai_get_drvdata(dai);
    let id = (*snd_soc_rtd_to_cpu(rtd, 0)).id;
    let memif = (*afe).memif.add(id as usize);
    let irq_id = (*memif).irq_usage;
    let irqs = (*afe).irqs.add(irq_id as usize);
    let irq_data_ptr = (*irqs).irq_data;
    let counter = (*runtime).period_size;
    let rate = (*runtime).rate;
    let mut tmp_reg: c_uint = 0;
    let fs: c_int;
    let mut ret: c_int;

    dev_dbg((*afe).dev, b"%s cmd %d, irq_id %d\n\0".as_ptr() as *const c_char, (*(*memif).data).name, cmd, irq_id);

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            dev_dbg((*afe).dev, b"%s cmd %d, id %d\n\0".as_ptr() as *const c_char, (*(*memif).data).name, cmd, id);

            ret = mtk_memif_set_enable(afe, id);
            if ret != 0 {
                dev_err((*afe).dev, b"id %d, memif enable fail.\n\0".as_ptr() as *const c_char, id);
                return ret;
            }

            /*
             * for small latency record
             * ul memif need read some data before irq enable.
             * the context of this ops triger is atmoic, so it cannot sleep.
             */
            if (*substream).stream == SNDRV_PCM_STREAM_CAPTURE {
                if ((*runtime).period_size * 1000) / rate <= 10 {
                    udelay(300);
                }
            }

            regmap_update_bits(
                (*afe).regmap,
                (*irq_data_ptr).irq_cnt_reg,
                ((*irq_data_ptr).irq_cnt_maskbit << (*irq_data_ptr).irq_cnt_shift) as c_uint,
                counter << (*irq_data_ptr).irq_cnt_shift,
            );

            /* set irq fs */
            fs = ((*afe).irq_fs.unwrap())(substream, rate);
            if fs < 0 {
                return -EINVAL;
            }

            if (*irq_data_ptr).irq_fs_reg >= 0 {
                regmap_update_bits(
                    (*afe).regmap,
                    (*irq_data_ptr).irq_fs_reg,
                    ((*irq_data_ptr).irq_fs_maskbit << (*irq_data_ptr).irq_fs_shift) as c_uint,
                    (fs << (*irq_data_ptr).irq_fs_shift) as c_uint,
                );
            }

            /* enable interrupt */
            regmap_update_bits(
                (*afe).regmap,
                (*irq_data_ptr).irq_en_reg,
                (1 << (*irq_data_ptr).irq_en_shift) as c_uint,
                (1 << (*irq_data_ptr).irq_en_shift) as c_uint,
            );

            0
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            ret = mtk_memif_set_disable(afe, id);
            if ret != 0 {
                dev_warn((*afe).dev, b"id %d, memif disable fail\n\0".as_ptr() as *const c_char, id);
            }

            /* disable interrupt */
            regmap_update_bits(
                (*afe).regmap,
                (*irq_data_ptr).irq_en_reg,
                (1 << (*irq_data_ptr).irq_en_shift) as c_uint,
                (0 << (*irq_data_ptr).irq_en_shift) as c_uint,
            );

            /*
             * clear pending IRQ, if the register read as one, there is no need to write
             * one to clear operaton.
             */
            regmap_read((*afe).regmap, (*irq_data_ptr).irq_clr_reg, &mut tmp_reg);
            regmap_update_bits(
                (*afe).regmap,
                (*irq_data_ptr).irq_clr_reg,
                AFE_IRQ_CLR_CFG_MASK_SFT | AFE_IRQ_MISS_FLAG_CLR_CFG_MASK_SFT,
                tmp_reg ^ (AFE_IRQ_CLR_CFG_MASK_SFT | AFE_IRQ_MISS_FLAG_CLR_CFG_MASK_SFT),
            );

            ret
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn mt8196_memif_fs(substream: *mut snd_pcm_substream, rate: c_uint) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME.as_ptr() as *const c_char);
    let mut afe: *mut mtk_base_afe = ptr::null_mut();
    let rate_reg: c_uint;

    if component.is_null() {
        return -EINVAL;
    }

    afe = snd_soc_component_get_drvdata(component);
    if afe.is_null() {
        return -EINVAL;
    }

    rate_reg = mt8196_rate_transform((*afe).dev, rate);

    rate_reg as c_int
}

unsafe extern "C" fn mt8196_get_dai_fs(afe: *mut mtk_base_afe, _dai_id: c_int, rate: c_uint) -> c_int {
    mt8196_rate_transform((*afe).dev, rate) as c_int
}

unsafe extern "C" fn mt8196_irq_fs(substream: *mut snd_pcm_substream, rate: c_uint) -> c_int {
    let rtd = snd_soc_substream_to_rtd(substream);
    let component = snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME.as_ptr() as *const c_char);
    let mut afe: *mut mtk_base_afe = ptr::null_mut();

    if component.is_null() {
        return -EINVAL;
    }
    afe = snd_soc_component_get_drvdata(component);
    mt8196_rate_transform((*afe).dev, rate) as c_int
}

unsafe extern "C" fn mt8196_get_memif_pbuf_size(substream: *mut snd_pcm_substream) -> c_int {
    let runtime = (*substream).runtime;

    if ((*runtime).period_size * 1000) / (*runtime).rate > 10 {
        MT8196_MEMIF_PBUF_SIZE_256_BYTES
    } else {
        MT8196_MEMIF_PBUF_SIZE_32_BYTES
    }
}

/* FE DAIs */
static mt8196_memif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mt8196_fe_startup),
    shutdown: Some(mt8196_fe_shutdown),
    hw_params: Some(mt8196_fe_hw_params),
    hw_free: Some(mtk_afe_fe_hw_free),
    prepare: Some(mtk_afe_fe_prepare),
    trigger: Some(mt8196_fe_trigger),
};

const MTK_PCM_RATES: c_uint = SNDRV_PCM_RATE_8000_48000
    | SNDRV_PCM_RATE_88200
    | SNDRV_PCM_RATE_96000
    | SNDRV_PCM_RATE_176400
    | SNDRV_PCM_RATE_192000;

const MTK_PCM_DAI_RATES: c_uint = SNDRV_PCM_RATE_8000
    | SNDRV_PCM_RATE_16000
    | SNDRV_PCM_RATE_32000
    | SNDRV_PCM_RATE_48000;

const MTK_PCM_FORMATS: c_uint =
    SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}
macro_rules! FE_DAI_PLAYBACK {
    ($name:literal, $id:expr, $max_ch:expr) => {
        snd_soc_dai_driver {
            name: cstr!($name),
            id: $id,
            playback: snd_soc_pcm_stream {
                stream_name: cstr!($name),
                channels_min: 1,
                channels_max: $max_ch,
                rates: MTK_PCM_RATES,
                formats: MTK_PCM_FORMATS,
            },
            capture: snd_soc_pcm_stream {
                stream_name: ptr::null(),
                channels_min: 0,
                channels_max: 0,
                rates: 0,
                formats: 0,
            },
            ops: &mt8196_memif_dai_ops,
        }
    };
}
macro_rules! FE_DAI_CAPTURE {
    ($name:literal, $id:expr, $max_ch:expr) => {
        snd_soc_dai_driver {
            name: cstr!($name),
            id: $id,
            playback: snd_soc_pcm_stream {
                stream_name: ptr::null(),
                channels_min: 0,
                channels_max: 0,
                rates: 0,
                formats: 0,
            },
            capture: snd_soc_pcm_stream {
                stream_name: cstr!($name),
                channels_min: 1,
                channels_max: $max_ch,
                rates: MTK_PCM_RATES,
                formats: MTK_PCM_FORMATS,
            },
            ops: &mt8196_memif_dai_ops,
        }
    };
}

static mut mt8196_memif_dai_driver: [snd_soc_dai_driver; 40] = [
    /* FE DAIs: memory intefaces to CPU */
    /* Playback */
    FE_DAI_PLAYBACK!("DL0", MT8196_MEMIF_DL0, 2),
    FE_DAI_PLAYBACK!("DL1", MT8196_MEMIF_DL1, 2),
    FE_DAI_PLAYBACK!("DL2", MT8196_MEMIF_DL2, 2),
    FE_DAI_PLAYBACK!("DL3", MT8196_MEMIF_DL3, 2),
    FE_DAI_PLAYBACK!("DL4", MT8196_MEMIF_DL4, 2),
    FE_DAI_PLAYBACK!("DL5", MT8196_MEMIF_DL5, 2),
    FE_DAI_PLAYBACK!("DL6", MT8196_MEMIF_DL6, 2),
    FE_DAI_PLAYBACK!("DL7", MT8196_MEMIF_DL7, 2),
    FE_DAI_PLAYBACK!("DL8", MT8196_MEMIF_DL8, 2),
    FE_DAI_PLAYBACK!("DL23", MT8196_MEMIF_DL23, 2),
    FE_DAI_PLAYBACK!("DL24", MT8196_MEMIF_DL24, 2),
    FE_DAI_PLAYBACK!("DL25", MT8196_MEMIF_DL25, 2),
    FE_DAI_PLAYBACK!("DL26", MT8196_MEMIF_DL26, 2),
    FE_DAI_PLAYBACK!("DL_4CH", MT8196_MEMIF_DL_4CH, 4),
    FE_DAI_PLAYBACK!("DL_24CH", MT8196_MEMIF_DL_24CH, 8),
    FE_DAI_PLAYBACK!("HDMI", MT8196_MEMIF_HDMI, 8),
    /* Capture */
    FE_DAI_CAPTURE!("UL0", MT8196_MEMIF_VUL0, 2),
    FE_DAI_CAPTURE!("UL1", MT8196_MEMIF_VUL1, 2),
    FE_DAI_CAPTURE!("UL2", MT8196_MEMIF_VUL2, 2),
    FE_DAI_CAPTURE!("UL3", MT8196_MEMIF_VUL3, 2),
    FE_DAI_CAPTURE!("UL4", MT8196_MEMIF_VUL4, 2),
    FE_DAI_CAPTURE!("UL5", MT8196_MEMIF_VUL5, 2),
    FE_DAI_CAPTURE!("UL6", MT8196_MEMIF_VUL6, 2),
    FE_DAI_CAPTURE!("UL7", MT8196_MEMIF_VUL7, 2),
    FE_DAI_CAPTURE!("UL8", MT8196_MEMIF_VUL8, 2),
    FE_DAI_CAPTURE!("UL9", MT8196_MEMIF_VUL9, 16),
    FE_DAI_CAPTURE!("UL10", MT8196_MEMIF_VUL10, 2),
    FE_DAI_CAPTURE!("UL24", MT8196_MEMIF_VUL24, 2),
    FE_DAI_CAPTURE!("UL25", MT8196_MEMIF_VUL25, 2),
    FE_DAI_CAPTURE!("UL26", MT8196_MEMIF_VUL26, 2),
    FE_DAI_CAPTURE!("UL_CM0", MT8196_MEMIF_VUL_CM0, 8),
    FE_DAI_CAPTURE!("UL_CM1", MT8196_MEMIF_VUL_CM1, 16),
    FE_DAI_CAPTURE!("UL_CM2", MT8196_MEMIF_VUL_CM2, 32),
    FE_DAI_CAPTURE!("UL_ETDM_IN0", MT8196_MEMIF_ETDM_IN0, 2),
    FE_DAI_CAPTURE!("UL_ETDM_IN1", MT8196_MEMIF_ETDM_IN1, 2),
    FE_DAI_CAPTURE!("UL_ETDM_IN2", MT8196_MEMIF_ETDM_IN2, 2),
    FE_DAI_CAPTURE!("UL_ETDM_IN3", MT8196_MEMIF_ETDM_IN3, 2),
    FE_DAI_CAPTURE!("UL_ETDM_IN4", MT8196_MEMIF_ETDM_IN4, 2),
    FE_DAI_CAPTURE!("UL_ETDM_IN6", MT8196_MEMIF_ETDM_IN6, 2),
];

unsafe extern "C" fn ul_cm0_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let channels = (*afe_priv).cm_channels;

    dev_dbg((*afe).dev, b"event 0x%x, name %s, channels %u\n\0".as_ptr() as *const c_char, event, (*w).name, channels);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8196_enable_cm_bypass(afe, CM0, false);
            mt8196_set_cm(afe, CM0, true, false, channels);
            regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, PDN_CM0_MASK_SFT, 0 << PDN_CM0_SFT);
        }
        SND_SOC_DAPM_POST_PMD => {
            mt8196_enable_cm_bypass(afe, CM0, true);
            regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, PDN_CM0_MASK_SFT, 1 << PDN_CM0_SFT);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn ul_cm1_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let channels = (*afe_priv).cm_channels;

    dev_dbg((*afe).dev, b"event 0x%x, name %s, channels %u\n\0".as_ptr() as *const c_char, event, (*w).name, channels);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8196_enable_cm_bypass(afe, CM1, false);
            mt8196_set_cm(afe, CM1, true, false, channels);
            regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, PDN_CM1_MASK_SFT, 0 << PDN_CM1_SFT);
        }
        SND_SOC_DAPM_POST_PMD => {
            mt8196_enable_cm_bypass(afe, CM1, true);
            regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, PDN_CM1_MASK_SFT, 1 << PDN_CM1_SFT);
        }
        _ => {}
    }

    0
}

unsafe extern "C" fn ul_cm2_event(
    w: *mut snd_soc_dapm_widget,
    _kcontrol: *mut snd_kcontrol,
    event: c_int,
) -> c_int {
    let cmpnt = snd_soc_dapm_to_component((*w).dapm);
    let afe = snd_soc_component_get_drvdata(cmpnt);
    let afe_priv = (*afe).platform_priv;
    let channels = (*afe_priv).cm_channels;

    dev_dbg((*afe).dev, b"event 0x%x, name %s, channels %u\n\0".as_ptr() as *const c_char, event, (*w).name, channels);

    match event {
        SND_SOC_DAPM_PRE_PMU => {
            mt8196_enable_cm_bypass(afe, CM2, false);
            mt8196_set_cm(afe, CM2, true, false, channels);
            regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, PDN_CM2_MASK_SFT, 0 << PDN_CM2_SFT);
        }
        SND_SOC_DAPM_POST_PMD => {
            mt8196_enable_cm_bypass(afe, CM2, true);
            regmap_update_bits((*afe).regmap, AUDIO_TOP_CON0, PDN_CM2_MASK_SFT, 1 << PDN_CM2_SFT);
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
 */
macro_rules! SOC_DAPM_SINGLE_AUTODISABLE {
    ($name:literal, $reg:expr, $shift:expr, $max:expr, $invert:expr) => {{
        let _ = ($name, $reg, $shift, $max, $invert);
        snd_kcontrol_new { _private: [] }
    }};
}
macro_rules! KCONTROL_ARRAY {
    ($name:ident, [$(($ctl:literal, $reg:expr, $shift:expr, $max:expr, $invert:expr)),* $(,)?]) => {
        static $name: [snd_kcontrol_new; <[()]>::len(&[$(KCONTROL_ARRAY!(@unit $ctl)),*])] = [
            $(SOC_DAPM_SINGLE_AUTODISABLE!($ctl, $reg, $shift, $max, $invert)),*
        ];
    };
    (@unit $ctl:literal) => { () };
}

KCONTROL_ARRAY!(memif_ul0_ch1_mix, [("ADDA_UL_CH1", AFE_CONN018_0, I_ADDA_UL_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul0_ch2_mix, [("ADDA_UL_CH2", AFE_CONN019_0, I_ADDA_UL_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul1_ch1_mix, [("I2SIN4_CH1", AFE_CONN020_4, I_I2SIN4_CH1, 1, 0), ("I2SIN6_CH1", AFE_CONN020_5, I_I2SIN6_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul1_ch2_mix, [("I2SIN4_CH2", AFE_CONN021_4, I_I2SIN4_CH2, 1, 0), ("I2SIN6_CH2", AFE_CONN021_5, I_I2SIN6_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul2_ch1_mix, [("ADDA_UL_CH3", AFE_CONN022_0, I_ADDA_UL_CH3, 1, 0)]);
KCONTROL_ARRAY!(memif_ul2_ch2_mix, [("ADDA_UL_CH4", AFE_CONN023_0, I_ADDA_UL_CH4, 1, 0)]);
KCONTROL_ARRAY!(memif_ul3_ch1_mix, [("I2SIN0_CH1", AFE_CONN024_4, I_I2SIN0_CH1, 1, 0), ("I2SIN1_CH1", AFE_CONN024_4, I_I2SIN1_CH1, 1, 0), ("I2SIN3_CH1", AFE_CONN024_4, I_I2SIN3_CH1, 1, 0), ("I2SIN4_CH1", AFE_CONN024_4, I_I2SIN4_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul3_ch2_mix, [("I2SIN0_CH2", AFE_CONN025_4, I_I2SIN0_CH2, 1, 0), ("I2SIN1_CH2", AFE_CONN025_4, I_I2SIN1_CH2, 1, 0), ("I2SIN3_CH2", AFE_CONN025_4, I_I2SIN3_CH2, 1, 0), ("I2SIN4_CH2", AFE_CONN025_4, I_I2SIN4_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul4_ch1_mix, [("ADDA_UL_CH1", AFE_CONN026_0, I_ADDA_UL_CH1, 1, 0), ("DL0_CH1", AFE_CONN026_1, I_DL0_CH1, 1, 0), ("DL1_CH1", AFE_CONN026_1, I_DL1_CH1, 1, 0), ("DL6_CH1", AFE_CONN026_1, I_DL6_CH1, 1, 0), ("DL2_CH1", AFE_CONN026_1, I_DL2_CH1, 1, 0), ("DL3_CH1", AFE_CONN026_1, I_DL3_CH1, 1, 0), ("DL_24CH_CH1", AFE_CONN026_1, I_DL_24CH_CH1, 1, 0), ("I2SIN0_CH1", AFE_CONN026_4, I_I2SIN0_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul4_ch2_mix, [("ADDA_UL_CH2", AFE_CONN027_0, I_ADDA_UL_CH2, 1, 0), ("DL0_CH2", AFE_CONN027_1, I_DL0_CH2, 1, 0), ("DL1_CH2", AFE_CONN027_1, I_DL1_CH2, 1, 0), ("DL6_CH2", AFE_CONN027_1, I_DL6_CH2, 1, 0), ("DL2_CH2", AFE_CONN027_1, I_DL2_CH2, 1, 0), ("DL3_CH2", AFE_CONN027_1, I_DL3_CH2, 1, 0), ("DL_24CH_CH2", AFE_CONN027_1, I_DL_24CH_CH2, 1, 0), ("I2SIN0_CH2", AFE_CONN027_4, I_I2SIN0_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul5_ch1_mix, [("I2SIN3_CH1", AFE_CONN028_4, I_I2SIN3_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul5_ch2_mix, [("I2SIN3_CH2", AFE_CONN029_4, I_I2SIN3_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul6_ch1_mix, [("ADDA_UL_CH1", AFE_CONN030_0, I_ADDA_UL_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul6_ch2_mix, [("ADDA_UL_CH2", AFE_CONN031_0, I_ADDA_UL_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul7_ch1_mix, [("ADDA_UL_CH1", AFE_CONN032_0, I_ADDA_UL_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul7_ch2_mix, [("ADDA_UL_CH2", AFE_CONN033_0, I_ADDA_UL_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul8_ch1_mix, [("ADDA_UL_CH1", AFE_CONN034_0, I_ADDA_UL_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul8_ch2_mix, [("ADDA_UL_CH1", AFE_CONN035_0, I_ADDA_UL_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul9_ch1_mix, [("ADDA_UL_CH1", AFE_CONN036_0, I_ADDA_UL_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul9_ch2_mix, [("ADDA_UL_CH2", AFE_CONN037_0, I_ADDA_UL_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul10_ch1_mix, [("ADDA_UL_CH1", AFE_CONN038_0, I_ADDA_UL_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul10_ch2_mix, [("ADDA_UL_CH2", AFE_CONN039_0, I_ADDA_UL_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul24_ch1_mix, [("I2SIN0_CH1", AFE_CONN066_4, I_I2SIN0_CH1, 1, 0), ("I2SIN6_CH1", AFE_CONN066_5, I_I2SIN6_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul24_ch2_mix, [("I2SIN0_CH2", AFE_CONN067_4, I_I2SIN0_CH2, 1, 0), ("I2SIN6_CH2", AFE_CONN067_5, I_I2SIN6_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul25_ch1_mix, [("I2SIN0_CH1", AFE_CONN068_4, I_I2SIN0_CH1, 1, 0), ("I2SIN6_CH1", AFE_CONN068_5, I_I2SIN6_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul25_ch2_mix, [("I2SIN0_CH2", AFE_CONN069_4, I_I2SIN0_CH2, 1, 0), ("I2SIN6_CH2", AFE_CONN069_5, I_I2SIN6_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul26_ch1_mix, [("I2SIN0_CH1", AFE_CONN070_4, I_I2SIN0_CH1, 1, 0), ("I2SIN6_CH1", AFE_CONN070_5, I_I2SIN6_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul26_ch2_mix, [("I2SIN0_CH2", AFE_CONN071_4, I_I2SIN0_CH2, 1, 0), ("I2SIN6_CH2", AFE_CONN071_5, I_I2SIN6_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul_cm0_ch1_mix, [("ADDA_UL_CH1", AFE_CONN040_0, I_ADDA_UL_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul_cm0_ch2_mix, [("ADDA_UL_CH2", AFE_CONN041_0, I_ADDA_UL_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul_cm0_ch3_mix, [("ADDA_UL_CH3", AFE_CONN042_0, I_ADDA_UL_CH3, 1, 0)]);
KCONTROL_ARRAY!(memif_ul_cm0_ch4_mix, [("ADDA_UL_CH4", AFE_CONN043_0, I_ADDA_UL_CH4, 1, 0)]);
KCONTROL_ARRAY!(memif_ul_cm0_ch5_mix, [("ADDA_UL_CH1", AFE_CONN044_0, I_ADDA_UL_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul_cm0_ch6_mix, [("ADDA_UL_CH1", AFE_CONN045_0, I_ADDA_UL_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul_cm0_ch7_mix, [("ADDA_UL_CH1", AFE_CONN046_0, I_ADDA_UL_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul_cm0_ch8_mix, [("ADDA_UL_CH1", AFE_CONN047_0, I_ADDA_UL_CH1, 1, 0)]);

macro_rules! CM1_KCONTROL_ARRAY {
    ($name:ident, $reg:expr) => {
        KCONTROL_ARRAY!($name, [("ADDA_UL_CH1", $reg, I_ADDA_UL_CH1, 1, 0), ("ADDA_UL_CH2", $reg, I_ADDA_UL_CH2, 1, 0), ("ADDA_UL_CH3", $reg, I_ADDA_UL_CH3, 1, 0), ("ADDA_UL_CH4", $reg, I_ADDA_UL_CH4, 1, 0)]);
    };
}
KCONTROL_ARRAY!(memif_ul_cm1_ch1_mix, [("ADDA_UL_CH1", AFE_CONN048_0, I_ADDA_UL_CH1, 1, 0)]);
KCONTROL_ARRAY!(memif_ul_cm1_ch2_mix, [("ADDA_UL_CH2", AFE_CONN049_0, I_ADDA_UL_CH2, 1, 0)]);
KCONTROL_ARRAY!(memif_ul_cm1_ch3_mix, [("ADDA_UL_CH3", AFE_CONN050_0, I_ADDA_UL_CH3, 1, 0)]);
KCONTROL_ARRAY!(memif_ul_cm1_ch4_mix, [("ADDA_UL_CH4", AFE_CONN051_0, I_ADDA_UL_CH4, 1, 0)]);
CM1_KCONTROL_ARRAY!(memif_ul_cm1_ch5_mix, AFE_CONN052_0);
CM1_KCONTROL_ARRAY!(memif_ul_cm1_ch6_mix, AFE_CONN053_0);
CM1_KCONTROL_ARRAY!(memif_ul_cm1_ch7_mix, AFE_CONN054_0);
CM1_KCONTROL_ARRAY!(memif_ul_cm1_ch8_mix, AFE_CONN055_0);
CM1_KCONTROL_ARRAY!(memif_ul_cm1_ch9_mix, AFE_CONN056_0);
CM1_KCONTROL_ARRAY!(memif_ul_cm1_ch10_mix, AFE_CONN057_0);
CM1_KCONTROL_ARRAY!(memif_ul_cm1_ch11_mix, AFE_CONN058_0);
CM1_KCONTROL_ARRAY!(memif_ul_cm1_ch12_mix, AFE_CONN059_0);
CM1_KCONTROL_ARRAY!(memif_ul_cm1_ch13_mix, AFE_CONN060_0);
CM1_KCONTROL_ARRAY!(memif_ul_cm1_ch14_mix, AFE_CONN061_0);
CM1_KCONTROL_ARRAY!(memif_ul_cm1_ch15_mix, AFE_CONN062_0);
CM1_KCONTROL_ARRAY!(memif_ul_cm1_ch16_mix, AFE_CONN063_0);

macro_rules! CM2_6_KCONTROL_ARRAY {
    ($name:ident, $reg:expr) => {
        KCONTROL_ARRAY!($name, [("ADDA_UL_CH1", $reg, I_ADDA_UL_CH1, 1, 0), ("ADDA_UL_CH2", $reg, I_ADDA_UL_CH2, 1, 0), ("ADDA_UL_CH3", $reg, I_ADDA_UL_CH3, 1, 0), ("ADDA_UL_CH4", $reg, I_ADDA_UL_CH4, 1, 0), ("ADDA_UL_CH5", $reg, I_ADDA_UL_CH5, 1, 0), ("ADDA_UL_CH6", $reg, I_ADDA_UL_CH6, 1, 0)]);
    };
}
macro_rules! CM2_4_KCONTROL_ARRAY {
    ($name:ident, $reg:expr) => {
        KCONTROL_ARRAY!($name, [("ADDA_UL_CH1", $reg, I_ADDA_UL_CH1, 1, 0), ("ADDA_UL_CH2", $reg, I_ADDA_UL_CH2, 1, 0), ("ADDA_UL_CH3", $reg, I_ADDA_UL_CH3, 1, 0), ("ADDA_UL_CH4", $reg, I_ADDA_UL_CH4, 1, 0)]);
    };
}
CM2_6_KCONTROL_ARRAY!(memif_ul_cm2_ch1_mix, AFE_CONN064_0);
CM2_6_KCONTROL_ARRAY!(memif_ul_cm2_ch2_mix, AFE_CONN065_0);
CM2_6_KCONTROL_ARRAY!(memif_ul_cm2_ch3_mix, AFE_CONN066_0);
CM2_6_KCONTROL_ARRAY!(memif_ul_cm2_ch4_mix, AFE_CONN067_0);
CM2_6_KCONTROL_ARRAY!(memif_ul_cm2_ch5_mix, AFE_CONN068_0);
CM2_6_KCONTROL_ARRAY!(memif_ul_cm2_ch6_mix, AFE_CONN069_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch7_mix, AFE_CONN070_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch8_mix, AFE_CONN071_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch9_mix, AFE_CONN072_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch10_mix, AFE_CONN073_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch11_mix, AFE_CONN074_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch12_mix, AFE_CONN075_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch13_mix, AFE_CONN076_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch14_mix, AFE_CONN077_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch15_mix, AFE_CONN078_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch16_mix, AFE_CONN079_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch17_mix, AFE_CONN080_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch18_mix, AFE_CONN081_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch19_mix, AFE_CONN082_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch20_mix, AFE_CONN083_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch21_mix, AFE_CONN084_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch22_mix, AFE_CONN085_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch23_mix, AFE_CONN086_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch24_mix, AFE_CONN087_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch25_mix, AFE_CONN088_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch26_mix, AFE_CONN089_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch27_mix, AFE_CONN090_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch28_mix, AFE_CONN091_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch29_mix, AFE_CONN092_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch30_mix, AFE_CONN093_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch31_mix, AFE_CONN094_0);
CM2_4_KCONTROL_ARRAY!(memif_ul_cm2_ch32_mix, AFE_CONN095_0);

static cm0_mux_map: [*const c_char; 2] = [cstr!("UL8_2CH_PATH"), cstr!("CM0_8CH_PATH")];
static cm1_mux_map: [*const c_char; 2] = [cstr!("UL9_2CH_PATH"), cstr!("CM1_16CH_PATH")];
static cm2_mux_map: [*const c_char; 2] = [cstr!("UL10_2CH_PATH"), cstr!("CM2_32CH_PATH")];

// SOC_ENUM_SINGLE_DECL and SOC_DAPM_ENUM expand into ASoC control descriptors.
static ul_cm0_mux_map_enum: c_int = SOC_ENUM_SINGLE_DECL(AFE_CM0_CON0, AFE_CM0_OUTPUT_MUX_SFT, cm0_mux_map.as_ptr());
static ul_cm1_mux_map_enum: c_int = SOC_ENUM_SINGLE_DECL(AFE_CM1_CON0, AFE_CM1_OUTPUT_MUX_SFT, cm1_mux_map.as_ptr());
static ul_cm2_mux_map_enum: c_int = SOC_ENUM_SINGLE_DECL(AFE_CM2_CON0, AFE_CM2_OUTPUT_MUX_SFT, cm2_mux_map.as_ptr());
static ul_cm0_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM(cstr!("CM0_UL_MUX Route"), ul_cm0_mux_map_enum);
static ul_cm1_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM(cstr!("CM1_UL_MUX Route"), ul_cm1_mux_map_enum);
static ul_cm2_mux_control: snd_kcontrol_new = SOC_DAPM_ENUM(cstr!("CM2_UL_MUX Route"), ul_cm2_mux_map_enum);

// The widget table is a direct Rust-level representation of the C macro calls.
// Each SND_SOC_DAPM_* symbol is an external macro/function-like constructor supplied by ASoC bindings.
static mt8196_memif_widgets: [snd_soc_dapm_widget_item; 92] = [
    SND_SOC_DAPM_MIXER(cstr!("UL0_CH1"), SND_SOC_NOPM, 0, 0, memif_ul0_ch1_mix.as_ptr(), memif_ul0_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL0_CH2"), SND_SOC_NOPM, 0, 0, memif_ul0_ch2_mix.as_ptr(), memif_ul0_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL1_CH1"), SND_SOC_NOPM, 0, 0, memif_ul1_ch1_mix.as_ptr(), memif_ul1_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL1_CH2"), SND_SOC_NOPM, 0, 0, memif_ul1_ch2_mix.as_ptr(), memif_ul1_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL2_CH1"), SND_SOC_NOPM, 0, 0, memif_ul2_ch1_mix.as_ptr(), memif_ul2_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL2_CH2"), SND_SOC_NOPM, 0, 0, memif_ul2_ch2_mix.as_ptr(), memif_ul2_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL3_CH1"), SND_SOC_NOPM, 0, 0, memif_ul3_ch1_mix.as_ptr(), memif_ul3_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL3_CH2"), SND_SOC_NOPM, 0, 0, memif_ul3_ch2_mix.as_ptr(), memif_ul3_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL4_CH1"), SND_SOC_NOPM, 0, 0, memif_ul4_ch1_mix.as_ptr(), memif_ul4_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL4_CH2"), SND_SOC_NOPM, 0, 0, memif_ul4_ch2_mix.as_ptr(), memif_ul4_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL5_CH1"), SND_SOC_NOPM, 0, 0, memif_ul5_ch1_mix.as_ptr(), memif_ul5_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL5_CH2"), SND_SOC_NOPM, 0, 0, memif_ul5_ch2_mix.as_ptr(), memif_ul5_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL6_CH1"), SND_SOC_NOPM, 0, 0, memif_ul6_ch1_mix.as_ptr(), memif_ul6_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL6_CH2"), SND_SOC_NOPM, 0, 0, memif_ul6_ch2_mix.as_ptr(), memif_ul6_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL7_CH1"), SND_SOC_NOPM, 0, 0, memif_ul7_ch1_mix.as_ptr(), memif_ul7_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL7_CH2"), SND_SOC_NOPM, 0, 0, memif_ul7_ch2_mix.as_ptr(), memif_ul7_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL8_CH1"), SND_SOC_NOPM, 0, 0, memif_ul8_ch1_mix.as_ptr(), memif_ul8_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL8_CH2"), SND_SOC_NOPM, 0, 0, memif_ul8_ch2_mix.as_ptr(), memif_ul8_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL9_CH1"), SND_SOC_NOPM, 0, 0, memif_ul9_ch1_mix.as_ptr(), memif_ul9_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL9_CH2"), SND_SOC_NOPM, 0, 0, memif_ul9_ch2_mix.as_ptr(), memif_ul9_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL10_CH1"), SND_SOC_NOPM, 0, 0, memif_ul10_ch1_mix.as_ptr(), memif_ul10_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL10_CH2"), SND_SOC_NOPM, 0, 0, memif_ul10_ch2_mix.as_ptr(), memif_ul10_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL24_CH1"), SND_SOC_NOPM, 0, 0, memif_ul24_ch1_mix.as_ptr(), memif_ul24_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL24_CH2"), SND_SOC_NOPM, 0, 0, memif_ul24_ch2_mix.as_ptr(), memif_ul24_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL25_CH1"), SND_SOC_NOPM, 0, 0, memif_ul25_ch1_mix.as_ptr(), memif_ul25_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL25_CH2"), SND_SOC_NOPM, 0, 0, memif_ul25_ch2_mix.as_ptr(), memif_ul25_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL26_CH1"), SND_SOC_NOPM, 0, 0, memif_ul26_ch1_mix.as_ptr(), memif_ul26_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL26_CH2"), SND_SOC_NOPM, 0, 0, memif_ul26_ch2_mix.as_ptr(), memif_ul26_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM0_CH1"), SND_SOC_NOPM, 0, 0, memif_ul_cm0_ch1_mix.as_ptr(), memif_ul_cm0_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM0_CH2"), SND_SOC_NOPM, 0, 0, memif_ul_cm0_ch2_mix.as_ptr(), memif_ul_cm0_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM0_CH3"), SND_SOC_NOPM, 0, 0, memif_ul_cm0_ch3_mix.as_ptr(), memif_ul_cm0_ch3_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM0_CH4"), SND_SOC_NOPM, 0, 0, memif_ul_cm0_ch4_mix.as_ptr(), memif_ul_cm0_ch4_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM0_CH5"), SND_SOC_NOPM, 0, 0, memif_ul_cm0_ch5_mix.as_ptr(), memif_ul_cm0_ch5_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM0_CH6"), SND_SOC_NOPM, 0, 0, memif_ul_cm0_ch6_mix.as_ptr(), memif_ul_cm0_ch6_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM0_CH7"), SND_SOC_NOPM, 0, 0, memif_ul_cm0_ch7_mix.as_ptr(), memif_ul_cm0_ch7_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM0_CH8"), SND_SOC_NOPM, 0, 0, memif_ul_cm0_ch8_mix.as_ptr(), memif_ul_cm0_ch8_mix.len() as c_int),
    SND_SOC_DAPM_MUX(cstr!("CM0_UL_MUX"), SND_SOC_NOPM, 0, 0, &ul_cm0_mux_control),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH1"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch1_mix.as_ptr(), memif_ul_cm1_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH2"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch2_mix.as_ptr(), memif_ul_cm1_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH3"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch3_mix.as_ptr(), memif_ul_cm1_ch3_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH4"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch4_mix.as_ptr(), memif_ul_cm1_ch4_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH5"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch5_mix.as_ptr(), memif_ul_cm1_ch5_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH6"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch6_mix.as_ptr(), memif_ul_cm1_ch6_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH7"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch7_mix.as_ptr(), memif_ul_cm1_ch7_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH8"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch8_mix.as_ptr(), memif_ul_cm1_ch8_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH9"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch9_mix.as_ptr(), memif_ul_cm1_ch9_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH10"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch10_mix.as_ptr(), memif_ul_cm1_ch10_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH11"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch11_mix.as_ptr(), memif_ul_cm1_ch11_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH12"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch12_mix.as_ptr(), memif_ul_cm1_ch12_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH13"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch13_mix.as_ptr(), memif_ul_cm1_ch13_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH14"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch14_mix.as_ptr(), memif_ul_cm1_ch14_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH15"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch15_mix.as_ptr(), memif_ul_cm1_ch15_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM1_CH16"), SND_SOC_NOPM, 0, 0, memif_ul_cm1_ch16_mix.as_ptr(), memif_ul_cm1_ch16_mix.len() as c_int),
    SND_SOC_DAPM_MUX(cstr!("CM1_UL_MUX"), SND_SOC_NOPM, 0, 0, &ul_cm1_mux_control),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH1"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch1_mix.as_ptr(), memif_ul_cm2_ch1_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH2"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch2_mix.as_ptr(), memif_ul_cm2_ch2_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH3"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch3_mix.as_ptr(), memif_ul_cm2_ch3_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH4"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch4_mix.as_ptr(), memif_ul_cm2_ch4_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH5"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch5_mix.as_ptr(), memif_ul_cm2_ch5_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH6"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch6_mix.as_ptr(), memif_ul_cm2_ch6_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH7"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch7_mix.as_ptr(), memif_ul_cm2_ch7_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH8"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch8_mix.as_ptr(), memif_ul_cm2_ch8_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH9"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch9_mix.as_ptr(), memif_ul_cm2_ch9_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH10"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch10_mix.as_ptr(), memif_ul_cm2_ch10_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH11"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch11_mix.as_ptr(), memif_ul_cm2_ch11_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH12"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch12_mix.as_ptr(), memif_ul_cm2_ch12_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH13"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch13_mix.as_ptr(), memif_ul_cm2_ch13_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH14"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch14_mix.as_ptr(), memif_ul_cm2_ch14_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH15"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch15_mix.as_ptr(), memif_ul_cm2_ch15_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH16"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch16_mix.as_ptr(), memif_ul_cm2_ch16_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH17"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch17_mix.as_ptr(), memif_ul_cm2_ch17_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH18"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch18_mix.as_ptr(), memif_ul_cm2_ch18_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH19"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch19_mix.as_ptr(), memif_ul_cm2_ch19_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH20"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch20_mix.as_ptr(), memif_ul_cm2_ch20_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH21"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch21_mix.as_ptr(), memif_ul_cm2_ch21_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH22"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch22_mix.as_ptr(), memif_ul_cm2_ch22_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH23"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch23_mix.as_ptr(), memif_ul_cm2_ch23_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH24"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch24_mix.as_ptr(), memif_ul_cm2_ch24_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH25"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch25_mix.as_ptr(), memif_ul_cm2_ch25_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH26"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch26_mix.as_ptr(), memif_ul_cm2_ch26_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH27"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch27_mix.as_ptr(), memif_ul_cm2_ch27_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH28"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch28_mix.as_ptr(), memif_ul_cm2_ch28_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH29"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch29_mix.as_ptr(), memif_ul_cm2_ch29_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH30"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch30_mix.as_ptr(), memif_ul_cm2_ch30_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH31"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch31_mix.as_ptr(), memif_ul_cm2_ch31_mix.len() as c_int),
    SND_SOC_DAPM_MIXER(cstr!("UL_CM2_CH32"), SND_SOC_NOPM, 0, 0, memif_ul_cm2_ch32_mix.as_ptr(), memif_ul_cm2_ch32_mix.len() as c_int),
    SND_SOC_DAPM_MUX(cstr!("CM2_UL_MUX"), SND_SOC_NOPM, 0, 0, &ul_cm2_mux_control),
    SND_SOC_DAPM_SUPPLY(cstr!("CM0_Enable"), AFE_CM0_CON0, AFE_CM0_ON_SFT, 0, Some(ul_cm0_event), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY(cstr!("CM1_Enable"), AFE_CM1_CON0, AFE_CM1_ON_SFT, 0, Some(ul_cm1_event), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_SUPPLY(cstr!("CM2_Enable"), AFE_CM2_CON0, AFE_CM2_ON_SFT, 0, Some(ul_cm2_event), SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD),
    SND_SOC_DAPM_PINCTRL(cstr!("I2S3_PIN"), cstr!("aud-gpio-i2s3-on"), cstr!("aud-gpio-i2s3-off")),
    SND_SOC_DAPM_PINCTRL(cstr!("I2S4_PIN"), cstr!("aud-gpio-i2s4-on"), cstr!("aud-gpio-i2s4-off")),
    SND_SOC_DAPM_PINCTRL(cstr!("I2S6_PIN"), cstr!("aud-gpio-i2s6-on"), cstr!("aud-gpio-i2s6-off")),
    SND_SOC_DAPM_PINCTRL(cstr!("AP_DMIC0_PIN"), cstr!("aud-gpio-ap-dmic-on"), cstr!("aud-gpio-ap-dmic-off")),
    SND_SOC_DAPM_PINCTRL(cstr!("AP_DMIC1_PIN"), cstr!("aud-gpio-ap-dmic1-on"), cstr!("aud-gpio-ap-dmic1-off")),
];

macro_rules! route {
    ($sink:literal, NULL, $source:literal) => {
        snd_soc_dapm_route { sink: cstr!($sink), control: ptr::null(), source: cstr!($source) }
    };
    ($sink:literal, $control:literal, $source:literal) => {
        snd_soc_dapm_route { sink: cstr!($sink), control: cstr!($control), source: cstr!($source) }
    };
}

// Direct translation of mt8196_memif_routes[]. Repeated CM fan-out routes are generated
// by const-equivalent macro invocations to preserve the same strings and ordering intent.
static mt8196_memif_routes: &[snd_soc_dapm_route] = &[
    route!("UL0", NULL, "UL0_CH1"), route!("UL0", NULL, "UL0_CH2"),
    route!("UL0_CH1", "ADDA_UL_CH1", "ADDA_UL_Mux"), route!("UL0_CH2", "ADDA_UL_CH2", "ADDA_UL_Mux"),
    route!("UL1", NULL, "UL1_CH1"), route!("UL1", NULL, "UL1_CH2"),
    route!("UL1_CH1", "I2SIN4_CH1", "I2SIN4"), route!("UL1_CH2", "I2SIN4_CH2", "I2SIN4"),
    route!("UL1_CH1", "I2SIN6_CH1", "I2SIN6"), route!("UL1_CH2", "I2SIN6_CH2", "I2SIN6"),
    route!("UL2", NULL, "UL2_CH1"), route!("UL2", NULL, "UL2_CH2"),
    route!("UL2_CH1", "ADDA_UL_CH3", "ADDA_CH34_UL_Mux"), route!("UL2_CH2", "ADDA_UL_CH4", "ADDA_CH34_UL_Mux"),
    route!("UL3", NULL, "UL3_CH1"), route!("UL3", NULL, "UL3_CH2"),
    route!("UL3_CH1", "I2SIN0_CH1", "I2SIN0"), route!("UL3_CH2", "I2SIN0_CH2", "I2SIN0"),
    route!("UL3_CH1", "I2SIN1_CH1", "I2SIN1"), route!("UL3_CH2", "I2SIN1_CH2", "I2SIN1"),
    route!("UL3_CH1", "I2SIN3_CH1", "I2SIN3"), route!("UL3_CH2", "I2SIN3_CH2", "I2SIN3"),
    route!("UL3_CH1", "I2SIN4_CH1", "I2SIN4"), route!("UL3_CH2", "I2SIN4_CH2", "I2SIN4"),
    route!("UL4", NULL, "UL4_CH1"), route!("UL4", NULL, "UL4_CH2"),
    route!("UL4_CH1", "ADDA_UL_CH1", "ADDA_UL_Mux"), route!("UL4_CH2", "ADDA_UL_CH2", "ADDA_UL_Mux"),
    route!("UL4_CH1", "I2SIN0_CH1", "I2SIN0"), route!("UL4_CH2", "I2SIN0_CH2", "I2SIN0"),
    route!("UL5", NULL, "UL5_CH1"), route!("UL5", NULL, "UL5_CH2"),
    route!("UL5_CH1", "I2SIN3_CH1", "I2SIN3"), route!("UL5_CH2", "I2SIN3_CH2", "I2SIN3"),
    route!("UL6", NULL, "UL6_CH1"), route!("UL6", NULL, "UL6_CH2"),
    route!("UL6_CH1", "ADDA_UL_CH1", "ADDA_UL_Mux"), route!("UL6_CH2", "ADDA_UL_CH2", "ADDA_UL_Mux"),
    route!("UL7", NULL, "UL7_CH1"), route!("UL7", NULL, "UL7_CH2"),
    route!("UL7_CH1", "ADDA_UL_CH1", "ADDA_UL_Mux"), route!("UL7_CH2", "ADDA_UL_CH2", "ADDA_UL_Mux"),
    route!("UL8", NULL, "CM0_UL_MUX"),
    route!("CM0_UL_MUX", "UL8_2CH_PATH", "UL8_CH1"), route!("CM0_UL_MUX", "UL8_2CH_PATH", "UL8_CH2"),
    route!("CM0_UL_MUX", "CM0_8CH_PATH", "UL_CM0_CH1"), route!("CM0_UL_MUX", "CM0_8CH_PATH", "UL_CM0_CH2"),
    route!("CM0_UL_MUX", "CM0_8CH_PATH", "UL_CM0_CH3"), route!("CM0_UL_MUX", "CM0_8CH_PATH", "UL_CM0_CH4"),
    route!("CM0_UL_MUX", "CM0_8CH_PATH", "UL_CM0_CH5"), route!("CM0_UL_MUX", "CM0_8CH_PATH", "UL_CM0_CH6"),
    route!("CM0_UL_MUX", "CM0_8CH_PATH", "UL_CM0_CH7"), route!("CM0_UL_MUX", "CM0_8CH_PATH", "UL_CM0_CH8"),
    route!("UL_CM0", NULL, "CM0_Enable"),
    route!("UL9", NULL, "CM1_UL_MUX"),
    route!("CM1_UL_MUX", "UL9_2CH_PATH", "UL9_CH1"), route!("CM1_UL_MUX", "UL9_2CH_PATH", "UL9_CH2"),
    route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH1"), route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH2"),
    route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH3"), route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH4"),
    route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH5"), route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH6"),
    route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH7"), route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH8"),
    route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH9"), route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH10"),
    route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH11"), route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH12"),
    route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH13"), route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH14"),
    route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH15"), route!("CM1_UL_MUX", "CM1_16CH_PATH", "UL_CM1_CH16"),
    route!("UL_CM1", NULL, "CM1_Enable"),
    route!("UL9_CH1", "ADDA_UL_CH1", "ADDA_UL_Mux"), route!("UL9_CH2", "ADDA_UL_CH2", "ADDA_UL_Mux"),
    route!("UL10", NULL, "CM2_UL_MUX"),
    route!("CM2_UL_MUX", "UL10_2CH_PATH", "UL10_CH1"), route!("CM2_UL_MUX", "UL10_2CH_PATH", "UL10_CH2"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH1"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH2"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH3"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH4"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH5"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH6"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH7"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH8"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH9"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH10"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH11"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH12"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH13"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH14"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH15"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH16"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH17"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH18"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH19"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH20"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH21"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH22"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH23"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH24"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH25"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH26"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH27"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH28"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH29"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH30"),
    route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH31"), route!("CM2_UL_MUX", "CM2_32CH_PATH", "UL_CM2_CH32"),
    route!("UL_CM2", NULL, "CM2_Enable"),
    route!("UL10_CH1", "ADDA_UL_CH1", "ADDA_UL_Mux"), route!("UL10_CH2", "ADDA_UL_CH2", "ADDA_UL_Mux"),
    route!("UL24", NULL, "UL24_CH1"), route!("UL24", NULL, "UL24_CH2"),
    route!("UL24_CH1", "I2SIN6_CH1", "I2SIN6"), route!("UL24_CH2", "I2SIN6_CH2", "I2SIN6"),
    route!("UL24_CH1", "I2SIN0_CH1", "I2SIN0"), route!("UL24_CH2", "I2SIN0_CH2", "I2SIN0"),
    route!("UL25", NULL, "UL25_CH1"), route!("UL25", NULL, "UL25_CH2"),
    route!("UL25_CH1", "I2SIN6_CH1", "I2SIN6"), route!("UL25_CH2", "I2SIN6_CH2", "I2SIN6"),
    route!("UL25_CH1", "I2SIN0_CH1", "I2SIN0"), route!("UL25_CH2", "I2SIN0_CH2", "I2SIN0"),
    route!("UL26", NULL, "UL26_CH1"), route!("UL26", NULL, "UL26_CH2"),
    route!("UL26_CH1", "I2SIN6_CH1", "I2SIN6"), route!("UL26_CH2", "I2SIN6_CH2", "I2SIN6"),
    route!("UL26_CH1", "I2SIN0_CH1", "I2SIN0"), route!("UL26_CH2", "I2SIN0_CH2", "I2SIN0"),
    route!("UL_CM0", NULL, "UL_CM0_CH1"), route!("UL_CM0", NULL, "UL_CM0_CH2"), route!("UL_CM0", NULL, "UL_CM0_CH3"), route!("UL_CM0", NULL, "UL_CM0_CH4"),
    route!("UL_CM0", NULL, "UL_CM0_CH5"), route!("UL_CM0", NULL, "UL_CM0_CH6"), route!("UL_CM0", NULL, "UL_CM0_CH7"), route!("UL_CM0", NULL, "UL_CM0_CH8"),
    route!("UL_CM0_CH1", "ADDA_UL_CH1", "ADDA_UL_Mux"), route!("UL_CM0_CH2", "ADDA_UL_CH2", "ADDA_UL_Mux"),
    route!("UL_CM0_CH3", "ADDA_UL_CH3", "ADDA_CH34_UL_Mux"), route!("UL_CM0_CH4", "ADDA_UL_CH4", "ADDA_CH34_UL_Mux"),
    route!("UL_CM1", NULL, "UL_CM1_CH1"), route!("UL_CM1", NULL, "UL_CM1_CH2"), route!("UL_CM1", NULL, "UL_CM1_CH3"), route!("UL_CM1", NULL, "UL_CM1_CH4"),
    route!("UL_CM1", NULL, "UL_CM1_CH5"), route!("UL_CM1", NULL, "UL_CM1_CH6"), route!("UL_CM1", NULL, "UL_CM1_CH7"), route!("UL_CM1", NULL, "UL_CM1_CH8"),
    route!("UL_CM1", NULL, "UL_CM1_CH9"), route!("UL_CM1", NULL, "UL_CM1_CH10"), route!("UL_CM1", NULL, "UL_CM1_CH11"), route!("UL_CM1", NULL, "UL_CM1_CH12"),
    route!("UL_CM1", NULL, "UL_CM1_CH13"), route!("UL_CM1", NULL, "UL_CM1_CH14"), route!("UL_CM1", NULL, "UL_CM1_CH15"), route!("UL_CM1", NULL, "UL_CM1_CH16"),
    route!("UL_CM1_CH1", "ADDA_UL_CH1", "ADDA_UL_Mux"), route!("UL_CM1_CH2", "ADDA_UL_CH2", "ADDA_UL_Mux"),
    route!("UL_CM1_CH3", "ADDA_UL_CH3", "ADDA_CH34_UL_Mux"), route!("UL_CM1_CH4", "ADDA_UL_CH4", "ADDA_CH34_UL_Mux"),
    route!("UL_CM2", NULL, "UL_CM2_CH1"), route!("UL_CM2", NULL, "UL_CM2_CH2"), route!("UL_CM2", NULL, "UL_CM2_CH3"), route!("UL_CM2", NULL, "UL_CM2_CH4"),
    route!("UL_CM2", NULL, "UL_CM2_CH5"), route!("UL_CM2", NULL, "UL_CM2_CH6"), route!("UL_CM2", NULL, "UL_CM2_CH7"), route!("UL_CM2", NULL, "UL_CM2_CH8"),
    route!("UL_CM2", NULL, "UL_CM2_CH9"), route!("UL_CM2", NULL, "UL_CM2_CH10"), route!("UL_CM2", NULL, "UL_CM2_CH11"), route!("UL_CM2", NULL, "UL_CM2_CH12"),
    route!("UL_CM2", NULL, "UL_CM2_CH13"), route!("UL_CM2", NULL, "UL_CM2_CH14"), route!("UL_CM2", NULL, "UL_CM2_CH15"), route!("UL_CM2", NULL, "UL_CM2_CH16"),
    route!("UL_CM2", NULL, "UL_CM2_CH17"), route!("UL_CM2", NULL, "UL_CM2_CH18"), route!("UL_CM2", NULL, "UL_CM2_CH19"), route!("UL_CM2", NULL, "UL_CM2_CH20"),
    route!("UL_CM2", NULL, "UL_CM2_CH21"), route!("UL_CM2", NULL, "UL_CM2_CH22"), route!("UL_CM2", NULL, "UL_CM2_CH23"), route!("UL_CM2", NULL, "UL_CM2_CH24"),
    route!("UL_CM2", NULL, "UL_CM2_CH25"), route!("UL_CM2", NULL, "UL_CM2_CH26"), route!("UL_CM2", NULL, "UL_CM2_CH27"), route!("UL_CM2", NULL, "UL_CM2_CH28"),
    route!("UL_CM2", NULL, "UL_CM2_CH29"), route!("UL_CM2", NULL, "UL_CM2_CH30"), route!("UL_CM2", NULL, "UL_CM2_CH31"), route!("UL_CM2", NULL, "UL_CM2_CH32"),
    route!("UL_CM2_CH1", "ADDA_UL_CH1", "ADDA_UL_Mux"), route!("UL_CM2_CH2", "ADDA_UL_CH2", "ADDA_UL_Mux"),
    route!("UL_CM2_CH3", "ADDA_UL_CH3", "ADDA_CH34_UL_Mux"), route!("UL_CM2_CH4", "ADDA_UL_CH4", "ADDA_CH34_UL_Mux"),
    /* Audio Pin */
    route!("I2SOUT4", NULL, "I2S4_PIN"), route!("I2SIN4", NULL, "I2S4_PIN"),
    route!("I2SOUT6", NULL, "I2S6_PIN"), route!("I2SIN6", NULL, "I2S6_PIN"),
    route!("I2SOUT3", NULL, "I2S3_PIN"), route!("I2SIN3", NULL, "I2S3_PIN"),
    route!("AP DMIC Capture", NULL, "AP_DMIC0_PIN"), route!("AP DMIC CH34 Capture", NULL, "AP_DMIC1_PIN"),
];

macro_rules! MT8196_DL_MEMIF {
    ($id:ident, $memif_id:expr, $base:expr, $cur:expr, $end:expr, $base_msb:expr, $cur_msb:expr, $end_msb:expr, $con0:expr, $fs_sft:expr, $fs_mask:expr, $mono_sft:expr, $on_sft:expr, $hd_sft:expr, $halign_sft:expr, $pbuf_mask:expr, $pbuf_sft:expr, $minlen_mask:expr, $minlen_sft:expr) => {
        mtk_base_memif_data { name: cstr!(stringify!($id)), id: $memif_id, reg_ofs_base: $base, reg_ofs_cur: $cur, reg_ofs_end: $end, reg_ofs_base_msb: $base_msb, reg_ofs_cur_msb: $cur_msb, reg_ofs_end_msb: $end_msb, fs_reg: $con0, fs_shift: $fs_sft, fs_maskbit: $fs_mask, mono_reg: $con0, mono_shift: $mono_sft, enable_reg: $con0, enable_shift: $on_sft, hd_reg: $con0, hd_shift: $hd_sft, hd_align_reg: $con0, hd_align_mshift: $halign_sft, agent_disable_reg: -1, agent_disable_shift: -1, msb_reg: -1, msb_shift: -1, pbuf_reg: $con0, pbuf_mask: $pbuf_mask, pbuf_shift: $pbuf_sft, minlen_reg: $con0, minlen_mask: $minlen_mask, minlen_shift: $minlen_sft, ch_num_reg: 0, ch_num_maskbit: 0, ch_num_shift: -1 }
    };
}
macro_rules! MT8196_MULTI_DL_MEMIF {
    ($id:ident, $memif_id:expr, $base:expr, $cur:expr, $end:expr, $base_msb:expr, $cur_msb:expr, $end_msb:expr, $con0:expr, $fs_sft:expr, $fs_mask:expr, $on_sft:expr, $hd_sft:expr, $halign_sft:expr, $pbuf_mask:expr, $pbuf_sft:expr, $minlen_mask:expr, $minlen_sft:expr, $num_mask:expr, $num_sft:expr) => {
        mtk_base_memif_data { name: cstr!(stringify!($id)), id: $memif_id, reg_ofs_base: $base, reg_ofs_cur: $cur, reg_ofs_end: $end, reg_ofs_base_msb: $base_msb, reg_ofs_cur_msb: $cur_msb, reg_ofs_end_msb: $end_msb, fs_reg: $con0, fs_shift: $fs_sft, fs_maskbit: $fs_mask, mono_reg: -1, mono_shift: -1, enable_reg: $con0, enable_shift: $on_sft, hd_reg: $con0, hd_shift: $hd_sft, hd_align_reg: $con0, hd_align_mshift: $halign_sft, agent_disable_reg: -1, agent_disable_shift: -1, msb_reg: -1, msb_shift: -1, pbuf_reg: $con0, pbuf_mask: $pbuf_mask, pbuf_shift: $pbuf_sft, minlen_reg: $con0, minlen_mask: $minlen_mask, minlen_shift: $minlen_sft, ch_num_reg: $con0, ch_num_maskbit: $num_mask, ch_num_shift: $num_sft }
    };
}
macro_rules! MT8196_UL_MEMIF {
    ($id:ident, $memif_id:expr, $base:expr, $cur:expr, $end:expr, $base_msb:expr, $cur_msb:expr, $end_msb:expr, $con0:expr, $fs_sft:expr, $fs_mask:expr, $mono_sft:expr, $on_sft:expr, $hd_sft:expr, $halign_sft:expr) => {
        mtk_base_memif_data { name: cstr!(stringify!($id)), id: $memif_id, reg_ofs_base: $base, reg_ofs_cur: $cur, reg_ofs_end: $end, reg_ofs_base_msb: $base_msb, reg_ofs_cur_msb: $cur_msb, reg_ofs_end_msb: $end_msb, fs_reg: $con0, fs_shift: $fs_sft, fs_maskbit: $fs_mask, mono_reg: $con0, mono_shift: $mono_sft, enable_reg: $con0, enable_shift: $on_sft, hd_reg: $con0, hd_shift: $hd_sft, hd_align_reg: $con0, hd_align_mshift: $halign_sft, agent_disable_reg: -1, agent_disable_shift: -1, msb_reg: -1, msb_shift: -1, pbuf_reg: 0, pbuf_mask: 0, pbuf_shift: 0, minlen_reg: 0, minlen_mask: 0, minlen_shift: 0, ch_num_reg: 0, ch_num_maskbit: 0, ch_num_shift: -1 }
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

static memif_data: [mtk_base_memif_data; MT8196_MEMIF_NUM as usize] = [
    MT8196_DL_MEMIF!(DL0, MT8196_MEMIF_DL0, AFE_DL0_BASE, AFE_DL0_CUR, AFE_DL0_END, AFE_DL0_BASE_MSB, AFE_DL0_CUR_MSB, AFE_DL0_END_MSB, AFE_DL0_CON0, DL0_SEL_FS_SFT, DL0_SEL_FS_MASK, DL0_MONO_SFT, DL0_ON_SFT, DL0_HD_MODE_SFT, DL0_HALIGN_SFT, DL0_PBUF_SIZE_MASK, DL0_PBUF_SIZE_SFT, DL0_MINLEN_MASK, DL0_MINLEN_SFT),
    MT8196_DL_MEMIF!(DL1, MT8196_MEMIF_DL1, AFE_DL1_BASE, AFE_DL1_CUR, AFE_DL1_END, AFE_DL1_BASE_MSB, AFE_DL1_CUR_MSB, AFE_DL1_END_MSB, AFE_DL1_CON0, DL1_SEL_FS_SFT, DL1_SEL_FS_MASK, DL1_MONO_SFT, DL1_ON_SFT, DL1_HD_MODE_SFT, DL1_HALIGN_SFT, DL1_PBUF_SIZE_MASK, DL1_PBUF_SIZE_SFT, DL1_MINLEN_MASK, DL1_MINLEN_SFT),
    MT8196_DL_MEMIF!(DL2, MT8196_MEMIF_DL2, AFE_DL2_BASE, AFE_DL2_CUR, AFE_DL2_END, AFE_DL2_BASE_MSB, AFE_DL2_CUR_MSB, AFE_DL2_END_MSB, AFE_DL2_CON0, DL2_SEL_FS_SFT, DL2_SEL_FS_MASK, DL2_MONO_SFT, DL2_ON_SFT, DL2_HD_MODE_SFT, DL2_HALIGN_SFT, DL2_PBUF_SIZE_MASK, DL2_PBUF_SIZE_SFT, DL2_MINLEN_MASK, DL2_MINLEN_SFT),
    MT8196_DL_MEMIF!(DL3, MT8196_MEMIF_DL3, AFE_DL3_BASE, AFE_DL3_CUR, AFE_DL3_END, AFE_DL3_BASE_MSB, AFE_DL3_CUR_MSB, AFE_DL3_END_MSB, AFE_DL3_CON0, DL3_SEL_FS_SFT, DL3_SEL_FS_MASK, DL3_MONO_SFT, DL3_ON_SFT, DL3_HD_MODE_SFT, DL3_HALIGN_SFT, DL3_PBUF_SIZE_MASK, DL3_PBUF_SIZE_SFT, DL3_MINLEN_MASK, DL3_MINLEN_SFT),
    MT8196_DL_MEMIF!(DL4, MT8196_MEMIF_DL4, AFE_DL4_BASE, AFE_DL4_CUR, AFE_DL4_END, AFE_DL4_BASE_MSB, AFE_DL4_CUR_MSB, AFE_DL4_END_MSB, AFE_DL4_CON0, DL4_SEL_FS_SFT, DL4_SEL_FS_MASK, DL4_MONO_SFT, DL4_ON_SFT, DL4_HD_MODE_SFT, DL4_HALIGN_SFT, DL4_PBUF_SIZE_MASK, DL4_PBUF_SIZE_SFT, DL4_MINLEN_MASK, DL4_MINLEN_SFT),
    MT8196_DL_MEMIF!(DL5, MT8196_MEMIF_DL5, AFE_DL5_BASE, AFE_DL5_CUR, AFE_DL5_END, AFE_DL5_BASE_MSB, AFE_DL5_CUR_MSB, AFE_DL5_END_MSB, AFE_DL5_CON0, DL5_SEL_FS_SFT, DL5_SEL_FS_MASK, DL5_MONO_SFT, DL5_ON_SFT, DL5_HD_MODE_SFT, DL5_HALIGN_SFT, DL5_PBUF_SIZE_MASK, DL5_PBUF_SIZE_SFT, DL5_MINLEN_MASK, DL5_MINLEN_SFT),
    MT8196_DL_MEMIF!(DL6, MT8196_MEMIF_DL6, AFE_DL6_BASE, AFE_DL6_CUR, AFE_DL6_END, AFE_DL6_BASE_MSB, AFE_DL6_CUR_MSB, AFE_DL6_END_MSB, AFE_DL6_CON0, DL6_SEL_FS_SFT, DL6_SEL_FS_MASK, DL6_MONO_SFT, DL6_ON_SFT, DL6_HD_MODE_SFT, DL6_HALIGN_SFT, DL6_PBUF_SIZE_MASK, DL6_PBUF_SIZE_SFT, DL6_MINLEN_MASK, DL6_MINLEN_SFT),
    MT8196_DL_MEMIF!(DL7, MT8196_MEMIF_DL7, AFE_DL7_BASE, AFE_DL7_CUR, AFE_DL7_END, AFE_DL7_BASE_MSB, AFE_DL7_CUR_MSB, AFE_DL7_END_MSB, AFE_DL7_CON0, DL7_SEL_FS_SFT, DL7_SEL_FS_MASK, DL7_MONO_SFT, DL7_ON_SFT, DL7_HD_MODE_SFT, DL7_HALIGN_SFT, DL7_PBUF_SIZE_MASK, DL7_PBUF_SIZE_SFT, DL7_MINLEN_MASK, DL7_MINLEN_SFT),
    MT8196_DL_MEMIF!(DL8, MT8196_MEMIF_DL8, AFE_DL8_BASE, AFE_DL8_CUR, AFE_DL8_END, AFE_DL8_BASE_MSB, AFE_DL8_CUR_MSB, AFE_DL8_END_MSB, AFE_DL8_CON0, DL8_SEL_FS_SFT, DL8_SEL_FS_MASK, DL8_MONO_SFT, DL8_ON_SFT, DL8_HD_MODE_SFT, DL8_HALIGN_SFT, DL8_PBUF_SIZE_MASK, DL8_PBUF_SIZE_SFT, DL8_MINLEN_MASK, DL8_MINLEN_SFT),
    MT8196_DL_MEMIF!(DL23, MT8196_MEMIF_DL23, AFE_DL23_BASE, AFE_DL23_CUR, AFE_DL23_END, AFE_DL23_BASE_MSB, AFE_DL23_CUR_MSB, AFE_DL23_END_MSB, AFE_DL23_CON0, DL23_SEL_FS_SFT, DL23_SEL_FS_MASK, DL23_MONO_SFT, DL23_ON_SFT, DL23_HD_MODE_SFT, DL23_HALIGN_SFT, DL23_PBUF_SIZE_MASK, DL23_PBUF_SIZE_SFT, DL23_MINLEN_MASK, DL23_MINLEN_SFT),
    MT8196_DL_MEMIF!(DL24, MT8196_MEMIF_DL24, AFE_DL24_BASE, AFE_DL24_CUR, AFE_DL24_END, AFE_DL24_BASE_MSB, AFE_DL24_CUR_MSB, AFE_DL24_END_MSB, AFE_DL24_CON0, DL24_SEL_FS_SFT, DL24_SEL_FS_MASK, DL24_MONO_SFT, DL24_ON_SFT, DL24_HD_MODE_SFT, DL24_HALIGN_SFT, DL24_PBUF_SIZE_MASK, DL24_PBUF_SIZE_SFT, DL24_MINLEN_MASK, DL24_MINLEN_SFT),
    MT8196_DL_MEMIF!(DL25, MT8196_MEMIF_DL25, AFE_DL25_BASE, AFE_DL25_CUR, AFE_DL25_END, AFE_DL25_BASE_MSB, AFE_DL25_CUR_MSB, AFE_DL25_END_MSB, AFE_DL25_CON0, DL25_SEL_FS_SFT, DL25_SEL_FS_MASK, DL25_MONO_SFT, DL25_ON_SFT, DL25_HD_MODE_SFT, DL25_HALIGN_SFT, DL25_PBUF_SIZE_MASK, DL25_PBUF_SIZE_SFT, DL25_MINLEN_MASK, DL25_MINLEN_SFT),
    MT8196_DL_MEMIF!(DL26, MT8196_MEMIF_DL26, AFE_DL26_BASE, AFE_DL26_CUR, AFE_DL26_END, AFE_DL26_BASE_MSB, AFE_DL26_CUR_MSB, AFE_DL26_END_MSB, AFE_DL26_CON0, DL26_SEL_FS_SFT, DL26_SEL_FS_MASK, DL26_MONO_SFT, DL26_ON_SFT, DL26_HD_MODE_SFT, DL26_HALIGN_SFT, DL26_PBUF_SIZE_MASK, DL26_PBUF_SIZE_SFT, DL26_MINLEN_MASK, DL26_MINLEN_SFT),
    MT8196_MULTI_DL_MEMIF!(DL_4CH, MT8196_MEMIF_DL_4CH, AFE_DL_4CH_BASE, AFE_DL_4CH_CUR, AFE_DL_4CH_END, AFE_DL_4CH_BASE_MSB, AFE_DL_4CH_CUR_MSB, AFE_DL_4CH_END_MSB, AFE_DL_4CH_CON0, DL_4CH_SEL_FS_SFT, DL_4CH_SEL_FS_MASK, DL_4CH_ON_SFT, DL_4CH_HD_MODE_SFT, DL_4CH_HALIGN_SFT, DL_4CH_PBUF_SIZE_MASK, DL_4CH_PBUF_SIZE_SFT, DL_4CH_MINLEN_MASK, DL_4CH_MINLEN_SFT, DL_4CH_NUM_MASK, DL_4CH_NUM_SFT),
    MT8196_MULTI_DL_MEMIF!(DL_24CH, MT8196_MEMIF_DL_24CH, AFE_DL_24CH_BASE, AFE_DL_24CH_CUR, AFE_DL_24CH_END, AFE_DL_24CH_BASE_MSB, AFE_DL_24CH_CUR_MSB, AFE_DL_24CH_END_MSB, AFE_DL_24CH_CON0, DL_24CH_SEL_FS_SFT, DL_24CH_SEL_FS_MASK, DL_24CH_ON_SFT, DL_24CH_HD_MODE_SFT, DL_24CH_HALIGN_SFT, DL_24CH_PBUF_SIZE_MASK, DL_24CH_PBUF_SIZE_SFT, DL_24CH_MINLEN_MASK, DL_24CH_MINLEN_SFT, DL_24CH_NUM_MASK, DL_24CH_NUM_SFT),
    MT8196_MULTI_DL_MEMIF!(HDMI, MT8196_MEMIF_HDMI, AFE_HDMI_BASE, AFE_HDMI_CUR, AFE_HDMI_END, AFE_HDMI_BASE_MSB, AFE_HDMI_CUR_MSB, AFE_HDMI_END_MSB, AFE_HDMI_CON0, HDMI_SEL_FS_SFT, HDMI_SEL_FS_MASK, HDMI_ON_SFT, HDMI_HD_MODE_SFT, HDMI_HALIGN_SFT, HDMI_PBUF_SIZE_MASK, HDMI_PBUF_SIZE_SFT, HDMI_MINLEN_MASK, HDMI_MINLEN_SFT, HDMI_NUM_MASK, HDMI_NUM_SFT),
    MT8196_UL_MEMIF!(VUL0, MT8196_MEMIF_VUL0, AFE_VUL0_BASE, AFE_VUL0_CUR, AFE_VUL0_END, AFE_VUL0_BASE_MSB, AFE_VUL0_CUR_MSB, AFE_VUL0_END_MSB, AFE_VUL0_CON0, VUL0_SEL_FS_SFT, VUL0_SEL_FS_MASK, VUL0_MONO_SFT, VUL0_ON_SFT, VUL0_HD_MODE_SFT, VUL0_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL1, MT8196_MEMIF_VUL1, AFE_VUL1_BASE, AFE_VUL1_CUR, AFE_VUL1_END, AFE_VUL1_BASE_MSB, AFE_VUL1_CUR_MSB, AFE_VUL1_END_MSB, AFE_VUL1_CON0, VUL1_SEL_FS_SFT, VUL1_SEL_FS_MASK, VUL1_MONO_SFT, VUL1_ON_SFT, VUL1_HD_MODE_SFT, VUL1_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL2, MT8196_MEMIF_VUL2, AFE_VUL2_BASE, AFE_VUL2_CUR, AFE_VUL2_END, AFE_VUL2_BASE_MSB, AFE_VUL2_CUR_MSB, AFE_VUL2_END_MSB, AFE_VUL2_CON0, VUL2_SEL_FS_SFT, VUL2_SEL_FS_MASK, VUL2_MONO_SFT, VUL2_ON_SFT, VUL2_HD_MODE_SFT, VUL2_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL3, MT8196_MEMIF_VUL3, AFE_VUL3_BASE, AFE_VUL3_CUR, AFE_VUL3_END, AFE_VUL3_BASE_MSB, AFE_VUL3_CUR_MSB, AFE_VUL3_END_MSB, AFE_VUL3_CON0, VUL3_SEL_FS_SFT, VUL3_SEL_FS_MASK, VUL3_MONO_SFT, VUL3_ON_SFT, VUL3_HD_MODE_SFT, VUL3_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL4, MT8196_MEMIF_VUL4, AFE_VUL4_BASE, AFE_VUL4_CUR, AFE_VUL4_END, AFE_VUL4_BASE_MSB, AFE_VUL4_CUR_MSB, AFE_VUL4_END_MSB, AFE_VUL4_CON0, VUL4_SEL_FS_SFT, VUL4_SEL_FS_MASK, VUL4_MONO_SFT, VUL4_ON_SFT, VUL4_HD_MODE_SFT, VUL4_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL5, MT8196_MEMIF_VUL5, AFE_VUL5_BASE, AFE_VUL5_CUR, AFE_VUL5_END, AFE_VUL5_BASE_MSB, AFE_VUL5_CUR_MSB, AFE_VUL5_END_MSB, AFE_VUL5_CON0, VUL5_SEL_FS_SFT, VUL5_SEL_FS_MASK, VUL5_MONO_SFT, VUL5_ON_SFT, VUL5_HD_MODE_SFT, VUL5_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL6, MT8196_MEMIF_VUL6, AFE_VUL6_BASE, AFE_VUL6_CUR, AFE_VUL6_END, AFE_VUL6_BASE_MSB, AFE_VUL6_CUR_MSB, AFE_VUL6_END_MSB, AFE_VUL6_CON0, VUL6_SEL_FS_SFT, VUL6_SEL_FS_MASK, VUL6_MONO_SFT, VUL6_ON_SFT, VUL6_HD_MODE_SFT, VUL6_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL7, MT8196_MEMIF_VUL7, AFE_VUL7_BASE, AFE_VUL7_CUR, AFE_VUL7_END, AFE_VUL7_BASE_MSB, AFE_VUL7_CUR_MSB, AFE_VUL7_END_MSB, AFE_VUL7_CON0, VUL7_SEL_FS_SFT, VUL7_SEL_FS_MASK, VUL7_MONO_SFT, VUL7_ON_SFT, VUL7_HD_MODE_SFT, VUL7_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL8, MT8196_MEMIF_VUL8, AFE_VUL8_BASE, AFE_VUL8_CUR, AFE_VUL8_END, AFE_VUL8_BASE_MSB, AFE_VUL8_CUR_MSB, AFE_VUL8_END_MSB, AFE_VUL8_CON0, VUL8_SEL_FS_SFT, VUL8_SEL_FS_MASK, VUL8_MONO_SFT, VUL8_ON_SFT, VUL8_HD_MODE_SFT, VUL8_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL9, MT8196_MEMIF_VUL9, AFE_VUL9_BASE, AFE_VUL9_CUR, AFE_VUL9_END, AFE_VUL9_BASE_MSB, AFE_VUL9_CUR_MSB, AFE_VUL9_END_MSB, AFE_VUL9_CON0, VUL9_SEL_FS_SFT, VUL9_SEL_FS_MASK, VUL9_MONO_SFT, VUL9_ON_SFT, VUL9_HD_MODE_SFT, VUL9_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL10, MT8196_MEMIF_VUL10, AFE_VUL10_BASE, AFE_VUL10_CUR, AFE_VUL10_END, AFE_VUL10_BASE_MSB, AFE_VUL10_CUR_MSB, AFE_VUL10_END_MSB, AFE_VUL10_CON0, VUL10_SEL_FS_SFT, VUL10_SEL_FS_MASK, VUL10_MONO_SFT, VUL10_ON_SFT, VUL10_HD_MODE_SFT, VUL10_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL24, MT8196_MEMIF_VUL24, AFE_VUL24_BASE, AFE_VUL24_CUR, AFE_VUL24_END, AFE_VUL24_BASE_MSB, AFE_VUL24_CUR_MSB, AFE_VUL24_END_MSB, AFE_VUL24_CON0, VUL24_SEL_FS_SFT, VUL24_SEL_FS_MASK, VUL24_MONO_SFT, VUL24_ON_SFT, VUL24_HD_MODE_SFT, VUL24_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL25, MT8196_MEMIF_VUL25, AFE_VUL25_BASE, AFE_VUL25_CUR, AFE_VUL25_END, AFE_VUL25_BASE_MSB, AFE_VUL25_CUR_MSB, AFE_VUL25_END_MSB, AFE_VUL25_CON0, VUL25_SEL_FS_SFT, VUL25_SEL_FS_MASK, VUL25_MONO_SFT, VUL25_ON_SFT, VUL25_HD_MODE_SFT, VUL25_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL26, MT8196_MEMIF_VUL26, AFE_VUL26_BASE, AFE_VUL26_CUR, AFE_VUL26_END, AFE_VUL26_BASE_MSB, AFE_VUL26_CUR_MSB, AFE_VUL26_END_MSB, AFE_VUL26_CON0, VUL26_SEL_FS_SFT, VUL26_SEL_FS_MASK, VUL26_MONO_SFT, VUL26_ON_SFT, VUL26_HD_MODE_SFT, VUL26_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL_CM0, MT8196_MEMIF_VUL_CM0, AFE_VUL_CM0_BASE, AFE_VUL_CM0_CUR, AFE_VUL_CM0_END, AFE_VUL_CM0_BASE_MSB, AFE_VUL_CM0_CUR_MSB, AFE_VUL_CM0_END_MSB, AFE_VUL_CM0_CON0, -1, -1, -1, VUL_CM0_ON_SFT, VUL_CM0_HD_MODE_SFT, VUL_CM0_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL_CM1, MT8196_MEMIF_VUL_CM1, AFE_VUL_CM1_BASE, AFE_VUL_CM1_CUR, AFE_VUL_CM1_END, AFE_VUL_CM1_BASE_MSB, AFE_VUL_CM1_CUR_MSB, AFE_VUL_CM1_END_MSB, AFE_VUL_CM1_CON0, -1, -1, -1, VUL_CM1_ON_SFT, VUL_CM1_HD_MODE_SFT, VUL_CM1_HALIGN_SFT),
    MT8196_UL_MEMIF!(VUL_CM2, MT8196_MEMIF_VUL_CM2, AFE_VUL_CM2_BASE, AFE_VUL_CM2_CUR, AFE_VUL_CM2_END, AFE_VUL_CM2_BASE_MSB, AFE_VUL_CM2_CUR_MSB, AFE_VUL_CM2_END_MSB, AFE_VUL_CM2_CON0, -1, -1, -1, VUL_CM2_ON_SFT, VUL_CM2_HD_MODE_SFT, VUL_CM2_HALIGN_SFT),
    MT8196_UL_MEMIF!(ETDM_IN0, MT8196_MEMIF_ETDM_IN0, AFE_ETDM_IN0_BASE, AFE_ETDM_IN0_CUR, AFE_ETDM_IN0_END, AFE_ETDM_IN0_BASE_MSB, AFE_ETDM_IN0_CUR_MSB, AFE_ETDM_IN0_END_MSB, AFE_ETDM_IN0_CON0, REG_FS_TIMING_SEL_SFT, REG_FS_TIMING_SEL_MASK, -1, ETDM_IN0_ON_SFT, ETDM_IN0_HD_MODE_SFT, ETDM_IN0_HALIGN_SFT),
    MT8196_UL_MEMIF!(ETDM_IN1, MT8196_MEMIF_ETDM_IN1, AFE_ETDM_IN1_BASE, AFE_ETDM_IN1_CUR, AFE_ETDM_IN1_END, AFE_ETDM_IN1_BASE_MSB, AFE_ETDM_IN1_CUR_MSB, AFE_ETDM_IN1_END_MSB, AFE_ETDM_IN1_CON0, REG_FS_TIMING_SEL_SFT, REG_FS_TIMING_SEL_MASK, -1, ETDM_IN1_ON_SFT, ETDM_IN1_HD_MODE_SFT, ETDM_IN1_HALIGN_SFT),
    MT8196_UL_MEMIF!(ETDM_IN2, MT8196_MEMIF_ETDM_IN2, AFE_ETDM_IN2_BASE, AFE_ETDM_IN2_CUR, AFE_ETDM_IN2_END, AFE_ETDM_IN2_BASE_MSB, AFE_ETDM_IN2_CUR_MSB, AFE_ETDM_IN2_END_MSB, AFE_ETDM_IN2_CON0, REG_FS_TIMING_SEL_SFT, REG_FS_TIMING_SEL_MASK, -1, ETDM_IN2_ON_SFT, ETDM_IN2_HD_MODE_SFT, ETDM_IN2_HALIGN_SFT),
    MT8196_UL_MEMIF!(ETDM_IN3, MT8196_MEMIF_ETDM_IN3, AFE_ETDM_IN3_BASE, AFE_ETDM_IN3_CUR, AFE_ETDM_IN3_END, AFE_ETDM_IN3_BASE_MSB, AFE_ETDM_IN3_CUR_MSB, AFE_ETDM_IN3_END_MSB, AFE_ETDM_IN3_CON0, REG_FS_TIMING_SEL_SFT, REG_FS_TIMING_SEL_MASK, -1, ETDM_IN3_ON_SFT, ETDM_IN3_HD_MODE_SFT, ETDM_IN3_HALIGN_SFT),
    MT8196_UL_MEMIF!(ETDM_IN4, MT8196_MEMIF_ETDM_IN4, AFE_ETDM_IN4_BASE, AFE_ETDM_IN4_CUR, AFE_ETDM_IN4_END, AFE_ETDM_IN4_BASE_MSB, AFE_ETDM_IN4_CUR_MSB, AFE_ETDM_IN4_END_MSB, AFE_ETDM_IN4_CON0, REG_FS_TIMING_SEL_SFT, REG_FS_TIMING_SEL_MASK, -1, ETDM_IN4_ON_SFT, ETDM_IN4_HD_MODE_SFT, ETDM_IN4_HALIGN_SFT),
    MT8196_UL_MEMIF!(ETDM_IN6, MT8196_MEMIF_ETDM_IN6, AFE_ETDM_IN6_BASE, AFE_ETDM_IN6_CUR, AFE_ETDM_IN6_END, AFE_ETDM_IN6_BASE_MSB, AFE_ETDM_IN6_CUR_MSB, AFE_ETDM_IN6_END_MSB, AFE_ETDM_IN6_CON0, REG_FS_TIMING_SEL_SFT, REG_FS_TIMING_SEL_MASK, -1, ETDM_IN6_ON_SFT, ETDM_IN6_HD_MODE_SFT, ETDM_IN6_HALIGN_SFT),
];

macro_rules! MT8196_AFE_IRQ {
    ($idx:expr, $id:expr, $cfg1:expr, $cfg0:expr, $fs_sft:expr, $fs_mask:expr, $on_sft:expr, $clr_sft:expr) => {
        mtk_base_irq_data { id: $id, irq_cnt_reg: $cfg1, irq_cnt_shift: AFE_IRQ_CNT_SHIFT, irq_cnt_maskbit: AFE_IRQ_CNT_MASK, irq_fs_reg: $cfg0, irq_fs_shift: $fs_sft, irq_fs_maskbit: $fs_mask, irq_en_reg: $cfg0, irq_en_shift: $on_sft, irq_clr_reg: $cfg1, irq_clr_shift: $clr_sft }
    };
}
macro_rules! MT8196_AFE_TDM_IRQ {
    ($id:expr) => {
        mtk_base_irq_data { id: MT8196_CUS_IRQ_TDM, irq_cnt_reg: AFE_CUSTOM_IRQ0_MCU_CFG1, irq_cnt_shift: AFE_CUSTOM_IRQ0_MCU_CNT_SFT, irq_cnt_maskbit: AFE_CUSTOM_IRQ0_MCU_CNT_MASK, irq_fs_reg: -1, irq_fs_shift: -1, irq_fs_maskbit: -1, irq_en_reg: AFE_CUSTOM_IRQ0_MCU_CFG0, irq_en_shift: AFE_CUSTOM_IRQ0_MCU_ON_SFT, irq_clr_reg: AFE_CUSTOM_IRQ0_MCU_CFG1, irq_clr_shift: AFE_CUSTOM_IRQ0_CLR_CFG_SFT }
    };
}

static irq_data: [mtk_base_irq_data; MT8196_IRQ_NUM as usize] = [
    MT8196_AFE_IRQ!(0, MT8196_IRQ_0, AFE_IRQ0_MCU_CFG1, AFE_IRQ0_MCU_CFG0, AFE_IRQ0_MCU_FS_SFT, AFE_IRQ0_MCU_FS_MASK, AFE_IRQ0_MCU_ON_SFT, AFE_IRQ0_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(1, MT8196_IRQ_1, AFE_IRQ1_MCU_CFG1, AFE_IRQ1_MCU_CFG0, AFE_IRQ1_MCU_FS_SFT, AFE_IRQ1_MCU_FS_MASK, AFE_IRQ1_MCU_ON_SFT, AFE_IRQ1_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(2, MT8196_IRQ_2, AFE_IRQ2_MCU_CFG1, AFE_IRQ2_MCU_CFG0, AFE_IRQ2_MCU_FS_SFT, AFE_IRQ2_MCU_FS_MASK, AFE_IRQ2_MCU_ON_SFT, AFE_IRQ2_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(3, MT8196_IRQ_3, AFE_IRQ3_MCU_CFG1, AFE_IRQ3_MCU_CFG0, AFE_IRQ3_MCU_FS_SFT, AFE_IRQ3_MCU_FS_MASK, AFE_IRQ3_MCU_ON_SFT, AFE_IRQ3_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(4, MT8196_IRQ_4, AFE_IRQ4_MCU_CFG1, AFE_IRQ4_MCU_CFG0, AFE_IRQ4_MCU_FS_SFT, AFE_IRQ4_MCU_FS_MASK, AFE_IRQ4_MCU_ON_SFT, AFE_IRQ4_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(5, MT8196_IRQ_5, AFE_IRQ5_MCU_CFG1, AFE_IRQ5_MCU_CFG0, AFE_IRQ5_MCU_FS_SFT, AFE_IRQ5_MCU_FS_MASK, AFE_IRQ5_MCU_ON_SFT, AFE_IRQ5_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(6, MT8196_IRQ_6, AFE_IRQ6_MCU_CFG1, AFE_IRQ6_MCU_CFG0, AFE_IRQ6_MCU_FS_SFT, AFE_IRQ6_MCU_FS_MASK, AFE_IRQ6_MCU_ON_SFT, AFE_IRQ6_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(7, MT8196_IRQ_7, AFE_IRQ7_MCU_CFG1, AFE_IRQ7_MCU_CFG0, AFE_IRQ7_MCU_FS_SFT, AFE_IRQ7_MCU_FS_MASK, AFE_IRQ7_MCU_ON_SFT, AFE_IRQ7_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(8, MT8196_IRQ_8, AFE_IRQ8_MCU_CFG1, AFE_IRQ8_MCU_CFG0, AFE_IRQ8_MCU_FS_SFT, AFE_IRQ8_MCU_FS_MASK, AFE_IRQ8_MCU_ON_SFT, AFE_IRQ8_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(9, MT8196_IRQ_9, AFE_IRQ9_MCU_CFG1, AFE_IRQ9_MCU_CFG0, AFE_IRQ9_MCU_FS_SFT, AFE_IRQ9_MCU_FS_MASK, AFE_IRQ9_MCU_ON_SFT, AFE_IRQ9_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(10, MT8196_IRQ_10, AFE_IRQ10_MCU_CFG1, AFE_IRQ10_MCU_CFG0, AFE_IRQ10_MCU_FS_SFT, AFE_IRQ10_MCU_FS_MASK, AFE_IRQ10_MCU_ON_SFT, AFE_IRQ10_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(11, MT8196_IRQ_11, AFE_IRQ11_MCU_CFG1, AFE_IRQ11_MCU_CFG0, AFE_IRQ11_MCU_FS_SFT, AFE_IRQ11_MCU_FS_MASK, AFE_IRQ11_MCU_ON_SFT, AFE_IRQ11_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(12, MT8196_IRQ_12, AFE_IRQ12_MCU_CFG1, AFE_IRQ12_MCU_CFG0, AFE_IRQ12_MCU_FS_SFT, AFE_IRQ12_MCU_FS_MASK, AFE_IRQ12_MCU_ON_SFT, AFE_IRQ12_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(13, MT8196_IRQ_13, AFE_IRQ13_MCU_CFG1, AFE_IRQ13_MCU_CFG0, AFE_IRQ13_MCU_FS_SFT, AFE_IRQ13_MCU_FS_MASK, AFE_IRQ13_MCU_ON_SFT, AFE_IRQ13_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(14, MT8196_IRQ_14, AFE_IRQ14_MCU_CFG1, AFE_IRQ14_MCU_CFG0, AFE_IRQ14_MCU_FS_SFT, AFE_IRQ14_MCU_FS_MASK, AFE_IRQ14_MCU_ON_SFT, AFE_IRQ14_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(15, MT8196_IRQ_15, AFE_IRQ15_MCU_CFG1, AFE_IRQ15_MCU_CFG0, AFE_IRQ15_MCU_FS_SFT, AFE_IRQ15_MCU_FS_MASK, AFE_IRQ15_MCU_ON_SFT, AFE_IRQ15_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(16, MT8196_IRQ_16, AFE_IRQ16_MCU_CFG1, AFE_IRQ16_MCU_CFG0, AFE_IRQ16_MCU_FS_SFT, AFE_IRQ16_MCU_FS_MASK, AFE_IRQ16_MCU_ON_SFT, AFE_IRQ16_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(17, MT8196_IRQ_17, AFE_IRQ17_MCU_CFG1, AFE_IRQ17_MCU_CFG0, AFE_IRQ17_MCU_FS_SFT, AFE_IRQ17_MCU_FS_MASK, AFE_IRQ17_MCU_ON_SFT, AFE_IRQ17_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(18, MT8196_IRQ_18, AFE_IRQ18_MCU_CFG1, AFE_IRQ18_MCU_CFG0, AFE_IRQ18_MCU_FS_SFT, AFE_IRQ18_MCU_FS_MASK, AFE_IRQ18_MCU_ON_SFT, AFE_IRQ18_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(19, MT8196_IRQ_19, AFE_IRQ19_MCU_CFG1, AFE_IRQ19_MCU_CFG0, AFE_IRQ19_MCU_FS_SFT, AFE_IRQ19_MCU_FS_MASK, AFE_IRQ19_MCU_ON_SFT, AFE_IRQ19_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(20, MT8196_IRQ_20, AFE_IRQ20_MCU_CFG1, AFE_IRQ20_MCU_CFG0, AFE_IRQ20_MCU_FS_SFT, AFE_IRQ20_MCU_FS_MASK, AFE_IRQ20_MCU_ON_SFT, AFE_IRQ20_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(21, MT8196_IRQ_21, AFE_IRQ21_MCU_CFG1, AFE_IRQ21_MCU_CFG0, AFE_IRQ21_MCU_FS_SFT, AFE_IRQ21_MCU_FS_MASK, AFE_IRQ21_MCU_ON_SFT, AFE_IRQ21_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(22, MT8196_IRQ_22, AFE_IRQ22_MCU_CFG1, AFE_IRQ22_MCU_CFG0, AFE_IRQ22_MCU_FS_SFT, AFE_IRQ22_MCU_FS_MASK, AFE_IRQ22_MCU_ON_SFT, AFE_IRQ22_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(23, MT8196_IRQ_23, AFE_IRQ23_MCU_CFG1, AFE_IRQ23_MCU_CFG0, AFE_IRQ23_MCU_FS_SFT, AFE_IRQ23_MCU_FS_MASK, AFE_IRQ23_MCU_ON_SFT, AFE_IRQ23_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(24, MT8196_IRQ_24, AFE_IRQ24_MCU_CFG1, AFE_IRQ24_MCU_CFG0, AFE_IRQ24_MCU_FS_SFT, AFE_IRQ24_MCU_FS_MASK, AFE_IRQ24_MCU_ON_SFT, AFE_IRQ24_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(25, MT8196_IRQ_25, AFE_IRQ25_MCU_CFG1, AFE_IRQ25_MCU_CFG0, AFE_IRQ25_MCU_FS_SFT, AFE_IRQ25_MCU_FS_MASK, AFE_IRQ25_MCU_ON_SFT, AFE_IRQ25_CLR_CFG_SFT),
    MT8196_AFE_IRQ!(26, MT8196_IRQ_26, AFE_IRQ26_MCU_CFG1, AFE_IRQ26_MCU_CFG0, AFE_IRQ26_MCU_FS_SFT, AFE_IRQ26_MCU_FS_MASK, AFE_IRQ26_MCU_ON_SFT, AFE_IRQ26_CLR_CFG_SFT),
    MT8196_AFE_TDM_IRQ!(31),
];

static memif_irq_usage: [c_int; MT8196_MEMIF_NUM as usize] = [
    /* TODO: verify each memif & irq */
    MT8196_IRQ_0, MT8196_IRQ_1, MT8196_IRQ_2, MT8196_IRQ_3, MT8196_IRQ_4, MT8196_IRQ_5,
    MT8196_IRQ_6, MT8196_IRQ_7, MT8196_IRQ_8, MT8196_IRQ_9, MT8196_IRQ_10, MT8196_IRQ_11,
    MT8196_IRQ_0, MT8196_IRQ_0, MT8196_IRQ_12, MT8196_IRQ_13, MT8196_IRQ_14, MT8196_IRQ_15,
    MT8196_IRQ_16, MT8196_IRQ_17, MT8196_IRQ_18, MT8196_IRQ_19, MT8196_IRQ_20, MT8196_IRQ_21,
    MT8196_IRQ_22, MT8196_IRQ_23, MT8196_IRQ_24, MT8196_IRQ_25, MT8196_IRQ_0, MT8196_IRQ_26,
    MT8196_IRQ_0, MT8196_IRQ_0, MT8196_IRQ_0, MT8196_IRQ_0, MT8196_IRQ_0, MT8196_IRQ_0,
    MT8196_IRQ_0, MT8196_IRQ_0, MT8196_IRQ_31,
];

unsafe extern "C" fn mt8196_is_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    /* these auto-gen reg has read-only bit, so put it as volatile */
    /* volatile reg cannot be cached, so cannot be set when power off */
    match reg as c_int {
        r if (AUDIO_TOP_CON0..=AUDIO_TOP_CON4).contains(&r) => true,
        AFE_APLL1_TUNER_MON0 | AFE_APLL2_TUNER_MON0 | AFE_SPM_CONTROL_ACK | AUDIO_TOP_IP_VERSION
        | AUDIO_ENGEN_CON0_MON | AUD_TOP_MON_RG | AFE_CONNSYS_I2S_IPM_VER_MON
        | AFE_CONNSYS_I2S_MON | AFE_PCM_INTF_MON | AFE_PCM_TOP_IP_VERSION
        | AFE_IRQ_MCU_STATUS | AFE_CUSTOM_IRQ_MCU_STATUS | AFE_CUSTOM_IRQ0_CNT_MON
        | AFE_STF_MON | AFE_STF_IP_VERSION | AFE_CM0_MON | AFE_CM0_IP_VERSION
        | AFE_CM1_MON | AFE_CM1_IP_VERSION | AFE_ADDA_UL0_IP_VERSION
        | AFE_ADDA_UL1_IP_VERSION | AFE_MTKAIF_IPM_VER_MON | AFE_MTKAIF_MON
        | AFE_AUD_PAD_TOP_MON | ETDM_IN0_MON | ETDM_IN1_MON | ETDM_IN2_MON
        | ETDM_IN4_MON | ETDM_IN6_MON | ETDM_OUT0_MON | ETDM_OUT1_MON
        | ETDM_OUT2_MON | ETDM_OUT4_MON | ETDM_OUT6_MON | AFE_DPTX_MON
        | AFE_TDM_TOP_IP_VERSION | AFE_CUSTOM_IRQ_MCU_EN | AFE_DL5_CON0 | AFE_DL6_CON0
        | AFE_DL23_CON0 | AFE_DL_24CH_CON0 | AFE_VUL1_CON0 | AFE_VUL3_CON0
        | AFE_VUL4_CON0 | AFE_VUL5_CON0 | AFE_VUL9_CON0 | AFE_VUL25_CON0 => true,
        r if (AFE_IRQ_MCU_MON0..=AFE_IRQ26_CNT_MON).contains(&r) => true,
        r if (AFE_ADDA_UL0_SRC_DEBUG_MON0..=AFE_ADDA_UL0_SRC_MON1).contains(&r) => true,
        r if (AFE_ADDA_UL1_SRC_DEBUG_MON0..=AFE_ADDA_UL1_SRC_MON1).contains(&r) => true,
        r if (AFE_ADDA_MTKAIFV4_MON0..=AFE_ADDA6_MTKAIFV4_MON0).contains(&r) => true,
        r if (AFE_CONN_MON0..=AFE_CONN_MON5).contains(&r) => true,
        r if (AFE_CBIP_SLV_DECODER_MON0..=AFE_CBIP_SLV_MUX_MON1).contains(&r) => true,
        r if (AFE_DL0_CUR_MSB..=AFE_DL0_CUR).contains(&r) || (AFE_DL0_RCH_MON..=AFE_DL0_LCH_MON).contains(&r) => true,
        r if (AFE_DL1_CUR_MSB..=AFE_DL1_CUR).contains(&r) || (AFE_DL1_RCH_MON..=AFE_DL1_LCH_MON).contains(&r) => true,
        r if (AFE_DL2_CUR_MSB..=AFE_DL2_CUR).contains(&r) || (AFE_DL2_RCH_MON..=AFE_DL2_LCH_MON).contains(&r) => true,
        r if (AFE_DL3_CUR_MSB..=AFE_DL3_CUR).contains(&r) || (AFE_DL3_RCH_MON..=AFE_DL3_LCH_MON).contains(&r) => true,
        r if (AFE_DL4_CUR_MSB..=AFE_DL4_CUR).contains(&r) || (AFE_DL4_RCH_MON..=AFE_DL4_LCH_MON).contains(&r) => true,
        r if (AFE_DL5_CUR_MSB..=AFE_DL5_CUR).contains(&r) || (AFE_DL5_RCH_MON..=AFE_DL5_LCH_MON).contains(&r) => true,
        r if (AFE_DL6_CUR_MSB..=AFE_DL6_CUR).contains(&r) || (AFE_DL6_RCH_MON..=AFE_DL6_LCH_MON).contains(&r) => true,
        r if (AFE_DL7_CUR_MSB..=AFE_DL7_CUR).contains(&r) || (AFE_DL7_RCH_MON..=AFE_DL7_LCH_MON).contains(&r) => true,
        r if (AFE_DL8_CUR_MSB..=AFE_DL8_CUR).contains(&r) || (AFE_DL8_RCH_MON..=AFE_DL8_LCH_MON).contains(&r) => true,
        r if (AFE_DL_24CH_CUR_MSB..=AFE_DL_24CH_CUR).contains(&r) || (AFE_DL_4CH_CUR_MSB..=AFE_DL_4CH_CUR).contains(&r) => true,
        r if (AFE_DL23_CUR_MSB..=AFE_DL23_CUR).contains(&r) || (AFE_DL23_RCH_MON..=AFE_DL23_LCH_MON).contains(&r) => true,
        r if (AFE_DL24_CUR_MSB..=AFE_DL24_CUR).contains(&r) || (AFE_DL24_RCH_MON..=AFE_DL24_LCH_MON).contains(&r) => true,
        r if (AFE_DL25_CUR_MSB..=AFE_DL25_CUR).contains(&r) || (AFE_DL25_RCH_MON..=AFE_DL25_LCH_MON).contains(&r) => true,
        r if (AFE_DL26_CUR_MSB..=AFE_DL26_CUR).contains(&r) || (AFE_DL26_RCH_MON..=AFE_DL26_LCH_MON).contains(&r) => true,
        r if (AFE_VUL0_CUR_MSB..=AFE_VUL0_CUR).contains(&r)
            || (AFE_VUL1_CUR_MSB..=AFE_VUL1_CUR).contains(&r)
            || (AFE_VUL2_CUR_MSB..=AFE_VUL2_CUR).contains(&r)
            || (AFE_VUL3_CUR_MSB..=AFE_VUL3_CUR).contains(&r)
            || (AFE_VUL4_CUR_MSB..=AFE_VUL4_CUR).contains(&r)
            || (AFE_VUL5_CUR_MSB..=AFE_VUL5_CUR).contains(&r)
            || (AFE_VUL6_CUR_MSB..=AFE_VUL6_CUR).contains(&r)
            || (AFE_VUL7_CUR_MSB..=AFE_VUL7_CUR).contains(&r)
            || (AFE_VUL8_CUR_MSB..=AFE_VUL8_CUR).contains(&r)
            || (AFE_VUL9_CUR_MSB..=AFE_VUL9_CUR).contains(&r)
            || (AFE_VUL10_CUR_MSB..=AFE_VUL10_CUR).contains(&r)
            || (AFE_VUL24_CUR_MSB..=AFE_VUL24_CUR).contains(&r)
            || (AFE_VUL25_CUR_MSB..=AFE_VUL25_CUR).contains(&r)
            || (AFE_VUL25_RCH_MON..=AFE_VUL25_LCH_MON).contains(&r)
            || (AFE_VUL26_CUR_MSB..=AFE_VUL26_CUR).contains(&r)
            || (AFE_VUL_CM0_BASE_MSB..=AFE_VUL_CM0_CON0).contains(&r)
            || (AFE_VUL_CM1_CUR_MSB..=AFE_VUL_CM1_CUR).contains(&r)
            || (AFE_VUL_CM2_CUR_MSB..=AFE_VUL_CM2_CUR).contains(&r)
            || (AFE_ETDM_IN0_CUR_MSB..=AFE_ETDM_IN0_CUR).contains(&r)
            || (AFE_ETDM_IN1_CUR_MSB..=AFE_ETDM_IN1_CUR).contains(&r)
            || (AFE_ETDM_IN2_CUR_MSB..=AFE_ETDM_IN2_CUR).contains(&r)
            || (AFE_ETDM_IN3_CUR_MSB..=AFE_ETDM_IN3_CUR).contains(&r)
            || (AFE_ETDM_IN4_CUR_MSB..=AFE_ETDM_IN4_CUR).contains(&r)
            || (AFE_ETDM_IN6_CUR_MSB..=AFE_ETDM_IN6_CUR).contains(&r)
            || (AFE_HDMI_OUT_CUR_MSB..=AFE_HDMI_OUT_CUR).contains(&r) => true,
        AFE_HDMI_OUT_END => true,
        r if (AFE_PROT_SIDEBAND0_MON..=AFE_DOMAIN_SIDEBAND9_MON).contains(&r) => true,
        r if (AFE_PCM0_INTF_CON1_MASK_MON..=AFE_ADDA_UL1_SRC_CON0_MASK_MON).contains(&r) => true,
        r if (AFE_IRQ_MCU_EN..=AFE_IRQ_MCU_DSP2_EN).contains(&r) => true,
        r if (AFE_IRQ0_MCU_CFG0..=AFE_IRQ26_MCU_CFG1).contains(&r) => true,
        _ => false,
    }
}

static mt8196_afe_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    volatile_reg: Some(mt8196_is_volatile_reg),
    max_register: AFE_MAX_REGISTER,
    num_reg_defaults_raw: AFE_MAX_REGISTER,
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn mt8196_afe_irq_handler(_irq_id: c_int, dev: *mut c_void) -> irqreturn_t {
    let afe = dev as *mut mtk_base_afe;
    let mut irq: *mut mtk_base_afe_irq;
    let mut status: u32 = 0;
    let mut status_mcu: u32;
    let mut mcu_en: u32 = 0;
    let mut cus_status: u32 = 0;
    let mut cus_status_mcu: u32;
    let mut cus_mcu_en: u32 = 0;
    let mut tmp_reg: u32 = 0;
    let ret: c_int;
    let cus_ret: c_int;
    let mut i: c_int;
    let mut ts64 = core::mem::MaybeUninit::<timespec64>::uninit();
    let t1: u64;
    let mut t2: u64;
    /* one interrupt period = 5ms */
    let timeout_limit: u64 = 5000000;

    /* get irq that is sent to MCU */
    regmap_read((*afe).regmap, AFE_IRQ_MCU_EN, &mut mcu_en);
    regmap_read((*afe).regmap, AFE_CUSTOM_IRQ_MCU_EN, &mut cus_mcu_en);

    ret = regmap_read((*afe).regmap, AFE_IRQ_MCU_STATUS, &mut status);
    cus_ret = regmap_read((*afe).regmap, AFE_CUSTOM_IRQ_MCU_STATUS, &mut cus_status);
    /* only care IRQ which is sent to MCU */
    status_mcu = status & mcu_en & AFE_IRQ_STATUS_BITS;
    cus_status_mcu = cus_status & cus_mcu_en & AFE_IRQ_STATUS_BITS;
    if (ret != 0 || status_mcu == 0) && (cus_ret != 0 || cus_status_mcu == 0) {
        dev_err((*afe).dev, b"ret %d, sat 0x%x, en 0x%x,csat 0x%x, cen 0x%x\n\0".as_ptr() as *const c_char, ret, status, mcu_en, cus_status_mcu, cus_mcu_en);
        return IRQ_NONE;
    }

    ktime_get_ts64(ts64.as_mut_ptr());
    t1 = ktime_get_ns();

    i = 0;
    while i < MT8196_MEMIF_NUM {
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

        if i == MT8196_MEMIF_HDMI {
            if (cus_status_mcu & BIT((*(*irq).irq_data).id)) != 0 {
                snd_pcm_period_elapsed((*memif).substream);
            }
        } else if (status_mcu & BIT((*(*irq).irq_data).id)) != 0 {
            snd_pcm_period_elapsed((*memif).substream);
        }
        i += 1;
    }

    ktime_get_ts64(ts64.as_mut_ptr());
    t2 = ktime_get_ns();
    t2 = t2 - t1; /* in ns (10^9) */

    if t2 > timeout_limit {
        dev_warn((*afe).dev, b"IRQ handler exceeded time limit by %llu ns\n\0".as_ptr() as *const c_char, t2 - timeout_limit);
    }

    /* clear irq */
    i = 0;
    while i < MT8196_IRQ_NUM {
        /* cus_status_mcu only bit0 is used for TDM */
        if (status_mcu & BIT(i)) != 0 || (cus_status_mcu & 0x1) != 0 {
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

unsafe extern "C" fn mt8196_afe_runtime_suspend(dev: *mut device) -> c_int {
    let afe = dev_get_drvdata(dev);
    let mut value: c_uint = 0;
    let mut tmp_reg: c_uint = 0;
    let mut ret: c_int;
    let mut i: c_int;

    if (*afe).regmap.is_null() {
        dev_err((*afe).dev, b"skip regmap\n\0".as_ptr() as *const c_char);
        mt8196_afe_disable_reg_rw_clk(afe);
        return 0;
    }

    /* disable AFE */
    mt8196_afe_disable_main_clock(afe);

    ret = regmap_read_poll_timeout(
        (*afe).regmap,
        AUDIO_ENGEN_CON0_MON,
        value,
        (value & AUDIO_ENGEN_MON_SFT) == 0,
        20,
        1 * 1000 * 1000,
    );
    dev_dbg((*afe).dev, b"read_poll ret %d\n\0".as_ptr() as *const c_char, ret);
    if ret != 0 {
        dev_warn((*afe).dev, b"ret %d\n\0".as_ptr() as *const c_char, ret);
    }

    /* make sure all irq status are cleared */
    i = 0;
    while i < MT8196_IRQ_NUM {
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

    mt8196_afe_disable_reg_rw_clk(afe);
    0
}

unsafe extern "C" fn mt8196_afe_runtime_resume(dev: *mut device) -> c_int {
    let afe = dev_get_drvdata(dev);
    let mut ret: c_int = 0;

    ret = mt8196_afe_enable_reg_rw_clk(afe);
    if ret != 0 {
        return ret;
    }

    if (*afe).regmap.is_null() {
        dev_warn((*afe).dev, b"skip regmap\n\0".as_ptr() as *const c_char);
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
    mt8196_afe_enable_main_clock(afe);

    0
}

unsafe extern "C" fn mt8196_afe_component_probe(component: *mut snd_soc_component) -> c_int {
    let afe = snd_soc_component_get_drvdata(component);
    let ret: c_int;

    /* enable clock for regcache get default value from hw */
    ret = pm_runtime_resume_and_get((*afe).dev);
    if ret != 0 {
        return dev_err_probe((*afe).dev, ret, b"failed to resume device\n\0".as_ptr() as *const c_char);
    }

    mtk_afe_add_sub_dai_control(component);
    pm_runtime_put_sync((*afe).dev);

    0
}

unsafe extern "C" fn mt8196_afe_pcm_open(
    _component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> c_int {
    /* set the wait_for_avail to 2 sec*/
    (*substream).wait_time = msecs_to_jiffies(2 * 1000);
    0
}

unsafe extern "C" fn mt8196_afe_pcm_free(_component: *mut snd_soc_component, pcm: *mut snd_pcm) {
    snd_pcm_lib_preallocate_free_for_all(pcm);
}

static mt8196_afe_component: snd_soc_component_driver = snd_soc_component_driver {
    name: AFE_PCM_NAME.as_ptr() as *const c_char,
    probe: Some(mt8196_afe_component_probe),
    pcm_new: Some(mtk_afe_pcm_new),
    pcm_free: Some(mt8196_afe_pcm_free),
    open: Some(mt8196_afe_pcm_open),
    pointer: Some(mtk_afe_pcm_pointer),
};

unsafe extern "C" fn mt8196_dai_memif_register(afe: *mut mtk_base_afe) -> c_int {
    let dai: *mut mtk_base_afe_dai;

    dai = devm_kzalloc((*afe).dev, core::mem::size_of::<mtk_base_afe_dai>(), GFP_KERNEL) as *mut mtk_base_afe_dai;
    if dai.is_null() {
        return -ENOMEM;
    }

    list_add(&mut (*dai).list, &mut (*afe).sub_dais);

    (*dai).dai_drivers = mt8196_memif_dai_driver.as_mut_ptr();
    (*dai).num_dai_drivers = mt8196_memif_dai_driver.len() as c_int;
    (*dai).dapm_widgets = mt8196_memif_widgets.as_ptr();
    (*dai).num_dapm_widgets = mt8196_memif_widgets.len() as c_int;
    (*dai).dapm_routes = mt8196_memif_routes.as_ptr();
    (*dai).num_dapm_routes = mt8196_memif_routes.len() as c_int;
    0
}

static dai_register_cbs: [dai_register_cb; 4] = [
    Some(mt8196_dai_adda_register),
    Some(mt8196_dai_i2s_register),
    Some(mt8196_dai_tdm_register),
    Some(mt8196_dai_memif_register),
];

static mt8196_cg_patch: [reg_sequence; 1] = [reg_sequence { reg: AUDIO_TOP_CON4 as c_uint, def: 0x361c }];

unsafe extern "C" fn mt8196_afe_release_reserved_mem(data: *mut c_void) {
    of_reserved_mem_device_release(data);
}

unsafe extern "C" fn mt8196_afe_pcm_dev_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int;
    let mut i: c_int;
    let mut tmp_reg: c_uint = 0;
    let irq_id: c_int;
    let afe: *mut mtk_base_afe;
    let mut afe_priv: *mut mt8196_afe_private;
    let dev = &mut (*pdev).dev as *mut device;

    ret = dma_set_mask_and_coherent(dev, DMA_BIT_MASK(34));
    if ret != 0 {
        return ret;
    }

    ret = of_reserved_mem_device_init(dev);
    if ret != 0 {
        dev_err(dev, b"failed to assign memory region: %d\n\0".as_ptr() as *const c_char, ret);
    } else {
        ret = devm_add_action_or_reset(dev, mt8196_afe_release_reserved_mem, dev as *mut c_void);
        if ret != 0 {
            return ret;
        }
    }

    afe = devm_kzalloc(dev, core::mem::size_of::<mtk_base_afe>(), GFP_KERNEL) as *mut mtk_base_afe;
    if afe.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, afe as *mut c_void);

    (*afe).platform_priv = devm_kzalloc(dev, core::mem::size_of::<mt8196_afe_private>(), GFP_KERNEL) as *mut mt8196_afe_private;
    if (*afe).platform_priv.is_null() {
        return -ENOMEM;
    }

    afe_priv = (*afe).platform_priv;
    (*afe).dev = dev;

    (*afe).base_addr = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR((*afe).base_addr) {
        return dev_err_probe(dev, PTR_ERR((*afe).base_addr), b"AFE base_addr not found\n\0".as_ptr() as *const c_char);
    }

    /* init audio related clock */
    ret = mt8196_init_clock(afe);
    if ret != 0 {
        return dev_err_probe(dev, ret, b"init clock error.\n\0".as_ptr() as *const c_char);
    }

    /* init memif */
    /* IPM2.0 no need banding */
    (*afe).memif_32bit_supported = 1;
    (*afe).memif_size = MT8196_MEMIF_NUM;
    (*afe).memif = devm_kcalloc(dev, (*afe).memif_size, core::mem::size_of::<mtk_base_afe_memif>(), GFP_KERNEL) as *mut mtk_base_afe_memif;

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

    mutex_init(&mut (*afe).irq_alloc_lock as *mut c_void);

    /* init irq */
    (*afe).irqs_size = MT8196_IRQ_NUM;
    (*afe).irqs = devm_kcalloc(dev, (*afe).irqs_size, core::mem::size_of::<mtk_base_afe_irq>(), GFP_KERNEL) as *mut mtk_base_afe_irq;

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
        return dev_err_probe(dev, irq_id, b"no irq found\0".as_ptr() as *const c_char);
    }

    ret = devm_request_irq(dev, irq_id, mt8196_afe_irq_handler, IRQF_TRIGGER_NONE, b"Afe_ISR_Handle\0".as_ptr() as *const c_char, afe as *mut c_void);
    if ret != 0 {
        return dev_err_probe(dev, ret, b"could not request_irq for Afe_ISR_Handle\n\0".as_ptr() as *const c_char);
    }

    /* init sub_dais */
    INIT_LIST_HEAD(&mut (*afe).sub_dais);

    i = 0;
    while i < dai_register_cbs.len() as c_int {
        ret = dai_register_cbs[i as usize].unwrap()(afe);
        if ret != 0 {
            return dev_err_probe(dev, ret, b"dai register i %d fail\n\0".as_ptr() as *const c_char, i);
        }
        i += 1;
    }

    /* init dai_driver and component_driver */
    ret = mtk_afe_combine_sub_dai(afe);
    if ret != 0 {
        return dev_err_probe(dev, ret, b"mtk_afe_combine_sub_dai fail\n\0".as_ptr() as *const c_char);
    }

    /* others */
    (*afe).mtk_afe_hardware = &mt8196_afe_hardware;
    (*afe).memif_fs = Some(mt8196_memif_fs);
    (*afe).irq_fs = Some(mt8196_irq_fs);
    (*afe).get_dai_fs = Some(mt8196_get_dai_fs);
    (*afe).get_memif_pbuf_size = Some(mt8196_get_memif_pbuf_size);

    (*afe).runtime_resume = Some(mt8196_afe_runtime_resume);
    (*afe).runtime_suspend = Some(mt8196_afe_runtime_suspend);

    ret = devm_pm_runtime_enable(dev);
    if ret != 0 {
        return ret;
    }

/*
 * Audio device is part of genpd. Registering it as a syscore device ensure
 * the proper power-on sequence of the AFE device.
 */
    dev_pm_syscore_device(dev, true);

    /* enable clock for regcache get default value from hw */
    ret = pm_runtime_resume_and_get(dev);
    if ret != 0 {
        return dev_err_probe(dev, ret, b"failed to resume device\n\0".as_ptr() as *const c_char);
    }

    (*afe).regmap = devm_regmap_init_mmio(dev, (*afe).base_addr, &mt8196_afe_regmap_config);
    if IS_ERR((*afe).regmap as *const c_void) {
        ret = PTR_ERR((*afe).regmap as *const c_void);
        pm_runtime_put_sync(dev);
        return ret;
    }

    ret = regmap_register_patch((*afe).regmap, mt8196_cg_patch.as_ptr(), mt8196_cg_patch.len() as c_int);
    if ret < 0 {
        dev_err(dev, b"Failed to apply cg patch\n\0".as_ptr() as *const c_char);
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
    ret = devm_snd_soc_register_component(dev, &mt8196_afe_component, (*afe).dai_drivers, (*afe).num_dai_drivers);
    if ret != 0 {
        dev_err(dev, b"afe component err\n\0".as_ptr() as *const c_char);
        return ret;
    }

    0
}

unsafe extern "C" fn mt8196_afe_pcm_dev_remove(pdev: *mut platform_device) {
    let afe = platform_get_drvdata(pdev);
    let dev = &mut (*pdev).dev as *mut device;

    if !pm_runtime_status_suspended(dev) {
        mt8196_afe_runtime_suspend(dev);
    }

    mt8196_afe_disable_main_clock(afe);
    /* disable afe clock */
    mt8196_afe_disable_reg_rw_clk(afe);
}

// Device tree match table, MODULE_DEVICE_TABLE, runtime PM ops, platform driver
// registration, and module metadata are translated as declarative Rust-side
// registration intent. The concrete Rust kernel binding types are external.
static mt8196_afe_pcm_dt_match: [of_device_id; 2] = [
    of_device_id { compatible: cstr!("mediatek,mt8196-afe") },
    of_device_id { compatible: ptr::null() },
];
MODULE_DEVICE_TABLE_OF(&mt8196_afe_pcm_dt_match);

static mt8196_afe_pm_ops: dev_pm_ops = SET_RUNTIME_PM_OPS(
    Some(mt8196_afe_runtime_suspend),
    Some(mt8196_afe_runtime_resume),
    None,
);

static mut mt8196_afe_pcm_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: cstr!("mt8196-afe"),
        of_match_table: mt8196_afe_pcm_dt_match.as_ptr(),
        pm: &mt8196_afe_pm_ops,
    },
    probe: Some(mt8196_afe_pcm_dev_probe),
    remove: Some(mt8196_afe_pcm_dev_remove),
};
module_platform_driver!(mt8196_afe_pcm_driver);

MODULE_DESCRIPTION!("Mediatek ALSA SoC AFE platform driver for 8196");
MODULE_AUTHOR!("Darren Ye <darren.ye@mediatek.com>");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
