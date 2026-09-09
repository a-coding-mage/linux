// SPDX-License-Identifier: GPL-2.0
//
// Copyright (C) 2018 BayLibre SAS
// Author: Bartosz Golaszewski <bgolaszewski@baylibre.com>
//
// GPIO driver for MAXIM 77650/77651 charger/power-supply.

const MAX77650_GPIO_DIR_MASK: i32 = 1 << 0;
const MAX77650_GPIO_INVAL_MASK: i32 = 1 << 1;
const MAX77650_GPIO_DRV_MASK: i32 = 1 << 2;
const MAX77650_GPIO_OUTVAL_MASK: i32 = 1 << 3;
const MAX77650_GPIO_DEBOUNCE_MASK: i32 = 1 << 4;

const MAX77650_GPIO_DIR_OUT: i32 = 0x00;
const MAX77650_GPIO_DIR_IN: i32 = 1 << 0;
const MAX77650_GPIO_OUT_LOW: i32 = 0x00;
const MAX77650_GPIO_OUT_HIGH: i32 = 1 << 3;
const MAX77650_GPIO_DRV_OPEN_DRAIN: i32 = 0x00;
const MAX77650_GPIO_DRV_PUSH_PULL: i32 = 1 << 2;
const MAX77650_GPIO_DEBOUNCE: i32 = 1 << 4;

#[inline]
fn max77650_gpio_dir_bits(reg: i32) -> i32 {
    reg & MAX77650_GPIO_DIR_MASK
}

#[inline]
fn max77650_gpio_inval_bits(reg: i32) -> i32 {
    (reg & MAX77650_GPIO_INVAL_MASK) >> 1
}

#[repr(C)]
struct max77650_gpio_chip {
    map: *mut regmap,
    gc: gpio_chip,
    irq: i32,
}

unsafe fn max77650_gpio_direction_input(gc: *mut gpio_chip, _offset: u32) -> i32 {
    let chip = gpiochip_get_data(gc);

    regmap_update_bits(
        (*chip).map,
        MAX77650_REG_CNFG_GPIO,
        MAX77650_GPIO_DIR_MASK,
        MAX77650_GPIO_DIR_IN,
    )
}

unsafe fn max77650_gpio_direction_output(
    gc: *mut gpio_chip,
    _offset: u32,
    value: i32,
) -> i32 {
    let chip = gpiochip_get_data(gc);
    let mask: i32 = MAX77650_GPIO_DIR_MASK | MAX77650_GPIO_OUTVAL_MASK;
    let mut regval: i32 = if value != 0 {
        MAX77650_GPIO_OUT_HIGH
    } else {
        MAX77650_GPIO_OUT_LOW
    };
    regval |= MAX77650_GPIO_DIR_OUT;

    regmap_update_bits((*chip).map, MAX77650_REG_CNFG_GPIO, mask, regval)
}

unsafe fn max77650_gpio_set_value(gc: *mut gpio_chip, _offset: u32, value: i32) -> i32 {
    let chip = gpiochip_get_data(gc);
    let regval: i32 = if value != 0 {
        MAX77650_GPIO_OUT_HIGH
    } else {
        MAX77650_GPIO_OUT_LOW
    };

    regmap_update_bits(
        (*chip).map,
        MAX77650_REG_CNFG_GPIO,
        MAX77650_GPIO_OUTVAL_MASK,
        regval,
    )
}

unsafe fn max77650_gpio_get_value(gc: *mut gpio_chip, _offset: u32) -> i32 {
    let chip = gpiochip_get_data(gc);
    let mut val: u32 = 0;
    let rv = regmap_read((*chip).map, MAX77650_REG_CNFG_GPIO, &mut val);
    if rv != 0 {
        return rv;
    }

    max77650_gpio_inval_bits(val as i32)
}

unsafe fn max77650_gpio_get_direction(gc: *mut gpio_chip, _offset: u32) -> i32 {
    let chip = gpiochip_get_data(gc);
    let mut val: u32 = 0;
    let rv = regmap_read((*chip).map, MAX77650_REG_CNFG_GPIO, &mut val);
    if rv != 0 {
        return rv;
    }

    max77650_gpio_dir_bits(val as i32)
}

unsafe fn max77650_gpio_set_config(
    gc: *mut gpio_chip,
    _offset: u32,
    cfg: c_ulong,
) -> i32 {
    let chip = gpiochip_get_data(gc);

    match pinconf_to_config_param(cfg) {
        PIN_CONFIG_DRIVE_OPEN_DRAIN => regmap_update_bits(
            (*chip).map,
            MAX77650_REG_CNFG_GPIO,
            MAX77650_GPIO_DRV_MASK,
            MAX77650_GPIO_DRV_OPEN_DRAIN,
        ),
        PIN_CONFIG_DRIVE_PUSH_PULL => regmap_update_bits(
            (*chip).map,
            MAX77650_REG_CNFG_GPIO,
            MAX77650_GPIO_DRV_MASK,
            MAX77650_GPIO_DRV_PUSH_PULL,
        ),
        PIN_CONFIG_INPUT_DEBOUNCE => regmap_update_bits(
            (*chip).map,
            MAX77650_REG_CNFG_GPIO,
            MAX77650_GPIO_DEBOUNCE_MASK,
            MAX77650_GPIO_DEBOUNCE,
        ),
        _ => -ENOTSUPP,
    }
}

unsafe fn max77650_gpio_to_irq(gc: *mut gpio_chip, _offset: u32) -> i32 {
    let chip = gpiochip_get_data(gc);
    (*chip).irq
}

unsafe fn max77650_gpio_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let parent = (*dev).parent;
    let i2c = to_i2c_client(parent);

    let chip = devm_kzalloc(dev, core::mem::size_of::<max77650_gpio_chip>(), GFP_KERNEL)
        as *mut max77650_gpio_chip;
    if chip.is_null() {
        return -ENOMEM;
    }

    (*chip).map = dev_get_regmap(parent, core::ptr::null());
    if (*chip).map.is_null() {
        return -ENODEV;
    }

    (*chip).irq = platform_get_irq_byname(pdev, c_str!("GPI"));
    if (*chip).irq < 0 {
        return (*chip).irq;
    }

    (*chip).gc.base = -1;
    (*chip).gc.ngpio = 1;
    (*chip).gc.label = (*i2c).name;
    (*chip).gc.parent = dev;
    (*chip).gc.owner = THIS_MODULE;
    (*chip).gc.can_sleep = true;

    (*chip).gc.direction_input = Some(max77650_gpio_direction_input);
    (*chip).gc.direction_output = Some(max77650_gpio_direction_output);
    (*chip).gc.set = Some(max77650_gpio_set_value);
    (*chip).gc.get = Some(max77650_gpio_get_value);
    (*chip).gc.get_direction = Some(max77650_gpio_get_direction);
    (*chip).gc.set_config = Some(max77650_gpio_set_config);
    (*chip).gc.to_irq = Some(max77650_gpio_to_irq);

    devm_gpiochip_add_data(dev, &mut (*chip).gc, chip as *mut core::ffi::c_void)
}

#[repr(C)]
static mut max77650_gpio_driver: platform_driver = platform_driver {
    driver: driver {
        name: c_str!("max77650-gpio"),
    },
    probe: Some(max77650_gpio_probe),
};

// Equivalent to module_platform_driver(max77650_gpio_driver).
module_platform_driver!(max77650_gpio_driver);

module_description!("MAXIM 77650/77651 GPIO driver");
module_author!("Bartosz Golaszewski <bgolaszewski@baylibre.com>");
module_license!("GPL v2");
module_alias!("platform:max77650-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
