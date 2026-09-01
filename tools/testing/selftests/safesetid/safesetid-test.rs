// SPDX-License-Identifier: GPL-2.0
// C dependencies translated as libc-compatible extern declarations.

#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::ptr;

type size_t = usize;
type ssize_t = isize;
type uid_t = c_uint;
type gid_t = c_uint;
type pid_t = c_int;
type FILE = c_void;
type cap_t = *mut c_void;
type cap_value_t = c_int;
type cap_flag_t = c_int;
type cap_flag_value_t = c_int;

const CLONE_NEWUSER: c_int = 0x10000000;

const ROOT_UGID: c_uint = 0;
const RESTRICTED_PARENT_UGID: c_uint = 1;
const ALLOWED_CHILD1_UGID: c_uint = 2;
const ALLOWED_CHILD2_UGID: c_uint = 3;
const NO_POLICY_UGID: c_uint = 4;

const UGID_POLICY_STRING: &[u8] = b"1:2\n1:3\n2:2\n3:3\n\0";

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const O_WRONLY: c_int = 1;
const ENOENT: c_int = 2;
const SEEK_END: c_int = 2;
const WUNTRACED: c_int = 2;
const WCONTINUED: c_int = 8;
const SYS_clone: c_long = 56;
const PR_SET_KEEPCAPS: c_int = 8;
const PR_SET_DUMPABLE: c_int = 4;
const CAP_SETGID: cap_value_t = 6;
const CAP_SETUID: cap_value_t = 7;
const CAP_EFFECTIVE: cap_flag_t = 0;
const CAP_SET: cap_flag_value_t = 1;

#[repr(C)]
struct passwd {
    pw_name: *mut c_char,
    pw_passwd: *mut c_char,
    pw_uid: uid_t,
    pw_gid: gid_t,
    pw_gecos: *mut c_char,
    pw_dir: *mut c_char,
    pw_shell: *mut c_char,
}

#[repr(C)]
struct group {
    gr_name: *mut c_char,
    gr_passwd: *mut c_char,
    gr_gid: gid_t,
    gr_mem: *mut *mut c_char,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn printf(fmt: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, fmt: *const c_char, ...) -> c_int;
    static mut stderr: *mut FILE;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn getpwuid(uid: uid_t) -> *mut passwd;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn fseek(stream: *mut FILE, offset: c_long, whence: c_int) -> c_int;
    fn putpwent(p: *const passwd, stream: *mut FILE) -> c_int;
    fn fclose(stream: *mut FILE) -> c_int;
    fn getgrgid(gid: gid_t) -> *mut group;
    fn putgrent(g: *const group, stream: *mut FILE) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn syscall(number: c_long, ...) -> c_long;
    fn getuid() -> uid_t;
    fn sleep(seconds: c_uint) -> c_uint;
    fn exit(status: c_int) -> !;
    fn fork() -> pid_t;
    fn setuid(uid: uid_t) -> c_int;
    fn getgid() -> gid_t;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
    fn setgid(gid: gid_t) -> c_int;
    fn setgroups(size: size_t, list: *const gid_t) -> c_int;
    fn getgroups(size: c_int, list: *mut gid_t) -> c_int;
    fn cap_get_proc() -> cap_t;
    fn cap_set_flag(
        cap_p: cap_t,
        flag: cap_flag_t,
        ncap: c_int,
        caps: *const cap_value_t,
        value: cap_flag_value_t,
    ) -> c_int;
    fn cap_clear(cap_p: cap_t) -> c_int;
    fn cap_set_proc(cap_p: cap_t) -> c_int;
    fn cap_free(cap_p: *mut c_void) -> c_int;
    fn prctl(option: c_int, ...) -> c_int;
}

static mut add_uid_whitelist_policy_file: *mut c_char =
    b"/sys/kernel/security/safesetid/uid_allowlist_policy\0".as_ptr() as *mut c_char;
static mut add_gid_whitelist_policy_file: *mut c_char =
    b"/sys/kernel/security/safesetid/gid_allowlist_policy\0".as_ptr() as *mut c_char;

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! die {
    ($fmt:literal $(, $arg:expr)* $(,)?) => {{
        unsafe {
            fprintf(stderr, cstr!($fmt) $(, $arg)*);
            exit(EXIT_FAILURE);
        }
    }};
}

