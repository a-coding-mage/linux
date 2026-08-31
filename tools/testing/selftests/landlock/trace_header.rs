// SPDX-License-Identifier: GPL-2.0
/*
 * Landlock trace test helpers
 *
 * Copyright (c) 2026 Cloudflare, Inc.
 */

/* C header dependencies: errno.h, fcntl.h, regex.h, stdbool.h, stdio.h,
 * stdlib.h, string.h, sys/mount.h, sys/stat.h, unistd.h, and
 * kselftest_harness.h.
 */

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};

pub const TRACEFS_ROOT: &str = "/sys/kernel/tracing";
pub const TRACEFS_LANDLOCK_DIR: &str = "/sys/kernel/tracing/events/landlock";
pub const TRACEFS_CREATE_RULESET_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_create_ruleset/enable";
pub const TRACEFS_CREATE_DOMAIN_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_create_domain/enable";
pub const TRACEFS_ENFORCE_DOMAIN_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_enforce_domain/enable";
pub const TRACEFS_ADD_RULE_FS_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_add_rule_fs/enable";
pub const TRACEFS_ADD_RULE_NET_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_add_rule_net/enable";
pub const TRACEFS_CHECK_RULE_FS_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_check_rule_fs/enable";
pub const TRACEFS_CHECK_RULE_NET_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_check_rule_net/enable";
pub const TRACEFS_DENY_ACCESS_FS_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_deny_access_fs/enable";
pub const TRACEFS_DENY_ACCESS_NET_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_deny_access_net/enable";
pub const TRACEFS_DENY_PTRACE_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_deny_ptrace/enable";
pub const TRACEFS_DENY_SCOPE_SIGNAL_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_deny_scope_signal/enable";
pub const TRACEFS_DENY_SCOPE_ABSTRACT_UNIX_SOCKET_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_deny_scope_abstract_unix_socket/enable";
pub const TRACEFS_FREE_DOMAIN_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_free_domain/enable";
pub const TRACEFS_FREE_RULESET_ENABLE: &str =
    "/sys/kernel/tracing/events/landlock/landlock_free_ruleset/enable";
pub const TRACEFS_TRACE: &str = "/sys/kernel/tracing/trace";
pub const TRACEFS_SET_EVENT_PID: &str = "/sys/kernel/tracing/set_event_pid";
pub const TRACEFS_OPTIONS_EVENT_FORK: &str = "/sys/kernel/tracing/options/event-fork";

pub const TRACE_BUFFER_SIZE: usize = 64 * 1024;

/*
 * Trace line prefix: matches the ftrace "trace" file format.  Format: "
 * <task>-<pid> [<cpu>] <flags> <timestamp>: "
 *
 * The task parameter must be a string literal truncated to 15 chars
 * (TASK_COMM_LEN - 1), matching what the kernel stores in task->comm.  The
 * pattern accepts either the expected task name or "<...>" because the ftrace
 * comm cache may evict short-lived processes (e.g., forked children that exit
 * before the trace buffer is read).
 *
 * No unescaped '.' in any REGEX macro; literal dots use '\\.'.
 */
pub fn TRACE_PREFIX(task: &str) -> String {
    format!(
        "^ *\\(<\\.\\.\\.>\\|{}\\)-[0-9]\\+ *\\[[0-9]\\+\\] [^ ]\\+ \\+[0-9]\\+\\.[0-9]\\+: ",
        task
    )
}

/*
 * Task name for events emitted by kworker threads (e.g., free_domain fires from
 * a work queue, not from the test process).
 */
pub const KWORKER_TASK: &str = "kworker/[0-9]\\+:[0-9]\\+";

pub fn REGEX_ADD_RULE_FS(task: &str) -> String {
    TRACE_PREFIX(task)
        + "landlock_add_rule_fs: "
        + "ruleset=[0-9a-f]\\+\\.[0-9]\\+ "
        + "access_rights=[a-z_|]* "
        + "dev=[0-9]\\+:[0-9]\\+ "
        + "ino=[0-9]\\+ "
        + "path=[^ ]\\+$"
}

