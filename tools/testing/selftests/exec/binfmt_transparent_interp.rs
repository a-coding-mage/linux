// SPDX-License-Identifier: GPL-2.0
/*
 * Asserting interpreter for the transparent binfmt_misc mode. It runs in
 * place of the dispatched binary and verifies the identity the kernel
 * constructed: the aux vector contract, the exe link, argv, cmdline, comm
 * and the write denial on the binary. BINFMT_TEST_BINARY names the binary;
 * the harness execs it with the arguments "argone argtwo". Prints
 * TRANSPARENT_OK and exits 0 when every check holds.
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

type c_char = i8;
type c_int = i32;
type c_ulong = u64;
type c_void = core::ffi::c_void;
type size_t = usize;
type ssize_t = isize;
type dev_t = u64;
type ino_t = u64;
type mode_t = u32;
type nlink_t = u64;
type uid_t = u32;
type gid_t = u32;
type off_t = i64;
type blksize_t = i64;
type blkcnt_t = i64;
type time_t = i64;

const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;
const AT_EXECFD: c_ulong = 2;
const AT_FLAGS: c_ulong = 8;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct timespec {
    pub tv_sec: time_t,
    pub tv_nsec: i64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct stat {
    pub st_dev: dev_t,
    pub st_ino: ino_t,
    pub st_nlink: nlink_t,
    pub st_mode: mode_t,
    pub st_uid: uid_t,
    pub st_gid: gid_t,
    pub __pad0: c_int,
    pub st_rdev: dev_t,
    pub st_size: off_t,
    pub st_blksize: blksize_t,
    pub st_blocks: blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [i64; 3],
}

unsafe extern "C" {
    static mut stderr: *mut c_void;

    static PAYLOAD_ARG1: *const c_char;
    static PAYLOAD_ARG2: *const c_char;

    fn __errno_location() -> *mut c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn fstat(fd: c_int, statbuf: *mut stat) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;

    fn exe_is(path: *const c_char) -> c_int;
    fn comm_is(comm: *const c_char) -> c_int;
    fn write_denied(path: *const c_char) -> c_int;
}

const AT_FLAGS_TRANSPARENT_INTERP: c_ulong = 1 << 1;

static mut fail: c_int = 0;

unsafe fn ok(cond: c_int, what: *const c_char) {
    if cond == 0 {
        fprintf(
            stderr,
            b"TRANSPARENT_FAIL: %s (errno %d)\n\0".as_ptr() as *const c_char,
            what,
            *__errno_location(),
        );
        fail = 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let binary: *const c_char =
        getenv(b"BINFMT_TEST_BINARY\0".as_ptr() as *const c_char) as *const c_char;
    let argv0: *const c_char =
        getenv(b"BINFMT_TEST_ARGV0\0".as_ptr() as *const c_char) as *const c_char;
    let mut expect: [c_char; PATH_MAX + 32] = [0; PATH_MAX + 32];
    let mut buf: [c_char; PATH_MAX] = [0; PATH_MAX];
    let execfd: c_ulong;
    let mut stb: stat = core::mem::zeroed();
    let mut stfd: stat = core::mem::zeroed();
    let mut want: [*const c_char; 3] = [core::ptr::null(); 3];
    let mut base: *const c_char;
    let mut expect_len: size_t;
    let mut i: size_t;
    let fd: c_int;
    let have_stb: c_int;
    let have_stfd: c_int;
    let n: ssize_t;

    if binary.is_null() {
        fprintf(
            stderr,
            b"TRANSPARENT_FAIL: BINFMT_TEST_BINARY unset\n\0".as_ptr() as *const c_char,
        );
        return 1;
    }
    /* Distinct from the binary path, so a classic argv splice is caught. */
    want[0] = if !argv0.is_null() { argv0 } else { binary };
    want[1] = PAYLOAD_ARG1;
    want[2] = PAYLOAD_ARG2;

    /* The aux vector announces the transparent contract. */
    ok(
        (getauxval(AT_FLAGS) & AT_FLAGS_TRANSPARENT_INTERP) as c_int,
        b"AT_FLAGS lacks AT_FLAGS_TRANSPARENT_INTERP\0".as_ptr() as *const c_char,
    );

    /* AT_EXECFD refers to the very file that was executed. */
    execfd = getauxval(AT_EXECFD);
    ok((execfd > 2) as c_int, b"no AT_EXECFD\0".as_ptr() as *const c_char);
    have_stb = (stat(binary, &mut stb) == 0) as c_int;
    ok(have_stb, b"cannot stat the binary\0".as_ptr() as *const c_char);
    have_stfd = (fstat(execfd as c_int, &mut stfd) == 0) as c_int;
    ok(have_stfd, b"cannot fstat AT_EXECFD\0".as_ptr() as *const c_char);
    ok(
        (have_stb != 0 && have_stfd != 0 && stb.st_dev == stfd.st_dev && stb.st_ino == stfd.st_ino)
            as c_int,
        b"AT_EXECFD is not the binary\0".as_ptr() as *const c_char,
    );

    /* The exe link names the binary, not this interpreter. */
    ok(
        exe_is(binary),
        b"/proc/self/exe is not the binary\0".as_ptr() as *const c_char,
    );

    /* argv arrived unspliced. */
    ok(
        (argc == want.len() as c_int) as c_int,
        b"argv was rewritten\0".as_ptr() as *const c_char,
    );
    i = 0;
    while i < want.len() && i < argc as size_t {
        ok(
            (strcmp(*argv.add(i), want[i]) == 0) as c_int,
            b"argv was rewritten\0".as_ptr() as *const c_char,
        );
        i += 1;
    }

    /* And so did the kernel's copy of it: the same strings, NUL separated. */
    i = 0;
    expect_len = 0;
    while i < want.len() {
        let len: size_t = strlen(want[i]) + 1;

        if expect_len + len > expect.len() {
            ok(
                0,
                b"argv does not fit the expectation buffer\0".as_ptr() as *const c_char,
            );
            break;
        }
        memcpy(
            expect.as_mut_ptr().add(expect_len) as *mut c_void,
            want[i] as *const c_void,
            len,
        );
        expect_len += len;
        i += 1;
    }
    fd = open(
        b"/proc/self/cmdline\0".as_ptr() as *const c_char,
        O_RDONLY,
    );
    n = if fd >= 0 {
        read(fd, buf.as_mut_ptr() as *mut c_void, buf.len())
    } else {
        -1
    };
    if fd >= 0 {
        close(fd);
    }
    ok(
        (n == expect_len as ssize_t
            && memcmp(
                buf.as_ptr() as *const c_void,
                expect.as_ptr() as *const c_void,
                expect_len,
            ) == 0) as c_int,
        b"/proc/self/cmdline was rewritten\0".as_ptr() as *const c_char,
    );

    /* comm is the binary's basename. */
    base = strrchr(binary, '/' as c_int) as *const c_char;
    base = if !base.is_null() { base.add(1) } else { binary };
    ok(
        comm_is(base),
        b"comm is not the binary's basename\0".as_ptr() as *const c_char,
    );

    /* The binary is write-denied while it runs, like a direct exec. */
    ok(
        write_denied(binary),
        b"binary is writable while running\0".as_ptr() as *const c_char,
    );
    ok(
        write_denied(b"/proc/self/exe\0".as_ptr() as *const c_char),
        b"exe link is writable while running\0".as_ptr() as *const c_char,
    );

    if fail == 0 {
        printf(b"TRANSPARENT_OK\n\0".as_ptr() as *const c_char);
    }
    fail
}
