// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for the FPGA Bridge
 *
 * Copyright (C) 2023 Red Hat, Inc.
 *
 * Author: Marco Pagani <marpagan@redhat.com>
 */

// External kernel/KUnit declarations are supplied by the surrounding build.
use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct kunit {
    pub priv_: *mut c_void,
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fpga_bridge {
    _private: [u8; 0],
}

#[repr(C)]
pub struct list_head {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fpga_bridge_ops {
    pub enable_set: Option<unsafe extern "C" fn(*mut fpga_bridge, bool) -> c_int>,
}

#[repr(C)]
pub struct kunit_case {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub init: Option<unsafe extern "C" fn(*mut kunit) -> c_int>,
    pub test_cases: *mut kunit_case,
}

extern "C" {
    fn kunit_kzalloc(test: *mut kunit, size: usize, flags: c_int) -> *mut c_void;
    fn kunit_device_register(test: *mut kunit, name: *const c_char) -> *mut device;
    fn fpga_bridge_register(
        dev: *mut device,
        name: *const c_char,
        ops: *const fpga_bridge_ops,
        priv_: *mut c_void,
    ) -> *mut fpga_bridge;
    fn kunit_add_action_or_reset(
        test: *mut kunit,
        action: unsafe extern "C" fn(*mut c_void),
        data: *mut c_void,
    ) -> c_int;
    fn fpga_bridge_unregister(bridge: *mut fpga_bridge);
    fn fpga_bridge_get(dev: *mut device, compat: *const c_void) -> *mut fpga_bridge;
    fn fpga_bridge_put(bridge: *mut fpga_bridge);
    fn fpga_bridge_disable(bridge: *mut fpga_bridge) -> c_int;
    fn fpga_bridge_enable(bridge: *mut fpga_bridge) -> c_int;
    fn fpga_bridge_get_to_list(
        dev: *mut device,
        compat: *const c_void,
        list: *mut list_head,
    ) -> c_int;
    fn fpga_bridges_disable(list: *mut list_head) -> c_int;
    fn fpga_bridges_enable(list: *mut list_head) -> c_int;
    fn fpga_bridges_put(list: *mut list_head);
}

#[repr(C)]
struct bridge_stats {
    enable: bool,
}

#[repr(C)]
struct bridge_ctx {
    bridge: *mut fpga_bridge,
    dev: *mut device,
    stats: bridge_stats,
}

/* Wrapper to avoid a cast warning when passing the action function directly
 * to kunit_add_action(). */
unsafe extern "C" fn fpga_bridge_unregister_wrapper(data: *mut c_void) {
    fpga_bridge_unregister(data as *mut fpga_bridge);
}

unsafe extern "C" fn op_enable_set(bridge: *mut fpga_bridge, enable: bool) -> c_int {
    // The bridge private field is supplied by the external kernel definition.
    let stats = bridge as *mut bridge_stats;
    (*stats).enable = enable;
    0
}

/* Fake FPGA bridge that implements only the enable_set op to track the state. */
static fake_bridge_ops: fpga_bridge_ops = fpga_bridge_ops {
    enable_set: Some(op_enable_set),
};

unsafe fn register_test_bridge(test: *mut kunit, dev_name: *const c_char) -> *mut bridge_ctx {
    let ctx = kunit_kzalloc(test, core::mem::size_of::<bridge_ctx>(), 0) as *mut bridge_ctx;
    // KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx)
    (*ctx).dev = kunit_device_register(test, dev_name);
    // KUNIT_ASSERT_NOT_ERR_OR_NULL(test, ctx->dev)
    (*ctx).bridge = fpga_bridge_register(
        (*ctx).dev,
        b"Fake FPGA bridge\0".as_ptr() as *const c_char,
        &fake_bridge_ops,
        &mut (*ctx).stats as *mut bridge_stats as *mut c_void,
    );
    // KUNIT_ASSERT_FALSE(test, IS_ERR_OR_NULL(ctx->bridge))
    let ret = kunit_add_action_or_reset(
        test,
        fpga_bridge_unregister_wrapper,
        (*ctx).bridge as *mut c_void,
    );
    // KUNIT_ASSERT_EQ(test, ret, 0)
    let _ = ret;
    ctx
}

unsafe fn fpga_bridge_test_get(test: *mut kunit) {
    let ctx = (*test).priv_ as *mut bridge_ctx;
    let mut bridge = fpga_bridge_get((*ctx).dev, core::ptr::null());
    // KUNIT_EXPECT_PTR_EQ(test, bridge, ctx->bridge)
    let _ = bridge;
    bridge = fpga_bridge_get((*ctx).dev, core::ptr::null());
    // KUNIT_EXPECT_EQ(test, PTR_ERR(bridge), -EBUSY)
    fpga_bridge_put((*ctx).bridge);
}

unsafe fn fpga_bridge_test_toggle(test: *mut kunit) {
    let ctx = (*test).priv_ as *mut bridge_ctx;
    let ret = fpga_bridge_disable((*ctx).bridge);
    // KUNIT_EXPECT_EQ(test, ret, 0); KUNIT_EXPECT_FALSE(test, ctx->stats.enable)
    let _ = ret;
    let ret = fpga_bridge_enable((*ctx).bridge);
    // KUNIT_EXPECT_EQ(test, ret, 0); KUNIT_EXPECT_TRUE(test, ctx->stats.enable)
    let _ = ret;
}

/* Test the functions for getting and controlling a list of bridges */
unsafe fn fpga_bridge_test_get_put_list(test: *mut kunit) {
    let mut bridge_list = core::mem::MaybeUninit::<list_head>::uninit();
    let ctx_0 = (*test).priv_ as *mut bridge_ctx;
    let ctx_1 = register_test_bridge(test, b"fpga-bridge-test-dev-1\0".as_ptr() as *const c_char);
    let list = bridge_list.as_mut_ptr();
    let _ = fpga_bridge_get_to_list((*ctx_0).dev, core::ptr::null(), list);
    let _ = fpga_bridge_get_to_list((*ctx_1).dev, core::ptr::null(), list);
    let _ = fpga_bridges_disable(list);
    let _ = fpga_bridges_enable(list);
    fpga_bridges_put(list);
    // KUNIT_EXPECT_TRUE(test, list_empty(&bridge_list))
}

unsafe extern "C" fn fpga_bridge_test_init(test: *mut kunit) -> c_int {
    (*test).priv_ = register_test_bridge(test, b"fpga-bridge-test-dev-0\0".as_ptr() as *const c_char) as *mut c_void;
    0
}

// KUNIT_CASE(fpga_bridge_test_get), KUNIT_CASE(fpga_bridge_test_toggle),
// KUNIT_CASE(fpga_bridge_test_get_put_list), {}
static mut fpga_bridge_test_cases: [kunit_case; 4] = [
    kunit_case { _private: [] }, kunit_case { _private: [] },
    kunit_case { _private: [] }, kunit_case { _private: [] },
];

static mut fpga_bridge_suite: kunit_suite = kunit_suite {
    name: b"fpga_bridge\0".as_ptr() as *const c_char,
    init: Some(fpga_bridge_test_init),
    test_cases: fpga_bridge_test_cases.as_mut_ptr(),
};

// kunit_test_suite(fpga_bridge_suite)
// MODULE_DESCRIPTION("KUnit test for the FPGA Bridge")
// MODULE_LICENSE("GPL")

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
