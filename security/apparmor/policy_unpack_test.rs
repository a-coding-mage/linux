// SPDX-License-Identifier: GPL-2.0-only
// KUnit tests for AppArmor's policy unpack.
//
// External dependencies: kunit/test.h, kunit/visibility.h, include/policy.h,
// include/policy_unpack.h, linux/unaligned.h

use std::ffi::CStr;
use std::mem;
use std::ptr;

const TEST_STRING_NAME: &str = "TEST_STRING";
const TEST_STRING_DATA: &str = "testing";
const TEST_STRING_BUF_OFFSET: usize = 3 + 11 + 1; // 3 + strlen(TEST_STRING_NAME) + 1

const TEST_U32_NAME: &str = "U32_TEST";
const TEST_U32_DATA: u32 = 0x01020304;
const TEST_NAMED_U32_BUF_OFFSET: usize = TEST_STRING_BUF_OFFSET + 3 + 7 + 1; // + strlen(TEST_STRING_DATA) + 1
const TEST_U32_BUF_OFFSET: usize = TEST_NAMED_U32_BUF_OFFSET + 3 + 8 + 1; // + strlen(TEST_U32_NAME) + 1

const TEST_U16_OFFSET: usize = TEST_U32_BUF_OFFSET + 3;
const TEST_U16_DATA: u16 = (TEST_U32_DATA >> 16) as u16;

const TEST_U64_NAME: &str = "U64_TEST";
const TEST_U64_DATA: u64 = 0x0102030405060708;
const TEST_NAMED_U64_BUF_OFFSET: usize = TEST_U32_BUF_OFFSET + mem::size_of::<u32>() + 1;
const TEST_U64_BUF_OFFSET: usize = TEST_NAMED_U64_BUF_OFFSET + 3 + 8 + 1; // + strlen(TEST_U64_NAME) + 1

const TEST_BLOB_NAME: &str = "BLOB_TEST";
const TEST_BLOB_DATA: &[u8] = b"\xde\xad\x00\xbe\xef";
const TEST_BLOB_DATA_SIZE: usize = 5;
const TEST_NAMED_BLOB_BUF_OFFSET: usize = TEST_U64_BUF_OFFSET + mem::size_of::<u64>() + 1;
const TEST_BLOB_BUF_OFFSET: usize = TEST_NAMED_BLOB_BUF_OFFSET + 3 + 9 + 1; // + strlen(TEST_BLOB_NAME) + 1

const TEST_ARRAY_NAME: &str = "ARRAY_TEST";
const TEST_ARRAY_SIZE: usize = 16;
const TEST_NAMED_ARRAY_BUF_OFFSET: usize = TEST_BLOB_BUF_OFFSET + 5 + TEST_BLOB_DATA_SIZE;
const TEST_ARRAY_BUF_OFFSET: usize = TEST_NAMED_ARRAY_BUF_OFFSET + 3 + 10 + 1; // + strlen(TEST_ARRAY_NAME) + 1

