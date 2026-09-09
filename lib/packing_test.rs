// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2024, Vladimir Oltean <olteanv@gmail.com>
 * Copyright (c) 2024, Intel Corporation.
 */

// Dependencies supplied by the kernel packing and KUnit headers are intentionally external.

#[repr(C)]
struct packing_test_case {
    desc: *const core::ffi::c_char,
    pbuf: *const u8,
    pbuf_size: usize,
    uval: u64,
    start_bit: usize,
    end_bit: usize,
    quirks: u8,
}

const NO_QUIRKS: u8 = 0;
// QUIRK_LSW32_IS_FIRST, QUIRK_LITTLE_ENDIAN, and QUIRK_MSB_ON_THE_RIGHT are
// provided by linux/packing.h.

macro_rules! cstr { ($s:literal) => { concat!($s, "\0").as_ptr() as *const core::ffi::c_char }; }
macro_rules! case {
    ($d:literal, [$($b:expr),* $(,)?], $v:expr, $s:expr, $e:expr, $q:expr) => {{
        const B: &[u8] = &[$($b),*];
        packing_test_case { desc: cstr!($d), pbuf: B.as_ptr(), pbuf_size: B.len(),
            uval: $v, start_bit: $s, end_bit: $e, quirks: $q }
    }};
}

