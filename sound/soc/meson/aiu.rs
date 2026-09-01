// SPDX-License-Identifier: GPL-2.0
//
// Copyright (c) 2020 BayLibre, SAS.
// Author: Jerome Brunet <jbrunet@baylibre.com>

// C dependencies:
// linux/bitfield.h, linux/clk.h, linux/module.h, linux/of_platform.h,
// linux/regmap.h, linux/reset.h, sound/soc.h, sound/soc-dai.h,
// dt-bindings/sound/meson-aiu.h, aiu.h, aiu-fifo.h

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const AIU_I2S_MISC_958_SRC_SHIFT: c_uint = 3;

const AIU_WIDGET_SPDIF_SRC_SEL: usize = 0;
const AIU_WIDGET_I2S_FORMATTER: usize = 1;

extern "C" {
    static aiu_spdif_encode_sel_enum: soc_enum;
    static aiu_fifo_i2s_dai_ops: snd_soc_dai_ops;
    static aiu_fifo_spdif_dai_ops: snd_soc_dai_ops;
    static aiu_encoder_i2s_dai_ops: snd_soc_dai_ops;
    static aiu_encoder_spdif_dai_ops: snd_soc_dai_ops;
    static aiu_formatter_i2s_ops: gx_formatter_ops;

    fn gx_formatter_event(
        w: *mut snd_soc_dapm_widget,
        kcontrol: *mut snd_kcontrol,
        event: c_int,
    ) -> c_int;
    fn aiu_fifo_pointer(substream: *mut snd_pcm_substream) -> snd_pcm_uframes_t;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn devm_kcalloc(
        dev: *mut device,
        n: usize,
        size: usize,
        flags: gfp_t,
    ) -> *mut c_void;
    fn devm_clk_bulk_get(
        dev: *mut device,
        num: c_int,
        clks: *mut clk_bulk_data,
    ) -> c_int;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn dev_err_probe(dev: *mut device, err: c_long, fmt: *const c_char, ...) -> c_int;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn device_reset(dev: *mut device) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: gfp_t) -> *mut c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn platform_get_irq_byname(pdev: *mut platform_device, name: *const c_char) -> c_int;
    fn gx_formatter_create(
        dev: *mut device,
        widget: *mut snd_soc_dapm_widget,
        drv: *const gx_formatter_driver,
        map: *mut regmap,
    ) -> c_int;
    fn snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn aiu_hdmi_ctrl_register_component(dev: *mut device) -> c_int;
    fn aiu_acodec_ctrl_register_component(dev: *mut device) -> c_int;
    fn gx_formatter_free(widget: *mut snd_soc_dapm_widget);
    fn snd_soc_unregister_component(dev: *mut device);
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
}

type gfp_t = c_uint;
type snd_pcm_uframes_t = usize;

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_kcontrol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct gx_formatter_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct soc_enum {
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
pub struct of_phandle_args {
    pub args_count: c_int,
    pub args: [c_uint; 16],
}

#[repr(C)]
pub struct snd_soc_dai_driver_name {
    pub name: *const c_char,
}

#[repr(C)]
pub struct snd_soc_dai {
    pub driver: *mut snd_soc_dai_driver_name,
}

#[repr(C)]
pub struct snd_soc_component {
    pub num_dai: c_int,
}

#[repr(C)]
pub struct snd_kcontrol_new {
    pub name: *const c_char,
    pub private_value: usize,
}

#[repr(C)]
pub struct snd_soc_dapm_widget {
    pub id: c_int,
    pub name: *const c_char,
    pub reg: c_int,
    pub shift: c_int,
    pub invert: c_int,
    pub kcontrol_news: *const snd_kcontrol_new,
    pub num_kcontrols: c_int,
    pub event: Option<unsafe extern "C" fn(*mut snd_soc_dapm_widget, *mut snd_kcontrol, c_int) -> c_int>,
    pub event_flags: c_int,
}

#[repr(C)]
pub struct snd_soc_dapm_route {
    pub sink: *const c_char,
    pub control: *const c_char,
    pub source: *const c_char,
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
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub dapm_widgets: *mut snd_soc_dapm_widget,
    pub num_dapm_widgets: c_uint,
    pub dapm_routes: *const snd_soc_dapm_route,
    pub num_dapm_routes: c_uint,
    pub of_xlate_dai_name: Option<
        unsafe extern "C" fn(
            *mut snd_soc_component,
            *const of_phandle_args,
            *mut *const c_char,
        ) -> c_int,
    >,
    pub pointer: Option<unsafe extern "C" fn(*mut snd_pcm_substream) -> snd_pcm_uframes_t>,
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_component) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut snd_soc_component)>,
    // CONFIG_DEBUG_FS: debugfs_prefix = "cpu"
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub val_bits: c_int,
    pub reg_stride: c_int,
    pub max_register: c_uint,
}

