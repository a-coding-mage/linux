/* SPDX-License-Identifier: GPL-2.0 */

/*
 * Translated from testing/selftests/clone3/clone3_selftests.h.
 * C include dependencies preserved as Rust external dependencies:
 * sched.h, linux/sched.h, linux/types.h, stdint.h, syscall.h, sys/wait.h,
 * and kselftest.h.
 */

pub type __aligned_u64 = u64;

#[inline]
pub fn ptr_to_u64<T>(ptr: *const T) -> __aligned_u64 {
    ptr as usize as __aligned_u64
}

/* Fallback value used when __NR_clone3 is not provided by syscall headers. */
pub const __NR_clone3: libc::c_long = 435;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct __clone_args {
    pub flags: __aligned_u64,
    pub pidfd: __aligned_u64,
    pub child_tid: __aligned_u64,
    pub parent_tid: __aligned_u64,
    pub exit_signal: __aligned_u64,
    pub stack: __aligned_u64,
    pub stack_size: __aligned_u64,
    pub tls: __aligned_u64,
    pub set_tid: __aligned_u64,
    pub set_tid_size: __aligned_u64,
    pub cgroup: __aligned_u64,
}

extern "C" {
    static mut stdout: *mut libc::FILE;
    static mut stderr: *mut libc::FILE;

    fn fflush(stream: *mut libc::FILE) -> libc::c_int;
    fn syscall(num: libc::c_long, ...) -> libc::c_long;
    fn wait(status: *mut libc::c_int) -> libc::pid_t;
    fn exit(status: libc::c_int) -> !;

    fn ksft_exit_skip(msg: *const libc::c_char, ...) -> !;
    fn ksft_exit_fail_msg(msg: *const libc::c_char, ...) -> !;
    fn ksft_print_msg(msg: *const libc::c_char, ...);
}

#[inline]
unsafe fn errno_value() -> libc::c_int {
    *libc::__errno_location()
}

unsafe fn sys_clone3(args: *mut __clone_args, size: libc::size_t) -> libc::pid_t {
    fflush(stdout);
    fflush(stderr);
    syscall(__NR_clone3, args, size) as libc::pid_t
}

#[inline]
pub unsafe fn test_clone3_supported() {
    let pid: libc::pid_t;
    let mut args: __clone_args = core::mem::zeroed();

    if __NR_clone3 < 0 {
        ksft_exit_skip(b"clone3() syscall is not supported\n\0".as_ptr() as *const libc::c_char);
    }

    /* Set to something that will always cause EINVAL. */
    args.exit_signal = -1_i64 as __aligned_u64;
    pid = sys_clone3(&mut args, core::mem::size_of_val(&args) as libc::size_t);
    if pid == 0 {
        exit(libc::EXIT_SUCCESS);
    }

    if pid > 0 {
        wait(core::ptr::null_mut());
        ksft_exit_fail_msg(
            b"Managed to create child process with invalid exit_signal\n\0".as_ptr()
                as *const libc::c_char,
        );
    }

    if errno_value() == libc::ENOSYS {
        ksft_exit_skip(b"clone3() syscall is not supported\n\0".as_ptr() as *const libc::c_char);
    }

    ksft_print_msg(b"clone3() syscall supported\n\0".as_ptr() as *const libc::c_char);
}