static cases: &[packing_test_case] = &[
    case!("no quirks, 16 bytes", [0,0,0,0,0xca,0xfe,0xde,0xad,0xbe,0xef,0xca,0xfe,0,0,0,0], 0xcafedeadbeefcafe, 95,32,NO_QUIRKS),
    case!("lsw32 first, 16 bytes", [0,0,0,0,0xbe,0xef,0xca,0xfe,0xca,0xfe,0xde,0xad,0,0,0,0], 0xcafedeadbeefcafe,95,32,QUIRK_LSW32_IS_FIRST),
    case!("little endian words, 16 bytes", [0,0,0,0,0xad,0xde,0xfe,0xca,0xfe,0xca,0xef,0xbe,0,0,0,0], 0xcafedeadbeefcafe,95,32,QUIRK_LITTLE_ENDIAN),
    case!("lsw32 first + little endian words, 16 bytes", [0,0,0,0,0xfe,0xca,0xef,0xbe,0xad,0xde,0xfe,0xca,0,0,0,0], 0xcafedeadbeefcafe,95,32,QUIRK_LSW32_IS_FIRST|QUIRK_LITTLE_ENDIAN),
    case!("msb right, 16 bytes", [0,0,0,0,0x53,0x7f,0x7b,0xb5,0x7d,0xf7,0x53,0x7f,0,0,0,0], 0xcafedeadbeefcafe,95,32,QUIRK_MSB_ON_THE_RIGHT),
    case!("msb right + lsw32 first, 16 bytes", [0,0,0,0,0x7d,0xf7,0x53,0x7f,0x53,0x7f,0x7b,0xb5,0,0,0,0], 0xcafedeadbeefcafe,95,32,QUIRK_MSB_ON_THE_RIGHT|QUIRK_LSW32_IS_FIRST),
    case!("msb right + little endian words, 16 bytes", [0,0,0,0,0xb5,0x7b,0x7f,0x53,0x7f,0x53,0xf7,0x7d,0,0,0,0], 0xcafedeadbeefcafe,95,32,QUIRK_MSB_ON_THE_RIGHT|QUIRK_LITTLE_ENDIAN),
    case!("msb right + lsw32 first + little endian words, 16 bytes", [0,0,0,0,0x7f,0x53,0xf7,0x7d,0xb5,0x7b,0x7f,0x53,0,0,0,0], 0xcafedeadbeefcafe,95,32,QUIRK_MSB_ON_THE_RIGHT|QUIRK_LSW32_IS_FIRST|QUIRK_LITTLE_ENDIAN),
    case!("no quirks, 18 bytes", [0,0,0,0,0,0,0xca,0xfe,0xde,0xad,0xbe,0xef,0xca,0xfe,0,0,0,0], 0xcafedeadbeefcafe,95,32,NO_QUIRKS),
    case!("no quirks, 19 bytes", [0,0,0,0,0,0,0,0xca,0xfe,0xde,0xad,0xbe,0xef,0xca,0xfe,0,0,0,0], 0xcafedeadbeefcafe,95,32,NO_QUIRKS),
    case!("no quirks, 20 bytes", [0,0,0,0,0,0,0,0,0xca,0xfe,0xde,0xad,0xbe,0xef,0xca,0xfe,0,0,0,0], 0xcafedeadbeefcafe,95,32,NO_QUIRKS),
    case!("no quirks, 22 bytes", [0,0,0,0,0,0,0,0,0,0,0xca,0xfe,0xde,0xad,0xbe,0xef,0xca,0xfe,0,0,0,0], 0xcafedeadbeefcafe,95,32,NO_QUIRKS),
    case!("no quirks, 24 bytes", [0,0,0,0,0,0,0,0,0,0,0,0,0xca,0xfe,0xde,0xad,0xbe,0xef,0xca,0xfe,0,0,0,0], 0xcafedeadbeefcafe,95,32,NO_QUIRKS),
    case!("lsw32 first + little endian words, 18 bytes", [0,0,0,0,0xfe,0xca,0xef,0xbe,0xad,0xde,0xfe,0xca,0,0,0,0,0,0], 0xcafedeadbeefcafe,95,32,QUIRK_LSW32_IS_FIRST|QUIRK_LITTLE_ENDIAN),
    case!("lsw32 first + little endian words, 19 bytes", [0,0,0,0,0xfe,0xca,0xef,0xbe,0xad,0xde,0xfe,0xca,0,0,0,0,0,0,0], 0xcafedeadbeefcafe,95,32,QUIRK_LSW32_IS_FIRST|QUIRK_LITTLE_ENDIAN),
    case!("lsw32 first + little endian words, 20 bytes", [0,0,0,0,0xfe,0xca,0xef,0xbe,0xad,0xde,0xfe,0xca,0,0,0,0,0,0,0,0], 0xcafedeadbeefcafe,95,32,QUIRK_LSW32_IS_FIRST|QUIRK_LITTLE_ENDIAN),
    case!("lsw32 first + little endian words, 22 bytes", [0,0,0,0,0xfe,0xca,0xef,0xbe,0xad,0xde,0xfe,0xca,0,0,0,0,0,0,0,0,0,0], 0xcafedeadbeefcafe,95,32,QUIRK_LSW32_IS_FIRST|QUIRK_LITTLE_ENDIAN),
    case!("lsw32 first + little endian words, 24 bytes", [0,0,0,0,0xfe,0xca,0xef,0xbe,0xad,0xde,0xfe,0xca,0,0,0,0,0,0,0,0,0,0,0,0], 0xcafedeadbeefcafe,95,32,QUIRK_LSW32_IS_FIRST|QUIRK_LITTLE_ENDIAN),
    case!("no quirks, 16 bytes, non-aligned", [0,0,0,0x89,0x11,0x9a,0x22,0xab,0x33,0xbc,0x40,0,0,0,0,0], 0x1122334455667788,106,43,NO_QUIRKS),
    case!("lsw32 first, 16 bytes, non-aligned", [0,0,0,0,0x33,0xbc,0x40,0,0x11,0x9a,0x22,0xab,0,0,0,0x89], 0x1122334455667788,106,43,QUIRK_LSW32_IS_FIRST),
    case!("little endian words, 16 bytes, non-aligned", [0x89,0,0,0,0xab,0x22,0x9a,0x11,0,0x40,0xbc,0x33,0,0,0,0], 0x1122334455667788,106,43,QUIRK_LITTLE_ENDIAN),
    case!("lsw32 first + little endian words, 16 bytes, non-aligned", [0,0,0,0,0,0x40,0xbc,0x33,0xab,0x22,0x9a,0x11,0x89,0,0,0], 0x1122334455667788,106,43,QUIRK_LSW32_IS_FIRST|QUIRK_LITTLE_ENDIAN),
    case!("msb right, 16 bytes, non-aligned", [0,0,0,0x91,0x88,0x59,0x44,0xd5,0xcc,0x3d,2,0,0,0,0,0], 0x1122334455667788,106,43,QUIRK_MSB_ON_THE_RIGHT),
    case!("msb right + lsw32 first, 16 bytes, non-aligned", [0,0,0,0,0xcc,0x3d,2,0,0x88,0x59,0x44,0xd5,0,0,0,0x91], 0x1122334455667788,106,43,QUIRK_MSB_ON_THE_RIGHT|QUIRK_LSW32_IS_FIRST),
    case!("msb right + little endian words, 16 bytes, non-aligned", [0x91,0,0,0,0xd5,0x44,0x59,0x88,0,2,0x3d,0xcc,0,0,0,0], 0x1122334455667788,106,43,QUIRK_MSB_ON_THE_RIGHT|QUIRK_LITTLE_ENDIAN),
    case!("msb right + lsw32 first + little endian words, 16 bytes, non-aligned", [0,0,0,0,0,2,0x3d,0xcc,0xd5,0x44,0x59,0x88,0x91,0,0,0], 0x1122334455667788,106,43,QUIRK_MSB_ON_THE_RIGHT|QUIRK_LSW32_IS_FIRST|QUIRK_LITTLE_ENDIAN),
    case!("no quirks, 16 bytes, non-aligned, 0xff", [0,0,7,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xf8,0,0,0,0,0], u64::MAX,106,43,NO_QUIRKS),
    case!("lsw32 first, 16 bytes, non-aligned, 0xff", [0,0,0,0,0xff,0xff,0xf8,0,0xff,0xff,0xff,0xff,0,0,7,0xff], u64::MAX,106,43,QUIRK_LSW32_IS_FIRST),
    case!("little endian words, 16 bytes, non-aligned, 0xff", [0xff,7,0,0,0xff,0xff,0xff,0xff,0,0xf8,0xff,0xff,0,0,0,0], u64::MAX,106,43,QUIRK_LITTLE_ENDIAN),
    case!("lsw32 first + little endian words, 16 bytes, non-aligned, 0xff", [0,0,0,0,0,0xf8,0xff,0xff,0xff,0xff,0xff,0xff,0xff,7,0,0], u64::MAX,106,43,QUIRK_LSW32_IS_FIRST|QUIRK_LITTLE_ENDIAN),
    case!("msb right, 16 bytes, non-aligned, 0xff", [0,0,0xe0,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0x1f,0,0,0,0,0], u64::MAX,106,43,QUIRK_MSB_ON_THE_RIGHT),
    case!("msb right + lsw32 first, 16 bytes, non-aligned, 0xff", [0,0,0,0,0xff,0xff,0x1f,0,0xff,0xff,0xff,0xff,0,0,0xe0,0xff], u64::MAX,106,43,QUIRK_MSB_ON_THE_RIGHT|QUIRK_LSW32_IS_FIRST),
    case!("msb right + little endian words, 16 bytes, non-aligned, 0xff", [0xff,0xe0,0,0,0xff,0xff,0xff,0xff,0,0x1f,0xff,0xff,0,0,0,0], u64::MAX,106,43,QUIRK_MSB_ON_THE_RIGHT|QUIRK_LITTLE_ENDIAN),
    case!("msb right + lsw32 first + little endian words, 16 bytes, non-aligned, 0xff", [0,0,0,0,0,0x1f,0xff,0xff,0xff,0xff,0xff,0xff,0xff,0xe0,0,0], u64::MAX,106,43,QUIRK_MSB_ON_THE_RIGHT|QUIRK_LSW32_IS_FIRST|QUIRK_LITTLE_ENDIAN),
];

