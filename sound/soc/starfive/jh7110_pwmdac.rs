// SPDX-License-Identifier: GPL-2.0
/*
 * jh7110_pwmdac.rs -- StarFive JH7110 PWM-DAC driver
 *
 * Copyright (C) 2021-2023 StarFive Technology Co., Ltd.
 *
 * Authors: Jenny Zhang
 *	    Curry Zhang
 *	    Xingyu Wu <xingyu.wu@starfivetech.com>
 *	    Hal Feng <hal.feng@starfivetech.com>
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::null_mut;

type u16 = u16;
type u32 = u32;
type resource_size_t = usize;
type bool_ = bool;

const fn bit(n: u32) -> u32 {
    1u32 << n
}

const fn genmask(h: u32, l: u32) -> u32 {
    ((!0u32) << l) & ((!0u32) >> (31 - h))
}

const JH7110_PWMDAC_WDATA: c_int = 0x00;
const JH7110_PWMDAC_CTRL: c_int = 0x04;
const JH7110_PWMDAC_ENABLE: u32 = bit(0);
const JH7110_PWMDAC_SHIFT: u32 = bit(1);
const JH7110_PWMDAC_DUTY_CYCLE_SHIFT: u32 = 2;
const JH7110_PWMDAC_DUTY_CYCLE_MASK: u32 = genmask(3, 2);
const JH7110_PWMDAC_CNT_N_SHIFT: u32 = 4;
const JH7110_PWMDAC_CNT_N_MASK: u32 = genmask(12, 4);
const JH7110_PWMDAC_DATA_CHANGE: u32 = bit(13);
const JH7110_PWMDAC_DATA_MODE: u32 = bit(14);
const JH7110_PWMDAC_DATA_SHIFT_SHIFT: u32 = 15;
const JH7110_PWMDAC_DATA_SHIFT_MASK: u32 = genmask(17, 15);

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_int = 0;
const DMA_SLAVE_BUSWIDTH_2_BYTES: c_int = 2;
const DMA_SLAVE_BUSWIDTH_4_BYTES: c_int = 4;
const SND_SOC_TRIGGER_ORDER_LDC: c_int = 0;
const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;
const SNDRV_PCM_RATE_8000_48000: u32 = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 0;

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum JH7110_PWMDAC_SHIFT_VAL {
    PWMDAC_SHIFT_8 = 0,
    PWMDAC_SHIFT_10 = 1,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum JH7110_PWMDAC_DUTY_CYCLE_VAL {
    PWMDAC_CYCLE_LEFT = 0,
    PWMDAC_CYCLE_RIGHT = 1,
    PWMDAC_CYCLE_CENTER = 2,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum JH7110_PWMDAC_CNT_N_VAL {
    PWMDAC_SAMPLE_CNT_1 = 1,
    PWMDAC_SAMPLE_CNT_2 = 2,
    PWMDAC_SAMPLE_CNT_3 = 3,
    PWMDAC_SAMPLE_CNT_512 = 512, /* max */
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum JH7110_PWMDAC_DATA_CHANGE_VAL {
    NO_CHANGE = 0,
    CHANGE = 1,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum JH7110_PWMDAC_DATA_MODE_VAL {
    UNSIGNED_DATA = 0,
    INVERTER_DATA_MSB = 1,
}

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq)]
enum JH7110_PWMDAC_DATA_SHIFT_VAL {
    PWMDAC_DATA_LEFT_SHIFT_BIT_0 = 0,
    PWMDAC_DATA_LEFT_SHIFT_BIT_1 = 1,
    PWMDAC_DATA_LEFT_SHIFT_BIT_2 = 2,
    PWMDAC_DATA_LEFT_SHIFT_BIT_3 = 3,
    PWMDAC_DATA_LEFT_SHIFT_BIT_4 = 4,
    PWMDAC_DATA_LEFT_SHIFT_BIT_5 = 5,
    PWMDAC_DATA_LEFT_SHIFT_BIT_6 = 6,
    PWMDAC_DATA_LEFT_SHIFT_BIT_7 = 7,
}

#[repr(C)]
struct jh7110_pwmdac_cfg {
    shift: JH7110_PWMDAC_SHIFT_VAL,
    duty_cycle: JH7110_PWMDAC_DUTY_CYCLE_VAL,
    cnt_n: u16,
    data_change: JH7110_PWMDAC_DATA_CHANGE_VAL,
    data_mode: JH7110_PWMDAC_DATA_MODE_VAL,
    data_shift: JH7110_PWMDAC_DATA_SHIFT_VAL,
}

