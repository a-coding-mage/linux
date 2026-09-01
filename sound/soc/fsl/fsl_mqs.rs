// SPDX-License-Identifier: GPL-2.0
//
// ALSA SoC IMX MQS driver
//
// Copyright (C) 2014-2015 Freescale Semiconductor, Inc.
// Copyright 2019 NXP

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::ptr;

const REG_MQS_CTRL: c_int = 0x00;

const MQS_EN_MASK: c_int = 0x1 << 28;
const MQS_EN_SHIFT: c_int = 28;
const MQS_SW_RST_MASK: c_int = 0x1 << 24;
const MQS_SW_RST_SHIFT: c_int = 24;
const MQS_OVERSAMPLE_MASK: c_int = 0x1 << 20;
const MQS_OVERSAMPLE_SHIFT: c_int = 20;
const MQS_CLK_DIV_MASK: c_int = 0xFF << 0;
const MQS_CLK_DIV_SHIFT: c_int = 0;

const FSL_MQS_RATES: c_uint = SNDRV_PCM_RATE_44100 | SNDRV_PCM_RATE_48000;
const FSL_MQS_FORMATS: c_uint = SNDRV_PCM_FMTBIT_S16_LE;

const fn BIT(nr: c_uint) -> c_int {
    (1u32 << nr) as c_int
}