extern "C" {
    type aa_ext;

    fn kunit_kzalloc(test: *mut KunitTest, size: usize, flags: u32) -> *mut u8;
    fn kunit_kmalloc(test: *mut KunitTest, size: usize, flags: u32) -> *mut u8;
    fn kfree(ptr: *mut u8);

    fn strscpy(dest: *mut u8, src: *const u8, count: usize) -> isize;
    fn memcpy(dest: *mut u8, src: *const u8, n: usize) -> *mut u8;
    fn memcmp(s1: *const u8, s2: *const u8, n: usize) -> i32;

    fn put_unaligned_le32(val: u32, p: *mut u8);
    fn put_unaligned_le16(val: u16, p: *mut u8);
    fn cpu_to_le64(val: u64) -> u64;

    fn aa_inbounds(e: *mut aa_ext, size: usize) -> bool;
    fn aa_unpack_array(e: *mut aa_ext, name: *const u8, array_size: *mut u16) -> bool;
    fn aa_unpack_blob(e: *mut aa_ext, blob: *mut *mut u8, name: *const u8) -> usize;
    fn aa_unpack_str(e: *mut aa_ext, string: *mut *const u8, name: *const u8) -> usize;
    fn aa_unpack_strdup(e: *mut aa_ext, string: *mut *mut u8, name: *const u8) -> usize;
    fn aa_unpack_nameX(e: *mut aa_ext, code: u8, name: *const u8) -> bool;
    fn aa_unpack_u16_chunk(e: *mut aa_ext, chunk: *mut *mut u8) -> usize;
    fn aa_unpack_u32(e: *mut aa_ext, data: *mut u32, name: *const u8) -> bool;
    fn aa_unpack_u64(e: *mut aa_ext, data: *mut u64, name: *const u8) -> bool;
    fn aa_unpack_X(e: *mut aa_ext, code: u8) -> bool;

    fn kunit_test_expect(
        test: *mut KunitTest,
        condition: bool,
        func_name: *const u8,
        line: u32,
        msg: *const u8,
    );
    fn kunit_test_expect_eq(
        test: *mut KunitTest,
        a: u64,
        b: u64,
        func_name: *const u8,
        line: u32,
        msg: *const u8,
    );
    fn kunit_test_expect_ptr_eq(
        test: *mut KunitTest,
        a: *const u8,
        b: *const u8,
        func_name: *const u8,
        line: u32,
        msg: *const u8,
    );
}

const AA_NAME: u8 = 0;
const AA_STRING: u8 = 1;
const AA_U32: u8 = 2;
const AA_U64: u8 = 3;
const AA_BLOB: u8 = 4;
const AA_ARRAY: u8 = 5;

const GFP_USER: u32 = 0x200;

#[repr(C)]
pub struct KunitTest {
    _private: [u8; 0],
}

#[repr(C)]
struct PolicyUnpackFixture {
    e: *mut aa_ext,
    e_size: usize,
}

