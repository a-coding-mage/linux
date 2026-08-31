// SPDX-License-Identifier: GPL-2.0+

// Translated from testing/selftests/powerpc/dexcr/hashchk_test.c.
// C includes removed; external symbols from dexcr.h and utils.h are declared
// or referenced below as repository-provided dependencies.

use core::ffi::{c_char, c_int, c_ulong, c_void};

type pid_t = c_int;
type ssize_t = isize;
type jmp_buf = [c_ulong; 64];

#[repr(C)]
pub struct siginfo_t {
    pub si_signo: c_int,
    pub si_errno: c_int,
    pub si_code: c_int,
}

#[repr(C)]
pub struct sigaction {
    _private: [usize; 0],
}

unsafe extern "C" {
    fn dexcr_exists() -> c_int;
    fn pr_set_dexcr(aspect: c_ulong, value: c_ulong);
    fn get_dexcr(which: c_ulong) -> c_ulong;
    fn push_signal_handler(
        signum: c_int,
        handler: unsafe extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    ) -> sigaction;
    fn pop_signal_handler(signum: c_int, old: sigaction);
    fn do_bad_hashchk();
    fn hashst(index: c_ulong, ptr: *mut c_ulong);
    fn write(fd: c_int, buf: *const c_void, count: usize) -> ssize_t;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> pid_t;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn execve(pathname: *const c_char, argv: *const *mut c_char, envp: *const *mut c_char) -> c_int;
    fn _exit(status: c_int) -> !;
    fn await_child_success(pid: pid_t);
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> ssize_t;
    fn mmap(
        addr: *mut c_void,
        length: usize,
        prot: c_int,
        flags: c_int,
        fd: c_int,
        offset: isize,
    ) -> *mut c_void;
    fn clone(
        fn_: unsafe extern "C" fn(*mut c_void) -> c_int,
        child_stack: *mut c_void,
        flags: c_int,
        arg: *mut c_void,
    ) -> pid_t;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn setjmp(env: *mut jmp_buf) -> c_int;
    fn longjmp(env: *mut jmp_buf, val: c_int) -> !;
    fn test_harness(test: unsafe extern "C" fn() -> c_int, name: *const c_char) -> c_int;
}

// Constants supplied by system headers, dexcr.h, and utils.h in the original C.
const SIGILL: c_int = 4;
const SIGCHLD: c_int = 17;
const ILL_ILLOPN: c_int = 1;
const STDOUT_FILENO: c_int = 1;
const EOVERFLOW: c_int = 75;
const PROT_READ: c_int = 0x1;
const PROT_WRITE: c_int = 0x2;
const MAP_PRIVATE: c_int = 0x02;
const MAP_ANONYMOUS: c_int = 0x20;
const MAP_STACK: c_int = 0x20000;
const CLONE_VM: c_int = 0x00000100;
const MAP_FAILED: *mut c_void = !0usize as *mut c_void;

// Repository-provided constants/macros from dexcr.h and utils.h.
unsafe extern "C" {
    static mut errno: c_int;
}

const PR_PPC_DEXCR_NPHIE: c_ulong = 0;
const PR_PPC_DEXCR_CTRL_SET: c_ulong = 0;
const PR_PPC_DEXCR_CTRL_SET_ONEXEC: c_ulong = 0;
const EFFECTIVE: c_ulong = 0;
const DEXCR_PR_NPHIE: c_ulong = 0;

unsafe fn skip_if_msg(condition: bool, _msg: *const c_char) {
    if condition {
        _exit(0);
    }
}

unsafe fn fail_if_msg(condition: bool, msg: *const c_char) {
    if condition {
        panic!("{}", core::ffi::CStr::from_ptr(msg).to_string_lossy());
    }
}

unsafe fn fail_if_exit_msg(condition: bool, msg: *const c_char) {
    if condition {
        panic!("{}", core::ffi::CStr::from_ptr(msg).to_string_lossy());
    }
}

unsafe extern "C" fn require_nphie() -> c_int {
    skip_if_msg(
        dexcr_exists() == 0,
        b"DEXCR not supported\0".as_ptr() as *const c_char,
    );

    pr_set_dexcr(
        PR_PPC_DEXCR_NPHIE,
        PR_PPC_DEXCR_CTRL_SET | PR_PPC_DEXCR_CTRL_SET_ONEXEC,
    );

    if get_dexcr(EFFECTIVE) & DEXCR_PR_NPHIE != 0 {
        return 0;
    }

    skip_if_msg(
        !(get_dexcr(EFFECTIVE) & DEXCR_PR_NPHIE != 0),
        b"Failed to enable DEXCR[NPHIE]\0".as_ptr() as *const c_char,
    );

    0
}