const fn GENMASK(h: c_uint, l: c_uint) -> c_int {
    let all = !0u32;
    ((all << l) & (all >> (31 - h))) as c_int
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum reg_type {
    TYPE_REG_OWN, /* module own register space */
    TYPE_REG_GPR, /* register in GPR space */
    TYPE_REG_SM,  /* System Manager controls the register */
}

/**
 * struct fsl_mqs_soc_data - soc specific data
 *
 * @type: control register space type
 * @sm_index: index from definition in system manager
 * @ctrl_off: control register offset
 * @en_mask: enable bit mask
 * @en_shift: enable bit shift
 * @rst_mask: reset bit mask
 * @rst_shift: reset bit shift
 * @osr_mask: oversample bit mask
 * @osr_shift: oversample bit shift
 * @div_mask: clock divider mask
 * @div_shift: clock divider bit shift
 */
#[repr(C)]
struct fsl_mqs_soc_data {
    type_: reg_type,
    sm_index: c_int,
    ctrl_off: c_int,
    en_mask: c_int,
    en_shift: c_int,
    rst_mask: c_int,
    rst_shift: c_int,
    osr_mask: c_int,
    osr_shift: c_int,
    div_mask: c_int,
    div_shift: c_int,
}

/* codec private data */
#[repr(C)]
struct fsl_mqs {
    regmap: *mut regmap,
    mclk: *mut clk,
    ipg: *mut clk,
    soc: *const fsl_mqs_soc_data,

    reg_mqs_ctrl: c_uint,
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
struct clk {
    _private: [u8; 0],
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
struct snd_soc_dai {
    component: *mut snd_soc_component,
}
#[repr(C)]
struct snd_soc_component {
    dev: *mut device,
}
#[repr(C)]
struct device {
    of_node: *mut device_node,
}
#[repr(C)]
struct platform_device {
    dev: device,
}
#[repr(C)]
struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
struct regmap_bus {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_component_driver {
    idle_bias_on: c_uint,
}

#[repr(C)]
struct snd_soc_dai_ops {
    startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    hw_params: Option<
        unsafe extern "C" fn(
            *mut snd_pcm_substream,
            *mut snd_pcm_hw_params,
            *mut snd_soc_dai,
        ) -> c_int,
    >,
    set_fmt: Option<unsafe extern "C" fn(*mut snd_soc_dai, c_uint) -> c_int>,
}

#[repr(C)]
struct snd_soc_pcm_stream {
    stream_name: *const c_char,
    channels_min: c_uint,
    channels_max: c_uint,
    rates: c_uint,
    formats: c_uint,
}

#[repr(C)]
struct snd_soc_dai_driver {
    name: *const c_char,
    playback: snd_soc_pcm_stream,
    ops: *const snd_soc_dai_ops,
}

#[repr(C)]
struct regmap_config {
    reg_bits: c_int,
    reg_stride: c_int,
    val_bits: c_int,
    max_register: c_int,
    cache_type: c_int,
    reg_read: Option<unsafe extern "C" fn(*mut c_void, c_uint, *mut c_uint) -> c_int>,
    reg_write: Option<unsafe extern "C" fn(*mut c_void, c_uint, c_uint) -> c_int>,
}

#[repr(C)]
struct dev_pm_ops {
    runtime_suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    runtime_idle: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    suspend: Option<unsafe extern "C" fn(*mut device) -> c_int>,
    resume: Option<unsafe extern "C" fn(*mut device) -> c_int>,
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
    pm: *const dev_pm_ops,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    driver: device_driver,
}

extern "C" {
    static IOMUXC_GPR2: c_int;
    static IMX6SX_GPR2_MQS_EN_MASK: c_int;
    static IMX6SX_GPR2_MQS_EN_SHIFT: c_int;
    static IMX6SX_GPR2_MQS_SW_RST_MASK: c_int;
    static IMX6SX_GPR2_MQS_SW_RST_SHIFT: c_int;
    static IMX6SX_GPR2_MQS_OVERSAMPLE_MASK: c_int;
    static IMX6SX_GPR2_MQS_OVERSAMPLE_SHIFT: c_int;
    static IMX6SX_GPR2_MQS_CLK_DIV_MASK: c_int;
    static IMX6SX_GPR2_MQS_CLK_DIV_SHIFT: c_int;
    static SCMI_IMX95_CTRL_MQS1_SETTINGS: c_int;
    static SCMI_IMX94_CTRL_MQS1_SETTINGS: c_int;
    static SCMI_IMX94_CTRL_MQS2_SETTINGS: c_int;

    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_FMTBIT_S16_LE: c_uint;
    static SND_SOC_DAIFMT_FORMAT_MASK: c_uint;
    static SND_SOC_DAIFMT_LEFT_J: c_uint;
    static SND_SOC_DAIFMT_INV_MASK: c_uint;
    static SND_SOC_DAIFMT_NB_NF: c_uint;
    static SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK: c_uint;
    static SND_SOC_DAIFMT_CBC_CFC: c_uint;
    static CONFIG_IMX_SCMI_MISC_DRV: bool;
    static REGCACHE_NONE: c_int;
    static GFP_KERNEL: c_uint;

    fn scmi_imx_misc_ctrl_get(index: c_int, num: *mut c_int, val: *mut c_uint) -> c_int;
    fn scmi_imx_misc_ctrl_set(index: c_int, val: c_uint) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_int;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_int,
        mask: c_int,
        val: c_int,
    ) -> c_int;
    fn snd_soc_component_get_drvdata(component: *mut snd_soc_component) -> *mut c_void;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn of_device_get_match_data(dev: *mut device) -> *const c_void;
    fn of_parse_phandle(np: *mut device_node, phandle_name: *const c_char, index: c_int)
        -> *mut device_node;
    fn syscon_node_to_regmap(np: *mut device_node) -> *mut regmap;
    fn of_node_put(node: *mut device_node);
    fn devm_regmap_init(
        dev: *mut device,
        bus: *const regmap_bus,
        bus_context: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio_clk(
        dev: *mut device,
        clk_id: *const c_char,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn regmap_write(map: *mut regmap, reg: c_int, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_int, val: *mut c_uint) -> c_int;
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

unsafe fn IS_ENABLED(config: bool) -> bool {
    config
}

unsafe fn IS_ERR<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0 && (ptr as isize) >= -4095
}

unsafe fn PTR_ERR<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

unsafe extern "C" fn fsl_mqs_sm_read(
    context: *mut c_void,
    reg: c_uint,
    val: *mut c_uint,
) -> c_int {
    let mqs_priv = context as *mut fsl_mqs;
    let mut num: c_int = 1;

    if IS_ENABLED(CONFIG_IMX_SCMI_MISC_DRV) && (*(*mqs_priv).soc).ctrl_off as c_uint == reg {
        return scmi_imx_misc_ctrl_get((*(*mqs_priv).soc).sm_index, &mut num, val);
    }

    -EINVAL
}

unsafe extern "C" fn fsl_mqs_sm_write(
    context: *mut c_void,
    reg: c_uint,
    val: c_uint,
) -> c_int {
    let mqs_priv = context as *mut fsl_mqs;

    if IS_ENABLED(CONFIG_IMX_SCMI_MISC_DRV) && (*(*mqs_priv).soc).ctrl_off as c_uint == reg {
        return scmi_imx_misc_ctrl_set((*(*mqs_priv).soc).sm_index, val);
    }

    -EINVAL
}

unsafe extern "C" fn fsl_mqs_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let mqs_priv = snd_soc_component_get_drvdata(component) as *mut fsl_mqs;
    let mclk_rate: c_ulong;
    let div: c_int;
    let res: c_int;
    let lrclk: c_int;

    mclk_rate = clk_get_rate((*mqs_priv).mclk);
    lrclk = params_rate(params);

    /*
     * mclk_rate / (oversample(32,64) * FS * 2 * divider ) = repeat_rate;
     * if repeat_rate is 8, mqs can achieve better quality.
     * oversample rate is fix to 32 currently.
     */
    div = (mclk_rate / (32 * lrclk * 2 * 8) as c_ulong) as c_int;
    res = (mclk_rate % (32 * lrclk * 2 * 8) as c_ulong) as c_int;

    if res == 0 && div > 0 && div <= 256 {
        regmap_update_bits(
            (*mqs_priv).regmap,
            (*(*mqs_priv).soc).ctrl_off,
            (*(*mqs_priv).soc).div_mask,
            (div - 1) << (*(*mqs_priv).soc).div_shift,
        );
        regmap_update_bits(
            (*mqs_priv).regmap,
            (*(*mqs_priv).soc).ctrl_off,
            (*(*mqs_priv).soc).osr_mask,
            0,
        );
    } else {
        dev_err((*component).dev, c"can't get proper divider\n".as_ptr());
    }

    0
}

unsafe extern "C" fn fsl_mqs_set_dai_fmt(_dai: *mut snd_soc_dai, fmt: c_uint) -> c_int {
    /* Only LEFT_J & SLAVE mode is supported. */
    match fmt & SND_SOC_DAIFMT_FORMAT_MASK {
        x if x == SND_SOC_DAIFMT_LEFT_J => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_INV_MASK {
        x if x == SND_SOC_DAIFMT_NB_NF => {}
        _ => return -EINVAL,
    }

    match fmt & SND_SOC_DAIFMT_CLOCK_PROVIDER_MASK {
        x if x == SND_SOC_DAIFMT_CBC_CFC => {}
        _ => return -EINVAL,
    }

    0
}

unsafe extern "C" fn fsl_mqs_startup(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let component = (*dai).component;
    let mqs_priv = snd_soc_component_get_drvdata(component) as *mut fsl_mqs;

    regmap_update_bits(
        (*mqs_priv).regmap,
        (*(*mqs_priv).soc).ctrl_off,
        (*(*mqs_priv).soc).en_mask,
        1 << (*(*mqs_priv).soc).en_shift,
    );
    0
}

unsafe extern "C" fn fsl_mqs_shutdown(
    _substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let component = (*dai).component;
    let mqs_priv = snd_soc_component_get_drvdata(component) as *mut fsl_mqs;

    regmap_update_bits(
        (*mqs_priv).regmap,
        (*(*mqs_priv).soc).ctrl_off,
        (*(*mqs_priv).soc).en_mask,
        0,
    );
}

static soc_codec_fsl_mqs: snd_soc_component_driver = snd_soc_component_driver { idle_bias_on: 1 };

static fsl_mqs_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(fsl_mqs_startup),
    shutdown: Some(fsl_mqs_shutdown),
    hw_params: Some(fsl_mqs_hw_params),
    set_fmt: Some(fsl_mqs_set_dai_fmt),
};

static mut fsl_mqs_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: c"fsl-mqs-dai".as_ptr(),
    playback: snd_soc_pcm_stream {
        stream_name: c"Playback".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: FSL_MQS_RATES,
        formats: FSL_MQS_FORMATS,
    },
    ops: &fsl_mqs_dai_ops,
};

static fsl_mqs_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: REG_MQS_CTRL,
    cache_type: unsafe { REGCACHE_NONE },
    reg_read: None,
    reg_write: None,
};

static fsl_mqs_sm_regmap: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 0,
    val_bits: 32,
    max_register: 0,
    cache_type: 0,
    reg_read: Some(fsl_mqs_sm_read),
    reg_write: Some(fsl_mqs_sm_write),
};

unsafe extern "C" fn fsl_mqs_probe(pdev: *mut platform_device) -> c_int {
    let np = (*pdev).dev.of_node;
    let mut gpr_np: *mut device_node = ptr::null_mut();
    let mqs_priv: *mut fsl_mqs;
    let regs: *mut c_void;
    let ret: c_int;

    mqs_priv = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<fsl_mqs>(),
        GFP_KERNEL,
    ) as *mut fsl_mqs;
    if mqs_priv.is_null() {
        return -ENOMEM;
    }

    /* On i.MX6sx the MQS control register is in GPR domain
     * But in i.MX8QM/i.MX8QXP the control register is moved
     * to its own domain.
     */
    (*mqs_priv).soc = of_device_get_match_data(&mut (*pdev).dev) as *const fsl_mqs_soc_data;

    if (*(*mqs_priv).soc).type_ == reg_type::TYPE_REG_GPR {
        gpr_np = of_parse_phandle(np, c"gpr".as_ptr(), 0);
        if gpr_np.is_null() {
            dev_err(
                &mut (*pdev).dev,
                c"failed to get gpr node by phandle\n".as_ptr(),
            );
            return -EINVAL;
        }

        (*mqs_priv).regmap = syscon_node_to_regmap(gpr_np);
        of_node_put(gpr_np);
        if IS_ERR((*mqs_priv).regmap) {
            dev_err(&mut (*pdev).dev, c"failed to get gpr regmap\n".as_ptr());
            return PTR_ERR((*mqs_priv).regmap);
        }
    } else if (*(*mqs_priv).soc).type_ == reg_type::TYPE_REG_SM {
        (*mqs_priv).regmap = devm_regmap_init(
            &mut (*pdev).dev,
            ptr::null(),
            mqs_priv as *mut c_void,
            &fsl_mqs_sm_regmap,
        );
        if IS_ERR((*mqs_priv).regmap) {
            dev_err(
                &mut (*pdev).dev,
                c"failed to init regmap: %ld\n".as_ptr(),
                PTR_ERR((*mqs_priv).regmap),
            );
            return PTR_ERR((*mqs_priv).regmap);
        }
    } else {
        regs = devm_platform_ioremap_resource(pdev, 0);
        if IS_ERR(regs) {
            return PTR_ERR(regs);
        }

        (*mqs_priv).regmap = devm_regmap_init_mmio_clk(
            &mut (*pdev).dev,
            c"core".as_ptr(),
            regs,
            &fsl_mqs_regmap_config,
        );
        if IS_ERR((*mqs_priv).regmap) {
            dev_err(
                &mut (*pdev).dev,
                c"failed to init regmap: %ld\n".as_ptr(),
                PTR_ERR((*mqs_priv).regmap),
            );
            return PTR_ERR((*mqs_priv).regmap);
        }

        (*mqs_priv).ipg = devm_clk_get(&mut (*pdev).dev, c"core".as_ptr());
        if IS_ERR((*mqs_priv).ipg) {
            dev_err(
                &mut (*pdev).dev,
                c"failed to get the clock: %ld\n".as_ptr(),
                PTR_ERR((*mqs_priv).ipg),
            );
            return PTR_ERR((*mqs_priv).ipg);
        }
    }

    (*mqs_priv).mclk = devm_clk_get(&mut (*pdev).dev, c"mclk".as_ptr());
    if IS_ERR((*mqs_priv).mclk) {
        dev_err(
            &mut (*pdev).dev,
            c"failed to get the clock: %ld\n".as_ptr(),
            PTR_ERR((*mqs_priv).mclk),
        );
        return PTR_ERR((*mqs_priv).mclk);
    }

    dev_set_drvdata(&mut (*pdev).dev, mqs_priv as *mut c_void);
    pm_runtime_enable(&mut (*pdev).dev);

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &soc_codec_fsl_mqs,
        &raw mut fsl_mqs_dai,
        1,
    );
    if ret != 0 {
        return ret;
    }

    0
}

