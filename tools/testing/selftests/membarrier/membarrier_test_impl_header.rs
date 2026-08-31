/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_long};

unsafe extern "C" {
    static mut errno: c_int;

    static __NR_membarrier: c_long;
    static MEMBARRIER_CMD_GET_REGISTRATIONS: c_int;
    static MEMBARRIER_CMD_QUERY: c_int;
    static MEMBARRIER_CMD_GLOBAL: c_int;
    static MEMBARRIER_CMD_PRIVATE_EXPEDITED: c_int;
    static MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED: c_int;
    static MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE: c_int;
    static MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE: c_int;
    static MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED: c_int;
    static MEMBARRIER_CMD_GLOBAL_EXPEDITED: c_int;

    static EINVAL: c_int;
    static EPERM: c_int;
    static ENOSYS: c_int;

    fn syscall(number: c_long, ...) -> c_long;
    fn strerror(errnum: c_int) -> *mut c_char;

    fn ksft_exit_fail_msg(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_exit_skip(fmt: *const c_char, ...);
}

static mut registrations: c_int = 0;

unsafe fn sys_membarrier(cmd: c_int, flags: c_int) -> c_int {
    unsafe { syscall(__NR_membarrier, cmd, flags) as c_int }
}

unsafe fn test_membarrier_get_registrations(cmd: c_int) -> c_int {
    let ret: c_int;
    let flags: c_int = 0;
    let test_name: *const c_char =
        c"sys membarrier MEMBARRIER_CMD_GET_REGISTRATIONS".as_ptr();

    unsafe {
        registrations |= cmd;

        ret = sys_membarrier(MEMBARRIER_CMD_GET_REGISTRATIONS, 0);
        if ret < 0 {
            ksft_exit_fail_msg(
                c"%s test: flags = %d, errno = %d\n".as_ptr(),
                test_name,
                flags,
                errno,
            );
        } else if ret != registrations {
            ksft_exit_fail_msg(
                c"%s test: flags = %d, ret = %d, registrations = %d\n".as_ptr(),
                test_name,
                flags,
                ret,
                registrations,
            );
        }
        ksft_test_result_pass(
            c"%s test: flags = %d, ret = %d, registrations = %d\n".as_ptr(),
            test_name,
            flags,
            ret,
            registrations,
        );
    }

    0
}

unsafe fn test_membarrier_cmd_fail() -> c_int {
    let cmd: c_int = -1;
    let flags: c_int = 0;
    let test_name: *const c_char = c"sys membarrier invalid command".as_ptr();

    unsafe {
        if sys_membarrier(cmd, flags) != -1 {
            ksft_exit_fail_msg(
                c"%s test: command = %d, flags = %d. Should fail, but passed\n".as_ptr(),
                test_name,
                cmd,
                flags,
            );
        }
        if errno != EINVAL {
            ksft_exit_fail_msg(
                c"%s test: flags = %d. Should return (%d: \"%s\"), but returned (%d: \"%s\").\n".as_ptr(),
                test_name,
                flags,
                EINVAL,
                strerror(EINVAL),
                errno,
                strerror(errno),
            );
        }

        ksft_test_result_pass(
            c"%s test: command = %d, flags = %d, errno = %d. Failed as expected\n".as_ptr(),
            test_name,
            cmd,
            flags,
            errno,
        );
    }
    0
}

unsafe fn test_membarrier_flags_fail() -> c_int {
    let cmd: c_int = unsafe { MEMBARRIER_CMD_QUERY };
    let flags: c_int = 1;
    let test_name: *const c_char =
        c"sys membarrier MEMBARRIER_CMD_QUERY invalid flags".as_ptr();

    unsafe {
        if sys_membarrier(cmd, flags) != -1 {
            ksft_exit_fail_msg(
                c"%s test: flags = %d. Should fail, but passed\n".as_ptr(),
                test_name,
                flags,
            );
        }
        if errno != EINVAL {
            ksft_exit_fail_msg(
                c"%s test: flags = %d. Should return (%d: \"%s\"), but returned (%d: \"%s\").\n".as_ptr(),
                test_name,
                flags,
                EINVAL,
                strerror(EINVAL),
                errno,
                strerror(errno),
            );
        }

        ksft_test_result_pass(
            c"%s test: flags = %d, errno = %d. Failed as expected\n".as_ptr(),
            test_name,
            flags,
            errno,
        );
    }
    0
}

unsafe fn test_membarrier_global_success() -> c_int {
    let cmd: c_int = unsafe { MEMBARRIER_CMD_GLOBAL };
    let flags: c_int = 0;
    let test_name: *const c_char = c"sys membarrier MEMBARRIER_CMD_GLOBAL".as_ptr();

    unsafe {
        if sys_membarrier(cmd, flags) != 0 {
            ksft_exit_fail_msg(
                c"%s test: flags = %d, errno = %d\n".as_ptr(),
                test_name,
                flags,
                errno,
            );
        }

        ksft_test_result_pass(c"%s test: flags = %d\n".as_ptr(), test_name, flags);
    }
    0
}

unsafe fn test_membarrier_private_expedited_fail() -> c_int {
    let cmd: c_int = unsafe { MEMBARRIER_CMD_PRIVATE_EXPEDITED };
    let flags: c_int = 0;
    let test_name: *const c_char =
        c"sys membarrier MEMBARRIER_CMD_PRIVATE_EXPEDITED not registered failure".as_ptr();

    unsafe {
        if sys_membarrier(cmd, flags) != -1 {
            ksft_exit_fail_msg(
                c"%s test: flags = %d. Should fail, but passed\n".as_ptr(),
                test_name,
                flags,
            );
        }
        if errno != EPERM {
            ksft_exit_fail_msg(
                c"%s test: flags = %d. Should return (%d: \"%s\"), but returned (%d: \"%s\").\n".as_ptr(),
                test_name,
                flags,
                EPERM,
                strerror(EPERM),
                errno,
                strerror(errno),
            );
        }

        ksft_test_result_pass(
            c"%s test: flags = %d, errno = %d\n".as_ptr(),
            test_name,
            flags,
            errno,
        );
    }
    0
}

unsafe fn test_membarrier_register_private_expedited_success() -> c_int {
    let cmd: c_int = unsafe { MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED };
    let flags: c_int = 0;
    let test_name: *const c_char =
        c"sys membarrier MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED".as_ptr();

    unsafe {
        if sys_membarrier(cmd, flags) != 0 {
            ksft_exit_fail_msg(
                c"%s test: flags = %d, errno = %d\n".as_ptr(),
                test_name,
                flags,
                errno,
            );
        }

        ksft_test_result_pass(c"%s test: flags = %d\n".as_ptr(), test_name, flags);

        test_membarrier_get_registrations(cmd);
    }
    0
}

unsafe fn test_membarrier_private_expedited_success() -> c_int {
    let cmd: c_int = unsafe { MEMBARRIER_CMD_PRIVATE_EXPEDITED };
    let flags: c_int = 0;
    let test_name: *const c_char = c"sys membarrier MEMBARRIER_CMD_PRIVATE_EXPEDITED".as_ptr();

    unsafe {
        if sys_membarrier(cmd, flags) != 0 {
            ksft_exit_fail_msg(
                c"%s test: flags = %d, errno = %d\n".as_ptr(),
                test_name,
                flags,
                errno,
            );
        }

        ksft_test_result_pass(c"%s test: flags = %d\n".as_ptr(), test_name, flags);
    }
    0
}

unsafe fn test_membarrier_private_expedited_sync_core_fail() -> c_int {
    let cmd: c_int = unsafe { MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE };
    let flags: c_int = 0;
    let test_name: *const c_char =
        c"sys membarrier MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE not registered failure".as_ptr();

    unsafe {
        if sys_membarrier(cmd, flags) != -1 {
            ksft_exit_fail_msg(
                c"%s test: flags = %d. Should fail, but passed\n".as_ptr(),
                test_name,
                flags,
            );
        }
        if errno != EPERM {
            ksft_exit_fail_msg(
                c"%s test: flags = %d. Should return (%d: \"%s\"), but returned (%d: \"%s\").\n".as_ptr(),
                test_name,
                flags,
                EPERM,
                strerror(EPERM),
                errno,
                strerror(errno),
            );
        }

        ksft_test_result_pass(
            c"%s test: flags = %d, errno = %d\n".as_ptr(),
            test_name,
            flags,
            errno,
        );
    }
    0
}

unsafe fn test_membarrier_register_private_expedited_sync_core_success() -> c_int {
    let cmd: c_int = unsafe { MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE };
    let flags: c_int = 0;
    let test_name: *const c_char =
        c"sys membarrier MEMBARRIER_CMD_REGISTER_PRIVATE_EXPEDITED_SYNC_CORE".as_ptr();

    unsafe {
        if sys_membarrier(cmd, flags) != 0 {
            ksft_exit_fail_msg(
                c"%s test: flags = %d, errno = %d\n".as_ptr(),
                test_name,
                flags,
                errno,
            );
        }

        ksft_test_result_pass(c"%s test: flags = %d\n".as_ptr(), test_name, flags);

        test_membarrier_get_registrations(cmd);
    }
    0
}

unsafe fn test_membarrier_private_expedited_sync_core_success() -> c_int {
    let cmd: c_int = unsafe { MEMBARRIER_CMD_PRIVATE_EXPEDITED };
    let flags: c_int = 0;
    let test_name: *const c_char =
        c"sys membarrier MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE".as_ptr();

    unsafe {
        if sys_membarrier(cmd, flags) != 0 {
            ksft_exit_fail_msg(
                c"%s test: flags = %d, errno = %d\n".as_ptr(),
                test_name,
                flags,
                errno,
            );
        }

        ksft_test_result_pass(c"%s test: flags = %d\n".as_ptr(), test_name, flags);
    }
    0
}

unsafe fn test_membarrier_register_global_expedited_success() -> c_int {
    let cmd: c_int = unsafe { MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED };
    let flags: c_int = 0;
    let test_name: *const c_char =
        c"sys membarrier MEMBARRIER_CMD_REGISTER_GLOBAL_EXPEDITED".as_ptr();

    unsafe {
        if sys_membarrier(cmd, flags) != 0 {
            ksft_exit_fail_msg(
                c"%s test: flags = %d, errno = %d\n".as_ptr(),
                test_name,
                flags,
                errno,
            );
        }

        ksft_test_result_pass(c"%s test: flags = %d\n".as_ptr(), test_name, flags);

        test_membarrier_get_registrations(cmd);
    }
    0
}

unsafe fn test_membarrier_global_expedited_success() -> c_int {
    let cmd: c_int = unsafe { MEMBARRIER_CMD_GLOBAL_EXPEDITED };
    let flags: c_int = 0;
    let test_name: *const c_char = c"sys membarrier MEMBARRIER_CMD_GLOBAL_EXPEDITED".as_ptr();

    unsafe {
        if sys_membarrier(cmd, flags) != 0 {
            ksft_exit_fail_msg(
                c"%s test: flags = %d, errno = %d\n".as_ptr(),
                test_name,
                flags,
                errno,
            );
        }

        ksft_test_result_pass(c"%s test: flags = %d\n".as_ptr(), test_name, flags);
    }
    0
}

unsafe fn test_membarrier_fail() -> c_int {
    let mut status: c_int;

    unsafe {
        status = test_membarrier_cmd_fail();
        if status != 0 {
            return status;
        }
        status = test_membarrier_flags_fail();
        if status != 0 {
            return status;
        }
        status = test_membarrier_private_expedited_fail();
        if status != 0 {
            return status;
        }
        status = sys_membarrier(MEMBARRIER_CMD_QUERY, 0);
        if status < 0 {
            ksft_test_result_fail(c"sys_membarrier() failed\n".as_ptr());
            return status;
        }
        if (status & MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE) != 0 {
            status = test_membarrier_private_expedited_sync_core_fail();
            if status != 0 {
                return status;
            }
        }
    }
    0
}

unsafe fn test_membarrier_success() -> c_int {
    let mut status: c_int;

    unsafe {
        status = test_membarrier_global_success();
        if status != 0 {
            return status;
        }
        status = test_membarrier_register_private_expedited_success();
        if status != 0 {
            return status;
        }
        status = test_membarrier_private_expedited_success();
        if status != 0 {
            return status;
        }
        status = sys_membarrier(MEMBARRIER_CMD_QUERY, 0);
        if status < 0 {
            ksft_test_result_fail(c"sys_membarrier() failed\n".as_ptr());
            return status;
        }
        if (status & MEMBARRIER_CMD_PRIVATE_EXPEDITED_SYNC_CORE) != 0 {
            status = test_membarrier_register_private_expedited_sync_core_success();
            if status != 0 {
                return status;
            }
            status = test_membarrier_private_expedited_sync_core_success();
            if status != 0 {
                return status;
            }
        }
        /*
         * It is valid to send a global membarrier from a non-registered
         * process.
         */
        status = test_membarrier_global_expedited_success();
        if status != 0 {
            return status;
        }
        status = test_membarrier_register_global_expedited_success();
        if status != 0 {
            return status;
        }
        status = test_membarrier_global_expedited_success();
        if status != 0 {
            return status;
        }
    }
    0
}

unsafe fn test_membarrier_query() -> c_int {
    let flags: c_int = 0;
    let ret: c_int;

    unsafe {
        ret = sys_membarrier(MEMBARRIER_CMD_QUERY, flags);
        if ret < 0 {
            if errno == ENOSYS {
                /*
                 * It is valid to build a kernel with
                 * CONFIG_MEMBARRIER=n. However, this skips the tests.
                 */
                ksft_exit_skip(c"sys membarrier (CONFIG_MEMBARRIER) is disabled.\n".as_ptr());
            }
            ksft_exit_fail_msg(c"sys_membarrier() failed\n".as_ptr());
        }
        if (ret & MEMBARRIER_CMD_GLOBAL) == 0 {
            ksft_exit_skip(c"sys_membarrier unsupported: CMD_GLOBAL not found.\n".as_ptr());
        }

        ksft_test_result_pass(c"sys_membarrier available\n".as_ptr());
    }
    0
}
