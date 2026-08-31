// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and included:
// <errno.h>, <sched.h>, <stdio.h>, <stdlib.h>, <sys/socket.h>,
// <sys/wait.h>, <unistd.h>, "../kselftest_harness.h",
// "../filesystems/utils.h", and "wrappers.h".

use core::ffi::{c_char, c_int, c_long, c_void};
use core::mem::size_of;
use core::ptr;

type PidT = c_int;
type SsizeT = isize;
type U64 = u64;

const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const CLONE_NEWUSER: c_int = 0x10000000;
const SIGKILL: c_int = 9;
const ENOSYS: c_int = 38;

#[repr(C)]
struct ns_id_req {
    size: u32,
    spare: u32,
    ns_id: U64,
    ns_type: u32,
    spare2: u32,
    user_ns_id: U64,
}

unsafe extern "C" {
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn fork() -> PidT;
    fn close(fd: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> SsizeT;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> SsizeT;
    fn kill(pid: PidT, sig: c_int) -> c_int;
    fn waitpid(pid: PidT, status: *mut c_int, options: c_int) -> PidT;
    fn setup_userns() -> c_int;
    fn sys_listns(
        req: *mut ns_id_req,
        nsids: *mut U64,
        size: usize,
        flags: c_int,
    ) -> SsizeT;
    fn __errno_location() -> *mut c_int;
    fn printf(format: *const c_char, ...) -> c_int;
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right);
    };
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {
        assert!($left >= $right);
    };
}

macro_rules! ASSERT_TRUE {
    ($expr:expr) => {
        assert!($expr);
    };
}

macro_rules! TH_LOG {
    ($fmt:expr $(, $arg:expr)* $(,)?) => {{
        unsafe {
            printf(concat!($fmt, "\n\0").as_ptr() as *const c_char $(, $arg)*);
        }
    }};
}

macro_rules! SKIP {
    (return, $fmt:expr $(, $arg:expr)* $(,)?) => {{
        TH_LOG!($fmt $(, $arg)*);
        return;
    }};
}

/*
 * Minimal test case to reproduce KASAN out-of-bounds in listns pagination.
 *
 * The bug occurs when:
 * 1. Filtering by a specific namespace type (e.g., CLONE_NEWUSER)
 * 2. Using pagination (req.ns_id != 0)
 * 3. The lookup_ns_id_at() call in do_listns() passes ns_type=0 instead of
 *    the filtered type, causing it to search the unified tree and potentially
 *    return a namespace of the wrong type.
 */
unsafe fn pagination_with_type_filter() {
    let mut req = ns_id_req {
        size: size_of::<ns_id_req>() as u32,
        spare: 0,
        ns_id: 0,
        ns_type: CLONE_NEWUSER as u32, /* Filter by user namespace */
        spare2: 0,
        user_ns_id: 0,
    };
    let mut pids: [PidT; 10] = [0; 10];
    let num_children: c_int = 10;
    let mut i: c_int;
    let mut sv: [c_int; 2] = [0; 2];
    let mut first_batch: [U64; 3] = [0; 3];
    let mut ret: SsizeT;

    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, sv.as_mut_ptr()), 0);

    /* Create children with user namespaces */
    i = 0;
    while i < num_children {
        pids[i as usize] = fork();
        ASSERT_GE!(pids[i as usize], 0);

        if pids[i as usize] == 0 {
            let mut c: c_char = 0;
            close(sv[0]);

            if setup_userns() < 0 {
                close(sv[1]);
                exit(1);
            }

            /* Signal parent we're ready */
            if write(sv[1], &mut c as *mut c_char as *const c_void, 1) != 1 {
                close(sv[1]);
                exit(1);
            }

            /* Wait for parent signal to exit */
            if read(sv[1], &mut c as *mut c_char as *mut c_void, 1) != 1 {
                close(sv[1]);
                exit(1);
            }

            close(sv[1]);
            exit(0);
        }

        i += 1;
    }

    close(sv[1]);

    /* Wait for all children to signal ready */
    i = 0;
    while i < num_children {
        let mut c: c_char = 0;
        if read(sv[0], &mut c as *mut c_char as *mut c_void, 1) != 1 {
            close(sv[0]);
            let mut j: c_int = 0;
            while j < num_children {
                kill(pids[j as usize], SIGKILL);
                j += 1;
            }
            j = 0;
            while j < num_children {
                waitpid(pids[j as usize], ptr::null_mut(), 0);
                j += 1;
            }
            ASSERT_TRUE!(false);
        }
        i += 1;
    }

    /* First batch - this should work */
    ret = sys_listns(&mut req, first_batch.as_mut_ptr(), 3, 0);
    if ret < 0 {
        if *__errno_location() == ENOSYS {
            close(sv[0]);
            i = 0;
            while i < num_children {
                kill(pids[i as usize], SIGKILL);
                i += 1;
            }
            i = 0;
            while i < num_children {
                waitpid(pids[i as usize], ptr::null_mut(), 0);
                i += 1;
            }
            SKIP!(return, "listns() not supported");
        }
        ASSERT_GE!(ret, 0);
    }

    TH_LOG!("First batch returned %zd entries", ret as c_long);

    if ret == 3 {
        let mut second_batch: [U64; 3] = [0; 3];

        /* Second batch - pagination triggers the bug */
        req.ns_id = first_batch[2]; /* Continue from last ID */
        ret = sys_listns(&mut req, second_batch.as_mut_ptr(), 3, 0);

        TH_LOG!("Second batch returned %zd entries", ret as c_long);
        ASSERT_GE!(ret, 0);
    }

    /* Signal all children to exit */
    i = 0;
    while i < num_children {
        let c: c_char = b'X' as c_char;
        if write(sv[0], &c as *const c_char as *const c_void, 1) != 1 {
            close(sv[0]);
            let mut j: c_int = i;
            while j < num_children {
                kill(pids[j as usize], SIGKILL);
                j += 1;
            }
            j = 0;
            while j < num_children {
                waitpid(pids[j as usize], ptr::null_mut(), 0);
                j += 1;
            }
            ASSERT_TRUE!(false);
        }
        i += 1;
    }

    close(sv[0]);

    /* Cleanup */
    i = 0;
    while i < num_children {
        let mut status: c_int = 0;
        waitpid(pids[i as usize], &mut status, 0);
        i += 1;
    }
}

fn main() {
    unsafe {
        pagination_with_type_filter();
    }
}
