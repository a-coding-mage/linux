// SPDX-License-Identifier: GPL-2.0-only
/*
 * tegra20_das.c - Tegra20 DAS driver
 *
 * Author: Stephen Warren <swarren@nvidia.com>
 * Copyright (C) 2010 - NVIDIA, Inc.
 */

// Dependencies from:
// <linux/device.h>
// <linux/io.h>
// <linux/module.h>
// <linux/platform_device.h>
// <linux/regmap.h>
// <linux/slab.h>
// <sound/soc.h>

const DRV_NAME: &[u8] = b"tegra20-das\0";

/* Register TEGRA20_DAS_DAP_CTRL_SEL */
const TEGRA20_DAS_DAP_CTRL_SEL: u32 = 0x00;
const TEGRA20_DAS_DAP_CTRL_SEL_COUNT: u32 = 5;
const TEGRA20_DAS_DAP_CTRL_SEL_STRIDE: u32 = 4;
const TEGRA20_DAS_DAP_CTRL_SEL_DAP_MS_SEL_P: u32 = 31;
const TEGRA20_DAS_DAP_CTRL_SEL_DAP_MS_SEL_S: u32 = 1;
const TEGRA20_DAS_DAP_CTRL_SEL_DAP_SDATA1_TX_RX_P: u32 = 30;
const TEGRA20_DAS_DAP_CTRL_SEL_DAP_SDATA1_TX_RX_S: u32 = 1;
const TEGRA20_DAS_DAP_CTRL_SEL_DAP_SDATA2_TX_RX_P: u32 = 29;
const TEGRA20_DAS_DAP_CTRL_SEL_DAP_SDATA2_TX_RX_S: u32 = 1;
const TEGRA20_DAS_DAP_CTRL_SEL_DAP_CTRL_SEL_P: u32 = 0;
const TEGRA20_DAS_DAP_CTRL_SEL_DAP_CTRL_SEL_S: u32 = 5;

/* Values for field TEGRA20_DAS_DAP_CTRL_SEL_DAP_CTRL_SEL */
const TEGRA20_DAS_DAP_SEL_DAC1: i32 = 0;
const TEGRA20_DAS_DAP_SEL_DAC2: i32 = 1;
const TEGRA20_DAS_DAP_SEL_DAC3: i32 = 2;
const TEGRA20_DAS_DAP_SEL_DAP1: i32 = 16;
const TEGRA20_DAS_DAP_SEL_DAP2: i32 = 17;
const TEGRA20_DAS_DAP_SEL_DAP3: i32 = 18;
const TEGRA20_DAS_DAP_SEL_DAP4: i32 = 19;
const TEGRA20_DAS_DAP_SEL_DAP5: i32 = 20;

/* Register TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL */
const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL: u32 = 0x40;
const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_COUNT: u32 = 3;
const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_STRIDE: u32 = 4;
const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA2_SEL_P: u32 = 28;
const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA2_SEL_S: u32 = 4;
const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA1_SEL_P: u32 = 24;
const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA1_SEL_S: u32 = 4;
const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_CLK_SEL_P: u32 = 0;
const TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_CLK_SEL_S: u32 = 4;

/*
 * Values for:
 * TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA2_SEL
 * TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA1_SEL
 * TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_CLK_SEL
 */
const TEGRA20_DAS_DAC_SEL_DAP1: i32 = 0;
const TEGRA20_DAS_DAC_SEL_DAP2: i32 = 1;
const TEGRA20_DAS_DAC_SEL_DAP3: i32 = 2;
const TEGRA20_DAS_DAC_SEL_DAP4: i32 = 3;
const TEGRA20_DAS_DAC_SEL_DAP5: i32 = 4;

/*
 * Names/IDs of the DACs/DAPs.
 */

const TEGRA20_DAS_DAP_ID_1: i32 = 0;
const TEGRA20_DAS_DAP_ID_2: i32 = 1;
const TEGRA20_DAS_DAP_ID_3: i32 = 2;
const TEGRA20_DAS_DAP_ID_4: i32 = 3;
const TEGRA20_DAS_DAP_ID_5: i32 = 4;

const TEGRA20_DAS_DAC_ID_1: i32 = 0;
const TEGRA20_DAS_DAC_ID_2: i32 = 1;
const TEGRA20_DAS_DAC_ID_3: i32 = 2;

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
}

#[repr(C)]
struct of_device_id {
    compatible: *const i8,
}

type WriteableRegFn = unsafe extern "C" fn(*mut device, u32) -> bool;

#[repr(C)]
struct regmap_config {
    reg_bits: u32,
    reg_stride: u32,
    val_bits: u32,
    max_register: u32,
    writeable_reg: Option<WriteableRegFn>,
    readable_reg: Option<WriteableRegFn>,
    cache_type: u32,
}

#[repr(C)]
struct driver {
    name: *const i8,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    driver: driver,
}

#[repr(C)]
struct tegra20_das {
    regmap: *mut regmap,
}

const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;
const REGCACHE_FLAT: u32 = 0;

unsafe extern "C" {
    fn regmap_write(map: *mut regmap, reg: u32, val: u32) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, gfp: u32) -> *mut core::ffi::c_void;
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: u32,
    ) -> *mut core::ffi::c_void;
    fn IS_ERR(ptr: *const core::ffi::c_void) -> bool;
    fn PTR_ERR(ptr: *const core::ffi::c_void) -> i32;
    fn devm_regmap_init_mmio(
        dev: *mut device,
        regs: *mut core::ffi::c_void,
        config: *const regmap_config,
    ) -> *mut regmap;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const i8, ...) -> i32;
}

