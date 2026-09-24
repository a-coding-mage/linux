// SPDX-License-Identifier: GPL-2.0
/*
 * base64_kunit_test.c - KUnit tests for base64 encoding and decoding functions
 *
 * Copyright (c) 2025, Guan-Chun Wu <409411716@gms.tku.edu.tw>
 */
//! Original four Base64 KUnit cases, including the original timed workloads.

use core::{alloc::Layout, ffi::CStr, mem::MaybeUninit, ptr};
use kernel::{
    alloc::{allocator::Kmalloc, flags::GFP_KERNEL, Allocator, NumaNode},
    bindings, ffi,
};

type Variant = bindings::base64_variant;

// These local macros keep the C formatter kind and operand spelling. Raw test
// access deliberately remains in each caller's explicit unsafe context.
macro_rules! location {
    ($test:expr) => {{
        struct Location(bindings::kunit_loc);
        // SAFETY: The pointer names an immutable static C string.
        unsafe impl Sync for Location {}
        static LOCATION: Location = Location(bindings::kunit_loc {
            file: concat!(file!(), "\0").as_ptr().cast(),
            line: line!() as ffi::c_int,
        });
        kernel::sync::atomic::atomic_store(
            ptr::addr_of_mut!((*$test).last_seen.file),
            LOCATION.0.file,
            kernel::sync::atomic::Relaxed,
        );
        kernel::sync::atomic::atomic_store(
            ptr::addr_of_mut!((*$test).last_seen.line),
            LOCATION.0.line,
            kernel::sync::atomic::Relaxed,
        );
        ptr::addr_of!(LOCATION.0)
    }};
}

macro_rules! text {
    ($left:expr, $right:expr) => {{
        struct Text(bindings::kunit_binary_assert_text);
        // SAFETY: The formatter never modifies these static C strings.
        unsafe impl Sync for Text {}
        static TEXT: Text = Text(bindings::kunit_binary_assert_text {
            operation: c"==".as_ptr().cast(),
            left_text: $left.as_ptr().cast(),
            right_text: $right.as_ptr().cast(),
        });
        ptr::addr_of!(TEXT.0)
    }};
}

macro_rules! expect_eq {
    ($test:expr, $left:expr, $right:expr, $left_text:expr, $right_text:expr) => {{
        let test = $test;
        let left = $left;
        let right = $right;
        let loc = location!(test);
        // All unsigned operands in this original suite are lengths <=1024;
        // this comparison also preserves negative native int error results.
        if left as ffi::c_longlong != right as ffi::c_longlong {
            let assertion = bindings::kunit_binary_assert {
                assert: bindings::kunit_assert {},
                text: text!($left_text, $right_text),
                left_value: left as ffi::c_longlong,
                right_value: right as ffi::c_longlong,
            };
            bindings::__kunit_do_failed_assertion(
                test,
                loc,
                bindings::kunit_assert_type_KUNIT_EXPECTATION,
                ptr::addr_of!(assertion.assert),
                Some(bindings::kunit_binary_assert_format),
                ptr::null(),
            );
        }
    }};
}

fn is_err_or_null(value: *const ffi::c_void) -> bool {
    value.is_null() || value as usize >= usize::MAX - (bindings::MAX_ERRNO as usize - 1)
}

macro_rules! assert_pointer {
    ($test:expr, $pointer:expr, $name:expr) => {{
        let test = $test;
        let pointer = $pointer;
        let loc = location!(test);
        if is_err_or_null(pointer.cast()) {
            let assertion = bindings::kunit_ptr_not_err_assert {
                assert: bindings::kunit_assert {},
                text: $name.as_ptr().cast(),
                value: pointer.cast(),
            };
            bindings::__kunit_do_failed_assertion(
                test,
                loc,
                bindings::kunit_assert_type_KUNIT_ASSERTION,
                ptr::addr_of!(assertion.assert),
                Some(bindings::kunit_ptr_not_err_assert_format),
                ptr::null(),
            );
            bindings::__kunit_abort(test);
        }
    }};
}

