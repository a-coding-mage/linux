// SPDX-License-Identifier: GPL-2.0
//
// Loongson I2S controller master mode dirver(platform device)
//
// Copyright (C) 2023-2026 Loongson Technology Corporation Limited
//
// Author: Yingkun Meng <mengyingkun@loongson.cn>
//         Binbin Zhou <zhoubinbin@loongson.cn>

// Dependencies from Linux kernel and local headers:
// linux/clk.h, linux/dma-mapping.h, linux/module.h, linux/of_dma.h,
// linux/platform_device.h, linux/pm_runtime.h, sound/dmaengine_pcm.h,
// sound/pcm.h, sound/pcm_params.h, sound/soc.h, loongson_i2s.h,
// loongson_dma.h.

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

/* Loongson-2K1000 APBDMA routing */
const LOONGSON_I2S_RX_DMA_OFFSET: c_int = 21;
const LOONGSON_I2S_TX_DMA_OFFSET: c_int = 18;

const LOONGSON_DMA0_CONF: c_int = 0x0;
const LOONGSON_DMA1_CONF: c_int = 0x1;
const LOONGSON_DMA2_CONF: c_int = 0x2;
const LOONGSON_DMA3_CONF: c_int = 0x3;
const LOONGSON_DMA4_CONF: c_int = 0x4;

const GFP_KERNEL: c_uint = 0;
const ENOMEM: c_int = 12;
const EINVAL: c_int = 22;
const DMA_SLAVE_BUSWIDTH_4_BYTES: c_int = 4;
const SND_DMAENGINE_PCM_FLAG_COMPAT: c_uint = 0;
const LS_I2S_TX_DATA: resource_size_t = 0;
const LS_I2S_RX_DATA: resource_size_t = 0;
const LS_I2S_CTRL: c_uint = 0;
const I2S_CTRL_RESET: c_uint = 0;
const LS_I2S_DRVNAME: *const c_char = b"loongson-i2s\0".as_ptr().cast();

type c_uint = u32;
type resource_size_t = u64;

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct resource {
    start: resource_size_t,
}

#[repr(C)]
struct clk {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap_config {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_dmaengine_dai_dma_data {
    addr: resource_size_t,
    addr_width: c_int,
    maxburst: c_uint,
}

#[repr(C)]
struct loongson_i2s {
    reg_base: *mut c_void,
    regmap: *mut regmap,
    playback_dma_data: snd_dmaengine_dai_dma_data,
    capture_dma_data: snd_dmaengine_dai_dma_data,
    clk_rate: c_ulong,
    rev_id: c_int,
}

#[repr(C)]
struct snd_soc_component_driver {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_soc_dai_driver {
    _private: [u8; 0],
}

#[repr(C)]
struct snd_dmaengine_pcm_config {
    _private: [u8; 0],
}

#[repr(C)]
struct dev_pm_ops {
    _private: [u8; 0],
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
    data: *const c_void,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    pm: *const dev_pm_ops,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    driver: device_driver,
}

#[repr(C)]
struct loongson_i2s_plat_config {
    rev_id: c_int,
    i2s_dma_config: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

unsafe extern "C" {
    static loongson_i2s_regmap_config: regmap_config;
    static loongson_i2s_edma_component: snd_soc_component_driver;
    static loongson_i2s_dai: snd_soc_dai_driver;
    static loongson_dmaengine_pcm_config: snd_dmaengine_pcm_config;
    static loongson_i2s_pm: dev_pm_ops;

    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_platform_get_and_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
        res: *mut *mut resource,
    ) -> *mut c_void;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn readl(addr: *const c_void) -> c_uint;
    fn writel(value: c_uint, addr: *mut c_void);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn device_get_match_data(dev: *mut device) -> *const c_void;
    fn dev_err_probe(dev: *mut device, err: c_int, fmt: *const c_char, ...) -> c_int;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_clk_get_enabled(dev: *mut device, id: *const c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> c_ulong;
    fn dma_set_mask_and_coherent(dev: *mut device, mask: u64) -> c_int;
    fn dev_set_name(dev: *mut device, name: *const c_char, ...);
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn regmap_update_bits(
        map: *mut regmap,
        reg: c_uint,
        mask: c_uint,
        val: c_uint,
    ) -> c_int;
    fn fsleep(usecs: c_ulong);
    fn devm_snd_soc_register_component(
        dev: *mut device,
        cmpnt_drv: *const snd_soc_component_driver,
        dai_drv: *const snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn devm_snd_dmaengine_pcm_register(
        dev: *mut device,
        config: *const snd_dmaengine_pcm_config,
        flags: c_uint,
    ) -> c_int;
    fn pm_sleep_ptr(ptr: *const dev_pm_ops) -> *const dev_pm_ops;
    fn module_platform_driver(driver: *mut platform_driver);
}

const fn DMA_BIT_MASK(n: u32) -> u64 {
    if n == 64 {
        !0u64
    } else {
        (1u64 << n) - 1
    }
}

unsafe extern "C" fn loongson_i2s_apbdma_config(pdev: *mut platform_device) -> c_int {
    let mut val: c_uint;
    let regs: *mut c_void;

    regs = unsafe { devm_platform_ioremap_resource(pdev, 1) };
    if unsafe { IS_ERR(regs.cast_const()) } {
        return unsafe { PTR_ERR(regs.cast_const()) };
    }

    val = unsafe { readl(regs.cast_const()) };
    val |= (LOONGSON_DMA2_CONF << LOONGSON_I2S_TX_DMA_OFFSET) as c_uint;
    val |= (LOONGSON_DMA3_CONF << LOONGSON_I2S_RX_DMA_OFFSET) as c_uint;
    unsafe { writel(val, regs) };

    0
}

static ls2k0300_i2s_plat_config: loongson_i2s_plat_config = loongson_i2s_plat_config {
    rev_id: 1,
    i2s_dma_config: None,
};

static ls2k1000_i2s_plat_config: loongson_i2s_plat_config = loongson_i2s_plat_config {
    rev_id: 0,
    i2s_dma_config: Some(loongson_i2s_apbdma_config),
};

unsafe extern "C" fn loongson_i2s_plat_probe(pdev: *mut platform_device) -> c_int {
    let mut plat_config: *const loongson_i2s_plat_config;
    let dev: *mut device = unsafe { &mut (*pdev).dev };
    let i2s: *mut loongson_i2s;
    let mut res: *mut resource = ptr::null_mut();
    let i2s_clk: *mut clk;
    let mut ret: c_int;

    i2s = unsafe { devm_kzalloc(dev, size_of::<loongson_i2s>(), GFP_KERNEL).cast() };
    if i2s.is_null() {
        return -ENOMEM;
    }

    plat_config = unsafe { device_get_match_data(dev).cast() };
    if plat_config.is_null() {
        return -EINVAL;
    }

    if let Some(i2s_dma_config) = unsafe { (*plat_config).i2s_dma_config } {
        ret = unsafe { i2s_dma_config(pdev) };
        if ret != 0 {
            return ret;
        }
    }

    unsafe {
        (*i2s).reg_base = devm_platform_get_and_ioremap_resource(pdev, 0, &mut res);
    }
    if unsafe { IS_ERR((*i2s).reg_base.cast_const()) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR((*i2s).reg_base.cast_const()),
                b"devm_ioremap_resource failed\n\0".as_ptr().cast(),
            )
        };
    }

