// SPDX-License-Identifier: GPL-2.0
/*
 * memfd GUP test-case
 * This tests memfd interactions with get_user_pages(). We require the
 * fuse_mnt.c program to provide a fake direct-IO FUSE mount-point for us. This
 * file-system delays _all_ reads by 1s and forces direct-IO. This means, any
 * read() on files in that file-system will pin the receive-buffer pages for at
 * least 1s via get_user_pages().
 *
 * We use this trick to race ADD_SEALS against a write on a memfd object. The
 * ADD_SEALS must fail if the memfd pages are still pinned. Note that we use
 * the read() syscall with our memory-mapped memfd object as receive buffer to
 * force the kernel to write into our memfd object.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type loff_t = i64;
type __u64 = u64;

const MFD_DEF_SIZE: size_t = 8192;
const STACK_SIZE: size_t = 65536;

const F_GET_SEALS: c_int = 1034;
const F_ADD_SEALS: c_int = 1033;
const F_SEAL_WRITE: __u64 = 0x0008;

const MFD_CLOEXEC: c_uint = 0x0001;
const MFD_ALLOW_SEALING: c_uint = 0x0002;

const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_SHARED: c_int = 0x01;
const MAP_PRIVATE: c_int = 0x02;

const O_RDONLY: c_int = 0;
const O_CLOEXEC: c_int = 0o2000000;

const SIGCHLD: c_int = 17;
const CLONE_VM: c_int = 0x00000100;
const CLONE_FS: c_int = 0x00000200;
const CLONE_FILES: c_int = 0x00000400;

const EBUSY: c_int = 16;

const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

static mut mfd_def_size: size_t = MFD_DEF_SIZE;

unsafe extern "C" {
    static mut errno: c_int;
    static mut hugetlbfs_test: c_int;

    fn printf(format: *const c_char, ...) -> c_int;
    fn abort() -> !;
    fn ftruncate(fd: c_int, length: loff_t) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn mmap(
        addr: *mut c_void,
        length: size_t,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: loff_t,
    ) -> *mut c_void;
    fn munmap(addr: *mut c_void, length: size_t) -> c_int;
    fn usleep(usec: c_uint) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn malloc(size: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn clone(
        func: unsafe extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
    ) -> c_int;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn close(fd: c_int) -> c_int;

    /* common.h */
    fn sys_memfd_create(name: *const c_char, flags: c_uint) -> c_int;
    fn default_huge_page_size() -> c_ulong;
}

unsafe fn mfd_assert_new(name: *const c_char, sz: loff_t, flags: c_uint) -> c_int {
    let mut r: c_int;
    let fd: c_int;

    fd = sys_memfd_create(name, flags);
    if fd < 0 {
        printf(
            b"memfd_create(\"%s\", %u) failed: %m\n\0".as_ptr() as *const c_char,
            name,
            flags,
        );
        abort();
    }

    r = ftruncate(fd, sz);
    if r < 0 {
        printf(
            b"ftruncate(%llu) failed: %m\n\0".as_ptr() as *const c_char,
            sz as c_ulong,
        );
        abort();
    }

    fd
}

unsafe fn mfd_assert_get_seals(fd: c_int) -> __u64 {
    let r: c_long;

    r = fcntl(fd, F_GET_SEALS) as c_long;
    if r < 0 {
        printf(b"GET_SEALS(%d) failed: %m\n\0".as_ptr() as *const c_char, fd);
        abort();
    }

    r as __u64
}

unsafe fn mfd_assert_has_seals(fd: c_int, seals: __u64) {
    let s: __u64;

    s = mfd_assert_get_seals(fd);
    if s != seals {
        printf(
            b"%llu != %llu = GET_SEALS(%d)\n\0".as_ptr() as *const c_char,
            seals as c_ulong,
            s as c_ulong,
            fd,
        );
        abort();
    }
}

unsafe fn mfd_assert_add_seals(fd: c_int, seals: __u64) {
    let mut r: c_long;
    let s: __u64;

    s = mfd_assert_get_seals(fd);
    r = fcntl(fd, F_ADD_SEALS, seals) as c_long;
    if r < 0 {
        printf(
            b"ADD_SEALS(%d, %llu -> %llu) failed: %m\n\0".as_ptr() as *const c_char,
            fd,
            s as c_ulong,
            seals as c_ulong,
        );
        abort();
    }
}

