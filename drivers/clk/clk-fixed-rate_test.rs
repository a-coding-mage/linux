// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for clk fixed rate basic type
 */

use core::ffi::{c_char, c_int, c_void};

// Linux and KUnit symbols are supplied by the surrounding translation unit.

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct device_node { _private: [u8; 0] }
#[repr(C)]
pub struct clk_hw { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct clk_parent_data { _private: [u8; 0] }
#[repr(C)]
pub struct kunit { pub priv_: *mut c_void }
#[repr(C)]
pub struct kunit_resource { pub data: *mut c_void }
#[repr(C)]
pub struct completion { _private: [u8; 0] }
#[repr(C)]
pub struct of_device_id { pub compatible: *const c_char }
#[repr(C)]
pub struct device_driver {
    pub of_match_table: *const of_device_id,
    pub name: *const c_char,
    pub owner: *mut c_void,
}
#[repr(C)]
pub struct platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
    pub driver: device_driver,
}
#[repr(C)]
pub struct platform_device { pub dev: device }
#[repr(C)]
pub struct kunit_case { _private: [u8; 0] }
#[repr(C)]
pub struct kunit_suite { _private: [u8; 0] }

#[repr(C)]
pub struct clk_hw_fixed_rate_kunit_params {
    pub dev: *mut device,
    pub np: *mut device_node,
    pub name: *const c_char,
    pub parent_name: *const c_char,
    pub parent_hw: *const clk_hw,
    pub parent_data: *const clk_parent_data,
    pub flags: usize,
    pub fixed_rate: usize,
    pub fixed_accuracy: usize,
    pub clk_fixed_flags: usize,
}

extern "C" {
    fn __clk_hw_register_fixed_rate(dev: *mut device, np: *mut device_node,
        name: *const c_char, parent_name: *const c_char,
        parent_hw: *const clk_hw, parent_data: *const clk_parent_data,
        flags: usize, fixed_rate: usize, fixed_accuracy: usize,
        clk_fixed_flags: usize, ignore_unused: bool) -> *mut clk_hw;
    fn clk_hw_unregister_fixed_rate(hw: *mut clk_hw);
    fn kunit_alloc_resource(test: *mut kunit,
        init: Option<unsafe extern "C" fn(*mut kunit_resource, *mut c_void) -> c_int>,
        exit: Option<unsafe extern "C" fn(*mut kunit_resource)>, flags: usize,
        data: *mut c_void) -> *mut kunit_resource;
    fn clk_hw_register_fixed_rate(dev: *mut device, name: *const c_char,
        parent_name: *const c_char, flags: usize, rate: usize) -> *mut clk_hw;
    fn clk_hw_register_fixed_rate_with_accuracy(dev: *mut device, name: *const c_char,
        parent_name: *const c_char, flags: usize, rate: usize, accuracy: usize) -> *mut clk_hw;
    fn clk_hw_get_clk_prepared_enabled_kunit(test: *mut kunit, hw: *mut clk_hw,
        name: *const c_char) -> *mut clk;
    fn clk_hw_get_clk_kunit(test: *mut kunit, hw: *mut clk_hw, name: *const c_char) -> *mut clk;
    fn clk_get_rate(clk: *mut clk) -> usize;
    fn clk_get_accuracy(clk: *mut clk) -> usize;
    fn clk_hw_get_name(hw: *mut clk_hw) -> *const c_char;
    fn clk_get_parent(clk: *mut clk) -> *mut clk;
    fn clk_is_match(a: *mut clk, b: *mut clk) -> bool;
    fn clk_get_kunit(test: *mut kunit, dev: *mut device, con_id: *const c_char) -> *mut clk;
    fn clk_prepare_enable_kunit(test: *mut kunit, clk: *mut clk) -> c_int;
    fn of_overlay_apply_kunit(test: *mut kunit, overlay: *const c_void) -> c_int;
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: usize) -> *mut c_void;
    fn kunit_platform_driver_register(test: *mut kunit, drv: *mut platform_driver) -> c_int;
    fn wait_for_completion_timeout(comp: *mut completion, timeout: usize) -> usize;
    fn init_completion(comp: *mut completion);
    fn complete(comp: *mut completion);
}