pub fn REGEX_ADD_RULE_NET(task: &str) -> String {
    TRACE_PREFIX(task)
        + "landlock_add_rule_net: "
        + "ruleset=[0-9a-f]\\+\\.[0-9]\\+ "
        + "access_rights=[a-z_|]* "
        + "port=[0-9]\\+$"
}

pub fn REGEX_CREATE_RULESET(task: &str) -> String {
    TRACE_PREFIX(task)
        + "landlock_create_ruleset: "
        + "ruleset=[0-9a-f]\\+\\.[0-9]\\+ "
        + "handled_fs=[a-z_|]* "
        + "handled_net=[a-z_|]* "
        + "scoped=[a-z_|]*$"
}

pub fn REGEX_CREATE_DOMAIN(task: &str) -> String {
    TRACE_PREFIX(task)
        + "landlock_create_domain: "
        + "domain=[0-9a-f]\\+ "
        + "parent=[0-9a-f]\\+ "
        + "ruleset=[0-9a-f]\\+\\.[0-9]\\+$"
}

pub fn REGEX_CHECK_RULE_FS(task: &str) -> String {
    TRACE_PREFIX(task)
        + "landlock_check_rule_fs: "
        + "domain=[0-9a-f]\\+ "
        + "access_request=[a-z_|]* "
        + "dev=[0-9]\\+:[0-9]\\+ "
        + "ino=[0-9]\\+ "
        + "grants={[a-z_|,]*}$"
}

pub fn REGEX_CHECK_RULE_NET(task: &str) -> String {
    TRACE_PREFIX(task)
        + "landlock_check_rule_net: "
        + "domain=[0-9a-f]\\+ "
        + "access_request=[a-z_|]* "
        + "port=[0-9]\\+ "
        + "grants={[a-z_|,]*}$"
}

pub fn REGEX_DENY_ACCESS_FS(task: &str) -> String {
    TRACE_PREFIX(task)
        + "landlock_deny_access_fs: "
        + "domain=[0-9a-f]\\+ "
        + "same_exec=[01] "
        + "logged=[01] "
        + "blockers=[a-z_|]* "
        + "dev=[0-9]\\+:[0-9]\\+ "
        + "ino=[0-9]\\+ "
        + "path=[^ ]*$"
}

pub fn REGEX_DENY_ACCESS_NET(task: &str) -> String {
    TRACE_PREFIX(task)
        + "landlock_deny_access_net: "
        + "domain=[0-9a-f]\\+ "
        + "same_exec=[01] "
        + "logged=[01] "
        + "blockers=[a-z_|]* "
        + "sport=[0-9]\\+ "
        + "dport=[0-9]\\+$"
}

pub fn REGEX_DENY_PTRACE(task: &str) -> String {
    TRACE_PREFIX(task)
        + "landlock_deny_ptrace: "
        + "domain=[0-9a-f]\\+ "
        + "same_exec=[01] "
        + "logged=[01] "
        + "tracee_domain=[0-9a-f]\\+ "
        + "tracee_pid=[0-9]\\+ "
        + "tracee_comm=[^ ]*$"
}

pub fn REGEX_DENY_SCOPE_SIGNAL(task: &str) -> String {
    TRACE_PREFIX(task)
        + "landlock_deny_scope_signal: "
        + "domain=[0-9a-f]\\+ "
        + "same_exec=[01] "
        + "logged=[01] "
        + "target_domain=[0-9a-f]\\+ "
        + "target_pid=[0-9]\\+ "
        + "target_comm=[^ ]*$"
}

pub fn REGEX_DENY_SCOPE_ABSTRACT_UNIX_SOCKET(task: &str) -> String {
    TRACE_PREFIX(task)
        + "landlock_deny_scope_abstract_unix_socket: "
        + "domain=[0-9a-f]\\+ "
        + "same_exec=[01] "
        + "logged=[01] "
        + "peer_domain=[0-9a-f]\\+ "
        + "peer_pid=[0-9]\\+ "
        + "sun_path=[^ ]*$"
}

