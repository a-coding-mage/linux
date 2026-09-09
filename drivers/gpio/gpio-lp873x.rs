// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2016 Texas Instruments Incorporated - http://www.ti.com/
 *	Keerthy <j-keerthy@ti.com>
 *
 * Based on the TPS65218 driver
 */

// The Linux GPIO, module, platform-device, regmap, and LP873X declarations
// used here are supplied by the surrounding kernel translation.

const BITS_PER_GPO: u32 = 0x4;
const LP873X_GPO_CTRL_OD: u32 = 0x2;

#[repr(C)]
struct Lp873xGpio {
    chip: GpioChip,
    lp873: *mut Lp873x,
}

unsafe fn lp873x_gpio_get_direction(_chip: *mut GpioChip, _offset: u32) -> i32 {
    // This device is output only
    GPIO_LINE_DIRECTION_OUT
}

unsafe fn lp873x_gpio_direction_input(_chip: *mut GpioChip, _offset: u32) -> i32 {
    // This device is output only
    -EINVAL
}

unsafe fn lp873x_gpio_direction_output(
    chip: *mut GpioChip,
    offset: u32,
    value: i32,
) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut Lp873xGpio;

    // Set the initial value
    regmap_update_bits(
        (*(*gpio).lp873).regmap,
        LP873X_REG_GPO_CTRL,
        BIT(offset.wrapping_mul(BITS_PER_GPO)),
        if value != 0 { BIT(offset.wrapping_mul(BITS_PER_GPO)) } else { 0 },
    )
}

unsafe fn lp873x_gpio_get(chip: *mut GpioChip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip) as *mut Lp873xGpio;
    let mut val: i32 = 0;

    let ret = regmap_read((*(*gpio).lp873).regmap, LP873X_REG_GPO_CTRL, &mut val);
    if ret < 0 {
        return ret;
    }

    if (val & BIT(offset.wrapping_mul(BITS_PER_GPO))) != 0 { 1 } else { 0 }
}

unsafe fn lp873x_gpio_set(chip: *mut GpioChip, offset: u32, value: i32) {
    let gpio = gpiochip_get_data(chip) as *mut Lp873xGpio;

    let _ = regmap_update_bits(
        (*(*gpio).lp873).regmap,
        LP873X_REG_GPO_CTRL,
        BIT(offset.wrapping_mul(BITS_PER_GPO)),
        if value != 0 { BIT(offset.wrapping_mul(BITS_PER_GPO)) } else { 0 },
    );
}

unsafe fn lp873x_gpio_request(gc: *mut GpioChip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut Lp873xGpio;
    let ret: i32;

    match offset {
        0 => {
            // No MUX Set up Needed for GPO
        }
        1 => {
            // Setup the CLKIN_PIN_SEL MUX to GPO2
            ret = regmap_update_bits(
                (*(*gpio).lp873).regmap,
                LP873X_REG_CONFIG,
                LP873X_CONFIG_CLKIN_PIN_SEL,
                0,
            );
            if ret != 0 {
                return ret;
            }
        }
        _ => return -EINVAL,
    }

    0
}

unsafe fn lp873x_gpio_set_config(
    gc: *mut GpioChip,
    offset: u32,
    config: c_ulong,
) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut Lp873xGpio;
    let bit = BIT(offset.wrapping_mul(BITS_PER_GPO).wrapping_add(LP873X_GPO_CTRL_OD));

    match pinconf_to_config_param(config) {
        PIN_CONFIG_DRIVE_OPEN_DRAIN => regmap_update_bits(
            (*(*gpio).lp873).regmap,
            LP873X_REG_GPO_CTRL,
            bit,
            bit,
        ),
        PIN_CONFIG_DRIVE_PUSH_PULL => regmap_update_bits(
            (*(*gpio).lp873).regmap,
            LP873X_REG_GPO_CTRL,
            bit,
            0,
        ),
        _ => -ENOTSUPP,
    }
}

static mut TEMPLATE_CHIP: GpioChip = GpioChip {
    label: "lp873x-gpio\0".as_ptr() as *const c_char,
    owner: THIS_MODULE,
    request: Some(lp873x_gpio_request),
    get_direction: Some(lp873x_gpio_get_direction),
    direction_input: Some(lp873x_gpio_direction_input),
    direction_output: Some(lp873x_gpio_direction_output),
    get: Some(lp873x_gpio_get),
    set: Some(lp873x_gpio_set),
    set_config: Some(lp873x_gpio_set_config),
    base: -1,
    ngpio: 2,
    can_sleep: true,
    ..GpioChip::ZERO
};

unsafe fn lp873x_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let gpio = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Lp873xGpio>(), GFP_KERNEL)
        as *mut Lp873xGpio;
    if gpio.is_null() {
        return -ENOMEM;
    }

    platform_set_drvdata(pdev, gpio as *mut c_void);

    (*gpio).lp873 = dev_get_drvdata((*pdev).dev.parent) as *mut Lp873x;
    (*gpio).chip = TEMPLATE_CHIP;
    (*gpio).chip.parent = (*(*gpio).lp873).dev;

    let ret = devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*gpio).chip, gpio as *mut c_void);
    if ret < 0 {
        dev_err(&mut (*pdev).dev, "Could not register gpiochip, %d\n", ret);
        return ret;
    }

    0
}

static LP873X_GPIO_ID_TABLE: &[PlatformDeviceId] = &[
    PlatformDeviceId { name: "lp873x-gpio\0" },
    PlatformDeviceId { name: "\0" },
];

static mut LP873X_GPIO_DRIVER: PlatformDriver = PlatformDriver {
    driver: Driver { name: "lp873x-gpio\0" },
    probe: Some(lp873x_gpio_probe),
    id_table: LP873X_GPIO_ID_TABLE.as_ptr(),
};

// module_platform_driver!(LP873X_GPIO_DRIVER);
// MODULE_DEVICE_TABLE(platform, LP873X_GPIO_ID_TABLE);
// MODULE_AUTHOR!("Keerthy <j-keerthy@ti.com>");
// MODULE_DESCRIPTION!("LP873X GPIO driver");
// MODULE_LICENSE!("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
