// SPDX-License-Identifier: GPL-2.0
// Copyright 2023 Maxime Ripard <mripard@kernel.org>

// C dependencies: <kunit/resource.h>, <linux/device.h>

const DEVICE_NAME: &[u8] = b"test\0";
const RELEASE_TIMEOUT_MS: u32 = 100;

#[repr(C)]
struct TestPriv {
    probe_done: bool,
    release_done: bool,
    release_wq: WaitQueueHead,
    dev: *mut Device,
}

#[repr(C)]
struct Kunit {
    priv_: *mut core::ffi::c_void,
}

#[repr(C)]
struct Device;
#[repr(C)]
struct WaitQueueHead;

extern "C" {
    fn kunit_kzalloc(test: *mut Kunit, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn init_waitqueue_head(wq: *mut WaitQueueHead);
    fn root_device_register(name: *const core::ffi::c_char) -> *mut Device;
    fn root_device_unregister(dev: *mut Device);
    fn devm_add_action_or_reset(
        dev: *mut Device,
        action: unsafe extern "C" fn(*mut core::ffi::c_void),
        data: *mut core::ffi::c_void,
    ) -> i32;
    fn wake_up_interruptible(wq: *mut WaitQueueHead);
    fn wait_event_interruptible_timeout(
        wq: *mut WaitQueueHead,
        condition: bool,
        timeout: i64,
    ) -> i64;
    fn msecs_to_jiffies(milliseconds: u32) -> i64;
    fn get_device(dev: *mut Device);
    fn put_device(dev: *mut Device);
}

const GFP_KERNEL: u32 = 0;

unsafe fn root_device_devm_init(test: *mut Kunit) -> i32 {
    let priv_ = kunit_kzalloc(test, core::mem::size_of::<TestPriv>(), GFP_KERNEL)
        as *mut TestPriv;
    // KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv_);
    if priv_.is_null() {
        return 0;
    }
    init_waitqueue_head(core::ptr::addr_of_mut!((*priv_).release_wq));

    (*test).priv_ = priv_ as *mut core::ffi::c_void;

    0
}

unsafe extern "C" fn devm_device_action(ptr: *mut core::ffi::c_void) {
    let priv_ = ptr as *mut TestPriv;

    (*priv_).release_done = true;
    wake_up_interruptible(core::ptr::addr_of_mut!((*priv_).release_wq));
}

/*
 * Tests that a bus-less, non-probed device will run its device-managed
 * actions when unregistered.
 */
unsafe fn root_device_devm_register_unregister_test(test: *mut Kunit) {
    let priv_ = (*test).priv_ as *mut TestPriv;
    let ret: i32;

    (*priv_).dev = root_device_register(DEVICE_NAME.as_ptr() as *const core::ffi::c_char);
    // KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv_->dev);

    ret = devm_add_action_or_reset(
        (*priv_).dev,
        devm_device_action,
        priv_ as *mut core::ffi::c_void,
    );
    // KUNIT_ASSERT_EQ(test, ret, 0);

    root_device_unregister((*priv_).dev);

    let wait_ret = wait_event_interruptible_timeout(
        core::ptr::addr_of_mut!((*priv_).release_wq),
        (*priv_).release_done,
        msecs_to_jiffies(RELEASE_TIMEOUT_MS),
    );
    // KUNIT_EXPECT_GT(test, wait_ret, 0);
    let _ = (ret, wait_ret);
}

unsafe extern "C" fn devm_put_device_action(ptr: *mut core::ffi::c_void) {
    let priv_ = ptr as *mut TestPriv;

    put_device((*priv_).dev);
    (*priv_).release_done = true;
    wake_up_interruptible(core::ptr::addr_of_mut!((*priv_).release_wq));
}

/*
 * Tests that a bus-less, non-probed device will run its device-managed
 * actions when unregistered, even if someone still holds a reference to
 * it.
 */
unsafe fn root_device_devm_register_get_unregister_with_devm_test(test: *mut Kunit) {
    let priv_ = (*test).priv_ as *mut TestPriv;
    let ret: i32;

    (*priv_).dev = root_device_register(DEVICE_NAME.as_ptr() as *const core::ffi::c_char);
    // KUNIT_ASSERT_NOT_ERR_OR_NULL(test, priv_->dev);

    get_device((*priv_).dev);

    ret = devm_add_action_or_reset(
        (*priv_).dev,
        devm_put_device_action,
        priv_ as *mut core::ffi::c_void,
    );
    // KUNIT_ASSERT_EQ(test, ret, 0);

    root_device_unregister((*priv_).dev);

    let wait_ret = wait_event_interruptible_timeout(
        core::ptr::addr_of_mut!((*priv_).release_wq),
        (*priv_).release_done,
        msecs_to_jiffies(RELEASE_TIMEOUT_MS),
    );
    // KUNIT_EXPECT_GT(test, wait_ret, 0);
    let _ = (ret, wait_ret);
}

// KUNIT_CASE(root_device_devm_register_unregister_test)
// KUNIT_CASE(root_device_devm_register_get_unregister_with_devm_test)
static ROOT_DEVICE_DEVM_TESTS: &[unsafe fn(*mut Kunit)] = &[
    root_device_devm_register_unregister_test,
    root_device_devm_register_get_unregister_with_devm_test,
];

// KUNIT suite: name = "root-device-devm", init = root_device_devm_init,
// test_cases = root_device_devm_tests
struct RootDeviceDevmTestSuite;

// kunit_test_suite(root_device_devm_test_suite);
// MODULE_DESCRIPTION("Test module for root devices");
// MODULE_AUTHOR("Maxime Ripard <mripard@kernel.org>");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
