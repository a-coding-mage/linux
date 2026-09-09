/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mux/consumer.h - definitions for the multiplexer consumer interface
 *
 * Copyright (C) 2017 Axentia Technologies AB
 *
 * Author: Peter Rosin <peda@axentia.se>
 */

// The CONFIG_MULTIPLEXER conditional is represented by the Rust cfg feature
// `CONFIG_MULTIPLEXER`. Required kernel symbols and error helpers are external
// dependencies supplied by the surrounding translation.

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mux_control {
    _private: [u8; 0],
}

#[repr(C)]
pub struct mux_state {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}

#[cfg(feature = "CONFIG_MULTIPLEXER")]
extern "C" {
    pub fn mux_control_states(mux: *mut mux_control) -> ::core::ffi::c_uint;
    pub fn mux_control_select_delay(
        mux: *mut mux_control,
        state: ::core::ffi::c_uint,
        delay_us: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn mux_state_select_delay(
        mstate: *mut mux_state,
        delay_us: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn mux_control_try_select_delay(
        mux: *mut mux_control,
        state: ::core::ffi::c_uint,
        delay_us: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;
    pub fn mux_state_try_select_delay(
        mstate: *mut mux_state,
        delay_us: ::core::ffi::c_uint,
    ) -> ::core::ffi::c_int;

    pub fn mux_control_deselect(mux: *mut mux_control) -> ::core::ffi::c_int;
    pub fn mux_state_deselect(mstate: *mut mux_state) -> ::core::ffi::c_int;

    pub fn mux_control_get(
        dev: *mut device,
        mux_name: *const ::core::ffi::c_char,
    ) -> *mut mux_control;
    pub fn mux_control_get_optional(
        dev: *mut device,
        mux_name: *const ::core::ffi::c_char,
    ) -> *mut mux_control;
    pub fn mux_control_put(mux: *mut mux_control);

    pub fn devm_mux_control_get(
        dev: *mut device,
        mux_name: *const ::core::ffi::c_char,
    ) -> *mut mux_control;
    pub fn devm_mux_state_get_from_np(
        dev: *mut device,
        mux_name: *const ::core::ffi::c_char,
        np: *mut device_node,
    ) -> *mut mux_state;
    pub fn devm_mux_state_get_optional(
        dev: *mut device,
        mux_name: *const ::core::ffi::c_char,
    ) -> *mut mux_state;
    pub fn devm_mux_state_get_selected(
        dev: *mut device,
        mux_name: *const ::core::ffi::c_char,
    ) -> *mut mux_state;
    pub fn devm_mux_state_get_optional_selected(
        dev: *mut device,
        mux_name: *const ::core::ffi::c_char,
    ) -> *mut mux_state;
}

#[cfg(feature = "CONFIG_MULTIPLEXER")]
#[inline]
pub unsafe fn mux_control_select(mux: *mut mux_control, state: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    mux_control_select_delay(mux, state, 0)
}

#[cfg(feature = "CONFIG_MULTIPLEXER")]
#[inline]
pub unsafe fn mux_state_select(mstate: *mut mux_state) -> ::core::ffi::c_int {
    mux_state_select_delay(mstate, 0)
}

#[cfg(feature = "CONFIG_MULTIPLEXER")]
#[inline]
pub unsafe fn mux_control_try_select(mux: *mut mux_control, state: ::core::ffi::c_uint) -> ::core::ffi::c_int {
    mux_control_try_select_delay(mux, state, 0)
}

#[cfg(feature = "CONFIG_MULTIPLEXER")]
#[inline]
pub unsafe fn mux_state_try_select(mstate: *mut mux_state) -> ::core::ffi::c_int {
    mux_state_try_select_delay(mstate, 0)
}

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_control_states(_mux: *mut mux_control) -> ::core::ffi::c_uint { 0 }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_control_select_delay(_mux: *mut mux_control, _state: ::core::ffi::c_uint, _delay_us: ::core::ffi::c_uint) -> ::core::ffi::c_int { -95 }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_state_select_delay(_mstate: *mut mux_state, _delay_us: ::core::ffi::c_uint) -> ::core::ffi::c_int { -95 }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_control_try_select_delay(_mux: *mut mux_control, _state: ::core::ffi::c_uint, _delay_us: ::core::ffi::c_uint) -> ::core::ffi::c_int { -95 }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_state_try_select_delay(_mstate: *mut mux_state, _delay_us: ::core::ffi::c_uint) -> ::core::ffi::c_int { -95 }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_control_select(_mux: *mut mux_control, _state: ::core::ffi::c_uint) -> ::core::ffi::c_int { -95 }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_state_select(_mstate: *mut mux_state) -> ::core::ffi::c_int { -95 }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_control_try_select(_mux: *mut mux_control, _state: ::core::ffi::c_uint) -> ::core::ffi::c_int { -95 }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_state_try_select(_mstate: *mut mux_state) -> ::core::ffi::c_int { -95 }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_control_deselect(_mux: *mut mux_control) -> ::core::ffi::c_int { -95 }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_state_deselect(_mstate: *mut mux_state) -> ::core::ffi::c_int { -95 }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_control_get(_dev: *mut device, _mux_name: *const ::core::ffi::c_char) -> *mut mux_control { ::core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_control_get_optional(_dev: *mut device, _mux_name: *const ::core::ffi::c_char) -> *mut mux_control { ::core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn mux_control_put(_mux: *mut mux_control) {}

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn devm_mux_control_get(_dev: *mut device, _mux_name: *const ::core::ffi::c_char) -> *mut mux_control { ::core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn devm_mux_state_get(_dev: *mut device, _mux_name: *const ::core::ffi::c_char) -> *mut mux_state { ::core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn devm_mux_state_get_optional(_dev: *mut device, _mux_name: *const ::core::ffi::c_char) -> *mut mux_state { ::core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn devm_mux_state_get_selected(_dev: *mut device, _mux_name: *const ::core::ffi::c_char) -> *mut mux_state { ::core::ptr::null_mut() }

#[cfg(not(feature = "CONFIG_MULTIPLEXER"))]
#[inline]
pub unsafe fn devm_mux_state_get_optional_selected(_dev: *mut device, _mux_name: *const ::core::ffi::c_char) -> *mut mux_state { ::core::ptr::null_mut() }

#[macro_export]
macro_rules! devm_mux_state_get {
    ($dev:expr, $mux_name:expr) => {
        devm_mux_state_get_from_np($dev, $mux_name, ::core::ptr::null_mut())
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