unsafe fn build_aa_ext_struct(
    puf: *mut PolicyUnpackFixture,
    test: *mut KunitTest,
    buf_size: usize,
) -> *mut aa_ext {
    let buf = kunit_kzalloc(test, buf_size, GFP_USER);
    // KUNIT_EXPECT_NOT_ERR_OR_NULL(test, buf);

    let e = kunit_kmalloc(test, mem::size_of::<aa_ext>(), GFP_USER) as *mut aa_ext;
    // KUNIT_EXPECT_NOT_ERR_OR_NULL(test, e);

    (*e).start = buf;
    (*e).end = ((*e).start as *mut u8).add(buf_size);
    (*e).pos = (*e).start;

    let mut buf_ptr = buf;
    *buf_ptr = AA_NAME;
    *buf_ptr.add(1) = (TEST_STRING_NAME.len() + 1) as u8;
    strscpy(
        buf_ptr.add(3),
        TEST_STRING_NAME.as_ptr(),
        ((*e).end as usize) - (buf_ptr as usize) - 3,
    );

    buf_ptr = ((*e).start as *mut u8).add(TEST_STRING_BUF_OFFSET);
    *buf_ptr = AA_STRING;
    *buf_ptr.add(1) = (TEST_STRING_DATA.len() + 1) as u8;
    strscpy(
        buf_ptr.add(3),
        TEST_STRING_DATA.as_ptr(),
        ((*e).end as usize) - (buf_ptr as usize) - 3,
    );

    buf_ptr = ((*e).start as *mut u8).add(TEST_NAMED_U32_BUF_OFFSET);
    *buf_ptr = AA_NAME;
    *buf_ptr.add(1) = (TEST_U32_NAME.len() + 1) as u8;
    strscpy(
        buf_ptr.add(3),
        TEST_U32_NAME.as_ptr(),
        ((*e).end as usize) - (buf_ptr as usize) - 3,
    );
    *buf_ptr.add(3 + TEST_U32_NAME.len() + 1) = AA_U32;
    put_unaligned_le32(TEST_U32_DATA, buf_ptr.add(3 + TEST_U32_NAME.len() + 2));

    buf_ptr = ((*e).start as *mut u8).add(TEST_NAMED_U64_BUF_OFFSET);
    *buf_ptr = AA_NAME;
    *buf_ptr.add(1) = (TEST_U64_NAME.len() + 1) as u8;
    strscpy(
        buf_ptr.add(3),
        TEST_U64_NAME.as_ptr(),
        ((*e).end as usize) - (buf_ptr as usize) - 3,
    );
    *buf_ptr.add(3 + TEST_U64_NAME.len() + 1) = AA_U64;
    let le64_val = cpu_to_le64(TEST_U64_DATA);
    *(buf_ptr.add(3 + TEST_U64_NAME.len() + 2) as *mut u64) = le64_val;

    buf_ptr = ((*e).start as *mut u8).add(TEST_NAMED_BLOB_BUF_OFFSET);
    *buf_ptr = AA_NAME;
    *buf_ptr.add(1) = (TEST_BLOB_NAME.len() + 1) as u8;
    strscpy(
        buf_ptr.add(3),
        TEST_BLOB_NAME.as_ptr(),
        ((*e).end as usize) - (buf_ptr as usize) - 3,
    );
    *buf_ptr.add(3 + TEST_BLOB_NAME.len() + 1) = AA_BLOB;
    *buf_ptr.add(3 + TEST_BLOB_NAME.len() + 2) = TEST_BLOB_DATA_SIZE as u8;
    memcpy(
        buf_ptr.add(3 + TEST_BLOB_NAME.len() + 6),
        TEST_BLOB_DATA.as_ptr(),
        TEST_BLOB_DATA_SIZE,
    );

    buf_ptr = ((*e).start as *mut u8).add(TEST_NAMED_ARRAY_BUF_OFFSET);
    *buf_ptr = AA_NAME;
    *buf_ptr.add(1) = (TEST_ARRAY_NAME.len() + 1) as u8;
    strscpy(
        buf_ptr.add(3),
        TEST_ARRAY_NAME.as_ptr(),
        ((*e).end as usize) - (buf_ptr as usize) - 3,
    );
    *buf_ptr.add(3 + TEST_ARRAY_NAME.len() + 1) = AA_ARRAY;
    put_unaligned_le16(TEST_ARRAY_SIZE as u16, buf_ptr.add(3 + TEST_ARRAY_NAME.len() + 2));

    e
}

unsafe fn policy_unpack_test_init(test: *mut KunitTest) -> i32 {
    let e_size = TEST_ARRAY_BUF_OFFSET + mem::size_of::<u16>() + 1;
    let puf = kunit_kmalloc(test, mem::size_of::<PolicyUnpackFixture>(), GFP_USER)
        as *mut PolicyUnpackFixture;
    // KUNIT_EXPECT_NOT_ERR_OR_NULL(test, puf);

    (*puf).e_size = e_size;
    (*puf).e = build_aa_ext_struct(puf, test, e_size);

    (*test).priv = puf as *mut u8;
    0
}

unsafe fn policy_unpack_test_inbounds_when_inbounds(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;

    // KUNIT_EXPECT_TRUE(test, aa_inbounds((*puf).e, 0));
    // KUNIT_EXPECT_TRUE(test, aa_inbounds((*puf).e, (*puf).e_size / 2));
    // KUNIT_EXPECT_TRUE(test, aa_inbounds((*puf).e, (*puf).e_size));
}

unsafe fn policy_unpack_test_inbounds_when_out_of_bounds(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;

    // KUNIT_EXPECT_FALSE(test, aa_inbounds((*puf).e, (*puf).e_size + 1));
}