#[repr(C)]
pub struct gx_formatter_driver {
    pub regmap_cfg: *const regmap_config,
    pub ops: *const gx_formatter_ops,
}

#[repr(C)]
pub struct clk_bulk_data {
    pub id: *const c_char,
    pub clk: *mut clk,
}

#[repr(C)]
pub struct aiu_interface {
    pub clks: *mut clk_bulk_data,
    pub clk_num: c_uint,
    pub irq: c_int,
}

#[repr(C)]
pub struct aiu_platform_data {
    pub has_acodec: bool,
    pub has_clk_ctrl_more_i2s_div: bool,
}

#[repr(C)]
pub struct aiu {
    pub i2s: aiu_interface,
    pub spdif: aiu_interface,
    pub spdif_mclk: *mut clk,
    pub platform: *const aiu_platform_data,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
    pub data: *const c_void,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: platform_driver_inner,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const ENODEV: c_int = 19;
const GFP_KERNEL: gfp_t = 0;
const AIU_CPU: c_uint = 0;
const PCLK: usize = 0;
const AOCLK: usize = 1;
const MCLK: usize = 2;
const MIXER: usize = 3;
const CPU_I2S_FIFO: usize = 0;
const CPU_SPDIF_FIFO: usize = 1;
const CPU_I2S_ENCODER: usize = 2;
const CPU_SPDIF_ENCODER: usize = 3;
const AIU_I2S_MISC: c_uint = 0;
const SND_SOC_NOPM: c_int = -1;
const SND_SOC_DAPM_PRE_PMU: c_int = 1;
const SND_SOC_DAPM_PRE_PMD: c_int = 2;
const SNDRV_PCM_RATE_CONTINUOUS: c_uint = 1 << 30;
const SNDRV_PCM_RATE_8000_192000: c_uint = 0xffff;
const SNDRV_PCM_RATE_32000: c_uint = 1 << 0;
const SNDRV_PCM_RATE_44100: c_uint = 1 << 1;
const SNDRV_PCM_RATE_48000: c_uint = 1 << 2;
const SNDRV_PCM_RATE_88200: c_uint = 1 << 3;
const SNDRV_PCM_RATE_96000: c_uint = 1 << 4;
const SNDRV_PCM_RATE_176400: c_uint = 1 << 5;
const SNDRV_PCM_RATE_192000: c_uint = 1 << 6;
const AIU_FORMATS: u64 = 0;

unsafe fn IS_ERR<T>(ptr: *const T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *const T) -> c_long {
    ptr as c_long
}

static aiu_spdif_encode_sel_texts: [*const c_char; 2] = [
    b"SPDIF\0".as_ptr() as *const c_char,
    b"I2S\0".as_ptr() as *const c_char,
];

// static SOC_ENUM_SINGLE_DECL(aiu_spdif_encode_sel_enum, AIU_I2S_MISC,
//                             AIU_I2S_MISC_958_SRC_SHIFT,
//                             aiu_spdif_encode_sel_texts);

static aiu_spdif_encode_mux: snd_kcontrol_new = snd_kcontrol_new {
    name: b"SPDIF Buffer Src\0".as_ptr() as *const c_char,
    private_value: unsafe { &aiu_spdif_encode_sel_enum as *const soc_enum as usize },
};

static mut aiu_cpu_dapm_widgets: [snd_soc_dapm_widget; 2] = [
    snd_soc_dapm_widget {
        id: 0,
        name: b"SPDIF SRC SEL\0".as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol_news: &aiu_spdif_encode_mux,
        num_kcontrols: 1,
        event: None,
        event_flags: 0,
    },
    snd_soc_dapm_widget {
        id: 0,
        name: b"I2S Formatter\0".as_ptr() as *const c_char,
        reg: SND_SOC_NOPM,
        shift: 0,
        invert: 0,
        kcontrol_news: ptr::null(),
        num_kcontrols: 0,
        event: Some(gx_formatter_event),
        event_flags: SND_SOC_DAPM_PRE_PMU | SND_SOC_DAPM_PRE_PMD,
    },
];

static aiu_cpu_dapm_routes: [snd_soc_dapm_route; 5] = [
    snd_soc_dapm_route {
        sink: b"I2S Formatter\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"I2S FIFO Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"I2S Encoder Playback\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"I2S Formatter\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"SPDIF SRC SEL\0".as_ptr() as *const c_char,
        control: b"SPDIF\0".as_ptr() as *const c_char,
        source: b"SPDIF FIFO Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"SPDIF SRC SEL\0".as_ptr() as *const c_char,
        control: b"I2S\0".as_ptr() as *const c_char,
        source: b"I2S FIFO Playback\0".as_ptr() as *const c_char,
    },
    snd_soc_dapm_route {
        sink: b"SPDIF Encoder Playback\0".as_ptr() as *const c_char,
        control: ptr::null(),
        source: b"SPDIF SRC SEL\0".as_ptr() as *const c_char,
    },
];

pub unsafe extern "C" fn aiu_of_xlate_dai_name(
    component: *mut snd_soc_component,
    args: *const of_phandle_args,
    dai_name: *mut *const c_char,
    component_id: c_uint,
) -> c_int {
    let mut dai: *mut snd_soc_dai = ptr::null_mut();
    let mut id: c_int;

    if (*args).args_count != 2 {
        return -EINVAL;
    }

    if (*args).args[0] != component_id {
        return -EINVAL;
    }

    id = (*args).args[1] as c_int;

    if id < 0 || id >= (*component).num_dai {
        return -EINVAL;
    }

    // for_each_component_dais(component, dai)
    while !dai.is_null() {
        if id == 0 {
            break;
        }
        id -= 1;
    }

    *dai_name = (*(*dai).driver).name;

    0
}

unsafe extern "C" fn aiu_cpu_of_xlate_dai_name(
    component: *mut snd_soc_component,
    args: *const of_phandle_args,
    dai_name: *mut *const c_char,
) -> c_int {
    aiu_of_xlate_dai_name(component, args, dai_name, AIU_CPU)
}

unsafe extern "C" fn aiu_cpu_component_probe(component: *mut snd_soc_component) -> c_int {
    let aiu = snd_soc_component_get_drvdata(component) as *mut aiu;

    /* Required for the SPDIF Source control operation */
    clk_prepare_enable((*(*aiu).i2s.clks.add(PCLK)).clk)
}

unsafe extern "C" fn aiu_cpu_component_remove(component: *mut snd_soc_component) {
    let aiu = snd_soc_component_get_drvdata(component) as *mut aiu;

    clk_disable_unprepare((*(*aiu).i2s.clks.add(PCLK)).clk);
}

static aiu_cpu_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"AIU CPU\0".as_ptr() as *const c_char,
    dapm_widgets: unsafe { aiu_cpu_dapm_widgets.as_ptr() as *mut snd_soc_dapm_widget },
    num_dapm_widgets: 2,
    dapm_routes: aiu_cpu_dapm_routes.as_ptr(),
    num_dapm_routes: 5,
    of_xlate_dai_name: Some(aiu_cpu_of_xlate_dai_name),
    pointer: Some(aiu_fifo_pointer),
    probe: Some(aiu_cpu_component_probe),
    remove: Some(aiu_cpu_component_remove),
    // CONFIG_DEBUG_FS: debugfs_prefix = "cpu"
};

