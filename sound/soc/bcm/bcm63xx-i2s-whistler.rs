// SPDX-License-Identifier: GPL-2.0-or-later
// linux/sound/bcm/bcm63xx-i2s-whistler.c
// BCM63xx whistler i2s driver
// Copyright (c) 2020 Broadcom Corporation
// Author: Kevin-Ke Li <kevin-ke.li@broadcom.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::{addr_of, addr_of_mut, null};

const DRV_NAME: *const c_char = b"brcm-i2s\0".as_ptr() as *const c_char;

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
pub struct snd_pcm_substream {
    pub stream: c_int,
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
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct bcm_i2s_priv {
    pub dev: *mut device,
    pub i2s_clk: *mut clk,
    pub regmap_i2s: *mut regmap,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: c_int,
    pub reg_stride: c_int,
    pub val_bits: c_int,
    pub max_register: c_uint,
    pub writeable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub readable_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub volatile_reg: Option<unsafe extern "C" fn(*mut device, c_uint) -> bool>,
    pub cache_type: c_int,
}

#[repr(C)]
pub struct snd_soc_dai_ops {
    pub startup: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai) -> c_int>,
    pub shutdown: Option<unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_soc_dai)>,
    pub hw_params: Option<
        unsafe extern "C" fn(*mut snd_pcm_substream, *mut snd_pcm_hw_params, *mut snd_soc_dai) -> c_int,
    >,
}

#[repr(C)]
pub struct snd_soc_pcm_stream {
    pub channels_min: c_uint,
    pub channels_max: c_uint,
    pub rates: c_uint,
    pub formats: c_ulong,
}

#[repr(C)]
pub struct snd_soc_dai_driver {
    pub name: *const c_char,
    pub playback: snd_soc_pcm_stream,
    pub capture: snd_soc_pcm_stream,
    pub ops: *const snd_soc_dai_ops,
    pub symmetric_rate: c_uint,
    pub symmetric_channels: c_uint,
}

#[repr(C)]
pub struct snd_soc_component_driver {
    pub name: *const c_char,
    pub legacy_dai_naming: c_uint,
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const c_char,
}

#[repr(C)]
pub struct device_driver {
    pub name: *const c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub driver: device_driver,
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut platform_device)>,
}

