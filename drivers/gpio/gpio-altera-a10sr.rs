// SPDX-License-Identifier: GPL-2.0-only
/*
 *  Copyright Intel Corporation (C) 2014-2016. All Rights Reserved
 *
 * GPIO driver for  Altera Arria10 MAX5 System Resource Chip
 *
 * Adapted from gpio-tps65910.c
 */

// C dependencies supplied by the surrounding kernel environment.

#[repr(C)]
pub struct altr_a10sr_gpio {
    pub gp: gpio_chip,
    pub regmap: *mut regmap,
}

pub unsafe fn altr_a10sr_gpio_get(chip: *mut gpio_chip, offset: c_uint) -> c_int {
    let gpio = gpiochip_get_data(chip) as *mut altr_a10sr_gpio;
    let mut val: c_uint = 0;

    let ret = regmap_read((*gpio).regmap, ALTR_A10SR_PBDSW_REG, &mut val);
    if ret < 0 {
        return ret;
    }

    ((val & BIT(offset - ALTR_A10SR_LED_VALID_SHIFT)) != 0) as c_int
}

pub unsafe fn altr_a10sr_gpio_set(
    chip: *mut gpio_chip,
    offset: c_uint,
    value: c_int,
) -> c_int {
    let gpio = gpiochip_get_data(chip) as *mut altr_a10sr_gpio;
    let bit = BIT(ALTR_A10SR_LED_VALID_SHIFT + offset);

    regmap_update_bits(
        (*gpio).regmap,
        ALTR_A10SR_LED_REG,
        bit,
        if value != 0 { bit } else { 0 },
    )
}

pub unsafe fn altr_a10sr_gpio_direction_input(
    _gc: *mut gpio_chip,
    nr: c_uint,
) -> c_int {
    if nr < (ALTR_A10SR_IN_VALID_RANGE_LO - ALTR_A10SR_LED_VALID_SHIFT) {
        return -EINVAL;
    }

    0
}

pub unsafe fn altr_a10sr_gpio_direction_output(
    gc: *mut gpio_chip,
    nr: c_uint,
    value: c_int,
) -> c_int {
    if nr > (ALTR_A10SR_OUT_VALID_RANGE_HI - ALTR_A10SR_LED_VALID_SHIFT) {
        return -EINVAL;
    }

    altr_a10sr_gpio_set(gc, nr, value);
    0
}

pub static altr_a10sr_gc: gpio_chip = gpio_chip {
    label: "altr_a10sr_gpio" as *const _ as *const c_char,
    owner: THIS_MODULE,
    get: Some(altr_a10sr_gpio_get),
    set: Some(altr_a10sr_gpio_set),
    direction_input: Some(altr_a10sr_gpio_direction_input),
    direction_output: Some(altr_a10sr_gpio_direction_output),
    can_sleep: true,
    ngpio: 12,
    base: -1,
    ..unsafe { core::mem::zeroed() }
};

pub unsafe fn altr_a10sr_gpio_probe(pdev: *mut platform_device) -> c_int {
    let a10sr = dev_get_drvdata((*pdev).dev.parent) as *mut altr_a10sr;
    let gpio = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<altr_a10sr_gpio>(),
        GFP_KERNEL,
    ) as *mut altr_a10sr_gpio;
    if gpio.is_null() {
        return -ENOMEM;
    }

    (*gpio).regmap = (*a10sr).regmap;
    (*gpio).gp = altr_a10sr_gc;
    (*gpio).gp.parent = (*pdev).dev.parent;
    (*gpio).gp.fwnode = dev_fwnode(&(*pdev).dev);

    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*gpio).gp, gpio)
}

pub static mut altr_a10sr_gpio_of_match: [of_device_id; 2] = [
    of_device_id {
        compatible: "altr,a10sr-gpio" as *const _ as *const c_char,
        ..unsafe { core::mem::zeroed() }
    },
    unsafe { core::mem::zeroed() },
];

pub static mut altr_a10sr_gpio_driver: platform_driver = platform_driver {
    probe: Some(altr_a10sr_gpio_probe),
    driver: driver {
        name: "altr_a10sr_gpio" as *const _ as *const c_char,
        of_match_table: unsafe { altr_a10sr_gpio_of_match.as_ptr() },
        ..unsafe { core::mem::zeroed() }
    },
};

// module_platform_driver(altr_a10sr_gpio_driver);
// MODULE_DEVICE_TABLE(of, altr_a10sr_gpio_of_match);
// MODULE_LICENSE("GPL v2");
// MODULE_AUTHOR("Thor Thayer <tthayer@opensource.altera.com>");
// MODULE_DESCRIPTION("Altera Arria10 System Resource Chip GPIO");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
