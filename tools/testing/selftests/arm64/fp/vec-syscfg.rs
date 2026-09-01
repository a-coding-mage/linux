// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 ARM Limited.
 * Original author: Mark Brown <broonie@kernel.org>
 */

use core::arch::asm;
use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::ptr;

// C dependencies from assert.h, errno.h, stdio.h, stdlib.h, string.h, unistd.h,
// sys/auxv.h, sys/prctl.h, sys/types.h, sys/wait.h, asm/hwcap.h,
// asm/sigcontext.h, kselftest.h, and rdvl.h are expected from the surrounding
// translated repository/build.

type FILE = c_void;
type PidT = c_int;

const EXIT_FAILURE: c_int = 1;
const STDOUT_FILENO: c_int = 1;
const AT_HWCAP: c_ulong = 16;
const AT_HWCAP2: c_ulong = 26;

const HWCAP_SVE: c_ulong = 1 << 22;
const HWCAP2_SME: c_ulong = 1 << 23;

const SVE_VL_MIN: c_int = 16;
const SVE_VQ_MIN: c_int = 1;
const SVE_VQ_MAX: c_int = 512;
const PR_SVE_VL_LEN_MASK: c_int = 0xffff;
const PR_SVE_VL_INHERIT: c_int = 1 << 17;
const PR_SVE_SET_VL_ONEXEC: c_int = 1 << 18;

// Values supplied by <sys/prctl.h> in C.
const PR_SVE_SET_VL: c_int = 50;
const PR_SVE_GET_VL: c_int = 51;
const PR_SME_SET_VL: c_int = 63;
const PR_SME_GET_VL: c_int = 64;

const ARCH_MIN_VL: c_int = SVE_VL_MIN;

const VEC_SVE: usize = 0;
const VEC_SME: usize = 1;

