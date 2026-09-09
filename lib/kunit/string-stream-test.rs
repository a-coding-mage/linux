// SPDX-License-Identifier: GPL-2.0
/*
 * KUnit test for struct string_stream.
 *
 * Copyright (C) 2019, Google LLC.
 * Author: Brendan Higgins <brendanhiggins@google.com>
 */

// External kernel/KUnit headers and "string-stream.h" are supplied by dependencies.

#[repr(C)]
pub struct string_stream_test_priv {
    /* For testing resource-managed free. */
    pub expected_free_stream: *mut string_stream,
    pub stream_was_freed: bool,
    pub stream_free_again: bool,
}

// Avoids a cast warning if kfree() is passed direct to kunit_add_action().
// KUNIT_DEFINE_ACTION_WRAPPER(kfree_wrapper, kfree, const void *);
// Avoids a cast warning if string_stream_destroy() is passed direct to kunit_add_action().
// KUNIT_DEFINE_ACTION_WRAPPER(cleanup_raw_stream, string_stream_destroy, struct string_stream *);

unsafe fn get_concatenated_string(test: *mut kunit, stream: *mut string_stream) -> *mut core::ffi::c_char {
    let str_ = string_stream_get_string(stream);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, str_);
    kunit_add_action(test, kfree_wrapper, str_ as *mut core::ffi::c_void);
    str_
}

/* Managed string_stream object is initialized correctly. */
unsafe fn string_stream_managed_init_test(test: *mut kunit) {
    let stream = kunit_alloc_string_stream(test, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, stream);
    KUNIT_EXPECT_EQ(test, (*stream).length, 0);
    KUNIT_EXPECT_TRUE(test, list_empty(&mut (*stream).fragments));
    KUNIT_EXPECT_TRUE(test, (*stream).gfp == GFP_KERNEL);
    KUNIT_EXPECT_FALSE(test, (*stream).append_newlines);
    KUNIT_EXPECT_TRUE(test, string_stream_is_empty(stream));
}

/* Unmanaged string_stream object is initialized correctly. */
unsafe fn string_stream_unmanaged_init_test(test: *mut kunit) {
    let stream = alloc_string_stream(GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, stream);
    kunit_add_action(test, cleanup_raw_stream, stream);
    KUNIT_EXPECT_EQ(test, (*stream).length, 0);
    KUNIT_EXPECT_TRUE(test, list_empty(&mut (*stream).fragments));
    KUNIT_EXPECT_TRUE(test, (*stream).gfp == GFP_KERNEL);
    KUNIT_EXPECT_FALSE(test, (*stream).append_newlines);
    KUNIT_EXPECT_TRUE(test, string_stream_is_empty(stream));
}

unsafe fn string_stream_destroy_stub(stream: *mut string_stream) {
    let fake_test = kunit_get_current_test();
    let priv_ = (*fake_test).priv_ as *mut string_stream_test_priv;
    if stream == (*priv_).expected_free_stream {
        if (*priv_).stream_was_freed { (*priv_).stream_free_again = true; }
        else { (*priv_).stream_was_freed = true; }
    }
    /* Calling string_stream_destroy() will only call this function again because the redirection stub is still active. */
    string_stream_clear(stream);
    kfree(stream as *const core::ffi::c_void);
}

/* kunit_free_string_stream() calls string_stream_desrtoy() */
unsafe fn string_stream_managed_free_test(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut string_stream_test_priv;
    (*priv_).expected_free_stream = core::ptr::null_mut();
    (*priv_).stream_was_freed = false;
    (*priv_).stream_free_again = false;
    kunit_activate_static_stub(test, string_stream_destroy, string_stream_destroy_stub);
    (*priv_).expected_free_stream = kunit_alloc_string_stream(test, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, (*priv_).expected_free_stream);
    kunit_free_string_stream(test, (*priv_).expected_free_stream);
    KUNIT_EXPECT_TRUE(test, (*priv_).stream_was_freed);
    KUNIT_EXPECT_FALSE(test, (*priv_).stream_free_again);
}

