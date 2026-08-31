// SPDX-License-Identifier: GPL-2.0
// Translated from lib/subcmd/exec-cmd.c.
// C dependencies included: linux/compiler.h, linux/string.h, sys/types.h,
// sys/stat.h, unistd.h, string.h, stdlib.h, stdio.h, subcmd-util.h,
// exec-cmd.h, subcmd-config.h.

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

const MAX_ARGS: usize = 32;
const PATH_MAX: usize = 4096;

#[repr(C)]
pub struct stat {
    pub st_dev: u64,
    pub st_ino: u64,
}

#[repr(C)]
pub struct subcmd_config {
    pub exec_name: *const c_char,
    pub prefix: *const c_char,
    pub exec_path: *const c_char,
    pub exec_path_env: *const c_char,
}

unsafe extern "C" {
    static mut subcmd_config: subcmd_config;
    static mut stderr: *mut c_void;

    fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
    fn getcwd(buf: *mut c_char, size: usize) -> *mut c_char;
    fn getenv(name: *const c_char) -> *mut c_char;
    fn stat(path: *const c_char, buf: *mut stat) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;
    fn strlcpy(dst: *mut c_char, src: *const c_char, size: usize) -> usize;
    fn strcat(dst: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn strndup(s: *const c_char, n: usize) -> *mut c_char;
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn execvp(file: *const c_char, argv: *mut *mut c_char) -> c_int;
    fn fprintf(stream: *mut c_void, format: *const c_char, ...) -> c_int;

    fn die(format: *const c_char, ...) -> !;
    fn astrcat(buf: *mut *mut c_char, str_: *const c_char);
    fn astrcatf(buf: *mut *mut c_char, format: *const c_char, ...);
}

static mut argv_exec_path: *const c_char = ptr::null();
static mut argv0_path: *const c_char = ptr::null();

#[unsafe(no_mangle)]
pub unsafe extern "C" fn exec_cmd_init(
    exec_name: *const c_char,
    prefix: *const c_char,
    exec_path: *const c_char,
    exec_path_env: *const c_char,
) {
    unsafe {
        subcmd_config.exec_name = exec_name;
        subcmd_config.prefix = prefix;
        subcmd_config.exec_path = exec_path;
        subcmd_config.exec_path_env = exec_path_env;

        /* Setup environment variable for invoked shell script. */
        setenv(c"PREFIX".as_ptr(), prefix, 1);
    }
}

#[inline]
unsafe fn is_dir_sep(c: c_char) -> bool {
    c == b'/' as c_char
}

unsafe fn is_absolute_path(path: *const c_char) -> c_int {
    unsafe { (*path == b'/' as c_char) as c_int }
}

unsafe fn get_pwd_cwd(buf: *mut c_char, sz: usize) -> *const c_char {
    let pwd: *mut c_char;
    let mut cwd_stat: stat = unsafe { mem::zeroed() };
    let mut pwd_stat: stat = unsafe { mem::zeroed() };

    unsafe {
        if getcwd(buf, sz).is_null() {
            return ptr::null();
        }
        pwd = getenv(c"PWD".as_ptr());
        if !pwd.is_null() && strcmp(pwd, buf) != 0 {
            stat(buf, &mut cwd_stat);
            if stat(pwd, &mut pwd_stat) == 0
                && pwd_stat.st_dev == cwd_stat.st_dev
                && pwd_stat.st_ino == cwd_stat.st_ino
            {
                strlcpy(buf, pwd, sz);
            }
        }
    }
    buf
}

unsafe fn make_nonrelative_path(
    buf: *mut c_char,
    sz: usize,
    path: *const c_char,
) -> *const c_char {
    unsafe {
        if is_absolute_path(path) != 0 {
            if strlcpy(buf, path, sz) >= sz {
                die(c"Too long path: %.*s".as_ptr(), 60, path);
            }
        } else {
            let cwd = get_pwd_cwd(buf, sz);

            if cwd.is_null() {
                die(c"Cannot determine the current working directory".as_ptr());
            }

            if strlen(cwd) + strlen(path) + 2 >= sz {
                die(c"Too long path: %.*s".as_ptr(), 60, path);
            }

            strcat(buf, c"/".as_ptr());
            strcat(buf, path);
        }
    }
    buf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn system_path(path: *const c_char) -> *mut c_char {
    let mut buf: *mut c_char = ptr::null_mut();

    unsafe {
        if is_absolute_path(path) != 0 {
            return strdup(path);
        }

        astrcatf(&mut buf, c"%s/%s".as_ptr(), subcmd_config.prefix, path);
    }

    buf
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn extract_argv0_path(argv0: *const c_char) -> *const c_char {
    let mut slash: *const c_char;

    unsafe {
        if argv0.is_null() || *argv0 == 0 {
            return ptr::null();
        }
        slash = argv0.add(strlen(argv0));

        while argv0 <= slash && !is_dir_sep(*slash) {
            slash = slash.sub(1);
        }

        if slash >= argv0 {
            argv0_path = strndup(argv0, slash.offset_from(argv0) as usize);
            return if !argv0_path.is_null() {
                slash.add(1)
            } else {
                ptr::null()
            };
        }
    }

    argv0
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn set_argv_exec_path(exec_path: *const c_char) {
    unsafe {
        argv_exec_path = exec_path;
        /*
         * Propagate this setting to external programs.
         */
        setenv(subcmd_config.exec_path_env, exec_path, 1);
    }
}

/* Returns the highest-priority location to look for subprograms. */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn get_argv_exec_path() -> *mut c_char {
    let env: *mut c_char;

    unsafe {
        if !argv_exec_path.is_null() {
            return strdup(argv_exec_path);
        }

        env = getenv(subcmd_config.exec_path_env);
        if !env.is_null() && *env != 0 {
            return strdup(env);
        }

        system_path(subcmd_config.exec_path)
    }
}

unsafe fn add_path(out: *mut *mut c_char, path: *const c_char) {
    unsafe {
        if !path.is_null() && *path != 0 {
            if is_absolute_path(path) != 0 {
                astrcat(out, path);
            } else {
                let mut buf = [0 as c_char; PATH_MAX];

                astrcat(out, make_nonrelative_path(buf.as_mut_ptr(), buf.len(), path));
            }

            astrcat(out, c":".as_ptr());
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn setup_path() {
    unsafe {
        let old_path = getenv(c"PATH".as_ptr());
        let mut new_path: *mut c_char = ptr::null_mut();
        let tmp = get_argv_exec_path();

        add_path(&mut new_path, tmp);
        add_path(&mut new_path, argv0_path);
        free(tmp.cast::<c_void>());

        if !old_path.is_null() {
            astrcat(&mut new_path, old_path);
        } else {
            astrcat(&mut new_path, c"/usr/local/bin:/usr/bin:/bin".as_ptr());
        }

        setenv(c"PATH".as_ptr(), new_path, 1);

        free(new_path.cast::<c_void>());
    }
}

unsafe fn prepare_exec_cmd(argv: *const *const c_char) -> *mut *const c_char {
    let mut argc: c_int;
    let nargv: *mut *const c_char;

    unsafe {
        argc = 0;
        while !(*argv.add(argc as usize)).is_null() {
            argc += 1;
        } /* just counting */
        nargv = malloc(mem::size_of::<*const c_char>() * (argc as usize + 2)).cast();

        *nargv.add(0) = subcmd_config.exec_name;
        argc = 0;
        while !(*argv.add(argc as usize)).is_null() {
            *nargv.add(argc as usize + 1) = *argv.add(argc as usize);
            argc += 1;
        }
        *nargv.add(argc as usize + 1) = ptr::null();
    }
    nargv
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn execv_cmd(argv: *const *const c_char) -> c_int {
    let nargv = unsafe { prepare_exec_cmd(argv) };

    unsafe {
        /* execvp() can only ever return if it fails */
        execvp(subcmd_config.exec_name, nargv.cast::<*mut c_char>());

        free(nargv.cast::<c_void>());
    }
    -1
}

/*
 * Original C definition:
 *
 * int execl_cmd(const char *cmd,...)
 * {
 *     int argc;
 *     const char *argv[MAX_ARGS + 1];
 *     const char *arg;
 *     va_list param;
 *
 *     va_start(param, cmd);
 *     argv[0] = cmd;
 *     argc = 1;
 *     while (argc < MAX_ARGS) {
 *         arg = argv[argc++] = va_arg(param, char *);
 *         if (!arg)
 *             break;
 *     }
 *     va_end(param);
 *     if (MAX_ARGS <= argc) {
 *         fprintf(stderr, " Error: too many args to run %s\n", cmd);
 *         return -1;
 *     }
 *
 *     argv[argc] = NULL;
 *     return execv_cmd(argv);
 * }
 *
 * A faithful Rust body requires defining a C-variadic Rust function and reading
 * its VaList. That is a Rust language support concern outside this isolated
 * file translation, so the C signature and behavior are preserved here as an
 * explicit dependency point.
 */
unsafe extern "C" {
    pub fn execl_cmd(cmd: *const c_char, ...) -> c_int;
}
