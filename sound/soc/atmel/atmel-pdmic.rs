// SPDX-License-Identifier: GPL-2.0-or-later
/* Atmel PDMIC driver
 *
 * Copyright (C) 2015 Atmel
 *
 * Author: Songjun Wu <songjun.wu@atmel.com>
 */

/* Rust translation of dependencies originally included from:
 * linux/of.h, linux/clk.h, linux/module.h, linux/platform_device.h,
 * linux/regmap.h, sound/core.h, sound/dmaengine_pcm.h,
 * sound/pcm_params.h, sound/tlv.h, and "atmel-pdmic.h".
 */

type u32 = u32;
type s32 = i32;
type dma_addr_t = usize;
type irqreturn_t = i32;

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    pub of_node: *mut device_node,
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_pcm_substream {
    pub runtime: *mut snd_pcm_runtime,
}
#[repr(C)]
pub struct snd_pcm_runtime {
    pub hw: snd_pcm_hardware,
}
#[repr(C)]
pub struct snd_soc_dai {
    pub component: *mut snd_soc_component,
}
#[repr(C)]
pub struct snd_soc_component {
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_soc_pcm_runtime {
    pub card: *mut snd_soc_card,
}
#[repr(C)]
pub struct snd_soc_card {
    pub dai_link: *mut snd_soc_dai_link,
    pub num_links: i32,
    pub name: *const i8,
    pub dev: *mut device,
}
#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}
#[repr(C)]
pub struct dma_slave_config {
    pub src_addr: dma_addr_t,
    pub src_maxburst: u32,
    pub dst_maxburst: u32,
}
#[repr(C)]
pub struct snd_dmaengine_pcm_config {
    pub prepare_slave_config: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut dma_slave_config,
        ) -> i32,
    >,
    pub pcm_hardware: *const snd_pcm_hardware,
    pub prealloc_buffer_size: usize,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_pcm_hardware {
    pub info: u32,
    pub formats: u64,
    pub buffer_bytes_max: usize,
    pub period_bytes_min: usize,
    pub period_bytes_max: usize,
    pub periods_min: u32,
    pub periods_max: u32,
    pub rate_min: u32,
    pub rate_max: u32,
}
#[repr(C)]
pub struct snd_kcontrol {
    pub private_value: usize,
}
#[repr(C)]
pub struct snd_ctl_elem_value {
    pub value: snd_ctl_elem_value_value,
}
#[repr(C)]
pub union snd_ctl_elem_value_value {
    pub integer: snd_ctl_elem_value_integer,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub struct snd_ctl_elem_value_integer {
    pub value: [i64; 128],
}
#[repr(C)]
pub struct soc_mixer_control {
    pub max: i32,
}
#[repr(C)]
pub struct snd_kcontrol_new {
    _private: [u8; 0],
}
#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> i32>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub prepare: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> i32>,
    pub hw_params:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> i32>,
    pub trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, i32, *mut snd_soc_dai) -> i32>,
}
#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const i8,
    pub channels_min: u32,
    pub channels_max: u32,
    pub rates: u32,
    pub formats: u64,
    pub rate_min: u32,
    pub rate_max: u32,
}
#[repr(C)]
pub struct snd_soc_dai_driver {
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}
#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const i8,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> i32>,
    pub controls: *const snd_kcontrol_new,
    pub num_controls: usize,
    pub idle_bias_on: i32,
    pub use_pmdown_time: i32,
    pub legacy_dai_naming: i32,
}
#[repr(C)]
pub struct snd_soc_dai_link_component {
    pub dai_name: *const i8,
}
#[repr(C)]
pub struct snd_soc_dai_link {
    pub cpus: *mut snd_soc_dai_link_component,
    pub codecs: *const snd_soc_dai_link_component,
    pub num_cpus: u32,
    pub num_codecs: u32,
    pub name: *const i8,
    pub stream_name: *const i8,
}
#[repr(C)]
pub struct resource {
    pub start: dma_addr_t,
}
#[repr(C)]
pub struct platform_device {
    pub dev: device,
}
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const i8,
}
#[repr(C)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub reg_stride: u32,
    pub val_bits: u32,
    pub max_register: u32,
}
#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const i8,
    pub of_match_table: *const of_device_id,
    pub pm: *const core::ffi::c_void,
}
#[repr(C)]
pub struct platform_driver {
    pub driver: platform_driver_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
}

extern "C" {
    static snd_soc_dummy_dlc: snd_soc_dai_link_component;
    static snd_soc_pm_ops: core::ffi::c_void;

    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dev_err(dev: *mut device, fmt: *const i8, ...);
    fn dev_warn(dev: *mut device, fmt: *const i8, ...);
    fn dev_name(dev: *mut device) -> *const i8;
    fn of_property_read_string(np: *mut device_node, propname: *const i8, out_string: *mut *const i8) -> i32;
    fn of_property_read_u32(np: *mut device_node, propname: *const i8, out_value: *mut u32) -> i32;
    fn of_property_read_s32(np: *mut device_node, propname: *const i8, out_value: *mut s32) -> i32;
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_get_rate(clk: *mut clk) -> u64;
    fn clk_set_rate(clk: *mut clk, rate: u64) -> i32;
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> i32;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn snd_soc_card_get_drvdata(card: *mut snd_soc_card) -> *mut core::ffi::c_void;
    fn snd_soc_card_set_drvdata(card: *mut snd_soc_card, data: *mut core::ffi::c_void);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut core::ffi::c_void;
    fn snd_kcontrol_chip(kcontrol: *mut snd_kcontrol) -> *mut snd_soc_component;
    fn snd_soc_component_read(component: *mut snd_soc_component, reg: u32) -> u32;
    fn snd_soc_component_update_bits(component: *mut snd_soc_component, reg: u32, mask: u32, val: u32) -> i32;
    fn snd_hwparams_to_dma_slave_config(
        substream: *mut snd_pcm_substream,
        params: *mut snd_pcm_hw_params,
        slave_config: *mut dma_slave_config,
    ) -> i32;
    fn params_rate(params: *mut snd_pcm_hw_params) -> i32;
    fn params_width(params: *mut snd_pcm_hw_params) -> i32;
    fn params_channels(params: *mut snd_pcm_hw_params) -> u32;
    fn snd_pcm_stop_xrun(substream: *mut snd_pcm_substream);
    fn platform_get_irq(pdev: *mut platform_device, num: u32) -> i32;
    fn devm_clk_get(dev: *mut device, id: *const i8) -> *mut clk;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
        res: *mut *mut resource,
    ) -> *mut core::ffi::c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut core::ffi::c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_request_irq(
        dev: *mut device,
        irq: i32,
        handler: unsafe extern "C" fn(i32, *mut core::ffi::c_void) -> irqreturn_t,
        irqflags: u64,
        devname: *const i8,
        dev_id: *mut core::ffi::c_void,
    ) -> i32;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: i32,
    ) -> i32;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: u32,
    ) -> i32;
    fn devm_snd_soc_register_card(dev: *mut device, card: *mut snd_soc_card) -> i32;
}

