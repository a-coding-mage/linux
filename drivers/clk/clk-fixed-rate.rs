// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2010-2011 Canonical Ltd <jeremy.kerr@canonical.com>
 * Copyright (C) 2011-2012 Mike Turquette, Linaro Ltd <mturquette@linaro.org>
 *
 * Fixed rate clock implementation
 */

/* Linux kernel dependencies are supplied by other translated units. */

/*
 * DOC: basic fixed-rate clock that cannot gate
 *
 * Traits of this clock:
 * prepare - clk_(un)prepare only ensures parents are prepared
 * enable - clk_enable only ensures parents are enabled
 * rate - rate is always a fixed value.  No clk_set_rate support
 * parent - fixed parent.  No clk_set_parent support
 */

#[allow(non_camel_case_types)]
type c_ulong = usize;

#[allow(non_camel_case_types)]
type c_int = i32;

#[repr(C)]
pub struct clk_fixed_rate {
    pub hw: clk_hw,
    pub fixed_rate: c_ulong,
    pub fixed_accuracy: c_ulong,
    pub flags: c_ulong,
}

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
    pub clk: *mut clk,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const u8,
    pub ops: *const clk_ops,
    pub flags: c_ulong,
    pub parent_names: *const *const u8,
    pub parent_hws: *const *const clk_hw,
    pub parent_data: *const clk_parent_data,
    pub num_parents: u8,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
    pub recalc_accuracy: Option<unsafe extern "C" fn(*mut clk_hw, c_ulong) -> c_ulong>,
}

