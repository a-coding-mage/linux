/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright 2022 William Breathitt Gray */

// Forward declarations corresponding to the C header's external types.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_irq {
    _private: [u8; 0],
}

/**
 * struct idio_16_regmap_config - Configuration for the IDIO-16 register map
 * @parent:          parent device
 * @map:             regmap for the IDIO-16 device
 * @regmap_irqs:     descriptors for individual IRQs
 * @num_regmap_irqs: number of IRQ descriptors
 * @irq:             IRQ number for the IDIO-16 device
 * @no_status:       device has no status register
 * @filters:         device has input filters
 */
#[repr(C)]
pub struct idio_16_regmap_config {
    pub parent: *mut device,
    pub map: *mut regmap,
    pub regmap_irqs: *const regmap_irq,
    pub num_regmap_irqs: ::core::ffi::c_int,
    pub irq: ::core::ffi::c_uint,
    pub no_status: bool,
    pub filters: bool,
}

pub unsafe extern "C" fn devm_idio_16_regmap_register(
    dev: *mut device,
    config: *const idio_16_regmap_config,
) -> ::core::ffi::c_int;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
