// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) Qualcomm Technologies, Inc. and/or its subsidiaries
 */

// The Linux, KUnit, and platform-device headers supplying the types, macros,
// constants, and functions referenced below are external dependencies.

unsafe fn swnode_count_suppliers(fwnode: *mut fwnode_handle) -> i32 {
    let mut count: u32 = 0;
    // list_for_each_entry(link, &fwnode->suppliers, c_hook)
    unsafe {
        let mut link: *mut fwnode_link = core::ptr::null_mut();
        while list_for_each_entry_next(&mut link, fwnode, 0) {
            count = count.wrapping_add(1);
        }
    }
    count as i32
}

/* True if a supplier link con->sup exists, checked from both list ends. */
unsafe fn swnode_has_link(
    consumer: *mut fwnode_handle,
    supplier: *mut fwnode_handle,
) -> bool {
    let mut from_con = false;
    let mut from_sup = false;
    let mut link: *mut fwnode_link = core::ptr::null_mut();

    // list_for_each_entry(link, &consumer->suppliers, c_hook)
    while list_for_each_entry_next(&mut link, consumer, 0) {
        if (*link).supplier == supplier && (*link).consumer == consumer {
            from_con = true;
        }
    }
    // list_for_each_entry(link, &supplier->consumers, s_hook)
    while list_for_each_entry_next(&mut link, supplier, 1) {
        if (*link).supplier == supplier && (*link).consumer == consumer {
            from_sup = true;
        }
    }
    from_con && from_sup
}

unsafe fn swnode_devlink_test_single_ref(test: *mut kunit) {
    static mut SUPP_SWNODE: software_node = software_node { name: "swnode-devlink-test-supplier\0".as_ptr() as *const i8 };
    let mut props = [PROPERTY_ENTRY_REF!("supplier", &raw const SUPP_SWNODE), PROPERTY_ENTRY_END!()];
    let supp_fwnode = kunit_software_node_register(test, &raw const SUPP_SWNODE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, supp_fwnode);
    let cons_fwnode = kunit_fwnode_create_software_node(test, props.as_mut_ptr(), core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, cons_fwnode);
    let ret = fwnode_call_int_op(cons_fwnode, add_links);
    KUNIT_EXPECT_EQ!(test, ret, 0);
    KUNIT_EXPECT_EQ!(test, swnode_count_suppliers(cons_fwnode), 1);
    KUNIT_EXPECT_TRUE!(test, swnode_has_link(cons_fwnode, supp_fwnode));
}

unsafe fn swnode_devlink_test_multiple_refs(test: *mut kunit) {
    static mut SUPP1_SWNODE: software_node = software_node { name: "swnode-devlink-test-supplier-1\0".as_ptr() as *const i8 };
    static mut SUPP2_SWNODE: software_node = software_node { name: "swnode-devlink-test-supplier-2\0".as_ptr() as *const i8 };
    let mut supp_nodes = [(&raw const SUPP1_SWNODE), (&raw const SUPP2_SWNODE), core::ptr::null()];
    let mut props = [PROPERTY_ENTRY_REF!("foo", &raw const SUPP1_SWNODE), PROPERTY_ENTRY_REF!("bar", &raw const SUPP2_SWNODE), PROPERTY_ENTRY_END!()];
    let ret = kunit_software_node_register_node_group(test, supp_nodes.as_mut_ptr());
    KUNIT_ASSERT_EQ!(test, ret, 0);
    let fwnode = kunit_fwnode_create_software_node(test, props.as_mut_ptr(), core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, fwnode);
    let ret = fwnode_call_int_op(fwnode, add_links);
    KUNIT_EXPECT_EQ!(test, ret, 0);
    KUNIT_EXPECT_EQ!(test, swnode_count_suppliers(fwnode), 2);
    KUNIT_EXPECT_TRUE!(test, swnode_has_link(fwnode, software_node_fwnode(&raw const SUPP1_SWNODE)));
    KUNIT_EXPECT_TRUE!(test, swnode_has_link(fwnode, software_node_fwnode(&raw const SUPP2_SWNODE)));
}

