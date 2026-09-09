/*
 * Copyright 2012-15 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 *
 * Authors: AMD
 */

#[repr(C)]
pub struct addr_mask {
    pub addr: u32,
    pub mask: u32,
}

#[repr(C)]
pub struct hw_gpio_pin {
    pub funcs: *const hw_gpio_pin_funcs,
    pub id: gpio_id,
    pub en: u32,
    pub mode: gpio_mode,
    pub opened: bool,
    pub ctx: *mut dc_context,
}

#[repr(C)]
pub struct hw_gpio_pin_funcs {
    pub destroy: Option<unsafe extern "C" fn(ptr: *mut *mut hw_gpio_pin)>,
    pub open: Option<unsafe extern "C" fn(pin: *mut hw_gpio_pin, mode: gpio_mode) -> bool>,
    pub get_value: Option<unsafe extern "C" fn(pin: *const hw_gpio_pin, value: *mut u32) -> gpio_result>,
    pub set_value: Option<unsafe extern "C" fn(pin: *const hw_gpio_pin, value: u32) -> gpio_result>,
    pub set_config: Option<unsafe extern "C" fn(pin: *mut hw_gpio_pin, config_data: *const gpio_config_data) -> gpio_result>,
    pub change_mode: Option<unsafe extern "C" fn(pin: *mut hw_gpio_pin, mode: gpio_mode) -> gpio_result>,
    pub close: Option<unsafe extern "C" fn(pin: *mut hw_gpio_pin)>,
}

#[repr(C)]
pub struct hw_gpio_pin_reg {
    pub DC_GPIO_DATA_MASK: addr_mask,
    pub DC_GPIO_DATA_A: addr_mask,
    pub DC_GPIO_DATA_EN: addr_mask,
    pub DC_GPIO_DATA_Y: addr_mask,
}

#[repr(C)]
pub struct hw_gpio_mux_reg {
    pub GPIO_MUX_CONTROL: addr_mask,
    pub GPIO_MUX_STEREO_SEL: addr_mask,
}

#[repr(C)]
pub struct hw_gpio_store {
    pub mask: u32,
    pub a: u32,
    pub en: u32,
    pub mux: u32,
}

#[repr(C)]
pub struct hw_gpio {
    pub base: hw_gpio_pin,
    pub store: hw_gpio_store,
    pub mux_supported: bool,
    pub regs: *const gpio_registers,
}

/* Equivalent of the C container_of macros. */
#[inline]
pub unsafe fn FROM_HW_GPIO_PIN(ptr: *mut hw_gpio_pin) -> *mut hw_gpio {
    (ptr as *mut u8).sub(core::mem::offset_of!(hw_gpio, base)) as *mut hw_gpio
}

#[inline]
pub unsafe fn HW_GPIO_FROM_BASE(ptr: *mut hw_gpio_pin) -> *mut hw_gpio {
    FROM_HW_GPIO_PIN(ptr)
}

extern "C" {
    pub fn dal_hw_gpio_construct(pin: *mut hw_gpio, id: gpio_id, en: u32, ctx: *mut dc_context);
    pub fn dal_hw_gpio_open(pin: *mut hw_gpio_pin, mode: gpio_mode) -> bool;
    pub fn dal_hw_gpio_get_value(pin: *const hw_gpio_pin, value: *mut u32) -> gpio_result;
    pub fn dal_hw_gpio_config_mode(pin: *mut hw_gpio, mode: gpio_mode) -> gpio_result;
    pub fn dal_hw_gpio_destruct(pin: *mut hw_gpio);
    pub fn dal_hw_gpio_set_value(ptr: *const hw_gpio_pin, value: u32) -> gpio_result;
    pub fn dal_hw_gpio_change_mode(ptr: *mut hw_gpio_pin, mode: gpio_mode) -> gpio_result;
    pub fn dal_hw_gpio_close(ptr: *mut hw_gpio_pin);
}

/* Shared helper used by all GPIO register helpers that pass a field shift
 * (stored as uint32_t) into register functions that expect uint8_t.
 */
#[inline]
pub fn gpio_reg_shift(shift: u32) -> u8 {
    shift as u8
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
