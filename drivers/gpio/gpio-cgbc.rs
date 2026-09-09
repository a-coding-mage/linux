// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Congatec Board Controller GPIO driver
 *
 * Copyright (C) 2024 Bootlin
 * Author: Thomas Richard <thomas.richard@bootlin.com>
 */

// Linux kernel dependencies supplied by the surrounding repository.

const CGBC_GPIO_NGPIO: u32 = 14;

const CGBC_GPIO_CMD_GET: u8 = 0x64;
const CGBC_GPIO_CMD_SET: u8 = 0x65;
const CGBC_GPIO_CMD_DIR_GET: u8 = 0x66;
const CGBC_GPIO_CMD_DIR_SET: u8 = 0x67;

#[repr(C)]
struct CgbcGpioData {
    chip: GpioChip,
    cgbc: *mut CgbcDeviceData,
    lock: Mutex,
}

#[repr(C)]
struct GpioChip {
    label: *const core::ffi::c_char,
    owner: *mut core::ffi::c_void,
    parent: *mut Device,
    base: i32,
    direction_input: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    direction_output: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32>,
    get_direction: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    get: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    set: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32)>,
    ngpio: u32,
}

#[repr(C)]
struct CgbcDeviceData;
#[repr(C)]
struct Mutex;
#[repr(C)]
struct Device;
#[repr(C)]
struct PlatformDevice {
    dev: Device,
}

extern "C" {
    fn cgbc_command(
        cgbc: *mut CgbcDeviceData,
        cmd: *mut u8,
        cmd_len: usize,
        value: *mut u8,
        value_len: usize,
        extra: *mut core::ffi::c_void,
    ) -> i32;
    fn gpiochip_get_data(chip: *mut GpioChip) -> *mut CgbcGpioData;
}

unsafe fn cgbc_gpio_cmd(
    cgbc: *mut CgbcDeviceData,
    cmd0: u8,
    cmd1: u8,
    cmd2: u8,
    value: *mut u8,
) -> i32 {
    let mut cmd = [cmd0, cmd1, cmd2];
    cgbc_command(cgbc, cmd.as_mut_ptr(), core::mem::size_of_val(&cmd), value, 1, core::ptr::null_mut())
}

unsafe extern "C" fn cgbc_gpio_get(chip: *mut GpioChip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let cgbc = (*gpio).cgbc;
    let mut val = 0u8;
    // scoped_guard(mutex, &gpio->lock)
    let ret = cgbc_gpio_cmd(cgbc, CGBC_GPIO_CMD_GET, if offset > 7 { 1 } else { 0 }, 0, &mut val);
    let offset = offset % 8;
    if ret != 0 { return ret; }
    if (val & (1u8 << offset)) != 0 { 1 } else { 0 }
}

unsafe fn __cgbc_gpio_set(chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let cgbc = (*gpio).cgbc;
    let mut val = 0u8;
    let ret = cgbc_gpio_cmd(cgbc, CGBC_GPIO_CMD_GET, if offset > 7 { 1 } else { 0 }, 0, &mut val);
    if ret != 0 { return ret; }
    if value != 0 { val |= 1u8 << (offset % 8); } else { val &= !(1u8 << (offset % 8)); }
    cgbc_gpio_cmd(cgbc, CGBC_GPIO_CMD_SET, if offset > 7 { 1 } else { 0 }, val, &mut val)
}

unsafe extern "C" fn cgbc_gpio_set(chip: *mut GpioChip, offset: u32, value: i32) {
    let gpio = gpiochip_get_data(chip);
    // guard(mutex)(&gpio->lock)
    let _ = __cgbc_gpio_set(chip, offset, value);
}