#[repr(C)]
struct clk_bulk_data {
    id: *const c_char,
    clk: *mut clk,
}

#[repr(C)]
struct snd_dmaengine_dai_dma_data {
    addr: resource_size_t,
    addr_width: c_int,
    fifo_size: u32,
    maxburst: u32,
}

#[repr(C)]
struct jh7110_pwmdac_dev {
    base: *mut c_void,
    mapbase: resource_size_t,
    cfg: jh7110_pwmdac_cfg,
    clks: [clk_bulk_data; 2],
    rst_apb: *mut reset_control,
    dev: *mut device,
    play_dma_data: snd_dmaengine_dai_dma_data,
    saved_ctrl: u32,
}

#[repr(C)]
struct resource {
    start: resource_size_t,
}

#[repr(C)]
struct snd_soc_dai_link {
    trigger_stop: c_int,
}

#[repr(C)]
struct snd_soc_pcm_runtime {
    dai_link: *mut snd_soc_dai_link,
}

#[repr(C)]
struct snd_soc_dai {
    dev: *mut device,
}

#[repr(C)]
struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct clk {
    _private: [u8; 0],
}

#[repr(C)]
struct reset_control {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    trigger: Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
}

#[repr(C)]
struct snd_soc_component_driver {
    name: *const c_char,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    channels_min: u32,
    channels_max: u32,
    rates: u32,
    formats: u64,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    id: c_int,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    system_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    system_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct platform_driver_inner {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct platform_driver {
    driver: platform_driver_inner,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

unsafe extern "C" {
    fn writel(val: u32, addr: *mut c_void);
    fn readl(addr: *mut c_void) -> u32;
    fn snd_soc_substream_to_rtd(substream: *mut snd_pcm_substream) -> *mut snd_soc_pcm_runtime;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn params_channels(params: *mut snd_pcm_hw_params) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...) -> c_int;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_soc_dai_set_drvdata(dai: *mut snd_soc_dai, data: *mut c_void);
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn clk_bulk_prepare_enable(num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    fn clk_bulk_disable_unprepare(num_clks: c_int, clks: *mut clk_bulk_data);
    fn reset_control_deassert(rst: *mut reset_control) -> c_int;
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool_;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_int) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_int,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool_;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_clk_bulk_get(dev: *mut device, num_clks: c_int, clks: *mut clk_bulk_data) -> c_int;
    fn devm_reset_control_get_exclusive(
        dev: *mut device,
        id: *const c_char,
    ) -> *mut reset_control;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const c_void,
        flags: c_int,
    ) -> c_int;
}

#[inline]
unsafe fn jh7110_pwmdac_write_reg(io_base: *mut c_void, reg: c_int, val: u32) {
    unsafe {
        writel(val, (io_base as *mut u8).offset(reg as isize) as *mut c_void);
    }
}

#[inline]
unsafe fn jh7110_pwmdac_read_reg(io_base: *mut c_void, reg: c_int) -> u32 {
    unsafe { readl((io_base as *mut u8).offset(reg as isize) as *mut c_void) }
}

unsafe extern "C" fn jh7110_pwmdac_set_enable(dev: *mut jh7110_pwmdac_dev, enable: bool_) {
    let mut value: u32;

    unsafe {
        value = jh7110_pwmdac_read_reg((*dev).base, JH7110_PWMDAC_CTRL);
        if enable {
            value |= JH7110_PWMDAC_ENABLE;
        } else {
            value &= !JH7110_PWMDAC_ENABLE;
        }

        jh7110_pwmdac_write_reg((*dev).base, JH7110_PWMDAC_CTRL, value);
    }
}

unsafe extern "C" fn jh7110_pwmdac_set_shift(dev: *mut jh7110_pwmdac_dev) {
    let mut value: u32;

    unsafe {
        value = jh7110_pwmdac_read_reg((*dev).base, JH7110_PWMDAC_CTRL);
        if (*dev).cfg.shift == JH7110_PWMDAC_SHIFT_VAL::PWMDAC_SHIFT_8 {
            value &= !JH7110_PWMDAC_SHIFT;
        } else if (*dev).cfg.shift == JH7110_PWMDAC_SHIFT_VAL::PWMDAC_SHIFT_10 {
            value |= JH7110_PWMDAC_SHIFT;
        }

        jh7110_pwmdac_write_reg((*dev).base, JH7110_PWMDAC_CTRL, value);
    }
}

unsafe extern "C" fn jh7110_pwmdac_set_duty_cycle(dev: *mut jh7110_pwmdac_dev) {
    let mut value: u32;

    unsafe {
        value = jh7110_pwmdac_read_reg((*dev).base, JH7110_PWMDAC_CTRL);
        value &= !JH7110_PWMDAC_DUTY_CYCLE_MASK;
        value |= (((*dev).cfg.duty_cycle as u32) & 0x3) << JH7110_PWMDAC_DUTY_CYCLE_SHIFT;

        jh7110_pwmdac_write_reg((*dev).base, JH7110_PWMDAC_CTRL, value);
    }
}

unsafe extern "C" fn jh7110_pwmdac_set_cnt_n(dev: *mut jh7110_pwmdac_dev) {
    let mut value: u32;

    unsafe {
        value = jh7110_pwmdac_read_reg((*dev).base, JH7110_PWMDAC_CTRL);
        value &= !JH7110_PWMDAC_CNT_N_MASK;
        value |= (((*dev).cfg.cnt_n.wrapping_sub(1) as u32) & 0x1ff) << JH7110_PWMDAC_CNT_N_SHIFT;

        jh7110_pwmdac_write_reg((*dev).base, JH7110_PWMDAC_CTRL, value);
    }
}

unsafe extern "C" fn jh7110_pwmdac_set_data_change(dev: *mut jh7110_pwmdac_dev) {
    let mut value: u32;

    unsafe {
        value = jh7110_pwmdac_read_reg((*dev).base, JH7110_PWMDAC_CTRL);
        if (*dev).cfg.data_change == JH7110_PWMDAC_DATA_CHANGE_VAL::NO_CHANGE {
            value &= !JH7110_PWMDAC_DATA_CHANGE;
        } else if (*dev).cfg.data_change == JH7110_PWMDAC_DATA_CHANGE_VAL::CHANGE {
            value |= JH7110_PWMDAC_DATA_CHANGE;
        }

        jh7110_pwmdac_write_reg((*dev).base, JH7110_PWMDAC_CTRL, value);
    }
}

unsafe extern "C" fn jh7110_pwmdac_set_data_mode(dev: *mut jh7110_pwmdac_dev) {
    let mut value: u32;

    unsafe {
        value = jh7110_pwmdac_read_reg((*dev).base, JH7110_PWMDAC_CTRL);
        if (*dev).cfg.data_mode == JH7110_PWMDAC_DATA_MODE_VAL::UNSIGNED_DATA {
            value &= !JH7110_PWMDAC_DATA_MODE;
        } else if (*dev).cfg.data_mode == JH7110_PWMDAC_DATA_MODE_VAL::INVERTER_DATA_MSB {
            value |= JH7110_PWMDAC_DATA_MODE;
        }

        jh7110_pwmdac_write_reg((*dev).base, JH7110_PWMDAC_CTRL, value);
    }
}

unsafe extern "C" fn jh7110_pwmdac_set_data_shift(dev: *mut jh7110_pwmdac_dev) {
    let mut value: u32;

    unsafe {
        value = jh7110_pwmdac_read_reg((*dev).base, JH7110_PWMDAC_CTRL);
        value &= !JH7110_PWMDAC_DATA_SHIFT_MASK;
        value |= (((*dev).cfg.data_shift as u32) & 0x7) << JH7110_PWMDAC_DATA_SHIFT_SHIFT;

        jh7110_pwmdac_write_reg((*dev).base, JH7110_PWMDAC_CTRL, value);
    }
}

unsafe extern "C" fn jh7110_pwmdac_set(dev: *mut jh7110_pwmdac_dev) {
    unsafe {
        jh7110_pwmdac_set_shift(dev);
        jh7110_pwmdac_set_duty_cycle(dev);
        jh7110_pwmdac_set_cnt_n(dev);
        jh7110_pwmdac_set_enable(dev, true);

        jh7110_pwmdac_set_data_change(dev);
        jh7110_pwmdac_set_data_mode(dev);
        jh7110_pwmdac_set_data_shift(dev);
    }
}

unsafe extern "C" fn jh7110_pwmdac_stop(dev: *mut jh7110_pwmdac_dev) {
    unsafe {
        jh7110_pwmdac_set_enable(dev, false);
    }
}

unsafe extern "C" fn jh7110_pwmdac_startup(
    substream: *mut snd_pcm_substream,
    _dai: *mut snd_soc_dai,
) -> c_int {
    unsafe {
        let rtd: *mut snd_soc_pcm_runtime = snd_soc_substream_to_rtd(substream);
        let dai_link: *mut snd_soc_dai_link = (*rtd).dai_link;

        (*dai_link).trigger_stop = SND_SOC_TRIGGER_ORDER_LDC;
    }

    0
}

unsafe extern "C" fn jh7110_pwmdac_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev: *mut jh7110_pwmdac_dev;
    let mut core_clk_rate: c_ulong;
    let ret: c_int;

