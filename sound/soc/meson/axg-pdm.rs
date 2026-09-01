// SPDX-License-Identifier: (GPL-2.0 OR MIT)
//
// Copyright (c) 2018 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type bool_ = bool;
type u64_ = u64;

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

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
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub trigger:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub rate_min: c_uint,
    pub rate_max: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub reg_stride: c_int,
    pub max_register: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct platform_driver_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: platform_driver_driver,
}

unsafe extern "C" {
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_get_reg_stride(map: *mut regmap) -> c_uint;
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_width(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_uint;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
    fn WARN_ON(condition: bool_) -> bool_;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: c_uint = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 30;
const SNDRV_PCM_FMTBIT_S24_LE: u64 = 1 << 6;
const SNDRV_PCM_FMTBIT_S32_LE: u64 = 1 << 10;

const fn BIT(nr: c_uint) -> c_uint {
    1u32 << nr
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_uint {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const fn DIV_ROUND_UP_ULL(ll: u64_, d: c_ulong) -> c_uint {
    ((ll + d as u64 - 1) / d as u64) as c_uint
}

const PDM_CTRL: c_uint = 0x00;
const PDM_CTRL_EN: c_uint = BIT(31);
const PDM_CTRL_OUT_MODE: c_uint = BIT(29);
const PDM_CTRL_BYPASS_MODE: c_uint = BIT(28);
const PDM_CTRL_RST_FIFO: c_uint = BIT(16);
const PDM_CTRL_CHAN_RSTN_MASK: c_uint = GENMASK(15, 8);
const fn PDM_CTRL_CHAN_RSTN(x: c_uint) -> c_uint {
    x << 8
}
const PDM_CTRL_CHAN_EN_MASK: c_uint = GENMASK(7, 0);
const fn PDM_CTRL_CHAN_EN(x: c_uint) -> c_uint {
    x << 0
}
const PDM_HCIC_CTRL1: c_uint = 0x04;
const PDM_FILTER_EN: c_uint = BIT(31);
const PDM_HCIC_CTRL1_GAIN_SFT_MASK: c_uint = GENMASK(29, 24);
const fn PDM_HCIC_CTRL1_GAIN_SFT(x: c_uint) -> c_uint {
    x << 24
}
const PDM_HCIC_CTRL1_GAIN_MULT_MASK: c_uint = GENMASK(23, 16);
const fn PDM_HCIC_CTRL1_GAIN_MULT(x: c_uint) -> c_uint {
    x << 16
}
const PDM_HCIC_CTRL1_DSR_MASK: c_uint = GENMASK(8, 4);
const fn PDM_HCIC_CTRL1_DSR(x: c_uint) -> c_uint {
    x << 4
}
const PDM_HCIC_CTRL1_STAGE_NUM_MASK: c_uint = GENMASK(3, 0);
const fn PDM_HCIC_CTRL1_STAGE_NUM(x: c_uint) -> c_uint {
    x << 0
}
const PDM_HCIC_CTRL2: c_uint = 0x08;
const PDM_F1_CTRL: c_uint = 0x0c;
const PDM_LPF_ROUND_MODE_MASK: c_uint = GENMASK(17, 16);
const fn PDM_LPF_ROUND_MODE(x: c_uint) -> c_uint {
    x << 16
}
const PDM_LPF_DSR_MASK: c_uint = GENMASK(15, 12);
const fn PDM_LPF_DSR(x: c_uint) -> c_uint {
    x << 12
}
const PDM_LPF_STAGE_NUM_MASK: c_uint = GENMASK(8, 0);
const fn PDM_LPF_STAGE_NUM(x: c_uint) -> c_uint {
    x << 0
}
const PDM_LPF_MAX_STAGE: c_uint = 336;
const PDM_LPF_NUM: usize = 3;
const PDM_F2_CTRL: c_uint = 0x10;
const PDM_F3_CTRL: c_uint = 0x14;
const PDM_HPF_CTRL: c_uint = 0x18;
const PDM_HPF_SFT_STEPS_MASK: c_uint = GENMASK(20, 16);
const fn PDM_HPF_SFT_STEPS(x: c_uint) -> c_uint {
    x << 16
}
const PDM_HPF_OUT_FACTOR_MASK: c_uint = GENMASK(15, 0);
const fn PDM_HPF_OUT_FACTOR(x: c_uint) -> c_uint {
    x << 0
}
const PDM_CHAN_CTRL: c_uint = 0x1c;
const PDM_CHAN_CTRL_POINTER_WIDTH: c_uint = 8;
const PDM_CHAN_CTRL_POINTER_MAX: c_uint = (1 << PDM_CHAN_CTRL_POINTER_WIDTH) - 1;
const PDM_CHAN_CTRL_NUM: usize = 4;
const PDM_CHAN_CTRL1: c_uint = 0x20;
const PDM_COEFF_ADDR: c_uint = 0x24;
const PDM_COEFF_DATA: c_uint = 0x28;
const PDM_CLKG_CTRL: c_uint = 0x2c;
const PDM_STS: c_uint = 0x30;

#[repr(C)]
pub struct axg_pdm_lpf {
    pub ds: c_uint,
    pub round_mode: c_uint,
    pub tap: *const c_uint,
    pub tap_num: c_uint,
}

#[repr(C)]
pub struct axg_pdm_hcic {
    pub shift: c_uint,
    pub mult: c_uint,
    pub steps: c_uint,
    pub ds: c_uint,
}

#[repr(C)]
pub struct axg_pdm_hpf {
    pub out_factor: c_uint,
    pub steps: c_uint,
}

#[repr(C)]
pub struct axg_pdm_filters {
    pub hcic: axg_pdm_hcic,
    pub hpf: axg_pdm_hpf,
    pub lpf: [axg_pdm_lpf; PDM_LPF_NUM],
}

#[repr(C)]
pub struct axg_pdm_cfg {
    pub filters: *const axg_pdm_filters,
    pub sys_rate: c_uint,
}

#[repr(C)]
pub struct axg_pdm {
    pub cfg: *const axg_pdm_cfg,
    pub map: *mut regmap,
    pub dclk: *mut clk,
    pub sysclk: *mut clk,
    pub pclk: *mut clk,
}

unsafe extern "C" fn axg_pdm_enable(map: *mut regmap) {
    /* Reset AFIFO */
    unsafe {
        regmap_update_bits(map, PDM_CTRL, PDM_CTRL_RST_FIFO, PDM_CTRL_RST_FIFO);
        regmap_update_bits(map, PDM_CTRL, PDM_CTRL_RST_FIFO, 0);

        /* Enable PDM */
        regmap_update_bits(map, PDM_CTRL, PDM_CTRL_EN, PDM_CTRL_EN);
    }
}

unsafe extern "C" fn axg_pdm_disable(map: *mut regmap) {
    unsafe {
        regmap_update_bits(map, PDM_CTRL, PDM_CTRL_EN, 0);
    }
}

unsafe extern "C" fn axg_pdm_filters_enable(map: *mut regmap, enable: bool_) {
    let val: c_uint = if enable { PDM_FILTER_EN } else { 0 };

    unsafe {
        regmap_update_bits(map, PDM_HCIC_CTRL1, PDM_FILTER_EN, val);
        regmap_update_bits(map, PDM_F1_CTRL, PDM_FILTER_EN, val);
        regmap_update_bits(map, PDM_F2_CTRL, PDM_FILTER_EN, val);
        regmap_update_bits(map, PDM_F3_CTRL, PDM_FILTER_EN, val);
        regmap_update_bits(map, PDM_HPF_CTRL, PDM_FILTER_EN, val);
    }
}

unsafe extern "C" fn axg_pdm_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_: *mut axg_pdm = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_pdm };

    match cmd {
        SNDRV_PCM_TRIGGER_START | SNDRV_PCM_TRIGGER_RESUME | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            unsafe { axg_pdm_enable((*priv_).map) };
            0
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            unsafe { axg_pdm_disable((*priv_).map) };
            0
        }
        _ => -EINVAL,
    }
}

unsafe extern "C" fn axg_pdm_get_os(priv_: *mut axg_pdm) -> c_uint {
    let filters: *const axg_pdm_filters = unsafe { (*(*priv_).cfg).filters };
    let mut os: c_uint = unsafe { (*filters).hcic.ds };
    let mut i: usize;

    /*
     * The global oversampling factor is defined by the down sampling
     * factor applied by each filter (HCIC and LPFs)
     */

    i = 0;
    while i < PDM_LPF_NUM {
        os = os.wrapping_mul(unsafe { (*filters).lpf[i].ds });
        i += 1;
    }

    os
}

unsafe extern "C" fn axg_pdm_set_sysclk(
    priv_: *mut axg_pdm,
    os: c_uint,
    rate: c_uint,
) -> c_int {
    let sys_rate: c_uint = os
        .wrapping_mul(2)
        .wrapping_mul(rate)
        .wrapping_mul(PDM_CHAN_CTRL_POINTER_MAX);

    /*
     * Set the default system clock rate unless it is too fast for
     * the requested sample rate. In this case, the sample pointer
     * counter could overflow so set a lower system clock rate
     */
    if sys_rate < unsafe { (*(*priv_).cfg).sys_rate } {
        return unsafe { clk_set_rate((*priv_).sysclk, sys_rate as c_ulong) };
    }

    unsafe { clk_set_rate((*priv_).sysclk, (*(*priv_).cfg).sys_rate as c_ulong) }
}

unsafe extern "C" fn axg_pdm_set_sample_pointer(priv_: *mut axg_pdm) -> c_int {
    let spmax: c_uint;
    let sp: c_uint;
    let mut val: c_uint;
    let mut i: usize;

    /* Max sample counter value per half period of dclk */
    spmax = DIV_ROUND_UP_ULL(
        unsafe { clk_get_rate((*priv_).sysclk) } as u64_,
        unsafe { clk_get_rate((*priv_).dclk) }.wrapping_mul(2),
    );

    /* Check if sysclk is not too fast - should not happen */
    if unsafe { WARN_ON(spmax > PDM_CHAN_CTRL_POINTER_MAX) } {
        return -EINVAL;
    }

    /* Capture the data when we are at 75% of the half period */
    sp = spmax.wrapping_mul(3) / 4;

    i = 0;
    val = 0;
    while i < PDM_CHAN_CTRL_NUM {
        val |= sp << (PDM_CHAN_CTRL_POINTER_WIDTH * i as c_uint);
        i += 1;
    }

    unsafe {
        regmap_write((*priv_).map, PDM_CHAN_CTRL, val);
        regmap_write((*priv_).map, PDM_CHAN_CTRL1, val);
    }

    0
}

unsafe extern "C" fn axg_pdm_set_channel_mask(priv_: *mut axg_pdm, channels: c_uint) {
    let mask: c_uint = GENMASK(channels - 1, 0);

    /* Put all channel in reset */
    unsafe {
        regmap_update_bits((*priv_).map, PDM_CTRL, PDM_CTRL_CHAN_RSTN_MASK, 0);

        /* Take the necessary channels out of reset and enable them */
        regmap_update_bits(
            (*priv_).map,
            PDM_CTRL,
            PDM_CTRL_CHAN_RSTN_MASK | PDM_CTRL_CHAN_EN_MASK,
            PDM_CTRL_CHAN_RSTN(mask) | PDM_CTRL_CHAN_EN(mask),
        );
    }
}

unsafe extern "C" fn axg_pdm_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_: *mut axg_pdm = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_pdm };
    let os: c_uint = unsafe { axg_pdm_get_os(priv_) };
    let rate: c_uint = unsafe { params_rate(params) };
    let val: c_uint;
    let mut ret: c_int;

    match unsafe { params_width(params) } {
        24 => {
            val = PDM_CTRL_OUT_MODE;
        }
        32 => {
            val = 0;
        }
        _ => {
            unsafe { dev_err((*dai).dev, c"unsupported sample width\n".as_ptr()) };
            return -EINVAL;
        }
    }

    unsafe {
        regmap_update_bits((*priv_).map, PDM_CTRL, PDM_CTRL_OUT_MODE, val);
    }

    ret = unsafe { axg_pdm_set_sysclk(priv_, os, rate) };
    if ret != 0 {
        unsafe { dev_err((*dai).dev, c"failed to set system clock\n".as_ptr()) };
        return ret;
    }

    ret = unsafe { clk_set_rate((*priv_).dclk, rate.wrapping_mul(os) as c_ulong) };
    if ret != 0 {
        unsafe { dev_err((*dai).dev, c"failed to set dclk\n".as_ptr()) };
        return ret;
    }

    ret = unsafe { axg_pdm_set_sample_pointer(priv_) };
    if ret != 0 {
        unsafe { dev_err((*dai).dev, c"invalid clock setting\n".as_ptr()) };
        return ret;
    }

    unsafe { axg_pdm_set_channel_mask(priv_, params_channels(params)) };

    0
}

