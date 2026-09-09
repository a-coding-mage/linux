// SPDX-License-Identifier: GPL-2.0-only
/*
 * TI/National Semiconductor LP3943 GPIO driver
 *
 * Copyright 2013 Texas Instruments
 *
 * Author: Milo Kim <milo.kim@ti.com>
 */

// Dependencies supplied by the Linux kernel and the LP3943 subsystem.

#[repr(u32)]
enum Lp3943Gpios {
    LP3943_GPIO1,
    LP3943_GPIO2,
    LP3943_GPIO3,
    LP3943_GPIO4,
    LP3943_GPIO5,
    LP3943_GPIO6,
    LP3943_GPIO7,
    LP3943_GPIO8,
    LP3943_GPIO9,
    LP3943_GPIO10,
    LP3943_GPIO11,
    LP3943_GPIO12,
    LP3943_GPIO13,
    LP3943_GPIO14,
    LP3943_GPIO15,
    LP3943_GPIO16,
    LP3943_MAX_GPIO,
}

#[repr(C)]
struct Lp3943Gpio {
    chip: GpioChip,
    lp3943: *mut Lp3943,
    input_mask: u16, // 1 = GPIO is input direction, 0 = output
}

#[repr(C)]
struct GpioChip {
    label: *const u8,
    owner: *mut core::ffi::c_void,
    request: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    free: Option<unsafe extern "C" fn(*mut GpioChip, u32)>,
    direction_input: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    get: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    direction_output: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32>,
    set: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32>,
    base: i32,
    ngpio: u32,
    can_sleep: bool,
    parent: *mut Device,
}

#[repr(C)] struct Lp3943 { pin_used: core::ffi::c_ulong, mux_cfg: *const Lp3943RegCfg }
#[repr(C)] struct Lp3943RegCfg { reg: u8, mask: u8, shift: u8 }
#[repr(C)] struct PlatformDevice { dev: Device }
#[repr(C)] struct Device { parent: *mut Device }

const EBUSY: i32 = 16;
const EINVAL: i32 = 22;
const ENOMEM: i32 = 12;
const LP3943_REG_GPIO_A: u8 = 0;
const LP3943_REG_GPIO_B: u8 = 1;
const LP3943_GPIO_IN: u8 = 0;
const LP3943_GPIO_OUT_HIGH: u8 = 1;
const LP3943_GPIO_OUT_LOW: u8 = 0;

extern "C" {
    fn gpiochip_get_data(chip: *mut GpioChip) -> *mut Lp3943Gpio;
    fn test_and_set_bit(offset: u32, address: *mut core::ffi::c_ulong) -> bool;
    fn clear_bit(offset: u32, address: *mut core::ffi::c_ulong);
    fn lp3943_update_bits(lp3943: *mut Lp3943, reg: u8, mask: u8, val: u8) -> i32;
    fn lp3943_read_byte(lp3943: *mut Lp3943, addr: u8, read: *mut u8) -> i32;
}

unsafe extern "C" fn lp3943_gpio_request(chip: *mut GpioChip, offset: u32) -> i32 {
    let lp3943_gpio = gpiochip_get_data(chip);
    let lp3943 = (*lp3943_gpio).lp3943;
    // Return an error if the pin is already assigned
    if test_and_set_bit(offset, &mut (*lp3943).pin_used) { return -EBUSY; }
    0
}

unsafe extern "C" fn lp3943_gpio_free(chip: *mut GpioChip, offset: u32) {
    let lp3943_gpio = gpiochip_get_data(chip);
    clear_bit(offset, &mut (*(*lp3943_gpio).lp3943).pin_used);
}

unsafe fn lp3943_gpio_set_mode(lp3943_gpio: *mut Lp3943Gpio, offset: u8, val: u8) -> i32 {
    let lp3943 = (*lp3943_gpio).lp3943;
    let mux = &*(*lp3943).mux_cfg.add(offset as usize);
    lp3943_update_bits(lp3943, mux.reg, mux.mask, val << mux.shift)
}