    unsafe {
        dev = dev_get_drvdata((*dai).dev) as *mut jh7110_pwmdac_dev;

        match params_rate(params) {
            8000 => {
                (*dev).cfg.cnt_n = JH7110_PWMDAC_CNT_N_VAL::PWMDAC_SAMPLE_CNT_3 as u16;
                core_clk_rate = 6144000;
            }
            11025 => {
                (*dev).cfg.cnt_n = JH7110_PWMDAC_CNT_N_VAL::PWMDAC_SAMPLE_CNT_2 as u16;
                core_clk_rate = 5644800;
            }
            16000 => {
                (*dev).cfg.cnt_n = JH7110_PWMDAC_CNT_N_VAL::PWMDAC_SAMPLE_CNT_3 as u16;
                core_clk_rate = 12288000;
            }
            22050 => {
                (*dev).cfg.cnt_n = JH7110_PWMDAC_CNT_N_VAL::PWMDAC_SAMPLE_CNT_1 as u16;
                core_clk_rate = 5644800;
            }
            32000 => {
                (*dev).cfg.cnt_n = JH7110_PWMDAC_CNT_N_VAL::PWMDAC_SAMPLE_CNT_1 as u16;
                core_clk_rate = 8192000;
            }
            44100 => {
                (*dev).cfg.cnt_n = JH7110_PWMDAC_CNT_N_VAL::PWMDAC_SAMPLE_CNT_1 as u16;
                core_clk_rate = 11289600;
            }
            48000 => {
                (*dev).cfg.cnt_n = JH7110_PWMDAC_CNT_N_VAL::PWMDAC_SAMPLE_CNT_1 as u16;
                core_clk_rate = 12288000;
            }
            _ => {
                dev_err(
                    (*dai).dev,
                    b"%d rate not supported\n\0".as_ptr() as *const c_char,
                    params_rate(params),
                );
                return -EINVAL;
            }
        }

        match params_channels(params) {
            1 => {
                (*dev).play_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_2_BYTES;
            }
            2 => {
                (*dev).play_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
            }
            _ => {
                dev_err(
                    (*dai).dev,
                    b"%d channels not supported\n\0".as_ptr() as *const c_char,
                    params_channels(params),
                );
                return -EINVAL;
            }
        }

        /*
         * The clock rate always rounds down when using clk_set_rate()
         * so increase the rate a bit
         */
        core_clk_rate = core_clk_rate.wrapping_add(64);
        jh7110_pwmdac_set(dev);

        ret = clk_set_rate((*dev).clks[1].clk, core_clk_rate);
        if ret != 0 {
            return dev_err_probe(
                (*dai).dev,
                ret,
                b"failed to set rate %lu for core clock\n\0".as_ptr() as *const c_char,
                core_clk_rate,
            );
        }
    }

