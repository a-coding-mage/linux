// SPDX-License-Identifier: GPL-2.0-only
/* Test that empty argvs are swapped out for a single empty string. */

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

type pid_t = c_int;

#[repr(C)]
struct KsftCnt {
    ksft_pass: c_int,
}

unsafe extern "C" {
    fn fork() -> pid_t;
    fn execve(
        pathname: *const c_char,
        argv: *const *mut c_char,
        envp: *const *mut c_char,
    ) -> c_int;
    fn perror(s: *const c_char);
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;

    static mut stderr: *mut c_void;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_test_result_fail(msg: *const c_char, ...);
    fn ksft_test_result_pass(msg: *const c_char, ...);
    fn ksft_exit(condition: bool) -> !;

    static ksft_cnt: KsftCnt;
    static ksft_plan: c_int;
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn check_result(pid: pid_t, msg: *const c_char) {
    let mut wstatus: c_int = 0;

    if pid == -1 as pid_t {
        perror(c"# fork".as_ptr());
        ksft_test_result_fail(c"fork failed: %s\n".as_ptr(), msg);
        return;
    }
    if waitpid(pid, &mut wstatus, 0) < 0 {
        perror(c"# waitpid".as_ptr());
        ksft_test_result_fail(c"waitpid failed: %s\n".as_ptr(), msg);
        return;
    }
    if !wifexited(wstatus) {
        ksft_test_result_fail(c"child did not exit: %s\n".as_ptr(), msg);
        return;
    }
    if wexitstatus(wstatus) != 0 {
        ksft_test_result_fail(c"non-zero exit: %s\n".as_ptr(), msg);
        return;
    }
    ksft_test_result_pass(c"%s\n".as_ptr(), msg);
}

unsafe fn fork_exec<F>(msg: *const c_char, exec: F) -> Option<c_int>
where
    F: FnOnce() -> c_int,
{
    let pid = fork();
    if pid == 0 {
        /* Child */
        exec(); /* Some kind of exec */
        perror(msg);
        return Some(1);
    }
    check_result(pid, msg);
    None
}

#[no_mangle]
pub unsafe extern "C" fn main(
    argc: c_int,
    argv: *mut *mut c_char,
    envp: *mut *mut c_char,
) -> c_int {
    static mut ARGS: [*mut c_char; 1] = [ptr::null_mut()];
    static EMPTY: [c_char; 1] = [0];
    static mut STR: [*mut c_char; 2] = [EMPTY.as_ptr() as *mut c_char, ptr::null_mut()];

    /* argc counting checks */
    if argc < 1 {
        fprintf(
            stderr,
            c"# FAIL: saw argc == 0 (old kernel?)\n".as_ptr(),
        );
        return 1;
    }
    if argc != 1 {
        fprintf(stderr, c"# FAIL: unknown argc (%d)\n".as_ptr(), argc);
        return 1;
    }
    if *(*argv.add(0)).add(0) == '\0' as c_char {
        /* Good, we found a NULL terminated string at argv[0]! */
        return 0;
    }

    /* Test runner. */
    ksft_print_header();
    ksft_set_plan(5);

    if let Some(ret) = fork_exec(c"# execve(argv[0], str, NULL)".as_ptr(), || {
        execve(*argv.add(0), STR.as_ptr(), ptr::null())
    }) {
        return ret;
    }
    if let Some(ret) = fork_exec(c"# execve(argv[0], NULL, NULL)".as_ptr(), || {
        execve(*argv.add(0), ptr::null(), ptr::null())
    }) {
        return ret;
    }
    if let Some(ret) = fork_exec(c"# execve(argv[0], NULL, envp)".as_ptr(), || {
        execve(*argv.add(0), ptr::null(), envp)
    }) {
        return ret;
    }
    if let Some(ret) = fork_exec(c"# execve(argv[0], args, NULL)".as_ptr(), || {
        execve(*argv.add(0), ARGS.as_ptr(), ptr::null())
    }) {
        return ret;
    }
    if let Some(ret) = fork_exec(c"# execve(argv[0], args, envp)".as_ptr(), || {
        execve(*argv.add(0), ARGS.as_ptr(), envp)
    }) {
        return ret;
    }

    ksft_exit(ksft_cnt.ksft_pass == ksft_plan);
}