unsafe extern "C" fn fsl_mqs_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn fsl_mqs_runtime_resume(dev: *mut device) -> c_int {
    let mqs_priv = dev_get_drvdata(dev) as *mut fsl_mqs;
    let mut ret: c_int;

    ret = clk_prepare_enable((*mqs_priv).ipg);
    if ret != 0 {
        dev_err(dev, c"failed to enable ipg clock\n".as_ptr());
        return ret;
    }

    ret = clk_prepare_enable((*mqs_priv).mclk);
    if ret != 0 {
        dev_err(dev, c"failed to enable mclk clock\n".as_ptr());
        clk_disable_unprepare((*mqs_priv).ipg);
        return ret;
    }

    regmap_write(
        (*mqs_priv).regmap,
        (*(*mqs_priv).soc).ctrl_off,
        (*mqs_priv).reg_mqs_ctrl,
    );
    0
}

unsafe extern "C" fn fsl_mqs_runtime_suspend(dev: *mut device) -> c_int {
    let mqs_priv = dev_get_drvdata(dev) as *mut fsl_mqs;

    regmap_read(
        (*mqs_priv).regmap,
        (*(*mqs_priv).soc).ctrl_off,
        &mut (*mqs_priv).reg_mqs_ctrl,
    );

    clk_disable_unprepare((*mqs_priv).mclk);
    clk_disable_unprepare((*mqs_priv).ipg);

    0
}