extern "Rust" {
    static PDMIC_CR: u32;
    static PDMIC_IER: u32;
    static PDMIC_IER_OVRE: u32;
    static PDMIC_IDR: u32;
    static PDMIC_IDR_OVRE: u32;
    static PDMIC_CDR: u32;
    static PDMIC_DSPR1: u32;
    static PDMIC_DSPR1_DGAIN_MASK: u32;
    static PDMIC_DSPR1_DGAIN_SHIFT: u32;
    static PDMIC_DSPR0: u32;
    static PDMIC_DSPR0_SCALE_MASK: u32;
    static PDMIC_DSPR0_SCALE_SHIFT: u32;
    static PDMIC_DSPR0_HPFBYP_SHIFT: u32;
    static PDMIC_DSPR0_SINBYP_SHIFT: u32;
    static PDMIC_DSPR1_OFFSET_MASK: u32;
    static PDMIC_DSPR1_OFFSET_SHIFT: u32;
    static PDMIC_DSPR0_SIZE_16_BITS: u32;
    static PDMIC_DSPR0_SIZE_32_BITS: u32;
    static PDMIC_DSPR0_SIZE_SHIFT: u32;
    static PDMIC_DSPR0_OSR_64: u32;
    static PDMIC_DSPR0_OSR_128: u32;
    static PDMIC_DSPR0_OSR_SHIFT: u32;
    static PDMIC_MR: u32;
    static PDMIC_MR_PRESCAL_SHIFT: u32;
    static PDMIC_MR_CLKS_GCK: u32;
    static PDMIC_MR_CLKS_SHIFT: u32;
    static PDMIC_MR_CLKS_PCK: u32;
    static PDMIC_MR_PRESCAL_MASK: u32;
    static PDMIC_MR_CLKS_MASK: u32;
    static PDMIC_DSPR0_OSR_MASK: u32;
    static PDMIC_DSPR0_SIZE_MASK: u32;
    static PDMIC_CR_ENPDM_MASK: u32;
    static PDMIC_CR_ENPDM_DIS: u32;
    static PDMIC_CR_ENPDM_SHIFT: u32;
    static PDMIC_CR_ENPDM_EN: u32;
    static PDMIC_ISR: u32;
    static PDMIC_ISR_OVRE: u32;
}

const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const GFP_KERNEL: u32 = 0;
const S16_MAX: s32 = i16::MAX as s32;
const S16_MIN: s32 = i16::MIN as s32;
const IRQ_NONE: irqreturn_t = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 1 << 2;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;
const SNDRV_PCM_INFO_MMAP: u32 = 1 << 0;
const SNDRV_PCM_INFO_MMAP_VALID: u32 = 1 << 1;
const SNDRV_PCM_INFO_INTERLEAVED: u32 = 1 << 2;
const SNDRV_PCM_INFO_RESUME: u32 = 1 << 4;
const SNDRV_PCM_INFO_PAUSE: u32 = 1 << 5;
const SNDRV_PCM_RATE_KNOT: u32 = 1 << 31;
const SNDRV_PCM_TRIGGER_START: i32 = 0;
const SNDRV_PCM_TRIGGER_STOP: i32 = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: i32 = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: i32 = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: i32 = 5;
const SNDRV_PCM_TRIGGER_RESUME: i32 = 6;

unsafe fn ERR_PTR<T>(err: i32) -> *mut T {
    err as isize as *mut T
}
unsafe fn PTR_ERR<T>(ptr: *const T) -> i32 {
    ptr as isize as i32
}
unsafe fn IS_ERR<T>(ptr: *const T) -> bool {
    (ptr as usize) >= (!4095usize)
}
const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}
const fn DIV_ROUND_CLOSEST(x: u32, divisor: u32) -> u32 {
    (x + divisor / 2) / divisor
}

#[repr(C)]
struct atmel_pdmic_pdata {
    mic_min_freq: u32,
    mic_max_freq: u32,
    mic_offset: s32,
    card_name: *const i8,
}

#[repr(C)]
struct atmel_pdmic {
    phy_base: dma_addr_t,
    regmap: *mut regmap,
    pclk: *mut clk,
    gclk: *mut clk,
    dev: *mut device,
    irq: i32,
    substream: *mut snd_pcm_substream,
    pdata: *const atmel_pdmic_pdata,
}

static atmel_pdmic_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"atmel,sama5d2-pdmic\0".as_ptr() as *const i8,
    },
    of_device_id {
        /* sentinel */
        compatible: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, atmel_pdmic_of_match); */

const PDMIC_OFFSET_MAX_VAL: s32 = S16_MAX;
const PDMIC_OFFSET_MIN_VAL: s32 = S16_MIN;