unsafe extern "C" fn axg_pdm_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let priv_: *mut axg_pdm = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_pdm };
    let ret: c_int;

    ret = unsafe { clk_prepare_enable((*priv_).dclk) };
    if ret != 0 {
        unsafe { dev_err((*dai).dev, c"enabling dclk failed\n".as_ptr()) };
        return ret;
    }

    /* Enable the filters */
    unsafe { axg_pdm_filters_enable((*priv_).map, true) };

    ret
}

unsafe extern "C" fn axg_pdm_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let priv_: *mut axg_pdm = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_pdm };

    unsafe {
        axg_pdm_filters_enable((*priv_).map, false);
        clk_disable_unprepare((*priv_).dclk);
    }
}

unsafe extern "C" fn axg_pdm_set_hcic_ctrl(priv_: *mut axg_pdm) {
    let hcic: *const axg_pdm_hcic = unsafe { &(*(*(*priv_).cfg).filters).hcic };
    let mut val: c_uint;

    val = PDM_HCIC_CTRL1_STAGE_NUM(unsafe { (*hcic).steps });
    val |= PDM_HCIC_CTRL1_DSR(unsafe { (*hcic).ds });
    val |= PDM_HCIC_CTRL1_GAIN_MULT(unsafe { (*hcic).mult });
    val |= PDM_HCIC_CTRL1_GAIN_SFT(unsafe { (*hcic).shift });

    unsafe {
        regmap_update_bits(
            (*priv_).map,
            PDM_HCIC_CTRL1,
            PDM_HCIC_CTRL1_STAGE_NUM_MASK
                | PDM_HCIC_CTRL1_DSR_MASK
                | PDM_HCIC_CTRL1_GAIN_MULT_MASK
                | PDM_HCIC_CTRL1_GAIN_SFT_MASK,
            val,
        );
    }
}