static fsl_mqs_pm_ops: dev_pm_ops = dev_pm_ops {
    runtime_suspend: Some(fsl_mqs_runtime_suspend),
    runtime_resume: Some(fsl_mqs_runtime_resume),
    runtime_idle: None,
    suspend: Some(pm_runtime_force_suspend),
    resume: Some(pm_runtime_force_resume),
};

static fsl_mqs_imx8qm_data: fsl_mqs_soc_data = fsl_mqs_soc_data {
    type_: reg_type::TYPE_REG_OWN,
    sm_index: 0,
    ctrl_off: REG_MQS_CTRL,
    en_mask: MQS_EN_MASK,
    en_shift: MQS_EN_SHIFT,
    rst_mask: MQS_SW_RST_MASK,
    rst_shift: MQS_SW_RST_SHIFT,
    osr_mask: MQS_OVERSAMPLE_MASK,
    osr_shift: MQS_OVERSAMPLE_SHIFT,
    div_mask: MQS_CLK_DIV_MASK,
    div_shift: MQS_CLK_DIV_SHIFT,
};

static fsl_mqs_imx6sx_data: fsl_mqs_soc_data = unsafe {
    fsl_mqs_soc_data {
        type_: reg_type::TYPE_REG_GPR,
        sm_index: 0,
        ctrl_off: IOMUXC_GPR2,
        en_mask: IMX6SX_GPR2_MQS_EN_MASK,
        en_shift: IMX6SX_GPR2_MQS_EN_SHIFT,
        rst_mask: IMX6SX_GPR2_MQS_SW_RST_MASK,
        rst_shift: IMX6SX_GPR2_MQS_SW_RST_SHIFT,
        osr_mask: IMX6SX_GPR2_MQS_OVERSAMPLE_MASK,
        osr_shift: IMX6SX_GPR2_MQS_OVERSAMPLE_SHIFT,
        div_mask: IMX6SX_GPR2_MQS_CLK_DIV_MASK,
        div_shift: IMX6SX_GPR2_MQS_CLK_DIV_SHIFT,
    }
};

