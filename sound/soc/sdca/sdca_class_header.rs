/* SPDX-License-Identifier: GPL-2.0 */
/*
 * The MIPI SDCA specification is available for public downloads at
 * https://www.mipi.org/mipi-sdca-v1-0-download
 *
 * Copyright (C) 2025 Cirrus Logic, Inc. and
 *                    Cirrus Logic International Semiconductor Ltd.
 */

// C dependency intent:
// #include <linux/completion.h>
// #include <linux/mutex.h>
// #include <linux/workqueue.h>

// Forward declarations / external dependency types from the surrounding tree:
// struct device;
// struct regmap;
// struct sdw_slave;
// struct sdca_function_data;

#[repr(C)]
pub struct sdca_class_drv {
    pub dev: *mut device,
    pub dev_regmap: *mut regmap,
    pub sdw: *mut sdw_slave,

    pub irq_info: *mut sdca_interrupt_info,

    pub regmap_lock: mutex,
    /* Serialise function initialisations */
    pub init_lock: mutex,
    pub boot_work: work_struct,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
