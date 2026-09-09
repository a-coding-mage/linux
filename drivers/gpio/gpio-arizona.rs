// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * gpiolib support for Wolfson Arizona class devices
 *
 * Copyright 2012 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

// Linux kernel dependencies are supplied by the surrounding translation.

#[repr(C)]
struct arizona_gpio {
    arizona: *mut arizona,
    gpio_chip: gpio_chip,
}

unsafe fn arizona_gpio_direction_in(chip: *mut gpio_chip, offset: u32) -> i32 {
    let arizona_gpio = gpiochip_get_data(chip) as *mut arizona_gpio;
    let arizona = (*arizona_gpio).arizona;
    let persistent: bool = gpiochip_line_is_persistent(chip, offset);
    let mut change = false;
    let ret = regmap_update_bits_check(
        (*arizona).regmap,
        ARIZONA_GPIO1_CTRL.wrapping_add(offset),
        ARIZONA_GPN_DIR,
        ARIZONA_GPN_DIR,
        &mut change,
    );
    if ret < 0 {
        return ret;
    }
    if change && persistent {
        pm_runtime_put_autosuspend((*chip).parent);
    }
    0
}

unsafe fn arizona_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let arizona_gpio = gpiochip_get_data(chip) as *mut arizona_gpio;
    let arizona = (*arizona_gpio).arizona;
    let reg = ARIZONA_GPIO1_CTRL.wrapping_add(offset);
    let mut val: u32 = 0;
    let mut ret = regmap_read((*arizona).regmap, reg, &mut val);
    if ret < 0 { return ret; }

    // Resume to read actual registers for input pins
    if val & ARIZONA_GPN_DIR != 0 {
        ret = pm_runtime_get_sync((*chip).parent);
        if ret < 0 {
            dev_err((*chip).parent, "Failed to resume: %d\n", ret);
            pm_runtime_put_autosuspend((*chip).parent);
            return ret;
        }
        // Register is cached, drop it to ensure a physical read
        ret = regcache_drop_region((*arizona).regmap, reg, reg);
        if ret < 0 {
            dev_err((*chip).parent, "Failed to drop cache: %d\n", ret);
            pm_runtime_put_autosuspend((*chip).parent);
            return ret;
        }
        ret = regmap_read((*arizona).regmap, reg, &mut val);
        if ret < 0 {
            pm_runtime_put_autosuspend((*chip).parent);
            return ret;
        }
        pm_runtime_put_autosuspend((*chip).parent);
    }
    if val & ARIZONA_GPN_LVL != 0 { 1 } else { 0 }
}

unsafe fn arizona_gpio_direction_out(chip: *mut gpio_chip, offset: u32, mut value: i32) -> i32 {
    let arizona_gpio = gpiochip_get_data(chip) as *mut arizona_gpio;
    let arizona = (*arizona_gpio).arizona;
    let persistent = gpiochip_line_is_persistent(chip, offset);
    let mut val: u32 = 0;
    let mut ret = regmap_read((*arizona).regmap, ARIZONA_GPIO1_CTRL.wrapping_add(offset), &mut val);
    if ret < 0 { return ret; }
    if val & ARIZONA_GPN_DIR != 0 && persistent {
        ret = pm_runtime_get_sync((*chip).parent);
        if ret < 0 {
            dev_err((*chip).parent, "Failed to resume: %d\n", ret);
            pm_runtime_put((*chip).parent);
            return ret;
        }
    }
    if value != 0 { value = ARIZONA_GPN_LVL; }
    regmap_update_bits((*arizona).regmap, ARIZONA_GPIO1_CTRL.wrapping_add(offset),
                       ARIZONA_GPN_DIR | ARIZONA_GPN_LVL, value)
}

unsafe fn arizona_gpio_set(chip: *mut gpio_chip, offset: u32, mut value: i32) -> i32 {
    let arizona_gpio = gpiochip_get_data(chip) as *mut arizona_gpio;
    let arizona = (*arizona_gpio).arizona;
    if value != 0 { value = ARIZONA_GPN_LVL; }
    regmap_update_bits((*arizona).regmap, ARIZONA_GPIO1_CTRL.wrapping_add(offset),
                       ARIZONA_GPN_LVL, value)
}

static template_chip: gpio_chip = gpio_chip {
    label: "arizona",
    owner: THIS_MODULE,
    direction_input: Some(arizona_gpio_direction_in),
    get: Some(arizona_gpio_get),
    direction_output: Some(arizona_gpio_direction_out),
    set: Some(arizona_gpio_set),
    can_sleep: true,
    ..gpio_chip::default()
};

unsafe fn arizona_gpio_probe(pdev: *mut platform_device) -> i32 {
    let arizona = dev_get_drvdata((*pdev).dev.parent) as *mut arizona;
    let pdata = &mut (*arizona).pdata;
    let arizona_gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<arizona_gpio>(), GFP_KERNEL)
        as *mut arizona_gpio;
    if arizona_gpio.is_null() { return -ENOMEM; }
    device_set_node(&mut (*pdev).dev, dev_fwnode((*pdev).dev.parent));
    (*arizona_gpio).arizona = arizona;
    (*arizona_gpio).gpio_chip = template_chip;
    (*arizona_gpio).gpio_chip.parent = &mut (*pdev).dev;
    match (*arizona).type_ {
        WM5102 | WM5110 | WM8280 | WM8997 | WM8998 | WM1814 => (*arizona_gpio).gpio_chip.ngpio = 5,
        WM1831 | CS47L24 => (*arizona_gpio).gpio_chip.ngpio = 2,
        _ => { dev_err(&(*pdev).dev, "Unknown chip variant %d\n", (*arizona).type_); return -EINVAL; }
    }
    (*arizona_gpio).gpio_chip.base = if pdata.gpio_base != 0 { pdata.gpio_base } else { -1 };
    pm_runtime_enable(&mut (*pdev).dev);
    let ret = devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*arizona_gpio).gpio_chip, arizona_gpio);
    if ret < 0 {
        pm_runtime_disable(&mut (*pdev).dev);
        dev_err(&(*pdev).dev, "Could not register gpiochip, %d\n", ret);
        return ret;
    }
    0
}

static mut arizona_gpio_driver: platform_driver = platform_driver {
    driver: driver { name: "arizona-gpio", ..driver::default() },
    probe: Some(arizona_gpio_probe),
    ..platform_driver::default()
};

module_platform_driver!(arizona_gpio_driver);
module_author!("Mark Brown <broonie@opensource.wolfsonmicro.com>");
module_description!("GPIO interface for Arizona devices");
module_license!("GPL");
module_alias!("platform:arizona-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