static fsl_mqs_imx93_data: fsl_mqs_soc_data = fsl_mqs_soc_data {
    type_: reg_type::TYPE_REG_GPR,
    sm_index: 0,
    ctrl_off: 0x20,
    en_mask: BIT(1),
    en_shift: 1,
    rst_mask: BIT(2),
    rst_shift: 2,
    osr_mask: BIT(3),
    osr_shift: 3,
    div_mask: GENMASK(15, 8),
    div_shift: 8,
};

static fsl_mqs_imx95_aon_data: fsl_mqs_soc_data = unsafe {
    fsl_mqs_soc_data {
        type_: reg_type::TYPE_REG_SM,
        sm_index: SCMI_IMX95_CTRL_MQS1_SETTINGS,
        ctrl_off: 0x88,
        en_mask: BIT(1),
        en_shift: 1,
        rst_mask: BIT(2),
        rst_shift: 2,
        osr_mask: BIT(3),
        osr_shift: 3,
        div_mask: GENMASK(15, 8),
        div_shift: 8,
    }
};

static fsl_mqs_imx95_netc_data: fsl_mqs_soc_data = fsl_mqs_soc_data {
    type_: reg_type::TYPE_REG_GPR,
    sm_index: 0,
    ctrl_off: 0x0,
    en_mask: BIT(2),
    en_shift: 2,
    rst_mask: BIT(3),
    rst_shift: 3,
    osr_mask: BIT(4),
    osr_shift: 4,
    div_mask: GENMASK(16, 9),
    div_shift: 9,
};