unsafe fn cgbc_gpio_direction_set(chip: *mut GpioChip, offset: u32, direction: i32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let mut val = 0u8;
    let cgbc = (*gpio).cgbc;
    let mut ret = cgbc_gpio_cmd(cgbc, CGBC_GPIO_CMD_DIR_GET, if offset > 7 { 1 } else { 0 }, 0, &mut val);
    if ret != 0 { return ret; }
    if direction == 1 { val &= !(1u8 << (offset % 8)); } else { val |= 1u8 << (offset % 8); }
    ret = cgbc_gpio_cmd(cgbc, CGBC_GPIO_CMD_DIR_SET, if offset > 7 { 1 } else { 0 }, val, &mut val);
    ret
}

unsafe extern "C" fn cgbc_gpio_direction_input(chip: *mut GpioChip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip); let _ = gpio;
    cgbc_gpio_direction_set(chip, offset, 1)
}

unsafe extern "C" fn cgbc_gpio_direction_output(chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(chip); let _ = gpio;
    let ret = __cgbc_gpio_set(chip, offset, value);
    if ret != 0 { return ret; }
    cgbc_gpio_direction_set(chip, offset, 0)
}

unsafe extern "C" fn cgbc_gpio_get_direction(chip: *mut GpioChip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(chip);
    let mut val = 0u8;
    let ret = cgbc_gpio_cmd((*gpio).cgbc, CGBC_GPIO_CMD_DIR_GET, if offset > 7 { 1 } else { 0 }, 0, &mut val);
    if ret != 0 { return ret; }
    if (val & (1u8 << (offset % 8))) != 0 { 0 } else { 1 }
}

unsafe extern "C" fn cgbc_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev;
    let cgbc = dev_get_drvdata((*dev).parent);
    let gpio = devm_kzalloc(dev, core::mem::size_of::<CgbcGpioData>(), 0);
    if gpio.is_null() { return -12; }
    (*gpio).cgbc = cgbc;
    platform_set_drvdata(pdev, gpio as *mut core::ffi::c_void);
    let chip = &mut (*gpio).chip;
    chip.label = dev_name(dev);
    chip.owner = THIS_MODULE;
    chip.parent = dev;
    chip.base = -1;
    chip.direction_input = Some(cgbc_gpio_direction_input);
    chip.direction_output = Some(cgbc_gpio_direction_output);
    chip.get_direction = Some(cgbc_gpio_get_direction);
    chip.get = Some(cgbc_gpio_get);
    chip.set = Some(cgbc_gpio_set);
    chip.ngpio = CGBC_GPIO_NGPIO;
    let ret = devm_mutex_init(dev, &mut (*gpio).lock);
    if ret != 0 { return ret; }
    let ret = devm_gpiochip_add_data(dev, chip, gpio);
    if ret != 0 { return dev_err_probe(dev, ret, b"Could not register GPIO chip\0".as_ptr() as *const _); }
    0
}

#[repr(C)]
struct PlatformDriver {
    name: *const core::ffi::c_char,
    probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
}

static mut CGBC_GPIO_DRIVER: PlatformDriver = PlatformDriver {
    name: b"cgbc-gpio\0".as_ptr() as *const _,
    probe: Some(cgbc_gpio_probe),
};

extern "C" {
    static mut THIS_MODULE: *mut core::ffi::c_void;
    fn dev_get_drvdata(dev: *mut Device) -> *mut CgbcDeviceData;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut CgbcGpioData;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut core::ffi::c_void);
    fn dev_name(dev: *mut Device) -> *const core::ffi::c_char;
    fn devm_mutex_init(dev: *mut Device, lock: *mut Mutex) -> i32;
    fn devm_gpiochip_add_data(dev: *mut Device, chip: *mut GpioChip, data: *mut CgbcGpioData) -> i32;
    fn dev_err_probe(dev: *mut Device, err: i32, fmt: *const core::ffi::c_char) -> i32;
}

// module_platform_driver(cgbc_gpio_driver);
// MODULE_DESCRIPTION("Congatec Board Controller GPIO Driver");
// MODULE_AUTHOR("Thomas Richard <thomas.richard@bootlin.com>");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:cgbc-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
