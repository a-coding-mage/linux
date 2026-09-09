/* SPDX-License-Identifier: GPL-2.0-only */

// Translated from linux/gpio/regmap.h. Required kernel types are supplied by
// the surrounding translation unit.

use core::ffi::{c_ulong, c_void};

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct gpio_chip {
    _private: [u8; 0],
}
#[repr(C)]
pub struct irq_domain {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}
#[repr(C)]
pub struct regmap_irq_chip {
    _private: [u8; 0],
}

pub const GPIO_REGMAP_ADDR_ZERO: u32 = (-1i32) as u32;

#[inline]
pub const fn GPIO_REGMAP_ADDR(addr: u32) -> u32 {
    if addr != 0 { addr } else { GPIO_REGMAP_ADDR_ZERO }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum gpio_regmap_operation {
    GPIO_REGMAP_GET_OP,
    GPIO_REGMAP_SET_OP,
    GPIO_REGMAP_GET_DIR_OP,
    GPIO_REGMAP_SET_DIR_OP,
}

#[repr(C)]
pub struct gpio_regmap_config {
    pub parent: *mut device,
    pub regmap: *mut regmap,
    pub fwnode: *mut fwnode_handle,

    pub label: *const core::ffi::c_char,
    pub ngpio: i32,
    pub names: *const *const core::ffi::c_char,

    pub reg_dat_base: u32,
    pub reg_set_base: u32,
    pub reg_clr_base: u32,
    pub reg_dir_in_base: u32,
    pub reg_dir_out_base: u32,
    pub reg_stride: i32,
    pub ngpio_per_reg: i32,
    pub irq_domain: *mut irq_domain,
    pub fixed_direction_mask: *mut c_ulong,
    pub fixed_direction_output: *mut c_ulong,

    // Present when CONFIG_REGMAP_IRQ is enabled.
    #[cfg(CONFIG_REGMAP_IRQ)]
    pub regmap_irq_chip: *mut regmap_irq_chip,
    #[cfg(CONFIG_REGMAP_IRQ)]
    pub regmap_irq_line: i32,
    #[cfg(CONFIG_REGMAP_IRQ)]
    pub regmap_irq_flags: c_ulong,

    pub reg_mask_xlate: Option<unsafe extern "C" fn(
        gpio: *mut gpio_regmap,
        operation: gpio_regmap_operation,
        base: u32,
        offset: u32,
        reg: *mut u32,
        mask: *mut u32,
    ) -> i32>,
    pub init_valid_mask: Option<unsafe extern "C" fn(
        gc: *mut gpio_chip,
        valid_mask: *mut c_ulong,
        ngpios: u32,
    ) -> i32>,
    pub value_xlate: Option<unsafe extern "C" fn(
        gpio: *mut gpio_regmap,
        operation: gpio_regmap_operation,
        base: u32,
        offset: u32,
        reg: u32,
        mask: *mut u32,
        val: *mut u32,
    ) -> i32>,
    pub set_config: Option<unsafe extern "C" fn(
        gpio: *mut gpio_regmap,
        chip: *mut gpio_chip,
        offset: u32,
        config: c_ulong,
    ) -> i32>,

    pub drvdata: *mut c_void,
}

unsafe extern "C" {
    pub fn gpio_regmap_register(config: *const gpio_regmap_config) -> *mut gpio_regmap;
    pub fn gpio_regmap_unregister(gpio: *mut gpio_regmap);
    pub fn devm_gpio_regmap_register(
        dev: *mut device,
        config: *const gpio_regmap_config,
    ) -> *mut gpio_regmap;
    pub fn gpio_regmap_get_drvdata(gpio: *mut gpio_regmap) -> *mut c_void;

    pub fn gpio_regmap_reqres_irq(gpio: *mut gpio_regmap, offset: u32) -> i32;
    pub fn gpio_regmap_relres_irq(gpio: *mut gpio_regmap, offset: u32);

    pub fn gpio_regmap_enable_irq(gpio: *mut gpio_regmap, hwirq: irq_hw_number_t);
    pub fn gpio_regmap_disable_irq(gpio: *mut gpio_regmap, hwirq: irq_hw_number_t);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
