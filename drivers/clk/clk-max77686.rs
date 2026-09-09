// SPDX-License-Identifier: GPL-2.0+
//
// clk-max77686.c - Clock driver for Maxim 77686/MAX77802
//
// Copyright (C) 2012 Samsung Electornics
// Jonghwa Lee <jonghwa3.lee@samsung.com>

// Linux kernel headers and device-tree clock bindings are supplied by other
// translation units/dependencies.

const MAX77802_CLOCK_LOW_JITTER_SHIFT: u32 = 0x3;

#[repr(C)]
#[derive(Copy, Clone)]
enum max77686_chip_name {
    CHIP_MAX77686,
    CHIP_MAX77802,
    CHIP_MAX77620,
}

#[repr(C)]
struct max77686_hw_clk_info {
    name: *const core::ffi::c_char,
    clk_reg: u32,
    clk_enable_mask: u32,
    flags: u32,
}

#[repr(C)]
struct max77686_clk_init_data {
    regmap: *mut regmap,
    hw: clk_hw,
    clk_idata: clk_init_data,
    clk_info: *const max77686_hw_clk_info,
}

#[repr(C)]
struct max77686_clk_driver_data {
    chip: max77686_chip_name,
    num_clks: usize,
    // C flexible array member: max_clk_data[] __counted_by(num_clks)
    max_clk_data: [max77686_clk_init_data; 0],
}

static max77686_hw_clks_info: [max77686_hw_clk_info; MAX77686_CLKS_NUM as usize] = [
    max77686_hw_clk_info {
        name: b"32khz_ap\0".as_ptr() as *const core::ffi::c_char,
        clk_reg: MAX77686_REG_32KHZ,
        clk_enable_mask: BIT(MAX77686_CLK_AP),
        flags: 0,
    },
    max77686_hw_clk_info {
        name: b"32khz_cp\0".as_ptr() as *const core::ffi::c_char,
        clk_reg: MAX77686_REG_32KHZ,
        clk_enable_mask: BIT(MAX77686_CLK_CP),
        flags: 0,
    },
    max77686_hw_clk_info {
        name: b"32khz_pmic\0".as_ptr() as *const core::ffi::c_char,
        clk_reg: MAX77686_REG_32KHZ,
        clk_enable_mask: BIT(MAX77686_CLK_PMIC),
        flags: 0,
    },
];

static max77802_hw_clks_info: [max77686_hw_clk_info; MAX77802_CLKS_NUM as usize] = [
    max77686_hw_clk_info {
        name: b"32khz_ap\0".as_ptr() as *const core::ffi::c_char,
        clk_reg: MAX77802_REG_32KHZ,
        clk_enable_mask: BIT(MAX77802_CLK_32K_AP),
        flags: 0,
    },
    max77686_hw_clk_info {
        name: b"32khz_cp\0".as_ptr() as *const core::ffi::c_char,
        clk_reg: MAX77802_REG_32KHZ,
        clk_enable_mask: BIT(MAX77802_CLK_32K_CP),
        flags: 0,
    },
];

static max77620_hw_clks_info: [max77686_hw_clk_info; MAX77620_CLKS_NUM as usize] = [
    max77686_hw_clk_info {
        name: b"32khz_out0\0".as_ptr() as *const core::ffi::c_char,
        clk_reg: MAX77620_REG_CNFG1_32K,
        clk_enable_mask: MAX77620_CNFG1_32K_OUT0_EN,
        flags: 0,
    },
];

unsafe fn to_max77686_clk_init_data(hw: *mut clk_hw) -> *mut max77686_clk_init_data {
    container_of!(hw, max77686_clk_init_data, hw)
}

unsafe extern "C" fn max77686_clk_prepare(hw: *mut clk_hw) -> i32 {
    let max77686 = &mut *to_max77686_clk_init_data(hw);
    regmap_update_bits(
        max77686.regmap,
        (*max77686.clk_info).clk_reg,
        (*max77686.clk_info).clk_enable_mask,
        (*max77686.clk_info).clk_enable_mask,
    )
}

unsafe extern "C" fn max77686_clk_unprepare(hw: *mut clk_hw) {
    let max77686 = &mut *to_max77686_clk_init_data(hw);
    regmap_update_bits(
        max77686.regmap,
        (*max77686.clk_info).clk_reg,
        (*max77686.clk_info).clk_enable_mask,
        !(*max77686.clk_info).clk_enable_mask,
    );
}

unsafe extern "C" fn max77686_clk_is_prepared(hw: *mut clk_hw) -> i32 {
    let max77686 = &mut *to_max77686_clk_init_data(hw);
    let mut val: u32 = 0;
    let ret = regmap_read(max77686.regmap, (*max77686.clk_info).clk_reg, &mut val);
    if ret < 0 { return -EINVAL; }
    (val & (*max77686.clk_info).clk_enable_mask) as i32
}

unsafe extern "C" fn max77686_recalc_rate(_hw: *mut clk_hw, _parent_rate: c_ulong) -> c_ulong {
    32768
}