pub fn REGEX_FREE_DOMAIN(task: &str) -> String {
    TRACE_PREFIX(task) + "landlock_free_domain: " + "domain=[0-9a-f]\\+ " + "denials=[0-9]\\+$"
}

pub fn REGEX_FREE_RULESET(task: &str) -> String {
    TRACE_PREFIX(task) + "landlock_free_ruleset: " + "ruleset=[0-9a-f]\\+\\.[0-9]\\+$"
}

pub type size_t = usize;
pub type ssize_t = isize;
pub type pid_t = c_int;
pub type __u32 = u32;
pub type __u64 = u64;
pub type cap_t = *mut c_void;
pub type cap_value_t = c_int;

#[repr(C)]
pub struct regex_t {
    _private: [usize; 0],
}

#[repr(C)]
pub struct stat {
    _private: [usize; 0],
}

#[repr(C)]
pub struct __test_metadata {
    _private: [usize; 0],
}

#[repr(C)]
pub struct landlock_ruleset_attr {
    pub handled_access_fs: __u64,
}

#[repr(C)]
pub struct landlock_path_beneath_attr {
    pub allowed_access: __u64,
    pub parent_fd: c_int,
}

unsafe extern "C" {
    static mut errno: c_int;

    fn open(pathname: *const c_char, flags: c_int, ...) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: size_t) -> ssize_t;
    fn read(fd: c_int, buf: *mut c_void, count: size_t) -> ssize_t;
    fn strlen(s: *const c_char) -> size_t;
    fn snprintf(s: *mut c_char, n: size_t, format: *const c_char, ...) -> c_int;
    fn stat(pathname: *const c_char, statbuf: *mut stat) -> c_int;
    fn mount(
        source: *const c_char,
        target: *const c_char,
        filesystemtype: *const c_char,
        mountflags: c_ulong,
        data: *const c_void,
    ) -> c_int;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strstr(haystack: *const c_char, needle: *const c_char) -> *mut c_char;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: size_t,
        pmatch: *mut c_void,
        eflags: c_int,
    ) -> c_int;
    fn regfree(preg: *mut regex_t);
    fn getpid() -> pid_t;
    fn cap_get_proc() -> cap_t;
    fn cap_set_flag(
        cap_p: cap_t,
        flag: c_int,
        ncap: c_int,
        caps: *const cap_value_t,
        value: c_int,
    ) -> c_int;
    fn cap_set_proc(cap_p: cap_t) -> c_int;
    fn cap_free(cap_p: cap_t) -> c_int;
    fn fork() -> pid_t;
    fn waitpid(pid: pid_t, stat_loc: *mut c_int, options: c_int) -> pid_t;
    fn _exit(status: c_int) -> !;
    fn landlock_create_ruleset(
        attr: *const landlock_ruleset_attr,
        size: size_t,
        flags: __u32,
    ) -> c_int;
    fn landlock_add_rule(
        ruleset_fd: c_int,
        rule_type: c_int,
        rule_attr: *const c_void,
        flags: __u32,
    ) -> c_int;
    fn prctl(option: c_int, arg2: c_ulong, arg3: c_ulong, arg4: c_ulong, arg5: c_ulong) -> c_int;
    fn landlock_restrict_self(ruleset_fd: c_int, flags: __u32) -> c_int;
    fn execl(path: *const c_char, arg: *const c_char, ...) -> c_int;
}

