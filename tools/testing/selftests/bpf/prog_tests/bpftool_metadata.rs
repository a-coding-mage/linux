// SPDX-License-Identifier: GPL-2.0-only
//
// Translated from C. External declarations correspond to symbols provided by
// the original test harness and system headers.

use core::ffi::{c_char, c_int};

const BPFFS_DIR: &[u8] = b"/sys/fs/bpf/test_metadata\0";
const BPFFS_USED: &[u8] = b"/sys/fs/bpf/test_metadata/used\0";
const BPFFS_UNUSED: &[u8] = b"/sys/fs/bpf/test_metadata/unused\0";

const BPF_FILE_USED: &[u8] = b"metadata_used.bpf.o\0";
const BPF_FILE_UNUSED: &[u8] = b"metadata_unused.bpf.o\0";
const METADATA_MAP_NAME: &[u8] = b"metadata.rodata\0";

const MAX_BPFTOOL_OUTPUT_LEN: usize = 64 * 1024;

const MAX_TOKENS_TO_CHECK: usize = 3;
static mut OUTPUT: [c_char; MAX_BPFTOOL_OUTPUT_LEN] = [0; MAX_BPFTOOL_OUTPUT_LEN];

#[repr(C)]
struct test_desc {
    name: *mut c_char,
    bpf_prog: *mut c_char,
    bpffs_path: *mut c_char,
    expected_output: [*mut c_char; MAX_TOKENS_TO_CHECK],
    expected_output_json: [*mut c_char; MAX_TOKENS_TO_CHECK],
    metadata_map_name: *mut c_char,
}

