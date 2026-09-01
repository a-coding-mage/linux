// SPDX-License-Identifier: GPL-2.0
// C source defined _GNU_SOURCE and included libc/system headers for errno,
// fcntl, sched, stdio/stdlib/string, mount/stat/statfs, and unistd APIs.

use std::ffi::{CStr, CString};
use std::fmt;
use std::os::raw::{c_char, c_int, c_void};
use std::process;

const MS_NOSYMFOLLOW: libc::c_ulong = 256; /* Do not follow symlinks */
const ST_NOSYMFOLLOW: libc::c_ulong = 0x2000; /* Do not follow symlinks */

const DATA: &str = "/tmp/data";
const LINK: &str = "/tmp/symlink";
const TMP: &str = "/tmp";

fn errno() -> c_int {
    unsafe { *libc::__errno_location() }
}

fn strerror_string(err: c_int) -> String {
    unsafe {
        CStr::from_ptr(libc::strerror(err))
            .to_string_lossy()
            .into_owned()
    }
}

fn die(args: fmt::Arguments<'_>) -> ! {
    eprint!("{}", args);
    process::exit(libc::EXIT_FAILURE);
}

macro_rules! die {
    ($($arg:tt)*) => {
        die(format_args!($($arg)*))
    };
}

fn cstring(s: &str) -> CString {
    CString::new(s).unwrap_or_else(|_| die!("CString conversion failed\n"))
}

fn vmaybe_write_file(enoent_ok: bool, filename: &str, args: fmt::Arguments<'_>) {
    let buf = format!("{}", args);
    if buf.len() >= 4096 {
        die!("vsnprintf output truncated\n");
    }

    let filename_c = cstring(filename);
    let fd = unsafe { libc::open(filename_c.as_ptr(), libc::O_WRONLY) };
    if fd < 0 {
        let err = errno();
        if err == libc::ENOENT && enoent_ok {
            return;
        }
        die!("open of {} failed: {}\n", filename, strerror_string(err));
    }

    let written = unsafe { libc::write(fd, buf.as_ptr() as *const c_void, buf.len()) };
    if written != buf.len() as libc::ssize_t {
        if written >= 0 {
            die!("short write to {}\n", filename);
        } else {
            die!("write to {} failed: {}\n", filename, strerror_string(errno()));
        }
    }

    if unsafe { libc::close(fd) } != 0 {
        die!("close of {} failed: {}\n", filename, strerror_string(errno()));
    }
}

macro_rules! maybe_write_file {
    ($filename:expr, $($arg:tt)*) => {
        vmaybe_write_file(true, $filename, format_args!($($arg)*))
    };
}

macro_rules! write_file {
    ($filename:expr, $($arg:tt)*) => {
        vmaybe_write_file(false, $filename, format_args!($($arg)*))
    };
}

fn create_and_enter_ns() {
    let uid = unsafe { libc::getuid() };
    let gid = unsafe { libc::getgid() };

    if unsafe { libc::unshare(libc::CLONE_NEWUSER) } != 0 {
        die!(
            "unshare(CLONE_NEWUSER) failed: {}\n",
            strerror_string(errno())
        );
    }

    maybe_write_file!("/proc/self/setgroups", "deny");
    write_file!("/proc/self/uid_map", "0 {} 1", uid);
    write_file!("/proc/self/gid_map", "0 {} 1", gid);

    if unsafe { libc::setgid(0) } != 0 {
        die!("setgid(0) failed {}\n", strerror_string(errno()));
    }
    if unsafe { libc::setuid(0) } != 0 {
        die!("setuid(0) failed {}\n", strerror_string(errno()));
    }

    if unsafe { libc::unshare(libc::CLONE_NEWNS) } != 0 {
        die!(
            "unshare(CLONE_NEWNS) failed: {}\n",
            strerror_string(errno())
        );
    }
}

fn setup_symlink() {
    let data_c = cstring(DATA);
    let link_c = cstring(LINK);

    let data = unsafe { libc::creat(data_c.as_ptr(), libc::O_RDWR as libc::mode_t) };
    if data < 0 {
        die!("creat failed: {}\n", strerror_string(errno()));
    }

    let err = unsafe { libc::symlink(data_c.as_ptr(), link_c.as_ptr()) };
    if err < 0 {
        die!("symlink failed: {}\n", strerror_string(errno()));
    }

    if unsafe { libc::close(data) } != 0 {
        die!("close of {} failed: {}\n", DATA, strerror_string(errno()));
    }
}

