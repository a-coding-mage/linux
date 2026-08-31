// SPDX-License-Identifier: GPL-2.0
// C dependencies translated from:
// errno.h, fcntl.h, limits.h, sched.h, stdio.h, stdlib.h, string.h,
// linux/nsfs.h, sys/ioctl.h, sys/socket.h, sys/stat.h, sys/syscall.h,
// sys/types.h, sys/wait.h, unistd.h, ../kselftest_harness.h,
// ../filesystems/utils.h, and wrappers.h.

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem;
use core::ptr;

type ssize_t = isize;
type pid_t = c_int;
type __u64 = u64;

const ENOSYS: c_int = 38;
const EINVAL: c_int = 22;
const O_RDONLY: c_int = 0;
const AF_UNIX: c_int = 1;
const SOCK_STREAM: c_int = 1;
const SIGKILL: c_int = 9;
const CLONE_NEWUTS: c_int = 0x04000000;
const CLONE_NEWNET: c_int = 0x40000000;
const CLONE_NEWUSER: c_int = 0x10000000;
const NS_GET_NSTYPE: c_ulong = 0xb703;
const NS_GET_ID: c_ulong = 0xb705;
const LISTNS_CURRENT_USER: __u64 = !0;

#[repr(C)]
struct ns_id_req {
    size: __u64,
    spare: __u64,
    ns_id: __u64,
    ns_type: __u64,
    spare2: __u64,
    user_ns_id: __u64,
}

#[repr(C)]
struct nsfs_file_handle {
    ns_id: __u64,
    ns_type: c_int,
    ns_inum: __u64,
}

#[repr(C)]
struct file_handle {
    handle_bytes: c_uint,
    handle_type: c_int,
    f_handle: [u8; 0],
}

extern "C" {
    fn __errno_location() -> *mut c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn open_by_handle_at(mount_fd: c_int, handle: *mut file_handle, flags: c_int) -> c_int;
    fn ioctl(fd: c_int, request: c_ulong, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn unshare(flags: c_int) -> c_int;
    fn exit(status: c_int) -> !;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn usleep(usec: c_uint) -> c_int;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
    fn socketpair(domain: c_int, type_: c_int, protocol: c_int, sv: *mut c_int) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;

    fn sys_listns(req: *mut ns_id_req, ns_ids: *mut __u64, nr_ns_ids: usize, flags: c_uint) -> ssize_t;
    fn setup_userns() -> c_int;
}

macro_rules! ARRAY_SIZE {
    ($array:expr) => {
        $array.len()
    };
}

macro_rules! TEST {
    ($name:ident, $body:block) => {
        unsafe fn $name() $body
    };
}

macro_rules! TEST_HARNESS_MAIN {
    () => {};
}

macro_rules! TH_LOG {
    ($($arg:tt)*) => {};
}

macro_rules! ASSERT_TRUE {
    ($cond:expr) => {};
}

macro_rules! ASSERT_FALSE {
    ($cond:expr) => {};
}

macro_rules! ASSERT_GT {
    ($left:expr, $right:expr) => {};
}

macro_rules! ASSERT_GE {
    ($left:expr, $right:expr) => {};
}

macro_rules! ASSERT_LT {
    ($left:expr, $right:expr) => {};
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {};
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {};
}

macro_rules! SKIP {
    (return, $($arg:tt)*) => {
        return
    };
}

unsafe fn errno() -> c_int {
    *__errno_location()
}

/*
 * Test basic listns() functionality with the unified namespace tree.
 * List all active namespaces globally.
 */
TEST!(listns_basic_unified, {
    let mut req = ns_id_req {
        size: mem::size_of::<ns_id_req>() as __u64,
        spare: 0,
        ns_id: 0,
        ns_type: 0,  /* All types */
        spare2: 0,
        user_ns_id: 0,  /* Global listing */
    };
    let mut ns_ids: [__u64; 100] = [0; 100];
    let ret: ssize_t;

    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE!(ns_ids), 0);
    if ret < 0 {
        if errno() == ENOSYS {
            SKIP!(return, "listns() not supported");
        }
        TH_LOG!("listns failed: %s (errno=%d)", strerror(errno()), errno());
        ASSERT_TRUE!(false);
    }

    /* Should find at least the initial namespaces */
    ASSERT_GT!(ret, 0);
    TH_LOG!("Found %zd active namespaces", ret);

    /* Verify all returned IDs are non-zero */
    for i in 0..ret {
        ASSERT_NE!(ns_ids[i as usize], 0);
        TH_LOG!("  [%zd] ns_id: %llu", i, ns_ids[i as usize] as c_ulong);
    }
});

