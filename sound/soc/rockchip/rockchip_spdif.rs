// SPDX-License-Identifier: GPL-2.0-only
/* sound/soc/rockchip/rk_spdif.c
 *
 * ALSA SoC Audio Layer - Rockchip I2S Controller driver
 *
 * Copyright (c) 2014 Rockchip Electronics Co. Ltd.
 * Author: Jianqun <jay.xu@rock-chips.com>
 * Copyright (c) 2015-2026 Collabora Ltd.
 * Author: Sjoerd Simons <sjoerd.simons@collabora.co.uk>
 */

/* C dependencies translated as external Rust dependencies:
 * linux/bitfield.h, linux/module.h, linux/delay.h, linux/clk.h,
 * linux/pm_runtime.h, linux/mfd/syscon.h, linux/regmap.h,
 * sound/pcm_params.h, sound/pcm_iec958.h, sound/dmaengine_pcm.h,
 * and rockchip_spdif.h.
 */

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_dmaengine_dai_dma_data {
    pub addr: resource_size_t,
    pub addr_width: c_int,
    pub maxburst: c_uint,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
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
    _private: [u8; 0],
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub set_sysclk: Option<
        unsafe extern "C" fn(
            dai: *mut snd_soc_dai,
            clk_id: c_int,
            freq: c_uint,
            dir: c_int,
        ) -> c_int,
    >,
    pub probe: Option<unsafe extern "C" fn(dai: *mut snd_soc_dai) -> c_int>,
    pub hw_params: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            params: *mut snd_pcm_hw_params,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
    pub trigger: Option<
        unsafe extern "C" fn(
            substream: *mut snd_pcm_substream,
            cmd: c_int,
            dai: *mut snd_soc_dai,
        ) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub stream_name: *const u8,
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
    pub name: *const u8,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub reg_stride: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub writeable_reg:
        Option<unsafe extern "C" fn(dev: *mut device, reg: c_uint) -> bool>,
    pub readable_reg:
        Option<unsafe extern "C" fn(dev: *mut device, reg: c_uint) -> bool>,
    pub volatile_reg:
        Option<unsafe extern "C" fn(dev: *mut device, reg: c_uint) -> bool>,
    pub cache_type: c_int,
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
pub struct resource {
    pub start: resource_size_t,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device_with_of_node,
}

#[repr(C)]
pub struct device_with_of_node {
    pub of_node: *mut device_node,
}

#[repr(C)]
pub struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const u8,
    pub data: *const c_void,
}

#[repr(C)]
pub struct platform_driver_inner {
    pub name: *const u8,
    pub of_match_table: *const of_device_id,
    pub pm: *const dev_pm_ops,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(pdev: *mut platform_device) -> c_int>,
    pub driver: platform_driver_inner,
}

pub type resource_size_t = u64;

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum rk_spdif_type {
    RK_SPDIF_RK3066,
    RK_SPDIF_RK3188,
    RK_SPDIF_RK3288,
    RK_SPDIF_RK3366,
}

/*
 *      |  7  |  6  |  5  |  4  |  3  |  2  |  1  |  0  |
 * CS0: |   Mode    |        d        |  c  |  b  |  a  |
 * CS1: |               Category Code                   |
 * CS2: |    Channel Number     |     Source Number     |
 * CS3: |    Clock Accuracy     |     Sample Freq       |
 * CS4: |    Ori Sample Freq    |     Word Length       |
 * CS5: |                                   |   CGMS-A  |
 * CS6~CS23: Reserved
 *
 * a: use of channel status block
 * b: linear PCM identification: 0 for lpcm, 1 for nlpcm
 * c: copyright information
 * d: additional format information
 */
const CS_BYTE: usize = 6;
const fn CS_FRAME(c: u16) -> c_uint {
    (((c as c_uint) << 16) | (c as c_uint)) as c_uint
}

const RK3288_GRF_SOC_CON2: c_uint = 0x24c;

#[repr(C)]
struct rk_spdif_dev {
    dev: *mut device,
    mclk: *mut clk,
    hclk: *mut clk,
    playback_dma_data: snd_dmaengine_dai_dma_data,
    regmap: *mut regmap,
}