unsafe extern "C" {
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn fscanf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fprintf(stream: *mut FILE, format: *const c_char, ...) -> c_int;
    fn fflush(stream: *mut FILE) -> c_int;
    fn pipe(pipefd: *mut c_int) -> c_int;
    fn fork() -> PidT;
    fn close(fd: c_int) -> c_int;
    fn dup2(oldfd: c_int, newfd: c_int) -> c_int;
    fn execl(path: *const c_char, arg: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;
    fn wait(wstatus: *mut c_int) -> PidT;
    fn fdopen(fd: c_int, mode: *const c_char) -> *mut FILE;
    fn fclose(stream: *mut FILE) -> c_int;
    fn fopen(pathname: *const c_char, mode: *const c_char) -> *mut FILE;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn geteuid() -> c_int;
    fn prctl(option: c_int, ...) -> c_int;

    static mut errno: c_int;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;

    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_test_result_fail(fmt: *const c_char, ...);
    fn ksft_test_result_pass(fmt: *const c_char, ...);
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result(pass: bool, fmt: *const c_char, ...);
    fn ksft_exit_pass() -> !;

    fn rdvl_sve() -> c_int;
    fn rdvl_sme() -> c_int;
}

#[repr(C)]
struct VecData {
    name: *const c_char,
    hwcap_type: c_ulong,
    hwcap: c_ulong,
    rdvl_binary: *const c_char,
    rdvl: unsafe extern "C" fn() -> c_int,

    prctl_get: c_int,
    prctl_set: c_int,
    default_vl_file: *const c_char,

    default_vl: c_int,
    min_vl: c_int,
    max_vl: c_int,
}

static mut VEC_DATA: [VecData; 2] = [
    VecData {
        name: c"SVE".as_ptr(),
        hwcap_type: AT_HWCAP,
        hwcap: HWCAP_SVE,
        rdvl: rdvl_sve,
        rdvl_binary: c"./rdvl-sve".as_ptr(),
        prctl_get: PR_SVE_GET_VL,
        prctl_set: PR_SVE_SET_VL,
        default_vl_file: c"/proc/sys/abi/sve_default_vector_length".as_ptr(),
        default_vl: 0,
        min_vl: 0,
        max_vl: 0,
    },
    VecData {
        name: c"SME".as_ptr(),
        hwcap_type: AT_HWCAP2,
        hwcap: HWCAP2_SME,
        rdvl: rdvl_sme,
        rdvl_binary: c"./rdvl-sme".as_ptr(),
        prctl_get: PR_SME_GET_VL,
        prctl_set: PR_SME_SET_VL,
        default_vl_file: c"/proc/sys/abi/sme_default_vector_length".as_ptr(),
        default_vl: 0,
        min_vl: 0,
        max_vl: 0,
    },
];

unsafe fn vec_type_supported(data: *mut VecData) -> bool {
    (getauxval((*data).hwcap_type) & (*data).hwcap) != 0
}

unsafe fn stdio_read_integer(f: *mut FILE, what: *const c_char, val: *mut c_int) -> c_int {
    let mut n: c_int = 0;
    let ret: c_int;

    ret = fscanf(f, c"%d%*1[\n]%n".as_ptr(), val, &mut n as *mut c_int);
    if ret < 1 || n < 1 {
        ksft_print_msg(c"failed to parse integer from %s\n".as_ptr(), what);
        return -1;
    }

    0
}

/* Start a new process and return the vector length it sees */
unsafe fn get_child_rdvl(data: *mut VecData) -> c_int {
    let mut out: *mut FILE;
    let mut pipefd: [c_int; 2] = [0; 2];
    let mut pid: PidT;
    let child: PidT;
    let mut read_vl: c_int = 0;
    let mut ret: c_int;

    ret = pipe(pipefd.as_mut_ptr());
    if ret == -1 {
        ksft_print_msg(
            c"pipe() failed: %d (%s)\n".as_ptr(),
            errno,
            strerror(errno),
        );
        return -1;
    }

    fflush(stdout);

    child = fork();
    if child == -1 {
        ksft_print_msg(
            c"fork() failed: %d (%s)\n".as_ptr(),
            errno,
            strerror(errno),
        );
        close(pipefd[0]);
        close(pipefd[1]);
        return -1;
    }

    /* Child: put vector length on the pipe */
    if child == 0 {
        /*
         * Replace stdout with the pipe, errors to stderr from
         * here as kselftest prints to stdout.
         */
        ret = dup2(pipefd[1], STDOUT_FILENO);
        if ret == -1 {
            fprintf(stderr, c"dup2() %d\n".as_ptr(), errno);
            exit(EXIT_FAILURE);
        }

        /* exec() a new binary which puts the VL on stdout */
        ret = execl((*data).rdvl_binary, (*data).rdvl_binary, ptr::null::<c_char>());
        fprintf(
            stderr,
            c"execl(%s) failed: %d (%s)\n".as_ptr(),
            (*data).rdvl_binary,
            errno,
            strerror(errno),
        );

        exit(EXIT_FAILURE);
    }

    close(pipefd[1]);

    /* Parent; wait for the exit status from the child & verify it */
    loop {
        pid = wait(&mut ret as *mut c_int);
        if pid == -1 {
            ksft_print_msg(
                c"wait() failed: %d (%s)\n".as_ptr(),
                errno,
                strerror(errno),
            );
            close(pipefd[0]);
            return -1;
        }
        if pid == child {
            break;
        }
    }

    assert!(pid == child);

    if !wifexited(ret) {
        ksft_print_msg(c"child exited abnormally\n".as_ptr());
        close(pipefd[0]);
        return -1;
    }

    if wexitstatus(ret) != 0 {
        ksft_print_msg(c"child returned error %d\n".as_ptr(), wexitstatus(ret));
        close(pipefd[0]);
        return -1;
    }

    out = fdopen(pipefd[0], c"r".as_ptr());
    if out.is_null() {
        ksft_print_msg(c"failed to open child stdout\n".as_ptr());
        close(pipefd[0]);
        return -1;
    }

    ret = stdio_read_integer(out, c"child".as_ptr(), &mut read_vl as *mut c_int);
    fclose(out);
    if ret != 0 {
        return ret;
    }

    read_vl
}

unsafe fn file_read_integer(name: *const c_char, val: *mut c_int) -> c_int {
    let f: *mut FILE;
    let ret: c_int;

    f = fopen(name, c"r".as_ptr());
    if f.is_null() {
        ksft_test_result_fail(
            c"Unable to open %s: %d (%s)\n".as_ptr(),
            name,
            errno,
            strerror(errno),
        );
        return -1;
    }

    ret = stdio_read_integer(f, name, val);
    fclose(f);

    ret
}

unsafe fn file_write_integer(name: *const c_char, val: c_int) -> c_int {
    let f: *mut FILE;

    f = fopen(name, c"w".as_ptr());
    if f.is_null() {
        ksft_test_result_fail(
            c"Unable to open %s: %d (%s)\n".as_ptr(),
            name,
            errno,
            strerror(errno),
        );
        return -1;
    }

    fprintf(f, c"%d".as_ptr(), val);
    fclose(f);

    0
}

/*
 * Verify that we can read the default VL via proc, checking that it
 * is set in a freshly spawned child.
 */
unsafe fn proc_read_default(data: *mut VecData) {
    let mut default_vl: c_int = 0;
    let child_vl: c_int;
    let ret: c_int;

    ret = file_read_integer((*data).default_vl_file, &mut default_vl as *mut c_int);
    if ret != 0 {
        return;
    }

    /* Is this the actual default seen by new processes? */
    child_vl = get_child_rdvl(data);
    if child_vl != default_vl {
        ksft_test_result_fail(
            c"%s is %d but child VL is %d\n".as_ptr(),
            (*data).default_vl_file,
            default_vl,
            child_vl,
        );
        return;
    }

    ksft_test_result_pass(
        c"%s default vector length %d\n".as_ptr(),
        (*data).name,
        default_vl,
    );
    (*data).default_vl = default_vl;
}

/* Verify that we can write a minimum value and have it take effect */
unsafe fn proc_write_min(data: *mut VecData) {
    let ret: c_int;
    let mut new_default: c_int = 0;
    let child_vl: c_int;

    if geteuid() != 0 {
        ksft_test_result_skip(c"Need to be root to write to /proc\n".as_ptr());
        return;
    }

    let mut ret = file_write_integer((*data).default_vl_file, ARCH_MIN_VL);
    if ret != 0 {
        return;
    }

    /* What was the new value? */
    ret = file_read_integer((*data).default_vl_file, &mut new_default as *mut c_int);
    if ret != 0 {
        return;
    }

    /* Did it take effect in a new process? */
    child_vl = get_child_rdvl(data);
    if child_vl != new_default {
        ksft_test_result_fail(
            c"%s is %d but child VL is %d\n".as_ptr(),
            (*data).default_vl_file,
            new_default,
            child_vl,
        );
        return;
    }

    ksft_test_result_pass(
        c"%s minimum vector length %d\n".as_ptr(),
        (*data).name,
        new_default,
    );
    (*data).min_vl = new_default;

    file_write_integer((*data).default_vl_file, (*data).default_vl);
}

/* Verify that we can write a maximum value and have it take effect */
unsafe fn proc_write_max(data: *mut VecData) {
    let mut new_default: c_int = 0;
    let child_vl: c_int;
    let mut ret: c_int;

    if geteuid() != 0 {
        ksft_test_result_skip(c"Need to be root to write to /proc\n".as_ptr());
        return;
    }

    /* -1 is accepted by the /proc interface as the maximum VL */
    ret = file_write_integer((*data).default_vl_file, -1);
    if ret != 0 {
        return;
    }

    /* What was the new value? */
    ret = file_read_integer((*data).default_vl_file, &mut new_default as *mut c_int);
    if ret != 0 {
        return;
    }

    /* Did it take effect in a new process? */
    child_vl = get_child_rdvl(data);
    if child_vl != new_default {
        ksft_test_result_fail(
            c"%s is %d but child VL is %d\n".as_ptr(),
            (*data).default_vl_file,
            new_default,
            child_vl,
        );
        return;
    }

    ksft_test_result_pass(
        c"%s maximum vector length %d\n".as_ptr(),
        (*data).name,
        new_default,
    );
    (*data).max_vl = new_default;

    file_write_integer((*data).default_vl_file, (*data).default_vl);
}

/* Can we read back a VL from prctl? */
unsafe fn prctl_get(data: *mut VecData) {
    let mut ret: c_int;

    ret = prctl((*data).prctl_get);
    if ret == -1 {
        ksft_test_result_fail(
            c"%s prctl() read failed: %d (%s)\n".as_ptr(),
            (*data).name,
            errno,
            strerror(errno),
        );
        return;
    }

    /* Mask out any flags */
    ret &= PR_SVE_VL_LEN_MASK;

    /* Is that what we can read back directly? */
    if ret == ((*data).rdvl)() {
        ksft_test_result_pass(c"%s current VL is %d\n".as_ptr(), (*data).name, ret);
    } else {
        ksft_test_result_fail(
            c"%s prctl() VL %d but RDVL is %d\n".as_ptr(),
            (*data).name,
            ret,
            ((*data).rdvl)(),
        );
    }
}

/* Does the prctl let us set the VL we already have? */
unsafe fn prctl_set_same(data: *mut VecData) {
    let cur_vl: c_int = ((*data).rdvl)();
    let ret: c_int;

    ret = prctl((*data).prctl_set, cur_vl);
    if ret < 0 {
        ksft_test_result_fail(
            c"%s prctl set failed: %d (%s)\n".as_ptr(),
            (*data).name,
            errno,
            strerror(errno),
        );
        return;
    }

    ksft_test_result(
        cur_vl == ((*data).rdvl)(),
        c"%s set VL %d and have VL %d\n".as_ptr(),
        (*data).name,
        cur_vl,
        ((*data).rdvl)(),
    );
}

/* Can we set a new VL for this process? */
unsafe fn prctl_set(data: *mut VecData) {
    let mut ret: c_int;

    if (*data).min_vl == (*data).max_vl {
        ksft_test_result_skip(c"%s only one VL supported\n".as_ptr(), (*data).name);
        return;
    }

    /* Try to set the minimum VL */
    ret = prctl((*data).prctl_set, (*data).min_vl);
    if ret < 0 {
        ksft_test_result_fail(
            c"%s prctl set failed for %d: %d (%s)\n".as_ptr(),
            (*data).name,
            (*data).min_vl,
            errno,
            strerror(errno),
        );
        return;
    }

    if (ret & PR_SVE_VL_LEN_MASK) != (*data).min_vl {
        ksft_test_result_fail(
            c"%s prctl set %d but return value is %d\n".as_ptr(),
            (*data).name,
            (*data).min_vl,
            ((*data).rdvl)(),
        );
        return;
    }

    if ((*data).rdvl)() != (*data).min_vl {
        ksft_test_result_fail(
            c"%s set %d but RDVL is %d\n".as_ptr(),
            (*data).name,
            (*data).min_vl,
            ((*data).rdvl)(),
        );
        return;
    }

    /* Try to set the maximum VL */
    ret = prctl((*data).prctl_set, (*data).max_vl);
    if ret < 0 {
        ksft_test_result_fail(
            c"%s prctl set failed for %d: %d (%s)\n".as_ptr(),
            (*data).name,
            (*data).max_vl,
            errno,
            strerror(errno),
        );
        return;
    }

    if (ret & PR_SVE_VL_LEN_MASK) != (*data).max_vl {
        ksft_test_result_fail(
            c"%s prctl() set %d but return value is %d\n".as_ptr(),
            (*data).name,
            (*data).max_vl,
            ((*data).rdvl)(),
        );
        return;
    }

    /* The _INHERIT flag should not be present when we read the VL */
    ret = prctl((*data).prctl_get);
    if ret == -1 {
        ksft_test_result_fail(
            c"%s prctl() read failed: %d (%s)\n".as_ptr(),
            (*data).name,
            errno,
            strerror(errno),
        );
        return;
    }

    if (ret & PR_SVE_VL_INHERIT) != 0 {
        ksft_test_result_fail(c"%s prctl() reports _INHERIT\n".as_ptr(), (*data).name);
        return;
    }

    ksft_test_result_pass(c"%s prctl() set min/max\n".as_ptr(), (*data).name);
}

/* If we didn't request it a new VL shouldn't affect the child */
unsafe fn prctl_set_no_child(data: *mut VecData) {
    let mut ret: c_int;
    let child_vl: c_int;

    if (*data).min_vl == (*data).max_vl {
        ksft_test_result_skip(c"%s only one VL supported\n".as_ptr(), (*data).name);
        return;
    }

    ret = prctl((*data).prctl_set, (*data).min_vl);
    if ret < 0 {
        ksft_test_result_fail(
            c"%s prctl set failed for %d: %d (%s)\n".as_ptr(),
            (*data).name,
            (*data).min_vl,
            errno,
            strerror(errno),
        );
        return;
    }

    /* Ensure the default VL is different */
    ret = file_write_integer((*data).default_vl_file, (*data).max_vl);
    if ret != 0 {
        return;
    }

    /* Check that the child has the default we just set */
    child_vl = get_child_rdvl(data);
    if child_vl != (*data).max_vl {
        ksft_test_result_fail(
            c"%s is %d but child VL is %d\n".as_ptr(),
            (*data).default_vl_file,
            (*data).max_vl,
            child_vl,
        );
        return;
    }

    ksft_test_result_pass(c"%s vector length used default\n".as_ptr(), (*data).name);

    file_write_integer((*data).default_vl_file, (*data).default_vl);
}

/* If we didn't request it a new VL shouldn't affect the child */
unsafe fn prctl_set_for_child(data: *mut VecData) {
    let mut ret: c_int;
    let child_vl: c_int;

    if (*data).min_vl == (*data).max_vl {
        ksft_test_result_skip(c"%s only one VL supported\n".as_ptr(), (*data).name);
        return;
    }

    ret = prctl((*data).prctl_set, (*data).min_vl | PR_SVE_VL_INHERIT);
    if ret < 0 {
        ksft_test_result_fail(
            c"%s prctl set failed for %d: %d (%s)\n".as_ptr(),
            (*data).name,
            (*data).min_vl,
            errno,
            strerror(errno),
        );
        return;
    }

    /* The _INHERIT flag should be present when we read the VL */
    ret = prctl((*data).prctl_get);
    if ret == -1 {
        ksft_test_result_fail(
            c"%s prctl() read failed: %d (%s)\n".as_ptr(),
            (*data).name,
            errno,
            strerror(errno),
        );
        return;
    }
    if (ret & PR_SVE_VL_INHERIT) == 0 {
        ksft_test_result_fail(
            c"%s prctl() does not report _INHERIT\n".as_ptr(),
            (*data).name,
        );
        return;
    }

    /* Ensure the default VL is different */
    ret = file_write_integer((*data).default_vl_file, (*data).max_vl);
    if ret != 0 {
        return;
    }

    /* Check that the child inherited our VL */
    child_vl = get_child_rdvl(data);
    if child_vl != (*data).min_vl {
        ksft_test_result_fail(
            c"%s is %d but child VL is %d\n".as_ptr(),
            (*data).default_vl_file,
            (*data).min_vl,
            child_vl,
        );
        return;
    }

    ksft_test_result_pass(c"%s vector length was inherited\n".as_ptr(), (*data).name);

    file_write_integer((*data).default_vl_file, (*data).default_vl);
}

/* _ONEXEC takes effect only in the child process */
unsafe fn prctl_set_onexec(data: *mut VecData) {
    let mut ret: c_int;
    let child_vl: c_int;

    if (*data).min_vl == (*data).max_vl {
        ksft_test_result_skip(c"%s only one VL supported\n".as_ptr(), (*data).name);
        return;
    }

    /* Set a known value for the default and our current VL */
    ret = file_write_integer((*data).default_vl_file, (*data).max_vl);
    if ret != 0 {
        return;
    }

    ret = prctl((*data).prctl_set, (*data).max_vl);
    if ret < 0 {
        ksft_test_result_fail(
            c"%s prctl set failed for %d: %d (%s)\n".as_ptr(),
            (*data).name,
            (*data).min_vl,
            errno,
            strerror(errno),
        );
        return;
    }

    /* Set a different value for the child to have on exec */
    ret = prctl((*data).prctl_set, (*data).min_vl | PR_SVE_SET_VL_ONEXEC);
    if ret < 0 {
        ksft_test_result_fail(
            c"%s prctl set failed for %d: %d (%s)\n".as_ptr(),
            (*data).name,
            (*data).min_vl,
            errno,
            strerror(errno),
        );
        return;
    }

    /* Our current VL should stay the same */
    if ((*data).rdvl)() != (*data).max_vl {
        ksft_test_result_fail(c"%s VL changed by _ONEXEC prctl()\n".as_ptr(), (*data).name);
        return;
    }

    /* Check that the child inherited our VL */
    child_vl = get_child_rdvl(data);
    if child_vl != (*data).min_vl {
        ksft_test_result_fail(
            c"Set %d _ONEXEC but child VL is %d\n".as_ptr(),
            (*data).min_vl,
            child_vl,
        );
        return;
    }

    ksft_test_result_pass(c"%s vector length set on exec\n".as_ptr(), (*data).name);

    file_write_integer((*data).default_vl_file, (*data).default_vl);
}

/* For each VQ verify that setting via prctl() does the right thing */
unsafe fn prctl_set_all_vqs(data: *mut VecData) {
    let mut ret: c_int;
    let mut vq: c_int;
    let mut vl: c_int;
    let mut new_vl: c_int;
    let mut i: usize;
    let mut orig_vls: [c_int; 2] = [0; 2];
    let mut errors: c_int = 0;

    if (*data).min_vl == 0 || (*data).max_vl == 0 {
        ksft_test_result_skip(
            c"%s Failed to enumerate VLs, not testing VL setting\n".as_ptr(),
            (*data).name,
        );
        return;
    }

    i = 0;
    while i < VEC_DATA.len() {
        if !vec_type_supported(&mut VEC_DATA[i] as *mut VecData) {
            i += 1;
            continue;
        }
        orig_vls[i] = (VEC_DATA[i].rdvl)();
        i += 1;
    }

    vq = SVE_VQ_MIN;
    while vq <= SVE_VQ_MAX {
        vl = sve_vl_from_vq(vq);

        /* Attempt to set the VL */
        ret = prctl((*data).prctl_set, vl);
        if ret < 0 {
            errors += 1;
            ksft_print_msg(
                c"%s prctl set failed for %d: %d (%s)\n".as_ptr(),
                (*data).name,
                vl,
                errno,
                strerror(errno),
            );
            vq += 1;
            continue;
        }

        new_vl = ret & PR_SVE_VL_LEN_MASK;

        /* Check that we actually have the reported new VL */
        if ((*data).rdvl)() != new_vl {
            ksft_print_msg(
                c"Set %s VL %d but RDVL reports %d\n".as_ptr(),
                (*data).name,
                new_vl,
                ((*data).rdvl)(),
            );
            errors += 1;
        }

        /* Did any other VLs change? */
        i = 0;
        while i < VEC_DATA.len() {
            if (&mut VEC_DATA[i] as *mut VecData) == data {
                i += 1;
                continue;
            }

            if !vec_type_supported(&mut VEC_DATA[i] as *mut VecData) {
                i += 1;
                continue;
            }

            if (VEC_DATA[i].rdvl)() != orig_vls[i] {
                ksft_print_msg(
                    c"%s VL changed from %d to %d\n".as_ptr(),
                    VEC_DATA[i].name,
                    orig_vls[i],
                    (VEC_DATA[i].rdvl)(),
                );
                errors += 1;
            }
            i += 1;
        }

        /* Was that the VL we asked for? */
        if new_vl == vl {
            vq += 1;
            continue;
        }

        /* Should round up to the minimum VL if below it */
        if vl < (*data).min_vl {
            if new_vl != (*data).min_vl {
                ksft_print_msg(
                    c"%s VL %d returned %d not minimum %d\n".as_ptr(),
                    (*data).name,
                    vl,
                    new_vl,
                    (*data).min_vl,
                );
                errors += 1;
            }

            vq += 1;
            continue;
        }

        /* Should round down to maximum VL if above it */
        if vl > (*data).max_vl {
            if new_vl != (*data).max_vl {
                ksft_print_msg(
                    c"%s VL %d returned %d not maximum %d\n".as_ptr(),
                    (*data).name,
                    vl,
                    new_vl,
                    (*data).max_vl,
                );
                errors += 1;
            }

            vq += 1;
            continue;
        }

        /* Otherwise we should've rounded down */
        if !(new_vl < vl) {
            ksft_print_msg(
                c"%s VL %d returned %d, did not round down\n".as_ptr(),
                (*data).name,
                vl,
                new_vl,
            );
            errors += 1;

            vq += 1;
            continue;
        }

        vq += 1;
    }

    ksft_test_result(
        errors == 0,
        c"%s prctl() set all VLs, %d errors\n".as_ptr(),
        (*data).name,
        errors,
    );
}

type TestType = unsafe fn(*mut VecData);

static TESTS: [TestType; 10] = [
    /*
     * The default/min/max tests must be first and in this order
     * to provide data for other tests.
     */
    proc_read_default,
    proc_write_min,
    proc_write_max,
    prctl_get,
    prctl_set_same,
    prctl_set,
    prctl_set_no_child,
    prctl_set_for_child,
    prctl_set_onexec,
    prctl_set_all_vqs,
];

#[inline]
unsafe fn smstart() {
    asm!("msr S0_3_C4_C7_3, xzr", options(nostack, preserves_flags));
}

#[inline]
unsafe fn smstart_sm() {
    asm!("msr S0_3_C4_C3_3, xzr", options(nostack, preserves_flags));
}

#[inline]
unsafe fn smstop() {
    asm!("msr S0_3_C4_C6_3, xzr", options(nostack, preserves_flags));
}

/*
 * Verify we can change the SVE vector length while SME is active and
 * continue to use SME afterwards.
 */
unsafe fn change_sve_with_za() {
    let sve_data: *mut VecData = &mut VEC_DATA[VEC_SVE] as *mut VecData;
    let mut pass: bool = true;
    let mut ret: c_int;
    let mut i: c_int;

    if (*sve_data).min_vl == (*sve_data).max_vl {
        ksft_print_msg(c"Only one SVE VL supported, can't change\n".as_ptr());
        ksft_test_result_skip(c"change_sve_while_sme\n".as_ptr());
        return;
    }

    /* Ensure we will trigger a change when we set the maximum */
    ret = prctl((*sve_data).prctl_set, (*sve_data).min_vl);
    if ret != (*sve_data).min_vl {
        ksft_print_msg(
            c"Failed to set SVE VL %d: %d\n".as_ptr(),
            (*sve_data).min_vl,
            ret,
        );
        pass = false;
    }

    /* Enable SM and ZA */
    smstart();

    /* Trigger another VL change */
    ret = prctl((*sve_data).prctl_set, (*sve_data).max_vl);
    if ret != (*sve_data).max_vl {
        ksft_print_msg(
            c"Failed to set SVE VL %d: %d\n".as_ptr(),
            (*sve_data).max_vl,
            ret,
        );
        pass = false;
    }

    /*
     * Spin for a bit with SM enabled to try to trigger another
     * save/restore.  We can't use syscalls without exiting
     * streaming mode.
     */
    i = 0;
    while i < 100000000 {
        smstart_sm();
        i += 1;
    }

    /*
     * TODO: Verify that ZA was preserved over the VL change and
     * spin.
     */

    /* Clean up after ourselves */
    smstop();
    ret = prctl((*sve_data).prctl_set, (*sve_data).default_vl);
    if ret != (*sve_data).default_vl {
        ksft_print_msg(
            c"Failed to restore SVE VL %d: %d\n".as_ptr(),
            (*sve_data).default_vl,
            ret,
        );
        pass = false;
    }

    ksft_test_result(pass, c"change_sve_with_za\n".as_ptr());
}

type TestAllType = unsafe fn();

#[repr(C)]
struct AllTypesTest {
    name: *const c_char,
    test: TestAllType,
}

static ALL_TYPES_TESTS: [AllTypesTest; 1] = [AllTypesTest {
    name: c"change_sve_with_za".as_ptr(),
    test: change_sve_with_za,
}];

#[no_mangle]
pub unsafe extern "C" fn main() -> c_int {
    let mut all_supported: bool = true;
    let mut i: usize;
    let mut j: usize;

    ksft_print_header();
    ksft_set_plan((TESTS.len() * VEC_DATA.len() + ALL_TYPES_TESTS.len()) as c_int);

    i = 0;
    while i < VEC_DATA.len() {
        let data: *mut VecData = &mut VEC_DATA[i] as *mut VecData;
        let supported: bool;

        supported = vec_type_supported(data);
        if !supported {
            all_supported = false;
        }

        j = 0;
        while j < TESTS.len() {
            if supported {
                TESTS[j](data);
            } else {
                ksft_test_result_skip(c"%s not supported\n".as_ptr(), (*data).name);
            }
            j += 1;
        }

        i += 1;
    }

    i = 0;
    while i < ALL_TYPES_TESTS.len() {
        if all_supported {
            (ALL_TYPES_TESTS[i].test)();
        } else {
            ksft_test_result_skip(c"%s\n".as_ptr(), ALL_TYPES_TESTS[i].name);
        }

        i += 1;
    }

    ksft_exit_pass();
}

fn wifexited(status: c_int) -> bool {
    (status & 0x7f) == 0
}

fn wexitstatus(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}

fn sve_vl_from_vq(vq: c_int) -> c_int {
    vq * 16
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
