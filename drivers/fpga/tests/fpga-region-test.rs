// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for the FPGA Region
 *
 * Copyright (C) 2023 Red Hat, Inc.
 *
 * Author: Marco Pagani <marpagan@redhat.com>
 */

// C dependencies: <kunit/device.h>, <kunit/test.h>,
// <linux/fpga/fpga-bridge.h>, <linux/fpga/fpga-mgr.h>,
// <linux/fpga/fpga-region.h>, <linux/module.h>, and <linux/types.h>.

#[repr(C)]
struct mgr_stats {
    write_count: u32,
}

#[repr(C)]
struct bridge_stats {
    enable: bool,
    cycles_count: u32,
}

#[repr(C)]
struct test_ctx {
    mgr: *mut fpga_manager,
    mgr_dev: *mut device,
    bridge: *mut fpga_bridge,
    bridge_dev: *mut device,
    region: *mut fpga_region,
    region_dev: *mut device,
    bridge_stats: bridge_stats,
    mgr_stats: mgr_stats,
}

// Wrappers to avoid cast warnings when passing action functions directly
// to kunit_add_action().
unsafe extern "C" fn fpga_image_info_free_wrapper(info: *mut fpga_image_info) {
    fpga_image_info_free(info);
}

unsafe extern "C" fn fpga_bridge_unregister_wrapper(bridge: *mut fpga_bridge) {
    fpga_bridge_unregister(bridge);
}

unsafe extern "C" fn fpga_region_unregister_wrapper(region: *mut fpga_region) {
    fpga_region_unregister(region);
}

unsafe extern "C" fn op_write(
    mgr: *mut fpga_manager,
    _buf: *const core::ffi::c_char,
    _count: usize,
) -> i32 {
    let stats = (*mgr).priv_ as *mut mgr_stats;

    (*stats).write_count = (*stats).write_count.wrapping_add(1);

    0
}

/*
 * Fake FPGA manager that implements only the write op to count the number
 * of programming cycles. The internals of the programming sequence are
 * tested in the Manager suite since they are outside the responsibility
 * of the Region.
 */
static fake_mgr_ops: fpga_manager_ops = fpga_manager_ops { write: Some(op_write) };

unsafe extern "C" fn op_enable_set(bridge: *mut fpga_bridge, enable: bool) -> i32 {
    let stats = (*bridge).priv_ as *mut bridge_stats;

    if !(*stats).enable && enable {
        (*stats).cycles_count = (*stats).cycles_count.wrapping_add(1);
    }

    (*stats).enable = enable;

    0
}

/*
 * Fake FPGA bridge that implements only enable_set op to count the number
 * of activation cycles.
 */
static fake_bridge_ops: fpga_bridge_ops = fpga_bridge_ops { enable_set: Some(op_enable_set) };

unsafe extern "C" fn fake_region_get_bridges(region: *mut fpga_region) -> i32 {
    let bridge = (*region).priv_ as *mut fpga_bridge;

    fpga_bridge_get_to_list((*bridge).dev.parent, (*region).info, &mut (*region).bridge_list)
}

unsafe extern "C" fn fake_region_match(dev: *mut device, data: *const core::ffi::c_void) -> i32 {
    if (*dev).parent == data as *mut device { 1 } else { 0 }
}

unsafe extern "C" fn fpga_region_test_class_find(test: *mut kunit) {
    let ctx = (*test).priv_ as *mut test_ctx;

    let region = fpga_region_class_find(core::ptr::null_mut(), (*ctx).region_dev, Some(fake_region_match));
    kunit_expect_ptr_eq(test, region, (*ctx).region);

    put_device(&mut (*region).dev);
}

/*
 * FPGA Region programming test. The Region must call get_bridges() to get
 * and control the bridges, and then the Manager for the actual programming.
 */
