/* SPDX-License-Identifier: GPL-2.0 */

use std::os::raw::c_char;

unsafe extern "C" {
    pub fn ui_progress__finish();
}

#[repr(C)]
pub struct ui_progress {
    pub title: *const c_char,
    pub curr: u64,
    pub next: u64,
    pub step: u64,
    pub total: u64,
    pub size: bool,
}

unsafe extern "C" {
    pub fn __ui_progress__init(p: *mut ui_progress, total: u64, title: *const c_char, size: bool);
}

#[inline]
pub unsafe fn ui_progress__init(p: *mut ui_progress, total: u64, title: *const c_char) {
    unsafe {
        __ui_progress__init(p, total, title, false);
    }
}

#[inline]
pub unsafe fn ui_progress__init_size(p: *mut ui_progress, total: u64, title: *const c_char) {
    unsafe {
        __ui_progress__init(p, total, title, true);
    }
}

unsafe extern "C" {
    pub fn ui_progress__update(p: *mut ui_progress, adv: u64);
}

#[repr(C)]
pub struct ui_progress_ops {
    pub init: Option<unsafe extern "C" fn(p: *mut ui_progress)>,
    pub update: Option<unsafe extern "C" fn(p: *mut ui_progress)>,
    pub finish: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" {
    pub static mut ui_progress__ops: *mut ui_progress_ops;
}