extern "C" {
    static I2S_TX_CFG: c_uint;
    static I2S_TX_DESC_IFF_LEN: c_uint;
    static I2S_TX_CFG_2: c_uint;
    static I2S_RX_DESC_IFF_LEN: c_uint;
    static I2S_RX_CFG_2: c_uint;
    static I2S_REG_MAX: c_uint;
    static I2S_TX_IRQ_CTL: c_uint;
    static I2S_TX_DESC_IFF_ADDR: c_uint;
    static I2S_TX_DESC_OFF_ADDR: c_uint;
    static I2S_TX_DESC_OFF_LEN: c_uint;
    static I2S_RX_CFG: c_uint;
    static I2S_RX_IRQ_CTL: c_uint;
    static I2S_RX_DESC_OFF_ADDR: c_uint;
    static I2S_RX_DESC_OFF_LEN: c_uint;
    static I2S_RX_DESC_IFF_ADDR: c_uint;
    static I2S_TX_OUT_R: c_uint;
    static I2S_TX_DATA_ALIGNMENT: c_uint;
    static I2S_TX_DATA_ENABLE: c_uint;
    static I2S_TX_CLOCK_ENABLE: c_uint;
    static I2S_TX_IRQ_IFF_THLD: c_uint;
    static I2S_TX_IRQ_OFF_THLD: c_uint;
    static I2S_RX_SLAVE_MODE_MASK: c_uint;
    static I2S_TX_SLAVE_MODE_MASK: c_uint;
    static I2S_TX_MASTER_MODE: c_uint;
    static I2S_TX_SLAVE_MODE: c_uint;
    static I2S_RX_IN_R: c_uint;
    static I2S_RX_DATA_ALIGNMENT: c_uint;
    static I2S_RX_CLOCK_ENABLE: c_uint;
    static I2S_RX_IRQ_IFF_THLD: c_uint;
    static I2S_RX_IRQ_OFF_THLD: c_uint;
    static I2S_RX_SLAVE_MODE: c_uint;
    static I2S_RX_ENABLE_MASK: c_uint;
    static I2S_RX_MASTER_MODE: c_uint;
    static I2S_TX_ENABLE_MASK: c_uint;
    static I2S_MISC_CFG: c_uint;
    static I2S_PAD_LVL_LOOP_DIS_MASK: c_uint;
    static I2S_PAD_LVL_LOOP_DIS_ENABLE: c_uint;
    static REGCACHE_FLAT: c_int;
    static SNDRV_PCM_STREAM_PLAYBACK: c_int;
    static SNDRV_PCM_RATE_8000_192000: c_uint;
    static SNDRV_PCM_FMTBIT_S32_LE: c_ulong;
    static GFP_KERNEL: c_uint;
    static ENOMEM: c_int;

    fn clk_set_rate(clk: *mut clk, rate: c_ulong) -> c_int;
    fn params_rate(params: *mut snd_pcm_hw_params) -> c_ulong;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn snd_soc_dai_get_drvdata(dai: *mut snd_soc_dai) -> *mut c_void;
    fn regmap_update_bits(map: *mut regmap, reg: c_uint, mask: c_uint, val: c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: c_uint, val: c_uint) -> c_int;
    fn regmap_read(map: *mut regmap, reg: c_uint, val: *mut c_uint) -> c_int;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_long;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: c_uint) -> *mut c_void;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn devm_snd_soc_register_component(
        dev: *mut device,
        component_driver: *const snd_soc_component_driver,
        dai_drv: *mut snd_soc_dai_driver,
        num_dai: c_int,
    ) -> c_int;
    fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
    fn bcm63xx_soc_platform_probe(pdev: *mut platform_device, i2s_priv: *mut bcm_i2s_priv) -> c_int;
    fn bcm63xx_soc_platform_remove(pdev: *mut platform_device);
    fn module_platform_driver(driver: *mut platform_driver);
    fn MODULE_AUTHOR(author: *const c_char);
    fn MODULE_DESCRIPTION(description: *const c_char);
    fn MODULE_LICENSE(license: *const c_char);
}

unsafe extern "C" fn brcm_i2s_wr_reg(_dev: *mut device, reg: c_uint) -> bool {
    if reg >= I2S_TX_CFG && reg <= I2S_TX_DESC_IFF_LEN {
        return true;
    }
    if reg >= I2S_TX_CFG_2 && reg <= I2S_RX_DESC_IFF_LEN {
        return true;
    }
    if reg >= I2S_RX_CFG_2 && reg <= I2S_REG_MAX {
        return true;
    }
    false
}

unsafe extern "C" fn brcm_i2s_rd_reg(_dev: *mut device, reg: c_uint) -> bool {
    if reg >= I2S_TX_CFG && reg <= I2S_REG_MAX {
        return true;
    }
    false
}

unsafe extern "C" fn brcm_i2s_volatile_reg(_dev: *mut device, reg: c_uint) -> bool {
    if reg == I2S_TX_CFG
        || reg == I2S_TX_IRQ_CTL
        || reg == I2S_TX_DESC_IFF_ADDR
        || reg == I2S_TX_DESC_IFF_LEN
        || reg == I2S_TX_DESC_OFF_ADDR
        || reg == I2S_TX_DESC_OFF_LEN
        || reg == I2S_TX_CFG_2
        || reg == I2S_RX_CFG
        || reg == I2S_RX_IRQ_CTL
        || reg == I2S_RX_DESC_OFF_ADDR
        || reg == I2S_RX_DESC_OFF_LEN
        || reg == I2S_RX_DESC_IFF_LEN
        || reg == I2S_RX_DESC_IFF_ADDR
        || reg == I2S_RX_CFG_2
    {
        return true;
    }
    false
}

static brcm_i2s_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: unsafe { I2S_REG_MAX },
    writeable_reg: Some(brcm_i2s_wr_reg),
    readable_reg: Some(brcm_i2s_rd_reg),
    volatile_reg: Some(brcm_i2s_volatile_reg),
    cache_type: unsafe { REGCACHE_FLAT },
};