static mut HASHCHK_DETECTED_BUF: jmp_buf = [0; 64];
static mut HASHCHK_FAILURE_MSG: *const c_char = core::ptr::null();

unsafe extern "C" fn hashchk_handler(signum: c_int, info: *mut siginfo_t, _context: *mut c_void) {
    if signum != SIGILL {
        HASHCHK_FAILURE_MSG = b"wrong signal received\0".as_ptr() as *const c_char;
    } else if (*info).si_code != ILL_ILLOPN {
        HASHCHK_FAILURE_MSG = b"wrong signal code received\0".as_ptr() as *const c_char;
    }

    longjmp(&raw mut HASHCHK_DETECTED_BUF, 0);
}

/*
 * Check that hashchk triggers when DEXCR[NPHIE] is enabled
 * and is detected as such by the kernel exception handler
 */
unsafe extern "C" fn hashchk_detected_test() -> c_int {
    let old: sigaction;
    let mut err: c_int;

    err = require_nphie();
    if err != 0 {
        return err;
    }

    old = push_signal_handler(SIGILL, hashchk_handler);
    if setjmp(&raw mut HASHCHK_DETECTED_BUF) != 0 {
        pop_signal_handler(SIGILL, old);
        fail_if_msg(!HASHCHK_FAILURE_MSG.is_null(), HASHCHK_FAILURE_MSG);
        return 0;
    }

    HASHCHK_FAILURE_MSG = core::ptr::null();
    do_bad_hashchk();
    HASHCHK_FAILURE_MSG = b"hashchk failed to trigger\0".as_ptr() as *const c_char;

    pop_signal_handler(SIGILL, old);
    fail_if_msg(!HASHCHK_FAILURE_MSG.is_null(), HASHCHK_FAILURE_MSG);
    0
}

const HASH_COUNT: usize = 8;

static mut HASH_VALUES: [c_ulong; HASH_COUNT + 1] = [0; HASH_COUNT + 1];

unsafe fn fill_hash_values() {
    let mut i: c_ulong = 0;
    while i < HASH_COUNT as c_ulong {
        hashst(i, &raw mut HASH_VALUES[i as usize]);
        i += 1;
    }

    /* Used to ensure the checks uses the same addresses as the hashes */
    HASH_VALUES[HASH_COUNT] = (&raw const HASH_VALUES) as c_ulong;
}

unsafe fn count_hash_values_matches() -> c_uint {
    let mut matches: c_ulong = 0;
    let mut i: c_ulong = 0;

    while i < HASH_COUNT as c_ulong {
        let orig_hash: c_ulong = HASH_VALUES[i as usize];
        HASH_VALUES[i as usize] = 0;

        hashst(i, &raw mut HASH_VALUES[i as usize]);

        if HASH_VALUES[i as usize] == orig_hash {
            matches += 1;
        }
        i += 1;
    }

    matches as c_uint
}

type c_uint = u32;

unsafe extern "C" fn hashchk_exec_child() -> c_int {
    let count: ssize_t;

    fill_hash_values();

    count = write(
        STDOUT_FILENO,
        (&raw const HASH_VALUES) as *const c_void,
        core::mem::size_of_val(&HASH_VALUES),
    );
    if count == core::mem::size_of_val(&HASH_VALUES) as ssize_t {
        0
    } else {
        EOVERFLOW
    }
}

static mut HASHCHK_EXEC_CHILD_ARG0: [c_char; 19] = *b"hashchk_exec_child\0";
static mut HASHCHK_EXEC_CHILD_ARGS: [*mut c_char; 2] =
    [unsafe { HASHCHK_EXEC_CHILD_ARG0.as_mut_ptr() }, core::ptr::null_mut()];

/*
 * Check that new programs get different keys so a malicious process
 * can't recreate a victim's hash values.
 */