macro_rules! expect_string {
    ($test:expr, $left:expr, $right:expr, $left_text:expr, $right_text:expr) => {{
        let test = $test;
        let left: *const ffi::c_char = $left;
        let right: *const ffi::c_char = $right;
        let loc = location!(test);
        if is_err_or_null(left.cast())
            || is_err_or_null(right.cast())
            || bindings::strcmp(left, right) != 0
        {
            let assertion = bindings::kunit_binary_str_assert {
                assert: bindings::kunit_assert {},
                text: text!($left_text, $right_text),
                left_value: left,
                right_value: right,
            };
            bindings::__kunit_do_failed_assertion(
                test,
                loc,
                bindings::kunit_assert_type_KUNIT_EXPECTATION,
                ptr::addr_of!(assertion.assert),
                Some(bindings::kunit_binary_str_assert_format),
                ptr::null(),
            );
        }
    }};
}

macro_rules! expect_memory {
    ($test:expr, $left:expr, $right:expr, $size:expr, $left_text:expr, $right_text:expr) => {{
        let test = $test;
        let left: *const ffi::c_void = $left.cast();
        let right: *const ffi::c_void = $right.cast();
        let size = $size;
        let loc = location!(test);
        if left.is_null() || right.is_null() || bindings::memcmp(left, right, size) != 0 {
            let assertion = bindings::kunit_mem_assert {
                assert: bindings::kunit_assert {},
                text: text!($left_text, $right_text),
                left_value: left,
                right_value: right,
                size,
            };
            bindings::__kunit_do_failed_assertion(
                test,
                loc,
                bindings::kunit_assert_type_KUNIT_EXPECTATION,
                ptr::addr_of!(assertion.assert),
                Some(bindings::kunit_mem_assert_format),
                ptr::null(),
            );
        }
    }};
}

#[cfg(CONFIG_PRINTK_INDEX)]
#[repr(transparent)]
struct PrintkEntry(bindings::pi_entry);
// SAFETY: Only immutable static strings or NULL are referenced by this record.
#[cfg(CONFIG_PRINTK_INDEX)]
unsafe impl Sync for PrintkEntry {}
#[cfg(CONFIG_PRINTK_INDEX)]
#[repr(transparent)]
struct PrintkPointer {
    // Read by the kernel's printk index walker, not by this Rust crate.
    _entry: *const bindings::pi_entry,
}
// SAFETY: The pointer names an immutable static PrintkEntry.
#[cfg(CONFIG_PRINTK_INDEX)]
unsafe impl Sync for PrintkPointer {}

macro_rules! benchmark_log {
    ($test:expr, $operation:literal, $label:expr, $time:expr) => {{
        const FORMAT: &CStr =
            kernel::c_str!(concat!("    # %s: [%s] ", $operation, " run : %lluns"));
        const FULL: &CStr =
            kernel::c_str!(concat!("\x016    # %s: [%s] ", $operation, " run : %lluns"));
        #[cfg(CONFIG_PRINTK_INDEX)]
        {
            #[used]
            static ENTRY: PrintkEntry = PrintkEntry(bindings::pi_entry {
                fmt: FULL.as_ptr().cast(),
                func: c"run_perf_and_check".as_ptr().cast(),
                file: concat!(file!(), "\0").as_ptr().cast(),
                line: line!(),
                level: ptr::null(),
                subsys_fmt_prefix: ptr::null(),
            });
            #[used]
            #[link_section = ".printk_index"]
            static POINTER: PrintkPointer = PrintkPointer { _entry: &ENTRY.0 };
        }
        let name = ptr::addr_of!((*$test).name).read();
        #[cfg(CONFIG_PRINTK)]
        bindings::_printk(FULL.as_ptr().cast(), name, $label, $time);
        #[cfg(not(any(CONFIG_PRINTK, CONFIG_PRINTK_INDEX)))]
        let _ = FULL;
        bindings::kunit_log_append(
            ptr::addr_of!((*$test).log).read(),
            FORMAT.as_ptr().cast(),
            name,
            $label,
            $time,
        );
    }};
}

