// SPDX-License-Identifier: GPL-2.0
/*
 * mtk-afe-fe-dais.c  --  Mediatek afe fe dai operator
 *
 * Copyright (c) 2016 MediaTek Inc.
 * Author: Garlic Tseng <garlic.tseng@mediatek.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const AFE_BASE_END_OFFSET: c_int = 8;

type size_t = usize;
type dma_addr_t = u64;
type snd_pcm_format_t = c_int;
type bool_ = bool;

const SNDRV_PCM_STREAM_PLAYBACK: c_int = 0;
const SNDRV_PCM_STREAM_CAPTURE: c_int = 1;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIODS: c_int = 1;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_FORMAT_S16_LE: snd_pcm_format_t = 2;
const SNDRV_PCM_FORMAT_U16_LE: snd_pcm_format_t = 4;
const SNDRV_PCM_FORMAT_S24_LE: snd_pcm_format_t = 6;
const SNDRV_PCM_FORMAT_U24_LE: snd_pcm_format_t = 8;
const SNDRV_PCM_FORMAT_S32_LE: snd_pcm_format_t = 10;
const SNDRV_PCM_FORMAT_U32_LE: snd_pcm_format_t = 12;
const EBUSY: c_int = 16;
const EINVAL: c_int = 22;
const GFP_KERNEL: c_uint = 0;
const AFE_PCM_NAME: *const c_char = b"AFE_PCM\0".as_ptr() as *const c_char;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_addr: dma_addr_t,
    pub dma_area: *mut c_uchar,
    pub dma_bytes: size_t,
    pub period_size: c_uint,
    pub rate: c_uint,
}

type c_uchar = u8;

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
    pub stream: c_int,
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
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mutex {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub periods_max: c_uint,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub hw_free:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub prepare:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub trigger: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int,
    >,
}

#[repr(C)]
pub struct mtk_base_irq_data {
    pub irq_cnt_reg: c_int,
    pub irq_cnt_maskbit: c_uint,
    pub irq_cnt_shift: c_int,
    pub irq_fs_reg: c_int,
    pub irq_fs_maskbit: c_uint,
    pub irq_fs_shift: c_int,
    pub irq_en_reg: c_int,
    pub irq_en_shift: c_int,
    pub irq_clr_reg: c_int,
    pub irq_clr_shift: c_int,
}

#[repr(C)]
pub struct mtk_base_afe_irq {
    pub irq_data: *const mtk_base_irq_data,
    pub irq_occupyed: c_int,
}

#[repr(C)]
pub struct mtk_base_memif_data {
    pub name: *const c_char,
    pub agent_disable_reg: c_int,
    pub agent_disable_shift: c_int,
    pub enable_shift: c_int,
    pub enable_reg: c_int,
    pub reg_ofs_base: c_int,
    pub reg_ofs_end: c_int,
    pub reg_ofs_base_msb: c_int,
    pub reg_ofs_end_msb: c_int,
    pub msb_reg: c_int,
    pub msb_shift: c_int,
    pub msb_end_reg: c_int,
    pub msb_end_shift: c_int,
    pub mono_shift: c_int,
    pub quad_ch_mask: c_uint,
    pub quad_ch_reg: c_int,
    pub quad_ch_shift: c_int,
    pub mono_invert: bool_,
    pub int_odd_flag_reg: c_int,
    pub int_odd_flag_shift: c_int,
    pub mono_reg: c_int,
    pub fs_shift: c_int,
    pub fs_reg: c_int,
    pub fs_maskbit: c_uint,
    pub hd_reg: c_int,
    pub hd_shift: c_int,
    pub hd_align_reg: c_int,
    pub hd_align_mshift: c_int,
    pub pbuf_mask: c_uint,
    pub minlen_mask: c_uint,
    pub pbuf_reg: c_int,
    pub pbuf_shift: c_int,
    pub minlen_reg: c_int,
    pub minlen_shift: c_int,
}

#[repr(C)]
pub struct mtk_base_afe_memif {
    pub substream: *mut snd_pcm_substream,
    pub data: *const mtk_base_memif_data,
    pub irq_usage: c_int,
    pub const_irq: bool_,
    pub dma_area: *mut c_uchar,
    pub dma_addr: dma_addr_t,
    pub dma_bytes: size_t,
}

#[repr(C)]
pub struct mtk_base_afe {
    pub memif: *mut mtk_base_afe_memif,
    pub mtk_afe_hardware: *const snd_pcm_hardware,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub irqs_size: c_int,
    pub irqs: *mut mtk_base_afe_irq,
    pub irq_alloc_lock: mutex,
    pub request_dram_resource: Option<unsafe extern "C" fn(*mut device)>,
    pub release_dram_resource: Option<unsafe extern "C" fn(*mut device)>,
    pub irq_fs: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_uint) -> c_int>,
    pub get_memif_pbuf_size: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> c_int>,
    pub reg_back_up: *mut c_uint,
    pub reg_back_up_list_num: c_int,
    pub reg_back_up_list: *mut c_int,
    pub suspended: bool_,
    pub runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    pub memif_fs: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_uint) -> c_int>,
    pub memif_32bit_supported: bool_,
}

unsafe extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: c_int, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_int, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_int, val: *mut c_uint) -> c_int;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        step: c_uint,
    ) -> c_int;
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    );
    fn snd_pcm_hw_constraint_minmax(
        runtime: *mut snd_pcm_runtime,
        var: c_int,
        min: c_uint,
        max: c_uint,
    ) -> c_int;
    fn snd_pcm_hw_constraint_integer(runtime: *mut snd_pcm_runtime, var: c_int) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> snd_pcm_format_t;
    fn memset_io(addr: *mut c_void, val: c_int, count: size_t);
    fn snd_soc_rtdcom_lookup(
        rtd: *mut snd_soc_pcm_runtime,
        name: *const c_char,
    ) -> *mut snd_soc_component;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn pm_runtime_status_suspended(dev: *mut device) -> bool_;
    fn devm_kcalloc(dev: *mut device, n: size_t, size: size_t, flags: c_uint) -> *mut c_void;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
}

macro_rules! dev_err {
    ($($arg:tt)*) => {{}};
}

macro_rules! dev_dbg {
    ($($arg:tt)*) => {{}};
}

macro_rules! dev_warn {
    ($($arg:tt)*) => {{}};
}

macro_rules! WARN_ON_ONCE {
    ($cond:expr) => {
        $cond
    };
}

#[inline]
fn lower_32_bits(n: dma_addr_t) -> c_uint {
    n as c_uint
}

#[inline]
fn upper_32_bits(n: dma_addr_t) -> c_uint {
    (n >> 32) as c_uint
}

struct MutexGuard {
    lock: *mut mutex,
}

impl MutexGuard {
    unsafe fn new(lock: *mut mutex) -> Self {
        unsafe {
            mutex_lock(lock);
        }
        Self { lock }
    }
}

impl Drop for MutexGuard {
    fn drop(&mut self) {
        unsafe {
            mutex_unlock(self.lock);
        }
    }
}

unsafe fn mtk_regmap_update_bits(
    map: *mut regmap,
    reg: c_int,
    mask: c_uint,
    val: c_uint,
    shift: c_int,
) -> c_int {
    if reg < 0 || WARN_ON_ONCE!(shift < 0) {
        return 0;
    }
    unsafe { regmap_update_bits(map, reg, mask << shift, val << shift) }
}

unsafe fn mtk_regmap_write(map: *mut regmap, reg: c_int, val: c_uint) -> c_int {
    if reg < 0 {
        return 0;
    }
    unsafe { regmap_write(map, reg, val) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_afe_fe_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    let afe = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe };
    let runtime = unsafe { (*substream).runtime };
    let memif_num = unsafe { (*snd_soc_rtd_to_cpu(rtd, 0)).id };
    let memif = unsafe { (*afe).memif.add(memif_num as usize) };
    let mtk_afe_hardware = unsafe { (*afe).mtk_afe_hardware };
    let mut ret: c_int;

    unsafe {
        (*memif).substream = substream;
    }

    unsafe {
        snd_pcm_hw_constraint_step(
            (*substream).runtime,
            0,
            SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
            16,
        );
        /* enable agent */
        mtk_regmap_update_bits(
            (*afe).regmap,
            (*(*memif).data).agent_disable_reg,
            1,
            0,
            (*(*memif).data).agent_disable_shift,
        );

        snd_soc_set_runtime_hwparams(substream, mtk_afe_hardware);
    }

    /*
     * Capture cannot use ping-pong buffer since hw_ptr at IRQ may be
     * smaller than period_size due to AFE's internal buffer.
     * This easily leads to overrun when avail_min is period_size.
     * One more period can hold the possible unread buffer.
     */
    if unsafe { (*substream).stream == SNDRV_PCM_STREAM_CAPTURE } {
        let periods_max = unsafe { (*mtk_afe_hardware).periods_max };

        ret = unsafe {
            snd_pcm_hw_constraint_minmax(runtime, SNDRV_PCM_HW_PARAM_PERIODS, 3, periods_max)
        };
        if ret < 0 {
            dev_err!(
                (*afe).dev,
                "%s\0".as_ptr(),
                b"hw_constraint_minmax failed\n\0".as_ptr()
            );
            return ret;
        }
    }

    ret = unsafe { snd_pcm_hw_constraint_integer(runtime, SNDRV_PCM_HW_PARAM_PERIODS) };
    if ret < 0 {
        dev_err!((*afe).dev, "snd_pcm_hw_constraint_integer failed\n");
    }

    /* dynamic allocate irq to memif */
    if unsafe { (*memif).irq_usage < 0 } {
        let irq_id = unsafe { mtk_dynamic_irq_acquire(afe) };

        if unsafe { irq_id != (*afe).irqs_size } {
            /* link */
            unsafe {
                (*memif).irq_usage = irq_id;
            }
        } else {
            dev_err!((*afe).dev, "%s() error: no more asys irq\n", "__func__");
            ret = -EBUSY;
        }
    }
    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_afe_fe_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    let afe = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe };
    let memif =
        unsafe { (*afe).memif.add((*snd_soc_rtd_to_cpu(rtd, 0)).id as usize) };
    let irq_id: c_int;

    irq_id = unsafe { (*memif).irq_usage };

    unsafe {
        mtk_regmap_update_bits(
            (*afe).regmap,
            (*(*memif).data).agent_disable_reg,
            1,
            1,
            (*(*memif).data).agent_disable_shift,
        );
    }

    if unsafe { !(*memif).const_irq } {
        unsafe {
            mtk_dynamic_irq_release(afe, irq_id);
            (*memif).irq_usage = -1;
            (*memif).substream = ptr::null_mut();
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_afe_fe_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    let afe = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe };
    let id = unsafe { (*snd_soc_rtd_to_cpu(rtd, 0)).id };
    let memif = unsafe { (*afe).memif.add(id as usize) };
    let mut ret: c_int;
    let channels = unsafe { params_channels(params) };
    let rate = unsafe { params_rate(params) };
    let format = unsafe { params_format(params) };

    if unsafe { (*afe).request_dram_resource.is_some() } {
        unsafe { ((*afe).request_dram_resource.unwrap())((*afe).dev) };
    }

    dev_dbg!(
        (*afe).dev,
        "%s(), %s, ch %d, rate %d, fmt %d, dma_addr %pad, dma_area %p, dma_bytes 0x%zx\n",
        "__func__",
        (*(*memif).data).name,
        channels,
        rate,
        format,
        &mut (*(*substream).runtime).dma_addr,
        (*(*substream).runtime).dma_area,
        (*(*substream).runtime).dma_bytes
    );

    unsafe {
        memset_io(
            (*(*substream).runtime).dma_area as *mut c_void,
            0,
            (*(*substream).runtime).dma_bytes,
        );
    }

    /* set addr */
    ret = unsafe {
        mtk_memif_set_addr(
            afe,
            id,
            (*(*substream).runtime).dma_area,
            (*(*substream).runtime).dma_addr,
            (*(*substream).runtime).dma_bytes,
        )
    };
    if ret != 0 {
        dev_err!(
            (*afe).dev,
            "%s(), error, id %d, set addr, ret %d\n",
            "__func__",
            id,
            ret
        );
        return ret;
    }

    /* set channel */
    ret = unsafe { mtk_memif_set_channel(afe, id, channels) };
    if ret != 0 {
        dev_err!(
            (*afe).dev,
            "%s(), error, id %d, set channel %d, ret %d\n",
            "__func__",
            id,
            channels,
            ret
        );
        return ret;
    }

    /* set rate */
    ret = unsafe { mtk_memif_set_rate_substream(substream, id, rate) };
    if ret != 0 {
        dev_err!(
            (*afe).dev,
            "%s(), error, id %d, set rate %d, ret %d\n",
            "__func__",
            id,
            rate,
            ret
        );
        return ret;
    }

    /* set format */
    ret = unsafe { mtk_memif_set_format(afe, id, format) };
    if ret != 0 {
        dev_err!(
            (*afe).dev,
            "%s(), error, id %d, set format %d, ret %d\n",
            "__func__",
            id,
            format,
            ret
        );
        return ret;
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_afe_fe_hw_free(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let afe = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe };

    if unsafe { (*afe).release_dram_resource.is_some() } {
        unsafe { ((*afe).release_dram_resource.unwrap())((*afe).dev) };
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_afe_fe_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    let runtime = unsafe { (*substream).runtime };
    let afe = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe };
    let id = unsafe { (*snd_soc_rtd_to_cpu(rtd, 0)).id };
    let memif = unsafe { (*afe).memif.add(id as usize) };
    let irqs = unsafe { (*afe).irqs.add((*memif).irq_usage as usize) };
    let irq_data = unsafe { (*irqs).irq_data };
    let counter = unsafe { (*runtime).period_size };
    let mut fs: c_int;
    let mut ret: c_int;

    dev_dbg!((*afe).dev, "%s %s cmd=%d\n", "__func__", (*(*memif).data).name, cmd);

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME => {
            ret = unsafe { mtk_memif_set_enable(afe, id) };
            if ret != 0 {
                dev_err!(
                    (*afe).dev,
                    "%s(), error, id %d, memif enable, ret %d\n",
                    "__func__",
                    id,
                    ret
                );
                return ret;
            }

            /* set irq counter */
            unsafe {
                mtk_regmap_update_bits(
                    (*afe).regmap,
                    (*irq_data).irq_cnt_reg,
                    (*irq_data).irq_cnt_maskbit,
                    counter,
                    (*irq_data).irq_cnt_shift,
                );
            }

            /* set irq fs */
            fs = unsafe { ((*afe).irq_fs.unwrap())(substream, (*runtime).rate) };

            if fs < 0 {
                return -EINVAL;
            }

            unsafe {
                mtk_regmap_update_bits(
                    (*afe).regmap,
                    (*irq_data).irq_fs_reg,
                    (*irq_data).irq_fs_maskbit,
                    fs as c_uint,
                    (*irq_data).irq_fs_shift,
                );

                /* enable interrupt */
                mtk_regmap_update_bits(
                    (*afe).regmap,
                    (*irq_data).irq_en_reg,
                    1,
                    1,
                    (*irq_data).irq_en_shift,
                );
            }

            0
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND => {
            ret = unsafe { mtk_memif_set_disable(afe, id) };
            if ret != 0 {
                dev_err!(
                    (*afe).dev,
                    "%s(), error, id %d, memif enable, ret %d\n",
                    "__func__",
                    id,
                    ret
                );
            }

            unsafe {
                /* disable interrupt */
                mtk_regmap_update_bits(
                    (*afe).regmap,
                    (*irq_data).irq_en_reg,
                    1,
                    0,
                    (*irq_data).irq_en_shift,
                );
                /* and clear pending IRQ */
                mtk_regmap_write(
                    (*afe).regmap,
                    (*irq_data).irq_clr_reg,
                    1u32 << (*irq_data).irq_clr_shift,
                );
            }
            ret
        }
        _ => -EINVAL,
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_afe_fe_prepare(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    let afe = unsafe { snd_soc_dai_get_drvdata(dai) as *mut mtk_base_afe };
    let id = unsafe { (*snd_soc_rtd_to_cpu(rtd, 0)).id };
    let pbuf_size: c_int;

    if unsafe { (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK } {
        if unsafe { (*afe).get_memif_pbuf_size.is_some() } {
            pbuf_size = unsafe { ((*afe).get_memif_pbuf_size.unwrap())(substream) };
            unsafe {
                mtk_memif_set_pbuf_size(afe, id, pbuf_size);
            }
        }
    }
    0
}

#[unsafe(no_mangle)]
pub static mtk_afe_fe_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(mtk_afe_fe_startup),
    shutdown: Some(mtk_afe_fe_shutdown),
    hw_params: Some(mtk_afe_fe_hw_params),
    hw_free: Some(mtk_afe_fe_hw_free),
    prepare: Some(mtk_afe_fe_prepare),
    trigger: Some(mtk_afe_fe_trigger),
};

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_dynamic_irq_acquire(afe: *mut mtk_base_afe) -> c_int {
    let _guard = unsafe { MutexGuard::new(&mut (*afe).irq_alloc_lock) };
    let mut i: c_int = 0;

    while unsafe { i < (*afe).irqs_size } {
        if unsafe { (*(*afe).irqs.add(i as usize)).irq_occupyed == 0 } {
            unsafe {
                (*(*afe).irqs.add(i as usize)).irq_occupyed = 1;
            }
            return i;
        }
        i += 1;
    }
    unsafe { (*afe).irqs_size }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_dynamic_irq_release(
    afe: *mut mtk_base_afe,
    irq_id: c_int,
) -> c_int {
    let _guard = unsafe { MutexGuard::new(&mut (*afe).irq_alloc_lock) };
    if unsafe { irq_id >= 0 && irq_id < (*afe).irqs_size } {
        unsafe {
            (*(*afe).irqs.add(irq_id as usize)).irq_occupyed = 0;
        }
        return 0;
    }
    -EINVAL
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_afe_suspend(component: *mut snd_soc_component) -> c_int {
    let afe = unsafe { snd_soc_component_get_drvdata(component) as *mut mtk_base_afe };
    let dev = unsafe { (*afe).dev };
    let regmap = unsafe { (*afe).regmap };
    let mut i: c_int;

    if unsafe { pm_runtime_status_suspended(dev) || (*afe).suspended } {
        return 0;
    }

    if unsafe { (*afe).reg_back_up.is_null() } {
        unsafe {
            (*afe).reg_back_up = devm_kcalloc(
                dev,
                (*afe).reg_back_up_list_num as size_t,
                size_of::<c_uint>(),
                GFP_KERNEL,
            ) as *mut c_uint;
        }
    }

    if unsafe { !(*afe).reg_back_up.is_null() } {
        i = 0;
        while unsafe { i < (*afe).reg_back_up_list_num } {
            unsafe {
                regmap_read(
                    regmap,
                    *(*afe).reg_back_up_list.add(i as usize),
                    (*afe).reg_back_up.add(i as usize),
                );
            }
            i += 1;
        }
    }

    unsafe {
        (*afe).suspended = true;
        ((*afe).runtime_suspend.unwrap())(dev);
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_afe_resume(component: *mut snd_soc_component) -> c_int {
    let afe = unsafe { snd_soc_component_get_drvdata(component) as *mut mtk_base_afe };
    let dev = unsafe { (*afe).dev };
    let regmap = unsafe { (*afe).regmap };
    let mut i: c_int;

    if unsafe { pm_runtime_status_suspended(dev) || !(*afe).suspended } {
        return 0;
    }

    unsafe {
        ((*afe).runtime_resume.unwrap())(dev);
    }

    if unsafe { (*afe).reg_back_up.is_null() } {
        dev_dbg!(dev, "%s no reg_backup\n", "__func__");
    } else {
        i = 0;
        while unsafe { i < (*afe).reg_back_up_list_num } {
            unsafe {
                mtk_regmap_write(
                    regmap,
                    *(*afe).reg_back_up_list.add(i as usize),
                    *(*afe).reg_back_up.add(i as usize),
                );
            }
            i += 1;
        }
    }

    unsafe {
        (*afe).suspended = false;
    }
    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_memif_set_enable(afe: *mut mtk_base_afe, id: c_int) -> c_int {
    let memif = unsafe { (*afe).memif.add(id as usize) };

    if unsafe { (*(*memif).data).enable_shift < 0 } {
        dev_warn!((*afe).dev, "%s(), error, id %d, enable_shift < 0\n", "__func__", id);
        return 0;
    }
    unsafe {
        mtk_regmap_update_bits(
            (*afe).regmap,
            (*(*memif).data).enable_reg,
            1,
            1,
            (*(*memif).data).enable_shift,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_memif_set_disable(afe: *mut mtk_base_afe, id: c_int) -> c_int {
    let memif = unsafe { (*afe).memif.add(id as usize) };

    if unsafe { (*(*memif).data).enable_shift < 0 } {
        dev_warn!((*afe).dev, "%s(), error, id %d, enable_shift < 0\n", "__func__", id);
        return 0;
    }
    unsafe {
        mtk_regmap_update_bits(
            (*afe).regmap,
            (*(*memif).data).enable_reg,
            1,
            0,
            (*(*memif).data).enable_shift,
        )
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_memif_set_addr(
    afe: *mut mtk_base_afe,
    id: c_int,
    dma_area: *mut c_uchar,
    dma_addr: dma_addr_t,
    dma_bytes: size_t,
) -> c_int {
    let memif = unsafe { (*afe).memif.add(id as usize) };
    let msb_at_bit33: c_int = if upper_32_bits(dma_addr) != 0 { 1 } else { 0 };
    let phys_buf_addr = lower_32_bits(dma_addr);
    let phys_buf_addr_upper_32 = upper_32_bits(dma_addr);

    unsafe {
        (*memif).dma_area = dma_area;
        (*memif).dma_addr = dma_addr;
        (*memif).dma_bytes = dma_bytes;

        /* start */
        mtk_regmap_write((*afe).regmap, (*(*memif).data).reg_ofs_base, phys_buf_addr);
    }
    /* end */
    if unsafe { (*(*memif).data).reg_ofs_end != 0 } {
        unsafe {
            mtk_regmap_write(
                (*afe).regmap,
                (*(*memif).data).reg_ofs_end,
                phys_buf_addr.wrapping_add(dma_bytes as c_uint).wrapping_sub(1),
            );
        }
    } else {
        unsafe {
            mtk_regmap_write(
                (*afe).regmap,
                (*(*memif).data).reg_ofs_base + AFE_BASE_END_OFFSET,
                phys_buf_addr.wrapping_add(dma_bytes as c_uint).wrapping_sub(1),
            );
        }
    }

    /* set start, end, upper 32 bits */
    if unsafe { (*(*memif).data).reg_ofs_base_msb != 0 } {
        unsafe {
            mtk_regmap_write(
                (*afe).regmap,
                (*(*memif).data).reg_ofs_base_msb,
                phys_buf_addr_upper_32,
            );
            mtk_regmap_write(
                (*afe).regmap,
                (*(*memif).data).reg_ofs_end_msb,
                phys_buf_addr_upper_32,
            );
        }
    }

    /*
     * set MSB to 33-bit, for memif address
     * only for memif base address, if msb_end_reg exists
     */
    if unsafe { (*(*memif).data).msb_reg != 0 } {
        unsafe {
            mtk_regmap_update_bits(
                (*afe).regmap,
                (*(*memif).data).msb_reg,
                1,
                msb_at_bit33 as c_uint,
                (*(*memif).data).msb_shift,
            );
        }
    }

    /* set MSB to 33-bit, for memif end address */
    if unsafe { (*(*memif).data).msb_end_reg != 0 } {
        unsafe {
            mtk_regmap_update_bits(
                (*afe).regmap,
                (*(*memif).data).msb_end_reg,
                1,
                msb_at_bit33 as c_uint,
                (*(*memif).data).msb_end_shift,
            );
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_memif_set_channel(
    afe: *mut mtk_base_afe,
    id: c_int,
    channel: c_uint,
) -> c_int {
    let memif = unsafe { (*afe).memif.add(id as usize) };
    let mono: c_uint;

    if unsafe { (*(*memif).data).mono_shift < 0 } {
        return 0;
    }

    if unsafe { (*(*memif).data).quad_ch_mask != 0 } {
        let quad_ch: c_uint = if channel == 4 { 1 } else { 0 };

        unsafe {
            mtk_regmap_update_bits(
                (*afe).regmap,
                (*(*memif).data).quad_ch_reg,
                (*(*memif).data).quad_ch_mask,
                quad_ch,
                (*(*memif).data).quad_ch_shift,
            );
        }
    }

    if unsafe { (*(*memif).data).mono_invert } {
        mono = if channel == 1 { 0 } else { 1 };
    } else {
        mono = if channel == 1 { 1 } else { 0 };
    }

    /* for specific configuration of memif mono mode */
    if unsafe { (*(*memif).data).int_odd_flag_reg != 0 } {
        unsafe {
            mtk_regmap_update_bits(
                (*afe).regmap,
                (*(*memif).data).int_odd_flag_reg,
                1,
                mono,
                (*(*memif).data).int_odd_flag_shift,
            );
        }
    }

    unsafe {
        mtk_regmap_update_bits(
            (*afe).regmap,
            (*(*memif).data).mono_reg,
            1,
            mono,
            (*(*memif).data).mono_shift,
        )
    }
}

unsafe fn mtk_memif_set_rate_fs(afe: *mut mtk_base_afe, id: c_int, fs: c_int) -> c_int {
    let memif = unsafe { (*afe).memif.add(id as usize) };

    if unsafe { (*(*memif).data).fs_shift >= 0 } {
        unsafe {
            mtk_regmap_update_bits(
                (*afe).regmap,
                (*(*memif).data).fs_reg,
                (*(*memif).data).fs_maskbit,
                fs as c_uint,
                (*(*memif).data).fs_shift,
            );
        }
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_memif_set_rate_substream(
    substream: *mut snd_pcm_substream,
    id: c_int,
    rate: c_uint,
) -> c_int {
    let rtd = unsafe { snd_soc_substream_to_rtd(substream) };
    let component = unsafe { snd_soc_rtdcom_lookup(rtd, AFE_PCM_NAME) };
    let afe = unsafe { snd_soc_component_get_drvdata(component) as *mut mtk_base_afe };

    let mut fs: c_int = 0;

    if unsafe { (*afe).memif_fs.is_none() } {
        dev_err!((*afe).dev, "%s(), error, afe->memif_fs == NULL\n", "__func__");
        return -EINVAL;
    }

    fs = unsafe { ((*afe).memif_fs.unwrap())(substream, rate) };

    if fs < 0 {
        return -EINVAL;
    }

    unsafe { mtk_memif_set_rate_fs(afe, id, fs) }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_memif_set_format(
    afe: *mut mtk_base_afe,
    id: c_int,
    format: snd_pcm_format_t,
) -> c_int {
    let memif = unsafe { (*afe).memif.add(id as usize) };
    let mut hd_audio: c_int = 0;
    let mut hd_align: c_int = 0;

    /* set hd mode */
    match format {
        SNDRV_PCM_FORMAT_S16_LE | SNDRV_PCM_FORMAT_U16_LE => {
            hd_audio = 0;
        }
        SNDRV_PCM_FORMAT_S32_LE | SNDRV_PCM_FORMAT_U32_LE => {
            if unsafe { (*afe).memif_32bit_supported } {
                hd_audio = 2;
                hd_align = 0;
            } else {
                hd_audio = 1;
                hd_align = 1;
            }
        }
        SNDRV_PCM_FORMAT_S24_LE | SNDRV_PCM_FORMAT_U24_LE => {
            hd_audio = 1;
        }
        _ => {
            dev_err!(
                (*afe).dev,
                "%s() error: unsupported format %d\n",
                "__func__",
                format
            );
        }
    }

    unsafe {
        mtk_regmap_update_bits(
            (*afe).regmap,
            (*(*memif).data).hd_reg,
            0x3,
            hd_audio as c_uint,
            (*(*memif).data).hd_shift,
        );

        mtk_regmap_update_bits(
            (*afe).regmap,
            (*(*memif).data).hd_align_reg,
            0x1,
            hd_align as c_uint,
            (*(*memif).data).hd_align_mshift,
        );
    }

    0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn mtk_memif_set_pbuf_size(
    afe: *mut mtk_base_afe,
    id: c_int,
    pbuf_size: c_int,
) -> c_int {
    let memif_data = unsafe { (*(*afe).memif.add(id as usize)).data };

    if unsafe { (*memif_data).pbuf_mask == 0 || (*memif_data).minlen_mask == 0 } {
        return 0;
    }

    unsafe {
        mtk_regmap_update_bits(
            (*afe).regmap,
            (*memif_data).pbuf_reg,
            (*memif_data).pbuf_mask,
            pbuf_size as c_uint,
            (*memif_data).pbuf_shift,
        );

        mtk_regmap_update_bits(
            (*afe).regmap,
            (*memif_data).minlen_reg,
            (*memif_data).minlen_mask,
            pbuf_size as c_uint,
            (*memif_data).minlen_shift,
        );
    }
    0
}

/* MODULE_DESCRIPTION("Mediatek simple fe dai operator"); */
/* MODULE_AUTHOR("Garlic Tseng <garlic.tseng@mediatek.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
