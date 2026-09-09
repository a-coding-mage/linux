// SPDX-License-Identifier: GPL-2.0-only
/*
 * GPIO driver for Exar XR17V35X chip
 *
 * Copyright (C) 2015 Sudip Mukherjee <sudip.mukherjee@codethink.co.uk>
 */

// Kernel dependencies:
// linux/bitops.h, linux/device.h, linux/gpio/driver.h, linux/idr.h,
// linux/init.h, linux/kernel.h, linux/module.h, linux/pci.h,
// linux/platform_device.h, linux/regmap.h

const EXAR_OFFSET_MPIOLVL_LO: u32 = 0x90;
const EXAR_OFFSET_MPIOSEL_LO: u32 = 0x93;
const EXAR_OFFSET_MPIOLVL_HI: u32 = 0x96;
const EXAR_OFFSET_MPIOSEL_HI: u32 = 0x99;

/*
 * The Device Configuration and UART Configuration Registers
 * for each UART channel take 1KB of memory address space.
 */
const EXAR_UART_CHANNEL_SIZE: u32 = 0x400;

const DRIVER_NAME: &str = "gpio_exar";

extern "C" {
    static mut ida_index: Ida;
}

#[repr(C)]
pub struct ExarGpioChip {
    pub gpio_chip: GpioChip,
    pub regmap: *mut Regmap,
    pub index: i32,
    pub name: [u8; 20],
    pub first_pin: u32,
    /*
     * The offset to the cascaded device's (if existing)
     * Device Configuration Registers.
     */
    pub cascaded_offset: u32,
}

extern "C" {
    pub fn gpiochip_get_data(chip: *mut GpioChip) -> *mut ExarGpioChip;
    pub fn regmap_test_bits(map: *mut Regmap, reg: u32, mask: u32) -> i32;
    pub fn regmap_write_bits(map: *mut Regmap, reg: u32, mask: u32, val: u32) -> i32;
    pub fn regmap_clear_bits(map: *mut Regmap, reg: u32, mask: u32) -> i32;
    pub fn regmap_set_bits(map: *mut Regmap, reg: u32, mask: u32) -> i32;
    pub fn ida_free(ida: *mut Ida, id: i32);
}

#[inline]
unsafe fn exar_offset_to_sel_addr(exar_gpio: *mut ExarGpioChip, offset: u32) -> u32 {
    let pin = (*exar_gpio).first_pin + (offset % 16);
    let cascaded = offset / 16;
    let addr = if pin / 8 != 0 { EXAR_OFFSET_MPIOSEL_HI } else { EXAR_OFFSET_MPIOSEL_LO };
    addr + if cascaded != 0 { (*exar_gpio).cascaded_offset } else { 0 }
}

#[inline]
unsafe fn exar_offset_to_lvl_addr(exar_gpio: *mut ExarGpioChip, offset: u32) -> u32 {
    let pin = (*exar_gpio).first_pin + (offset % 16);
    let cascaded = offset / 16;
    let addr = if pin / 8 != 0 { EXAR_OFFSET_MPIOLVL_HI } else { EXAR_OFFSET_MPIOLVL_LO };
    addr + if cascaded != 0 { (*exar_gpio).cascaded_offset } else { 0 }
}

#[inline]
unsafe fn exar_offset_to_bit(exar_gpio: *mut ExarGpioChip, offset: u32) -> u32 {
    let pin = (*exar_gpio).first_pin + (offset % 16);
    pin % 8
}

pub unsafe extern "C" fn exar_get_direction(chip: *mut GpioChip, offset: u32) -> i32 {
    let exar_gpio = gpiochip_get_data(chip);
    let addr = exar_offset_to_sel_addr(exar_gpio, offset);
    let bit = exar_offset_to_bit(exar_gpio, offset);
    if regmap_test_bits((*exar_gpio).regmap, addr, 1u32 << bit) != 0 { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT }
}

pub unsafe extern "C" fn exar_get_value(chip: *mut GpioChip, offset: u32) -> i32 {
    let exar_gpio = gpiochip_get_data(chip);
    let addr = exar_offset_to_lvl_addr(exar_gpio, offset);
    let bit = exar_offset_to_bit(exar_gpio, offset);
    (regmap_test_bits((*exar_gpio).regmap, addr, 1u32 << bit) != 0) as i32
}

pub unsafe extern "C" fn exar_set_value(chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let exar_gpio = gpiochip_get_data(chip);
    let addr = exar_offset_to_lvl_addr(exar_gpio, offset);
    let bit = exar_offset_to_bit(exar_gpio, offset);
    let bit_value = if value != 0 { 1u32 << bit } else { 0 };
    /* regmap_write_bits() forces value to be written when an external pull up/down might otherwise indicate value was already set. */
    regmap_write_bits((*exar_gpio).regmap, addr, 1u32 << bit, bit_value)
}

pub unsafe extern "C" fn exar_direction_output(chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let exar_gpio = gpiochip_get_data(chip);
    let addr = exar_offset_to_sel_addr(exar_gpio, offset);
    let bit = exar_offset_to_bit(exar_gpio, offset);
    let ret = exar_set_value(chip, offset, value);
    if ret != 0 { return ret; }
    regmap_clear_bits((*exar_gpio).regmap, addr, 1u32 << bit)
}