unsafe extern "C" fn fpga_region_test_program_fpga(test: *mut kunit) {
    let ctx = (*test).priv_ as *mut test_ctx;
    let mut img_buf = [0i8; 4];

    let img_info = fpga_image_info_alloc((*ctx).mgr_dev);
    kunit_assert_not_err_or_null(test, img_info);

    let mut ret = kunit_add_action_or_reset(test, Some(fpga_image_info_free_wrapper), img_info);
    kunit_assert_eq(test, ret, 0);

    (*img_info).buf = img_buf.as_mut_ptr();
    (*img_info).count = core::mem::size_of_val(&img_buf);

    (*(*ctx).region).info = img_info;
    ret = fpga_region_program_fpga((*ctx).region);
    kunit_assert_eq(test, ret, 0);

    kunit_expect_eq(test, 1, (*ctx).mgr_stats.write_count);
    kunit_expect_eq(test, 1, (*ctx).bridge_stats.cycles_count);

    fpga_bridges_put(&mut (*(*ctx).region).bridge_list);

    ret = fpga_region_program_fpga((*ctx).region);
    kunit_assert_eq(test, ret, 0);

    kunit_expect_eq(test, 2, (*ctx).mgr_stats.write_count);
    kunit_expect_eq(test, 2, (*ctx).bridge_stats.cycles_count);

    fpga_bridges_put(&mut (*(*ctx).region).bridge_list);
}

/*
 * The configuration used in this test suite uses a single bridge to
 * limit the code under test to a single unit. The functions used by the
 * Region for getting and controlling bridges are tested (with a list of
 * multiple bridges) in the Bridge suite.
 */
unsafe extern "C" fn fpga_region_test_init(test: *mut kunit) -> i32 {
    let ctx = kunit_kzalloc(test, core::mem::size_of::<test_ctx>(), GFP_KERNEL);
    kunit_assert_not_err_or_null(test, ctx);

    let ctx = ctx as *mut test_ctx;
    (*ctx).mgr_dev = kunit_device_register(test, b"fpga-manager-test-dev\0".as_ptr() as *const _);
    kunit_assert_not_err_or_null(test, (*ctx).mgr_dev);

    (*ctx).mgr = devm_fpga_mgr_register((*ctx).mgr_dev, b"Fake FPGA Manager\0".as_ptr() as *const _, &fake_mgr_ops, &mut (*ctx).mgr_stats);
    kunit_assert_false(test, is_err_or_null((*ctx).mgr));

    (*ctx).bridge_dev = kunit_device_register(test, b"fpga-bridge-test-dev\0".as_ptr() as *const _);
    kunit_assert_not_err_or_null(test, (*ctx).bridge_dev);

    (*ctx).bridge = fpga_bridge_register((*ctx).bridge_dev, b"Fake FPGA Bridge\0".as_ptr() as *const _, &fake_bridge_ops, &mut (*ctx).bridge_stats);
    kunit_assert_false(test, is_err_or_null((*ctx).bridge));

    (*ctx).bridge_stats.enable = true;

    let mut ret = kunit_add_action_or_reset(test, Some(fpga_bridge_unregister_wrapper), (*ctx).bridge);
    kunit_assert_eq(test, ret, 0);

    (*ctx).region_dev = kunit_device_register(test, b"fpga-region-test-dev\0".as_ptr() as *const _);
    kunit_assert_not_err_or_null(test, (*ctx).region_dev);

    let region_info = fpga_region_info {
        mgr: (*ctx).mgr,
        priv_: (*ctx).bridge as *mut core::ffi::c_void,
        get_bridges: Some(fake_region_get_bridges),
    };

    (*ctx).region = fpga_region_register_full((*ctx).region_dev, &region_info);
    kunit_assert_false(test, is_err_or_null((*ctx).region));

    ret = kunit_add_action_or_reset(test, Some(fpga_region_unregister_wrapper), (*ctx).region);
    kunit_assert_eq(test, ret, 0);

    (*test).priv_ = ctx as *mut core::ffi::c_void;

    0
}

static mut fpga_region_test_cases: [kunit_case; 3] = [
    kunit_case { run_case: Some(fpga_region_test_class_find) },
    kunit_case { run_case: Some(fpga_region_test_program_fpga) },
    kunit_case { run_case: None },
];

static mut fpga_region_suite: kunit_suite = kunit_suite {
    name: b"fpga_region\0".as_ptr() as *const _,
    init: Some(fpga_region_test_init),
    test_cases: fpga_region_test_cases.as_mut_ptr(),
};

// kunit_test_suite(fpga_region_suite);
// MODULE_DESCRIPTION("KUnit test for the FPGA Region");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
