/* SPDX-License-Identifier: GPL-2.0 */

use core::ffi::{c_char, c_int, c_long};

/* Dependencies from the original C source:
 * - linux/compiler.h for __maybe_unused
 * - linux/types.h for __u64 and __s32
 * - unistd.h for syscall()
 * - ../tests.h for DEFINE_WORKLOAD()
 */

/* This workload was initially added to test enum augmentation with BTF in perf
 * trace because its the only syscall that has an enum argument. Since it is
 * a recent addition to the Linux kernel (at the time of the introduction of this
 * 'perf test' workload) we just add the required types and defines here instead
 * of including linux/landlock, that isn't available in older systems.
 *
 * We are not interested in the result of the syscall, just in intercepting
 * its arguments.
 */

const __NR_LANDLOCK_ADD_RULE: c_long = 445;

const LANDLOCK_ACCESS_FS_READ_FILE: u64 = 1_u64 << 2;

const LANDLOCK_RULE_PATH_BENEATH: c_int = 1;

#[repr(C)]
struct landlock_path_beneath_attr {
    allowed_access: u64,
    parent_fd: i32,
}

const LANDLOCK_ACCESS_NET_CONNECT_TCP: u64 = 1_u64 << 1;

const LANDLOCK_RULE_NET_PORT: c_int = 2;

#[repr(C)]
struct landlock_net_port_attr {
    allowed_access: u64,
    port: u64,
}

unsafe extern "C" {
    fn syscall(num: c_long, ...) -> c_long;
}

unsafe extern "C" fn landlock(
    _argc: c_int,
    _argv: *const *const c_char,
) -> c_int {
    let fd: c_int = 11;
    let flags: c_int = 45;

    let path_beneath_attr = landlock_path_beneath_attr {
        allowed_access: LANDLOCK_ACCESS_FS_READ_FILE,
        parent_fd: 14,
    };

    let net_port_attr = landlock_net_port_attr {
        port: 19,
        allowed_access: LANDLOCK_ACCESS_NET_CONNECT_TCP,
    };

    unsafe {
        syscall(
            __NR_LANDLOCK_ADD_RULE,
            fd,
            LANDLOCK_RULE_PATH_BENEATH,
            &path_beneath_attr as *const landlock_path_beneath_attr,
            flags,
        );

        syscall(
            __NR_LANDLOCK_ADD_RULE,
            fd,
            LANDLOCK_RULE_NET_PORT,
            &net_port_attr as *const landlock_net_port_attr,
            flags,
        );
    }

    0
}

define_workload!(landlock);