// This established allocator reaches krealloc_node_align(NULL,size,1,GFP_KERNEL,
// NUMA_NO_NODE): slab.h documents that NULL behaves exactly like kmalloc. The
// existing helper retains real alloc_hooks and partition-token handling.
unsafe fn allocate(size: usize) -> *mut u8 {
    // SAFETY: Callers request only the original positive sizes64,88,1024,1368.
    let layout = unsafe { Layout::from_size_align_unchecked(size, 1) };
    Kmalloc::alloc(layout, GFP_KERNEL, NumaNode::NO_NODE)
        .map_or(ptr::null_mut(), |memory| memory.as_ptr().cast())
}

unsafe fn bench_encode_ns(
    data: *const u8,
    len: i32,
    dst: *mut ffi::c_char,
    reps: i32,
    variant: Variant,
) -> u64 {
    // SAFETY: Live buffers satisfy the native encoder; reps is the original1000.
    unsafe {
        let start = bindings::ktime_get() as u64;
        for _ in 0..reps {
            bindings::base64_encode(data, len, dst, true, variant);
        }
        let end = bindings::ktime_get() as u64;
        kernel::math::div64_u64(end.wrapping_sub(start), reps as u64).unwrap_unchecked()
    }
}

unsafe fn bench_decode_ns(
    data: *const ffi::c_char,
    len: i32,
    dst: *mut u8,
    reps: i32,
    variant: Variant,
) -> u64 {
    // SAFETY: As for encoding; original1000 is nonzero and buffers remain live.
    unsafe {
        let start = bindings::ktime_get() as u64;
        for _ in 0..reps {
            bindings::base64_decode(data, len, dst, true, variant);
        }
        let end = bindings::ktime_get() as u64;
        kernel::math::div64_u64(end.wrapping_sub(start), reps as u64).unwrap_unchecked()
    }
}

unsafe fn run_perf_and_check(
    test: *mut bindings::kunit,
    label: &CStr,
    size: i32,
    variant: Variant,
) {
    // SAFETY: KUnit provides the live context; sizes are the two original
    // benchmark sizes. Keep all three attempts before any fatal assertion.
    // Only raw pointers span abort: no Rust guard/destructor would be skipped.
    // As in C, earlier successful allocations are not freed after fatal OOM.
    unsafe {
        let outlen = ((size as usize + 2) / 3) * 4;
        let input = allocate(size as usize);
        let enc = allocate(outlen).cast::<ffi::c_char>();
        let decoded = allocate(size as usize);
        assert_pointer!(test, input, c"in");
        assert_pointer!(test, enc, c"enc");
        assert_pointer!(test, decoded, c"decoded");
        bindings::get_random_bytes(input.cast(), size as usize);
        let enc_len = bindings::base64_encode(input, size, enc, true, variant);
        let dec_len = bindings::base64_decode(enc, enc_len, decoded, true, variant);
        expect_eq!(test, dec_len, size, c"dec_len", c"size");
        expect_memory!(test, decoded, input, size as usize, c"decoded", c"in");
        let time = bench_encode_ns(input, size, enc, 1000, variant);
        benchmark_log!(test, "encode", label.as_ptr().cast::<ffi::c_char>(), time);
        let time = bench_decode_ns(enc, enc_len, decoded, 1000, variant);
        benchmark_log!(test, "decode", label.as_ptr().cast::<ffi::c_char>(), time);
        bindings::kfree(input.cast());
        bindings::kfree(enc.cast());
        bindings::kfree(decoded.cast());
    }
}

