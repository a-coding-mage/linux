/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2022 William Breathitt Gray */

// Opaque declarations supplied by other translation units.
#[repr(C)]
pub struct device {
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

// Corresponds to: regmap_reg_range(_base, _base + 0x2)
#[macro_export]
macro_rules! i8255_volatile_regmap_range {
    ($base:expr) => {
        regmap_reg_range($base, $base + 0x2)
    };
}

/**
 * Configuration for the register map of an i8255
 * @parent: parent device
 * @map: regmap for the i8255
 * @num_ppi: number of i8255 Programmable Peripheral Interface
 * @names: (optional) array of names for gpios
 * @domain: (optional) IRQ domain if the controller is interrupt-capable
 *
 * Note: The regmap is expected to have cache enabled and i8255 control
 * registers not marked as volatile.
 */
#[repr(C)]
pub struct i8255_regmap_config {
    pub parent: *mut device,
    pub map: *mut regmap,
    pub num_ppi: core::ffi::c_int,
    pub names: *const *const core::ffi::c_char,
    pub domain: *mut irq_domain,
}

unsafe extern "C" {
    pub fn devm_i8255_regmap_register(
        dev: *mut device,
        config: *const i8255_regmap_config,
    ) -> core::ffi::c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
