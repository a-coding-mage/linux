// SPDX-License-Identifier: GPL-2.0-only
//
// uapi_test.c - An application of Kunit to check layout of structures exposed to user space for
//              FireWire subsystem.
//
// Copyright (c) 2023 Takashi Sakamoto

// Translated dependencies supplied by the kernel/KUnit environment.

// Known issue added at v2.6.27 kernel.
unsafe fn structure_layout_event_response(test: *mut kunit) {
    #[cfg(target_arch = "x86")]
    {
        // 4 bytes alignment for aggregate type including 8 bytes storage types.
        KUNIT_EXPECT_EQ!(test, 20, core::mem::size_of::<fw_cdev_event_response>());
    }
    #[cfg(not(target_arch = "x86"))]
    {
        // 8 bytes alignment for aggregate type including 8 bytes storage types.
        KUNIT_EXPECT_EQ!(test, 24, core::mem::size_of::<fw_cdev_event_response>());
    }

    KUNIT_EXPECT_EQ!(test, 0, core::mem::offset_of!(fw_cdev_event_response, closure));
    KUNIT_EXPECT_EQ!(test, 8, core::mem::offset_of!(fw_cdev_event_response, type_));
    KUNIT_EXPECT_EQ!(test, 12, core::mem::offset_of!(fw_cdev_event_response, rcode));
    KUNIT_EXPECT_EQ!(test, 16, core::mem::offset_of!(fw_cdev_event_response, length));
    KUNIT_EXPECT_EQ!(test, 20, core::mem::offset_of!(fw_cdev_event_response, data));
}

// Added at v6.5.
unsafe fn structure_layout_event_request3(test: *mut kunit) {
    KUNIT_EXPECT_EQ!(test, 56, core::mem::size_of::<fw_cdev_event_request3>());

    KUNIT_EXPECT_EQ!(test, 0, core::mem::offset_of!(fw_cdev_event_request3, closure));
    KUNIT_EXPECT_EQ!(test, 8, core::mem::offset_of!(fw_cdev_event_request3, type_));
    KUNIT_EXPECT_EQ!(test, 12, core::mem::offset_of!(fw_cdev_event_request3, tcode));
    KUNIT_EXPECT_EQ!(test, 16, core::mem::offset_of!(fw_cdev_event_request3, offset));
    KUNIT_EXPECT_EQ!(test, 24, core::mem::offset_of!(fw_cdev_event_request3, source_node_id));
    KUNIT_EXPECT_EQ!(test, 28, core::mem::offset_of!(fw_cdev_event_request3, destination_node_id));
    KUNIT_EXPECT_EQ!(test, 32, core::mem::offset_of!(fw_cdev_event_request3, card));
    KUNIT_EXPECT_EQ!(test, 36, core::mem::offset_of!(fw_cdev_event_request3, generation));
    KUNIT_EXPECT_EQ!(test, 40, core::mem::offset_of!(fw_cdev_event_request3, handle));
    KUNIT_EXPECT_EQ!(test, 44, core::mem::offset_of!(fw_cdev_event_request3, length));
    KUNIT_EXPECT_EQ!(test, 48, core::mem::offset_of!(fw_cdev_event_request3, tstamp));
    KUNIT_EXPECT_EQ!(test, 56, core::mem::offset_of!(fw_cdev_event_request3, data));
}

// Added at v6.5.
unsafe fn structure_layout_event_response2(test: *mut kunit) {
    KUNIT_EXPECT_EQ!(test, 32, core::mem::size_of::<fw_cdev_event_response2>());

    KUNIT_EXPECT_EQ!(test, 0, core::mem::offset_of!(fw_cdev_event_response2, closure));
    KUNIT_EXPECT_EQ!(test, 8, core::mem::offset_of!(fw_cdev_event_response2, type_));
    KUNIT_EXPECT_EQ!(test, 12, core::mem::offset_of!(fw_cdev_event_response2, rcode));
    KUNIT_EXPECT_EQ!(test, 16, core::mem::offset_of!(fw_cdev_event_response2, length));
    KUNIT_EXPECT_EQ!(test, 20, core::mem::offset_of!(fw_cdev_event_response2, request_tstamp));
    KUNIT_EXPECT_EQ!(test, 24, core::mem::offset_of!(fw_cdev_event_response2, response_tstamp));
    KUNIT_EXPECT_EQ!(test, 32, core::mem::offset_of!(fw_cdev_event_response2, data));
}

// Added at v6.5.
unsafe fn structure_layout_event_phy_packet2(test: *mut kunit) {
    KUNIT_EXPECT_EQ!(test, 24, core::mem::size_of::<fw_cdev_event_phy_packet2>());

    KUNIT_EXPECT_EQ!(test, 0, core::mem::offset_of!(fw_cdev_event_phy_packet2, closure));
    KUNIT_EXPECT_EQ!(test, 8, core::mem::offset_of!(fw_cdev_event_phy_packet2, type_));
    KUNIT_EXPECT_EQ!(test, 12, core::mem::offset_of!(fw_cdev_event_phy_packet2, rcode));
    KUNIT_EXPECT_EQ!(test, 16, core::mem::offset_of!(fw_cdev_event_phy_packet2, length));
    KUNIT_EXPECT_EQ!(test, 20, core::mem::offset_of!(fw_cdev_event_phy_packet2, tstamp));
    KUNIT_EXPECT_EQ!(test, 24, core::mem::offset_of!(fw_cdev_event_phy_packet2, data));
}

static mut structure_layout_test_cases: [kunit_case; 5] = [
    KUNIT_CASE!(structure_layout_event_response),
    KUNIT_CASE!(structure_layout_event_request3),
    KUNIT_CASE!(structure_layout_event_response2),
    KUNIT_CASE!(structure_layout_event_phy_packet2),
    kunit_case::default(),
];

static mut structure_layout_test_suite: kunit_suite = kunit_suite {
    name: "firewire-uapi-structure-layout",
    test_cases: structure_layout_test_cases.as_ptr(),
};

kunit_test_suite!(structure_layout_test_suite);

module_description!("FireWire UAPI unit test suite");
module_license!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