unsafe extern "C" fn base64_performance_tests(test: *mut bindings::kunit) {
    // SAFETY: The framework owns the context; both sizes and labels are original.
    unsafe {
        run_perf_and_check(test, c"64B", 64, Variant::BASE64_STD);
        run_perf_and_check(test, c"1KB", 1024, Variant::BASE64_STD);
    }
}

unsafe fn expect_encode_ok(
    test: *mut bindings::kunit,
    src: &CStr,
    srclen: i32,
    expected: &CStr,
    padding: bool,
    variant: Variant,
) {
    let mut buffer = MaybeUninit::<[ffi::c_char; 128]>::uninit();
    let buf = buffer.as_mut_ptr().cast::<ffi::c_char>();
    // SAFETY: Every original vector fits128 bytes, including its NUL. The
    // selected native encoder initializes its result before string comparison.
    unsafe {
        let encoded_len =
            bindings::base64_encode(src.as_ptr().cast(), srclen, buf, padding, variant);
        buf.add(encoded_len as usize).write(0);
        expect_eq!(
            test,
            encoded_len,
            bindings::strlen(expected.as_ptr().cast()),
            c"encoded_len",
            c"strlen(expected)"
        );
        expect_string!(test, buf, expected.as_ptr().cast(), c"buf", c"expected");
    }
}

unsafe fn expect_decode_ok(
    test: *mut bindings::kunit,
    src: &CStr,
    expected: &CStr,
    expected_len: i32,
    padding: bool,
    variant: Variant,
) {
    let mut buffer = MaybeUninit::<[u8; 128]>::uninit();
    let buf = buffer.as_mut_ptr().cast::<u8>();
    // SAFETY: Each original vector's expected initialized result fits128 bytes.
    unsafe {
        let decoded_len = bindings::base64_decode(
            src.as_ptr().cast(),
            bindings::strlen(src.as_ptr().cast()) as i32,
            buf,
            padding,
            variant,
        );
        expect_eq!(
            test,
            decoded_len,
            expected_len,
            c"decoded_len",
            c"expected_len"
        );
        expect_memory!(
            test,
            buf,
            expected.as_ptr(),
            expected_len as usize,
            c"buf",
            c"expected"
        );
    }
}

unsafe fn expect_decode_err(
    test: *mut bindings::kunit,
    src: *const ffi::c_char,
    srclen: i32,
    padding: bool,
    variant: Variant,
) {
    let mut buffer = MaybeUninit::<[u8; 64]>::uninit();
    // SAFETY: Original error inputs specify their exact readable extents, and
    // no output byte is read after the failing call (matching the C suite).
    unsafe {
        let decoded_len =
            bindings::base64_decode(src, srclen, buffer.as_mut_ptr().cast(), padding, variant);
        expect_eq!(test, decoded_len, -1, c"decoded_len", c"-1");
    }
}

