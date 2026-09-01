// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2025 Christian Brauner <brauner@kernel.org>

// C source used _GNU_SOURCE and included:
// <fcntl.h>, <stdio.h>, <stdlib.h>, <sys/stat.h>, <unistd.h>,
// <errno.h>, <string.h>, <linux/nsfs.h>, and "kselftest_harness.h".

use core::ffi::{c_char, c_int, c_uint};

// Constants supplied by <linux/nsfs.h>.
use linux_nsfs::{
    CGROUP_NS_INIT_INO, IPC_NS_INIT_INO, MNT_NS_INIT_INO, NET_NS_INIT_INO, PID_NS_INIT_INO,
    TIME_NS_INIT_INO, USER_NS_INIT_INO, UTS_NS_INIT_INO,
};

#[repr(C)]
struct ns_info {
    name: *const c_char,
    proc_path: *const c_char,
    expected_ino: c_uint,
}

extern "C" {
    fn stat(pathname: *const c_char, statbuf: *mut libc::stat) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
}

extern "C" {
    fn __errno_location() -> *mut c_int;
}

// Functions/macros supplied by "kselftest_harness.h".
extern "C" {
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_print_msg(fmt: *const c_char, ...);
}

const ENOENT: c_int = 2;

static mut namespaces: [ns_info; 8] = unsafe {
    [
        ns_info {
            name: b"ipc\0".as_ptr() as *const c_char,
            proc_path: b"/proc/1/ns/ipc\0".as_ptr() as *const c_char,
            expected_ino: IPC_NS_INIT_INO,
        },
        ns_info {
            name: b"uts\0".as_ptr() as *const c_char,
            proc_path: b"/proc/1/ns/uts\0".as_ptr() as *const c_char,
            expected_ino: UTS_NS_INIT_INO,
        },
        ns_info {
            name: b"user\0".as_ptr() as *const c_char,
            proc_path: b"/proc/1/ns/user\0".as_ptr() as *const c_char,
            expected_ino: USER_NS_INIT_INO,
        },
        ns_info {
            name: b"pid\0".as_ptr() as *const c_char,
            proc_path: b"/proc/1/ns/pid\0".as_ptr() as *const c_char,
            expected_ino: PID_NS_INIT_INO,
        },
        ns_info {
            name: b"cgroup\0".as_ptr() as *const c_char,
            proc_path: b"/proc/1/ns/cgroup\0".as_ptr() as *const c_char,
            expected_ino: CGROUP_NS_INIT_INO,
        },
        ns_info {
            name: b"time\0".as_ptr() as *const c_char,
            proc_path: b"/proc/1/ns/time\0".as_ptr() as *const c_char,
            expected_ino: TIME_NS_INIT_INO,
        },
        ns_info {
            name: b"net\0".as_ptr() as *const c_char,
            proc_path: b"/proc/1/ns/net\0".as_ptr() as *const c_char,
            expected_ino: NET_NS_INIT_INO,
        },
        ns_info {
            name: b"mnt\0".as_ptr() as *const c_char,
            proc_path: b"/proc/1/ns/mnt\0".as_ptr() as *const c_char,
            expected_ino: MNT_NS_INIT_INO,
        },
    ]
};

unsafe fn init_namespace_inodes() {
    let mut st: libc::stat = core::mem::zeroed();

    for i in 0..(core::mem::size_of_val(&namespaces) / core::mem::size_of::<ns_info>()) {
        let ret = stat(namespaces[i].proc_path, &mut st);

        /* Some namespaces might not be available (e.g., time namespace on older kernels) */
        if ret < 0 {
            if *__errno_location() == ENOENT {
                ksft_test_result_skip(
                    b"%s namespace not available\n\0".as_ptr() as *const c_char,
                    namespaces[i].name,
                );
                continue;
            }
            assert!(ret >= 0);
            // TH_LOG("Failed to stat %s: %s", namespaces[i].proc_path, strerror(errno));
            eprintln!(
                "Failed to stat {:?}: {:?}",
                namespaces[i].proc_path,
                strerror(*__errno_location())
            );
        }

        assert_eq!(st.st_ino, namespaces[i].expected_ino as _);
        // TH_LOG("Namespace %s has inode 0x%lx, expected 0x%x",
        //        namespaces[i].name, st.st_ino, namespaces[i].expected_ino);

        ksft_print_msg(
            b"Namespace %s: inode 0x%lx matches expected 0x%x\n\0".as_ptr() as *const c_char,
            namespaces[i].name,
            st.st_ino,
            namespaces[i].expected_ino,
        );
    }
}

fn main() {
    unsafe {
        init_namespace_inodes();
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
