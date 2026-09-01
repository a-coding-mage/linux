/* SPDX-License-Identifier: GPL-2.0 */

use std::ffi::{CStr, CString};
use std::mem;
use std::os::raw::{c_char, c_int, c_long, c_uint, c_void};
use std::ptr;

const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;
const O_PATH: c_int = 0o10000000;
const R_OK: c_int = 4;
const X_OK: c_int = 1;
const DT_DIR: u8 = 4;
const DT_UNKNOWN: u8 = 0;
const S_IFMT: c_uint = 0o170000;
const S_IFLNK: c_uint = 0o120000;
const TEST_OK: c_int = 0;
const TEST_FAIL: c_int = -1;
const TEST_SKIP: c_int = -2;

#[repr(C)]
pub struct stat {
    pub st_mode: c_uint,
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
pub struct io {
    pub fd: c_int,
}

#[repr(C)]
pub struct test_case {
    pub name: *mut c_char,
    pub desc: *mut c_char,
    pub run_case: Option<unsafe extern "C" fn(*mut test_suite, c_int) -> c_int>,
    pub exclusive: bool,
}

#[repr(C)]
pub struct test_suite {
    pub desc: *mut c_char,
    pub test_cases: *mut test_case,
    pub priv_: *mut c_void,
}

unsafe extern "C" {
    static verbose: bool;

    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: usize) -> isize;
    fn lstat(path: *const c_char, buf: *mut stat) -> c_int;
    fn fstatat(dirfd: c_int, pathname: *const c_char, buf: *mut stat, flags: c_int) -> c_int;
    fn faccessat(dirfd: c_int, pathname: *const c_char, mode: c_int, flags: c_int) -> c_int;
    fn getpid() -> c_int;
    fn system(command: *const c_char) -> c_int;
    fn abort() -> !;
    fn free(ptr: *mut c_void);
    fn malloc(size: usize) -> *mut c_void;
    fn calloc(nmemb: usize, size: usize) -> *mut c_void;
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
    fn strrchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn scnprintf(s: *mut c_char, n: usize, format: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn scandirat(
        dirfd: c_int,
        dirp: *const c_char,
        namelist: *mut *mut *mut dirent,
        filter: Option<unsafe extern "C" fn(*const dirent) -> c_int>,
        compar: Option<unsafe extern "C" fn(*const *const dirent, *const *const dirent) -> c_int>,
    ) -> c_int;
    fn alphasort(a: *const *const dirent, b: *const *const dirent) -> c_int;

    fn get_argv_exec_path() -> *mut c_char;
    fn io__init(io: *mut io, fd: c_int, buf: *mut c_char, size: usize);
    fn io__getline(io: *mut io, line: *mut *mut c_char, line_len: *mut usize) -> isize;
    fn zalloc(size: usize) -> *mut c_void;
    fn zfree(ptr: *mut *mut c_void);
    fn pr_err(fmt: *const c_char, ...);
}

unsafe fn c_lit(bytes: &'static [u8]) -> *const c_char {
    bytes.as_ptr() as *const c_char
}

unsafe fn isspace(c: c_char) -> bool {
    matches!(c as u8, b' ' | b'\t' | b'\n' | b'\r' | 0x0b | 0x0c)
}

unsafe fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn s_isdir(mode: c_uint) -> bool {
    (mode & S_IFMT) == 0o040000
}

unsafe fn shell_tests__dir_fd() -> c_int {
    let mut st: stat = mem::zeroed();
    let mut path = [0 as c_char; PATH_MAX];
    let mut path2 = [0 as c_char; PATH_MAX];
    let devel_dirs: [*const c_char; 3] = [
        c_lit(b"./tools/perf/tests/shell\0"),
        c_lit(b"./tests/shell\0"),
        c_lit(b"./source/tests/shell\0"),
    ];

    for dir in devel_dirs {
        let fd = open(dir, O_PATH);

        if fd >= 0 {
            return fd;
        }
    }

    /* Use directory of executable */
    let mut len = readlink(c_lit(b"/proc/self/exe\0"), path2.as_mut_ptr(), path2.len() - 1);
    if len < 0 {
        return -1;
    }
    path2[len as usize] = 0;
    /* Follow another level of symlink if there */
    if lstat(path2.as_ptr(), &mut st) == 0 && (st.st_mode & S_IFMT) == S_IFLNK {
        scnprintf(path.as_mut_ptr(), path.len(), c_lit(b"%s\0"), path2.as_ptr());
        len = readlink(path.as_ptr(), path2.as_mut_ptr(), path2.len() - 1);
        if len < 0 {
            return -1;
        }
        path2[len as usize] = 0;
    }
    /* Get directory */
    let p = strrchr(path2.as_ptr(), b'/' as c_int);
    if !p.is_null() {
        *p = 0;
    }
    scnprintf(
        path.as_mut_ptr(),
        path.len(),
        c_lit(b"%s/tests/shell\0"),
        path2.as_ptr(),
    );
    let mut fd = open(path.as_ptr(), O_PATH);
    if fd >= 0 {
        return fd;
    }
    scnprintf(
        path.as_mut_ptr(),
        path.len(),
        c_lit(b"%s/source/tests/shell\0"),
        path2.as_ptr(),
    );
    fd = open(path.as_ptr(), O_PATH);
    if fd >= 0 {
        return fd;
    }

    /* Then installed path. */
    let exec_path = get_argv_exec_path();
    scnprintf(
        path.as_mut_ptr(),
        path.len(),
        c_lit(b"%s/tests/shell\0"),
        exec_path,
    );
    free(exec_path as *mut c_void);
    open(path.as_ptr(), O_PATH)
}

unsafe fn shell_test__description(dir_fd: c_int, name: *const c_char) -> *mut c_char {
    let mut io_data: io = mem::zeroed();
    let mut buf = [0 as c_char; 128];
    let mut line: *mut c_char = ptr::null_mut();
    let mut line_len: usize = 0;
    let mut desc: *mut c_char = ptr::null_mut();
    let spdx = c_lit(b"SPDX-License\0");

    io__init(
        &mut io_data,
        openat(dir_fd, name, O_RDONLY),
        buf.as_mut_ptr(),
        buf.len(),
    );
    if io_data.fd < 0 {
        return ptr::null_mut();
    }

    while io__getline(&mut io_data, &mut line, &mut line_len) > 0 {
        let mut p = line;

        /* Skip leading whitespace */
        while *p != 0 && isspace(*p) {
            p = p.add(1);
        }

        /* Must be a comment */
        if *p != b'#' as c_char {
            continue;
        }
        p = p.add(1);

        /* Skip shebang or SPDX lines */
        if *p == b'!' as c_char
            || (!strstr(p, spdx).is_null() && !strstr(p, c_lit(b"-Identifier:\0")).is_null())
        {
            continue;
        }

        /* Skip whitespace after # */
        while *p != 0 && isspace(*p) {
            p = p.add(1);
        }

        /* If we found non-empty text, this is the description! */
        if *p != 0 && *p != b'\n' as c_char {
            let mut end = p.add(strlen(p));

            while end > p && isspace(*end.sub(1)) {
                end = end.sub(1);
            }
            *end = 0;
            desc = strdup(p);
            break;
        }
    }
    free(line as *mut c_void);
    close(io_data.fd);
    desc
}

/* Is this full file path a shell script */
unsafe fn is_shell_script(dir_fd: c_int, path: *const c_char) -> bool {
    let ext = strrchr(path, b'.' as c_int);

    if ext.is_null() {
        return false;
    }
    if strcmp(ext, c_lit(b".sh\0")) == 0 {
        /* Has .sh extension */
        if faccessat(dir_fd, path, R_OK | X_OK, 0) == 0 {
            /* Is executable */
            return true;
        }
    }
    false
}

/* Is this file in this dir a shell script (for test purposes) */
unsafe fn is_test_script(dir_fd: c_int, name: *const c_char) -> bool {
    is_shell_script(dir_fd, name)
}

/* Duplicate a string and fall over and die if we run out of memory */
unsafe fn strdup_check(str_: *const c_char) -> *mut c_char {
    let newstr = strdup(str_);

    if newstr.is_null() {
        pr_err(c_lit(
            b"Out of memory while duplicating test script string\n\0",
        ));
        abort();
    }
    newstr
}

unsafe extern "C" fn shell_test__run(test: *mut test_suite, _subtest: c_int) -> c_int {
    let file = (*test).priv_ as *const c_char;
    let mut cmd: *mut c_char = ptr::null_mut();

    if asprintf(
        &mut cmd,
        c_lit(b"%s%s\0"),
        file,
        if verbose {
            c_lit(b" -v\0")
        } else {
            c_lit(b"\0")
        },
    ) < 0
    {
        return TEST_FAIL;
    }
    let err = system(cmd);
    free(cmd as *mut c_void);
    if err == 0 {
        return TEST_OK;
    }

    if wexitstatus(err) == 2 {
        TEST_SKIP
    } else {
        TEST_FAIL
    }
}

unsafe fn append_script(
    dir_fd: c_int,
    name: *const c_char,
    desc: *mut c_char,
    result: *mut *mut *mut test_suite,
    result_sz: *mut usize,
) {
    let mut filename = [0 as c_char; PATH_MAX];
    let mut link = [0 as c_char; 128];

    snprintf(
        link.as_mut_ptr(),
        link.len(),
        c_lit(b"/proc/%d/fd/%d\0"),
        getpid(),
        dir_fd,
    );
    let mut len = readlink(link.as_ptr(), filename.as_mut_ptr(), filename.len() - 1);
    if len < 0 || len as usize > filename.len() - strlen(name) - 2 {
        pr_err(
            c_lit(b"Failed to readlink %s or path too long\0"),
            link.as_ptr(),
        );
        return;
    }
    filename[len as usize] = b'/' as c_char;
    len += 1;
    strcpy(filename.as_mut_ptr().add(len as usize), name);

    let tests = calloc(2, mem::size_of::<test_case>()) as *mut test_case;
    if tests.is_null() {
        pr_err(c_lit(
            b"Out of memory while building script test suite list\n\0",
        ));
        return;
    }
    (*tests.add(0)).name = strdup_check(name);
    let exclusive = strstr(desc, c_lit(b" (exclusive)\0"));
    if !exclusive.is_null() {
        (*tests.add(0)).exclusive = true;
        *exclusive = 0;
    }
    (*tests.add(0)).desc = strdup_check(desc);
    (*tests.add(0)).run_case = Some(shell_test__run);
    let test_suite_ptr = zalloc(mem::size_of::<test_suite>()) as *mut test_suite;
    if test_suite_ptr.is_null() {
        pr_err(c_lit(
            b"Out of memory while building script test suite list\n\0",
        ));
        free(tests as *mut c_void);
        return;
    }
    (*test_suite_ptr).desc = desc;
    (*test_suite_ptr).test_cases = tests;
    (*test_suite_ptr).priv_ = strdup_check(filename.as_ptr()) as *mut c_void;
    /* Realloc is good enough, though we could realloc by chunks, not that
     * anyone will ever measure performance here */
    let result_tmp = realloc(
        *result as *mut c_void,
        (*result_sz + 1) * mem::size_of::<*mut test_suite>(),
    ) as *mut *mut test_suite;
    if result_tmp.is_null() {
        pr_err(c_lit(
            b"Out of memory while building script test suite list\n\0",
        ));
        free(tests as *mut c_void);
        free(test_suite_ptr as *mut c_void);
        return;
    }
    /* Add file to end and NULL terminate the struct array */
    *result = result_tmp;
    *(*result).add(*result_sz) = test_suite_ptr;
    *result_sz += 1;
}

unsafe fn append_scripts_in_dir(
    dir_fd: c_int,
    result: *mut *mut *mut test_suite,
    result_sz: *mut usize,
) {
    let mut entlist: *mut *mut dirent = ptr::null_mut();

    /* List files, sorted by alpha */
    let n_dirs = scandirat(
        dir_fd,
        c_lit(b".\0"),
        &mut entlist,
        None,
        Some(alphasort),
    );
    if n_dirs == -1 {
        return;
    }
    let mut i = 0;
    while i < n_dirs {
        let ent = *entlist.add(i as usize);
        if ent.is_null() {
            break;
        }
        let name = (*ent).d_name.as_ptr();

        if (*ent).d_name[0] == b'.' as c_char {
            i += 1;
            continue; /* Skip hidden files */
        }
        if is_test_script(dir_fd, name) {
            /* It's a test */
            let desc = shell_test__description(dir_fd, name);

            if !desc.is_null() {
                /* It has a desc line - valid script */
                append_script(dir_fd, name, desc, result, result_sz);
            }
            i += 1;
            continue;
        }
        if (*ent).d_type != DT_DIR {
            let mut st: stat = mem::zeroed();

            if (*ent).d_type != DT_UNKNOWN {
                i += 1;
                continue;
            }
            fstatat(dir_fd, name, &mut st, 0);
            if !s_isdir(st.st_mode) {
                i += 1;
                continue;
            }
        }
        if strncmp(name, c_lit(b"base_\0"), 5) == 0 {
            i += 1;
            continue; /* Skip scripts that have a separate driver. */
        }
        let fd = openat(dir_fd, name, O_PATH);
        append_scripts_in_dir(fd, result, result_sz);
        close(fd);
        i += 1;
    }
    i = 0;
    while i < n_dirs {
        /* Clean up */
        zfree(entlist.add(i as usize) as *mut *mut c_void);
        i += 1;
    }
    free(entlist as *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn create_script_test_suites() -> *mut *mut test_suite {
    let mut result: *mut *mut test_suite = ptr::null_mut();
    let mut result_sz: usize = 0;
    let dir_fd = shell_tests__dir_fd(); /* Walk  dir */

    /*
     * Append scripts if fd is good, otherwise return a NULL terminated zero
     * length array.
     */
    if dir_fd >= 0 {
        append_scripts_in_dir(dir_fd, &mut result, &mut result_sz);
    }

    let result_tmp = realloc(
        result as *mut c_void,
        (result_sz + 1) * mem::size_of::<*mut test_suite>(),
    ) as *mut *mut test_suite;
    if result_tmp.is_null() {
        pr_err(c_lit(
            b"Out of memory while building script test suite list\n\0",
        ));
        abort();
    }
    /* NULL terminate the test suite array. */
    result = result_tmp;
    *result.add(result_sz) = ptr::null_mut();
    if dir_fd >= 0 {
        close(dir_fd);
    }
    result
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