/*
 * Terminology:
 * DAS: Digital audio switch (HW module controlled by this driver)
 * DAP: Digital audio port (port/pins on Tegra device)
 * DAC: Digital audio controller (e.g. I2S or AC97 controller elsewhere)
 *
 * The Tegra DAS is a mux/cross-bar which can connect each DAP to a specific
 * DAC, or another DAP. When DAPs are connected, one must be the master and
 * one the slave. Each DAC allows selection of a specific DAP for input, to
 * cater for the case where N DAPs are connected to 1 DAC for broadcast
 * output.
 *
 * This driver is dumb; no attempt is made to ensure that a valid routing
 * configuration is programmed.
 */

#[inline]
unsafe fn tegra20_das_write(das: *mut tegra20_das, reg: u32, val: u32) {
    unsafe {
        regmap_write((*das).regmap, reg, val);
    }
}

unsafe fn tegra20_das_connect_dap_to_dac(das: *mut tegra20_das, dap: i32, dac: i32) {
    let addr: u32;
    let reg: u32;

    addr = TEGRA20_DAS_DAP_CTRL_SEL.wrapping_add(
        (dap as u32).wrapping_mul(TEGRA20_DAS_DAP_CTRL_SEL_STRIDE),
    );
    reg = (dac as u32) << TEGRA20_DAS_DAP_CTRL_SEL_DAP_CTRL_SEL_P;

    unsafe {
        tegra20_das_write(das, addr, reg);
    }
}

unsafe fn tegra20_das_connect_dac_to_dap(das: *mut tegra20_das, dac: i32, dap: i32) {
    let addr: u32;
    let reg: u32;

    addr = TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL.wrapping_add(
        (dac as u32).wrapping_mul(TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_STRIDE),
    );
    reg = ((dap as u32) << TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_CLK_SEL_P)
        | ((dap as u32) << TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA1_SEL_P)
        | ((dap as u32) << TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_DAC_SDATA2_SEL_P);

    unsafe {
        tegra20_das_write(das, addr, reg);
    }
}

const fn LAST_REG(base: u32, stride: u32, count: u32) -> u32 {
    base + (stride * (count - 1))
}

unsafe extern "C" fn tegra20_das_wr_rd_reg(_dev: *mut device, reg: u32) -> bool {
    if reg <= LAST_REG(
        TEGRA20_DAS_DAP_CTRL_SEL,
        TEGRA20_DAS_DAP_CTRL_SEL_STRIDE,
        TEGRA20_DAS_DAP_CTRL_SEL_COUNT,
    ) {
        return true;
    }
    if (reg >= TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL)
        && (reg
            <= LAST_REG(
                TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL,
                TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_STRIDE,
                TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_COUNT,
            ))
    {
        return true;
    }

    false
}

static tegra20_das_regmap_config: regmap_config = regmap_config {
    reg_bits: 32,
    reg_stride: 4,
    val_bits: 32,
    max_register: LAST_REG(
        TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL,
        TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_STRIDE,
        TEGRA20_DAS_DAC_INPUT_DATA_CLK_SEL_COUNT,
    ),
    writeable_reg: Some(tegra20_das_wr_rd_reg),
    readable_reg: Some(tegra20_das_wr_rd_reg),
    cache_type: REGCACHE_FLAT,
};

unsafe extern "C" fn tegra20_das_probe(pdev: *mut platform_device) -> i32 {
    let regs: *mut core::ffi::c_void;
    let das: *mut tegra20_das;

    unsafe {
        das = devm_kzalloc(
            &mut (*pdev).dev,
            core::mem::size_of::<tegra20_das>(),
            GFP_KERNEL,
        ) as *mut tegra20_das;
        if das.is_null() {
            return -ENOMEM;
        }

        regs = devm_platform_ioremap_resource(pdev, 0);
        if IS_ERR(regs as *const core::ffi::c_void) {
            return PTR_ERR(regs as *const core::ffi::c_void);
        }

        (*das).regmap = devm_regmap_init_mmio(
            &mut (*pdev).dev,
            regs,
            &tegra20_das_regmap_config,
        );
        if IS_ERR((*das).regmap as *const core::ffi::c_void) {
            return dev_err_probe(
                &mut (*pdev).dev,
                PTR_ERR((*das).regmap as *const core::ffi::c_void),
                c"regmap init failed\n".as_ptr(),
            );
        }

        tegra20_das_connect_dap_to_dac(
            das,
            TEGRA20_DAS_DAP_ID_1,
            TEGRA20_DAS_DAP_SEL_DAC1,
        );
        tegra20_das_connect_dac_to_dap(
            das,
            TEGRA20_DAS_DAC_ID_1,
            TEGRA20_DAS_DAC_SEL_DAP1,
        );
        tegra20_das_connect_dap_to_dac(
            das,
            TEGRA20_DAS_DAP_ID_3,
            TEGRA20_DAS_DAP_SEL_DAC3,
        );
        tegra20_das_connect_dac_to_dap(
            das,
            TEGRA20_DAS_DAC_ID_3,
            TEGRA20_DAS_DAC_SEL_DAP3,
        );
    }

    0
}

static tegra20_das_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: c"nvidia,tegra20-das".as_ptr(),
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, tegra20_das_of_match);

static mut tegra20_das_driver: platform_driver = platform_driver {
    probe: Some(tegra20_das_probe),
    driver: driver {
        name: DRV_NAME.as_ptr() as *const i8,
        of_match_table: tegra20_das_of_match.as_ptr(),
    },
};

// module_platform_driver(tegra20_das_driver);

// MODULE_AUTHOR("Stephen Warren <swarren@nvidia.com>");
// MODULE_DESCRIPTION("Tegra20 DAS driver");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:" DRV_NAME);

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
