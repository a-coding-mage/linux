// SPDX-License-Identifier: GPL-2.0
/*
 * Driver for the ps-mode pin configuration.
 *
 * Copyright (c) 2021 Xilinx, Inc.
 */

// Dependencies supplied by the Linux kernel bindings:
// linux/delay.h, linux/err.h, linux/gpio/driver.h, linux/io.h,
// linux/kernel.h, linux/module.h, linux/platform_device.h, linux/slab.h,
// linux/firmware/xlnx-zynqmp.h

/* 4-bit boot mode pins */
const MODE_PINS: u32 = 4;

extern "C" {
    fn zynqmp_pm_bootmode_read(value: *mut u32) -> i32;
    fn zynqmp_pm_bootmode_write(value: u32) -> i32;
}

/**
 * modepin_gpio_get_value - Get the state of the specified pin of GPIO device
 * @chip: gpio_chip instance to be worked on
 * @pin: gpio pin number within the device
 *
 * This function reads the state of the specified pin of the GPIO device.
 *
 * Return: 0 if the pin is low, 1 if pin is high, -EINVAL wrong pin configured
 *         or error value.
 */
unsafe extern "C" fn modepin_gpio_get_value(
    _chip: *mut gpio_chip,
    pin: u32,
) -> i32 {
    let mut regval: u32 = 0;
    let ret = zynqmp_pm_bootmode_read(&mut regval);
    if ret != 0 {
        return ret;
    }

    /* When [0:3] corresponding bit is set, then read output bit [8:11],
     * if the bit is clear then read input bit [4:7] for status or value.
     */
    if (regval & (1u32.wrapping_shl(pin))) != 0 {
        ((regval & (1u32.wrapping_shl(pin.wrapping_add(8)))) != 0) as i32
    } else {
        ((regval & (1u32.wrapping_shl(pin.wrapping_add(4)))) != 0) as i32
    }
}

/**
 * modepin_gpio_set_value - Modify the state of the pin with specified value
 * @chip: gpio_chip instance to be worked on
 * @pin: gpio pin number within the device
 * @state: value used to modify the state of the specified pin
 *
 * This function reads the state of the specified pin of the GPIO device, mask
 * with the capture state of GPIO pin, and update pin of GPIO device.
 *
 * Return: None.
 */
unsafe extern "C" fn modepin_gpio_set_value(
    _chip: *mut gpio_chip,
    pin: u32,
    state: i32,
) -> i32 {
    let mut bootpin_val: u32 = 0;
    zynqmp_pm_bootmode_read(&mut bootpin_val);

    /* Configure pin as an output by set bit [0:3] */
    bootpin_val |= 1u32.wrapping_shl(pin);

    if state != 0 {
        bootpin_val |= 1u32.wrapping_shl(pin.wrapping_add(8));
    } else {
        bootpin_val &= !(1u32.wrapping_shl(pin.wrapping_add(8)));
    }

    /* Configure bootpin value */
    let ret = zynqmp_pm_bootmode_write(bootpin_val);
    if ret != 0 {
        // pr_err("modepin: set value error %d for pin %d\n", ret, pin);
    }

    ret
}

/**
 * modepin_gpio_dir_in - Set the direction of the specified GPIO pin as input
 * @chip: gpio_chip instance to be worked on
 * @pin: gpio pin number within the device
 *
 * Return: 0 always
 */
unsafe extern "C" fn modepin_gpio_dir_in(
    _chip: *mut gpio_chip,
    _pin: u32,
) -> i32 {
    0
}

/**
 * modepin_gpio_dir_out - Set the direction of the specified GPIO pin as output
 * @chip: gpio_chip instance to be worked on
 * @pin: gpio pin number within the device
 * @state: value to be written to specified pin
 *
 * Return: 0 always
 */
unsafe extern "C" fn modepin_gpio_dir_out(
    chip: *mut gpio_chip,
    pin: u32,
    state: i32,
) -> i32 {
    modepin_gpio_set_value(chip, pin, state)
}

/**
 * modepin_gpio_probe - Initialization method for modepin_gpio
 * @pdev: platform device instance
 *
 * Return: 0 on success, negative error otherwise.
 */
unsafe extern "C" fn modepin_gpio_probe(pdev: *mut platform_device) -> i32 {
    let mut chip: *mut gpio_chip;
    let status: i32;

    chip = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<gpio_chip>(), GFP_KERNEL);
    if chip.is_null() {
        return -12; // -ENOMEM
    }

    platform_set_drvdata(pdev, chip as *mut core::ffi::c_void);

    /* configure the gpio chip */
    (*chip).base = -1;
    (*chip).ngpio = MODE_PINS;
    (*chip).owner = THIS_MODULE;
    (*chip).parent = &mut (*pdev).dev;
    (*chip).get = Some(modepin_gpio_get_value);
    (*chip).set = Some(modepin_gpio_set_value);
    (*chip).direction_input = Some(modepin_gpio_dir_in);
    (*chip).direction_output = Some(modepin_gpio_dir_out);
    (*chip).label = dev_name(&(*pdev).dev);

    /* modepin gpio registration */
    status = devm_gpiochip_add_data(&mut (*pdev).dev, chip, chip as *mut core::ffi::c_void);
    if status != 0 {
        return dev_err_probe(&mut (*pdev).dev, status, "Failed to add GPIO chip\n");
    }

    status
}

// Device-table and module-driver declarations corresponding to the C macros:
// modepin_platform_id: compatible = "xlnx,zynqmp-gpio-modepin"
// modepin_platform_driver: name = "modepin-gpio", probe = modepin_gpio_probe
// MODULE_DEVICE_TABLE(of, modepin_platform_id)
// module_platform_driver(modepin_platform_driver)
// MODULE_AUTHOR("Piyush Mehta <piyush.mehta@xilinx.com>")
// MODULE_DESCRIPTION("ZynqMP Boot PS_MODE Configuration")
// MODULE_LICENSE("GPL v2")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