#[repr(C)]
pub struct clk_parent_data {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_node {
    pub name: *const u8,
}

#[repr(C)]
pub struct platform_device {
    pub dev: device,
}

#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}

pub const CLK_FIXED_RATE_PARENT_ACCURACY: c_ulong = 1;

pub const clk_fixed_rate_ops: clk_ops = clk_ops {
    recalc_rate: Some(clk_fixed_rate_recalc_rate),
    recalc_accuracy: Some(clk_fixed_rate_recalc_accuracy),
};

unsafe fn to_clk_fixed_rate(hw: *mut clk_hw) -> *mut clk_fixed_rate {
    (hw as *mut u8).sub(core::mem::offset_of!(clk_fixed_rate, hw)) as *mut clk_fixed_rate
}

unsafe extern "C" fn clk_fixed_rate_recalc_rate(
    hw: *mut clk_hw,
    _parent_rate: c_ulong,
) -> c_ulong {
    (*to_clk_fixed_rate(hw)).fixed_rate
}

unsafe extern "C" fn clk_fixed_rate_recalc_accuracy(
    hw: *mut clk_hw,
    parent_accuracy: c_ulong,
) -> c_ulong {
    let fixed = to_clk_fixed_rate(hw);

    if (*fixed).flags & CLK_FIXED_RATE_PARENT_ACCURACY != 0 {
        return parent_accuracy;
    }

    (*fixed).fixed_accuracy
}

extern "C" {
    fn devres_alloc(release: unsafe extern "C" fn(*mut device, *mut core::ffi::c_void), size: usize, gfp: c_ulong) -> *mut clk_fixed_rate;
    fn kzalloc_obj() -> *mut clk_fixed_rate;
    fn devres_free(ptr: *mut clk_fixed_rate);
    fn kfree(ptr: *mut clk_fixed_rate);
    fn clk_hw_unregister(hw: *mut clk_hw);
    fn clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> c_int;
    fn of_clk_hw_register(node: *mut device_node, hw: *mut clk_hw) -> c_int;
    fn devres_add(dev: *mut device, res: *mut clk_fixed_rate);
    fn clk_hw_register_fixed_rate_with_accuracy(dev: *mut device, name: *const u8, parent_name: *const u8, flags: c_ulong, fixed_rate: c_ulong, fixed_accuracy: c_ulong) -> *mut clk_hw;
    fn __clk_get_hw(clk: *mut clk) -> *mut clk_hw;
    fn clk_unregister(clk: *mut clk);
    fn of_property_read_u32(node: *mut device_node, name: *const u8, value: *mut u32) -> c_int;
    fn of_property_read_string(node: *mut device_node, name: *const u8, value: *mut *const u8) -> c_int;
    fn of_clk_add_hw_provider(node: *mut device_node, get: unsafe extern "C" fn(*mut device_node, *mut u32) -> *mut clk_hw, hw: *mut clk_hw) -> c_int;
    fn of_clk_hw_simple_get(node: *mut device_node, index: *mut u32) -> *mut clk_hw;
    fn of_clk_del_provider(node: *mut device_node);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut clk_hw;
    fn platform_set_drvdata(pdev: *mut platform_device, data: *mut clk_hw);
}

unsafe extern "C" fn devm_clk_hw_register_fixed_rate_release(
    _dev: *mut device,
    res: *mut core::ffi::c_void,
) {
    let fix = res as *mut clk_fixed_rate;

    /*
     * We can not use clk_hw_unregister_fixed_rate, since it will kfree()
     * the hw, resulting in double free. Just unregister the hw and let
     * devres code kfree() it.
     */
    clk_hw_unregister(&mut (*fix).hw);
}

pub unsafe extern "C" fn __clk_hw_register_fixed_rate(
    dev: *mut device,
    np: *mut device_node,
    name: *const u8,
    parent_name: *const u8,
    parent_hw: *const clk_hw,
    parent_data: *const clk_parent_data,
    flags: c_ulong,
    fixed_rate: c_ulong,
    fixed_accuracy: c_ulong,
    clk_fixed_flags: c_ulong,
    devm: bool,
) -> *mut clk_hw {
    let fixed: *mut clk_fixed_rate;
    let hw: *mut clk_hw;
    let mut init: clk_init_data = unsafe { core::mem::zeroed() };
    let mut ret: c_int = -22;

    if devm {
        fixed = devres_alloc(devm_clk_hw_register_fixed_rate_release, core::mem::size_of::<clk_fixed_rate>(), 0);
    } else {
        fixed = kzalloc_obj();
    }
    if fixed.is_null() {
        return core::ptr::null_mut();
    }

    init.name = name;
    init.ops = &clk_fixed_rate_ops;
    init.flags = flags;
    init.parent_names = if !parent_name.is_null() { &parent_name } else { core::ptr::null() };
    init.parent_hws = if !parent_hw.is_null() { &parent_hw } else { core::ptr::null() };
    init.parent_data = parent_data;
    init.num_parents = if !parent_name.is_null() || !parent_hw.is_null() || !parent_data.is_null() { 1 } else { 0 };

    (*fixed).flags = clk_fixed_flags;
    (*fixed).fixed_rate = fixed_rate;
    (*fixed).fixed_accuracy = fixed_accuracy;
    (*fixed).hw.init = &init;

    hw = &mut (*fixed).hw;
    if !dev.is_null() || np.is_null() {
        ret = clk_hw_register(dev, hw);
    } else {
        ret = of_clk_hw_register(np, hw);
    }
    if ret != 0 {
        if devm { devres_free(fixed); } else { kfree(fixed); }
        return core::ptr::null_mut();
    } else if devm {
        devres_add(dev, fixed);
    }

    hw
}

pub unsafe extern "C" fn clk_register_fixed_rate(
    dev: *mut device,
    name: *const u8,
    parent_name: *const u8,
    flags: c_ulong,
    fixed_rate: c_ulong,
) -> *mut clk {
    let hw = clk_hw_register_fixed_rate_with_accuracy(dev, name, parent_name, flags, fixed_rate, 0);
    if hw.is_null() { return core::ptr::null_mut(); }
    (*hw).clk
}

pub unsafe extern "C" fn clk_unregister_fixed_rate(clk: *mut clk) {
    let hw = __clk_get_hw(clk);
    if hw.is_null() { return; }
    clk_unregister(clk);
    kfree(to_clk_fixed_rate(hw));
}

pub unsafe extern "C" fn clk_hw_unregister_fixed_rate(hw: *mut clk_hw) {
    let fixed = to_clk_fixed_rate(hw);
    clk_hw_unregister(hw);
    kfree(fixed);
}

/* CONFIG_OF-dependent declarations and definitions are preserved below. */
#[cfg(feature = "CONFIG_OF")]
unsafe fn _of_fixed_clk_setup(node: *mut device_node) -> *mut clk_hw {
    let mut clk_name = (*node).name;
    let mut rate: u32 = 0;
    let mut accuracy: u32 = 0;

    if of_property_read_u32(node, b"clock-frequency\0".as_ptr(), &mut rate) != 0 {
        return core::ptr::null_mut();
    }
    of_property_read_u32(node, b"clock-accuracy\0".as_ptr(), &mut accuracy);
    of_property_read_string(node, b"clock-output-names\0".as_ptr(), &mut clk_name);

    let hw = clk_hw_register_fixed_rate_with_accuracy(core::ptr::null_mut(), clk_name, core::ptr::null(), 0, rate as c_ulong, accuracy as c_ulong);
    if hw.is_null() { return hw; }
    if of_clk_add_hw_provider(node, of_clk_hw_simple_get, hw) != 0 {
        clk_hw_unregister_fixed_rate(hw);
        return core::ptr::null_mut();
    }
    hw
}

#[cfg(feature = "CONFIG_OF")]
pub unsafe extern "C" fn of_fixed_clk_setup(node: *mut device_node) {
    _of_fixed_clk_setup(node);
}

#[cfg(feature = "CONFIG_OF")]
unsafe fn of_fixed_clk_remove(pdev: *mut platform_device) {
    let hw = platform_get_drvdata(pdev);
    of_clk_del_provider((*pdev).dev.of_node);
    clk_hw_unregister_fixed_rate(hw);
}

#[cfg(feature = "CONFIG_OF")]
unsafe fn of_fixed_clk_probe(pdev: *mut platform_device) -> c_int {
    let hw = _of_fixed_clk_setup((*pdev).dev.of_node);
    if hw.is_null() { return -1; }
    platform_set_drvdata(pdev, hw);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
