/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (C) 2024, KEBA Industrial Automation Gmbh */

// Dependency supplied by <linux/auxiliary_bus.h>.
// The following types are declared by the surrounding kernel translation.

#[repr(C)]
pub struct keba_i2c_auxdev {
    pub auxdev: auxiliary_device,
    pub io: resource,
    pub info_size: core::ffi::c_int,
    pub info: *mut i2c_board_info,
}

#[repr(C)]
pub struct keba_spi_auxdev {
    pub auxdev: auxiliary_device,
    pub io: resource,
    pub info_size: core::ffi::c_int,
    pub info: *mut spi_board_info,
}

#[repr(C)]
pub struct keba_fan_auxdev {
    pub auxdev: auxiliary_device,
    pub io: resource,
}

#[repr(C)]
pub struct keba_batt_auxdev {
    pub auxdev: auxiliary_device,
    pub io: resource,
}

#[repr(C)]
pub struct keba_uart_auxdev {
    pub auxdev: auxiliary_device,
    pub io: resource,
    pub irq: core::ffi::c_uint,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