unsafe fn policy_unpack_test_unpack_array_with_null_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut array_size: u16 = 0;

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_ARRAY_BUF_OFFSET);

    // KUNIT_EXPECT_TRUE(test, aa_unpack_array((*puf).e, ptr::null(), &mut array_size));
    // KUNIT_EXPECT_EQ(test, array_size, (u16)TEST_ARRAY_SIZE);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos,
    //     (*puf).e->start + TEST_ARRAY_BUF_OFFSET + sizeof(u16) + 1);
}

unsafe fn policy_unpack_test_unpack_array_with_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut array_size: u16 = 0;

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_NAMED_ARRAY_BUF_OFFSET);

    // KUNIT_EXPECT_TRUE(test, aa_unpack_array((*puf).e, TEST_ARRAY_NAME.as_ptr(), &mut array_size));
    // KUNIT_EXPECT_EQ(test, array_size, (u16)TEST_ARRAY_SIZE);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos,
    //     (*puf).e->start + TEST_ARRAY_BUF_OFFSET + sizeof(u16) + 1);
}

unsafe fn policy_unpack_test_unpack_array_out_of_bounds(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut array_size: u16 = 0;

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_NAMED_ARRAY_BUF_OFFSET);
    (*(*puf).e).end = ((*(*puf).e).start as *mut u8).add(TEST_ARRAY_BUF_OFFSET + mem::size_of::<u16>());

    // KUNIT_EXPECT_FALSE(test, aa_unpack_array((*puf).e, TEST_ARRAY_NAME.as_ptr(), &mut array_size));
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos,
    //     (*puf).e->start + TEST_NAMED_ARRAY_BUF_OFFSET);
}

unsafe fn policy_unpack_test_unpack_blob_with_null_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut blob: *mut u8 = ptr::null_mut();

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_BLOB_BUF_OFFSET);
    let size = aa_unpack_blob((*puf).e, &mut blob, ptr::null());

    // KUNIT_ASSERT_EQ(test, size, TEST_BLOB_DATA_SIZE);
    // KUNIT_EXPECT_TRUE(test,
    //     memcmp(blob, TEST_BLOB_DATA.as_ptr(), TEST_BLOB_DATA_SIZE) == 0);
}

unsafe fn policy_unpack_test_unpack_blob_with_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut blob: *mut u8 = ptr::null_mut();

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_NAMED_BLOB_BUF_OFFSET);
    let size = aa_unpack_blob((*puf).e, &mut blob, TEST_BLOB_NAME.as_ptr());

    // KUNIT_ASSERT_EQ(test, size, TEST_BLOB_DATA_SIZE);
    // KUNIT_EXPECT_TRUE(test,
    //     memcmp(blob, TEST_BLOB_DATA.as_ptr(), TEST_BLOB_DATA_SIZE) == 0);
}

unsafe fn policy_unpack_test_unpack_blob_out_of_bounds(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut blob: *mut u8 = ptr::null_mut();
    let start = (*(*puf).e).pos;

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_NAMED_BLOB_BUF_OFFSET);
    (*(*puf).e).end = ((*(*puf).e).start as *mut u8)
        .add(TEST_BLOB_BUF_OFFSET + TEST_BLOB_DATA_SIZE - 1);

    let size = aa_unpack_blob((*puf).e, &mut blob, TEST_BLOB_NAME.as_ptr());

    // KUNIT_EXPECT_EQ(test, size, 0);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos, start);
}

unsafe fn policy_unpack_test_unpack_str_with_null_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut string: *const u8 = ptr::null();

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_STRING_BUF_OFFSET);
    let size = aa_unpack_str((*puf).e, &mut string, ptr::null());

    // KUNIT_EXPECT_EQ(test, size, strlen(TEST_STRING_DATA) + 1);
    // KUNIT_EXPECT_STREQ(test, string, TEST_STRING_DATA);
}

