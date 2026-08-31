// SPDX-License-Identifier: GPL-2.0
// C dependencies: <linux/kernel.h>, "progress.h"

use core::ffi::c_char;

pub type u64 = u64;

#[repr(C)]
pub struct ui_progress {
    pub curr: u64,
    pub next: u64,
    pub step: u64,
    pub total: u64,
    pub title: *const c_char,
    pub size: bool,
}

#[repr(C)]
pub struct ui_progress_ops {
    pub update: Option<unsafe extern "C" fn(p: *mut ui_progress)>,
    pub init: Option<unsafe extern "C" fn(p: *mut ui_progress)>,
    pub finish: Option<unsafe extern "C" fn()>,
}

unsafe extern "C" fn null_progress__update(_p: *mut ui_progress) {}

static mut null_progress__ops: ui_progress_ops = ui_progress_ops {
    update: Some(null_progress__update),
    init: None,
    finish: None,
};

#[no_mangle]
pub static mut ui_progress__ops: *mut ui_progress_ops = unsafe { &raw mut null_progress__ops };

#[inline]
fn DIV_ROUND_UP(n: u64, d: u64) -> u64 {
    n.div_ceil(d)
}

#[no_mangle]
pub unsafe extern "C" fn ui_progress__update(p: *mut ui_progress, adv: u64) {
    let last: u64 = unsafe { (*p).curr };

    unsafe {
        (*p).curr += adv;
    }

    if unsafe { (*p).curr >= (*p).next } {
        let nr: u64 = DIV_ROUND_UP(unsafe { (*p).curr - last }, unsafe { (*p).step });

        unsafe {
            (*p).next += nr * (*p).step;
            if let Some(update) = (*ui_progress__ops).update {
                update(p);
            }
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn __ui_progress__init(
    p: *mut ui_progress,
    total: u64,
    title: *const c_char,
    size: bool,
) {
    unsafe {
        (*p).curr = 0;
        (*p).step = if total / 16 != 0 { total / 16 } else { 1 };
        (*p).next = (*p).step;
        (*p).total = total;
        (*p).title = title;
        (*p).size = size;

        if let Some(init) = (*ui_progress__ops).init {
            init(p);
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn ui_progress__finish() {
    unsafe {
        if let Some(finish) = (*ui_progress__ops).finish {
            finish();
        }
    }
}