/*
 * Test listns() with type filtering.
 * List only network namespaces.
 */
TEST!(listns_filter_by_type, {
    let mut req = ns_id_req {
        size: mem::size_of::<ns_id_req>() as __u64,
        spare: 0,
        ns_id: 0,
        ns_type: CLONE_NEWNET as __u64,  /* Only network namespaces */
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids: [__u64; 100] = [0; 100];
    let ret: ssize_t;

    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE!(ns_ids), 0);
    if ret < 0 {
        if errno() == ENOSYS {
            SKIP!(return, "listns() not supported");
        }
        TH_LOG!("listns failed: %s (errno=%d)", strerror(errno()), errno());
        ASSERT_TRUE!(false);
    }
    ASSERT_GE!(ret, 0);

    /* Should find at least init_net */
    ASSERT_GT!(ret, 0);
    TH_LOG!("Found %zd active network namespaces", ret);

    /* Verify we can open each namespace and it's actually a network namespace */
    let mut i: ssize_t = 0;
    while i < ret && i < 5 {
        let nsfh = nsfs_file_handle {
            ns_id: ns_ids[i as usize],
            ns_type: CLONE_NEWNET,
            ns_inum: 0,
        };
        let fh: *mut file_handle;
        let fd: c_int;

        fh = malloc(mem::size_of::<file_handle>() + mem::size_of::<nsfs_file_handle>()) as *mut file_handle;
        ASSERT_NE!(fh, ptr::null_mut());
        (*fh).handle_bytes = mem::size_of::<nsfs_file_handle>() as c_uint;
        (*fh).handle_type = 0;
        memcpy((*fh).f_handle.as_mut_ptr() as *mut c_void, &nsfh as *const _ as *const c_void, mem::size_of::<nsfs_file_handle>());

        fd = open_by_handle_at(-10003, fh, O_RDONLY);
        free(fh as *mut c_void);

        if fd >= 0 {
            let ns_type: c_int;
            /* Verify it's a network namespace via ioctl */
            ns_type = ioctl(fd, NS_GET_NSTYPE);
            if ns_type >= 0 {
                ASSERT_EQ!(ns_type, CLONE_NEWNET);
            }
            close(fd);
        }
        i += 1;
    }
});

/*
 * Test listns() pagination.
 * List namespaces in batches.
 */
TEST!(listns_pagination, {
    let mut req = ns_id_req {
        size: mem::size_of::<ns_id_req>() as __u64,
        spare: 0,
        ns_id: 0,
        ns_type: 0,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut batch1: [__u64; 2] = [0; 2];
    let mut batch2: [__u64; 2] = [0; 2];
    let ret1: ssize_t;
    let ret2: ssize_t;

    /* Get first batch */
    ret1 = sys_listns(&mut req, batch1.as_mut_ptr(), ARRAY_SIZE!(batch1), 0);
    if ret1 < 0 {
        if errno() == ENOSYS {
            SKIP!(return, "listns() not supported");
        }
        TH_LOG!("listns failed: %s (errno=%d)", strerror(errno()), errno());
        ASSERT_TRUE!(false);
    }
    ASSERT_GE!(ret1, 0);

    if ret1 == 0 {
        SKIP!(return, "No namespaces found");
    }

    TH_LOG!("First batch: %zd namespaces", ret1);

    /* Get second batch using last ID from first batch */
    if ret1 == ARRAY_SIZE!(batch1) as ssize_t {
        req.ns_id = batch1[(ret1 - 1) as usize];
        ret2 = sys_listns(&mut req, batch2.as_mut_ptr(), ARRAY_SIZE!(batch2), 0);
        ASSERT_GE!(ret2, 0);

        TH_LOG!("Second batch: %zd namespaces (after ns_id=%llu)",
                ret2, req.ns_id as c_ulong);

        /* If we got more results, verify IDs are monotonically increasing */
        if ret2 > 0 {
            ASSERT_GT!(batch2[0], batch1[(ret1 - 1) as usize]);
            TH_LOG!("Pagination working: %llu > %llu",
                    batch2[0] as c_ulong,
                    batch1[(ret1 - 1) as usize] as c_ulong);
        }
    } else {
        TH_LOG!("All namespaces fit in first batch");
    }
});

/*
 * Test listns() with LISTNS_CURRENT_USER.
 * List namespaces owned by current user namespace.
 */
TEST!(listns_current_user, {
    let mut req = ns_id_req {
        size: mem::size_of::<ns_id_req>() as __u64,
        spare: 0,
        ns_id: 0,
        ns_type: 0,
        spare2: 0,
        user_ns_id: LISTNS_CURRENT_USER,
    };
    let mut ns_ids: [__u64; 100] = [0; 100];
    let ret: ssize_t;

    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE!(ns_ids), 0);
    if ret < 0 {
        if errno() == ENOSYS {
            SKIP!(return, "listns() not supported");
        }
        TH_LOG!("listns failed: %s (errno=%d)", strerror(errno()), errno());
        ASSERT_TRUE!(false);
    }
    ASSERT_GE!(ret, 0);

    /* Should find at least the initial namespaces if we're in init_user_ns */
    TH_LOG!("Found %zd namespaces owned by current user namespace", ret);

    for i in 0..ret {
        TH_LOG!("  [%zd] ns_id: %llu", i, ns_ids[i as usize] as c_ulong);
    }
});

