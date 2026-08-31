// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE before including libc and project headers.

use core::ffi::{c_char, c_int, c_void};

pub type size_t = usize;

pub const PATH_MAX: usize = 4096;
pub const ENOENT: c_int = 2;
pub const EACCES: c_int = 13;

#[repr(C)]
pub struct DIR {
    _private: [u8; 0],
}

#[repr(C)]
pub struct dirent {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn asprintf(strp: *mut *mut c_char, fmt: *const c_char, ...) -> c_int;
    fn free(ptr: *mut c_void);
    fn opendir(name: *const c_char) -> *mut DIR;
    fn scandir(
        dirp: *const c_char,
        namelist: *mut *mut *mut dirent,
        filter: Option<unsafe extern "C" fn(*const dirent) -> c_int>,
        compar: Option<unsafe extern "C" fn(*const *const dirent, *const *const dirent) -> c_int>,
    ) -> c_int;
    fn alphasort(a: *const *const dirent, b: *const *const dirent) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;

    fn tracefs__mount() -> *const c_char;
    fn debugfs__mount() -> *const c_char;
    fn tracefs__configured() -> bool;
    fn debugfs__configured() -> bool;
    fn str_error_r(errnum: c_int, buf: *mut c_char, buflen: size_t) -> *const c_char;
}

static mut tracing_path: [c_char; PATH_MAX] = {
    let mut data = [0 as c_char; PATH_MAX];
    data[0] = b'/' as c_char;
    data[1] = b's' as c_char;
    data[2] = b'y' as c_char;
    data[3] = b's' as c_char;
    data[4] = b'/' as c_char;
    data[5] = b'k' as c_char;
    data[6] = b'e' as c_char;
    data[7] = b'r' as c_char;
    data[8] = b'n' as c_char;
    data[9] = b'e' as c_char;
    data[10] = b'l' as c_char;
    data[11] = b'/' as c_char;
    data[12] = b't' as c_char;
    data[13] = b'r' as c_char;
    data[14] = b'a' as c_char;
    data[15] = b'c' as c_char;
    data[16] = b'i' as c_char;
    data[17] = b'n' as c_char;
    data[18] = b'g' as c_char;
    data
};

unsafe fn __tracing_path_set(tracing: *const c_char, mountpoint: *const c_char) {
    unsafe {
        snprintf(
            core::ptr::addr_of_mut!(tracing_path) as *mut c_char,
            PATH_MAX,
            c"%s/%s".as_ptr(),
            mountpoint,
            tracing,
        );
    }
}

unsafe fn tracing_path_tracefs_mount() -> *const c_char {
    let mnt: *const c_char;

    unsafe {
        mnt = tracefs__mount();
        if mnt.is_null() {
            return core::ptr::null();
        }

        __tracing_path_set(c"".as_ptr(), mnt);

        core::ptr::addr_of!(tracing_path) as *const c_char
    }
}

