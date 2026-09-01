// SPDX-License-Identifier: GPL-2.0
/* Copyright (c) 2025 Meta Platforms, Inc. and affiliates. */

/*
 * Translated from C. External types, constants, functions, and test macros are
 * provided by the surrounding selftest/libbpf environment.
 */

use core::ffi::{c_char, c_int, c_long, c_void};

/*
 * memcg stats are cached per-cpu and only become visible once the periodic
 * flusher runs (FLUSH_TIME, 2s), or once pending updates cross
 * MEMCG_CHARGE_BATCH * num_online_cpus(). That threshold grows with the CPU
 * count, so on a large machine a single pass does not reach it and
 * bpf_mem_cgroup_flush_stats() returns without flushing anything. Retry for
 * long enough to cover a flusher cycle.
 */
const MEMCG_STAT_RETRIES: c_int = 16;
const MEMCG_STAT_RETRY_DELAY_US: c_int = 250 * 1000;

const _SC_PAGESIZE: c_int = 30;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const O_CREAT: c_int = 0o100;
const O_RDWR: c_int = 0o2;
const BPF_CGROUP_ITER_SELF_ONLY: u32 = 0;

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

#[repr(C)]
pub struct bpf_link {
    _private: [u8; 0],
}

#[repr(C)]
pub struct bpf_program {
    _private: [u8; 0],
}

#[repr(C)]
pub struct memcg_query {
    pub nr_anon_mapped: u64,
    pub nr_file_pages: u64,
    pub nr_file_mapped: u64,
    pub nr_shmem: u64,
    pub pgfault: u64,
}

#[repr(C)]
pub struct cgroup_iter_memcg__progs {
    pub cgroup_memcg_query: *mut bpf_program,
}

#[repr(C)]
pub struct cgroup_iter_memcg__data_query {
    pub memcg_query: memcg_query,
}

#[repr(C)]
pub struct cgroup_iter_memcg {
    pub progs: cgroup_iter_memcg__progs,
    pub data_query: *mut cgroup_iter_memcg__data_query,
}

#[repr(C)]
pub struct bpf_iter_attach_opts {
    pub sz: usize,
    pub link_info: *mut c_void,
    pub link_info_len: u32,
}

impl Default for bpf_iter_attach_opts {
    fn default() -> Self {
        Self {
            sz: core::mem::size_of::<Self>(),
            link_info: core::ptr::null_mut(),
            link_info_len: 0,
        }
    }
}

#[repr(C)]
pub struct bpf_iter_cgroup_link_info {
    pub cgroup_fd: c_int,
    pub order: u32,
}

#[repr(C)]
pub union bpf_iter_link_info {
    pub cgroup: core::mem::ManuallyDrop<bpf_iter_cgroup_link_info>,
}

unsafe extern "C" {
    fn bpf_link__fd(link: *mut bpf_link) -> c_int;
    fn bpf_iter_create(link_fd: c_int) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> isize;
    fn close(fd: c_int) -> c_int;
    fn sysconf(name: c_int) -> c_long;
    fn mmap(
        addr: *mut c_void,
        len: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, len: usize) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn usleep(usec: u32) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, mode: c_int) -> c_int;
    fn ftruncate(fd: c_int, length: usize) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn memfd_create(name: *const c_char, flags: c_int) -> c_int;
    fn fallocate(fd: c_int, mode: c_int, offset: isize, len: usize) -> c_int;

    fn cgroup_setup_and_join(path: *const c_char) -> c_int;
    fn cleanup_cgroup_environment();
    fn cgroup_iter_memcg__open_and_load() -> *mut cgroup_iter_memcg;
    fn cgroup_iter_memcg__destroy(skel: *mut cgroup_iter_memcg);
    fn bpf_program__attach_iter(
        prog: *mut bpf_program,
        opts: *mut bpf_iter_attach_opts,
    ) -> *mut bpf_link;
    fn bpf_link__destroy(link: *mut bpf_link);
    fn test__start_subtest(name: *const c_char) -> bool;

    fn ASSERT_OK_FD(fd: c_int, name: *const c_char) -> bool;
    fn ASSERT_EQ(left: isize, right: isize, name: *const c_char) -> bool;
    fn ASSERT_NEQ(left: *mut c_void, right: *mut c_void, name: *const c_char) -> bool;
    fn ASSERT_OK(ret: c_int, name: *const c_char) -> bool;
    fn ASSERT_GT(left: u64, right: u64, name: *const c_char) -> bool;
    fn ASSERT_OK_PTR(ptr: *mut c_void, name: *const c_char) -> bool;
}