unsafe extern "C" fn base64_std_encode_tests(test: *mut bindings::kunit) {
    let variant = Variant::BASE64_STD;
    // SAFETY: Exact original vectors and their declared lengths.
    unsafe {
        expect_encode_ok(test, c"", 0, c"", true, variant);
        expect_encode_ok(test, c"f", 1, c"Zg==", true, variant);
        expect_encode_ok(test, c"fo", 2, c"Zm8=", true, variant);
        expect_encode_ok(test, c"foo", 3, c"Zm9v", true, variant);
        expect_encode_ok(test, c"foob", 4, c"Zm9vYg==", true, variant);
        expect_encode_ok(test, c"fooba", 5, c"Zm9vYmE=", true, variant);
        expect_encode_ok(test, c"foobar", 6, c"Zm9vYmFy", true, variant);
        expect_encode_ok(
            test,
            c"Hello, world!",
            13,
            c"SGVsbG8sIHdvcmxkIQ==",
            true,
            variant,
        );
        expect_encode_ok(
            test,
            c"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            26,
            c"QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVo=",
            true,
            variant,
        );
        expect_encode_ok(
            test,
            c"abcdefghijklmnopqrstuvwxyz",
            26,
            c"YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXo=",
            true,
            variant,
        );
        expect_encode_ok(
            test,
            c"0123456789+/",
            12,
            c"MDEyMzQ1Njc4OSsv",
            true,
            variant,
        );
        expect_encode_ok(test, c"", 0, c"", false, variant);
        expect_encode_ok(test, c"f", 1, c"Zg", false, variant);
        expect_encode_ok(test, c"fo", 2, c"Zm8", false, variant);
        expect_encode_ok(test, c"foo", 3, c"Zm9v", false, variant);
        expect_encode_ok(test, c"foob", 4, c"Zm9vYg", false, variant);
        expect_encode_ok(test, c"fooba", 5, c"Zm9vYmE", false, variant);
        expect_encode_ok(test, c"foobar", 6, c"Zm9vYmFy", false, variant);
        expect_encode_ok(
            test,
            c"Hello, world!",
            13,
            c"SGVsbG8sIHdvcmxkIQ",
            false,
            variant,
        );
        expect_encode_ok(
            test,
            c"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            26,
            c"QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVo",
            false,
            variant,
        );
        expect_encode_ok(
            test,
            c"abcdefghijklmnopqrstuvwxyz",
            26,
            c"YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXo",
            false,
            variant,
        );
        expect_encode_ok(
            test,
            c"0123456789+/",
            12,
            c"MDEyMzQ1Njc4OSsv",
            false,
            variant,
        );
    }
}
unsafe extern "C" fn base64_std_decode_tests(test: *mut bindings::kunit) {
    let variant = Variant::BASE64_STD;
    // SAFETY: Exact original valid/error vectors, including embedded NULs.
    unsafe {
        expect_decode_ok(test, c"", c"", 0, true, variant);
        expect_decode_ok(test, c"Zg==", c"f", 1, true, variant);
        expect_decode_ok(test, c"Zm8=", c"fo", 2, true, variant);
        expect_decode_ok(test, c"Zm9v", c"foo", 3, true, variant);
        expect_decode_ok(test, c"Zm9vYg==", c"foob", 4, true, variant);
        expect_decode_ok(test, c"Zm9vYmE=", c"fooba", 5, true, variant);
        expect_decode_ok(test, c"Zm9vYmFy", c"foobar", 6, true, variant);
        expect_decode_ok(
            test,
            c"SGVsbG8sIHdvcmxkIQ==",
            c"Hello, world!",
            13,
            true,
            variant,
        );
        expect_decode_ok(
            test,
            c"QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVo=",
            c"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            26,
            true,
            variant,
        );
        expect_decode_ok(
            test,
            c"YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXo=",
            c"abcdefghijklmnopqrstuvwxyz",
            26,
            true,
            variant,
        );
        expect_decode_err(test, c"Zg=!".as_ptr().cast(), 4, true, variant);
        expect_decode_err(test, c"Zm$=".as_ptr().cast(), 4, true, variant);
        expect_decode_err(test, c"Z===".as_ptr().cast(), 4, true, variant);
        expect_decode_err(test, c"Zg".as_ptr().cast(), 2, true, variant);
        expect_decode_err(test, c"Zm9v====".as_ptr().cast(), 8, true, variant);
        expect_decode_err(test, c"Zm==A".as_ptr().cast(), 5, true, variant);
        expect_decode_err(
            test,
            [b'Z', b'g', 0, b'='].as_ptr().cast(),
            4,
            true,
            variant,
        );
        expect_decode_ok(test, c"", c"", 0, false, variant);
        expect_decode_ok(test, c"Zg", c"f", 1, false, variant);
        expect_decode_ok(test, c"Zm8", c"fo", 2, false, variant);
        expect_decode_ok(test, c"Zm9v", c"foo", 3, false, variant);
        expect_decode_ok(test, c"Zm9vYg", c"foob", 4, false, variant);
        expect_decode_ok(test, c"Zm9vYmE", c"fooba", 5, false, variant);
        expect_decode_ok(test, c"Zm9vYmFy", c"foobar", 6, false, variant);
        expect_decode_ok(test, c"TWFu", c"Man", 3, false, variant);
        expect_decode_ok(
            test,
            c"SGVsbG8sIHdvcmxkIQ",
            c"Hello, world!",
            13,
            false,
            variant,
        );
        expect_decode_ok(
            test,
            c"QUJDREVGR0hJSktMTU5PUFFSU1RVVldYWVo",
            c"ABCDEFGHIJKLMNOPQRSTUVWXYZ",
            26,
            false,
            variant,
        );
        expect_decode_ok(
            test,
            c"YWJjZGVmZ2hpamtsbW5vcHFyc3R1dnd4eXo",
            c"abcdefghijklmnopqrstuvwxyz",
            26,
            false,
            variant,
        );
        expect_decode_ok(
            test,
            c"MDEyMzQ1Njc4OSsv",
            c"0123456789+/",
            12,
            false,
            variant,
        );
        expect_decode_err(test, c"Zg=!".as_ptr().cast(), 4, false, variant);
        expect_decode_err(test, c"Zm$=".as_ptr().cast(), 4, false, variant);
        expect_decode_err(test, c"Z===".as_ptr().cast(), 4, false, variant);
        expect_decode_err(test, c"Zg=".as_ptr().cast(), 3, false, variant);
        expect_decode_err(test, c"Zm9v====".as_ptr().cast(), 8, false, variant);
        expect_decode_err(test, c"Zm==v".as_ptr().cast(), 4, false, variant);
        expect_decode_err(
            test,
            [b'Z', b'g', 0, b'='].as_ptr().cast(),
            4,
            false,
            variant,
        );
    }
}

