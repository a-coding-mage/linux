// SPDX-License-Identifier: GPL-2.0
/*
 * Test the 'D' (register disabled) flag of binfmt_misc. An entry
 * registered with it exists but cannot be matched until userspace enables
 * it, which splits a registration into create and activate.
 *
 * Needs root for the registration; no bpf toolchain involved.
 */
/* C source defined _GNU_SOURCE and included:
 * <stdio.h>, <stdlib.h>, "binfmt_misc_common.h", "kselftest_harness.h"
 */

const MAGIC: &str = "#DISABLED-SELFTEST#";
const TARGET_PATH: &str = "/tmp/binfmt_disabled_target";
const INTERP_PATH: &str = "/tmp/binfmt_disabled_interp.sh";
const ENTRY: &str = "test_disabled";

fn RULE(flags: &str) -> String {
    format!(":{}:M:0:{}::{}:{}", ENTRY, MAGIC, INTERP_PATH, flags)
}

/* The interpreter exits with a code the harness can recognise. */
const EXIT_INTERP: i32 = 7;

/* External items supplied by the translated includes or libc. */
extern "C" {
    fn unlink(pathname: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    fn open(pathname: *const ::std::os::raw::c_char, flags: ::std::os::raw::c_int, ...) -> ::std::os::raw::c_int;
    fn write(
        fd: ::std::os::raw::c_int,
        buf: *const ::std::os::raw::c_void,
        count: usize,
    ) -> isize;
    fn close(fd: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    fn getuid() -> ::std::os::raw::c_uint;
    fn access(pathname: *const ::std::os::raw::c_char, mode: ::std::os::raw::c_int) -> ::std::os::raw::c_int;

    static mut errno: ::std::os::raw::c_int;

    fn binfmt_misc_available() -> bool;
    fn binfmt_flag_supported(flag: ::std::os::raw::c_char) -> bool;
    fn write_reg(rule: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    fn entry_shows(entry: *const ::std::os::raw::c_char, needle: *const ::std::os::raw::c_char) -> bool;
    fn run_payload(path: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
    fn entry_command(
        entry: *const ::std::os::raw::c_char,
        command: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    fn unregister(entry: *const ::std::os::raw::c_char);
}

const O_WRONLY: ::std::os::raw::c_int = 1;
const O_CREAT: ::std::os::raw::c_int = 0o100;
const O_EXCL: ::std::os::raw::c_int = 0o200;
const O_CLOEXEC: ::std::os::raw::c_int = 0o2000000;
const F_OK: ::std::os::raw::c_int = 0;
const EINVAL: ::std::os::raw::c_int = 22;

/* Supplied by binfmt_misc_common.h in the original source. */
const BINFMT_DIR: &str = "/proc/sys/fs/binfmt_misc";
const RUN_ENOEXEC: ::std::os::raw::c_int = 126;

fn c_string(s: &str) -> ::std::ffi::CString {
    ::std::ffi::CString::new(s).unwrap()
}

/* The target only has to carry the magic; it is never actually loaded. */
unsafe fn create_target() -> ::std::os::raw::c_int {
    let buf = [MAGIC.as_bytes(), b"\n"].concat();
    let target_path = c_string(TARGET_PATH);
    let fd: ::std::os::raw::c_int;

    unlink(target_path.as_ptr());
    fd = open(
        target_path.as_ptr(),
        O_WRONLY | O_CREAT | O_EXCL,
        0o755 as ::std::os::raw::c_int,
    );
    if fd < 0 {
        return -1;
    }
    if write(fd, buf.as_ptr() as *const ::std::os::raw::c_void, buf.len()) != buf.len() as isize {
        close(fd);
        return -1;
    }
    close(fd);
    0
}

unsafe fn create_interp() -> ::std::os::raw::c_int {
    let buf = format!("#!/bin/sh\nexit {}\n", EXIT_INTERP);
    let interp_path = c_string(INTERP_PATH);
    let fd: ::std::os::raw::c_int;

    unlink(interp_path.as_ptr());
    fd = open(
        interp_path.as_ptr(),
        O_WRONLY | O_CREAT | O_EXCL,
        0o755 as ::std::os::raw::c_int,
    );
    if fd < 0 {
        return -1;
    }
    if write(fd, buf.as_ptr() as *const ::std::os::raw::c_void, buf.len()) != buf.len() as isize {
        close(fd);
        return -1;
    }
    close(fd)
}

FIXTURE!(disabled, {});

FIXTURE_SETUP!(disabled, {
    unsafe {
        if getuid() != 0 {
            SKIP!(return, "test must be run as root");
        }
        if !binfmt_misc_available() {
            SKIP!(return, "no binfmt_misc");
        }

        /* Skip the whole suite on a kernel that does not know 'D'. */
        if !binfmt_flag_supported('D' as ::std::os::raw::c_char) {
            ASSERT_EQ!(errno, EINVAL);
            SKIP!(return, "kernel without the 'D' flag");
        }

        ASSERT_EQ!(create_interp(), 0);
        ASSERT_EQ!(create_target(), 0);
    }
});

FIXTURE_TEARDOWN!(disabled, {
    unsafe {
        let entry = c_string(ENTRY);
        let target_path = c_string(TARGET_PATH);
        let interp_path = c_string(INTERP_PATH);

        unregister(entry.as_ptr());
        unlink(target_path.as_ptr());
        unlink(interp_path.as_ptr());
    }
});

/* The entry exists but does not dispatch until it is enabled. */
TEST_F!(disabled, inert_until_enabled, {
    unsafe {
        let rule = c_string(&RULE("D"));
        let entry = c_string(ENTRY);
        let target_path = c_string(TARGET_PATH);
        let disabled = c_string("disabled");
        let enabled = c_string("enabled");
        let one_newline = c_string("1\n");

        ASSERT_EQ!(write_reg(rule.as_ptr()), 0);
        EXPECT_TRUE!(entry_shows(entry.as_ptr(), disabled.as_ptr()));

        /* Nothing matches it, so no binary format claims the target. */
        EXPECT_EQ!(run_payload(target_path.as_ptr()), RUN_ENOEXEC);

        ASSERT_EQ!(entry_command(entry.as_ptr(), one_newline.as_ptr()), 0);
        EXPECT_TRUE!(entry_shows(entry.as_ptr(), enabled.as_ptr()));
        EXPECT_EQ!(run_payload(target_path.as_ptr()), EXIT_INTERP);
    }
});

/* Without 'D' an entry is matchable the moment it is registered. */
TEST_F!(disabled, enabled_without_the_flag, {
    unsafe {
        let rule = c_string(&RULE(""));
        let entry = c_string(ENTRY);
        let target_path = c_string(TARGET_PATH);
        let enabled = c_string("enabled");

        ASSERT_EQ!(write_reg(rule.as_ptr()), 0);
        EXPECT_TRUE!(entry_shows(entry.as_ptr(), enabled.as_ptr()));
        EXPECT_EQ!(run_payload(target_path.as_ptr()), EXIT_INTERP);
    }
});

/* 'D' is spent on the registration: the entry does not report it back. */
TEST_F!(disabled, flag_not_reported, {
    unsafe {
        let rule = c_string(&RULE("D"));
        let entry = c_string(ENTRY);
        let flags_d = c_string("flags: D");
        let flags = c_string("flags: ");

        ASSERT_EQ!(write_reg(rule.as_ptr()), 0);
        EXPECT_FALSE!(entry_shows(entry.as_ptr(), flags_d.as_ptr()));
        EXPECT_TRUE!(entry_shows(entry.as_ptr(), flags.as_ptr()));
    }
});

/* A disabled entry can be disabled and enabled like any other. */
TEST_F!(disabled, toggles_like_any_entry, {
    unsafe {
        let rule = c_string(&RULE("D"));
        let entry = c_string(ENTRY);
        let target_path = c_string(TARGET_PATH);
        let one_newline = c_string("1\n");
        let zero_newline = c_string("0\n");

        ASSERT_EQ!(write_reg(rule.as_ptr()), 0);

        ASSERT_EQ!(entry_command(entry.as_ptr(), one_newline.as_ptr()), 0);
        ASSERT_EQ!(run_payload(target_path.as_ptr()), EXIT_INTERP);
        ASSERT_EQ!(entry_command(entry.as_ptr(), zero_newline.as_ptr()), 0);
        EXPECT_EQ!(run_payload(target_path.as_ptr()), RUN_ENOEXEC);
        ASSERT_EQ!(entry_command(entry.as_ptr(), one_newline.as_ptr()), 0);
        EXPECT_EQ!(run_payload(target_path.as_ptr()), EXIT_INTERP);
    }
});

/* 'D' composes with the invocation flags a static entry can carry. */
TEST_F!(disabled, composes_with_invocation_flags, {
    unsafe {
        let rule = c_string(&RULE("PD"));
        let entry = c_string(ENTRY);
        let disabled = c_string("disabled");
        let flags_p = c_string("flags: P");

        ASSERT_EQ!(write_reg(rule.as_ptr()), 0);
        EXPECT_TRUE!(entry_shows(entry.as_ptr(), disabled.as_ptr()));
        EXPECT_TRUE!(entry_shows(entry.as_ptr(), flags_p.as_ptr()));
    }
});

/* '-1' to the status file sweeps a staged entry with everything else. */
TEST_F!(disabled, removed_by_remove_all, {
    unsafe {
        let fd: ::std::os::raw::c_int;
        let rule = c_string(&RULE("D"));
        let entry = c_string(ENTRY);
        let disabled = c_string("disabled");
        let status_path = c_string(&format!("{}/status", BINFMT_DIR));
        let entry_path = c_string(&format!("{}/{}", BINFMT_DIR, ENTRY));

        ASSERT_EQ!(write_reg(rule.as_ptr()), 0);
        EXPECT_TRUE!(entry_shows(entry.as_ptr(), disabled.as_ptr()));

        fd = open(status_path.as_ptr(), O_WRONLY | O_CLOEXEC);
        ASSERT_GE!(fd, 0);
        ASSERT_EQ!(write(fd, b"-1".as_ptr() as *const ::std::os::raw::c_void, 2), 2);
        close(fd);

        EXPECT_NE!(access(entry_path.as_ptr(), F_OK), 0);
    }
});

/* A file handle held across a removal cannot resurrect the entry. */
TEST_F!(disabled, no_resurrection_after_remove, {
    unsafe {
        let fd: ::std::os::raw::c_int;
        let rule = c_string(&RULE("D"));
        let entry_path = c_string(&format!("{}/{}", BINFMT_DIR, ENTRY));
        let target_path = c_string(TARGET_PATH);

        ASSERT_EQ!(write_reg(rule.as_ptr()), 0);
        fd = open(entry_path.as_ptr(), O_WRONLY | O_CLOEXEC);
        ASSERT_GE!(fd, 0);

        ASSERT_EQ!(write(fd, b"-1".as_ptr() as *const ::std::os::raw::c_void, 2), 2);
        EXPECT_NE!(access(entry_path.as_ptr(), F_OK), 0);

        /* Accepted like any toggle of a removed entry, but publishes nothing. */
        EXPECT_EQ!(write(fd, b"1".as_ptr() as *const ::std::os::raw::c_void, 1), 1);
        EXPECT_EQ!(run_payload(target_path.as_ptr()), RUN_ENOEXEC);
        close(fd);
    }
});

TEST_HARNESS_MAIN!();
