// SPDX-License-Identifier: GPL-2.0
// C includes removed; this translation expects libc and the common memfd test
// helpers/constants to be supplied by the surrounding build, as in common.h.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(unused_variables)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;
use std::ffi::CString;

type size_t = usize;
type ssize_t = isize;
type loff_t = i64;
type mode_t = libc::mode_t;
type pid_t = libc::pid_t;

const MEMFD_STR: &[u8] = b"memfd:\0";
const MEMFD_HUGE_STR: &[u8] = b"memfd-hugetlb:\0";
const SHARED_FT_STR: &[u8] = b"(shared file-table)\0";

const MFD_DEF_SIZE: size_t = 8192;
const STACK_SIZE: usize = 65536;

const F_SEAL_EXEC: c_uint = 0x0020;
const F_WX_SEALS: c_uint = (libc::F_SEAL_SHRINK as c_uint)
    | (libc::F_SEAL_GROW as c_uint)
    | (libc::F_SEAL_WRITE as c_uint)
    | (libc::F_SEAL_FUTURE_WRITE as c_uint)
    | F_SEAL_EXEC;

const MFD_NOEXEC_SEAL: c_uint = 0x0008;
const SEM_KEY: libc::key_t = 0xdeadbeef_u32 as libc::key_t;

#[repr(C)]
union semun {
    val: c_int,
    buf: *mut libc::semid_ds,
    array: *mut libc::c_ushort,
    __buf: *mut libc::seminfo,
}

static mut mfd_def_size: size_t = MFD_DEF_SIZE;
static mut memfd_str: *const c_char = MEMFD_STR.as_ptr() as *const c_char;

unsafe extern "C" {
    static mut hugetlbfs_test: c_int;
    fn sys_memfd_create(name: *const c_char, flags: c_uint) -> c_int;
    fn default_huge_page_size() -> c_ulong;
    fn fallocate(fd: c_int, mode: c_int, offset: loff_t, len: loff_t) -> c_int;
    fn clone(
        fn_: extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
    ) -> pid_t;
    fn semget(key: libc::key_t, nsems: c_int, semflg: c_int) -> c_int;
    fn semop(semid: c_int, sops: *mut libc::sembuf, nsops: size_t) -> c_int;
    fn semctl(semid: c_int, semnum: c_int, cmd: c_int, ...) -> c_int;
}

unsafe fn fd2name(fd: c_int, buf: *mut c_char, bufsize: size_t) -> ssize_t {
    let mut buf1 = [0 as c_char; libc::PATH_MAX as usize];
    let size = libc::snprintf(
        buf1.as_mut_ptr(),
        libc::PATH_MAX as usize,
        c"/proc/self/fd/%d".as_ptr(),
        fd,
    );
    if size < 0 {
        libc::printf(c"snprintf(%d) failed on %m\n".as_ptr(), fd);
        libc::abort();
    }

    /*
     * reserver one byte for string termination.
     */
    let nbytes = libc::readlink(buf1.as_ptr(), buf, bufsize - 1);
    if nbytes == -1 {
        libc::printf(c"readlink(%s) failed %m\n".as_ptr(), buf1.as_ptr());
        libc::abort();
    }
    *buf.add(nbytes as usize) = 0;
    nbytes
}

unsafe fn mfd_assert_new(name: *const c_char, sz: loff_t, flags: c_uint) -> c_int {
    let fd = sys_memfd_create(name, flags);
    if fd < 0 {
        libc::printf(c"memfd_create(\"%s\", %u) failed: %m\n".as_ptr(), name, flags);
        libc::abort();
    }

    let r = libc::ftruncate(fd, sz as libc::off_t);
    if r < 0 {
        libc::printf(c"ftruncate(%llu) failed: %m\n".as_ptr(), sz as c_ulong);
        libc::abort();
    }

    fd
}

unsafe fn sysctl_assert_write(val: *const c_char) {
    let fd = libc::open(c"/proc/sys/vm/memfd_noexec".as_ptr(), libc::O_WRONLY | libc::O_CLOEXEC);

    if fd < 0 {
        libc::printf(c"open sysctl failed: %m\n".as_ptr());
        libc::abort();
    }

    if libc::write(fd, val as *const c_void, libc::strlen(val)) < 0 {
        libc::printf(c"write sysctl %s failed: %m\n".as_ptr(), val);
        libc::abort();
    }
}

unsafe fn sysctl_fail_write(val: *const c_char) {
    let fd = libc::open(c"/proc/sys/vm/memfd_noexec".as_ptr(), libc::O_WRONLY | libc::O_CLOEXEC);

    if fd < 0 {
        libc::printf(c"open sysctl failed: %m\n".as_ptr());
        libc::abort();
    }

    if libc::write(fd, val as *const c_void, libc::strlen(val)) >= 0 {
        libc::printf(c"write sysctl %s succeeded, but failure expected\n".as_ptr(), val);
        libc::abort();
    }
}

unsafe fn sysctl_assert_equal(val: *const c_char) {
    let mut buf = [0 as c_char; 128];
    let fd = libc::open(c"/proc/sys/vm/memfd_noexec".as_ptr(), libc::O_RDONLY | libc::O_CLOEXEC);

    if fd < 0 {
        libc::printf(c"open sysctl failed: %m\n".as_ptr());
        libc::abort();
    }

    if libc::read(fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 128]>()) < 0 {
        libc::printf(c"read sysctl failed: %m\n".as_ptr());
        libc::abort();
    }

    /* Strip trailing whitespace. */
    let mut p = buf.as_mut_ptr();
    while libc::isspace(*p as c_int) == 0 {
        p = p.add(1);
    }
    *p = 0;

    if libc::strcmp(buf.as_ptr(), val) != 0 {
        libc::printf(c"unexpected sysctl value: expected %s, got %s\n".as_ptr(), val, buf.as_ptr());
        libc::abort();
    }
}

unsafe fn mfd_assert_reopen_fd(fd_in: c_int) -> c_int {
    let mut path = [0 as c_char; 100];
    libc::sprintf(path.as_mut_ptr(), c"/proc/self/fd/%d".as_ptr(), fd_in);

    let fd = libc::open(path.as_ptr(), libc::O_RDWR);
    if fd < 0 {
        libc::printf(c"re-open of existing fd %d failed\n".as_ptr(), fd_in);
        libc::abort();
    }

    fd
}

unsafe fn mfd_fail_new(name: *const c_char, flags: c_uint) {
    let r = sys_memfd_create(name, flags);
    if r >= 0 {
        libc::printf(
            c"memfd_create(\"%s\", %u) succeeded, but failure expected\n".as_ptr(),
            if !name.is_null() { name } else { c"NULL".as_ptr() },
            flags,
        );
        libc::close(r);
        libc::abort();
    }
}

unsafe fn mfd_assert_get_seals(fd: c_int) -> c_uint {
    let r = libc::fcntl(fd, libc::F_GET_SEALS);
    if r < 0 {
        libc::printf(c"GET_SEALS(%d) failed: %m\n".as_ptr(), fd);
        libc::abort();
    }

    r as c_uint
}