unsafe extern "C" fn axg_pdm_set_lpf_ctrl(priv_: *mut axg_pdm, index: c_uint) {
    let lpf: *const axg_pdm_lpf =
        unsafe { &(*(*(*priv_).cfg).filters).lpf[index as usize] };
    let offset: c_uint =
        index.wrapping_mul(unsafe { regmap_get_reg_stride((*priv_).map) }) + PDM_F1_CTRL;
    let mut val: c_uint;

    val = PDM_LPF_STAGE_NUM(unsafe { (*lpf).tap_num });
    val |= PDM_LPF_DSR(unsafe { (*lpf).ds });
    val |= PDM_LPF_ROUND_MODE(unsafe { (*lpf).round_mode });

    unsafe {
        regmap_update_bits(
            (*priv_).map,
            offset,
            PDM_LPF_STAGE_NUM_MASK | PDM_LPF_DSR_MASK | PDM_LPF_ROUND_MODE_MASK,
            val,
        );
    }
}

unsafe extern "C" fn axg_pdm_set_hpf_ctrl(priv_: *mut axg_pdm) {
    let hpf: *const axg_pdm_hpf = unsafe { &(*(*(*priv_).cfg).filters).hpf };
    let mut val: c_uint;

    val = PDM_HPF_OUT_FACTOR(unsafe { (*hpf).out_factor });
    val |= PDM_HPF_SFT_STEPS(unsafe { (*hpf).steps });

    unsafe {
        regmap_update_bits(
            (*priv_).map,
            PDM_HPF_CTRL,
            PDM_HPF_OUT_FACTOR_MASK | PDM_HPF_SFT_STEPS_MASK,
            val,
        );
    }
}