extern "C" {
    fn dev_get_drvdata(dev: *mut device) -> *mut c_void;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn regcache_cache_only(map: *mut regmap, enable: bool);
    fn regcache_mark_dirty(map: *mut regmap);
    fn regcache_sync(map: *mut regmap) -> c_int;
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> c_int;
    fn clk_get_rate(clk: *mut clk) -> c_uint;
    fn clk_set_rate(clk: *mut clk, rate: c_uint) -> c_int;
    fn dev_err(dev: *mut device, fmt: *const u8, ...) -> c_int;
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn snd_pcm_create_iec958_consumer_hw_params(
        params: *mut snd_pcm_hw_params,
        cs: *mut u8,
        len: usize,
    ) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_uint;
    fn params_format(params: *mut snd_pcm_hw_params) -> c_int;
    fn udelay(usecs: c_uint);
    fn snd_soc_dai_dma_data_set_playback(
        dai: *mut snd_soc_dai,
        data: *mut snd_dmaengine_dai_dma_data,
    );
    fn pm_runtime_status_suspended(dev: *mut device) -> bool;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn syscon_regmap_lookup_by_phandle(
        np: *mut device_node,
        property: *const u8,
    ) -> *mut regmap;
    fn dev_err_probe(dev: *mut device, err: isize, fmt: *const u8, ...) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const u8) -> *mut clk;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn devm_regmap_init_mmio_clk(
        dev: *mut device,
        clk_id: *const u8,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_add_action_or_reset(
        dev: *mut device,
        action: unsafe extern "C" fn(data: *mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn devm_pm_runtime_enable(dev: *mut device);
    fn pm_runtime_enabled(dev: *mut device) -> bool;
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
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> isize;
    fn pm_ptr(ops: *const dev_pm_ops) -> *const dev_pm_ops;
}

extern "C" {
    static rk_spdif_pm_ops: dev_pm_ops;
}

unsafe extern "C" fn rk_spdif_runtime_suspend(dev: *mut device) -> c_int {
    let spdif = dev_get_drvdata(dev) as *mut rk_spdif_dev;

    regcache_cache_only((*spdif).regmap, true);
    clk_disable_unprepare((*spdif).mclk);
    clk_disable_unprepare((*spdif).hclk);

    0
}

unsafe extern "C" fn rk_spdif_runtime_resume(dev: *mut device) -> c_int {
    let spdif = dev_get_drvdata(dev) as *mut rk_spdif_dev;
    let mut ret: c_int;

    ret = clk_prepare_enable((*spdif).hclk);
    if ret != 0 {
        dev_err(
            (*spdif).dev,
            b"hclk clock enable failed %d\n\0".as_ptr(),
            ret,
        );
        return ret;
    }

    ret = clk_prepare_enable((*spdif).mclk);
    if ret != 0 {
        clk_disable_unprepare((*spdif).hclk);
        dev_err(
            (*spdif).dev,
            b"mclk clock enable failed %d\n\0".as_ptr(),
            ret,
        );
        return ret;
    }

    regcache_cache_only((*spdif).regmap, false);
    regcache_mark_dirty((*spdif).regmap);

    ret = regcache_sync((*spdif).regmap);
    if ret != 0 {
        regcache_cache_only((*spdif).regmap, true);
        clk_disable_unprepare((*spdif).mclk);
        clk_disable_unprepare((*spdif).hclk);
    }

    ret
}

unsafe extern "C" fn rk_spdif_hw_params(
    substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let spdif = snd_soc_dai_get_drvdata(dai) as *mut rk_spdif_dev;
    let mclk_rate: c_uint = clk_get_rate((*spdif).mclk);
    let mut val: c_uint = SPDIF_CFGR_HALFWORD_ENABLE;
    let mut bmc: c_int;
    let mut div: c_int;
    let mut ret: c_int;
    let fc: *mut u16;
    let mut cs = [0u8; CS_BYTE];

    let _ = substream;

    ret = snd_pcm_create_iec958_consumer_hw_params(params, cs.as_mut_ptr(), cs.len());
    if ret < 0 {
        return ret;
    }

    fc = cs.as_mut_ptr() as *mut u16;
    for i in 0..(CS_BYTE / 2) {
        regmap_write((*spdif).regmap, SPDIF_CHNSRn(i as c_uint), CS_FRAME(*fc.add(i)));
    }

    regmap_update_bits(
        (*spdif).regmap,
        SPDIF_CFGR,
        SPDIF_CFGR_CSE_MASK,
        SPDIF_CFGR_CSE_EN,
    );

    /* bmc = 128fs */
    bmc = (128u32.wrapping_mul(params_rate(params))) as c_int;
    div = DIV_ROUND_CLOSEST(mclk_rate as c_int, bmc);
    val |= SPDIF_CFGR_CLK_DIV(div as c_uint);

    match params_format(params) {
        SNDRV_PCM_FORMAT_S16_LE => {
            val |= SPDIF_CFGR_VDW_16;
            val |= SPDIF_CFGR_ADJ_RIGHT_J;
        }
        SNDRV_PCM_FORMAT_S20_3LE => {
            val |= SPDIF_CFGR_VDW_20;
            val |= SPDIF_CFGR_ADJ_RIGHT_J;
        }
        SNDRV_PCM_FORMAT_S24_LE => {
            val |= SPDIF_CFGR_VDW_24;
            val |= SPDIF_CFGR_ADJ_RIGHT_J;
        }
        SNDRV_PCM_FORMAT_S32_LE => {
            val |= SPDIF_CFGR_VDW_24;
            val |= SPDIF_CFGR_ADJ_LEFT_J;
        }
        _ => return -EINVAL,
    }

    /*
     * clear MCLK domain logic before setting Fmclk and Fsdo to ensure
     * that switching between S16_LE and S32_LE audio does not result
     * in accidential channels swap.
     */
    regmap_update_bits(
        (*spdif).regmap,
        SPDIF_CFGR,
        SPDIF_CFGR_CLR_MASK,
        SPDIF_CFGR_CLR_EN,
    );
    udelay(1);

    ret = regmap_update_bits(
        (*spdif).regmap,
        SPDIF_CFGR,
        SPDIF_CFGR_CLK_DIV_MASK
            | SPDIF_CFGR_HALFWORD_MASK
            | SDPIF_CFGR_VDW_MASK
            | SPDIF_CFGR_ADJ_MASK,
        val,
    );

    ret
}

unsafe extern "C" fn rk_spdif_trigger(
    substream: *mut snd_pcm_substream,
    cmd: c_int,
    dai: *mut snd_soc_dai,
) -> c_int {
    let spdif = snd_soc_dai_get_drvdata(dai) as *mut rk_spdif_dev;
    let mut ret: c_int;

    let _ = substream;

    match cmd {
        SNDRV_PCM_TRIGGER_START
        | SNDRV_PCM_TRIGGER_RESUME
        | SNDRV_PCM_TRIGGER_PAUSE_RELEASE => {
            ret = regmap_update_bits(
                (*spdif).regmap,
                SPDIF_DMACR,
                SPDIF_DMACR_TDE_MASK | SPDIF_DMACR_TDL_MASK,
                SPDIF_DMACR_TDE_ENABLE | SPDIF_DMACR_TDL(16),
            );

            if ret != 0 {
                return ret;
            }

            ret = regmap_update_bits(
                (*spdif).regmap,
                SPDIF_XFER,
                SPDIF_XFER_TXS_MASK,
                SPDIF_XFER_TXS_START,
            );
        }
        SNDRV_PCM_TRIGGER_SUSPEND
        | SNDRV_PCM_TRIGGER_STOP
        | SNDRV_PCM_TRIGGER_PAUSE_PUSH => {
            ret = regmap_update_bits(
                (*spdif).regmap,
                SPDIF_DMACR,
                SPDIF_DMACR_TDE_MASK,
                SPDIF_DMACR_TDE_DISABLE,
            );

            if ret != 0 {
                return ret;
            }

            ret = regmap_update_bits(
                (*spdif).regmap,
                SPDIF_XFER,
                SPDIF_XFER_TXS_MASK,
                SPDIF_XFER_TXS_STOP,
            );
        }
        _ => {
            ret = -EINVAL;
        }
    }

    ret
}

unsafe extern "C" fn rk_spdif_dai_probe(dai: *mut snd_soc_dai) -> c_int {
    let spdif = snd_soc_dai_get_drvdata(dai) as *mut rk_spdif_dev;

    snd_soc_dai_dma_data_set_playback(dai, &mut (*spdif).playback_dma_data);

    0
}

unsafe extern "C" fn rk_spdif_set_sysclk(
    dai: *mut snd_soc_dai,
    clk_id: c_int,
    freq: c_uint,
    dir: c_int,
) -> c_int {
    let spdif = snd_soc_dai_get_drvdata(dai) as *mut rk_spdif_dev;
    let ret: c_int;

    let _ = clk_id;
    let _ = dir;

    if freq == 0 {
        return 0;
    }

    ret = clk_set_rate((*spdif).mclk, freq);
    if ret != 0 {
        dev_err((*spdif).dev, b"Failed to set mclk: %d\n\0".as_ptr(), ret);
    }

    ret
}

static rk_spdif_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    set_sysclk: Some(rk_spdif_set_sysclk),
    probe: Some(rk_spdif_dai_probe),
    hw_params: Some(rk_spdif_hw_params),
    trigger: Some(rk_spdif_trigger),
};

static mut rk_spdif_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    playback: snd_soc_pcm_stream {
        stream_name: b"Playback\0".as_ptr(),
        channels_min: 2,
        channels_max: 2,
        rates: SNDRV_PCM_RATE_8000_192000,
        formats: (SNDRV_PCM_FMTBIT_S16_LE
            | SNDRV_PCM_FMTBIT_S20_3LE
            | SNDRV_PCM_FMTBIT_S24_LE
            | SNDRV_PCM_FMTBIT_S32_LE) as u64,
    },
    ops: &rk_spdif_dai_ops,
};

