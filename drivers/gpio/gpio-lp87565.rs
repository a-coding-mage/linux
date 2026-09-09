// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2017 Texas Instruments Incorporated - http://www.ti.com/
 *	Keerthy <j-keerthy@ti.com>
 *
 * Based on the LP873X driver
 */

// Linux kernel dependencies are supplied by the surrounding translation.

#[repr(C)]
pub struct lp87565_gpio {
    pub chip: gpio_chip,
    pub map: *mut regmap,
}

unsafe fn lp87565_gpio_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let mut val: i32 = 0;

    let ret = regmap_read((*gpio).map, LP87565_REG_GPIO_IN, &mut val);
    if ret < 0 {
        return ret;
    }

    !!((val as u32) & BIT(offset)) as i32
}

unsafe fn lp87565_gpio_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(chip);

    regmap_update_bits(
        (*gpio).map,
        LP87565_REG_GPIO_OUT,
        BIT(offset),
        if value != 0 { BIT(offset) } else { 0 },
    )
}

unsafe fn lp87565_gpio_get_direction(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let mut val: i32 = 0;

    let ret = regmap_read((*gpio).map, LP87565_REG_GPIO_CONFIG, &mut val);
    if ret < 0 {
        return ret;
    }

    if (val as u32) & BIT(offset) != 0 {
        return GPIO_LINE_DIRECTION_OUT;
    }

    GPIO_LINE_DIRECTION_IN
}

unsafe fn lp87565_gpio_direction_input(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip);

    regmap_update_bits((*gpio).map, LP87565_REG_GPIO_CONFIG, BIT(offset), 0)
}

unsafe fn lp87565_gpio_direction_output(
    chip: *mut gpio_chip,
    offset: u32,
    value: i32,
) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let ret = lp87565_gpio_set(chip, offset, value);
    if ret != 0 {
        return ret;
    }

    regmap_update_bits(
        (*gpio).map,
        LP87565_REG_GPIO_CONFIG,
        BIT(offset),
        BIT(offset),
    )
}

unsafe fn lp87565_gpio_request(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(gc);
    let ret: i32;

    match offset {
        0 | 1 | 2 => {
            /*
             * MUX can program the pin to be in EN1/2/3 pin mode
             * Or GPIO1/2/3 mode.
             * Setup the GPIO*_SEL MUX to GPIO mode
             */
            ret = regmap_update_bits(
                (*gpio).map,
                LP87565_REG_PIN_FUNCTION,
                BIT(offset),
                BIT(offset),
            );
            if ret != 0 {
                return ret;
            }
        }
        _ => return -EINVAL,
    }

    0
}

unsafe fn lp87565_gpio_set_config(
    gc: *mut gpio_chip,
    offset: u32,
    config: c_ulong,
) -> i32 {
    let gpio = gpiochip_get_data(gc);

    match pinconf_to_config_param(config) {
        PIN_CONFIG_DRIVE_OPEN_DRAIN => regmap_update_bits(
            (*gpio).map,
            LP87565_REG_GPIO_CONFIG,
            BIT(offset + __ffs(LP87565_GPIO1_OD)),
            BIT(offset + __ffs(LP87565_GPIO1_OD)),
        ),
        PIN_CONFIG_DRIVE_PUSH_PULL => regmap_update_bits(
            (*gpio).map,
            LP87565_REG_GPIO_CONFIG,
            BIT(offset + __ffs(LP87565_GPIO1_OD)),
            0,
        ),
        _ => -ENOTSUPP,
    }
}

static mut template_chip: gpio_chip = gpio_chip {
    label: "lp87565-gpio\0".as_ptr() as *const c_char,
    owner: THIS_MODULE,
    request: Some(lp87565_gpio_request),
    get_direction: Some(lp87565_gpio_get_direction),
    direction_input: Some(lp87565_gpio_direction_input),
    direction_output: Some(lp87565_gpio_direction_output),
    get: Some(lp87565_gpio_get),
    set: Some(lp87565_gpio_set),
    set_config: Some(lp87565_gpio_set_config),
    base: -1,
    ngpio: 3,
    can_sleep: true,
};

unsafe fn lp87565_gpio_probe(pdev: *mut platform_device) -> i32 {
    let gpio: *mut lp87565_gpio;
    let lp87565: *mut lp87565;

    gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<lp87565_gpio>(), GFP_KERNEL)
        as *mut lp87565_gpio;
    if gpio.is_null() {
        return -ENOMEM;
    }

    lp87565 = dev_get_drvdata((*pdev).dev.parent);
    (*gpio).chip = template_chip;
    (*gpio).chip.parent = (*lp87565).dev;
    (*gpio).map = (*lp87565).regmap;

    let ret = devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*gpio).chip, gpio as *mut c_void);
    if ret < 0 {
        dev_err(&mut (*pdev).dev, "Could not register gpiochip, %d\n", ret);
        return ret;
    }

    0
}

static mut lp87565_gpio_id_table: [platform_device_id; 2] = [
    platform_device_id { name: "lp87565-q1-gpio\0".as_ptr() as *const c_char },
    platform_device_id { name: core::ptr::null() },
];

static mut lp87565_gpio_driver: platform_driver = platform_driver {
    driver: device_driver { name: "lp87565-gpio\0".as_ptr() as *const c_char },
    probe: Some(lp87565_gpio_probe),
    id_table: lp87565_gpio_id_table.as_ptr(),
};

// MODULE_DEVICE_TABLE(platform, lp87565_gpio_id_table);
// module_platform_driver(lp87565_gpio_driver);
// MODULE_AUTHOR("Keerthy <j-keerthy@ti.com>");
// MODULE_DESCRIPTION("LP87565 GPIO driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
