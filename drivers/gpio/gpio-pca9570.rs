// SPDX-License-Identifier: GPL-2.0-only
/*
 * Driver for PCA9570 I2C GPO expander
 *
 * Copyright (C) 2020 Sungbo Eo <mans0n@gorani.run>
 *
 * Based on gpio-tpic2810.c
 * Copyright (C) 2015 Texas Instruments Incorporated - http://www.ti.com/
 *	Andrew F. Davis <afd@ti.com>
 */

// C dependencies: linux/bits.h, linux/cleanup.h, linux/device/devres.h,
// linux/errno.h, linux/gpio/driver.h, linux/i2c.h, linux/module.h,
// linux/mutex.h, linux/property.h, linux/types.h

const SLG7XL45106_GPO_REG: u32 = 0xDB;

/**
 * struct pca9570_chip_data - GPIO platformdata
 * @ngpio: no of gpios
 * @command: Command to be sent
 */
#[repr(C)]
pub struct pca9570_chip_data {
    pub ngpio: u16,
    pub command: u32,
}

/**
 * struct pca9570 - GPIO driver data
 * @chip: GPIO controller chip
 * @chip_data: GPIO controller platform data
 * @lock: Protects write sequences
 * @out: Buffer for device register
 */
#[repr(C)]
pub struct pca9570 {
    pub chip: gpio_chip,
    pub chip_data: *const pca9570_chip_data,
    pub lock: mutex,
    pub out: u8,
}

unsafe fn pca9570_read(gpio: *mut pca9570, value: *mut u8) -> i32 {
    let client: *mut i2c_client = to_i2c_client((*(*gpio).chip.parent).cast());
    let ret: i32;

    if (*(*gpio).chip_data).command != 0 {
        ret = i2c_smbus_read_byte_data(client, (*(*gpio).chip_data).command as u8);
    } else {
        ret = i2c_smbus_read_byte(client);
    }

    if ret < 0 {
        return ret;
    }

    *value = ret as u8;
    0
}

unsafe fn pca9570_write(gpio: *mut pca9570, value: u8) -> i32 {
    let client: *mut i2c_client = to_i2c_client((*(*gpio).chip.parent).cast());

    if (*(*gpio).chip_data).command != 0 {
        return i2c_smbus_write_byte_data(client, (*(*gpio).chip_data).command as u8, value);
    }

    i2c_smbus_write_byte(client, value)
}

unsafe extern "C" fn pca9570_get_direction(_chip: *mut gpio_chip, _offset: u32) -> i32 {
    // This device always output
    GPIO_LINE_DIRECTION_OUT
}

unsafe extern "C" fn pca9570_get(chip: *mut gpio_chip, offset: u32) -> i32 {
    let gpio: *mut pca9570 = gpiochip_get_data(chip);
    let mut buffer: u8 = 0;
    let ret: i32 = pca9570_read(gpio, &mut buffer);
    if ret != 0 {
        return ret;
    }

    ((buffer & (1u8 << offset)) != 0) as i32
}

unsafe extern "C" fn pca9570_set(chip: *mut gpio_chip, offset: u32, value: i32) {
    let gpio: *mut pca9570 = gpiochip_get_data(chip);
    let mut buffer: u8;

    // guard(mutex)(&gpio->lock);
    mutex_lock(&mut (*gpio).lock);

    buffer = (*gpio).out;
    if value != 0 {
        buffer |= 1u8 << offset;
    } else {
        buffer &= !(1u8 << offset);
    }

    let ret: i32 = pca9570_write(gpio, buffer);
    if ret == 0 {
        (*gpio).out = buffer;
    }

    mutex_unlock(&mut (*gpio).lock);
}

unsafe extern "C" fn pca9570_probe(client: *mut i2c_client) -> i32 {
    let dev: *mut device = &mut (*client).dev;
    let gpio: *mut pca9570 = devm_kzalloc(dev, core::mem::size_of::<pca9570>(), GFP_KERNEL)
        as *mut pca9570;
    if gpio.is_null() {
        return -ENOMEM;
    }

    (*gpio).chip.label = (*client).name;
    (*gpio).chip.parent = dev;
    (*gpio).chip.owner = THIS_MODULE;
    (*gpio).chip.get_direction = Some(pca9570_get_direction);
    (*gpio).chip.get = Some(pca9570_get);
    (*gpio).chip.set = Some(pca9570_set);
    (*gpio).chip.base = -1;
    (*gpio).chip_data = i2c_get_match_data(client);
    (*gpio).chip.ngpio = (*(*gpio).chip_data).ngpio;
    (*gpio).chip.can_sleep = true;

    let ret: i32 = devm_mutex_init(dev, &mut (*gpio).lock);
    if ret != 0 {
        return ret;
    }

    // Read the current output level
    pca9570_read(gpio, &mut (*gpio).out);

    i2c_set_clientdata(client, gpio.cast());
    devm_gpiochip_add_data(dev, &mut (*gpio).chip, gpio.cast())
}

static pca9570_gpio: pca9570_chip_data = pca9570_chip_data { ngpio: 4, command: 0 };
static pca9571_gpio: pca9570_chip_data = pca9570_chip_data { ngpio: 8, command: 0 };
static slg7xl45106_gpio: pca9570_chip_data = pca9570_chip_data {
    ngpio: 8,
    command: SLG7XL45106_GPO_REG,
};

// i2c_device_id table: pca9570, pca9571, slg7xl45106; followed by a sentinel.
static pca9570_id_table: [i2c_device_id; 4] = [
    i2c_device_id { name: "pca9570", driver_data: &pca9570_gpio as *const _ as kernel_ulong_t },
    i2c_device_id { name: "pca9571", driver_data: &pca9571_gpio as *const _ as kernel_ulong_t },
    i2c_device_id { name: "slg7xl45106", driver_data: &slg7xl45106_gpio as *const _ as kernel_ulong_t },
    i2c_device_id::sentinel(),
];

// of_device_id table: dlg,slg7xl45106; nxp,pca9570; nxp,pca9571; followed by a sentinel.
static pca9570_of_match_table: [of_device_id; 4] = [
    of_device_id { compatible: "dlg,slg7xl45106", data: &slg7xl45106_gpio },
    of_device_id { compatible: "nxp,pca9570", data: &pca9570_gpio },
    of_device_id { compatible: "nxp,pca9571", data: &pca9571_gpio },
    of_device_id::sentinel(),
];

static mut pca9570_driver: i2c_driver = i2c_driver {
    driver: device_driver {
        name: "pca9570",
        of_match_table: pca9570_of_match_table.as_ptr(),
    },
    probe: Some(pca9570_probe),
    id_table: pca9570_id_table.as_ptr(),
};

// MODULE_DEVICE_TABLE(i2c, pca9570_id_table);
// MODULE_DEVICE_TABLE(of, pca9570_of_match_table);
// module_i2c_driver(pca9570_driver);
// MODULE_AUTHOR("Sungbo Eo <mans0n@gorani.run>");
// MODULE_DESCRIPTION("GPIO expander driver for PCA9570");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