unsafe fn mfd_assert_has_seals(fd: c_int, seals: c_uint) {
    let mut buf = [0 as c_char; libc::PATH_MAX as usize];
    fd2name(fd, buf.as_mut_ptr(), libc::PATH_MAX as usize);

    let s = mfd_assert_get_seals(fd);
    if s != seals {
        libc::printf(c"%u != %u = GET_SEALS(%s)\n".as_ptr(), seals, s, buf.as_ptr());
        libc::abort();
    }
}

unsafe fn mfd_assert_add_seals(fd: c_int, seals: c_uint) {
    let s = mfd_assert_get_seals(fd);
    let r = libc::fcntl(fd, libc::F_ADD_SEALS, seals);
    if r < 0 {
        libc::printf(c"ADD_SEALS(%d, %u -> %u) failed: %m\n".as_ptr(), fd, s, seals);
        libc::abort();
    }
}

unsafe fn mfd_fail_add_seals(fd: c_int, seals: c_uint) {
    let r = libc::fcntl(fd, libc::F_GET_SEALS);
    let s = if r < 0 { 0 } else { r as c_uint };

    let r = libc::fcntl(fd, libc::F_ADD_SEALS, seals);
    if r >= 0 {
        libc::printf(c"ADD_SEALS(%d, %u -> %u) didn't fail as expected\n".as_ptr(), fd, s, seals);
        libc::abort();
    }
}

unsafe fn mfd_assert_size(fd: c_int, size: size_t) {
    let mut st: libc::stat = zeroed();
    let r = libc::fstat(fd, &mut st);
    if r < 0 {
        libc::printf(c"fstat(%d) failed: %m\n".as_ptr(), fd);
        libc::abort();
    } else if st.st_size != size as libc::off_t {
        libc::printf(
            c"wrong file size %lld, but expected %lld\n".as_ptr(),
            st.st_size as libc::c_longlong,
            size as libc::c_longlong,
        );
        libc::abort();
    }
}

unsafe fn mfd_assert_dup(fd: c_int) -> c_int {
    let r = libc::dup(fd);
    if r < 0 {
        libc::printf(c"dup(%d) failed: %m\n".as_ptr(), fd);
        libc::abort();
    }
    r
}

unsafe fn mfd_assert_mmap_shared(fd: c_int) -> *mut c_void {
    let p = libc::mmap(ptr::null_mut(), mfd_def_size, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0);
    if p == libc::MAP_FAILED {
        libc::printf(c"mmap() failed: %m\n".as_ptr());
        libc::abort();
    }
    p
}

unsafe fn mfd_assert_mmap_read_shared(fd: c_int) -> *mut c_void {
    let p = libc::mmap(ptr::null_mut(), mfd_def_size, libc::PROT_READ, libc::MAP_SHARED, fd, 0);
    if p == libc::MAP_FAILED {
        libc::printf(c"mmap() failed: %m\n".as_ptr());
        libc::abort();
    }
    p
}

unsafe fn mfd_assert_mmap_private(fd: c_int) -> *mut c_void {
    let p = libc::mmap(ptr::null_mut(), mfd_def_size, libc::PROT_READ, libc::MAP_PRIVATE, fd, 0);
    if p == libc::MAP_FAILED {
        libc::printf(c"mmap() failed: %m\n".as_ptr());
        libc::abort();
    }
    p
}

unsafe fn mfd_assert_open(fd: c_int, flags: c_int, mode: mode_t) -> c_int {
    let mut buf = [0 as c_char; 512];
    libc::sprintf(buf.as_mut_ptr(), c"/proc/self/fd/%d".as_ptr(), fd);
    let r = libc::open(buf.as_ptr(), flags, mode);
    if r < 0 {
        libc::printf(c"open(%s) failed: %m\n".as_ptr(), buf.as_ptr());
        libc::abort();
    }
    r
}

unsafe fn mfd_fail_open(fd: c_int, flags: c_int, mode: mode_t) {
    let mut buf = [0 as c_char; 512];
    libc::sprintf(buf.as_mut_ptr(), c"/proc/self/fd/%d".as_ptr(), fd);
    let r = libc::open(buf.as_ptr(), flags, mode);
    if r >= 0 {
        libc::printf(c"open(%s) didn't fail as expected\n".as_ptr(), buf.as_ptr());
        libc::abort();
    }
}

unsafe fn mfd_assert_read(fd: c_int) {
    let mut buf = [0 as c_char; 16];
    let l = libc::read(fd, buf.as_mut_ptr() as *mut c_void, size_of::<[c_char; 16]>());
    if l != size_of::<[c_char; 16]>() as ssize_t {
        libc::printf(c"read() failed: %m\n".as_ptr());
        libc::abort();
    }

    /* verify PROT_READ *is* allowed */
    let mut p = libc::mmap(ptr::null_mut(), mfd_def_size, libc::PROT_READ, libc::MAP_PRIVATE, fd, 0);
    if p == libc::MAP_FAILED {
        libc::printf(c"mmap() failed: %m\n".as_ptr());
        libc::abort();
    }
    libc::munmap(p, mfd_def_size);

    /* verify MAP_PRIVATE is *always* allowed (even writable) */
    p = libc::mmap(ptr::null_mut(), mfd_def_size, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE, fd, 0);
    if p == libc::MAP_FAILED {
        libc::printf(c"mmap() failed: %m\n".as_ptr());
        libc::abort();
    }
    libc::munmap(p, mfd_def_size);
}

/* Test that PROT_READ + MAP_SHARED mappings work. */
unsafe fn mfd_assert_read_shared(fd: c_int) {
    /* verify PROT_READ and MAP_SHARED *is* allowed */
    let p = libc::mmap(ptr::null_mut(), mfd_def_size, libc::PROT_READ, libc::MAP_SHARED, fd, 0);
    if p == libc::MAP_FAILED {
        libc::printf(c"mmap() failed: %m\n".as_ptr());
        libc::abort();
    }
    libc::munmap(p, mfd_def_size);
}

unsafe fn mfd_assert_fork_private_write(fd: c_int) {
    let p = libc::mmap(ptr::null_mut(), mfd_def_size, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_PRIVATE, fd, 0) as *mut c_int;
    if p as *mut c_void == libc::MAP_FAILED {
        libc::printf(c"mmap() failed: %m\n".as_ptr());
        libc::abort();
    }

    *p.add(0) = 22;

    let pid = libc::fork();
    if pid == 0 {
        *p.add(0) = 33;
        libc::exit(0);
    } else {
        libc::waitpid(pid, ptr::null_mut(), 0);
        if *p.add(0) != 22 {
            libc::printf(c"MAP_PRIVATE copy-on-write failed: %m\n".as_ptr());
            libc::abort();
        }
    }

    libc::munmap(p as *mut c_void, mfd_def_size);
}

