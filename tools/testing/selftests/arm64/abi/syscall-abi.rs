// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2021 ARM Limited.
 */

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

/*
 * C includes translated as external dependencies:
 * errno.h, stdbool.h, stddef.h, stdio.h, stdlib.h, string.h, unistd.h,
 * sys/auxv.h, sys/prctl.h, asm/hwcap.h, asm/sigcontext.h, asm/unistd.h,
 * kselftest.h, syscall-abi.h.
 */

/*
 * The kernel defines a much larger SVE_VQ_MAX than is expressable in
 * the architecture, this creates a *lot* of overhead filling the
 * buffers (especially ZA) on emulated platforms so use the actual
 * architectural maximum instead.
 */
const ARCH_SVE_VQ_MAX: usize = 16;

/* Constants supplied by the included C headers. */
const AT_HWCAP: c_ulong = 16;
const AT_HWCAP2: c_ulong = 26;
const HWCAP_SVE: c_ulong = 1 << 22;
const HWCAP2_SME: c_ulong = 1 << 23;
const HWCAP2_SME2: c_ulong = 1 << 37;
const HWCAP2_SME_FA64: c_ulong = 1 << 63;
const PR_SVE_SET_VL: c_int = 50;
const PR_SME_SET_VL: c_int = 63;
const PR_SVE_VL_LEN_MASK: c_int = 0xffff;
const PR_SME_VL_LEN_MASK: c_int = 0xffff;
const __NR_GETPID: c_int = 172;
const __NR_SCHED_YIELD: c_int = 124;
const SVCR_SM_MASK: u64 = 1;
const SVCR_ZA_MASK: u64 = 2;
const SVE_NUM_ZREGS: usize = 32;
const SVE_NUM_PREGS: usize = 16;
const ZT_SIG_REG_BYTES: usize = 512;

const fn __SVE_ZREG_SIZE(vq: usize) -> usize {
    vq * 16
}

const fn __SVE_PREG_SIZE(vq: usize) -> usize {
    vq * 2
}

const fn ZA_SIG_REGS_SIZE(vq: usize) -> usize {
    let vl = vq * 16;
    vl * vl
}

fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> usize {
    N
}

fn sve_vq_from_vl(vl: c_int) -> usize {
    (vl as usize) / 16
}

static mut default_sme_vl: c_int = 0;

static mut sve_vl_count: c_int = 0;
static mut sve_vls: [u32; ARCH_SVE_VQ_MAX] = [0; ARCH_SVE_VQ_MAX];
static mut sme_vl_count: c_int = 0;
static mut sme_vls: [u32; ARCH_SVE_VQ_MAX] = [0; ARCH_SVE_VQ_MAX];

unsafe extern "C" {
    fn do_syscall(sve_vl: c_int, sme_vl: c_int);

    fn random() -> i64;
    fn srandom(seed: u32);
    fn getpid() -> c_int;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn prctl(option: c_int, ...) -> c_int;
    fn memset(s: *mut c_void, c: c_int, n: usize) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: usize) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;

    static mut errno: c_int;

    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_test_result(pass: bool, fmt: *const c_char, ...);
    fn ksft_print_header();
    fn ksft_set_plan(plan: c_int);
    fn ksft_print_cnts();
}

unsafe fn fill_random(buf: *mut c_void, size: usize) {
    let mut i: c_int;
    let lbuf = buf as *mut u32;

    /* random() returns a 32 bit number regardless of the size of long */
    i = 0;
    while (i as usize) < size / size_of::<u32>() {
        *lbuf.add(i as usize) = random() as u32;
        i += 1;
    }
}

/*
 * We also repeat the test for several syscalls to try to expose different
 * behaviour.
 */
#[repr(C)]
struct syscall_cfg {
    syscall_nr: c_int,
    name: *const c_char,
}

static mut syscalls: [syscall_cfg; 2] = [
    syscall_cfg {
        syscall_nr: __NR_GETPID,
        name: c"getpid()".as_ptr(),
    },
    syscall_cfg {
        syscall_nr: __NR_SCHED_YIELD,
        name: c"sched_yield()".as_ptr(),
    },
];

