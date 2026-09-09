/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Core MFD defines for ATC260x PMICs
 *
 * Copyright (C) 2019 Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>
 * Copyright (C) 2020 Cristian Ciocaltea <cristian.ciocaltea@gmail.com>
 */

use core::ffi::{c_char, c_int, c_uint, c_void};

// Declarations supplied by the included ATC260x and Linux headers.
pub enum device {}
pub enum regmap {}
pub enum regmap_irq_chip {}
pub enum regmap_irq_chip_data {}
pub enum mutex {}
pub enum mfd_cell {}
pub enum atc260x_init_regs {}
pub enum regmap_config {}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum atc260x_type {
    ATC2603A = 0,
    ATC2603C,
    ATC2609A,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum atc260x_ver {
    ATC260X_A = 0,
    ATC260X_B,
    ATC260X_C,
    ATC260X_D,
    ATC260X_E,
    ATC260X_F,
    ATC260X_G,
    ATC260X_H,
}

#[repr(C)]
pub struct atc260x {
    pub dev: *mut device,

    pub regmap: *mut regmap,
    pub regmap_irq_chip: *const regmap_irq_chip,
    pub irq_data: *mut regmap_irq_chip_data,

    /* mutex for custom regmap locking */
    pub regmap_mutex: *mut mutex,

    pub cells: *const mfd_cell,
    pub nr_cells: c_int,
    pub irq: c_int,

    pub ic_type: atc260x_type,
    pub ic_ver: atc260x_ver,
    pub type_name: *const c_char,
    pub rev_reg: c_uint,

    /* regs for device init */
    pub init_regs: *const atc260x_init_regs,
}

extern "C" {
    pub fn atc260x_match_device(
        atc260x: *mut atc260x,
        regmap_cfg: *mut regmap_config,
    ) -> c_int;
    pub fn atc260x_device_probe(atc260x: *mut atc260x) -> c_int;
}

// Keep the C header's dependency surface visible to downstream translations.
#[allow(dead_code)]
type __atc260x_c_void = c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
