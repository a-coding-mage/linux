/* SPDX-License-Identifier: GPL-2.0-only */

/*
 * Input Multitouch Library
 *
 * Copyright (c) 2010 Henrik Rydberg
 */

// Dependency declarations such as input_dev, input_event, and ABS_* constants
// are supplied by the Linux input interface.

pub const TRKID_MAX: i32 = 0xffff;

pub const INPUT_MT_POINTER: u32 = 0x0001;
pub const INPUT_MT_DIRECT: u32 = 0x0002;
pub const INPUT_MT_DROP_UNUSED: u32 = 0x0004;
pub const INPUT_MT_TRACK: u32 = 0x0008;
pub const INPUT_MT_SEMI_MT: u32 = 0x0010;
pub const INPUT_MT_TOTAL_FORCE: u32 = 0x0020;

#[repr(C)]
pub struct input_mt_slot {
    pub abs: [i32; (ABS_MT_LAST - ABS_MT_FIRST + 1) as usize],
    pub frame: u32,
    pub key: u32,
}

#[repr(C)]
pub struct input_mt {
    pub trkid: i32,
    pub num_slots: i32,
    pub slot: i32,
    pub flags: u32,
    pub frame: u32,
    pub red: *mut i32,
    pub slots: [input_mt_slot; 0],
}

#[inline]
pub unsafe fn input_mt_set_value(slot: *mut input_mt_slot, code: u32, value: i32) {
    (*slot).abs[(code - ABS_MT_FIRST) as usize] = value;
}

#[inline]
pub unsafe fn input_mt_get_value(slot: *const input_mt_slot, code: u32) -> i32 {
    (*slot).abs[(code - ABS_MT_FIRST) as usize]
}

#[inline]
pub unsafe fn input_mt_is_active(slot: *const input_mt_slot) -> bool {
    input_mt_get_value(slot, ABS_MT_TRACKING_ID) >= 0
}

#[inline]
pub unsafe fn input_mt_is_used(mt: *const input_mt, slot: *const input_mt_slot) -> bool {
    (*slot).frame == (*mt).frame
}

unsafe extern "C" {
    pub fn input_mt_init_slots(dev: *mut input_dev, num_slots: u32, flags: u32) -> i32;
    pub fn input_mt_destroy_slots(dev: *mut input_dev);
}

#[inline]
pub unsafe fn input_mt_new_trkid(mt: *mut input_mt) -> i32 {
    let trkid = (*mt).trkid;
    (*mt).trkid = (*mt).trkid.wrapping_add(1);
    trkid & TRKID_MAX
}

#[inline]
pub unsafe fn input_mt_slot(dev: *mut input_dev, slot: i32) {
    input_event(dev, EV_ABS, ABS_MT_SLOT, slot);
}

#[inline]
pub fn input_is_mt_value(axis: i32) -> bool {
    axis >= ABS_MT_FIRST as i32 && axis <= ABS_MT_LAST as i32
}

#[inline]
pub fn input_is_mt_axis(axis: i32) -> bool {
    axis == ABS_MT_SLOT as i32 || input_is_mt_value(axis)
}

unsafe extern "C" {
    pub fn input_mt_report_slot_state(
        dev: *mut input_dev,
        tool_type: u32,
        active: bool,
    ) -> bool;
}

#[inline]
pub unsafe fn input_mt_report_slot_inactive(dev: *mut input_dev) {
    input_mt_report_slot_state(dev, 0, false);
}

unsafe extern "C" {
    pub fn input_mt_report_finger_count(dev: *mut input_dev, count: i32);
    pub fn input_mt_report_pointer_emulation(dev: *mut input_dev, use_count: bool);
    pub fn input_mt_drop_unused(dev: *mut input_dev);
    pub fn input_mt_sync_frame(dev: *mut input_dev);
}

#[repr(C)]
pub struct input_mt_pos {
    pub x: i16,
    pub y: i16,
}

unsafe extern "C" {
    pub fn input_mt_assign_slots(
        dev: *mut input_dev,
        slots: *mut i32,
        pos: *const input_mt_pos,
        num_pos: i32,
        dmax: i32,
    ) -> i32;
    pub fn input_mt_get_slot_by_key(dev: *mut input_dev, key: i32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
