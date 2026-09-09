// SPDX-License-Identifier: GPL-2.0
/*
 * TI TPS6586x GPIO driver
 *
 * Copyright (c) 2012, NVIDIA CORPORATION.  All rights reserved.
 * Author: Laxman dewangan <ldewangan@nvidia.com>
 *
 * Based on tps6586x.c
 * Copyright (c) 2010 CompuLab Ltd.
 * Mike Rapoport <mike@compulab.co.il>
 */

// Dependencies supplied by the Linux kernel bindings.
use crate::{
    device, dev_fwnode, dev_get_platdata, devm_gpiochip_add_data, devm_kzalloc,
    device_set_node, gpiochip_get_data, platform_driver_register, tps6586x_irq_get_virq,
    tps6586x_read, tps6586x_update, Device, GpioChip, PlatformDevice, PlatformDriver,
    Tps6586xPlatformData, GFP_KERNEL, THIS_MODULE,
};

/* GPIO control registers */
const TPS6586X_GPIOSET1: u8 = 0x5d;
const TPS6586X_GPIOSET2: u8 = 0x5e;

#[repr(C)]
pub struct Tps6586xGpio {
    pub gpio_chip: GpioChip,
    pub parent: *mut Device,
}

pub unsafe extern "C" fn tps6586x_gpio_get(gc: *mut GpioChip, offset: u32) -> i32 {
    let tps6586x_gpio = gpiochip_get_data(gc) as *mut Tps6586xGpio;
    let mut val: u8 = 0;
    let ret = tps6586x_read((*tps6586x_gpio).parent, TPS6586X_GPIOSET2, &mut val);
    if ret != 0 {
        return ret;
    }
    ((val & (1u8 << offset)) != 0) as i32
}

pub unsafe extern "C" fn tps6586x_gpio_set(
    gc: *mut GpioChip,
    offset: u32,
    value: i32,
) -> i32 {
    let tps6586x_gpio = gpiochip_get_data(gc) as *mut Tps6586xGpio;
    tps6586x_update(
        (*tps6586x_gpio).parent,
        TPS6586X_GPIOSET2,
        value.wrapping_shl(offset),
        1i32.wrapping_shl(offset),
    )
}

pub unsafe extern "C" fn tps6586x_gpio_output(
    gc: *mut GpioChip,
    offset: u32,
    value: i32,
) -> i32 {
    let tps6586x_gpio = gpiochip_get_data(gc) as *mut Tps6586xGpio;
    let ret = tps6586x_gpio_set(gc, offset, value);
    if ret != 0 {
        return ret;
    }

    let val: u8 = 0x1u8.wrapping_shl(offset * 2);
    let mask: u8 = 0x3u8.wrapping_shl(offset * 2);

    tps6586x_update(
        (*tps6586x_gpio).parent,
        TPS6586X_GPIOSET1,
        val as i32,
        mask as i32,
    )
}

pub unsafe extern "C" fn tps6586x_gpio_to_irq(
    gc: *mut GpioChip,
    offset: u32,
) -> i32 {
    let tps6586x_gpio = gpiochip_get_data(gc) as *mut Tps6586xGpio;
    tps6586x_irq_get_virq((*tps6586x_gpio).parent, TPS6586X_INT_PLDO_0 + offset)
}

pub unsafe extern "C" fn tps6586x_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let pdata: *mut Tps6586xPlatformData;
    let tps6586x_gpio: *mut Tps6586xGpio;

    device_set_node(&mut (*pdev).dev, dev_fwnode((*pdev).dev.parent));

    pdata = dev_get_platdata((*pdev).dev.parent);
    tps6586x_gpio = devm_kzalloc(
        &mut (*pdev).dev,
        core::mem::size_of::<Tps6586xGpio>(),
        GFP_KERNEL,
    ) as *mut Tps6586xGpio;
    if tps6586x_gpio.is_null() {
        return -12; // -ENOMEM
    }

    (*tps6586x_gpio).parent = (*pdev).dev.parent;
    (*tps6586x_gpio).gpio_chip.owner = THIS_MODULE;
    (*tps6586x_gpio).gpio_chip.label = (*pdev).name;
    (*tps6586x_gpio).gpio_chip.parent = &mut (*pdev).dev;
    (*tps6586x_gpio).gpio_chip.ngpio = 4;
    (*tps6586x_gpio).gpio_chip.can_sleep = true;

    /* FIXME: add handling of GPIOs as dedicated inputs */
    (*tps6586x_gpio).gpio_chip.direction_output = Some(tps6586x_gpio_output);
    (*tps6586x_gpio).gpio_chip.set = Some(tps6586x_gpio_set);
    (*tps6586x_gpio).gpio_chip.get = Some(tps6586x_gpio_get);
    (*tps6586x_gpio).gpio_chip.to_irq = Some(tps6586x_gpio_to_irq);

    if !pdata.is_null() && (*pdata).gpio_base != 0 {
        (*tps6586x_gpio).gpio_chip.base = (*pdata).gpio_base;
    } else {
        (*tps6586x_gpio).gpio_chip.base = -1;
    }

    devm_gpiochip_add_data(
        &mut (*pdev).dev,
        &mut (*tps6586x_gpio).gpio_chip,
        tps6586x_gpio as *mut core::ffi::c_void,
    )
}

pub static mut tps6586x_gpio_driver: PlatformDriver = PlatformDriver {
    driver: device::Driver {
        name: "tps6586x-gpio",
    },
    probe: Some(tps6586x_gpio_probe),
};

pub unsafe extern "C" fn tps6586x_gpio_init() -> i32 {
    platform_driver_register(&mut tps6586x_gpio_driver)
}

// subsys_initcall(tps6586x_gpio_init);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
