// SPDX-License-Identifier: GPL-2.0
/*
 * Test the static transparent flag 'T' of binfmt_misc. A magic-matched
 * binary is dispatched to an interpreter with the argument vector left
 * untouched, the binary passed through AT_EXECFD and mm->exe_file labeled
 * with the binary. The asserting interpreter (binfmt_transparent_interp)
 * verifies the constructed identity from inside the process and exits 0.
 *
 * Needs root for the registration; no bpf toolchain involved.
 */

// C dependencies: stdio.h, stdlib.h, binfmt_misc_common.h,
// kselftest_harness.h. The original file also defines _GNU_SOURCE.

use core::ffi::{c_char, c_int, c_uint, c_void};

const MAGIC: &[u8] = b"#TRANSPARENT-SELFTEST#";
const TARGET_PATH: *const c_char = b"/tmp/binfmt_transparent_target\0".as_ptr() as *const c_char;
const INTERP_PATH: *const c_char = b"/tmp/binfmt_transparent_interp\0".as_ptr() as *const c_char;
const ENTRY: *const c_char = b"test_transparent\0".as_ptr() as *const c_char;

const PATH_MAX: usize = 4096;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_EXCL: c_int = 0o200;
const EINVAL: c_int = 22;

type SsizeT = isize;
type SizeT = usize;
type ModeT = c_uint;

unsafe extern "C" {
    static mut errno: c_int;

    fn unlink(pathname: *const c_char) -> c_int;
    fn open(pathname: *const c_char, flags: c_int, mode: ModeT) -> c_int;
    fn write(fd: c_int, buf: *const c_void, count: SizeT) -> SsizeT;
    fn close(fd: c_int) -> c_int;
    fn getuid() -> c_uint;
    fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;

    fn binfmt_misc_available() -> bool;
    fn artifact_path(buf: *mut c_char, size: SizeT, name: *const c_char) -> c_int;
    fn copy_file(src: *const c_char, dst: *const c_char) -> c_int;
    fn binfmt_flag_supported(flag: c_char) -> bool;
    fn r#unregister(entry: *const c_char) -> c_int;
    fn write_reg(rule: *const c_char) -> c_int;
    fn run_payload(path: *const c_char) -> c_int;

    static PAYLOAD_ARGV0: *const c_char;
}

unsafe fn rule(flags: *const c_char) -> *mut c_char {
    const PREFIX: &[u8] = b":test_transparent:M:0:#TRANSPARENT-SELFTEST#::/tmp/binfmt_transparent_interp:";
    let prefix_len = PREFIX.len();
    let mut flags_len = 0usize;

    while *flags.add(flags_len) != 0 {
        flags_len += 1;
    }

    let total_len = prefix_len + flags_len + 1;
    let rule = malloc(total_len) as *mut c_char;
    if rule.is_null() {
        return rule;
    }

    core::ptr::copy_nonoverlapping(PREFIX.as_ptr() as *const c_char, rule, prefix_len);
    core::ptr::copy_nonoverlapping(flags, rule.add(prefix_len), flags_len);
    *rule.add(prefix_len + flags_len) = 0;
    rule
}

unsafe extern "C" {
    fn malloc(size: SizeT) -> *mut c_void;
    fn free(ptr: *mut c_void);
}

/* The target only has to carry the magic; it is never actually loaded. */
unsafe fn create_target() -> c_int {
    let buf: [u8; 128] = {
        let mut tmp = [0u8; 128];
        let mut i = 0usize;
        while i < MAGIC.len() {
            tmp[i] = MAGIC[i];
            i += 1;
        }
        tmp[MAGIC.len()] = b'\n';
        tmp
    };
    let fd: c_int;

    unlink(TARGET_PATH);
    fd = open(TARGET_PATH, O_WRONLY | O_CREAT | O_EXCL, 0o755);
    if fd < 0 {
        return -1;
    }
    if write(fd, buf.as_ptr() as *const c_void, buf.len()) != buf.len() as SsizeT {
        close(fd);
        return -1;
    }
    close(fd);
    0
}

#[repr(C)]
struct transparent {}

unsafe fn transparent_setup() {
    let mut src = [0 as c_char; PATH_MAX];

    if getuid() != 0 {
        /* SKIP(return, "test must be run as root"); */
        return;
    }
    if !binfmt_misc_available() {
        /* SKIP(return, "no binfmt_misc"); */
        return;
    }

    assert_eq!(
        artifact_path(
            src.as_mut_ptr(),
            src.len(),
            b"binfmt_transparent_interp\0".as_ptr() as *const c_char,
        ),
        0
    );
    assert_eq!(copy_file(src.as_ptr(), INTERP_PATH), 0);
    assert_eq!(create_target(), 0);

    /* Skip the whole suite on a kernel that does not know 'T'. */
    if !binfmt_flag_supported(b'T' as c_char) {
        assert_eq!(errno, EINVAL);
        /* SKIP(return, "kernel without the 'T' flag"); */
        return;
    }
}

unsafe fn transparent_teardown() {
    r#unregister(ENTRY);
    unlink(TARGET_PATH);
    unlink(INTERP_PATH);
}

/* Grammar sanity check: the same entry without 'T' has to register. */
unsafe fn transparent_plain_entry_registers() {
    let rule_value = rule(b"\0".as_ptr() as *const c_char);
    assert_eq!(write_reg(rule_value), 0);
    free(rule_value as *mut c_void);
}

/* 'T' preserves the whole argv, so combining it with 'P' is rejected. */
unsafe fn transparent_rejects_preserve_argv0() {
    let rule_value = rule(b"TP\0".as_ptr() as *const c_char);
    assert_ne!(write_reg(rule_value), 0);
    assert_eq!(errno, EINVAL);
    free(rule_value as *mut c_void);
}

/* The interpreter asserts the identity the kernel built for it. */
unsafe fn transparent_dispatch() {
    let rule_value = rule(b"T\0".as_ptr() as *const c_char);
    assert_eq!(write_reg(rule_value), 0);
    free(rule_value as *mut c_void);

    setenv(
        b"BINFMT_TEST_BINARY\0".as_ptr() as *const c_char,
        TARGET_PATH,
        1,
    );
    setenv(
        b"BINFMT_TEST_ARGV0\0".as_ptr() as *const c_char,
        PAYLOAD_ARGV0,
        1,
    );
    assert_eq!(run_payload(TARGET_PATH), 0);
}

fn main() {
    // TEST_HARNESS_MAIN
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