unsafe extern "C" fn atmel_pdmic_dt_init(dev: *mut device) -> *mut atmel_pdmic_pdata {
    let np: *mut device_node = (*dev).of_node;
    let mut pdata: *mut atmel_pdmic_pdata;

    if np.is_null() {
        dev_err(dev, b"device node not found\n\0".as_ptr() as *const i8);
        return ERR_PTR(-EINVAL);
    }

    pdata = devm_kzalloc(dev, core::mem::size_of::<atmel_pdmic_pdata>(), GFP_KERNEL)
        as *mut atmel_pdmic_pdata;
    if pdata.is_null() {
        return ERR_PTR(-ENOMEM);
    }

    if of_property_read_string(
        np,
        b"atmel,model\0".as_ptr() as *const i8,
        &mut (*pdata).card_name,
    ) != 0
    {
        (*pdata).card_name = b"PDMIC\0".as_ptr() as *const i8;
    }

    if of_property_read_u32(
        np,
        b"atmel,mic-min-freq\0".as_ptr() as *const i8,
        &mut (*pdata).mic_min_freq,
    ) != 0
    {
        dev_err(dev, b"failed to get mic-min-freq\n\0".as_ptr() as *const i8);
        return ERR_PTR(-EINVAL);
    }

    if of_property_read_u32(
        np,
        b"atmel,mic-max-freq\0".as_ptr() as *const i8,
        &mut (*pdata).mic_max_freq,
    ) != 0
    {
        dev_err(dev, b"failed to get mic-max-freq\n\0".as_ptr() as *const i8);
        return ERR_PTR(-EINVAL);
    }

    if (*pdata).mic_max_freq < (*pdata).mic_min_freq {
        dev_err(
            dev,
            b"mic-max-freq should not be less than mic-min-freq\n\0".as_ptr() as *const i8,
        );
        return ERR_PTR(-EINVAL);
    }

    if of_property_read_s32(
        np,
        b"atmel,mic-offset\0".as_ptr() as *const i8,
        &mut (*pdata).mic_offset,
    ) != 0
    {
        (*pdata).mic_offset = 0;
    }

    if (*pdata).mic_offset > PDMIC_OFFSET_MAX_VAL {
        dev_warn(
            dev,
            b"mic-offset value %d is larger than the max value %d, the max value is specified\n\0"
                .as_ptr() as *const i8,
            (*pdata).mic_offset,
            PDMIC_OFFSET_MAX_VAL,
        );
        (*pdata).mic_offset = PDMIC_OFFSET_MAX_VAL;
    } else if (*pdata).mic_offset < PDMIC_OFFSET_MIN_VAL {
        dev_warn(
            dev,
            b"mic-offset value %d is less than the min value %d, the min value is specified\n\0"
                .as_ptr() as *const i8,
            (*pdata).mic_offset,
            PDMIC_OFFSET_MIN_VAL,
        );
        (*pdata).mic_offset = PDMIC_OFFSET_MIN_VAL;
    }

    pdata
}

/* cpu dai component */
unsafe extern "C" fn atmel_pdmic_cpu_dai_startup(
    substream: *mut snd_pcm_substream,
    _cpu_dai: *mut snd_soc_dai,
) -> i32 {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let dd: *mut atmel_pdmic = snd_soc_card_get_drvdata((*rtd).card) as *mut atmel_pdmic;
    let mut ret: i32;

    ret = clk_prepare_enable((*dd).gclk);
    if ret != 0 {
        return ret;
    }

    ret = clk_prepare_enable((*dd).pclk);
    if ret != 0 {
        clk_disable_unprepare((*dd).gclk);
        return ret;
    }

    /* Clear all bits in the Control Register(PDMIC_CR) */
    regmap_write((*dd).regmap, PDMIC_CR, 0);

    (*dd).substream = substream;

    /* Enable the overrun error interrupt */
    regmap_write((*dd).regmap, PDMIC_IER, PDMIC_IER_OVRE);

    0
}

unsafe extern "C" fn atmel_pdmic_cpu_dai_shutdown(
    substream: *mut snd_pcm_substream,
    _cpu_dai: *mut snd_soc_dai,
) {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let dd: *mut atmel_pdmic = snd_soc_card_get_drvdata((*rtd).card) as *mut atmel_pdmic;

    /* Disable the overrun error interrupt */
    regmap_write((*dd).regmap, PDMIC_IDR, PDMIC_IDR_OVRE);

    clk_disable_unprepare((*dd).gclk);
    clk_disable_unprepare((*dd).pclk);
}

unsafe extern "C" fn atmel_pdmic_cpu_dai_prepare(
    substream: *mut snd_pcm_substream,
    cpu_dai: *mut snd_soc_dai,
) -> i32 {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let dd: *mut atmel_pdmic = snd_soc_card_get_drvdata((*rtd).card) as *mut atmel_pdmic;
    let component: *mut snd_soc_component = (*cpu_dai).component;
    let mut val: u32 = 0;
    let mut ret: i32;

    /* Clean the PDMIC Converted Data Register */
    ret = regmap_read((*dd).regmap, PDMIC_CDR, &mut val);
    if ret < 0 {
        return 0;
    }

    ret = snd_soc_component_update_bits(
        component,
        PDMIC_CR,
        PDMIC_CR_ENPDM_MASK,
        PDMIC_CR_ENPDM_DIS << PDMIC_CR_ENPDM_SHIFT,
    );
    if ret < 0 {
        return ret;
    }

    0
}

const ATMEL_PDMIC_FORMATS: u64 = SNDRV_PCM_FMTBIT_S16_LE | SNDRV_PCM_FMTBIT_S32_LE;

/* platform */
const ATMEL_PDMIC_MAX_BUF_SIZE: usize = 64 * 1024;
const ATMEL_PDMIC_PREALLOC_BUF_SIZE: usize = ATMEL_PDMIC_MAX_BUF_SIZE;

