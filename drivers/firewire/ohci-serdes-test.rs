// SPDX-License-Identifier: GPL-2.0-or-later
//
// ohci-serdes-test.c - An application of Kunit to check serialization/deserialization of data in
//                      buffers and registers defined in 1394 OHCI specification.
//
// Copyright (c) 2024 Takashi Sakamoto

// Dependencies supplied by kunit/test.h and ohci.h are intentionally external.

use core::ffi::c_void;

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn ohci1394_self_id_count_is_error(value: u32) -> bool;
    fn ohci1394_self_id_count_get_generation(value: u32) -> u8;
    fn ohci1394_self_id_count_get_size(value: u32) -> u32;

    fn ohci1394_self_id_receive_q0_get_generation(value: u32) -> u8;
    fn ohci1394_self_id_receive_q0_get_timestamp(value: u32) -> u16;

    fn ohci1394_at_data_get_src_bus_id(expected: *const u32) -> bool;
    fn ohci1394_at_data_get_speed(expected: *const u32) -> u32;
    fn ohci1394_at_data_get_tlabel(expected: *const u32) -> u32;
    fn ohci1394_at_data_get_retry(expected: *const u32) -> u32;
    fn ohci1394_at_data_get_tcode(expected: *const u32) -> u32;
    fn ohci1394_at_data_get_destination_id(expected: *const u32) -> u32;
    fn ohci1394_at_data_get_destination_offset(expected: *const u32) -> u64;
    fn ohci1394_at_data_set_src_bus_id(quadlets: *mut u32, value: bool);
    fn ohci1394_at_data_set_speed(quadlets: *mut u32, value: u32);
    fn ohci1394_at_data_set_tlabel(quadlets: *mut u32, value: u32);
    fn ohci1394_at_data_set_retry(quadlets: *mut u32, value: u32);
    fn ohci1394_at_data_set_tcode(quadlets: *mut u32, value: u32);
    fn ohci1394_at_data_set_destination_id(quadlets: *mut u32, value: u32);
    fn ohci1394_at_data_set_destination_offset(quadlets: *mut u32, value: u64);

    fn ohci1394_it_data_get_speed(expected: *const u32) -> u32;
    fn ohci1394_it_data_get_tag(expected: *const u32) -> u32;
    fn ohci1394_it_data_get_channel(expected: *const u32) -> u32;
    fn ohci1394_it_data_get_tcode(expected: *const u32) -> u32;
    fn ohci1394_it_data_get_sync(expected: *const u32) -> u32;
    fn ohci1394_it_data_get_data_length(expected: *const u32) -> u32;
    fn ohci1394_it_data_set_speed(quadlets: *mut u32, value: u32);
    fn ohci1394_it_data_set_tag(quadlets: *mut u32, value: u32);
    fn ohci1394_it_data_set_channel(quadlets: *mut u32, value: u32);
    fn ohci1394_it_data_set_tcode(quadlets: *mut u32, value: u32);
    fn ohci1394_it_data_set_sync(quadlets: *mut u32, value: u32);
    fn ohci1394_it_data_set_data_length(quadlets: *mut u32, value: u32);
}

unsafe fn test_self_id_count_register_deserialization(_test: *mut kunit) {
    let expected: u32 = 0x803d0594;
    let is_error = ohci1394_self_id_count_is_error(expected);
    let generation = ohci1394_self_id_count_get_generation(expected);
    let size = ohci1394_self_id_count_get_size(expected);
    assert!(is_error);
    assert_eq!(generation, 0x3d);
    assert_eq!(size, 0x165);
}

unsafe fn test_self_id_receive_buffer_deserialization(_test: *mut kunit) {
    let buffer: [u32; 5] = [0x0006f38b, 0x807fcc56, 0x7f8033a9, 0x8145cc5e, 0x7eba33a1];
    let generation = ohci1394_self_id_receive_q0_get_generation(buffer[0]);
    let timestamp = ohci1394_self_id_receive_q0_get_timestamp(buffer[0]);
    assert_eq!(generation, 0x6);
    assert_eq!(timestamp, 0xf38b);
}

unsafe fn test_at_data_serdes(_test: *mut kunit) {
    let expected: [u32; 3] = [0x00020e80, 0xffc2ffff, 0xe0000000];
    let mut quadlets = [0u32; 3];
    let has_src_bus_id = ohci1394_at_data_get_src_bus_id(expected.as_ptr());
    let speed = ohci1394_at_data_get_speed(expected.as_ptr());
    let tlabel = ohci1394_at_data_get_tlabel(expected.as_ptr());
    let retry = ohci1394_at_data_get_retry(expected.as_ptr());
    let tcode = ohci1394_at_data_get_tcode(expected.as_ptr());
    let destination_id = ohci1394_at_data_get_destination_id(expected.as_ptr());
    let destination_offset = ohci1394_at_data_get_destination_offset(expected.as_ptr());
    assert!(!has_src_bus_id);
    assert_eq!(speed, 0x02); assert_eq!(tlabel, 0x03); assert_eq!(retry, 0x02); assert_eq!(tcode, 0x08);
    ohci1394_at_data_set_src_bus_id(quadlets.as_mut_ptr(), has_src_bus_id);
    ohci1394_at_data_set_speed(quadlets.as_mut_ptr(), speed);
    ohci1394_at_data_set_tlabel(quadlets.as_mut_ptr(), tlabel);
    ohci1394_at_data_set_retry(quadlets.as_mut_ptr(), retry);
    ohci1394_at_data_set_tcode(quadlets.as_mut_ptr(), tcode);
    ohci1394_at_data_set_destination_id(quadlets.as_mut_ptr(), destination_id);
    ohci1394_at_data_set_destination_offset(quadlets.as_mut_ptr(), destination_offset);
    assert_eq!(quadlets, expected);
}

unsafe fn test_it_data_serdes(_test: *mut kunit) {
    let expected: [u32; 2] = [0x000349a7, 0x02300000];
    let mut quadlets = [0u32; 2];
    let scode = ohci1394_it_data_get_speed(expected.as_ptr());
    let tag = ohci1394_it_data_get_tag(expected.as_ptr());
    let channel = ohci1394_it_data_get_channel(expected.as_ptr());
    let tcode = ohci1394_it_data_get_tcode(expected.as_ptr());
    let sync = ohci1394_it_data_get_sync(expected.as_ptr());
    let data_length = ohci1394_it_data_get_data_length(expected.as_ptr());
    assert_eq!(scode, 0x03); assert_eq!(tag, 0x01); assert_eq!(channel, 0x09);
    assert_eq!(tcode, 0x0a); assert_eq!(sync, 0x7); assert_eq!(data_length, 0x0230);
    ohci1394_it_data_set_speed(quadlets.as_mut_ptr(), scode);
    ohci1394_it_data_set_tag(quadlets.as_mut_ptr(), tag);
    ohci1394_it_data_set_channel(quadlets.as_mut_ptr(), channel);
    ohci1394_it_data_set_tcode(quadlets.as_mut_ptr(), tcode);
    ohci1394_it_data_set_sync(quadlets.as_mut_ptr(), sync);
    ohci1394_it_data_set_data_length(quadlets.as_mut_ptr(), data_length);
    assert_eq!(quadlets, expected);
}

// KUNIT_CASE entries and kunit_test_suite(ohci_serdes_test_suite) register the four tests above.
// MODULE_DESCRIPTION("FireWire buffers and registers serialization/deserialization unit test suite");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
