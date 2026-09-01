// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// C dependencies translated as external items:
// linux/bitfield.h, linux/clk.h, linux/of_irq.h, linux/of_platform.h,
// linux/module.h, linux/regmap.h, linux/reset.h, sound/pcm_params.h,
// sound/soc.h, sound/soc-dai.h, and "axg-fifo.h".

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

type bool_ = bool;
type size_t = usize;
type u8 = u8;
type dma_addr_t = usize;
type snd_pcm_uframes_t = c_ulong;
type irqreturn_t = c_uint;

#[repr(C)]
pub struct snd_pcm_hardware {
    pub info: c_uint,
    pub formats: u64,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub period_bytes_min: size_t,
    pub period_bytes_max: size_t,
    pub periods_min: c_uint,
    pub periods_max: c_uint,
    pub buffer_bytes_max: size_t,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub reg_stride: c_int,
    pub max_register: c_uint,
}

#[repr(C)]
pub struct snd_soc_component {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
    pub pcm: *mut snd_pcm,
}

#[repr(C)]
pub struct snd_soc_card {
    pub snd_card: *mut snd_card,
}

#[repr(C)]
pub struct snd_card {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_pcm {
    pub streams: [snd_pcm_str; 2],
}

#[repr(C)]
pub struct snd_pcm_str {
    pub substream: *mut snd_pcm_substream,
}

#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    pub dma_addr: dma_addr_t,
    pub dma_bytes: size_t,
    pub no_period_wakeup: bool_,
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct axg_fifo {
    pub map: *mut regmap,
    pub field_threshold: *mut regmap_field,
    pub pclk: *mut clk,
    pub arb: *mut reset_control,
    pub irq: c_int,
    pub depth: c_uint,
}

#[repr(C)]
pub struct axg_fifo_match_data {
    pub field_threshold: reg_field,
    pub component_drv: *const snd_soc_component_driver,
    pub dai_drv: *const snd_soc_dai_driver,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct reg_field {
    pub reg: c_uint,
    pub lsb: c_uint,
    pub msb: c_uint,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_field {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_component_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    _private: [u8; 0],
}

extern "C" {
    static axg_fifo_hw_formats: u64;

    fn snd_soc_substream_to_rtd(ss: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_rtd_to_cpu(rtd: *mut snd_soc_pcm_runtime, num: c_int) -> *mut snd_soc_dai;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: dma_addr_t) -> c_int;
    fn regmap_field_write(field: *mut regmap_field, val: c_uint) -> c_int;
    fn bytes_to_frames(runtime: *mut snd_pcm_runtime, bytes: c_uint) -> snd_pcm_uframes_t;
    fn params_period_bytes(params: *mut snd_pcm_hw_params) -> c_uint;
    fn snd_pcm_period_elapsed(ss: *mut snd_pcm_substream);
    fn snd_soc_set_runtime_hwparams(ss: *mut snd_pcm_substream, hw: *const snd_pcm_hardware);
    fn snd_pcm_hw_constraint_step(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_uint,
        step: c_uint,
    ) -> c_int;
    fn request_threaded_irq(
        irq: c_uint,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        thread_fn: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        irqflags: c_ulong,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn free_irq(irq: c_uint, dev_id: *mut c_void);
    fn dev_name(dev: *mut device) -> *const c_char;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn reset_control_deassert(rstc: *mut reset_control) -> c_int;
    fn reset_control_assert(rstc: *mut reset_control) -> c_int;
    fn snd_pcm_set_managed_buffer(
        substream: *mut snd_pcm_substream,
        type_: c_int,
        data: *mut device,
        size: size_t,
        max: size_t,
    );
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: size_t, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_reset_control_get_exclusive(dev: *mut device, id: *const c_char) -> *mut reset_control;
    fn of_irq_get(dev: *mut device_node, index: c_int) -> c_int;
    fn devm_regmap_field_alloc(
        dev: *mut device,
        regmap: *mut regmap,
        field: reg_field,
    ) -> *mut regmap_field;
    fn of_property_read_u32(np: *mut device_node, propname: *const c_char, out_value: *mut c_uint) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *const snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_dbg(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
}

type c_long = isize;

const UINT_MAX: c_uint = c_uint::MAX;
const EINVAL: c_int = 22;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;

const SNDRV_PCM_INFO_INTERLEAVED: c_uint = 1 << 0;
const SNDRV_PCM_INFO_MMAP: c_uint = 1 << 1;
const SNDRV_PCM_INFO_MMAP_VALID: c_uint = 1 << 2;
const SNDRV_PCM_INFO_BLOCK_TRANSFER: c_uint = 1 << 3;
const SNDRV_PCM_INFO_PAUSE: c_uint = 1 << 4;
const SNDRV_PCM_INFO_NO_PERIOD_WAKEUP: c_uint = 1 << 5;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 2;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 4;
const SNDRV_PCM_TRIGGER_STOP: c_int = 5;
const SNDRV_PCM_HW_PARAM_BUFFER_BYTES: c_uint = 0;
const SNDRV_PCM_HW_PARAM_PERIOD_BYTES: c_uint = 1;
const SNDRV_DMA_TYPE_DEV: c_int = 0;

const IRQF_ONESHOT: c_ulong = 1 << 0;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;

const AXG_FIFO_FORMATS: u64 = unsafe { axg_fifo_hw_formats };
const AXG_FIFO_CH_MAX: c_uint = 128;
const AXG_FIFO_BURST: c_uint = 8;
const FIFO_CTRL0: c_uint = 0x00;
const FIFO_CTRL1: c_uint = 0x04;
const FIFO_CTRL2: c_uint = 0x08;
const FIFO_START_ADDR: c_uint = 0x0c;
const FIFO_FINISH_ADDR: c_uint = 0x10;
const FIFO_INT_ADDR: c_uint = 0x14;
const FIFO_STATUS1: c_uint = 0x18;
const FIFO_STATUS2: c_uint = 0x1c;
const FIFO_INIT_ADDR: c_uint = 0x24;
const CTRL0_DMA_EN: c_uint = 1 << 31;
const CTRL0_INT_EN: c_uint = 0xff << 0;
const CTRL1_INT_CLR: c_uint = 0xff << 0;
const CTRL1_STATUS2_SEL: c_uint = 0x7 << 8;
const STATUS1_INT_STS: c_uint = 0xff << 0;
const STATUS2_SEL_DDR_READ: c_uint = 0;
const FIFO_INT_COUNT_REPEAT: c_uint = 1 << 0;
const FIFO_INT_MASK: u8 = 0xff;

unsafe fn field_prep(mask: c_uint, val: c_uint) -> c_uint {
    let shift = mask.trailing_zeros();
    (val << shift) & mask
}

unsafe fn field_get(mask: c_uint, val: c_uint) -> c_uint {
    let shift = mask.trailing_zeros();
    (val & mask) >> shift
}

unsafe fn is_err<T>(ptr: *mut T) -> bool {
    let value = ptr as isize;
    value >= -4095 && value < 0
}

unsafe fn ptr_err<T>(ptr: *mut T) -> c_long {
    ptr as c_long
}

/*
 * This file implements the platform operations common to the playback and
 * capture frontend DAI. The logic behind this two types of fifo is very
 * similar but some difference exist.
 * These differences are handled in the respective DAI drivers
 */
static axg_fifo_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_BLOCK_TRANSFER
        | SNDRV_PCM_INFO_PAUSE
        | SNDRV_PCM_INFO_NO_PERIOD_WAKEUP,
    formats: AXG_FIFO_FORMATS,
    rate_min: 5512,
    rate_max: 768000,
    channels_min: 1,
    channels_max: AXG_FIFO_CH_MAX,
    period_bytes_min: AXG_FIFO_BURST as size_t,
    period_bytes_max: UINT_MAX as size_t,
    periods_min: 2,
    periods_max: UINT_MAX,

    /* No real justification for this */
    buffer_bytes_max: 1 * 1024 * 1024,
};

unsafe fn axg_fifo_dai(ss: *mut snd_pcm_substream) -> *mut snd_soc_dai {
    let rtd = snd_soc_substream_to_rtd(ss);

    snd_soc_rtd_to_cpu(rtd, 0)
}

unsafe fn axg_fifo_data(ss: *mut snd_pcm_substream) -> *mut axg_fifo {
    let dai = axg_fifo_dai(ss);

    snd_soc_dai_get_drvdata(dai) as *mut axg_fifo
}

unsafe fn axg_fifo_dev(ss: *mut snd_pcm_substream) -> *mut device {
    let dai = axg_fifo_dai(ss);

    (*dai).dev
}

unsafe fn __dma_enable(fifo: *mut axg_fifo, enable: bool_) {
    regmap_update_bits(
        (*fifo).map,
        FIFO_CTRL0,
        CTRL0_DMA_EN,
        if enable { CTRL0_DMA_EN } else { 0 },
    );
}

#[no_mangle]
pub unsafe extern "C" fn axg_fifo_pcm_trigger(
    _component: *mut snd_soc_component,
    ss: *mut snd_pcm_substream,
    cmd: c_int,
) -> c_int {
    let fifo = axg_fifo_data(ss);

    match cmd {
        SNDRV_PCM_TRIGGER_START
        | SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            __dma_enable(fifo, true);
        }
        SNDRV_PCM_TRIGGER_SUSPEND
        | SNDRV_PCM_TRIGGER_PAUSE_PUSH
        | SNDRV_PCM_TRIGGER_STOP => {
            __dma_enable(fifo, false);
        }
        _ => return -EINVAL,
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn axg_fifo_pcm_pointer(
    _component: *mut snd_soc_component,
    ss: *mut snd_pcm_substream,
) -> snd_pcm_uframes_t {
    let fifo = axg_fifo_data(ss);
    let runtime = (*ss).runtime;
    let mut addr: c_uint = 0;

    regmap_read((*fifo).map, FIFO_STATUS2, &mut addr);

    bytes_to_frames(
        runtime,
        addr.wrapping_sub((*runtime).dma_addr as c_uint),
    )
}

#[no_mangle]
pub unsafe extern "C" fn axg_fifo_pcm_hw_params(
    _component: *mut snd_soc_component,
    ss: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let runtime = (*ss).runtime;
    let fifo = axg_fifo_data(ss);
    let mut burst_num: c_uint;
    let mut period: c_uint;
    let mut threshold: c_uint;
    let irq_en: c_uint;
    let end_ptr: dma_addr_t;

    period = params_period_bytes(params);

    /* Setup dma memory pointers */
    end_ptr = (*runtime)
        .dma_addr
        .wrapping_add((*runtime).dma_bytes)
        .wrapping_sub(AXG_FIFO_BURST as size_t);
    regmap_write((*fifo).map, FIFO_START_ADDR, (*runtime).dma_addr);
    regmap_write((*fifo).map, FIFO_FINISH_ADDR, end_ptr);

    /* Setup interrupt periodicity */
    burst_num = period / AXG_FIFO_BURST;
    regmap_write((*fifo).map, FIFO_INT_ADDR, burst_num as dma_addr_t);

    /*
     * Start the fifo request on the smallest of the following:
     * - Half the fifo size
     * - Half the period size
     */
    threshold = core::cmp::min(period / 2, (*fifo).depth / 2);

    /*
     * With the threshold in bytes, register value is:
     * V = (threshold / burst) - 1
     */
    threshold /= AXG_FIFO_BURST;
    regmap_field_write(
        (*fifo).field_threshold,
        if threshold != 0 { threshold - 1 } else { 0 },
    );

    /* Enable irq if necessary  */
    irq_en = if (*runtime).no_period_wakeup {
        0
    } else {
        FIFO_INT_COUNT_REPEAT
    };
    regmap_update_bits(
        (*fifo).map,
        FIFO_CTRL0,
        CTRL0_INT_EN,
        field_prep(CTRL0_INT_EN, irq_en),
    );

    0
}

#[no_mangle]
pub unsafe extern "C" fn g12a_fifo_pcm_hw_params(
    component: *mut snd_soc_component,
    ss: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
) -> c_int {
    let fifo = axg_fifo_data(ss);
    let runtime = (*ss).runtime;
    let ret: c_int;

    ret = axg_fifo_pcm_hw_params(component, ss, params);
    if ret != 0 {
        return ret;
    }

    /* Set the initial memory address of the DMA */
    regmap_write((*fifo).map, FIFO_INIT_ADDR, (*runtime).dma_addr);

    0
}

#[no_mangle]
pub unsafe extern "C" fn axg_fifo_pcm_hw_free(
    _component: *mut snd_soc_component,
    ss: *mut snd_pcm_substream,
) -> c_int {
    let fifo = axg_fifo_data(ss);

    /* Disable irqs */
    regmap_update_bits((*fifo).map, FIFO_CTRL0, CTRL0_INT_EN, 0);

    0
}

unsafe fn axg_fifo_ack_irq(fifo: *mut axg_fifo, mask: u8) {
    regmap_update_bits(
        (*fifo).map,
        FIFO_CTRL1,
        CTRL1_INT_CLR,
        field_prep(CTRL1_INT_CLR, mask as c_uint),
    );

    /* Clear must also be cleared */
    regmap_update_bits(
        (*fifo).map,
        FIFO_CTRL1,
        CTRL1_INT_CLR,
        field_prep(CTRL1_INT_CLR, 0),
    );
}

unsafe extern "C" fn axg_fifo_pcm_irq_block(_irq: c_int, dev_id: *mut c_void) -> irqreturn_t {
    let ss = dev_id as *mut snd_pcm_substream;
    let fifo = axg_fifo_data(ss);
    let mut status: c_uint = 0;

    regmap_read((*fifo).map, FIFO_STATUS1, &mut status);
    status = field_get(STATUS1_INT_STS, status);
    axg_fifo_ack_irq(fifo, status as u8);

    if (status & !FIFO_INT_COUNT_REPEAT) != 0 {
        dev_dbg(
            axg_fifo_dev(ss),
            b"unexpected irq - STS 0x%02x\n\0".as_ptr() as *const c_char,
            status,
        );
    }

    if (status & FIFO_INT_COUNT_REPEAT) != 0 {
        snd_pcm_period_elapsed(ss);
        return IRQ_HANDLED;
    }

    IRQ_NONE
}

#[no_mangle]
pub unsafe extern "C" fn axg_fifo_pcm_open(
    _component: *mut snd_soc_component,
    ss: *mut snd_pcm_substream,
) -> c_int {
    let fifo = axg_fifo_data(ss);
    let dev = axg_fifo_dev(ss);
    let mut ret: c_int;

    snd_soc_set_runtime_hwparams(ss, &axg_fifo_hw);

    /*
     * Make sure the buffer and period size are multiple of the FIFO
     * burst
     */
    ret = snd_pcm_hw_constraint_step(
        (*ss).runtime,
        0,
        SNDRV_PCM_HW_PARAM_BUFFER_BYTES,
        AXG_FIFO_BURST,
    );
    if ret != 0 {
        return ret;
    }

    ret = snd_pcm_hw_constraint_step(
        (*ss).runtime,
        0,
        SNDRV_PCM_HW_PARAM_PERIOD_BYTES,
        AXG_FIFO_BURST,
    );
    if ret != 0 {
        return ret;
    }

    /* Use the threaded irq handler only with non-atomic links */
    ret = request_threaded_irq(
        (*fifo).irq as c_uint,
        None,
        Some(axg_fifo_pcm_irq_block),
        IRQF_ONESHOT,
        dev_name(dev),
        ss as *mut c_void,
    );
    if ret != 0 {
        return ret;
    }

    /* Enable pclk to access registers and clock the fifo ip */
    ret = clk_prepare_enable((*fifo).pclk);
    if ret != 0 {
        free_irq((*fifo).irq as c_uint, ss as *mut c_void);
        return ret;
    }

    /* Setup status2 so it reports the memory pointer */
    regmap_update_bits(
        (*fifo).map,
        FIFO_CTRL1,
        CTRL1_STATUS2_SEL,
        field_prep(CTRL1_STATUS2_SEL, STATUS2_SEL_DDR_READ),
    );

    /* Make sure the dma is initially disabled */
    __dma_enable(fifo, false);

    /* Disable irqs until params are ready */
    regmap_update_bits((*fifo).map, FIFO_CTRL0, CTRL0_INT_EN, 0);

    /* Clear any pending interrupt */
    axg_fifo_ack_irq(fifo, FIFO_INT_MASK);

    /* Take memory arbitror out of reset */
    ret = reset_control_deassert((*fifo).arb);
    if ret != 0 {
        clk_disable_unprepare((*fifo).pclk);
        free_irq((*fifo).irq as c_uint, ss as *mut c_void);
        return ret;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn axg_fifo_pcm_close(
    _component: *mut snd_soc_component,
    ss: *mut snd_pcm_substream,
) -> c_int {
    let fifo = axg_fifo_data(ss);
    let ret: c_int;

    /* Put the memory arbitror back in reset */
    ret = reset_control_assert((*fifo).arb);

    /* Disable fifo ip and register access */
    clk_disable_unprepare((*fifo).pclk);

    /* remove IRQ */
    free_irq((*fifo).irq as c_uint, ss as *mut c_void);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn axg_fifo_pcm_new(
    rtd: *mut snd_soc_pcm_runtime,
    type_: c_uint,
) -> c_int {
    let card = (*(*rtd).card).snd_card;
    let size: size_t = axg_fifo_hw.buffer_bytes_max;

    snd_pcm_set_managed_buffer(
        (*(*(*rtd).pcm).streams.as_ptr().add(type_ as usize)).substream,
        SNDRV_DMA_TYPE_DEV,
        (*card).dev,
        size,
        size,
    );
    0
}

static axg_fifo_regmap_cfg: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    max_register: FIFO_CTRL2,
};

#[no_mangle]
pub unsafe extern "C" fn axg_fifo_probe(pdev: *mut platform_device) -> c_int {
    let dev = &mut (*pdev).dev as *mut device;
    let data: *const axg_fifo_match_data;
    let fifo: *mut axg_fifo;
    let regs: *mut c_void;
    let mut ret: c_int;

    data = of_device_get_match_data(dev) as *const axg_fifo_match_data;
    if data.is_null() {
        dev_err(dev, b"failed to match device\n\0".as_ptr() as *const c_char);
        return -ENODEV;
    }

    fifo = devm_kzalloc(dev, core::mem::size_of::<axg_fifo>(), GFP_KERNEL) as *mut axg_fifo;
    if fifo.is_null() {
        return -ENOMEM;
    }
    platform_set_drvdata(pdev, fifo as *mut c_void);

    regs = devm_platform_ioremap_resource(pdev, 0);
    if is_err(regs) {
        return ptr_err(regs) as c_int;
    }

    (*fifo).map = devm_regmap_init_mmio(dev, regs, &axg_fifo_regmap_cfg);
    if is_err((*fifo).map) {
        dev_err(
            dev,
            b"failed to init regmap: %ld\n\0".as_ptr() as *const c_char,
            ptr_err((*fifo).map),
        );
        return ptr_err((*fifo).map) as c_int;
    }

    (*fifo).pclk = devm_clk_get(dev, ptr::null());
    if is_err((*fifo).pclk) {
        return dev_err_probe(
            dev,
            ptr_err((*fifo).pclk),
            b"failed to get pclk\n\0".as_ptr() as *const c_char,
        );
    }

    (*fifo).arb = devm_reset_control_get_exclusive(dev, ptr::null());
    if is_err((*fifo).arb) {
        return dev_err_probe(
            dev,
            ptr_err((*fifo).arb),
            b"failed to get arb reset\n\0".as_ptr() as *const c_char,
        );
    }

    (*fifo).irq = of_irq_get((*dev).of_node, 0);
    if (*fifo).irq <= 0 {
        dev_err(
            dev,
            b"failed to get irq: %d\n\0".as_ptr() as *const c_char,
            (*fifo).irq,
        );
        return (*fifo).irq;
    }

    (*fifo).field_threshold = devm_regmap_field_alloc(dev, (*fifo).map, (*data).field_threshold);
    if is_err((*fifo).field_threshold) {
        return ptr_err((*fifo).field_threshold) as c_int;
    }

    ret = of_property_read_u32(
        (*dev).of_node,
        b"amlogic,fifo-depth\0".as_ptr() as *const c_char,
        &mut (*fifo).depth,
    );
    if ret != 0 {
        /* Error out for anything but a missing property */
        if ret != -EINVAL {
            return ret;
        }
        /*
         * If the property is missing, it might be because of an old
         * DT. In such case, assume the smallest known fifo depth
         */
        (*fifo).depth = 256;
        dev_warn(
            dev,
            b"fifo depth not found, assume %u bytes\n\0".as_ptr() as *const c_char,
            (*fifo).depth,
        );
    }

    devm_snd_soc_register_component(dev, (*data).component_drv, (*data).dai_drv, 1)
}

// MODULE_DESCRIPTION("Amlogic AXG/G12A fifo driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