unsafe fn vmaybe_write_file(_enoent_ok: bool, _filename: *mut c_char, _fmt: *mut c_char) -> bool {
    /*
     * The C source accepts a va_list and formats through vsnprintf. Rust has no
     * stable source-level equivalent for constructing or forwarding C va_list
     * values here, so write_file below preserves the only local call pattern.
     */
    false
}

unsafe fn write_file(filename: *mut c_char, fmt: *mut c_char, value: c_uint) -> bool {
    let mut buf = [0 as c_char; 4096];
    let fd: c_int;
    let written: ssize_t;
    let buf_len: c_int;

    buf_len = snprintf(buf.as_mut_ptr(), buf.len(), fmt, value);
    if buf_len < 0 {
        printf(cstr!("vsnprintf failed: %s\n"), strerror(errno));
        return false;
    }
    if buf_len as usize >= buf.len() {
        printf(cstr!("vsnprintf output truncated\n"));
        return false;
    }

    fd = open(filename, O_WRONLY);
    if fd < 0 {
        return false;
    }
    written = write(fd, buf.as_ptr() as *const c_void, buf_len as size_t);
    if written != buf_len as ssize_t {
        if written >= 0 {
            printf(cstr!("short write to %s\n"), filename);
            return false;
        } else {
            printf(cstr!("write to %s failed: %s\n"), filename, strerror(errno));
            return false;
        }
    }
    if close(fd) != 0 {
        printf(cstr!("close of %s failed: %s\n"), filename, strerror(errno));
        return false;
    }
    true
}

unsafe fn ensure_user_exists(uid: uid_t) {
    let mut p: passwd = core::mem::zeroed();
    let fd: *mut FILE;
    let mut name_str = [0 as c_char; 10];

    if getpwuid(uid).is_null() {
        fd = fopen(cstr!("/etc/passwd"), cstr!("a"));
        if fd.is_null() {
            die!("couldn't open file\n");
        }
        if fseek(fd, 0, SEEK_END) != 0 {
            die!("couldn't fseek\n");
        }
        snprintf(name_str.as_mut_ptr(), 10, cstr!("user %d"), uid);
        p.pw_name = name_str.as_mut_ptr();
        p.pw_uid = uid;
        p.pw_gid = uid;
        p.pw_gecos = cstr!("Test account") as *mut c_char;
        p.pw_dir = cstr!("/dev/null") as *mut c_char;
        p.pw_shell = cstr!("/bin/false") as *mut c_char;
        let value = putpwent(&p, fd);
        if value != 0 {
            die!("putpwent failed\n");
        }
        if fclose(fd) != 0 {
            die!("fclose failed\n");
        }
    }
}

unsafe fn ensure_group_exists(gid: gid_t) {
    let mut g: group = core::mem::zeroed();
    let fd: *mut FILE;
    let mut name_str = [0 as c_char; 10];

    if getgrgid(gid).is_null() {
        fd = fopen(cstr!("/etc/group"), cstr!("a"));
        if fd.is_null() {
            die!("couldn't open group file\n");
        }
        if fseek(fd, 0, SEEK_END) != 0 {
            die!("couldn't fseek group file\n");
        }
        snprintf(name_str.as_mut_ptr(), 10, cstr!("group %d"), gid);
        g.gr_name = name_str.as_mut_ptr();
        g.gr_gid = gid;
        g.gr_passwd = ptr::null_mut();
        g.gr_mem = ptr::null_mut();
        let value = putgrent(&g, fd);
        if value != 0 {
            die!("putgrent failed\n");
        }
        if fclose(fd) != 0 {
            die!("fclose failed\n");
        }
    }
}

unsafe fn ensure_securityfs_mounted() {
    let fd = open(add_uid_whitelist_policy_file, O_WRONLY);
    if fd < 0 {
        if errno == ENOENT {
            // Need to mount securityfs
            if mount(
                cstr!("securityfs"),
                cstr!("/sys/kernel/security"),
                cstr!("securityfs"),
                0,
                ptr::null(),
            ) < 0
            {
                die!("mounting securityfs failed\n");
            }
        } else {
            die!("couldn't find securityfs for unknown reason\n");
        }
    } else if close(fd) != 0 {
        die!(
            "close of %s failed: %s\n",
            add_uid_whitelist_policy_file,
            strerror(errno)
        );
    }
}

