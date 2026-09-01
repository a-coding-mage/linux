// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// C dependencies:
// linux/bitfield.h, linux/clk.h, linux/dma-mapping.h
// sound/pcm_params.h, sound/soc.h, sound/soc-dai.h
// "aiu-fifo.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_int, c_uint, c_void};

type snd_pcm_uframes_t = usize;
type dma_addr_t = usize;
type size_t = usize;
type irqreturn_t = c_int;

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const IRQ_HANDLED: irqreturn_t = 1;

const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_int = 0;
const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_int = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;

const fn BIT(nr: u32) -> c_uint {
    1u32 << nr
}

const fn GENMASK(h: u32, l: u32) -> c_uint {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const fn field_shift(mask: c_uint) -> u32 {
    mask.trailing_zeros()
}

const fn FIELD_PREP(mask: c_uint, val: c_uint) -> c_uint {
    (val << field_shift(mask)) & mask
}

const fn DMA_BIT_MASK(n: u32) -> u64 {
    if n == 64 {
        !0u64
    } else {
        (1u64 << n) - 1
    }
}

const AIU_MEM_START: c_uint = 0x00;
const AIU_MEM_RD: c_uint = 0x04;
const AIU_MEM_END: c_uint = 0x08;
const AIU_MEM_MASKS: c_uint = 0x0c;
const AIU_MEM_MASK_CH_RD: c_uint = GENMASK(7, 0);
const AIU_MEM_MASK_CH_MEM: c_uint = GENMASK(15, 8);
const AIU_MEM_CONTROL: c_uint = 0x10;
const AIU_MEM_CONTROL_INIT: c_uint = BIT(0);
const AIU_MEM_CONTROL_FILL_EN: c_uint = BIT(1);
const AIU_MEM_CONTROL_EMPTY_EN: c_uint = BIT(2);

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hardware {
    pub buffer_bytes_max: size_t,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_addr: dma_addr_t,
    pub dma_bytes: size_t,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
    pub dev: *mut device,
}

#[repr(C)]
pub struct aiu_fifo {
    pub mem_offset: c_uint,
    pub fifo_block: dma_addr_t,
    pub pclk: *mut clk,
    pub irq: c_uint,
    pub pcm: *const snd_pcm_hardware,
}

unsafe extern "C" {
    fn snd_soc_substream_to_rtd(ss: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_dma_data_get_playback(dai: *mut snd_soc_dai) -> *mut aiu_fifo;
    fn snd_soc_dai_dma_data_set_playback(dai: *mut snd_soc_dai, data: *mut aiu_fifo);
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: c_uint) -> c_uint;
    fn snd_soc_component_update_bits(
        component: *mut snd_soc_component,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn snd_soc_component_write(component: *mut snd_soc_component, reg: c_uint, val: dma_addr_t);
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn snd_pcm_period_elapsed(substream: *mut snd_pcm_substream);
    fn snd_soc_set_runtime_hwparams(
        substream: *mut snd_pcm_substream,
        hw: *const snd_pcm_hardware,
    );
    fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        step: dma_addr_t,
    ) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn request_irq(
        irq: c_uint,
        handler: unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t,
        flags: c_uint,
        name: *const i8,
        dev: *mut c_void,
    ) -> c_int;
    fn free_irq(irq: c_uint, dev_id: *mut c_void);
    fn dev_name(dev: *mut device) -> *const i8;
    fn dma_coerce_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn snd_pcm_set_managed_buffer_all(
        pcm: *mut snd_pcm,
        type_: c_int,
        dev: *mut device,
        size: size_t,
        max: size_t,
    );
    fn kzalloc_obj(size: size_t) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
}

unsafe fn aiu_fifo_dai(ss: *mut snd_pcm_substream) -> *mut snd_soc_dai {
    let rtd = snd_soc_substream_to_rtd(ss);

    snd_soc_rtd_to_cpu(rtd, 0)
}

#[no_mangle]
pub unsafe extern "C" fn aiu_fifo_pointer(
    component: *mut snd_soc_component,
    substream: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let dai = aiu_fifo_dai(substream);
    let fifo = snd_soc_dai_dma_data_get_playback(dai);
    let runtime = (*substream).runtime;
    let addr: c_uint;

    addr = snd_soc_component_read(component, (*fifo).mem_offset + AIU_MEM_RD);

    bytes_to_frames(runtime, addr.wrapping_sub((*runtime).dma_addr as c_uint))
}

unsafe fn aiu_fifo_enable(dai: *mut snd_soc_dai, enable: bool) {
    let component = (*dai).component;
    let fifo = snd_soc_dai_dma_data_get_playback(dai);
    let en_mask = AIU_MEM_CONTROL_FILL_EN | AIU_MEM_CONTROL_EMPTY_EN;

    snd_soc_component_update_bits(
        component,
        (*fifo).mem_offset + AIU_MEM_CONTROL,
        en_mask,
        if enable { en_mask } else { 0 },
    );
}

#[no_mangle]
pub unsafe extern "C" fn aiu_fifo_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    match cmd {
        SNDRV_PCM_TRIGGER_START
        | SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            aiu_fifo_enable(dai, true);
        }
        SNDRV_PCM_TRIGGER_SUSPEND
        | SNDRV_PCM_TRIGGER_PAUSE_PUSH
        | SNDRV_PCM_TRIGGER_STOP => {
            aiu_fifo_enable(dai, false);
        }
        _ => {
            return -EINVAL;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn aiu_fifo_prepare(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let fifo = snd_soc_dai_dma_data_get_playback(dai);

    snd_soc_component_update_bits(
        component,
        (*fifo).mem_offset + AIU_MEM_CONTROL,
        AIU_MEM_CONTROL_INIT,
        AIU_MEM_CONTROL_INIT,
    );
    snd_soc_component_update_bits(
        component,
        (*fifo).mem_offset + AIU_MEM_CONTROL,
        AIU_MEM_CONTROL_INIT,
        0,
    );
    0
}

#[no_mangle]
pub unsafe extern "C" fn aiu_fifo_hw_params(
    substream: *mut snd_pcm_substream,
    _params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let runtime = (*substream).runtime;
    let component = (*dai).component;
    let fifo = snd_soc_dai_dma_data_get_playback(dai);
    let end: dma_addr_t;

    /* Setup the fifo boundaries */
    end = (*runtime)
        .dma_addr
        .wrapping_add((*runtime).dma_bytes)
        .wrapping_sub((*fifo).fifo_block);
    snd_soc_component_write(
        component,
        (*fifo).mem_offset + AIU_MEM_START,
        (*runtime).dma_addr,
    );
    snd_soc_component_write(
        component,
        (*fifo).mem_offset + AIU_MEM_RD,
        (*runtime).dma_addr,
    );
    snd_soc_component_write(component, (*fifo).mem_offset + AIU_MEM_END, end);

    /* Setup the fifo to read all the memory - no skip */
    snd_soc_component_update_bits(
        component,
        (*fifo).mem_offset + AIU_MEM_MASKS,
        AIU_MEM_MASK_CH_RD | AIU_MEM_MASK_CH_MEM,
        FIELD_PREP(AIU_MEM_MASK_CH_RD, 0xff) | FIELD_PREP(AIU_MEM_MASK_CH_MEM, 0xff),
    );

    0
}

unsafe extern "C" fn aiu_fifo_isr(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let playback = dev_id as *mut snd_pcm_substream;

    snd_pcm_period_elapsed(playback);

    IRQ_HANDLED
}

#[no_mangle]
pub unsafe extern "C" fn aiu_fifo_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let fifo = snd_soc_dai_dma_data_get_playback(dai);
    let mut ret: c_int;

    snd_soc_set_runtime_hwparams(substream, (*fifo).pcm);

    /*
     * Make sure the buffer and period size are multiple of the fifo burst
     * size
     */
    ret = snd_pcm_hw_constraint_step(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
        (*fifo).fifo_block,
    );
    if ret != 0 {
        return ret;
    }

    ret = snd_pcm_hw_constraint_step(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_PERIOD_BYTES,
        (*fifo).fifo_block,
    );
    if ret != 0 {
        return ret;
    }

    ret = clk_prepare_enable((*fifo).pclk);
    if ret != 0 {
        return ret;
    }

    ret = request_irq(
        (*fifo).irq,
        aiu_fifo_isr,
        0,
        dev_name((*dai).dev),
        substream as *mut c_void,
    );
    if ret != 0 {
        clk_disable_unprepare((*fifo).pclk);
    }

    ret
}

#[no_mangle]
pub unsafe extern "C" fn aiu_fifo_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let fifo = snd_soc_dai_dma_data_get_playback(dai);

    free_irq((*fifo).irq, substream as *mut c_void);
    clk_disable_unprepare((*fifo).pclk);
}

