// SPDX-License-Identifier: GPL-2.0-only
/*
 * AppliedMicro X-Gene SoC GPIO Driver
 *
 * Copyright (c) 2014, Applied Micro Circuits Corporation
 * Author: Feng Kan <fkan@apm.com>.
 */

// Linux kernel dependencies supplied by other translation units.

const GPIO_SET_DR_OFFSET: usize = 0x0C;
const GPIO_DATA_OFFSET: usize = 0x14;
const GPIO_BANK_STRIDE: usize = 0x0C;

const XGENE_GPIOS_PER_BANK: usize = 16;
const XGENE_MAX_GPIO_BANKS: usize = 3;
const XGENE_MAX_GPIOS: usize = XGENE_GPIOS_PER_BANK * XGENE_MAX_GPIO_BANKS;

#[inline]
const fn gpio_bit_offset(x: usize) -> usize { x % XGENE_GPIOS_PER_BANK }

#[inline]
const fn gpio_bank_offset(x: usize) -> usize {
    (x / XGENE_GPIOS_PER_BANK) * GPIO_BANK_STRIDE
}

#[repr(C)]
pub struct XgeneGpio {
    pub chip: GpioChip,
    pub base: *mut u8,
    pub lock: Spinlock,
    pub set_dr_val: [u32; XGENE_MAX_GPIO_BANKS],
}

#[repr(C)]
pub struct GpioChip {
    pub ngpio: u32,
    pub parent: *mut Device,
    pub get_direction: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub direction_input: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub direction_output: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32)>,
    pub label: *const i8,
    pub base: i32,
}

#[repr(C)]
pub struct Spinlock { _private: [u8; 0] }
#[repr(C)]
pub struct Device { _private: [u8; 0] }
#[repr(C)]
pub struct PlatformDevice { _private: [u8; 0] }

extern "C" {
    fn gpiochip_get_data(gc: *mut GpioChip) -> *mut XgeneGpio;
    fn ioread32(addr: *mut u8) -> u32;
    fn iowrite32(value: u32, addr: *mut u8);
    fn spin_lock_irqsave(lock: *mut Spinlock, flags: *mut usize);
    fn spin_unlock_irqrestore(lock: *mut Spinlock, flags: usize);
    fn spin_lock_init(lock: *mut Spinlock);
    fn dev_get_drvdata(dev: *mut Device) -> *mut XgeneGpio;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut XgeneGpio;
    fn devm_platform_ioremap_resource(pdev: *mut PlatformDevice, index: u32) -> *mut u8;
    fn platform_set_drvdata(pdev: *mut PlatformDevice, data: *mut XgeneGpio);
    fn devm_gpiochip_add_data(dev: *mut Device, chip: *mut GpioChip, data: *mut XgeneGpio) -> i32;
    fn dev_name(dev: *mut Device) -> *const i8;
}

const GFP_KERNEL: u32 = 0;
const GPIO_LINE_DIRECTION_IN: i32 = 1;
const GPIO_LINE_DIRECTION_OUT: i32 = 0;
const ENOMEM: i32 = 12;

unsafe extern "C" fn xgene_gpio_get(gc: *mut GpioChip, offset: u32) -> i32 {
    let chip = gpiochip_get_data(gc);
    let bank_offset = GPIO_DATA_OFFSET + gpio_bank_offset(offset as usize);
    let bit_offset = gpio_bit_offset(offset as usize);
    (((ioread32((*chip).base.add(bank_offset)) & (1u32 << bit_offset)) != 0) as i32)
}

unsafe fn __xgene_gpio_set(gc: *mut GpioChip, offset: u32, val: i32) {
    let chip = gpiochip_get_data(gc);
    let bank_offset = GPIO_SET_DR_OFFSET + gpio_bank_offset(offset as usize);
    let bit_offset = gpio_bit_offset(offset as usize) + XGENE_GPIOS_PER_BANK;
    let mut setval = ioread32((*chip).base.add(bank_offset));
    if val != 0 { setval |= 1u32 << bit_offset; }
    else { setval &= !(1u32 << bit_offset); }
    iowrite32(setval, (*chip).base.add(bank_offset));
}

unsafe extern "C" fn xgene_gpio_set(gc: *mut GpioChip, offset: u32, val: i32) {
    let chip = gpiochip_get_data(gc);
    let mut flags = 0usize;
    spin_lock_irqsave(&mut (*chip).lock, &mut flags);
    __xgene_gpio_set(gc, offset, val);
    spin_unlock_irqrestore(&mut (*chip).lock, flags);
}

unsafe extern "C" fn xgene_gpio_get_direction(gc: *mut GpioChip, offset: u32) -> i32 {
    let chip = gpiochip_get_data(gc);
    let bank_offset = GPIO_SET_DR_OFFSET + gpio_bank_offset(offset as usize);
    let bit_offset = gpio_bit_offset(offset as usize);
    if ioread32((*chip).base.add(bank_offset)) & (1u32 << bit_offset) != 0 { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT }
}

