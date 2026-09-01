// SPDX-License-Identifier: (LGPL-2.1 OR BSD-2-Clause)

// Dependencies from "test_progs.h" and "testing_helpers.h" are external to
// this isolated translation.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct test_filter {
    pub name: *mut c_char,
    pub subtests: *mut *mut c_char,
    pub subtest_cnt: c_int,
}

#[repr(C)]
pub struct test_filter_set {
    pub cnt: c_int,
    pub tests: *mut test_filter,
}

pub enum FILE {}

unsafe extern "C" {
    fn free(ptr: *mut c_void);
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn mkstemp(template: *mut c_char) -> c_int;
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    fn close(fd: c_int) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn ferror(stream: *mut FILE) -> c_int;
    fn fsync(fd: c_int) -> c_int;
    fn fileno(stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn remove(pathname: *const c_char) -> c_int;

    fn parse_test_list(
        test_list: *const c_char,
        set: *mut test_filter_set,
        is_glob_pattern: bool,
    ) -> c_int;
    fn parse_test_list_file(
        path: *const c_char,
        set: *mut test_filter_set,
        is_glob_pattern: bool,
    ) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_NEQ(actual: *mut c_void, expected: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
}

unsafe fn init_test_filter_set(set: *mut test_filter_set) {
    unsafe {
        (*set).cnt = 0;
        (*set).tests = core::ptr::null_mut();
    }
}

unsafe fn free_test_filter_set(set: *mut test_filter_set) {
    let mut i: c_int;
    let mut j: c_int;

    unsafe {
        i = 0;
        while i < (*set).cnt {
            j = 0;
            while j < (*(*set).tests.add(i as usize)).subtest_cnt {
                free((*(*set).tests.add(i as usize)).subtests.add(j as usize).read() as *mut c_void);
                j += 1;
            }
            free((*(*set).tests.add(i as usize)).subtests as *mut c_void);
            free((*(*set).tests.add(i as usize)).name as *mut c_void);
            i += 1;
        }

        free((*set).tests as *mut c_void);
        init_test_filter_set(set);
    }
}

unsafe fn test_parse_test_list() {
    let mut set = test_filter_set {
        cnt: 0,
        tests: core::ptr::null_mut(),
    };

    unsafe {
        init_test_filter_set(&mut set);

        ASSERT_OK(
            parse_test_list(b"arg_parsing\0".as_ptr() as *const c_char, &mut set, true),
            b"parsing\0".as_ptr() as *const c_char,
        );
        if !ASSERT_EQ(set.cnt, 1, b"test filters count\0".as_ptr() as *const c_char) {
            free_test_filter_set(&mut set);
            return;
        }
        if !ASSERT_OK_PTR(
            set.tests as *mut c_void,
            b"test filters initialized\0".as_ptr() as *const c_char,
        ) {
            free_test_filter_set(&mut set);
            return;
        }
        ASSERT_EQ(
            (*set.tests.add(0)).subtest_cnt,
            0,
            b"subtest filters count\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"arg_parsing\0".as_ptr() as *const c_char,
                (*set.tests.add(0)).name,
            ),
            b"subtest name\0".as_ptr() as *const c_char,
        );
        free_test_filter_set(&mut set);

        ASSERT_OK(
            parse_test_list(
                b"arg_parsing,bpf_cookie\0".as_ptr() as *const c_char,
                &mut set,
                true,
            ),
            b"parsing\0".as_ptr() as *const c_char,
        );
        if !ASSERT_EQ(set.cnt, 2, b"count of test filters\0".as_ptr() as *const c_char) {
            free_test_filter_set(&mut set);
            return;
        }
        if !ASSERT_OK_PTR(
            set.tests as *mut c_void,
            b"test filters initialized\0".as_ptr() as *const c_char,
        ) {
            free_test_filter_set(&mut set);
            return;
        }
        ASSERT_EQ(
            (*set.tests.add(0)).subtest_cnt,
            0,
            b"subtest filters count\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ(
            (*set.tests.add(1)).subtest_cnt,
            0,
            b"subtest filters count\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"arg_parsing\0".as_ptr() as *const c_char,
                (*set.tests.add(0)).name,
            ),
            b"test name\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"bpf_cookie\0".as_ptr() as *const c_char,
                (*set.tests.add(1)).name,
            ),
            b"test name\0".as_ptr() as *const c_char,
        );
        free_test_filter_set(&mut set);

        ASSERT_OK(
            parse_test_list(
                b"arg_parsing/arg_parsing,bpf_cookie\0".as_ptr() as *const c_char,
                &mut set,
                true,
            ),
            b"parsing\0".as_ptr() as *const c_char,
        );
        if !ASSERT_EQ(set.cnt, 2, b"count of test filters\0".as_ptr() as *const c_char) {
            free_test_filter_set(&mut set);
            return;
        }
        if !ASSERT_OK_PTR(
            set.tests as *mut c_void,
            b"test filters initialized\0".as_ptr() as *const c_char,
        ) {
            free_test_filter_set(&mut set);
            return;
        }
        if !ASSERT_EQ(
            (*set.tests.add(0)).subtest_cnt,
            1,
            b"subtest filters count\0".as_ptr() as *const c_char,
        ) {
            free_test_filter_set(&mut set);
            return;
        }
        ASSERT_EQ(
            (*set.tests.add(1)).subtest_cnt,
            0,
            b"subtest filters count\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"arg_parsing\0".as_ptr() as *const c_char,
                (*set.tests.add(0)).name,
            ),
            b"test name\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"arg_parsing\0".as_ptr() as *const c_char,
                *(*set.tests.add(0)).subtests.add(0),
            ),
            b"subtest name\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"bpf_cookie\0".as_ptr() as *const c_char,
                (*set.tests.add(1)).name,
            ),
            b"test name\0".as_ptr() as *const c_char,
        );
        free_test_filter_set(&mut set);

        ASSERT_OK(
            parse_test_list(
                b"arg_parsing/arg_parsing\0".as_ptr() as *const c_char,
                &mut set,
                true,
            ),
            b"parsing\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            parse_test_list(b"bpf_cookie\0".as_ptr() as *const c_char, &mut set, true),
            b"parsing\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            parse_test_list(b"send_signal\0".as_ptr() as *const c_char, &mut set, true),
            b"parsing\0".as_ptr() as *const c_char,
        );
        if !ASSERT_EQ(set.cnt, 3, b"count of test filters\0".as_ptr() as *const c_char) {
            free_test_filter_set(&mut set);
            return;
        }
        if !ASSERT_OK_PTR(
            set.tests as *mut c_void,
            b"test filters initialized\0".as_ptr() as *const c_char,
        ) {
            free_test_filter_set(&mut set);
            return;
        }
        if !ASSERT_EQ(
            (*set.tests.add(0)).subtest_cnt,
            1,
            b"subtest filters count\0".as_ptr() as *const c_char,
        ) {
            free_test_filter_set(&mut set);
            return;
        }
        ASSERT_EQ(
            (*set.tests.add(1)).subtest_cnt,
            0,
            b"subtest filters count\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ(
            (*set.tests.add(2)).subtest_cnt,
            0,
            b"subtest filters count\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"arg_parsing\0".as_ptr() as *const c_char,
                (*set.tests.add(0)).name,
            ),
            b"test name\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"arg_parsing\0".as_ptr() as *const c_char,
                *(*set.tests.add(0)).subtests.add(0),
            ),
            b"subtest name\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"bpf_cookie\0".as_ptr() as *const c_char,
                (*set.tests.add(1)).name,
            ),
            b"test name\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"send_signal\0".as_ptr() as *const c_char,
                (*set.tests.add(2)).name,
            ),
            b"test name\0".as_ptr() as *const c_char,
        );
        free_test_filter_set(&mut set);

        ASSERT_OK(
            parse_test_list(
                b"bpf_cookie/trace\0".as_ptr() as *const c_char,
                &mut set,
                false,
            ),
            b"parsing\0".as_ptr() as *const c_char,
        );
        if !ASSERT_EQ(set.cnt, 1, b"count of test filters\0".as_ptr() as *const c_char) {
            free_test_filter_set(&mut set);
            return;
        }
        if !ASSERT_OK_PTR(
            set.tests as *mut c_void,
            b"test filters initialized\0".as_ptr() as *const c_char,
        ) {
            free_test_filter_set(&mut set);
            return;
        }
        if !ASSERT_EQ(
            (*set.tests.add(0)).subtest_cnt,
            1,
            b"subtest filters count\0".as_ptr() as *const c_char,
        ) {
            free_test_filter_set(&mut set);
            return;
        }
        ASSERT_OK(
            strcmp(
                b"*bpf_cookie*\0".as_ptr() as *const c_char,
                (*set.tests.add(0)).name,
            ),
            b"test name\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"*trace*\0".as_ptr() as *const c_char,
                *(*set.tests.add(0)).subtests.add(0),
            ),
            b"subtest name\0".as_ptr() as *const c_char,
        );
        free_test_filter_set(&mut set);

        ASSERT_OK(
            parse_test_list(
                b"t/subtest1,t/subtest2\0".as_ptr() as *const c_char,
                &mut set,
                true,
            ),
            b"parsing\0".as_ptr() as *const c_char,
        );
        if !ASSERT_EQ(set.cnt, 1, b"count of test filters\0".as_ptr() as *const c_char) {
            free_test_filter_set(&mut set);
            return;
        }
        if !ASSERT_OK_PTR(
            set.tests as *mut c_void,
            b"test filters initialized\0".as_ptr() as *const c_char,
        ) {
            free_test_filter_set(&mut set);
            return;
        }
        if !ASSERT_EQ(
            (*set.tests.add(0)).subtest_cnt,
            2,
            b"subtest filters count\0".as_ptr() as *const c_char,
        ) {
            free_test_filter_set(&mut set);
            return;
        }
        ASSERT_OK(
            strcmp(b"t\0".as_ptr() as *const c_char, (*set.tests.add(0)).name),
            b"test name\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"subtest1\0".as_ptr() as *const c_char,
                *(*set.tests.add(0)).subtests.add(0),
            ),
            b"subtest name\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"subtest2\0".as_ptr() as *const c_char,
                *(*set.tests.add(0)).subtests.add(1),
            ),
            b"subtest name\0".as_ptr() as *const c_char,
        );
        free_test_filter_set(&mut set);
    }
}

