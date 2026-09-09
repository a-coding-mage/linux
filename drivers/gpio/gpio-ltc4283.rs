// SPDX-License-Identifier: GPL-2.0-only
/*
 * Analog Devices LTC4283 GPIO driver
 *
 * Copyright 2025 Analog Devices Inc.
 */

// Kernel dependencies supplied by the surrounding build.

const LTC4283_PINS_MAX: u32 = 8;
const LTC4283_PGIOX_START_NR: u32 = 4;
const LTC4283_INPUT_STATUS: u32 = 0x02;
const LTC4283_PGIO_CONFIG: u32 = 0x10;
const LTC4283_PGIO_CONFIG_2: u32 = 0x11;
const LTC4283_ADIO_CONFIG: u32 = 0x12;
const LTC4283_PGIO_DIR_IN: u32 = 3;
const LTC4283_PGIO_DIR_OUT: u32 = 2;

#[inline]
const fn ltc4283_pgio_cfg_mask(pin: u32) -> u32 {
    genmask((pin - LTC4283_PGIOX_START_NR) * 2 + 1,
            (pin - LTC4283_PGIOX_START_NR) * 2)
}

/* starts at bit 4 */
#[inline]
const fn ltc4283_adiox_config_mask(pin: u32) -> u32 {
    1u32 << (pin + 4)
}

#[repr(C)]
struct ltc4283_gpio {
    gpio_chip: gpio_chip,
    regmap: *mut regmap,
}

unsafe fn ltc4283_pgio_get_direction(st: *const ltc4283_gpio, off: u32) -> i32 {
    let mut val: u32 = 0;
    let ret = regmap_read((*st).regmap, LTC4283_PGIO_CONFIG, &mut val);
    if ret != 0 {
        return ret;
    }

    val = field_get(ltc4283_pgio_cfg_mask(off), val);
    if val == LTC4283_PGIO_DIR_IN {
        return GPIO_LINE_DIRECTION_IN;
    }
    GPIO_LINE_DIRECTION_OUT
}

unsafe fn ltc4283_gpio_get_direction(gc: *mut gpio_chip, off: u32) -> i32 {
    let st = gpiochip_get_data(gc);
    let mut val: u32 = 0;
    if off >= LTC4283_PGIOX_START_NR {
        return ltc4283_pgio_get_direction(st, off);
    }

    let ret = regmap_read((*st).regmap, LTC4283_ADIO_CONFIG, &mut val);
    if ret != 0 {
        return ret;
    }
    if val & ltc4283_adiox_config_mask(off) != 0 {
        GPIO_LINE_DIRECTION_IN
    } else {
        GPIO_LINE_DIRECTION_OUT
    }
}

unsafe fn ltc4283_gpio_direction_set(st: *const ltc4283_gpio, off: u32, input: bool) -> i32 {
    if off >= LTC4283_PGIOX_START_NR {
        let mut val = LTC4283_PGIO_DIR_OUT;
        if input {
            val = LTC4283_PGIO_DIR_IN;
        }
        val = field_prep(ltc4283_pgio_cfg_mask(off), val);
        return regmap_update_bits((*st).regmap, LTC4283_PGIO_CONFIG,
                                  ltc4283_pgio_cfg_mask(off), val);
    }
    regmap_update_bits((*st).regmap, LTC4283_ADIO_CONFIG,
                       ltc4283_adiox_config_mask(off),
                       field_prep(ltc4283_adiox_config_mask(off), input as u32))
}

unsafe fn __ltc4283_gpio_set_value(st: *const ltc4283_gpio, off: u32, val: i32) -> i32 {
    let reg = if off < LTC4283_PGIOX_START_NR { LTC4283_ADIO_CONFIG } else { LTC4283_PGIO_CONFIG_2 };
    regmap_update_bits((*st).regmap, reg, 1u32 << off,
                       field_prep(1u32 << off, (val != 0) as u32))
}

unsafe fn ltc4283_gpio_direction_input(gc: *mut gpio_chip, off: u32) -> i32 {
    ltc4283_gpio_direction_set(gpiochip_get_data(gc), off, true)
}