static rk_spdif_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"rockchip-spdif\0".as_ptr(),
    legacy_dai_naming: 1,
};

unsafe extern "C" fn rk_spdif_wr_reg(dev: *mut device, reg: c_uint) -> bool {
    let _ = dev;

    match reg {
        SPDIF_CFGR
        | SPDIF_DMACR
        | SPDIF_INTCR
        | SPDIF_XFER
        | SPDIF_SMPDR => true,
        r if r >= SPDIF_VLDFRn(0) && r <= SPDIF_VLDFRn(11) => true,
        r if r >= SPDIF_USRDRn(0) && r <= SPDIF_USRDRn(11) => true,
        r if r >= SPDIF_CHNSRn(0) && r <= SPDIF_CHNSRn(11) => true,
        _ => false,
    }
}

unsafe extern "C" fn rk_spdif_rd_reg(dev: *mut device, reg: c_uint) -> bool {
    let _ = dev;

    match reg {
        SPDIF_CFGR
        | SPDIF_SDBLR
        | SPDIF_INTCR
        | SPDIF_INTSR
        | SPDIF_XFER
        | SPDIF_SMPDR => true,
        r if r >= SPDIF_VLDFRn(0) && r <= SPDIF_VLDFRn(11) => true,
        r if r >= SPDIF_USRDRn(0) && r <= SPDIF_USRDRn(11) => true,
        r if r >= SPDIF_CHNSRn(0) && r <= SPDIF_CHNSRn(11) => true,
        _ => false,
    }
}