unsafe fn mfd_assert_write(fd: c_int) {
    /*
     * huegtlbfs does not support write, but we want to
     * verify everything else here.
     */
    if hugetlbfs_test == 0 {
        /* verify write() succeeds */
        let l = libc::write(fd, c"\0\0\0\0".as_ptr() as *const c_void, 4);
        if l != 4 {
            libc::printf(c"write() failed: %m\n".as_ptr());
            libc::abort();
        }
    }

    /* verify PROT_READ | PROT_WRITE is allowed */
    let mut p = libc::mmap(ptr::null_mut(), mfd_def_size, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0);
    if p == libc::MAP_FAILED {
        libc::printf(c"mmap() failed: %m\n".as_ptr());
        libc::abort();
    }
    *(p as *mut c_char) = 0;
    libc::munmap(p, mfd_def_size);

    /* verify PROT_WRITE is allowed */
    p = libc::mmap(ptr::null_mut(), mfd_def_size, libc::PROT_WRITE, libc::MAP_SHARED, fd, 0);
    if p == libc::MAP_FAILED {
        libc::printf(c"mmap() failed: %m\n".as_ptr());
        libc::abort();
    }
    *(p as *mut c_char) = 0;
    libc::munmap(p, mfd_def_size);

    /* verify PROT_READ with MAP_SHARED is allowed and a following
     * mprotect(PROT_WRITE) allows writing */
    p = libc::mmap(ptr::null_mut(), mfd_def_size, libc::PROT_READ, libc::MAP_SHARED, fd, 0);
    if p == libc::MAP_FAILED {
        libc::printf(c"mmap() failed: %m\n".as_ptr());
        libc::abort();
    }

    let r = libc::mprotect(p, mfd_def_size, libc::PROT_READ | libc::PROT_WRITE);
    if r < 0 {
        libc::printf(c"mprotect() failed: %m\n".as_ptr());
        libc::abort();
    }

    *(p as *mut c_char) = 0;
    libc::munmap(p, mfd_def_size);

    /* verify PUNCH_HOLE works */
    let r = fallocate(fd, libc::FALLOC_FL_PUNCH_HOLE | libc::FALLOC_FL_KEEP_SIZE, 0, mfd_def_size as loff_t);
    if r < 0 {
        libc::printf(c"fallocate(PUNCH_HOLE) failed: %m\n".as_ptr());
        libc::abort();
    }
}

unsafe fn mfd_fail_write(fd: c_int) {
    /* verify write() fails */
    let l = libc::write(fd, c"data".as_ptr() as *const c_void, 4);
    if l != -(libc::EPERM as ssize_t) {
        libc::printf(c"expected EPERM on write(), but got %d: %m\n".as_ptr(), l as c_int);
        libc::abort();
    }

    /* verify PROT_READ | PROT_WRITE is not allowed */
    let mut p = libc::mmap(ptr::null_mut(), mfd_def_size, libc::PROT_READ | libc::PROT_WRITE, libc::MAP_SHARED, fd, 0);
    if p != libc::MAP_FAILED {
        libc::printf(c"mmap() didn't fail as expected\n".as_ptr());
        libc::abort();
    }

    /* verify PROT_WRITE is not allowed */
    p = libc::mmap(ptr::null_mut(), mfd_def_size, libc::PROT_WRITE, libc::MAP_SHARED, fd, 0);
    if p != libc::MAP_FAILED {
        libc::printf(c"mmap() didn't fail as expected\n".as_ptr());
        libc::abort();
    }

    /* Verify PROT_READ with MAP_SHARED with a following mprotect is not
     * allowed. Note that for r/w the kernel already prevents the mmap. */
    p = libc::mmap(ptr::null_mut(), mfd_def_size, libc::PROT_READ, libc::MAP_SHARED, fd, 0);
    if p != libc::MAP_FAILED {
        let r = libc::mprotect(p, mfd_def_size, libc::PROT_READ | libc::PROT_WRITE);
        if r >= 0 {
            libc::printf(c"mmap()+mprotect() didn't fail as expected\n".as_ptr());
            libc::abort();
        }
        libc::munmap(p, mfd_def_size);
    }

    /* verify PUNCH_HOLE fails */
    let r = fallocate(fd, libc::FALLOC_FL_PUNCH_HOLE | libc::FALLOC_FL_KEEP_SIZE, 0, mfd_def_size as loff_t);
    if r >= 0 {
        libc::printf(c"fallocate(PUNCH_HOLE) didn't fail as expected\n".as_ptr());
        libc::abort();
    }
}

unsafe fn mfd_assert_shrink(fd: c_int) {
    let r = libc::ftruncate(fd, (mfd_def_size / 2) as libc::off_t);
    if r < 0 {
        libc::printf(c"ftruncate(SHRINK) failed: %m\n".as_ptr());
        libc::abort();
    }
    mfd_assert_size(fd, mfd_def_size / 2);

    let fd2 = mfd_assert_open(fd, libc::O_RDWR | libc::O_CREAT | libc::O_TRUNC, (libc::S_IRUSR | libc::S_IWUSR) as mode_t);
    libc::close(fd2);

    mfd_assert_size(fd, 0);
}

unsafe fn mfd_fail_shrink(fd: c_int) {
    let r = libc::ftruncate(fd, (mfd_def_size / 2) as libc::off_t);
    if r >= 0 {
        libc::printf(c"ftruncate(SHRINK) didn't fail as expected\n".as_ptr());
        libc::abort();
    }

    mfd_fail_open(fd, libc::O_RDWR | libc::O_CREAT | libc::O_TRUNC, (libc::S_IRUSR | libc::S_IWUSR) as mode_t);
}

unsafe fn mfd_assert_grow(fd: c_int) {
    let mut r = libc::ftruncate(fd, (mfd_def_size * 2) as libc::off_t);
    if r < 0 {
        libc::printf(c"ftruncate(GROW) failed: %m\n".as_ptr());
        libc::abort();
    }
    mfd_assert_size(fd, mfd_def_size * 2);

    r = fallocate(fd, 0, 0, (mfd_def_size * 4) as loff_t);
    if r < 0 {
        libc::printf(c"fallocate(ALLOC) failed: %m\n".as_ptr());
        libc::abort();
    }
    mfd_assert_size(fd, mfd_def_size * 4);
}

unsafe fn mfd_fail_grow(fd: c_int) {
    let mut r = libc::ftruncate(fd, (mfd_def_size * 2) as libc::off_t);
    if r >= 0 {
        libc::printf(c"ftruncate(GROW) didn't fail as expected\n".as_ptr());
        libc::abort();
    }

    r = fallocate(fd, 0, 0, (mfd_def_size * 4) as loff_t);
    if r >= 0 {
        libc::printf(c"fallocate(ALLOC) didn't fail as expected\n".as_ptr());
        libc::abort();
    }
}

unsafe fn mfd_assert_grow_write(fd: c_int) {
    static mut buf: *mut c_char = ptr::null_mut();
    /* hugetlbfs does not support write */
    if hugetlbfs_test != 0 {
        return;
    }

    buf = libc::calloc(1, mfd_def_size * 8) as *mut c_char;
    if buf.is_null() {
        libc::printf(c"calloc(1, %zu) failed: %m\n".as_ptr(), mfd_def_size * 8);
        libc::abort();
    }

    let l = libc::pwrite(fd, buf as *const c_void, mfd_def_size * 8, 0);
    if l != (mfd_def_size * 8) as ssize_t {
        libc::printf(c"pwrite() failed: %m\n".as_ptr());
        libc::abort();
    }
    mfd_assert_size(fd, mfd_def_size * 8);
}

