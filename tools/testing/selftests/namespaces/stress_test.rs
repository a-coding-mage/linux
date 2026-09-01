// SPDX-License-Identifier: GPL-2.0
// C includes translated as external dependencies/imports:
// errno.h, fcntl.h, limits.h, sched.h, stdio.h, stdlib.h, string.h,
// sys/ioctl.h, sys/socket.h, sys/stat.h, sys/syscall.h, sys/types.h,
// sys/wait.h, unistd.h, linux/nsfs.h, ../kselftest_harness.h,
// ../filesystems/utils.h, wrappers.h

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type SsizeT = isize;
type PidT = c_int;
type UidT = c_uint;
type U64 = u64;

const ENOSYS: c_int = 38;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SIGKILL: c_int = 9;
const CLONE_NEWUSER: c_int = 0x10000000;
const CLONE_NEWNET: c_int = 0x40000000;
const CLONE_NEWUTS: c_int = 0x04000000;
const CLONE_NEWIPC: c_int = 0x08000000;

#[repr(C)]
struct ns_id_req {
    size: c_ulong,
    spare: U64,
    ns_id: U64,
    ns_type: c_int,
    spare2: U64,
    user_ns_id: U64,
}

unsafe extern "C" {
    fn fork() -> PidT;
    fn exit(status: c_int) -> !;
    fn waitpid(pid: PidT, status: *mut c_int, options: c_int) -> PidT;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> SsizeT;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> SsizeT;
    fn kill(pid: PidT, sig: c_int) -> c_int;
    fn unshare(flags: c_int) -> c_int;
    fn setns(fd: c_int, nstype: c_int) -> c_int;
    fn getuid() -> UidT;
    fn usleep(usec: c_uint) -> c_int;

    fn setup_userns() -> c_int;
    fn get_userns_fd(fd: c_int, uid: UidT, map: c_int) -> c_int;
    fn sys_listns(req: *mut ns_id_req, ns_ids: *mut U64, size: usize, flags: c_uint) -> SsizeT;
    fn __errno_location() -> *mut c_int;
}

fn errno() -> c_int {
    unsafe { *__errno_location() }
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

macro_rules! assert_ge {
    ($left:expr, $right:expr) => {
        assert!($left >= $right, "ASSERT_GE({}, {})", stringify!($left), stringify!($right));
    };
}

macro_rules! assert_eq_c {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right, "ASSERT_EQ({}, {})", stringify!($left), stringify!($right));
    };
}

macro_rules! assert_true {
    ($expr:expr) => {
        assert!($expr, "ASSERT_TRUE({})", stringify!($expr));
    };
}

macro_rules! th_log {
    ($($arg:tt)*) => {
        eprintln!($($arg)*);
    };
}

macro_rules! skip_return {
    ($($arg:tt)*) => {{
        th_log!($($arg)*);
        return;
    }};
}

/*
 * Stress tests for namespace active reference counting.
 *
 * These tests validate that the active reference counting system can handle
 * high load scenarios including rapid namespace creation/destruction, large
 * numbers of concurrent namespaces, and various edge cases under stress.
 */

/*
 * Test rapid creation and destruction of user namespaces.
 * Create and destroy namespaces in quick succession to stress the
 * active reference tracking and ensure no leaks occur.
 */
