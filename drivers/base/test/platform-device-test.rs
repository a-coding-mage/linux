// SPDX-License-Identifier: GPL-2.0

// Kernel dependencies supplied by the surrounding tree.

const DEVICE_NAME: *const core::ffi::c_char = c"test".as_ptr();

#[repr(C)]
struct test_priv {
    probe_done: bool,
    release_done: bool,
    probe_wq: wait_queue_head_t,
    release_wq: wait_queue_head_t,
    dev: *mut device,
}

unsafe fn platform_device_devm_init(test: *mut kunit) -> i32 {
    let priv_: *mut test_priv = kunit_kzalloc(test, core::mem::size_of::<test_priv>(), GFP_KERNEL) as *mut test_priv;
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv_);
    init_waitqueue_head(&mut (*priv_).probe_wq);
    init_waitqueue_head(&mut (*priv_).release_wq);
    (*test).priv_ = priv_ as *mut core::ffi::c_void;
    0
}

unsafe fn devm_device_action(ptr: *mut core::ffi::c_void) {
    let priv_: *mut test_priv = ptr as *mut test_priv;
    (*priv_).release_done = true;
    wake_up_interruptible(&mut (*priv_).release_wq);
}

unsafe fn devm_put_device_action(ptr: *mut core::ffi::c_void) {
    let priv_: *mut test_priv = ptr as *mut test_priv;
    put_device((*priv_).dev);
    (*priv_).release_done = true;
    wake_up_interruptible(&mut (*priv_).release_wq);
}

const RELEASE_TIMEOUT_MS: i32 = 100;