pub const O_WRONLY: c_int = 1;
pub const O_RDONLY: c_int = 0;
pub const O_TRUNC: c_int = 0o1000;
pub const O_CLOEXEC: c_int = 0o2000000;
pub const O_DIRECTORY: c_int = 0o200000;
pub const O_PATH: c_int = 0o10000000;
pub const EIO: c_int = 5;
pub const ENOENT: c_int = 2;
pub const EINVAL: c_int = 22;
pub const EPERM: c_int = 1;
pub const CAP_EFFECTIVE: c_int = 0;
pub const CAP_SET: c_int = 1;
pub const CAP_CLEAR: c_int = 0;
pub const CAP_SYS_ADMIN: cap_value_t = 21;
pub const LANDLOCK_RULE_PATH_BENEATH: c_int = 1;
pub const LANDLOCK_ACCESS_FS_EXECUTE: __u64 = 1 << 0;
pub const LANDLOCK_ACCESS_FS_READ_DIR: __u64 = 1 << 3;
pub const PR_SET_NO_NEW_PRIVS: c_int = 38;

fn c_string_bytes(s: &str) -> Vec<c_char> {
    s.as_bytes()
        .iter()
        .copied()
        .chain(core::iter::once(0))
        .map(|b| b as c_char)
        .collect()
}

pub unsafe fn tracefs_write(path: *const c_char, value: *const c_char) -> c_int {
    let fd: c_int;
    let ret: ssize_t;
    let len: size_t = unsafe { strlen(value) };

    fd = unsafe { open(path, O_WRONLY | O_TRUNC | O_CLOEXEC) };
    if fd < 0 {
        return unsafe { -errno };
    }

    ret = unsafe { write(fd, value as *const c_void, len) };
    unsafe {
        close(fd);
    }
    if ret < 0 {
        return unsafe { -errno };
    }
    if ret as size_t != len {
        return -EIO;
    }

    0
}

pub unsafe fn tracefs_write_int(path: *const c_char, value: c_int) -> c_int {
    let mut buf = [0 as c_char; 32];
    let fmt = b"%d\0";

    unsafe {
        snprintf(buf.as_mut_ptr(), buf.len(), fmt.as_ptr() as *const c_char, value);
        tracefs_write(path, buf.as_ptr())
    }
}

pub unsafe fn tracefs_setup() -> c_int {
    let mut st: stat = unsafe { core::mem::zeroed() };
    let tracefs_root = c_string_bytes(TRACEFS_ROOT);

    /* Mount tracefs if not already mounted. */
    if unsafe { stat(tracefs_root.as_ptr(), &mut st) } != 0 {
        let tracefs = b"tracefs\0";
        let ret = unsafe {
            mount(
                tracefs.as_ptr() as *const c_char,
                tracefs_root.as_ptr(),
                tracefs.as_ptr() as *const c_char,
                0,
                core::ptr::null(),
            )
        };

        if ret != 0 {
            return unsafe { -errno };
        }
    }

    /* Verify landlock events are available. */
    let landlock_dir = c_string_bytes(TRACEFS_LANDLOCK_DIR);
    if unsafe { stat(landlock_dir.as_ptr(), &mut st) } != 0 {
        return -ENOENT;
    }

    0
}

/*
 * Set up PID-based event filtering so only events from the current process and
 * its children are recorded.  This is analogous to audit's AUDIT_EXE filter: it
 * prevents events from unrelated processes from polluting the trace buffer.
 */
pub unsafe fn tracefs_set_pid_filter(pid: pid_t) -> c_int {
    let ret: c_int;
    let event_fork = c_string_bytes(TRACEFS_OPTIONS_EVENT_FORK);
    let one = b"1\0";

    /* Enable event-fork so children inherit the PID filter. */
    ret = unsafe { tracefs_write(event_fork.as_ptr(), one.as_ptr() as *const c_char) };
    if ret != 0 {
        return ret;
    }

    let set_event_pid = c_string_bytes(TRACEFS_SET_EVENT_PID);
    unsafe { tracefs_write_int(set_event_pid.as_ptr(), pid) }
}

/* Clear the PID filter to stop filtering by PID. */
pub unsafe fn tracefs_clear_pid_filter() -> c_int {
    let set_event_pid = c_string_bytes(TRACEFS_SET_EVENT_PID);
    let empty = b"\0";
    unsafe { tracefs_write(set_event_pid.as_ptr(), empty.as_ptr() as *const c_char) }
}