static mut aiu_cpu_dai_drv: [snd_soc_dai_driver; 4] = [
    snd_soc_dai_driver {
        name: b"I2S FIFO\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"I2S FIFO Playback\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_CONTINUOUS,
            rate_min: 5512,
            rate_max: 192000,
            formats: AIU_FORMATS,
        },
        ops: unsafe { &aiu_fifo_i2s_dai_ops },
        symmetric_rate: 0,
    },
    snd_soc_dai_driver {
        name: b"SPDIF FIFO\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"SPDIF FIFO Playback\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_CONTINUOUS,
            rate_min: 5512,
            rate_max: 192000,
            formats: AIU_FORMATS,
        },
        ops: unsafe { &aiu_fifo_spdif_dai_ops },
        symmetric_rate: 0,
    },
    snd_soc_dai_driver {
        name: b"I2S Encoder\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"I2S Encoder Playback\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 8,
            rates: SNDRV_PCM_RATE_8000_192000,
            rate_min: 0,
            rate_max: 0,
            formats: AIU_FORMATS,
        },
        ops: unsafe { &aiu_encoder_i2s_dai_ops },
        symmetric_rate: 1,
    },
    snd_soc_dai_driver {
        name: b"SPDIF Encoder\0".as_ptr() as *const c_char,
        playback: snd_soc_pcm_stream {
            stream_name: b"SPDIF Encoder Playback\0".as_ptr() as *const c_char,
            channels_min: 2,
            channels_max: 2,
            rates: SNDRV_PCM_RATE_32000
                | SNDRV_PCM_RATE_44100
                | SNDRV_PCM_RATE_48000
                | SNDRV_PCM_RATE_88200
                | SNDRV_PCM_RATE_96000
                | SNDRV_PCM_RATE_176400
                | SNDRV_PCM_RATE_192000,
            rate_min: 0,
            rate_max: 0,
            formats: AIU_FORMATS,
        },
        ops: unsafe { &aiu_encoder_spdif_dai_ops },
        symmetric_rate: 0,
    },
];

