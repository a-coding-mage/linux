// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2012-2013, Analog Devices Inc.
 * Author: Lars-Peter Clausen <lars@metafoo.de>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type resource_size_t = c_ulong;

const GFP_KERNEL: c_uint = 0;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;

const SNDRV_PCM_HW_PARAM_RATE: c_int = 0;
const SNDRV_PCM_RATE_KNOT: c_uint = 0;
const SNDRV_PCM_FMTBIT_S16_LE: u64 = 0;

const AXI_SPDIF_REG_CTRL: c_uint = 0x0;
const AXI_SPDIF_REG_STAT: c_uint = 0x4;
const AXI_SPDIF_REG_TX_FIFO: resource_size_t = 0xc;

const AXI_SPDIF_CTRL_TXDATA: c_uint = 1 << 1;
const AXI_SPDIF_CTRL_TXEN: c_uint = 1 << 0;
const AXI_SPDIF_CTRL_CLKDIV_OFFSET: c_uint = 8;
const AXI_SPDIF_CTRL_CLKDIV_MASK: c_uint = 0xff << 8;

const AXI_SPDIF_FREQ_44100: c_uint = 0x0 << 6;
const AXI_SPDIF_FREQ_48000: c_uint = 0x1 << 6;
const AXI_SPDIF_FREQ_32000: c_uint = 0x2 << 6;
const AXI_SPDIF_FREQ_NA: c_uint = 0x3 << 6;

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
pub struct snd_pcm_substream {
    runtime: *mut snd_pcm_runtime,
}

#[repr(C)]
pub struct snd_pcm_runtime {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_pcm_hw_params {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    addr: resource_size_t,
    addr_width: c_uint,
    maxburst: c_uint,
}

#[repr(C)]
pub struct snd_ratnum {
    num: c_uint,
    den_min: c_uint,
    den_max: c_uint,
    den_step: c_uint,
}

#[repr(C)]
pub struct snd_pcm_hw_constraint_ratnums {
    nrats: c_uint,
    rats: *mut snd_ratnum,
}

#[repr(C)]
pub struct axi_spdif {
    regmap: *mut regmap,
    clk: *mut clk,
    clk_ref: *mut clk,

    dma_data: snd_dmaengine_dai_dma_data,

    ratnum: snd_ratnum,
    rate_constraints: snd_pcm_hw_constraint_ratnums,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    trigger:
        Option<unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int>,
    hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    name: *const c_char,
    legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    reg_bits: c_uint,
    reg_stride: c_uint,
    val_bits: c_uint,
    max_register: c_uint,
}

#[repr(C)]
pub struct resource {
    start: resource_size_t,
}

#[repr(C)]
pub struct platform_device {
    dev: device,
}

#[repr(C)]
pub struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

unsafe extern "C" {
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn snd_soc_dai_init_dma_data(
        dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn snd_pcm_hw_constraint_ratnums(
        runtime: *mut snd_pcm_runtime,
        cond: c_uint,
        var: c_int,
        r: *mut snd_pcm_hw_constraint_ratnums,
    ) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const c_void,
        flags: c_uint,
    ) -> c_int;
}

fn div_round_closest(x: c_ulong, divisor: c_ulong) -> c_ulong {
    (x.wrapping_add(divisor / 2)) / divisor
}

unsafe extern "C" fn axi_spdif_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let spdif = snd_soc_dai_get_drvdata(dai) as *mut axi_spdif;
    let val: c_uint;

    match cmd {
        SNDRV_PCM_TRIGGER_START
        | SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            val = AXI_SPDIF_CTRL_TXDATA;
        }
        SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            val = 0;
        }
        _ => {
            return -EINVAL;
        }
    }

    regmap_update_bits(
        (*spdif).regmap,
        AXI_SPDIF_REG_CTRL,
        AXI_SPDIF_CTRL_TXDATA,
        val,
    );

    0
}

unsafe extern "C" fn axi_spdif_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let spdif = snd_soc_dai_get_drvdata(dai) as *mut axi_spdif;
    let rate: c_uint = params_rate(params);
    let mut clkdiv: c_uint;
    let stat: c_uint;

    match params_rate(params) {
        32000 => {
            stat = AXI_SPDIF_FREQ_32000;
        }
        44100 => {
            stat = AXI_SPDIF_FREQ_44100;
        }
        48000 => {
            stat = AXI_SPDIF_FREQ_48000;
        }
        _ => {
            stat = AXI_SPDIF_FREQ_NA;
        }
    }

    clkdiv = div_round_closest(
        clk_get_rate((*spdif).clk_ref),
        (rate as c_ulong).wrapping_mul(64).wrapping_mul(2),
    )
    .wrapping_sub(1) as c_uint;
    clkdiv <<= AXI_SPDIF_CTRL_CLKDIV_OFFSET;

    regmap_write((*spdif).regmap, AXI_SPDIF_REG_STAT, stat);
    regmap_update_bits(
        (*spdif).regmap,
        AXI_SPDIF_REG_CTRL,
        AXI_SPDIF_CTRL_CLKDIV_MASK,
        clkdiv,
    );

    0
}