unsafe fn write_uid_policies() {
    let policy_str = UGID_POLICY_STRING.as_ptr() as *mut c_char;
    let written: ssize_t;
    let fd: c_int;

    fd = open(add_uid_whitelist_policy_file, O_WRONLY);
    if fd < 0 {
        die!("can't open add_uid_whitelist_policy file\n");
    }
    written = write(fd, policy_str as *const c_void, strlen(policy_str));
    if written != strlen(policy_str) as ssize_t {
        if written >= 0 {
            die!("short write to %s\n", add_uid_whitelist_policy_file);
        } else {
            die!(
                "write to %s failed: %s\n",
                add_uid_whitelist_policy_file,
                strerror(errno)
            );
        }
    }
    if close(fd) != 0 {
        die!(
            "close of %s failed: %s\n",
            add_uid_whitelist_policy_file,
            strerror(errno)
        );
    }
}

unsafe fn write_gid_policies() {
    let policy_str = UGID_POLICY_STRING.as_ptr() as *mut c_char;
    let written: ssize_t;
    let fd: c_int;

    fd = open(add_gid_whitelist_policy_file, O_WRONLY);
    if fd < 0 {
        die!("can't open add_gid_whitelist_policy file\n");
    }
    written = write(fd, policy_str as *const c_void, strlen(policy_str));
    if written != strlen(policy_str) as ssize_t {
        if written >= 0 {
            die!("short write to %s\n", add_gid_whitelist_policy_file);
        } else {
            die!(
                "write to %s failed: %s\n",
                add_gid_whitelist_policy_file,
                strerror(errno)
            );
        }
    }
    if close(fd) != 0 {
        die!(
            "close of %s failed: %s\n",
            add_gid_whitelist_policy_file,
            strerror(errno)
        );
    }
}

