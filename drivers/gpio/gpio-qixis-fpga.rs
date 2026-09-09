// SPDX-License-Identifier: GPL-2.0-only
/*
 * Layerscape GPIO QIXIS FPGA driver
 *
 * Copyright 2025 NXP
 */

// Translated from the Linux kernel implementation. Kernel-provided types,
// helpers, macros, and registration interfaces are external dependencies.

#[repr(C)]
pub struct qixis_cpld_gpio_config {
    pub output_lines: u64,
}

static lx2160ardb_sfp_cfg: qixis_cpld_gpio_config = qixis_cpld_gpio_config {
    output_lines: 1u64 << 0,
};

static ls1046aqds_stat_pres2_cfg: qixis_cpld_gpio_config = qixis_cpld_gpio_config {
    output_lines: 0x0,
};

// struct regmap_config regmap_config_8r_8v = { .reg_bits = 8, .val_bits = 8 };
extern "C" {
    static regmap_config_8r_8v: regmap_config;
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub val_bits: u32,
}

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
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
pub struct gpio_regmap_config {
    pub reg_dat_base: usize,
    pub reg_set_base: usize,
    pub drvdata: *mut core::ffi::c_void,
    pub regmap: *mut regmap,
    pub parent: *mut device,
    pub ngpio_per_reg: u32,
    pub ngpio: u32,
    pub fixed_direction_output: *mut u8,
}

extern "C" {
    fn device_get_match_data(dev: *const device) -> *const qixis_cpld_gpio_config;
    fn device_property_read_u32(dev: *const device, name: *const core::ffi::c_char, value: *mut u32) -> i32;
    fn dev_get_regmap(parent: *mut device, name: *const core::ffi::c_char) -> *mut regmap;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn devm_regmap_init_mmio(dev: *mut device, reg: *mut core::ffi::c_void, config: *const regmap_config) -> *mut regmap;
    fn devm_gpio_regmap_register(dev: *mut device, config: *mut gpio_regmap_config) -> *mut core::ffi::c_void;
    fn ptr_err_or_zero(ptr: *mut core::ffi::c_void) -> i32;
    fn ptr_err(ptr: *mut core::ffi::c_void) -> i32;
    fn gpio_regmap_addr(base: u32) -> usize;
}

unsafe fn qixis_cpld_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut fixed_direction_output = [0u8; 1];
    let cfg: *const qixis_cpld_gpio_config;
    let mut config: gpio_regmap_config = core::mem::zeroed();
    let mut regmap: *mut regmap;
    let reg: *mut core::ffi::c_void;
    let mut base: u32 = 0;
    let ret: i32;
    let dev = pdev as *mut device;

    // if (!pdev->dev.parent)
    //     return -ENODEV;
    if dev.is_null() {
        return -19;
    }

    cfg = device_get_match_data(dev);

    ret = device_property_read_u32(dev, b"reg\0".as_ptr() as *const core::ffi::c_char, &mut base);
    if ret != 0 {
        return ret;
    }

    regmap = dev_get_regmap(dev, core::ptr::null());
    if regmap.is_null() {
        /* In case there is no regmap configured by the parent device,
         * create our own from the MMIO space.
         */
        reg = devm_platform_ioremap_resource(pdev, 0);
        if reg.is_null() {
            return ptr_err(reg);
        }

        regmap = devm_regmap_init_mmio(dev, reg, &regmap_config_8r_8v);
        if regmap.is_null() {
            return ptr_err(regmap as *mut core::ffi::c_void);
        }

        /* In this case, the offset of our register is 0 inside the
         * regmap area that we just created.
         */
        base = 0;
    }
    config.reg_dat_base = gpio_regmap_addr(base);
    config.reg_set_base = gpio_regmap_addr(base);

    config.drvdata = cfg as *mut core::ffi::c_void;
    config.regmap = regmap;
    config.parent = dev;
    config.ngpio_per_reg = 8;
    config.ngpio = 8;

    fixed_direction_output[0] = ( (*cfg).output_lines & 0xff ) as u8;
    config.fixed_direction_output = fixed_direction_output.as_mut_ptr();

    ptr_err_or_zero(devm_gpio_regmap_register(dev, &mut config))
}

#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

static qixis_cpld_gpio_of_match: [of_device_id; 3] = [
    of_device_id {
        compatible: b"fsl,lx2160ardb-fpga-gpio-sfp\0".as_ptr() as *const core::ffi::c_char,
        data: &lx2160ardb_sfp_cfg as *const _ as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: b"fsl,ls1046aqds-fpga-gpio-stat-pres2\0".as_ptr() as *const core::ffi::c_char,
        data: &ls1046aqds_stat_pres2_cfg as *const _ as *const core::ffi::c_void,
    },
    of_device_id {
        compatible: core::ptr::null(),
        data: core::ptr::null(),
    },
];

// MODULE_DEVICE_TABLE(of, qixis_cpld_gpio_of_match);

#[repr(C)]
pub struct device_driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe fn(*mut platform_device) -> i32>,
    pub driver: device_driver,
}

static mut qixis_cpld_gpio_driver: platform_driver = platform_driver {
    probe: Some(qixis_cpld_gpio_probe),
    driver: device_driver {
        name: b"gpio-qixis-cpld\0".as_ptr() as *const core::ffi::c_char,
        of_match_table: qixis_cpld_gpio_of_match.as_ptr(),
    },
};

// module_platform_driver(qixis_cpld_gpio_driver);
// MODULE_LICENSE("GPL");
// MODULE_AUTHOR("Ioana Ciornei <ioana.ciornei@nxp.com>");
// MODULE_DESCRIPTION("Layerscape GPIO QIXIS FPGA driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
