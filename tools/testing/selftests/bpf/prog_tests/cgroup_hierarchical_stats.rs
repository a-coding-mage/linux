// SPDX-License-Identifier: GPL-2.0-only
/*
 * This test makes sure BPF stats collection using rstat works correctly.
 * The test uses 3 BPF progs:
 * (a) counter: This BPF prog is invoked every time we attach a process to a
 *              cgroup and locklessly increments a percpu counter.
 *              The program then calls cgroup_rstat_updated() to inform rstat
 *              of an update on the (cpu, cgroup) pair.
 *
 * (b) flusher: This BPF prog is invoked when an rstat flush is ongoing, it
 *              aggregates all percpu counters to a total counter, and also
 *              propagates the changes to the ancestor cgroups.
 *
 * (c) dumper: This BPF prog is a cgroup_iter. It is used to output the total
 *             counter of a cgroup through reading a file in userspace.
 *
 * The test sets up a cgroup hierarchy, and the above programs. It spawns a few
 * processes in the leaf cgroups and makes sure all the counters are aggregated
 * correctly.
 *
 * Copyright 2022 Google LLC.
 */

use core::ffi::c_void;
use std::ffi::CStr;
use std::os::raw::{c_char, c_int, c_uint, c_ulonglong};
use std::ptr;

const PAGE_SIZE: c_int = 4096;
const PROCESSES_PER_CGROUP: c_int = 3;

const BPFFS_ROOT: &[u8] = b"/sys/fs/bpf/\0";
const BPFFS_ATTACH_COUNTERS: &[u8] = b"/sys/fs/bpf/attach_counters/\0";

const CG_ROOT_NAME: &[u8] = b"root\0";
const CG_ROOT_ID: c_ulonglong = 1;

const N_CGROUPS: usize = 7;
const N_NON_LEAF_CGROUPS: c_int = 3;

const O_RDONLY: c_int = 0;
const EBUSY: c_int = 16;
const EACCES: c_int = 13;
const EFAULT: c_int = 14;
const BPF_CGROUP_ITER_SELF_ONLY: c_uint = 1;

macro_rules! MB {
    ($x:expr) => {
        ($x << 20)
    };
}

#[repr(C)]
struct Cgroup {
    path: *const c_char,
    name: *const c_char,
    id: c_ulonglong,
    fd: c_int,
}

#[repr(C)]
struct cgroup_hierarchical_stats {
    progs: cgroup_hierarchical_stats_progs,
}

#[repr(C)]
struct cgroup_hierarchical_stats_progs {
    dumper: *mut bpf_program,
}

#[repr(C)]
struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
struct bpf_iter_attach_opts {
    sz: usize,
    link_info: *mut c_void,
    link_info_len: c_uint,
}

#[repr(C)]
struct bpf_iter_link_info_cgroup {
    cgroup_fd: c_int,
    order: c_uint,
}

#[repr(C)]
union bpf_iter_link_info {
    cgroup: bpf_iter_link_info_cgroup,
}

static mut CGROUPS: [Cgroup; N_CGROUPS] = [
    Cgroup {
        path: b"/test\0".as_ptr() as *const c_char,
        name: b"test\0".as_ptr() as *const c_char,
        id: 0,
        fd: 0,
    },
    Cgroup {
        path: b"/test/child1\0".as_ptr() as *const c_char,
        name: b"child1\0".as_ptr() as *const c_char,
        id: 0,
        fd: 0,
    },
    Cgroup {
        path: b"/test/child2\0".as_ptr() as *const c_char,
        name: b"child2\0".as_ptr() as *const c_char,
        id: 0,
        fd: 0,
    },
    Cgroup {
        path: b"/test/child1/child1_1\0".as_ptr() as *const c_char,
        name: b"child1_1\0".as_ptr() as *const c_char,
        id: 0,
        fd: 0,
    },
    Cgroup {
        path: b"/test/child1/child1_2\0".as_ptr() as *const c_char,
        name: b"child1_2\0".as_ptr() as *const c_char,
        id: 0,
        fd: 0,
    },
    Cgroup {
        path: b"/test/child2/child2_1\0".as_ptr() as *const c_char,
        name: b"child2_1\0".as_ptr() as *const c_char,
        id: 0,
        fd: 0,
    },
    Cgroup {
        path: b"/test/child2/child2_2\0".as_ptr() as *const c_char,
        name: b"child2_2\0".as_ptr() as *const c_char,
        id: 0,
        fd: 0,
    },
];

