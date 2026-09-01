// SPDX-License-Identifier: GPL-2.0+
// Copyright 2020 NXP

// Translated from C implementation source. Kernel, ASoC, regmap, platform,
// PM, IRQ, and local fsl_aud2htx/imx-pcm definitions are external dependencies.

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

extern "C" {
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn snd_soc_dai_init_dma_data(
        cpu_dai: *mut snd_soc_dai,
        playback: *mut snd_dmaengine_dai_dma_data,
        capture: *mut snd_dmaengine_dai_dma_data,
    );
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
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
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn platform_get_irq(pdev: *mut platform_device, num: c_uint) -> c_int;
    fn devm_request_irq(
        dev: *mut device,
        irq: c_uint,
        handler: Option<unsafe extern "C" fn(c_int, *mut c_void) -> irqreturn_t>,
        irqflags: c_ulong,
        devname: *const c_char,
        dev_id: *mut c_void,
    ) -> c_int;
    fn dev_name(dev: *mut device) -> *const c_char;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut c_void);
    fn pm_runtime_enable(dev: *mut device);
    fn pm_runtime_disable(dev: *mut device);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const c_void,
        flags: c_uint,
    ) -> c_int;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn pm_runtime_force_suspend(dev: *mut device) -> c_int;
    fn pm_runtime_force_resume(dev: *mut device) -> c_int;
}

type c_ulong = core::ffi::c_ulong;

#[repr(C)]
pub struct snd_pcm_substream {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai {
    pub dev: *mut device,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
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
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct resource {
    pub start: dma_addr_t,
}

type dma_addr_t = u64;
type irqreturn_t = c_uint;

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub chan_name: *const c_char,
    pub maxburst: c_uint,
    pub addr: dma_addr_t,
}

#[repr(C)]
pub struct fsl_aud2htx {
    pub pdev: *mut platform_device,
    pub regmap: *mut regmap,
    pub bus_clk: *mut clk,
    pub dma_params_tx: snd_dmaengine_dai_dma_data,
    pub dma_params_rx: snd_dmaengine_dai_dma_data,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub probe: Option<unsafe extern "C" fn(*mut snd_soc_dai) -> c_int>,
    pub trigger: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, c_int, *mut snd_soc_dai) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const c_char,
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: u64,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub playback: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_uint,
    pub reg_stride: c_uint,
    pub val_bits: c_uint,
    pub max_register: c_uint,
    pub reg_defaults: *const reg_default,
    pub num_reg_defaults: c_uint,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub pm: *const dev_pm_ops,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
    pub driver: device_driver,
}

const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const IRQ_HANDLED: irqreturn_t = 1;
const REGCACHE_MAPLE: c_uint = 0;

const SNDRV_PCM_TRIGGER_START: c_int = 0;
const SNDRV_PCM_TRIGGER_STOP: c_int = 1;
const SNDRV_PCM_TRIGGER_PAUSE_PUSH: c_int = 3;
const SNDRV_PCM_TRIGGER_PAUSE_RELEASE: c_int = 4;
const SNDRV_PCM_TRIGGER_SUSPEND: c_int = 5;
const SNDRV_PCM_TRIGGER_RESUME: c_int = 6;

extern "C" {
    static AUD2HTX_CTRL: c_uint;
    static AUD2HTX_CTRL_EXT: c_uint;
    static AUD2HTX_WR: c_uint;
    static AUD2HTX_STATUS: c_uint;
    static AUD2HTX_IRQ_NOMASK: c_uint;
    static AUD2HTX_IRQ_MASKED: c_uint;
    static AUD2HTX_IRQ_MASK: c_uint;
    static AUD2HTX_CTRL_EN: c_uint;
    static AUD2HTX_CTRE_DE: c_uint;
    static AUD2HTX_CTRE_DT_MASK: c_uint;
    static AUD2HTX_WM_HIGH_IRQ_MASK: c_uint;
    static AUD2HTX_WM_LOW_IRQ_MASK: c_uint;
    static AUD2HTX_OVF_MASK: c_uint;
    static AUD2HTX_CTRE_WL_MASK: c_uint;
    static AUD2HTX_CTRE_WH_MASK: c_uint;
    static AUD2HTX_WTMK_LOW: c_uint;
    static AUD2HTX_WTMK_HIGH: c_uint;
    static AUD2HTX_CTRE_WL_SHIFT: c_uint;
    static AUD2HTX_CTRE_WH_SHIFT: c_uint;
    static AUD2HTX_MAXBURST: c_uint;
    static FSL_AUD2HTX_FORMATS: u64;
    static SNDRV_PCM_RATE_32000: c_uint;
    static SNDRV_PCM_RATE_44100: c_uint;
    static SNDRV_PCM_RATE_48000: c_uint;
    static SNDRV_PCM_RATE_88200: c_uint;
    static SNDRV_PCM_RATE_96000: c_uint;
    static SNDRV_PCM_RATE_176400: c_uint;
    static SNDRV_PCM_RATE_192000: c_uint;
}