pub unsafe fn tracefs_enable_event(enable_path: *const c_char, enable: bool) -> c_int {
    let one = b"1\0";
    let zero = b"0\0";
    unsafe {
        tracefs_write(
            enable_path,
            if enable {
                one.as_ptr() as *const c_char
            } else {
                zero.as_ptr() as *const c_char
            },
        )
    }
}

pub unsafe fn tracefs_clear() -> c_int {
    let trace = c_string_bytes(TRACEFS_TRACE);
    let empty = b"\0";
    unsafe { tracefs_write(trace.as_ptr(), empty.as_ptr() as *const c_char) }
}

/*
 * Reads the trace buffer content into a newly allocated buffer.  The caller is
 * responsible for freeing the returned buffer.  Returns NULL on error.
 */
pub unsafe fn tracefs_read_trace() -> *mut c_char {
    let buf: *mut c_char;
    let fd: c_int;
    let mut total: ssize_t = 0;
    let mut ret: ssize_t;

    buf = unsafe { malloc(TRACE_BUFFER_SIZE) as *mut c_char };
    if buf.is_null() {
        return core::ptr::null_mut();
    }

    let trace = c_string_bytes(TRACEFS_TRACE);
    fd = unsafe { open(trace.as_ptr(), O_RDONLY | O_CLOEXEC) };
    if fd < 0 {
        unsafe {
            free(buf as *mut c_void);
        }
        return core::ptr::null_mut();
    }

    while total < (TRACE_BUFFER_SIZE - 1) as ssize_t {
        ret = unsafe {
            read(
                fd,
                buf.offset(total) as *mut c_void,
                TRACE_BUFFER_SIZE - 1 - total as usize,
            )
        };
        if ret <= 0 {
            break;
        }
        total += ret;
    }
    unsafe {
        close(fd);
        *buf.offset(total) = 0;
    }
    buf
}

/* Counts the number of lines in @buf matching the basic regex @pattern. */
pub unsafe fn tracefs_count_matches(buf: *const c_char, pattern: *const c_char) -> c_int {
    let mut regex: regex_t = unsafe { core::mem::zeroed() };
    let mut count: c_int = 0;
    let mut line: *const c_char;
    let mut end: *const c_char;

    if unsafe { regcomp(&mut regex, pattern, 0) } != 0 {
        return -EINVAL;
    }

    line = buf;
    while unsafe { *line } != 0 {
        end = unsafe { strchr(line, '\n' as c_int) as *const c_char };
        if end.is_null() {
            end = unsafe { line.add(strlen(line)) };
        }

        /* Create a temporary NUL-terminated line. */
        let len: size_t = (end as usize).wrapping_sub(line as usize);
        let tmp = unsafe { malloc(len + 1) as *mut c_char };

        if !tmp.is_null() {
            unsafe {
                memcpy(tmp as *mut c_void, line as *const c_void, len);
                *tmp.add(len) = 0;
                if regexec(&regex, tmp, 0, core::ptr::null_mut(), 0) == 0 {
                    count += 1;
                }
                free(tmp as *mut c_void);
            }
        }

        if unsafe { *end } == '\n' as c_char {
            line = unsafe { end.add(1) };
        } else {
            break;
        }
    }

    unsafe {
        regfree(&mut regex);
    }
    count
}

/*
 * Extracts the value of a named field from a trace line in @buf.  Searches for
 * the first line matching @line_pattern, then extracts the value after
 * "@field_name=" into @out.  Stops at space or newline.
 *
 * Returns 0 on success, -ENOENT if no match.
 */