static max77686_clk_ops: clk_ops = clk_ops {
    prepare: Some(max77686_clk_prepare),
    unprepare: Some(max77686_clk_unprepare),
    is_prepared: Some(max77686_clk_is_prepared),
    recalc_rate: Some(max77686_recalc_rate),
};

unsafe extern "C" fn of_clk_max77686_get(
    clkspec: *mut of_phandle_args,
    data: *mut core::ffi::c_void,
) -> *mut clk_hw {
    let drv_data = &mut *(data as *mut max77686_clk_driver_data);
    let idx = (*clkspec).args[0] as usize;
    if idx >= drv_data.num_clks {
        pr_err!("%s: invalid index %u\n", "of_clk_max77686_get", idx);
        return ERR_PTR(-EINVAL);
    }
    (&mut *drv_data.max_clk_data.as_mut_ptr().add(idx)).hw.as_mut_ptr()
}

unsafe extern "C" fn max77686_clk_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev;
    let parent = (*dev).parent;
    let id = platform_get_device_id(pdev);
    let (num_clks, hw_clks) = match (*id).driver_data {
        CHIP_MAX77686 => (MAX77686_CLKS_NUM as usize, max77686_hw_clks_info.as_ptr()),
        CHIP_MAX77802 => (MAX77802_CLKS_NUM as usize, max77802_hw_clks_info.as_ptr()),
        CHIP_MAX77620 => (MAX77620_CLKS_NUM as usize, max77620_hw_clks_info.as_ptr()),
        _ => { dev_err!(dev, "Unknown Chip ID\n"); return -EINVAL; }
    };
    let drv_data = devm_kzalloc(dev, struct_size!(max77686_clk_driver_data, max_clk_data, num_clks), GFP_KERNEL)
        as *mut max77686_clk_driver_data;
    if drv_data.is_null() { return -ENOMEM; }
    (*drv_data).num_clks = num_clks;
    (*drv_data).chip = (*id).driver_data;
    let regmap = dev_get_regmap(parent, core::ptr::null());
    if regmap.is_null() { dev_err!(dev, "Failed to get rtc regmap\n"); return -ENODEV; }
    for i in 0..num_clks {
        let max_clk_data = &mut *(*drv_data).max_clk_data.as_mut_ptr().add(i);
        max_clk_data.regmap = regmap;
        max_clk_data.clk_info = hw_clks.add(i);
        max_clk_data.clk_idata.flags = (*max_clk_data.clk_info).flags;
        max_clk_data.clk_idata.ops = &max77686_clk_ops;
        max_clk_data.clk_idata.name = (*max_clk_data.clk_info).name;
        max_clk_data.hw.init = &max_clk_data.clk_idata;
        let ret = devm_clk_hw_register(dev, &mut max_clk_data.hw);
        if ret != 0 { dev_err!(dev, "Failed to clock register: %d\n", ret); return ret; }
        let ret = devm_clk_hw_register_clkdev(dev, &mut max_clk_data.hw, max_clk_data.clk_idata.name, core::ptr::null());
        if ret < 0 { dev_err!(dev, "Failed to clkdev register: %d\n", ret); return ret; }
    }
    if !(*parent).of_node.is_null() {
        let ret = devm_of_clk_add_hw_provider(dev, Some(of_clk_max77686_get), drv_data as *mut _);
        if ret < 0 { dev_err!(dev, "Failed to register OF clock provider: %d\n", ret); return ret; }
    }
    if (*drv_data).chip == CHIP_MAX77802 {
        let ret = regmap_update_bits(regmap, MAX77802_REG_32KHZ, 1 << MAX77802_CLOCK_LOW_JITTER_SHIFT, 1 << MAX77802_CLOCK_LOW_JITTER_SHIFT);
        if ret < 0 { dev_err!(dev, "Failed to config low-jitter: %d\n", ret); return ret; }
    }
    0
}

// Platform-device ID table, driver registration, and module metadata are
// supplied using the target kernel's Rust module/driver registration macros.
static max77686_clk_id: [platform_device_id; 4] = [
    platform_device_id { name: b"max77686-clk\0".as_ptr() as *const c_char, driver_data: CHIP_MAX77686 },
    platform_device_id { name: b"max77802-clk\0".as_ptr() as *const c_char, driver_data: CHIP_MAX77802 },
    platform_device_id { name: b"max77620-clock\0".as_ptr() as *const c_char, driver_data: CHIP_MAX77620 },
    platform_device_id { name: core::ptr::null(), driver_data: CHIP_MAX77686 },
];

static mut max77686_clk_driver: platform_driver = platform_driver {
    driver: driver { name: b"max77686-clk\0".as_ptr() as *const c_char },
    probe: Some(max77686_clk_probe),
    id_table: max77686_clk_id.as_ptr(),
};

// MODULE_DEVICE_TABLE(platform, max77686_clk_id);
// module_platform_driver(max77686_clk_driver);
// MODULE_DESCRIPTION("MAXIM 77686 Clock Driver");
// MODULE_AUTHOR("Jonghwa Lee <jonghwa3.lee@samsung.com>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