    0
}

unsafe extern "C" fn jh7110_pwmdac_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let dev: *mut jh7110_pwmdac_dev;
    let mut ret: c_int = 0;

    unsafe {
        dev = snd_soc_dai_get_drvdata(dai) as *mut jh7110_pwmdac_dev;

        match cmd {
            SNDRV_PCM_TRIGGER_START
            | SNDRV_PCM_TRIGGER_RESUME
            | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
                jh7110_pwmdac_set(dev);
            }
            SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
                jh7110_pwmdac_stop(dev);
            }
            _ => {
                ret = -EINVAL;
            }
        }
    }

    ret
}

unsafe extern "C" fn jh7110_pwmdac_crg_enable(
    dev: *mut jh7110_pwmdac_dev,
    enable: bool_,
) -> c_int {
    let mut ret: c_int;

    unsafe {
        if enable {
            ret = clk_bulk_prepare_enable((*dev).clks.len() as c_int, (*dev).clks.as_mut_ptr());
            if ret != 0 {
                return dev_err_probe(
                    (*dev).dev,
                    ret,
                    b"failed to enable pwmdac clocks\n\0".as_ptr() as *const c_char,
                );
            }

            ret = reset_control_deassert((*dev).rst_apb);
            if ret != 0 {
                dev_err(
                    (*dev).dev,
                    b"failed to deassert pwmdac apb reset\n\0".as_ptr() as *const c_char,
                );
                clk_bulk_disable_unprepare((*dev).clks.len() as c_int, (*dev).clks.as_mut_ptr());
                return ret;
            }
        } else {
            clk_bulk_disable_unprepare((*dev).clks.len() as c_int, (*dev).clks.as_mut_ptr());
        }
    }

    0
}

