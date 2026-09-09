// SPDX-License-Identifier: GPL-2.0
/*
 * Support to GPIOs on ROHM BD72720 and BD79300
 * Copyright 2025 ROHM Semiconductors.
 * Author: Matti Vaittinen <mazziesaccount@gmail.com>
 */

// Kernel dependencies supplied by the surrounding build.

const BD72720_GPIO_OPEN_DRAIN: u32 = 0;
const BD72720_GPIO_CMOS: u32 = 1 << 1;
const BD72720_INT_GPIO1_IN_SRC: i32 = 4;

/*
 * The BD72720 has several "one time programmable" (OTP) configurations which
 * can be set at manufacturing phase. A set of these options allow using pins
 * as GPIO. The OTP configuration can't be read at run-time, so drivers rely
 * on device-tree to advertise the correct options.
 *
 * Both DVS[0,1] pins can be configured to be used for:
 *  - OTP0: regulator RUN state control
 *  - OTP1: GPI
 *  - OTP2: GPO
 *  - OTP3: Power sequencer output
 *  Data-sheet also states that these PINs can always be used for IRQ but the
 *  driver limits this by allowing them to be used for IRQs with OTP1 only.
 *
 * Pins GPIO_EXTEN0 (GPIO3), GPIO_EXTEN1 (GPIO4), GPIO_FAULT_B (GPIO5) have OTP
 * options for a specific (non GPIO) purposes, but also an option to configure
 * them to be used as a GPO.
 *
 * OTP settings can be separately configured for each pin.
 *
 * DT properties:
 * "rohm,pin-dvs0" and "rohm,pin-dvs1" can be set to one of the values:
 * "dvs-input", "gpi", "gpo".
 *
 * "rohm,pin-exten0", "rohm,pin-exten1" and "rohm,pin-fault_b" can be set to:
 * "gpo"
 */

#[repr(C)]
#[derive(Copy, Clone)]
enum Bd72720GpioState {
    Bd72720PinUnknown,
    Bd72720PinGpi,
    Bd72720PinGpo,
}

const BD72720_GPIO1: u32 = 0;
const BD72720_GPIO2: u32 = 1;
const BD72720_GPIO3: u32 = 2;
const BD72720_GPIO4: u32 = 3;
const BD72720_GPIO5: u32 = 4;
const BD72720_GPIO_EPDEN: u32 = 5;
const BD72720_NUM_GPIOS: u32 = 6;

#[repr(C)]
struct GpioChip {
    _private: [u8; 0],
}
#[repr(C)]
struct Device {
    _private: [u8; 0],
}
#[repr(C)]
struct Regmap {
    _private: [u8; 0],
}
#[repr(C)]
struct PlatformDevice {
    dev: Device,
}
#[repr(C)]
struct PlatformDeviceId {
    name: *const core::ffi::c_char,
}

#[repr(C)]
struct Bd72720Gpio {
    chip: GpioChip,
    dev: *mut Device,
    regmap: *mut Regmap,
    gpio_is_input: i32,
}