static atmel_pdmic_hw: snd_pcm_hardware = snd_pcm_hardware {
    info: SNDRV_PCM_INFO_MMAP
        | SNDRV_PCM_INFO_MMAP_VALID
        | SNDRV_PCM_INFO_INTERLEAVED
        | SNDRV_PCM_INFO_RESUME
        | SNDRV_PCM_INFO_PAUSE,
    formats: ATMEL_PDMIC_FORMATS,
    buffer_bytes_max: ATMEL_PDMIC_MAX_BUF_SIZE,
    period_bytes_min: 256,
    period_bytes_max: 32 * 1024,
    periods_min: 2,
    periods_max: 256,
    rate_min: 0,
    rate_max: 0,
};

unsafe extern "C" fn atmel_pdmic_platform_configure_dma(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    slave_config: *mut dma_slave_config,
) -> i32 {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let dd: *mut atmel_pdmic = snd_soc_card_get_drvdata((*rtd).card) as *mut atmel_pdmic;
    let ret: i32;

    ret = snd_hwparams_to_dma_slave_config(substream, params, slave_config);
    if ret != 0 {
        dev_err((*dd).dev, b"hw params to dma slave configure failed\n\0".as_ptr() as *const i8);
        return ret;
    }

    (*slave_config).src_addr = (*dd).phy_base + PDMIC_CDR as usize;
    (*slave_config).src_maxburst = 1;
    (*slave_config).dst_maxburst = 1;

    0
}

static atmel_pdmic_dmaengine_pcm_config: snd_dmaengine_pcm_config = snd_dmaengine_pcm_config {
    prepare_slave_config: Some(atmel_pdmic_platform_configure_dma),
    pcm_hardware: &atmel_pdmic_hw,
    prealloc_buffer_size: ATMEL_PDMIC_PREALLOC_BUF_SIZE,
};

/* codec */
/* Mic Gain = dgain * 2^(-scale) */
#[repr(C)]
#[derive(Copy, Clone)]
struct mic_gain {
    dgain: u32,
    scale: u32,
}

