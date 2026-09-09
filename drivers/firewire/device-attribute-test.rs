// SPDX-License-Identifier: GPL-2.0-only
//
// device-attribute-test.c - An application of Kunit to test implementation for device attributes.
//
// Copyright (c) 2023 Takashi Sakamoto
//
// This file can not be built independently since it is intentionally included in core-device.c.

// Configuration ROM for AV/C Devices 1.0 (Dec. 12, 2000, 1394 Trading Association)
// Annex C:Configuration ROM example(informative)
// C.1 Simple AV/C device
//
// Copied from the documentation.
static SIMPLE_AVC_CONFIG_ROM: [u32; 27] = [
	0x0404eabf, 0x31333934, 0xe0646102, 0xffffffff, 0xffffffff,
	0x00063287, // root directory.
	0x03ffffff, 0x8100000a, 0x17ffffff, 0x8100000e, 0x0c0083c0,
	0xd1000001,
	0x0004442d, // unit 0 directory.
	0x1200a02d, 0x13010001, 0x17ffffff, 0x81000007,
	0x0005c915, // leaf for textual descriptor.
	0x00000000, 0x00000000, 0x56656e64, 0x6f72204e, 0x616d6500,
	0x00057f16, // leaf for textual descriptor.
	0x00000000, 0x00000000, 0x4d6f6465, 0x6c204e61, 0x6d650000,
];

// Ibid.
// Annex A:Consideration for configuration ROM reader design (informative)
// A.1 Vendor directory
//
// Written by hand.
static LEGACY_AVC_CONFIG_ROM: [u32; 25] = [
	0x04199fe7, 0x31333934, 0xe0644000, 0x00112233, 0x44556677,
	0x0005dace, // root directory.
	0x03012345, 0x0c0083c0, 0x8d000009, 0xd1000002, 0xc3000004,
	0x0002e107, // unit 0 directory.
	0x12abcdef, 0x13543210,
	0x0002cb73, // vendor directory.
	0x17fedcba, 0x81000004,
	0x00026dc1, // leaf for EUI-64.
	0x00112233, 0x44556677,
	0x00050e84, // leaf for textual descriptor.
	0x00000000, 0x00000000, 0x41424344, 0x45464748, 0x494a0000,
];

unsafe fn device_attr_simple_avc(test: *mut kunit) {
	static NODE: fw_device = fw_device {
		device: device { type_: &fw_device_type, parent: core::ptr::null_mut() },
		config_rom: SIMPLE_AVC_CONFIG_ROM.as_ptr(),
		config_rom_length: core::mem::size_of_val(&SIMPLE_AVC_CONFIG_ROM),
	};
	static UNIT0: fw_unit = fw_unit {
		device: device { type_: &fw_unit_type, parent: &NODE.device as *const device as *mut device },
		directory: unsafe { SIMPLE_AVC_CONFIG_ROM.as_ptr().add(12) },
	};
	let node_dev = &NODE.device as *const device as *mut device;
	let unit0_dev = &UNIT0.device as *const device as *mut device;
	static UNIT0_EXPECTED_IDS: [i32; 4] = [0x00ffffff, 0x00ffffff, 0x0000a02d, 0x00010001];
	let buf = kunit_kzalloc(test, PAGE_SIZE, GFP_KERNEL);
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, buf);
	let mut ids = [0i32; 4];

	// Ensure associations for node and unit devices.
	KUNIT_ASSERT_TRUE(test, is_fw_device(node_dev));
	KUNIT_ASSERT_FALSE(test, is_fw_unit(node_dev));
	KUNIT_ASSERT_PTR_EQ(test, fw_device(node_dev), &NODE);
	KUNIT_ASSERT_FALSE(test, is_fw_device(unit0_dev));
	KUNIT_ASSERT_TRUE(test, is_fw_unit(unit0_dev));
	KUNIT_ASSERT_PTR_EQ(test, fw_parent_device(&UNIT0), &NODE);
	KUNIT_ASSERT_PTR_EQ(test, fw_unit(unit0_dev), &UNIT0);

	// For entries in root directory.
	KUNIT_EXPECT_GT(test, show_immediate(node_dev, &config_rom_attributes[0].attr, buf), 0);
	KUNIT_EXPECT_STREQ(test, buf, "0xffffff\n");
	KUNIT_EXPECT_GT(test, show_immediate(node_dev, &config_rom_attributes[4].attr, buf), 0);
	KUNIT_EXPECT_STREQ(test, buf, "0xffffff\n");
	KUNIT_EXPECT_GT(test, show_text_leaf(node_dev, &config_rom_attributes[5].attr, buf), 0);
	KUNIT_EXPECT_STREQ(test, buf, "Vendor Name\n");
	KUNIT_EXPECT_GT(test, show_text_leaf(node_dev, &config_rom_attributes[6].attr, buf), 0);
	KUNIT_EXPECT_STREQ(test, buf, "Model Name\n");

	// For entries in unit 0 directory.
	KUNIT_EXPECT_LT(test, show_immediate(unit0_dev, &config_rom_attributes[0].attr, buf), 0);
	KUNIT_EXPECT_GT(test, show_immediate(unit0_dev, &config_rom_attributes[4].attr, buf), 0);
	KUNIT_EXPECT_STREQ(test, buf, "0xffffff\n");
	KUNIT_EXPECT_LT(test, show_text_leaf(unit0_dev, &config_rom_attributes[5].attr, buf), 0);
	KUNIT_EXPECT_GT(test, show_text_leaf(unit0_dev, &config_rom_attributes[6].attr, buf), 0);
	KUNIT_EXPECT_STREQ(test, buf, "Model Name\n");
	KUNIT_EXPECT_GT(test, show_immediate(unit0_dev, &config_rom_attributes[2].attr, buf), 0);
	KUNIT_EXPECT_STREQ(test, buf, "0x00a02d\n");
	KUNIT_EXPECT_GT(test, show_immediate(unit0_dev, &config_rom_attributes[3].attr, buf), 0);
	KUNIT_EXPECT_STREQ(test, buf, "0x010001\n");
	kunit_kfree(test, buf);
	get_modalias_ids(&UNIT0, ids.as_mut_ptr());
	KUNIT_EXPECT_MEMEQ(test, ids.as_ptr(), UNIT0_EXPECTED_IDS.as_ptr(), core::mem::size_of_val(&ids));
}

