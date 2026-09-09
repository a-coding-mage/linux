// SPDX-License-Identifier: GPL-2.0+
/*
 * Access to GPOs on TWL6040 chip
 *
 * Copyright (C) 2012 Texas Instruments, Inc.
 *
 * Authors:
 *	Sergio Aguirre <saaguirre@ti.com>
 *	Peter Ujfalusi <peter.ujfalusi@ti.com>
 */

// Dependencies supplied by the Linux kernel and the TWL6040 MFD subsystem.

unsafe fn twl6040gpo_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let twl6040 = gpiochip_get_data(chip);
    let mut ret: i32 = 0;

    ret = twl6040_reg_read(twl6040, TWL6040_REG_GPOCTL);
    if ret < 0 {
        return ret;
    }

    return if (ret & (1_i32.wrapping_shl(offset))) != 0 { 1 } else { 0 };
}

unsafe fn twl6040gpo_get_direction(_chip: *mut gpio_chip, _offset: u32) -> i32 {
    return GPIO_LINE_DIRECTION_OUT;
}

unsafe fn twl6040gpo_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let twl6040 = gpiochip_get_data(chip);
    let ret: i32;
    let gpoctl: u8;

    ret = twl6040_reg_read(twl6040, TWL6040_REG_GPOCTL);
    if ret < 0 {
        return ret;
    }

    if value != 0 {
        gpoctl = (ret | (1_i32.wrapping_shl(offset))) as u8;
    } else {
        gpoctl = (ret & !(1_i32.wrapping_shl(offset))) as u8;
    }

    return twl6040_reg_write(twl6040, TWL6040_REG_GPOCTL, gpoctl);
}

unsafe fn twl6040gpo_direction_out(
    chip: *mut gpio_chip,
    offset: u32,
    value: i32,
) -> i32 {
    /* This only drives GPOs, and can't change direction */
    return twl6040gpo_set(chip, offset, value);
}

static mut twl6040gpo_chip: gpio_chip = gpio_chip {
    label: "twl6040",
    owner: THIS_MODULE,
    get: Some(twl6040gpo_get),
    direction_output: Some(twl6040gpo_direction_out),
    get_direction: Some(twl6040gpo_get_direction),
    set: Some(twl6040gpo_set),
    can_sleep: true,
    ..gpio_chip::default()
};

/*----------------------------------------------------------------------*/

unsafe fn gpo_twl6040_probe(pdev: *mut platform_device) -> i32 {
    let twl6040_core_dev = (*pdev).dev.parent;
    let twl6040 = dev_get_drvdata(twl6040_core_dev);
    let ret: i32;

    device_set_node(&mut (*pdev).dev, dev_fwnode((*pdev).dev.parent));

    twl6040gpo_chip.base = -1;

    if twl6040_get_revid(twl6040) < TWL6041_REV_ES2_0 {
        twl6040gpo_chip.ngpio = 3; /* twl6040 have 3 GPO */
    } else {
        twl6040gpo_chip.ngpio = 1; /* twl6041 have 1 GPO */
    }

    twl6040gpo_chip.parent = &mut (*pdev).dev;

    ret = devm_gpiochip_add_data(&mut (*pdev).dev, &mut twl6040gpo_chip, twl6040);
    if ret < 0 {
        dev_err(&mut (*pdev).dev, "could not register gpiochip, %d\n", ret);
        twl6040gpo_chip.ngpio = 0;
    }

    return ret;
}

/* Note:  this hardware lives inside an I2C-based multi-function device. */
module_alias!("platform:twl6040-gpo");

static mut gpo_twl6040_driver: platform_driver = platform_driver {
    driver: driver {
        name: "twl6040-gpo",
        ..driver::default()
    },
    probe: Some(gpo_twl6040_probe),
    ..platform_driver::default()
};

module_platform_driver!(gpo_twl6040_driver);

module_author!("Texas Instruments, Inc.");
module_description!("GPO interface for TWL6040");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