static mut ROOT_CGROUP_FD: c_int = 0;
static mut MOUNTED_BPFFS: bool = false;

unsafe extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: usize,
        data: *const c_void,
    ) -> c_int;
    fn mkdir(pathname: *const c_char, mode: c_uint) -> c_int;
    fn rmdir(pathname: *const c_char) -> c_int;
    fn umount(target: *const c_char) -> c_int;
    fn fork() -> c_int;
    fn exit(status: c_int) -> !;
    fn waitpid(pid: c_int, wstatus: *mut c_int, options: c_int) -> c_int;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn sscanf(s: *const c_char, format: *const c_char, ...) -> c_int;
    fn remove(pathname: *const c_char) -> c_int;

    fn setup_cgroup_environment() -> c_int;
    fn get_root_cgroup() -> c_int;
    fn create_and_get_cgroup(path: *const c_char) -> c_int;
    fn get_cgroup_id(path: *const c_char) -> c_ulonglong;
    fn cleanup_cgroup_environment();
    fn join_parent_cgroup(path: *const c_char) -> c_int;

    fn bpf_program__attach_iter(
        prog: *mut bpf_program,
        opts: *mut bpf_iter_attach_opts,
    ) -> *mut bpf_link;
    fn bpf_link__pin(link: *mut bpf_link, path: *const c_char) -> c_int;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn bpf_program__set_autoattach(prog: *mut bpf_program, autoattach: bool);

    fn cgroup_hierarchical_stats__open_and_load() -> *mut cgroup_hierarchical_stats;
    fn cgroup_hierarchical_stats__attach(obj: *mut cgroup_hierarchical_stats) -> c_int;
    fn cgroup_hierarchical_stats__destroy(obj: *mut cgroup_hierarchical_stats);

    fn ASSERT_FALSE(condition: bool, name: *const c_char) -> bool;
    fn ASSERT_OK(res: c_int, name: *const c_char) -> bool;
    fn ASSERT_GE(left: c_longlong, right: c_longlong, name: *const c_char) -> bool;
    fn ASSERT_TRUE(condition: bool, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: c_ulonglong, right: c_ulonglong, name: *const c_char) -> bool;
    fn ASSERT_GT(left: c_ulonglong, right: c_ulonglong, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *const c_void, name: *const c_char) -> bool;
}

type c_longlong = i64;

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

/* reads file at 'path' to 'buf', returns 0 on success. */
unsafe fn read_from_file(path: *const c_char, buf: *mut c_char, size: usize) -> c_int {
    let fd: c_int;
    let len: isize;

    fd = open(path, O_RDONLY);
    if fd < 0 {
        return fd;
    }

    len = read(fd, buf as *mut c_void, size);
    close(fd);
    if len < 0 {
        return len as c_int;
    }

    *buf.add(len as usize) = 0;
    0
}

/* mounts bpffs and mkdir for reading stats, returns 0 on success. */
unsafe fn setup_bpffs() -> c_int {
    let mut err: c_int;

    /* Mount bpffs */
    err = mount(
        b"bpf\0".as_ptr() as *const c_char,
        BPFFS_ROOT.as_ptr() as *const c_char,
        b"bpf\0".as_ptr() as *const c_char,
        0,
        ptr::null(),
    );
    MOUNTED_BPFFS = err == 0;
    if ASSERT_FALSE(err != 0 && errno != EBUSY, b"mount\0".as_ptr() as *const c_char) {
        return err;
    }

    /* Create a directory to contain stat files in bpffs */
    err = mkdir(BPFFS_ATTACH_COUNTERS.as_ptr() as *const c_char, 0o755);
    if !ASSERT_OK(err, b"mkdir\0".as_ptr() as *const c_char) {
        return err;
    }

    0
}

