// SPDX-License-Identifier: GPL-2.0
// C dependencies: cap-ng.h, linux/capability.h, kselftest.h, and libc/POSIX APIs.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_void};

type bool_t = bool;
type pid_t = c_int;
type uid_t = c_uint;
type gid_t = c_uint;
type ssize_t = isize;
type size_t = usize;
type mode_t = c_uint;

const O_WRONLY: c_int = 1;
const O_RDONLY: c_int = 0;
const O_CREAT: c_int = 0o100;
const O_EXCL: c_int = 0o200;
const O_DIRECTORY: c_int = 0o200000;

const ENOENT: c_int = 2;
const EPERM: c_int = 1;
const EINVAL: c_int = 22;

const CLONE_NEWNS: c_int = 0x00020000;
const CLONE_NEWUSER: c_int = 0x10000000;

const MS_REC: c_ulong = 16384;
const MS_PRIVATE: c_ulong = 1 << 18;

type c_ulong = u64;

const PATH_MAX: usize = 4096;
const S_ISUID: mode_t = 0o4000;
const S_ISGID: mode_t = 0o2000;

const PR_SET_KEEPCAPS: c_int = 8;
const PR_CAP_AMBIENT: c_int = 47;
const PR_CAP_AMBIENT_IS_SET: c_int = 1;
const PR_CAP_AMBIENT_RAISE: c_int = 2;
const PR_CAP_AMBIENT_CLEAR_ALL: c_int = 4;

const CAP_LAST_CAP: c_int = 40;
const CAP_NET_BIND_SERVICE: c_int = 10;
const CAP_NET_RAW: c_int = 13;

const CAPNG_EFFECTIVE: c_int = 0;
const CAPNG_PERMITTED: c_int = 1;
const CAPNG_INHERITABLE: c_int = 2;
const CAPNG_ADD: c_int = 1;
const CAPNG_DROP: c_int = 0;
const CAPNG_SELECT_CAPS: c_int = 0;

static mut nerrs: c_int = 0;
static mut mpid: pid_t = 0; /*  main() pid is used to avoid duplicate test counts */

unsafe extern "C" {
    static mut errno: c_int;

    fn vsnprintf(s: *mut c_char, n: size_t, format: *const c_char, arg: VaList) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn openat(dirfd: c_int, pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn getuid() -> uid_t;
    fn getgid() -> gid_t;
    fn getegid() -> gid_t;
    fn unshare(flags: c_int) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
    fn setresuid(ruid: uid_t, euid: uid_t, suid: uid_t) -> c_int;
    fn setresgid(rgid: gid_t, egid: gid_t, sgid: gid_t) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn getcwd(buf: *mut c_char, size: size_t) -> *mut c_char;
    fn chdir(path: *const c_char) -> c_int;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, wstatus: *mut c_int, options: c_int) -> pid_t;
    fn getpid() -> pid_t;
    fn execl(path: *const c_char, arg: *const c_char, ...) -> c_int;
    fn chown(path: *const c_char, owner: uid_t, group: gid_t) -> c_int;
    fn chmod(path: *const c_char, mode: mode_t) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn dirname(path: *mut c_char) -> *mut c_char;
    fn free(ptr: *mut c_void);

    fn capng_get_caps_process() -> c_int;
    fn capng_have_capability(set: c_int, capability: c_int) -> c_int;
    fn capng_update(action: c_int, type_: c_int, capability: c_int) -> c_int;
    fn capng_apply(set: c_int) -> c_int;

    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_skip(fmt: *const c_char, ...) -> !;
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_print_cnts();
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
}

// Placeholder for the C va_list ABI type supplied by the target C ABI.
type VaList = *mut c_void;

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