unsafe extern "C" fn xgene_gpio_dir_in(gc: *mut GpioChip, offset: u32) -> i32 {
    let chip = gpiochip_get_data(gc);
    let mut flags = 0usize;
    let bank_offset = GPIO_SET_DR_OFFSET + gpio_bank_offset(offset as usize);
    let bit_offset = gpio_bit_offset(offset as usize);
    spin_lock_irqsave(&mut (*chip).lock, &mut flags);
    let dirval = ioread32((*chip).base.add(bank_offset)) | (1u32 << bit_offset);
    iowrite32(dirval, (*chip).base.add(bank_offset));
    spin_unlock_irqrestore(&mut (*chip).lock, flags);
    0
}

unsafe extern "C" fn xgene_gpio_dir_out(gc: *mut GpioChip, offset: u32, val: i32) -> i32 {
    let chip = gpiochip_get_data(gc);
    let mut flags = 0usize;
    let bank_offset = GPIO_SET_DR_OFFSET + gpio_bank_offset(offset as usize);
    let bit_offset = gpio_bit_offset(offset as usize);
    spin_lock_irqsave(&mut (*chip).lock, &mut flags);
    let dirval = ioread32((*chip).base.add(bank_offset)) & !(1u32 << bit_offset);
    iowrite32(dirval, (*chip).base.add(bank_offset));
    __xgene_gpio_set(gc, offset, val);
    spin_unlock_irqrestore(&mut (*chip).lock, flags);
    0
}

unsafe extern "C" fn xgene_gpio_suspend(dev: *mut Device) -> i32 {
    let gpio = dev_get_drvdata(dev);
    for bank in 0..XGENE_MAX_GPIO_BANKS {
        let bank_offset = GPIO_SET_DR_OFFSET + bank * GPIO_BANK_STRIDE;
        (*gpio).set_dr_val[bank] = ioread32((*gpio).base.add(bank_offset));
    }
    0
}

unsafe extern "C" fn xgene_gpio_resume(dev: *mut Device) -> i32 {
    let gpio = dev_get_drvdata(dev);
    for bank in 0..XGENE_MAX_GPIO_BANKS {
        let bank_offset = GPIO_SET_DR_OFFSET + bank * GPIO_BANK_STRIDE;
        iowrite32((*gpio).set_dr_val[bank], (*gpio).base.add(bank_offset));
    }
    0
}

unsafe extern "C" fn xgene_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let gpio = devm_kzalloc(pdev as *mut Device, core::mem::size_of::<XgeneGpio>(), GFP_KERNEL);
    if gpio.is_null() { return -ENOMEM; }
    (*gpio).base = devm_platform_ioremap_resource(pdev, 0);
    (*gpio).chip.ngpio = XGENE_MAX_GPIOS as u32;
    spin_lock_init(&mut (*gpio).lock);
    (*gpio).chip.parent = pdev as *mut Device;
    (*gpio).chip.get_direction = Some(xgene_gpio_get_direction);
    (*gpio).chip.direction_input = Some(xgene_gpio_dir_in);
    (*gpio).chip.direction_output = Some(xgene_gpio_dir_out);
    (*gpio).chip.get = Some(xgene_gpio_get);
    (*gpio).chip.set = Some(xgene_gpio_set);
    (*gpio).chip.label = dev_name(pdev as *mut Device);
    (*gpio).chip.base = -1;
    platform_set_drvdata(pdev, gpio);
    devm_gpiochip_add_data(pdev as *mut Device, &mut (*gpio).chip, gpio)
}

#[repr(C)]
pub struct OfDeviceId { pub compatible: *const i8 }

#[repr(C)]
pub struct AcpiDeviceId { pub id: *const i8, pub driver_data: usize }

static XGENE_GPIO_OF_MATCH: [OfDeviceId; 2] = [
    OfDeviceId { compatible: c"apm,xgene-gpio".as_ptr() },
    OfDeviceId { compatible: core::ptr::null() },
];

#[cfg(feature = "CONFIG_ACPI")]
static XGENE_GPIO_ACPI_MATCH: [AcpiDeviceId; 2] = [
    AcpiDeviceId { id: c"APMC0D14".as_ptr(), driver_data: 0 },
    AcpiDeviceId { id: core::ptr::null(), driver_data: 0 },
];

#[repr(C)]
pub struct PlatformDriver {
    pub name: *const i8,
    pub of_match_table: *const OfDeviceId,
    pub acpi_match_table: *const AcpiDeviceId,
    pub probe: Option<unsafe extern "C" fn(*mut PlatformDevice) -> i32>,
}

static XGENE_GPIO_DRIVER: PlatformDriver = PlatformDriver {
    name: c"xgene-gpio".as_ptr(),
    of_match_table: XGENE_GPIO_OF_MATCH.as_ptr(),
    acpi_match_table: core::ptr::null(),
    probe: Some(xgene_gpio_probe),
};

// Equivalent of builtin_platform_driver(xgene_gpio_driver).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