pub unsafe fn tracefs_extract_field(
    buf: *const c_char,
    line_pattern: *const c_char,
    field_name: *const c_char,
    out: *mut c_char,
    out_size: size_t,
) -> c_int {
    let mut regex: regex_t = unsafe { core::mem::zeroed() };
    let mut line: *const c_char;
    let mut end: *const c_char;

    if unsafe { regcomp(&mut regex, line_pattern, 0) } != 0 {
        return -EINVAL;
    }

    line = buf;
    while unsafe { *line } != 0 {
        end = unsafe { strchr(line, '\n' as c_int) as *const c_char };
        if end.is_null() {
            end = unsafe { line.add(strlen(line)) };
        }

        let len: size_t = (end as usize).wrapping_sub(line as usize);
        let tmp = unsafe { malloc(len + 1) as *mut c_char };

        if !tmp.is_null() {
            let mut field: *const c_char;
            let val_start: *const c_char;
            let field_len: size_t;
            let mut val_len: size_t;

            unsafe {
                memcpy(tmp as *mut c_void, line as *const c_void, len);
                *tmp.add(len) = 0;
            }

            if unsafe { regexec(&regex, tmp, 0, core::ptr::null_mut(), 0) } != 0 {
                unsafe {
                    free(tmp as *mut c_void);
                }
            } else {
                /*
                 * Find "field_name=" in the line, ensuring a word
                 * boundary before the field name to avoid substring
                 * matches (e.g., "port" in "sport").
                 */
                field_len = unsafe { strlen(field_name) };
                field = tmp;
                loop {
                    field = unsafe { strstr(field, field_name) };
                    if field.is_null() {
                        break;
                    }
                    if unsafe { *field.add(field_len) } == '=' as c_char
                        && (field == tmp
                            || unsafe { *field.offset(-1) } == ' ' as c_char)
                    {
                        break;
                    }
                    field = unsafe { field.add(1) };
                }
                if field.is_null() {
                    unsafe {
                        free(tmp as *mut c_void);
                        regfree(&mut regex);
                    }
                    return -ENOENT;
                }

                val_start = unsafe { field.add(field_len + 1) };
                val_len = 0;
                while unsafe { *val_start.add(val_len) } != 0
                    && unsafe { *val_start.add(val_len) } != ' ' as c_char
                    && unsafe { *val_start.add(val_len) } != '\n' as c_char
                {
                    val_len += 1;
                }

                if val_len >= out_size {
                    val_len = out_size - 1;
                }
                unsafe {
                    memcpy(out as *mut c_void, val_start as *const c_void, val_len);
                    *out.add(val_len) = 0;

                    free(tmp as *mut c_void);
                    regfree(&mut regex);
                }
                return 0;
            }
        }

        if unsafe { *end } == '\n' as c_char {
            line = unsafe { end.add(1) };
        } else {
            break;
        }
    }

    unsafe {
        regfree(&mut regex);
    }
    -ENOENT
}

/*
 * Common fixture setup for trace tests.  Mounts tracefs if needed and sets a
 * PID filter.  The caller must create a mount namespace first
 * (unshare(CLONE_NEWNS) + mount(MS_REC | MS_PRIVATE)) to isolate the tracefs
 * mount; the trace buffer, per-event enable flags, and PID filter are global
 * kernel state, scoped to the test by the PID filter.
 *
 * Returns 0 on success, -errno on failure (caller should SKIP).
 */
pub unsafe fn tracefs_fixture_setup() -> c_int {
    let ret: c_int;

    ret = unsafe { tracefs_setup() };
    if ret != 0 {
        return ret;
    }

    unsafe { tracefs_set_pid_filter(getpid()) }
}

pub unsafe fn tracefs_fixture_teardown() {
    unsafe {
        tracefs_clear_pid_filter();
    }
}

/*
 * Temporarily raises CAP_SYS_ADMIN effective capability, calls @func, then
 * drops the capability.  Returns the value from @func, or -EPERM if the
 * capability manipulation fails.
 */