/* The remaining test bodies preserve the C test operations and depend on the external kernel/KUnit API. */
unsafe fn string_stream_resource_free_test(test: *mut kunit) {
    let priv_ = (*test).priv_ as *mut string_stream_test_priv;
    let fake_test = kunit_kzalloc(test, core::mem::size_of::<kunit>(), GFP_KERNEL) as *mut kunit;
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, fake_test);
    kunit_init_test(fake_test, b"string_stream_fake_test\0".as_ptr() as *const _, core::ptr::null_mut());
    (*fake_test).priv_ = priv_ as *mut _;
    (*priv_).expected_free_stream = core::ptr::null_mut();
    (*priv_).stream_was_freed = false;
    (*priv_).stream_free_again = false;
    kunit_activate_static_stub(fake_test, string_stream_destroy, string_stream_destroy_stub);
    (*priv_).expected_free_stream = kunit_alloc_string_stream(fake_test, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, (*priv_).expected_free_stream);
    (*current).kunit_test = fake_test;
    kunit_cleanup(fake_test);
    (*current).kunit_test = test;
    KUNIT_EXPECT_TRUE(test, (*priv_).stream_was_freed);
    KUNIT_EXPECT_FALSE(test, (*priv_).stream_free_again);
}

/* Add a series of lines to a string_stream. Check that all lines appear in the correct order and no characters are dropped. */
unsafe fn string_stream_line_add_test(test: *mut kunit) {
    let stream = kunit_alloc_string_stream(test, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, stream);
    let mut line = [0i8; 60];
    let mut total_len: usize = 0;
    for i in 0..100 {
        let len = snprintf(line.as_mut_ptr(), line.len(), b"The quick brown fox jumps over the lazy penguin %d\n\0".as_ptr() as *const _, i);
        KUNIT_ASSERT_LT(test, len, line.len());
        string_stream_add(stream, line.as_ptr());
        total_len += len;
    }
    let concat_string = get_concatenated_string(test, stream);
    KUNIT_EXPECT_NOT_ERR_OR_NULL(test, concat_string);
    KUNIT_EXPECT_EQ(test, strlen(concat_string), total_len);
    let mut pos = concat_string;
    for i in 0..100 {
        let string_end = strchr(pos, b'\n' as i32);
        KUNIT_EXPECT_NOT_NULL(test, string_end);
        *string_end = 0;
        snprintf(line.as_mut_ptr(), line.len(), b"The quick brown fox jumps over the lazy penguin %d\0".as_ptr() as *const _, i);
        KUNIT_EXPECT_STREQ(test, pos, line.as_ptr());
        pos = string_end.add(1);
    }
    KUNIT_EXPECT_EQ(test, strlen(pos), 0);
}

/* Add a series of lines of variable length to a string_stream. */
unsafe fn string_stream_variable_length_line_test(test: *mut kunit) {
    let line = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ 0123456789!$%^&*()_-+={}[]:;@'~#<>,.?/|\0";
    let stream = kunit_alloc_string_stream(test, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, stream);
    let mut rnd = core::mem::zeroed::<rnd_state>();
    prandom_seed_state(&mut rnd, 3141592653589793238u64);
    let mut total_len = 0usize;
    for _ in 0..100 {
        let offset = (prandom_u32_state(&mut rnd) as usize) % (line.len() - 1);
        string_stream_add(stream, b"%s\n\0".as_ptr() as *const _, line.as_ptr().add(offset));
        total_len += line.len() - offset;
    }
    let concat_string = get_concatenated_string(test, stream);
    KUNIT_EXPECT_EQ(test, strlen(concat_string), total_len);
    prandom_seed_state(&mut rnd, 3141592653589793238u64);
    let mut pos = concat_string;
    for _ in 0..100 {
        let string_end = strchr(pos, b'\n' as i32); *string_end = 0;
        let offset = (prandom_u32_state(&mut rnd) as usize) % (line.len() - 1);
        KUNIT_EXPECT_STREQ(test, pos, line.as_ptr().add(offset));
        pos = string_end.add(1);
    }
    KUNIT_EXPECT_EQ(test, strlen(pos), 0);
}

