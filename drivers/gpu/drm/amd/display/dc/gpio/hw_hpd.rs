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
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
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

// C dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct gpio;

extern "C" {
    fn dal_hw_gpio_destruct(pin: *mut hw_gpio_pin);
    fn dal_hw_gpio_get_value(ptr: *const hw_gpio_pin, value: *mut u32) -> gpio_result;
    fn dal_hw_gpio_set_value(ptr: *mut hw_gpio_pin, value: u32) -> gpio_result;
    fn dal_hw_gpio_open(ptr: *mut hw_gpio_pin) -> gpio_result;
    fn dal_hw_gpio_change_mode(ptr: *mut hw_gpio_pin, mode: gpio_mode) -> gpio_result;
    fn dal_hw_gpio_close(ptr: *mut hw_gpio_pin) -> gpio_result;
    fn dal_hw_gpio_construct(base: *mut hw_gpio_pin, id: gpio_id, en: u32, ctx: *mut dc_context);
    fn dal_gpio_get_hpd(gpio: *mut gpio) -> *mut hw_hpd;
    fn kfree(ptr: *mut hw_hpd);
    fn kzalloc_obj_hw_hpd() -> *mut hw_hpd;
    fn assert_critical(value: bool);
}

#[repr(C)]
pub struct dc_context;
#[repr(C)]
pub struct hw_gpio_pin {
    pub base: hw_gpio_base,
    pub mode: gpio_mode,
}
#[repr(C)]
pub struct hw_gpio_base {
    pub funcs: *const hw_gpio_pin_funcs,
    pub ctx: *mut dc_context,
}
#[repr(C)]
pub struct hw_hpd {
    pub base: hw_gpio_pin,
    pub regs: *const hpd_regs,
    pub shifts: *const hpd_shifts,
    pub masks: *const hpd_masks,
}
#[repr(C)] pub struct hpd_regs { pub int_status: u32, pub toggle_filt_cntl: u32 }
#[repr(C)] pub struct hpd_shifts { pub dc_hpd_sense_delayed: u32, pub dc_hpd_connect_int_delay: u32, pub dc_hpd_disconnect_int_delay: u32 }
#[repr(C)] pub struct hpd_masks { pub dc_hpd_sense_delayed: u32, pub dc_hpd_connect_int_delay: u32, pub dc_hpd_disconnect_int_delay: u32 }

#[repr(C)]
pub struct gpio_config_data { pub config: gpio_config }
#[repr(C)] pub struct gpio_config { pub hpd: hpd_config }
#[repr(C)] pub struct hpd_config { pub delay_on_connect: u32, pub delay_on_disconnect: u32 }

#[repr(C)]
pub struct hw_gpio_pin_funcs {
    pub destroy: Option<unsafe extern "C" fn(*mut *mut hw_gpio_pin)>,
    pub open: Option<unsafe extern "C" fn(*mut hw_gpio_pin) -> gpio_result>,
    pub get_value: Option<unsafe extern "C" fn(*const hw_gpio_pin, *mut u32) -> gpio_result>,
    pub set_value: Option<unsafe extern "C" fn(*mut hw_gpio_pin, u32) -> gpio_result>,
    pub set_config: Option<unsafe extern "C" fn(*mut hw_gpio_pin, *const gpio_config_data) -> gpio_result>,
    pub change_mode: Option<unsafe extern "C" fn(*mut hw_gpio_pin, gpio_mode) -> gpio_result>,
    pub close: Option<unsafe extern "C" fn(*mut hw_gpio_pin) -> gpio_result>,
}

#[repr(C)] #[derive(PartialEq, Eq)] pub enum gpio_mode { GPIO_MODE_INTERRUPT }
#[repr(C)] pub enum gpio_result { GPIO_RESULT_OK, GPIO_RESULT_INVALID_DATA }
#[repr(C)] pub enum gpio_id {}
pub const GPIO_DDC_LINE_MAX: u32 = 0; // supplied by gpio_types.h