unsafe fn ltc4283_gpio_direction_output(gc: *mut gpio_chip, off: u32, val: i32) -> i32 {
    let st = gpiochip_get_data(gc);
    let ret = ltc4283_gpio_direction_set(st, off, false);
    if ret != 0 { return ret; }
    __ltc4283_gpio_set_value(st, off, val)
}

unsafe fn ltc4283_gpio_get_value(gc: *mut gpio_chip, off: u32) -> i32 {
    let st = gpiochip_get_data(gc);
    let mut val: u32 = 0;
    let dir = ltc4283_gpio_get_direction(gc, off);
    if dir < 0 { return dir; }
    if dir == GPIO_LINE_DIRECTION_IN {
        let ret = regmap_read((*st).regmap, LTC4283_INPUT_STATUS, &mut val);
        if ret != 0 { return ret; }
        if off < LTC4283_PGIOX_START_NR { return ((val & (1u32 << (3 - off))) != 0) as i32; }
        return ((val & (1u32 << (7 - (off - LTC4283_PGIOX_START_NR)))) != 0) as i32;
    }
    let reg = if off < LTC4283_PGIOX_START_NR { LTC4283_ADIO_CONFIG } else { LTC4283_PGIO_CONFIG_2 };
    let ret = regmap_read((*st).regmap, reg, &mut val);
    if ret != 0 { return ret; }
    ((val & (1u32 << off)) != 0) as i32
}

unsafe fn ltc4283_gpio_set_value(gc: *mut gpio_chip, off: u32, val: i32) -> i32 {
    __ltc4283_gpio_set_value(gpiochip_get_data(gc), off, val)
}

unsafe fn ltc4283_init_valid_mask(gc: *mut gpio_chip, valid_mask: *mut c_ulong, ngpios: u32) -> i32 {
    let mask = dev_get_platdata((*gc).parent);
    bitmap_copy(valid_mask, mask, ngpios);
    0
}

unsafe fn ltc4283_gpio_probe(adev: *mut auxiliary_device, id: *const auxiliary_device_id) -> i32 {
    let dev = &mut (*adev).dev;
    let st = devm_kzalloc(dev, core::mem::size_of::<ltc4283_gpio>(), GFP_KERNEL) as *mut ltc4283_gpio;
    if st.is_null() { return -ENOMEM; }
    (*st).regmap = dev_get_regmap((*dev).parent, core::ptr::null());
    if (*st).regmap.is_null() { return dev_err_probe(dev, -ENODEV, "Failed to get regmap\0"); }
    let gc = &mut (*st).gpio_chip;
    gc.parent = dev;
    gc.get_direction = Some(ltc4283_gpio_get_direction);
    gc.direction_input = Some(ltc4283_gpio_direction_input);
    gc.direction_output = Some(ltc4283_gpio_direction_output);
    gc.get = Some(ltc4283_gpio_get_value);
    gc.set = Some(ltc4283_gpio_set_value);
    gc.init_valid_mask = Some(ltc4283_init_valid_mask);
    gc.can_sleep = true;
    gc.base = -1;
    gc.ngpio = LTC4283_PINS_MAX;
    gc.label = (*adev).name;
    gc.owner = THIS_MODULE;
    devm_gpiochip_add_data(dev, gc, st)
}

static LTC4283_AUX_ID_TABLE: [auxiliary_device_id; 2] = [
    auxiliary_device_id { name: "ltc4283.gpio\0" },
    auxiliary_device_id { name: "\0" },
];

static mut ltc4283_gpio_driver: auxiliary_driver = auxiliary_driver {
    probe: Some(ltc4283_gpio_probe),
    id_table: LTC4283_AUX_ID_TABLE.as_ptr(),
};

// MODULE_DEVICE_TABLE(auxiliary, ltc4283_aux_id_table)
// module_auxiliary_driver(ltc4283_gpio_driver)
// MODULE_AUTHOR("Nuno Sá <nuno.sa@analog.com>")
// MODULE_DESCRIPTION("GPIO LTC4283 Driver")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