static aiu_regmap_cfg: regmap_config = regmap_config {
    reg_bits: 32,
    val_bits: 32,
    reg_stride: 4,
    max_register: 0x2ac,
};

static aiu_formatter_i2s_drv: gx_formatter_driver = gx_formatter_driver {
    regmap_cfg: &aiu_regmap_cfg,
    ops: unsafe { &aiu_formatter_i2s_ops },
};

unsafe extern "C" fn aiu_clk_bulk_get(
    dev: *mut device,
    ids: *const *const c_char,
    num: c_uint,
    interface: *mut aiu_interface,
) -> c_int {
    let clks: *mut clk_bulk_data;
    let mut i: c_uint;
    let mut ret: c_int;

    clks = devm_kcalloc(dev, num as usize, size_of::<clk_bulk_data>(), GFP_KERNEL)
        as *mut clk_bulk_data;
    if clks.is_null() {
        return -ENOMEM;
    }

    i = 0;
    while i < num {
        (*clks.add(i as usize)).id = *ids.add(i as usize);
        i += 1;
    }

    ret = devm_clk_bulk_get(dev, num as c_int, clks);
    if ret < 0 {
        return ret;
    }

    (*interface).clks = clks;
    (*interface).clk_num = num;
    0
}

static aiu_i2s_ids: [*const c_char; 4] = [
    b"i2s_pclk\0".as_ptr() as *const c_char,
    b"i2s_aoclk\0".as_ptr() as *const c_char,
    b"i2s_mclk\0".as_ptr() as *const c_char,
    b"i2s_mixer\0".as_ptr() as *const c_char,
];