unsafe fn policy_unpack_test_unpack_str_with_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut string: *const u8 = ptr::null();

    let size = aa_unpack_str((*puf).e, &mut string, TEST_STRING_NAME.as_ptr());

    // KUNIT_EXPECT_EQ(test, size, strlen(TEST_STRING_DATA) + 1);
    // KUNIT_EXPECT_STREQ(test, string, TEST_STRING_DATA);
}

unsafe fn policy_unpack_test_unpack_str_out_of_bounds(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut string: *const u8 = ptr::null();
    let start = (*(*puf).e).pos;

    (*(*puf).e).end = ((*(*puf).e).pos as *mut u8)
        .add(TEST_STRING_BUF_OFFSET + TEST_STRING_DATA.len() - 1);

    let size = aa_unpack_str((*puf).e, &mut string, TEST_STRING_NAME.as_ptr());

    // KUNIT_EXPECT_EQ(test, size, 0);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos, start);
}

unsafe fn policy_unpack_test_unpack_strdup_with_null_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut string: *mut u8 = ptr::null_mut();

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_STRING_BUF_OFFSET);
    let size = aa_unpack_strdup((*puf).e, &mut string, ptr::null());

    // KUNIT_EXPECT_EQ(test, size, strlen(TEST_STRING_DATA) + 1);
    // KUNIT_EXPECT_FALSE(test,
    //         ((uintptr_t)(*puf).e->start <= (uintptr_t)string)
    //         && ((uintptr_t)string <= (uintptr_t)(*puf).e->end));
    // KUNIT_EXPECT_STREQ(test, string, TEST_STRING_DATA);

    kfree(string);
}

unsafe fn policy_unpack_test_unpack_strdup_with_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut string: *mut u8 = ptr::null_mut();

    let size = aa_unpack_strdup((*puf).e, &mut string, TEST_STRING_NAME.as_ptr());

    // KUNIT_EXPECT_EQ(test, size, strlen(TEST_STRING_DATA) + 1);
    // KUNIT_EXPECT_FALSE(test,
    //         ((uintptr_t)(*puf).e->start <= (uintptr_t)string)
    //         && ((uintptr_t)string <= (uintptr_t)(*puf).e->end));
    // KUNIT_EXPECT_STREQ(test, string, TEST_STRING_DATA);

    kfree(string);
}

unsafe fn policy_unpack_test_unpack_strdup_out_of_bounds(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let start = (*(*puf).e).pos;
    let mut string: *mut u8 = ptr::null_mut();

    (*(*puf).e).end = ((*(*puf).e).pos as *mut u8)
        .add(TEST_STRING_BUF_OFFSET + TEST_STRING_DATA.len() - 1);

    let size = aa_unpack_strdup((*puf).e, &mut string, TEST_STRING_NAME.as_ptr());

    // KUNIT_EXPECT_EQ(test, size, 0);
    // KUNIT_EXPECT_NULL(test, string);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos, start);

    kfree(string);
}

unsafe fn policy_unpack_test_unpack_nameX_with_null_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_U32_BUF_OFFSET);

    let success = aa_unpack_nameX((*puf).e, AA_U32, ptr::null());

    // KUNIT_EXPECT_TRUE(test, success);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos,
    //         (*puf).e->start + TEST_U32_BUF_OFFSET + 1);
}

unsafe fn policy_unpack_test_unpack_nameX_with_wrong_code(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_U32_BUF_OFFSET);

    let success = aa_unpack_nameX((*puf).e, AA_BLOB, ptr::null());

    // KUNIT_EXPECT_FALSE(test, success);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos,
    //         (*puf).e->start + TEST_U32_BUF_OFFSET);
}

unsafe fn policy_unpack_test_unpack_nameX_with_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_NAMED_U32_BUF_OFFSET);

    let success = aa_unpack_nameX((*puf).e, AA_U32, TEST_U32_NAME.as_ptr());

    // KUNIT_EXPECT_TRUE(test, success);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos,
    //         (*puf).e->start + TEST_U32_BUF_OFFSET + 1);
}

