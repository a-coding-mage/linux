// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2020 ARM Limited

// C dependencies from:
// <sys/auxv.h>, <sys/types.h>, <sys/wait.h>, <signal.h>, <setjmp.h>, <sched.h>
// "kselftest_harness.h", "helper.h"

use core::ffi::{c_char, c_int, c_void};
use core::mem;
use core::ptr;

const PAC_COLLISION_ATTEMPTS: c_int = 1000;
/*
 * The kernel sets TBID by default. So bits 55 and above should remain
 * untouched no matter what.
 * The VA space size is 48 bits. Bigger is opt-in.
 */
const PAC_MASK: usize = !(0xff80ffffffffffffusize);
const ARBITRARY_VALUE: usize = 0x1234;

extern "C" {
    static mut jmpbuf: sigjmp_buf;

    static mut stderr: *mut libc::FILE;

    fn getauxval(type_: libc::c_ulong) -> libc::c_ulong;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn perror(s: *const c_char);
    fn sched_getaffinity(pid: libc::pid_t, cpusetsize: usize, mask: *mut libc::cpu_set_t) -> c_int;
    fn sched_setaffinity(pid: libc::pid_t, cpusetsize: usize, mask: *const libc::cpu_set_t) -> c_int;
    fn fork() -> libc::pid_t;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn execl(path: *const c_char, arg0: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn write(fd: c_int, buf: *const c_void, count: usize) -> libc::ssize_t;
    fn waitpid(pid: libc::pid_t, status: *mut c_int, options: c_int) -> libc::pid_t;
    fn fprintf(stream: *mut libc::FILE, format: *const c_char, ...) -> c_int;
    fn read(fd: c_int, buf: *mut c_void, count: usize) -> libc::ssize_t;
    fn siglongjmp(env: sigjmp_buf, val: c_int) -> !;
    fn sigsetjmp(env: sigjmp_buf, savesigs: c_int) -> c_int;
    fn sigemptyset(set: *mut libc::sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const libc::sigaction, oldact: *mut libc::sigaction) -> c_int;

    fn keyia_sign(val: usize) -> usize;
    fn keyib_sign(val: usize) -> usize;
    fn keyda_sign(val: usize) -> usize;
    fn keydb_sign(val: usize) -> usize;
    fn keyg_sign(val: usize) -> usize;
    fn pac_corruptor();

    fn TH_LOG(format: *const c_char, ...);
    fn SKIP_return(format: *const c_char, ...) -> !;
}

type sigjmp_buf = *mut c_void;

#[repr(C)]
pub struct signatures {
    pub keyia: usize,
    pub keyib: usize,
    pub keyda: usize,
    pub keydb: usize,
    pub keyg: usize,
}

extern "C" {
    static NKEYS: c_int;
}

const AT_HWCAP: libc::c_ulong = libc::AT_HWCAP;
const HWCAP_PACA: libc::c_ulong = 1 << 30;
const HWCAP_PACG: libc::c_ulong = 1 << 31;

macro_rules! ASSERT_PAUTH_ENABLED {
    () => {{
        let hwcaps = unsafe { getauxval(AT_HWCAP) };
        /* data key instructions are not in NOP space. This prevents a SIGILL */
        if (hwcaps & HWCAP_PACA) == 0 {
            unsafe { SKIP_return(c"PAUTH not enabled".as_ptr()) };
        }
    }};
}

macro_rules! ASSERT_GENERIC_PAUTH_ENABLED {
    () => {{
        let hwcaps = unsafe { getauxval(AT_HWCAP) };
        /* generic key instructions are not in NOP space. This prevents a SIGILL */
        if (hwcaps & HWCAP_PACG) == 0 {
            unsafe { SKIP_return(c"Generic PAUTH not enabled".as_ptr()) };
        }
    }};
}

macro_rules! ASSERT_TRUE {
    ($expr:expr) => {{
        assert!($expr != 0);
    }};
}

macro_rules! ASSERT_NE {
    ($left:expr, $right:expr) => {{
        assert_ne!($left, $right);
    }};
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {{
        assert_eq!($left, $right);
    }};
}

#[no_mangle]
pub unsafe extern "C" fn sign_specific(sign: *mut signatures, val: usize) {
    (*sign).keyia = keyia_sign(val);
    (*sign).keyib = keyib_sign(val);
    (*sign).keyda = keyda_sign(val);
    (*sign).keydb = keydb_sign(val);
}

#[no_mangle]
pub unsafe extern "C" fn sign_all(sign: *mut signatures, val: usize) {
    (*sign).keyia = keyia_sign(val);
    (*sign).keyib = keyib_sign(val);
    (*sign).keyda = keyda_sign(val);
    (*sign).keydb = keydb_sign(val);
    (*sign).keyg = keyg_sign(val);
}

#[no_mangle]
pub unsafe extern "C" fn n_same(old: *mut signatures, new: *mut signatures, nkeys: c_int) -> c_int {
    let mut res: c_int = 0;

    res += ((*old).keyia == (*new).keyia) as c_int;
    res += ((*old).keyib == (*new).keyib) as c_int;
    res += ((*old).keyda == (*new).keyda) as c_int;
    res += ((*old).keydb == (*new).keydb) as c_int;
    if nkeys == NKEYS {
        res += ((*old).keyg == (*new).keyg) as c_int;
    }

    res
}

#[no_mangle]
pub unsafe extern "C" fn n_same_single_set(sign: *mut signatures, nkeys: c_int) -> c_int {
    let mut vals = vec![0usize; nkeys as usize];
    let mut same: c_int = 0;

    *vals.as_mut_ptr().add(0) = (*sign).keyia & PAC_MASK;
    *vals.as_mut_ptr().add(1) = (*sign).keyib & PAC_MASK;
    *vals.as_mut_ptr().add(2) = (*sign).keyda & PAC_MASK;
    *vals.as_mut_ptr().add(3) = (*sign).keydb & PAC_MASK;

    if nkeys >= 4 {
        *vals.as_mut_ptr().add(4) = (*sign).keyg & PAC_MASK;
    }

    for i in 0..(nkeys - 1) {
        for j in (i + 1)..nkeys {
            if *vals.as_ptr().add(i as usize) == *vals.as_ptr().add(j as usize) {
                same += 1;
            }
        }
    }
    same
}

#[no_mangle]
pub unsafe extern "C" fn exec_sign_all(signed_vals: *mut signatures, val: usize) -> c_int {
    let mut new_stdin: [c_int; 2] = [0; 2];
    let mut new_stdout: [c_int; 2] = [0; 2];
    let mut status: c_int = 0;
    let mut i: usize;
    let mut ret: libc::ssize_t;
    let pid: libc::pid_t;
    let mut mask: libc::cpu_set_t = mem::zeroed();

    ret = pipe(new_stdin.as_mut_ptr()) as libc::ssize_t;
    if ret == -1 {
        perror(c"pipe returned error".as_ptr());
        return -1;
    }

    ret = pipe(new_stdout.as_mut_ptr()) as libc::ssize_t;
    if ret == -1 {
        perror(c"pipe returned error".as_ptr());
        return -1;
    }

    /*
     * pin this process and all its children to a single CPU, so it can also
     * guarantee a context switch with its child
     */
    sched_getaffinity(0, mem::size_of_val(&mask), &mut mask);

    i = 0;
    while i < mem::size_of::<libc::cpu_set_t>() {
        if libc::CPU_ISSET(i, &mask) {
            break;
        }
        i += 1;
    }

    libc::CPU_ZERO(&mut mask);
    libc::CPU_SET(i, &mut mask);
    sched_setaffinity(0, mem::size_of_val(&mask), &mask);

    pid = fork();
    // child
    if pid == 0 {
        dup2(new_stdin[0], libc::STDIN_FILENO);
        if ret == -1 {
            perror(c"dup2 returned error".as_ptr());
            exit(1);
        }

        dup2(new_stdout[1], libc::STDOUT_FILENO);
        if ret == -1 {
            perror(c"dup2 returned error".as_ptr());
            exit(1);
        }

        close(new_stdin[0]);
        close(new_stdin[1]);
        close(new_stdout[0]);
        close(new_stdout[1]);

        ret = execl(
            c"exec_target".as_ptr(),
            c"exec_target".as_ptr(),
            ptr::null::<c_char>(),
        ) as libc::ssize_t;
        if ret == -1 {
            perror(c"exec returned error".as_ptr());
            exit(1);
        }
    }

    close(new_stdin[0]);
    close(new_stdout[1]);

    ret = write(
        new_stdin[1],
        &val as *const usize as *const c_void,
        mem::size_of::<usize>(),
    );
    if ret == -1 {
        perror(c"write returned error".as_ptr());
        return -1;
    }

    /*
     * wait for the worker to finish, so that read() reads all data
     * will also context switch with worker so that this function can be used
     * for context switch tests
     */
    waitpid(pid, &mut status, 0);
    if libc::WIFEXITED(status) == false {
        fprintf(stderr, c"worker exited unexpectedly\n".as_ptr());
        return -1;
    }
    if libc::WEXITSTATUS(status) != 0 {
        fprintf(stderr, c"worker exited with error\n".as_ptr());
        return -1;
    }

    ret = read(
        new_stdout[0],
        signed_vals as *mut c_void,
        mem::size_of::<signatures>(),
    );
    if ret == -1 {
        perror(c"read returned error".as_ptr());
        return -1;
    }

    close(new_stdin[1]);
    close(new_stdout[0]);

    0
}

#[no_mangle]
pub unsafe extern "C" fn pac_signal_handler(signum: c_int, _si: *mut libc::siginfo_t, _uc: *mut c_void) {
    if signum == libc::SIGSEGV || signum == libc::SIGILL {
        siglongjmp(jmpbuf, 1);
    }
}

/* check that a corrupted PAC results in SIGSEGV or SIGILL */
// TEST(corrupt_pac)
pub unsafe fn corrupt_pac() {
    let mut sa: libc::sigaction = mem::zeroed();

    ASSERT_PAUTH_ENABLED!();
    if sigsetjmp(jmpbuf, 1) == 0 {
        sa.sa_sigaction = pac_signal_handler as usize;
        sa.sa_flags = libc::SA_SIGINFO | libc::SA_RESETHAND;
        sigemptyset(&mut sa.sa_mask);

        sigaction(libc::SIGSEGV, &sa, ptr::null_mut());
        sigaction(libc::SIGILL, &sa, ptr::null_mut());

        pac_corruptor();
        ASSERT_TRUE!(0);
        TH_LOG(c"SIGSEGV/SIGILL signal did not occur".as_ptr());
    }
}

/*
 * There are no separate pac* and aut* controls so checking only the pac*
 * instructions is sufficient
 */
// TEST(pac_instructions_not_nop)
pub unsafe fn pac_instructions_not_nop() {
    let mut keyia: usize = 0;
    let mut keyib: usize = 0;
    let mut keyda: usize = 0;
    let mut keydb: usize = 0;

    ASSERT_PAUTH_ENABLED!();

    for i in 0..PAC_COLLISION_ATTEMPTS {
        keyia |= keyia_sign(i as usize) & PAC_MASK;
        keyib |= keyib_sign(i as usize) & PAC_MASK;
        keyda |= keyda_sign(i as usize) & PAC_MASK;
        keydb |= keydb_sign(i as usize) & PAC_MASK;
    }

    ASSERT_NE!(0, keyia);
    TH_LOG(c"keyia instructions did nothing".as_ptr());
    ASSERT_NE!(0, keyib);
    TH_LOG(c"keyib instructions did nothing".as_ptr());
    ASSERT_NE!(0, keyda);
    TH_LOG(c"keyda instructions did nothing".as_ptr());
    ASSERT_NE!(0, keydb);
    TH_LOG(c"keydb instructions did nothing".as_ptr());
}

// TEST(pac_instructions_not_nop_generic)
pub unsafe fn pac_instructions_not_nop_generic() {
    let mut keyg: usize = 0;

    ASSERT_GENERIC_PAUTH_ENABLED!();

    for i in 0..PAC_COLLISION_ATTEMPTS {
        keyg |= keyg_sign(i as usize) & PAC_MASK;
    }

    ASSERT_NE!(0, keyg);
    TH_LOG(c"keyg instructions did nothing".as_ptr());
}

// TEST(single_thread_different_keys)
pub unsafe fn single_thread_different_keys() {
    let mut same: c_int = 10;
    let mut nkeys: c_int = NKEYS;
    let mut tmp: c_int;
    let mut signed_vals: signatures = mem::zeroed();
    let hwcaps: libc::c_ulong = getauxval(AT_HWCAP);

    /* generic and data key instructions are not in NOP space. This prevents a SIGILL */
    ASSERT_PAUTH_ENABLED!();
    if (hwcaps & HWCAP_PACG) == 0 {
        TH_LOG(c"WARNING: Generic PAUTH not enabled. Skipping generic key checks".as_ptr());
        nkeys = NKEYS - 1;
    }

    /*
     * In Linux the PAC field can be up to 7 bits wide. Even if keys are
     * different, there is about 5% chance for PACs to collide with
     * different addresses. This chance rapidly increases with fewer bits
     * allocated for the PAC (e.g. wider address). A comparison of the keys
     * directly will be more reliable.
     * All signed values need to be different at least once out of n
     * attempts to be certain that the keys are different
     */
    for i in 0..PAC_COLLISION_ATTEMPTS {
        if nkeys == NKEYS {
            sign_all(&mut signed_vals, i as usize);
        } else {
            sign_specific(&mut signed_vals, i as usize);
        }

        tmp = n_same_single_set(&mut signed_vals, nkeys);
        if tmp < same {
            same = tmp;
        }
    }

    ASSERT_EQ!(0, same);
    TH_LOG(c"%d keys clashed every time".as_ptr(), same);
}

/*
 * fork() does not change keys. Only exec() does so call a worker program.
 * Its only job is to sign a value and report back the results
 */
// TEST(exec_changed_keys)
pub unsafe fn exec_changed_keys() {
    let mut new_keys: signatures = mem::zeroed();
    let mut old_keys: signatures = mem::zeroed();
    let mut ret: c_int;
    let mut same: c_int = 10;
    let mut nkeys: c_int = NKEYS;
    let hwcaps: libc::c_ulong = getauxval(AT_HWCAP);

    /* generic and data key instructions are not in NOP space. This prevents a SIGILL */
    ASSERT_PAUTH_ENABLED!();
    if (hwcaps & HWCAP_PACG) == 0 {
        TH_LOG(c"WARNING: Generic PAUTH not enabled. Skipping generic key checks".as_ptr());
        nkeys = NKEYS - 1;
    }

    for i in 0..PAC_COLLISION_ATTEMPTS {
        ret = exec_sign_all(&mut new_keys, i as usize);
        ASSERT_EQ!(0, ret);
        TH_LOG(c"failed to run worker".as_ptr());

        if nkeys == NKEYS {
            sign_all(&mut old_keys, i as usize);
        } else {
            sign_specific(&mut old_keys, i as usize);
        }

        ret = n_same(&mut old_keys, &mut new_keys, nkeys);
        if ret < same {
            same = ret;
        }
    }

    ASSERT_EQ!(0, same);
    TH_LOG(c"exec() did not change %d keys".as_ptr(), same);
}

// TEST(context_switch_keep_keys)
pub unsafe fn context_switch_keep_keys() {
    let ret: c_int;
    let mut trash: signatures = mem::zeroed();
    let mut before: signatures = mem::zeroed();
    let mut after: signatures = mem::zeroed();

    ASSERT_PAUTH_ENABLED!();

    sign_specific(&mut before, ARBITRARY_VALUE);

    /* will context switch with a process with different keys at least once */
    ret = exec_sign_all(&mut trash, ARBITRARY_VALUE);
    ASSERT_EQ!(0, ret);
    TH_LOG(c"failed to run worker".as_ptr());

    sign_specific(&mut after, ARBITRARY_VALUE);

    ASSERT_EQ!(before.keyia, after.keyia);
    TH_LOG(c"keyia changed after context switching".as_ptr());
    ASSERT_EQ!(before.keyib, after.keyib);
    TH_LOG(c"keyib changed after context switching".as_ptr());
    ASSERT_EQ!(before.keyda, after.keyda);
    TH_LOG(c"keyda changed after context switching".as_ptr());
    ASSERT_EQ!(before.keydb, after.keydb);
    TH_LOG(c"keydb changed after context switching".as_ptr());
}

// TEST(context_switch_keep_keys_generic)
pub unsafe fn context_switch_keep_keys_generic() {
    let ret: c_int;
    let mut trash: signatures = mem::zeroed();
    let before: usize;
    let after: usize;

    ASSERT_GENERIC_PAUTH_ENABLED!();

    before = keyg_sign(ARBITRARY_VALUE);

    /* will context switch with a process with different keys at least once */
    ret = exec_sign_all(&mut trash, ARBITRARY_VALUE);
    ASSERT_EQ!(0, ret);
    TH_LOG(c"failed to run worker".as_ptr());

    after = keyg_sign(ARBITRARY_VALUE);

    ASSERT_EQ!(before, after);
    TH_LOG(c"keyg changed after context switching".as_ptr());
}

// TEST_HARNESS_MAIN

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