static aiu_spdif_ids: [*const c_char; 3] = [
    b"spdif_pclk\0".as_ptr() as *const c_char,
    b"spdif_aoclk\0".as_ptr() as *const c_char,
    b"spdif_mclk_sel\0".as_ptr() as *const c_char,
];

unsafe extern "C" fn aiu_clk_get(dev: *mut device) -> c_int {
    let aiu = dev_get_drvdata(dev) as *mut aiu;
    let pclk: *mut clk;
    let mut ret: c_int;

    pclk = devm_clk_get_enabled(dev, b"pclk\0".as_ptr() as *const c_char);
    if IS_ERR(pclk) {
        return dev_err_probe(
            dev,
            PTR_ERR(pclk),
            b"Can't get the aiu pclk\n\0".as_ptr() as *const c_char,
        );
    }

    (*aiu).spdif_mclk = devm_clk_get(dev, b"spdif_mclk\0".as_ptr() as *const c_char);
    if IS_ERR((*aiu).spdif_mclk) {
        return dev_err_probe(
            dev,
            PTR_ERR((*aiu).spdif_mclk),
            b"Can't get the aiu spdif master clock\n\0".as_ptr() as *const c_char,
        );
    }

    ret = aiu_clk_bulk_get(
        dev,
        aiu_i2s_ids.as_ptr(),
        aiu_i2s_ids.len() as c_uint,
        &mut (*aiu).i2s,
    );
    if ret != 0 {
        return dev_err_probe(
            dev,
            ret as c_long,
            b"Can't get the i2s clocks\n\0".as_ptr() as *const c_char,
        );
    }

    ret = aiu_clk_bulk_get(
        dev,
        aiu_spdif_ids.as_ptr(),
        aiu_spdif_ids.len() as c_uint,
        &mut (*aiu).spdif,
    );
    if ret != 0 {
        return dev_err_probe(
            dev,
            ret as c_long,
            b"Can't get the spdif clocks\n\0".as_ptr() as *const c_char,
        );
    }

    ret
}