unsafe extern "C" fn base64_variant_tests(test: *mut bindings::kunit) {
    let sample1 = [0x00, 0xfb, 0xff, 0x7f, 0x80];
    let mut std_storage = MaybeUninit::<[ffi::c_char; 128]>::uninit();
    let mut url_storage = MaybeUninit::<[ffi::c_char; 128]>::uninit();
    let mut imap_storage = MaybeUninit::<[ffi::c_char; 128]>::uninit();
    let mut back_storage = MaybeUninit::<[u8; 128]>::uninit();
    let std_buf = std_storage.as_mut_ptr().cast::<ffi::c_char>();
    let url_buf = url_storage.as_mut_ptr().cast::<ffi::c_char>();
    let imap_buf = imap_storage.as_mut_ptr().cast::<ffi::c_char>();
    let back = back_storage.as_mut_ptr().cast::<u8>();
    // SAFETY: Actual native functions initialize precisely the bytes consumed
    // below; every original vector and its terminator fits the local storage.
    unsafe {
        let n_std =
            bindings::base64_encode(sample1.as_ptr(), 5, std_buf, false, Variant::BASE64_STD);
        let n_url =
            bindings::base64_encode(sample1.as_ptr(), 5, url_buf, false, Variant::BASE64_URLSAFE);
        std_buf.add(n_std as usize).write(0);
        url_buf.add(n_url as usize).write(0);
        for i in 0..n_std {
            let byte = std_buf.add(i as usize);
            if byte.read() == b'+' {
                byte.write(b'-');
            } else if byte.read() == b'/' {
                byte.write(b'_');
            }
        }
        expect_string!(test, std_buf, url_buf, c"std_buf", c"url_buf");
        let m = bindings::base64_decode(url_buf, n_url, back, false, Variant::BASE64_URLSAFE);
        expect_eq!(test, m, sample1.len(), c"m", c"(int)sizeof(sample1)");
        expect_memory!(
            test,
            back,
            sample1.as_ptr(),
            sample1.len(),
            c"back",
            c"sample1"
        );
        let n_std =
            bindings::base64_encode(sample1.as_ptr(), 5, std_buf, false, Variant::BASE64_STD);
        let n_imap =
            bindings::base64_encode(sample1.as_ptr(), 5, imap_buf, false, Variant::BASE64_IMAP);
        std_buf.add(n_std as usize).write(0);
        imap_buf.add(n_imap as usize).write(0);
        for i in 0..n_std {
            let byte = std_buf.add(i as usize);
            if byte.read() == b'/' {
                byte.write(b',');
            }
        }
        expect_string!(test, std_buf, imap_buf, c"std_buf", c"imap_buf");
        let m = bindings::base64_decode(imap_buf, n_imap, back, false, Variant::BASE64_IMAP);
        expect_eq!(test, m, sample1.len(), c"m", c"(int)sizeof(sample1)");
        expect_memory!(
            test,
            back,
            sample1.as_ptr(),
            sample1.len(),
            c"back",
            c"sample1"
        );
        let bad = c"Zg==";
        let mut tmp = MaybeUninit::<[u8; 8]>::uninit();
        let m = bindings::base64_decode(
            bad.as_ptr().cast(),
            bindings::strlen(bad.as_ptr().cast()) as i32,
            tmp.as_mut_ptr().cast(),
            false,
            Variant::BASE64_URLSAFE,
        );
        expect_eq!(test, m, -1, c"m", c"-1");
        let m = bindings::base64_decode(
            bad.as_ptr().cast(),
            bindings::strlen(bad.as_ptr().cast()) as i32,
            tmp.as_mut_ptr().cast(),
            false,
            Variant::BASE64_IMAP,
        );
        expect_eq!(test, m, -1, c"m", c"-1");
    }
}

