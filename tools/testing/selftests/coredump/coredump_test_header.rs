/* SPDX-License-Identifier: GPL-2.0 */

// Translated from testing/selftests/coredump/coredump_test.h.
// C include dependencies: stdbool.h, sys/types.h, linux/coredump.h,
// ../kselftest_harness.h, ../pidfd/pidfd.h.

use core::ffi::{c_char, c_int, c_void};

// Original C condition:
// #ifndef PAGE_SIZE
// #define PAGE_SIZE 4096
// #endif
pub const PAGE_SIZE: usize = 4096;

pub const NUM_THREAD_SPAWN: c_int = 128;

/* Coredump fixture */
#[repr(C)]
pub struct coredump {
    pub original_core_pattern: [c_char; 256],
    pub pid_coredump_server: pid_t,
    pub fd_tmpfs_detached: c_int,
}

/* Shared helper function declarations */
unsafe extern "C" {
    pub fn do_nothing(arg: *mut c_void) -> *mut c_void;
    pub fn crashing_child();
    pub fn create_detached_tmpfs() -> c_int;
    pub fn create_and_listen_unix_socket(path: *const c_char) -> c_int;
    pub fn set_core_pattern(pattern: *const c_char) -> bool;
    pub fn get_peer_pidfd(fd: c_int) -> c_int;
    pub fn get_pidfd_info(fd_peer_pidfd: c_int, info: *mut pidfd_info) -> bool;
}

/* Inline helper that uses harness types */
pub unsafe fn wait_and_check_coredump_server(
    pid_coredump_server: pid_t,
    _metadata: *const __test_metadata,
    self_: *mut coredump,
) {
    let mut status: c_int = 0;
    unsafe {
        waitpid(pid_coredump_server, &mut status as *mut c_int, 0);
        (*self_).pid_coredump_server = -ESRCH;
        ASSERT_TRUE(WIFEXITED(status));
        ASSERT_EQ(WEXITSTATUS(status), 0);
    }
}

/* Protocol helper function declarations */
unsafe extern "C" {
    pub fn recv_marker(fd: c_int) -> ssize_t;
    pub fn read_marker(fd: c_int, mark: coredump_mark) -> bool;
    pub fn read_coredump_req(fd: c_int, req: *mut coredump_req) -> bool;
    pub fn send_coredump_ack(
        fd: c_int,
        req: *const coredump_req,
        mask: __u64,
        size_ack: size_t,
    ) -> bool;
    pub fn check_coredump_req(
        req: *const coredump_req,
        min_size: size_t,
        required_mask: __u64,
    ) -> bool;
    pub fn open_coredump_tmpfile(fd_tmpfs_detached: c_int) -> c_int;
    pub fn process_coredump_worker(fd_coredump: c_int, fd_peer_pidfd: c_int, fd_core_file: c_int);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