unsafe fn tracing_path_debugfs_mount() -> *const c_char {
    let mnt: *const c_char;

    unsafe {
        mnt = debugfs__mount();
        if mnt.is_null() {
            return core::ptr::null();
        }

        __tracing_path_set(c"tracing/".as_ptr(), mnt);

        core::ptr::addr_of!(tracing_path) as *const c_char
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracing_path_mount() -> *const c_char {
    let mut mnt: *const c_char;

    unsafe {
        mnt = tracing_path_tracefs_mount();
        if !mnt.is_null() {
            return mnt;
        }

        mnt = tracing_path_debugfs_mount();

        mnt
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracing_path_set(mntpt: *const c_char) {
    unsafe {
        __tracing_path_set(c"tracing/".as_ptr(), mntpt);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_tracing_file(name: *const c_char) -> *mut c_char {
    let mut file: *mut c_char = core::ptr::null_mut();

    unsafe {
        if asprintf(&mut file, c"%s%s".as_ptr(), tracing_path_mount(), name) < 0 {
            return core::ptr::null_mut();
        }

        file
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn put_tracing_file(file: *mut c_char) {
    unsafe {
        free(file as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_events_file(name: *const c_char) -> *mut c_char {
    let mut file: *mut c_char = core::ptr::null_mut();

    unsafe {
        if asprintf(
            &mut file,
            c"%s/events/%s".as_ptr(),
            tracing_path_mount(),
            name,
        ) < 0
        {
            return core::ptr::null_mut();
        }

        file
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn put_events_file(file: *mut c_char) {
    unsafe {
        free(file as *mut c_void);
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracing_events__opendir() -> *mut DIR {
    let mut dir: *mut DIR = core::ptr::null_mut();
    let path: *mut c_char = unsafe { get_tracing_file(c"events".as_ptr()) };

    if !path.is_null() {
        unsafe {
            dir = opendir(path);
            put_events_file(path);
        }
    }

    dir
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracing_events__scandir_alphasort(
    namelist: *mut *mut *mut dirent,
) -> c_int {
    let path: *mut c_char = unsafe { get_tracing_file(c"events".as_ptr()) };
    let ret: c_int;

    if path.is_null() {
        unsafe {
            *namelist = core::ptr::null_mut();
        }
        return 0;
    }

    unsafe {
        ret = scandir(path, namelist, None, Some(alphasort));
        put_events_file(path);
    }

    ret
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn tracing_path__strerror_open_tp(
    err: c_int,
    buf: *mut c_char,
    size: size_t,
    sys: *const c_char,
    name: *const c_char,
) -> c_int {
    let mut sbuf: [c_char; 128] = [0; 128];
    let mut filename: [c_char; PATH_MAX] = [0; PATH_MAX];
    let name_or_star = if !name.is_null() { name } else { c"*".as_ptr() };

    unsafe {
        snprintf(
            filename.as_mut_ptr(),
            PATH_MAX,
            c"%s/%s".as_ptr(),
            sys,
            name_or_star,
        );

        match err {
            ENOENT => {
                /*
                 * We will get here if we can't find the tracepoint, but one of
                 * debugfs or tracefs is configured, which means you probably
                 * want some tracepoint which wasn't compiled in your kernel.
                 * - jirka
                 */
                if debugfs__configured() || tracefs__configured() {
                    /* sdt markers */
                    if strncmp(filename.as_ptr(), c"sdt_".as_ptr(), 4) == 0 {
                        snprintf(
                            buf,
                            size,
                            c"Error:\tFile %s/events/%s not found.\nHint:\tSDT event cannot be directly recorded on.\n\tPlease first use 'perf probe %s:%s' before recording it.\n".as_ptr(),
                            core::ptr::addr_of!(tracing_path) as *const c_char,
                            filename.as_ptr(),
                            sys,
                            name,
                        );
                    } else {
                        snprintf(
                            buf,
                            size,
                            c"Error:\tFile %s/events/%s not found.\nHint:\tPerhaps this kernel misses some CONFIG_ setting to enable this feature?.\n".as_ptr(),
                            core::ptr::addr_of!(tracing_path) as *const c_char,
                            filename.as_ptr(),
                        );
                    }
                } else {
                    snprintf(
                        buf,
                        size,
                        c"%s".as_ptr(),
                        c"Error:\tUnable to find debugfs/tracefs\nHint:\tWas your kernel compiled with debugfs/tracefs support?\nHint:\tIs the debugfs/tracefs filesystem mounted?\nHint:\tTry 'sudo mount -t debugfs nodev /sys/kernel/debug'".as_ptr(),
                    );
                }
            }
            EACCES => {
                snprintf(
                    buf,
                    size,
                    c"Error:\tNo permissions to read %s/events/%s\nHint:\tTry 'sudo mount -o remount,mode=755 %s'\n".as_ptr(),
                    core::ptr::addr_of!(tracing_path) as *const c_char,
                    filename.as_ptr(),
                    tracing_path_mount(),
                );
            }
            _ => {
                snprintf(
                    buf,
                    size,
                    c"%s".as_ptr(),
                    str_error_r(err, sbuf.as_mut_ptr(), sbuf.len()),
                );
            }
        }
    }

    0
}