unsafe fn vmaybe_write_file(
    enoent_ok: bool_t,
    filename: *mut c_char,
    fmt: *mut c_char,
    ap: VaList,
) {
    let mut buf = [0 as c_char; 4096];
    let fd: c_int;
    let written: ssize_t;
    let buf_len: c_int;

    buf_len = vsnprintf(buf.as_mut_ptr(), buf.len(), fmt, ap);
    if buf_len < 0 {
        ksft_exit_fail_msg(c"vsnprintf failed - %s\n".as_ptr(), strerror(errno));
    }

    if buf_len as usize >= buf.len() {
        ksft_exit_fail_msg(c"vsnprintf output truncated\n".as_ptr());
    }

    fd = open(filename, O_WRONLY);
    if fd < 0 {
        if errno == ENOENT && enoent_ok {
            return;
        }
        ksft_exit_fail_msg(
            c"open of %s failed - %s\n".as_ptr(),
            filename,
            strerror(errno),
        );
    }
    written = write(fd, buf.as_ptr() as *const c_void, buf_len as size_t);
    if written != buf_len as ssize_t {
        if written >= 0 {
            ksft_exit_fail_msg(c"short write to %s\n".as_ptr(), filename);
        } else {
            ksft_exit_fail_msg(
                c"write to %s failed - %s\n".as_ptr(),
                filename,
                strerror(errno),
            );
        }
    }
    if close(fd) != 0 {
        ksft_exit_fail_msg(
            c"close of %s failed - %s\n".as_ptr(),
            filename,
            strerror(errno),
        );
    }
}

// Rust has no stable C-variadic function definitions; these keep the original
// helper intent and delegate through the va_list-taking translation above.
unsafe extern "C" fn maybe_write_file(_filename: *mut c_char, _fmt: *mut c_char, _args: ...) {
    todo!("translate C va_start/va_end when C-variadic Rust definitions are available");
}

unsafe extern "C" fn write_file(_filename: *mut c_char, _fmt: *mut c_char, _args: ...) {
    todo!("translate C va_start/va_end when C-variadic Rust definitions are available");
}

unsafe fn create_and_enter_ns(inner_uid: uid_t) -> bool_t {
    let outer_uid: uid_t;
    let outer_gid: gid_t;
    let mut i: c_int;
    let ret: c_int;
    let have_outer_privilege: bool_t;

    outer_uid = getuid();
    outer_gid = getgid();

    if outer_uid == 0 && unshare(CLONE_NEWNS) == 0 {
        ksft_print_msg(c"[NOTE]\tUsing global UIDs for tests\n".as_ptr());
        if prctl(PR_SET_KEEPCAPS, 1 as c_long, 0 as c_long, 0 as c_long, 0 as c_long) != 0 {
            ksft_exit_fail_msg(c"PR_SET_KEEPCAPS - %s\n".as_ptr(), strerror(errno));
        }
        if setresuid(inner_uid, inner_uid, -1i32 as uid_t) != 0 {
            ksft_exit_fail_msg(c"setresuid - %s\n".as_ptr(), strerror(errno));
        }

        // Re-enable effective caps
        ret = capng_get_caps_process();
        if ret == -1 {
            ksft_exit_fail_msg(c"capng_get_caps_process failed\n".as_ptr());
        }

        i = 0;
        while i < CAP_LAST_CAP {
            if capng_have_capability(CAPNG_PERMITTED, i) != 0 {
                capng_update(CAPNG_ADD, CAPNG_EFFECTIVE, i);
            }
            i += 1;
        }
        if capng_apply(CAPNG_SELECT_CAPS) != 0 {
            ksft_exit_fail_msg(c"capng_apply - %s\n".as_ptr(), strerror(errno));
        }

        have_outer_privilege = true;
    } else if unshare(CLONE_NEWUSER | CLONE_NEWNS) == 0 {
        ksft_print_msg(c"[NOTE]\tUsing a user namespace for tests\n".as_ptr());
        maybe_write_file(c"/proc/self/setgroups".as_ptr() as *mut c_char, c"deny".as_ptr() as *mut c_char);
        write_file(
            c"/proc/self/uid_map".as_ptr() as *mut c_char,
            c"%d %d 1".as_ptr() as *mut c_char,
            inner_uid,
            outer_uid,
        );
        write_file(
            c"/proc/self/gid_map".as_ptr() as *mut c_char,
            c"0 %d 1".as_ptr() as *mut c_char,
            outer_gid,
        );

        have_outer_privilege = false;
    } else {
        ksft_exit_skip(c"must be root or be able to create a userns\n".as_ptr());
    }

    if mount(
        c"none".as_ptr(),
        c"/".as_ptr(),
        core::ptr::null(),
        MS_REC | MS_PRIVATE,
        core::ptr::null(),
    ) != 0
    {
        ksft_exit_fail_msg(c"remount everything private - %s\n".as_ptr(), strerror(errno));
    }

    have_outer_privilege
}

