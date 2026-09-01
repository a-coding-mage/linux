// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2015-2021 ARM Limited.
 * Original author: Dave Martin <Dave.Martin@arm.com>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

type size_t = usize;
type pid_t = c_int;

// C headers used by the original file provide these declarations, constants,
// and layout helpers.
unsafe extern "C" {
    fn ptrace(request: c_int, ...) -> c_long;
    fn raise(sig: c_int) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn realloc(ptr: *mut c_void, size: size_t) -> *mut c_void;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn random() -> c_long;
    fn srandom(seed: c_uint);
    fn getpid() -> pid_t;
    fn fork() -> pid_t;
    fn wait(wstatus: *mut c_int) -> pid_t;
    fn perror(s: *const c_char);
    fn prctl(option: c_int, ...) -> c_int;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn printf(format: *const c_char, ...) -> c_int;
    fn __errno_location() -> *mut c_int;

    fn ksft_exit_fail_msg(msg: *const c_char, ...) -> !;
    fn ksft_perror(msg: *const c_char);
    fn ksft_test_result(condition: bool, msg: *const c_char, ...);
    fn ksft_test_result_fail(msg: *const c_char, ...);
    fn ksft_test_result_pass(msg: *const c_char, ...);
    fn ksft_test_result_skip(msg: *const c_char, ...);
    fn ksft_print_msg(msg: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_uint);
    fn ksft_print_cnts();

    fn sve_vl_from_vq(vq: c_uint) -> c_uint;
    fn sve_vq_from_vl(vl: c_uint) -> c_uint;
    fn SVE_PT_SIZE(vq: c_uint, flags: c_uint) -> size_t;
    fn SVE_PT_FPSIMD_SIZE(vq: c_uint, flags: c_uint) -> size_t;
    fn SVE_PT_SVE_SIZE(vq: c_uint, flags: c_uint) -> size_t;
    fn SVE_PT_SVE_ZREG_OFFSET(vq: c_uint, n: c_int) -> size_t;
    fn SVE_PT_SVE_ZREG_SIZE(vq: c_uint) -> size_t;
    fn SVE_PT_SVE_PREG_OFFSET(vq: c_uint, n: c_int) -> size_t;
    fn SVE_PT_SVE_PREG_SIZE(vq: c_uint) -> size_t;
    fn SVE_PT_SVE_FPSR_OFFSET(vq: c_uint) -> size_t;
    fn SVE_PT_SVE_FPCR_OFFSET(vq: c_uint) -> size_t;
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct siginfo_t {
    si_signo: c_int,
    si_errno: c_int,
    si_code: c_int,
    si_pid: pid_t,
}

#[repr(C)]
struct user_sve_header {
    size: c_uint,
    max_size: c_uint,
    vl: c_uint,
    max_vl: c_uint,
    flags: c_uint,
    reserved: c_uint,
}

#[repr(C)]
struct user_fpsimd_state {
    vregs: [u128; 32],
    fpsr: u32,
    fpcr: u32,
}

#[repr(C)]
struct vec_type {
    name: *const c_char,
    hwcap_type: c_ulong,
    hwcap: c_ulong,
    regset: c_int,
    prctl_set: c_int,
}

const EXIT_SUCCESS: c_int = 0;
const EXIT_FAILURE: c_int = 1;
const ENOMEM: c_int = 12;
const ESRCH: c_int = 3;
const EINVAL: c_int = 22;
const SIGSTOP: c_int = 19;
const SIGKILL: c_int = 9;
const SI_TKILL: c_int = -6;
const PTRACE_TRACEME: c_int = 0;
const PTRACE_CONT: c_int = 7;
const PTRACE_GETREGSET: c_int = 0x4204;
const PTRACE_SETREGSET: c_int = 0x4205;
const PTRACE_GETSIGINFO: c_int = 0x4202;
const NT_PRFPREG: c_int = 2;
const AT_HWCAP: c_ulong = 16;
const AT_HWCAP2: c_ulong = 26;
const HWCAP_SVE: c_ulong = 1 << 22;
const HWCAP2_SME: c_ulong = 1 << 23;
const PR_SVE_SET_VL: c_int = 50;
const PR_SME_SET_VL: c_int = 63;
const SVE_VQ_MIN: c_uint = 1;
const SVE_VQ_BYTES: c_uint = 16;
const SVE_VL_MIN: c_uint = 16;
const SVE_VL_MAX: c_uint = 8192;
const SVE_PT_REGS_FPSIMD: c_uint = 1;
const SVE_PT_REGS_SVE: c_uint = 2;
const SVE_PT_REGS_MASK: c_uint = 3;
const SVE_PT_VL_INHERIT: c_uint = 1 << 17;
const SVE_PT_FPSIMD_OFFSET: size_t = 0x10;
const SVE_PT_SVE_OFFSET: size_t = 0x10;
const SVE_PT_SVE_FPSR_SIZE: size_t = 4;
const SVE_PT_SVE_FPCR_SIZE: size_t = 4;
const __SVE_NUM_ZREGS: c_int = 32;
const __SVE_NUM_PREGS: c_int = 16;
const __BIG_ENDIAN: c_int = 4321;
const __BYTE_ORDER: c_int = 1234;

/* <linux/elf.h> and <sys/auxv.h> don't like each other, so: */
const NT_ARM_SVE: c_int = 0x405;
const NT_ARM_SSVE: c_int = 0x40b;

/*
 * The architecture defines the maximum VQ as 16 but for extensibility
 * the kernel specifies the SVE_VQ_MAX as 512 resulting in us running
 * a *lot* more tests than are useful if we use it.  Until the
 * architecture is extended let's limit our coverage to what is
 * currently allowed, plus one extra to ensure we cover constraining
 * the VL as expected.
 */
const TEST_VQ_MAX: c_uint = 17;

static vec_types: [vec_type; 2] = [
    vec_type {
        name: b"SVE\0".as_ptr() as *const c_char,
        hwcap_type: AT_HWCAP,
        hwcap: HWCAP_SVE,
        regset: NT_ARM_SVE,
        prctl_set: PR_SVE_SET_VL,
    },
    vec_type {
        name: b"Streaming SVE\0".as_ptr() as *const c_char,
        hwcap_type: AT_HWCAP2,
        hwcap: HWCAP2_SME,
        regset: NT_ARM_SSVE,
        prctl_set: PR_SME_SET_VL,
    },
];

const VL_TESTS: usize = (((TEST_VQ_MAX - SVE_VQ_MIN) + 1) as usize) * 4;
const FLAG_TESTS: usize = 4;
const FPSIMD_TESTS: usize = 2;
const EXPECTED_TESTS: usize = (VL_TESTS + FLAG_TESTS + FPSIMD_TESTS) * 2;

unsafe fn errno_value() -> c_int {
    unsafe { *__errno_location() }
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    (status & 0x7f) == 0
}

unsafe fn WIFSIGNALED(status: c_int) -> bool {
    (((status & 0x7f) + 1) >> 1) > 0
}

unsafe fn WIFSTOPPED(status: c_int) -> bool {
    (status & 0xff) == 0x7f
}

unsafe fn WSTOPSIG(status: c_int) -> c_int {
    (status >> 8) & 0xff
}

unsafe fn fill_buf(buf: *mut c_char, size: size_t) {
    let mut i: c_int = 0;

    while (i as size_t) < size {
        unsafe {
            *buf.add(i as usize) = random() as c_char;
        }
        i += 1;
    }
}

unsafe fn do_child() -> c_int {
    unsafe {
        if ptrace(PTRACE_TRACEME, -1, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) != 0 {
            ksft_exit_fail_msg(
                c"ptrace(PTRACE_TRACEME) failed: %s (%d)\n".as_ptr(),
                strerror(errno_value()),
                errno_value(),
            );
        }

        if raise(SIGSTOP) != 0 {
            ksft_exit_fail_msg(
                c"raise(SIGSTOP) failed: %s (%d)\n".as_ptr(),
                strerror(errno_value()),
                errno_value(),
            );
        }
    }

    EXIT_SUCCESS
}

unsafe fn get_fpsimd(pid: pid_t, fpsimd: *mut user_fpsimd_state) -> c_int {
    let mut iov = iovec {
        iov_base: fpsimd as *mut c_void,
        iov_len: size_of::<user_fpsimd_state>(),
    };
    let ret: c_int = unsafe { ptrace(PTRACE_GETREGSET, pid, NT_PRFPREG, &mut iov) as c_int };
    if ret == -1 {
        unsafe { ksft_perror(c"ptrace(PTRACE_GETREGSET)".as_ptr()) };
    }
    ret
}

unsafe fn set_fpsimd(pid: pid_t, fpsimd: *mut user_fpsimd_state) -> c_int {
    let mut iov = iovec {
        iov_base: fpsimd as *mut c_void,
        iov_len: size_of::<user_fpsimd_state>(),
    };
    let ret: c_int = unsafe { ptrace(PTRACE_SETREGSET, pid, NT_PRFPREG, &mut iov) as c_int };
    if ret == -1 {
        unsafe { ksft_perror(c"ptrace(PTRACE_SETREGSET)".as_ptr()) };
    }
    ret
}

unsafe fn get_sve(
    pid: pid_t,
    type_: *const vec_type,
    buf: *mut *mut c_void,
    size: *mut size_t,
) -> *mut user_sve_header {
    let mut sve: *mut user_sve_header;
    let mut p: *mut c_void;
    let mut sz: size_t = size_of::<user_sve_header>();
    let mut ret: c_int;

    loop {
        unsafe {
            if *size < sz {
                p = realloc(*buf, sz);
                if p.is_null() {
                    *__errno_location() = ENOMEM;
                    return ptr::null_mut();
                }

                *buf = p;
                *size = sz;
            }

            let mut iov = iovec {
                iov_base: *buf,
                iov_len: sz,
            };
            ret = ptrace(PTRACE_GETREGSET, pid, (*type_).regset, &mut iov) as c_int;
            if ret != 0 {
                ksft_perror(c"ptrace(PTRACE_GETREGSET)".as_ptr());
                return ptr::null_mut();
            }

            sve = *buf as *mut user_sve_header;
            if (*sve).size as size_t <= sz {
                break;
            }

            sz = (*sve).size as size_t;
        }
    }

    sve
}

unsafe fn set_sve(pid: pid_t, type_: *const vec_type, sve: *const user_sve_header) -> c_int {
    let mut iov = unsafe {
        iovec {
            iov_base: sve as *mut c_void,
            iov_len: (*sve).size as size_t,
        }
    };
    let ret: c_int = unsafe { ptrace(PTRACE_SETREGSET, pid, (*type_).regset, &mut iov) as c_int };
    if ret == -1 {
        unsafe { ksft_perror(c"ptrace(PTRACE_SETREGSET)".as_ptr()) };
    }
    ret
}

/* A read operation fails */
unsafe fn read_fails(child: pid_t, type_: *const vec_type) {
    let mut new_sve: *mut user_sve_header = ptr::null_mut();
    let mut new_sve_size: size_t = 0;
    let ret: *mut c_void = unsafe {
        get_sve(
            child,
            type_,
            &mut new_sve as *mut *mut user_sve_header as *mut *mut c_void,
            &mut new_sve_size,
        ) as *mut c_void
    };

    unsafe {
        ksft_test_result(
            ret.is_null(),
            c"%s unsupported read fails\n".as_ptr(),
            (*type_).name,
        );
        free(new_sve as *mut c_void);
    }
}

/* A write operation fails */
unsafe fn write_fails(child: pid_t, type_: *const vec_type) {
    let mut sve: user_sve_header = unsafe { core::mem::zeroed() };

    /* Just the header, no data */
    unsafe {
        memset(
            &mut sve as *mut user_sve_header as *mut c_void,
            0,
            size_of::<user_sve_header>(),
        );
    }
    sve.size = size_of::<user_sve_header>() as c_uint;
    sve.flags = SVE_PT_REGS_SVE;
    sve.vl = SVE_VL_MIN;
    let ret = unsafe { set_sve(child, type_, &sve) };

    unsafe {
        ksft_test_result(
            ret != 0,
            c"%s unsupported write fails\n".as_ptr(),
            (*type_).name,
        );
    }
}

/* Validate setting and getting the inherit flag */
unsafe fn ptrace_set_get_inherit(child: pid_t, type_: *const vec_type) {
    let mut sve: user_sve_header = unsafe { core::mem::zeroed() };
    let mut new_sve: *mut user_sve_header = ptr::null_mut();
    let mut new_sve_size: size_t = 0;
    let mut ret: c_int;

    /* First set the flag */
    unsafe {
        memset(&mut sve as *mut _ as *mut c_void, 0, size_of::<user_sve_header>());
    }
    sve.size = size_of::<user_sve_header>() as c_uint;
    sve.vl = unsafe { sve_vl_from_vq(SVE_VQ_MIN) };
    sve.flags = SVE_PT_VL_INHERIT | SVE_PT_REGS_SVE;
    ret = unsafe { set_sve(child, type_, &sve) };
    if ret != 0 {
        unsafe {
            ksft_test_result_fail(
                c"Failed to set %s SVE_PT_VL_INHERIT\n".as_ptr(),
                (*type_).name,
            );
        }
        return;
    }

    /*
     * Read back the new register state and verify that we have
     * set the flags we expected.
     */
    if unsafe {
        get_sve(
            child,
            type_,
            &mut new_sve as *mut *mut user_sve_header as *mut *mut c_void,
            &mut new_sve_size,
        )
        .is_null()
    } {
        unsafe {
            ksft_test_result_fail(c"Failed to read %s SVE flags\n".as_ptr(), (*type_).name);
        }
        return;
    }

    unsafe {
        ksft_test_result(
            ((*new_sve).flags & SVE_PT_VL_INHERIT) != 0,
            c"%s SVE_PT_VL_INHERIT set\n".as_ptr(),
            (*type_).name,
        );
    }

    /* Now clear */
    sve.flags &= !SVE_PT_VL_INHERIT;
    ret = unsafe { set_sve(child, type_, &sve) };
    if ret != 0 {
        unsafe {
            ksft_test_result_fail(
                c"Failed to clear %s SVE_PT_VL_INHERIT\n".as_ptr(),
                (*type_).name,
            );
        }
        return;
    }

    if unsafe {
        get_sve(
            child,
            type_,
            &mut new_sve as *mut *mut user_sve_header as *mut *mut c_void,
            &mut new_sve_size,
        )
        .is_null()
    } {
        unsafe {
            ksft_test_result_fail(c"Failed to read %s SVE flags\n".as_ptr(), (*type_).name);
        }
        return;
    }

    unsafe {
        ksft_test_result(
            !((*new_sve).flags & SVE_PT_VL_INHERIT != 0),
            c"%s SVE_PT_VL_INHERIT cleared\n".as_ptr(),
            (*type_).name,
        );

        free(new_sve as *mut c_void);
    }
}

/* Validate attempting to set the specfied VL via ptrace */
unsafe fn ptrace_set_get_vl(
    child: pid_t,
    type_: *const vec_type,
    vl: c_uint,
    supported: *mut bool,
) {
    let mut sve: user_sve_header = unsafe { core::mem::zeroed() };
    let mut new_sve: *mut user_sve_header = ptr::null_mut();
    let mut new_sve_size: size_t = 0;
    let ret: c_int;
    let prctl_vl: c_int;

    unsafe {
        *supported = false;
    }

    /* Check if the VL is supported in this process */
    prctl_vl = unsafe { prctl((*type_).prctl_set, vl) };
    if prctl_vl == -1 {
        unsafe {
            ksft_exit_fail_msg(
                c"prctl(PR_%s_SET_VL) failed: %s (%d)\n".as_ptr(),
                (*type_).name,
                strerror(errno_value()),
                errno_value(),
            );
        }
    }

    /* If the VL is not supported then a supported VL will be returned */
    unsafe {
        *supported = prctl_vl as c_uint == vl;
    }

    /* Set the VL by doing a set with no register payload */
    unsafe {
        memset(&mut sve as *mut _ as *mut c_void, 0, size_of::<user_sve_header>());
    }
    sve.size = size_of::<user_sve_header>() as c_uint;
    sve.flags = SVE_PT_REGS_SVE;
    sve.vl = vl;
    ret = unsafe { set_sve(child, type_, &sve) };
    if ret != 0 {
        unsafe {
            ksft_test_result_fail(c"Failed to set %s VL %u\n".as_ptr(), (*type_).name, vl);
        }
        return;
    }

    /*
     * Read back the new register state and verify that we have the
     * same VL that we got from prctl() on ourselves.
     */
    if unsafe {
        get_sve(
            child,
            type_,
            &mut new_sve as *mut *mut user_sve_header as *mut *mut c_void,
            &mut new_sve_size,
        )
        .is_null()
    } {
        unsafe {
            ksft_test_result_fail(c"Failed to read %s VL %u\n".as_ptr(), (*type_).name, vl);
        }
        return;
    }

    unsafe {
        ksft_test_result(
            (*new_sve).vl == prctl_vl as c_uint,
            c"Set %s VL %u\n".as_ptr(),
            (*type_).name,
            vl,
        );

        free(new_sve as *mut c_void);
    }
}

unsafe fn check_u32(
    vl: c_uint,
    reg: *const c_char,
    in_: *mut u32,
    out: *mut u32,
    errors: *mut c_int,
) {
    unsafe {
        if *in_ != *out {
            printf(c"# VL %d %s wrote %x read %x\n".as_ptr(), vl, reg, *in_, *out);
            *errors += 1;
        }
    }
}

/* Set out of range VLs */
unsafe fn ptrace_set_vl_ranges(child: pid_t, type_: *const vec_type) {
    let mut sve: user_sve_header = unsafe { core::mem::zeroed() };
    let mut ret: c_int;

    unsafe {
        memset(&mut sve as *mut _ as *mut c_void, 0, size_of::<user_sve_header>());
    }
    sve.flags = SVE_PT_REGS_SVE;
    sve.size = size_of::<user_sve_header>() as c_uint;

    ret = unsafe { set_sve(child, type_, &sve) };
    unsafe {
        ksft_test_result(ret != 0, c"%s Set invalid VL 0\n".as_ptr(), (*type_).name);
    }

    sve.vl = SVE_VL_MAX + SVE_VQ_BYTES;
    ret = unsafe { set_sve(child, type_, &sve) };
    unsafe {
        ksft_test_result(
            ret != 0,
            c"%s Set invalid VL %d\n".as_ptr(),
            (*type_).name,
            SVE_VL_MAX + SVE_VQ_BYTES,
        );
    }
}

/* Access the FPSIMD registers via the SVE regset */
unsafe fn ptrace_sve_fpsimd(child: pid_t, type_: *const vec_type) {
    let svebuf: *mut c_void;
    let sve: *mut user_sve_header;
    let fpsimd: *mut user_fpsimd_state;
    let mut new_fpsimd: user_fpsimd_state = unsafe { core::mem::zeroed() };
    let mut i: c_uint;
    let mut j: c_uint;
    let mut p: *mut u8;
    let ret: c_int;

    svebuf = unsafe { malloc(SVE_PT_SIZE(0, SVE_PT_REGS_FPSIMD)) };
    if svebuf.is_null() {
        unsafe { ksft_test_result_fail(c"Failed to allocate FPSIMD buffer\n".as_ptr()) };
        return;
    }

    unsafe {
        memset(svebuf, 0, SVE_PT_SIZE(0, SVE_PT_REGS_FPSIMD));
    }
    sve = svebuf as *mut user_sve_header;
    unsafe {
        (*sve).flags = SVE_PT_REGS_FPSIMD;
        (*sve).size = SVE_PT_SIZE(0, SVE_PT_REGS_FPSIMD) as c_uint;
        (*sve).vl = 16; /* We don't care what the VL is */
    }

    /* Try to set a known FPSIMD state via PT_REGS_SVE */
    fpsimd = unsafe { (sve as *mut c_char).add(SVE_PT_FPSIMD_OFFSET) as *mut user_fpsimd_state };
    i = 0;
    while i < 32 {
        p = unsafe { &mut (*fpsimd).vregs[i as usize] as *mut u128 as *mut u8 };

        j = 0;
        while (j as usize) < size_of::<u128>() {
            unsafe {
                *p.add(j as usize) = j as u8;
            }
            j += 1;
        }
        i += 1;
    }

    /* This should only succeed for SVE */
    ret = unsafe { set_sve(child, type_, sve) };
    unsafe {
        ksft_test_result(
            ((*type_).regset == NT_ARM_SVE) == (ret == 0),
            c"%s FPSIMD set via SVE: %d\n".as_ptr(),
            (*type_).name,
            ret,
        );
    }
    if ret != 0 {
        unsafe { free(svebuf) };
        return;
    }

    /* Verify via the FPSIMD regset */
    if unsafe { get_fpsimd(child, &mut new_fpsimd) } != 0 {
        unsafe {
            ksft_test_result_fail(c"get_fpsimd(): %s\n".as_ptr(), strerror(errno_value()));
            free(svebuf);
        }
        return;
    }
    unsafe {
        if memcmp(
            fpsimd as *const c_void,
            &new_fpsimd as *const user_fpsimd_state as *const c_void,
            size_of::<user_fpsimd_state>(),
        ) == 0
        {
            ksft_test_result_pass(c"%s get_fpsimd() gave same state\n".as_ptr(), (*type_).name);
        } else {
            ksft_test_result_fail(c"%s get_fpsimd() gave different state\n".as_ptr(), (*type_).name);
        }

        free(svebuf);
    }
}

/* Write the FPSIMD registers via the SVE regset when SVE is not supported */
unsafe fn ptrace_sve_fpsimd_no_sve(child: pid_t) {
    let svebuf: *mut c_void;
    let sve: *mut user_sve_header;
    let fpsimd: *mut user_fpsimd_state;
    let mut new_fpsimd: user_fpsimd_state = unsafe { core::mem::zeroed() };
    let mut i: c_uint;
    let mut j: c_uint;
    let mut p: *mut u8;
    let ret: c_int;

    svebuf = unsafe { malloc(SVE_PT_SIZE(0, SVE_PT_REGS_FPSIMD)) };
    if svebuf.is_null() {
        unsafe { ksft_test_result_fail(c"Failed to allocate FPSIMD buffer\n".as_ptr()) };
        return;
    }

    /* On a system without SVE the VL should be set to 0 */
    unsafe {
        memset(svebuf, 0, SVE_PT_SIZE(0, SVE_PT_REGS_FPSIMD));
    }
    sve = svebuf as *mut user_sve_header;
    unsafe {
        (*sve).flags = SVE_PT_REGS_FPSIMD;
        (*sve).size = SVE_PT_SIZE(0, SVE_PT_REGS_FPSIMD) as c_uint;
        (*sve).vl = 0;
    }

    /* Try to set a known FPSIMD state via PT_REGS_SVE */
    fpsimd = unsafe { (sve as *mut c_char).add(SVE_PT_FPSIMD_OFFSET) as *mut user_fpsimd_state };
    i = 0;
    while i < 32 {
        p = unsafe { &mut (*fpsimd).vregs[i as usize] as *mut u128 as *mut u8 };

        j = 0;
        while (j as usize) < size_of::<u128>() {
            unsafe {
                *p.add(j as usize) = j as u8;
            }
            j += 1;
        }
        i += 1;
    }

    ret = unsafe { set_sve(child, &vec_types[0], sve) };
    unsafe { ksft_test_result(ret == 0, c"FPSIMD write via SVE\n".as_ptr()) };
    if ret != 0 {
        unsafe {
            ksft_test_result_skip(c"Verify FPSIMD write via SVE\n".as_ptr());
            free(svebuf);
        }
        return;
    }

    /* Verify via the FPSIMD regset */
    if unsafe { get_fpsimd(child, &mut new_fpsimd) } != 0 {
        unsafe {
            ksft_test_result_skip(c"Verify FPSIMD write via SVE\n".as_ptr());
            free(svebuf);
        }
        return;
    }
    unsafe {
        ksft_test_result(
            memcmp(
                fpsimd as *const c_void,
                &new_fpsimd as *const user_fpsimd_state as *const c_void,
                size_of::<user_fpsimd_state>(),
            ) == 0,
            c"Verify FPSIMD write via SVE\n".as_ptr(),
        );

        free(svebuf);
    }
}

/* Validate attempting to set SVE data and read SVE data */
unsafe fn ptrace_set_sve_get_sve_data(child: pid_t, type_: *const vec_type, vl: c_uint) {
    let write_buf: *mut c_void;
    let mut read_buf: *mut c_void = ptr::null_mut();
    let write_sve: *mut user_sve_header;
    let read_sve: *mut user_sve_header;
    let mut read_sve_size: size_t = 0;
    let vq: c_uint = unsafe { sve_vq_from_vl(vl) };
    let mut ret: c_int;
    let mut i: c_int;
    let data_size: size_t;
    let mut errors: c_int = 0;

    data_size = unsafe { SVE_PT_SVE_OFFSET + SVE_PT_SVE_SIZE(vq, SVE_PT_REGS_SVE) };
    write_buf = unsafe { malloc(data_size) };
    if write_buf.is_null() {
        unsafe {
            ksft_test_result_fail(
                c"Error allocating %ld byte buffer for %s VL %u\n".as_ptr(),
                data_size,
                (*type_).name,
                vl,
            );
        }
        return;
    }
    write_sve = write_buf as *mut user_sve_header;

    /* Set up some data and write it out */
    unsafe {
        memset(write_sve as *mut c_void, 0, data_size);
        (*write_sve).size = data_size as c_uint;
        (*write_sve).vl = vl;
        (*write_sve).flags = SVE_PT_REGS_SVE;
    }

    i = 0;
    while i < __SVE_NUM_ZREGS {
        unsafe {
            fill_buf(
                (write_buf as *mut c_char).add(SVE_PT_SVE_ZREG_OFFSET(vq, i)),
                SVE_PT_SVE_ZREG_SIZE(vq),
            );
        }
        i += 1;
    }

    i = 0;
    while i < __SVE_NUM_PREGS {
        unsafe {
            fill_buf(
                (write_buf as *mut c_char).add(SVE_PT_SVE_PREG_OFFSET(vq, i)),
                SVE_PT_SVE_PREG_SIZE(vq),
            );
        }
        i += 1;
    }

    unsafe {
        fill_buf(
            (write_buf as *mut c_char).add(SVE_PT_SVE_FPSR_OFFSET(vq)),
            SVE_PT_SVE_FPSR_SIZE,
        );
        fill_buf(
            (write_buf as *mut c_char).add(SVE_PT_SVE_FPCR_OFFSET(vq)),
            SVE_PT_SVE_FPCR_SIZE,
        );
    }

    /* TODO: Generate a valid FFR pattern */

    ret = unsafe { set_sve(child, type_, write_sve) };
    if ret != 0 {
        unsafe {
            ksft_test_result_fail(c"Failed to set %s VL %u data\n".as_ptr(), (*type_).name, vl);
            free(write_buf);
        }
        return;
    }

    /* Read the data back */
    if unsafe { get_sve(child, type_, &mut read_buf, &mut read_sve_size).is_null() } {
        unsafe {
            ksft_test_result_fail(c"Failed to read %s VL %u data\n".as_ptr(), (*type_).name, vl);
            free(write_buf);
        }
        return;
    }
    read_sve = read_buf as *mut user_sve_header;

    /* We might read more data if there's extensions we don't know */
    if unsafe { (*read_sve).size < (*write_sve).size } {
        unsafe {
            ksft_test_result_fail(
                c"%s wrote %d bytes, only read %d\n".as_ptr(),
                (*type_).name,
                (*write_sve).size,
                (*read_sve).size,
            );
            free(read_buf);
            free(write_buf);
        }
        return;
    }

    i = 0;
    while i < __SVE_NUM_ZREGS {
        unsafe {
            if memcmp(
                (write_buf as *mut c_char).add(SVE_PT_SVE_ZREG_OFFSET(vq, i)) as *const c_void,
                (read_buf as *mut c_char).add(SVE_PT_SVE_ZREG_OFFSET(vq, i)) as *const c_void,
                SVE_PT_SVE_ZREG_SIZE(vq),
            ) != 0
            {
                printf(c"# Mismatch in %u Z%d\n".as_ptr(), vl, i);
                errors += 1;
            }
        }
        i += 1;
    }

    i = 0;
    while i < __SVE_NUM_PREGS {
        unsafe {
            if memcmp(
                (write_buf as *mut c_char).add(SVE_PT_SVE_PREG_OFFSET(vq, i)) as *const c_void,
                (read_buf as *mut c_char).add(SVE_PT_SVE_PREG_OFFSET(vq, i)) as *const c_void,
                SVE_PT_SVE_PREG_SIZE(vq),
            ) != 0
            {
                printf(c"# Mismatch in %u P%d\n".as_ptr(), vl, i);
                errors += 1;
            }
        }
        i += 1;
    }

    unsafe {
        check_u32(
            vl,
            c"FPSR".as_ptr(),
            (write_buf as *mut c_char).add(SVE_PT_SVE_FPSR_OFFSET(vq)) as *mut u32,
            (read_buf as *mut c_char).add(SVE_PT_SVE_FPSR_OFFSET(vq)) as *mut u32,
            &mut errors,
        );
        check_u32(
            vl,
            c"FPCR".as_ptr(),
            (write_buf as *mut c_char).add(SVE_PT_SVE_FPCR_OFFSET(vq)) as *mut u32,
            (read_buf as *mut c_char).add(SVE_PT_SVE_FPCR_OFFSET(vq)) as *mut u32,
            &mut errors,
        );

        ksft_test_result(
            errors == 0,
            c"Set and get %s data for VL %u\n".as_ptr(),
            (*type_).name,
            vl,
        );

        free(read_buf);
        free(write_buf);
    }
}

/* Validate attempting to set SVE data and read it via the FPSIMD regset */
unsafe fn ptrace_set_sve_get_fpsimd_data(child: pid_t, type_: *const vec_type, vl: c_uint) {
    let write_buf: *mut c_void;
    let write_sve: *mut user_sve_header;
    let vq: c_uint = unsafe { sve_vq_from_vl(vl) };
    let mut fpsimd_state: user_fpsimd_state = unsafe { core::mem::zeroed() };
    let mut ret: c_int;
    let mut i: c_int;
    let data_size: size_t;
    let mut errors: c_int = 0;

    if __BYTE_ORDER == __BIG_ENDIAN {
        unsafe { ksft_test_result_skip(c"Big endian not supported\n".as_ptr()) };
        return;
    }

    data_size = unsafe { SVE_PT_SVE_OFFSET + SVE_PT_SVE_SIZE(vq, SVE_PT_REGS_SVE) };
    write_buf = unsafe { malloc(data_size) };
    if write_buf.is_null() {
        unsafe {
            ksft_test_result_fail(
                c"Error allocating %ld byte buffer for %s VL %u\n".as_ptr(),
                data_size,
                (*type_).name,
                vl,
            );
        }
        return;
    }
    write_sve = write_buf as *mut user_sve_header;

    /* Set up some data and write it out */
    unsafe {
        memset(write_sve as *mut c_void, 0, data_size);
        (*write_sve).size = data_size as c_uint;
        (*write_sve).vl = vl;
        (*write_sve).flags = SVE_PT_REGS_SVE;
    }

    i = 0;
    while i < __SVE_NUM_ZREGS {
        unsafe {
            fill_buf(
                (write_buf as *mut c_char).add(SVE_PT_SVE_ZREG_OFFSET(vq, i)),
                SVE_PT_SVE_ZREG_SIZE(vq),
            );
        }
        i += 1;
    }

    unsafe {
        fill_buf(
            (write_buf as *mut c_char).add(SVE_PT_SVE_FPSR_OFFSET(vq)),
            SVE_PT_SVE_FPSR_SIZE,
        );
        fill_buf(
            (write_buf as *mut c_char).add(SVE_PT_SVE_FPCR_OFFSET(vq)),
            SVE_PT_SVE_FPCR_SIZE,
        );
    }

    ret = unsafe { set_sve(child, type_, write_sve) };
    if ret != 0 {
        unsafe {
            ksft_test_result_fail(c"Failed to set %s VL %u data\n".as_ptr(), (*type_).name, vl);
            free(write_buf);
        }
        return;
    }

    /* Read the data back */
    if unsafe { get_fpsimd(child, &mut fpsimd_state) } != 0 {
        unsafe {
            ksft_test_result_fail(
                c"Failed to read %s VL %u FPSIMD data\n".as_ptr(),
                (*type_).name,
                vl,
            );
            free(write_buf);
        }
        return;
    }

    i = 0;
    while i < __SVE_NUM_ZREGS {
        let mut tmp: u128 = 0;

        /*
         * Z regs are stored endianness invariant, this won't
         * work for big endian
         */
        unsafe {
            memcpy(
                &mut tmp as *mut u128 as *mut c_void,
                (write_buf as *mut c_char).add(SVE_PT_SVE_ZREG_OFFSET(vq, i)) as *const c_void,
                size_of::<u128>(),
            );

            if tmp != fpsimd_state.vregs[i as usize] {
                printf(
                    c"# Mismatch in FPSIMD for %s VL %u Z%d\n".as_ptr(),
                    (*type_).name,
                    vl,
                    i,
                );
                errors += 1;
            }
        }
        i += 1;
    }

    unsafe {
        check_u32(
            vl,
            c"FPSR".as_ptr(),
            (write_buf as *mut c_char).add(SVE_PT_SVE_FPSR_OFFSET(vq)) as *mut u32,
            &mut fpsimd_state.fpsr,
            &mut errors,
        );
        check_u32(
            vl,
            c"FPCR".as_ptr(),
            (write_buf as *mut c_char).add(SVE_PT_SVE_FPCR_OFFSET(vq)) as *mut u32,
            &mut fpsimd_state.fpcr,
            &mut errors,
        );

        ksft_test_result(
            errors == 0,
            c"Set and get FPSIMD data for %s VL %u\n".as_ptr(),
            (*type_).name,
            vl,
        );

        free(write_buf);
    }
}

/* Validate attempting to set FPSIMD data and read it via the SVE regset */
unsafe fn ptrace_set_fpsimd_get_sve_data(child: pid_t, type_: *const vec_type, vl: c_uint) {
    let mut read_buf: *mut c_void = ptr::null_mut();
    let mut p: *mut u8;
    let read_sve: *mut user_sve_header;
    let vq: c_uint = unsafe { sve_vq_from_vl(vl) };
    let mut write_fpsimd: user_fpsimd_state = unsafe { core::mem::zeroed() };
    let mut ret: c_int;
    let mut i: c_int;
    let mut j: c_int;
    let mut read_sve_size: size_t = 0;
    let expected_size: size_t;
    let mut errors: c_int = 0;

    if __BYTE_ORDER == __BIG_ENDIAN {
        unsafe { ksft_test_result_skip(c"Big endian not supported\n".as_ptr()) };
        return;
    }

    i = 0;
    while i < 32 {
        p = &mut write_fpsimd.vregs[i as usize] as *mut u128 as *mut u8;

        j = 0;
        while (j as usize) < size_of::<u128>() {
            unsafe {
                *p.add(j as usize) = j as u8;
            }
            j += 1;
        }
        i += 1;
    }

    ret = unsafe { set_fpsimd(child, &mut write_fpsimd) };
    if ret != 0 {
        unsafe { ksft_test_result_fail(c"Failed to set FPSIMD state: %d\n)".as_ptr(), ret) };
        return;
    }

    if unsafe { get_sve(child, type_, &mut read_buf, &mut read_sve_size).is_null() } {
        unsafe {
            ksft_test_result_fail(c"Failed to read %s VL %u data\n".as_ptr(), (*type_).name, vl);
        }
        return;
    }
    read_sve = read_buf as *mut user_sve_header;

    if unsafe { (*read_sve).vl != vl } {
        unsafe {
            ksft_test_result_fail(
                c"Child VL != expected VL: %u != %u\n".as_ptr(),
                (*read_sve).vl,
                vl,
            );
            free(read_buf);
        }
        return;
    }

    /* The kernel may return either SVE or FPSIMD format */
    match unsafe { (*read_sve).flags & SVE_PT_REGS_MASK } {
        SVE_PT_REGS_FPSIMD => {
            expected_size = unsafe { SVE_PT_FPSIMD_SIZE(vq, SVE_PT_REGS_FPSIMD) };
            if read_sve_size < expected_size {
                unsafe {
                    ksft_test_result_fail(
                        c"Read %ld bytes, expected %ld\n".as_ptr(),
                        read_sve_size,
                        expected_size,
                    );
                    free(read_buf);
                }
                return;
            }

            ret = unsafe {
                memcmp(
                    &write_fpsimd as *const user_fpsimd_state as *const c_void,
                    (read_buf as *mut c_char).add(SVE_PT_FPSIMD_OFFSET) as *const c_void,
                    size_of::<user_fpsimd_state>(),
                )
            };
            if ret != 0 {
                unsafe { ksft_print_msg(c"Read FPSIMD data mismatch\n".as_ptr()) };
                errors += 1;
            }
        }
        SVE_PT_REGS_SVE => {
            expected_size = unsafe { SVE_PT_SVE_SIZE(vq, SVE_PT_REGS_SVE) };
            if read_sve_size < expected_size {
                unsafe {
                    ksft_test_result_fail(
                        c"Read %ld bytes, expected %ld\n".as_ptr(),
                        read_sve_size,
                        expected_size,
                    );
                    free(read_buf);
                }
                return;
            }

            i = 0;
            while i < __SVE_NUM_ZREGS {
                let mut tmp: u128 = 0;

                /*
                 * Z regs are stored endianness invariant, this won't
                 * work for big endian
                 */
                unsafe {
                    memcpy(
                        &mut tmp as *mut u128 as *mut c_void,
                        (read_buf as *mut c_char).add(SVE_PT_SVE_ZREG_OFFSET(vq, i))
                            as *const c_void,
                        size_of::<u128>(),
                    );

                    if tmp != write_fpsimd.vregs[i as usize] {
                        ksft_print_msg(
                            c"Mismatch in FPSIMD for %s VL %u Z%d/V%d\n".as_ptr(),
                            (*type_).name,
                            vl,
                            i,
                            i,
                        );
                        errors += 1;
                    }
                }
                i += 1;
            }

            unsafe {
                check_u32(
                    vl,
                    c"FPSR".as_ptr(),
                    &mut write_fpsimd.fpsr,
                    (read_buf as *mut c_char).add(SVE_PT_SVE_FPSR_OFFSET(vq)) as *mut u32,
                    &mut errors,
                );
                check_u32(
                    vl,
                    c"FPCR".as_ptr(),
                    &mut write_fpsimd.fpcr,
                    (read_buf as *mut c_char).add(SVE_PT_SVE_FPCR_OFFSET(vq)) as *mut u32,
                    &mut errors,
                );
            }
        }
        _ => {
            unsafe {
                ksft_print_msg(
                    c"Unexpected regs type %d\n".as_ptr(),
                    (*read_sve).flags & SVE_PT_REGS_MASK,
                );
            }
            errors += 1;
        }
    }

    unsafe {
        ksft_test_result(
            errors == 0,
            c"Set FPSIMD, read via SVE for %s VL %u\n".as_ptr(),
            (*type_).name,
            vl,
        );

        free(read_buf);
    }
}

unsafe fn do_parent(child: pid_t) -> c_int {
    let mut ret: c_int = EXIT_FAILURE;
    let mut pid: pid_t;
    let mut status: c_int = 0;
    let mut i: usize;
    let mut si: siginfo_t = unsafe { core::mem::zeroed() };
    let mut vq: c_uint;
    let mut vl: c_uint;
    let mut vl_supported: bool;

    unsafe { ksft_print_msg(c"Parent is %d, child is %d\n".as_ptr(), getpid(), child) };

    /* Attach to the child */
    loop {
        let mut sig: c_int;

        pid = unsafe { wait(&mut status) };
        if pid == -1 {
            unsafe { perror(c"wait".as_ptr()) };
            unsafe { kill(child, SIGKILL) };
            return ret;
        }

        /*
         * This should never happen but it's hard to flag in
         * the framework.
         */
        if pid != child {
            continue;
        }

        if unsafe { WIFEXITED(status) || WIFSIGNALED(status) } {
            unsafe { ksft_exit_fail_msg(c"Child died unexpectedly\n".as_ptr()) };
        }

        if unsafe { !WIFSTOPPED(status) } {
            unsafe { kill(child, SIGKILL) };
            return ret;
        }

        sig = unsafe { WSTOPSIG(status) };

        if unsafe { ptrace(PTRACE_GETSIGINFO, pid, ptr::null_mut::<c_void>(), &mut si) } != 0 {
            if unsafe { errno_value() } == ESRCH {
                return ret;
            }

            if unsafe { errno_value() } == EINVAL {
                sig = 0; /* bust group-stop */
            } else {
                unsafe {
                    ksft_test_result_fail(
                        c"PTRACE_GETSIGINFO: %s\n".as_ptr(),
                        strerror(errno_value()),
                    );
                    kill(child, SIGKILL);
                }
                return ret;
            }
        } else if sig == SIGSTOP && si.si_code == SI_TKILL && si.si_pid == pid {
            break;
        }

        if unsafe { ptrace(PTRACE_CONT, pid, ptr::null_mut::<c_void>(), sig) } != 0 {
            if unsafe { errno_value() } == ESRCH {
                return ret;
            }

            unsafe {
                ksft_test_result_fail(c"PTRACE_CONT: %s\n".as_ptr(), strerror(errno_value()));
                kill(child, SIGKILL);
            }
            return ret;
        }
    }

    i = 0;
    while i < vec_types.len() {
        /*
         * If the vector type isn't supported reads and writes
         * should fail.
         */
        if unsafe { (getauxval(vec_types[i].hwcap_type) & vec_types[i].hwcap) == 0 } {
            unsafe {
                read_fails(child, &vec_types[i]);
                write_fails(child, &vec_types[i]);
            }
        } else {
            unsafe {
                ksft_test_result_skip(
                    c"%s unsupported read fails\n".as_ptr(),
                    vec_types[i].name,
                );
                ksft_test_result_skip(
                    c"%s unsupported write fails\n".as_ptr(),
                    vec_types[i].name,
                );
            }
        }

        /* FPSIMD via SVE regset */
        if unsafe { (getauxval(vec_types[i].hwcap_type) & vec_types[i].hwcap) != 0 } {
            unsafe { ptrace_sve_fpsimd(child, &vec_types[i]) };
        } else {
            unsafe {
                ksft_test_result_skip(c"%s FPSIMD set via SVE\n".as_ptr(), vec_types[i].name);
                ksft_test_result_skip(c"%s FPSIMD read\n".as_ptr(), vec_types[i].name);
            }
        }

        /* prctl() flags */
        if unsafe { (getauxval(vec_types[i].hwcap_type) & vec_types[i].hwcap) != 0 } {
            unsafe { ptrace_set_get_inherit(child, &vec_types[i]) };
        } else {
            unsafe {
                ksft_test_result_skip(
                    c"%s SVE_PT_VL_INHERIT set\n".as_ptr(),
                    vec_types[i].name,
                );
                ksft_test_result_skip(
                    c"%s SVE_PT_VL_INHERIT cleared\n".as_ptr(),
                    vec_types[i].name,
                );
            }
        }

        /* Setting out of bounds VLs should fail */
        if unsafe { (getauxval(vec_types[i].hwcap_type) & vec_types[i].hwcap) != 0 } {
            unsafe { ptrace_set_vl_ranges(child, &vec_types[i]) };
        } else {
            unsafe {
                ksft_test_result_skip(c"%s Set invalid VL 0\n".as_ptr(), vec_types[i].name);
                ksft_test_result_skip(
                    c"%s Set invalid VL %d\n".as_ptr(),
                    vec_types[i].name,
                    SVE_VL_MAX + SVE_VQ_BYTES,
                );
            }
        }

        /* Step through every possible VQ */
        vq = SVE_VQ_MIN;
        while vq <= TEST_VQ_MAX {
            vl = unsafe { sve_vl_from_vq(vq) };

            /* First, try to set this vector length */
            if unsafe { (getauxval(vec_types[i].hwcap_type) & vec_types[i].hwcap) != 0 } {
                unsafe { ptrace_set_get_vl(child, &vec_types[i], vl, &mut vl_supported) };
            } else {
                unsafe {
                    ksft_test_result_skip(
                        c"%s get/set VL %d\n".as_ptr(),
                        vec_types[i].name,
                        vl,
                    );
                }
                vl_supported = false;
            }

            /* If the VL is supported validate data set/get */
            if vl_supported {
                unsafe {
                    ptrace_set_sve_get_sve_data(child, &vec_types[i], vl);
                    ptrace_set_sve_get_fpsimd_data(child, &vec_types[i], vl);
                    ptrace_set_fpsimd_get_sve_data(child, &vec_types[i], vl);
                }
            } else {
                unsafe {
                    ksft_test_result_skip(
                        c"%s set SVE get SVE for VL %d\n".as_ptr(),
                        vec_types[i].name,
                        vl,
                    );
                    ksft_test_result_skip(
                        c"%s set SVE get FPSIMD for VL %d\n".as_ptr(),
                        vec_types[i].name,
                        vl,
                    );
                    ksft_test_result_skip(
                        c"%s set FPSIMD get SVE for VL %d\n".as_ptr(),
                        vec_types[i].name,
                        vl,
                    );
                }
            }

            vq += 1;
        }

        i += 1;
    }

    /* We support SVE writes of FPSMID format on SME only systems */
    if unsafe {
        (getauxval(AT_HWCAP) & HWCAP_SVE) == 0 && (getauxval(AT_HWCAP2) & HWCAP2_SME) != 0
    } {
        unsafe { ptrace_sve_fpsimd_no_sve(child) };
    } else {
        unsafe {
            ksft_test_result_skip(c"FPSIMD write via SVE\n".as_ptr());
            ksft_test_result_skip(c"Verify FPSIMD write via SVE\n".as_ptr());
        }
    }

    ret = EXIT_SUCCESS;

    unsafe { kill(child, SIGKILL) };

    ret
}

fn main() -> std::process::ExitCode {
    let mut ret: c_int = EXIT_SUCCESS;
    let child: pid_t;

    unsafe {
        srandom(getpid() as c_uint);

        ksft_print_header();
        ksft_set_plan(EXPECTED_TESTS as c_uint);

        child = fork();
        if child == 0 {
            return std::process::ExitCode::from(do_child() as u8);
        }

        if do_parent(child) != 0 {
            ret = EXIT_FAILURE;
        }

        ksft_print_cnts();
    }

    std::process::ExitCode::from(ret as u8)
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