/* range from -90 dB to 90 dB */
static mic_gain_table: [mic_gain; 157] = [
mic_gain { dgain:     1, scale: 15}, mic_gain { dgain:     1, scale: 14},                           /* -90, -84 dB */
mic_gain { dgain:     3, scale: 15}, mic_gain { dgain:     1, scale: 13}, mic_gain { dgain:     3, scale: 14}, mic_gain { dgain:     1, scale: 12}, /* -81, -78, -75, -72 dB */
mic_gain { dgain:     5, scale: 14}, mic_gain { dgain:    13, scale: 15},                           /* -70, -68 dB */
mic_gain { dgain:     9, scale: 14}, mic_gain { dgain:    21, scale: 15}, mic_gain { dgain:    23, scale: 15}, mic_gain { dgain:    13, scale: 14}, /* -65 ~ -62 dB */
mic_gain { dgain:    29, scale: 15}, mic_gain { dgain:    33, scale: 15}, mic_gain { dgain:    37, scale: 15}, mic_gain { dgain:    41, scale: 15}, /* -61 ~ -58 dB */
mic_gain { dgain:    23, scale: 14}, mic_gain { dgain:    13, scale: 13}, mic_gain { dgain:    58, scale: 15}, mic_gain { dgain:    65, scale: 15}, /* -57 ~ -54 dB */
mic_gain { dgain:    73, scale: 15}, mic_gain { dgain:    41, scale: 14}, mic_gain { dgain:    23, scale: 13}, mic_gain { dgain:    13, scale: 12}, /* -53 ~ -50 dB */
mic_gain { dgain:    29, scale: 13}, mic_gain { dgain:    65, scale: 14}, mic_gain { dgain:    73, scale: 14}, mic_gain { dgain:    41, scale: 13}, /* -49 ~ -46 dB */
mic_gain { dgain:    23, scale: 12}, mic_gain { dgain:   207, scale: 15}, mic_gain { dgain:    29, scale: 12}, mic_gain { dgain:    65, scale: 13}, /* -45 ~ -42 dB */
mic_gain { dgain:    73, scale: 13}, mic_gain { dgain:    41, scale: 12}, mic_gain { dgain:    23, scale: 11}, mic_gain { dgain:   413, scale: 15}, /* -41 ~ -38 dB */
mic_gain { dgain:   463, scale: 15}, mic_gain { dgain:   519, scale: 15}, mic_gain { dgain:   583, scale: 15}, mic_gain { dgain:   327, scale: 14}, /* -37 ~ -34 dB */
mic_gain { dgain:   367, scale: 14}, mic_gain { dgain:   823, scale: 15}, mic_gain { dgain:   231, scale: 13}, mic_gain { dgain:  1036, scale: 15}, /* -33 ~ -30 dB */
mic_gain { dgain:  1163, scale: 15}, mic_gain { dgain:  1305, scale: 15}, mic_gain { dgain:   183, scale: 12}, mic_gain { dgain:  1642, scale: 15}, /* -29 ~ -26 dB */
mic_gain { dgain:  1843, scale: 15}, mic_gain { dgain:  2068, scale: 15}, mic_gain { dgain:   145, scale: 11}, mic_gain { dgain:  2603, scale: 15}, /* -25 ~ -22 dB */
mic_gain { dgain:   365, scale: 12}, mic_gain { dgain:  3277, scale: 15}, mic_gain { dgain:  3677, scale: 15}, mic_gain { dgain:  4125, scale: 15}, /* -21 ~ -18 dB */
mic_gain { dgain:  4629, scale: 15}, mic_gain { dgain:  5193, scale: 15}, mic_gain { dgain:  5827, scale: 15}, mic_gain { dgain:  3269, scale: 14}, /* -17 ~ -14 dB */
mic_gain { dgain:   917, scale: 12}, mic_gain { dgain:  8231, scale: 15}, mic_gain { dgain:  9235, scale: 15}, mic_gain { dgain:  5181, scale: 14}, /* -13 ~ -10 dB */
mic_gain { dgain: 11627, scale: 15}, mic_gain { dgain: 13045, scale: 15}, mic_gain { dgain: 14637, scale: 15}, mic_gain { dgain: 16423, scale: 15}, /*  -9 ~ -6 dB */
mic_gain { dgain: 18427, scale: 15}, mic_gain { dgain: 20675, scale: 15}, mic_gain { dgain:  5799, scale: 13}, mic_gain { dgain: 26029, scale: 15}, /*  -5 ~ -2 dB */
mic_gain { dgain:  7301, scale: 13}, mic_gain { dgain:     1, scale:  0}, mic_gain { dgain: 18383, scale: 14}, mic_gain { dgain: 10313, scale: 13}, /*  -1 ~ 2 dB */
mic_gain { dgain: 23143, scale: 14}, mic_gain { dgain: 25967, scale: 14}, mic_gain { dgain: 29135, scale: 14}, mic_gain { dgain: 16345, scale: 13}, /*   3 ~ 6 dB */
mic_gain { dgain:  4585, scale: 11}, mic_gain { dgain: 20577, scale: 13}, mic_gain { dgain:  1443, scale:  9}, mic_gain { dgain: 25905, scale: 13}, /*   7 ~ 10 dB */
mic_gain { dgain: 14533, scale: 12}, mic_gain { dgain:  8153, scale: 11}, mic_gain { dgain:  2287, scale:  9}, mic_gain { dgain: 20529, scale: 12}, /*  11 ~ 14 dB */
mic_gain { dgain: 11517, scale: 11}, mic_gain { dgain:  6461, scale: 10}, mic_gain { dgain: 28997, scale: 12}, mic_gain { dgain:  4067, scale:  9}, /*  15 ~ 18 dB */
mic_gain { dgain: 18253, scale: 11}, mic_gain { dgain:    10, scale:  0}, mic_gain { dgain: 22979, scale: 11}, mic_gain { dgain: 25783, scale: 11}, /*  19 ~ 22 dB */
mic_gain { dgain: 28929, scale: 11}, mic_gain { dgain: 32459, scale: 11}, mic_gain { dgain:  9105, scale:  9}, mic_gain { dgain: 20431, scale: 10}, /*  23 ~ 26 dB */
mic_gain { dgain: 22925, scale: 10}, mic_gain { dgain: 12861, scale:  9}, mic_gain { dgain:  7215, scale:  8}, mic_gain { dgain: 16191, scale:  9}, /*  27 ~ 30 dB */
mic_gain { dgain:  9083, scale:  8}, mic_gain { dgain: 20383, scale:  9}, mic_gain { dgain: 11435, scale:  8}, mic_gain { dgain:  6145, scale:  7}, /*  31 ~ 34 dB */
mic_gain { dgain:  3599, scale:  6}, mic_gain { dgain: 32305, scale:  9}, mic_gain { dgain: 18123, scale:  8}, mic_gain { dgain: 20335, scale:  8}, /*  35 ~ 38 dB */
mic_gain { dgain:   713, scale:  3}, mic_gain { dgain:   100, scale:  0}, mic_gain { dgain:  7181, scale:  6}, mic_gain { dgain:  8057, scale:  6}, /*  39 ~ 42 dB */
mic_gain { dgain:   565, scale:  2}, mic_gain { dgain: 20287, scale:  7}, mic_gain { dgain: 11381, scale:  6}, mic_gain { dgain: 25539, scale:  7}, /*  43 ~ 46 dB */
mic_gain { dgain:  1791, scale:  3}, mic_gain { dgain:  4019, scale:  4}, mic_gain { dgain:  9019, scale:  5}, mic_gain { dgain: 20239, scale:  6}, /*  47 ~ 50 dB */
mic_gain { dgain:  5677, scale:  4}, mic_gain { dgain: 25479, scale:  6}, mic_gain { dgain:  7147, scale:  4}, mic_gain { dgain:  8019, scale:  4}, /*  51 ~ 54 dB */
mic_gain { dgain: 17995, scale:  5}, mic_gain { dgain: 20191, scale:  5}, mic_gain { dgain: 11327, scale:  4}, mic_gain { dgain: 12709, scale:  4}, /*  55 ~ 58 dB */
mic_gain { dgain:  3565, scale:  2}, mic_gain { dgain:  1000, scale:  0}, mic_gain { dgain:  1122, scale:  0}, mic_gain { dgain:  1259, scale:  0}, /*  59 ~ 62 dB */
mic_gain { dgain:  2825, scale:  1}, mic_gain { dgain: 12679, scale:  3}, mic_gain { dgain:  7113, scale:  2}, mic_gain { dgain:  7981, scale:  2}, /*  63 ~ 66 dB */
mic_gain { dgain:  8955, scale:  2}, mic_gain { dgain: 20095, scale:  3}, mic_gain { dgain: 22547, scale:  3}, mic_gain { dgain: 12649, scale:  2}, /*  67 ~ 70 dB */
mic_gain { dgain: 28385, scale:  3}, mic_gain { dgain:  3981, scale:  0}, mic_gain { dgain: 17867, scale:  2}, mic_gain { dgain: 20047, scale:  2}, /*  71 ~ 74 dB */
mic_gain { dgain: 11247, scale:  1}, mic_gain { dgain: 12619, scale:  1}, mic_gain { dgain: 14159, scale:  1}, mic_gain { dgain: 31773, scale:  2}, /*  75 ~ 78 dB */
mic_gain { dgain: 17825, scale:  1}, mic_gain { dgain: 10000, scale:  0}, mic_gain { dgain: 11220, scale:  0}, mic_gain { dgain: 12589, scale:  0}, /*  79 ~ 82 dB */
mic_gain { dgain: 28251, scale:  1}, mic_gain { dgain: 15849, scale:  0}, mic_gain { dgain: 17783, scale:  0}, mic_gain { dgain: 19953, scale:  0}, /*  83 ~ 86 dB */
mic_gain { dgain: 22387, scale:  0}, mic_gain { dgain: 25119, scale:  0}, mic_gain { dgain: 28184, scale:  0}, mic_gain { dgain: 31623, scale:  0}, /*  87 ~ 90 dB */
];