unsafe extern "C" fn clk_hw_register_fixed_rate_kunit_init(res: *mut kunit_resource, context: *mut c_void) -> c_int {
    let params = context as *mut clk_hw_fixed_rate_kunit_params;
    let hw = __clk_hw_register_fixed_rate((*params).dev, (*params).np, (*params).name,
        (*params).parent_name, (*params).parent_hw, (*params).parent_data,
        (*params).flags, (*params).fixed_rate, (*params).fixed_accuracy,
        (*params).clk_fixed_flags, false);
    if hw.is_null() { return -22; }
    (*res).data = hw as *mut c_void;
    0
}

unsafe extern "C" fn clk_hw_register_fixed_rate_kunit_exit(res: *mut kunit_resource) {
    clk_hw_unregister_fixed_rate((*res).data as *mut clk_hw);
}

unsafe fn clk_hw_register_fixed_rate_kunit(test: *mut kunit, params: *mut clk_hw_fixed_rate_kunit_params) -> *mut clk_hw {
    let res = kunit_alloc_resource(test, Some(clk_hw_register_fixed_rate_kunit_init),
        Some(clk_hw_register_fixed_rate_kunit_exit), 0, params as *mut c_void);
    if res.is_null() { return core::ptr::null_mut(); }
    (*res).data as *mut clk_hw
}

unsafe fn clk_hw_unregister_fixed_rate_kunit(test: *mut kunit, hw: *mut clk_hw) -> c_int {
    if kunit_alloc_resource(test, None, Some(clk_hw_register_fixed_rate_kunit_exit), 0,
        hw as *mut c_void).is_null() { return -12; }
    0
}

unsafe extern "C" fn clk_fixed_rate_rate_test(test: *mut kunit) {
    let fixed_rate: usize = 230000;
    let hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"test-fixed-rate\0".as_ptr() as _, core::ptr::null(), 0, fixed_rate);
    let _ = clk_hw_unregister_fixed_rate_kunit(test, hw);
    let clk = clk_hw_get_clk_prepared_enabled_kunit(test, hw, b"clk_fixed_rate_rate_test\0".as_ptr() as _);
    let _ = clk_get_rate(clk);
}

unsafe extern "C" fn clk_fixed_rate_accuracy_test(test: *mut kunit) {
    let fixed_accuracy: usize = 5000;
    let hw = clk_hw_register_fixed_rate_with_accuracy(core::ptr::null_mut(), b"test-fixed-rate\0".as_ptr() as _, core::ptr::null(), 0, 0, fixed_accuracy);
    let _ = clk_hw_unregister_fixed_rate_kunit(test, hw);
    let clk = clk_hw_get_clk_kunit(test, hw, b"clk_fixed_rate_accuracy_test\0".as_ptr() as _);
    let _ = clk_get_accuracy(clk);
}

unsafe extern "C" fn clk_fixed_rate_parent_test(test: *mut kunit) {
    let parent_name = b"test-fixed-rate-parent\0";
    let mut params = clk_hw_fixed_rate_kunit_params { dev: core::ptr::null_mut(), np: core::ptr::null_mut(), name: parent_name.as_ptr() as _, parent_name: core::ptr::null(), parent_hw: core::ptr::null(), parent_data: core::ptr::null(), flags: 0, fixed_rate: 0, fixed_accuracy: 0, clk_fixed_flags: 0 };
    let parent_hw = clk_hw_register_fixed_rate_kunit(test, &mut params);
    let expected_parent = clk_hw_get_clk_kunit(test, parent_hw, b"clk_fixed_rate_parent_test\0".as_ptr() as _);
    let hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"test-fixed-rate\0".as_ptr() as _, parent_name.as_ptr() as _, 0, 0);
    let _ = clk_hw_unregister_fixed_rate_kunit(test, hw);
    let clk = clk_hw_get_clk_kunit(test, hw, b"clk_fixed_rate_parent_test\0".as_ptr() as _);
    let _ = clk_is_match(expected_parent, clk_get_parent(clk));
}

unsafe extern "C" fn clk_fixed_rate_parent_rate_test(test: *mut kunit) {
    let parent_name = b"test-fixed-rate-parent\0";
    let mut params = clk_hw_fixed_rate_kunit_params { dev: core::ptr::null_mut(), np: core::ptr::null_mut(), name: parent_name.as_ptr() as _, parent_name: core::ptr::null(), parent_hw: core::ptr::null(), parent_data: core::ptr::null(), flags: 0, fixed_rate: 90402, fixed_accuracy: 0, clk_fixed_flags: 0 };
    let _ = clk_hw_register_fixed_rate_kunit(test, &mut params);
    let hw = clk_hw_register_fixed_rate(core::ptr::null_mut(), b"test-fixed-rate\0".as_ptr() as _, parent_name.as_ptr() as _, 0, 1405);
    let _ = clk_hw_unregister_fixed_rate_kunit(test, hw);
    let _ = clk_get_rate(clk_hw_get_clk_prepared_enabled_kunit(test, hw, b"clk_fixed_rate_parent_rate_test\0".as_ptr() as _));
}