unsafe extern "C" fn aiu_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let regs: *mut c_void;
    let map: *mut regmap;
    let aiu: *mut aiu;
    let mut ret: c_int;

    aiu = devm_kzalloc(dev, size_of::<aiu>(), GFP_KERNEL) as *mut aiu;
    if aiu.is_null() {
        return -ENOMEM;
    }

    (*aiu).platform = device_get_match_data(dev) as *const aiu_platform_data;
    if (*aiu).platform.is_null() {
        return -ENODEV;
    }

    platform_set_drvdata(pdev, aiu as *mut c_void);

    ret = device_reset(dev);
    if ret != 0 {
        return dev_err_probe(
            dev,
            ret as c_long,
            b"Failed to reset device\n\0".as_ptr() as *const c_char,
        );
    }

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs) {
        return PTR_ERR(regs) as c_int;
    }

    map = devm_regmap_init_mmio(dev, regs, &aiu_regmap_cfg);
    if IS_ERR(map) {
        dev_err(
            dev,
            b"failed to init regmap: %ld\n\0".as_ptr() as *const c_char,
            PTR_ERR(map),
        );
        return PTR_ERR(map) as c_int;
    }

    (*aiu).i2s.irq = platform_get_irq_byname(pdev, b"i2s\0".as_ptr() as *const c_char);
    if (*aiu).i2s.irq < 0 {
        return (*aiu).i2s.irq;
    }

    (*aiu).spdif.irq = platform_get_irq_byname(pdev, b"spdif\0".as_ptr() as *const c_char);
    if (*aiu).spdif.irq < 0 {
        return (*aiu).spdif.irq;
    }

    ret = aiu_clk_get(dev);
    if ret != 0 {
        return ret;
    }

    /* Allocate the aiu-formatter into its widget */
    ret = gx_formatter_create(
        dev,
        &mut aiu_cpu_dapm_widgets[AIU_WIDGET_I2S_FORMATTER],
        &aiu_formatter_i2s_drv,
        map,
    );
    if ret != 0 {
        dev_err(
            dev,
            b"Failed to allocate aiu formatter\n\0".as_ptr() as *const c_char,
        );
        goto_err(ret, dev);
        return ret;
    }

    /* Register the cpu component of the aiu */
    ret = snd_soc_register_component(
        dev,
        &aiu_cpu_component,
        aiu_cpu_dai_drv.as_mut_ptr(),
        aiu_cpu_dai_drv.len() as c_int,
    );
    if ret != 0 {
        dev_err(
            dev,
            b"Failed to register cpu component\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    /* Register the hdmi codec control component */
    ret = aiu_hdmi_ctrl_register_component(dev);
    if ret != 0 {
        dev_err(
            dev,
            b"Failed to register hdmi control component\n\0".as_ptr() as *const c_char,
        );
        goto_err(ret, dev);
        return ret;
    }

    /* Register the internal dac control component on gxl */
    if (*(*aiu).platform).has_acodec {
        ret = aiu_acodec_ctrl_register_component(dev);
        if ret != 0 {
            dev_err(
                dev,
                b"Failed to register acodec control component\n\0".as_ptr() as *const c_char,
            );
            goto_err(ret, dev);
            return ret;
        }
    }

    return 0;

    unsafe fn goto_err(ret: c_int, dev: *mut device) {
        let _ = ret;
        gx_formatter_free(&mut aiu_cpu_dapm_widgets[AIU_WIDGET_I2S_FORMATTER]);
        snd_soc_unregister_component(dev);
    }
}

unsafe extern "C" fn aiu_remove(pdev: *mut platform_device) {
    gx_formatter_free(&mut aiu_cpu_dapm_widgets[AIU_WIDGET_I2S_FORMATTER]);
    snd_soc_unregister_component(&mut (*pdev).dev);
}

static aiu_gxbb_pdata: aiu_platform_data = aiu_platform_data {
    has_acodec: false,
    has_clk_ctrl_more_i2s_div: true,
};

static aiu_gxl_pdata: aiu_platform_data = aiu_platform_data {
    has_acodec: true,
    has_clk_ctrl_more_i2s_div: true,
};

static aiu_meson8_pdata: aiu_platform_data = aiu_platform_data {
    has_acodec: false,
    has_clk_ctrl_more_i2s_div: false,
};

static aiu_of_match: [of_device_id; 5] = [
    of_device_id {
        compatible: b"amlogic,aiu-gxbb\0".as_ptr() as *const c_char,
        data: &aiu_gxbb_pdata as *const aiu_platform_data as *const c_void,
    },
    of_device_id {
        compatible: b"amlogic,aiu-gxl\0".as_ptr() as *const c_char,
        data: &aiu_gxl_pdata as *const aiu_platform_data as *const c_void,
    },
    of_device_id {
        compatible: b"amlogic,aiu-meson8\0".as_ptr() as *const c_char,
        data: &aiu_meson8_pdata as *const aiu_platform_data as *const c_void,
    },
    of_device_id {
        compatible: b"amlogic,aiu-meson8b\0".as_ptr() as *const c_char,
        data: &aiu_meson8_pdata as *const aiu_platform_data as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, aiu_of_match);

static mut aiu_pdrv: platform_driver = platform_driver {
    probe: Some(aiu_probe),
    remove: Some(aiu_remove),
    driver: platform_driver_inner {
        name: b"meson-aiu\0".as_ptr() as *const c_char,
        of_match_table: aiu_of_match.as_ptr(),
    },
};
// module_platform_driver(aiu_pdrv);

// MODULE_DESCRIPTION("Meson AIU Driver");
// MODULE_AUTHOR("Jerome Brunet <jbrunet@baylibre.com>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