unsafe extern "C" fn jh7110_pwmdac_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    unsafe {
        let dev: *mut jh7110_pwmdac_dev = dev_get_drvdata((*dai).dev) as *mut jh7110_pwmdac_dev;

        snd_soc_dai_init_dma_data(dai, &mut (*dev).play_dma_data, null_mut());
        snd_soc_dai_set_drvdata(dai, dev as *mut c_void);
    }

    0
}

static jh7110_pwmdac_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(jh7110_pwmdac_dai_probe),
    startup: Some(jh7110_pwmdac_startup),
    hw_params: Some(jh7110_pwmdac_hw_params),
    trigger: Some(jh7110_pwmdac_trigger),
};

static jh7110_pwmdac_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"jh7110-pwmdac\0".as_ptr() as *const c_char,
};

static mut jh7110_pwmdac_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: b"jh7110-pwmdac\0".as_ptr() as *const c_char,
    id: 0,
    playback: snd_soc_pcm_stream {
        channels_min: 1,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_48000,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    ops: &jh7110_pwmdac_dai_ops,
};

unsafe extern "C" fn jh7110_pwmdac_runtime_suspend(dev: *mut device) -> c_int {
    unsafe {
        let pwmdac: *mut jh7110_pwmdac_dev = dev_get_drvdata(dev) as *mut jh7110_pwmdac_dev;

        jh7110_pwmdac_crg_enable(pwmdac, false)
    }
}

unsafe extern "C" fn jh7110_pwmdac_runtime_resume(dev: *mut device) -> c_int {
    unsafe {
        let pwmdac: *mut jh7110_pwmdac_dev = dev_get_drvdata(dev) as *mut jh7110_pwmdac_dev;

        jh7110_pwmdac_crg_enable(pwmdac, true)
    }
}

unsafe extern "C" fn jh7110_pwmdac_system_suspend(dev: *mut device) -> c_int {
    unsafe {
        let pwmdac: *mut jh7110_pwmdac_dev = dev_get_drvdata(dev) as *mut jh7110_pwmdac_dev;

        /* save the CTRL register value */
        (*pwmdac).saved_ctrl = jh7110_pwmdac_read_reg((*pwmdac).base, JH7110_PWMDAC_CTRL);
        pm_runtime_force_suspend(dev)
    }
}

unsafe extern "C" fn jh7110_pwmdac_system_resume(dev: *mut device) -> c_int {
    let ret: c_int;

    unsafe {
        let pwmdac: *mut jh7110_pwmdac_dev = dev_get_drvdata(dev) as *mut jh7110_pwmdac_dev;

        ret = pm_runtime_force_resume(dev);
        if ret != 0 {
            return ret;
        }

        /* restore the CTRL register value */
        jh7110_pwmdac_write_reg((*pwmdac).base, JH7110_PWMDAC_CTRL, (*pwmdac).saved_ctrl);
    }

    0
}

static jh7110_pwmdac_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(jh7110_pwmdac_runtime_suspend),
    runtime_resume: Some(jh7110_pwmdac_runtime_resume),
    system_suspend: Some(jh7110_pwmdac_system_suspend),
    system_resume: Some(jh7110_pwmdac_system_resume),
};