fn test_link_traversal(nosymfollow: bool) {
    let link_c = cstring(LINK);

    let link = unsafe { libc::open(link_c.as_ptr(), 0, libc::O_RDWR) };
    if nosymfollow {
        if link != -1 || errno() != libc::ELOOP {
            die!(
                "link traversal unexpected result: {}, {}\n",
                link,
                strerror_string(errno())
            );
        }
    } else {
        if link < 0 {
            die!("link traversal failed: {}\n", strerror_string(errno()));
        }

        if unsafe { libc::close(link) } != 0 {
            die!("close of link failed: {}\n", strerror_string(errno()));
        }
    }
}

fn test_readlink() {
    let link_c = cstring(LINK);
    let mut buf = [0 as c_char; 4096];

    let ret = unsafe { libc::readlink(link_c.as_ptr(), buf.as_mut_ptr(), buf.len()) };
    if ret < 0 {
        die!("readlink failed: {}\n", strerror_string(errno()));
    }
    let data_c = cstring(DATA);
    if unsafe { libc::strcmp(buf.as_ptr(), data_c.as_ptr()) } != 0 {
        let buf_s = unsafe { CStr::from_ptr(buf.as_ptr()) }
            .to_string_lossy()
            .into_owned();
        die!("readlink strcmp failed: '{}' '{}'\n", buf_s, DATA);
    }
}

fn test_realpath() {
    let link_c = cstring(LINK);
    let path = unsafe { libc::realpath(link_c.as_ptr(), std::ptr::null_mut()) };

    if path.is_null() {
        die!("realpath failed: {}\n", strerror_string(errno()));
    }
    let data_c = cstring(DATA);
    if unsafe { libc::strcmp(path, data_c.as_ptr()) } != 0 {
        die!("realpath strcmp failed\n");
    }

    unsafe { libc::free(path as *mut c_void) };
}

fn test_statfs(nosymfollow: bool) {
    let tmp_c = cstring(TMP);
    let mut buf: libc::statfs = unsafe { std::mem::zeroed() };

    let ret = unsafe { libc::statfs(tmp_c.as_ptr(), &mut buf) };
    if ret != 0 {
        die!("statfs failed: {}\n", strerror_string(errno()));
    }

    if nosymfollow {
        if (buf.f_flags as libc::c_ulong & ST_NOSYMFOLLOW) == 0 {
            die!("ST_NOSYMFOLLOW not set on {}\n", TMP);
        }
    } else {
        if (buf.f_flags as libc::c_ulong & ST_NOSYMFOLLOW) != 0 {
            die!("ST_NOSYMFOLLOW set on {}\n", TMP);
        }
    }
}

fn run_tests(nosymfollow: bool) {
    test_link_traversal(nosymfollow);
    test_readlink();
    test_realpath();
    test_statfs(nosymfollow);
}

fn main() {
    create_and_enter_ns();

    let testing_c = cstring("testing");
    let tmp_c = cstring(TMP);
    let ramfs_c = cstring("ramfs");

    if unsafe {
        libc::mount(
            testing_c.as_ptr() as *const c_char,
            tmp_c.as_ptr() as *const c_char,
            ramfs_c.as_ptr() as *const c_char,
            0,
            std::ptr::null::<c_void>(),
        )
    } != 0
    {
        die!("mount failed: {}\n", strerror_string(errno()));
    }

    setup_symlink();
    run_tests(false);

    if unsafe {
        libc::mount(
            testing_c.as_ptr() as *const c_char,
            tmp_c.as_ptr() as *const c_char,
            ramfs_c.as_ptr() as *const c_char,
            (libc::MS_REMOUNT as libc::c_ulong) | MS_NOSYMFOLLOW,
            std::ptr::null::<c_void>(),
        )
    } != 0
    {
        die!("remount failed: {}\n", strerror_string(errno()));
    }

    run_tests(true);

    process::exit(libc::EXIT_SUCCESS);
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
