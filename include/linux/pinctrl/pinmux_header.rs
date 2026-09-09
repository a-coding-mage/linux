/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Interface the pinmux subsystem
 *
 * Copyright (C) 2011 ST-Ericsson SA
 * Written on behalf of Linaro for ST-Ericsson
 * Based on bits of regulator core, gpio core and clk core
 *
 * Author: Linus Walleij <linus.walleij@linaro.org>
 */

use ::core::ffi::{c_char, c_int};

// Dependency corresponding to <linux/types.h>.

pub struct pinctrl_dev;
pub struct pinctrl_gpio_range;

/**
 * struct pinmux_ops - pinmux operations, to be implemented by pin controller
 * drivers that support pinmuxing
 * @request: called by the core to see if a certain pin can be made
 *\tavailable for muxing. This is called by the core to acquire the pins
 *\tbefore selecting any actual mux setting across a function. The driver
 *\tis allowed to answer "no" by returning a negative error code
 * @free: the reverse function of the request() callback, frees a pin after
 *\tbeing requested
 * @get_functions_count: returns number of selectable named functions available
 *\tin this pinmux driver
 * @get_function_name: return the function name of the muxing selector,
 *\tcalled by the core to figure out which mux setting it shall map a
 *\tcertain device to
 * @get_function_groups: return an array of groups names (in turn
 *\treferencing pins) connected to a certain function selector. The group
 *\tname can be used with the generic @pinctrl_ops to retrieve the
 *\tactual pins affected. The applicable groups will be returned in
 *\t@groups and the number of groups in @num_groups
 * @function_is_gpio: determine if the indicated function selector passed
 *\tcorresponds to the GPIO function which is used by the accelerated GPIO
 *\tfunctions @gpio_request_enable, @gpio_disable_free and
 *\t@gpio_set_direction. When the pin control core can properly determine
 *\tif a function is a GPIO function, it is easier to use the @strict mode
 *\ton the pin controller. Since a single function is passed, this is
 *\tonly useful on pin controllers that use a specific function for GPIO,
 *\tand that usually presupposes that a one-group-per-pin approach is
 *\tused, so that a single function can be set on a single pin to turn
 *\tit to GPIO mode.
 * @set_mux: enable a certain muxing function with a certain pin group. The
 *\tdriver does not need to figure out whether enabling this function
 *\tconflicts some other use of the pins in that group, such collisions
 *\tare handled by the pinmux subsystem. The @func_selector selects a
 *\tcertain function whereas @group_selector selects a certain set of pins
 *\tto be used. On simple controllers the latter argument may be ignored
 * @release_mux: Release software resources acquired by @set_mux. This callback
 *\tmust not change hardware state to avoid glitches when switching mux.
 * @gpio_request_enable: requests and enables GPIO on a certain pin.
 *\tImplement this only if you can mux every pin individually as GPIO. The
 *\taffected GPIO range is passed along with an offset(pin number) into that
 *\tspecific GPIO range - function selectors and pin groups are orthogonal
 *\tto this, the core will however make sure the pins do not collide.
 * @gpio_disable_free: free up GPIO muxing on a certain pin, the reverse of
 *\t@gpio_request_enable
 * @gpio_set_direction: Since controllers may need different configurations
 *\tdepending on whether the GPIO is configured as input or output,
 *\ta direction selector function may be implemented as a backing
 *\tto the GPIO controllers that need pin muxing.
 * @strict: do not allow simultaneous use of the same pin for GPIO and another
 *\tfunction. Check both gpio_owner and mux_owner strictly before approving
 *\tthe pin request.
 */
#[repr(C)]
pub struct pinmux_ops {
    pub request: Option<unsafe extern "C" fn(pctldev: *mut pinctrl_dev, offset: u32) -> c_int>,
    pub free: Option<unsafe extern "C" fn(pctldev: *mut pinctrl_dev, offset: u32) -> c_int>,
    pub get_functions_count: Option<unsafe extern "C" fn(pctldev: *mut pinctrl_dev) -> c_int>,
    pub get_function_name:
        Option<unsafe extern "C" fn(pctldev: *mut pinctrl_dev, selector: u32) -> *const c_char>,
    pub get_function_groups: Option<unsafe extern "C" fn(
        pctldev: *mut pinctrl_dev,
        selector: u32,
        groups: *mut *const *const c_char,
        num_groups: *mut u32,
    ) -> c_int>,
    pub function_is_gpio:
        Option<unsafe extern "C" fn(pctldev: *mut pinctrl_dev, selector: u32) -> bool>,
    pub set_mux: Option<unsafe extern "C" fn(
        pctldev: *mut pinctrl_dev,
        func_selector: u32,
        group_selector: u32,
    ) -> c_int>,
    pub release_mux:
        Option<unsafe extern "C" fn(pctldev: *mut pinctrl_dev, func_selector: u32, group_selector: u32)>,
    pub gpio_request_enable: Option<unsafe extern "C" fn(
        pctldev: *mut pinctrl_dev,
        range: *mut pinctrl_gpio_range,
        offset: u32,
    ) -> c_int>,
    pub gpio_disable_free: Option<unsafe extern "C" fn(
        pctldev: *mut pinctrl_dev,
        range: *mut pinctrl_gpio_range,
        offset: u32,
    )>,
    pub gpio_set_direction: Option<unsafe extern "C" fn(
        pctldev: *mut pinctrl_dev,
        range: *mut pinctrl_gpio_range,
        offset: u32,
        input: bool,
    ) -> c_int>,
    pub strict: bool,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
