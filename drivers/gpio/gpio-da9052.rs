// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * GPIO Driver for Dialog DA9052 PMICs.
 *
 * Copyright(c) 2011 Dialog Semiconductor Ltd.
 *
 * Author: David Dajun Chen <dchen@diasemi.com>
 */

// Linux kernel headers and symbols are supplied by the surrounding crate.

const DA9052_INPUT: i32 = 1;
const DA9052_OUTPUT_OPENDRAIN: i32 = 2;
const DA9052_OUTPUT_PUSHPULL: i32 = 3;
const DA9052_SUPPLY_VDD_IO1: i32 = 0;
const DA9052_DEBOUNCING_OFF: i32 = 0;
const DA9052_DEBOUNCING_ON: i32 = 1;
const DA9052_OUTPUT_LOWLEVEL: i32 = 0;
const DA9052_ACTIVE_LOW: i32 = 0;
const DA9052_ACTIVE_HIGH: i32 = 1;
const DA9052_GPIO_MAX_PORTS_PER_REGISTER: u32 = 8;
const DA9052_GPIO_MASK_UPPER_NIBBLE: i32 = 0xf0;
const DA9052_GPIO_MASK_LOWER_NIBBLE: i32 = 0x0f;
const DA9052_GPIO_NIBBLE_SHIFT: i32 = 4;
const DA9052_IRQ_GPI0: u32 = 16;
const DA9052_GPIO_ODD_SHIFT: i32 = 7;
const DA9052_GPIO_EVEN_SHIFT: i32 = 3;

#[repr(C)]
pub struct da9052_gpio {
    pub da9052: *mut da9052,
    pub gp: gpio_chip,
}