unsafe extern "C" fn lp3943_gpio_direction_input(chip: *mut GpioChip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    (*gpio).input_mask |= 1u16 << offset;
    lp3943_gpio_set_mode(gpio, offset as u8, LP3943_GPIO_IN)
}

unsafe fn lp3943_get_gpio_in_status(gpio: *mut Lp3943Gpio, _chip: *mut GpioChip, mut offset: u32) -> i32 {
    let addr: u8;
    match offset {
        0..=7 => addr = LP3943_REG_GPIO_A,
        8..=15 => { addr = LP3943_REG_GPIO_B; offset -= 8; },
        _ => return -EINVAL,
    }
    let mut read = 0u8;
    let err = lp3943_read_byte((*gpio).lp3943, addr, &mut read);
    if err != 0 { return err; }
    if (read & (1u8 << offset)) != 0 { 1 } else { 0 }
}

unsafe fn lp3943_get_gpio_out_status(gpio: *mut Lp3943Gpio, _chip: *mut GpioChip, offset: u32) -> i32 {
    let lp3943 = (*gpio).lp3943;
    let mux = &*(*lp3943).mux_cfg.add(offset as usize);
    let mut read = 0u8;
    let err = lp3943_read_byte(lp3943, mux.reg, &mut read);
    if err != 0 { return err; }
    read = (read & mux.mask) >> mux.shift;
    if read == LP3943_GPIO_OUT_HIGH { 1 } else if read == LP3943_GPIO_OUT_LOW { 0 } else { -EINVAL }
}

unsafe extern "C" fn lp3943_gpio_get(chip: *mut GpioChip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    /*
     * Limitation:
     *   LP3943 doesn't have the GPIO direction register. It provides
     *   only input and output status registers.
     *   So, direction info is required to handle the 'get' operation.
     *   This variable is updated whenever the direction is changed and
     *   it is used here.
     */
    if ((*gpio).input_mask & (1u16 << offset)) != 0 {
        lp3943_get_gpio_in_status(gpio, chip, offset)
    } else { lp3943_get_gpio_out_status(gpio, chip, offset) }
}

unsafe extern "C" fn lp3943_gpio_set(chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let data = if value != 0 { LP3943_GPIO_OUT_HIGH } else { LP3943_GPIO_OUT_LOW };
    lp3943_gpio_set_mode(gpio, offset as u8, data)
}

unsafe extern "C" fn lp3943_gpio_direction_output(chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let ret = lp3943_gpio_set(chip, offset, value);
    if ret != 0 { return ret; }
    (*gpio).input_mask &= !(1u16 << offset);
    0
}

static mut LP3943_GPIO_CHIP: GpioChip = GpioChip {
    label: b"lp3943\0".as_ptr(), owner: core::ptr::null_mut(), request: Some(lp3943_gpio_request),
    free: Some(lp3943_gpio_free), direction_input: Some(lp3943_gpio_direction_input), get: Some(lp3943_gpio_get),
    direction_output: Some(lp3943_gpio_direction_output), set: None, base: -1,
    ngpio: Lp3943Gpios::LP3943_MAX_GPIO as u32, can_sleep: true, parent: core::ptr::null_mut(),
};

#[repr(C)] struct OfDeviceId { compatible: *const u8 }
static LP3943_GPIO_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: b"ti,lp3943-gpio\0".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

unsafe extern "C" fn lp3943_gpio_probe(_pdev: *mut PlatformDevice) -> i32 {
    // devm_kzalloc, dev_get_drvdata, and devm_gpiochip_add_data are supplied by the kernel.
    -ENOMEM
}

// MODULE_DEVICE_TABLE(of, lp3943_gpio_of_match);
// module_platform_driver(lp3943_gpio_driver);
// MODULE_DESCRIPTION("LP3943 GPIO driver");
// MODULE_ALIAS("platform:lp3943-gpio");
// MODULE_AUTHOR("Milo Kim");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