unsafe extern "C" fn hashchk_exec_random_key_test() -> c_int {
    let mut pid: pid_t;
    let mut err: c_int;
    let mut pipefd: [c_int; 2] = [0; 2];

    err = require_nphie();
    if err != 0 {
        return err;
    }

    fail_if_msg(pipe(pipefd.as_mut_ptr()) != 0, b"failed to create pipe\0".as_ptr() as *const c_char);

    pid = fork();
    if pid == 0 {
        if dup2(pipefd[1], STDOUT_FILENO) == -1 {
            _exit(errno);
        }

        execve(
            b"/proc/self/exe\0".as_ptr() as *const c_char,
            HASHCHK_EXEC_CHILD_ARGS.as_ptr(),
            core::ptr::null(),
        );
        _exit(errno);
    }

    await_child_success(pid);
    fail_if_msg(
        read(
            pipefd[0],
            (&raw mut HASH_VALUES) as *mut c_void,
            core::mem::size_of_val(&HASH_VALUES),
        ) != core::mem::size_of_val(&HASH_VALUES) as ssize_t,
        b"missing expected child output\0".as_ptr() as *const c_char,
    );

    /* Verify the child used the same hash_values address */
    fail_if_exit_msg(
        HASH_VALUES[HASH_COUNT] != (&raw const HASH_VALUES) as c_ulong,
        b"bad address check\0".as_ptr() as *const c_char,
    );

    /* If all hashes are the same it means (most likely) same key */
    fail_if_msg(
        count_hash_values_matches() == HASH_COUNT as c_uint,
        b"shared key detected\0".as_ptr() as *const c_char,
    );

    0
}

/*
 * Check that forks share the same key so that existing hash values
 * remain valid.
 */
unsafe extern "C" fn hashchk_fork_share_key_test() -> c_int {
    let mut pid: pid_t;
    let mut err: c_int;

    err = require_nphie();
    if err != 0 {
        return err;
    }

    fill_hash_values();

    pid = fork();
    if pid == 0 {
        if count_hash_values_matches() != HASH_COUNT as c_uint {
            _exit(1);
        }
        _exit(0);
    }

    await_child_success(pid);
    0
}

const STACK_SIZE: usize = 1024 * 1024;

unsafe extern "C" fn hashchk_clone_child_fn(_args: *mut c_void) -> c_int {
    fill_hash_values();
    0
}

/*
 * Check that threads share the same key so that existing hash values
 * remain valid.
 */
unsafe extern "C" fn hashchk_clone_share_key_test() -> c_int {
    let mut child_stack: *mut c_void;
    let mut pid: pid_t;
    let mut err: c_int;

    err = require_nphie();
    if err != 0 {
        return err;
    }

    child_stack = mmap(
        core::ptr::null_mut(),
        STACK_SIZE,
        PROT_READ | PROT_WRITE,
        MAP_PRIVATE | MAP_ANONYMOUS | MAP_STACK,
        -1,
        0,
    );

    fail_if_msg(
        child_stack == MAP_FAILED,
        b"failed to map child stack\0".as_ptr() as *const c_char,
    );

    pid = clone(
        hashchk_clone_child_fn,
        (child_stack as *mut u8).add(STACK_SIZE) as *mut c_void,
        CLONE_VM | SIGCHLD,
        core::ptr::null_mut(),
    );

    await_child_success(pid);
    fail_if_msg(
        count_hash_values_matches() != HASH_COUNT as c_uint,
        b"different key detected\0".as_ptr() as *const c_char,
    );

    0
}

#[no_mangle]
pub unsafe extern "C" fn main(argc: c_int, argv: *mut *mut c_char) -> c_int {
    let mut err: c_int = 0;

    if argc >= 1 && strcmp(*argv, HASHCHK_EXEC_CHILD_ARGS[0]) == 0 {
        return hashchk_exec_child();
    }

    err |= test_harness(
        hashchk_detected_test,
        b"hashchk_detected\0".as_ptr() as *const c_char,
    );
    err |= test_harness(
        hashchk_exec_random_key_test,
        b"hashchk_exec_random_key\0".as_ptr() as *const c_char,
    );
    err |= test_harness(
        hashchk_fork_share_key_test,
        b"hashchk_fork_share_key\0".as_ptr() as *const c_char,
    );
    err |= test_harness(
        hashchk_clone_share_key_test,
        b"hashchk_clone_share_key\0".as_ptr() as *const c_char,
    );

    err
}