pub unsafe fn tracefs_priv_call(func: Option<unsafe extern "C" fn() -> c_int>) -> c_int {
    let admin: cap_value_t = CAP_SYS_ADMIN;
    let cap_p: cap_t;
    let ret: c_int;

    cap_p = unsafe { cap_get_proc() };
    if cap_p.is_null() {
        return -EPERM;
    }

    if unsafe { cap_set_flag(cap_p, CAP_EFFECTIVE, 1, &admin, CAP_SET) } != 0
        || unsafe { cap_set_proc(cap_p) } != 0
    {
        unsafe {
            cap_free(cap_p);
        }
        return -EPERM;
    }

    ret = unsafe { func.unwrap_unchecked()() };

    unsafe {
        cap_set_flag(cap_p, CAP_EFFECTIVE, 1, &admin, CAP_CLEAR);
        cap_set_proc(cap_p);
        cap_free(cap_p);
    }
    ret
}

/* Read the trace buffer with elevated privileges.  Returns NULL on failure. */
pub unsafe fn tracefs_read_buf() -> *mut c_char {
    /* Cannot use tracefs_priv_call() because the return type is char *. */
    let cap_p: cap_t;
    let buf: *mut c_char;
    let admin: cap_value_t = CAP_SYS_ADMIN;

    cap_p = unsafe { cap_get_proc() };
    if cap_p.is_null() {
        return core::ptr::null_mut();
    }

    if unsafe { cap_set_flag(cap_p, CAP_EFFECTIVE, 1, &admin, CAP_SET) } != 0
        || unsafe { cap_set_proc(cap_p) } != 0
    {
        unsafe {
            cap_free(cap_p);
        }
        return core::ptr::null_mut();
    }

    buf = unsafe { tracefs_read_trace() };

    unsafe {
        cap_set_flag(cap_p, CAP_EFFECTIVE, 1, &admin, CAP_CLEAR);
        cap_set_proc(cap_p);
        cap_free(cap_p);
    }
    buf
}

unsafe extern "C" fn tracefs_clear_trampoline() -> c_int {
    unsafe { tracefs_clear() }
}

/* Clear the trace buffer with elevated privileges.  Returns 0 on success. */
pub unsafe fn tracefs_clear_buf() -> c_int {
    unsafe { tracefs_priv_call(Some(tracefs_clear_trampoline)) }
}

fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