fn rapid_namespace_creation_destruction() {
    let mut req = ns_id_req {
        size: size_of::<ns_id_req>() as c_ulong,
        spare: 0,
        ns_id: 0,
        ns_type: CLONE_NEWUSER,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids_before = [0_u64; 256];
    let mut ns_ids_after = [0_u64; 256];
    let ret_before: SsizeT;
    let ret_after: SsizeT;
    let mut i: c_int;

    /* Get baseline count of active user namespaces */
    unsafe {
        ret_before = sys_listns(&mut req, ns_ids_before.as_mut_ptr(), ns_ids_before.len(), 0);
    }
    if ret_before < 0 {
        if errno() == ENOSYS {
            skip_return!("listns() not supported");
        }
        assert_ge!(ret_before, 0);
    }

    th_log!("Baseline: {} active user namespaces", ret_before);

    /* Rapidly create and destroy 100 user namespaces */
    i = 0;
    while i < 100 {
        let pid = unsafe { fork() };
        assert_ge!(pid, 0);

        if pid == 0 {
            /* Child: create user namespace and immediately exit */
            unsafe {
                if setup_userns() < 0 {
                    exit(1);
                }
                exit(0);
            }
        }

        /* Parent: wait for child */
        let mut status: c_int = 0;
        unsafe {
            waitpid(pid, &mut status, 0);
        }
        assert_true!(wifexited(status));
        assert_eq_c!(wexitstatus(status), 0);
        i += 1;
    }

    /* Verify we're back to baseline (no leaked namespaces) */
    unsafe {
        ret_after = sys_listns(&mut req, ns_ids_after.as_mut_ptr(), ns_ids_after.len(), 0);
    }
    assert_ge!(ret_after, 0);

    th_log!("After 100 rapid create/destroy cycles: {} active user namespaces", ret_after);
    assert_eq_c!(ret_before, ret_after);
}

/*
 * Test creating many concurrent namespaces.
 * Verify that listns() correctly tracks all of them and that they all
 * become inactive after processes exit.
 */
fn many_concurrent_namespaces() {
    let mut req = ns_id_req {
        size: size_of::<ns_id_req>() as c_ulong,
        spare: 0,
        ns_id: 0,
        ns_type: CLONE_NEWUSER,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids_before = [0_u64; 512];
    let mut ns_ids_during = [0_u64; 512];
    let mut ns_ids_after = [0_u64; 512];
    let ret_before: SsizeT;
    let ret_during: SsizeT;
    let ret_after: SsizeT;
    let mut pids = [0 as PidT; 50];
    let num_children: c_int = 50;
    let mut i: c_int;
    let mut sv = [0 as c_int; 2];

    /* Get baseline */
    unsafe {
        ret_before = sys_listns(&mut req, ns_ids_before.as_mut_ptr(), ns_ids_before.len(), 0);
    }
    if ret_before < 0 {
        if errno() == ENOSYS {
            skip_return!("listns() not supported");
        }
        assert_ge!(ret_before, 0);
    }

    th_log!("Baseline: {} active user namespaces", ret_before);

    assert_eq_c!(unsafe { socketpair(AF_UNIX, SOCK_STREAM, 0, sv.as_mut_ptr()) }, 0);

    /* Create many children, each with their own user namespace */
    i = 0;
    while i < num_children {
        pids[i as usize] = unsafe { fork() };
        assert_ge!(pids[i as usize], 0);

        if pids[i as usize] == 0 {
            /* Child: create user namespace and wait for parent signal */
            let mut c: c_char = 0;

            unsafe {
                close(sv[0]);

                if setup_userns() < 0 {
                    close(sv[1]);
                    exit(1);
                }

                /* Signal parent we're ready */
                if write(sv[1], &c as *const _ as *const c_void, 1) != 1 {
                    close(sv[1]);
                    exit(1);
                }

                /* Wait for parent signal to exit */
                if read(sv[1], &mut c as *mut _ as *mut c_void, 1) != 1 {
                    close(sv[1]);
                    exit(1);
                }

                close(sv[1]);
                exit(0);
            }
        }
        i += 1;
    }

    unsafe {
        close(sv[1]);
    }

    /* Wait for all children to signal ready */
    i = 0;
    while i < num_children {
        let mut c: c_char = 0;
        if unsafe { read(sv[0], &mut c as *mut _ as *mut c_void, 1) } != 1 {
            /* If we fail to read, kill all children and exit */
            unsafe {
                close(sv[0]);
            }
            let mut j: c_int = 0;
            while j < num_children {
                unsafe {
                    kill(pids[j as usize], SIGKILL);
                }
                j += 1;
            }
            j = 0;
            while j < num_children {
                unsafe {
                    waitpid(pids[j as usize], ptr::null_mut(), 0);
                }
                j += 1;
            }
            assert_true!(false);
        }
        i += 1;
    }

    /* List namespaces while all children are running */
    unsafe {
        ret_during = sys_listns(&mut req, ns_ids_during.as_mut_ptr(), ns_ids_during.len(), 0);
    }
    assert_ge!(ret_during, 0);

    th_log!("With {} children running: {} active user namespaces", num_children, ret_during);

    /* Should have at least num_children more namespaces than baseline */
    assert_ge!(ret_during, ret_before + num_children as isize);

    /* Signal all children to exit */
    i = 0;
    while i < num_children {
        let c: c_char = b'X' as c_char;
        if unsafe { write(sv[0], &c as *const _ as *const c_void, 1) } != 1 {
            /* If we fail to write, kill remaining children */
            unsafe {
                close(sv[0]);
            }
            let mut j = i;
            while j < num_children {
                unsafe {
                    kill(pids[j as usize], SIGKILL);
                }
                j += 1;
            }
            j = 0;
            while j < num_children {
                unsafe {
                    waitpid(pids[j as usize], ptr::null_mut(), 0);
                }
                j += 1;
            }
            assert_true!(false);
        }
        i += 1;
    }

    unsafe {
        close(sv[0]);
    }

    /* Wait for all children */
    i = 0;
    while i < num_children {
        let mut status: c_int = 0;
        unsafe {
            waitpid(pids[i as usize], &mut status, 0);
        }
        assert_true!(wifexited(status));
        i += 1;
    }

    /* Verify we're back to baseline */
    unsafe {
        ret_after = sys_listns(&mut req, ns_ids_after.as_mut_ptr(), ns_ids_after.len(), 0);
    }
    assert_ge!(ret_after, 0);

    th_log!("After all children exit: {} active user namespaces", ret_after);
    assert_eq_c!(ret_before, ret_after);
}

/*
 * Test rapid namespace creation with different namespace types.
 * Create multiple types of namespaces rapidly to stress the tracking system.
 */
fn rapid_mixed_namespace_creation() {
    let mut req = ns_id_req {
        size: size_of::<ns_id_req>() as c_ulong,
        spare: 0,
        ns_id: 0,
        ns_type: 0, /* All types */
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids_before = [0_u64; 512];
    let mut ns_ids_after = [0_u64; 512];
    let ret_before: SsizeT;
    let ret_after: SsizeT;
    let mut i: c_int;

    /* Get baseline count */
    unsafe {
        ret_before = sys_listns(&mut req, ns_ids_before.as_mut_ptr(), ns_ids_before.len(), 0);
    }
    if ret_before < 0 {
        if errno() == ENOSYS {
            skip_return!("listns() not supported");
        }
        assert_ge!(ret_before, 0);
    }

    th_log!("Baseline: {} active namespaces (all types)", ret_before);

    /* Rapidly create and destroy namespaces with multiple types */
    i = 0;
    while i < 50 {
        let pid = unsafe { fork() };
        assert_ge!(pid, 0);

        if pid == 0 {
            /* Child: create multiple namespace types */
            unsafe {
                if setup_userns() < 0 {
                    exit(1);
                }

                /* Create additional namespace types */
                if unshare(CLONE_NEWNET) < 0 {
                    exit(1);
                }
                if unshare(CLONE_NEWUTS) < 0 {
                    exit(1);
                }
                if unshare(CLONE_NEWIPC) < 0 {
                    exit(1);
                }

                exit(0);
            }
        }

        /* Parent: wait for child */
        let mut status: c_int = 0;
        unsafe {
            waitpid(pid, &mut status, 0);
        }
        assert_true!(wifexited(status));
        i += 1;
    }

    /* Verify we're back to baseline */
    unsafe {
        ret_after = sys_listns(&mut req, ns_ids_after.as_mut_ptr(), ns_ids_after.len(), 0);
    }
    assert_ge!(ret_after, 0);

    th_log!("After 50 rapid mixed namespace cycles: {} active namespaces", ret_after);
    assert_eq_c!(ret_before, ret_after);
}

/*
 * Test nested namespace creation under stress.
 * Create deeply nested namespace hierarchies and verify proper cleanup.
 */
fn nested_namespace_stress() {
    let mut req = ns_id_req {
        size: size_of::<ns_id_req>() as c_ulong,
        spare: 0,
        ns_id: 0,
        ns_type: CLONE_NEWUSER,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids_before = [0_u64; 512];
    let mut ns_ids_after = [0_u64; 512];
    let ret_before: SsizeT;
    let ret_after: SsizeT;
    let mut i: c_int;

    /* Get baseline */
    unsafe {
        ret_before = sys_listns(&mut req, ns_ids_before.as_mut_ptr(), ns_ids_before.len(), 0);
    }
    if ret_before < 0 {
        if errno() == ENOSYS {
            skip_return!("listns() not supported");
        }
        assert_ge!(ret_before, 0);
    }

    th_log!("Baseline: {} active user namespaces", ret_before);

    /* Create 20 processes, each with nested user namespaces */
    i = 0;
    while i < 20 {
        let pid = unsafe { fork() };
        assert_ge!(pid, 0);

        if pid == 0 {
            let mut userns_fd: c_int;
            let orig_uid: UidT = unsafe { getuid() };
            let mut depth: c_int;

            /* Create nested user namespaces (up to 5 levels) */
            depth = 0;
            while depth < 5 {
                userns_fd = unsafe { get_userns_fd(0, if depth == 0 { orig_uid } else { 0 }, 1) };
                if userns_fd < 0 {
                    unsafe {
                        exit(1);
                    }
                }

                if unsafe { setns(userns_fd, CLONE_NEWUSER) } < 0 {
                    unsafe {
                        close(userns_fd);
                        exit(1);
                    }
                }
                unsafe {
                    close(userns_fd);
                }
                depth += 1;
            }

            unsafe {
                exit(0);
            }
        }

        /* Parent: wait for child */
        let mut status: c_int = 0;
        unsafe {
            waitpid(pid, &mut status, 0);
        }
        assert_true!(wifexited(status));
        i += 1;
    }

    /* Verify we're back to baseline */
    unsafe {
        ret_after = sys_listns(&mut req, ns_ids_after.as_mut_ptr(), ns_ids_after.len(), 0);
    }
    assert_ge!(ret_after, 0);

    th_log!("After 20 nested namespace hierarchies: {} active user namespaces", ret_after);
    assert_eq_c!(ret_before, ret_after);
}

/*
 * Test listns() pagination under stress.
 * Create many namespaces and verify pagination works correctly.
 */
fn listns_pagination_stress() {
    let mut req = ns_id_req {
        size: size_of::<ns_id_req>() as c_ulong,
        spare: 0,
        ns_id: 0,
        ns_type: CLONE_NEWUSER,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut pids = [0 as PidT; 30];
    let num_children: c_int = 30;
    let mut i: c_int;
    let mut sv = [0 as c_int; 2];
    let mut all_ns_ids = [0_u64; 512];
    let mut total_found: c_int = 0;

    assert_eq_c!(unsafe { socketpair(AF_UNIX, SOCK_STREAM, 0, sv.as_mut_ptr()) }, 0);

    /* Create many children with user namespaces */
    i = 0;
    while i < num_children {
        pids[i as usize] = unsafe { fork() };
        assert_ge!(pids[i as usize], 0);

        if pids[i as usize] == 0 {
            let mut c: c_char = 0;
            unsafe {
                close(sv[0]);

                if setup_userns() < 0 {
                    close(sv[1]);
                    exit(1);
                }

                /* Signal parent we're ready */
                if write(sv[1], &c as *const _ as *const c_void, 1) != 1 {
                    close(sv[1]);
                    exit(1);
                }

                /* Wait for parent signal to exit */
                if read(sv[1], &mut c as *mut _ as *mut c_void, 1) != 1 {
                    close(sv[1]);
                    exit(1);
                }

                close(sv[1]);
                exit(0);
            }
        }
        i += 1;
    }

    unsafe {
        close(sv[1]);
    }

    /* Wait for all children to signal ready */
    i = 0;
    while i < num_children {
        let mut c: c_char = 0;
        if unsafe { read(sv[0], &mut c as *mut _ as *mut c_void, 1) } != 1 {
            /* If we fail to read, kill all children and exit */
            unsafe {
                close(sv[0]);
            }
            let mut j: c_int = 0;
            while j < num_children {
                unsafe {
                    kill(pids[j as usize], SIGKILL);
                }
                j += 1;
            }
            j = 0;
            while j < num_children {
                unsafe {
                    waitpid(pids[j as usize], ptr::null_mut(), 0);
                }
                j += 1;
            }
            assert_true!(false);
        }
        i += 1;
    }

    /* Paginate through all namespaces using small batch sizes */
    req.ns_id = 0;
    loop {
        let mut batch = [0_u64; 5]; /* Small batch size to force pagination */
        let ret: SsizeT;

        unsafe {
            ret = sys_listns(&mut req, batch.as_mut_ptr(), batch.len(), 0);
        }
        if ret < 0 {
            if errno() == ENOSYS {
                unsafe {
                    close(sv[0]);
                }
                i = 0;
                while i < num_children {
                    unsafe {
                        kill(pids[i as usize], SIGKILL);
                    }
                    i += 1;
                }
                i = 0;
                while i < num_children {
                    unsafe {
                        waitpid(pids[i as usize], ptr::null_mut(), 0);
                    }
                    i += 1;
                }
                skip_return!("listns() not supported");
            }
            assert_ge!(ret, 0);
        }

        if ret == 0 {
            break;
        }

        /* Store results */
        i = 0;
        while (i as isize) < ret && total_found < 512 {
            all_ns_ids[total_found as usize] = batch[i as usize];
            total_found += 1;
            i += 1;
        }

        /* Update cursor for next batch */
        if ret == batch.len() as isize {
            req.ns_id = batch[(ret - 1) as usize];
        } else {
            break;
        }
    }

    th_log!("Paginated through {} user namespaces", total_found);

    /* Verify no duplicates in pagination */
    i = 0;
    while i < total_found {
        let mut j = i + 1;
        while j < total_found {
            if all_ns_ids[i as usize] == all_ns_ids[j as usize] {
                th_log!(
                    "Found duplicate ns_id: {} at positions {} and {}",
                    all_ns_ids[i as usize],
                    i,
                    j
                );
                assert_true!(false);
            }
            j += 1;
        }
        i += 1;
    }

    /* Signal all children to exit */
    i = 0;
    while i < num_children {
        let c: c_char = b'X' as c_char;
        if unsafe { write(sv[0], &c as *const _ as *const c_void, 1) } != 1 {
            unsafe {
                close(sv[0]);
            }
            let mut j = i;
            while j < num_children {
                unsafe {
                    kill(pids[j as usize], SIGKILL);
                }
                j += 1;
            }
            j = 0;
            while j < num_children {
                unsafe {
                    waitpid(pids[j as usize], ptr::null_mut(), 0);
                }
                j += 1;
            }
            assert_true!(false);
        }
        i += 1;
    }

    unsafe {
        close(sv[0]);
    }

    /* Wait for all children */
    i = 0;
    while i < num_children {
        let mut status: c_int = 0;
        unsafe {
            waitpid(pids[i as usize], &mut status, 0);
        }
        i += 1;
    }
}

/*
 * Test concurrent namespace operations.
 * Multiple processes creating, querying, and destroying namespaces concurrently.
 */
fn concurrent_namespace_operations() {
    let mut req = ns_id_req {
        size: size_of::<ns_id_req>() as c_ulong,
        spare: 0,
        ns_id: 0,
        ns_type: 0,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids_before = [0_u64; 512];
    let mut ns_ids_after = [0_u64; 512];
    let ret_before: SsizeT;
    let ret_after: SsizeT;
    let mut pids = [0 as PidT; 20];
    let num_workers: c_int = 20;
    let mut i: c_int;

    /* Get baseline */
    unsafe {
        ret_before = sys_listns(&mut req, ns_ids_before.as_mut_ptr(), ns_ids_before.len(), 0);
    }
    if ret_before < 0 {
        if errno() == ENOSYS {
            skip_return!("listns() not supported");
        }
        assert_ge!(ret_before, 0);
    }

    th_log!("Baseline: {} active namespaces", ret_before);

    /* Create worker processes that do concurrent operations */
    i = 0;
    while i < num_workers {
        pids[i as usize] = unsafe { fork() };
        assert_ge!(pids[i as usize], 0);

        if pids[i as usize] == 0 {
            /* Each worker: create namespaces, list them, repeat */
            let mut iterations: c_int;

            iterations = 0;
            while iterations < 10 {
                let userns_fd: c_int;
                let mut temp_ns_ids = [0_u64; 100];
                let ret: SsizeT;

                /* Create a user namespace */
                userns_fd = unsafe { get_userns_fd(0, getuid(), 1) };
                if userns_fd < 0 {
                    iterations += 1;
                    continue;
                }

                /* List namespaces */
                unsafe {
                    ret = sys_listns(&mut req, temp_ns_ids.as_mut_ptr(), temp_ns_ids.len(), 0);
                }
                let _ = ret;

                unsafe {
                    close(userns_fd);
                }

                /* Small delay */
                unsafe {
                    usleep(1000);
                }
                iterations += 1;
            }

            unsafe {
                exit(0);
            }
        }
        i += 1;
    }

    /* Wait for all workers */
    i = 0;
    while i < num_workers {
        let mut status: c_int = 0;
        unsafe {
            waitpid(pids[i as usize], &mut status, 0);
        }
        assert_true!(wifexited(status));
        assert_eq_c!(wexitstatus(status), 0);
        i += 1;
    }

    /* Verify we're back to baseline */
    unsafe {
        ret_after = sys_listns(&mut req, ns_ids_after.as_mut_ptr(), ns_ids_after.len(), 0);
    }
    assert_ge!(ret_after, 0);

    th_log!("After concurrent operations: {} active namespaces", ret_after);
    assert_eq_c!(ret_before, ret_after);
}

/*
 * Test namespace churn - continuous creation and destruction.
 * Simulates high-churn scenarios like container orchestration.
 */
fn namespace_churn() {
    let mut req = ns_id_req {
        size: size_of::<ns_id_req>() as c_ulong,
        spare: 0,
        ns_id: 0,
        ns_type: CLONE_NEWUSER | CLONE_NEWNET | CLONE_NEWUTS,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids_before = [0_u64; 512];
    let mut ns_ids_after = [0_u64; 512];
    let ret_before: SsizeT;
    let ret_after: SsizeT;
    let mut cycle: c_int;

    /* Get baseline */
    unsafe {
        ret_before = sys_listns(&mut req, ns_ids_before.as_mut_ptr(), ns_ids_before.len(), 0);
    }
    if ret_before < 0 {
        if errno() == ENOSYS {
            skip_return!("listns() not supported");
        }
        assert_ge!(ret_before, 0);
    }

    th_log!("Baseline: {} active namespaces", ret_before);

    /* Simulate churn: batches of namespaces created and destroyed */
    cycle = 0;
    while cycle < 10 {
        let mut batch_pids = [0 as PidT; 10];
        let mut i: c_int;

        /* Create batch */
        i = 0;
        while i < 10 {
            batch_pids[i as usize] = unsafe { fork() };
            assert_ge!(batch_pids[i as usize], 0);

            if batch_pids[i as usize] == 0 {
                /* Create multiple namespace types */
                unsafe {
                    if setup_userns() < 0 {
                        exit(1);
                    }
                    if unshare(CLONE_NEWNET) < 0 {
                        exit(1);
                    }
                    if unshare(CLONE_NEWUTS) < 0 {
                        exit(1);
                    }

                    /* Keep namespaces alive briefly */
                    usleep(10000);
                    exit(0);
                }
            }
            i += 1;
        }

        /* Wait for batch to complete */
        i = 0;
        while i < 10 {
            let mut status: c_int = 0;
            unsafe {
                waitpid(batch_pids[i as usize], &mut status, 0);
            }
            i += 1;
        }
        cycle += 1;
    }

    /* Verify we're back to baseline */
    unsafe {
        ret_after = sys_listns(&mut req, ns_ids_after.as_mut_ptr(), ns_ids_after.len(), 0);
    }
    assert_ge!(ret_after, 0);

    th_log!("After 10 churn cycles (100 namespace sets): {} active namespaces", ret_after);
    assert_eq_c!(ret_before, ret_after);
}

fn main() {
    rapid_namespace_creation_destruction();
    many_concurrent_namespaces();
    rapid_mixed_namespace_creation();
    nested_namespace_stress();
    listns_pagination_stress();
    concurrent_namespace_operations();
    namespace_churn();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
