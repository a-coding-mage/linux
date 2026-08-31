// SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause)
/* Copyright (c) 2015-2017 Daniel Borkmann */
/* Copyright (c) 2018 Netronome Systems, Inc. */

// C dependencies:
// errno.h, limits.h, signal.h, stdio.h, string.h, unistd.h,
// linux/magic.h, fcntl.h, sys/vfs.h, and "main.h".

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

const TRACEFS_MAGIC: c_ulong = 0x74726163;

const PATH_MAX: usize = libc::PATH_MAX as usize;
const ENOENT: c_int = libc::ENOENT;
const SIGHUP: c_int = libc::SIGHUP;
const SIGINT: c_int = libc::SIGINT;
const SIGTERM: c_int = libc::SIGTERM;

type size_t = usize;
type ssize_t = isize;

pub static mut trace_pipe_fd: *mut libc::FILE = core::ptr::null_mut();
pub static mut buff: *mut c_char = core::ptr::null_mut();

unsafe extern "C" {
    static mut errno: c_int;
    static mut block_mount: bool;
    static mut json_output: bool;
    static mut json_wtr: *mut c_void;

    fn statfs(path: *const c_char, buf: *mut libc::statfs) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcat(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut libc::FILE;
    fn fclose(stream: *mut libc::FILE) -> c_int;
    fn fscanf(stream: *mut libc::FILE, format: *const c_char, ...) -> c_int;
    fn getline(
        lineptr: *mut *mut c_char,
        n: *mut size_t,
        stream: *mut libc::FILE,
    ) -> ssize_t;
    fn printf(format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn free(ptr: *mut c_void);
    fn exit(status: c_int) -> !;
    fn sigaction(signum: c_int, act: *const libc::sigaction, oldact: *mut libc::sigaction) -> c_int;

    fn p_err(format: *const c_char, ...);
    fn p_info(format: *const c_char, ...);
    fn mount_tracefs(mnt: *const c_char) -> c_int;
    fn jsonw_start_array(wtr: *mut c_void);
    fn jsonw_end_array(wtr: *mut c_void);
    fn jsonw_destroy(wtr: *mut *mut c_void);
    fn jsonw_string(wtr: *mut c_void, str_: *const c_char);
}

unsafe fn validate_tracefs_mnt(mnt: *const c_char, magic: c_ulong) -> c_int {
    let mut st_fs: libc::statfs = core::mem::zeroed();

    if statfs(mnt, &mut st_fs) < 0 {
        return -ENOENT;
    }
    if st_fs.f_type as c_ulong != magic {
        return -ENOENT;
    }

    0
}

unsafe fn find_tracefs_mnt_single(
    magic: c_ulong,
    mnt: *mut c_char,
    mntpt: *const c_char,
) -> bool {
    let src_len: size_t;

    if validate_tracefs_mnt(mntpt, magic) != 0 {
        return false;
    }

    src_len = strlen(mntpt);
    if src_len + 1 >= PATH_MAX {
        p_err(c"tracefs mount point name too long".as_ptr());
        return false;
    }

    strcpy(mnt, mntpt);
    true
}

unsafe fn get_tracefs_pipe(mnt: *mut c_char) -> bool {
    static KNOWN_MNTS: [*const c_char; 2] = [
        c"/sys/kernel/tracing".as_ptr(),
        c"/sys/kernel/debug/tracing".as_ptr(),
    ];
    let pipe_name: *const c_char = c"/trace_pipe".as_ptr();
    let fstype: *const c_char = c"tracefs".as_ptr();
    let mut type_: [c_char; 100] = [0; 100];
    let mut format: [c_char; 32] = [0; 32];
    let mut ptr: *const *const c_char;
    let mut found: bool = false;
    let mut fp: *mut libc::FILE;

    ptr = KNOWN_MNTS.as_ptr();
    while ptr < unsafe { KNOWN_MNTS.as_ptr().add(KNOWN_MNTS.len()) } {
        if find_tracefs_mnt_single(TRACEFS_MAGIC, mnt, *ptr) {
            return exit_found(mnt, pipe_name);
        }
        ptr = ptr.add(1);
    }

    fp = fopen(c"/proc/mounts".as_ptr(), c"r".as_ptr());
    if fp.is_null() {
        return false;
    }

    /* Allow room for NULL terminating byte and pipe file name */
    snprintf(
        format.as_mut_ptr(),
        format.len(),
        c"%%*s %%%zus %%99s %%*s %%*d %%*d\n".as_ptr(),
        PATH_MAX - strlen(pipe_name) - 1,
    );
    while fscanf(fp, format.as_ptr(), mnt, type_.as_mut_ptr()) == 2 {
        if strcmp(type_.as_ptr(), fstype) == 0 {
            found = true;
            break;
        }
    }
    fclose(fp);

    /* The string from fscanf() might be truncated, check mnt is valid */
    if found && validate_tracefs_mnt(mnt, TRACEFS_MAGIC) != 0 {
        return exit_found(mnt, pipe_name);
    }

    if block_mount {
        return false;
    }

    p_info(c"could not find tracefs, attempting to mount it now".as_ptr());
    strcpy(mnt, KNOWN_MNTS[0]);
    if mount_tracefs(mnt) != 0 {
        return false;
    }

    exit_found(mnt, pipe_name)
}

unsafe fn exit_found(mnt: *mut c_char, pipe_name: *const c_char) -> bool {
    strcat(mnt, pipe_name);
    true
}

unsafe extern "C" fn exit_tracelog(_signum: c_int) {
    fclose(trace_pipe_fd);
    free(buff as *mut c_void);

    if json_output {
        jsonw_end_array(json_wtr);
        jsonw_destroy(&mut json_wtr);
    }

    exit(0);
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn do_tracelog(_argc: c_int, _argv: *mut *mut c_char) -> c_int {
    let mut act: libc::sigaction = core::mem::zeroed();
    let mut trace_pipe: [c_char; PATH_MAX] = [0; PATH_MAX];
    let mut buff_len: size_t = 0;

    act.sa_sigaction = exit_tracelog as usize;

    if json_output {
        jsonw_start_array(json_wtr);
    }

    if !get_tracefs_pipe(trace_pipe.as_mut_ptr()) {
        return -1;
    }

    trace_pipe_fd = fopen(trace_pipe.as_ptr(), c"r".as_ptr());
    if trace_pipe_fd.is_null() {
        p_err(
            c"could not open trace pipe: %s".as_ptr(),
            strerror(errno),
        );
        return -1;
    }

    sigaction(SIGHUP, &act, core::ptr::null_mut());
    sigaction(SIGINT, &act, core::ptr::null_mut());
    sigaction(SIGTERM, &act, core::ptr::null_mut());
    loop {
        let ret: ssize_t;

        ret = getline(&mut buff, &mut buff_len, trace_pipe_fd);
        if ret <= 0 {
            p_err(
                c"failed to read content from trace pipe: %s".as_ptr(),
                strerror(errno),
            );
            break;
        }
        if json_output {
            jsonw_string(json_wtr, buff);
        } else {
            printf(c"%s".as_ptr(), buff);
        }
    }

    fclose(trace_pipe_fd);
    free(buff as *mut c_void);
    -1
}