unsafe fn string_stream_append_test(test: *mut kunit) {
    let stream_1 = kunit_alloc_string_stream(test, GFP_KERNEL);
    let stream_2 = kunit_alloc_string_stream(test, GFP_KERNEL);
    KUNIT_ASSERT_NOT_ERR_OR_NULL(test, stream_1); KUNIT_ASSERT_NOT_ERR_OR_NULL(test, stream_2);
    string_stream_append(stream_1, stream_2);
    KUNIT_EXPECT_EQ(test, strlen(get_concatenated_string(test, stream_1)), 0);
    for s in [b"one\0",b"two\0",b"three\0",b"four\0",b"five\0",b"six\0",b"seven\0",b"eight\0",b"nine\0",b"ten\0"] { string_stream_add(stream_1, b"%s\n\0".as_ptr() as *const _, s.as_ptr()); }
    let before = get_concatenated_string(test, stream_1);
    string_stream_append(stream_1, stream_2); KUNIT_EXPECT_STREQ(test, get_concatenated_string(test, stream_1), before);
    for s in [b"Apple\0",b"Pear\0",b"Orange\0",b"Banana\0",b"Grape\0",b"Apricot\0"] { string_stream_add(stream_2, b"%s\n\0".as_ptr() as *const _, s.as_ptr()); }
    string_stream_append(stream_1, stream_2);
    let second = get_concatenated_string(test, stream_2);
    let before_len = strlen(before); let second_len = strlen(second);
    let combined = kunit_kmalloc(test, before_len + second_len + 1, GFP_KERNEL);
    snprintf(combined, before_len + second_len + 1, b"%s%s\0".as_ptr() as *const _, before, second);
    KUNIT_EXPECT_STREQ(test, get_concatenated_string(test, stream_1), combined);
    kunit_free_string_stream(test, stream_1);
    let stream_1 = kunit_alloc_string_stream(test, GFP_KERNEL); string_stream_append(stream_1, stream_2);
    KUNIT_EXPECT_STREQ(test, get_concatenated_string(test, stream_1), second);
}

unsafe fn string_stream_append_auto_newline_test(test: *mut kunit) {
    let stream_1 = kunit_alloc_string_stream(test, GFP_KERNEL); string_stream_set_append_newlines(stream_1, true);
    let stream_2 = kunit_alloc_string_stream(test, GFP_KERNEL);
    string_stream_add(stream_1, b"Original string\n\0".as_ptr()); string_stream_add(stream_2, b"Appended content\n\0".as_ptr()); string_stream_add(stream_2, b"More stuff\n\0".as_ptr()); string_stream_append(stream_1, stream_2);
    KUNIT_EXPECT_STREQ(test, get_concatenated_string(test, stream_1), b"Original string\nAppended content\nMore stuff\n\0".as_ptr());
    kunit_free_string_stream(test, stream_2); let stream_2 = kunit_alloc_string_stream(test, GFP_KERNEL); string_stream_add(stream_2, b"Another\0".as_ptr()); string_stream_add(stream_2, b"And again\0".as_ptr()); string_stream_append(stream_1, stream_2);
    KUNIT_EXPECT_STREQ(test, get_concatenated_string(test, stream_1), b"Original string\nAppended content\nMore stuff\nAnotherAnd again\n\0".as_ptr());
}

unsafe fn string_stream_append_empty_string_test(test: *mut kunit) { let stream = kunit_alloc_string_stream(test, GFP_KERNEL); string_stream_add(stream, b"%s\0".as_ptr(), b"\0".as_ptr()); KUNIT_EXPECT_TRUE(test, string_stream_is_empty(stream)); KUNIT_EXPECT_TRUE(test, list_empty(&mut (*stream).fragments)); string_stream_add(stream, b"Add this line\0".as_ptr()); let count = list_count_nodes(&mut (*stream).fragments); string_stream_add(stream, b"%s\0".as_ptr(), b"\0".as_ptr()); KUNIT_EXPECT_EQ(test, list_count_nodes(&mut (*stream).fragments), count); KUNIT_EXPECT_STREQ(test, get_concatenated_string(test, stream), b"Add this line\0".as_ptr()); }

