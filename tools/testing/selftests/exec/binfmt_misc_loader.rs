// SPDX-License-Identifier: GPL-2.0
/*
 * Test the 'L' (loader substitution) flag of binfmt_misc. A matched
 * binary runs as the MAIN image - a fully native exec - with the
 * registered interpreter substituted for its PT_INTERP. The payload
 * (binfmt_loader_payload) asserts the native identity from inside.
 *
 * The substitute is a copy of the system loader found via our own
 * PT_INTERP; magic matching pokes a marker into the ELF header's
 * e_ident padding, which kernel and loader ignore.
 *
 * Needs root for the registration; no bpf toolchain involved.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

const ENTRY: &str = "test_loader";
const INTERP_PATH: &str = "/tmp/binfmt_loader_interp";
const MOVED_PATH: &str = "/tmp/binfmt_loader_interp.moved";
const TARGET_PATH: &str = "/tmp/binfmt_loader_target.ldrtest";
const STATIC_PATH: &str = "/tmp/binfmt_loader_static.ldrtest";
const FOREIGN_PATH: &str = "/tmp/binfmt_loader_foreign.ldrtest";
const SCRIPT_PATH: &str = "/tmp/binfmt_loader_script.ldrtest";
const M_RULE: &str = ":test_loader:M:9:LOADER_MARKER::/tmp/binfmt_loader_interp:L";
const E_RULE: &str = ":test_loader:E::ldrtest::/tmp/binfmt_loader_interp:L";
const FL_RULE: &str = ":test_loader:E::ldrtest::/tmp/binfmt_loader_interp:FL";

/* External declarations supplied by the translated test harness/common files. */
unsafe extern "C" {
    static mut environ: *mut *mut c_char;

    fn fork() -> pid_t;
    fn memfd_create(name: *const c_char, flags: c_uint) -> c_int;
    fn open(path: *const c_char, flags: c_int, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn close(fd: c_int) -> c_int;
    fn setenv(name: *const c_char, value: *const c_char, overwrite: c_int) -> c_int;
    fn unsetenv(name: *const c_char) -> c_int;
    fn syscall(num: c_long, ...) -> c_long;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn ptrace(request: c_uint, ...) -> c_long;
    fn raise(sig: c_int) -> c_int;
    fn execl(path: *const c_char, arg0: *const c_char, ...) -> c_int;
    fn readlink(path: *const c_char, buf: *mut c_char, bufsiz: usize) -> ssize_t;
    fn snprintf(s: *mut c_char, maxlen: usize, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    static mut stderr: *mut FILE;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn getuid() -> uid_t;
    fn access(path: *const c_char, amode: c_int) -> c_int;
    fn rename(old: *const c_char, new: *const c_char) -> c_int;
    fn unlink(path: *const c_char) -> c_int;
    fn strlen(s: *const c_char) -> usize;

    fn binfmt_misc_available() -> bool;
    fn find_loader(loader: *mut c_char, size: usize) -> c_int;
    fn copy_file(src: *const c_char, dst: *const c_char) -> c_int;
    fn artifact_path(buf: *mut c_char, size: usize, name: *const c_char) -> c_int;
    fn patch_file(path: *const c_char, off: usize, buf: *const c_void, len: usize) -> c_int;
    fn write_reg(rule: *const c_char) -> c_int;
    fn unregister(name: *const c_char);
    fn stat_codes(pid: pid_t, start_code: *mut c_ulong, end_code: *mut c_ulong) -> c_int;
    fn run_payload(path: *const c_char) -> c_int;
    fn __errno_location() -> *mut c_int;
}

type pid_t = c_int;
type uid_t = c_uint;
type ssize_t = isize;
type c_long = i64;
type c_uint = u32;

#[repr(C)]
pub struct FILE {
    _private: [u8; 0],
}

const NULL: *mut c_void = ptr::null_mut();
const MFD_CLOEXEC: c_uint = 0x0001;
const O_RDONLY: c_int = 0;
const O_WRONLY: c_int = 1;
const O_CREAT: c_int = 0o100;
const O_EXCL: c_int = 0o200;
const F_OK: c_int = 0;
const SIGSTOP: c_int = 19;
const SIGTRAP: c_int = 5;
const SIGKILL: c_int = 9;
const EINVAL: c_int = 22;
const SYS_EXECVEAT: c_long = 322;
const AT_EMPTY_PATH: c_int = 0x1000;
const PTRACE_TRACEME: c_uint = 0;
const PTRACE_CONT: c_uint = 7;
const PTRACE_SETOPTIONS: c_uint = 0x4200;
const PTRACE_O_TRACEEXEC: c_ulong = 0x0000_0010;
const PTRACE_EVENT_EXEC: c_int = 4;
const PATH_MAX: usize = 4096;
const AT_BASE: c_ulong = 7;
const AT_ENTRY: c_ulong = 9;
const AT_FLAGS: c_ulong = 8;
const AT_EXECFD: c_ulong = 2;
const EI_PAD: usize = 9;
const RUN_ENOEXEC: c_int = 126;

const PAYLOAD_ARGV0: *const c_char = c"binfmt_loader_payload".as_ptr();
const PAYLOAD_ARG1: *const c_char = c"arg1".as_ptr();
const PAYLOAD_ARG2: *const c_char = c"arg2".as_ptr();
const LOADER_MARKER: *const c_char = c"LOADER_MARKER".as_ptr();

unsafe fn errno() -> c_int {
    *__errno_location()
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

fn wifstopped(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

/* Execute the binary from an inaccessible O_CLOEXEC memfd. */
unsafe fn run_memfd(path: *const c_char) -> c_int {
    let mut status: c_int = 0;
    let pid: pid_t;

    pid = fork();
    if pid == 0 {
        let mut argv: [*mut c_char; 4] = [
            PAYLOAD_ARGV0 as *mut c_char,
            PAYLOAD_ARG1 as *mut c_char,
            PAYLOAD_ARG2 as *mut c_char,
            ptr::null_mut(),
        ];
        let mut buf = [0u8; 4096];
        let in_fd: c_int;
        let mfd: c_int;
        let mut n: ssize_t;

        mfd = memfd_create(c"loader-test".as_ptr(), MFD_CLOEXEC);
        in_fd = open(path, O_RDONLY);
        if mfd < 0 || in_fd < 0 {
            libc_exit(125);
        }
        loop {
            n = read(in_fd, buf.as_mut_ptr() as *mut c_void, size_of_val(&buf));
            if n <= 0 {
                break;
            }
            if write(mfd, buf.as_ptr() as *const c_void, n as usize) != n {
                libc_exit(125);
            }
        }
        close(in_fd);
        setenv(c"BINFMT_TEST_MEMFD".as_ptr(), c"1".as_ptr(), 1);
        unsetenv(c"BINFMT_TEST_BINARY".as_ptr());
        syscall(
            SYS_EXECVEAT,
            mfd,
            c"".as_ptr(),
            argv.as_mut_ptr(),
            environ,
            AT_EMPTY_PATH,
        );
        libc_exit(126);
    }
    if pid < 0 || waitpid(pid, &mut status, 0) != pid || !wifexited(status) {
        return -1;
    }
    wexitstatus(status)
}

unsafe fn libc_exit(status: c_int) -> ! {
    unsafe extern "C" {
        fn _exit(status: c_int) -> !;
    }
    _exit(status)
}

/*
 * The differentiator against the transparent mode: at PTRACE_EVENT_EXEC
 * the identity is already complete - exe, auxv and the stat code markers
 * are mutually consistent with no window a debugger could observe.
 */
unsafe fn ptrace_probe(target: *const c_char) -> c_int {
    let mut auxv = [0 as c_ulong; 2 * 64];
    let mut base: c_ulong = 0;
    let mut entry: c_ulong = 0;
    let mut at_flags: c_ulong = 0;
    let mut start_code: c_ulong = 0;
    let mut end_code: c_ulong = 0;
    let mut status: c_int = 0;
    let mut fd: c_int;
    let mut execfd_seen: c_int = 0;
    let mut failed: c_int = 0;
    let mut path = [0 as c_char; 64];
    let mut buf = [0 as c_char; PATH_MAX];
    let mut n: ssize_t;
    let pid: pid_t;
    let mut i: c_int;

    pid = fork();
    if pid == 0 {
        ptrace(PTRACE_TRACEME, 0, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>());
        raise(SIGSTOP);
        execl(
            target,
            PAYLOAD_ARGV0,
            PAYLOAD_ARG1,
            PAYLOAD_ARG2,
            ptr::null_mut::<c_char>(),
        );
        libc_exit(126);
    }
    if pid < 0 {
        return -1;
    }
    if waitpid(pid, &mut status, 0) != pid || !wifstopped(status) {
        return fail_kill(pid, &mut status);
    }
    if ptrace(
        PTRACE_SETOPTIONS,
        pid,
        ptr::null_mut::<c_void>(),
        PTRACE_O_TRACEEXEC as *mut c_void,
    ) != 0
    {
        return fail_kill(pid, &mut status);
    }
    if ptrace(PTRACE_CONT, pid, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
        return fail_kill(pid, &mut status);
    }
    if waitpid(pid, &mut status, 0) != pid
        || !wifstopped(status)
        || status >> 8 != (SIGTRAP | (PTRACE_EVENT_EXEC << 8))
    {
        fprintf(stderr, c"no exec stop (status %#x)\n".as_ptr(), status);
        return fail_kill(pid, &mut status);
    }

    snprintf(path.as_mut_ptr(), size_of_val(&path), c"/proc/%d/exe".as_ptr(), pid);
    n = readlink(path.as_ptr(), buf.as_mut_ptr(), size_of_val(&buf) - 1);
    if n <= 0 {
        failed = 1;
    } else {
        buf[n as usize] = 0;
        if strcmp(buf.as_ptr(), target) != 0 {
            fprintf(stderr, c"exe at exec stop: %s\n".as_ptr(), buf.as_ptr());
            failed = 1;
        }
    }

    snprintf(path.as_mut_ptr(), size_of_val(&path), c"/proc/%d/auxv".as_ptr(), pid);
    fd = open(path.as_ptr(), O_RDONLY);
    if fd < 0 {
        n = -1;
    } else {
        n = read(fd, auxv.as_mut_ptr() as *mut c_void, size_of_val(&auxv));
        close(fd);
    }
    if n <= 0 {
        failed = 1;
        n = 0;
    }
    i = 0;
    while i + 1 < (n as usize / size_of::<c_ulong>()) as c_int {
        match auxv[i as usize] {
            AT_BASE => base = auxv[i as usize + 1],
            AT_ENTRY => entry = auxv[i as usize + 1],
            AT_FLAGS => at_flags = auxv[i as usize + 1],
            AT_EXECFD => execfd_seen = 1,
            _ => {}
        }
        i += 2;
    }

    if stat_codes(pid, &mut start_code, &mut end_code) != 0 {
        failed = 1;
    }

    if base == 0 || execfd_seen != 0 || at_flags != 0 {
        fprintf(stderr, c"auxv at exec stop not native\n".as_ptr());
        failed = 1;
    }
    if start_code == 0 || entry < start_code || entry >= end_code {
        fprintf(stderr, c"auxv/stat inconsistent at exec stop\n".as_ptr());
        failed = 1;
    }

    if ptrace(PTRACE_CONT, pid, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
        return fail_kill(pid, &mut status);
    }
    if waitpid(pid, &mut status, 0) != pid || !wifexited(status) || wexitstatus(status) != 0 {
        failed = 1;
    }
    if failed != 0 { -1 } else { 0 }
}

unsafe fn fail_kill(pid: pid_t, status: *mut c_int) -> c_int {
    kill(pid, SIGKILL);
    waitpid(pid, status, 0);
    -1
}

#[repr(C)]
struct loader {
    have_static: bool,
}

unsafe fn loader_setup(self_: *mut loader) {
    let mut foreign_machine: u16 = 0xdead;
    let mut src = [0 as c_char; PATH_MAX];
    let mut loader_path = [0 as c_char; PATH_MAX];

    if getuid() != 0 {
        SKIP_return!("test must be run as root");
    }
    if !binfmt_misc_available() {
        SKIP_return!("no binfmt_misc");
    }
    if find_loader(loader_path.as_mut_ptr(), size_of_val(&loader_path)) != 0 {
        SKIP_return!("cannot determine own PT_INTERP");
    }

    ASSERT_EQ!(copy_file(loader_path.as_ptr(), c"/tmp/binfmt_loader_interp".as_ptr()), 0);

    ASSERT_EQ!(
        artifact_path(
            src.as_mut_ptr(),
            size_of_val(&src),
            c"binfmt_loader_payload".as_ptr()
        ),
        0
    );
    ASSERT_EQ!(copy_file(src.as_ptr(), c"/tmp/binfmt_loader_target.ldrtest".as_ptr()), 0);
    ASSERT_EQ!(
        patch_file(
            c"/tmp/binfmt_loader_target.ldrtest".as_ptr(),
            EI_PAD,
            LOADER_MARKER as *const c_void,
            strlen(LOADER_MARKER)
        ),
        0
    );

    /* The same payload with a machine type this kernel cannot load. */
    ASSERT_EQ!(copy_file(src.as_ptr(), c"/tmp/binfmt_loader_foreign.ldrtest".as_ptr()), 0);
    ASSERT_EQ!(
        patch_file(
            c"/tmp/binfmt_loader_foreign.ldrtest".as_ptr(),
            EI_PAD,
            LOADER_MARKER as *const c_void,
            strlen(LOADER_MARKER)
        ),
        0
    );
    ASSERT_EQ!(
        patch_file(
            c"/tmp/binfmt_loader_foreign.ldrtest".as_ptr(),
            elf_ehdr_e_machine_offset(),
            &mut foreign_machine as *mut u16 as *const c_void,
            size_of_val(&foreign_machine)
        ),
        0
    );

    (*self_).have_static =
        artifact_path(
            src.as_mut_ptr(),
            size_of_val(&src),
            c"binfmt_loader_payload_static".as_ptr(),
        ) == 0
            && copy_file(src.as_ptr(), c"/tmp/binfmt_loader_static.ldrtest".as_ptr()) == 0;

    setenv(c"BINFMT_TEST_BINARY".as_ptr(), c"/tmp/binfmt_loader_target.ldrtest".as_ptr(), 1);
    setenv(c"BINFMT_TEST_INTERP".as_ptr(), c"/tmp/binfmt_loader_interp".as_ptr(), 1);

    /* Everything below needs the flag; find out once. */
    if write_reg(c":test_loader:E::ldrtest::/tmp/binfmt_loader_interp:L".as_ptr()) != 0 {
        ASSERT_EQ!(errno(), EINVAL);
        SKIP_return!("kernel without the 'L' flag");
    }
    unregister(c"test_loader".as_ptr());
}

unsafe fn loader_teardown(_self: *mut loader) {
    unregister(c"test_loader".as_ptr());
    if access(c"/tmp/binfmt_loader_interp.moved".as_ptr(), F_OK) == 0 {
        rename(c"/tmp/binfmt_loader_interp.moved".as_ptr(), c"/tmp/binfmt_loader_interp".as_ptr());
    }
    unlink(c"/tmp/binfmt_loader_target.ldrtest".as_ptr());
    unlink(c"/tmp/binfmt_loader_static.ldrtest".as_ptr());
    unlink(c"/tmp/binfmt_loader_foreign.ldrtest".as_ptr());
    unlink(c"/tmp/binfmt_loader_script.ldrtest".as_ptr());
    unlink(c"/tmp/binfmt_loader_interp".as_ptr());
}

fn elf_ehdr_e_machine_offset() -> usize {
    18
}

/* Grammar sanity check: the same entry without 'L' has to register. */
unsafe fn loader_plain_entry_registers(_self: *mut loader) {
    ASSERT_EQ!(
        write_reg(c":test_loader:E::ldrtest::/tmp/binfmt_loader_interp:".as_ptr()),
        0
    );
}

/* 'L' is a native exec: every classic-dispatch flag is rejected. */
unsafe fn loader_rejects_classic_flags(_self: *mut loader) {
    let combos: [*const c_char; 4] = [c"LT".as_ptr(), c"LP".as_ptr(), c"LC".as_ptr(), c"LO".as_ptr()];
    let mut rule = [0 as c_char; PATH_MAX];
    let mut i: c_uint;

    i = 0;
    while (i as usize) < combos.len() {
        let rc: c_int;

        snprintf(
            rule.as_mut_ptr(),
            size_of_val(&rule),
            c":test_loader:E::ldrtest::/tmp/binfmt_loader_interp:%s".as_ptr(),
            combos[i as usize],
        );
        rc = write_reg(rule.as_ptr());
        EXPECT_EQ!(rc, -1);
        TH_LOG!("'%s' was not rejected", combos[i as usize]);
        if rc == 0 {
            unregister(c"test_loader".as_ptr());
            i += 1;
            continue;
        }
        EXPECT_EQ!(errno(), EINVAL);
        i += 1;
    }
}

/*
 * Without 'F' the interpreter is opened when the binary is executed, so a
 * relative path would be resolved against the caller's working directory.
 */
unsafe fn loader_rejects_relative_interpreter(_self: *mut loader) {
    let flags: [*const c_char; 2] = [c"L".as_ptr(), c"C".as_ptr()];
    let mut rule = [0 as c_char; PATH_MAX];
    let mut i: c_uint;

    i = 0;
    while (i as usize) < flags.len() {
        let rc: c_int;

        snprintf(
            rule.as_mut_ptr(),
            size_of_val(&rule),
            c":test_loader:E::ldrtest::binfmt_loader_interp:%s".as_ptr(),
            flags[i as usize],
        );
        rc = write_reg(rule.as_ptr());
        EXPECT_EQ!(rc, -1);
        TH_LOG!("'%s' accepted a relative interpreter", flags[i as usize]);
        if rc == 0 {
            unregister(c"test_loader".as_ptr());
            i += 1;
            continue;
        }
        EXPECT_EQ!(errno(), EINVAL);
        i += 1;
    }
}

unsafe fn loader_extension_matched(_self: *mut loader) {
    ASSERT_EQ!(write_reg(c":test_loader:E::ldrtest::/tmp/binfmt_loader_interp:L".as_ptr()), 0);
    EXPECT_EQ!(run_payload(c"/tmp/binfmt_loader_target.ldrtest".as_ptr()), 0);
}

unsafe fn loader_magic_matched(_self: *mut loader) {
    ASSERT_EQ!(write_reg(c":test_loader:M:9:LOADER_MARKER::/tmp/binfmt_loader_interp:L".as_ptr()), 0);
    EXPECT_EQ!(run_payload(c"/tmp/binfmt_loader_target.ldrtest".as_ptr()), 0);
}

/*
 * The differentiator against the transparent mode: at PTRACE_EVENT_EXEC the
 * identity is already complete, with no window a debugger could observe.
 */
unsafe fn loader_exec_stop_consistency(_self: *mut loader) {
    ASSERT_EQ!(write_reg(c":test_loader:E::ldrtest::/tmp/binfmt_loader_interp:L".as_ptr()), 0);
    EXPECT_EQ!(ptrace_probe(c"/tmp/binfmt_loader_target.ldrtest".as_ptr()), 0);
}

/* A binary without PT_INTERP drops the override and runs natively. */
unsafe fn loader_static_binary_runs_natively(self_: *mut loader) {
    if !(*self_).have_static {
        SKIP_return!("no static payload built");
    }

    ASSERT_EQ!(write_reg(c":test_loader:E::ldrtest::/tmp/binfmt_loader_interp:L".as_ptr()), 0);
    setenv(c"BINFMT_TEST_BINARY".as_ptr(), c"/tmp/binfmt_loader_static.ldrtest".as_ptr(), 1);
    setenv(c"BINFMT_TEST_STATIC".as_ptr(), c"1".as_ptr(), 1);
    EXPECT_EQ!(run_payload(c"/tmp/binfmt_loader_static.ldrtest".as_ptr()), 0);
    unsetenv(c"BINFMT_TEST_STATIC".as_ptr());
    setenv(c"BINFMT_TEST_BINARY".as_ptr(), c"/tmp/binfmt_loader_target.ldrtest".as_ptr(), 1);
}

/*
 * A '#!' file that matched an 'L' entry is claimed by binfmt_script, which
 * sits ahead of binfmt_elf. The substitute the entry staged has to be
 * released when the interpreter replaces the file, not leaked.
 */
unsafe fn loader_script_claims_the_file(_self: *mut loader) {
    static SCRIPT: &[u8] = b"#!/bin/sh\nexit 0\n\0";
    let fd: c_int;

    unlink(c"/tmp/binfmt_loader_script.ldrtest".as_ptr());
    fd = open(
        c"/tmp/binfmt_loader_script.ldrtest".as_ptr(),
        O_WRONLY | O_CREAT | O_EXCL,
        0o755,
    );
    ASSERT_GE!(fd, 0);
    ASSERT_EQ!(
        write(fd, SCRIPT.as_ptr() as *const c_void, SCRIPT.len() - 1),
        (SCRIPT.len() - 1) as ssize_t
    );
    ASSERT_EQ!(close(fd), 0);

    ASSERT_EQ!(write_reg(c":test_loader:E::ldrtest::/tmp/binfmt_loader_interp:L".as_ptr()), 0);
    EXPECT_EQ!(run_payload(c"/tmp/binfmt_loader_script.ldrtest".as_ptr()), 0);

    /* A leaked substitute keeps its write denial on the loader. */
    let fd = open(c"/tmp/binfmt_loader_interp".as_ptr(), O_WRONLY);
    EXPECT_GE!(fd, 0);
    TH_LOG!("loader still write denied (errno %d)", errno());
    if fd >= 0 {
        close(fd);
    }
}

/* Nothing needs the binary's path, so an inaccessible fd works. */
unsafe fn loader_inaccessible_memfd(_self: *mut loader) {
    ASSERT_EQ!(write_reg(c":test_loader:M:9:LOADER_MARKER::/tmp/binfmt_loader_interp:L".as_ptr()), 0);
    EXPECT_EQ!(run_memfd(c"/tmp/binfmt_loader_target.ldrtest".as_ptr()), 0);
}

/* The whole exec of a wrong-arch binary fails as if unhandled. */
unsafe fn loader_foreign_arch_enoexec(_self: *mut loader) {
    ASSERT_EQ!(write_reg(c":test_loader:M:9:LOADER_MARKER::/tmp/binfmt_loader_interp:L".as_ptr()), 0);
    EXPECT_EQ!(run_payload(c"/tmp/binfmt_loader_foreign.ldrtest".as_ptr()), RUN_ENOEXEC);
}

/* 'F' pre-opens the substitute, so it survives losing its path. */
unsafe fn loader_fixed_interpreter_survives_rename(_self: *mut loader) {
    ASSERT_EQ!(write_reg(c":test_loader:E::ldrtest::/tmp/binfmt_loader_interp:FL".as_ptr()), 0);
    ASSERT_EQ!(
        rename(c"/tmp/binfmt_loader_interp".as_ptr(), c"/tmp/binfmt_loader_interp.moved".as_ptr()),
        0
    );
    EXPECT_EQ!(run_payload(c"/tmp/binfmt_loader_target.ldrtest".as_ptr()), 0);
}

TEST_HARNESS_MAIN!();