const fn case(
    name: &'static kernel::str::CStr,
    run: unsafe extern "C" fn(*mut bindings::kunit),
) -> bindings::kunit_case {
    let mut result = kernel::kunit::kunit_case(name, run);
    result.module_name = c"base64_kunit".as_ptr().cast_mut().cast();
    result.attr.speed = bindings::kunit_speed_KUNIT_SPEED_UNSET;
    result
}
static mut TEST_CASES: [bindings::kunit_case; 5] = [
    case(c"base64_performance_tests", base64_performance_tests),
    case(c"base64_std_encode_tests", base64_std_encode_tests),
    case(c"base64_std_decode_tests", base64_std_decode_tests),
    case(c"base64_variant_tests", base64_variant_tests),
    // SAFETY: The original case array terminates in an all-zero entry.
    unsafe { core::mem::zeroed() },
];
// SAFETY: All cases and callbacks remain live for the registered suite.
kernel::kunit_unsafe_test_suite!("base64", TEST_CASES);

#[cfg(MODULE)]
const MODINFO: &str = concat!(
    "author=Guan-Chun Wu <409411716@gms.tku.edu.tw>\0",
    "description=KUnit tests for Base64 encoding/decoding, including performance checks\0",
    "license=GPL\0",
);
#[cfg(not(MODULE))]
const MODINFO: &str = concat!(
    "base64_kunit.author=Guan-Chun Wu <409411716@gms.tku.edu.tw>\0",
    "base64_kunit.description=KUnit tests for Base64 encoding/decoding, including performance checks\0",
    "base64_kunit.license=GPL\0",
    "base64_kunit.file=", env!("RUST_MODFILE"), "\0",
);
#[used]
#[link_section = ".modinfo"]
static MODULE_INFO: [u8; MODINFO.len()] = {
    let mut bytes = [0; MODINFO.len()];
    let mut index = 0;
    while index < bytes.len() {
        bytes[index] = MODINFO.as_bytes()[index];
        index += 1;
    }
    bytes
};
#[cfg(MODULE)]
#[used]
static __IS_RUST_MODULE: () = ();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
