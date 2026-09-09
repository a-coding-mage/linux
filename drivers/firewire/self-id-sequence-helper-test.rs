// SPDX-License-Identifier: GPL-2.0-or-later
//
// self-id-sequence-helper-test.c - An application of Kunit to test helpers of self ID sequence.
//
// Copyright (c) 2024 Takashi Sakamoto

// External dependencies supplied by the KUnit and PHY packet definition components.

unsafe fn test_self_id_sequence_enumerator_valid(test: *mut kunit) {
    static VALID_SEQUENCES: [u32; 7] = [
        0x00000000,
        0x00000001, 0x00800000,
        0x00000001, 0x00800001, 0x00900000,
        0x00000000,
    ];
    let mut enumerator: self_id_sequence_enumerator = core::mem::zeroed();
    let mut entry: *const u32;
    let mut quadlet_count: u32 = 0;

    enumerator.cursor = VALID_SEQUENCES.as_ptr();
    enumerator.quadlet_count = VALID_SEQUENCES.len() as _;

    entry = self_id_sequence_enumerator_next(&mut enumerator, &mut quadlet_count);
    KUNIT_EXPECT_PTR_EQ(test, entry, VALID_SEQUENCES.as_ptr());
    KUNIT_EXPECT_EQ(test, quadlet_count, 1);
    KUNIT_EXPECT_EQ(test, enumerator.quadlet_count, 6);

    entry = self_id_sequence_enumerator_next(&mut enumerator, &mut quadlet_count);
    KUNIT_EXPECT_PTR_EQ(test, entry, VALID_SEQUENCES.as_ptr().add(1));
    KUNIT_EXPECT_EQ(test, quadlet_count, 2);
    KUNIT_EXPECT_EQ(test, enumerator.quadlet_count, 4);

    entry = self_id_sequence_enumerator_next(&mut enumerator, &mut quadlet_count);
    KUNIT_EXPECT_PTR_EQ(test, entry, VALID_SEQUENCES.as_ptr().add(3));
    KUNIT_EXPECT_EQ(test, quadlet_count, 3);
    KUNIT_EXPECT_EQ(test, enumerator.quadlet_count, 1);

    entry = self_id_sequence_enumerator_next(&mut enumerator, &mut quadlet_count);
    KUNIT_EXPECT_PTR_EQ(test, entry, VALID_SEQUENCES.as_ptr().add(6));
    KUNIT_EXPECT_EQ(test, quadlet_count, 1);
    KUNIT_EXPECT_EQ(test, enumerator.quadlet_count, 0);

    entry = self_id_sequence_enumerator_next(&mut enumerator, &mut quadlet_count);
    KUNIT_EXPECT_EQ(test, PTR_ERR(entry), -ENODATA);
}

unsafe fn test_self_id_sequence_enumerator_invalid(test: *mut kunit) {
    static INVALID_SEQUENCES: [u32; 1] = [0x00000001];
    let mut enumerator: self_id_sequence_enumerator = core::mem::zeroed();
    let mut count: u32 = 0;

    enumerator.cursor = INVALID_SEQUENCES.as_ptr();
    enumerator.quadlet_count = INVALID_SEQUENCES.len() as _;

    let entry = self_id_sequence_enumerator_next(&mut enumerator, &mut count);
    KUNIT_EXPECT_EQ(test, PTR_ERR(entry), -EPROTO);
}

unsafe fn test_self_id_sequence_get_port_status(test: *mut kunit) {
    static EXPECTED: [u32; 4] = [0x000000e5, 0x00839e79, 0x0091e79d, 0x00a279e4];
    let mut quadlets: [u32; 4] = [0x00000001, 0x00800001, 0x00900001, 0x00a00000];
    let mut port_status: [phy_packet_self_id_port_status; 28] = [
        PHY_PACKET_SELF_ID_PORT_STATUS_NONE; 28
    ];
    let port_capacity: u32;
    let mut port_index: u32;

    KUNIT_ASSERT_EQ(test, EXPECTED.len(), quadlets.len());

    // With an extra port.
    port_capacity = self_id_sequence_get_port_capacity(EXPECTED.len() as _) + 1;
    KUNIT_ASSERT_EQ(test, port_capacity, port_status.len());

    port_index = 0;
    while port_index < port_capacity {
        port_status[port_index as usize] =
            self_id_sequence_get_port_status(EXPECTED.as_ptr(), EXPECTED.len() as _, port_index);
        self_id_sequence_set_port_status(
            quadlets.as_mut_ptr(), quadlets.len() as _, port_index,
            port_status[port_index as usize],
        );
        port_index += 1;
    }

    // Self ID zero.
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_CHILD, port_status[0]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_PARENT, port_status[1]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_NCONN, port_status[2]);

    // Self ID one.
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_CHILD, port_status[3]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_PARENT, port_status[4]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_NCONN, port_status[5]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_CHILD, port_status[6]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_PARENT, port_status[7]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_NCONN, port_status[8]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_CHILD, port_status[9]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_PARENT, port_status[10]);

    // Self ID two.
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_NCONN, port_status[11]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_CHILD, port_status[12]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_PARENT, port_status[13]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_NCONN, port_status[14]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_CHILD, port_status[15]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_PARENT, port_status[16]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_NCONN, port_status[17]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_CHILD, port_status[18]);

    // Self ID three.
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_PARENT, port_status[19]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_NCONN, port_status[20]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_CHILD, port_status[21]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_PARENT, port_status[22]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_NCONN, port_status[23]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_CHILD, port_status[24]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_PARENT, port_status[25]);
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_NCONN, port_status[26]);

    // Our of order.
    KUNIT_EXPECT_EQ(test, PHY_PACKET_SELF_ID_PORT_STATUS_NONE, port_status[27]);

    KUNIT_EXPECT_MEMEQ(test, quadlets.as_ptr(), EXPECTED.as_ptr(), core::mem::size_of_val(&EXPECTED));
}

static self_id_sequence_helper_test_cases: [kunit_case; 4] = [
    KUNIT_CASE!(test_self_id_sequence_enumerator_valid),
    KUNIT_CASE!(test_self_id_sequence_enumerator_invalid),
    KUNIT_CASE!(test_self_id_sequence_get_port_status),
    kunit_case {},
];

static self_id_sequence_helper_test_suite: kunit_suite = kunit_suite {
    name: "self-id-sequence-helper",
    test_cases: self_id_sequence_helper_test_cases.as_ptr(),
};

// Registers self_id_sequence_helper_test_suite with KUnit.
KUNIT_TEST_SUITE!(self_id_sequence_helper_test_suite);

// MODULE_DESCRIPTION("Unit test suite for helpers of self ID sequence");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
