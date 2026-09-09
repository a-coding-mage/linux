/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Definitions for the Linux I2C OF component prober
 *
 * Copyright (C) 2024 Google LLC
 */

use core::ffi::c_void;

/* C forward declarations. */
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

/**
 * struct i2c_of_probe_ops - I2C OF component prober callbacks
 *
 * A set of callbacks to be used by i2c_of_probe_component().
 *
 * All callbacks are optional. Callbacks are called only once per run, and are
 * used in the order they are defined in this structure.
 *
 * All callbacks that have return values shall return %0 on success,
 * or a negative error number on failure.
 *
 * The @dev parameter passed to the callbacks is the same as @dev passed to
 * i2c_of_probe_component(). It should only be used for dev_printk() calls
 * and nothing else, especially not managed device resource (devres) APIs.
 */
#[repr(C)]
pub struct i2c_of_probe_ops {
    /** @enable: Retrieve and enable resources so that the components respond to probes. */
    pub enable: Option<unsafe extern "C" fn(*mut device, *mut device_node, *mut c_void) -> i32>,
    /** @cleanup_early: Release exclusive resources prior to calling probe() on a detected component. */
    pub cleanup_early: Option<unsafe extern "C" fn(*mut device, *mut c_void)>,
    /** @cleanup: Opposite of @enable to balance refcounts and free resources after probing. */
    pub cleanup: Option<unsafe extern "C" fn(*mut device, *mut c_void)>,
}

/**
 * struct i2c_of_probe_cfg - I2C OF component prober configuration
 * @ops: Callbacks for the prober to use.
 * @type: A string to match the device node name prefix to probe for.
 */
#[repr(C)]
pub struct i2c_of_probe_cfg {
    pub ops: *const i2c_of_probe_ops,
    pub r#type: *const core::ffi::c_char,
}

/* Preserved build-time condition: declarations are available when CONFIG_OF_DYNAMIC is enabled. */
extern "C" {
    pub fn i2c_of_probe_component(
        dev: *mut device,
        cfg: *const i2c_of_probe_cfg,
        ctx: *mut c_void,
    ) -> i32;
}

/** Options for simple I2C component prober callbacks. */
#[repr(C)]
pub struct i2c_of_probe_simple_opts {
    pub res_node_compatible: *const core::ffi::c_char,
    pub supply_name: *const core::ffi::c_char,
    pub gpio_name: *const core::ffi::c_char,
    pub post_power_on_delay_ms: u32,
    pub post_gpio_config_delay_ms: u32,
    pub gpio_assert_to_enable: bool,
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regulator {
    _private: [u8; 0],
}

#[repr(C)]
pub struct i2c_of_probe_simple_ctx {
    /* public: provided by user before helpers are used. */
    pub opts: *const i2c_of_probe_simple_opts,
    /* private: internal fields for helpers. */
    pub supply: *mut regulator,
    pub gpiod: *mut gpio_desc,
}

extern "C" {
    pub fn i2c_of_probe_simple_enable(
        dev: *mut device,
        bus_node: *mut device_node,
        data: *mut c_void,
    ) -> i32;
    pub fn i2c_of_probe_simple_cleanup_early(dev: *mut device, data: *mut c_void);
    pub fn i2c_of_probe_simple_cleanup(dev: *mut device, data: *mut c_void);
    pub static mut i2c_of_probe_simple_ops: i2c_of_probe_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