pub unsafe extern "C" fn exar_direction_input(chip: *mut GpioChip, offset: u32) -> i32 {
    let exar_gpio = gpiochip_get_data(chip);
    let addr = exar_offset_to_sel_addr(exar_gpio, offset);
    let bit = exar_offset_to_bit(exar_gpio, offset);
    regmap_set_bits((*exar_gpio).regmap, addr, 1u32 << bit);
    0
}

pub unsafe extern "C" fn exar_devm_ida_free(data: *mut core::ffi::c_void) {
    let exar_gpio = data as *mut ExarGpioChip;
    ida_free(&raw mut ida_index, (*exar_gpio).index);
}

extern "C" {
    pub static exar_regmap_config: RegmapConfig;
    pub fn to_pci_dev(parent: *mut Device) -> *mut PciDev;
    pub fn pcim_iomap_table(dev: *mut PciDev) -> *mut *mut core::ffi::c_void;
    pub fn device_property_read_u32(dev: *mut Device, name: *const u8, value: *mut u32) -> i32;
    pub fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    pub fn devm_regmap_init_mmio(dev: *mut Device, p: *mut core::ffi::c_void, config: *const RegmapConfig) -> *mut Regmap;
    pub fn ida_alloc(ida: *mut Ida, flags: u32) -> i32;
    pub fn devm_add_action_or_reset(dev: *mut Device, action: unsafe extern "C" fn(*mut core::ffi::c_void), data: *mut core::ffi::c_void) -> i32;
    pub fn sprintf(dst: *mut u8, fmt: *const u8, ...) -> i32;
    pub fn devm_gpiochip_add_data(dev: *mut Device, chip: *mut GpioChip, data: *mut core::ffi::c_void) -> i32;
}

pub unsafe extern "C" fn gpio_exar_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev as *mut Device;
    let pcidev = to_pci_dev((*dev).parent);
    let mut first_pin: u32 = 0;
    let mut ngpios: u32 = 0;
    let p = *pcim_iomap_table(pcidev);
    if p.is_null() { return -12; }

    let ret = device_property_read_u32(dev, b"exar,first-pin\0".as_ptr(), &mut first_pin);
    if ret != 0 { return ret; }
    let ret = device_property_read_u32(dev, b"ngpios\0".as_ptr(), &mut ngpios);
    if ret != 0 { return ret; }

    let exar_gpio = devm_kzalloc(dev, core::mem::size_of::<ExarGpioChip>(), 0) as *mut ExarGpioChip;
    if exar_gpio.is_null() { return -12; }

    /* If cascaded, secondary xr17v354 or xr17v358 have the same amount of MPIOs as their primaries and the last 4 bits of the primary's PCI Device ID is the number of its UART channels. */
    if (*pcidev).device & 0xf000 != 0 {
        ngpios += ngpios;
        (*exar_gpio).cascaded_offset = ((*pcidev).device & 0xf) * EXAR_UART_CHANNEL_SIZE;
    }

    (*exar_gpio).regmap = devm_regmap_init_mmio(dev, p, &exar_regmap_config);
    if (*exar_gpio).regmap.is_null() { return -22; }
    let index = ida_alloc(&raw mut ida_index, 0);
    if index < 0 { return index; }
    let ret = devm_add_action_or_reset(dev, exar_devm_ida_free, exar_gpio as *mut core::ffi::c_void);
    if ret != 0 { return ret; }
    sprintf((*exar_gpio).name.as_mut_ptr(), b"exar_gpio%d\0".as_ptr(), index);
    (*exar_gpio).gpio_chip.label = (*exar_gpio).name.as_mut_ptr();
    (*exar_gpio).gpio_chip.parent = dev;
    (*exar_gpio).gpio_chip.direction_output = Some(exar_direction_output);
    (*exar_gpio).gpio_chip.direction_input = Some(exar_direction_input);
    (*exar_gpio).gpio_chip.get_direction = Some(exar_get_direction);
    (*exar_gpio).gpio_chip.get = Some(exar_get_value);
    (*exar_gpio).gpio_chip.set = Some(exar_set_value);
    (*exar_gpio).gpio_chip.base = -1;
    (*exar_gpio).gpio_chip.ngpio = ngpios;
    (*exar_gpio).index = index;
    (*exar_gpio).first_pin = first_pin;
    let ret = devm_gpiochip_add_data(dev, &mut (*exar_gpio).gpio_chip, exar_gpio as *mut core::ffi::c_void);
    if ret != 0 { return ret; }
    0
}

// The following kernel types, constants, and registration macros are supplied by the kernel bindings.
#[repr(C)] pub struct Ida { _private: [u8; 0] }
#[repr(C)] pub struct Regmap { _private: [u8; 0] }
#[repr(C)] pub struct RegmapConfig { _private: [u8; 0] }
#[repr(C)] pub struct Device { pub parent: *mut Device }
#[repr(C)] pub struct PciDev { pub device: u16 }
#[repr(C)] pub struct PlatformDevice { pub dev: Device }
#[repr(C)] pub struct GpioChip {
    pub label: *mut u8, pub parent: *mut Device,
    pub direction_output: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32>,
    pub direction_input: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub get_direction: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32>,
    pub base: i32, pub ngpio: u32,
}
extern "C" {
    pub static GPIO_LINE_DIRECTION_IN: i32;
    pub static GPIO_LINE_DIRECTION_OUT: i32;
}

// module_platform_driver(gpio_exar_driver);
// MODULE_ALIAS("platform:" DRIVER_NAME);
// MODULE_DESCRIPTION("Exar GPIO driver");
// MODULE_AUTHOR("Sudip Mukherjee <sudip.mukherjee@codethink.co.uk>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