const NUM_GPR: usize = 31;
#[unsafe(no_mangle)]
static mut gpr_in: [u64; NUM_GPR] = [0; NUM_GPR];
#[unsafe(no_mangle)]
static mut gpr_out: [u64; NUM_GPR] = [0; NUM_GPR];

unsafe fn setup_gpr(cfg: *mut syscall_cfg, _sve_vl: c_int, _sme_vl: c_int, _svcr: u64) {
    fill_random(ptr::addr_of_mut!(gpr_in) as *mut c_void, size_of::<[u64; NUM_GPR]>());
    gpr_in[8] = (*cfg).syscall_nr as u64;
    memset(ptr::addr_of_mut!(gpr_out) as *mut c_void, 0, size_of::<[u64; NUM_GPR]>());
}

unsafe fn check_gpr(cfg: *mut syscall_cfg, sve_vl: c_int, _sme_vl: c_int, _svcr: u64) -> c_int {
    let mut errors: c_int = 0;
    let mut i: c_int;

    /*
     * GPR x0-x7 may be clobbered, and all others should be preserved.
     */
    i = 9;
    while (i as usize) < ARRAY_SIZE(&gpr_in) {
        if gpr_in[i as usize] != gpr_out[i as usize] {
            ksft_print_msg(
                c"%s SVE VL %d mismatch in GPR %d: %lx != %lx\n".as_ptr(),
                (*cfg).name,
                sve_vl,
                i,
                gpr_in[i as usize],
                gpr_out[i as usize],
            );
            errors += 1;
        }
        i += 1;
    }

    errors
}

const NUM_FPR: usize = 32;
#[unsafe(no_mangle)]
static mut fpr_in: [u64; NUM_FPR * 2] = [0; NUM_FPR * 2];
#[unsafe(no_mangle)]
static mut fpr_out: [u64; NUM_FPR * 2] = [0; NUM_FPR * 2];
#[unsafe(no_mangle)]
static mut fpr_zero: [u64; NUM_FPR * 2] = [0; NUM_FPR * 2];

unsafe fn setup_fpr(_cfg: *mut syscall_cfg, _sve_vl: c_int, _sme_vl: c_int, _svcr: u64) {
    fill_random(ptr::addr_of_mut!(fpr_in) as *mut c_void, size_of::<[u64; NUM_FPR * 2]>());
    memset(ptr::addr_of_mut!(fpr_out) as *mut c_void, 0, size_of::<[u64; NUM_FPR * 2]>());
}

unsafe fn check_fpr(cfg: *mut syscall_cfg, sve_vl: c_int, _sme_vl: c_int, svcr: u64) -> c_int {
    let mut errors: c_int = 0;
    let mut i: c_int;

    if sve_vl == 0 && (svcr & SVCR_SM_MASK) == 0 {
        i = 0;
        while (i as usize) < ARRAY_SIZE(&fpr_in) {
            if fpr_in[i as usize] != fpr_out[i as usize] {
                ksft_print_msg(
                    c"%s Q%d/%d mismatch %lx != %lx\n".as_ptr(),
                    (*cfg).name,
                    i / 2,
                    i % 2,
                    fpr_in[i as usize],
                    fpr_out[i as usize],
                );
                errors += 1;
            }
            i += 1;
        }
    }

    /*
     * In streaming mode the whole register set should be cleared
     * by the transition out of streaming mode.
     */
    if (svcr & SVCR_SM_MASK) != 0 {
        if memcmp(
            ptr::addr_of!(fpr_zero) as *const c_void,
            ptr::addr_of!(fpr_out) as *const c_void,
            size_of::<[u64; NUM_FPR * 2]>(),
        ) != 0
        {
            ksft_print_msg(c"%s FPSIMD registers non-zero exiting SM\n".as_ptr(), (*cfg).name);
            errors += 1;
        }
    }

    errors
}

const SVE_Z_SHARED_BYTES: usize = 128 / 8;

static mut z_zero: [u8; __SVE_ZREG_SIZE(ARCH_SVE_VQ_MAX)] =
    [0; __SVE_ZREG_SIZE(ARCH_SVE_VQ_MAX)];