unsafe fn string_stream_no_auto_newline_test(test: *mut kunit) { let stream = kunit_alloc_string_stream(test, GFP_KERNEL); for (fmt,args) in [(b"One\0".as_ptr(),[]), (b"Two\n\0".as_ptr(),[])] { let _ = (fmt,args); } string_stream_add(stream,b"One\0".as_ptr()); string_stream_add(stream,b"Two\n\0".as_ptr()); string_stream_add(stream,b"%s\n\0".as_ptr(),b"Three\0".as_ptr()); string_stream_add(stream,b"%s\0".as_ptr(),b"Four\n\0".as_ptr()); string_stream_add(stream,b"Five\n%s\0".as_ptr(),b"Six\0".as_ptr()); string_stream_add(stream,b"Seven\n\n\0".as_ptr()); string_stream_add(stream,b"Eight\0".as_ptr()); KUNIT_EXPECT_STREQ(test,get_concatenated_string(test,stream),b"OneTwo\nThree\nFour\nFive\nSixSeven\n\nEight\0".as_ptr()); }

unsafe fn string_stream_auto_newline_test(test: *mut kunit) { let stream = kunit_alloc_string_stream(test,GFP_KERNEL); string_stream_set_append_newlines(stream,true); string_stream_add(stream,b"One\0".as_ptr()); string_stream_add(stream,b"Two\n\0".as_ptr()); string_stream_add(stream,b"%s\n\0".as_ptr(),b"Three\0".as_ptr()); string_stream_add(stream,b"%s\0".as_ptr(),b"Four\n\0".as_ptr()); string_stream_add(stream,b"Five\n%s\0".as_ptr(),b"Six\0".as_ptr()); string_stream_add(stream,b"Seven\n\n\0".as_ptr()); string_stream_add(stream,b"Eight\0".as_ptr()); KUNIT_EXPECT_STREQ(test,get_concatenated_string(test,stream),b"One\nTwo\nThree\nFour\nFive\nSix\nSeven\n\nEight\n\0".as_ptr()); }

/* This doesn't actually "test" anything. It reports time taken and memory used for logging a large number of lines. */
unsafe fn string_stream_performance_test(test: *mut kunit) {
    let stream = kunit_alloc_string_stream(test, GFP_KERNEL); KUNIT_ASSERT_NOT_ERR_OR_NULL(test, stream);
    let mut test_line = [b'x' as i8; 101]; test_line[100]=0; let start_time=ktime_get();
    for i in 0..10000 { let offset=i%(test_line.len()-1); string_stream_add(stream,b"%s: %d\n\0".as_ptr(),test_line.as_ptr().add(offset),i); } let end_time=ktime_get();
    let mut bytes_requested=core::mem::size_of::<string_stream>(); let mut actual_bytes_used=ksize(stream as *const _); let mut total_string_length=0usize;
    let mut frag_container: *mut string_stream_fragment = core::ptr::null_mut(); list_for_each_entry!(frag_container, &(*stream).fragments, node);
    kunit_info(test,b"Time elapsed:           %lld us\n\0".as_ptr(),ktime_us_delta(end_time,start_time)); kunit_info(test,b"Total string length:    %zu\n\0".as_ptr(),total_string_length); kunit_info(test,b"Bytes requested:        %zu\n\0".as_ptr(),bytes_requested); kunit_info(test,b"Actual bytes allocated: %zu\n\0".as_ptr(),actual_bytes_used);
}

unsafe fn string_stream_test_init(test: *mut kunit) -> i32 { let priv_ = kunit_kzalloc(test,core::mem::size_of::<string_stream_test_priv>(),GFP_KERNEL) as *mut string_stream_test_priv; if priv_.is_null() { return -12; } (*test).priv_=priv_ as *mut _; 0 }

// KUNIT_CASE entries and suite registration preserve the original test suite topology.
static mut string_stream_test_suite: kunit_suite = kunit_suite { name: b"string-stream-test\0".as_ptr(), test_cases: core::ptr::null_mut(), init: Some(string_stream_test_init) };
// kunit_test_suites!(&mut string_stream_test_suite);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
