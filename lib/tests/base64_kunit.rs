// SPDX-License-Identifier: GPL-2.0
/*
 * base64_kunit_test.c - KUnit tests for base64 encoding and decoding functions
 *
 * Copyright (c) 2025, Guan-Chun Wu <409411716@gms.tku.edu.tw>
 */

/* External kernel/KUnit declarations are supplied by the surrounding build. */

use core::ffi::{c_char, c_int, c_void};

type u8_ = u8;
type u64_ = u64;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum base64_variant {
    BASE64_STD,
    BASE64_URLSAFE,
    BASE64_IMAP,
}

#[repr(C)]
pub struct kunit;

extern "C" {
    fn ktime_get_ns() -> u64_;
    fn div64_u64(a: u64_, b: u64_) -> u64_;
    fn base64_encode(data: *const u8_, len: c_int, dst: *mut c_char, padding: bool, variant: base64_variant) -> c_int;
    fn base64_decode(data: *const c_char, len: c_int, dst: *mut u8_, padding: bool, variant: base64_variant) -> c_int;
    fn kmalloc(size: usize, flags: u32) -> *mut c_void;
    fn kfree(ptr: *mut c_void);
    fn get_random_bytes(buf: *mut c_void, len: usize);
    fn strlen(s: *const c_char) -> usize;
}

const GFP_KERNEL: u32 = 0;

unsafe fn bench_encode_ns(data: *const u8_, len: c_int, dst: *mut c_char, reps: c_int, variant: base64_variant) -> u64_ {
    let t0 = ktime_get_ns();
    for _ in 0..reps {
        base64_encode(data, len, dst, true, variant);
    }
    let t1 = ktime_get_ns();
    div64_u64(t1.wrapping_sub(t0), reps as u64_)
}

unsafe fn bench_decode_ns(data: *const c_char, len: c_int, dst: *mut u8_, reps: c_int, variant: base64_variant) -> u64_ {
    let t0 = ktime_get_ns();
    for _ in 0..reps {
        base64_decode(data, len, dst, true, variant);
    }
    let t1 = ktime_get_ns();
    div64_u64(t1.wrapping_sub(t0), reps as u64_)
}

unsafe fn run_perf_and_check(test: *mut kunit, label: *const c_char, size: c_int, variant: base64_variant) {
    let reps: c_int = 1000;
    let outlen = ((size as usize + 2) / 3) * 4;
    let input = kmalloc(size as usize, GFP_KERNEL) as *mut u8_;
    let enc = kmalloc(outlen, GFP_KERNEL) as *mut c_char;
    let decoded = kmalloc(size as usize, GFP_KERNEL) as *mut u8_;

    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, input);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, enc);
    KUNIT_ASSERT_NOT_ERR_OR_NULL!(test, decoded);

    get_random_bytes(input as *mut c_void, size as usize);
    let enc_len = base64_encode(input, size, enc, true, variant);
    let dec_len = base64_decode(enc, enc_len, decoded, true, variant);

    /* correctness sanity check */
    KUNIT_EXPECT_EQ!(test, dec_len, size);
    KUNIT_EXPECT_MEMEQ!(test, decoded, input, size as usize);

    /* benchmark encode */
    let t1 = bench_encode_ns(input, size, enc, reps, variant);
    kunit_info!(test, "[%s] encode run : %lluns", label, t1);
    let t2 = bench_decode_ns(enc, enc_len, decoded, reps, variant);
    kunit_info!(test, "[%s] decode run : %lluns", label, t2);

    kfree(input as *mut c_void);
    kfree(enc as *mut c_void);
    kfree(decoded as *mut c_void);
}

unsafe fn base64_performance_tests(test: *mut kunit) {
    /* run on STD variant only */
    run_perf_and_check(test, b"64B\0".as_ptr() as *const c_char, 64, base64_variant::BASE64_STD);
    run_perf_and_check(test, b"1KB\0".as_ptr() as *const c_char, 1024, base64_variant::BASE64_STD);
}

unsafe fn expect_encode_ok(test: *mut kunit, src: *const u8_, srclen: c_int, expected: *const c_char, padding: bool, variant: base64_variant) {
    let mut buf = [0 as c_char; 128];
    let encoded_len = base64_encode(src, srclen, buf.as_mut_ptr(), padding, variant);
    buf[encoded_len as usize] = 0;
    KUNIT_EXPECT_EQ!(test, encoded_len as usize, strlen(expected));
    KUNIT_EXPECT_STREQ!(test, buf.as_ptr(), expected);
}

unsafe fn expect_decode_ok(test: *mut kunit, src: *const c_char, expected: *const u8_, expected_len: c_int, padding: bool, variant: base64_variant) {
    let mut buf = [0 as u8_; 128];
    let decoded_len = base64_decode(src, strlen(src) as c_int, buf.as_mut_ptr(), padding, variant);
    KUNIT_EXPECT_EQ!(test, decoded_len, expected_len);
    KUNIT_EXPECT_MEMEQ!(test, buf.as_ptr(), expected, expected_len as usize);
}