unsafe extern "C" fn bcm63xx_i2s_hw_params(
    _substream: *mut snd_pcm_substream,
    params: *mut snd_pcm_hw_params,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mut ret: c_int = 0;
    let i2s_priv: *mut bcm_i2s_priv = snd_soc_dai_get_drvdata(dai) as *mut bcm_i2s_priv;

    ret = clk_set_rate((*i2s_priv).i2s_clk, params_rate(params));
    if ret < 0 {
        dev_err(
            (*i2s_priv).dev,
            b"Can't set sample rate, err: %d\n\0".as_ptr() as *const c_char,
            ret,
        );
    }

    ret
}

unsafe extern "C" fn bcm63xx_i2s_startup(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) -> c_int {
    let mut slavemode: c_uint = 0;
    let i2s_priv: *mut bcm_i2s_priv = snd_soc_dai_get_drvdata(dai) as *mut bcm_i2s_priv;
    let regmap_i2s: *mut regmap = (*i2s_priv).regmap_i2s;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits(
            regmap_i2s,
            I2S_TX_CFG,
            I2S_TX_OUT_R | I2S_TX_DATA_ALIGNMENT | I2S_TX_DATA_ENABLE | I2S_TX_CLOCK_ENABLE,
            I2S_TX_OUT_R | I2S_TX_DATA_ALIGNMENT | I2S_TX_DATA_ENABLE | I2S_TX_CLOCK_ENABLE,
        );
        regmap_write(regmap_i2s, I2S_TX_IRQ_CTL, 0);
        regmap_write(regmap_i2s, I2S_TX_IRQ_IFF_THLD, 0);
        regmap_write(regmap_i2s, I2S_TX_IRQ_OFF_THLD, 1);

        /* TX and RX block each have an independent bit to indicate
         * if it is generating the clock for the I2S bus. The bus
         * clocks need to be generated from either the TX or RX block,
         * but not both
         */
        regmap_read(regmap_i2s, I2S_RX_CFG_2, &mut slavemode);
        if (slavemode & I2S_RX_SLAVE_MODE_MASK) != 0 {
            regmap_update_bits(
                regmap_i2s,
                I2S_TX_CFG_2,
                I2S_TX_SLAVE_MODE_MASK,
                I2S_TX_MASTER_MODE,
            );
        } else {
            regmap_update_bits(
                regmap_i2s,
                I2S_TX_CFG_2,
                I2S_TX_SLAVE_MODE_MASK,
                I2S_TX_SLAVE_MODE,
            );
        }
    } else {
        regmap_update_bits(
            regmap_i2s,
            I2S_RX_CFG,
            I2S_RX_IN_R | I2S_RX_DATA_ALIGNMENT | I2S_RX_CLOCK_ENABLE,
            I2S_RX_IN_R | I2S_RX_DATA_ALIGNMENT | I2S_RX_CLOCK_ENABLE,
        );
        regmap_write(regmap_i2s, I2S_RX_IRQ_CTL, 0);
        regmap_write(regmap_i2s, I2S_RX_IRQ_IFF_THLD, 0);
        regmap_write(regmap_i2s, I2S_RX_IRQ_OFF_THLD, 1);

        regmap_read(regmap_i2s, I2S_TX_CFG_2, &mut slavemode);
        if (slavemode & I2S_TX_SLAVE_MODE_MASK) != 0 {
            regmap_update_bits(regmap_i2s, I2S_RX_CFG_2, I2S_RX_SLAVE_MODE_MASK, 0);
        } else {
            regmap_update_bits(
                regmap_i2s,
                I2S_RX_CFG_2,
                I2S_RX_SLAVE_MODE_MASK,
                I2S_RX_SLAVE_MODE,
            );
        }
    }
    0
}

