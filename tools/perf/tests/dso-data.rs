// SPDX-License-Identifier: GPL-2.0
//
// Translated from perf/tests/dso-data.c. C include dependencies are represented
// by the external declarations below.

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem;
use core::ptr;

type off_t = i64;
type size_t = usize;
type ssize_t = isize;
type u8 = u8;

const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;
const RLIMIT_NOFILE: c_int = 7;
const DSO__DATA_CACHE_SIZE: c_int = 4096;
const TEST_FILE_SIZE: c_int = DSO__DATA_CACHE_SIZE * 20;
const BUFSIZE: usize = 10;

#[repr(C)]
pub struct dso {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dso_data {
    pub fd: c_int,
}

#[repr(C)]
pub struct dsos {
    pub cnt: c_uint,
    pub dsos: *mut *mut dso,
}

#[repr(C)]
pub struct machine {
    pub dsos: dsos,
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct test_case {
    pub name: *const c_char,
    pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
}

#[repr(C)]
pub struct rlimit {
    pub rlim_cur: u64,
    pub rlim_max: u64,
}

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    pub d_ino: u64,
    pub d_off: i64,
    pub d_reclen: u16,
    pub d_type: u8,
    pub d_name: [c_char; 256],
}

#[repr(C)]
struct test_data_offset {
    offset: off_t,
    data: [u8; 10],
    size: c_int,
}

unsafe extern "C" {
    fn mkstemp(template: *mut c_char) -> c_int;
    fn perror(s: *const c_char);
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn unlink(pathname: *const c_char) -> c_int;
    fn opendir(name: *const c_char) -> *mut DIR;
    fn readdir(dirp: *mut DIR) -> *mut dirent;
    fn closedir(dirp: *mut DIR) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn getrlimit(resource: c_int, rlim: *mut rlimit) -> c_int;
    fn setrlimit(resource: c_int, rlim: *const rlimit) -> c_int;

    fn scnprintf(buf: *mut c_char, size: size_t, fmt: *const c_char, ...) -> c_int;
    fn procfs__mountpoint() -> *const c_char;
    fn pr_debug(fmt: *const c_char, ...);

    fn dso__data_get_fd(dso: *mut dso, machine: *mut machine, fd: *mut c_int) -> c_int;
    fn dso__data_put_fd(dso: *mut dso);
    fn dso__data_close(dso: *mut dso);
    fn dso__name(dso: *mut dso) -> *const c_char;
    fn dsos__exit(dsos: *mut dsos);
    fn dsos__init(dsos: *mut dsos);
    fn dso__new(name: *const c_char) -> *mut dso;
    fn dsos__add(dsos: *mut dsos, dso: *mut dso) -> c_int;
    fn dso__data_read_offset(
        dso: *mut dso,
        machine: *mut machine,
        offset: off_t,
        buf: *mut u8,
        size: size_t,
    ) -> ssize_t;
    fn dso__put(dso: *mut dso);
    fn reset_fd_limit();
    fn dso__data(dso: *mut dso) -> *mut dso_data;
}

unsafe fn test_assert_val(_msg: *const c_char, cond: bool) -> c_int {
    if !cond {
        return -1;
    }
    0
}

unsafe fn test_file(size: c_int) -> *mut c_char {
    const TEMPL: &[u8] = b"/tmp/perf-test-XXXXXX\0";
    static mut BUF_TEMPL: [c_char; TEMPL.len()] = [0; TEMPL.len()];

    let templ = ptr::addr_of_mut!(BUF_TEMPL).cast::<c_char>();
    let mut fd: c_int;
    let mut i: c_int;
    let buf: *mut u8;

    strcpy(templ, TEMPL.as_ptr().cast());

    fd = mkstemp(templ);
    if fd < 0 {
        perror(c"mkstemp failed".as_ptr());
        return ptr::null_mut();
    }

    buf = malloc(size as size_t).cast::<u8>();
    if buf.is_null() {
        close(fd);
        return ptr::null_mut();
    }

    i = 0;
    while i < size {
        *buf.add(i as usize) = (i % 10) as u8;
        i += 1;
    }

    let mut ret = templ;
    if size as ssize_t != write(fd, buf.cast::<c_void>(), size as size_t) {
        ret = ptr::null_mut();
    }

    free(buf.cast::<c_void>());
    close(fd);
    ret
}

static mut OFFSETS: [test_data_offset; 7] = [
    /* Fill first cache page. */
    test_data_offset {
        offset: 10,
        data: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        size: 10,
    },
    /* Read first cache page. */
    test_data_offset {
        offset: 10,
        data: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        size: 10,
    },
    /* Fill cache boundary pages. */
    test_data_offset {
        offset: (DSO__DATA_CACHE_SIZE - DSO__DATA_CACHE_SIZE % 10) as off_t,
        data: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        size: 10,
    },
    /* Read cache boundary pages. */
    test_data_offset {
        offset: (DSO__DATA_CACHE_SIZE - DSO__DATA_CACHE_SIZE % 10) as off_t,
        data: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        size: 10,
    },
    /* Fill final cache page. */
    test_data_offset {
        offset: (TEST_FILE_SIZE - 10) as off_t,
        data: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        size: 10,
    },
    /* Read final cache page. */
    test_data_offset {
        offset: (TEST_FILE_SIZE - 10) as off_t,
        data: [0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        size: 10,
    },
    /* Read final cache page. */
    test_data_offset {
        offset: (TEST_FILE_SIZE - 3) as off_t,
        data: [7, 8, 9, 0, 0, 0, 0, 0, 0, 0],
        size: 3,
    },
];

/* move it from util/dso.c for compatibility */
unsafe fn dso__data_fd(dso: *mut dso, machine: *mut machine) -> c_int {
    let mut fd: c_int = -1;

    if dso__data_get_fd(dso, machine, &mut fd) != 0 {
        dso__data_put_fd(dso);
    }

    fd
}

unsafe fn dsos__delete(dsos: *mut dsos) {
    let mut i: c_uint = 0;
    while i < (*dsos).cnt {
        let dso = *(*dsos).dsos.add(i as usize);

        dso__data_close(dso);
        unlink(dso__name(dso));
        i += 1;
    }
    dsos__exit(dsos);
}

unsafe extern "C" fn test__dso_data(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut machine: machine = mem::zeroed();
    let mut dso: *mut dso;
    let file = test_file(TEST_FILE_SIZE);
    let mut i: size_t;

    if test_assert_val(c"No test file".as_ptr(), !file.is_null()) != 0 {
        return -1;
    }

    memset((&mut machine as *mut machine).cast::<c_void>(), 0, mem::size_of::<machine>());
    dsos__init(&mut machine.dsos);

    dso = dso__new(file);
    if test_assert_val(c"Failed to add dso".as_ptr(), dsos__add(&mut machine.dsos, dso) == 0) != 0 {
        return -1;
    }
    if test_assert_val(
        c"Failed to access to dso".as_ptr(),
        dso__data_fd(dso, &mut machine) >= 0,
    ) != 0 {
        return -1;
    }

    /* Basic 10 bytes tests. */
    i = 0;
    while i < OFFSETS.len() {
        let data = ptr::addr_of_mut!(OFFSETS).cast::<test_data_offset>().add(i);
        let size: ssize_t;
        let mut buf: [u8; 10] = [0; 10];

        memset(buf.as_mut_ptr().cast::<c_void>(), 0, 10);
        size = dso__data_read_offset(dso, &mut machine, (*data).offset, buf.as_mut_ptr(), 10);

        if test_assert_val(c"Wrong size".as_ptr(), size == (*data).size as ssize_t) != 0 {
            return -1;
        }
        if test_assert_val(
            c"Wrong data".as_ptr(),
            memcmp(buf.as_ptr().cast::<c_void>(), (*data).data.as_ptr().cast::<c_void>(), 10) == 0,
        ) != 0 {
            return -1;
        }
        i += 1;
    }

    /* Read cross multiple cache pages. */
    {
        let mut size: ssize_t;
        let mut c: c_int;
        let buf: *mut u8;

        buf = malloc(TEST_FILE_SIZE as size_t).cast::<u8>();
        if test_assert_val(c"ENOMEM\n".as_ptr(), !buf.is_null()) != 0 {
            return -1;
        }

        /* First iteration to fill caches, second one to read them. */
        c = 0;
        while c < 2 {
            memset(buf.cast::<c_void>(), 0, TEST_FILE_SIZE as size_t);
            size = dso__data_read_offset(
                dso,
                &mut machine,
                10,
                buf,
                TEST_FILE_SIZE as size_t,
            );

            if test_assert_val(c"Wrong size".as_ptr(), size == (TEST_FILE_SIZE - 10) as ssize_t) != 0 {
                return -1;
            }

            i = 0;
            while i < size as size_t {
                if test_assert_val(c"Wrong data".as_ptr(), *buf.add(i) == (i % 10) as u8) != 0 {
                    return -1;
                }
                i += 1;
            }
            c += 1;
        }

        free(buf.cast::<c_void>());
    }

    dso__put(dso);
    dsos__delete(&mut machine.dsos);
    unlink(file);
    0
}

unsafe fn open_files_cnt() -> c_long {
    let mut path: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut dent: *mut dirent;
    let dir: *mut DIR;
    let mut nr: c_long = 0;

    scnprintf(
        path.as_mut_ptr(),
        PATH_MAX,
        c"%s/self/fd".as_ptr(),
        procfs__mountpoint(),
    );
    pr_debug(c"fd path: %s\n".as_ptr(), path.as_ptr());

    dir = opendir(path.as_ptr());
    if test_assert_val(c"failed to open fd directory".as_ptr(), !dir.is_null()) != 0 {
        return -1;
    }

    loop {
        dent = readdir(dir);
        if dent.is_null() {
            break;
        }
        if strcmp((*dent).d_name.as_ptr(), c".".as_ptr()) == 0
            || strcmp((*dent).d_name.as_ptr(), c"..".as_ptr()) == 0
        {
            continue;
        }

        nr += 1;
    }

    closedir(dir);
    nr - 1
}

unsafe fn dsos__create(cnt: c_int, size: c_int, dsos: *mut dsos) -> c_int {
    let mut i: c_int;

    dsos__init(dsos);

    i = 0;
    while i < cnt {
        let dso: *mut dso;
        let file = test_file(size);

        if test_assert_val(c"failed to get dso file".as_ptr(), !file.is_null()) != 0 {
            return -1;
        }
        dso = dso__new(file);
        if test_assert_val(c"failed to get dso".as_ptr(), !dso.is_null()) != 0 {
            return -1;
        }
        if test_assert_val(c"failed to add dso".as_ptr(), dsos__add(dsos, dso) == 0) != 0 {
            return -1;
        }
        dso__put(dso);
        i += 1;
    }

    0
}

unsafe fn set_fd_limit(n: c_int) -> c_int {
    let mut rlim: rlimit = mem::zeroed();

    if getrlimit(RLIMIT_NOFILE, &mut rlim) != 0 {
        return -1;
    }

    pr_debug(c"file limit %ld, new %d\n".as_ptr(), rlim.rlim_cur as c_long, n);

    rlim.rlim_cur = n as u64;
    setrlimit(RLIMIT_NOFILE, &rlim)
}

unsafe extern "C" fn test__dso_data_cache(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut machine: machine = mem::zeroed();
    let mut nr_end: c_long;
    let nr = open_files_cnt();
    let mut dso_cnt: c_int;
    let limit: c_int;
    let mut i: c_int;
    let mut fd: c_int;

    /* Rest the internal dso open counter limit. */
    reset_fd_limit();

    memset((&mut machine as *mut machine).cast::<c_void>(), 0, mem::size_of::<machine>());

    /* set as system limit */
    limit = (nr * 4) as c_int;
    if test_assert_val(c"failed to set file limit".as_ptr(), set_fd_limit(limit) == 0) != 0 {
        return -1;
    }

    /* and this is now our dso open FDs limit */
    dso_cnt = limit / 2;
    if test_assert_val(
        c"failed to create dsos\n".as_ptr(),
        dsos__create(dso_cnt, TEST_FILE_SIZE, &mut machine.dsos) == 0,
    ) != 0 {
        return -1;
    }

    i = 0;
    while i < dso_cnt - 1 {
        let dso = *machine.dsos.dsos.add(i as usize);

        /*
         * Open dsos via dso__data_fd(), it opens the data
         * file and keep it open (unless open file limit).
         */
        fd = dso__data_fd(dso, &mut machine);
        if test_assert_val(c"failed to get fd".as_ptr(), fd > 0) != 0 {
            return -1;
        }

        if i % 2 != 0 {
            let mut buf: [u8; BUFSIZE] = [0; BUFSIZE];
            let n: ssize_t;

            n = dso__data_read_offset(dso, &mut machine, 0, buf.as_mut_ptr(), BUFSIZE);
            if test_assert_val(c"failed to read dso".as_ptr(), n == BUFSIZE as ssize_t) != 0 {
                return -1;
            }
        }
        i += 1;
    }

    /* verify the first one is already open */
    if test_assert_val(
        c"dsos[0] is not open".as_ptr(),
        (*dso__data(*machine.dsos.dsos.add(0))).fd != -1,
    ) != 0 {
        return -1;
    }

    /* open +1 dso to reach the allowed limit */
    fd = dso__data_fd(*machine.dsos.dsos.add(i as usize), &mut machine);
    if test_assert_val(c"failed to get fd".as_ptr(), fd > 0) != 0 {
        return -1;
    }

    /* should force the first one to be closed */
    if test_assert_val(
        c"failed to close dsos[0]".as_ptr(),
        (*dso__data(*machine.dsos.dsos.add(0))).fd == -1,
    ) != 0 {
        return -1;
    }

    /* cleanup everything */
    dsos__delete(&mut machine.dsos);

    /* Make sure we did not leak any file descriptor. */
    nr_end = open_files_cnt();
    pr_debug(c"nr start %ld, nr stop %ld\n".as_ptr(), nr, nr_end);
    if test_assert_val(c"failed leaking files".as_ptr(), nr == nr_end) != 0 {
        return -1;
    }
    0
}

unsafe fn new_limit(count: c_int) -> c_long {
    let fd = open(c"/dev/null".as_ptr(), O_RDONLY);
    let mut ret = fd as c_long;
    if count > 0 {
        ret = new_limit(count - 1);
    }
    close(fd);
    ret
}

unsafe extern "C" fn test__dso_data_reopen(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut machine: machine = mem::zeroed();
    let mut nr_end: c_long;
    let nr = open_files_cnt();
    let lim = new_limit(3);
    let mut fd: c_int;
    let fd_extra: c_int;

    /* Rest the internal dso open counter limit. */
    reset_fd_limit();

    memset((&mut machine as *mut machine).cast::<c_void>(), 0, mem::size_of::<machine>());

    /*
     * Test scenario:
     * - create 3 dso objects
     * - set process file descriptor limit to current
     *   files count + 3
     * - test that the first dso gets closed when we
     *   reach the files count limit
     */

    /* Make sure we are able to open 3 fds anyway */
    if test_assert_val(c"failed to set file limit".as_ptr(), set_fd_limit(lim as c_int) == 0) != 0 {
        return -1;
    }

    if test_assert_val(
        c"failed to create dsos\n".as_ptr(),
        dsos__create(3, TEST_FILE_SIZE, &mut machine.dsos) == 0,
    ) != 0 {
        return -1;
    }

    /* open dso_0 */
    fd = dso__data_fd(*machine.dsos.dsos.add(0), &mut machine);
    if test_assert_val(c"failed to get fd".as_ptr(), fd > 0) != 0 {
        return -1;
    }

    /* open dso_1 */
    fd = dso__data_fd(*machine.dsos.dsos.add(1), &mut machine);
    if test_assert_val(c"failed to get fd".as_ptr(), fd > 0) != 0 {
        return -1;
    }

    /*
     * open extra file descriptor and we just
     * reached the files count limit
     */
    fd_extra = open(c"/dev/null".as_ptr(), O_RDONLY);
    if test_assert_val(c"failed to open extra fd".as_ptr(), fd_extra > 0) != 0 {
        return -1;
    }

    /* open dso_2 */
    fd = dso__data_fd(*machine.dsos.dsos.add(2), &mut machine);
    if test_assert_val(c"failed to get fd".as_ptr(), fd > 0) != 0 {
        return -1;
    }

    /*
     * dso_0 should get closed, because we reached
     * the file descriptor limit
     */
    if test_assert_val(
        c"failed to close dso_0".as_ptr(),
        (*dso__data(*machine.dsos.dsos.add(0))).fd == -1,
    ) != 0 {
        return -1;
    }

    /* open dso_0 */
    fd = dso__data_fd(*machine.dsos.dsos.add(0), &mut machine);
    if test_assert_val(c"failed to get fd".as_ptr(), fd > 0) != 0 {
        return -1;
    }

    /*
     * dso_1 should get closed, because we reached
     * the file descriptor limit
     */
    if test_assert_val(
        c"failed to close dso_1".as_ptr(),
        (*dso__data(*machine.dsos.dsos.add(1))).fd == -1,
    ) != 0 {
        return -1;
    }

    /* cleanup everything */
    close(fd_extra);
    dsos__delete(&mut machine.dsos);

    /* Make sure we did not leak any file descriptor. */
    nr_end = open_files_cnt();
    pr_debug(c"nr start %ld, nr stop %ld\n".as_ptr(), nr, nr_end);
    if test_assert_val(c"failed leaking files".as_ptr(), nr == nr_end) != 0 {
        return -1;
    }
    0
}

static mut TESTS__DSO_DATA: [test_case; 4] = [
    test_case {
        name: c"read".as_ptr(),
        run_case: Some(test__dso_data),
    },
    test_case {
        name: c"cache".as_ptr(),
        run_case: Some(test__dso_data_cache),
    },
    test_case {
        name: c"reopen".as_ptr(),
        run_case: Some(test__dso_data_reopen),
    },
    test_case {
        name: ptr::null(),
        run_case: None,
    },
];

#[repr(C)]
pub struct test_suite_with_cases {
    pub desc: *const c_char,
    pub test_cases: *mut test_case,
}

#[unsafe(no_mangle)]
pub static mut suite__dso_data: test_suite_with_cases = test_suite_with_cases {
    desc: c"DSO data tests".as_ptr(),
    test_cases: ptr::addr_of_mut!(TESTS__DSO_DATA).cast::<test_case>(),
};