unsafe fn chdir_to_tmpfs() {
    let mut cwd = [0 as c_char; PATH_MAX];
    if getcwd(cwd.as_mut_ptr(), cwd.len()) != cwd.as_mut_ptr() {
        ksft_exit_fail_msg(c"getcwd - %s\n".as_ptr(), strerror(errno));
    }

    if mount(
        c"private_tmp".as_ptr(),
        c".".as_ptr(),
        c"tmpfs".as_ptr(),
        0,
        c"mode=0777".as_ptr() as *const c_void,
    ) != 0
    {
        ksft_exit_fail_msg(c"mount private tmpfs - %s\n".as_ptr(), strerror(errno));
    }

    if chdir(cwd.as_ptr()) != 0 {
        ksft_exit_fail_msg(c"chdir to private tmpfs - %s\n".as_ptr(), strerror(errno));
    }
}

unsafe fn copy_fromat_to(fromfd: c_int, fromname: *const c_char, toname: *const c_char) {
    let from = openat(fromfd, fromname, O_RDONLY);
    if from == -1 {
        ksft_exit_fail_msg(c"open copy source - %s\n".as_ptr(), strerror(errno));
    }

    let to = open(toname, O_CREAT | O_WRONLY | O_EXCL, 0o700 as mode_t);

    loop {
        let mut buf = [0 as c_char; 4096];
        let sz = read(from, buf.as_mut_ptr() as *mut c_void, buf.len());
        if sz == 0 {
            break;
        }
        if sz < 0 {
            ksft_exit_fail_msg(c"read - %s\n".as_ptr(), strerror(errno));
        }

        if write(to, buf.as_ptr() as *const c_void, sz as size_t) != sz {
            /* no short writes on tmpfs */
            ksft_exit_fail_msg(c"write - %s\n".as_ptr(), strerror(errno));
        }
    }

    close(from);
    close(to);
}

unsafe fn fork_wait() -> bool_t {
    let child = fork();
    if child == 0 {
        nerrs = 0;
        true
    } else if child > 0 {
        let mut status: c_int = 0;
        if waitpid(child, &mut status, 0) != child || !WIFEXITED(status) {
            ksft_print_msg(c"Child died\n".as_ptr());
            nerrs += 1;
        } else if WEXITSTATUS(status) != 0 {
            ksft_print_msg(c"Child failed\n".as_ptr());
            nerrs += 1;
        } else {
            /* don't print this message for mpid */
            if getpid() != mpid {
                ksft_test_result_pass(c"Passed\n".as_ptr());
            }
        }
        false
    } else {
        ksft_exit_fail_msg(c"fork - %s\n".as_ptr(), strerror(errno));
    }
}

unsafe fn exec_other_validate_cap(
    name: *const c_char,
    eff: bool_t,
    perm: bool_t,
    inh: bool_t,
    ambient: bool_t,
) {
    execl(
        name,
        name,
        if eff { c"1".as_ptr() } else { c"0".as_ptr() },
        if perm { c"1".as_ptr() } else { c"0".as_ptr() },
        if inh { c"1".as_ptr() } else { c"0".as_ptr() },
        if ambient { c"1".as_ptr() } else { c"0".as_ptr() },
        core::ptr::null::<c_char>(),
    );
    ksft_exit_fail_msg(c"execl - %s\n".as_ptr(), strerror(errno));
}

unsafe fn exec_validate_cap(eff: bool_t, perm: bool_t, inh: bool_t, ambient: bool_t) {
    exec_other_validate_cap(c"./validate_cap".as_ptr(), eff, perm, inh, ambient);
}