unsafe fn read_stats(link: *mut bpf_link) -> c_int {
    let fd: c_int;
    let mut ret: c_int = 0;
    let bytes: isize;

    fd = bpf_iter_create(bpf_link__fd(link));
    if !ASSERT_OK_FD(fd, c"bpf_iter_create".as_ptr()) {
        return 1;
    }

    /*
     * Invoke iter program by reading from its fd. We're not expecting any
     * data to be written by the bpf program so the result should be zero.
     * Results will be read directly through the custom data section
     * accessible through skel->data_query.memcg_query.
     */
    bytes = read(fd, core::ptr::null_mut(), 0);
    if !ASSERT_EQ(bytes, 0, c"read fd".as_ptr()) {
        ret = 1;
    }

    close(fd);
    ret
}

unsafe fn test_anon(link: *mut bpf_link, memcg_query: *mut memcg_query) {
    let mut retries: c_int = 0;
    let mut map: *mut c_void;
    let len: usize;

    len = (sysconf(_SC_PAGESIZE) as usize) * 1024;

    loop {
        /*
         * Increase memcg anon usage by mapping and writing
         * to a new anon region.
         */
        map = mmap(
            core::ptr::null_mut(),
            len,
            PROT_WRITE,
            MAP_ANONYMOUS | MAP_PRIVATE,
            -1,
            0,
        );
        if !ASSERT_NEQ(map, MAP_FAILED, c"mmap anon".as_ptr()) {
            return;
        }

        memset(map, 1, len);

        if !ASSERT_OK(read_stats(link), c"read stats".as_ptr()) {
            break;
        }

        if (*memcg_query).nr_anon_mapped == 0 && {
            retries += 1;
            retries < MEMCG_STAT_RETRIES
        } {
            usleep(MEMCG_STAT_RETRY_DELAY_US as u32);
            munmap(map, len);
            continue;
        }

        ASSERT_GT(
            (*memcg_query).nr_anon_mapped,
            0,
            c"final anon mapped val".as_ptr(),
        );
        break;
    }

    munmap(map, len);
}

unsafe fn test_file(link: *mut bpf_link, memcg_query: *mut memcg_query) {
    let mut retries: c_int = 0;
    let mut map: *mut c_void;
    let len: usize;
    let path: *const c_char;
    let fd: c_int;

    len = (sysconf(_SC_PAGESIZE) as usize) * 1024;
    path = c"/tmp/test_cgroup_iter_memcg".as_ptr();

    /*
     * Increase memcg file usage by creating and writing
     * to a mapped file.
     */
    fd = open(path, O_CREAT | O_RDWR, 0o644);
    if !ASSERT_OK_FD(fd, c"open fd".as_ptr()) {
        return;
    }

    loop {
        if !ASSERT_OK(ftruncate(fd, len), c"ftruncate".as_ptr()) {
            break;
        }

        map = mmap(
            core::ptr::null_mut(),
            len,
            PROT_WRITE,
            MAP_SHARED,
            fd,
            0,
        );
        if !ASSERT_NEQ(map, MAP_FAILED, c"mmap file".as_ptr()) {
            break;
        }

        memset(map, 1, len);

        if !ASSERT_OK(read_stats(link), c"read stats".as_ptr()) {
            munmap(map, len);
            break;
        }

        if ((*memcg_query).nr_file_pages == 0 || (*memcg_query).nr_file_mapped == 0) && {
            retries += 1;
            retries < MEMCG_STAT_RETRIES
        } {
            usleep(MEMCG_STAT_RETRY_DELAY_US as u32);
            munmap(map, len);
            continue;
        }

        ASSERT_GT((*memcg_query).nr_file_pages, 0, c"final file value".as_ptr());
        ASSERT_GT(
            (*memcg_query).nr_file_mapped,
            0,
            c"final file mapped value".as_ptr(),
        );

        munmap(map, len);
        break;
    }

    close(fd);
    unlink(path);
}

