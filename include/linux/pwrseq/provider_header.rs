/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2024 Linaro Ltd.
 */

// Forward declarations from the surrounding kernel translation unit.
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pwrseq_device {
    _private: [u8; 0],
}

pub type pwrseq_power_state_func =
    Option<unsafe extern "C" fn(*mut pwrseq_device) -> core::ffi::c_int>;
pub type pwrseq_match_func = Option<
    unsafe extern "C" fn(*mut pwrseq_device, *mut device) -> core::ffi::c_int,
>;

pub const PWRSEQ_NO_MATCH: core::ffi::c_int = 0;
pub const PWRSEQ_MATCH_OK: core::ffi::c_int = 1;

/**
 * struct pwrseq_unit_data - Configuration of a single power sequencing
 *                           unit.
 * @name: Name of the unit.
 * @deps: Units that must be enabled before this one and disabled after it
 *        in the order they come in this array. Must be NULL-terminated.
 * @enable: Callback running the part of the power-on sequence provided by
 *          this unit.
 * @disable: Callback running the part of the power-off sequence provided
 *           by this unit.
 */
#[repr(C)]
pub struct pwrseq_unit_data {
    pub name: *const core::ffi::c_char,
    pub deps: *const *const pwrseq_unit_data,
    pub enable: pwrseq_power_state_func,
    pub disable: pwrseq_power_state_func,
}

/**
 * struct pwrseq_target_data - Configuration of a power sequencing target.
 * @name: Name of the target.
 * @unit: Final unit that this target must reach in order to be considered
 *        enabled.
 * @post_enable: Callback run after the target unit has been enabled, *after*
 *               the state lock has been released. It's useful for implementing
 *               boot-up delays without blocking other users from powering up
 *               using the same power sequencer.
 */
#[repr(C)]
pub struct pwrseq_target_data {
    pub name: *const core::ffi::c_char,
    pub unit: *const pwrseq_unit_data,
    pub post_enable: pwrseq_power_state_func,
}

/**
 * struct pwrseq_config - Configuration used for registering a new provider.
 * @parent: Parent device for the sequencer. Must be set.
 * @owner: Module providing this device.
 * @drvdata: Private driver data.
 * @match: Provider callback used to match the consumer device to the sequencer.
 * @targets: Array of targets for this power sequencer. Must be NULL-terminated.
 */
#[repr(C)]
pub struct pwrseq_config {
    pub parent: *mut device,
    pub owner: *mut module,
    pub drvdata: *mut core::ffi::c_void,
    pub r#match: pwrseq_match_func,
    pub targets: *const *const pwrseq_target_data,
}

unsafe extern "C" {
    pub fn pwrseq_device_register(config: *const pwrseq_config) -> *mut pwrseq_device;
    pub fn pwrseq_device_unregister(pwrseq: *mut pwrseq_device);
    pub fn devm_pwrseq_device_register(
        dev: *mut device,
        config: *const pwrseq_config,
    ) -> *mut pwrseq_device;
    pub fn pwrseq_device_get_drvdata(pwrseq: *mut pwrseq_device) -> *mut core::ffi::c_void;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