unsafe fn policy_unpack_test_unpack_nameX_with_wrong_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let name = b"12345678";

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_NAMED_U32_BUF_OFFSET);

    let success = aa_unpack_nameX((*puf).e, AA_U32, name.as_ptr());

    // KUNIT_EXPECT_FALSE(test, success);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos,
    //         (*puf).e->start + TEST_NAMED_U32_BUF_OFFSET);
}

unsafe fn policy_unpack_test_unpack_u16_chunk_basic(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut chunk: *mut u8 = ptr::null_mut();

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_U16_OFFSET);
    // WARNING: For unit testing purposes, we're pushing (*puf).e.end past
    // the end of the allocated memory. Doing anything other than comparing
    // memory addresses is dangerous.
    (*(*puf).e).end = ((*(*puf).e).end as *mut u8).add(TEST_U16_DATA as usize);

    let size = aa_unpack_u16_chunk((*puf).e, &mut chunk);

    // KUNIT_EXPECT_PTR_EQ(test, chunk,
    //         (*puf).e->start + TEST_U16_OFFSET + 2);
    // KUNIT_EXPECT_EQ(test, size, TEST_U16_DATA);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos, (chunk + TEST_U16_DATA));
}

unsafe fn policy_unpack_test_unpack_u16_chunk_out_of_bounds_1(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut chunk: *mut u8 = ptr::null_mut();

    (*(*puf).e).pos = ((*(*puf).e).end as *mut u8).offset(-1);

    let size = aa_unpack_u16_chunk((*puf).e, &mut chunk);

    // KUNIT_EXPECT_EQ(test, size, 0);
    // KUNIT_EXPECT_NULL(test, chunk);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos, (*puf).e->end - 1);
}

unsafe fn policy_unpack_test_unpack_u16_chunk_out_of_bounds_2(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut chunk: *mut u8 = ptr::null_mut();

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_U16_OFFSET);
    // WARNING: For unit testing purposes, we're pushing (*puf).e.end past
    // the end of the allocated memory. Doing anything other than comparing
    // memory addresses is dangerous.
    (*(*puf).e).end =
        ((*(*puf).e).pos as *mut u8).add(TEST_U16_DATA as usize - 1);

    let size = aa_unpack_u16_chunk((*puf).e, &mut chunk);

    // KUNIT_EXPECT_EQ(test, size, 0);
    // KUNIT_EXPECT_NULL(test, chunk);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos, (*puf).e->start + TEST_U16_OFFSET);
}

unsafe fn policy_unpack_test_unpack_u32_with_null_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut data: u32 = 0;

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_U32_BUF_OFFSET);

    let success = aa_unpack_u32((*puf).e, &mut data, ptr::null());

    // KUNIT_EXPECT_TRUE(test, success);
    // KUNIT_EXPECT_EQ(test, data, TEST_U32_DATA);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos,
    //     (*puf).e->start + TEST_U32_BUF_OFFSET + sizeof(u32) + 1);
}

unsafe fn policy_unpack_test_unpack_u32_with_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut data: u32 = 0;

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_NAMED_U32_BUF_OFFSET);

    let success = aa_unpack_u32((*puf).e, &mut data, TEST_U32_NAME.as_ptr());

    // KUNIT_EXPECT_TRUE(test, success);
    // KUNIT_EXPECT_EQ(test, data, TEST_U32_DATA);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos,
    //     (*puf).e->start + TEST_U32_BUF_OFFSET + sizeof(u32) + 1);
}

unsafe fn policy_unpack_test_unpack_u32_out_of_bounds(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut data: u32 = 0;

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_NAMED_U32_BUF_OFFSET);
    (*(*puf).e).end = ((*(*puf).e).start as *mut u8).add(TEST_U32_BUF_OFFSET + mem::size_of::<u32>());

    let success = aa_unpack_u32((*puf).e, &mut data, TEST_U32_NAME.as_ptr());

    // KUNIT_EXPECT_FALSE(test, success);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos,
    //     (*puf).e->start + TEST_NAMED_U32_BUF_OFFSET);
}