unsafe extern "C" fn axi_spdif_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let spdif = snd_soc_dai_get_drvdata(dai) as *mut axi_spdif;

    snd_soc_dai_init_dma_data(dai, &mut (*spdif).dma_data, ptr::null_mut());

    0
}

unsafe extern "C" fn axi_spdif_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let spdif = snd_soc_dai_get_drvdata(dai) as *mut axi_spdif;
    let mut ret: c_int;

    ret = snd_pcm_hw_constraint_ratnums(
        (*substream).runtime,
        0,
        SNDRV_PCM_HW_PARAM_RATE,
        &mut (*spdif).rate_constraints,
    );
    if ret != 0 {
        return ret;
    }

    ret = clk_prepare_enable((*spdif).clk_ref);
    if ret != 0 {
        return ret;
    }

    regmap_update_bits(
        (*spdif).regmap,
        AXI_SPDIF_REG_CTRL,
        AXI_SPDIF_CTRL_TXEN,
        AXI_SPDIF_CTRL_TXEN,
    );

    0
}

unsafe extern "C" fn axi_spdif_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let spdif = snd_soc_dai_get_drvdata(dai) as *mut axi_spdif;

    regmap_update_bits(
        (*spdif).regmap,
        AXI_SPDIF_REG_CTRL,
        AXI_SPDIF_CTRL_TXEN,
        0,
    );

    clk_disable_unprepare((*spdif).clk_ref);
}

static axi_spdif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(axi_spdif_dai_probe),
    startup: Some(axi_spdif_startup),
    shutdown: Some(axi_spdif_shutdown),
    trigger: Some(axi_spdif_trigger),
    hw_params: Some(axi_spdif_hw_params),
};

static mut axi_spdif_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_KNOT,
        formats: SNDRV_PCM_FMTBIT_S16_LE,
    },
    ops: &axi_spdif_dai_ops,
};

static axi_spdif_component: snd_soc_component_driver = snd_soc_component_driver {
    name: c"axi-spdif".as_ptr(),
    legacy_dai_naming: 1,
};

static axi_spdif_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: AXI_SPDIF_REG_STAT,
};

unsafe extern "C" fn axi_spdif_probe(pdev: *mut platform_device) -> c_int {
    let spdif: *mut axi_spdif;
    let mut res: *mut resource = ptr::null_mut();
    let base: *mut c_void;
    let mut ret: c_int;

    spdif = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<axi_spdif>(),
        GFP_KERNEL,
    ) as *mut axi_spdif;
    if spdif.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, spdif as *mut c_void);

    base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(base) {
        return PTR_ERR(base);
    }

    (*spdif).regmap = devm_regmap_init_mmio(&mut (*pdev).dev, base, &axi_spdif_regmap_config);
    if IS_ERR((*spdif).regmap as *const c_void) {
        return PTR_ERR((*spdif).regmap as *const c_void);
    }

    (*spdif).clk = devm_clk_get(&mut (*pdev).dev, c"axi".as_ptr());
    if IS_ERR((*spdif).clk as *const c_void) {
        return PTR_ERR((*spdif).clk as *const c_void);
    }

    (*spdif).clk_ref = devm_clk_get(&mut (*pdev).dev, c"ref".as_ptr());
    if IS_ERR((*spdif).clk_ref as *const c_void) {
        return PTR_ERR((*spdif).clk_ref as *const c_void);
    }

    ret = clk_prepare_enable((*spdif).clk);
    if ret != 0 {
        return ret;
    }

    (*spdif).dma_data.addr = (*res).start.wrapping_add(AXI_SPDIF_REG_TX_FIFO);
    (*spdif).dma_data.addr_width = 4;
    (*spdif).dma_data.maxburst = 1;

    (*spdif).ratnum.num = (clk_get_rate((*spdif).clk_ref) / 128) as c_uint;
    (*spdif).ratnum.den_step = 1;
    (*spdif).ratnum.den_min = 1;
    (*spdif).ratnum.den_max = 64;

    (*spdif).rate_constraints.rats = &mut (*spdif).ratnum;
    (*spdif).rate_constraints.nrats = 1;

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &axi_spdif_component,
        &raw mut axi_spdif_dai,
        1,
    );
    if ret != 0 {
        goto_err_clk_disable(spdif);
        return ret;
    }

    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, ptr::null(), 0);
    if ret != 0 {
        goto_err_clk_disable(spdif);
        return ret;
    }

    0
}

unsafe fn goto_err_clk_disable(spdif: *mut axi_spdif) {
    clk_disable_unprepare((*spdif).clk);
}

unsafe extern "C" fn axi_spdif_dev_remove(pdev: *mut platform_device) {
    let spdif = platform_get_drvdata(pdev) as *mut axi_spdif;

    clk_disable_unprepare((*spdif).clk);
}

static axi_spdif_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"adi,axi-spdif-tx-1.00.a".as_ptr(),
    },
    of_device_id {
        compatible: ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, axi_spdif_of_match);

static mut axi_spdif_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: c"axi-spdif".as_ptr(),
        of_match_table: axi_spdif_of_match.as_ptr(),
    },
    probe: Some(axi_spdif_probe),
    remove: Some(axi_spdif_dev_remove),
};

// module_platform_driver(axi_spdif_driver);
// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_DESCRIPTION("AXI SPDIF driver");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
