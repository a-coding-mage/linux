// SPDX-License-Identifier: GPL-2.0+
/*
 * TI TPS6591x GPIO driver
 *
 * Copyright 2010 Texas Instruments Inc.
 *
 * Author: Graeme Gregory <gg@slimlogic.co.uk>
 * Author: Jorge Eduardo Candelaria <jedu@slimlogic.co.uk>
 */

// Kernel headers and symbols are supplied by the surrounding repository.

#[repr(C)]
struct Tps65910Gpio {
    gpio_chip: gpio_chip,
    tps65910: *mut tps65910,
}

unsafe fn tps65910_gpio_get(gc: *mut gpio_chip, offset: u32) -> i32 {
    let tps65910_gpio = gpiochip_get_data(gc) as *mut Tps65910Gpio;
    let tps65910 = (*tps65910_gpio).tps65910;
    let mut val: u32 = 0;

    regmap_read((*tps65910).regmap, TPS65910_GPIO0 + offset, &mut val);

    if val & GPIO_STS_MASK != 0 { 1 } else { 0 }
}

unsafe fn tps65910_gpio_set(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let tps65910_gpio = gpiochip_get_data(gc) as *mut Tps65910Gpio;
    let tps65910 = (*tps65910_gpio).tps65910;

    if value != 0 {
        return regmap_set_bits((*tps65910).regmap,
                               TPS65910_GPIO0 + offset, GPIO_SET_MASK);
    }

    regmap_clear_bits((*tps65910).regmap, TPS65910_GPIO0 + offset, GPIO_SET_MASK)
}

unsafe fn tps65910_gpio_output(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let tps65910_gpio = gpiochip_get_data(gc) as *mut Tps65910Gpio;
    let tps65910 = (*tps65910_gpio).tps65910;
    let ret = tps65910_gpio_set(gc, offset, value);

    /* Set the initial value */
    if ret != 0 { return ret; }

    regmap_set_bits((*tps65910).regmap, TPS65910_GPIO0 + offset, GPIO_CFG_MASK)
}

unsafe fn tps65910_gpio_input(gc: *mut gpio_chip, offset: u32) -> i32 {
    let tps65910_gpio = gpiochip_get_data(gc) as *mut Tps65910Gpio;
    let tps65910 = (*tps65910_gpio).tps65910;

    regmap_clear_bits((*tps65910).regmap, TPS65910_GPIO0 + offset, GPIO_CFG_MASK)
}

#[cfg(feature = "CONFIG_OF")]
unsafe fn tps65910_parse_dt_for_gpio(dev: *mut device, tps65910: *mut tps65910,
                                     chip_ngpio: i32) -> *mut tps65910_board {
    let tps65910_board = (*tps65910).of_plat_data as *mut tps65910_board;
    let mut prop_array = [0u32; TPS6591X_MAX_NUM_GPIO as usize];
    let ngpio = core::cmp::min(chip_ngpio, TPS6591X_MAX_NUM_GPIO);
    let ret = of_property_read_u32_array((*tps65910).dev.of_node,
                                         b"ti,en-gpio-sleep\0".as_ptr() as *const i8,
                                         prop_array.as_mut_ptr(), ngpio as usize);

    (*tps65910_board).gpio_base = -1;
    if ret < 0 {
        dev_dbg(dev, b"ti,en-gpio-sleep not specified\0".as_ptr() as *const i8);
        return tps65910_board;
    }
    for idx in 0..ngpio as usize {
        (*tps65910_board).en_gpio_sleep[idx] = prop_array[idx] != 0;
    }
    tps65910_board
}

#[cfg(not(feature = "CONFIG_OF"))]
unsafe fn tps65910_parse_dt_for_gpio(_dev: *mut device, _tps65910: *mut tps65910,
                                     _chip_ngpio: i32) -> *mut tps65910_board { core::ptr::null_mut() }

unsafe fn tps65910_gpio_probe(pdev: *mut platform_device) -> i32 {
    let tps65910 = dev_get_drvdata((*pdev).dev.parent) as *mut tps65910;
    let mut pdata = dev_get_platdata((*tps65910).dev) as *mut tps65910_board;
    let tps65910_gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Tps65910Gpio>(), GFP_KERNEL)
        as *mut Tps65910Gpio;
    if tps65910_gpio.is_null() { return -ENOMEM; }

    device_set_node(&mut (*pdev).dev, dev_fwnode((*pdev).dev.parent));
    (*tps65910_gpio).tps65910 = tps65910;
    (*tps65910_gpio).gpio_chip.owner = THIS_MODULE;
    (*tps65910_gpio).gpio_chip.label = (*tps65910).i2c_client.name;
    (*tps65910_gpio).gpio_chip.ngpio = match tps65910_chip_id(tps65910) {
        TPS65910 => TPS65910_NUM_GPIO,
        TPS65911 => TPS65911_NUM_GPIO,
        _ => return -EINVAL,
    };
    (*tps65910_gpio).gpio_chip.can_sleep = true;
    (*tps65910_gpio).gpio_chip.direction_input = Some(tps65910_gpio_input);
    (*tps65910_gpio).gpio_chip.direction_output = Some(tps65910_gpio_output);
    (*tps65910_gpio).gpio_chip.set = Some(tps65910_gpio_set);
    (*tps65910_gpio).gpio_chip.get = Some(tps65910_gpio_get);
    (*tps65910_gpio).gpio_chip.parent = &mut (*pdev).dev;
    (*tps65910_gpio).gpio_chip.base = if !pdata.is_null() && (*pdata).gpio_base != 0 {
        (*pdata).gpio_base
    } else { -1 };

    if pdata.is_null() && !(*tps65910).dev.of_node.is_null() {
        pdata = tps65910_parse_dt_for_gpio(&mut (*pdev).dev, tps65910,
                                           (*tps65910_gpio).gpio_chip.ngpio as i32);
    }
    if !pdata.is_null() {
        for i in 0..(*tps65910_gpio).gpio_chip.ngpio as usize {
            if !(*pdata).en_gpio_sleep[i] { continue; }
            let ret = regmap_set_bits((*tps65910).regmap, TPS65910_GPIO0 + i as u32, GPIO_SLEEP_MASK);
            if ret < 0 { dev_warn((*tps65910).dev, b"GPIO Sleep setting failed with err %d\n\0".as_ptr() as *const i8, ret); }
        }
    }
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*tps65910_gpio).gpio_chip, tps65910_gpio as *mut core::ffi::c_void)
}

static mut tps65910_gpio_driver: platform_driver = platform_driver {
    driver: driver { name: b"tps65910-gpio\0".as_ptr() as *const i8 },
    probe: Some(tps65910_gpio_probe),
};

unsafe fn tps65910_gpio_init() -> i32 { platform_driver_register(&mut tps65910_gpio_driver) }

// Equivalent of subsys_initcall(tps65910_gpio_init).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