#[unsafe(no_mangle)]
static mut z_in: [u8; SVE_NUM_ZREGS * __SVE_ZREG_SIZE(ARCH_SVE_VQ_MAX)] =
    [0; SVE_NUM_ZREGS * __SVE_ZREG_SIZE(ARCH_SVE_VQ_MAX)];
#[unsafe(no_mangle)]
static mut z_out: [u8; SVE_NUM_ZREGS * __SVE_ZREG_SIZE(ARCH_SVE_VQ_MAX)] =
    [0; SVE_NUM_ZREGS * __SVE_ZREG_SIZE(ARCH_SVE_VQ_MAX)];

unsafe fn setup_z(_cfg: *mut syscall_cfg, _sve_vl: c_int, _sme_vl: c_int, _svcr: u64) {
    fill_random(ptr::addr_of_mut!(z_in) as *mut c_void, size_of::<[u8; SVE_NUM_ZREGS * __SVE_ZREG_SIZE(ARCH_SVE_VQ_MAX)]>());
    fill_random(ptr::addr_of_mut!(z_out) as *mut c_void, size_of::<[u8; SVE_NUM_ZREGS * __SVE_ZREG_SIZE(ARCH_SVE_VQ_MAX)]>());
}

unsafe fn check_z(cfg: *mut syscall_cfg, sve_vl: c_int, _sme_vl: c_int, svcr: u64) -> c_int {
    let reg_size: usize = sve_vl as usize;
    let mut errors: c_int = 0;
    let mut i: c_int;

    if sve_vl == 0 {
        return 0;
    }

    i = 0;
    while i < SVE_NUM_ZREGS as c_int {
        let in_ = (ptr::addr_of_mut!(z_in) as *mut u8).add(reg_size * i as usize);
        let out = (ptr::addr_of_mut!(z_out) as *mut u8).add(reg_size * i as usize);

        if (svcr & SVCR_SM_MASK) != 0 {
            /*
             * In streaming mode the whole register should
             * be cleared by the transition out of
             * streaming mode.
             */
            if memcmp(ptr::addr_of!(z_zero) as *const c_void, out as *const c_void, reg_size) != 0 {
                ksft_print_msg(c"%s SVE VL %d Z%d non-zero\n".as_ptr(), (*cfg).name, sve_vl, i);
                errors += 1;
            }
        } else {
            /*
             * For standard SVE the low 128 bits should be
             * preserved and any additional bits cleared.
             */
            if memcmp(in_ as *const c_void, out as *const c_void, SVE_Z_SHARED_BYTES) != 0 {
                ksft_print_msg(
                    c"%s SVE VL %d Z%d low 128 bits changed\n".as_ptr(),
                    (*cfg).name,
                    sve_vl,
                    i,
                );
                errors += 1;
            }

            if reg_size > SVE_Z_SHARED_BYTES
                && memcmp(
                    ptr::addr_of!(z_zero) as *const c_void,
                    out.add(SVE_Z_SHARED_BYTES) as *const c_void,
                    reg_size - SVE_Z_SHARED_BYTES,
                ) != 0
            {
                ksft_print_msg(
                    c"%s SVE VL %d Z%d high bits non-zero\n".as_ptr(),
                    (*cfg).name,
                    sve_vl,
                    i,
                );
                errors += 1;
            }
        }
        i += 1;
    }

    errors
}

#[unsafe(no_mangle)]
static mut p_in: [u8; SVE_NUM_PREGS * __SVE_PREG_SIZE(ARCH_SVE_VQ_MAX)] =
    [0; SVE_NUM_PREGS * __SVE_PREG_SIZE(ARCH_SVE_VQ_MAX)];
#[unsafe(no_mangle)]
static mut p_out: [u8; SVE_NUM_PREGS * __SVE_PREG_SIZE(ARCH_SVE_VQ_MAX)] =
    [0; SVE_NUM_PREGS * __SVE_PREG_SIZE(ARCH_SVE_VQ_MAX)];

