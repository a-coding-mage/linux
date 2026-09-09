// SPDX-License-Identifier: GPL-2.0-only
/*
 * ROHM BD9571MWV-M and BD9574MWF-M GPIO driver
 *
 * Copyright (C) 2017 Marek Vasut <marek.vasut+renesas@gmail.com>
 *
 * Based on the TPS65086 driver
 *
 * NOTE: Interrupts are not supported yet.
 */

// Linux kernel dependencies supplied by the surrounding Rust bindings.

#[repr(C)]
struct bd9571mwv_gpio {
    regmap: *mut regmap,
    chip: gpio_chip,
}

unsafe fn bd9571mwv_gpio_get_direction(
    chip: *mut gpio_chip,
    offset: core::ffi::c_uint,
) -> core::ffi::c_int {
    let gpio = gpiochip_get_data(chip) as *mut bd9571mwv_gpio;
    let mut val: core::ffi::c_int = 0;

    let ret = regmap_read((*gpio).regmap, BD9571MWV_GPIO_DIR, &mut val);
    if ret < 0 {
        return ret;
    }
    if val & BIT(offset) != 0 {
        return GPIO_LINE_DIRECTION_IN;
    }

    GPIO_LINE_DIRECTION_OUT
}

unsafe fn bd9571mwv_gpio_direction_input(
    chip: *mut gpio_chip,
    offset: core::ffi::c_uint,
) -> core::ffi::c_int {
    let gpio = gpiochip_get_data(chip) as *mut bd9571mwv_gpio;

    regmap_update_bits((*gpio).regmap, BD9571MWV_GPIO_DIR, BIT(offset), 0);

    0
}

unsafe fn bd9571mwv_gpio_direction_output(
    chip: *mut gpio_chip,
    offset: core::ffi::c_uint,
    value: core::ffi::c_int,
) -> core::ffi::c_int {
    let gpio = gpiochip_get_data(chip) as *mut bd9571mwv_gpio;

    // Set the initial value
    regmap_update_bits(
        (*gpio).regmap,
        BD9571MWV_GPIO_OUT,
        BIT(offset),
        if value != 0 { BIT(offset) } else { 0 },
    );
    regmap_update_bits(
        (*gpio).regmap,
        BD9571MWV_GPIO_DIR,
        BIT(offset),
        BIT(offset),
    );

    0
}

unsafe fn bd9571mwv_gpio_get(
    chip: *mut gpio_chip,
    offset: core::ffi::c_uint,
) -> core::ffi::c_int {
    let gpio = gpiochip_get_data(chip) as *mut bd9571mwv_gpio;
    let mut val: core::ffi::c_int = 0;

    let ret = regmap_read((*gpio).regmap, BD9571MWV_GPIO_IN, &mut val);
    if ret < 0 {
        return ret;
    }

    if val & BIT(offset) != 0 { 1 } else { 0 }
}

unsafe fn bd9571mwv_gpio_set(
    chip: *mut gpio_chip,
    offset: core::ffi::c_uint,
    value: core::ffi::c_int,
) -> core::ffi::c_int {
    let gpio = gpiochip_get_data(chip) as *mut bd9571mwv_gpio;

    regmap_update_bits(
        (*gpio).regmap,
        BD9571MWV_GPIO_OUT,
        BIT(offset),
        if value != 0 { BIT(offset) } else { 0 },
    )
}

static template_chip: gpio_chip = gpio_chip {
    label: "bd9571mwv-gpio",
    owner: THIS_MODULE,
    get_direction: Some(bd9571mwv_gpio_get_direction),
    direction_input: Some(bd9571mwv_gpio_direction_input),
    direction_output: Some(bd9571mwv_gpio_direction_output),
    get: Some(bd9571mwv_gpio_get),
    set: Some(bd9571mwv_gpio_set),
    base: -1,
    ngpio: 2,
    can_sleep: true,
    ..unsafe { core::mem::zeroed() }
};

unsafe fn bd9571mwv_gpio_probe(
    pdev: *mut platform_device,
) -> core::ffi::c_int {
    let gpio = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<bd9571mwv_gpio>(),
        GFP_KERNEL,
    ) as *mut bd9571mwv_gpio;
    if gpio.is_null() {
        return -ENOMEM;
    }

    (*gpio).regmap = dev_get_regmap((*pdev).dev.parent, core::ptr::null());
    (*gpio).chip = template_chip;
    (*gpio).chip.parent = (*pdev).dev.parent;

    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*gpio).chip, gpio as *mut core::ffi::c_void)
}

static bd9571mwv_gpio_id_table: [platform_device_id; 3] = [
    platform_device_id { name: "bd9571mwv-gpio", driver_data: ROHM_CHIP_TYPE_BD9571 },
    platform_device_id { name: "bd9574mwf-gpio", driver_data: ROHM_CHIP_TYPE_BD9574 },
    platform_device_id { name: "", driver_data: 0 }, // sentinel
];

// MODULE_DEVICE_TABLE(platform, bd9571mwv_gpio_id_table);

static mut bd9571mwv_gpio_driver: platform_driver = platform_driver {
    driver: driver {
        name: "bd9571mwv-gpio",
        ..unsafe { core::mem::zeroed() }
    },
    probe: Some(bd9571mwv_gpio_probe),
    id_table: bd9571mwv_gpio_id_table.as_ptr(),
    ..unsafe { core::mem::zeroed() }
};

// module_platform_driver(bd9571mwv_gpio_driver);

// MODULE_AUTHOR("Marek Vasut <marek.vasut+renesas@gmail.com>");
// MODULE_DESCRIPTION("BD9571MWV GPIO driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