unsafe fn mfd_fail_grow_write(fd: c_int) {
    static mut buf: *mut c_char = ptr::null_mut();
    /* hugetlbfs does not support write */
    if hugetlbfs_test != 0 {
        return;
    }

    buf = libc::malloc(mfd_def_size * 8) as *mut c_char;
    if buf.is_null() {
        libc::printf(c"malloc(%zu) failed: %m\n".as_ptr(), mfd_def_size * 8);
        libc::abort();
    }

    let l = libc::pwrite(fd, buf as *const c_void, mfd_def_size * 8, 0);
    if l == (mfd_def_size * 8) as ssize_t {
        libc::printf(c"pwrite() didn't fail as expected\n".as_ptr());
        libc::abort();
    }
}

unsafe fn mfd_assert_mode(fd: c_int, mode: c_int) {
    let mut st: libc::stat = zeroed();
    let mut buf = [0 as c_char; libc::PATH_MAX as usize];

    fd2name(fd, buf.as_mut_ptr(), libc::PATH_MAX as usize);

    if libc::fstat(fd, &mut st) < 0 {
        libc::printf(c"fstat(%s) failed: %m\n".as_ptr(), buf.as_ptr());
        libc::abort();
    }

    if (st.st_mode & 0o7777) as c_int != mode {
        libc::printf(
            c"fstat(%s) wrong file mode 0%04o, but expected 0%04o\n".as_ptr(),
            buf.as_ptr(),
            (st.st_mode & 0o7777) as c_int,
            mode,
        );
        libc::abort();
    }
}

unsafe fn mfd_assert_chmod(fd: c_int, mode: c_int) {
    let mut buf = [0 as c_char; libc::PATH_MAX as usize];
    fd2name(fd, buf.as_mut_ptr(), libc::PATH_MAX as usize);

    if libc::fchmod(fd, mode as mode_t) < 0 {
        libc::printf(c"fchmod(%s, 0%04o) failed: %m\n".as_ptr(), buf.as_ptr(), mode);
        libc::abort();
    }

    mfd_assert_mode(fd, mode);
}

unsafe fn mfd_fail_chmod(fd: c_int, mode: c_int) {
    let mut st: libc::stat = zeroed();
    let mut buf = [0 as c_char; libc::PATH_MAX as usize];

    fd2name(fd, buf.as_mut_ptr(), libc::PATH_MAX as usize);

    if libc::fstat(fd, &mut st) < 0 {
        libc::printf(c"fstat(%s) failed: %m\n".as_ptr(), buf.as_ptr());
        libc::abort();
    }

    if libc::fchmod(fd, mode as mode_t) == 0 {
        libc::printf(c"fchmod(%s, 0%04o) didn't fail as expected\n".as_ptr(), buf.as_ptr(), mode);
        libc::abort();
    }

    /* verify that file mode bits did not change */
    mfd_assert_mode(fd, (st.st_mode & 0o7777) as c_int);
}

extern "C" fn idle_thread_fn(_arg: *mut c_void) -> c_int {
    unsafe {
        let mut set: libc::sigset_t = zeroed();
        let mut sig: c_int = 0;

        /* dummy waiter; SIGTERM terminates us anyway */
        libc::sigemptyset(&mut set);
        libc::sigaddset(&mut set, libc::SIGTERM);
        libc::sigwait(&set, &mut sig);

        0
    }
}

unsafe fn spawn_thread(flags: c_uint, fn_: extern "C" fn(*mut c_void) -> c_int, arg: *mut c_void) -> pid_t {
    let stack = libc::malloc(STACK_SIZE) as *mut u8;
    if stack.is_null() {
        libc::printf(c"malloc(STACK_SIZE) failed: %m\n".as_ptr());
        libc::abort();
    }

    let pid = clone(fn_, stack.add(STACK_SIZE) as *mut c_void, libc::SIGCHLD | flags as c_int, arg);
    if pid < 0 {
        libc::printf(c"clone() failed: %m\n".as_ptr());
        libc::abort();
    }
    pid
}

unsafe fn join_thread(pid: pid_t) {
    let mut wstatus: c_int = 0;

    if libc::waitpid(pid, &mut wstatus, 0) < 0 {
        libc::printf(c"newpid thread: waitpid() failed: %m\n".as_ptr());
        libc::abort();
    }

    if libc::WIFEXITED(wstatus) && libc::WEXITSTATUS(wstatus) != 0 {
        libc::printf(c"newpid thread: exited with non-zero error code %d\n".as_ptr(), libc::WEXITSTATUS(wstatus));
        libc::abort();
    }

    if libc::WIFSIGNALED(wstatus) {
        libc::printf(c"newpid thread: killed by signal %d\n".as_ptr(), libc::WTERMSIG(wstatus));
        libc::abort();
    }
}

unsafe fn spawn_idle_thread(flags: c_uint) -> pid_t {
    spawn_thread(flags, idle_thread_fn, ptr::null_mut())
}

unsafe fn join_idle_thread(pid: pid_t) {
    libc::kill(pid, libc::SIGTERM);
    libc::waitpid(pid, ptr::null_mut(), 0);
}

unsafe fn test_create() {
    let mut buf = [0 as c_char; 2048];
    let mut fd: c_int;

    libc::printf(c"%s CREATE\n".as_ptr(), memfd_str);

    /* test NULL name */
    mfd_fail_new(ptr::null(), 0);

    /* test over-long name (not zero-terminated) */
    libc::memset(buf.as_mut_ptr() as *mut c_void, 0xff, size_of::<[c_char; 2048]>());
    mfd_fail_new(buf.as_ptr(), 0);

    /* test over-long zero-terminated name */
    libc::memset(buf.as_mut_ptr() as *mut c_void, 0xff, size_of::<[c_char; 2048]>());
    buf[size_of::<[c_char; 2048]>() - 1] = 0;
    mfd_fail_new(buf.as_ptr(), 0);

    /* verify "" is a valid name */
    fd = mfd_assert_new(c"".as_ptr(), 0, 0);
    libc::close(fd);

    /* verify invalid O_* open flags */
    mfd_fail_new(c"".as_ptr(), 0x0100);
    mfd_fail_new(c"".as_ptr(), !(libc::MFD_CLOEXEC as c_uint));
    mfd_fail_new(c"".as_ptr(), !(libc::MFD_ALLOW_SEALING as c_uint));
    mfd_fail_new(c"".as_ptr(), !0);
    mfd_fail_new(c"".as_ptr(), 0x80000000);

    /* verify EXEC and NOEXEC_SEAL can't both be set */
    mfd_fail_new(c"".as_ptr(), libc::MFD_EXEC as c_uint | MFD_NOEXEC_SEAL);

    /* verify MFD_CLOEXEC is allowed */
    fd = mfd_assert_new(c"".as_ptr(), 0, libc::MFD_CLOEXEC as c_uint);
    libc::close(fd);

    /* verify MFD_ALLOW_SEALING is allowed */
    fd = mfd_assert_new(c"".as_ptr(), 0, libc::MFD_ALLOW_SEALING as c_uint);
    libc::close(fd);

    /* verify MFD_ALLOW_SEALING | MFD_CLOEXEC is allowed */
    fd = mfd_assert_new(c"".as_ptr(), 0, libc::MFD_ALLOW_SEALING as c_uint | libc::MFD_CLOEXEC as c_uint);
    libc::close(fd);
}