unsafe extern "C" fn fsl_aud2htx_trigger(
    _substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let aud2htx = snd_soc_dai_get_drvdata(dai) as *mut fsl_aud2htx;

    match cmd {
        SNDRV_PCM_TRIGGER_START
        | SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            regmap_update_bits(
                (*aud2htx).regmap,
                AUD2HTX_CTRL,
                AUD2HTX_CTRL_EN,
                AUD2HTX_CTRL_EN,
            );
            regmap_update_bits(
                (*aud2htx).regmap,
                AUD2HTX_CTRL_EXT,
                AUD2HTX_CTRE_DE,
                AUD2HTX_CTRE_DE,
            );
        }
        SNDRV_PCM_TRIGGER_SUSPEND | SNDRV_PCM_TRIGGER_STOP | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            regmap_update_bits((*aud2htx).regmap, AUD2HTX_CTRL_EXT, AUD2HTX_CTRE_DE, 0);
            regmap_update_bits((*aud2htx).regmap, AUD2HTX_CTRL, AUD2HTX_CTRL_EN, 0);
        }
        _ => return -EINVAL,
    }
    0
}

unsafe extern "C" fn fsl_aud2htx_dai_probe(cpu_dai: *mut snd_soc_dai) -> c_int {
    let aud2htx = dev_get_drvdata((*cpu_dai).dev) as *mut fsl_aud2htx;

    /* DMA request when number of entries < WTMK_LOW */
    regmap_update_bits((*aud2htx).regmap, AUD2HTX_CTRL_EXT, AUD2HTX_CTRE_DT_MASK, 0);

    /* Disable interrupts*/
    regmap_update_bits(
        (*aud2htx).regmap,
        AUD2HTX_IRQ_MASK,
        AUD2HTX_WM_HIGH_IRQ_MASK | AUD2HTX_WM_LOW_IRQ_MASK | AUD2HTX_OVF_MASK,
        AUD2HTX_WM_HIGH_IRQ_MASK | AUD2HTX_WM_LOW_IRQ_MASK | AUD2HTX_OVF_MASK,
    );

    /* Configure watermark */
    regmap_update_bits(
        (*aud2htx).regmap,
        AUD2HTX_CTRL_EXT,
        AUD2HTX_CTRE_WL_MASK,
        AUD2HTX_WTMK_LOW << AUD2HTX_CTRE_WL_SHIFT,
    );
    regmap_update_bits(
        (*aud2htx).regmap,
        AUD2HTX_CTRL_EXT,
        AUD2HTX_CTRE_WH_MASK,
        AUD2HTX_WTMK_HIGH << AUD2HTX_CTRE_WH_SHIFT,
    );

    snd_soc_dai_init_dma_data(
        cpu_dai,
        &mut (*aud2htx).dma_params_tx,
        &mut (*aud2htx).dma_params_rx,
    );

    0
}

static fsl_aud2htx_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    probe: Some(fsl_aud2htx_dai_probe),
    trigger: Some(fsl_aud2htx_trigger),
};

static mut fsl_aud2htx_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        stream_name: b"CPU-Playback\0".as_ptr() as *const c_char,
        channels_min: 1,
        channels_max: 8,
        rates: unsafe {
            SNDRV_PCM_RATE_32000
                | SNDRV_PCM_RATE_44100
                | SNDRV_PCM_RATE_48000
                | SNDRV_PCM_RATE_88200
                | SNDRV_PCM_RATE_96000
                | SNDRV_PCM_RATE_176400
                | SNDRV_PCM_RATE_192000
        },
        formats: unsafe { FSL_AUD2HTX_FORMATS },
    },
    ops: &fsl_aud2htx_dai_ops,
};