unsafe extern "C" fn axg_pdm_set_lpf_filters(priv_: *mut axg_pdm) -> c_int {
    let lpf: *const axg_pdm_lpf = unsafe { (*(*(*priv_).cfg).filters).lpf.as_ptr() };
    let mut count: c_uint = 0;
    let mut i: usize;
    let mut j: c_uint;

    i = 0;
    while i < PDM_LPF_NUM {
        count = count.wrapping_add(unsafe { (*lpf.add(i)).tap_num });
        i += 1;
    }

    /* Make sure the coeffs fit in the memory */
    if count >= PDM_LPF_MAX_STAGE {
        return -EINVAL;
    }

    /* Set the initial APB bus register address */
    unsafe {
        regmap_write((*priv_).map, PDM_COEFF_ADDR, 0);
    }

    /* Set the tap filter values of all 3 filters */
    i = 0;
    while i < PDM_LPF_NUM {
        unsafe { axg_pdm_set_lpf_ctrl(priv_, i as c_uint) };

        j = 0;
        while j < unsafe { (*lpf.add(i)).tap_num } {
            unsafe {
                regmap_write(
                    (*priv_).map,
                    PDM_COEFF_DATA,
                    *(*lpf.add(i)).tap.add(j as usize),
                );
            }
            j += 1;
        }
        i += 1;
    }

    0
}