unsafe fn test_shmem(link: *mut bpf_link, memcg_query: *mut memcg_query) {
    let mut retries: c_int = 0;
    let len: usize;
    let fd: c_int;

    len = (sysconf(_SC_PAGESIZE) as usize) * 1024;

    /*
     * Increase memcg shmem usage by creating and writing
     * to a memfd backed by shmem/tmpfs.
     */
    fd = memfd_create(c"tmp_shmem".as_ptr(), 0);
    if !ASSERT_OK_FD(fd, c"memfd_create".as_ptr()) {
        return;
    }

    loop {
        if !ASSERT_OK(fallocate(fd, 0, 0, len), c"fallocate".as_ptr()) {
            break;
        }

        if !ASSERT_OK(read_stats(link), c"read stats".as_ptr()) {
            break;
        }

        if (*memcg_query).nr_shmem == 0 && {
            retries += 1;
            retries < MEMCG_STAT_RETRIES
        } {
            usleep(MEMCG_STAT_RETRY_DELAY_US as u32);
            continue;
        }

        ASSERT_GT((*memcg_query).nr_shmem, 0, c"final shmem value".as_ptr());
        break;
    }

    close(fd);
}

unsafe fn test_pgfault(link: *mut bpf_link, memcg_query: *mut memcg_query) {
    let mut retries: c_int = 0;
    let mut map: *mut c_void;
    let len: usize;

    len = (sysconf(_SC_PAGESIZE) as usize) * 1024;

    loop {
        /* Create region to use for triggering a page fault. */
        map = mmap(
            core::ptr::null_mut(),
            len,
            PROT_WRITE,
            MAP_ANONYMOUS | MAP_PRIVATE,
            -1,
            0,
        );
        if !ASSERT_NEQ(map, MAP_FAILED, c"mmap anon".as_ptr()) {
            return;
        }

        /* Trigger page fault. */
        memset(map, 1, len);

        if !ASSERT_OK(read_stats(link), c"read stats".as_ptr()) {
            break;
        }

        if (*memcg_query).pgfault == 0 && {
            retries += 1;
            retries < MEMCG_STAT_RETRIES
        } {
            usleep(MEMCG_STAT_RETRY_DELAY_US as u32);
            munmap(map, len);
            continue;
        }

        ASSERT_GT((*memcg_query).pgfault, 0, c"final pgfault val".as_ptr());
        break;
    }

    munmap(map, len);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn test_cgroup_iter_memcg() {
    let cgroup_rel_path: *const c_char = c"/cgroup_iter_memcg_test".as_ptr();
    let skel: *mut cgroup_iter_memcg;
    let link: *mut bpf_link;
    let cgroup_fd: c_int;

    cgroup_fd = cgroup_setup_and_join(cgroup_rel_path);
    if !ASSERT_OK_FD(cgroup_fd, c"cgroup_setup_and_join".as_ptr()) {
        return;
    }

    skel = cgroup_iter_memcg__open_and_load();
    if !ASSERT_OK_PTR(
        skel as *mut c_void,
        c"cgroup_iter_memcg__open_and_load".as_ptr(),
    ) {
        close(cgroup_fd);
        cleanup_cgroup_environment();
        return;
    }

    let mut opts = bpf_iter_attach_opts::default();
    let mut linfo = bpf_iter_link_info {
        cgroup: core::mem::ManuallyDrop::new(bpf_iter_cgroup_link_info {
            cgroup_fd,
            order: BPF_CGROUP_ITER_SELF_ONLY,
        }),
    };
    opts.link_info = &mut linfo as *mut bpf_iter_link_info as *mut c_void;
    opts.link_info_len = core::mem::size_of::<bpf_iter_link_info>() as u32;

    link = bpf_program__attach_iter((*skel).progs.cgroup_memcg_query, &mut opts);
    if !ASSERT_OK_PTR(link as *mut c_void, c"bpf_program__attach_iter".as_ptr()) {
        cgroup_iter_memcg__destroy(skel);
        close(cgroup_fd);
        cleanup_cgroup_environment();
        return;
    }

    if test__start_subtest(c"cgroup_iter_memcg__anon".as_ptr()) {
        test_anon(link, &mut (*(*skel).data_query).memcg_query);
    }
    if test__start_subtest(c"cgroup_iter_memcg__shmem".as_ptr()) {
        test_shmem(link, &mut (*(*skel).data_query).memcg_query);
    }
    if test__start_subtest(c"cgroup_iter_memcg__file".as_ptr()) {
        test_file(link, &mut (*(*skel).data_query).memcg_query);
    }
    if test__start_subtest(c"cgroup_iter_memcg__pgfault".as_ptr()) {
        test_pgfault(link, &mut (*(*skel).data_query).memcg_query);
    }

    bpf_link__destroy(link);
    cgroup_iter_memcg__destroy(skel);
    close(cgroup_fd);
    cleanup_cgroup_environment();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