unsafe fn test_basic() {
    let fd: c_int;

    libc::printf(c"%s BASIC\n".as_ptr(), memfd_str);

    fd = mfd_assert_new(c"kern_memfd_basic".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);

    /* add basic seals */
    mfd_assert_has_seals(fd, 0);
    mfd_assert_add_seals(fd, libc::F_SEAL_SHRINK as c_uint | libc::F_SEAL_WRITE as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_SHRINK as c_uint | libc::F_SEAL_WRITE as c_uint);

    /* add them again */
    mfd_assert_add_seals(fd, libc::F_SEAL_SHRINK as c_uint | libc::F_SEAL_WRITE as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_SHRINK as c_uint | libc::F_SEAL_WRITE as c_uint);

    /* add more seals and seal against sealing */
    mfd_assert_add_seals(fd, libc::F_SEAL_GROW as c_uint | libc::F_SEAL_SEAL as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_SHRINK as c_uint | libc::F_SEAL_GROW as c_uint | libc::F_SEAL_WRITE as c_uint | libc::F_SEAL_SEAL as c_uint);

    /* verify that sealing no longer works */
    mfd_fail_add_seals(fd, libc::F_SEAL_GROW as c_uint);
    mfd_fail_add_seals(fd, 0);

    libc::close(fd);

    /* verify sealing does not work without MFD_ALLOW_SEALING */
    let fd = mfd_assert_new(c"kern_memfd_basic".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_SEAL as c_uint);
    mfd_fail_add_seals(fd, libc::F_SEAL_SHRINK as c_uint | libc::F_SEAL_GROW as c_uint | libc::F_SEAL_WRITE as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_SEAL as c_uint);
    libc::close(fd);
}