static fsl_aud2htx_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"fsl-aud2htx\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

static fsl_aud2htx_reg_defaults: [reg_default; 7] = unsafe {
    [
        reg_default { reg: AUD2HTX_CTRL, def: 0x00000000 },
        reg_default { reg: AUD2HTX_CTRL_EXT, def: 0x00000000 },
        reg_default { reg: AUD2HTX_WR, def: 0x00000000 },
        reg_default { reg: AUD2HTX_STATUS, def: 0x00000000 },
        reg_default { reg: AUD2HTX_IRQ_NOMASK, def: 0x00000000 },
        reg_default { reg: AUD2HTX_IRQ_MASKED, def: 0x00000000 },
        reg_default { reg: AUD2HTX_IRQ_MASK, def: 0x00000000 },
    ]
};

unsafe extern "C" fn fsl_aud2htx_readable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == AUD2HTX_CTRL => true,
        x if x == AUD2HTX_CTRL_EXT => true,
        x if x == AUD2HTX_STATUS => true,
        x if x == AUD2HTX_IRQ_NOMASK => true,
        x if x == AUD2HTX_IRQ_MASKED => true,
        x if x == AUD2HTX_IRQ_MASK => true,
        _ => false,
    }
}

unsafe extern "C" fn fsl_aud2htx_writeable_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == AUD2HTX_CTRL => true,
        x if x == AUD2HTX_CTRL_EXT => true,
        x if x == AUD2HTX_WR => true,
        x if x == AUD2HTX_IRQ_NOMASK => true,
        x if x == AUD2HTX_IRQ_MASKED => true,
        x if x == AUD2HTX_IRQ_MASK => true,
        _ => false,
    }
}

unsafe extern "C" fn fsl_aud2htx_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    match reg {
        x if x == AUD2HTX_STATUS => true,
        x if x == AUD2HTX_IRQ_NOMASK => true,
        x if x == AUD2HTX_IRQ_MASKED => true,
        _ => false,
    }
}

static fsl_aud2htx_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,

    max_register: unsafe { AUD2HTX_IRQ_MASK },
    reg_defaults: fsl_aud2htx_reg_defaults.as_ptr(),
    num_reg_defaults: fsl_aud2htx_reg_defaults.len() as c_uint,
    readable_reg: Some(fsl_aud2htx_readable_reg),
    volatile_reg: Some(fsl_aud2htx_volatile_reg),
    writeable_reg: Some(fsl_aud2htx_writeable_reg),
    cache_type: REGCACHE_MAPLE,
};