unsafe extern "C" fn axg_pdm_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let priv_: *mut axg_pdm = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_pdm };
    let mut ret: c_int;

    ret = unsafe { clk_prepare_enable((*priv_).pclk) };
    if ret != 0 {
        unsafe { dev_err((*dai).dev, c"enabling pclk failed\n".as_ptr()) };
        return ret;
    }

    /*
     * sysclk must be set and enabled as well to access the pdm registers
     * Accessing the register w/o it will give a bus error.
     */
    ret = unsafe { clk_set_rate((*priv_).sysclk, (*(*priv_).cfg).sys_rate as c_ulong) };
    if ret != 0 {
        unsafe { dev_err((*dai).dev, c"setting sysclk failed\n".as_ptr()) };
        unsafe { clk_disable_unprepare((*priv_).pclk) };
        return ret;
    }

    ret = unsafe { clk_prepare_enable((*priv_).sysclk) };
    if ret != 0 {
        unsafe { dev_err((*dai).dev, c"enabling sysclk failed\n".as_ptr()) };
        unsafe { clk_disable_unprepare((*priv_).pclk) };
        return ret;
    }

    /* Make sure the device is initially disabled */
    unsafe { axg_pdm_disable((*priv_).map) };

    /* Make sure filter bypass is disabled */
    unsafe { regmap_update_bits((*priv_).map, PDM_CTRL, PDM_CTRL_BYPASS_MODE, 0) };

    /* Load filter settings */
    unsafe {
        axg_pdm_set_hcic_ctrl(priv_);
        axg_pdm_set_hpf_ctrl(priv_);
    }

    ret = unsafe { axg_pdm_set_lpf_filters(priv_) };
    if ret != 0 {
        unsafe { dev_err((*dai).dev, c"invalid filter configuration\n".as_ptr()) };
        unsafe {
            clk_disable_unprepare((*priv_).sysclk);
            clk_disable_unprepare((*priv_).pclk);
        }
        return ret;
    }

    0
}

unsafe extern "C" fn axg_pdm_dai_remove(dai: *mut snd_soc_dai) -> c_int {
    let priv_: *mut axg_pdm = unsafe { snd_soc_dai_get_drvdata(dai) as *mut axg_pdm };

    unsafe {
        clk_disable_unprepare((*priv_).sysclk);
        clk_disable_unprepare((*priv_).pclk);
    }

    0
}

static axg_pdm_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(axg_pdm_dai_probe),
    remove: Some(axg_pdm_dai_remove),
    trigger: Some(axg_pdm_trigger),
    hw_params: Some(axg_pdm_hw_params),
    startup: Some(axg_pdm_startup),
    shutdown: Some(axg_pdm_shutdown),
};

static mut axg_pdm_dai_drv: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"PDM".as_ptr(),
    capture: snd_soc_pcm_stream {
        stream_name: c"Capture".as_ptr(),
        channels_min: 1,
        channels_max: 8,
        rates: SNDRV_PCM_RATE_CONTINUOUS,
        rate_min: 5512,
        rate_max: 48000,
        formats: SNDRV_PCM_FMTBIT_S24_LE | SNDRV_PCM_FMTBIT_S32_LE,
    },
    ops: &axg_pdm_dai_ops,
};

static axg_pdm_component_drv: snd_soc_component_driver = snd_soc_component_driver {
    legacy_dai_naming: 1,
};

static axg_pdm_regmap_cfg: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    max_register: PDM_STS,
};

