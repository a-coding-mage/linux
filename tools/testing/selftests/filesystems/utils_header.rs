/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies: errno.h, linux/types.h, sched.h, signal.h,
// stdbool.h, stdio.h, stdlib.h, string.h, syscall.h, sys/capability.h,
// sys/fsuid.h, sys/types.h, unistd.h.
// The original header also ensures _GNU_SOURCE is defined before including
// those headers so setns(2) and CLONE_NEWUSER are available.

use core::ffi::{c_char, c_int, c_ulong};

extern "C" {
    pub fn get_userns_fd(nsid: c_ulong, hostid: c_ulong, range: c_ulong) -> c_int;

    pub fn caps_down() -> c_int;
    pub fn cap_down(down: cap_value_t) -> c_int;

    pub fn switch_ids(uid: uid_t, gid: gid_t) -> bool;
    pub fn setup_userns() -> c_int;
    pub fn enter_userns() -> c_int;
}

pub unsafe fn switch_userns(fd: c_int, uid: uid_t, gid: gid_t, drop_caps: bool) -> bool {
    if setns(fd, CLONE_NEWUSER) != 0 {
        return false;
    }

    if !switch_ids(uid, gid) {
        return false;
    }

    if drop_caps && caps_down() == 0 {
        return false;
    }

    true
}

extern "C" {
    pub fn wait_for_pid(pid: pid_t) -> c_int;
    pub fn write_file(path: *const c_char, val: *const c_char) -> c_int;
    pub fn get_unique_mnt_id(path: *const c_char) -> u64;
}