unsafe fn cleanup_bpffs() {
    /* Remove created directory in bpffs */
    ASSERT_OK(
        rmdir(BPFFS_ATTACH_COUNTERS.as_ptr() as *const c_char),
        b"rmdir /sys/fs/bpf/attach_counters/\0".as_ptr() as *const c_char,
    );

    /* Unmount bpffs, if it wasn't already mounted when we started */
    if MOUNTED_BPFFS {
        return;
    }

    ASSERT_OK(
        umount(BPFFS_ROOT.as_ptr() as *const c_char),
        b"unmount bpffs\0".as_ptr() as *const c_char,
    );
}

/* sets up cgroups, returns 0 on success. */
unsafe fn setup_cgroups() -> c_int {
    let mut i: c_int;
    let mut fd: c_int;
    let mut err: c_int;

    err = setup_cgroup_environment();
    if !ASSERT_OK(
        err,
        b"setup_cgroup_environment\0".as_ptr() as *const c_char,
    ) {
        return err;
    }

    ROOT_CGROUP_FD = get_root_cgroup();
    if !ASSERT_GE(
        ROOT_CGROUP_FD as c_longlong,
        0,
        b"get_root_cgroup\0".as_ptr() as *const c_char,
    ) {
        return ROOT_CGROUP_FD;
    }

    i = 0;
    while i < N_CGROUPS as c_int {
        fd = create_and_get_cgroup(CGROUPS[i as usize].path);
        if !ASSERT_GE(
            fd as c_longlong,
            0,
            b"create_and_get_cgroup\0".as_ptr() as *const c_char,
        ) {
            return fd;
        }

        CGROUPS[i as usize].fd = fd;
        CGROUPS[i as usize].id = get_cgroup_id(CGROUPS[i as usize].path);
        i += 1;
    }
    0
}

unsafe fn cleanup_cgroups() {
    close(ROOT_CGROUP_FD);
    let mut i: c_int = 0;
    while i < N_CGROUPS as c_int {
        close(CGROUPS[i as usize].fd);
        i += 1;
    }
    cleanup_cgroup_environment();
}

/* Sets up cgroup hiearchary, returns 0 on success. */
unsafe fn setup_hierarchy() -> c_int {
    if setup_bpffs() != 0 {
        1
    } else if setup_cgroups() != 0 {
        1
    } else {
        0
    }
}

unsafe fn destroy_hierarchy() {
    cleanup_cgroups();
    cleanup_bpffs();
}

unsafe fn attach_processes() -> c_int {
    let mut i: c_int;
    let mut j: c_int;
    let mut status: c_int = 0;

    /* In every leaf cgroup, attach 3 processes */
    i = N_NON_LEAF_CGROUPS;
    while i < N_CGROUPS as c_int {
        j = 0;
        while j < PROCESSES_PER_CGROUP {
            let pid: c_int;

            /* Create child and attach to cgroup */
            pid = fork();
            if pid == 0 {
                if join_parent_cgroup(CGROUPS[i as usize].path) != 0 {
                    exit(EACCES);
                }
                exit(0);
            }

            /* Cleanup child */
            waitpid(pid, &mut status, 0);
            if !ASSERT_TRUE(
                WIFEXITED(status),
                b"child process exited\0".as_ptr() as *const c_char,
            ) {
                return 1;
            }
            if !ASSERT_EQ(
                WEXITSTATUS(status) as c_ulonglong,
                0,
                b"child process exit code\0".as_ptr() as *const c_char,
            ) {
                return 1;
            }
            j += 1;
        }
        i += 1;
    }
    0
}

