// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO driver for the Diamond Systems GPIO-MM
 * Copyright (C) 2016 William Breathitt Gray
 *
 * This driver supports the following Diamond Systems devices: GPIO-MM and
 * GPIO-MM-12.
 */

// C dependencies supplied by the surrounding kernel translation.
// #include <linux/device.h>
// #include <linux/errno.h>
// #include <linux/ioport.h>
// #include <linux/isa.h>
// #include <linux/kernel.h>
// #include <linux/module.h>
// #include <linux/moduleparam.h>
// #include <linux/regmap.h>
// #include <linux/types.h>
// #include "gpio-i8255.h"

// MODULE_IMPORT_NS("I8255");

const GPIOMM_EXTENT: usize = 8;
// const MAX_NUM_GPIOMM: usize = max_num_isa_dev(GPIOMM_EXTENT);

static mut BASE: [u32; MAX_NUM_GPIOMM] = [0; MAX_NUM_GPIOMM];
static mut NUM_GPIOMM: u32 = 0;
// module_param_hw_array!(BASE, uint, ioport, &mut NUM_GPIOMM, 0);
// MODULE_PARM_DESC(base, "Diamond Systems GPIO-MM base addresses");

const GPIOMM_NUM_PPI: usize = 2;

static GPIOMM_VOLATILE_RANGES: [struct_regmap_range; 2] = [
    i8255_volatile_regmap_range(0x0),
    i8255_volatile_regmap_range(0x4),
];
static GPIOMM_VOLATILE_TABLE: struct_regmap_access_table = struct_regmap_access_table {
    yes_ranges: GPIOMM_VOLATILE_RANGES.as_ptr(),
    n_yes_ranges: GPIOMM_VOLATILE_RANGES.len(),
};
static GPIOMM_REGMAP_CONFIG: struct_regmap_config = struct_regmap_config {
    reg_bits: 8,
    reg_stride: 1,
    val_bits: 8,
    io_port: true,
    max_register: 0x7,
    volatile_table: &GPIOMM_VOLATILE_TABLE,
    cache_type: REGCACHE_FLAT,
};

const GPIOMM_NGPIO: usize = 48;
static GPIOMM_NAMES: [&'static core::ffi::c_char; GPIOMM_NGPIO] = [
    c"Port 1A0", c"Port 1A1", c"Port 1A2", c"Port 1A3", c"Port 1A4", c"Port 1A5",
    c"Port 1A6", c"Port 1A7", c"Port 1B0", c"Port 1B1", c"Port 1B2", c"Port 1B3",
    c"Port 1B4", c"Port 1B5", c"Port 1B6", c"Port 1B7", c"Port 1C0", c"Port 1C1",
    c"Port 1C2", c"Port 1C3", c"Port 1C4", c"Port 1C5", c"Port 1C6", c"Port 1C7",
    c"Port 2A0", c"Port 2A1", c"Port 2A2", c"Port 2A3", c"Port 2A4", c"Port 2A5",
    c"Port 2A6", c"Port 2A7", c"Port 2B0", c"Port 2B1", c"Port 2B2", c"Port 2B3",
    c"Port 2B4", c"Port 2B5", c"Port 2B6", c"Port 2B7", c"Port 2C0", c"Port 2C1",
    c"Port 2C2", c"Port 2C3", c"Port 2C4", c"Port 2C5", c"Port 2C6", c"Port 2C7",
];

unsafe fn gpiomm_probe(dev: *mut struct_device, id: u32) -> i32 {
    let name: *const core::ffi::c_char = dev_name(dev);
    let mut config: struct_i8255_regmap_config = core::mem::zeroed();
    let mut regs: *mut core::ffi::c_void;

    if devm_request_region(dev, BASE[id as usize], GPIOMM_EXTENT, name).is_null() {
        dev_err(dev, "Unable to lock port addresses (0x%X-0x%X)\n", BASE[id as usize], BASE[id as usize] + GPIOMM_EXTENT as u32);
        return -EBUSY;
    }

    regs = devm_ioport_map(dev, BASE[id as usize], GPIOMM_EXTENT);
    if regs.is_null() { return -ENOMEM; }

    config.map = devm_regmap_init_mmio(dev, regs, &GPIOMM_REGMAP_CONFIG);
    if IS_ERR(config.map) {
        return dev_err_probe(dev, PTR_ERR(config.map), "Unable to initialize register map\n");
    }
    config.parent = dev;
    config.num_ppi = GPIOMM_NUM_PPI;
    config.names = GPIOMM_NAMES.as_ptr();
    devm_i8255_regmap_register(dev, &config)
}

static mut GPIOMM_DRIVER: struct_isa_driver = struct_isa_driver {
    probe: Some(gpiomm_probe),
    driver: struct_device_driver { name: c"gpio-mm" },
};

// module_isa_driver!(GPIOMM_DRIVER, NUM_GPIOMM);
// MODULE_AUTHOR("William Breathitt Gray <vilhelm.gray@gmail.com>");
// MODULE_DESCRIPTION("Diamond Systems GPIO-MM GPIO driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