unsafe fn test_parse_test_list_file() {
    let mut set = test_filter_set {
        cnt: 0,
        tests: core::ptr::null_mut(),
    };
    let mut tmpfile = [0 as c_char; 80];
    let mut fp: *mut FILE;
    let fd: c_int;

    unsafe {
        snprintf(
            tmpfile.as_mut_ptr(),
            tmpfile.len(),
            b"/tmp/bpf_arg_parsing_test.XXXXXX\0".as_ptr() as *const c_char,
        );
        fd = mkstemp(tmpfile.as_mut_ptr());
        if !ASSERT_GE(fd, 0, b"create tmp\0".as_ptr() as *const c_char) {
            return;
        }

        fp = fdopen(fd, b"w\0".as_ptr() as *const c_char);
        if !ASSERT_NEQ(
            fp as *mut c_void,
            core::ptr::null_mut(),
            b"fdopen tmp\0".as_ptr() as *const c_char,
        ) {
            close(fd);
            remove(tmpfile.as_ptr());
            return;
        }

        fprintf(fp, b"# comment\n\0".as_ptr() as *const c_char);
        fprintf(fp, b"  test_with_spaces    \n\0".as_ptr() as *const c_char);
        fprintf(fp, b"testA/subtest    # comment\n\0".as_ptr() as *const c_char);
        fprintf(fp, b"testB#comment with no space\n\0".as_ptr() as *const c_char);
        fprintf(fp, b"testB # duplicate\n\0".as_ptr() as *const c_char);
        fprintf(
            fp,
            b"testA/subtest # subtest duplicate\n\0".as_ptr() as *const c_char,
        );
        fprintf(fp, b"testA/subtest2\n\0".as_ptr() as *const c_char);
        fprintf(fp, b"testC_no_eof_newline\0".as_ptr() as *const c_char);
        fflush(fp);

        if !ASSERT_OK(ferror(fp), b"prepare tmp\0".as_ptr() as *const c_char) {
            fclose(fp);
            remove(tmpfile.as_ptr());
            return;
        }

        if !ASSERT_OK(fsync(fileno(fp)), b"fsync tmp\0".as_ptr() as *const c_char) {
            fclose(fp);
            remove(tmpfile.as_ptr());
            return;
        }

        init_test_filter_set(&mut set);

        if !ASSERT_OK(
            parse_test_list_file(tmpfile.as_ptr(), &mut set, true),
            b"parse file\0".as_ptr() as *const c_char,
        ) {
            fclose(fp);
            remove(tmpfile.as_ptr());
            return;
        }

        if !ASSERT_EQ(set.cnt, 4, b"test  count\0".as_ptr() as *const c_char) {
            free_test_filter_set(&mut set);
            fclose(fp);
            remove(tmpfile.as_ptr());
            return;
        }

        ASSERT_OK(
            strcmp(
                b"test_with_spaces\0".as_ptr() as *const c_char,
                (*set.tests.add(0)).name,
            ),
            b"test 0 name\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ(
            (*set.tests.add(0)).subtest_cnt,
            0,
            b"test 0 subtest count\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"testA\0".as_ptr() as *const c_char,
                (*set.tests.add(1)).name,
            ),
            b"test 1 name\0".as_ptr() as *const c_char,
        );
        ASSERT_EQ(
            (*set.tests.add(1)).subtest_cnt,
            2,
            b"test 1 subtest count\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"subtest\0".as_ptr() as *const c_char,
                *(*set.tests.add(1)).subtests.add(0),
            ),
            b"test 1 subtest 0\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"subtest2\0".as_ptr() as *const c_char,
                *(*set.tests.add(1)).subtests.add(1),
            ),
            b"test 1 subtest 1\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"testB\0".as_ptr() as *const c_char,
                (*set.tests.add(2)).name,
            ),
            b"test 2 name\0".as_ptr() as *const c_char,
        );
        ASSERT_OK(
            strcmp(
                b"testC_no_eof_newline\0".as_ptr() as *const c_char,
                (*set.tests.add(3)).name,
            ),
            b"test 3 name\0".as_ptr() as *const c_char,
        );

        free_test_filter_set(&mut set);
        fclose(fp);
        remove(tmpfile.as_ptr());
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_arg_parsing() {
    unsafe {
        if test__start_subtest(b"test_parse_test_list\0".as_ptr() as *const c_char) {
            test_parse_test_list();
        }
        if test__start_subtest(b"test_parse_test_list_file\0".as_ptr() as *const c_char) {
            test_parse_test_list_file();
        }
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