unsafe extern "C" fn rk_spdif_volatile_reg(dev: *mut device, reg: c_uint) -> bool {
    let _ = dev;

    match reg {
        SPDIF_INTSR | SPDIF_SDBLR | SPDIF_SMPDR => true,
        _ => false,
    }
}

static rk_spdif_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: SPDIF_VERSION,
    writeable_reg: Some(rk_spdif_wr_reg),
    readable_reg: Some(rk_spdif_rd_reg),
    volatile_reg: Some(rk_spdif_volatile_reg),
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn rk_spdif_suspend(data: *mut c_void) {
    let dev = data as *mut device;

    if !pm_runtime_status_suspended(dev) {
        rk_spdif_runtime_suspend(dev);
    }
}

unsafe extern "C" fn rk_spdif_probe(pdev: *mut platform_device) -> c_int {
    let np = (*(pdev as *mut platform_device)).dev.of_node;
    let spdif_type: rk_spdif_type;
    let spdif: *mut rk_spdif_dev;
    let mut res: *mut resource = core::ptr::null_mut();
    let regs: *mut c_void;
    let mut ret: c_int;
    let dev = &mut (*(pdev as *mut platform_device)).dev as *mut device_with_of_node as *mut device;

    spdif_type = core::mem::transmute::<usize, rk_spdif_type>(device_get_match_data(dev) as usize);
    if spdif_type == rk_spdif_type::RK_SPDIF_RK3288 {
        let grf: *mut regmap;

        grf = syscon_regmap_lookup_by_phandle(np, b"rockchip,grf\0".as_ptr());
        if IS_ERR(grf as *const c_void) {
            return dev_err_probe(
                dev,
                PTR_ERR(grf as *const c_void),
                b"rockchip_spdif missing 'rockchip,grf'\n\0".as_ptr(),
            );
        }

        /* Select the 8 channel SPDIF solution on RK3288 as
         * the 2 channel one does not appear to work
         */
        regmap_write(grf, RK3288_GRF_SOC_CON2, BIT(1) << 16);
    }

    spdif = devm_kzalloc(dev, core::mem::size_of::<rk_spdif_dev>(), GFP_KERNEL) as *mut rk_spdif_dev;
    if spdif.is_null() {
        return -ENOMEM;
    }

    (*spdif).hclk = devm_clk_get(dev, b"hclk\0".as_ptr());
    if IS_ERR((*spdif).hclk as *const c_void) {
        return PTR_ERR((*spdif).hclk as *const c_void) as c_int;
    }

    (*spdif).mclk = devm_clk_get(dev, b"mclk\0".as_ptr());
    if IS_ERR((*spdif).mclk as *const c_void) {
        return PTR_ERR((*spdif).mclk as *const c_void) as c_int;
    }

    regs = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    if IS_ERR(regs as *const c_void) {
        return PTR_ERR(regs as *const c_void) as c_int;
    }

    (*spdif).regmap =
        devm_regmap_init_mmio_clk(dev, b"hclk\0".as_ptr(), regs, &rk_spdif_regmap_config);
    if IS_ERR((*spdif).regmap as *const c_void) {
        return PTR_ERR((*spdif).regmap as *const c_void) as c_int;
    }

    (*spdif).playback_dma_data.addr = (*res).start + SPDIF_SMPDR as resource_size_t;
    (*spdif).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
    (*spdif).playback_dma_data.maxburst = 4;

    (*spdif).dev = dev;
    dev_set_drvdata(dev, spdif as *mut c_void);

    ret = devm_add_action_or_reset(dev, rk_spdif_suspend, dev as *mut c_void);
    if ret != 0 {
        return ret;
    }

    devm_pm_runtime_enable(dev);

    if !pm_runtime_enabled(dev) {
        ret = rk_spdif_runtime_resume(dev);
        if ret != 0 {
            return ret;
        }
    }

    ret = devm_snd_dmaengine_pcm_register(dev, core::ptr::null(), 0);
    if ret != 0 {
        return ret;
    }

    ret = devm_snd_soc_register_component(dev, &rk_spdif_component, &mut rk_spdif_dai, 1);
    if ret != 0 {
        return ret;
    }

    0
}

