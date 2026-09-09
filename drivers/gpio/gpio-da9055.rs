// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GPIO Driver for Dialog DA9055 PMICs.
 *
 * Copyright(c) 2012 Dialog Semiconductor Ltd.
 *
 * Author: David Dajun Chen <dchen@diasemi.com>
 */

// External Linux kernel types, functions, constants, and registration macros
// referenced by this translation are supplied by the surrounding kernel bindings.

const DA9055_VDD_IO: u32 = 0x0;
const DA9055_PUSH_PULL: u32 = 0x3;
const DA9055_ACT_LOW: u32 = 0x0;
const DA9055_GPI: u32 = 0x1;
const DA9055_PORT_MASK: u32 = 0x3;

#[inline]
const fn da9055_port_shift(offset: u32) -> u32 {
    4 * (offset % 2)
}

const DA9055_INPUT: u32 = DA9055_GPI;
const DA9055_OUTPUT: u32 = DA9055_PUSH_PULL;
const DA9055_IRQ_GPI0: u32 = 3;

#[repr(C)]
struct da9055_gpio {
    da9055: *mut da9055,
    gp: gpio_chip,
}

unsafe fn da9055_gpio_get(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut da9055_gpio;
    let mut gpio_direction: i32 = 0;
    let mut ret: i32;

    /* Get GPIO direction */
    ret = da9055_reg_read(
        (*gpio).da9055,
        (offset >> 1) + DA9055_REG_GPIO0_1,
    );
    if ret < 0 {
        return ret;
    }

    gpio_direction = ret & ((DA9055_PORT_MASK << da9055_port_shift(offset)) as i32);
    gpio_direction >>= da9055_port_shift(offset);
    match gpio_direction as u32 {
        DA9055_INPUT => {
            ret = da9055_reg_read((*gpio).da9055, DA9055_REG_STATUS_B);
            if ret < 0 {
                return ret;
            }
        }
        DA9055_OUTPUT => {
            ret = da9055_reg_read((*gpio).da9055, DA9055_REG_GPIO_MODE0_2);
            if ret < 0 {
                return ret;
            }
        }
        _ => {}
    }

    if (ret & (1i32 << offset)) != 0 { 1 } else { 0 }
}

unsafe fn da9055_gpio_set(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut da9055_gpio;

    da9055_reg_update(
        (*gpio).da9055,
        DA9055_REG_GPIO_MODE0_2,
        1i32 << offset,
        value << offset,
    )
}

unsafe fn da9055_gpio_direction_input(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut da9055_gpio;
    let reg_byte: u32 = (DA9055_ACT_LOW | DA9055_GPI) << da9055_port_shift(offset);

    da9055_reg_update(
        (*gpio).da9055,
        (offset >> 1) + DA9055_REG_GPIO0_1,
        (DA9055_PORT_MASK << da9055_port_shift(offset)) as i32,
        reg_byte as i32,
    )
}

unsafe fn da9055_gpio_direction_output(
    gc: *mut gpio_chip,
    offset: u32,
    value: i32,
) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut da9055_gpio;
    let reg_byte: u32 = (DA9055_VDD_IO | DA9055_PUSH_PULL) << da9055_port_shift(offset);

    let ret = da9055_reg_update(
        (*gpio).da9055,
        (offset >> 1) + DA9055_REG_GPIO0_1,
        (DA9055_PORT_MASK << da9055_port_shift(offset)) as i32,
        reg_byte as i32,
    );
    if ret < 0 {
        return ret;
    }

    da9055_gpio_set(gc, offset, value)
}

unsafe fn da9055_gpio_to_irq(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut da9055_gpio;
    let da9055 = (*gpio).da9055;

    regmap_irq_get_virq((*da9055).irq_data, DA9055_IRQ_GPI0 + offset)
}

#[repr(C)]
static reference_gp: gpio_chip = gpio_chip {
    label: "da9055-gpio",
    owner: THIS_MODULE,
    get: Some(da9055_gpio_get),
    set: Some(da9055_gpio_set),
    direction_input: Some(da9055_gpio_direction_input),
    direction_output: Some(da9055_gpio_direction_output),
    to_irq: Some(da9055_gpio_to_irq),
    can_sleep: true,
    ngpio: 3,
    base: -1,
};

unsafe fn da9055_gpio_probe(pdev: *mut platform_device) -> i32 {
    let gpio = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<da9055_gpio>(),
        GFP_KERNEL,
    ) as *mut da9055_gpio;
    if gpio.is_null() {
        return -ENOMEM;
    }

    (*gpio).da9055 = dev_get_drvdata((*pdev).dev.parent);
    let pdata = dev_get_platdata((*(*gpio).da9055).dev);

    (*gpio).gp = reference_gp;
    if !pdata.is_null() && (*pdata).gpio_base != 0 {
        (*gpio).gp.base = (*pdata).gpio_base;
    }

    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*gpio).gp, gpio)
}

#[repr(C)]
static mut da9055_gpio_driver: platform_driver = platform_driver {
    probe: Some(da9055_gpio_probe),
    driver: driver {
        name: "da9055-gpio",
    },
};

unsafe fn da9055_gpio_init() -> i32 {
    platform_driver_register(&mut da9055_gpio_driver)
}

unsafe fn da9055_gpio_exit() {
    platform_driver_unregister(&mut da9055_gpio_driver);
}

// subsys_initcall(da9055_gpio_init);
// module_exit(da9055_gpio_exit);
// MODULE_AUTHOR("David Dajun Chen <dchen@diasemi.com>");
// MODULE_DESCRIPTION("DA9055 GPIO Device Driver");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:da9055-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