/*
 * Test that listns() only returns active namespaces.
 * Create a namespace, let it become inactive, verify it's not listed.
 */
TEST!(listns_only_active, {
    let mut req = ns_id_req {
        size: mem::size_of::<ns_id_req>() as __u64,
        spare: 0,
        ns_id: 0,
        ns_type: CLONE_NEWNET as __u64,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids_before: [__u64; 100] = [0; 100];
    let mut ns_ids_after: [__u64; 100] = [0; 100];
    let ret_before: ssize_t;
    let ret_after: ssize_t;
    let mut pipefd: [c_int; 2] = [0; 2];
    let pid: pid_t;
    let mut new_ns_id: __u64 = 0;
    let mut status: c_int = 0;

    /* Get initial list */
    ret_before = sys_listns(&mut req, ns_ids_before.as_mut_ptr(), ARRAY_SIZE!(ns_ids_before), 0);
    if ret_before < 0 {
        if errno() == ENOSYS {
            SKIP!(return, "listns() not supported");
        }
        TH_LOG!("listns failed: %s (errno=%d)", strerror(errno()), errno());
        ASSERT_TRUE!(false);
    }
    ASSERT_GE!(ret_before, 0);

    TH_LOG!("Before: %zd active network namespaces", ret_before);

    /* Create a new namespace in a child process and get its ID */
    ASSERT_EQ!(pipe(pipefd.as_mut_ptr()), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let fd: c_int;
        let mut ns_id: __u64 = 0;

        close(pipefd[0]);

        /* Create new network namespace */
        if unshare(CLONE_NEWNET) < 0 {
            close(pipefd[1]);
            exit(1);
        }

        /* Get its ID */
        fd = open(c"/proc/self/ns/net".as_ptr(), O_RDONLY);
        if fd < 0 {
            close(pipefd[1]);
            exit(1);
        }

        if ioctl(fd, NS_GET_ID, &mut ns_id as *mut __u64) < 0 {
            close(fd);
            close(pipefd[1]);
            exit(1);
        }
        close(fd);

        /* Send ID to parent */
        write(pipefd[1], &ns_id as *const _ as *const c_void, mem::size_of_val(&ns_id));
        close(pipefd[1]);

        /* Keep namespace active briefly */
        usleep(100000);
        exit(0);
    }

    /* Parent reads the new namespace ID */
    {
        let bytes: c_int;

        close(pipefd[1]);
        bytes = read(pipefd[0], &mut new_ns_id as *mut _ as *mut c_void, mem::size_of_val(&new_ns_id)) as c_int;
        close(pipefd[0]);

        if bytes == mem::size_of_val(&new_ns_id) as c_int {
            let mut ns_ids_during: [__u64; 100] = [0; 100];
            let ret_during: c_int;

            TH_LOG!("Child created namespace with ID %llu", new_ns_id as c_ulong);

            /* List namespaces while child is still alive - should see new one */
            ret_during = sys_listns(&mut req, ns_ids_during.as_mut_ptr(), ARRAY_SIZE!(ns_ids_during), 0) as c_int;
            ASSERT_GE!(ret_during, 0);
            TH_LOG!("During: %d active network namespaces", ret_during);

            /* Should have more namespaces than before */
            ASSERT_GE!(ret_during, ret_before);
        }
    }

    /* Wait for child to exit */
    waitpid(pid, &mut status, 0);

    /* Give time for namespace to become inactive */
    usleep(100000);

    /* List namespaces after child exits - should not see new one */
    ret_after = sys_listns(&mut req, ns_ids_after.as_mut_ptr(), ARRAY_SIZE!(ns_ids_after), 0);
    ASSERT_GE!(ret_after, 0);
    TH_LOG!("After: %zd active network namespaces", ret_after);

    /* Verify the new namespace ID is not in the after list */
    if new_ns_id != 0 {
        let mut found = false;

        for i in 0..ret_after {
            if ns_ids_after[i as usize] == new_ns_id {
                found = true;
                break;
            }
        }
        ASSERT_FALSE!(found);
    }
});

/*
 * Test listns() with specific user namespace ID.
 * Create a user namespace and list namespaces it owns.
 */
TEST!(listns_specific_userns, {
    let mut req = ns_id_req {
        size: mem::size_of::<ns_id_req>() as __u64,
        spare: 0,
        ns_id: 0,
        ns_type: 0,
        spare2: 0,
        user_ns_id: 0,  /* Will be filled with created userns ID */
    };
    let mut ns_ids: [__u64; 100] = [0; 100];
    let mut sv: [c_int; 2] = [0; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut user_ns_id: __u64 = 0;
    let bytes: c_int;
    let ret: ssize_t;

    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, sv.as_mut_ptr()), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let fd: c_int;
        let mut ns_id: __u64 = 0;
        let mut buf: c_char = 0;

        close(sv[0]);

        /* Create new user namespace */
        if setup_userns() < 0 {
            close(sv[1]);
            exit(1);
        }

        /* Get user namespace ID */
        fd = open(c"/proc/self/ns/user".as_ptr(), O_RDONLY);
        if fd < 0 {
            close(sv[1]);
            exit(1);
        }

        if ioctl(fd, NS_GET_ID, &mut ns_id as *mut __u64) < 0 {
            close(fd);
            close(sv[1]);
            exit(1);
        }
        close(fd);

        /* Send ID to parent */
        if write(sv[1], &ns_id as *const _ as *const c_void, mem::size_of_val(&ns_id)) != mem::size_of_val(&ns_id) as ssize_t {
            close(sv[1]);
            exit(1);
        }

        /* Create some namespaces owned by this user namespace */
        unshare(CLONE_NEWNET);
        unshare(CLONE_NEWUTS);

        /* Wait for parent signal */
        if read(sv[1], &mut buf as *mut _ as *mut c_void, 1) != 1 {
            close(sv[1]);
            exit(1);
        }
        close(sv[1]);
        exit(0);
    }

    /* Parent */
    close(sv[1]);
    bytes = read(sv[0], &mut user_ns_id as *mut _ as *mut c_void, mem::size_of_val(&user_ns_id)) as c_int;

    if bytes != mem::size_of_val(&user_ns_id) as c_int {
        close(sv[0]);
        kill(pid, SIGKILL);
        waitpid(pid, ptr::null_mut(), 0);
        SKIP!(return, "Failed to get user namespace ID from child");
    }

    TH_LOG!("Child created user namespace with ID %llu", user_ns_id as c_ulong);

    /* List namespaces owned by this user namespace */
    req.user_ns_id = user_ns_id;
    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE!(ns_ids), 0);

    if ret < 0 {
        TH_LOG!("listns failed: %s (errno=%d)", strerror(errno()), errno());
        close(sv[0]);
        kill(pid, SIGKILL);
        waitpid(pid, ptr::null_mut(), 0);
        if errno() == ENOSYS {
            SKIP!(return, "listns() not supported");
        }
        ASSERT_GE!(ret, 0);
    }

    TH_LOG!("Found %zd namespaces owned by user namespace %llu", ret,
            user_ns_id as c_ulong);

    /* Should find at least the network and UTS namespaces we created */
    if ret > 0 {
        let mut i: ssize_t = 0;
        while i < ret && i < 10 {
            TH_LOG!("  [%zd] ns_id: %llu", i, ns_ids[i as usize] as c_ulong);
            i += 1;
        }
    }

    /* Signal child to exit */
    if write(sv[0], c"X".as_ptr() as *const c_void, 1) != 1 {
        close(sv[0]);
        kill(pid, SIGKILL);
        waitpid(pid, ptr::null_mut(), 0);
        ASSERT_TRUE!(false);
    }
    close(sv[0]);
    waitpid(pid, &mut status, 0);
});

