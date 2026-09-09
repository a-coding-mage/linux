// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2026 Linaro Inc.
//   Author: AKASHI takahiro <takahiro.akashi@linaro.org>

// Dependencies supplied by the kernel GPIO, pinctrl, platform-device, and
// gpiolib interfaces are intentionally referenced but not implemented here.

unsafe fn pin_control_gpio_get_direction(
    gc: *mut gpio_chip,
    offset: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut config: ::core::ffi::c_ulong = PIN_CONFIG_OUTPUT_ENABLE;
    let ret = pinctrl_gpio_get_config(gc, offset, &mut config);
    if ret != 0 {
        return ret;
    }
    if config != 0 {
        return GPIO_LINE_DIRECTION_OUT;
    }

    GPIO_LINE_DIRECTION_IN
}

unsafe fn pin_control_gpio_get(
    chip: *mut gpio_chip,
    offset: ::core::ffi::c_uint,
) -> ::core::ffi::c_int {
    let mut config: ::core::ffi::c_ulong = PIN_CONFIG_LEVEL;
    let ret = pinctrl_gpio_get_config(chip, offset, &mut config);
    if ret != 0 {
        return ret;
    }

    (config != 0) as ::core::ffi::c_int
}

unsafe fn pin_control_gpio_set(
    chip: *mut gpio_chip,
    offset: ::core::ffi::c_uint,
    val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let config = pinconf_to_config_packed(PIN_CONFIG_LEVEL, val);
    pinctrl_gpio_set_config(chip, offset, config)
}

unsafe fn pin_control_gpio_direction_output(
    chip: *mut gpio_chip,
    offset: ::core::ffi::c_uint,
    val: ::core::ffi::c_int,
) -> ::core::ffi::c_int {
    let ret = pin_control_gpio_set(chip, offset, val);
    if ret != 0 {
        return ret;
    }

    pinctrl_gpio_direction_output(chip, offset)
}

unsafe fn pin_control_gpio_probe(
    pdev: *mut platform_device,
) -> ::core::ffi::c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let chip = devm_kzalloc(dev, core::mem::size_of::<gpio_chip>(), GFP_KERNEL)
        as *mut gpio_chip;
    if chip.is_null() {
        return -ENOMEM;
    }

    (*chip).label = dev_name(dev);
    (*chip).parent = dev;
    (*chip).base = -1;

    (*chip).request = Some(gpiochip_generic_request);
    (*chip).free = Some(gpiochip_generic_free);
    (*chip).get_direction = Some(pin_control_gpio_get_direction);
    (*chip).direction_input = Some(pinctrl_gpio_direction_input);
    (*chip).direction_output = Some(pin_control_gpio_direction_output);
    (*chip).get = Some(pin_control_gpio_get);
    (*chip).set = Some(pin_control_gpio_set);
    (*chip).set_config = Some(gpiochip_generic_config);

    devm_gpiochip_add_data(dev, chip, core::ptr::null_mut())
}

static mut PIN_CONTROL_GPIO_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: c"scmi-pinctrl-gpio".as_ptr(),
    },
    of_device_id { /* sentinel */ },
];

static mut PIN_CONTROL_GPIO_DRIVER: platform_driver = platform_driver {
    probe: Some(pin_control_gpio_probe),
    driver: device_driver {
        name: c"pin-control-gpio".as_ptr(),
        of_match_table: unsafe { PIN_CONTROL_GPIO_MATCH.as_ptr() },
    },
};

// MODULE_DEVICE_TABLE(of, pin_control_gpio_match);
// module_platform_driver(pin_control_gpio_driver);

// MODULE_AUTHOR("AKASHI Takahiro <takahiro.akashi@linaro.org>");
// MODULE_DESCRIPTION("Pinctrl based GPIO driver");
// MODULE_LICENSE("GPL");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