macro_rules! ASSERT_LE {
    ($left:expr, $right:expr) => {
        assert!($left <= $right)
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

macro_rules! ASSERT_TRUE {
    ($expr:expr) => {
        assert!($expr)
    };
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {
        assert_eq!($left, $right)
    };
}

/*
 * Forks a child that creates a Landlock sandbox and performs an FS access.  The
 * parent waits for the child, then reads the trace buffer.
 *
 * Requires common.h and wrappers.h to be included before trace.h.
 */
pub unsafe fn sandbox_child_fs_access(
    _metadata: *mut __test_metadata,
    rule_path: *const c_char,
    handled_access: __u64,
    allowed_access: __u64,
    access_path: *const c_char,
) {
    let pid: pid_t;
    let mut status: c_int = 0;

    pid = unsafe { fork() };
    ASSERT_LE!(0, pid);

    if pid == 0 {
        let ruleset_attr = landlock_ruleset_attr {
            handled_access_fs: handled_access,
        };
        let mut path_beneath = landlock_path_beneath_attr {
            allowed_access,
            parent_fd: 0,
        };
        let ruleset_fd: c_int;
        let fd: c_int;

        ruleset_fd = unsafe {
            landlock_create_ruleset(
                &ruleset_attr,
                core::mem::size_of_val(&ruleset_attr),
                0,
            )
        };
        if ruleset_fd < 0 {
            unsafe { _exit(1) };
        }

        path_beneath.parent_fd =
            unsafe { open(rule_path, O_PATH | O_DIRECTORY | O_CLOEXEC) };
        if path_beneath.parent_fd < 0 {
            unsafe {
                close(ruleset_fd);
                _exit(1);
            }
        }

        if unsafe {
            landlock_add_rule(
                ruleset_fd,
                LANDLOCK_RULE_PATH_BENEATH,
                &path_beneath as *const _ as *const c_void,
                0,
            )
        } != 0
        {
            unsafe {
                close(path_beneath.parent_fd);
                close(ruleset_fd);
                _exit(1);
            }
        }
        unsafe {
            close(path_beneath.parent_fd);

            prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
        }
        if unsafe { landlock_restrict_self(ruleset_fd, 0) } != 0 {
            unsafe {
                close(ruleset_fd);
                _exit(1);
            }
        }
        unsafe {
            close(ruleset_fd);
        }

        fd = unsafe { open(access_path, O_RDONLY | O_DIRECTORY | O_CLOEXEC) };
        if fd >= 0 {
            unsafe {
                close(fd);
            }
        }

        unsafe { _exit(0) };
    }

    ASSERT_EQ!(pid, unsafe { waitpid(pid, &mut status, 0) });
    ASSERT_TRUE!(WIFEXITED(status));
    EXPECT_EQ!(0, WEXITSTATUS(status));
}

/*
 * Forks a child that creates a Landlock sandbox allowing execute+read_dir for
 * /usr and execute-only for ".", then execs ./true.  The true binary opens "."
 * on startup, triggering a read_dir denial with same_exec=0.  The parent waits
 * for the child to exit.
 */
pub unsafe fn sandbox_child_exec_true(_metadata: *mut __test_metadata, restrict_flags: __u32) {
    let pid: pid_t;
    let mut status: c_int = 0;

    pid = unsafe { fork() };
    ASSERT_LE!(0, pid);

    if pid == 0 {
        let attr = landlock_ruleset_attr {
            handled_access_fs: LANDLOCK_ACCESS_FS_READ_DIR | LANDLOCK_ACCESS_FS_EXECUTE,
        };
        let mut path_beneath = landlock_path_beneath_attr {
            allowed_access: LANDLOCK_ACCESS_FS_EXECUTE | LANDLOCK_ACCESS_FS_READ_DIR,
            parent_fd: 0,
        };
        let ruleset_fd: c_int;

        ruleset_fd =
            unsafe { landlock_create_ruleset(&attr, core::mem::size_of_val(&attr), 0) };
        if ruleset_fd < 0 {
            unsafe { _exit(1) };
        }

        path_beneath.parent_fd = unsafe {
            open(
                b"/usr\0".as_ptr() as *const c_char,
                O_PATH | O_DIRECTORY | O_CLOEXEC,
            )
        };
        if path_beneath.parent_fd >= 0 {
            unsafe {
                landlock_add_rule(
                    ruleset_fd,
                    LANDLOCK_RULE_PATH_BENEATH,
                    &path_beneath as *const _ as *const c_void,
                    0,
                );
                close(path_beneath.parent_fd);
            }
        }

        path_beneath.allowed_access = LANDLOCK_ACCESS_FS_EXECUTE;
        path_beneath.parent_fd = unsafe {
            open(
                b".\0".as_ptr() as *const c_char,
                O_PATH | O_DIRECTORY | O_CLOEXEC,
            )
        };
        if path_beneath.parent_fd >= 0 {
            unsafe {
                landlock_add_rule(
                    ruleset_fd,
                    LANDLOCK_RULE_PATH_BENEATH,
                    &path_beneath as *const _ as *const c_void,
                    0,
                );
                close(path_beneath.parent_fd);
            }
        }

        unsafe {
            prctl(PR_SET_NO_NEW_PRIVS, 1, 0, 0, 0);
        }
        if unsafe { landlock_restrict_self(ruleset_fd, restrict_flags) } != 0 {
            unsafe { _exit(1) };
        }
        unsafe {
            close(ruleset_fd);

            execl(
                b"./true\0".as_ptr() as *const c_char,
                b"./true\0".as_ptr() as *const c_char,
                core::ptr::null::<c_char>(),
            );
            _exit(1);
        }
    }

    ASSERT_EQ!(pid, unsafe { waitpid(pid, &mut status, 0) });
    ASSERT_TRUE!(WIFEXITED(status));
    EXPECT_EQ!(0, WEXITSTATUS(status));
}