unsafe fn swnode_devlink_test_unregistered_ref(test: *mut kunit) {
    static mut SUPP_SWNODE: software_node = software_node { name: "swnode-devlink-test-supplier\0".as_ptr() as *const i8 };
    let mut props = [PROPERTY_ENTRY_REF!("supplier", &raw const SUPP_SWNODE), PROPERTY_ENTRY_END!()];
    let fwnode = kunit_fwnode_create_software_node(test, props.as_mut_ptr(), core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, fwnode);
    let ret = fwnode_call_int_op(fwnode, add_links);
    KUNIT_EXPECT_EQ!(test, ret, 0);
    KUNIT_EXPECT_EQ!(test, swnode_count_suppliers(fwnode), 0);
}

/* Graph "remote-endpoint" references are excluded. */
unsafe fn swnode_devlink_test_remote_endpoint_excluded(test: *mut kunit) {
    static mut EP_SWNODE: software_node = software_node { name: "swnode-devlink-test-end-point\0".as_ptr() as *const i8 };
    let mut props = [PROPERTY_ENTRY_REF!("remote-endpoint", &raw const EP_SWNODE), PROPERTY_ENTRY_END!()];
    let supp_fwnode = kunit_software_node_register(test, &raw const EP_SWNODE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, supp_fwnode);
    let cons_fwnode = kunit_fwnode_create_software_node(test, props.as_mut_ptr(), core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, cons_fwnode);
    let ret = fwnode_call_int_op(cons_fwnode, add_links);
    KUNIT_EXPECT_EQ!(test, ret, 0);
    KUNIT_EXPECT_EQ!(test, swnode_count_suppliers(cons_fwnode), 0);
}

unsafe fn swnode_devlink_test_ref_array(test: *mut kunit) {
    static mut SUPP1_SWNODE: software_node = software_node { name: "swnode-devlink-test-supplier-1\0".as_ptr() as *const i8 };
    static mut SUPP2_SWNODE: software_node = software_node { name: "swnode-devlink-test-supplier-2\0".as_ptr() as *const i8 };
    let mut supp_nodes = [(&raw const SUPP1_SWNODE), (&raw const SUPP2_SWNODE), core::ptr::null()];
    let refs = [SOFTWARE_NODE_REFERENCE!(&raw const SUPP1_SWNODE), SOFTWARE_NODE_REFERENCE!(&raw const SUPP2_SWNODE, 4, 2)];
    let mut props = [PROPERTY_ENTRY_REF_ARRAY!("suppliers", refs.as_ptr()), PROPERTY_ENTRY_END!()];
    let ret = kunit_software_node_register_node_group(test, supp_nodes.as_mut_ptr());
    KUNIT_ASSERT_EQ!(test, ret, 0);
    let fwnode = kunit_fwnode_create_software_node(test, props.as_mut_ptr(), core::ptr::null_mut());
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, fwnode);
    let ret = fwnode_call_int_op(fwnode, add_links);
    KUNIT_EXPECT_EQ!(test, ret, 0);
    KUNIT_EXPECT_EQ!(test, swnode_count_suppliers(fwnode), 2);
    KUNIT_EXPECT_TRUE!(test, swnode_has_link(fwnode, software_node_fwnode(&raw const SUPP1_SWNODE)));
    KUNIT_EXPECT_TRUE!(test, swnode_has_link(fwnode, software_node_fwnode(&raw const SUPP2_SWNODE)));
}

// The remaining end-to-end KUnit test is retained as a direct low-level
// translation; its platform/KUnit structures and macros are external.
const SWNODE_DEVLINK_TEST_SUPPLIER: &str = "swnode-link-supplier";
const SWNODE_DEVLINK_TEST_CONSUMER: &str = "swnode-link-consumer";
const SWNODE_DEVLINK_TEST_TIMEOUT_MS: u64 = 2 * MSEC_PER_SEC as u64;

#[repr(C)]
struct swnode_test_probe_order {
    probed: [*const i8; 2],
    count: u32,
    wq: wait_queue_head_t,
}

unsafe fn swnode_test_record_probe(pdev: *mut platform_device) -> i32 {
    let order = platform_get_drvdata(pdev) as *mut swnode_test_probe_order;
    if !order.is_null() && (*order).count < 2 {
        (*order).probed[(*order).count as usize] = dev_name(&mut (*pdev).dev);
        (*order).count += 1;
        wake_up_interruptible(&mut (*order).wq);
    }
    0
}