unsafe fn test_userns(expect_success: bool) -> bool {
    let uid: uid_t;
    let mut map_file_name = [0 as c_char; 32];
    let sz: size_t = core::mem::size_of_val(&map_file_name);
    let cpid: pid_t;
    let success: bool;

    uid = getuid();

    let clone_flags = CLONE_NEWUSER;
    cpid = syscall(SYS_clone, clone_flags, ptr::null_mut::<c_void>()) as pid_t;
    if cpid == -1 {
        printf(cstr!("clone failed"));
        return false;
    }

    if cpid == 0 {
        /* Code executed by child */
        // Give parent 1 second to write map file
        sleep(1);
        exit(EXIT_SUCCESS);
    } else {
        /* Code executed by parent */
        if snprintf(
            map_file_name.as_mut_ptr(),
            sz,
            cstr!("/proc/%d/uid_map"),
            cpid,
        ) < 0
        {
            printf(cstr!("preparing file name string failed"));
            return false;
        }
        success = write_file(map_file_name.as_mut_ptr(), cstr!("0 %d 1") as *mut c_char, uid);
        return success == expect_success;
    }
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

unsafe fn test_setuid(child_uid: uid_t, expect_success: bool) {
    let cpid: pid_t;
    let mut w: pid_t;
    let mut wstatus: c_int = 0;

    cpid = fork();
    if cpid == -1 {
        die!("fork\n");
    }

    if cpid == 0 {
        /* Code executed by child */
        if setuid(child_uid) < 0 {
            exit(EXIT_FAILURE);
        }
        if getuid() == child_uid {
            exit(EXIT_SUCCESS);
        } else {
            exit(EXIT_FAILURE);
        }
    } else {
        /* Code executed by parent */
        loop {
            w = waitpid(cpid, &mut wstatus, WUNTRACED | WCONTINUED);
            if w == -1 {
                die!("waitpid\n");
            }

            if wifexited(wstatus) {
                if wexitstatus(wstatus) == EXIT_SUCCESS {
                    if expect_success {
                        return;
                    } else {
                        die!("unexpected success\n");
                    }
                } else if expect_success {
                    die!("unexpected failure\n");
                } else {
                    return;
                }
            } else if wifsignaled(wstatus) {
                if wtermsig(wstatus) == 9 {
                    if expect_success {
                        die!("killed unexpectedly\n");
                    } else {
                        return;
                    }
                } else {
                    die!("unexpected signal: %d\n", wstatus);
                }
            } else {
                die!("unexpected status: %d\n", wstatus);
            }

            if wifexited(wstatus) || wifsignaled(wstatus) {
                break;
            }
        }
    }

    die!("should not reach here\n");
}

unsafe fn test_setgid(child_gid: gid_t, expect_success: bool) {
    let cpid: pid_t;
    let mut w: pid_t;
    let mut wstatus: c_int = 0;

    cpid = fork();
    if cpid == -1 {
        die!("fork\n");
    }

    if cpid == 0 {
        /* Code executed by child */
        if setgid(child_gid) < 0 {
            exit(EXIT_FAILURE);
        }
        if getgid() == child_gid {
            exit(EXIT_SUCCESS);
        } else {
            exit(EXIT_FAILURE);
        }
    } else {
        /* Code executed by parent */
        loop {
            w = waitpid(cpid, &mut wstatus, WUNTRACED | WCONTINUED);
            if w == -1 {
                die!("waitpid\n");
            }

            if wifexited(wstatus) {
                if wexitstatus(wstatus) == EXIT_SUCCESS {
                    if expect_success {
                        return;
                    } else {
                        die!("unexpected success\n");
                    }
                } else if expect_success {
                    die!("unexpected failure\n");
                } else {
                    return;
                }
            } else if wifsignaled(wstatus) {
                if wtermsig(wstatus) == 9 {
                    if expect_success {
                        die!("killed unexpectedly\n");
                    } else {
                        return;
                    }
                } else {
                    die!("unexpected signal: %d\n", wstatus);
                }
            } else {
                die!("unexpected status: %d\n", wstatus);
            }

            if wifexited(wstatus) || wifsignaled(wstatus) {
                break;
            }
        }
    }

    die!("should not reach here\n");
}

unsafe fn test_setgroups(child_groups: *mut gid_t, len: size_t, expect_success: bool) {
    let cpid: pid_t;
    let mut w: pid_t;
    let mut wstatus: c_int = 0;
    let mut groupset = vec![0 as gid_t; len];
    let mut i: c_int;
    let mut j: c_int;

    cpid = fork();
    if cpid == -1 {
        die!("fork\n");
    }

    if cpid == 0 {
        /* Code executed by child */
        if setgroups(len, child_groups) != 0 {
            exit(EXIT_FAILURE);
        }
        if getgroups(len as c_int, groupset.as_mut_ptr()) != len as c_int {
            exit(EXIT_FAILURE);
        }
        i = 0;
        while (i as size_t) < len {
            j = 0;
            while (j as size_t) < len {
                if *child_groups.add(i as usize) == groupset[j as usize] {
                    break;
                }
                if j as size_t == len - 1 {
                    exit(EXIT_FAILURE);
                }
                j += 1;
            }
            i += 1;
        }
        exit(EXIT_SUCCESS);
    } else {
        /* Code executed by parent */
        loop {
            w = waitpid(cpid, &mut wstatus, WUNTRACED | WCONTINUED);
            if w == -1 {
                die!("waitpid\n");
            }

            if wifexited(wstatus) {
                if wexitstatus(wstatus) == EXIT_SUCCESS {
                    if expect_success {
                        return;
                    } else {
                        die!("unexpected success\n");
                    }
                } else if expect_success {
                    die!("unexpected failure\n");
                } else {
                    return;
                }
            } else if wifsignaled(wstatus) {
                if wtermsig(wstatus) == 9 {
                    if expect_success {
                        die!("killed unexpectedly\n");
                    } else {
                        return;
                    }
                } else {
                    die!("unexpected signal: %d\n", wstatus);
                }
            } else {
                die!("unexpected status: %d\n", wstatus);
            }

            if wifexited(wstatus) || wifsignaled(wstatus) {
                break;
            }
        }
    }

    die!("should not reach here\n");
}

unsafe fn ensure_users_exist() {
    ensure_user_exists(ROOT_UGID);
    ensure_user_exists(RESTRICTED_PARENT_UGID);
    ensure_user_exists(ALLOWED_CHILD1_UGID);
    ensure_user_exists(ALLOWED_CHILD2_UGID);
    ensure_user_exists(NO_POLICY_UGID);
}

unsafe fn ensure_groups_exist() {
    ensure_group_exists(ROOT_UGID);
    ensure_group_exists(RESTRICTED_PARENT_UGID);
    ensure_group_exists(ALLOWED_CHILD1_UGID);
    ensure_group_exists(ALLOWED_CHILD2_UGID);
    ensure_group_exists(NO_POLICY_UGID);
}

unsafe fn drop_caps(setid_retained: bool) {
    let cap_values: [cap_value_t; 2] = [CAP_SETUID, CAP_SETGID];
    let caps: cap_t;

    caps = cap_get_proc();
    if setid_retained {
        cap_set_flag(caps, CAP_EFFECTIVE, 2, cap_values.as_ptr(), CAP_SET);
    } else {
        cap_clear(caps);
    }
    cap_set_proc(caps);
    cap_free(caps);
}

fn main() {
    unsafe {
        ensure_groups_exist();
        ensure_users_exist();
        ensure_securityfs_mounted();
        write_uid_policies();
        write_gid_policies();

        if prctl(PR_SET_KEEPCAPS, 1 as c_long) != 0 {
            die!("Error with set keepcaps\n");
        }

        // First test to make sure we can write userns mappings from a non-root
        // user that doesn't have any restrictions (as long as it has
        // CAP_SETUID);
        if setgid(NO_POLICY_UGID) < 0 {
            die!("Error with set gid(%d)\n", NO_POLICY_UGID);
        }
        if setuid(NO_POLICY_UGID) < 0 {
            die!("Error with set uid(%d)\n", NO_POLICY_UGID);
        }
        // Take away all but setid caps
        drop_caps(true);
        // Need PR_SET_DUMPABLE flag set so we can write /proc/[pid]/uid_map
        // from non-root parent process.
        if prctl(PR_SET_DUMPABLE, 1, 0, 0, 0) != 0 {
            die!("Error with set dumpable\n");
        }
        if !test_userns(true) {
            die!("test_userns failed when it should work\n");
        }

        // Now switch to a user/group with restrictions
        if setgid(RESTRICTED_PARENT_UGID) < 0 {
            die!("Error with set gid(%d)\n", RESTRICTED_PARENT_UGID);
        }
        if setuid(RESTRICTED_PARENT_UGID) < 0 {
            die!("Error with set uid(%d)\n", RESTRICTED_PARENT_UGID);
        }

        test_setuid(ROOT_UGID, false);
        test_setuid(ALLOWED_CHILD1_UGID, true);
        test_setuid(ALLOWED_CHILD2_UGID, true);
        test_setuid(NO_POLICY_UGID, false);

        test_setgid(ROOT_UGID, false);
        test_setgid(ALLOWED_CHILD1_UGID, true);
        test_setgid(ALLOWED_CHILD2_UGID, true);
        test_setgid(NO_POLICY_UGID, false);

        let mut allowed_supp_groups: [gid_t; 2] = [ALLOWED_CHILD1_UGID, ALLOWED_CHILD2_UGID];
        let mut disallowed_supp_groups: [gid_t; 2] = [ROOT_UGID, NO_POLICY_UGID];
        test_setgroups(allowed_supp_groups.as_mut_ptr(), 2, true);
        test_setgroups(disallowed_supp_groups.as_mut_ptr(), 2, false);

        if !test_userns(false) {
            die!("test_userns worked when it should fail\n");
        }

        // Now take away all caps
        drop_caps(false);
        test_setuid(2, false);
        test_setuid(3, false);
        test_setuid(4, false);
        test_setgid(2, false);
        test_setgid(3, false);
        test_setgid(4, false);

        // NOTE: this test doesn't clean up users that were created in
        // /etc/passwd or flush policies that were added to the LSM.
        printf(cstr!("test successful!\n"));
        exit(EXIT_SUCCESS);
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