static lpf1_default_tap: [c_uint; 87] = [
    0x000014, 0xffffb2, 0xfffed9, 0xfffdce, 0xfffd45, 0xfffe32, 0x000147, 0x000645, 0x000b86,
    0x000e21, 0x000ae3, 0x000000, 0xffeece, 0xffdca8, 0xffd212, 0xffd7d1, 0xfff2a7, 0x001f4c,
    0x0050c2, 0x0072aa, 0x006ff1, 0x003c32, 0xffdc4e, 0xff6a18, 0xff0fef, 0xfefbaf, 0xff4c40,
    0x000000, 0x00ebc8, 0x01c077, 0x02209e, 0x01c1a4, 0x008e60, 0xfebe52, 0xfcd690, 0xfb8fa5,
    0xfba498, 0xfd9812, 0x0181ce, 0x06f5f3, 0x0d112f, 0x12a958, 0x169686, 0x18000e, 0x169686,
    0x12a958, 0x0d112f, 0x06f5f3, 0x0181ce, 0xfd9812, 0xfba498, 0xfb8fa5, 0xfcd690, 0xfebe52,
    0x008e60, 0x01c1a4, 0x02209e, 0x01c077, 0x00ebc8, 0x000000, 0xff4c40, 0xfefbaf, 0xff0fef,
    0xff6a18, 0xffdc4e, 0x003c32, 0x006ff1, 0x0072aa, 0x0050c2, 0x001f4c, 0xfff2a7, 0xffd7d1,
    0xffd212, 0xffdca8, 0xffeece, 0x000000, 0x000ae3, 0x000e21, 0x000b86, 0x000645, 0x000147,
    0xfffe32, 0xfffd45, 0xfffdce, 0xfffed9, 0xffffb2, 0x000014,
];

static lpf2_default_tap: [c_uint; 33] = [
    0x00050a, 0xfff004, 0x0002c1, 0x003c12, 0xffa818, 0xffc87d, 0x010aef, 0xff5223, 0xfebd93,
    0x028f41, 0xff5c0e, 0xfc63f8, 0x055f81, 0x000000, 0xf478a0, 0x11c5e3, 0x2ea74d, 0x11c5e3,
    0xf478a0, 0x000000, 0x055f81, 0xfc63f8, 0xff5c0e, 0x028f41, 0xfebd93, 0xff5223, 0x010aef,
    0xffc87d, 0xffa818, 0x003c12, 0x0002c1, 0xfff004, 0x00050a,
];

static lpf3_default_tap: [c_uint; 117] = [
    0x000000, 0x000081, 0x000000, 0xfffedb, 0x000000, 0x00022d, 0x000000, 0xfffc46, 0x000000,
    0x0005f7, 0x000000, 0xfff6eb, 0x000000, 0x000d4e, 0x000000, 0xffed1e, 0x000000, 0x001a1c,
    0x000000, 0xffdcb0, 0x000000, 0x002ede, 0x000000, 0xffc2d1, 0x000000, 0x004ebe, 0x000000,
    0xff9beb, 0x000000, 0x007dd7, 0x000000, 0xff633a, 0x000000, 0x00c1d2, 0x000000, 0xff11d5,
    0x000000, 0x012368, 0x000000, 0xfe9c45, 0x000000, 0x01b252, 0x000000, 0xfdebf6, 0x000000,
    0x0290b8, 0x000000, 0xfcca0d, 0x000000, 0x041d7c, 0x000000, 0xfa8152, 0x000000, 0x07e9c6,
    0x000000, 0xf28fb5, 0x000000, 0x28b216, 0x3fffde, 0x28b216, 0x000000, 0xf28fb5, 0x000000,
    0x07e9c6, 0x000000, 0xfa8152, 0x000000, 0x041d7c, 0x000000, 0xfcca0d, 0x000000, 0x0290b8,
    0x000000, 0xfdebf6, 0x000000, 0x01b252, 0x000000, 0xfe9c45, 0x000000, 0x012368, 0x000000,
    0xff11d5, 0x000000, 0x00c1d2, 0x000000, 0xff633a, 0x000000, 0x007dd7, 0x000000, 0xff9beb,
    0x000000, 0x004ebe, 0x000000, 0xffc2d1, 0x000000, 0x002ede, 0x000000, 0xffdcb0, 0x000000,
    0x001a1c, 0x000000, 0xffed1e, 0x000000, 0x000d4e, 0x000000, 0xfff6eb, 0x000000, 0x0005f7,
    0x000000, 0xfffc46, 0x000000, 0x00022d, 0x000000, 0xfffedb, 0x000000, 0x000081, 0x000000,
];

/*
 * These values are sane defaults for the axg platform:
 * - OS = 64
 * - Latency = 38700 (?)
 *
 * TODO: There is a lot of different HCIC, LPFs and HPF configurations possible.
 *       the configuration may depend on the dmic used by the platform, the
 *       expected tradeoff between latency and quality, etc ... If/When other
 *       settings are required, we should add a fw interface to this driver to
 *       load new filter settings.
 */