unsafe fn test_seal_write() {
    libc::printf(c"%s SEAL-WRITE\n".as_ptr(), memfd_str);

    let fd = mfd_assert_new(c"kern_memfd_seal_write".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_has_seals(fd, 0);
    mfd_assert_add_seals(fd, libc::F_SEAL_WRITE as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_WRITE as c_uint);

    mfd_assert_read(fd);
    mfd_fail_write(fd);
    mfd_assert_shrink(fd);
    mfd_assert_grow(fd);
    mfd_fail_grow_write(fd);

    libc::close(fd);
}

unsafe fn test_seal_future_write() {
    let fd: c_int;
    let fd2: c_int;
    let p: *mut c_void;

    libc::printf(c"%s SEAL-FUTURE-WRITE\n".as_ptr(), memfd_str);

    fd = mfd_assert_new(c"kern_memfd_seal_future_write".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);

    p = mfd_assert_mmap_shared(fd);

    mfd_assert_has_seals(fd, 0);

    mfd_assert_add_seals(fd, libc::F_SEAL_FUTURE_WRITE as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_FUTURE_WRITE as c_uint);

    /* read should pass, writes should fail */
    mfd_assert_read(fd);
    mfd_assert_read_shared(fd);
    mfd_fail_write(fd);

    fd2 = mfd_assert_reopen_fd(fd);
    /* read should pass, writes should still fail */
    mfd_assert_read(fd2);
    mfd_assert_read_shared(fd2);
    mfd_fail_write(fd2);

    mfd_assert_fork_private_write(fd);

    libc::munmap(p, mfd_def_size);
    libc::close(fd2);
    libc::close(fd);
}

unsafe fn test_seal_write_map_read_shared() {
    libc::printf(c"%s SEAL-WRITE-MAP-READ\n".as_ptr(), memfd_str);

    let fd = mfd_assert_new(c"kern_memfd_seal_write_map_read".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);

    mfd_assert_add_seals(fd, libc::F_SEAL_WRITE as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_WRITE as c_uint);

    let p = mfd_assert_mmap_read_shared(fd);

    mfd_assert_read(fd);
    mfd_assert_read_shared(fd);
    mfd_fail_write(fd);

    libc::munmap(p, mfd_def_size);
    libc::close(fd);
}

unsafe fn test_seal_shrink() {
    libc::printf(c"%s SEAL-SHRINK\n".as_ptr(), memfd_str);

    let fd = mfd_assert_new(c"kern_memfd_seal_shrink".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_has_seals(fd, 0);
    mfd_assert_add_seals(fd, libc::F_SEAL_SHRINK as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_SHRINK as c_uint);

    mfd_assert_read(fd);
    mfd_assert_write(fd);
    mfd_fail_shrink(fd);
    mfd_assert_grow(fd);
    mfd_assert_grow_write(fd);

    libc::close(fd);
}

unsafe fn test_seal_grow() {
    libc::printf(c"%s SEAL-GROW\n".as_ptr(), memfd_str);

    let fd = mfd_assert_new(c"kern_memfd_seal_grow".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_has_seals(fd, 0);
    mfd_assert_add_seals(fd, libc::F_SEAL_GROW as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_GROW as c_uint);

    mfd_assert_read(fd);
    mfd_assert_write(fd);
    mfd_assert_shrink(fd);
    mfd_fail_grow(fd);
    mfd_fail_grow_write(fd);

    libc::close(fd);
}

unsafe fn test_seal_resize() {
    libc::printf(c"%s SEAL-RESIZE\n".as_ptr(), memfd_str);

    let fd = mfd_assert_new(c"kern_memfd_seal_resize".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_has_seals(fd, 0);
    mfd_assert_add_seals(fd, libc::F_SEAL_SHRINK as c_uint | libc::F_SEAL_GROW as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_SHRINK as c_uint | libc::F_SEAL_GROW as c_uint);

    mfd_assert_read(fd);
    mfd_assert_write(fd);
    mfd_fail_shrink(fd);
    mfd_fail_grow(fd);
    mfd_fail_grow_write(fd);

    libc::close(fd);
}

unsafe fn test_exec_seal() {
    libc::printf(c"%s SEAL-EXEC\n".as_ptr(), memfd_str);

    libc::printf(c"%s\tApply SEAL_EXEC\n".as_ptr(), memfd_str);
    let mut fd = mfd_assert_new(c"kern_memfd_seal_exec".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint | libc::MFD_EXEC as c_uint);

    mfd_assert_mode(fd, 0o777);
    mfd_assert_chmod(fd, 0o644);

    mfd_assert_has_seals(fd, 0);
    mfd_assert_add_seals(fd, F_SEAL_EXEC);
    mfd_assert_has_seals(fd, F_SEAL_EXEC);

    mfd_assert_chmod(fd, 0o600);
    mfd_fail_chmod(fd, 0o777);
    mfd_fail_chmod(fd, 0o670);
    mfd_fail_chmod(fd, 0o605);
    mfd_fail_chmod(fd, 0o700);
    mfd_fail_chmod(fd, 0o100);
    mfd_assert_chmod(fd, 0o666);
    mfd_assert_write(fd);
    libc::close(fd);

    libc::printf(c"%s\tApply ALL_SEALS\n".as_ptr(), memfd_str);
    fd = mfd_assert_new(c"kern_memfd_seal_exec".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint | libc::MFD_EXEC as c_uint);

    mfd_assert_mode(fd, 0o777);
    mfd_assert_chmod(fd, 0o700);

    mfd_assert_has_seals(fd, 0);
    mfd_assert_add_seals(fd, F_SEAL_EXEC);
    mfd_assert_has_seals(fd, F_WX_SEALS);

    mfd_fail_chmod(fd, 0o711);
    mfd_fail_chmod(fd, 0o600);
    mfd_fail_write(fd);
    libc::close(fd);
}

unsafe fn test_exec_no_seal() {
    libc::printf(c"%s EXEC_NO_SEAL\n".as_ptr(), memfd_str);

    /* Create with EXEC but without ALLOW_SEALING */
    let fd = mfd_assert_new(c"kern_memfd_exec_no_sealing".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_EXEC as c_uint);
    mfd_assert_mode(fd, 0o777);
    mfd_assert_has_seals(fd, libc::F_SEAL_SEAL as c_uint);
    mfd_assert_chmod(fd, 0o666);
    libc::close(fd);
}

unsafe fn test_noexec_seal() {
    libc::printf(c"%s NOEXEC_SEAL\n".as_ptr(), memfd_str);

    /* Create with NOEXEC and ALLOW_SEALING */
    let mut fd = mfd_assert_new(c"kern_memfd_noexec".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint | MFD_NOEXEC_SEAL);
    mfd_assert_mode(fd, 0o666);
    mfd_assert_has_seals(fd, F_SEAL_EXEC);
    mfd_fail_chmod(fd, 0o777);
    libc::close(fd);

    /* Create with NOEXEC but without ALLOW_SEALING */
    fd = mfd_assert_new(c"kern_memfd_noexec".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | MFD_NOEXEC_SEAL);
    mfd_assert_mode(fd, 0o666);
    mfd_assert_has_seals(fd, F_SEAL_EXEC);
    mfd_fail_chmod(fd, 0o777);
    libc::close(fd);
}

unsafe fn test_sysctl_sysctl0() {
    sysctl_assert_equal(c"0".as_ptr());

    let fd = mfd_assert_new(c"kern_memfd_sysctl_0_dfl".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_mode(fd, 0o777);
    mfd_assert_has_seals(fd, 0);
    mfd_assert_chmod(fd, 0o644);
    libc::close(fd);
}

unsafe fn test_sysctl_set_sysctl0() {
    sysctl_assert_write(c"0".as_ptr());
    test_sysctl_sysctl0();
}

unsafe fn test_sysctl_sysctl1() {
    sysctl_assert_equal(c"1".as_ptr());

    let mut fd = mfd_assert_new(c"kern_memfd_sysctl_1_dfl".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_mode(fd, 0o666);
    mfd_assert_has_seals(fd, F_SEAL_EXEC);
    mfd_fail_chmod(fd, 0o777);
    libc::close(fd);

    fd = mfd_assert_new(c"kern_memfd_sysctl_1_exec".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_EXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_mode(fd, 0o777);
    mfd_assert_has_seals(fd, 0);
    mfd_assert_chmod(fd, 0o644);
    libc::close(fd);

    fd = mfd_assert_new(c"kern_memfd_sysctl_1_noexec".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | MFD_NOEXEC_SEAL | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_mode(fd, 0o666);
    mfd_assert_has_seals(fd, F_SEAL_EXEC);
    mfd_fail_chmod(fd, 0o777);
    libc::close(fd);
}

unsafe fn test_sysctl_set_sysctl1() {
    sysctl_assert_write(c"1".as_ptr());
    test_sysctl_sysctl1();
}

unsafe fn test_sysctl_sysctl2() {
    sysctl_assert_equal(c"2".as_ptr());

    let mut fd = mfd_assert_new(c"kern_memfd_sysctl_2_dfl".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_mode(fd, 0o666);
    mfd_assert_has_seals(fd, F_SEAL_EXEC);
    mfd_fail_chmod(fd, 0o777);
    libc::close(fd);

    mfd_fail_new(c"kern_memfd_sysctl_2_exec".as_ptr(), libc::MFD_CLOEXEC as c_uint | libc::MFD_EXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);

    fd = mfd_assert_new(c"kern_memfd_sysctl_2_noexec".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | MFD_NOEXEC_SEAL | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_mode(fd, 0o666);
    mfd_assert_has_seals(fd, F_SEAL_EXEC);
    mfd_fail_chmod(fd, 0o777);
    libc::close(fd);
}

unsafe fn test_sysctl_set_sysctl2() {
    sysctl_assert_write(c"2".as_ptr());
    test_sysctl_sysctl2();
}

extern "C" fn sysctl_simple_child(_arg: *mut c_void) -> c_int {
    unsafe {
        libc::printf(c"%s sysctl 0\n".as_ptr(), memfd_str);
        test_sysctl_set_sysctl0();
        libc::printf(c"%s sysctl 1\n".as_ptr(), memfd_str);
        test_sysctl_set_sysctl1();
        libc::printf(c"%s sysctl 0\n".as_ptr(), memfd_str);
        test_sysctl_set_sysctl0();
        libc::printf(c"%s sysctl 2\n".as_ptr(), memfd_str);
        test_sysctl_set_sysctl2();
        libc::printf(c"%s sysctl 1\n".as_ptr(), memfd_str);
        test_sysctl_set_sysctl1();
        libc::printf(c"%s sysctl 0\n".as_ptr(), memfd_str);
        test_sysctl_set_sysctl0();
        0
    }
}

unsafe fn test_sysctl_simple() {
    let pid = spawn_thread(libc::CLONE_NEWPID as c_uint, sysctl_simple_child, ptr::null_mut());
    join_thread(pid);
}

extern "C" fn sysctl_nested(arg: *mut c_void) -> c_int {
    unsafe {
        let fn_: unsafe fn() = core::mem::transmute(arg);
        fn_();
        0
    }
}

extern "C" fn sysctl_nested_wait(arg: *mut c_void) -> c_int {
    unsafe {
        let sem = semget(SEM_KEY, 1, 0o600);
        let mut sembuf: libc::sembuf = zeroed();

        if sem < 0 {
            libc::perror(c"semget:".as_ptr());
            libc::abort();
        }
        sembuf.sem_num = 0;
        sembuf.sem_flg = 0;
        sembuf.sem_op = 0;

        if semop(sem, &mut sembuf, 1) < 0 {
            libc::perror(c"semop:".as_ptr());
            libc::abort();
        }

        sysctl_nested(arg)
    }
}

unsafe fn test_sysctl_sysctl1_failset() {
    sysctl_fail_write(c"0".as_ptr());
    test_sysctl_sysctl1();
}

unsafe fn test_sysctl_sysctl2_failset() {
    sysctl_fail_write(c"1".as_ptr());
    test_sysctl_sysctl2();
    sysctl_fail_write(c"0".as_ptr());
    test_sysctl_sysctl2();
}

extern "C" fn sysctl_nested_child(_arg: *mut c_void) -> c_int {
    unsafe {
        let mut pid: c_int;
        let mut sem: c_int;
        let mut semun: semun = zeroed();
        let mut sembuf: libc::sembuf = zeroed();

        libc::printf(c"%s nested sysctl 0\n".as_ptr(), memfd_str);
        sysctl_assert_write(c"0".as_ptr());
        /* A further nested pidns works the same. */
        pid = spawn_thread(libc::CLONE_NEWPID as c_uint, sysctl_simple_child, ptr::null_mut());
        join_thread(pid);

        libc::printf(c"%s nested sysctl 1\n".as_ptr(), memfd_str);
        sysctl_assert_write(c"1".as_ptr());
        /* Child inherits our setting. */
        pid = spawn_thread(libc::CLONE_NEWPID as c_uint, sysctl_nested, test_sysctl_sysctl1 as usize as *mut c_void);
        join_thread(pid);
        /* Child cannot raise the setting. */
        pid = spawn_thread(libc::CLONE_NEWPID as c_uint, sysctl_nested, test_sysctl_sysctl1_failset as usize as *mut c_void);
        join_thread(pid);
        /* Child can lower the setting. */
        pid = spawn_thread(libc::CLONE_NEWPID as c_uint, sysctl_nested, test_sysctl_set_sysctl2 as usize as *mut c_void);
        join_thread(pid);
        /* Child lowering the setting has no effect on our setting. */
        test_sysctl_sysctl1();

        libc::printf(c"%s nested sysctl 2\n".as_ptr(), memfd_str);
        sysctl_assert_write(c"2".as_ptr());
        /* Child inherits our setting. */
        pid = spawn_thread(libc::CLONE_NEWPID as c_uint, sysctl_nested, test_sysctl_sysctl2 as usize as *mut c_void);
        join_thread(pid);
        /* Child cannot raise the setting. */
        pid = spawn_thread(libc::CLONE_NEWPID as c_uint, sysctl_nested, test_sysctl_sysctl2_failset as usize as *mut c_void);
        join_thread(pid);

        sem = semget(SEM_KEY, 1, libc::IPC_CREAT | 0o600);
        if sem < 0 {
            libc::perror(c"semget:".as_ptr());
            return 1;
        }
        semun.val = 1;
        sembuf.sem_op = -1;
        sembuf.sem_flg = 0;
        sembuf.sem_num = 0;

        /* Verify that the rules are actually inherited after fork. */
        libc::printf(c"%s nested sysctl 0 -> 1 after fork\n".as_ptr(), memfd_str);
        sysctl_assert_write(c"0".as_ptr());
        if semctl(sem, 0, libc::SETVAL, semun) < 0 {
            libc::perror(c"semctl:".as_ptr());
            return 1;
        }
        pid = spawn_thread(libc::CLONE_NEWPID as c_uint, sysctl_nested_wait, test_sysctl_sysctl1_failset as usize as *mut c_void);
        sysctl_assert_write(c"1".as_ptr());
        if semop(sem, &mut sembuf, 1) < 0 {
            libc::perror(c"semop:".as_ptr());
            return 1;
        }
        join_thread(pid);

        libc::printf(c"%s nested sysctl 0 -> 2 after fork\n".as_ptr(), memfd_str);
        sysctl_assert_write(c"0".as_ptr());
        if semctl(sem, 0, libc::SETVAL, semun) < 0 {
            libc::perror(c"semctl:".as_ptr());
            return 1;
        }
        pid = spawn_thread(libc::CLONE_NEWPID as c_uint, sysctl_nested_wait, test_sysctl_sysctl2_failset as usize as *mut c_void);
        sysctl_assert_write(c"2".as_ptr());
        if semop(sem, &mut sembuf, 1) < 0 {
            libc::perror(c"semop:".as_ptr());
            return 1;
        }
        join_thread(pid);

        /*
         * Verify that the current effective setting is saved on fork, meaning
         * that the parent lowering the sysctl doesn't affect already-forked
         * children.
         */
        libc::printf(c"%s nested sysctl 2 -> 1 after fork\n".as_ptr(), memfd_str);
        sysctl_assert_write(c"2".as_ptr());
        if semctl(sem, 0, libc::SETVAL, semun) < 0 {
            libc::perror(c"semctl:".as_ptr());
            return 1;
        }
        pid = spawn_thread(libc::CLONE_NEWPID as c_uint, sysctl_nested_wait, test_sysctl_sysctl2 as usize as *mut c_void);
        sysctl_assert_write(c"1".as_ptr());
        if semop(sem, &mut sembuf, 1) < 0 {
            libc::perror(c"semop:".as_ptr());
            return 1;
        }
        join_thread(pid);

        libc::printf(c"%s nested sysctl 2 -> 0 after fork\n".as_ptr(), memfd_str);
        sysctl_assert_write(c"2".as_ptr());
        if semctl(sem, 0, libc::SETVAL, semun) < 0 {
            libc::perror(c"semctl:".as_ptr());
            return 1;
        }
        pid = spawn_thread(libc::CLONE_NEWPID as c_uint, sysctl_nested_wait, test_sysctl_sysctl2 as usize as *mut c_void);
        sysctl_assert_write(c"0".as_ptr());
        if semop(sem, &mut sembuf, 1) < 0 {
            libc::perror(c"semop:".as_ptr());
            return 1;
        }
        join_thread(pid);

        libc::printf(c"%s nested sysctl 1 -> 0 after fork\n".as_ptr(), memfd_str);
        sysctl_assert_write(c"1".as_ptr());
        if semctl(sem, 0, libc::SETVAL, semun) < 0 {
            libc::perror(c"semctl:".as_ptr());
            return 1;
        }
        pid = spawn_thread(libc::CLONE_NEWPID as c_uint, sysctl_nested_wait, test_sysctl_sysctl1 as usize as *mut c_void);
        sysctl_assert_write(c"0".as_ptr());
        /* Allow child to continue */
        if semop(sem, &mut sembuf, 1) < 0 {
            libc::perror(c"semop:".as_ptr());
            return 1;
        }
        join_thread(pid);

        semctl(sem, 0, libc::IPC_RMID);
        0
    }
}

unsafe fn test_sysctl_nested() {
    let pid = spawn_thread(libc::CLONE_NEWPID as c_uint, sysctl_nested_child, ptr::null_mut());
    join_thread(pid);
}

unsafe fn test_share_dup(banner: *mut c_char, b_suffix: *mut c_char) {
    libc::printf(c"%s %s %s\n".as_ptr(), memfd_str, banner, b_suffix);

    let fd = mfd_assert_new(c"kern_memfd_share_dup".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_has_seals(fd, 0);

    let fd2 = mfd_assert_dup(fd);
    mfd_assert_has_seals(fd2, 0);

    mfd_assert_add_seals(fd, libc::F_SEAL_WRITE as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_WRITE as c_uint);
    mfd_assert_has_seals(fd2, libc::F_SEAL_WRITE as c_uint);

    mfd_assert_add_seals(fd2, libc::F_SEAL_SHRINK as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_WRITE as c_uint | libc::F_SEAL_SHRINK as c_uint);
    mfd_assert_has_seals(fd2, libc::F_SEAL_WRITE as c_uint | libc::F_SEAL_SHRINK as c_uint);

    mfd_assert_add_seals(fd, libc::F_SEAL_SEAL as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_WRITE as c_uint | libc::F_SEAL_SHRINK as c_uint | libc::F_SEAL_SEAL as c_uint);
    mfd_assert_has_seals(fd2, libc::F_SEAL_WRITE as c_uint | libc::F_SEAL_SHRINK as c_uint | libc::F_SEAL_SEAL as c_uint);

    mfd_fail_add_seals(fd, libc::F_SEAL_GROW as c_uint);
    mfd_fail_add_seals(fd2, libc::F_SEAL_GROW as c_uint);
    mfd_fail_add_seals(fd, libc::F_SEAL_SEAL as c_uint);
    mfd_fail_add_seals(fd2, libc::F_SEAL_SEAL as c_uint);

    libc::close(fd2);
    mfd_fail_add_seals(fd, libc::F_SEAL_GROW as c_uint);
    libc::close(fd);
}

unsafe fn test_share_mmap(banner: *mut c_char, b_suffix: *mut c_char) {
    libc::printf(c"%s %s %s\n".as_ptr(), memfd_str, banner, b_suffix);

    let fd = mfd_assert_new(c"kern_memfd_share_mmap".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_has_seals(fd, 0);

    /* shared/writable ref prevents sealing WRITE, but allows others */
    let mut p = mfd_assert_mmap_shared(fd);
    mfd_fail_add_seals(fd, libc::F_SEAL_WRITE as c_uint);
    mfd_assert_has_seals(fd, 0);
    mfd_assert_add_seals(fd, libc::F_SEAL_SHRINK as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_SHRINK as c_uint);
    libc::munmap(p, mfd_def_size);

    /* readable ref allows sealing */
    p = mfd_assert_mmap_private(fd);
    mfd_assert_add_seals(fd, libc::F_SEAL_WRITE as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_WRITE as c_uint | libc::F_SEAL_SHRINK as c_uint);
    libc::munmap(p, mfd_def_size);

    libc::close(fd);
}

unsafe fn test_share_open(banner: *mut c_char, b_suffix: *mut c_char) {
    libc::printf(c"%s %s %s\n".as_ptr(), memfd_str, banner, b_suffix);

    let mut fd = mfd_assert_new(c"kern_memfd_share_open".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_has_seals(fd, 0);

    let mut fd2 = mfd_assert_open(fd, libc::O_RDWR, 0);
    mfd_assert_add_seals(fd, libc::F_SEAL_WRITE as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_WRITE as c_uint);
    mfd_assert_has_seals(fd2, libc::F_SEAL_WRITE as c_uint);

    mfd_assert_add_seals(fd2, libc::F_SEAL_SHRINK as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_WRITE as c_uint | libc::F_SEAL_SHRINK as c_uint);
    mfd_assert_has_seals(fd2, libc::F_SEAL_WRITE as c_uint | libc::F_SEAL_SHRINK as c_uint);

    libc::close(fd);
    fd = mfd_assert_open(fd2, libc::O_RDONLY, 0);

    mfd_fail_add_seals(fd, libc::F_SEAL_SEAL as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_WRITE as c_uint | libc::F_SEAL_SHRINK as c_uint);
    mfd_assert_has_seals(fd2, libc::F_SEAL_WRITE as c_uint | libc::F_SEAL_SHRINK as c_uint);

    libc::close(fd2);
    fd2 = mfd_assert_open(fd, libc::O_RDWR, 0);

    mfd_assert_add_seals(fd2, libc::F_SEAL_SEAL as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_WRITE as c_uint | libc::F_SEAL_SHRINK as c_uint | libc::F_SEAL_SEAL as c_uint);
    mfd_assert_has_seals(fd2, libc::F_SEAL_WRITE as c_uint | libc::F_SEAL_SHRINK as c_uint | libc::F_SEAL_SEAL as c_uint);

    libc::close(fd2);
    libc::close(fd);
}

unsafe fn test_share_fork(banner: *mut c_char, b_suffix: *mut c_char) {
    libc::printf(c"%s %s %s\n".as_ptr(), memfd_str, banner, b_suffix);

    let fd = mfd_assert_new(c"kern_memfd_share_fork".as_ptr(), mfd_def_size as loff_t, libc::MFD_CLOEXEC as c_uint | libc::MFD_ALLOW_SEALING as c_uint);
    mfd_assert_has_seals(fd, 0);

    let pid = spawn_idle_thread(0);
    mfd_assert_add_seals(fd, libc::F_SEAL_SEAL as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_SEAL as c_uint);

    mfd_fail_add_seals(fd, libc::F_SEAL_WRITE as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_SEAL as c_uint);

    join_idle_thread(pid);

    mfd_fail_add_seals(fd, libc::F_SEAL_WRITE as c_uint);
    mfd_assert_has_seals(fd, libc::F_SEAL_SEAL as c_uint);

    libc::close(fd);
}

unsafe fn pid_ns_supported() -> bool {
    libc::access(c"/proc/self/ns/pid".as_ptr(), libc::F_OK) == 0
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let pid: pid_t;

    if argc == 2 {
        if libc::strcmp(*argv.add(1), c"hugetlbfs".as_ptr()) == 0 {
            let hpage_size = default_huge_page_size();

            if hpage_size == 0 {
                libc::printf(c"Unable to determine huge page size\n".as_ptr());
                libc::abort();
            }

            hugetlbfs_test = 1;
            memfd_str = MEMFD_HUGE_STR.as_ptr() as *const c_char;
            mfd_def_size = hpage_size as size_t * 2;
        } else {
            libc::printf(c"Unknown option: %s\n".as_ptr(), *argv.add(1));
            libc::abort();
        }
    }

    test_create();
    test_basic();
    test_exec_seal();
    test_exec_no_seal();
    test_noexec_seal();

    test_seal_write();
    test_seal_future_write();
    test_seal_write_map_read_shared();
    test_seal_shrink();
    test_seal_grow();
    test_seal_resize();

    if pid_ns_supported() {
        test_sysctl_simple();
        test_sysctl_nested();
    } else {
        libc::printf(c"PID namespaces are not supported; skipping sysctl tests\n".as_ptr());
    }

    test_share_dup(c"SHARE-DUP".as_ptr() as *mut c_char, c"".as_ptr() as *mut c_char);
    test_share_mmap(c"SHARE-MMAP".as_ptr() as *mut c_char, c"".as_ptr() as *mut c_char);
    test_share_open(c"SHARE-OPEN".as_ptr() as *mut c_char, c"".as_ptr() as *mut c_char);
    test_share_fork(c"SHARE-FORK".as_ptr() as *mut c_char, c"".as_ptr() as *mut c_char);

    /* Run test-suite in a multi-threaded environment with a shared
     * file-table. */
    pid = spawn_idle_thread((libc::CLONE_FILES | libc::CLONE_FS | libc::CLONE_VM) as c_uint);
    test_share_dup(c"SHARE-DUP".as_ptr() as *mut c_char, SHARED_FT_STR.as_ptr() as *mut c_char);
    test_share_mmap(c"SHARE-MMAP".as_ptr() as *mut c_char, SHARED_FT_STR.as_ptr() as *mut c_char);
    test_share_open(c"SHARE-OPEN".as_ptr() as *mut c_char, SHARED_FT_STR.as_ptr() as *mut c_char);
    test_share_fork(c"SHARE-FORK".as_ptr() as *mut c_char, SHARED_FT_STR.as_ptr() as *mut c_char);
    join_idle_thread(pid);

    libc::printf(c"memfd: DONE\n".as_ptr());

    0
}

fn main() {
    unsafe {
        let args: Vec<CString> = std::env::args()
            .map(|arg| CString::new(arg).unwrap())
            .collect();
        let mut argv: Vec<*mut c_char> = args
            .iter()
            .map(|arg| arg.as_ptr() as *mut c_char)
            .collect();
        argv.push(ptr::null_mut());
        main_impl(args.len() as c_int, argv.as_mut_ptr());
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