unsafe fn setup_p(_cfg: *mut syscall_cfg, _sve_vl: c_int, _sme_vl: c_int, _svcr: u64) {
    fill_random(ptr::addr_of_mut!(p_in) as *mut c_void, size_of::<[u8; SVE_NUM_PREGS * __SVE_PREG_SIZE(ARCH_SVE_VQ_MAX)]>());
    fill_random(ptr::addr_of_mut!(p_out) as *mut c_void, size_of::<[u8; SVE_NUM_PREGS * __SVE_PREG_SIZE(ARCH_SVE_VQ_MAX)]>());
}

unsafe fn check_p(cfg: *mut syscall_cfg, sve_vl: c_int, _sme_vl: c_int, _svcr: u64) -> c_int {
    let reg_size: usize = sve_vq_from_vl(sve_vl) * 2; /* 1 bit per VL byte */
    let mut errors: c_int = 0;
    let mut i: c_int;

    if sve_vl == 0 {
        return 0;
    }

    /* After a syscall the P registers should be zeroed */
    i = 0;
    while (i as usize) < SVE_NUM_PREGS * reg_size {
        if p_out[i as usize] != 0 {
            errors += 1;
        }
        i += 1;
    }
    if errors != 0 {
        ksft_print_msg(c"%s SVE VL %d predicate registers non-zero\n".as_ptr(), (*cfg).name, sve_vl);
    }

    errors
}

#[unsafe(no_mangle)]
static mut ffr_in: [u8; __SVE_PREG_SIZE(ARCH_SVE_VQ_MAX)] =
    [0; __SVE_PREG_SIZE(ARCH_SVE_VQ_MAX)];
#[unsafe(no_mangle)]
static mut ffr_out: [u8; __SVE_PREG_SIZE(ARCH_SVE_VQ_MAX)] =
    [0; __SVE_PREG_SIZE(ARCH_SVE_VQ_MAX)];

unsafe fn setup_ffr(_cfg: *mut syscall_cfg, _sve_vl: c_int, _sme_vl: c_int, svcr: u64) {
    /*
     * If we are in streaming mode and do not have FA64 then FFR
     * is unavailable.
     */
    if (svcr & SVCR_SM_MASK) != 0 && (getauxval(AT_HWCAP2) & HWCAP2_SME_FA64) == 0 {
        memset(ptr::addr_of_mut!(ffr_in) as *mut c_void, 0, size_of::<[u8; __SVE_PREG_SIZE(ARCH_SVE_VQ_MAX)]>());
        return;
    }

    /*
     * It is only valid to set a contiguous set of bits starting
     * at 0.  For now since we're expecting this to be cleared by
     * a syscall just set all bits.
     */
    memset(ptr::addr_of_mut!(ffr_in) as *mut c_void, 0xff, size_of::<[u8; __SVE_PREG_SIZE(ARCH_SVE_VQ_MAX)]>());
    fill_random(ptr::addr_of_mut!(ffr_out) as *mut c_void, size_of::<[u8; __SVE_PREG_SIZE(ARCH_SVE_VQ_MAX)]>());
}

unsafe fn check_ffr(cfg: *mut syscall_cfg, sve_vl: c_int, _sme_vl: c_int, svcr: u64) -> c_int {
    let reg_size: usize = sve_vq_from_vl(sve_vl) * 2; /* 1 bit per VL byte */
    let mut errors: c_int = 0;
    let mut i: c_int;

    if sve_vl == 0 {
        return 0;
    }

    if (svcr & SVCR_SM_MASK) != 0 && (getauxval(AT_HWCAP2) & HWCAP2_SME_FA64) == 0 {
        return 0;
    }

    /* After a syscall FFR should be zeroed */
    i = 0;
    while (i as usize) < reg_size {
        if ffr_out[i as usize] != 0 {
            errors += 1;
        }
        i += 1;
    }
    if errors != 0 {
        ksft_print_msg(c"%s SVE VL %d FFR non-zero\n".as_ptr(), (*cfg).name, sve_vl);
    }

    errors
}

#[unsafe(no_mangle)]
static mut svcr_in: u64 = 0;
#[unsafe(no_mangle)]
static mut svcr_out: u64 = 0;

unsafe fn setup_svcr(_cfg: *mut syscall_cfg, _sve_vl: c_int, _sme_vl: c_int, svcr: u64) {
    svcr_in = svcr;
}