    unsafe {
        (*i2s).regmap =
            devm_regmap_init_mmio(dev, (*i2s).reg_base, &loongson_i2s_regmap_config);
    }
    if unsafe { IS_ERR((*i2s).regmap.cast_const().cast()) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR((*i2s).regmap.cast_const().cast()),
                b"devm_regmap_init_mmio failed\n\0".as_ptr().cast(),
            )
        };
    }

    unsafe {
        (*i2s).playback_dma_data.addr = (*res).start + LS_I2S_TX_DATA;
        (*i2s).playback_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*i2s).playback_dma_data.maxburst = 4;

        (*i2s).capture_dma_data.addr = (*res).start + LS_I2S_RX_DATA;
        (*i2s).capture_dma_data.addr_width = DMA_SLAVE_BUSWIDTH_4_BYTES;
        (*i2s).capture_dma_data.maxburst = 4;
    }

    i2s_clk = unsafe { devm_clk_get_enabled(dev, ptr::null()) };
    if unsafe { IS_ERR(i2s_clk.cast_const().cast()) } {
        return unsafe {
            dev_err_probe(
                dev,
                PTR_ERR(i2s_clk.cast_const().cast()),
                b"clock property invalid\n\0".as_ptr().cast(),
            )
        };
    }
    unsafe {
        (*i2s).clk_rate = clk_get_rate(i2s_clk);
        (*i2s).rev_id = (*plat_config).rev_id;
    }

    unsafe {
        dma_set_mask_and_coherent(dev, DMA_BIT_MASK(64));
        dev_set_name(dev, LS_I2S_DRVNAME);
        dev_set_drvdata(dev, i2s.cast());
    }

    if unsafe { (*i2s).rev_id == 1 } {
        unsafe {
            regmap_update_bits(
                (*i2s).regmap,
                LS_I2S_CTRL,
                I2S_CTRL_RESET,
                I2S_CTRL_RESET,
            );
            fsleep(200);
        }
    }

    ret = unsafe {
        devm_snd_soc_register_component(dev, &loongson_i2s_edma_component, &loongson_i2s_dai, 1)
    };
    if ret != 0 {
        return unsafe { dev_err_probe(dev, ret, b"failed to register DAI\n\0".as_ptr().cast()) };
    }

    unsafe {
        devm_snd_dmaengine_pcm_register(
            dev,
            &loongson_dmaengine_pcm_config,
            SND_DMAENGINE_PCM_FLAG_COMPAT,
        )
    }
}

static loongson_i2s_ids: [of_device_id; 3] = [
    of_device_id {
        compatible: b"loongson,ls2k0300-i2s\0".as_ptr().cast(),
        data: (&ls2k0300_i2s_plat_config as *const loongson_i2s_plat_config).cast(),
    },
    of_device_id {
        compatible: b"loongson,ls2k1000-i2s\0".as_ptr().cast(),
        data: (&ls2k1000_i2s_plat_config as *const loongson_i2s_plat_config).cast(),
    },
    of_device_id {
        compatible: ptr::null(),
        data: ptr::null(),
    },
];
// MODULE_DEVICE_TABLE(of, loongson_i2s_ids);

static mut loongson_i2s_driver: platform_driver = platform_driver {
    probe: Some(loongson_i2s_plat_probe),
    driver: device_driver {
        name: b"loongson-i2s-plat\0".as_ptr().cast(),
        pm: unsafe { pm_sleep_ptr(&loongson_i2s_pm) },
        of_match_table: loongson_i2s_ids.as_ptr(),
    },
};

unsafe fn __register_loongson_i2s_driver() {
    unsafe { module_platform_driver(&raw mut loongson_i2s_driver) };
}

// module_platform_driver(loongson_i2s_driver);
// MODULE_DESCRIPTION("Loongson I2S Master Mode ASoC Driver");
// MODULE_AUTHOR("Loongson Technology Corporation Limited");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