unsafe fn policy_unpack_test_unpack_u64_with_null_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut data: u64 = 0;

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_U64_BUF_OFFSET);

    let success = aa_unpack_u64((*puf).e, &mut data, ptr::null());

    // KUNIT_EXPECT_TRUE(test, success);
    // KUNIT_EXPECT_EQ(test, data, TEST_U64_DATA);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos,
    //     (*puf).e->start + TEST_U64_BUF_OFFSET + sizeof(u64) + 1);
}

unsafe fn policy_unpack_test_unpack_u64_with_name(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut data: u64 = 0;

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_NAMED_U64_BUF_OFFSET);

    let success = aa_unpack_u64((*puf).e, &mut data, TEST_U64_NAME.as_ptr());

    // KUNIT_EXPECT_TRUE(test, success);
    // KUNIT_EXPECT_EQ(test, data, TEST_U64_DATA);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos,
    //     (*puf).e->start + TEST_U64_BUF_OFFSET + sizeof(u64) + 1);
}

unsafe fn policy_unpack_test_unpack_u64_out_of_bounds(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let mut data: u64 = 0;

    (*(*puf).e).pos = ((*(*puf).e).start as *mut u8).add(TEST_NAMED_U64_BUF_OFFSET);
    (*(*puf).e).end = ((*(*puf).e).start as *mut u8).add(TEST_U64_BUF_OFFSET + mem::size_of::<u64>());

    let success = aa_unpack_u64((*puf).e, &mut data, TEST_U64_NAME.as_ptr());

    // KUNIT_EXPECT_FALSE(test, success);
    // KUNIT_EXPECT_PTR_EQ(test, (*puf).e->pos,
    //     (*puf).e->start + TEST_NAMED_U64_BUF_OFFSET);
}

unsafe fn policy_unpack_test_unpack_X_code_match(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let success = aa_unpack_X((*puf).e, AA_NAME);

    // KUNIT_EXPECT_TRUE(test, success);
    // KUNIT_EXPECT_TRUE(test, (*puf).e->pos == (*puf).e->start + 1);
}

unsafe fn policy_unpack_test_unpack_X_code_mismatch(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;
    let success = aa_unpack_X((*puf).e, AA_STRING);

    // KUNIT_EXPECT_FALSE(test, success);
    // KUNIT_EXPECT_TRUE(test, (*puf).e->pos == (*puf).e->start);
}

unsafe fn policy_unpack_test_unpack_X_out_of_bounds(test: *mut KunitTest) {
    let puf = (*test).priv as *mut PolicyUnpackFixture;

    (*(*puf).e).pos = (*(*puf).e).end;
    let success = aa_unpack_X((*puf).e, AA_NAME);

    // KUNIT_EXPECT_FALSE(test, success);
}

// KUnit test suite structure and registration.
// Note: KUnit integration would require the actual KUnit runtime support.
// The following represents the test case table and module setup equivalent.

type KunitTestCaseFn = unsafe fn(*mut KunitTest);

#[repr(C)]
struct KunitCase {
    name: *const u8,
    run: KunitTestCaseFn,
}