unsafe fn do_tests(uid: c_int, our_path: *const c_char) -> c_int {
    let ret: c_int;
    let have_outer_privilege = create_and_enter_ns(uid as uid_t);

    let ourpath_fd = open(our_path, O_RDONLY | O_DIRECTORY);
    if ourpath_fd == -1 {
        ksft_exit_fail_msg(c"open '%s' - %s\n".as_ptr(), our_path, strerror(errno));
    }

    chdir_to_tmpfs();

    copy_fromat_to(ourpath_fd, c"validate_cap".as_ptr(), c"validate_cap".as_ptr());

    if have_outer_privilege {
        let gid: uid_t = getegid();

        copy_fromat_to(
            ourpath_fd,
            c"validate_cap".as_ptr(),
            c"validate_cap_suidroot".as_ptr(),
        );
        if chown(c"validate_cap_suidroot".as_ptr(), 0, -1i32 as gid_t) != 0 {
            ksft_exit_fail_msg(c"chown - %s\n".as_ptr(), strerror(errno));
        }
        if chmod(c"validate_cap_suidroot".as_ptr(), S_ISUID | 0o700) != 0 {
            ksft_exit_fail_msg(c"chmod - %s\n".as_ptr(), strerror(errno));
        }

        copy_fromat_to(
            ourpath_fd,
            c"validate_cap".as_ptr(),
            c"validate_cap_suidnonroot".as_ptr(),
        );
        if chown(c"validate_cap_suidnonroot".as_ptr(), (uid + 1) as uid_t, -1i32 as gid_t) != 0 {
            ksft_exit_fail_msg(c"chown - %s\n".as_ptr(), strerror(errno));
        }
        if chmod(c"validate_cap_suidnonroot".as_ptr(), S_ISUID | 0o700) != 0 {
            ksft_exit_fail_msg(c"chmod - %s\n".as_ptr(), strerror(errno));
        }

        copy_fromat_to(
            ourpath_fd,
            c"validate_cap".as_ptr(),
            c"validate_cap_sgidroot".as_ptr(),
        );
        if chown(c"validate_cap_sgidroot".as_ptr(), -1i32 as uid_t, 0) != 0 {
            ksft_exit_fail_msg(c"chown - %s\n".as_ptr(), strerror(errno));
        }
        if chmod(c"validate_cap_sgidroot".as_ptr(), S_ISGID | 0o710) != 0 {
            ksft_exit_fail_msg(c"chmod - %s\n".as_ptr(), strerror(errno));
        }

        copy_fromat_to(
            ourpath_fd,
            c"validate_cap".as_ptr(),
            c"validate_cap_sgidnonroot".as_ptr(),
        );
        if chown(c"validate_cap_sgidnonroot".as_ptr(), -1i32 as uid_t, gid + 1) != 0 {
            ksft_exit_fail_msg(c"chown - %s\n".as_ptr(), strerror(errno));
        }
        if chmod(c"validate_cap_sgidnonroot".as_ptr(), S_ISGID | 0o710) != 0 {
            ksft_exit_fail_msg(c"chmod - %s\n".as_ptr(), strerror(errno));
        }
    }

    ret = capng_get_caps_process();
    if ret == -1 {
        ksft_exit_fail_msg(c"capng_get_caps_process failed\n".as_ptr());
    }

    /* Make sure that i starts out clear */
    capng_update(CAPNG_DROP, CAPNG_INHERITABLE, CAP_NET_BIND_SERVICE);
    if capng_apply(CAPNG_SELECT_CAPS) != 0 {
        ksft_exit_fail_msg(c"capng_apply - %s\n".as_ptr(), strerror(errno));
    }

    if uid == 0 {
        ksft_print_msg(c"[RUN]\tRoot => ep\n".as_ptr());
        if fork_wait() {
            exec_validate_cap(true, true, false, false);
        }
    } else {
        ksft_print_msg(c"[RUN]\tNon-root => no caps\n".as_ptr());
        if fork_wait() {
            exec_validate_cap(false, false, false, false);
        }
    }

    ksft_print_msg(c"Check cap_ambient manipulation rules\n".as_ptr());

    /* We should not be able to add ambient caps yet. */
    if prctl(
        PR_CAP_AMBIENT,
        PR_CAP_AMBIENT_RAISE,
        CAP_NET_BIND_SERVICE,
        0,
        0,
        0,
    ) != -1
        || errno != EPERM
    {
        if errno == EINVAL {
            ksft_test_result_fail(c"PR_CAP_AMBIENT_RAISE isn't supported\n".as_ptr());
        } else {
            ksft_test_result_fail(c"PR_CAP_AMBIENT_RAISE should have failed eith EPERM on a non-inheritable cap\n".as_ptr());
        }
        return 1;
    }
    ksft_test_result_pass(c"PR_CAP_AMBIENT_RAISE failed on non-inheritable cap\n".as_ptr());

    capng_update(CAPNG_ADD, CAPNG_INHERITABLE, CAP_NET_RAW);
    capng_update(CAPNG_DROP, CAPNG_PERMITTED, CAP_NET_RAW);
    capng_update(CAPNG_DROP, CAPNG_EFFECTIVE, CAP_NET_RAW);
    if capng_apply(CAPNG_SELECT_CAPS) != 0 {
        ksft_exit_fail_msg(c"capng_apply - %s\n".as_ptr(), strerror(errno));
    }
    if prctl(PR_CAP_AMBIENT, PR_CAP_AMBIENT_RAISE, CAP_NET_RAW, 0, 0, 0) != -1 || errno != EPERM {
        ksft_test_result_fail(c"PR_CAP_AMBIENT_RAISE should have failed on a non-permitted cap\n".as_ptr());
        return 1;
    }
    ksft_test_result_pass(c"PR_CAP_AMBIENT_RAISE failed on non-permitted cap\n".as_ptr());

    capng_update(CAPNG_ADD, CAPNG_INHERITABLE, CAP_NET_BIND_SERVICE);
    if capng_apply(CAPNG_SELECT_CAPS) != 0 {
        ksft_exit_fail_msg(c"capng_apply - %s\n".as_ptr(), strerror(errno));
    }
    if prctl(PR_CAP_AMBIENT, PR_CAP_AMBIENT_RAISE, CAP_NET_BIND_SERVICE, 0, 0, 0) != 0 {
        ksft_test_result_fail(c"PR_CAP_AMBIENT_RAISE should have succeeded\n".as_ptr());
        return 1;
    }
    ksft_test_result_pass(c"PR_CAP_AMBIENT_RAISE worked\n".as_ptr());

    if prctl(PR_CAP_AMBIENT, PR_CAP_AMBIENT_IS_SET, CAP_NET_BIND_SERVICE, 0, 0, 0) != 1 {
        ksft_test_result_fail(c"PR_CAP_AMBIENT_IS_SET is broken\n".as_ptr());
        return 1;
    }

    if prctl(PR_CAP_AMBIENT, PR_CAP_AMBIENT_CLEAR_ALL, 0, 0, 0, 0) != 0 {
        ksft_exit_fail_msg(c"PR_CAP_AMBIENT_CLEAR_ALL - %s\n".as_ptr(), strerror(errno));
    }

    if prctl(PR_CAP_AMBIENT, PR_CAP_AMBIENT_IS_SET, CAP_NET_BIND_SERVICE, 0, 0, 0) != 0 {
        ksft_test_result_fail(c"PR_CAP_AMBIENT_CLEAR_ALL didn't work\n".as_ptr());
        return 1;
    }

    if prctl(PR_CAP_AMBIENT, PR_CAP_AMBIENT_RAISE, CAP_NET_BIND_SERVICE, 0, 0, 0) != 0 {
        ksft_exit_fail_msg(c"PR_CAP_AMBIENT_RAISE - %s\n".as_ptr(), strerror(errno));
    }

    capng_update(CAPNG_DROP, CAPNG_INHERITABLE, CAP_NET_BIND_SERVICE);
    if capng_apply(CAPNG_SELECT_CAPS) != 0 {
        ksft_exit_fail_msg(c"capng_apply - %s\n".as_ptr(), strerror(errno));
    }

    if prctl(PR_CAP_AMBIENT, PR_CAP_AMBIENT_IS_SET, CAP_NET_BIND_SERVICE, 0, 0, 0) != 0 {
        ksft_test_result_fail(c"Dropping I should have dropped A\n".as_ptr());
        return 1;
    }

    ksft_test_result_pass(c"Basic manipulation appears to work\n".as_ptr());

    capng_update(CAPNG_ADD, CAPNG_INHERITABLE, CAP_NET_BIND_SERVICE);
    if capng_apply(CAPNG_SELECT_CAPS) != 0 {
        ksft_exit_fail_msg(c"capng_apply - %s\n".as_ptr(), strerror(errno));
    }
    if uid == 0 {
        ksft_print_msg(c"[RUN]\tRoot +i => eip\n".as_ptr());
        if fork_wait() {
            exec_validate_cap(true, true, true, false);
        }
    } else {
        ksft_print_msg(c"[RUN]\tNon-root +i => i\n".as_ptr());
        if fork_wait() {
            exec_validate_cap(false, false, true, false);
        }
    }

    if prctl(PR_CAP_AMBIENT, PR_CAP_AMBIENT_RAISE, CAP_NET_BIND_SERVICE, 0, 0, 0) != 0 {
        ksft_exit_fail_msg(c"PR_CAP_AMBIENT_RAISE - %s\n".as_ptr(), strerror(errno));
    }

    ksft_print_msg(c"[RUN]\tUID %d +ia => eipa\n".as_ptr(), uid);
    if fork_wait() {
        exec_validate_cap(true, true, true, true);
    }

    /* The remaining tests need real privilege */

    if !have_outer_privilege {
        ksft_test_result_skip(c"SUID/SGID tests (needs privilege)\n".as_ptr());
        ksft_print_cnts();
        return if nerrs != 0 { 1 } else { 0 };
    }

    if uid == 0 {
        ksft_print_msg(c"[RUN]\tRoot +ia, suidroot => eipa\n".as_ptr());
        if fork_wait() {
            exec_other_validate_cap(c"./validate_cap_suidroot".as_ptr(), true, true, true, true);
        }

        ksft_print_msg(c"[RUN]\tRoot +ia, suidnonroot => ip\n".as_ptr());
        if fork_wait() {
            exec_other_validate_cap(c"./validate_cap_suidnonroot".as_ptr(), false, true, true, false);
        }

        ksft_print_msg(c"[RUN]\tRoot +ia, sgidroot => eipa\n".as_ptr());
        if fork_wait() {
            exec_other_validate_cap(c"./validate_cap_sgidroot".as_ptr(), true, true, true, true);
        }

        if fork_wait() {
            ksft_print_msg(c"[RUN]\tRoot, gid != 0, +ia, sgidroot => eip\n".as_ptr());
            if setresgid(1, 1, 1) != 0 {
                ksft_exit_fail_msg(c"setresgid - %s\n".as_ptr(), strerror(errno));
            }
            exec_other_validate_cap(c"./validate_cap_sgidroot".as_ptr(), true, true, true, false);
        }

        ksft_print_msg(c"[RUN]\tRoot +ia, sgidnonroot => eip\n".as_ptr());
        if fork_wait() {
            exec_other_validate_cap(c"./validate_cap_sgidnonroot".as_ptr(), true, true, true, false);
        }
    } else {
        ksft_print_msg(c"[RUN]\tNon-root +ia, sgidnonroot => i\n".as_ptr());
        if fork_wait() {
            exec_other_validate_cap(c"./validate_cap_sgidnonroot".as_ptr(), false, false, true, false);
        }

        if fork_wait() {
            ksft_print_msg(c"[RUN]\tNon-root +ia, sgidroot => i\n".as_ptr());
            if setresgid(1, 1, 1) != 0 {
                ksft_exit_fail_msg(c"setresgid - %s\n".as_ptr(), strerror(errno));
            }
            exec_other_validate_cap(c"./validate_cap_sgidroot".as_ptr(), false, false, true, false);
        }
    }

    ksft_print_cnts();
    if nerrs != 0 { 1 } else { 0 }
}