unsafe fn get_attach_counter(cgroup_id: c_ulonglong, file_name: *const c_char) -> c_ulonglong {
    let mut attach_counter: c_ulonglong = 0;
    let mut id: c_ulonglong = 0;
    static mut BUF: [c_char; 128] = [0; 128];
    static mut PATH: [c_char; 128] = [0; 128];

    /* For every cgroup, read the file generated by cgroup_iter */
    snprintf(
        PATH.as_mut_ptr(),
        128,
        b"%s%s\0".as_ptr() as *const c_char,
        BPFFS_ATTACH_COUNTERS.as_ptr() as *const c_char,
        file_name,
    );
    if !ASSERT_OK(
        read_from_file(PATH.as_mut_ptr(), BUF.as_mut_ptr(), 128),
        b"read cgroup_iter\0".as_ptr() as *const c_char,
    ) {
        return 0;
    }

    /* Check the output file formatting */
    ASSERT_EQ(
        sscanf(
            BUF.as_ptr(),
            b"cg_id: %llu, attach_counter: %llu\n\0".as_ptr() as *const c_char,
            &mut id as *mut c_ulonglong,
            &mut attach_counter as *mut c_ulonglong,
        ) as c_ulonglong,
        2,
        b"output format\0".as_ptr() as *const c_char,
    );

    /* Check that the cgroup_id is displayed correctly */
    ASSERT_EQ(id, cgroup_id, b"cgroup_id\0".as_ptr() as *const c_char);
    /* Check that the counter is non-zero */
    ASSERT_GT(
        attach_counter,
        0,
        b"attach counter non-zero\0".as_ptr() as *const c_char,
    );
    attach_counter
}

unsafe fn check_attach_counters() {
    let mut attach_counters: [c_ulonglong; N_CGROUPS] = [0; N_CGROUPS];
    let root_attach_counter: c_ulonglong;
    let mut i: c_int;

    i = 0;
    while i < N_CGROUPS as c_int {
        attach_counters[i as usize] =
            get_attach_counter(CGROUPS[i as usize].id, CGROUPS[i as usize].name);
        i += 1;
    }

    /* Read stats for root too */
    root_attach_counter = get_attach_counter(CG_ROOT_ID, CG_ROOT_NAME.as_ptr() as *const c_char);

    /* Check that all leafs cgroups have an attach counter of 3 */
    i = N_NON_LEAF_CGROUPS;
    while i < N_CGROUPS as c_int {
        ASSERT_EQ(
            attach_counters[i as usize],
            PROCESSES_PER_CGROUP as c_ulonglong,
            b"leaf cgroup attach counter\0".as_ptr() as *const c_char,
        );
        i += 1;
    }

    /* Check that child1 == child1_1 + child1_2 */
    ASSERT_EQ(
        attach_counters[1],
        attach_counters[3] + attach_counters[4],
        b"child1_counter\0".as_ptr() as *const c_char,
    );
    /* Check that child2 == child2_1 + child2_2 */
    ASSERT_EQ(
        attach_counters[2],
        attach_counters[5] + attach_counters[6],
        b"child2_counter\0".as_ptr() as *const c_char,
    );
    /* Check that test == child1 + child2 */
    ASSERT_EQ(
        attach_counters[0],
        attach_counters[1] + attach_counters[2],
        b"test_counter\0".as_ptr() as *const c_char,
    );
    /* Check that root >= test */
    ASSERT_GE(
        root_attach_counter as c_longlong,
        attach_counters[1] as c_longlong,
        b"root_counter\0".as_ptr() as *const c_char,
    );
}

/* Creates iter link and pins in bpffs, returns 0 on success, -errno on failure.
 */
unsafe fn setup_cgroup_iter(
    obj: *mut cgroup_hierarchical_stats,
    cgroup_fd: c_int,
    file_name: *const c_char,
) -> c_int {
    let mut opts = bpf_iter_attach_opts {
        sz: core::mem::size_of::<bpf_iter_attach_opts>(),
        link_info: ptr::null_mut(),
        link_info_len: 0,
    };
    let mut linfo = bpf_iter_link_info {
        cgroup: bpf_iter_link_info_cgroup {
            cgroup_fd: 0,
            order: 0,
        },
    };
    let link: *mut bpf_link;
    static mut PATH: [c_char; 128] = [0; 128];
    let err: c_int;

    /*
     * Create an iter link, parameterized by cgroup_fd. We only want to
     * traverse one cgroup, so set the traversal order to "self".
     */
    linfo.cgroup.cgroup_fd = cgroup_fd;
    linfo.cgroup.order = BPF_CGROUP_ITER_SELF_ONLY;
    opts.link_info = &mut linfo as *mut bpf_iter_link_info as *mut c_void;
    opts.link_info_len = core::mem::size_of::<bpf_iter_link_info>() as c_uint;
    link = bpf_program__attach_iter((*obj).progs.dumper, &mut opts);
    if !ASSERT_OK_PTR(link as *const c_void, b"attach_iter\0".as_ptr() as *const c_char) {
        return -EFAULT;
    }

    /* Pin the link to a bpffs file */
    snprintf(
        PATH.as_mut_ptr(),
        128,
        b"%s%s\0".as_ptr() as *const c_char,
        BPFFS_ATTACH_COUNTERS.as_ptr() as *const c_char,
        file_name,
    );
    err = bpf_link__pin(link, PATH.as_ptr());
    ASSERT_OK(err, b"pin cgroup_iter\0".as_ptr() as *const c_char);

    /* Remove the link, leaving only the ref held by the pinned file */
    bpf_link__destroy(link);
    err
}

