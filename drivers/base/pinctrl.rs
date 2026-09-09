// SPDX-License-Identifier: GPL-2.0
/*
 * Driver core interface to the pinctrl subsystem.
 *
 * Copyright (C) 2012 ST-Ericsson SA
 * Written on behalf of Linaro for ST-Ericsson
 * Based on bits of regulator core, gpio core and clk core
 *
 * Author: Linus Walleij <linus.walleij@linaro.org>
 */

// C dependencies supplied by the surrounding kernel translation.

#[allow(non_camel_case_types)]
type c_int = i32;

#[repr(C)]
pub struct device {
    pub pins: *mut dev_pin_info,
}

#[repr(C)]
pub struct dev_pin_info {
    pub p: *mut pinctrl,
    pub default_state: *mut pinctrl_state,
    pub init_state: *mut pinctrl_state,
    #[cfg(feature = "CONFIG_PM")]
    pub sleep_state: *mut pinctrl_state,
    #[cfg(feature = "CONFIG_PM")]
    pub idle_state: *mut pinctrl_state,
}

#[repr(C)]
pub struct pinctrl {
    _private: [u8; 0],
}

#[repr(C)]
pub struct pinctrl_state {
    _private: [u8; 0],
}

extern "C" {
    fn dev_of_node_reused(dev: *mut device) -> bool;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn devm_pinctrl_get(dev: *mut device) -> *mut pinctrl;
    fn pinctrl_lookup_state(p: *mut pinctrl, name: *const u8) -> *mut pinctrl_state;
    fn pinctrl_select_state(p: *mut pinctrl, state: *mut pinctrl_state) -> c_int;
    fn devm_pinctrl_put(p: *mut pinctrl);
    fn devm_kfree(dev: *mut device, p: *mut dev_pin_info);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn __is_err(ptr: *const core::ffi::c_void) -> bool;
    fn __ptr_err(ptr: *const core::ffi::c_void) -> c_int;
}

const GFP_KERNEL: u32 = 0;
const ENOMEM: c_int = 12;
const EPROBE_DEFER: c_int = 517;
const EINVAL: c_int = 22;

const PINCTRL_STATE_DEFAULT: &[u8] = b"default\0";
const PINCTRL_STATE_INIT: &[u8] = b"init\0";
#[cfg(feature = "CONFIG_PM")]
const PINCTRL_STATE_SLEEP: &[u8] = b"sleep\0";
#[cfg(feature = "CONFIG_PM")]
const PINCTRL_STATE_IDLE: &[u8] = b"idle\0";

#[inline]
unsafe fn is_err<T>(ptr: *mut T) -> bool {
    __is_err(ptr.cast())
}

#[inline]
unsafe fn ptr_err<T>(ptr: *mut T) -> c_int {
    __ptr_err(ptr.cast())
}

/**
 * pinctrl_bind_pins() - called by the device core before probe
 * @dev: the device that is just about to probe
 */
pub unsafe fn pinctrl_bind_pins(dev: *mut device) -> c_int {
    let mut ret: c_int;

    if dev_of_node_reused(dev) {
        return 0;
    }

    (*dev).pins = devm_kzalloc(dev, core::mem::size_of::<dev_pin_info>(), GFP_KERNEL)
        as *mut dev_pin_info;
    if (*dev).pins.is_null() {
        return -ENOMEM;
    }

    (*(*dev).pins).p = devm_pinctrl_get(dev);
    if is_err((*(*dev).pins).p) {
        dev_dbg(dev, b"no pinctrl handle\n\0".as_ptr());
        ret = ptr_err((*(*dev).pins).p);
        return cleanup_alloc(dev, ret);
    }

    (*(*dev).pins).default_state = pinctrl_lookup_state(
        (*(*dev).pins).p,
        PINCTRL_STATE_DEFAULT.as_ptr(),
    );
    if is_err((*(*dev).pins).default_state) {
        dev_dbg(dev, b"no default pinctrl state\n\0".as_ptr());
        ret = 0;
        return cleanup_get(dev, ret);
    }

    (*(*dev).pins).init_state = pinctrl_lookup_state(
        (*(*dev).pins).p,
        PINCTRL_STATE_INIT.as_ptr(),
    );
    if is_err((*(*dev).pins).init_state) {
        // Not supplying this state is perfectly legal
        dev_dbg(dev, b"no init pinctrl state\n\0".as_ptr());
        ret = pinctrl_select_state((*(*dev).pins).p, (*(*dev).pins).default_state);
    } else {
        ret = pinctrl_select_state((*(*dev).pins).p, (*(*dev).pins).init_state);
    }

    if ret != 0 {
        dev_dbg(dev, b"failed to activate initial pinctrl state\n\0".as_ptr());
        return cleanup_get(dev, ret);
    }

    #[cfg(feature = "CONFIG_PM")]
    {
        /*
         * If power management is enabled, we also look for the optional
         * sleep and idle pin states, with semantics as defined in
         * <linux/pinctrl/pinctrl-state.h>
         */
        (*(*dev).pins).sleep_state = pinctrl_lookup_state(
            (*(*dev).pins).p,
            PINCTRL_STATE_SLEEP.as_ptr(),
        );
        if is_err((*(*dev).pins).sleep_state) {
            // Not supplying this state is perfectly legal
            dev_dbg(dev, b"no sleep pinctrl state\n\0".as_ptr());
        }

        (*(*dev).pins).idle_state = pinctrl_lookup_state(
            (*(*dev).pins).p,
            PINCTRL_STATE_IDLE.as_ptr(),
        );
        if is_err((*(*dev).pins).idle_state) {
            // Not supplying this state is perfectly legal
            dev_dbg(dev, b"no idle pinctrl state\n\0".as_ptr());
        }
    }

    return 0;

    unsafe fn cleanup_get(dev: *mut device, ret: c_int) -> c_int {
        devm_pinctrl_put((*(*dev).pins).p);
        cleanup_alloc(dev, ret)
    }

    unsafe fn cleanup_alloc(dev: *mut device, ret: c_int) -> c_int {
        devm_kfree(dev, (*dev).pins);
        (*dev).pins = core::ptr::null_mut();
        if ret == -EPROBE_DEFER || ret == -EINVAL {
            return ret;
        }
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