unsafe fn main_impl(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let tmp1: *mut c_char;
    let tmp2: *mut c_char;
    let our_path: *mut c_char;

    let _ = argc;

    /* Find our path */
    tmp1 = strdup(*argv.add(0));
    if tmp1.is_null() {
        ksft_exit_fail_msg(c"strdup - %s\n".as_ptr(), strerror(errno));
    }
    tmp2 = dirname(tmp1);
    our_path = strdup(tmp2);
    if our_path.is_null() {
        ksft_exit_fail_msg(c"strdup - %s\n".as_ptr(), strerror(errno));
    }
    free(tmp1 as *mut c_void);

    mpid = getpid();

    if fork_wait() {
        ksft_print_header();
        ksft_set_plan(12);
        ksft_print_msg(c"[RUN]\t+++ Tests with uid == 0 +++\n".as_ptr());
        return do_tests(0, our_path);
    }

    ksft_print_msg(c"==================================================\n".as_ptr());

    if fork_wait() {
        ksft_print_header();
        ksft_set_plan(9);
        ksft_print_msg(c"[RUN]\t+++ Tests with uid != 0 +++\n".as_ptr());
        return do_tests(1, our_path);
    }

    if nerrs != 0 { 1 } else { 0 }
}

fn main() {
    unsafe {
        extern "C" {
            static mut __argc: c_int;
            static mut __argv: *mut *mut c_char;
        }
        core::process::exit(main_impl(__argc, __argv));
    }
}