static axg_default_filters: axg_pdm_filters = axg_pdm_filters {
    hcic: axg_pdm_hcic {
        shift: 0x15,
        mult: 0x80,
        steps: 7,
        ds: 8,
    },
    hpf: axg_pdm_hpf {
        out_factor: 0x8000,
        steps: 13,
    },
    lpf: [
        axg_pdm_lpf {
            ds: 2,
            round_mode: 1,
            tap: lpf1_default_tap.as_ptr(),
            tap_num: lpf1_default_tap.len() as c_uint,
        },
        axg_pdm_lpf {
            ds: 2,
            round_mode: 0,
            tap: lpf2_default_tap.as_ptr(),
            tap_num: lpf2_default_tap.len() as c_uint,
        },
        axg_pdm_lpf {
            ds: 2,
            round_mode: 1,
            tap: lpf3_default_tap.as_ptr(),
            tap_num: lpf3_default_tap.len() as c_uint,
        },
    ],
};

static axg_pdm_config: axg_pdm_cfg = axg_pdm_cfg {
    filters: &axg_default_filters,
    sys_rate: 250000000,
};

static axg_pdm_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"amlogic,axg-pdm".as_ptr(),
        data: &axg_pdm_config as *const axg_pdm_cfg as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, axg_pdm_of_match); */

unsafe extern "C" fn axg_pdm_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = unsafe { &mut (*pdev).dev };
    let priv_: *mut axg_pdm;
    let regs: *mut c_void;

    priv_ = unsafe { devm_kzalloc(dev, size_of::<axg_pdm>(), GFP_KERNEL) as *mut axg_pdm };
    if priv_.is_null() {
        return -ENOMEM;
    }
    unsafe { platform_set_drvdata(pdev, priv_ as *mut c_void) };

    unsafe {
        (*priv_).cfg = of_device_get_match_data(dev) as *const axg_pdm_cfg;
    }
    if unsafe { (*priv_).cfg.is_null() } {
        unsafe { dev_err(dev, c"failed to match device\n".as_ptr()) };
        return -ENODEV;
    }

    regs = unsafe { devm_platform_ioremap_resource(pdev, 0) };
    if unsafe { IS_ERR(regs as *const c_void) } {
        return unsafe { PTR_ERR(regs as *const c_void) as c_int };
    }

    unsafe {
        (*priv_).map = devm_regmap_init_mmio(dev, regs, &axg_pdm_regmap_cfg);
    }
    if unsafe { IS_ERR((*priv_).map as *const c_void) } {
        unsafe {
            dev_err(
                dev,
                c"failed to init regmap: %ld\n".as_ptr(),
                PTR_ERR((*priv_).map as *const c_void),
            )
        };
        return unsafe { PTR_ERR((*priv_).map as *const c_void) as c_int };
    }

    unsafe {
        (*priv_).pclk = devm_clk_get(dev, c"pclk".as_ptr());
    }
    if unsafe { IS_ERR((*priv_).pclk as *const c_void) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR((*priv_).pclk as *const c_void),
                c"failed to get pclk\n".as_ptr(),
            )
        };
    }

    unsafe {
        (*priv_).dclk = devm_clk_get(dev, c"dclk".as_ptr());
    }
    if unsafe { IS_ERR((*priv_).dclk as *const c_void) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR((*priv_).dclk as *const c_void),
                c"failed to get dclk\n".as_ptr(),
            )
        };
    }

    unsafe {
        (*priv_).sysclk = devm_clk_get(dev, c"sysclk".as_ptr());
    }
    if unsafe { IS_ERR((*priv_).sysclk as *const c_void) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR((*priv_).sysclk as *const c_void),
                c"failed to get dclk\n".as_ptr(),
            )
        };
    }

    unsafe {
        devm_snd_soc_register_component(dev, &axg_pdm_component_drv, &mut axg_pdm_dai_drv, 1)
    }
}

static mut axg_pdm_pdrv: platform_driver = platform_driver {
    probe: Some(axg_pdm_probe),
    driver: platform_driver_driver {
        name: c"axg-pdm".as_ptr(),
        of_match_table: axg_pdm_of_match.as_ptr(),
    },
};
/* module_platform_driver(axg_pdm_pdrv); */

/* MODULE_DESCRIPTION("Amlogic AXG PDM Input driver"); */
/* MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