static fsl_mqs_imx943_aon_data: fsl_mqs_soc_data = unsafe {
    fsl_mqs_soc_data {
        type_: reg_type::TYPE_REG_SM,
        sm_index: SCMI_IMX94_CTRL_MQS1_SETTINGS,
        ctrl_off: 0x88,
        en_mask: BIT(1),
        en_shift: 1,
        rst_mask: BIT(2),
        rst_shift: 2,
        osr_mask: BIT(3),
        osr_shift: 3,
        div_mask: GENMASK(15, 8),
        div_shift: 8,
    }
};

static fsl_mqs_imx943_wakeup_data: fsl_mqs_soc_data = unsafe {
    fsl_mqs_soc_data {
        type_: reg_type::TYPE_REG_SM,
        sm_index: SCMI_IMX94_CTRL_MQS2_SETTINGS,
        ctrl_off: 0x10,
        en_mask: BIT(1),
        en_shift: 1,
        rst_mask: BIT(2),
        rst_shift: 2,
        osr_mask: BIT(3),
        osr_shift: 3,
        div_mask: GENMASK(15, 8),
        div_shift: 8,
    }
};

static fsl_mqs_dt_ids: [of_device_id; 8] = [
    of_device_id {
        compatible: c"fsl,imx8qm-mqs".as_ptr(),
        data: &fsl_mqs_imx8qm_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx6sx-mqs".as_ptr(),
        data: &fsl_mqs_imx6sx_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx93-mqs".as_ptr(),
        data: &fsl_mqs_imx93_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx95-aonmix-mqs".as_ptr(),
        data: &fsl_mqs_imx95_aon_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx95-netcmix-mqs".as_ptr(),
        data: &fsl_mqs_imx95_netc_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx943-aonmix-mqs".as_ptr(),
        data: &fsl_mqs_imx943_aon_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: c"fsl,imx943-wakeupmix-mqs".as_ptr(),
        data: &fsl_mqs_imx943_wakeup_data as *const _ as *const c_void,
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, fsl_mqs_dt_ids); */

static mut fsl_mqs_driver: platform_driver = platform_driver {
    probe: Some(fsl_mqs_probe),
    remove: Some(fsl_mqs_remove),
    driver: device_driver {
        name: c"fsl-mqs".as_ptr(),
        of_match_table: fsl_mqs_dt_ids.as_ptr(),
        pm: &fsl_mqs_pm_ops,
    },
};

/* module_platform_driver(fsl_mqs_driver); */

/* MODULE_AUTHOR("Shengjiu Wang <Shengjiu.Wang@nxp.com>"); */
/* MODULE_DESCRIPTION("MQS codec driver"); */
/* MODULE_LICENSE("GPL v2"); */
/* MODULE_ALIAS("platform:fsl-mqs"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
