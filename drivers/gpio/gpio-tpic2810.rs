// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015-2023 Texas Instruments Incorporated - https://www.ti.com/
 *	Andrew Davis <afd@ti.com>
 */

// Dependencies supplied by the Linux kernel bindings.

const TPIC2810_WS_COMMAND: u8 = 0x44;

/**
 * struct tpic2810 - GPIO driver data
 * @chip: GPIO controller chip
 * @client: I2C device pointer
 * @buffer: Buffer for device register
 * @lock: Protects write sequences
 */
#[repr(C)]
struct tpic2810 {
    chip: gpio_chip,
    client: *mut i2c_client,
    buffer: u8,
    lock: mutex,
}

extern "C" {
    static THIS_MODULE: *mut module;
    fn gpiochip_get_data(chip: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn mutex_lock(lock: *mut mutex);
    fn mutex_unlock(lock: *mut mutex);
    fn i2c_smbus_write_byte_data(client: *mut i2c_client, command: u8, value: u8) -> i32;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn mutex_init(lock: *mut mutex);
    fn devm_gpiochip_add_data(
        dev: *mut device,
        chip: *mut gpio_chip,
        data: *mut core::ffi::c_void,
    ) -> i32;
}

#[repr(C)]
struct gpio_chip {
    label: *const u8,
    owner: *mut module,
    get_direction: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32)>,
    set_multiple: Option<unsafe extern "C" fn(*mut gpio_chip, *mut u64, *mut u64) -> i32>,
    base: i32,
    ngpio: u32,
    can_sleep: bool,
    parent: *mut device,
}

#[repr(C)]
struct i2c_client {
    dev: device,
}
#[repr(C)] struct device;
#[repr(C)] struct mutex;
#[repr(C)] struct module;
#[repr(C)] struct of_device_id;
#[repr(C)] struct i2c_device_id;
#[repr(C)] struct i2c_driver;

const GPIO_LINE_DIRECTION_OUT: i32 = 0;
const GFP_KERNEL: u32 = 0;
const ENOMEM: i32 = 12;

unsafe extern "C" fn tpic2810_get_direction(_chip: *mut gpio_chip, _offset: u32) -> i32 {
    // This device always output
    GPIO_LINE_DIRECTION_OUT
}

unsafe extern "C" fn tpic2810_direction_output(
    chip: *mut gpio_chip,
    offset: u32,
    value: i32,
) -> i32 {
    // This device always output
    tpic2810_set(chip, offset, value)
}

unsafe fn tpic2810_set_mask_bits(chip: *mut gpio_chip, mask: u8, bits: u8) {
    let gpio = gpiochip_get_data(chip) as *mut tpic2810;
    let mut buffer: u8;
    let err: i32;

    mutex_lock(&mut (*gpio).lock);

    buffer = (*gpio).buffer & !mask;
    buffer |= mask & bits;

    err = i2c_smbus_write_byte_data((*gpio).client, TPIC2810_WS_COMMAND, buffer);
    if err == 0 {
        (*gpio).buffer = buffer;
    }

    mutex_unlock(&mut (*gpio).lock);
}

unsafe extern "C" fn tpic2810_set(chip: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    tpic2810_set_mask_bits(chip, 1u8.wrapping_shl(offset), if value != 0 { 1u8.wrapping_shl(offset) } else { 0 });
    0
}

unsafe extern "C" fn tpic2810_set_multiple(
    chip: *mut gpio_chip,
    mask: *mut u64,
    bits: *mut u64,
) -> i32 {
    tpic2810_set_mask_bits(chip, *mask as u8, *bits as u8);
    0
}

static mut TEMPLATE_CHIP: gpio_chip = gpio_chip {
    label: b"tpic2810\0".as_ptr(),
    owner: core::ptr::null_mut(),
    get_direction: Some(tpic2810_get_direction),
    direction_output: Some(tpic2810_direction_output),
    set: Some(tpic2810_set),
    set_multiple: Some(tpic2810_set_multiple),
    base: -1,
    ngpio: 8,
    can_sleep: true,
    parent: core::ptr::null_mut(),
};

static mut TPIC2810_OF_MATCH_TABLE: [of_device_id; 2] = [
    of_device_id {},
    of_device_id {},
];

unsafe extern "C" fn tpic2810_probe(client: *mut i2c_client) -> i32 {
    let gpio = devm_kzalloc(&mut (*client).dev, core::mem::size_of::<tpic2810>(), GFP_KERNEL)
        as *mut tpic2810;
    if gpio.is_null() {
        return -ENOMEM;
    }

    (*gpio).chip = TEMPLATE_CHIP;
    (*gpio).chip.parent = &mut (*client).dev;
    (*gpio).client = client;
    mutex_init(&mut (*gpio).lock);

    devm_gpiochip_add_data(&mut (*client).dev, &mut (*gpio).chip, gpio as *mut core::ffi::c_void)
}

static mut TPIC2810_ID_TABLE: [i2c_device_id; 2] = [i2c_device_id {}, i2c_device_id {}];
static mut TPIC2810_DRIVER: i2c_driver = i2c_driver {};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
