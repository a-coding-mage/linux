/* SPDX-License-Identifier: GPL-2.0-or-later */
//
// Actions Semi Owl SoCs Reset Management Unit driver
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// Dependency supplied by the Linux reset-controller interfaces.

#[repr(C)]
pub struct owl_reset_map {
    pub reg: u32,
    pub bit: u32,
}

#[repr(C)]
pub struct owl_reset {
    pub rcdev: reset_controller_dev,
    pub reset_map: *const owl_reset_map,
    pub regmap: *mut regmap,
}

#[inline]
pub unsafe fn to_owl_reset(rcdev: *mut reset_controller_dev) -> *mut owl_reset {
    (rcdev as *mut u8).sub(core::mem::offset_of!(owl_reset, rcdev)) as *mut owl_reset
}

extern "C" {
    pub static owl_reset_ops: reset_control_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