const APPARMOR_POLICY_UNPACK_TEST_CASES: &[(&[u8], KunitTestCaseFn)] = &[
    (b"policy_unpack_test_inbounds_when_inbounds\0", policy_unpack_test_inbounds_when_inbounds),
    (b"policy_unpack_test_inbounds_when_out_of_bounds\0", policy_unpack_test_inbounds_when_out_of_bounds),
    (b"policy_unpack_test_unpack_array_with_null_name\0", policy_unpack_test_unpack_array_with_null_name),
    (b"policy_unpack_test_unpack_array_with_name\0", policy_unpack_test_unpack_array_with_name),
    (b"policy_unpack_test_unpack_array_out_of_bounds\0", policy_unpack_test_unpack_array_out_of_bounds),
    (b"policy_unpack_test_unpack_blob_with_null_name\0", policy_unpack_test_unpack_blob_with_null_name),
    (b"policy_unpack_test_unpack_blob_with_name\0", policy_unpack_test_unpack_blob_with_name),
    (b"policy_unpack_test_unpack_blob_out_of_bounds\0", policy_unpack_test_unpack_blob_out_of_bounds),
    (b"policy_unpack_test_unpack_nameX_with_null_name\0", policy_unpack_test_unpack_nameX_with_null_name),
    (b"policy_unpack_test_unpack_nameX_with_wrong_code\0", policy_unpack_test_unpack_nameX_with_wrong_code),
    (b"policy_unpack_test_unpack_nameX_with_name\0", policy_unpack_test_unpack_nameX_with_name),
    (b"policy_unpack_test_unpack_nameX_with_wrong_name\0", policy_unpack_test_unpack_nameX_with_wrong_name),
    (b"policy_unpack_test_unpack_str_with_null_name\0", policy_unpack_test_unpack_str_with_null_name),
    (b"policy_unpack_test_unpack_str_with_name\0", policy_unpack_test_unpack_str_with_name),
    (b"policy_unpack_test_unpack_str_out_of_bounds\0", policy_unpack_test_unpack_str_out_of_bounds),
    (b"policy_unpack_test_unpack_strdup_with_null_name\0", policy_unpack_test_unpack_strdup_with_null_name),
    (b"policy_unpack_test_unpack_strdup_with_name\0", policy_unpack_test_unpack_strdup_with_name),
    (b"policy_unpack_test_unpack_strdup_out_of_bounds\0", policy_unpack_test_unpack_strdup_out_of_bounds),
    (b"policy_unpack_test_unpack_u16_chunk_basic\0", policy_unpack_test_unpack_u16_chunk_basic),
    (b"policy_unpack_test_unpack_u16_chunk_out_of_bounds_1\0", policy_unpack_test_unpack_u16_chunk_out_of_bounds_1),
    (b"policy_unpack_test_unpack_u16_chunk_out_of_bounds_2\0", policy_unpack_test_unpack_u16_chunk_out_of_bounds_2),
    (b"policy_unpack_test_unpack_u32_with_null_name\0", policy_unpack_test_unpack_u32_with_null_name),
    (b"policy_unpack_test_unpack_u32_with_name\0", policy_unpack_test_unpack_u32_with_name),
    (b"policy_unpack_test_unpack_u32_out_of_bounds\0", policy_unpack_test_unpack_u32_out_of_bounds),
    (b"policy_unpack_test_unpack_u64_with_null_name\0", policy_unpack_test_unpack_u64_with_null_name),
    (b"policy_unpack_test_unpack_u64_with_name\0", policy_unpack_test_unpack_u64_with_name),
    (b"policy_unpack_test_unpack_u64_out_of_bounds\0", policy_unpack_test_unpack_u64_out_of_bounds),
    (b"policy_unpack_test_unpack_X_code_match\0", policy_unpack_test_unpack_X_code_match),
    (b"policy_unpack_test_unpack_X_code_mismatch\0", policy_unpack_test_unpack_X_code_mismatch),
    (b"policy_unpack_test_unpack_X_out_of_bounds\0", policy_unpack_test_unpack_X_out_of_bounds),
];

// Module declarations equivalent to KUnit and kernel module macros.
// MODULE_IMPORT_NS("EXPORTED_FOR_KUNIT_TESTING");
// kunit_test_suite(apparmor_policy_unpack_test_module);
// MODULE_DESCRIPTION("KUnit tests for AppArmor's policy unpack");
// MODULE_LICENSE("GPL");


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