/* static const DECLARE_TLV_DB_RANGE(mic_gain_tlv,
 *     0, 1, TLV_DB_SCALE_ITEM(-9000, 600, 0),
 *     2, 5, TLV_DB_SCALE_ITEM(-8100, 300, 0),
 *     6, 7, TLV_DB_SCALE_ITEM(-7000, 200, 0),
 *     8, ARRAY_SIZE(mic_gain_table)-1, TLV_DB_SCALE_ITEM(-6500, 100, 0),
 * );
 */
static mic_gain_tlv: [u32; 0] = [];

unsafe extern "C" fn pdmic_get_mic_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let dgain_val: u32;
    let scale_val: u32;
    let mut i: usize;

    dgain_val = (snd_soc_component_read(component, PDMIC_DSPR1) & PDMIC_DSPR1_DGAIN_MASK)
        >> PDMIC_DSPR1_DGAIN_SHIFT;

    scale_val = (snd_soc_component_read(component, PDMIC_DSPR0) & PDMIC_DSPR0_SCALE_MASK)
        >> PDMIC_DSPR0_SCALE_SHIFT;

    i = 0;
    while i < ARRAY_SIZE(&mic_gain_table) {
        if mic_gain_table[i].dgain == dgain_val && mic_gain_table[i].scale == scale_val {
            (*ucontrol).value.integer.value[0] = i as i64;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn pdmic_put_mic_volsw(
    kcontrol: *mut snd_kcontrol,
    ucontrol: *mut snd_ctl_elem_value,
) -> i32 {
    let mc: *mut soc_mixer_control = (*kcontrol).private_value as *mut soc_mixer_control;
    let component: *mut snd_soc_component = snd_kcontrol_chip(kcontrol);
    let max: i32 = (*mc).max;
    let val: u32;
    let mut ret: i32;

    val = (*ucontrol).value.integer.value[0] as u32;

    if val > max as u32 {
        return -EINVAL;
    }

    ret = snd_soc_component_update_bits(
        component,
        PDMIC_DSPR1,
        PDMIC_DSPR1_DGAIN_MASK,
        mic_gain_table[val as usize].dgain << PDMIC_DSPR1_DGAIN_SHIFT,
    );
    if ret < 0 {
        return ret;
    }

    ret = snd_soc_component_update_bits(
        component,
        PDMIC_DSPR0,
        PDMIC_DSPR0_SCALE_MASK,
        mic_gain_table[val as usize].scale << PDMIC_DSPR0_SCALE_SHIFT,
    );
    if ret < 0 {
        return ret;
    }

    0
}

/* SOC_SINGLE_EXT_TLV/SOC_SINGLE expand to snd_kcontrol_new initializers in C. */
static atmel_pdmic_snd_controls: [snd_kcontrol_new; 3] = [
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
    snd_kcontrol_new { _private: [] },
];

unsafe extern "C" fn atmel_pdmic_component_probe(component: *mut snd_soc_component) -> i32 {
    let card: *mut snd_soc_card = snd_soc_component_get_drvdata(component) as *mut snd_soc_card;
    let dd: *mut atmel_pdmic = snd_soc_card_get_drvdata(card) as *mut atmel_pdmic;

    snd_soc_component_update_bits(
        component,
        PDMIC_DSPR1,
        PDMIC_DSPR1_OFFSET_MASK,
        ((*(*dd).pdata).mic_offset << PDMIC_DSPR1_OFFSET_SHIFT) as u32,
    );

    0
}

const PDMIC_MR_PRESCAL_MAX_VAL: u32 = 127;

unsafe extern "C" fn atmel_pdmic_cpu_dai_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    cpu_dai: *mut snd_soc_dai,
) -> i32 {
    let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
    let dd: *mut atmel_pdmic = snd_soc_card_get_drvdata((*rtd).card) as *mut atmel_pdmic;
    let component: *mut snd_soc_component = (*cpu_dai).component;
    let rate_min: u32 = (*substream).runtime.as_ref().unwrap().hw.rate_min;
    let rate_max: u32 = (*substream).runtime.as_ref().unwrap().hw.rate_max;
    let fs: i32 = params_rate(params);
    let bits: i32 = params_width(params);
    let pclk_rate: u64;
    let gclk_rate: u64;
    let f_pdmic: u32;
    let mut mr_val: u32;
    let mut dspr0_val: u32;
    let pclk_prescal: u32;
    let gclk_prescal: u32;

    if params_channels(params) != 1 {
        dev_err((*component).dev, b"only supports one channel\n\0".as_ptr() as *const i8);
        return -EINVAL;
    }

    if fs < rate_min as i32 || fs > rate_max as i32 {
        dev_err(
            (*component).dev,
            b"sample rate is %dHz, min rate is %dHz, max rate is %dHz\n\0".as_ptr() as *const i8,
            fs,
            rate_min,
            rate_max,
        );

        return -EINVAL;
    }

    match bits {
        16 => {
            dspr0_val = PDMIC_DSPR0_SIZE_16_BITS << PDMIC_DSPR0_SIZE_SHIFT;
        }
        32 => {
            dspr0_val = PDMIC_DSPR0_SIZE_32_BITS << PDMIC_DSPR0_SIZE_SHIFT;
        }
        _ => {
            return -EINVAL;
        }
    }

    if ((fs as u32) << 7) > (rate_max << 6) {
        f_pdmic = (fs as u32) << 6;
        dspr0_val |= PDMIC_DSPR0_OSR_64 << PDMIC_DSPR0_OSR_SHIFT;
    } else {
        f_pdmic = (fs as u32) << 7;
        dspr0_val |= PDMIC_DSPR0_OSR_128 << PDMIC_DSPR0_OSR_SHIFT;
    }

    pclk_rate = clk_get_rate((*dd).pclk);
    gclk_rate = clk_get_rate((*dd).gclk);

    /* PRESCAL = SELCK/(2*f_pdmic) - 1*/
    pclk_prescal = (pclk_rate / ((f_pdmic << 1) as u64)) as u32 - 1;
    gclk_prescal = (gclk_rate / ((f_pdmic << 1) as u64)) as u32 - 1;

    if pclk_prescal > PDMIC_MR_PRESCAL_MAX_VAL
        || gclk_rate / (((gclk_prescal + 1) << 1) as u64)
            < pclk_rate / (((pclk_prescal + 1) << 1) as u64)
    {
        mr_val = gclk_prescal << PDMIC_MR_PRESCAL_SHIFT;
        mr_val |= PDMIC_MR_CLKS_GCK << PDMIC_MR_CLKS_SHIFT;
    } else {
        mr_val = pclk_prescal << PDMIC_MR_PRESCAL_SHIFT;
        mr_val |= PDMIC_MR_CLKS_PCK << PDMIC_MR_CLKS_SHIFT;
    }

    snd_soc_component_update_bits(
        component,
        PDMIC_MR,
        PDMIC_MR_PRESCAL_MASK | PDMIC_MR_CLKS_MASK,
        mr_val,
    );

    snd_soc_component_update_bits(
        component,
        PDMIC_DSPR0,
        PDMIC_DSPR0_OSR_MASK | PDMIC_DSPR0_SIZE_MASK,
        dspr0_val,
    );

    0
}

unsafe extern "C" fn atmel_pdmic_cpu_dai_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: i32,
    cpu_dai: *mut snd_soc_dai,
) -> i32 {
    let component: *mut snd_soc_component = (*cpu_dai).component;
    let val: u32;

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            val = PDMIC_CR_ENPDM_EN << PDMIC_CR_ENPDM_SHIFT;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            val = PDMIC_CR_ENPDM_DIS << PDMIC_CR_ENPDM_SHIFT;
        }
        _ => {
            return -EINVAL;
        }
    }

    snd_soc_component_update_bits(component, PDMIC_CR, PDMIC_CR_ENPDM_MASK, val);

    0
}