extern "C" {
    type da9052;
    type platform_device;
    type da9052_pdata;

    fn gpiochip_get_data(gc: *mut gpio_chip) -> *mut core::ffi::c_void;
    fn da9052_reg_read(da9052: *mut da9052, reg: u32) -> i32;
    fn da9052_reg_update(da9052: *mut da9052, reg: u32, mask: u32, value: i32) -> i32;
    fn regmap_irq_get_virq(data: *mut core::ffi::c_void, irq: u32) -> i32;
    fn devm_kzalloc(dev: *mut core::ffi::c_void, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn dev_get_drvdata(dev: *mut core::ffi::c_void) -> *mut da9052;
    fn dev_get_platdata(dev: *mut core::ffi::c_void) -> *mut da9052_pdata;
    fn devm_gpiochip_add_data(dev: *mut core::ffi::c_void, gc: *mut gpio_chip, data: *mut da9052_gpio) -> i32;
}

#[repr(C)]
pub struct gpio_chip {
    pub label: *const u8,
    pub owner: *mut core::ffi::c_void,
    pub get: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub set: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    pub direction_input: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub direction_output: Option<unsafe extern "C" fn(*mut gpio_chip, u32, i32) -> i32>,
    pub to_irq: Option<unsafe extern "C" fn(*mut gpio_chip, u32) -> i32>,
    pub can_sleep: bool,
    pub ngpio: u32,
    pub base: i32,
}

const DA9052_GPIO_0_1_REG: u32 = 0;
const DA9052_STATUS_C_REG: u32 = 0;
const DA9052_STATUS_D_REG: u32 = 0;
const DA9052_ODD_PORT_PIN: i32 = 0;
const DA9052_EVEN_PORT_PIN: i32 = 0;
const DA9052_ODD_PORT_MODE: u32 = 0;
const DA9052_EVEN_PORT_MODE: u32 = 0;

#[inline]
unsafe fn da9052_gpio_port_odd(offset: u32) -> u32 { offset % 2 }

unsafe extern "C" fn da9052_gpio_get(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut da9052_gpio;
    let mut direction = 0i32;
    let mut ret = da9052_reg_read((*gpio).da9052, DA9052_GPIO_0_1_REG + (offset >> 1));
    if ret < 0 { return ret; }
    if da9052_gpio_port_odd(offset) != 0 {
        direction = (ret & DA9052_ODD_PORT_PIN) >> 4;
    } else { direction = ret & DA9052_EVEN_PORT_PIN; }
    match direction {
        DA9052_INPUT => {
            ret = if offset < DA9052_GPIO_MAX_PORTS_PER_REGISTER { da9052_reg_read((*gpio).da9052, DA9052_STATUS_C_REG) } else { da9052_reg_read((*gpio).da9052, DA9052_STATUS_D_REG) };
            if ret < 0 { return ret; }
            (ret & (1 << (offset % 8)) != 0) as i32
        }
        DA9052_OUTPUT_PUSHPULL => if da9052_gpio_port_odd(offset) != 0 { (ret & DA9052_ODD_PORT_MODE as i32 != 0) as i32 } else { (ret & DA9052_EVEN_PORT_MODE as i32 != 0) as i32 },
        _ => -22,
    }
}

unsafe extern "C" fn da9052_gpio_set(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut da9052_gpio;
    let (mask, shift) = if da9052_gpio_port_odd(offset) != 0 { (DA9052_ODD_PORT_MODE, DA9052_GPIO_ODD_SHIFT) } else { (DA9052_GPIO_EVEN_PORT_MODE, DA9052_GPIO_EVEN_SHIFT) };
    da9052_reg_update((*gpio).da9052, (offset >> 1) + DA9052_GPIO_0_1_REG, mask, value << shift)
}

unsafe extern "C" fn da9052_gpio_direction_input(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut da9052_gpio;
    let value = DA9052_INPUT | (DA9052_ACTIVE_LOW << 2) | (DA9052_DEBOUNCING_ON << 3);
    if da9052_gpio_port_odd(offset) != 0 { da9052_reg_update((*gpio).da9052, (offset >> 1) + DA9052_GPIO_0_1_REG, DA9052_GPIO_MASK_UPPER_NIBBLE as u32, value << DA9052_GPIO_NIBBLE_SHIFT) } else { da9052_reg_update((*gpio).da9052, (offset >> 1) + DA9052_GPIO_0_1_REG, DA9052_GPIO_MASK_LOWER_NIBBLE as u32, value) }
}

unsafe extern "C" fn da9052_gpio_direction_output(gc: *mut gpio_chip, offset: u32, value: i32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut da9052_gpio;
    let register_value = DA9052_OUTPUT_PUSHPULL | (DA9052_SUPPLY_VDD_IO1 << 2) | (value << 3);
    if da9052_gpio_port_odd(offset) != 0 { da9052_reg_update((*gpio).da9052, (offset >> 1) + DA9052_GPIO_0_1_REG, DA9052_GPIO_MASK_UPPER_NIBBLE as u32, register_value << DA9052_GPIO_NIBBLE_SHIFT) } else { da9052_reg_update((*gpio).da9052, (offset >> 1) + DA9052_GPIO_0_1_REG, DA9052_GPIO_MASK_LOWER_NIBBLE as u32, register_value) }
}

unsafe extern "C" fn da9052_gpio_to_irq(gc: *mut gpio_chip, offset: u32) -> i32 {
    let gpio = gpiochip_get_data(gc) as *mut da9052_gpio;
    regmap_irq_get_virq(core::ptr::null_mut(), DA9052_IRQ_GPI0 + offset)
}

static REFERENCE_GP: gpio_chip = gpio_chip {
    label: b"da9052-gpio\0".as_ptr(), owner: core::ptr::null_mut(),
    get: Some(da9052_gpio_get), set: Some(da9052_gpio_set),
    direction_input: Some(da9052_gpio_direction_input), direction_output: Some(da9052_gpio_direction_output),
    to_irq: Some(da9052_gpio_to_irq), can_sleep: true, ngpio: 16, base: -1,
};

#[allow(dead_code)]
unsafe extern "C" fn da9052_gpio_probe(pdev: *mut platform_device) -> i32 {
    let gpio = devm_kzalloc(core::ptr::null_mut(), core::mem::size_of::<da9052_gpio>(), 0) as *mut da9052_gpio;
    if gpio.is_null() { return -12; }
    (*gpio).da9052 = dev_get_drvdata(core::ptr::null_mut());
    (*gpio).gp = REFERENCE_GP;
    devm_gpiochip_add_data(core::ptr::null_mut(), &mut (*gpio).gp, gpio)
}

// Equivalent of module_platform_driver(da9052_gpio_driver).
// MODULE_AUTHOR("David Dajun Chen <dchen@diasemi.com>");
// MODULE_DESCRIPTION("DA9052 GPIO Device Driver");
// MODULE_LICENSE("GPL");
// MODULE_ALIAS("platform:da9052-gpio");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
