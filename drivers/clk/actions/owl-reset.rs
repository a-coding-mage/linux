// SPDX-License-Identifier: GPL-2.0-or-later
//
// Actions Semi Owl SoCs Reset Management Unit driver
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// External kernel and owl-reset declarations supplied by other files.

extern "C" {
    fn regmap_update_bits(
        map: *mut regmap,
        reg: u32,
        mask: u32,
        val: u32,
    ) -> i32;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut u32) -> i32;
    fn udelay(usecs: u32);
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct reset_controller_dev {
    _private: [u8; 0],
}

#[repr(C)]
pub struct owl_reset_map {
    pub reg: u32,
    pub bit: u32,
}

#[repr(C)]
pub struct owl_reset {
    pub regmap: *mut regmap,
    pub reset_map: *const owl_reset_map,
}

extern "C" {
    fn to_owl_reset(rcdev: *mut reset_controller_dev) -> *mut owl_reset;
}

#[no_mangle]
pub unsafe extern "C" fn owl_reset_assert(
    rcdev: *mut reset_controller_dev,
    id: usize,
) -> i32 {
    let reset = to_owl_reset(rcdev);
    let map = &*reset.reset_map.add(id);

    regmap_update_bits((*reset).regmap, map.reg, map.bit, 0)
}

#[no_mangle]
pub unsafe extern "C" fn owl_reset_deassert(
    rcdev: *mut reset_controller_dev,
    id: usize,
) -> i32 {
    let reset = to_owl_reset(rcdev);
    let map = &*reset.reset_map.add(id);

    regmap_update_bits((*reset).regmap, map.reg, map.bit, map.bit)
}

#[no_mangle]
pub unsafe extern "C" fn owl_reset_reset(
    rcdev: *mut reset_controller_dev,
    id: usize,
) -> i32 {
    owl_reset_assert(rcdev, id);
    udelay(1);
    owl_reset_deassert(rcdev, id);

    0
}

#[no_mangle]
pub unsafe extern "C" fn owl_reset_status(
    rcdev: *mut reset_controller_dev,
    id: usize,
) -> i32 {
    let reset = to_owl_reset(rcdev);
    let map = &*reset.reset_map.add(id);
    let mut reg: u32 = 0;

    let ret = regmap_read((*reset).regmap, map.reg, &mut reg);
    if ret != 0 {
        return ret;
    }

    /*
     * The reset control API expects 0 if reset is not asserted,
     * which is the opposite of what our hardware uses.
     */
    if (map.bit & reg) == 0 { 1 } else { 0 }
}

#[repr(C)]
pub struct reset_control_ops {
    pub assert: Option<unsafe extern "C" fn(*mut reset_controller_dev, usize) -> i32>,
    pub deassert: Option<unsafe extern "C" fn(*mut reset_controller_dev, usize) -> i32>,
    pub reset: Option<unsafe extern "C" fn(*mut reset_controller_dev, usize) -> i32>,
    pub status: Option<unsafe extern "C" fn(*mut reset_controller_dev, usize) -> i32>,
}

#[no_mangle]
pub static owl_reset_ops: reset_control_ops = reset_control_ops {
    assert: Some(owl_reset_assert),
    deassert: Some(owl_reset_deassert),
    reset: Some(owl_reset_reset),
    status: Some(owl_reset_status),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