unsafe fn platform_device_devm_register_unregister_test(test: *mut kunit) {
    let pdev: *mut platform_device = platform_device_alloc(DEVICE_NAME, PLATFORM_DEVID_NONE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    let mut ret = platform_device_add(pdev);
    KUNIT_ASSERT_EQ(test, ret, 0);
    let priv_: *mut test_priv = (*test).priv_ as *mut test_priv;
    (*priv_).dev = &mut (*pdev).dev;
    ret = devm_add_action_or_reset((*priv_).dev, devm_device_action, priv_ as *mut core::ffi::c_void);
    KUNIT_ASSERT_EQ(test, ret, 0);
    platform_device_unregister(pdev);
    ret = wait_event_interruptible_timeout(&mut (*priv_).release_wq, (*priv_).release_done, msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_EXPECT_GT(test, ret, 0);
}

unsafe fn platform_device_devm_register_get_unregister_with_devm_test(test: *mut kunit) {
    let pdev = platform_device_alloc(DEVICE_NAME, PLATFORM_DEVID_NONE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    let mut ret = platform_device_add(pdev);
    KUNIT_ASSERT_EQ(test, ret, 0);
    let priv_: *mut test_priv = (*test).priv_ as *mut test_priv;
    (*priv_).dev = &mut (*pdev).dev;
    get_device((*priv_).dev);
    ret = devm_add_action_or_reset((*priv_).dev, devm_put_device_action, priv_ as *mut core::ffi::c_void);
    KUNIT_ASSERT_EQ(test, ret, 0);
    platform_device_unregister(pdev);
    ret = wait_event_interruptible_timeout(&mut (*priv_).release_wq, (*priv_).release_done, msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_EXPECT_GT(test, ret, 0);
}

unsafe fn fake_probe(pdev: *mut platform_device) -> i32 {
    let priv_: *mut test_priv = platform_get_drvdata(pdev) as *mut test_priv;
    (*priv_).probe_done = true;
    wake_up_interruptible(&mut (*priv_).probe_wq);
    0
}

static mut fake_driver: platform_driver = platform_driver {
    probe: Some(fake_probe),
    driver: device_driver { name: DEVICE_NAME },
};

unsafe fn probed_platform_device_devm_register_unregister_test(test: *mut kunit) {
    let mut ret = platform_driver_register(&mut fake_driver);
    KUNIT_ASSERT_EQ(test, ret, 0);
    let pdev = platform_device_alloc(DEVICE_NAME, PLATFORM_DEVID_NONE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    let priv_: *mut test_priv = (*test).priv_ as *mut test_priv;
    (*priv_).dev = &mut (*pdev).dev;
    platform_set_drvdata(pdev, priv_ as *mut core::ffi::c_void);
    ret = platform_device_add(pdev);
    KUNIT_ASSERT_EQ(test, ret, 0);
    ret = wait_event_interruptible_timeout(&mut (*priv_).probe_wq, (*priv_).probe_done, msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_ASSERT_GT(test, ret, 0);
    ret = devm_add_action_or_reset((*priv_).dev, devm_device_action, priv_ as *mut core::ffi::c_void);
    KUNIT_ASSERT_EQ(test, ret, 0);
    platform_device_unregister(pdev);
    ret = wait_event_interruptible_timeout(&mut (*priv_).release_wq, (*priv_).release_done, msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_EXPECT_GT(test, ret, 0);
    platform_driver_unregister(&mut fake_driver);
}

unsafe fn probed_platform_device_devm_register_get_unregister_with_devm_test(test: *mut kunit) {
    let mut ret = platform_driver_register(&mut fake_driver);
    KUNIT_ASSERT_EQ(test, ret, 0);
    let pdev = platform_device_alloc(DEVICE_NAME, PLATFORM_DEVID_NONE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    let priv_: *mut test_priv = (*test).priv_ as *mut test_priv;
    (*priv_).dev = &mut (*pdev).dev;
    platform_set_drvdata(pdev, priv_ as *mut core::ffi::c_void);
    ret = platform_device_add(pdev);
    KUNIT_ASSERT_EQ(test, ret, 0);
    ret = wait_event_interruptible_timeout(&mut (*priv_).probe_wq, (*priv_).probe_done, msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_ASSERT_GT(test, ret, 0);
    get_device((*priv_).dev);
    ret = devm_add_action_or_reset((*priv_).dev, devm_put_device_action, priv_ as *mut core::ffi::c_void);
    KUNIT_ASSERT_EQ(test, ret, 0);
    platform_device_unregister(pdev);
    ret = wait_event_interruptible_timeout(&mut (*priv_).release_wq, (*priv_).release_done, msecs_to_jiffies(RELEASE_TIMEOUT_MS));
    KUNIT_EXPECT_GT(test, ret, 0);
    platform_driver_unregister(&mut fake_driver);
}

static mut platform_device_devm_tests: [kunit_case; 5] = [
    KUNIT_CASE!(platform_device_devm_register_unregister_test),
    KUNIT_CASE!(platform_device_devm_register_get_unregister_with_devm_test),
    KUNIT_CASE!(probed_platform_device_devm_register_unregister_test),
    KUNIT_CASE!(probed_platform_device_devm_register_get_unregister_with_devm_test),
    KUNIT_CASE!(),
];

static mut platform_device_devm_test_suite: kunit_suite = kunit_suite {
    name: c"platform-device-devm".as_ptr(),
    init: Some(platform_device_devm_init),
    test_cases: platform_device_devm_tests.as_mut_ptr(),
};

unsafe fn platform_device_find_by_null_test(test: *mut kunit) {
    let pdev = kunit_platform_device_alloc(test, DEVICE_NAME, PLATFORM_DEVID_NONE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    let ret = kunit_platform_device_add(test, pdev);
    KUNIT_ASSERT_EQ(test, ret, 0);
    KUNIT_EXPECT_PTR_EQ(test, of_find_device_by_node(core::ptr::null_mut()), core::ptr::null_mut());
    KUNIT_EXPECT_PTR_EQ(test, bus_find_device_by_of_node(&mut platform_bus_type, core::ptr::null_mut()), core::ptr::null_mut());
    KUNIT_EXPECT_PTR_EQ(test, bus_find_device_by_fwnode(&mut platform_bus_type, core::ptr::null_mut()), core::ptr::null_mut());
    KUNIT_EXPECT_PTR_EQ(test, bus_find_device_by_acpi_dev(&mut platform_bus_type, core::ptr::null_mut()), core::ptr::null_mut());
    KUNIT_EXPECT_FALSE(test, device_match_of_node(&mut (*pdev).dev, core::ptr::null_mut()));
    KUNIT_EXPECT_FALSE(test, device_match_fwnode(&mut (*pdev).dev, core::ptr::null_mut()));
    KUNIT_EXPECT_FALSE(test, device_match_acpi_dev(&mut (*pdev).dev, core::ptr::null_mut()));
    KUNIT_EXPECT_FALSE(test, device_match_acpi_handle(&mut (*pdev).dev, core::ptr::null_mut()));
}

static mut platform_device_match_tests: [kunit_case; 2] = [KUNIT_CASE!(platform_device_find_by_null_test), KUNIT_CASE!()];
static mut platform_device_match_test_suite: kunit_suite = kunit_suite { name: c"platform-device-match".as_ptr(), test_cases: platform_device_match_tests.as_mut_ptr(), ..Default::default() };

unsafe fn platform_device_swnode_test_probe(_pdev: *mut platform_device) -> i32 { 0 }
static mut platform_swnode_test_driver: platform_driver = platform_driver { probe: Some(platform_device_swnode_test_probe), driver: device_driver { name: DEVICE_NAME } };
static platform_device_test_swnode: software_node = software_node { };

unsafe fn platform_device_swnode_add_twice(test: *mut kunit) {
    let fwnode = kunit_kzalloc(test, core::mem::size_of::<fwnode_handle>(), GFP_KERNEL) as *mut fwnode_handle;
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, fwnode);
    let mut ret = kunit_platform_driver_register(test, &mut platform_swnode_test_driver);
    KUNIT_ASSERT_EQ(test, ret, 0);
    fwnode_init(fwnode, core::ptr::null_mut());
    let pdevinfo = platform_device_info { name: DEVICE_NAME, id: PLATFORM_DEVID_NONE, fwnode, swnode: &platform_device_test_swnode, ..Default::default() };
    let mut pdev = platform_device_register_full(&pdevinfo);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    wait_for_device_probe();
    let mut bound = device_is_bound(&mut (*pdev).dev);
    KUNIT_ASSERT_TRUE(test, bound);
    platform_device_unregister(pdev);
    pdev = platform_device_register_full(&pdevinfo);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    wait_for_device_probe();
    bound = device_is_bound(&mut (*pdev).dev);
    KUNIT_ASSERT_TRUE(test, bound);
    platform_device_unregister(pdev);
}

unsafe fn platform_device_swnode_as_primary(test: *mut kunit) {
    let mut ret = kunit_platform_driver_register(test, &mut platform_swnode_test_driver);
    KUNIT_ASSERT_EQ(test, ret, 0);
    let fwnode = kunit_software_node_register(test, &platform_device_test_swnode);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, fwnode);
    let pdevinfo = platform_device_info { name: DEVICE_NAME, id: PLATFORM_DEVID_NONE, fwnode, ..Default::default() };
    let pdev = platform_device_register_full(&pdevinfo);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, pdev);
    wait_for_device_probe();
    KUNIT_ASSERT_TRUE(test, device_is_bound(&mut (*pdev).dev));
    platform_device_unregister(pdev);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, software_node_fwnode(&platform_device_test_swnode));
}

unsafe fn platform_device_two_swnodes(test: *mut kunit) {
    static properties: [property_entry; 2] = [PROPERTY_ENTRY_U32!(c"foo", 42), property_entry { }];
    let mut ret = kunit_platform_driver_register(test, &mut platform_swnode_test_driver);
    KUNIT_ASSERT_EQ(test, ret, 0);
    let fwnode = kunit_software_node_register(test, &platform_device_test_swnode);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, fwnode);
    let mut pdevinfo = platform_device_info { name: DEVICE_NAME, id: PLATFORM_DEVID_NONE, fwnode, swnode: &platform_device_test_swnode, ..Default::default() };
    let pdev = platform_device_register_full(&pdevinfo);
    KUNIT_ASSERT_TRUE(test, IS_ERR(pdev));
    KUNIT_ASSERT_EQ_MSG(test, PTR_ERR(pdev), -EINVAL, c"Expected errno == -EINVAL, got: %pe", pdev);
    pdevinfo = platform_device_info { name: DEVICE_NAME, id: PLATFORM_DEVID_NONE, swnode: &platform_device_test_swnode, properties: properties.as_ptr(), ..Default::default() };
    let pdev = platform_device_register_full(&pdevinfo);
    KUNIT_ASSERT_TRUE(test, IS_ERR(pdev));
    KUNIT_ASSERT_EQ_MSG(test, PTR_ERR(pdev), -EINVAL, c"Expected errno == -EINVAL, got: %pe", pdev);
    pdevinfo = platform_device_info { name: DEVICE_NAME, id: PLATFORM_DEVID_NONE, fwnode, properties: properties.as_ptr(), ..Default::default() };
    let pdev = platform_device_register_full(&pdevinfo);
    KUNIT_ASSERT_TRUE(test, IS_ERR(pdev));
    KUNIT_ASSERT_EQ_MSG(test, PTR_ERR(pdev), -EINVAL, c"Expected errno == -EINVAL, got: %pe", pdev);
}

static mut platform_device_swnode_tests: [kunit_case; 4] = [KUNIT_CASE!(platform_device_swnode_add_twice), KUNIT_CASE!(platform_device_swnode_as_primary), KUNIT_CASE!(platform_device_two_swnodes), KUNIT_CASE!()];
static mut platform_device_swnode_test_suite: kunit_suite = kunit_suite { name: c"platform-device-swnode".as_ptr(), test_cases: platform_device_swnode_tests.as_mut_ptr(), ..Default::default() };

kunit_test_suites!(&mut platform_device_devm_test_suite, &mut platform_device_match_test_suite, &mut platform_device_swnode_test_suite);

MODULE_DESCRIPTION!(c"Test module for platform devices");
MODULE_AUTHOR!(c"Maxime Ripard <mripard@kernel.org>");
MODULE_LICENSE!(c"GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