unsafe fn check_svcr(cfg: *mut syscall_cfg, _sve_vl: c_int, _sme_vl: c_int, _svcr: u64) -> c_int {
    let mut errors: c_int = 0;

    if (svcr_out & SVCR_SM_MASK) != 0 {
        ksft_print_msg(c"%s Still in SM, SVCR %lx\n".as_ptr(), (*cfg).name, svcr_out);
        errors += 1;
    }

    if (svcr_in & SVCR_ZA_MASK) != (svcr_out & SVCR_ZA_MASK) {
        ksft_print_msg(
            c"%s PSTATE.ZA changed, SVCR %lx != %lx\n".as_ptr(),
            (*cfg).name,
            svcr_in,
            svcr_out,
        );
        errors += 1;
    }

    errors
}

#[unsafe(no_mangle)]
static mut za_in: [u8; ZA_SIG_REGS_SIZE(ARCH_SVE_VQ_MAX)] =
    [0; ZA_SIG_REGS_SIZE(ARCH_SVE_VQ_MAX)];
#[unsafe(no_mangle)]
static mut za_out: [u8; ZA_SIG_REGS_SIZE(ARCH_SVE_VQ_MAX)] =
    [0; ZA_SIG_REGS_SIZE(ARCH_SVE_VQ_MAX)];

unsafe fn setup_za(_cfg: *mut syscall_cfg, _sve_vl: c_int, _sme_vl: c_int, _svcr: u64) {
    fill_random(ptr::addr_of_mut!(za_in) as *mut c_void, size_of::<[u8; ZA_SIG_REGS_SIZE(ARCH_SVE_VQ_MAX)]>());
    memset(ptr::addr_of_mut!(za_out) as *mut c_void, 0, size_of::<[u8; ZA_SIG_REGS_SIZE(ARCH_SVE_VQ_MAX)]>());
}

unsafe fn check_za(_cfg: *mut syscall_cfg, _sve_vl: c_int, sme_vl: c_int, svcr: u64) -> c_int {
    let reg_size: usize = (sme_vl * sme_vl) as usize;
    let mut errors: c_int = 0;

    if (svcr & SVCR_ZA_MASK) == 0 {
        return 0;
    }

    if memcmp(ptr::addr_of!(za_in) as *const c_void, ptr::addr_of!(za_out) as *const c_void, reg_size) != 0 {
        ksft_print_msg(c"SME VL %d ZA does not match\n".as_ptr(), sme_vl);
        errors += 1;
    }

    errors
}

#[repr(C, align(16))]
struct AlignedZt([u8; ZT_SIG_REG_BYTES]);

#[unsafe(no_mangle)]
static mut zt_in: AlignedZt = AlignedZt([0; ZT_SIG_REG_BYTES]);
#[unsafe(no_mangle)]
static mut zt_out: AlignedZt = AlignedZt([0; ZT_SIG_REG_BYTES]);

unsafe fn setup_zt(_cfg: *mut syscall_cfg, _sve_vl: c_int, _sme_vl: c_int, _svcr: u64) {
    fill_random(ptr::addr_of_mut!(zt_in) as *mut c_void, size_of::<AlignedZt>());
    memset(ptr::addr_of_mut!(zt_out) as *mut c_void, 0, size_of::<AlignedZt>());
}

unsafe fn check_zt(_cfg: *mut syscall_cfg, _sve_vl: c_int, sme_vl: c_int, svcr: u64) -> c_int {
    let mut errors: c_int = 0;

    if (getauxval(AT_HWCAP2) & HWCAP2_SME2) == 0 {
        return 0;
    }

    if (svcr & SVCR_ZA_MASK) == 0 {
        return 0;
    }

    if memcmp(ptr::addr_of!(zt_in) as *const c_void, ptr::addr_of!(zt_out) as *const c_void, size_of::<AlignedZt>()) != 0 {
        ksft_print_msg(c"SME VL %d ZT does not match\n".as_ptr(), sme_vl);
        errors += 1;
    }

    errors
}

type setup_fn = unsafe fn(cfg: *mut syscall_cfg, sve_vl: c_int, sme_vl: c_int, svcr: u64);
type check_fn = unsafe fn(cfg: *mut syscall_cfg, sve_vl: c_int, sme_vl: c_int, svcr: u64) -> c_int;