unsafe extern "C" fn bcm63xx_i2s_shutdown(
    substream: *mut snd_pcm_substream,
    dai: *mut snd_soc_dai,
) {
    let mut enabled: c_uint = 0;
    let mut slavemode: c_uint = 0;
    let i2s_priv: *mut bcm_i2s_priv = snd_soc_dai_get_drvdata(dai) as *mut bcm_i2s_priv;
    let regmap_i2s: *mut regmap = (*i2s_priv).regmap_i2s;

    if (*substream).stream == SNDRV_PCM_STREAM_PLAYBACK {
        regmap_update_bits(
            regmap_i2s,
            I2S_TX_CFG,
            I2S_TX_OUT_R | I2S_TX_DATA_ALIGNMENT | I2S_TX_DATA_ENABLE | I2S_TX_CLOCK_ENABLE,
            0,
        );
        regmap_write(regmap_i2s, I2S_TX_IRQ_CTL, 1);
        regmap_write(regmap_i2s, I2S_TX_IRQ_IFF_THLD, 4);
        regmap_write(regmap_i2s, I2S_TX_IRQ_OFF_THLD, 4);

        regmap_read(regmap_i2s, I2S_TX_CFG_2, &mut slavemode);
        slavemode = slavemode & I2S_TX_SLAVE_MODE_MASK;
        if slavemode == 0 {
            regmap_read(regmap_i2s, I2S_RX_CFG, &mut enabled);
            enabled = enabled & I2S_RX_ENABLE_MASK;
            if enabled != 0 {
                regmap_update_bits(
                    regmap_i2s,
                    I2S_RX_CFG_2,
                    I2S_RX_SLAVE_MODE_MASK,
                    I2S_RX_MASTER_MODE,
                );
            }
        }
        regmap_update_bits(
            regmap_i2s,
            I2S_TX_CFG_2,
            I2S_TX_SLAVE_MODE_MASK,
            I2S_TX_SLAVE_MODE,
        );
    } else {
        regmap_update_bits(
            regmap_i2s,
            I2S_RX_CFG,
            I2S_RX_IN_R | I2S_RX_DATA_ALIGNMENT | I2S_RX_CLOCK_ENABLE,
            0,
        );
        regmap_write(regmap_i2s, I2S_RX_IRQ_CTL, 1);
        regmap_write(regmap_i2s, I2S_RX_IRQ_IFF_THLD, 4);
        regmap_write(regmap_i2s, I2S_RX_IRQ_OFF_THLD, 4);

        regmap_read(regmap_i2s, I2S_RX_CFG_2, &mut slavemode);
        slavemode = slavemode & I2S_RX_SLAVE_MODE_MASK;
        if slavemode == 0 {
            regmap_read(regmap_i2s, I2S_TX_CFG, &mut enabled);
            enabled = enabled & I2S_TX_ENABLE_MASK;
            if enabled != 0 {
                regmap_update_bits(
                    regmap_i2s,
                    I2S_TX_CFG_2,
                    I2S_TX_SLAVE_MODE_MASK,
                    I2S_TX_MASTER_MODE,
                );
            }
        }

        regmap_update_bits(
            regmap_i2s,
            I2S_RX_CFG_2,
            I2S_RX_SLAVE_MODE_MASK,
            I2S_RX_SLAVE_MODE,
        );
    }
}

static bcm63xx_i2s_dai_ops: snd_soc_dai_ops = snd_soc_dai_ops {
    startup: Some(bcm63xx_i2s_startup),
    shutdown: Some(bcm63xx_i2s_shutdown),
    hw_params: Some(bcm63xx_i2s_hw_params),
};

static mut bcm63xx_i2s_dai: snd_soc_dai_driver = snd_soc_dai_driver {
    name: DRV_NAME,
    playback: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_192000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S32_LE },
    },
    capture: snd_soc_pcm_stream {
        channels_min: 2,
        channels_max: 2,
        rates: unsafe { SNDRV_PCM_RATE_8000_192000 },
        formats: unsafe { SNDRV_PCM_FMTBIT_S32_LE },
    },
    ops: addr_of!(bcm63xx_i2s_dai_ops),
    symmetric_rate: 1,
    symmetric_channels: 1,
};

static bcm63xx_i2s_component: snd_soc_component_driver = snd_soc_component_driver {
    name: b"bcm63xx\0".as_ptr() as *const c_char,
    legacy_dai_naming: 1,
};