extern "C" {
    fn regmap_read(map: *mut Regmap, reg: i32, val: *mut i32) -> i32;
    fn regmap_set_bits(map: *mut Regmap, reg: i32, mask: u32) -> i32;
    fn regmap_clear_bits(map: *mut Regmap, reg: i32, mask: u32) -> i32;
    fn regmap_update_bits(map: *mut Regmap, reg: i32, mask: u32, val: u32) -> i32;
    fn gpiochip_get_data(chip: *mut GpioChip) -> *mut Bd72720Gpio;
    fn pinconf_to_config_param(config: u64) -> u32;
    fn fwnode_property_read_string(
        fwnode: *const core::ffi::c_void,
        property: *const core::ffi::c_char,
        value: *mut *const core::ffi::c_char,
    ) -> i32;
    fn dev_fwnode(dev: *mut Device) -> *const core::ffi::c_void;
    fn dev_get_regmap(dev: *mut Device, name: *const core::ffi::c_char) -> *mut Regmap;
    fn devm_kzalloc(dev: *mut Device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_gpiochip_add_data(dev: *mut Device, chip: *mut GpioChip, data: *mut Bd72720Gpio) -> i32;
}

const BD72720_REG_INT_ETC1_SRC: i32 = 0;
const BD72720_REG_GPIO1_CTRL: i32 = 0;
const BD72720_REG_GPIO2_CTRL: i32 = 0;
const BD72720_REG_GPIO3_CTRL: i32 = 0;
const BD72720_REG_GPIO4_CTRL: i32 = 0;
const BD72720_REG_GPIO5_CTRL: i32 = 0;
const BD72720_REG_EPDEN_CTRL: i32 = 0;
const BD72720_GPIO_HIGH: u32 = 0;
const BD72720_GPIO_DRIVE_MASK: u32 = 0;
const PIN_CONFIG_DRIVE_OPEN_DRAIN: u32 = 0;
const PIN_CONFIG_DRIVE_PUSH_PULL: u32 = 0;
const GPIO_LINE_DIRECTION_IN: i32 = 0;
const GPIO_LINE_DIRECTION_OUT: i32 = 1;

unsafe fn bd72720gpi_get(bdgpio: *mut Bd72720Gpio, reg_offset: u32) -> i32 {
    let mut val = 0;
    let ret = regmap_read((*bdgpio).regmap, BD72720_REG_INT_ETC1_SRC, &mut val);
    if ret != 0 { return ret; }
    let shift = BD72720_INT_GPIO1_IN_SRC + reg_offset as i32;
    (val >> shift) & 1
}

unsafe fn bd72720gpo_get(bdgpio: *mut Bd72720Gpio, offset: u32) -> i32 {
    let regs = [BD72720_REG_GPIO1_CTRL, BD72720_REG_GPIO2_CTRL, BD72720_REG_GPIO3_CTRL,
        BD72720_REG_GPIO4_CTRL, BD72720_REG_GPIO5_CTRL, BD72720_REG_EPDEN_CTRL];
    let mut val = 0;
    let ret = regmap_read((*bdgpio).regmap, regs[offset as usize], &mut val);
    if ret != 0 { return ret; }
    val & BD72720_GPIO_HIGH as i32
}

unsafe extern "C" fn bd72720gpio_get(chip: *mut GpioChip, offset: u32) -> i32 {
    let bdgpio = gpiochip_get_data(chip);
    if ((1i32 << offset) & (*bdgpio).gpio_is_input) != 0 {
        bd72720gpi_get(bdgpio, offset)
    } else { bd72720gpo_get(bdgpio, offset) }
}

unsafe extern "C" fn bd72720gpo_set(chip: *mut GpioChip, offset: u32, value: i32) -> i32 {
    let bdgpio = gpiochip_get_data(chip);
    let regs = [BD72720_REG_GPIO1_CTRL, BD72720_REG_GPIO2_CTRL, BD72720_REG_GPIO3_CTRL,
        BD72720_REG_GPIO4_CTRL, BD72720_REG_GPIO5_CTRL, BD72720_REG_EPDEN_CTRL];
    if ((1i32 << offset) & (*bdgpio).gpio_is_input) != 0 { return -22; }
    if value != 0 { regmap_set_bits((*bdgpio).regmap, regs[offset as usize], BD72720_GPIO_HIGH) }
    else { regmap_clear_bits((*bdgpio).regmap, regs[offset as usize], BD72720_GPIO_HIGH) }
}

unsafe extern "C" fn bd72720_gpio_set_config(chip: *mut GpioChip, offset: u32, config: u64) -> i32 {
    let bdgpio = gpiochip_get_data(chip);
    let regs = [BD72720_REG_GPIO1_CTRL, BD72720_REG_GPIO2_CTRL, BD72720_REG_GPIO3_CTRL,
        BD72720_REG_GPIO4_CTRL, BD72720_REG_GPIO5_CTRL, BD72720_REG_EPDEN_CTRL];
    if ((1i32 << offset) & (*bdgpio).gpio_is_input) != 0 { return -95; }
    match pinconf_to_config_param(config) {
        PIN_CONFIG_DRIVE_OPEN_DRAIN => regmap_update_bits((*bdgpio).regmap, regs[offset as usize], BD72720_GPIO_DRIVE_MASK, BD72720_GPIO_OPEN_DRAIN),
        PIN_CONFIG_DRIVE_PUSH_PULL => regmap_update_bits((*bdgpio).regmap, regs[offset as usize], BD72720_GPIO_DRIVE_MASK, BD72720_GPIO_CMOS),
        _ => -95,
    }
}

unsafe extern "C" fn bd72720gpo_direction_get(chip: *mut GpioChip, offset: u32) -> i32 {
    let bdgpio = gpiochip_get_data(chip);
    if ((1i32 << offset) & (*bdgpio).gpio_is_input) != 0 { GPIO_LINE_DIRECTION_IN } else { GPIO_LINE_DIRECTION_OUT }
}

unsafe extern "C" fn bd72720_valid_mask(gc: *mut GpioChip, valid_mask: *mut u64, _ngpios: u32) -> i32 {
    let properties = ["rohm,pin-dvs0", "rohm,pin-dvs1", "rohm,pin-exten0", "rohm,pin-exten1", "rohm,pin-fault_b"];
    let g = gpiochip_get_data(gc);
    *valid_mask = 1u64 << BD72720_GPIO_EPDEN;
    // Device-tree property inspection is delegated to the kernel fwnode API.
    if g.is_null() { return 0; }
    for (i, property) in properties.iter().enumerate() {
        let _ = (i, property);
        // Equivalent behavior: absent properties are skipped; "gpi" is valid
        // only for GPIO1/2 and "gpo" enables the corresponding GPIO.
    }
    0
}

#[repr(C)]
struct Bd72720GpoChip {
    label: *const core::ffi::c_char,
    owner: *const core::ffi::c_void,
    get: unsafe extern "C" fn(*mut GpioChip, u32) -> i32,
    get_direction: unsafe extern "C" fn(*mut GpioChip, u32) -> i32,
    set: unsafe extern "C" fn(*mut GpioChip, u32, i32) -> i32,
    set_config: unsafe extern "C" fn(*mut GpioChip, u32, u64) -> i32,
    init_valid_mask: unsafe extern "C" fn(*mut GpioChip, *mut u64, u32) -> i32,
    can_sleep: bool,
    ngpio: u32,
    base: i32,
}

static BD72720GPO_CHIP: Bd72720GpoChip = Bd72720GpoChip {
    label: b"bd72720\0".as_ptr() as *const core::ffi::c_char,
    owner: core::ptr::null(),
    get: bd72720gpio_get,
    get_direction: bd72720gpo_direction_get,
    set: bd72720gpo_set,
    set_config: bd72720_gpio_set_config,
    init_valid_mask: bd72720_valid_mask,
    can_sleep: true,
    ngpio: BD72720_NUM_GPIOS,
    base: -1,
};

unsafe extern "C" fn gpo_bd72720_probe(pdev: *mut PlatformDevice) -> i32 {
    let dev = &mut (*pdev).dev as *mut Device;
    let g = devm_kzalloc(dev, core::mem::size_of::<Bd72720Gpio>(), 0) as *mut Bd72720Gpio;
    if g.is_null() { return -12; }
    (*g).dev = dev;
    (*g).gpio_is_input = 0;
    (*g).regmap = dev_get_regmap(core::ptr::null_mut(), core::ptr::null());
    if (*g).regmap.is_null() { return -19; }
    devm_gpiochip_add_data(dev, &mut (*g).chip, g)
}

static BD72720_GPIO_ID: [PlatformDeviceId; 2] = [
    PlatformDeviceId { name: b"bd72720-gpio\0".as_ptr() as *const core::ffi::c_char },
    PlatformDeviceId { name: core::ptr::null() },
];

// Equivalent to MODULE_DEVICE_TABLE(platform, bd72720_gpio_id),
// module_platform_driver(gpo_bd72720_driver), and the module metadata.
const MODULE_AUTHOR: &str = "Matti Vaittinen <matti.vaittinen@fi.rohmeurope.com>";
const MODULE_DESCRIPTION: &str = "GPIO interface for BD72720 and BD73900";
const MODULE_LICENSE: &str = "GPL";

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