#[no_mangle]
pub unsafe extern "C" fn aiu_fifo_pcm_new(
    rtd: *mut snd_soc_pcm_runtime,
    dai: *mut snd_soc_dai,
) -> c_int {
    let card = (*(*rtd).card).snd_card;
    let fifo = snd_soc_dai_dma_data_get_playback(dai);
    let size: size_t = (*(*fifo).pcm).buffer_bytes_max;
    let ret: c_int;

    ret = dma_coerce_mask_and_coherent((*card).dev, DMA_BIT_MASK(32));
    if ret != 0 {
        return ret;
    }

    snd_pcm_set_managed_buffer_all((*rtd).pcm, SNDRV_DMA_TYPE_DEV, (*card).dev, size, size);

    0
}

#[no_mangle]
pub unsafe extern "C" fn aiu_fifo_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let fifo: *mut aiu_fifo;

    fifo = kzalloc_obj(core::mem::size_of::<aiu_fifo>()) as *mut aiu_fifo;
    if fifo.is_null() {
        return -ENOMEM;
    }

    snd_soc_dai_dma_data_set_playback(dai, fifo);

    0
}

#[no_mangle]
pub unsafe extern "C" fn aiu_fifo_dai_remove(dai: *mut snd_soc_dai) -> c_int {
    let fifo = snd_soc_dai_dma_data_get_playback(dai);

    kfree(fifo as *mut c_void);

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
