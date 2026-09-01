// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2015-2019 Jason A. Donenfeld <Jason@zx2c4.com>. All Rights Reserved.
 */

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};
use core::mem::size_of;
use core::ptr::{null, null_mut};

type size_t = usize;
type ssize_t = isize;
type pid_t = c_int;
type time_t = c_long;
type off_t = c_long;
type mode_t = c_uint;

const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_RDWR: c_int = 2;
const O_NONBLOCK: c_int = 0o4000;
const F_SETFL: c_int = 4;
const RB_AUTOBOOT: c_int = 0x0123_4567;
const GRND_NONBLOCK: c_uint = 0x0001;
const RNDADDTOENTCNT: c_uint = 0x4004_5201;

#[repr(C)]
struct FILE {
    _private: [u8; 0],
}

#[repr(C)]
struct utsname {
    sysname: [c_char; 65],
    nodename: [c_char; 65],
    release: [c_char; 65],
    version: [c_char; 65],
    machine: [c_char; 65],
    domainname: [c_char; 65],
}

unsafe extern "C" {
    static mut errno: c_int;

    fn close(fd: c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn execl(path: *const c_char, arg: *const c_char, ...) -> c_int;
    fn fcntl(fd: c_int, cmd: c_int, ...) -> c_int;
    fn fileno(stream: *mut FILE) -> c_int;
    fn fork() -> pid_t;
    fn sleep(seconds: c_uint) -> c_uint;
    fn usleep(usec: c_uint) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;

    fn exit(status: c_int) -> !;
    fn fflush(stream: *mut FILE) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn puts(s: *const c_char) -> c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;

    fn fopen(path: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fgets(s: *mut c_char, size: c_int, stream: *mut FILE) -> *mut c_char;

    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn strchrnul(s: *const c_char, c: c_int) -> *mut c_char;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn strtok(str: *mut c_char, delim: *const c_char) -> *mut c_char;

    fn getrandom(buf: *mut c_void, buflen: size_t, flags: c_uint) -> ssize_t;
    fn ioctl(fd: c_int, request: c_uint, ...) -> c_int;
    fn mkdir(path: *const c_char, mode: mode_t) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: usize,
        data: *const c_void,
    ) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn reboot(cmd: c_int) -> c_int;
    fn sendfile(out_fd: c_int, in_fd: c_int, offset: *mut off_t, count: size_t) -> ssize_t;
    fn stime(t: *const time_t) -> c_int;
    fn symlink(target: *const c_char, linkpath: *const c_char) -> c_int;
    fn time(tloc: *mut time_t) -> time_t;
    fn uname(buf: *mut utsname) -> c_int;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

fn wifsignaled(status: c_int) -> bool {
    let term_sig = status & 0x7f;
    term_sig != 0 && term_sig != 0x7f
}

fn wtermsig(status: c_int) -> c_int {
    status & 0x7f
}

fn pretty_message(msg: *const c_char) {
    unsafe {
        printf(c"\x1b[32m\x1b[1m%s\x1b[0m\n".as_ptr(), msg);
    }
}

fn poweroff() -> ! {
    unsafe {
        fflush(stdout);
        fflush(stderr);
        reboot(RB_AUTOBOOT);
        sleep(30);
        fprintf(
            stderr,
            c"\x1b[37m\x1b[41m\x1b[1mFailed to power off!!!\x1b[0m\n".as_ptr(),
        );
        exit(1);
    }
}

fn panic(what: *const c_char) {
    unsafe {
        fprintf(
            stderr,
            c"\n\n\x1b[37m\x1b[41m\x1b[1mSOMETHING WENT HORRIBLY WRONG\x1b[0m\n\n    \x1b[31m\x1b[1m%s: %s\x1b[0m\n\n\x1b[37m\x1b[44m\x1b[1mPower off...\x1b[0m\n\n".as_ptr(),
            what,
            strerror(errno),
        );
    }
    poweroff();
}

fn print_banner() {
    unsafe {
        let mut utsname = core::mem::zeroed::<utsname>();

        if uname(&mut utsname) < 0 {
            panic(c"uname".as_ptr());
        }

        let len = strlen(c"    WireGuard Test Suite on       ".as_ptr())
            + strlen(utsname.sysname.as_ptr())
            + strlen(utsname.release.as_ptr())
            + strlen(utsname.machine.as_ptr());
        printf(
            c"\x1b[45m\x1b[33m\x1b[1m%*.s\x1b[0m\n\x1b[45m\x1b[33m\x1b[1m    WireGuard Test Suite on %s %s %s    \x1b[0m\n\x1b[45m\x1b[33m\x1b[1m%*.s\x1b[0m\n\n".as_ptr(),
            len as c_int,
            c"".as_ptr(),
            utsname.sysname.as_ptr(),
            utsname.release.as_ptr(),
            utsname.machine.as_ptr(),
            len as c_int,
            c"".as_ptr(),
        );
    }
}

fn seed_rng() {
    unsafe {
        let mut bits: c_int = 256;

        if getrandom(null_mut(), 0, GRND_NONBLOCK) == 0 {
            return;
        }
        pretty_message(c"[+] Fake seeding RNG...".as_ptr());
        let fd = open(c"/dev/random".as_ptr(), O_WRONLY);
        if fd < 0 {
            panic(c"open(random)".as_ptr());
        }
        if ioctl(fd, RNDADDTOENTCNT, &mut bits as *mut c_int) < 0 {
            panic(c"ioctl(RNDADDTOENTCNT)".as_ptr());
        }
        close(fd);
    }
}

fn set_time() {
    unsafe {
        if time(null_mut()) != 0 {
            return;
        }
        pretty_message(c"[+] Setting fake time...".as_ptr());
        let t: time_t = 1433512680;
        if stime(&t) < 0 {
            panic(c"settimeofday()".as_ptr());
        }
    }
}

fn mount_filesystems() {
    unsafe {
        pretty_message(c"[+] Mounting filesystems...".as_ptr());
        mkdir(c"/dev".as_ptr(), 0o755);
        mkdir(c"/proc".as_ptr(), 0o755);
        mkdir(c"/sys".as_ptr(), 0o755);
        mkdir(c"/tmp".as_ptr(), 0o755);
        mkdir(c"/run".as_ptr(), 0o755);
        mkdir(c"/var".as_ptr(), 0o755);
        if mount(c"none".as_ptr(), c"/dev".as_ptr(), c"devtmpfs".as_ptr(), 0, null()) != 0 {
            panic(c"devtmpfs mount".as_ptr());
        }
        if mount(c"none".as_ptr(), c"/proc".as_ptr(), c"proc".as_ptr(), 0, null()) != 0 {
            panic(c"procfs mount".as_ptr());
        }
        if mount(c"none".as_ptr(), c"/sys".as_ptr(), c"sysfs".as_ptr(), 0, null()) != 0 {
            panic(c"sysfs mount".as_ptr());
        }
        if mount(c"none".as_ptr(), c"/tmp".as_ptr(), c"tmpfs".as_ptr(), 0, null()) != 0 {
            panic(c"tmpfs mount".as_ptr());
        }
        if mount(c"none".as_ptr(), c"/run".as_ptr(), c"tmpfs".as_ptr(), 0, null()) != 0 {
            panic(c"tmpfs mount".as_ptr());
        }
        if mount(c"none".as_ptr(), c"/sys/kernel/debug".as_ptr(), c"debugfs".as_ptr(), 0, null()) != 0 {
            /* Not a problem if it fails.*/
        }
        if symlink(c"/run".as_ptr(), c"/var/run".as_ptr()) != 0 {
            panic(c"run symlink".as_ptr());
        }
        if symlink(c"/proc/self/fd".as_ptr(), c"/dev/fd".as_ptr()) != 0 {
            panic(c"fd symlink".as_ptr());
        }
    }
}

fn enable_logging() {
    unsafe {
        pretty_message(c"[+] Enabling logging...".as_ptr());
        let mut fd = open(c"/proc/sys/kernel/printk".as_ptr(), O_WRONLY);
        if fd >= 0 {
            if write(fd, c"9\n".as_ptr() as *const c_void, 2) != 2 {
                panic(c"write(printk)".as_ptr());
            }
            close(fd);
        }
        fd = open(c"/proc/sys/debug/exception-trace".as_ptr(), O_WRONLY);
        if fd >= 0 {
            if write(fd, c"1\n".as_ptr() as *const c_void, 2) != 2 {
                panic(c"write(exception-trace)".as_ptr());
            }
            close(fd);
        }
    }
}

fn kmod_selftests() {
    unsafe {
        let mut line = [0 as c_char; 2048];
        let mut success = true;
        pretty_message(c"[+] Module self-tests:".as_ptr());
        let file = fopen(c"/proc/kmsg".as_ptr(), c"r".as_ptr());
        if file.is_null() {
            panic(c"fopen(kmsg)".as_ptr());
        }
        if fcntl(fileno(file), F_SETFL, O_NONBLOCK) < 0 {
            panic(c"fcntl(kmsg, nonblock)".as_ptr());
        }
        while !fgets(line.as_mut_ptr(), size_of::<[c_char; 2048]>() as c_int, file).is_null() {
            let mut start = strstr(line.as_ptr(), c"wireguard: ".as_ptr());
            if start.is_null() {
                continue;
            }
            start = start.add(11);
            *strchrnul(start, '\n' as c_int) = '\0' as c_char;
            if !strstr(start, c"www.wireguard.com".as_ptr()).is_null() {
                break;
            }
            let pass = strstr(start, c": pass".as_ptr());
            if pass.is_null() || *pass.add(6) != '\0' as c_char {
                success = false;
                printf(c" \x1b[31m*  %s\x1b[0m\n".as_ptr(), start);
            } else {
                printf(c" \x1b[32m*  %s\x1b[0m\n".as_ptr(), start);
            }
        }
        fclose(file);
        if !success {
            puts(c"\x1b[31m\x1b[1m[-] Tests failed! \u{2639}\x1b[0m".as_ptr());
            poweroff();
        }
    }
}

fn launch_tests() {
    unsafe {
        let mut cmdline = [0 as c_char; 4096];
        let mut status: c_int = 0;

        pretty_message(c"[+] Launching tests...".as_ptr());
        let pid = fork();
        if pid == -1 {
            panic(c"fork".as_ptr());
        } else if pid == 0 {
            execl(c"/init.sh".as_ptr(), c"init".as_ptr(), null::<c_char>());
            panic(c"exec".as_ptr());
        }
        if waitpid(pid, &mut status, 0) < 0 {
            panic(c"waitpid".as_ptr());
        }
        if wifexited(status) && wexitstatus(status) == 0 {
            pretty_message(c"[+] Tests successful! :-)".as_ptr());
            let fd = open(c"/proc/cmdline".as_ptr(), O_RDONLY);
            if fd < 0 {
                panic(c"open(/proc/cmdline)".as_ptr());
            }
            if read(fd, cmdline.as_mut_ptr() as *mut c_void, size_of::<[c_char; 4096]>() - 1) <= 0 {
                panic(c"read(/proc/cmdline)".as_ptr());
            }
            cmdline[size_of::<[c_char; 4096]>() - 1] = '\0' as c_char;
            let mut success_dev = strtok(cmdline.as_mut_ptr(), c" \n".as_ptr());
            while !success_dev.is_null() {
                if strncmp(success_dev, c"wg.success=".as_ptr(), 11) == 0 {
                    memcpy(
                        success_dev.add(11 - 5) as *mut c_void,
                        c"/dev/".as_ptr() as *const c_void,
                        5,
                    );
                    success_dev = success_dev.add(11 - 5);
                    break;
                }
                success_dev = strtok(null_mut(), c" \n".as_ptr());
            }
            if success_dev.is_null() || strlen(success_dev) == 0 {
                panic(c"Unable to find success device".as_ptr());
            }

            let fd = open(success_dev, O_WRONLY);
            if fd < 0 {
                panic(c"open(success_dev)".as_ptr());
            }
            if write(fd, c"success\n".as_ptr() as *const c_void, 8) != 8 {
                panic(c"write(success_dev)".as_ptr());
            }
            close(fd);
        } else {
            let mut why = c"unknown cause".as_ptr();
            let mut what: c_int = -1;

            if wifexited(status) {
                why = c"exit code".as_ptr();
                what = wexitstatus(status);
            } else if wifsignaled(status) {
                why = c"signal".as_ptr();
                what = wtermsig(status);
            }
            printf(
                c"\x1b[31m\x1b[1m[-] Tests failed with %s %d! \u{2639}\x1b[0m\n".as_ptr(),
                why,
                what,
            );
        }
    }
}

fn ensure_console() {
    unsafe {
        for _i in 0u32..1000 {
            let fd = open(c"/dev/console".as_ptr(), O_RDWR);
            if fd < 0 {
                usleep(50000);
                continue;
            }
            dup2(fd, 0);
            dup2(fd, 1);
            dup2(fd, 2);
            close(fd);
            if write(1, c"\0\0\0\0\n".as_ptr() as *const c_void, 5) == 5 {
                return;
            }
        }
        panic(c"Unable to open console device".as_ptr());
    }
}

fn clear_leaks() {
    unsafe {
        let fd = open(c"/sys/kernel/debug/kmemleak".as_ptr(), O_WRONLY);
        if fd < 0 {
            return;
        }
        pretty_message(c"[+] Starting memory leak detection...".as_ptr());
        write(fd, c"clear\n".as_ptr() as *const c_void, 5);
        close(fd);
    }
}

fn check_leaks() {
    unsafe {
        let mut fd = open(c"/sys/kernel/debug/kmemleak".as_ptr(), O_WRONLY);
        if fd < 0 {
            return;
        }
        pretty_message(c"[+] Scanning for memory leaks...".as_ptr());
        sleep(2); /* Wait for any grace periods. */
        write(fd, c"scan\n".as_ptr() as *const c_void, 5);
        close(fd);

        fd = open(c"/sys/kernel/debug/kmemleak".as_ptr(), O_RDONLY);
        if fd < 0 {
            return;
        }
        if sendfile(1, fd, null_mut(), 0x7ffff000) > 0 {
            panic(c"Memory leaks encountered".as_ptr());
        }
        close(fd);
    }
}

fn main() {
    ensure_console();
    print_banner();
    mount_filesystems();
    seed_rng();
    set_time();
    kmod_selftests();
    enable_logging();
    clear_leaks();
    launch_tests();
    check_leaks();
    poweroff();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