static atmel_pdmic_cpu_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(atmel_pdmic_cpu_dai_startup),
    shutdown: Some(atmel_pdmic_cpu_dai_shutdown),
    prepare: Some(atmel_pdmic_cpu_dai_prepare),
    hw_params: Some(atmel_pdmic_cpu_dai_hw_params),
    trigger: Some(atmel_pdmic_cpu_dai_trigger),
};

static mut atmel_pdmic_cpu_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    capture: snd_soc_pcm_stream {
        stream_name: b"Capture\0".as_ptr() as *const i8,
        channels_min: 1,
        channels_max: 1,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: ATMEL_PDMIC_FORMATS,
        rate_min: 0,
        rate_max: 0,
    },
    ops: &atmel_pdmic_cpu_dai_ops,
};

static atmel_pdmic_cpu_dai_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"atmel-pdmic\0".as_ptr() as *const i8,
    probe: Some(atmel_pdmic_component_probe),
    controls: atmel_pdmic_snd_controls.as_ptr(),
    num_controls: ARRAY_SIZE(&atmel_pdmic_snd_controls),
    idle_bias_on: 1,
    use_pmdown_time: 1,
    legacy_dai_naming: 1,
};

/* ASoC sound card */
unsafe extern "C" fn atmel_pdmic_asoc_card_init(
    dev: *mut device,
    card: *mut snd_soc_card,
) -> i32 {
    let mut dai_link: *mut snd_soc_dai_link;
    let dd: *mut atmel_pdmic = snd_soc_card_get_drvdata(card) as *mut atmel_pdmic;
    let mut comp: *mut snd_soc_dai_link_component;

    dai_link = devm_kzalloc(dev, core::mem::size_of::<snd_soc_dai_link>(), GFP_KERNEL)
        as *mut snd_soc_dai_link;
    if dai_link.is_null() {
        return -ENOMEM;
    }

    comp = devm_kzalloc(
        dev,
        core::mem::size_of::<snd_soc_dai_link_component>(),
        GFP_KERNEL,
    ) as *mut snd_soc_dai_link_component;
    if comp.is_null() {
        return -ENOMEM;
    }

    (*dai_link).cpus = comp;
    (*dai_link).codecs = &snd_soc_dummy_dlc;

    (*dai_link).num_cpus = 1;
    (*dai_link).num_codecs = 1;

    (*dai_link).name = b"PDMIC\0".as_ptr() as *const i8;
    (*dai_link).stream_name = b"PDMIC PCM\0".as_ptr() as *const i8;
    (*(*dai_link).cpus).dai_name = dev_name(dev);

    (*card).dai_link = dai_link;
    (*card).num_links = 1;
    (*card).name = (*(*dd).pdata).card_name;
    (*card).dev = dev;

    0
}

unsafe extern "C" fn atmel_pdmic_get_sample_rate(
    dd: *mut atmel_pdmic,
    rate_min: *mut u32,
    rate_max: *mut u32,
) {
    let mut mic_min_freq: u32 = (*(*dd).pdata).mic_min_freq;
    let mut mic_max_freq: u32 = (*(*dd).pdata).mic_max_freq;
    let clk_max_rate: u32 = (clk_get_rate((*dd).pclk) >> 1) as u32;
    let clk_min_rate: u32 = (clk_get_rate((*dd).gclk) >> 8) as u32;

    if mic_max_freq > clk_max_rate {
        mic_max_freq = clk_max_rate;
    }

    if mic_min_freq < clk_min_rate {
        mic_min_freq = clk_min_rate;
    }

    *rate_min = DIV_ROUND_CLOSEST(mic_min_freq, 128);
    *rate_max = mic_max_freq >> 6;
}

/* PDMIC interrupt handler */
unsafe extern "C" fn atmel_pdmic_interrupt(
    _irq: i32,
    dev_id: *mut core::ffi::c_void,
) -> irqreturn_t {
    let dd: *mut atmel_pdmic = dev_id as *mut atmel_pdmic;
    let mut pdmic_isr: u32 = 0;
    let mut ret: irqreturn_t = IRQ_NONE;

    regmap_read((*dd).regmap, PDMIC_ISR, &mut pdmic_isr);

    if (pdmic_isr & PDMIC_ISR_OVRE) != 0 {
        regmap_update_bits(
            (*dd).regmap,
            PDMIC_CR,
            PDMIC_CR_ENPDM_MASK,
            PDMIC_CR_ENPDM_DIS << PDMIC_CR_ENPDM_SHIFT,
        );

        snd_pcm_stop_xrun((*dd).substream);

        ret = IRQ_HANDLED;
    }

    ret
}

