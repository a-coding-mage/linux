/*
 * Copyright (C) 2006 - 2007 Ivo van Doorn
 * Copyright (C) 2007 Dmitry Torokhov
 * Copyright 2009 Johannes Berg <johannes@sipsolutions.net>
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

// The following values are supplied by uapi/linux/rfkill.h.
pub const RFKILL_USER_STATE_SOFT_BLOCKED: i32 = RFKILL_STATE_SOFT_BLOCKED;
pub const RFKILL_USER_STATE_UNBLOCKED: i32 = RFKILL_STATE_UNBLOCKED;
pub const RFKILL_USER_STATE_HARD_BLOCKED: i32 = RFKILL_STATE_HARD_BLOCKED;

pub struct device;
// this is opaque
pub struct rfkill;

#[repr(C)]
pub struct rfkill_ops {
    pub poll: Option<unsafe extern "C" fn(rfkill: *mut rfkill, data: *mut core::ffi::c_void)>,
    pub query: Option<unsafe extern "C" fn(rfkill: *mut rfkill, data: *mut core::ffi::c_void)>,
    pub set_block: Option<unsafe extern "C" fn(data: *mut core::ffi::c_void, blocked: bool) -> i32>,
}

// CONFIG_RFKILL or CONFIG_RFKILL_MODULE
extern "C" {
    pub fn rfkill_alloc(
        name: *const core::ffi::c_char,
        parent: *mut device,
        type_: rfkill_type,
        ops: *const rfkill_ops,
        ops_data: *mut core::ffi::c_void,
    ) -> *mut rfkill;
    pub fn rfkill_register(rfkill: *mut rfkill) -> i32;
    pub fn rfkill_pause_polling(rfkill: *mut rfkill);
    pub fn rfkill_resume_polling(rfkill: *mut rfkill);
    pub fn rfkill_unregister(rfkill: *mut rfkill);
    pub fn rfkill_destroy(rfkill: *mut rfkill);
    pub fn rfkill_set_hw_state_reason(
        rfkill: *mut rfkill,
        blocked: bool,
        reason: rfkill_hard_block_reasons,
    ) -> bool;
    pub fn rfkill_set_sw_state(rfkill: *mut rfkill, blocked: bool) -> bool;
    pub fn rfkill_init_sw_state(rfkill: *mut rfkill, blocked: bool);
    pub fn rfkill_set_states(rfkill: *mut rfkill, sw: bool, hw: bool);
    pub fn rfkill_blocked(rfkill: *mut rfkill) -> bool;
    pub fn rfkill_soft_blocked(rfkill: *mut rfkill) -> bool;
    pub fn rfkill_find_type(name: *const core::ffi::c_char) -> rfkill_type;
}

#[inline]
pub unsafe fn rfkill_set_hw_state(rfkill: *mut rfkill, blocked: bool) -> bool {
    rfkill_set_hw_state_reason(rfkill, blocked, RFKILL_HARD_BLOCK_SIGNAL)
}

// When RFKILL and RFKILL_MODULE are disabled, the inline fallbacks are:
#[inline]
pub unsafe fn rfkill_set_hw_state_reason_disabled(
    _rfkill: *mut rfkill,
    blocked: bool,
    _reason: rfkill_hard_block_reasons,
) -> bool { blocked }

#[inline]
pub unsafe fn rfkill_set_hw_state_disabled(_rfkill: *mut rfkill, blocked: bool) -> bool { blocked }

#[inline]
pub unsafe fn rfkill_set_sw_state_disabled(_rfkill: *mut rfkill, blocked: bool) -> bool { blocked }

#[inline]
pub unsafe fn rfkill_blocked_disabled(_rfkill: *mut rfkill) -> bool { false }

#[inline]
pub unsafe fn rfkill_soft_blocked_disabled(_rfkill: *mut rfkill) -> bool { false }

#[inline]
pub unsafe fn rfkill_find_type_disabled(_name: *const core::ffi::c_char) -> rfkill_type {
    RFKILL_TYPE_ALL
}

// CONFIG_RFKILL_LEDS
extern "C" {
    pub fn rfkill_get_led_trigger_name(rfkill: *mut rfkill) -> *const core::ffi::c_char;
    pub fn rfkill_set_led_trigger_name(rfkill: *mut rfkill, name: *const core::ffi::c_char);
}

// CONFIG_RFKILL_LEDS disabled fallbacks:
#[inline]
pub unsafe fn rfkill_get_led_trigger_name_disabled(
    _rfkill: *mut rfkill,
) -> *const core::ffi::c_char { core::ptr::null() }

#[inline]
pub unsafe fn rfkill_set_led_trigger_name_disabled(
    _rfkill: *mut rfkill,
    _name: *const core::ffi::c_char,
) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