unsafe fn expect_decode_err(test: *mut kunit, src: *const c_char, srclen: c_int, padding: bool, variant: base64_variant) {
    let mut buf = [0 as u8_; 64];
    let decoded_len = base64_decode(src, srclen, buf.as_mut_ptr(), padding, variant);
    KUNIT_EXPECT_EQ!(test, decoded_len, -1);
}

unsafe fn base64_std_encode_tests(test: *mut kunit) {
    let s = |x: &'static [u8]| x.as_ptr() as *const c_char;
    let b = |x: &'static [u8]| x.as_ptr();
    let v = base64_variant::BASE64_STD;
    /* With padding */
    expect_encode_ok(test, b(b"\0"), 0, s(b"\0"), true, v);
    expect_encode_ok(test, b(b"f\0"), 1, s(b"Zg==\0"), true, v);
    expect_encode_ok(test, b(b"fo\0"), 2, s(b"Zm8=\0"), true, v);
    expect_encode_ok(test, b(b"foo\0"), 3, s(b"Zm9v\0"), true, v);
    expect_encode_ok(test, b(b"foob\0"), 4, s(b"Zm9vYg==\0"), true, v);
    expect_encode_ok(test, b(b"fooba\0"), 5, s(b"Zm9vYmE=\0"), true, v);
    expect_encode_ok(test, b(b"foobar\0"), 6, s(b"Zm9vYmFy\0"), true, v);
    expect_encode_ok(test, b(b"Hello, world!\0"), 13, s(b"SGVsbG8sIHdvcmxkIQ==\0"), true, v);
    expect_encode_ok(test, b(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\0"), 26, s(b"QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVo=\0"), true, v);
    expect_encode_ok(test, b(b"abcdefghijklmnopqrstuvwxyz\0"), 26, s(b"YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXo=\0"), true, v);
    expect_encode_ok(test, b(b"0123456789+/\0"), 12, s(b"MDEyMzQ1Njc4OSsv\0"), true, v);
    /* Without padding */
    expect_encode_ok(test, b(b"\0"), 0, s(b"\0"), false, v);
    expect_encode_ok(test, b(b"f\0"), 1, s(b"Zg\0"), false, v);
    expect_encode_ok(test, b(b"fo\0"), 2, s(b"Zm8\0"), false, v);
    expect_encode_ok(test, b(b"foo\0"), 3, s(b"Zm9v\0"), false, v);
    expect_encode_ok(test, b(b"foob\0"), 4, s(b"Zm9vYg\0"), false, v);
    expect_encode_ok(test, b(b"fooba\0"), 5, s(b"Zm9vYmE\0"), false, v);
    expect_encode_ok(test, b(b"foobar\0"), 6, s(b"Zm9vYmFy\0"), false, v);
    expect_encode_ok(test, b(b"Hello, world!\0"), 13, s(b"SGVsbG8sIHdvcmxkIQ\0"), false, v);
    expect_encode_ok(test, b(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\0"), 26, s(b"QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVo\0"), false, v);
    expect_encode_ok(test, b(b"abcdefghijklmnopqrstuvwxyz\0"), 26, s(b"YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXo\0"), false, v);
    expect_encode_ok(test, b(b"0123456789+/\0"), 12, s(b"MDEyMzQ1Njc4OSsv\0"), false, v);
}

unsafe fn base64_std_decode_tests(test: *mut kunit) {
    let s = |x: &'static [u8]| x.as_ptr() as *const c_char;
    let b = |x: &'static [u8]| x.as_ptr();
    let v = base64_variant::BASE64_STD;
    /* -------- With padding --------*/
    expect_decode_ok(test,s(b"\0"),b(b"\0"),0,true,v); expect_decode_ok(test,s(b"Zg==\0"),b(b"f\0"),1,true,v);
    expect_decode_ok(test,s(b"Zm8=\0"),b(b"fo\0"),2,true,v); expect_decode_ok(test,s(b"Zm9v\0"),b(b"foo\0"),3,true,v);
    expect_decode_ok(test,s(b"Zm9vYg==\0"),b(b"foob\0"),4,true,v); expect_decode_ok(test,s(b"Zm9vYmE=\0"),b(b"fooba\0"),5,true,v);
    expect_decode_ok(test,s(b"Zm9vYmFy\0"),b(b"foobar\0"),6,true,v); expect_decode_ok(test,s(b"SGVsbG8sIHdvcmxkIQ==\0"),b(b"Hello, world!\0"),13,true,v);
    expect_decode_ok(test,s(b"QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVo=\0"),b(b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\0"),26,true,v);
    expect_decode_ok(test,s(b"YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXo=\0"),b(b"abcdefghijklmnopqrstuvwxyz\0"),26,true,v);
    /* Error cases */
    for (src, len) in [(b"Zg=!",4),(b"Zm$=",4),(b"Z===",4),(b"Zg",2),(b"Zm9v====",8),(b"Zm==A",5)] { expect_decode_err(test,src.as_ptr() as *const c_char,len,true,v); }
    let with_nul = [b'Z' as c_char,b'g' as c_char,0,b'=' as c_char]; expect_decode_err(test,with_nul.as_ptr(),4,true,v);
    /* -------- Without padding --------*/
    for (src, exp, n) in [(b"",b"",0),(b"Zg",b"f",1),(b"Zm8",b"fo",2),(b"Zm9v",b"foo",3),(b"Zm9vYg",b"foob",4),(b"Zm9vYmE",b"fooba",5),(b"Zm9vYmFy",b"foobar",6),(b"TWFu",b"Man",3),(b"SGVsbG8sIHdvcmxkIQ",b"Hello, world!",13),(b"QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVo",b"ABCDEFGHIJKLMNOPQRSTUVWXYZ",26),(b"YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXo",b"abcdefghijklmnopqrstuvwxyz",26),(b"MDEyMzQ1Njc4OSsv",b"0123456789+/",12)] { expect_decode_ok(test,src.as_ptr() as *const c_char,exp.as_ptr(),n,false,v); }
    for (src, len) in [(b"Zg=!",4),(b"Zm$=",4),(b"Z===",4),(b"Zg=",3),(b"Zm9v====",8),(b"Zm==v",4)] { expect_decode_err(test,src.as_ptr() as *const c_char,len,false,v); }
    let with_nul = [b'Z' as c_char,b'g' as c_char,0,b'=' as c_char]; expect_decode_err(test,with_nul.as_ptr(),4,false,v);
}

unsafe fn base64_variant_tests(test: *mut kunit) {
    let sample1: [u8; 5] = [0x00,0xfb,0xff,0x7f,0x80]; let mut std_buf=[0 as c_char;128]; let mut url_buf=[0 as c_char;128]; let mut imap_buf=[0 as c_char;128]; let mut back=[0u8;128];
    let mut n_std=base64_encode(sample1.as_ptr(),5,std_buf.as_mut_ptr(),false,base64_variant::BASE64_STD); let n_url=base64_encode(sample1.as_ptr(),5,url_buf.as_mut_ptr(),false,base64_variant::BASE64_URLSAFE); std_buf[n_std as usize]=0; url_buf[n_url as usize]=0;
    for i in 0..n_std as usize { if std_buf[i]==b'+' as c_char {std_buf[i]=b'-' as c_char;} else if std_buf[i]==b'/' as c_char {std_buf[i]=b'_' as c_char;} } KUNIT_EXPECT_STREQ!(test,std_buf.as_ptr(),url_buf.as_ptr());
    let mut m=base64_decode(url_buf.as_ptr(),n_url,back.as_mut_ptr(),false,base64_variant::BASE64_URLSAFE); KUNIT_EXPECT_EQ!(test,m,5); KUNIT_EXPECT_MEMEQ!(test,back.as_ptr(),sample1.as_ptr(),5);
    n_std=base64_encode(sample1.as_ptr(),5,std_buf.as_mut_ptr(),false,base64_variant::BASE64_STD); let n_imap=base64_encode(sample1.as_ptr(),5,imap_buf.as_mut_ptr(),false,base64_variant::BASE64_IMAP); std_buf[n_std as usize]=0; imap_buf[n_imap as usize]=0; for i in 0..n_std as usize {if std_buf[i]==b'/' as c_char {std_buf[i]=b',' as c_char;}} KUNIT_EXPECT_STREQ!(test,std_buf.as_ptr(),imap_buf.as_ptr());
    m=base64_decode(imap_buf.as_ptr(),n_imap,back.as_mut_ptr(),false,base64_variant::BASE64_IMAP); KUNIT_EXPECT_EQ!(test,m,5); KUNIT_EXPECT_MEMEQ!(test,back.as_ptr(),sample1.as_ptr(),5);
    let bad=b"Zg==\0"; let mut tmp=[0u8;8]; m=base64_decode(bad.as_ptr() as *const c_char,4,tmp.as_mut_ptr(),false,base64_variant::BASE64_URLSAFE); KUNIT_EXPECT_EQ!(test,m,-1); m=base64_decode(bad.as_ptr() as *const c_char,4,tmp.as_mut_ptr(),false,base64_variant::BASE64_IMAP); KUNIT_EXPECT_EQ!(test,m,-1);
}

/* ---------- Test registration ---------- */
static mut base64_test_cases: [kunit_case; 5] = [KUNIT_CASE!(base64_performance_tests), KUNIT_CASE!(base64_std_encode_tests), KUNIT_CASE!(base64_std_decode_tests), KUNIT_CASE!(base64_variant_tests), KUNIT_CASE_NONE!()];
static mut base64_test_suite: kunit_suite = kunit_suite { name: b"base64\0".as_ptr() as *const c_char, test_cases: base64_test_cases.as_mut_ptr() };

kunit_test_suite!(base64_test_suite);

MODULE_AUTHOR!("Guan-Chun Wu <409411716@gms.tku.edu.tw>");
MODULE_DESCRIPTION!("KUnit tests for Base64 encoding/decoding, including performance checks");
MODULE_LICENSE!("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