/* regmap configuration */
const ATMEL_PDMIC_REG_MAX: u32 = 0x124;
static atmel_pdmic_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: ATMEL_PDMIC_REG_MAX,
};

unsafe extern "C" fn atmel_pdmic_probe(pdev: *mut platform_device) -> i32 {
    let dev: *mut device = &mut (*pdev).dev;
    let mut dd: *mut atmel_pdmic;
    let mut res: *mut resource = core::ptr::null_mut();
    let io_base: *mut core::ffi::c_void;
    let pdata: *const atmel_pdmic_pdata;
    let mut card: *mut snd_soc_card;
    let mut rate_min: u32 = 0;
    let mut rate_max: u32 = 0;
    let mut ret: i32;

    pdata = atmel_pdmic_dt_init(dev);
    if IS_ERR(pdata) {
        return PTR_ERR(pdata);
    }

    dd = devm_kzalloc(dev, core::mem::size_of::<atmel_pdmic>(), GFP_KERNEL) as *mut atmel_pdmic;
    if dd.is_null() {
        return -ENOMEM;
    }

    (*dd).pdata = pdata;
    (*dd).dev = dev;

    (*dd).irq = platform_get_irq(pdev, 0);
    if (*dd).irq < 0 {
        return (*dd).irq;
    }

    (*dd).pclk = devm_clk_get(dev, b"pclk\0".as_ptr() as *const i8);
    if IS_ERR((*dd).pclk) {
        ret = PTR_ERR((*dd).pclk);
        dev_err(dev, b"failed to get peripheral clock: %d\n\0".as_ptr() as *const i8, ret);
        return ret;
    }

    (*dd).gclk = devm_clk_get(dev, b"gclk\0".as_ptr() as *const i8);
    if IS_ERR((*dd).gclk) {
        ret = PTR_ERR((*dd).gclk);
        dev_err(dev, b"failed to get GCK: %d\n\0".as_ptr() as *const i8, ret);
        return ret;
    }

    /* The gclk clock frequency must always be three times
     * lower than the pclk clock frequency
     */
    ret = clk_set_rate((*dd).gclk, clk_get_rate((*dd).pclk) / 3);
    if ret != 0 {
        dev_err(dev, b"failed to set GCK clock rate: %d\n\0".as_ptr() as *const i8, ret);
        return ret;
    }

    io_base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(io_base) {
        return PTR_ERR(io_base);
    }

    (*dd).phy_base = (*res).start;

    (*dd).regmap = devm_regmap_init_mmio(dev, io_base, &atmel_pdmic_regmap_config);
    if IS_ERR((*dd).regmap) {
        ret = PTR_ERR((*dd).regmap);
        dev_err(dev, b"failed to init register map: %d\n\0".as_ptr() as *const i8, ret);
        return ret;
    }

    ret = devm_request_irq(
        dev,
        (*dd).irq,
        atmel_pdmic_interrupt,
        0,
        b"PDMIC\0".as_ptr() as *const i8,
        dd as *mut core::ffi::c_void,
    );
    if ret < 0 {
        dev_err(
            dev,
            b"can't register ISR for IRQ %u (ret=%i)\n\0".as_ptr() as *const i8,
            (*dd).irq,
            ret,
        );
        return ret;
    }

    /* Get the minimal and maximal sample rate that the microphone supports */
    atmel_pdmic_get_sample_rate(dd, &mut rate_min, &mut rate_max);

    /* register cpu dai */
    atmel_pdmic_cpu_dai.capture.rate_min = rate_min;
    atmel_pdmic_cpu_dai.capture.rate_max = rate_max;
    ret = devm_snd_soc_register_component(
        dev,
        &atmel_pdmic_cpu_dai_component,
        &mut atmel_pdmic_cpu_dai,
        1,
    );
    if ret != 0 {
        dev_err(dev, b"could not register CPU DAI: %d\n\0".as_ptr() as *const i8, ret);
        return ret;
    }

    /* register platform */
    ret = devm_snd_dmaengine_pcm_register(dev, &atmel_pdmic_dmaengine_pcm_config, 0);
    if ret != 0 {
        dev_err(dev, b"could not register platform: %d\n\0".as_ptr() as *const i8, ret);
        return ret;
    }

    /* register sound card */
    card = devm_kzalloc(dev, core::mem::size_of::<snd_soc_card>(), GFP_KERNEL) as *mut snd_soc_card;
    if card.is_null() {
        ret = -ENOMEM;
        return ret;
    }

    snd_soc_card_set_drvdata(card, dd as *mut core::ffi::c_void);

    ret = atmel_pdmic_asoc_card_init(dev, card);
    if ret != 0 {
        dev_err(dev, b"failed to init sound card: %d\n\0".as_ptr() as *const i8, ret);
        return ret;
    }

    ret = devm_snd_soc_register_card(dev, card);
    if ret != 0 {
        dev_err(dev, b"failed to register sound card: %d\n\0".as_ptr() as *const i8, ret);
        return ret;
    }

    0
}

static mut atmel_pdmic_driver: platform_driver = platform_driver {
    driver: platform_driver_driver {
        name: b"atmel-pdmic\0".as_ptr() as *const i8,
        of_match_table: atmel_pdmic_of_match.as_ptr(),
        pm: unsafe { &snd_soc_pm_ops },
    },
    probe: Some(atmel_pdmic_probe),
};
/* module_platform_driver(atmel_pdmic_driver); */

/* MODULE_DESCRIPTION("Atmel PDMIC driver under ALSA SoC architecture"); */
/* MODULE_AUTHOR("Songjun Wu <songjun.wu@atmel.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
