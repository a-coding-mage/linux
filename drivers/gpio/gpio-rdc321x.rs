// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RDC321x GPIO driver
 *
 * Copyright (C) 2008, Volker Weiss <dev@tintuc.de>
 * Copyright (C) 2007-2010 Florian Fainelli <florian@openwrt.org>
 */

// Linux kernel dependencies supplied by the surrounding kernel translation.
use core::ffi::c_void;

#[repr(C)]
struct SpinlockT {
    _opaque: [u8; 0],
}

#[repr(C)]
struct PciDev {
    _opaque: [u8; 0],
}

#[repr(C)]
struct Device {
    _opaque: [u8; 0],
}

#[repr(C)]
struct PlatformDevice {
    dev: Device,
    _opaque: [u8; 0],
}

#[repr(C)]
struct Resource {
    start: u64,
    _opaque: [u8; 0],
}

#[repr(C)]
struct Rdc321xGpioPdata {
    sb_pdev: *mut PciDev,
    max_gpios: u32,
}

type DirectionInput = unsafe extern "C" fn(*mut GpioChip, u32) -> i32;
type DirectionOutput = unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32;
type GetValue = unsafe extern "C" fn(*mut GpioChip, u32) -> i32;
type SetValue = unsafe extern "C" fn(*mut GpioChip, u32, i32);

#[repr(C)]
struct GpioChip {
    label: *const i8,
    owner: *mut c_void,
    direction_input: Option<DirectionInput>,
    direction_output: Option<DirectionOutput>,
    get: Option<GetValue>,
    set: Option<SetValue>,
    base: i32,
    ngpio: u32,
}

#[repr(C)]
struct Rdc321xGpio {
    lock: SpinlockT,
    sb_pdev: *mut PciDev,
    data_reg: [u32; 2],
    reg1_ctrl_base: i32,
    reg1_data_base: i32,
    reg2_ctrl_base: i32,
    reg2_data_base: i32,
    chip: GpioChip,
}

extern "C" {
    fn gpiochip_get_data(chip: *mut GpioChip) -> *mut c_void;
    fn spin_lock(lock: *mut SpinlockT);
    fn spin_unlock(lock: *mut SpinlockT);
    fn pci_write_config_dword(dev: *mut PciDev, reg: i32, value: u32) -> i32;
    fn pci_read_config_dword(dev: *mut PciDev, reg: i32, value: *mut u32) -> i32;
    fn pcibios_err_to_errno(err: i32) -> i32;
    fn dev_get_platdata(dev: *mut Device) -> *mut c_void;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut c_void;
    fn platform_get_resource_byname(pdev: *mut PlatformDevice, resource_type: u32, name: *const i8) -> *mut Resource;
    fn spin_lock_init(lock: *mut SpinlockT);
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut c_void);
    fn devm_gpiochip_add_data(dev: *mut Device, chip: *mut GpioChip, data: *mut c_void) -> i32;
}

const IORESOURCE_IO: u32 = 0x0000_0100;
const GFP_KERNEL: u32 = 0x0000_00d0;
const ENODEV: i32 = 19;
const ENOMEM: i32 = 12;

unsafe fn rdc_gpio_get_value(chip: *mut GpioChip, gpio: u32) -> i32 {
    let gpch = gpiochip_get_data(chip) as *mut Rdc321xGpio;
    let reg = if gpio < 32 { (*gpch).reg1_data_base } else { (*gpch).reg2_data_base };
    let mut value: u32 = 0;

    spin_lock(&mut (*gpch).lock);
    pci_write_config_dword((*gpch).sb_pdev, reg, (*gpch).data_reg[if gpio < 32 { 0 } else { 1 }]);
    pci_read_config_dword((*gpch).sb_pdev, reg, &mut value);
    spin_unlock(&mut (*gpch).lock);

    if (1u32 << (gpio & 0x1f)) & value != 0 { 1 } else { 0 }
}

unsafe fn rdc_gpio_set_value_impl(chip: *mut GpioChip, gpio: u32, value: i32) {
    let gpch = gpiochip_get_data(chip) as *mut Rdc321xGpio;
    let reg = if gpio < 32 { 0 } else { 1 };

    if value != 0 {
        (*gpch).data_reg[reg] |= 1u32 << (gpio & 0x1f);
    } else {
        (*gpch).data_reg[reg] &= !(1u32 << (gpio & 0x1f));
    }

    pci_write_config_dword(
        (*gpch).sb_pdev,
        if reg != 0 { (*gpch).reg2_data_base } else { (*gpch).reg1_data_base },
        (*gpch).data_reg[reg],
    );
}