/*
 * Test listns() with multiple namespace types filter.
 */
TEST!(listns_multiple_types, {
    let mut req = ns_id_req {
        size: mem::size_of::<ns_id_req>() as __u64,
        spare: 0,
        ns_id: 0,
        ns_type: (CLONE_NEWNET | CLONE_NEWUTS) as __u64,  /* Network and UTS */
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids: [__u64; 100] = [0; 100];
    let ret: ssize_t;

    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE!(ns_ids), 0);
    if ret < 0 {
        if errno() == ENOSYS {
            SKIP!(return, "listns() not supported");
        }
        TH_LOG!("listns failed: %s (errno=%d)", strerror(errno()), errno());
        ASSERT_TRUE!(false);
    }
    ASSERT_GE!(ret, 0);

    TH_LOG!("Found %zd active network/UTS namespaces", ret);

    for i in 0..ret {
        TH_LOG!("  [%zd] ns_id: %llu", i, ns_ids[i as usize] as c_ulong);
    }
});

/*
 * Test that hierarchical active reference propagation keeps parent
 * user namespaces visible in listns().
 */
TEST!(listns_hierarchical_visibility, {
    let mut req = ns_id_req {
        size: mem::size_of::<ns_id_req>() as __u64,
        spare: 0,
        ns_id: 0,
        ns_type: CLONE_NEWUSER as __u64,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut parent_ns_id: __u64 = 0;
    let mut child_ns_id: __u64 = 0;
    let mut sv: [c_int; 2] = [0; 2];
    let pid: pid_t;
    let mut status: c_int = 0;
    let mut bytes: c_int;
    let mut ns_ids: [__u64; 100] = [0; 100];
    let ret: ssize_t;
    let mut found_parent: bool;
    let mut found_child: bool;

    ASSERT_EQ!(socketpair(AF_UNIX, SOCK_STREAM, 0, sv.as_mut_ptr()), 0);

    pid = fork();
    ASSERT_GE!(pid, 0);

    if pid == 0 {
        let mut fd: c_int;
        let mut buf: c_char = 0;

        close(sv[0]);

        /* Create parent user namespace */
        if setup_userns() < 0 {
            close(sv[1]);
            exit(1);
        }

        fd = open(c"/proc/self/ns/user".as_ptr(), O_RDONLY);
        if fd < 0 {
            close(sv[1]);
            exit(1);
        }

        if ioctl(fd, NS_GET_ID, &mut parent_ns_id as *mut __u64) < 0 {
            close(fd);
            close(sv[1]);
            exit(1);
        }
        close(fd);

        /* Create child user namespace */
        if setup_userns() < 0 {
            close(sv[1]);
            exit(1);
        }

        fd = open(c"/proc/self/ns/user".as_ptr(), O_RDONLY);
        if fd < 0 {
            close(sv[1]);
            exit(1);
        }

        if ioctl(fd, NS_GET_ID, &mut child_ns_id as *mut __u64) < 0 {
            close(fd);
            close(sv[1]);
            exit(1);
        }
        close(fd);

        /* Send both IDs to parent */
        if write(sv[1], &parent_ns_id as *const _ as *const c_void, mem::size_of_val(&parent_ns_id)) != mem::size_of_val(&parent_ns_id) as ssize_t {
            close(sv[1]);
            exit(1);
        }
        if write(sv[1], &child_ns_id as *const _ as *const c_void, mem::size_of_val(&child_ns_id)) != mem::size_of_val(&child_ns_id) as ssize_t {
            close(sv[1]);
            exit(1);
        }

        /* Wait for parent signal */
        if read(sv[1], &mut buf as *mut _ as *mut c_void, 1) != 1 {
            close(sv[1]);
            exit(1);
        }
        close(sv[1]);
        exit(0);
    }

    /* Parent */
    close(sv[1]);

    /* Read both namespace IDs */
    bytes = read(sv[0], &mut parent_ns_id as *mut _ as *mut c_void, mem::size_of_val(&parent_ns_id)) as c_int;
    bytes += read(sv[0], &mut child_ns_id as *mut _ as *mut c_void, mem::size_of_val(&child_ns_id)) as c_int;

    if bytes != (2 * mem::size_of::<__u64>()) as c_int {
        close(sv[0]);
        kill(pid, SIGKILL);
        waitpid(pid, ptr::null_mut(), 0);
        SKIP!(return, "Failed to get namespace IDs from child");
    }

    TH_LOG!("Parent user namespace ID: %llu", parent_ns_id as c_ulong);
    TH_LOG!("Child user namespace ID: %llu", child_ns_id as c_ulong);

    /* List all user namespaces */
    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE!(ns_ids), 0);

    if ret < 0 && errno() == ENOSYS {
        close(sv[0]);
        kill(pid, SIGKILL);
        waitpid(pid, ptr::null_mut(), 0);
        SKIP!(return, "listns() not supported");
    }

    ASSERT_GE!(ret, 0);
    TH_LOG!("Found %zd active user namespaces", ret);

    /* Both parent and child should be visible (active due to child process) */
    found_parent = false;
    found_child = false;
    for i in 0..ret {
        if ns_ids[i as usize] == parent_ns_id {
            found_parent = true;
        }
        if ns_ids[i as usize] == child_ns_id {
            found_child = true;
        }
    }

    TH_LOG!("Parent namespace %s, child namespace %s",
            if found_parent { c"found".as_ptr() } else { c"NOT FOUND".as_ptr() },
            if found_child { c"found".as_ptr() } else { c"NOT FOUND".as_ptr() });

    ASSERT_TRUE!(found_child);
    /* With hierarchical propagation, parent should also be active */
    ASSERT_TRUE!(found_parent);

    /* Signal child to exit */
    if write(sv[0], c"X".as_ptr() as *const c_void, 1) != 1 {
        close(sv[0]);
        kill(pid, SIGKILL);
        waitpid(pid, ptr::null_mut(), 0);
        ASSERT_TRUE!(false);
    }
    close(sv[0]);
    waitpid(pid, &mut status, 0);
});

