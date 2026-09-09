// SPDX-License-Identifier: GPL-2.0
/*
 * Support to GPOs on ROHM BD71815
 * Copyright 2021 ROHM Semiconductors.
 * Author: Matti Vaittinen <matti.vaittinen@fi.rohmeurope.com>
 *
 * Copyright 2014 Embest Technology Co. Ltd. Inc.
 * Author: yanglsh@embest-tech.com
 */

// Linux kernel dependencies supplied by other translation units.

#[repr(C)]
struct bd71815_gpio {
    // chip.parent points the MFD which provides DT node and regmap
    chip: gpio_chip,
    // dev points to the platform device for devm and prints
    dev: *mut device,
    regmap: *mut regmap,
}

unsafe fn bd71815gpo_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let bd71815: *mut bd71815_gpio = gpiochip_get_data(chip);
    let mut val: i32 = 0;
    let ret = regmap_read((*bd71815).regmap, BD71815_REG_GPO, &mut val);
    if ret != 0 {
        return ret;
    }
    return (val >> offset) & 1;
}

unsafe fn bd71815gpo_set(chip: *mut gpio_chip, offset: u32, value: i32) {
    let bd71815: *mut bd71815_gpio = gpiochip_get_data(chip);
    let bit = BIT(offset);

    if value != 0 {
        regmap_set_bits((*bd71815).regmap, BD71815_REG_GPO, bit);
        return;
    }
    regmap_clear_bits((*bd71815).regmap, BD71815_REG_GPO, bit);
}

unsafe fn bd71815_gpio_set_config(
    chip: *mut gpio_chip,
    offset: u32,
    config: u64,
) -> i32 {
    let bdgpio: *mut bd71815_gpio = gpiochip_get_data(chip);

    match pinconf_to_config_param(config) {
        PIN_CONFIG_DRIVE_OPEN_DRAIN => regmap_update_bits(
            (*bdgpio).regmap,
            BD71815_REG_GPO,
            BD71815_GPIO_DRIVE_MASK << offset,
            BD71815_GPIO_OPEN_DRAIN << offset,
        ),
        PIN_CONFIG_DRIVE_PUSH_PULL => regmap_update_bits(
            (*bdgpio).regmap,
            BD71815_REG_GPO,
            BD71815_GPIO_DRIVE_MASK << offset,
            BD71815_GPIO_CMOS << offset,
        ),
        _ => -ENOTSUPP,
    }
}

/* BD71815 GPIO is actually GPO */
unsafe fn bd71815gpo_direction_get(_gc: *mut gpio_chip, _offset: u32) -> i32 {
    GPIO_LINE_DIRECTION_OUT
}

/* Template for GPIO chip */
static mut bd71815gpo_chip: gpio_chip = gpio_chip {
    label: "bd71815",
    owner: THIS_MODULE,
    get: Some(bd71815gpo_get),
    get_direction: Some(bd71815gpo_direction_get),
    set: Some(bd71815gpo_set),
    set_config: Some(bd71815_gpio_set_config),
    can_sleep: true,
    ..gpio_chip::ZERO
};

const BD71815_TWO_GPIOS: u32 = GENMASK(1, 0);
const BD71815_ONE_GPIO: u32 = BIT(0);

/*
 * Sigh. The BD71815 and BD71817 were originally designed to support two GPO
 * pins. At some point it was noticed the second GPO pin which is the E5 pin
 * located at the center of IC is hard to use on PCB (due to the location). It
 * was decided to not promote this second GPO and the pin is marked as GND in
 * the datasheet. The functionality is still there though! I guess driving a GPO
 * connected to the ground is a bad idea. Thus we do not support it by default.
 * OTOH - the original driver written by colleagues at Embest did support
 * controlling this second GPO. It is thus possible this is used in some of the
 * products.
 *
 * This driver does not by default support configuring this second GPO
 * but allows using it by providing the DT property
 * "rohm,enable-hidden-gpo".
 */
unsafe fn bd71815_init_valid_mask(
    gc: *mut gpio_chip,
    valid_mask: *mut u64,
    ngpios: u32,
) -> i32 {
    if ngpios != 2 {
        return 0;
    }

    if !(*gc).parent.is_null()
        && device_property_present((*gc).parent, "rohm,enable-hidden-gpo")
    {
        *valid_mask = BD71815_TWO_GPIOS as u64;
    } else {
        *valid_mask = BD71815_ONE_GPIO as u64;
    }
    0
}

unsafe fn gpo_bd71815_probe(pdev: *mut platform_device) -> i32 {
    let mut g: *mut bd71815_gpio;
    let parent: *mut device;
    let dev: *mut device;

    /*
     * Bind devm lifetime to this platform device => use dev for devm.
     * also the prints should originate from this device.
     */
    dev = &mut (*pdev).dev;
    /* The device-tree and regmap come from MFD => use parent for that */
    parent = (*dev).parent;

    g = devm_kzalloc(dev, core::mem::size_of::<bd71815_gpio>(), GFP_KERNEL)
        as *mut bd71815_gpio;
    if g.is_null() {
        return -ENOMEM;
    }

    (*g).chip = bd71815gpo_chip;

    /*
     * FIXME: As writing of this the sysfs interface for GPIO control does
     * not respect the valid_mask. Do not trust it but rather set the ngpios
     * to 1 if "rohm,enable-hidden-gpo" is not given.
     *
     * This check can be removed later if the sysfs export is fixed and
     * if the fix is backported.
     *
     * For now it is safest to just set the ngpios though.
     */
    if device_property_present(parent, "rohm,enable-hidden-gpo") {
        (*g).chip.ngpio = 2;
    } else {
        (*g).chip.ngpio = 1;
    }

    (*g).chip.init_valid_mask = Some(bd71815_init_valid_mask);
    (*g).chip.base = -1;
    (*g).chip.parent = parent;
    (*g).regmap = dev_get_regmap(parent, core::ptr::null());
    (*g).dev = dev;

    devm_gpiochip_add_data(dev, &mut (*g).chip, g)
}

static mut gpo_bd71815_driver: platform_driver = platform_driver {
    driver: driver {
        name: "bd71815-gpo",
        ..driver::ZERO
    },
    probe: Some(gpo_bd71815_probe),
    ..platform_driver::ZERO
};

// module_platform_driver(gpo_bd71815_driver);
// MODULE_ALIAS("platform:bd71815-gpo");
// MODULE_AUTHOR("Matti Vaittinen <matti.vaittinen@fi.rohmeurope.com>");
// MODULE_AUTHOR("Peter Yang <yanglsh@embest-tech.com>");
// MODULE_DESCRIPTION("GPO interface for BD71815");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