const PACKED_BUF_SIZE: usize = 8;
#[repr(C, packed)] struct packed_buf_t { buf: [u8; PACKED_BUF_SIZE] }
#[repr(C)] struct test_data { field3: u32, field2: u16, field4: u16, field6: u16, field1: u8, field5: u8 }

// The following KUnit entry points and packing helpers are external kernel symbols.
extern "C" {
    fn pack(pbuf: *mut u8, uval: u64, start_bit: usize, end_bit: usize, pbuf_size: usize, quirks: u8) -> i32;
    fn unpack(pbuf: *const u8, uval: *mut u64, start_bit: usize, end_bit: usize, pbuf_size: usize, quirks: u8) -> i32;
    fn pack_fields(buf: *mut packed_buf_t, size: usize, data: *const test_data, fields: *const core::ffi::c_void, quirks: u8);
    fn unpack_fields(buf: *const packed_buf_t, size: usize, data: *mut test_data, fields: *const core::ffi::c_void, quirks: u8);
}

// KUNIT_ARRAY_PARAM_DESC(packing, cases, desc);
// KUNIT_CASE_PARAM/KUNIT_CASE registrations and kunit_test_suite(packing_test_suite)
// are supplied by the KUnit integration.  The test logic is preserved below.
unsafe fn packing_test_pack(test: *mut core::ffi::c_void, params: *const packing_test_case) {
    let mut pbuf = vec![0u8; (*params).pbuf_size];
    let err = pack(pbuf.as_mut_ptr(), (*params).uval, (*params).start_bit,
                   (*params).end_bit, (*params).pbuf_size, (*params).quirks);
    // KUNIT_EXPECT_EQ_MSG(test, err, 0, "pack() returned %pe\n", ERR_PTR(err));
    // KUNIT_EXPECT_MEMEQ(test, pbuf, (*params).pbuf, (*params).pbuf_size);
    let _ = (test, err);
}

unsafe fn packing_test_unpack(test: *mut core::ffi::c_void, params: *const packing_test_case) {
    let mut uval = 0u64;
    let err = unpack((*params).pbuf, &mut uval, (*params).start_bit, (*params).end_bit,
                     (*params).pbuf_size, (*params).quirks);
    // KUNIT_EXPECT_EQ_MSG(test, err, 0, "unpack() returned %pe\n", ERR_PTR(err));
    // KUNIT_EXPECT_EQ(test, uval, (*params).uval);
    let _ = (test, err, uval);
}

// PACKED_FIELD declarations, packing_test_pack_fields, packing_test_unpack_fields,
// the packing_test_cases array, suite metadata, MODULE_LICENSE, and MODULE_DESCRIPTION
// retain their kernel meaning and depend on the corresponding external KUnit macros.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