unsafe extern "C" {
    static MAX_BPFTOOL_CMD_LEN: usize;

    fn mkdir(pathname: *const c_char, mode: c_int) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snprintf(str_: *mut c_char, size: usize, format: *const c_char, ...) -> c_int;

    fn run_bpftool_command(cmd: *const c_char) -> c_int;
    fn get_bpftool_command_output(cmd: *const c_char, output: *mut c_char, len: usize) -> c_int;
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_GT(actual: c_int, expected: c_int, name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
}

unsafe fn setup(_test: *mut test_desc) -> c_int {
    mkdir(BPFFS_DIR.as_ptr() as *const c_char, 0o700)
}

unsafe fn cleanup(test: *mut test_desc) {
    unlink((*test).bpffs_path);
    rmdir(BPFFS_DIR.as_ptr() as *const c_char);
}

unsafe fn check_metadata(buf: *mut c_char, tokens: *const *mut c_char, count: c_int) -> c_int {
    let mut i: c_int = 0;

    while i < count && !(*tokens.offset(i as isize)).is_null() {
        if strstr(buf, *tokens.offset(i as isize)).is_null() {
            return 1;
        }
        i += 1;
    }

    0
}

unsafe fn run_test(test: *mut test_desc) {
    let mut ret: c_int;
    let mut cmd: Vec<c_char> = vec![0; MAX_BPFTOOL_CMD_LEN];

    ret = snprintf(
        cmd.as_mut_ptr(),
        MAX_BPFTOOL_CMD_LEN,
        b"prog load %s %s\0".as_ptr() as *const c_char,
        (*test).bpf_prog,
        (*test).bpffs_path,
    );
    if !ASSERT_GT(ret, 0, b"format prog insert command\0".as_ptr() as *const c_char) {
        return;
    }
    ret = run_bpftool_command(cmd.as_ptr());
    if !ASSERT_OK(ret, b"load program\0".as_ptr() as *const c_char) {
        return;
    }

    /* Check output with default format */
    ret = snprintf(
        cmd.as_mut_ptr(),
        MAX_BPFTOOL_CMD_LEN,
        b"prog show pinned %s\0".as_ptr() as *const c_char,
        (*test).bpffs_path,
    );
    if !ASSERT_GT(
        ret,
        0,
        b"format pinned prog check command\0".as_ptr() as *const c_char,
    ) {
        return;
    }
    ret = get_bpftool_command_output(
        cmd.as_ptr(),
        core::ptr::addr_of_mut!(OUTPUT) as *mut c_char,
        MAX_BPFTOOL_OUTPUT_LEN,
    );
    if ASSERT_OK(ret, b"get program info\0".as_ptr() as *const c_char) {
        ret = check_metadata(
            core::ptr::addr_of_mut!(OUTPUT) as *mut c_char,
            (*test).expected_output.as_ptr(),
            (*test).expected_output.len() as c_int,
        );
        ASSERT_OK(ret, b"find metadata\0".as_ptr() as *const c_char);
    }

    /* Check output with json format */
    ret = snprintf(
        cmd.as_mut_ptr(),
        MAX_BPFTOOL_CMD_LEN,
        b"prog -j show pinned %s\0".as_ptr() as *const c_char,
        (*test).bpffs_path,
    );
    if !ASSERT_GT(
        ret,
        0,
        b"format pinned prog check command in json\0".as_ptr() as *const c_char,
    ) {
        return;
    }
    ret = get_bpftool_command_output(
        cmd.as_ptr(),
        core::ptr::addr_of_mut!(OUTPUT) as *mut c_char,
        MAX_BPFTOOL_OUTPUT_LEN,
    );
    if ASSERT_OK(
        ret,
        b"get program info in json\0".as_ptr() as *const c_char,
    ) {
        ret = check_metadata(
            core::ptr::addr_of_mut!(OUTPUT) as *mut c_char,
            (*test).expected_output_json.as_ptr(),
            (*test).expected_output_json.len() as c_int,
        );
        ASSERT_OK(ret, b"find metadata in json\0".as_ptr() as *const c_char);
    }

    /* Check that the corresponding map can be found and accessed */
    ret = snprintf(
        cmd.as_mut_ptr(),
        MAX_BPFTOOL_CMD_LEN,
        b"map show name %s\0".as_ptr() as *const c_char,
        (*test).metadata_map_name,
    );
    if !ASSERT_GT(ret, 0, b"format map check command\0".as_ptr() as *const c_char) {
        return;
    }
    ASSERT_OK(
        run_bpftool_command(cmd.as_ptr()),
        b"access metadata map\0".as_ptr() as *const c_char,
    );
}

static mut TESTS: [test_desc; 2] = [
    test_desc {
        name: b"metadata_unused\0".as_ptr() as *mut c_char,
        bpf_prog: BPF_FILE_UNUSED.as_ptr() as *mut c_char,
        bpffs_path: BPFFS_UNUSED.as_ptr() as *mut c_char,
        expected_output: [
            b"a = \"foo\"\0".as_ptr() as *mut c_char,
            b"b = 1\0".as_ptr() as *mut c_char,
            core::ptr::null_mut(),
        ],
        expected_output_json: [
            b"\"metadata\":{\"a\":\"foo\",\"b\":1}\0".as_ptr() as *mut c_char,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        ],
        metadata_map_name: METADATA_MAP_NAME.as_ptr() as *mut c_char,
    },
    test_desc {
        name: b"metadata_used\0".as_ptr() as *mut c_char,
        bpf_prog: BPF_FILE_USED.as_ptr() as *mut c_char,
        bpffs_path: BPFFS_USED.as_ptr() as *mut c_char,
        expected_output: [
            b"a = \"bar\"\0".as_ptr() as *mut c_char,
            b"b = 2\0".as_ptr() as *mut c_char,
            core::ptr::null_mut(),
        ],
        expected_output_json: [
            b"\"metadata\":{\"a\":\"bar\",\"b\":2}\0".as_ptr() as *mut c_char,
            core::ptr::null_mut(),
            core::ptr::null_mut(),
        ],
        metadata_map_name: METADATA_MAP_NAME.as_ptr() as *mut c_char,
    },
];
static TESTS_COUNT: c_int = 2;

#[no_mangle]
pub unsafe extern "C" fn test_bpftool_metadata() {
    let mut i: c_int = 0;

    while i < TESTS_COUNT {
        let test = core::ptr::addr_of_mut!(TESTS).cast::<test_desc>().offset(i as isize);
        if !test__start_subtest((*test).name) {
            i += 1;
            continue;
        }
        if ASSERT_OK(setup(test), b"setup bpffs pin dir\0".as_ptr() as *const c_char) {
            run_test(test);
            cleanup(test);
        }
        i += 1;
    }
}