unsafe fn mfd_busy_add_seals(fd: c_int, seals: __u64) -> c_int {
    let mut r: c_long;
    let s: __u64;

    r = fcntl(fd, F_GET_SEALS) as c_long;
    if r < 0 {
        s = 0;
    } else {
        s = r as __u64;
    }

    r = fcntl(fd, F_ADD_SEALS, seals) as c_long;
    if r < 0 && errno != EBUSY {
        printf(
            b"ADD_SEALS(%d, %llu -> %llu) didn't fail as expected with EBUSY: %m\n\0".as_ptr()
                as *const c_char,
            fd,
            s as c_ulong,
            seals as c_ulong,
        );
        abort();
    }

    r as c_int
}

unsafe fn mfd_assert_mmap_shared(fd: c_int) -> *mut c_void {
    let p: *mut c_void;

    p = mmap(
        core::ptr::null_mut(),
        mfd_def_size,
        PROT_READ | PROT_WRITE,
        MAP_SHARED,
        fd,
        0,
    );
    if p == MAP_FAILED {
        printf(b"mmap() failed: %m\n\0".as_ptr() as *const c_char);
        abort();
    }

    p
}

unsafe fn mfd_assert_mmap_private(fd: c_int) -> *mut c_void {
    let p: *mut c_void;

    p = mmap(
        core::ptr::null_mut(),
        mfd_def_size,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE,
        fd,
        0,
    );
    if p == MAP_FAILED {
        printf(b"mmap() failed: %m\n\0".as_ptr() as *const c_char);
        abort();
    }

    p
}

static mut global_mfd: c_int = -1;
static mut global_p: *mut c_void = core::ptr::null_mut();

unsafe extern "C" fn sealing_thread_fn(_arg: *mut c_void) -> c_int {
    let r: c_int;

    /*
     * This thread first waits 200ms so any pending operation in the parent
     * is correctly started. After that, it tries to seal @global_mfd as
     * SEAL_WRITE. This _must_ fail as the parent thread has a read() into
     * that memory mapped object still ongoing.
     * We then wait one more second and try sealing again. This time it
     * must succeed as there shouldn't be anyone else pinning the pages.
     */

    /* wait 200ms for FUSE-request to be active */
    usleep(200000);

    /* unmount mapping before sealing to avoid i_mmap_writable failures */
    munmap(global_p, mfd_def_size);

    /* Try sealing the global file; expect EBUSY or success. Current
     * kernels will never succeed, but in the future, kernels might
     * implement page-replacements or other fancy ways to avoid racing
     * writes. */
    r = mfd_busy_add_seals(global_mfd, F_SEAL_WRITE);
    if r >= 0 {
        printf(b"HURRAY! This kernel fixed GUP races!\n\0".as_ptr() as *const c_char);
    } else {
        /* wait 1s more so the FUSE-request is done */
        sleep(1);

        /* try sealing the global file again */
        mfd_assert_add_seals(global_mfd, F_SEAL_WRITE);
    }

    0
}

unsafe fn spawn_sealing_thread() -> pid_t {
    let stack: *mut u8;
    let pid: pid_t;

    stack = malloc(STACK_SIZE) as *mut u8;
    if stack.is_null() {
        printf(b"malloc(STACK_SIZE) failed: %m\n\0".as_ptr() as *const c_char);
        abort();
    }

    pid = clone(
        sealing_thread_fn,
        stack.add(STACK_SIZE) as *mut c_void,
        SIGCHLD | CLONE_FILES | CLONE_FS | CLONE_VM,
        core::ptr::null_mut(),
    );
    if pid < 0 {
        printf(b"clone() failed: %m\n\0".as_ptr() as *const c_char);
        abort();
    }

    pid
}