unsafe fn rdc_gpio_set_value(chip: *mut GpioChip, gpio: u32, value: i32) {
    let gpch = gpiochip_get_data(chip) as *mut Rdc321xGpio;
    spin_lock(&mut (*gpch).lock);
    rdc_gpio_set_value_impl(chip, gpio, value);
    spin_unlock(&mut (*gpch).lock);
}

unsafe fn rdc_gpio_config(chip: *mut GpioChip, gpio: u32, value: i32) -> i32 {
    let gpch = gpiochip_get_data(chip) as *mut Rdc321xGpio;
    let mut reg: u32 = 0;
    let mut err: i32;

    spin_lock(&mut (*gpch).lock);
    err = pci_read_config_dword(
        (*gpch).sb_pdev,
        if gpio < 32 { (*gpch).reg1_ctrl_base } else { (*gpch).reg2_ctrl_base },
        &mut reg,
    );
    if err != 0 {
        spin_unlock(&mut (*gpch).lock);
        return pcibios_err_to_errno(err);
    }

    reg |= 1u32 << (gpio & 0x1f);
    err = pci_write_config_dword(
        (*gpch).sb_pdev,
        if gpio < 32 { (*gpch).reg1_ctrl_base } else { (*gpch).reg2_ctrl_base },
        reg,
    );
    if err == 0 {
        rdc_gpio_set_value_impl(chip, gpio, value);
    }
    spin_unlock(&mut (*gpch).lock);
    pcibios_err_to_errno(err)
}

unsafe fn rdc_gpio_direction_input(chip: *mut GpioChip, gpio: u32) -> i32 {
    rdc_gpio_config(chip, gpio, 1)
}

unsafe fn rdc321x_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let pdata = dev_get_platdata(&mut (*pdev).dev) as *mut Rdc321xGpioPdata;
    if pdata.is_null() {
        return -ENODEV;
    }

    let dev = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<Rdc321xGpio>(), GFP_KERNEL)
        as *mut Rdc321xGpio;
    if dev.is_null() {
        return -ENOMEM;
    }

    let r1 = platform_get_resource_byname(pdev, IORESOURCE_IO, b"gpio-reg1\0".as_ptr() as *const i8);
    if r1.is_null() {
        return -ENODEV;
    }
    spin_lock_init(&mut (*dev).lock);
    (*dev).sb_pdev = (*pdata).sb_pdev;
    (*dev).reg1_ctrl_base = (*r1).start as i32;
    (*dev).reg1_data_base = (*r1).start.wrapping_add(0x4) as i32;

    let r2 = platform_get_resource_byname(pdev, IORESOURCE_IO, b"gpio-reg2\0".as_ptr() as *const i8);
    if r2.is_null() {
        return -ENODEV;
    }
    (*dev).reg2_ctrl_base = (*r2).start as i32;
    (*dev).reg2_data_base = (*r2).start.wrapping_add(0x4) as i32;

    (*dev).chip.label = b"rdc321x-gpio\0".as_ptr() as *const i8;
    (*dev).chip.owner = core::ptr::null_mut();
    (*dev).chip.direction_input = Some(rdc_gpio_direction_input);
    (*dev).chip.direction_output = Some(rdc_gpio_config);
    (*dev).chip.get = Some(rdc_gpio_get_value);
    (*dev).chip.set = Some(rdc_gpio_set_value);
    (*dev).chip.base = 0;
    (*dev).chip.ngpio = (*pdata).max_gpios;
    platform_set_drvdata(pdev, dev as *mut c_void);

    let mut err = pci_read_config_dword((*dev).sb_pdev, (*dev).reg1_data_base, &mut (*dev).data_reg[0]);
    if err != 0 { return pcibios_err_to_errno(err); }
    err = pci_read_config_dword((*dev).sb_pdev, (*dev).reg2_data_base, &mut (*dev).data_reg[1]);
    if err != 0 { return pcibios_err_to_errno(err); }
    devm_gpiochip_add_data(&mut (*pdev).dev, &mut (*dev).chip, dev as *mut c_void)
}

// Equivalent of module_platform_driver(rdc321x_gpio_driver).
#[allow(dead_code)]
static mut RDC321X_GPIO_DRIVER: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32> =
    Some(rdc321x_gpio_probe);

// MODULE_AUTHOR("Florian Fainelli <florian@openwrt.org>");
// MODULE_DESCRIPTION("RDC321x GPIO driver");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:rdc321x-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