unsafe fn device_attr_legacy_avc(test: *mut kunit) {
	static NODE: fw_device = fw_device {
		device: device { type_: &fw_device_type, parent: core::ptr::null_mut() },
		config_rom: LEGACY_AVC_CONFIG_ROM.as_ptr(),
		config_rom_length: core::mem::size_of_val(&LEGACY_AVC_CONFIG_ROM),
	};
	static UNIT0: fw_unit = fw_unit {
		device: device { type_: &fw_unit_type, parent: &NODE.device as *const device as *mut device },
		directory: unsafe { LEGACY_AVC_CONFIG_ROM.as_ptr().add(11) },
	};
	let node_dev = &NODE.device as *const device as *mut device;
	let unit0_dev = &UNIT0.device as *const device as *mut device;
	static UNIT0_EXPECTED_IDS: [i32; 4] = [0x00012345, 0x00fedcba, 0x00abcdef, 0x00543210];
	let buf = kunit_kzalloc(test, PAGE_SIZE, GFP_KERNEL);
	KUNIT_ASSERT_NOT_ERR_OR_NULL(test, buf);
	let mut ids = [0i32; 4];

	// Ensure associations for node and unit devices.
	KUNIT_ASSERT_TRUE(test, is_fw_device(node_dev));
	KUNIT_ASSERT_FALSE(test, is_fw_unit(node_dev));
	KUNIT_ASSERT_PTR_EQ(test, fw_device(node_dev), &NODE);
	KUNIT_ASSERT_FALSE(test, is_fw_device(unit0_dev));
	KUNIT_ASSERT_TRUE(test, is_fw_unit(unit0_dev));
	KUNIT_ASSERT_PTR_EQ(test, fw_parent_device(&UNIT0), &NODE);
	KUNIT_ASSERT_PTR_EQ(test, fw_unit(unit0_dev), &UNIT0);

	// For entries in root directory.
	KUNIT_EXPECT_GT(test, show_immediate(node_dev, &config_rom_attributes[0].attr, buf), 0);
	KUNIT_EXPECT_STREQ(test, buf, "0x012345\n");
	KUNIT_EXPECT_GT(test, show_immediate(node_dev, &config_rom_attributes[4].attr, buf), 0);
	KUNIT_EXPECT_STREQ(test, buf, "0xfedcba\n");
	KUNIT_EXPECT_LT(test, show_text_leaf(node_dev, &config_rom_attributes[5].attr, buf), 0);
	KUNIT_EXPECT_GT(test, show_text_leaf(node_dev, &config_rom_attributes[6].attr, buf), 0);
	KUNIT_EXPECT_STREQ(test, buf, "ABCDEFGHIJ\n");

	// For entries in unit 0 directory.
	KUNIT_EXPECT_LT(test, show_immediate(unit0_dev, &config_rom_attributes[0].attr, buf), 0);
	KUNIT_EXPECT_LT(test, show_immediate(unit0_dev, &config_rom_attributes[4].attr, buf), 0);
	KUNIT_EXPECT_LT(test, show_text_leaf(unit0_dev, &config_rom_attributes[5].attr, buf), 0);
	KUNIT_EXPECT_LT(test, show_text_leaf(unit0_dev, &config_rom_attributes[6].attr, buf), 0);
	KUNIT_EXPECT_GT(test, show_immediate(unit0_dev, &config_rom_attributes[2].attr, buf), 0);
	KUNIT_EXPECT_STREQ(test, buf, "0xabcdef\n");
	KUNIT_EXPECT_GT(test, show_immediate(unit0_dev, &config_rom_attributes[3].attr, buf), 0);
	KUNIT_EXPECT_STREQ(test, buf, "0x543210\n");
	kunit_kfree(test, buf);
	get_modalias_ids(&UNIT0, ids.as_mut_ptr());
	KUNIT_EXPECT_MEMEQ(test, ids.as_ptr(), UNIT0_EXPECTED_IDS.as_ptr(), core::mem::size_of_val(&ids));
}

static DEVICE_ATTR_TEST_CASES: &[unsafe fn(*mut kunit)] = &[device_attr_simple_avc, device_attr_legacy_avc];

static DEVICE_ATTR_TEST_SUITE: kunit_suite = kunit_suite {
	name: "firewire-device-attribute",
	test_cases: DEVICE_ATTR_TEST_CASES.as_ptr(),
};

// Equivalent of kunit_test_suite(device_attr_test_suite).
kunit_test_suite!(DEVICE_ATTR_TEST_SUITE);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