unsafe fn join_sealing_thread(pid: pid_t) {
    waitpid(pid, core::ptr::null_mut(), 0);
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let zero: *mut c_char;
    let fd: c_int;
    let mfd: c_int;
    let mut r: ssize_t;
    let mut p: *mut c_void;
    let was_sealed: c_int;
    let pid: pid_t;

    if argc < 2 {
        printf(b"error: please pass path to file in fuse_mnt mount-point\n\0".as_ptr() as *const c_char);
        abort();
    }

    if argc >= 3 {
        if strcmp(*argv.add(2), b"hugetlbfs\0".as_ptr() as *const c_char) == 0 {
            let hpage_size: c_ulong = default_huge_page_size();

            if hpage_size == 0 {
                printf(b"Unable to determine huge page size\n\0".as_ptr() as *const c_char);
                abort();
            }

            hugetlbfs_test = 1;
            mfd_def_size = (hpage_size * 2) as size_t;
        } else {
            printf(
                b"Unknown option: %s\n\0".as_ptr() as *const c_char,
                *argv.add(2),
            );
            abort();
        }
    }

    zero = calloc(core::mem::size_of::<c_char>(), mfd_def_size) as *mut c_char;

    /* open FUSE memfd file for GUP testing */
    printf(b"opening: %s\n\0".as_ptr() as *const c_char, *argv.add(1));
    fd = open(*argv.add(1), O_RDONLY | O_CLOEXEC);
    if fd < 0 {
        printf(
            b"cannot open(\"%s\"): %m\n\0".as_ptr() as *const c_char,
            *argv.add(1),
        );
        abort();
    }

    /* create new memfd-object */
    mfd = mfd_assert_new(
        b"kern_memfd_fuse\0".as_ptr() as *const c_char,
        mfd_def_size as loff_t,
        MFD_CLOEXEC | MFD_ALLOW_SEALING,
    );

    /* mmap memfd-object for writing */
    p = mfd_assert_mmap_shared(mfd);

    /* pass mfd+mapping to a separate sealing-thread which tries to seal
     * the memfd objects with SEAL_WRITE while we write into it */
    global_mfd = mfd;
    global_p = p;
    pid = spawn_sealing_thread();

    /* Use read() on the FUSE file to read into our memory-mapped memfd
     * object. This races the other thread which tries to seal the
     * memfd-object.
     * If @fd is on the memfd-fake-FUSE-FS, the read() is delayed by 1s.
     * This guarantees that the receive-buffer is pinned for 1s until the
     * data is written into it. The racing ADD_SEALS should thus fail as
     * the pages are still pinned. */
    r = read(fd, p, mfd_def_size);
    if r < 0 {
        printf(b"read() failed: %m\n\0".as_ptr() as *const c_char);
        abort();
    } else if r == 0 {
        printf(b"unexpected EOF on read()\n\0".as_ptr() as *const c_char);
        abort();
    }

    was_sealed = (mfd_assert_get_seals(mfd) & F_SEAL_WRITE) as c_int;

    /* Wait for sealing-thread to finish and verify that it
     * successfully sealed the file after the second try. */
    join_sealing_thread(pid);
    mfd_assert_has_seals(mfd, F_SEAL_WRITE);

    /* *IF* the memfd-object was sealed at the time our read() returned,
     * then the kernel did a page-replacement or canceled the read() (or
     * whatever magic it did..). In that case, the memfd object is still
     * all zero.
     * In case the memfd-object was *not* sealed, the read() was successful
     * and the memfd object must *not* be all zero.
     * Note that in real scenarios, there might be a mixture of both, but
     * in this test-cases, we have explicit 200ms delays which should be
     * enough to avoid any in-flight writes. */

    p = mfd_assert_mmap_private(mfd);
    if was_sealed != 0 && memcmp(p, zero as *const c_void, mfd_def_size) != 0 {
        printf(b"memfd sealed during read() but data not discarded\n\0".as_ptr() as *const c_char);
        abort();
    } else if was_sealed == 0 && memcmp(p, zero as *const c_void, mfd_def_size) == 0 {
        printf(b"memfd sealed after read() but data discarded\n\0".as_ptr() as *const c_char);
        abort();
    }

    close(mfd);
    close(fd);

    printf(b"fuse: DONE\n\0".as_ptr() as *const c_char);
    free(zero as *mut c_void);

    0
}