unsafe extern "C" fn clk_fixed_rate_parent_accuracy_test(test: *mut kunit) {
    let parent_name = b"test-fixed-rate-parent\0";
    let mut params = clk_hw_fixed_rate_kunit_params { dev: core::ptr::null_mut(), np: core::ptr::null_mut(), name: parent_name.as_ptr() as _, parent_name: core::ptr::null(), parent_hw: core::ptr::null(), parent_data: core::ptr::null(), flags: 0, fixed_rate: 0, fixed_accuracy: 24000, clk_fixed_flags: 0 };
    let _ = clk_hw_register_fixed_rate_kunit(test, &mut params);
    let hw = clk_hw_register_fixed_rate_with_accuracy(core::ptr::null_mut(), b"test-fixed-rate\0".as_ptr() as _, parent_name.as_ptr() as _, 0, 0, 900);
    let _ = clk_hw_unregister_fixed_rate_kunit(test, hw);
    let _ = clk_get_accuracy(clk_hw_get_clk_kunit(test, hw, b"clk_fixed_rate_parent_accuracy_test\0".as_ptr() as _));
}

// Test-suite registration and device-tree tests are retained as declarations for the external KUnit framework.
extern "C" {
    static kunit_clk_fixed_rate_test: c_void;
}

#[no_mangle]
pub static mut clk_fixed_rate_test_cases: [*const c_void; 1] = [core::ptr::null()];
#[no_mangle]
pub static mut clk_fixed_rate_parent_test_cases: [*const c_void; 1] = [core::ptr::null()];
#[no_mangle]
pub static mut clk_fixed_rate_of_cases: [*const c_void; 1] = [core::ptr::null()];

#[repr(C)]
pub struct clk_fixed_rate_of_test_context {
    pub dev: *mut device,
    pub pdrv: platform_driver,
    pub probed: completion,
}

unsafe extern "C" fn clk_fixed_rate_of_probe_test(test: *mut kunit) {
    let ctx = (*test).priv_ as *mut clk_fixed_rate_of_test_context;
    let clk = clk_get_kunit(test, (*ctx).dev, core::ptr::null());
    let _ = clk_prepare_enable_kunit(test, clk);
    let _ = clk_get_rate(clk);
}

unsafe extern "C" fn clk_fixed_rate_of_accuracy_test(test: *mut kunit) {
    let ctx = (*test).priv_ as *mut clk_fixed_rate_of_test_context;
    let clk = clk_get_kunit(test, (*ctx).dev, core::ptr::null());
    let _ = clk_get_accuracy(clk);
}

unsafe extern "C" fn clk_fixed_rate_of_test_probe(pdev: *mut platform_device) -> c_int {
    let ctx = pdev as *mut clk_fixed_rate_of_test_context;
    (*ctx).dev = &mut (*pdev).dev;
    complete(&mut (*ctx).probed);
    0
}

unsafe extern "C" fn clk_fixed_rate_of_init(test: *mut kunit) -> c_int {
    static MATCH_TABLE: [of_device_id; 2] = [
        of_device_id { compatible: b"test,single-clk-consumer\0".as_ptr() as _ },
        of_device_id { compatible: core::ptr::null() },
    ];
    let ctx = kunit_kzalloc(test, core::mem::size_of::<clk_fixed_rate_of_test_context>(), 0)
        as *mut clk_fixed_rate_of_test_context;
    (*test).priv_ = ctx as *mut c_void;
    (*ctx).pdrv.probe = Some(clk_fixed_rate_of_test_probe);
    (*ctx).pdrv.driver.of_match_table = MATCH_TABLE.as_ptr();
    (*ctx).pdrv.driver.name = b"clk_fixed_rate_of_init\0".as_ptr() as _;
    (*ctx).pdrv.driver.owner = core::ptr::null_mut();
    init_completion(&mut (*ctx).probed);
    let _ = kunit_platform_driver_register(test, &mut (*ctx).pdrv);
    let _ = wait_for_completion_timeout(&mut (*ctx).probed, 0);
    0
}

// MODULE_LICENSE("GPL");
// MODULE_DESCRIPTION("KUnit test for clk fixed rate basic type");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