unsafe extern "C" fn bcm63xx_i2s_dev_probe(pdev: *mut platform_device) -> c_int {
    let mut ret: c_int = 0;
    let regs: *mut c_void;
    let i2s_priv: *mut bcm_i2s_priv;
    let regmap_i2s: *mut regmap;
    let i2s_clk: *mut clk;

    i2s_priv = devm_kzalloc(
        &mut (*pdev).dev,
        size_of::<bcm_i2s_priv>(),
        GFP_KERNEL,
    ) as *mut bcm_i2s_priv;
    if i2s_priv.is_null() {
        return -ENOMEM;
    }

    i2s_clk = devm_clk_get(&mut (*pdev).dev, b"i2sclk\0".as_ptr() as *const c_char);
    if IS_ERR(i2s_clk as *const c_void) {
        dev_err(
            &mut (*pdev).dev,
            b"%s: cannot get a brcm clock: %ld\n\0".as_ptr() as *const c_char,
            b"bcm63xx_i2s_dev_probe\0".as_ptr() as *const c_char,
            PTR_ERR(i2s_clk as *const c_void),
        );
        return PTR_ERR(i2s_clk as *const c_void) as c_int;
    }

    regs = devm_platform_ioremap_resource(pdev, 0);
    if IS_ERR(regs as *const c_void) {
        ret = PTR_ERR(regs as *const c_void) as c_int;
        return ret;
    }

    regmap_i2s = devm_regmap_init_mmio(&mut (*pdev).dev, regs, addr_of!(brcm_i2s_regmap_config));
    if IS_ERR(regmap_i2s as *const c_void) {
        return PTR_ERR(regmap_i2s as *const c_void) as c_int;
    }

    regmap_update_bits(
        regmap_i2s,
        I2S_MISC_CFG,
        I2S_PAD_LVL_LOOP_DIS_MASK,
        I2S_PAD_LVL_LOOP_DIS_ENABLE,
    );

    ret = devm_snd_soc_register_component(
        &mut (*pdev).dev,
        addr_of!(bcm63xx_i2s_component),
        addr_of_mut!(bcm63xx_i2s_dai),
        1,
    );
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"failed to register the dai\n\0".as_ptr() as *const c_char,
        );
        return ret;
    }

    (*i2s_priv).dev = &mut (*pdev).dev;
    (*i2s_priv).i2s_clk = i2s_clk;
    (*i2s_priv).regmap_i2s = regmap_i2s;
    dev_set_drvdata(&mut (*pdev).dev, i2s_priv as *mut c_void);

    ret = bcm63xx_soc_platform_probe(pdev, i2s_priv);
    if ret != 0 {
        dev_err(
            &mut (*pdev).dev,
            b"failed to register the pcm\n\0".as_ptr() as *const c_char,
        );
    }

    ret
}

unsafe extern "C" fn bcm63xx_i2s_dev_remove(pdev: *mut platform_device) {
    bcm63xx_soc_platform_remove(pdev);
}

// Original C guarded this table with CONFIG_OF.
static snd_soc_bcm_audio_match: [of_device_id; 2] = [
    of_device_id {
        compatible: b"brcm,bcm63xx-i2s\0".as_ptr() as *const c_char,
    },
    of_device_id { compatible: null() },
];

static mut bcm63xx_i2s_driver: platform_driver = platform_driver {
    driver: device_driver {
        name: DRV_NAME,
        of_match_table: snd_soc_bcm_audio_match.as_ptr(),
    },
    probe: Some(bcm63xx_i2s_dev_probe),
    remove: Some(bcm63xx_i2s_dev_remove),
};

unsafe fn __module_init() {
    module_platform_driver(addr_of_mut!(bcm63xx_i2s_driver));

    MODULE_AUTHOR(b"Kevin,Li <kevin-ke.li@broadcom.com>\0".as_ptr() as *const c_char);
    MODULE_DESCRIPTION(b"Broadcom DSL XPON ASOC I2S Interface\0".as_ptr() as *const c_char);
    MODULE_LICENSE(b"GPL v2\0".as_ptr() as *const c_char);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
