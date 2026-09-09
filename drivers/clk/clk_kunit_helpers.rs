// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit helpers for clk providers and consumers
 */

// Declarations supplied by the corresponding kernel headers and other units.

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk {
    _private: [u8; 0],
}
#[repr(C)]
pub struct clk_hw {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device {
    _private: [u8; 0],
}
#[repr(C)]
pub struct device_node {
    _private: [u8; 0],
}
#[repr(C)]
pub struct of_phandle_args {
    _private: [u8; 0],
}

type ClkProviderGet = unsafe extern "C" fn(*mut of_phandle_args, *mut core::ffi::c_void) -> *mut clk_hw;
type KunitAction = unsafe extern "C" fn(*mut core::ffi::c_void);

extern "C" {
    fn clk_disable_unprepare(clk: *mut clk);
    fn clk_put(clk: *mut clk);
    fn clk_prepare_enable(clk: *mut clk) -> i32;
    fn clk_get(dev: *mut device, con_id: *const core::ffi::c_char) -> *mut clk;
    fn of_clk_get(np: *mut device_node, index: i32) -> *mut clk;
    fn clk_hw_get_clk(hw: *mut clk_hw, con_id: *const core::ffi::c_char) -> *mut clk;
    fn clk_hw_unregister(hw: *mut clk_hw);
    fn clk_hw_register(dev: *mut device, hw: *mut clk_hw) -> i32;
    fn of_clk_hw_register(node: *mut device_node, hw: *mut clk_hw) -> i32;
    fn of_clk_add_hw_provider(np: *mut device_node, get: Option<ClkProviderGet>, data: *mut core::ffi::c_void) -> i32;
    fn of_clk_del_provider(np: *mut device_node);
    fn kunit_add_action_or_reset(test: *mut kunit, action: KunitAction, data: *mut core::ffi::c_void) -> i32;
}

unsafe extern "C" fn clk_disable_unprepare_wrapper(data: *mut core::ffi::c_void) {
    clk_disable_unprepare(data as *mut clk);
}

/// Test managed clk_prepare_enable().
pub unsafe extern "C" fn clk_prepare_enable_kunit(test: *mut kunit, clk: *mut clk) -> i32 {
    let ret = clk_prepare_enable(clk);
    if ret != 0 {
        return ret;
    }
    kunit_add_action_or_reset(test, clk_disable_unprepare_wrapper, clk as *mut core::ffi::c_void)
}

unsafe extern "C" fn clk_put_wrapper(data: *mut core::ffi::c_void) {
    clk_put(data as *mut clk);
}

unsafe fn __clk_get_kunit(test: *mut kunit, clk: *mut clk) -> *mut clk {
    if (clk as isize) < 0 && (clk as usize) >= (-4095isize as usize) {
        return clk;
    }
    let ret = kunit_add_action_or_reset(test, clk_put_wrapper, clk as *mut core::ffi::c_void);
    if ret != 0 {
        return (-ret as isize) as *mut clk;
    }
    clk
}

/// Test managed clk_get().
pub unsafe extern "C" fn clk_get_kunit(test: *mut kunit, dev: *mut device, con_id: *const core::ffi::c_char) -> *mut clk {
    __clk_get_kunit(test, clk_get(dev, con_id))
}

/// Test managed of_clk_get().
pub unsafe extern "C" fn of_clk_get_kunit(test: *mut kunit, np: *mut device_node, index: i32) -> *mut clk {
    __clk_get_kunit(test, of_clk_get(np, index))
}

/// Test managed clk_hw_get_clk().
pub unsafe extern "C" fn clk_hw_get_clk_kunit(test: *mut kunit, hw: *mut clk_hw, con_id: *const core::ffi::c_char) -> *mut clk {
    __clk_get_kunit(test, clk_hw_get_clk(hw, con_id))
}

/// Test managed clk_hw_get_clk() + clk_prepare_enable().
pub unsafe extern "C" fn clk_hw_get_clk_prepared_enabled_kunit(test: *mut kunit, hw: *mut clk_hw, con_id: *const core::ffi::c_char) -> *mut clk {
    let clk = clk_hw_get_clk_kunit(test, hw, con_id);
    if (clk as isize) < 0 && (clk as usize) >= (-4095isize as usize) {
        return clk;
    }
    let ret = clk_prepare_enable_kunit(test, clk);
    if ret != 0 {
        return (-ret as isize) as *mut clk;
    }
    clk
}

unsafe extern "C" fn clk_hw_unregister_wrapper(data: *mut core::ffi::c_void) {
    clk_hw_unregister(data as *mut clk_hw);
}

/// Test managed clk_hw_register().
pub unsafe extern "C" fn clk_hw_register_kunit(test: *mut kunit, dev: *mut device, hw: *mut clk_hw) -> i32 {
    let ret = clk_hw_register(dev, hw);
    if ret != 0 { return ret; }
    kunit_add_action_or_reset(test, clk_hw_unregister_wrapper, hw as *mut core::ffi::c_void)
}

/// Test managed of_clk_hw_register().
pub unsafe extern "C" fn of_clk_hw_register_kunit(test: *mut kunit, node: *mut device_node, hw: *mut clk_hw) -> i32 {
    let ret = of_clk_hw_register(node, hw);
    if ret != 0 { return ret; }
    kunit_add_action_or_reset(test, clk_hw_unregister_wrapper, hw as *mut core::ffi::c_void)
}

unsafe extern "C" fn of_clk_del_provider_wrapper(data: *mut core::ffi::c_void) {
    of_clk_del_provider(data as *mut device_node);
}

/// Test managed of_clk_add_hw_provider().
pub unsafe extern "C" fn of_clk_add_hw_provider_kunit(test: *mut kunit, np: *mut device_node, get: Option<ClkProviderGet>, data: *mut core::ffi::c_void) -> i32 {
    let ret = of_clk_add_hw_provider(np, get, data);
    if ret != 0 { return ret; }
    kunit_add_action_or_reset(test, of_clk_del_provider_wrapper, np as *mut core::ffi::c_void)
}

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("KUnit helpers for clk providers and consumers");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