/* RUNTIME_PM_OPS(rk_spdif_runtime_suspend, rk_spdif_runtime_resume, NULL) */

static rk_spdif_match: [of_device_id; 10] = [
    of_device_id {
        compatible: b"rockchip,rk3066-spdif\0".as_ptr(),
        data: rk_spdif_type::RK_SPDIF_RK3066 as usize as *const c_void,
    },
    of_device_id {
        compatible: b"rockchip,rk3188-spdif\0".as_ptr(),
        data: rk_spdif_type::RK_SPDIF_RK3188 as usize as *const c_void,
    },
    of_device_id {
        compatible: b"rockchip,rk3228-spdif\0".as_ptr(),
        data: rk_spdif_type::RK_SPDIF_RK3366 as usize as *const c_void,
    },
    of_device_id {
        compatible: b"rockchip,rk3288-spdif\0".as_ptr(),
        data: rk_spdif_type::RK_SPDIF_RK3288 as usize as *const c_void,
    },
    of_device_id {
        compatible: b"rockchip,rk3328-spdif\0".as_ptr(),
        data: rk_spdif_type::RK_SPDIF_RK3366 as usize as *const c_void,
    },
    of_device_id {
        compatible: b"rockchip,rk3366-spdif\0".as_ptr(),
        data: rk_spdif_type::RK_SPDIF_RK3366 as usize as *const c_void,
    },
    of_device_id {
        compatible: b"rockchip,rk3368-spdif\0".as_ptr(),
        data: rk_spdif_type::RK_SPDIF_RK3366 as usize as *const c_void,
    },
    of_device_id {
        compatible: b"rockchip,rk3399-spdif\0".as_ptr(),
        data: rk_spdif_type::RK_SPDIF_RK3366 as usize as *const c_void,
    },
    of_device_id {
        compatible: b"rockchip,rk3568-spdif\0".as_ptr(),
        data: rk_spdif_type::RK_SPDIF_RK3366 as usize as *const c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];
/* MODULE_DEVICE_TABLE(of, rk_spdif_match); */

static mut rk_spdif_driver: platform_driver = platform_driver {
    probe: Some(rk_spdif_probe),
    driver: platform_driver_inner {
        name: b"rockchip-spdif\0".as_ptr(),
        of_match_table: rk_spdif_match.as_ptr(),
        pm: unsafe { pm_ptr(&rk_spdif_pm_ops) },
    },
};
/* module_platform_driver(rk_spdif_driver); */

/* MODULE_ALIAS("platform:rockchip-spdif"); */
/* MODULE_DESCRIPTION("ROCKCHIP SPDIF transceiver Interface"); */
/* MODULE_AUTHOR("Sjoerd Simons <sjoerd.simons@collabora.co.uk>"); */
/* MODULE_LICENSE("GPL v2"); */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
