// SPDX-License-Identifier: GPL-2.0
// Copyright (c) 2019, Linaro Limited

// Dependencies supplied by the Linux kernel bindings.

const WCD_REG_DIR_CTL_OFFSET: u32 = 0x42;
const WCD_REG_VAL_CTL_OFFSET: u32 = 0x43;
const WCD934X_NPINS: u32 = 5;

#[inline]
const fn wcd_pin_mask(p: u32) -> u32 {
    1u32 << p
}

#[repr(C)]
pub struct Regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct Device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct PlatformDevice {
    pub dev: Device,
}

#[repr(C)]
pub struct GpioChip {
    pub direction_input: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub direction_output: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32>,
    pub get_direction: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub get: Option<unsafe extern "C" fn(*mut GpioChip, u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut GpioChip, u32, i32)>,
    pub parent: *mut Device,
    pub base: i32,
    pub ngpio: u32,
    pub label: *const core::ffi::c_char,
    pub can_sleep: bool,
}

#[repr(C)]
pub struct WcdGpioData {
    pub map: *mut Regmap,
    pub chip: GpioChip,
}

extern "C" {
    fn gpiochip_get_data(chip: *mut GpioChip) -> *mut WcdGpioData;
    fn regmap_read(map: *mut Regmap, reg: u32, value: *mut u32) -> i32;
    fn regmap_update_bits(map: *mut Regmap, reg: u32, mask: u32, value: u32) -> i32;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dev_get_regmap(dev: *mut Device, name: *const core::ffi::c_char) -> *mut Regmap;
    fn dev_err(dev: *mut Device, fmt: *const core::ffi::c_char, ...);
    fn dev_name(dev: *mut Device) -> *const core::ffi::c_char;
    fn devm_gpiochip_add_data(
        dev: *mut Device,
        chip: *mut GpioChip,
        data: *mut WcdGpioData,
    ) -> i32;
}

const GFP_KERNEL: u32 = 0;
const GPIO_LINE_DIRECTION_OUT: i32 = 1;
const GPIO_LINE_DIRECTION_IN: i32 = 0;

unsafe extern "C" fn wcd_gpio_get_direction(chip: *mut GpioChip, pin: u32) -> i32 {
    let data = gpiochip_get_data(chip);
    let mut value: u32 = 0;
    let ret = regmap_read((*data).map, WCD_REG_DIR_CTL_OFFSET, &mut value);
    if ret < 0 {
        return ret;
    }

    if value & wcd_pin_mask(pin) != 0 {
        return GPIO_LINE_DIRECTION_OUT;
    }

    GPIO_LINE_DIRECTION_IN
}

unsafe extern "C" fn wcd_gpio_direction_input(chip: *mut GpioChip, pin: u32) -> i32 {
    let data = gpiochip_get_data(chip);
    regmap_update_bits(
        (*data).map,
        WCD_REG_DIR_CTL_OFFSET,
        wcd_pin_mask(pin),
        0,
    )
}

unsafe extern "C" fn wcd_gpio_direction_output(
    chip: *mut GpioChip,
    pin: u32,
    val: i32,
) -> i32 {
    let data = gpiochip_get_data(chip);
    let ret = regmap_update_bits(
        (*data).map,
        WCD_REG_DIR_CTL_OFFSET,
        wcd_pin_mask(pin),
        wcd_pin_mask(pin),
    );
    if ret != 0 {
        return ret;
    }

    regmap_update_bits(
        (*data).map,
        WCD_REG_VAL_CTL_OFFSET,
        wcd_pin_mask(pin),
        if val != 0 { wcd_pin_mask(pin) } else { 0 },
    )
}

unsafe extern "C" fn wcd_gpio_get(chip: *mut GpioChip, pin: u32) -> i32 {
    let data = gpiochip_get_data(chip);
    let mut value: u32 = 0;
    regmap_read((*data).map, WCD_REG_VAL_CTL_OFFSET, &mut value);
    if value & wcd_pin_mask(pin) != 0 { 1 } else { 0 }
}

unsafe extern "C" fn wcd_gpio_set(chip: *mut GpioChip, pin: u32, val: i32) {
    let data = gpiochip_get_data(chip);
    let _ = regmap_update_bits(
        (*data).map,
        WCD_REG_VAL_CTL_OFFSET,
        wcd_pin_mask(pin),
        if val != 0 { wcd_pin_mask(pin) } else { 0 },
    );
}

unsafe extern "C" fn wcd_gpio_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev as *mut Device;
    let data = devm_kzalloc(dev, core::mem::size_of::<WcdGpioData>(), GFP_KERNEL)
        as *mut WcdGpioData;
    if data.is_null() {
        return -12; // -ENOMEM
    }

    (*data).map = dev_get_regmap(core::ptr::null_mut(), core::ptr::null());
    if (*data).map.is_null() {
        // dev_err(dev, "%s: failed to get regmap\n", __func__);
        return -22; // -EINVAL
    }

    let chip = &mut (*data).chip as *mut GpioChip;
    (*chip).direction_input = Some(wcd_gpio_direction_input);
    (*chip).direction_output = Some(wcd_gpio_direction_output);
    (*chip).get_direction = Some(wcd_gpio_get_direction);
    (*chip).get = Some(wcd_gpio_get);
    (*chip).set = Some(wcd_gpio_set);
    (*chip).parent = dev;
    (*chip).base = -1;
    (*chip).ngpio = WCD934X_NPINS;
    (*chip).label = dev_name(dev);
    (*chip).can_sleep = true;

    devm_gpiochip_add_data(dev, chip, data)
}

// The following device-table and platform-driver declarations correspond to
// the Linux kernel's of_device_id, platform_driver, and module macros.
#[repr(C)]
struct OfDeviceId {
    compatible: *const core::ffi::c_char,
}

static WCD_GPIO_OF_MATCH: [OfDeviceId; 3] = [
    OfDeviceId { compatible: b"qcom,wcd9340-gpio\0".as_ptr() as *const _ },
    OfDeviceId { compatible: b"qcom,wcd9341-gpio\0".as_ptr() as *const _ },
    OfDeviceId { compatible: core::ptr::null() },
];

// MODULE_DEVICE_TABLE(of, wcd_gpio_of_match);
// module_platform_driver(wcd_gpio_driver);
// MODULE_DESCRIPTION("Qualcomm Technologies, Inc WCD GPIO control driver");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