// C's static platform_driver initializers and kunit_test_suite registration
// are represented by the corresponding external Rust macros/definitions.
static mut SWNODE_TEST_SUPPLIER_DRIVER: platform_driver = platform_driver::new(swnode_test_record_probe, SWNODE_DEVLINK_TEST_SUPPLIER);
static mut SWNODE_TEST_CONSUMER_DRIVER: platform_driver = platform_driver::new(swnode_test_record_probe, SWNODE_DEVLINK_TEST_CONSUMER);

// The probe-order test body follows the C implementation and depends on the
// external KUnit/platform APIs described above.
unsafe fn swnode_devlink_test_probe_order(test: *mut kunit) {
    static mut SUPPLIER_SWNODE: software_node = software_node::named("swnode-devlink-test-supplier");
    let mut consumer_props = [PROPERTY_ENTRY_REF!("supplier-ref", &raw const SUPPLIER_SWNODE), PROPERTY_ENTRY_END!()];
    let order = kunit_kzalloc(test, core::mem::size_of::<swnode_test_probe_order>(), GFP_KERNEL) as *mut swnode_test_probe_order;
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, order);
    init_waitqueue_head(&mut (*order).wq);
    let fwnode = kunit_software_node_register(test, &raw const SUPPLIER_SWNODE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, fwnode);
    let mut ret = kunit_platform_driver_register(test, &raw mut SWNODE_TEST_SUPPLIER_DRIVER);
    KUNIT_ASSERT_EQ!(test, ret, 0);
    ret = kunit_platform_driver_register(test, &raw mut SWNODE_TEST_CONSUMER_DRIVER);
    KUNIT_ASSERT_EQ!(test, ret, 0);
    let supplier = kunit_platform_device_alloc(test, SWNODE_DEVLINK_TEST_SUPPLIER.as_ptr() as *const i8, PLATFORM_DEVID_NONE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, supplier);
    let consumer = kunit_platform_device_alloc(test, SWNODE_DEVLINK_TEST_CONSUMER.as_ptr() as *const i8, PLATFORM_DEVID_NONE);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, consumer);
    platform_set_drvdata(supplier, order as *mut core::ffi::c_void);
    platform_set_drvdata(consumer, order as *mut core::ffi::c_void);
    ret = kunit_device_add_software_node(test, &mut (*supplier).dev, &raw const SUPPLIER_SWNODE);
    KUNIT_ASSERT_EQ!(test, ret, 0);
    ret = device_create_managed_software_node(&mut (*consumer).dev, consumer_props.as_mut_ptr(), core::ptr::null_mut());
    KUNIT_ASSERT_EQ!(test, ret, 0);
    ret = kunit_platform_device_add(test, consumer);
    KUNIT_ASSERT_EQ!(test, ret, 0);
    ret = kunit_platform_device_add(test, supplier);
    KUNIT_ASSERT_EQ!(test, ret, 0);
    ret = wait_event_interruptible_timeout(&mut (*order).wq, (*order).count == 2, msecs_to_jiffies(SWNODE_DEVLINK_TEST_TIMEOUT_MS));
    KUNIT_ASSERT_GT!(test, ret, 0);
    KUNIT_EXPECT_STREQ!(test, (*order).probed[0], SWNODE_DEVLINK_TEST_SUPPLIER.as_ptr());
    KUNIT_EXPECT_STREQ!(test, (*order).probed[1], SWNODE_DEVLINK_TEST_CONSUMER.as_ptr());
    kunit_platform_device_unregister(test, consumer);
}

extern "C" {
    fn list_for_each_entry_next(link: *mut *mut fwnode_link, node: *mut fwnode_handle, hook: i32) -> bool;
}

#[allow(non_camel_case_types)] pub enum fwnode_handle {}
#[allow(non_camel_case_types)] pub enum fwnode_link {}
#[allow(non_camel_case_types)] pub enum kunit {}
#[allow(non_camel_case_types)] pub enum platform_device {}
#[allow(non_camel_case_types)] pub enum software_node {}
#[allow(non_camel_case_types)] pub enum platform_driver {}
#[allow(non_camel_case_types)] pub enum wait_queue_head_t {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