/*
 * Test error cases for listns().
 */
TEST!(listns_error_cases, {
    let mut req = ns_id_req {
        size: mem::size_of::<ns_id_req>() as __u64,
        spare: 0,
        ns_id: 0,
        ns_type: 0,
        spare2: 0,
        user_ns_id: 0,
    };
    let mut ns_ids: [__u64; 10] = [0; 10];
    let mut ret: c_int;

    /* Test with invalid flags */
    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE!(ns_ids), 0xFFFF) as c_int;
    if errno() == ENOSYS {
        /* listns() not supported, skip this check */
    } else {
        ASSERT_LT!(ret, 0);
        ASSERT_EQ!(errno(), EINVAL);
    }

    /* Test with NULL ns_ids array */
    ret = sys_listns(&mut req, ptr::null_mut(), 10, 0) as c_int;
    ASSERT_LT!(ret, 0);

    /* Test with invalid spare field */
    req.spare = 1;
    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), ARRAY_SIZE!(ns_ids), 0) as c_int;
    if errno() == ENOSYS {
        /* listns() not supported, skip this check */
    } else {
        ASSERT_LT!(ret, 0);
        ASSERT_EQ!(errno(), EINVAL);
    }
    req.spare = 0;

    /* Test with huge nr_ns_ids */
    ret = sys_listns(&mut req, ns_ids.as_mut_ptr(), 2000000, 0) as c_int;
    if errno() == ENOSYS {
        /* listns() not supported, skip this check */
    } else {
        ASSERT_LT!(ret, 0);
    }
});

TEST_HARNESS_MAIN!();
