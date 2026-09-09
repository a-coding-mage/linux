/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Interface the generic pinconfig portions of the pinctrl subsystem
 *
 * Copyright (C) 2011 ST-Ericsson SA
 * Written on behalf of Linaro for ST-Ericsson
 * This interface is used in the core to keep track of pins.
 *
 * Author: Linus Walleij <linus.walleij@linaro.org>
 */

/* Dependencies supplied by the surrounding kernel translation. */

use core::ffi::c_char;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum pin_config_param {
    PIN_CONFIG_BIAS_BUS_HOLD,
    PIN_CONFIG_BIAS_DISABLE,
    PIN_CONFIG_BIAS_HIGH_IMPEDANCE,
    PIN_CONFIG_BIAS_PULL_DOWN,
    PIN_CONFIG_BIAS_PULL_PIN_DEFAULT,
    PIN_CONFIG_BIAS_PULL_UP,
    PIN_CONFIG_DRIVE_OPEN_DRAIN,
    PIN_CONFIG_DRIVE_OPEN_SOURCE,
    PIN_CONFIG_DRIVE_PUSH_PULL,
    PIN_CONFIG_DRIVE_STRENGTH,
    PIN_CONFIG_DRIVE_STRENGTH_UA,
    PIN_CONFIG_INPUT_DEBOUNCE,
    PIN_CONFIG_INPUT_ENABLE,
    PIN_CONFIG_INPUT_SCHMITT,
    PIN_CONFIG_INPUT_SCHMITT_ENABLE,
    PIN_CONFIG_INPUT_SCHMITT_UV,
    PIN_CONFIG_INPUT_VOLTAGE_UV,
    PIN_CONFIG_MODE_LOW_POWER,
    PIN_CONFIG_MODE_PWM,
    PIN_CONFIG_LEVEL,
    PIN_CONFIG_OUTPUT_ENABLE,
    PIN_CONFIG_OUTPUT_IMPEDANCE_OHMS,
    PIN_CONFIG_PERSIST_STATE,
    PIN_CONFIG_POWER_SOURCE,
    PIN_CONFIG_SKEW_DELAY,
    PIN_CONFIG_SKEW_DELAY_INPUT_PS,
    PIN_CONFIG_SKEW_DELAY_OUTPUT_PS,
    PIN_CONFIG_SLEEP_HARDWARE_STATE,
    PIN_CONFIG_SLEW_RATE,
    PIN_CONFIG_END = 0x7f,
    PIN_CONFIG_MAX = 0xff,
}

#[inline]
pub const fn pin_conf_packed(p: pin_config_param, a: u32) -> usize {
    ((a as usize) << 8) | ((p as usize) & 0xffusize)
}

#[inline]
pub const fn pinconf_to_config_param(config: usize) -> pin_config_param {
    match (config & 0xff) as u32 {
        0 => pin_config_param::PIN_CONFIG_BIAS_BUS_HOLD,
        1 => pin_config_param::PIN_CONFIG_BIAS_DISABLE,
        2 => pin_config_param::PIN_CONFIG_BIAS_HIGH_IMPEDANCE,
        3 => pin_config_param::PIN_CONFIG_BIAS_PULL_DOWN,
        4 => pin_config_param::PIN_CONFIG_BIAS_PULL_PIN_DEFAULT,
        5 => pin_config_param::PIN_CONFIG_BIAS_PULL_UP,
        6 => pin_config_param::PIN_CONFIG_DRIVE_OPEN_DRAIN,
        7 => pin_config_param::PIN_CONFIG_DRIVE_OPEN_SOURCE,
        8 => pin_config_param::PIN_CONFIG_DRIVE_PUSH_PULL,
        9 => pin_config_param::PIN_CONFIG_DRIVE_STRENGTH,
        10 => pin_config_param::PIN_CONFIG_DRIVE_STRENGTH_UA,
        11 => pin_config_param::PIN_CONFIG_INPUT_DEBOUNCE,
        12 => pin_config_param::PIN_CONFIG_INPUT_ENABLE,
        13 => pin_config_param::PIN_CONFIG_INPUT_SCHMITT,
        14 => pin_config_param::PIN_CONFIG_INPUT_SCHMITT_ENABLE,
        15 => pin_config_param::PIN_CONFIG_INPUT_SCHMITT_UV,
        16 => pin_config_param::PIN_CONFIG_INPUT_VOLTAGE_UV,
        17 => pin_config_param::PIN_CONFIG_MODE_LOW_POWER,
        18 => pin_config_param::PIN_CONFIG_MODE_PWM,
        19 => pin_config_param::PIN_CONFIG_LEVEL,
        20 => pin_config_param::PIN_CONFIG_OUTPUT_ENABLE,
        21 => pin_config_param::PIN_CONFIG_OUTPUT_IMPEDANCE_OHMS,
        22 => pin_config_param::PIN_CONFIG_PERSIST_STATE,
        23 => pin_config_param::PIN_CONFIG_POWER_SOURCE,
        24 => pin_config_param::PIN_CONFIG_SKEW_DELAY,
        25 => pin_config_param::PIN_CONFIG_SKEW_DELAY_INPUT_PS,
        26 => pin_config_param::PIN_CONFIG_SKEW_DELAY_OUTPUT_PS,
        27 => pin_config_param::PIN_CONFIG_SLEEP_HARDWARE_STATE,
        28 => pin_config_param::PIN_CONFIG_SLEW_RATE,
        0x7f => pin_config_param::PIN_CONFIG_END,
        0xff => pin_config_param::PIN_CONFIG_MAX,
        _ => panic!("invalid pin configuration parameter"),
    }
}

