/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2023 Javier Carrasco <javier.carrasco@wolfvision.net>
 */

// The original header includes <linux/types.h>.

#[allow(non_camel_case_types)]
pub type u16 = core::primitive::u16;

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct input_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct input_mt_pos {
    _private: [u8; 0],
}

unsafe extern "C" {
    pub fn touch_overlay_map(list: *mut list_head, input: *mut input_dev) -> core::ffi::c_int;

    pub fn touch_overlay_get_touchscreen_abs(
        list: *mut list_head,
        x: *mut u16,
        y: *mut u16,
    );

    pub fn touch_overlay_mapped_touchscreen(list: *mut list_head) -> bool;

    pub fn touch_overlay_process_contact(
        list: *mut list_head,
        input: *mut input_dev,
        pos: *mut input_mt_pos,
        slot: core::ffi::c_int,
    ) -> bool;

    pub fn touch_overlay_sync_frame(list: *mut list_head, input: *mut input_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