static fsl_aud2htx_dt_ids: [of_device_id; 2] = [
    of_device_id {
        compatible: b"fsl,imx8mp-aud2htx\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, fsl_aud2htx_dt_ids);

unsafe extern "C" fn fsl_aud2htx_isr(_irq: c_int, _dev_id: *mut c_void) -> irqreturn_t {
    IRQ_HANDLED
}

unsafe extern "C" fn fsl_aud2htx_probe(pdev: *mut platform_device) -> c_int {
    let mut aud2htx: *mut fsl_aud2htx;
    let mut res: *mut resource = ptr::null_mut();
    let regs: *mut c_void;
    let mut ret: c_int;
    let irq: c_int;

    aud2htx = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<fsl_aud2htx>(),
        GFP_KERNEL,
    ) as *mut fsl_aud2htx;
    if aud2htx.is_null() {
        return -ENOMEM;
    }

    (*aud2htx).pdev = pdev;

    regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(regs) {
        return PTR_ERR(regs);
    }

    (*aud2htx).regmap =
        devm_regmap_init_mmio(&mut (*pdev).dev, regs, &fsl_aud2htx_regmap_config);
    if IS_ERR((*aud2htx).regmap as *const c_void) {
        dev_err(&mut (*pdev).dev, b"failed to init regmap\0".as_ptr() as *const c_char);
        return PTR_ERR((*aud2htx).regmap as *const c_void);
    }

    irq = platform_get_irq(pdev, 0);
    if irq < 0 {
        return irq;
    }

    ret = devm_request_irq(
        &mut (*pdev).dev,
        irq as c_uint,
        Some(fsl_aud2htx_isr),
        0,
        dev_name(&mut (*pdev).dev),
        aud2htx as *mut c_void,
    );
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"failed to claim irq %u: %d\n\0".as_ptr() as *const c_char,
            irq,
            ret,
        );
        return ret;
    }

    (*aud2htx).bus_clk = devm_clk_get(&mut (*pdev).dev, b"bus\0".as_ptr() as *const c_char);
    if IS_ERR((*aud2htx).bus_clk as *const c_void) {
        dev_err(&mut (*pdev).dev, b"failed to get mem clock\n\0".as_ptr() as *const c_char);
        return PTR_ERR((*aud2htx).bus_clk as *const c_void);
    }

    (*aud2htx).dma_params_tx.chan_name = b"tx\0".as_ptr() as *const c_char;
    (*aud2htx).dma_params_tx.maxburst = AUD2HTX_MAXBURST;
    (*aud2htx).dma_params_tx.addr = (*res).start + AUD2HTX_WR as dma_addr_t;

    platform_set_drvdata(pdev, aud2htx as *mut c_void);
    pm_runtime_enable(&mut (*pdev).dev);

    regcache_cache_only((*aud2htx).regmap, true);

    /*
     * Register platform component before registering cpu dai for there
     * is not defer probe for platform component in snd_soc_add_pcm_runtime().
     */
    ret = devm_snd_dmaengine_pcm_register(&mut (*pdev).dev, ptr::null(), 0);
    if ret != 0 {
        dev_err(&mut (*pdev).dev, b"failed to pcm register\n\0".as_ptr() as *const c_char);
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        &fsl_aud2htx_component,
        &mut fsl_aud2htx_dai,
        1,
    );
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"failed to register ASoC DAI\n\0".as_ptr() as *const c_char,
        );
        pm_runtime_disable(&mut (*pdev).dev);
        return ret;
    }

    ret
}

unsafe extern "C" fn fsl_aud2htx_remove(pdev: *mut platform_device) {
    pm_runtime_disable(&mut (*pdev).dev);
}

unsafe extern "C" fn fsl_aud2htx_runtime_suspend(dev: *mut device) -> c_int {
    let aud2htx = dev_get_drvdata(dev) as *mut fsl_aud2htx;

    regcache_cache_only((*aud2htx).regmap, true);
    clk_disable_unprepare((*aud2htx).bus_clk);

    0
}

unsafe extern "C" fn fsl_aud2htx_runtime_resume(dev: *mut device) -> c_int {
    let aud2htx = dev_get_drvdata(dev) as *mut fsl_aud2htx;
    let ret: c_int;

    ret = clk_prepare_enable((*aud2htx).bus_clk);
    if ret != 0 {
        return ret;
    }

    regcache_cache_only((*aud2htx).regmap, false);
    regcache_mark_dirty((*aud2htx).regmap);
    regcache_sync((*aud2htx).regmap);

    0
}

// RUNTIME_PM_OPS(fsl_aud2htx_runtime_suspend, fsl_aud2htx_runtime_resume, NULL)
// SYSTEM_SLEEP_PM_OPS(pm_runtime_force_suspend, pm_runtime_force_resume)
static fsl_aud2htx_pm_ops: dev_pm_ops = dev_pm_ops { _private: [] };

static mut fsl_aud2htx_driver: platform_driver = platform_driver {
    probe: Some(fsl_aud2htx_probe),
    remove: Some(fsl_aud2htx_remove),
    driver: device_driver {
        name: b"fsl-aud2htx\0".as_ptr() as *const c_char,
        pm: &fsl_aud2htx_pm_ops,
        of_match_table: fsl_aud2htx_dt_ids.as_ptr(),
    },
};
// module_platform_driver(fsl_aud2htx_driver);

// MODULE_AUTHOR("Shengjiu Wang <Shengjiu.Wang@nxp.com>");
// MODULE_DESCRIPTION("NXP AUD2HTX driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
