// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 ROHM Semiconductors

// C dependencies: Linux kernel clock, platform-device, regmap, and ROHM MFD APIs.

/* clk control registers */
/* BD71815 */
const BD71815_REG_OUT32K: u8 = 0x1d;
/* BD71828 */
const BD71828_REG_OUT32K: u8 = 0x4B;
/* BD71837 and BD71847 */
const BD718XX_REG_OUT32K: u8 = 0x2E;
/* BD72720 */
const BD72720_REG_OUT32K: u8 = 0x9a;
/*
 * BD71837, BD71847, and BD71828 all use bit [0] to clk output control
 */
const CLK_OUT_EN_MASK: u8 = 1u8 << 0;

#[repr(C)]
struct bd718xx_clk {
    hw: clk_hw,
    reg: u8,
    mask: u8,
    pdev: *mut platform_device,
    regmap: *mut regmap,
}

unsafe fn bd71837_clk_set(c: *mut bd718xx_clk, status: u32) -> i32 {
    regmap_update_bits((*c).regmap, (*c).reg, (*c).mask, status)
}

unsafe extern "C" fn bd71837_clk_disable(hw: *mut clk_hw) {
    let c = container_of!(hw, bd718xx_clk, hw);
    let rv = bd71837_clk_set(c, 0);
    if rv != 0 {
        dev_dbg!(&(*(*c).pdev).dev, "Failed to disable 32K clk (%d)\n", rv);
    }
}

unsafe extern "C" fn bd71837_clk_enable(hw: *mut clk_hw) -> i32 {
    let c = container_of!(hw, bd718xx_clk, hw);
    bd71837_clk_set(c, 0xffff_ffff)
}

unsafe extern "C" fn bd71837_clk_is_enabled(hw: *mut clk_hw) -> i32 {
    let c = container_of!(hw, bd718xx_clk, hw);
    let mut enabled: i32 = 0;
    let rval = regmap_read((*c).regmap, (*c).reg, &mut enabled);
    if rval != 0 {
        return rval;
    }
    enabled & (*c).mask as i32
}

static bd71837_clk_ops: clk_ops = clk_ops {
    prepare: Some(bd71837_clk_enable),
    unprepare: Some(bd71837_clk_disable),
    is_prepared: Some(bd71837_clk_is_enabled),
};

unsafe extern "C" fn bd71837_clk_probe(pdev: *mut platform_device) -> i32 {
    let mut c: *mut bd718xx_clk;
    let mut rval: i32 = -ENOMEM;
    let mut parent_clk: *const c_char;
    let parent: *mut device = (*pdev).dev.parent;
    let mut init = clk_init_data {
        name: b"bd718xx-32k-out\0".as_ptr() as *const c_char,
        ops: &bd71837_clk_ops,
        num_parents: 0,
        parent_names: core::ptr::null(),
    };
    let chip: rohm_chip_type = (*platform_get_device_id(pdev)).driver_data;

    c = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<bd718xx_clk>(), GFP_KERNEL)
        as *mut bd718xx_clk;
    if c.is_null() {
        return -ENOMEM;
    }

    (*c).regmap = dev_get_regmap((*pdev).dev.parent, core::ptr::null());
    if (*c).regmap.is_null() {
        return -ENODEV;
    }

    init.num_parents = 1;
    parent_clk = of_clk_get_parent_name((*parent).of_node, 0);
    init.parent_names = &parent_clk;
    if parent_clk.is_null() {
        dev_err!(&(*pdev).dev, "No parent clk found\n");
        return -EINVAL;
    }
    match chip {
        ROHM_CHIP_TYPE_BD71837 | ROHM_CHIP_TYPE_BD71847 => {
            (*c).reg = BD718XX_REG_OUT32K;
            (*c).mask = CLK_OUT_EN_MASK;
        }
        ROHM_CHIP_TYPE_BD71828 => {
            (*c).reg = BD71828_REG_OUT32K;
            (*c).mask = CLK_OUT_EN_MASK;
        }
        ROHM_CHIP_TYPE_BD71815 => {
            (*c).reg = BD71815_REG_OUT32K;
            (*c).mask = CLK_OUT_EN_MASK;
        }
        ROHM_CHIP_TYPE_BD72720 => {
            (*c).reg = BD72720_REG_OUT32K;
            (*c).mask = CLK_OUT_EN_MASK;
        }
        _ => {
            dev_err!(&(*pdev).dev, "Unknown clk chip\n");
            return -EINVAL;
        }
    }
    (*c).pdev = pdev;
    (*c).hw.init = &init;
    of_property_read_string_index((*parent).of_node, b"clock-output-names\0".as_ptr() as *const c_char, 0, &mut init.name);

    rval = devm_clk_hw_register(&mut (*pdev).dev, &mut (*c).hw);
    if rval != 0 {
        dev_err!(&(*pdev).dev, "failed to register 32K clk");
        return rval;
    }
    rval = devm_of_clk_add_hw_provider(&mut (*pdev).dev, of_clk_hw_simple_get, &mut (*c).hw);
    if rval != 0 {
        dev_err!(&(*pdev).dev, "adding clk provider failed\n");
    }
    rval
}

static bd718x7_clk_id: [platform_device_id; 6] = [
    platform_device_id { name: b"bd71837-clk\0".as_ptr() as *const c_char, driver_data: ROHM_CHIP_TYPE_BD71837 },
    platform_device_id { name: b"bd71847-clk\0".as_ptr() as *const c_char, driver_data: ROHM_CHIP_TYPE_BD71847 },
    platform_device_id { name: b"bd71828-clk\0".as_ptr() as *const c_char, driver_data: ROHM_CHIP_TYPE_BD71828 },
    platform_device_id { name: b"bd71815-clk\0".as_ptr() as *const c_char, driver_data: ROHM_CHIP_TYPE_BD71815 },
    platform_device_id { name: b"bd72720-clk\0".as_ptr() as *const c_char, driver_data: ROHM_CHIP_TYPE_BD72720 },
    platform_device_id { name: core::ptr::null(), driver_data: 0 },
];

static mut bd71837_clk: platform_driver = platform_driver {
    driver: device_driver { name: b"bd718xx-clk\0".as_ptr() as *const c_char },
    probe: Some(bd71837_clk_probe),
    id_table: bd718x7_clk_id.as_ptr(),
};

// module_platform_driver(bd71837_clk)
// MODULE_DEVICE_TABLE(platform, bd718x7_clk_id)
// MODULE_AUTHOR("Matti Vaittinen <matti.vaittinen@fi.rohmeurope.com>")
// MODULE_DESCRIPTION("BD718(15/18/28/37/47/50) and BD72720 chip clk driver")
// MODULE_LICENSE("GPL")
// MODULE_ALIAS("platform:bd718xx-clk")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