unsafe extern "C" fn jh7110_pwmdac_init_params(dev: *mut jh7110_pwmdac_dev) {
    unsafe {
        (*dev).cfg.shift = JH7110_PWMDAC_SHIFT_VAL::PWMDAC_SHIFT_8;
        (*dev).cfg.duty_cycle = JH7110_PWMDAC_DUTY_CYCLE_VAL::PWMDAC_CYCLE_CENTER;
        (*dev).cfg.cnt_n = JH7110_PWMDAC_CNT_N_VAL::PWMDAC_SAMPLE_CNT_1 as u16;
        (*dev).cfg.data_change = JH7110_PWMDAC_DATA_CHANGE_VAL::NO_CHANGE;
        (*dev).cfg.data_mode = JH7110_PWMDAC_DATA_MODE_VAL::INVERTER_DATA_MSB;
        (*dev).cfg.data_shift = JH7110_PWMDAC_DATA_SHIFT_VAL::PWMDAC_DATA_LEFT_SHIFT_BIT_0;

        (*dev).play_dma_data.addr = (*dev).mapbase + JH7110_PWMDAC_WDATA as usize;
        (*dev).play_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*dev).play_dma_data.fifo_size = 1;
        (*dev).play_dma_data.maxburst = 16;
    }
}

unsafe extern "C" fn jh7110_pwmdac_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut jh7110_pwmdac_dev;
    let mut res: *mut resource = null_mut();
    let mut ret: c_int;

    unsafe {
        dev = devm_kzalloc(
            &mut (*pdev).dev,
            size_of::<jh7110_pwmdac_dev>(),
            GFP_KERNEL,
        ) as *mut jh7110_pwmdac_dev;
        if dev.is_null() {
            return -ENOMEM;
        }

        (*dev).base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
        if IS_ERR((*dev).base as *const c_void) {
            return PTR_ERR((*dev).base as *const c_void);
        }

        (*dev).mapbase = (*res).start;

        (*dev).clks[0].id = b"apb\0".as_ptr() as *const c_char;
        (*dev).clks[1].id = b"core\0".as_ptr() as *const c_char;

        ret = devm_clk_bulk_get(&mut (*pdev).dev, (*dev).clks.len() as c_int, (*dev).clks.as_mut_ptr());
        if ret != 0 {
            return dev_err_probe(
                &mut (*pdev).dev,
                ret,
                b"failed to get pwmdac clocks\n\0".as_ptr() as *const c_char,
            );
        }

        (*dev).rst_apb = devm_reset_control_get_exclusive(&mut (*pdev).dev, null_mut());
        if IS_ERR((*dev).rst_apb as *const c_void) {
            return dev_err_probe(
                &mut (*pdev).dev,
                PTR_ERR((*dev).rst_apb as *const c_void),
                b"failed to get pwmdac apb reset\n\0".as_ptr() as *const c_char,
            );
        }

        jh7110_pwmdac_init_params(dev);

        (*dev).dev = &mut (*pdev).dev;
        dev_set_drvdata(&mut (*pdev).dev, dev as *mut c_void);
        ret = devm_snd_soc_register_component(
            &mut (*pdev).dev,
            &jh7110_pwmdac_component,
            &raw mut jh7110_pwmdac_dai,
            1,
        );
        if ret != 0 {
            return ret;
        }

        ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, null_mut(), 0);
        if ret != 0 {
            return ret;
        }

        pm_runtime_enable((*dev).dev);
        if !pm_runtime_enabled(&mut (*pdev).dev) {
            ret = jh7110_pwmdac_runtime_resume(&mut (*pdev).dev);
            if ret != 0 {
                pm_runtime_disable(&mut (*pdev).dev);
                return ret;
            }
        }
    }

    0
}

unsafe extern "C" fn jh7110_pwmdac_remove(pdev: *mut platform_device) {
    unsafe {
        pm_runtime_disable(&mut (*pdev).dev);
    }
}

static jh7110_pwmdac_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"starfive,jh7110-pwmdac\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, jh7110_pwmdac_of_match); */

static mut jh7110_pwmdac_driver: platform_driver = platform_driver {
    driver: platform_driver_inner {
        name: b"jh7110-pwmdac\0".as_ptr() as *const c_char,
        of_match_table: jh7110_pwmdac_of_match.as_ptr(),
        pm: &jh7110_pwmdac_pm_ops,
    },
    probe: Some(jh7110_pwmdac_probe),
    remove: Some(jh7110_pwmdac_remove),
};
/* module_platform_driver(jh7110_pwmdac_driver); */

/* MODULE_AUTHOR("Jenny Zhang"); */
/* MODULE_AUTHOR("Curry Zhang"); */
/* MODULE_AUTHOR("Xingyu Wu <xingyu.wu@starfivetech.com>"); */
/* MODULE_AUTHOR("Hal Feng <hal.feng@starfivetech.com>"); */
/* MODULE_DESCRIPTION("StarFive JH7110 PWM-DAC driver"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