/*
 * Each set of registers has a setup function which is called before
 * the syscall to fill values in a global variable for loading by the
 * test code and a check function which validates that the results are
 * as expected.  Vector lengths are passed everywhere, a vector length
 * of 0 should be treated as do not test.
 */
struct Regset {
    setup: setup_fn,
    check: check_fn,
}

static mut regset: [Regset; 8] = [
    Regset { setup: setup_gpr, check: check_gpr },
    Regset { setup: setup_fpr, check: check_fpr },
    Regset { setup: setup_z, check: check_z },
    Regset { setup: setup_p, check: check_p },
    Regset { setup: setup_ffr, check: check_ffr },
    Regset { setup: setup_svcr, check: check_svcr },
    Regset { setup: setup_za, check: check_za },
    Regset { setup: setup_zt, check: check_zt },
];

unsafe fn do_test(cfg: *mut syscall_cfg, sve_vl: c_int, sme_vl: c_int, svcr: u64) -> bool {
    let mut errors: c_int = 0;
    let mut i: c_int;

    i = 0;
    while (i as usize) < ARRAY_SIZE(&regset) {
        (regset[i as usize].setup)(cfg, sve_vl, sme_vl, svcr);
        i += 1;
    }

    do_syscall(sve_vl, sme_vl);

    i = 0;
    while (i as usize) < ARRAY_SIZE(&regset) {
        errors += (regset[i as usize].check)(cfg, sve_vl, sme_vl, svcr);
        i += 1;
    }

    errors == 0
}

