// SPDX-License-Identifier: GPL-2.0-only
// C dependencies: <unistd.h>, <string.h>, <stdbool.h>, <limits.h>,
// "bpf_util.h", and "bpftool_helpers.h".

use core::ffi::{c_char, c_int, c_ulong, c_void};

type size_t = c_ulong;

const PATH_MAX: usize = 4096;
const X_OK: c_int = 1;

const BPFTOOL_FULL_CMD_MAX_LEN: usize = PATH_MAX * 2;

const BPFTOOL_DEFAULT_PATH: &[u8] = b"tools/sbin/bpftool\0";

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

extern "C" {
    static mut stderr: *mut FILE;

    fn access(pathname: *const c_char, mode: c_int) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn popen(command: *const c_char, type_: *const c_char) -> *mut FILE;
    fn pclose(stream: *mut FILE) -> c_int;
    fn fread(ptr: *mut c_void, size: size_t, nmemb: size_t, stream: *mut FILE) -> size_t;

    fn strscpy(dest: *mut c_char, src: *const c_char, count: size_t) -> isize;
}

unsafe fn detect_bpftool_path(buffer: *mut c_char, size: size_t) -> c_int {
    let mut tmp: [c_char; PATH_MAX] = [0; PATH_MAX];
    let env_path: *const c_char;

    /* First, check if BPFTOOL environment variable is set */
    env_path = getenv(b"BPFTOOL\0".as_ptr() as *const c_char);
    if !env_path.is_null() && access(env_path, X_OK) == 0 {
        strscpy(buffer, env_path, size);
        return 0;
    } else if !env_path.is_null() {
        fprintf(
            stderr,
            b"bpftool '%s' doesn't exist or is not executable\n\0".as_ptr() as *const c_char,
            env_path,
        );
        return 1;
    }

    /* Check default bpftool location (will work if we are running the
     * default flavor of test_progs)
     */
    snprintf(
        tmp.as_mut_ptr(),
        tmp.len() as size_t,
        b"./%s\0".as_ptr() as *const c_char,
        BPFTOOL_DEFAULT_PATH.as_ptr() as *const c_char,
    );
    if access(tmp.as_ptr(), X_OK) == 0 {
        strscpy(buffer, tmp.as_ptr(), size);
        return 0;
    }

    /* Check alternate bpftool location (will work if we are running a
     * specific flavor of test_progs, e.g. cpuv4 or no_alu32)
     */
    snprintf(
        tmp.as_mut_ptr(),
        tmp.len() as size_t,
        b"../%s\0".as_ptr() as *const c_char,
        BPFTOOL_DEFAULT_PATH.as_ptr() as *const c_char,
    );
    if access(tmp.as_ptr(), X_OK) == 0 {
        strscpy(buffer, tmp.as_ptr(), size);
        return 0;
    }

    fprintf(
        stderr,
        b"Failed to detect bpftool path, use BPFTOOL env var to override\n\0".as_ptr()
            as *const c_char,
    );
    1
}

unsafe fn run_command(args: *mut c_char, output_buf: *mut c_char, output_max_len: size_t) -> c_int {
    static mut BPFTOOL_PATH: [c_char; PATH_MAX] = [0; PATH_MAX];
    let suppress_output: bool = !(!output_buf.is_null() && output_max_len != 0);
    let mut command: [c_char; BPFTOOL_FULL_CMD_MAX_LEN] = [0; BPFTOOL_FULL_CMD_MAX_LEN];
    let f: *mut FILE;
    let mut ret: c_int;

    /* Detect and cache bpftool binary location */
    if BPFTOOL_PATH[0] == 0
        && detect_bpftool_path(BPFTOOL_PATH.as_mut_ptr(), BPFTOOL_PATH.len() as size_t) != 0
    {
        return 1;
    }

    ret = snprintf(
        command.as_mut_ptr(),
        command.len() as size_t,
        b"%s %s%s\0".as_ptr() as *const c_char,
        BPFTOOL_PATH.as_ptr(),
        args,
        if suppress_output {
            b" > /dev/null 2>&1\0".as_ptr() as *const c_char
        } else {
            b"\0".as_ptr() as *const c_char
        },
    );

    f = popen(command.as_ptr(), b"r\0".as_ptr() as *const c_char);
    if f.is_null() {
        return 1;
    }

    if !suppress_output {
        fread(output_buf as *mut c_void, 1, output_max_len, f);
    }
    ret = pclose(f);

    ret
}

#[no_mangle]
pub unsafe extern "C" fn run_bpftool_command(args: *mut c_char) -> c_int {
    run_command(args, core::ptr::null_mut(), 0)
}

#[no_mangle]
pub unsafe extern "C" fn get_bpftool_command_output(
    args: *mut c_char,
    output_buf: *mut c_char,
    output_max_len: size_t,
) -> c_int {
    run_command(args, output_buf, output_max_len)
}
