// SPDX-License-Identifier: GPL-2.0+
/*
 * gpiolib support for Wolfson WM831x PMICs
 *
 * Copyright 2009 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation unit.

#[repr(C)]
struct wm831x_gpio {
    wm831x: *mut wm831x,
    gpio_chip: gpio_chip,
}

unsafe extern "C" {
    static THIS_MODULE: module;
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn wm831x_set_bits(wm831x: *mut wm831x, reg: i32, mask: i32, val: i32) -> i32;
    fn wm831x_reg_read(wm831x: *mut wm831x, reg: i32) -> i32;
    fn irq_create_mapping(domain: *mut irq_domain, irq: i32) -> i32;
    fn pinconf_to_config_param(config: c_ulong) -> i32;
    fn pinconf_to_config_argument(config: c_ulong) -> c_ulong;
    fn dev_get_drvdata(dev: *mut device) -> *mut core::ffi::c_void;
    fn dev_fwnode(dev: *mut device) -> *mut fwnode_handle;
    fn device_set_node(dev: *mut device, node: *mut fwnode_handle);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_gpiochip_add_data(dev: *mut device, chip: *mut gpio_chip, data: *mut core::ffi::c_void) -> i32;
    fn platform_driver_register(driver: *mut platform_driver) -> i32;
    fn platform_driver_unregister(driver: *mut platform_driver);
}

unsafe fn wm831x_gpio_direction_in(chip: *mut gpio_chip, offset: c_uint) -> i32 {
    let wm831x_gpio = gpiochip_get_data(chip) as *mut wm831x_gpio;
    let wm831x = (*wm831x_gpio).wm831x;
    let mut val = WM831X_GPN_DIR;

    if (*wm831x).has_gpio_ena {
        val |= WM831X_GPN_TRI;
    }

    wm831x_set_bits(wm831x, WM831X_GPIO1_CONTROL + offset as i32,
        WM831X_GPN_DIR | WM831X_GPN_TRI | WM831X_GPN_FN_MASK, val)
}

unsafe fn wm831x_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> i32 {
    let wm831x_gpio = gpiochip_get_data(chip) as *mut wm831x_gpio;
    let wm831x = (*wm831x_gpio).wm831x;
    let ret = wm831x_reg_read(wm831x, WM831X_GPIO_LEVEL);
    if ret < 0 { return ret; }
    if ret & (1 << offset) != 0 { 1 } else { 0 }
}

unsafe fn wm831x_gpio_set(chip: *mut gpio_chip, offset: c_uint, value: i32) -> i32 {
    let wm831x_gpio = gpiochip_get_data(chip) as *mut wm831x_gpio;
    let wm831x = (*wm831x_gpio).wm831x;
    wm831x_set_bits(wm831x, WM831X_GPIO_LEVEL, 1 << offset, value << offset)
}

unsafe fn wm831x_gpio_direction_out(chip: *mut gpio_chip, offset: c_uint, value: i32) -> i32 {
    let wm831x_gpio = gpiochip_get_data(chip) as *mut wm831x_gpio;
    let wm831x = (*wm831x_gpio).wm831x;
    let mut val = 0;
    if (*wm831x).has_gpio_ena { val |= WM831X_GPN_TRI; }
    let ret = wm831x_set_bits(wm831x, WM831X_GPIO1_CONTROL + offset as i32,
        WM831X_GPN_DIR | WM831X_GPN_TRI | WM831X_GPN_FN_MASK, val);
    if ret < 0 { return ret; }
    wm831x_gpio_set(chip, offset, value)
}

unsafe fn wm831x_gpio_to_irq(chip: *mut gpio_chip, offset: c_uint) -> i32 {
    let wm831x_gpio = gpiochip_get_data(chip) as *mut wm831x_gpio;
    irq_create_mapping((*(*wm831x_gpio).wm831x).irq_domain, WM831X_IRQ_GPIO_1 + offset as i32)
}

unsafe fn wm831x_gpio_set_debounce(wm831x: *mut wm831x, offset: c_uint, debounce: c_uint) -> i32 {
    let reg = WM831X_GPIO1_CONTROL + offset as i32;
    let ret = wm831x_reg_read(wm831x, reg);
    if ret < 0 { return ret; }
    match ret & WM831X_GPN_FN_MASK {
        0 | 1 => {},
        _ => return -EBUSY,
    }
    let fn_ = if (32..=64).contains(&debounce) { 0 }
        else if (4000..=8000).contains(&debounce) { 1 }
        else { return -EINVAL; };
    wm831x_set_bits(wm831x, reg, WM831X_GPN_FN_MASK, fn_)
}

unsafe fn wm831x_set_config(chip: *mut gpio_chip, offset: c_uint, config: c_ulong) -> i32 {
    let wm831x_gpio = gpiochip_get_data(chip) as *mut wm831x_gpio;
    let wm831x = (*wm831x_gpio).wm831x;
    let reg = WM831X_GPIO1_CONTROL + offset as i32;
    match pinconf_to_config_param(config) {
        PIN_CONFIG_DRIVE_OPEN_DRAIN => wm831x_set_bits(wm831x, reg, WM831X_GPN_OD_MASK, WM831X_GPN_OD),
        PIN_CONFIG_DRIVE_PUSH_PULL => wm831x_set_bits(wm831x, reg, WM831X_GPN_OD_MASK, 0),
        PIN_CONFIG_INPUT_DEBOUNCE => wm831x_gpio_set_debounce(wm831x, offset, pinconf_to_config_argument(config) as c_uint),
        _ => -ENOTSUPP,
    }
}

// CONFIG_DEBUG_FS provides the source's optional debug display callback.

static mut template_chip: gpio_chip = gpio_chip {
    label: "wm831x" as *const _ as *mut _, owner: &THIS_MODULE,
    direction_input: Some(wm831x_gpio_direction_in), get: Some(wm831x_gpio_get),
    direction_output: Some(wm831x_gpio_direction_out), set: Some(wm831x_gpio_set),
    to_irq: Some(wm831x_gpio_to_irq), set_config: Some(wm831x_set_config),
    dbg_show: None, can_sleep: true,
};

unsafe fn wm831x_gpio_probe(pdev: *mut platform_device) -> i32 {
    let wm831x = dev_get_drvdata((*pdev).dev.parent) as *mut wm831x;
    let pdata = &mut (*wm831x).pdata;
    let wm831x_gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<wm831x_gpio>(), GFP_KERNEL) as *mut wm831x_gpio;
    if wm831x_gpio.is_null() { return -ENOMEM; }
    device_set_node(&mut (*pdev).dev, dev_fwnode((*pdev).dev.parent));
    (*wm831x_gpio).wm831x = wm831x;
    (*wm831x_gpio).gpio_chip = template_chip;
    (*wm831x_gpio).gpio_chip.ngpio = (*wm831x).num_gpio;
    (*wm831x_gpio).gpio_chip.parent = &mut (*pdev).dev;
    (*wm831x_gpio).gpio_chip.base = if !pdata.is_null() && (*pdata).gpio_base != 0 { (*pdata).gpio_base } else { -1 };
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*wm831x_gpio).gpio_chip, wm831x_gpio as *mut _)
}

static mut wm831x_gpio_driver: platform_driver = platform_driver {
    driver: driver { name: "wm831x-gpio", ..unsafe { core::mem::zeroed() } },
    probe: Some(wm831x_gpio_probe),
    ..unsafe { core::mem::zeroed() }
};

unsafe fn wm831x_gpio_init() -> i32 { platform_driver_register(&mut wm831x_gpio_driver) }
unsafe fn wm831x_gpio_exit() { platform_driver_unregister(&mut wm831x_gpio_driver); }

// subsys_initcall(wm831x_gpio_init);
// module_exit(wm831x_gpio_exit);
// MODULE_AUTHOR("Mark Brown <broonie@opensource.wolfsonmicro.com>");
// MODULE_DESCRIPTION("GPIO interface for WM831x PMICs");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:wm831x-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
