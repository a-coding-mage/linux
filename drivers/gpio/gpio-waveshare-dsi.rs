// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2024 Waveshare International Limited
 * Copyright (c) Qualcomm Technologies, Inc. and/or its subsidiaries.
 */

// Linux kernel dependencies supplied by the surrounding build.

const REG_TP: u8 = 0x94;
const REG_LCD: u8 = 0x95;
const REG_PWM: u8 = 0x96;
const REG_SIZE: u8 = 0x97;
const REG_ID: u8 = 0x98;
const REG_VERSION: u8 = 0x99;

const GPIO_AVDD: u32 = 0;
const GPIO_PANEL_RESET: u32 = 1;
const GPIO_BL_ENABLE: u32 = 2;
const GPIO_IOVCC: u32 = 4;
const GPIO_VCC: u32 = 8;
const GPIO_TS_RESET: u32 = 9;
const NUM_GPIO: u32 = 16;

#[repr(C)]
pub struct waveshare_gpio {
    pub dir_lock: mutex,
    pub pwr_lock: mutex,
    pub regmap: *mut regmap,
    pub poweron_state: u16,
    pub gc: gpio_chip,
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: u32,
    pub val_bits: u32,
    pub max_register: u8,
}

static waveshare_gpio_regmap_config: regmap_config = regmap_config {
    reg_bits: 8,
    val_bits: 8,
    max_register: REG_VERSION,
};

unsafe fn waveshare_gpio_get(state: *mut waveshare_gpio, offset: u32) -> i32 {
    let _guard = mutex_guard((*state).pwr_lock);
    let pwr_state = (*state).poweron_state & (1u16 << offset);
    (pwr_state != 0) as i32
}

unsafe fn waveshare_gpio_set(state: *mut waveshare_gpio, offset: u32, value: i32) -> i32 {
    let _guard = mutex_guard((*state).pwr_lock);
    let mut last_val = (*state).poweron_state;
    if value != 0 {
        last_val |= 1u16 << offset;
    } else {
        last_val &= !(1u16 << offset);
    }
    (*state).poweron_state = last_val;

    let mut err = regmap_write((*state).regmap, REG_TP, (last_val >> 8) as u32);
    if err == 0 {
        err = regmap_write((*state).regmap, REG_LCD, (last_val & 0xff) as u32);
    }
    err
}

unsafe fn waveshare_gpio_gpio_get_direction(_gc: *mut gpio_chip, _offset: u32) -> i32 {
    GPIO_LINE_DIRECTION_OUT
}

unsafe fn waveshare_gpio_gpio_get(gc: *mut gpio_chip, offset: u32) -> i32 {
    waveshare_gpio_get(gpiochip_get_data(gc), offset)
}

unsafe fn waveshare_gpio_gpio_set(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    waveshare_gpio_set(gpiochip_get_data(gc), offset, value)
}

unsafe fn waveshare_gpio_update_status(bl: *mut backlight_device) -> i32 {
    let state = bl_get_data(bl);
    let brightness = backlight_get_brightness(bl);
    waveshare_gpio_set(state, GPIO_BL_ENABLE, brightness);
    regmap_write((*state).regmap, REG_PWM, brightness as u32)
}

static waveshare_gpio_bl: backlight_ops = backlight_ops {
    update_status: Some(waveshare_gpio_update_status),
};

unsafe fn waveshare_gpio_probe(i2c: *mut i2c_client) -> i32 {
    let mut props: backlight_properties = core::mem::zeroed();
    let dev = &mut (*i2c).dev;
    let state = devm_kzalloc(dev, core::mem::size_of::<waveshare_gpio>(), GFP_KERNEL)
        as *mut waveshare_gpio;
    if state.is_null() { return -ENOMEM; }

    let mut ret = devm_mutex_init(dev, &mut (*state).dir_lock);
    if ret != 0 { return ret; }
    ret = devm_mutex_init(dev, &mut (*state).pwr_lock);
    if ret != 0 { return ret; }

    let regmap = devm_regmap_init_i2c(i2c, &waveshare_gpio_regmap_config);
    if is_err(regmap) { return dev_err_probe(dev, ptr_err(regmap), c"Failed to allocate register map\n"); }
    (*state).regmap = regmap;
    i2c_set_clientdata(i2c, state as *mut core::ffi::c_void);

    let mut data = 0u32;
    ret = regmap_read(regmap, REG_ID, &mut data);
    if ret < 0 { return dev_err_probe(dev, ret, c"Failed to read register\n"); }
    dev_dbg(dev, c"waveshare panel hw id = 0x%x\n", data);
    ret = regmap_read(regmap, REG_SIZE, &mut data);
    if ret < 0 { return dev_err_probe(dev, ret, c"Failed to read register\n"); }
    dev_dbg(dev, c"waveshare panel size = %d\n", data);
    ret = regmap_read(regmap, REG_VERSION, &mut data);
    if ret < 0 { return dev_err_probe(dev, ret, c"Failed to read register\n"); }
    dev_dbg(dev, c"waveshare panel mcu version = 0x%x\n", data);

    ret = waveshare_gpio_set(state, GPIO_TS_RESET, 1);
    if ret != 0 { return dev_err_probe(dev, ret, c"Failed to program GPIOs\n"); }
    msleep(20);

    (*state).gc.parent = dev;
    (*state).gc.label = (*i2c).name;
    (*state).gc.owner = THIS_MODULE;
    (*state).gc.base = -1;
    (*state).gc.ngpio = NUM_GPIO;
    (*state).gc.get = Some(waveshare_gpio_gpio_get);
    (*state).gc.set = Some(waveshare_gpio_gpio_set);
    (*state).gc.get_direction = Some(waveshare_gpio_gpio_get_direction);
    (*state).gc.can_sleep = true;

    ret = devm_gpiochip_add_data(dev, &mut (*state).gc, state as *mut core::ffi::c_void);
    if ret != 0 { return dev_err_probe(dev, ret, c"Failed to create gpiochip\n"); }
    props.type_ = BACKLIGHT_RAW;
    props.max_brightness = 255;
    props.brightness = 255;
    let bl = devm_backlight_device_register(dev, dev_name(dev), dev, state as *mut core::ffi::c_void,
                                            &waveshare_gpio_bl, &props);
    ptr_err_or_zero(bl)
}

// Device-table and module registration are provided by the kernel build system.
// MODULE_DEVICE_TABLE(of, waveshare_gpio_dt_ids);
// module_i2c_driver(waveshare_gpio_regulator_driver);
// MODULE_DESCRIPTION("GPIO controller driver for Waveshare DSI touch panels");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