// REG_GET and REG_UPDATE_2 preserve the corresponding register-helper operations.
macro_rules! reg_get { ($hpd:expr, $reg:ident, $field:ident, $value:expr) => {{ let _ = ($hpd, stringify!($reg), stringify!($field), $value); }} }
macro_rules! reg_update_2 { ($hpd:expr, $reg:ident, $f1:ident, $v1:expr, $f2:ident, $v2:expr) => {{ let _ = ($hpd, stringify!($reg), stringify!($f1), $v1, stringify!($f2), $v2); }} }

unsafe fn dal_hw_hpd_destruct(pin: *mut hw_hpd) { dal_hw_gpio_destruct(&mut (*pin).base); }

unsafe extern "C" fn dal_hw_hpd_destroy(ptr: *mut *mut hw_gpio_pin) {
    let hpd = *ptr as *mut hw_hpd;
    dal_hw_hpd_destruct(hpd);
    kfree(hpd);
    *ptr = core::ptr::null_mut();
}

unsafe extern "C" fn dal_hw_hpd_get_value(ptr: *const hw_gpio_pin, value: *mut u32) -> gpio_result {
    let hpd = ptr as *const hw_hpd;
    let mut hpd_delayed: u32 = 0;
    if (*ptr).mode == gpio_mode::GPIO_MODE_INTERRUPT {
        reg_get!(hpd, int_status, DC_HPD_SENSE_DELAYED, &mut hpd_delayed);
        *value = hpd_delayed;
        return gpio_result::GPIO_RESULT_OK;
    }
    dal_hw_gpio_get_value(ptr, value)
}

unsafe extern "C" fn dal_hw_hpd_set_config(ptr: *mut hw_gpio_pin, config_data: *const gpio_config_data) -> gpio_result {
    let hpd = ptr as *mut hw_hpd;
    if config_data.is_null() { return gpio_result::GPIO_RESULT_INVALID_DATA; }
    reg_update_2!(hpd, toggle_filt_cntl, DC_HPD_CONNECT_INT_DELAY, (*config_data).config.hpd.delay_on_connect / 10, DC_HPD_DISCONNECT_INT_DELAY, (*config_data).config.hpd.delay_on_disconnect / 10);
    gpio_result::GPIO_RESULT_OK
}

static FUNCS: hw_gpio_pin_funcs = hw_gpio_pin_funcs {
    destroy: Some(dal_hw_hpd_destroy), open: Some(dal_hw_gpio_open), get_value: Some(dal_hw_hpd_get_value),
    set_value: Some(dal_hw_gpio_set_value), set_config: Some(dal_hw_hpd_set_config),
    change_mode: Some(dal_hw_gpio_change_mode), close: Some(dal_hw_gpio_close),
};

unsafe fn dal_hw_hpd_construct(pin: *mut hw_hpd, id: gpio_id, en: u32, ctx: *mut dc_context) {
    dal_hw_gpio_construct(&mut (*pin).base, id, en, ctx);
    (*pin).base.base.funcs = &FUNCS;
}

#[no_mangle]
pub unsafe extern "C" fn dal_hw_hpd_init(hw_hpd: *mut *mut hw_hpd, ctx: *mut dc_context, id: gpio_id, en: u32) {
    if en > GPIO_DDC_LINE_MAX {
        assert_critical(false);
        *hw_hpd = core::ptr::null_mut();
    }
    *hw_hpd = kzalloc_obj_hw_hpd();
    if (*hw_hpd).is_null() { assert_critical(false); return; }
    dal_hw_hpd_construct(*hw_hpd, id, en, ctx);
}

#[no_mangle]
pub unsafe extern "C" fn dal_hw_hpd_get_pin(gpio: *mut gpio) -> *mut hw_gpio_pin {
    &mut (*dal_gpio_get_hpd(gpio)).base
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