unsafe fn test_one_syscall(cfg: *mut syscall_cfg) {
    let mut sve: c_int;
    let mut sme: c_int;
    let mut ret: c_int;

    /* FPSIMD only case */
    ksft_test_result(do_test(cfg, 0, default_sme_vl, 0), c"%s FPSIMD\n".as_ptr(), (*cfg).name);

    sve = 0;
    while sve < sve_vl_count {
        ret = prctl(PR_SVE_SET_VL, sve_vls[sve as usize]);
        if ret == -1 {
            ksft_exit_fail_msg(c"PR_SVE_SET_VL failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
        }

        ksft_test_result(
            do_test(cfg, sve_vls[sve as usize] as c_int, default_sme_vl, 0),
            c"%s SVE VL %d\n".as_ptr(),
            (*cfg).name,
            sve_vls[sve as usize],
        );

        sme = 0;
        while sme < sme_vl_count {
            ret = prctl(PR_SME_SET_VL, sme_vls[sme as usize]);
            if ret == -1 {
                ksft_exit_fail_msg(c"PR_SME_SET_VL failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
            }

            ksft_test_result(
                do_test(
                    cfg,
                    sve_vls[sve as usize] as c_int,
                    sme_vls[sme as usize] as c_int,
                    SVCR_ZA_MASK | SVCR_SM_MASK,
                ),
                c"%s SVE VL %d/SME VL %d SM+ZA\n".as_ptr(),
                (*cfg).name,
                sve_vls[sve as usize],
                sme_vls[sme as usize],
            );
            ksft_test_result(
                do_test(
                    cfg,
                    sve_vls[sve as usize] as c_int,
                    sme_vls[sme as usize] as c_int,
                    SVCR_SM_MASK,
                ),
                c"%s SVE VL %d/SME VL %d SM\n".as_ptr(),
                (*cfg).name,
                sve_vls[sve as usize],
                sme_vls[sme as usize],
            );
            ksft_test_result(
                do_test(
                    cfg,
                    sve_vls[sve as usize] as c_int,
                    sme_vls[sme as usize] as c_int,
                    SVCR_ZA_MASK,
                ),
                c"%s SVE VL %d/SME VL %d ZA\n".as_ptr(),
                (*cfg).name,
                sve_vls[sve as usize],
                sme_vls[sme as usize],
            );
            sme += 1;
        }
        sve += 1;
    }

    sme = 0;
    while sme < sme_vl_count {
        ret = prctl(PR_SME_SET_VL, sme_vls[sme as usize]);
        if ret == -1 {
            ksft_exit_fail_msg(c"PR_SME_SET_VL failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
        }

        ksft_test_result(
            do_test(cfg, 0, sme_vls[sme as usize] as c_int, SVCR_ZA_MASK | SVCR_SM_MASK),
            c"%s SME VL %d SM+ZA\n".as_ptr(),
            (*cfg).name,
            sme_vls[sme as usize],
        );
        ksft_test_result(
            do_test(cfg, 0, sme_vls[sme as usize] as c_int, SVCR_SM_MASK),
            c"%s SME VL %d SM\n".as_ptr(),
            (*cfg).name,
            sme_vls[sme as usize],
        );
        ksft_test_result(
            do_test(cfg, 0, sme_vls[sme as usize] as c_int, SVCR_ZA_MASK),
            c"%s SME VL %d ZA\n".as_ptr(),
            (*cfg).name,
            sme_vls[sme as usize],
        );
        sme += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sve_count_vls() {
    let mut vq: u32;
    let mut vl: c_int;

    if (getauxval(AT_HWCAP) & HWCAP_SVE) == 0 {
        return;
    }

    /*
     * Enumerate up to ARCH_SVE_VQ_MAX vector lengths
     */
    vq = ARCH_SVE_VQ_MAX as u32;
    while vq > 0 {
        vl = prctl(PR_SVE_SET_VL, vq * 16);
        if vl == -1 {
            ksft_exit_fail_msg(c"PR_SVE_SET_VL failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
        }

        vl &= PR_SVE_VL_LEN_MASK;

        if vq as usize != sve_vq_from_vl(vl) {
            vq = sve_vq_from_vl(vl) as u32;
        }

        sve_vls[sve_vl_count as usize] = vl as u32;
        sve_vl_count += 1;
        vq /= 2;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn sme_count_vls() {
    let mut vq: u32;
    let mut vl: c_int;

    if (getauxval(AT_HWCAP2) & HWCAP2_SME) == 0 {
        return;
    }

    /*
     * Enumerate up to ARCH_SVE_VQ_MAX vector lengths
     */
    vq = ARCH_SVE_VQ_MAX as u32;
    while vq > 0 {
        vl = prctl(PR_SME_SET_VL, vq * 16);
        if vl == -1 {
            ksft_exit_fail_msg(c"PR_SME_SET_VL failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
        }

        vl &= PR_SME_VL_LEN_MASK;

        /* Found lowest VL */
        if sve_vq_from_vl(vl) > vq as usize {
            break;
        }

        if vq as usize != sve_vq_from_vl(vl) {
            vq = sve_vq_from_vl(vl) as u32;
        }

        sme_vls[sme_vl_count as usize] = vl as u32;
        sme_vl_count += 1;
        vq /= 2;
    }

    /* Ensure we configure a SME VL, used to flag if SVCR is set */
    default_sme_vl = sme_vls[0] as c_int;
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    let mut i: c_int;
    let mut tests: c_int = 1; /* FPSIMD */
    let sme_ver: c_int;

    srandom(getpid() as u32);

    ksft_print_header();

    sve_count_vls();
    sme_count_vls();

    tests += sve_vl_count;
    tests += sme_vl_count * 3;
    tests += (sve_vl_count * sme_vl_count) * 3;
    ksft_set_plan((ARRAY_SIZE(&syscalls) as c_int) * tests);

    if (getauxval(AT_HWCAP2) & HWCAP2_SME2) != 0 {
        sme_ver = 2;
    } else {
        sme_ver = 1;
    }

    if (getauxval(AT_HWCAP2) & HWCAP2_SME_FA64) != 0 {
        ksft_print_msg(c"SME%d with FA64\n".as_ptr(), sme_ver);
    } else if (getauxval(AT_HWCAP2) & HWCAP2_SME) != 0 {
        ksft_print_msg(c"SME%d without FA64\n".as_ptr(), sme_ver);
    }

    i = 0;
    while (i as usize) < ARRAY_SIZE(&syscalls) {
        test_one_syscall(ptr::addr_of_mut!(syscalls[i as usize]));
        i += 1;
    }

    ksft_print_cnts();

    0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
