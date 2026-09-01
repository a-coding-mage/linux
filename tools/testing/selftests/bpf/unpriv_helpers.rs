// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::{c_char, c_int, c_uint, c_void};

type gzFile = *mut c_void;
type size_t = usize;
type ssize_t = isize;

const PATH_MAX: usize = 4096;
const O_RDONLY: c_int = 0;
const Z_OK: c_int = 0;
const Z_ERRNO: c_int = -1;

// From "unpriv_helpers.h".
const UNPRIV_SYSCTL: &[u8] = b"kernel/unprivileged_bpf_disabled\0";

#[repr(C)]
struct utsname {
    sysname: [c_char; 65],
    nodename: [c_char; 65],
    release: [c_char; 65],
    version: [c_char; 65],
    machine: [c_char; 65],
    domainname: [c_char; 65],
}

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

unsafe extern "C" {
    fn uname(buf: *mut utsname) -> c_int;
    fn perror(s: *const c_char);
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn gzopen(path: *const c_char, mode: *const c_char) -> gzFile;
    fn gzgets(file: gzFile, buf: *mut c_char, len: c_int) -> *mut c_char;
    fn gzerror(file: gzFile, errnum: *mut c_int) -> *const c_char;
    fn gzclose(file: gzFile) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;
    fn fclose(stream: *mut FILE) -> c_int;
    fn atoi(nptr: *const c_char) -> c_int;

    static mut errno: c_int;
    static mut stderr: *mut FILE;
}

unsafe fn open_config() -> gzFile {
    let mut uts: utsname = core::mem::zeroed();
    let mut buf = [0 as c_char; PATH_MAX];
    let mut config: gzFile;

    if uname(&mut uts) != 0 {
        perror(c"uname".as_ptr());
    } else {
        snprintf(
            buf.as_mut_ptr(),
            buf.len(),
            c"/boot/config-%s".as_ptr(),
            uts.release.as_ptr(),
        );
        config = gzopen(buf.as_ptr(), c"rb".as_ptr());
        if !config.is_null() {
            return config;
        }
        fprintf(
            stderr,
            c"gzopen %s: %s\n".as_ptr(),
            buf.as_ptr(),
            strerror(errno),
        );
    }

    config = gzopen(c"/proc/config.gz".as_ptr(), c"rb".as_ptr());
    if config.is_null() {
        perror(c"gzopen /proc/config.gz".as_ptr());
    }
    config
}

unsafe fn config_contains(pat: *const c_char) -> c_int {
    let mut msg: *const c_char;
    let mut buf = [0 as c_char; 1024];
    let config: gzFile;
    let mut n: c_int;
    let mut err: c_int = 0;

    config = open_config();
    if config.is_null() {
        return -1;
    }

    loop {
        if gzgets(config, buf.as_mut_ptr(), buf.len() as c_int).is_null() {
            msg = gzerror(config, &mut err);
            if err == Z_ERRNO {
                perror(c"gzgets /proc/config.gz".as_ptr());
            } else if err != Z_OK {
                fprintf(stderr, c"gzgets /proc/config.gz: %s".as_ptr(), msg);
            }
            gzclose(config);
            return -1;
        }
        n = strlen(buf.as_ptr()) as c_int;
        if buf[(n - 1) as usize] == b'\n' as c_char {
            buf[(n - 1) as usize] = 0;
        }
        if strcmp(buf.as_ptr(), pat) == 0 {
            gzclose(config);
            return 1;
        }
    }
}

unsafe fn cmdline_contains(pat: *const c_char) -> bool {
    let mut cmdline = [0 as c_char; 4096];
    let mut c: *mut c_char;
    let fd: c_int;
    let mut ret: c_int = 0;

    fd = open(c"/proc/cmdline".as_ptr(), O_RDONLY);
    if fd < 0 {
        perror(c"open /proc/cmdline".as_ptr());
        return false;
    }

    if read(
        fd,
        cmdline.as_mut_ptr() as *mut c_void,
        core::mem::size_of_val(&cmdline) - 1,
    ) < 0
    {
        perror(c"read /proc/cmdline".as_ptr());
    } else {
        cmdline[core::mem::size_of_val(&cmdline) - 1] = 0;
        c = strtok(cmdline.as_mut_ptr(), c" \n".as_ptr());
        while !c.is_null() {
            if strncmp(c, pat, strlen(c)) == 0 {
                ret = 1;
                break;
            }
            c = strtok(core::ptr::null_mut(), c" \n".as_ptr());
        }
    }

    close(fd);
    ret != 0
}

unsafe fn get_mitigations_off() -> c_int {
    let enabled_in_config: c_int;

    if cmdline_contains(c"mitigations=off".as_ptr()) {
        return 1;
    }
    enabled_in_config = config_contains(c"CONFIG_CPU_MITIGATIONS=y".as_ptr());
    if enabled_in_config < 0 {
        return -1;
    }
    (enabled_in_config == 0) as c_int
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_unpriv_disabled() -> bool {
    let mitigations_off: c_int;
    let mut disabled: bool;
    let mut buf = [0 as c_char; 2];
    let fd: *mut FILE;

    fd = fopen(c"/proc/sys/kernel/unprivileged_bpf_disabled".as_ptr(), c"r".as_ptr());
    if !fd.is_null() {
        disabled = fgets(buf.as_mut_ptr(), 2, fd) == buf.as_mut_ptr() && atoi(buf.as_ptr()) != 0;
        fclose(fd);
    } else {
        perror(c"fopen /proc/sys/kernel/unprivileged_bpf_disabled".as_ptr());
        disabled = true;
    }

    if disabled {
        return true;
    }

    /*
     * Some unpriv tests rely on spectre mitigations being on.
     * If mitigations are off or status can't be determined
     * assume that unpriv tests are disabled.
     */
    mitigations_off = get_mitigations_off();
    if mitigations_off < 0 {
        fprintf(
            stderr,
            c"Can't determine if mitigations are enabled, disabling unpriv tests.".as_ptr(),
        );
        return true;
    }
    mitigations_off != 0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