/* Sets up programs for collecting stats, returns 0 on success. */
unsafe fn setup_progs(skel: *mut *mut cgroup_hierarchical_stats) -> c_int {
    let mut i: c_int;
    let mut err: c_int;

    *skel = cgroup_hierarchical_stats__open_and_load();
    if !ASSERT_OK_PTR(*skel as *const c_void, b"open_and_load\0".as_ptr() as *const c_char) {
        return 1;
    }

    /* Attach cgroup_iter program that will dump the stats to cgroups */
    i = 0;
    while i < N_CGROUPS as c_int {
        err = setup_cgroup_iter(*skel, CGROUPS[i as usize].fd, CGROUPS[i as usize].name);
        if !ASSERT_OK(err, b"setup_cgroup_iter\0".as_ptr() as *const c_char) {
            return err;
        }
        i += 1;
    }

    /* Also dump stats for root */
    err = setup_cgroup_iter(
        *skel,
        ROOT_CGROUP_FD,
        CG_ROOT_NAME.as_ptr() as *const c_char,
    );
    if !ASSERT_OK(err, b"setup_cgroup_iter\0".as_ptr() as *const c_char) {
        return err;
    }

    bpf_program__set_autoattach((*(*skel)).progs.dumper, false);
    err = cgroup_hierarchical_stats__attach(*skel);
    if !ASSERT_OK(err, b"attach\0".as_ptr() as *const c_char) {
        return err;
    }

    0
}

unsafe fn destroy_progs(skel: *mut cgroup_hierarchical_stats) {
    static mut PATH: [c_char; 128] = [0; 128];
    let mut i: c_int;

    i = 0;
    while i < N_CGROUPS as c_int {
        /* Delete files in bpffs that cgroup_iters are pinned in */
        snprintf(
            PATH.as_mut_ptr(),
            128,
            b"%s%s\0".as_ptr() as *const c_char,
            BPFFS_ATTACH_COUNTERS.as_ptr() as *const c_char,
            CGROUPS[i as usize].name,
        );
        ASSERT_OK(
            remove(PATH.as_ptr()),
            b"remove cgroup_iter pin\0".as_ptr() as *const c_char,
        );
        i += 1;
    }

    /* Delete root file in bpffs */
    snprintf(
        PATH.as_mut_ptr(),
        128,
        b"%s%s\0".as_ptr() as *const c_char,
        BPFFS_ATTACH_COUNTERS.as_ptr() as *const c_char,
        CG_ROOT_NAME.as_ptr() as *const c_char,
    );
    ASSERT_OK(
        remove(PATH.as_ptr()),
        b"remove cgroup_iter root pin\0".as_ptr() as *const c_char,
    );
    cgroup_hierarchical_stats__destroy(skel);
}

#[no_mangle]
pub unsafe extern "C" fn test_cgroup_hierarchical_stats() {
    let mut skel: *mut cgroup_hierarchical_stats = ptr::null_mut();

    if setup_hierarchy() != 0 {
        destroy_hierarchy();
        return;
    }
    if setup_progs(&mut skel) != 0 {
        destroy_progs(skel);
        destroy_hierarchy();
        return;
    }
    if attach_processes() != 0 {
        destroy_progs(skel);
        destroy_hierarchy();
        return;
    }
    check_attach_counters();
    destroy_progs(skel);
    destroy_hierarchy();
}