#[inline]
pub const fn pinconf_to_config_argument(config: usize) -> u32 {
    ((config >> 8) & 0xffffff) as u32
}

#[inline]
pub const fn pinconf_to_config_packed(param: pin_config_param, argument: u32) -> usize {
    pin_conf_packed(param, argument)
}

#[macro_export]
macro_rules! PCONFDUMP_WITH_VALUES {
    ($a:expr, $b:expr, $c:expr, $d:expr, $e:expr, $f:expr) => {
        pin_config_item {
            param: $a,
            display: $b,
            format: $c,
            has_arg: $d,
            values: $e,
            num_values: $f,
        }
    };
}

#[macro_export]
macro_rules! PCONFDUMP {
    ($a:expr, $b:expr, $c:expr, $d:expr) => {
        PCONFDUMP_WITH_VALUES!($a, $b, $c, $d, core::ptr::null(), 0)
    };
}

#[repr(C)]
pub struct pin_config_item {
    pub param: pin_config_param,
    pub display: *const c_char,
    pub format: *const c_char,
    pub has_arg: bool,
    pub values: *const *const c_char,
    pub num_values: usize,
}

#[repr(C)]
pub struct pinconf_generic_params {
    pub property: *const c_char,
    pub param: pin_config_param,
    pub default_value: u32,
    pub values: *const *const c_char,
    pub num_values: usize,
}

#[repr(C)]
pub struct device_node;
#[repr(C)]
pub struct pinctrl_dev;
#[repr(C)]
pub struct pinctrl_map;

#[allow(improper_ctypes)]
extern "C" {
    pub fn pinconf_generic_dt_subnode_to_map(
        pctldev: *mut pinctrl_dev,
        np: *mut device_node,
        map: *mut *mut pinctrl_map,
        reserved_maps: *mut u32,
        num_maps: *mut u32,
        type_: pinctrl_map_type,
    ) -> i32;
    pub fn pinconf_generic_dt_node_to_map(
        pctldev: *mut pinctrl_dev,
        np_config: *mut device_node,
        map: *mut *mut pinctrl_map,
        num_maps: *mut u32,
        type_: pinctrl_map_type,
    ) -> i32;
    pub fn pinconf_generic_dt_free_map(
        pctldev: *mut pinctrl_dev,
        map: *mut pinctrl_map,
        num_maps: u32,
    );
}

/* Supplied by linux/pinctrl/machine.h. */
#[allow(non_camel_case_types)]
pub type pinctrl_map_type = i32;
pub const PIN_MAP_TYPE_CONFIGS_GROUP: pinctrl_map_type = 0;
pub const PIN_MAP_TYPE_CONFIGS_PIN: pinctrl_map_type = 1;
pub const PIN_MAP_TYPE_INVALID: pinctrl_map_type = -1;

#[inline]
pub unsafe fn pinconf_generic_dt_node_to_map_group(
    pctldev: *mut pinctrl_dev,
    np_config: *mut device_node,
    map: *mut *mut pinctrl_map,
    num_maps: *mut u32,
) -> i32 {
    pinconf_generic_dt_node_to_map(pctldev, np_config, map, num_maps, PIN_MAP_TYPE_CONFIGS_GROUP)
}

#[inline]
pub unsafe fn pinconf_generic_dt_node_to_map_pin(
    pctldev: *mut pinctrl_dev,
    np_config: *mut device_node,
    map: *mut *mut pinctrl_map,
    num_maps: *mut u32,
) -> i32 {
    pinconf_generic_dt_node_to_map(pctldev, np_config, map, num_maps, PIN_MAP_TYPE_CONFIGS_PIN)
}

#[inline]
pub unsafe fn pinconf_generic_dt_node_to_map_all(
    pctldev: *mut pinctrl_dev,
    np_config: *mut device_node,
    map: *mut *mut pinctrl_map,
    num_maps: *mut u32,
) -> i32 {
    /* Passing INVALID causes the parser to infer the map type from DT properties. */
    pinconf_generic_dt_node_to_map(pctldev, np_config, map, num_maps, PIN_MAP_TYPE_INVALID)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
