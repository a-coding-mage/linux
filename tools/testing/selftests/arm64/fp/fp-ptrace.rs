// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2023 ARM Limited.
 * Original author: Mark Brown <broonie@kernel.org>
 */

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]
#![allow(static_mut_refs)]

use core::ffi::{c_char, c_int, c_long, c_uint, c_ulong, c_void};
use core::mem::{size_of, zeroed};
use core::ptr;

type pid_t = c_int;
type size_t = usize;
type uint8_t = u8;
type uint32_t = u32;
type uint64_t = u64;
type __uint128_t = u128;

const fn GENMASK(h: u32, l: u32) -> u64 {
    (!0u64 >> (63 - (h - l))) << l
}

const FPMR_LSCALE2_MASK: u64 = GENMASK(37, 32);
const FPMR_NSCALE_MASK: u64 = GENMASK(31, 24);
const FPMR_LSCALE_MASK: u64 = GENMASK(22, 16);
const FPMR_OSC_MASK: u64 = GENMASK(15, 15);
const FPMR_OSM_MASK: u64 = GENMASK(14, 14);

/* <linux/elf.h> and <sys/auxv.h> don't like each other, so: */
const NT_ARM_SVE: c_int = 0x405;
const NT_ARM_SSVE: c_int = 0x40b;
const NT_ARM_ZA: c_int = 0x40c;
const NT_ARM_ZT: c_int = 0x40d;
const NT_ARM_FPMR: c_int = 0x40e;

const NT_PRSTATUS: c_int = 1;
const NT_PRFPREG: c_int = 2;

const ARCH_VQ_MAX: usize = 256;

/* VL 128..2048 in powers of 2 */
const MAX_NUM_VLS: usize = 5;

/* Sentinel for detecting buffer bytes the kernel did not write */
const REGSET_SENTINEL: u8 = 0xa5;

/*
 * FPMR bits we can set without doing feature checks to see if values
 * are valid.
 */
const FPMR_SAFE_BITS: u64 =
    FPMR_LSCALE2_MASK | FPMR_NSCALE_MASK | FPMR_LSCALE_MASK | FPMR_OSC_MASK | FPMR_OSM_MASK;

const NUM_FPR: usize = 32;
const __SVE_NUM_ZREGS: usize = 32;
const __SVE_NUM_PREGS: usize = 16;
const ZT_SIG_REG_BYTES: usize = 512;
const HAVE_SVE: c_int = 1 << 0;
const HAVE_SME: c_int = 1 << 1;
const HAVE_SME2: c_int = 1 << 2;
const HAVE_FA64: c_int = 1 << 3;
const HAVE_FPMR: c_int = 1 << 4;
const SVCR_SM: c_int = 1;
const SVCR_ZA: c_int = 2;
const SVE_PT_REGS_FPSIMD: c_int = 0;
const SVE_PT_REGS_SVE: c_int = 1;
const SVE_PT_FPSIMD_OFFSET: usize = 16;
const SVE_PT_REGS_OFFSET: usize = 16;
const SVE_PT_SVE_OFFSET: usize = 16;
const ZA_PT_ZA_OFFSET: usize = 16;
const PR_SVE_VL_LEN_MASK: c_int = 0xffff;
const PR_SVE_SET_VL: c_int = 50;
const PR_SME_SET_VL: c_int = 63;
const PTRACE_TRACEME: c_uint = 0;
const PTRACE_GETREGSET: c_uint = 0x4204;
const PTRACE_SETREGSET: c_uint = 0x4205;
const PTRACE_CONT: c_uint = 7;
const PTRACE_DETACH: c_uint = 17;
const AT_HWCAP: c_ulong = 16;
const AT_HWCAP2: c_ulong = 26;
const HWCAP_SVE: c_ulong = 1 << 22;
const HWCAP2_SME: c_ulong = 1 << 23;
const HWCAP2_SME2: c_ulong = 1 << 37;
const HWCAP2_SME_FA64: c_ulong = 1 << 48;
const HWCAP2_FPMR: c_ulong = 1 << 57;
const EINTR: c_int = 4;
const SIGKILL: c_int = 9;
const SIGALRM: c_int = 14;
const SA_RESTART: c_int = 0x10000000;
const SA_SIGINFO: c_int = 4;

const fn __sve_vq_from_vl(vl: c_int) -> c_int {
    vl / 16
}
const fn sve_vq_from_vl(vl: c_int) -> c_uint {
    (vl / 16) as c_uint
}
const fn __SVE_ZREG_SIZE(vq: usize) -> usize {
    vq * 16
}
const fn __SVE_PREG_SIZE(vq: usize) -> usize {
    vq * 2
}
const fn __SVE_FFR_SIZE(vq: usize) -> usize {
    __SVE_PREG_SIZE(vq)
}
const fn __SVE_ZREGS_SIZE(vq: usize) -> usize {
    __SVE_ZREG_SIZE(vq) * __SVE_NUM_ZREGS
}
const fn __SVE_PREGS_SIZE(vq: usize) -> usize {
    __SVE_PREG_SIZE(vq) * __SVE_NUM_PREGS
}
const fn __SVE_ZREG_OFFSET(vq: usize, n: usize) -> usize {
    __SVE_ZREG_SIZE(vq) * n
}
const fn SVE_PT_SVE_ZREG_OFFSET(vq: usize, n: usize) -> usize {
    SVE_PT_SVE_OFFSET + __SVE_ZREG_OFFSET(vq, n)
}
const fn SVE_PT_SVE_ZREGS_SIZE(vq: usize) -> usize {
    __SVE_ZREGS_SIZE(vq)
}
const fn SVE_PT_SVE_PREG_OFFSET(vq: usize, n: usize) -> usize {
    SVE_PT_SVE_ZREG_OFFSET(vq, __SVE_NUM_ZREGS) + __SVE_PREG_SIZE(vq) * n
}
const fn SVE_PT_SVE_PREGS_SIZE(vq: usize) -> usize {
    __SVE_PREGS_SIZE(vq)
}
const fn SVE_PT_SVE_FFR_OFFSET(vq: usize) -> usize {
    SVE_PT_SVE_PREG_OFFSET(vq, __SVE_NUM_PREGS)
}
const fn SVE_PT_SVE_PREG_SIZE(vq: usize) -> usize {
    __SVE_PREG_SIZE(vq)
}
const fn SVE_PT_SVE_SIZE(vq: usize, _flags: c_int) -> usize {
    __SVE_ZREGS_SIZE(vq) + __SVE_PREGS_SIZE(vq) + __SVE_PREG_SIZE(vq)
}
const fn SVE_PT_SIZE(vq: usize, flags: c_int) -> usize {
    if flags & SVE_PT_REGS_SVE != 0 {
        SVE_PT_SVE_OFFSET + SVE_PT_SVE_SIZE(vq, flags)
    } else {
        SVE_PT_REGS_OFFSET + size_of::<user_fpsimd_state>()
    }
}
const fn ZA_PT_ZA_SIZE(vq: usize) -> usize {
    let vl = vq * 16;
    vl * vl
}
const fn ZA_PT_SIZE(vq: usize) -> usize {
    ZA_PT_ZA_OFFSET + ZA_PT_ZA_SIZE(vq)
}
const fn ZA_SIG_REGS_SIZE(vq: usize) -> usize {
    ZA_PT_ZA_SIZE(vq)
}
const fn ZA_SIG_CONTEXT_SIZE(vq: usize) -> usize {
    ZA_PT_SIZE(vq)
}
const fn ARRAY_SIZE<T, const N: usize>(_: &[T; N]) -> c_int {
    N as c_int
}

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct user_sve_header {
    size: u32,
    max_size: u32,
    vl: u16,
    max_vl: u16,
    flags: u16,
    reserved: u16,
}

#[repr(C)]
struct user_za_header {
    size: u32,
    max_size: u32,
    vl: u16,
    max_vl: u16,
    flags: u16,
    reserved: u16,
}

#[repr(C)]
struct user_fpsimd_state {
    vregs: [__uint128_t; NUM_FPR],
    fpsr: u32,
    fpcr: u32,
}

#[repr(C)]
struct user_pt_regs {
    regs: [u64; 31],
    sp: u64,
    pc: u64,
    pstate: u64,
}

#[repr(C)]
struct sigset_t {
    __val: [c_ulong; 16],
}

#[repr(C)]
struct siginfo_t {
    _data: [u8; 128],
}

#[repr(C)]
struct sigaction {
    sa_sigaction: extern "C" fn(c_int, *mut siginfo_t, *mut c_void),
    sa_mask: sigset_t,
    sa_flags: c_int,
    sa_restorer: Option<extern "C" fn()>,
}

unsafe extern "C" {
    static mut errno: c_int;
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn prctl(option: c_int, ...) -> c_int;
    fn ptrace(request: c_uint, ...) -> c_long;
    fn process_vm_readv(
        pid: pid_t,
        local_iov: *const iovec,
        liovcnt: c_ulong,
        remote_iov: *const iovec,
        riovcnt: c_ulong,
        flags: c_ulong,
    ) -> isize;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn alarm(seconds: c_uint) -> c_uint;
    fn sigemptyset(set: *mut sigset_t) -> c_int;
    fn sigaction(signum: c_int, act: *const sigaction, oldact: *mut sigaction) -> c_int;
    fn fork() -> pid_t;
    fn getpid() -> pid_t;
    fn srandom(seed: c_uint);
    fn random() -> c_long;
    fn malloc(size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memset(s: *mut c_void, c: c_int, n: size_t) -> *mut c_void;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn memcmp(s1: *const c_void, s2: *const c_void, n: size_t) -> c_int;
    fn strerror(errnum: c_int) -> *mut c_char;
    fn snprintf(s: *mut c_char, maxlen: size_t, format: *const c_char, ...) -> c_int;
    fn exit(status: c_int) -> !;

    fn ksft_print_header();
    fn ksft_print_msg(fmt: *const c_char, ...);
    fn ksft_exit_fail() -> !;
    fn ksft_exit_fail_msg(fmt: *const c_char, ...) -> !;
    fn ksft_test_result_skip(fmt: *const c_char, ...);
    fn ksft_test_result(pass: bool, fmt: *const c_char, ...);
    fn ksft_set_plan(plan: c_int);
    fn ksft_finished() -> !;

    fn load_and_save(flags: c_int);
}

static mut v_in: [__uint128_t; NUM_FPR] = [0; NUM_FPR];
static mut v_expected: [__uint128_t; NUM_FPR] = [0; NUM_FPR];
static mut v_out: [__uint128_t; NUM_FPR] = [0; NUM_FPR];

static mut z_in: [c_char; __SVE_ZREGS_SIZE(ARCH_VQ_MAX)] = [0; __SVE_ZREGS_SIZE(ARCH_VQ_MAX)];
static mut z_expected: [c_char; __SVE_ZREGS_SIZE(ARCH_VQ_MAX)] = [0; __SVE_ZREGS_SIZE(ARCH_VQ_MAX)];
static mut z_out: [c_char; __SVE_ZREGS_SIZE(ARCH_VQ_MAX)] = [0; __SVE_ZREGS_SIZE(ARCH_VQ_MAX)];

static mut p_in: [c_char; __SVE_PREGS_SIZE(ARCH_VQ_MAX)] = [0; __SVE_PREGS_SIZE(ARCH_VQ_MAX)];
static mut p_expected: [c_char; __SVE_PREGS_SIZE(ARCH_VQ_MAX)] = [0; __SVE_PREGS_SIZE(ARCH_VQ_MAX)];
static mut p_out: [c_char; __SVE_PREGS_SIZE(ARCH_VQ_MAX)] = [0; __SVE_PREGS_SIZE(ARCH_VQ_MAX)];

static mut ffr_in: [c_char; __SVE_PREG_SIZE(ARCH_VQ_MAX)] = [0; __SVE_PREG_SIZE(ARCH_VQ_MAX)];
static mut ffr_expected: [c_char; __SVE_PREG_SIZE(ARCH_VQ_MAX)] = [0; __SVE_PREG_SIZE(ARCH_VQ_MAX)];
static mut ffr_out: [c_char; __SVE_PREG_SIZE(ARCH_VQ_MAX)] = [0; __SVE_PREG_SIZE(ARCH_VQ_MAX)];

static mut za_in: [c_char; ZA_SIG_REGS_SIZE(ARCH_VQ_MAX)] = [0; ZA_SIG_REGS_SIZE(ARCH_VQ_MAX)];
static mut za_expected: [c_char; ZA_SIG_REGS_SIZE(ARCH_VQ_MAX)] = [0; ZA_SIG_REGS_SIZE(ARCH_VQ_MAX)];
static mut za_out: [c_char; ZA_SIG_REGS_SIZE(ARCH_VQ_MAX)] = [0; ZA_SIG_REGS_SIZE(ARCH_VQ_MAX)];

static mut zt_in: [c_char; ZT_SIG_REG_BYTES] = [0; ZT_SIG_REG_BYTES];
static mut zt_expected: [c_char; ZT_SIG_REG_BYTES] = [0; ZT_SIG_REG_BYTES];
static mut zt_out: [c_char; ZT_SIG_REG_BYTES] = [0; ZT_SIG_REG_BYTES];

static mut fpmr_in: uint64_t = 0;
static mut fpmr_expected: uint64_t = 0;
static mut fpmr_out: uint64_t = 0;

static mut sve_vl_out: uint64_t = 0;
static mut sme_vl_out: uint64_t = 0;
static mut svcr_in: uint64_t = 0;
static mut svcr_expected: uint64_t = 0;
static mut svcr_out: uint64_t = 0;

static mut got_alarm: bool = false;

extern "C" fn handle_alarm(_sig: c_int, _info: *mut siginfo_t, _context: *mut c_void) {
    unsafe {
        got_alarm = true;
    }
}

/* CONFIG_CPU_BIG_ENDIAN maps this helper to a byte-swapping version in C. */
fn arm64_cpu_to_le128(x: __uint128_t) -> __uint128_t {
    x
}

fn arm64_le128_to_cpu(x: __uint128_t) -> __uint128_t {
    arm64_cpu_to_le128(x)
}

unsafe fn sve_supported() -> bool {
    getauxval(AT_HWCAP) & HWCAP_SVE != 0
}

unsafe fn sme_supported() -> bool {
    getauxval(AT_HWCAP2) & HWCAP2_SME != 0
}

unsafe fn sme2_supported() -> bool {
    getauxval(AT_HWCAP2) & HWCAP2_SME2 != 0
}

unsafe fn fa64_supported() -> bool {
    getauxval(AT_HWCAP2) & HWCAP2_SME_FA64 != 0
}

unsafe fn fpmr_supported() -> bool {
    getauxval(AT_HWCAP2) & HWCAP2_FPMR != 0
}

unsafe fn compare_buffer(name: *const c_char, out: *mut c_void, expected: *mut c_void, size: size_t) -> bool {
    let tmp: *mut c_void;

    if memcmp(out, expected, size) == 0 {
        return true;
    }

    ksft_print_msg(c"Mismatch in %s\n".as_ptr(), name);

    /* Did we just get zeros back? */
    tmp = malloc(size);
    if tmp.is_null() {
        ksft_print_msg(c"OOM allocating %lu bytes for %s\n".as_ptr(), size, name);
        ksft_exit_fail();
    }
    memset(tmp, 0, size);

    if memcmp(out, tmp, size) == 0 {
        ksft_print_msg(c"%s is zero\n".as_ptr(), name);
    }

    free(tmp);

    false
}

unsafe fn buffer_is_filled(buffer: *const c_void, size: size_t, value: u8) -> bool {
    let bytes = buffer as *const u8;
    let mut i: size_t = 0;

    while i < size {
        if *bytes.add(i) != value {
            return false;
        }
        i += 1;
    }

    true
}

#[repr(C)]
struct test_config {
    sve_vl_in: c_int,
    sve_vl_expected: c_int,
    sme_vl_in: c_int,
    sme_vl_expected: c_int,
    svcr_in: c_int,
    svcr_expected: c_int,
}

#[repr(C)]
struct test_definition {
    name: *const c_char,
    sve_vl_change: bool,
    supported: Option<unsafe fn(*mut test_config) -> bool>,
    set_expected_values: Option<unsafe fn(*mut test_config)>,
    modify_values: Option<unsafe fn(pid_t, *mut test_config)>,
}

unsafe fn vl_in(config: *mut test_config) -> c_int {
    if (*config).svcr_in & SVCR_SM != 0 {
        (*config).sme_vl_in
    } else {
        (*config).sve_vl_in
    }
}

unsafe fn vl_expected(config: *mut test_config) -> c_int {
    if (*config).svcr_expected & SVCR_SM != 0 {
        (*config).sme_vl_expected
    } else {
        (*config).sve_vl_expected
    }
}

unsafe fn run_child(config: *mut test_config) {
    let mut ret: c_int;
    let mut flags: c_int;

    /* Let the parent attach to us */
    ret = ptrace(PTRACE_TRACEME, 0, 0, 0) as c_int;
    if ret < 0 {
        ksft_exit_fail_msg(c"PTRACE_TRACEME failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
    }

    /* VL setup */
    if sve_supported() {
        ret = prctl(PR_SVE_SET_VL, (*config).sve_vl_in);
        if ret != (*config).sve_vl_in {
            ksft_print_msg(c"Failed to set SVE VL %d: %d\n".as_ptr(), (*config).sve_vl_in, ret);
        }
    }

    if sme_supported() {
        ret = prctl(PR_SME_SET_VL, (*config).sme_vl_in);
        if ret != (*config).sme_vl_in {
            ksft_print_msg(c"Failed to set SME VL %d: %d\n".as_ptr(), (*config).sme_vl_in, ret);
        }
    }

    /* Load values and wait for the parent */
    flags = 0;
    if sve_supported() {
        flags |= HAVE_SVE;
    }
    if sme_supported() {
        flags |= HAVE_SME;
    }
    if sme2_supported() {
        flags |= HAVE_SME2;
    }
    if fa64_supported() {
        flags |= HAVE_FA64;
    }
    if fpmr_supported() {
        flags |= HAVE_FPMR;
    }

    load_and_save(flags);

    exit(0);
}

unsafe fn read_one_child_regs(child: pid_t, name: *mut c_char, iov_parent: *mut iovec, iov_child: *mut iovec) {
    let len = (*iov_parent).iov_len as isize;
    let ret = process_vm_readv(child, iov_parent, 1, iov_child, 1, 0);
    if ret == -1 {
        ksft_print_msg(c"%s read failed: %s (%d)\n".as_ptr(), name, strerror(errno), errno);
    } else if ret != len {
        ksft_print_msg(c"Short read of %s: %d\n".as_ptr(), name, ret as c_int);
    }
}

unsafe fn read_child_regs(child: pid_t) {
    let mut iov_parent: iovec = zeroed();
    let mut iov_child: iovec = zeroed();

    /*
     * Since the child fork()ed from us the buffer addresses are
     * the same in parent and child.
     */
    iov_parent.iov_base = ptr::addr_of_mut!(v_out) as *mut c_void;
    iov_parent.iov_len = size_of_val_raw(ptr::addr_of!(v_out));
    iov_child.iov_base = ptr::addr_of_mut!(v_out) as *mut c_void;
    iov_child.iov_len = size_of_val_raw(ptr::addr_of!(v_out));
    read_one_child_regs(child, c"FPSIMD".as_ptr() as *mut c_char, &mut iov_parent, &mut iov_child);

    if sve_supported() || sme_supported() {
        iov_parent.iov_base = ptr::addr_of_mut!(sve_vl_out) as *mut c_void;
        iov_parent.iov_len = size_of::<uint64_t>();
        iov_child.iov_base = ptr::addr_of_mut!(sve_vl_out) as *mut c_void;
        iov_child.iov_len = size_of::<uint64_t>();
        read_one_child_regs(child, c"SVE VL".as_ptr() as *mut c_char, &mut iov_parent, &mut iov_child);

        iov_parent.iov_base = ptr::addr_of_mut!(z_out) as *mut c_void;
        iov_parent.iov_len = size_of_val_raw(ptr::addr_of!(z_out));
        iov_child.iov_base = ptr::addr_of_mut!(z_out) as *mut c_void;
        iov_child.iov_len = size_of_val_raw(ptr::addr_of!(z_out));
        read_one_child_regs(child, c"Z".as_ptr() as *mut c_char, &mut iov_parent, &mut iov_child);

        iov_parent.iov_base = ptr::addr_of_mut!(p_out) as *mut c_void;
        iov_parent.iov_len = size_of_val_raw(ptr::addr_of!(p_out));
        iov_child.iov_base = ptr::addr_of_mut!(p_out) as *mut c_void;
        iov_child.iov_len = size_of_val_raw(ptr::addr_of!(p_out));
        read_one_child_regs(child, c"P".as_ptr() as *mut c_char, &mut iov_parent, &mut iov_child);

        iov_parent.iov_base = ptr::addr_of_mut!(ffr_out) as *mut c_void;
        iov_parent.iov_len = size_of_val_raw(ptr::addr_of!(ffr_out));
        iov_child.iov_base = ptr::addr_of_mut!(ffr_out) as *mut c_void;
        iov_child.iov_len = size_of_val_raw(ptr::addr_of!(ffr_out));
        read_one_child_regs(child, c"FFR".as_ptr() as *mut c_char, &mut iov_parent, &mut iov_child);
    }

    if sme_supported() {
        iov_parent.iov_base = ptr::addr_of_mut!(sme_vl_out) as *mut c_void;
        iov_parent.iov_len = size_of::<uint64_t>();
        iov_child.iov_base = ptr::addr_of_mut!(sme_vl_out) as *mut c_void;
        iov_child.iov_len = size_of::<uint64_t>();
        read_one_child_regs(child, c"SME VL".as_ptr() as *mut c_char, &mut iov_parent, &mut iov_child);

        iov_parent.iov_base = ptr::addr_of_mut!(svcr_out) as *mut c_void;
        iov_parent.iov_len = size_of::<uint64_t>();
        iov_child.iov_base = ptr::addr_of_mut!(svcr_out) as *mut c_void;
        iov_child.iov_len = size_of::<uint64_t>();
        read_one_child_regs(child, c"SVCR".as_ptr() as *mut c_char, &mut iov_parent, &mut iov_child);

        iov_parent.iov_base = ptr::addr_of_mut!(za_out) as *mut c_void;
        iov_parent.iov_len = size_of_val_raw(ptr::addr_of!(za_out));
        iov_child.iov_base = ptr::addr_of_mut!(za_out) as *mut c_void;
        iov_child.iov_len = size_of_val_raw(ptr::addr_of!(za_out));
        read_one_child_regs(child, c"ZA".as_ptr() as *mut c_char, &mut iov_parent, &mut iov_child);
    }

    if sme2_supported() {
        iov_parent.iov_base = ptr::addr_of_mut!(zt_out) as *mut c_void;
        iov_parent.iov_len = size_of_val_raw(ptr::addr_of!(zt_out));
        iov_child.iov_base = ptr::addr_of_mut!(zt_out) as *mut c_void;
        iov_child.iov_len = size_of_val_raw(ptr::addr_of!(zt_out));
        read_one_child_regs(child, c"ZT".as_ptr() as *mut c_char, &mut iov_parent, &mut iov_child);
    }

    if fpmr_supported() {
        iov_parent.iov_base = ptr::addr_of_mut!(fpmr_out) as *mut c_void;
        iov_parent.iov_len = size_of::<uint64_t>();
        iov_child.iov_base = ptr::addr_of_mut!(fpmr_out) as *mut c_void;
        iov_child.iov_len = size_of::<uint64_t>();
        read_one_child_regs(child, c"FPMR".as_ptr() as *mut c_char, &mut iov_parent, &mut iov_child);
    }
}

unsafe fn size_of_val_raw<T>(_: *const T) -> usize {
    size_of::<T>()
}

unsafe fn WIFEXITED(status: c_int) -> bool {
    status & 0x7f == 0
}
unsafe fn WEXITSTATUS(status: c_int) -> c_int {
    (status & 0xff00) >> 8
}
unsafe fn WIFSIGNALED(status: c_int) -> bool {
    (((status & 0x7f) + 1) >> 1) > 0
}
unsafe fn WTERMSIG(status: c_int) -> c_int {
    status & 0x7f
}

unsafe fn continue_breakpoint(child: pid_t, restart_type: c_uint) -> bool {
    let mut pt_regs: user_pt_regs = zeroed();
    let mut iov: iovec = zeroed();
    let mut ret: c_int;

    /* Get PC */
    iov.iov_base = &mut pt_regs as *mut _ as *mut c_void;
    iov.iov_len = size_of::<user_pt_regs>();
    ret = ptrace(PTRACE_GETREGSET, child, NT_PRSTATUS, &mut iov) as c_int;
    if ret < 0 {
        ksft_print_msg(c"Failed to get PC: %s (%d)\n".as_ptr(), strerror(errno), errno);
        return false;
    }

    /* Skip over the BRK */
    pt_regs.pc = pt_regs.pc.wrapping_add(4);
    ret = ptrace(PTRACE_SETREGSET, child, NT_PRSTATUS, &mut iov) as c_int;
    if ret < 0 {
        ksft_print_msg(c"Failed to skip BRK: %s (%d)\n".as_ptr(), strerror(errno), errno);
        return false;
    }

    /* Restart */
    ret = ptrace(restart_type, child, 0, 0) as c_int;
    if ret < 0 {
        ksft_print_msg(c"Failed to restart child: %s (%d)\n".as_ptr(), strerror(errno), errno);
        return false;
    }

    true
}

unsafe fn check_ptrace_values_sve(child: pid_t, config: *mut test_config) -> bool {
    let mut sve: *mut user_sve_header;
    let mut fpsimd: *mut user_fpsimd_state;
    let mut iov: iovec = zeroed();
    let buf_size: size_t;
    let ret: c_int;
    let vq: c_int;
    let mut pass = true;

    if !sve_supported() {
        return true;
    }

    vq = __sve_vq_from_vl((*config).sve_vl_in);
    buf_size = SVE_PT_SVE_OFFSET + SVE_PT_SVE_SIZE(vq as usize, SVE_PT_REGS_SVE);
    iov.iov_len = buf_size;
    iov.iov_base = malloc(buf_size);
    if iov.iov_base.is_null() {
        ksft_print_msg(c"OOM allocating %lu byte SVE buffer\n".as_ptr(), iov.iov_len);
        return false;
    }

    memset(iov.iov_base, REGSET_SENTINEL as c_int, buf_size);
    ret = ptrace(PTRACE_GETREGSET, child, NT_ARM_SVE, &mut iov) as c_int;
    if ret != 0 {
        ksft_print_msg(c"Failed to read initial SVE: %s (%d)\n".as_ptr(), strerror(errno), errno);
        pass = false;
        free(iov.iov_base);
        return pass;
    }

    sve = iov.iov_base as *mut user_sve_header;

    if (*sve).vl as c_int != (*config).sve_vl_in {
        ksft_print_msg(c"Mismatch in initial SVE VL: %d != %d\n".as_ptr(), (*sve).vl as c_int, (*config).sve_vl_in);
        pass = false;
    }

    /* If we are in streaming mode we should just read FPSIMD */
    if ((*config).svcr_in & SVCR_SM != 0) && ((*sve).flags as c_int & SVE_PT_REGS_SVE != 0) {
        ksft_print_msg(c"NT_ARM_SVE reports SVE with PSTATE.SM\n".as_ptr());
        pass = false;
    }

    if svcr_in & SVCR_SM as u64 != 0 {
        if (*sve).size as usize != size_of::<user_sve_header>() {
            ksft_print_msg(c"NT_ARM_SVE reports data with PSTATE.SM\n".as_ptr());
            pass = false;
        }
        if !buffer_is_filled((iov.iov_base as *mut u8).add(size_of::<user_sve_header>()) as *const c_void, buf_size - size_of::<user_sve_header>(), REGSET_SENTINEL) {
            ksft_print_msg(c"NT_ARM_SVE wrote beyond its header with PSTATE.SM\n".as_ptr());
            pass = false;
        }
        free(iov.iov_base);
        return pass;
    } else if (*sve).size as usize != SVE_PT_SIZE(vq as usize, (*sve).flags as c_int) {
        ksft_print_msg(c"Mismatch in SVE header size: %d != %lu\n".as_ptr(), (*sve).size as c_int, SVE_PT_SIZE(vq as usize, (*sve).flags as c_int));
        pass = false;
    }

    /* The registers might be in completely different formats! */
    if (*sve).flags as c_int & SVE_PT_REGS_SVE != 0 {
        if !compare_buffer(c"initial SVE Z".as_ptr(), (iov.iov_base as *mut u8).add(SVE_PT_SVE_ZREG_OFFSET(vq as usize, 0)) as *mut c_void, ptr::addr_of_mut!(z_in) as *mut c_void, SVE_PT_SVE_ZREGS_SIZE(vq as usize)) {
            pass = false;
        }
        if !compare_buffer(c"initial SVE P".as_ptr(), (iov.iov_base as *mut u8).add(SVE_PT_SVE_PREG_OFFSET(vq as usize, 0)) as *mut c_void, ptr::addr_of_mut!(p_in) as *mut c_void, SVE_PT_SVE_PREGS_SIZE(vq as usize)) {
            pass = false;
        }
        if !compare_buffer(c"initial SVE FFR".as_ptr(), (iov.iov_base as *mut u8).add(SVE_PT_SVE_FFR_OFFSET(vq as usize)) as *mut c_void, ptr::addr_of_mut!(ffr_in) as *mut c_void, SVE_PT_SVE_PREG_SIZE(vq as usize)) {
            pass = false;
        }
    } else {
        fpsimd = (iov.iov_base as *mut u8).add(SVE_PT_FPSIMD_OFFSET) as *mut user_fpsimd_state;
        if !compare_buffer(c"initial V via SVE".as_ptr(), ptr::addr_of_mut!((*fpsimd).vregs) as *mut c_void, ptr::addr_of_mut!(v_in) as *mut c_void, size_of_val_raw(ptr::addr_of!(v_in))) {
            pass = false;
        }
    }

    free(iov.iov_base);
    pass
}

unsafe fn check_ptrace_values_ssve(child: pid_t, config: *mut test_config) -> bool {
    let mut sve: *mut user_sve_header;
    let mut fpsimd: *mut user_fpsimd_state;
    let mut iov: iovec = zeroed();
    let buf_size: size_t;
    let ret: c_int;
    let vq: c_int;
    let mut pass = true;

    if !sme_supported() {
        return true;
    }

    vq = __sve_vq_from_vl((*config).sme_vl_in);

    buf_size = SVE_PT_SVE_OFFSET + SVE_PT_SVE_SIZE(vq as usize, SVE_PT_REGS_SVE);
    iov.iov_len = buf_size;
    iov.iov_base = malloc(buf_size);
    if iov.iov_base.is_null() {
        ksft_print_msg(c"OOM allocating %lu byte SSVE buffer\n".as_ptr(), iov.iov_len);
        return false;
    }

    memset(iov.iov_base, REGSET_SENTINEL as c_int, buf_size);
    ret = ptrace(PTRACE_GETREGSET, child, NT_ARM_SSVE, &mut iov) as c_int;
    if ret != 0 {
        ksft_print_msg(c"Failed to read initial SSVE: %s (%d)\n".as_ptr(), strerror(errno), errno);
        pass = false;
        free(iov.iov_base);
        return pass;
    }

    sve = iov.iov_base as *mut user_sve_header;

    if (*sve).vl as c_int != (*config).sme_vl_in {
        ksft_print_msg(c"Mismatch in initial SSVE VL: %d != %d\n".as_ptr(), (*sve).vl as c_int, (*config).sme_vl_in);
        pass = false;
    }

    if ((*config).svcr_in & SVCR_SM != 0) && !((*sve).flags as c_int & SVE_PT_REGS_SVE != 0) {
        ksft_print_msg(c"NT_ARM_SSVE reports FPSIMD with PSTATE.SM\n".as_ptr());
        pass = false;
    }

    if !(svcr_in & SVCR_SM as u64 != 0) {
        if (*sve).size as usize != size_of::<user_sve_header>() {
            ksft_print_msg(c"NT_ARM_SSVE reports data without PSTATE.SM\n".as_ptr());
            pass = false;
        }
        if !buffer_is_filled((iov.iov_base as *mut u8).add(size_of::<user_sve_header>()) as *const c_void, buf_size - size_of::<user_sve_header>(), REGSET_SENTINEL) {
            ksft_print_msg(c"NT_ARM_SSVE wrote beyond its header without PSTATE.SM\n".as_ptr());
            pass = false;
        }
        free(iov.iov_base);
        return pass;
    } else if (*sve).size as usize != SVE_PT_SIZE(vq as usize, (*sve).flags as c_int) {
        ksft_print_msg(c"Mismatch in SSVE header size: %d != %lu\n".as_ptr(), (*sve).size as c_int, SVE_PT_SIZE(vq as usize, (*sve).flags as c_int));
        pass = false;
    }

    /* The registers might be in completely different formats! */
    if (*sve).flags as c_int & SVE_PT_REGS_SVE != 0 {
        if !compare_buffer(c"initial SSVE Z".as_ptr(), (iov.iov_base as *mut u8).add(SVE_PT_SVE_ZREG_OFFSET(vq as usize, 0)) as *mut c_void, ptr::addr_of_mut!(z_in) as *mut c_void, SVE_PT_SVE_ZREGS_SIZE(vq as usize)) {
            pass = false;
        }
        if !compare_buffer(c"initial SSVE P".as_ptr(), (iov.iov_base as *mut u8).add(SVE_PT_SVE_PREG_OFFSET(vq as usize, 0)) as *mut c_void, ptr::addr_of_mut!(p_in) as *mut c_void, SVE_PT_SVE_PREGS_SIZE(vq as usize)) {
            pass = false;
        }
        if !compare_buffer(c"initial SSVE FFR".as_ptr(), (iov.iov_base as *mut u8).add(SVE_PT_SVE_FFR_OFFSET(vq as usize)) as *mut c_void, ptr::addr_of_mut!(ffr_in) as *mut c_void, SVE_PT_SVE_PREG_SIZE(vq as usize)) {
            pass = false;
        }
    } else {
        fpsimd = (iov.iov_base as *mut u8).add(SVE_PT_FPSIMD_OFFSET) as *mut user_fpsimd_state;
        if !compare_buffer(c"initial V via SSVE".as_ptr(), ptr::addr_of_mut!((*fpsimd).vregs) as *mut c_void, ptr::addr_of_mut!(v_in) as *mut c_void, size_of_val_raw(ptr::addr_of!(v_in))) {
            pass = false;
        }
    }

    free(iov.iov_base);
    pass
}

unsafe fn check_ptrace_values_za(child: pid_t, config: *mut test_config) -> bool {
    let mut za: *mut user_za_header;
    let mut iov: iovec = zeroed();
    let ret: c_int;
    let vq: c_int;
    let mut pass = true;

    if !sme_supported() {
        return true;
    }

    vq = __sve_vq_from_vl((*config).sme_vl_in);
    iov.iov_len = ZA_SIG_CONTEXT_SIZE(vq as usize);
    iov.iov_base = malloc(iov.iov_len);
    if iov.iov_base.is_null() {
        ksft_print_msg(c"OOM allocating %lu byte ZA buffer\n".as_ptr(), iov.iov_len);
        return false;
    }

    ret = ptrace(PTRACE_GETREGSET, child, NT_ARM_ZA, &mut iov) as c_int;
    if ret != 0 {
        ksft_print_msg(c"Failed to read initial ZA: %s (%d)\n".as_ptr(), strerror(errno), errno);
        free(iov.iov_base);
        return false;
    }

    za = iov.iov_base as *mut user_za_header;
    if (*za).vl as c_int != (*config).sme_vl_in {
        ksft_print_msg(c"Mismatch in initial SME VL: %d != %d\n".as_ptr(), (*za).vl as c_int, (*config).sme_vl_in);
        pass = false;
    }

    /* If PSTATE.ZA is not set we should just read the header */
    if (*config).svcr_in & SVCR_ZA != 0 {
        if (*za).size as usize != ZA_PT_SIZE(vq as usize) {
            ksft_print_msg(c"Unexpected ZA ptrace read size: %d != %lu\n".as_ptr(), (*za).size as c_int, ZA_PT_SIZE(vq as usize));
            pass = false;
        }
        if !compare_buffer(c"initial ZA".as_ptr(), (iov.iov_base as *mut u8).add(ZA_PT_ZA_OFFSET) as *mut c_void, ptr::addr_of_mut!(za_in) as *mut c_void, ZA_PT_ZA_SIZE(vq as usize)) {
            pass = false;
        }
    } else if (*za).size as usize != size_of::<user_za_header>() {
        ksft_print_msg(c"Unexpected ZA ptrace read size: %d != %lu\n".as_ptr(), (*za).size as c_int, size_of::<user_za_header>());
        pass = false;
    }

    free(iov.iov_base);
    pass
}

unsafe fn check_ptrace_values_zt(child: pid_t, _config: *mut test_config) -> bool {
    let mut buf = [0u8; 512];
    let mut iov: iovec = zeroed();
    let ret: c_int;

    if !sme2_supported() {
        return true;
    }

    iov.iov_base = buf.as_mut_ptr() as *mut c_void;
    iov.iov_len = ZT_SIG_REG_BYTES;
    ret = ptrace(PTRACE_GETREGSET, child, NT_ARM_ZT, &mut iov) as c_int;
    if ret != 0 {
        ksft_print_msg(c"Failed to read initial ZT: %s (%d)\n".as_ptr(), strerror(errno), errno);
        return false;
    }

    compare_buffer(c"initial ZT".as_ptr(), buf.as_mut_ptr() as *mut c_void, ptr::addr_of_mut!(zt_in) as *mut c_void, ZT_SIG_REG_BYTES)
}

unsafe fn check_ptrace_values_fpmr(child: pid_t, _config: *mut test_config) -> bool {
    let mut val: uint64_t = 0;
    let mut iov: iovec = zeroed();
    let ret: c_int;

    if !fpmr_supported() {
        return true;
    }

    iov.iov_base = &mut val as *mut _ as *mut c_void;
    iov.iov_len = size_of::<uint64_t>();
    ret = ptrace(PTRACE_GETREGSET, child, NT_ARM_FPMR, &mut iov) as c_int;
    if ret != 0 {
        ksft_print_msg(c"Failed to read initial FPMR: %s (%d)\n".as_ptr(), strerror(errno), errno);
        return false;
    }

    compare_buffer(c"initial FPMR".as_ptr(), &mut val as *mut _ as *mut c_void, ptr::addr_of_mut!(fpmr_in) as *mut c_void, size_of::<uint64_t>())
}

unsafe fn check_ptrace_values(child: pid_t, config: *mut test_config) -> bool {
    let mut pass = true;
    let mut fpsimd: user_fpsimd_state = zeroed();
    let mut iov: iovec = zeroed();
    let ret: c_int;

    iov.iov_base = &mut fpsimd as *mut _ as *mut c_void;
    iov.iov_len = size_of::<user_fpsimd_state>();
    ret = ptrace(PTRACE_GETREGSET, child, NT_PRFPREG, &mut iov) as c_int;
    if ret == 0 {
        if !compare_buffer(c"initial V".as_ptr(), ptr::addr_of_mut!(fpsimd.vregs) as *mut c_void, ptr::addr_of_mut!(v_in) as *mut c_void, size_of_val_raw(ptr::addr_of!(v_in))) {
            pass = false;
        }
    } else {
        ksft_print_msg(c"Failed to read initial V: %s (%d)\n".as_ptr(), strerror(errno), errno);
        pass = false;
    }

    if !check_ptrace_values_sve(child, config) { pass = false; }
    if !check_ptrace_values_ssve(child, config) { pass = false; }
    if !check_ptrace_values_za(child, config) { pass = false; }
    if !check_ptrace_values_zt(child, config) { pass = false; }
    if !check_ptrace_values_fpmr(child, config) { pass = false; }

    pass
}

unsafe fn run_parent(child: pid_t, test: *mut test_definition, config: *mut test_config) -> bool {
    let mut wait_status: c_int = 0;
    let mut pid: pid_t;
    let mut pass: bool;

    /* Initial attach */
    loop {
        pid = waitpid(child, &mut wait_status, 0);
        if pid < 0 {
            if errno == EINTR { continue; }
            ksft_exit_fail_msg(c"waitpid() failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
        }
        if pid == child { break; }
    }

    if WIFEXITED(wait_status) {
        ksft_print_msg(c"Child exited loading values with status %d\n".as_ptr(), WEXITSTATUS(wait_status));
        return false;
    }
    if WIFSIGNALED(wait_status) {
        ksft_print_msg(c"Child died from signal %d loading values\n".as_ptr(), WTERMSIG(wait_status));
        return false;
    }

    /* Read initial values via ptrace */
    pass = check_ptrace_values(child, config);

    /* Do whatever writes we want to do */
    if let Some(modify_values) = (*test).modify_values {
        modify_values(child, config);
    }

    if !continue_breakpoint(child, PTRACE_CONT) {
        kill(child, SIGKILL);
        return false;
    }

    loop {
        pid = waitpid(child, &mut wait_status, 0);
        if pid < 0 {
            if errno == EINTR { continue; }
            ksft_exit_fail_msg(c"waitpid() failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
        }
        if pid == child { break; }
    }

    if WIFEXITED(wait_status) {
        ksft_print_msg(c"Child exited saving values with status %d\n".as_ptr(), WEXITSTATUS(wait_status));
        return false;
    }
    if WIFSIGNALED(wait_status) {
        ksft_print_msg(c"Child died from signal %d saving values\n".as_ptr(), WTERMSIG(wait_status));
        return false;
    }

    /* See what happened as a result */
    read_child_regs(child);

    if !continue_breakpoint(child, PTRACE_DETACH) {
        kill(child, SIGKILL);
        return false;
    }

    /* The child should exit cleanly */
    got_alarm = false;
    alarm(1);
    loop {
        if got_alarm {
            ksft_print_msg(c"Wait for child timed out\n".as_ptr());
            kill(child, SIGKILL);
            return false;
        }
        pid = waitpid(child, &mut wait_status, 0);
        if pid < 0 {
            if errno == EINTR { continue; }
            ksft_exit_fail_msg(c"waitpid() failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
        }
        if pid == child { break; }
    }
    alarm(0);

    if got_alarm {
        ksft_print_msg(c"Timed out waiting for child\n".as_ptr());
        pass = false;
    } else if pid == child && WIFSIGNALED(wait_status) {
        ksft_print_msg(c"Child died from signal %d cleaning up\n".as_ptr(), WTERMSIG(wait_status));
        pass = false;
    } else if pid == child && WIFEXITED(wait_status) {
        if WEXITSTATUS(wait_status) != 0 {
            ksft_print_msg(c"Child exited with error %d\n".as_ptr(), WEXITSTATUS(wait_status));
            pass = false;
        }
    } else {
        ksft_print_msg(c"Child did not exit cleanly\n".as_ptr());
        pass = false;
    }

    pass
}

unsafe fn fill_random(buf: *mut c_void, size: size_t) {
    let lbuf = buf as *mut uint32_t;
    let mut i: c_int = 0;

    /* random() returns a 32 bit number regardless of the size of long */
    while (i as usize) < size / size_of::<uint32_t>() {
        *lbuf.add(i as usize) = random() as uint32_t;
        i += 1;
    }
}

unsafe fn fill_random_ffr(buf: *mut c_void, vq: size_t) {
    let lbuf = buf as *mut uint8_t;
    let mut i: c_int;

    /*
     * Only values with a continuous set of 0..n bits set are
     * valid for FFR, set all bits then clear a random number of
     * high bits.
     */
    memset(buf, 0, __SVE_FFR_SIZE(vq));
    let bits = (random() as usize % (__SVE_FFR_SIZE(vq) * 8)) as c_int;
    i = 0;
    while i < bits / 8 {
        *lbuf.add(i as usize) = 0xff;
        i += 1;
    }
    if bits / 8 != __SVE_FFR_SIZE(vq) as c_int {
        *lbuf.add(i as usize) = ((1 << (bits % 8)) - 1) as u8;
    }
}

unsafe fn fpsimd_to_sve(v: *mut __uint128_t, z: *mut c_char, vl: c_int) {
    let vq = __sve_vq_from_vl(vl) as usize;
    let mut i: c_int = 0;
    let mut p: *mut __uint128_t;

    if vl == 0 {
        return;
    }

    while i < __SVE_NUM_ZREGS as c_int {
        p = z.add(__SVE_ZREG_OFFSET(vq, i as usize)) as *mut __uint128_t;
        *p = arm64_cpu_to_le128(*v.add(i as usize));
        i += 1;
    }
}

unsafe fn set_initial_values(config: *mut test_config) {
    let vq = __sve_vq_from_vl(vl_in(config)) as usize;
    let sme_vq = __sve_vq_from_vl((*config).sme_vl_in) as usize;

    svcr_in = (*config).svcr_in as u64;
    svcr_expected = (*config).svcr_expected as u64;
    svcr_out = 0;

    fill_random(ptr::addr_of_mut!(v_in) as *mut c_void, size_of_val_raw(ptr::addr_of!(v_in)));
    memcpy(ptr::addr_of_mut!(v_expected) as *mut c_void, ptr::addr_of!(v_in) as *const c_void, size_of_val_raw(ptr::addr_of!(v_in)));
    memset(ptr::addr_of_mut!(v_out) as *mut c_void, 0, size_of_val_raw(ptr::addr_of!(v_out)));

    /* Changes will be handled in the test case */
    if sve_supported() || ((*config).svcr_in & SVCR_SM != 0) {
        /* The low 128 bits of Z are shared with the V registers */
        fill_random(ptr::addr_of_mut!(z_in) as *mut c_void, __SVE_ZREGS_SIZE(vq));
        fpsimd_to_sve(ptr::addr_of_mut!(v_in) as *mut __uint128_t, ptr::addr_of_mut!(z_in) as *mut c_char, vl_in(config));
        memcpy(ptr::addr_of_mut!(z_expected) as *mut c_void, ptr::addr_of!(z_in) as *const c_void, __SVE_ZREGS_SIZE(vq));
        memset(ptr::addr_of_mut!(z_out) as *mut c_void, 0, size_of_val_raw(ptr::addr_of!(z_out)));

        fill_random(ptr::addr_of_mut!(p_in) as *mut c_void, __SVE_PREGS_SIZE(vq));
        memcpy(ptr::addr_of_mut!(p_expected) as *mut c_void, ptr::addr_of!(p_in) as *const c_void, __SVE_PREGS_SIZE(vq));
        memset(ptr::addr_of_mut!(p_out) as *mut c_void, 0, size_of_val_raw(ptr::addr_of!(p_out)));

        if ((*config).svcr_in & SVCR_SM != 0) && !fa64_supported() {
            memset(ptr::addr_of_mut!(ffr_in) as *mut c_void, 0, __SVE_PREG_SIZE(vq));
        } else {
            fill_random_ffr(ptr::addr_of_mut!(ffr_in) as *mut c_void, vq);
        }
        memcpy(ptr::addr_of_mut!(ffr_expected) as *mut c_void, ptr::addr_of!(ffr_in) as *const c_void, __SVE_PREG_SIZE(vq));
        memset(ptr::addr_of_mut!(ffr_out) as *mut c_void, 0, __SVE_PREG_SIZE(vq));
    }

    if (*config).svcr_in & SVCR_ZA != 0 {
        fill_random(ptr::addr_of_mut!(za_in) as *mut c_void, ZA_SIG_REGS_SIZE(sme_vq));
    } else {
        memset(ptr::addr_of_mut!(za_in) as *mut c_void, 0, ZA_SIG_REGS_SIZE(sme_vq));
    }
    if (*config).svcr_expected & SVCR_ZA != 0 {
        memcpy(ptr::addr_of_mut!(za_expected) as *mut c_void, ptr::addr_of!(za_in) as *const c_void, ZA_SIG_REGS_SIZE(sme_vq));
    } else {
        memset(ptr::addr_of_mut!(za_expected) as *mut c_void, 0, ZA_SIG_REGS_SIZE(sme_vq));
    }
    if sme_supported() {
        memset(ptr::addr_of_mut!(za_out) as *mut c_void, 0, size_of_val_raw(ptr::addr_of!(za_out)));
    }

    if sme2_supported() {
        if (*config).svcr_in & SVCR_ZA != 0 {
            fill_random(ptr::addr_of_mut!(zt_in) as *mut c_void, ZT_SIG_REG_BYTES);
        } else {
            memset(ptr::addr_of_mut!(zt_in) as *mut c_void, 0, ZT_SIG_REG_BYTES);
        }
        if (*config).svcr_expected & SVCR_ZA != 0 {
            memcpy(ptr::addr_of_mut!(zt_expected) as *mut c_void, ptr::addr_of!(zt_in) as *const c_void, ZT_SIG_REG_BYTES);
        } else {
            memset(ptr::addr_of_mut!(zt_expected) as *mut c_void, 0, ZT_SIG_REG_BYTES);
        }
        memset(ptr::addr_of_mut!(zt_out) as *mut c_void, 0, size_of_val_raw(ptr::addr_of!(zt_out)));
    }

    if fpmr_supported() {
        fill_random(ptr::addr_of_mut!(fpmr_in) as *mut c_void, size_of::<uint64_t>());
        fpmr_in &= FPMR_SAFE_BITS;
        fpmr_expected = fpmr_in;
    } else {
        fpmr_in = 0;
        fpmr_expected = 0;
        fpmr_out = 0;
    }
}

unsafe fn check_memory_values(config: *mut test_config) -> bool {
    let mut pass = true;

    if !compare_buffer(c"saved V".as_ptr(), ptr::addr_of_mut!(v_out) as *mut c_void, ptr::addr_of_mut!(v_expected) as *mut c_void, size_of_val_raw(ptr::addr_of!(v_out))) { pass = false; }

    let vq = __sve_vq_from_vl(vl_expected(config)) as usize;
    let sme_vq = __sve_vq_from_vl((*config).sme_vl_expected) as usize;

    if svcr_out != svcr_expected {
        ksft_print_msg(c"Mismatch in saved SVCR %lx != %lx\n".as_ptr(), svcr_out, svcr_expected);
        pass = false;
    }
    if sve_vl_out != (*config).sve_vl_expected as u64 {
        ksft_print_msg(c"Mismatch in SVE VL: %ld != %d\n".as_ptr(), sve_vl_out, (*config).sve_vl_expected);
        pass = false;
    }
    if sme_vl_out != (*config).sme_vl_expected as u64 {
        ksft_print_msg(c"Mismatch in SME VL: %ld != %d\n".as_ptr(), sme_vl_out, (*config).sme_vl_expected);
        pass = false;
    }
    if !compare_buffer(c"saved Z".as_ptr(), ptr::addr_of_mut!(z_out) as *mut c_void, ptr::addr_of_mut!(z_expected) as *mut c_void, __SVE_ZREGS_SIZE(vq)) { pass = false; }
    if !compare_buffer(c"saved P".as_ptr(), ptr::addr_of_mut!(p_out) as *mut c_void, ptr::addr_of_mut!(p_expected) as *mut c_void, __SVE_PREGS_SIZE(vq)) { pass = false; }
    if !compare_buffer(c"saved FFR".as_ptr(), ptr::addr_of_mut!(ffr_out) as *mut c_void, ptr::addr_of_mut!(ffr_expected) as *mut c_void, __SVE_PREG_SIZE(vq)) { pass = false; }
    if !compare_buffer(c"saved ZA".as_ptr(), ptr::addr_of_mut!(za_out) as *mut c_void, ptr::addr_of_mut!(za_expected) as *mut c_void, ZA_PT_ZA_SIZE(sme_vq)) { pass = false; }
    if !compare_buffer(c"saved ZT".as_ptr(), ptr::addr_of_mut!(zt_out) as *mut c_void, ptr::addr_of_mut!(zt_expected) as *mut c_void, ZT_SIG_REG_BYTES) { pass = false; }
    if fpmr_out != fpmr_expected {
        ksft_print_msg(c"Mismatch in saved FPMR: %lx != %lx\n".as_ptr(), fpmr_out, fpmr_expected);
        pass = false;
    }

    pass
}

unsafe fn sve_sme_same(config: *mut test_config) -> bool {
    (*config).sve_vl_in == (*config).sve_vl_expected
        && (*config).sme_vl_in == (*config).sme_vl_expected
        && (*config).svcr_in == (*config).svcr_expected
}

unsafe fn sve_write_supported(config: *mut test_config) -> bool {
    if !sve_supported() && !sme_supported() { return false; }
    if ((*config).svcr_in & SVCR_ZA) != ((*config).svcr_expected & SVCR_ZA) { return false; }
    if (*config).svcr_expected & SVCR_SM != 0 {
        if (*config).sve_vl_in != (*config).sve_vl_expected { return false; }
        /* Changing the SME VL disables ZA */
        if ((*config).svcr_expected & SVCR_ZA != 0) && ((*config).sme_vl_in != (*config).sme_vl_expected) { return false; }
    } else {
        if (*config).sme_vl_in != (*config).sme_vl_expected { return false; }
        if !sve_supported() { return false; }
    }
    true
}

unsafe fn sve_write_fpsimd_supported(config: *mut test_config) -> bool {
    if !sve_supported() && !sme_supported() { return false; }
    if ((*config).svcr_in & SVCR_ZA) != ((*config).svcr_expected & SVCR_ZA) { return false; }
    if (*config).svcr_expected & SVCR_SM != 0 { return false; }
    if (*config).sme_vl_in != (*config).sme_vl_expected { return false; }
    true
}

unsafe fn fpsimd_write_expected(_config: *mut test_config) {
    fill_random(ptr::addr_of_mut!(v_expected) as *mut c_void, size_of_val_raw(ptr::addr_of!(v_expected)));
    let vl = vl_expected(_config);
    memset(ptr::addr_of_mut!(z_expected) as *mut c_void, 0, __SVE_ZREGS_SIZE(__sve_vq_from_vl(vl) as usize));
    memset(ptr::addr_of_mut!(p_expected) as *mut c_void, 0, __SVE_PREGS_SIZE(__sve_vq_from_vl(vl) as usize));
    memset(ptr::addr_of_mut!(ffr_expected) as *mut c_void, 0, __SVE_PREG_SIZE(__sve_vq_from_vl(vl) as usize));
    fpsimd_to_sve(ptr::addr_of_mut!(v_expected) as *mut __uint128_t, ptr::addr_of_mut!(z_expected) as *mut c_char, vl);
}

unsafe fn fpsimd_write(child: pid_t, _test_config: *mut test_config) {
    let mut fpsimd: user_fpsimd_state = zeroed();
    let mut iov: iovec = zeroed();
    memset(&mut fpsimd as *mut _ as *mut c_void, 0, size_of::<user_fpsimd_state>());
    memcpy(ptr::addr_of_mut!(fpsimd.vregs) as *mut c_void, ptr::addr_of!(v_expected) as *const c_void, size_of_val_raw(ptr::addr_of!(v_expected)));
    iov.iov_base = &mut fpsimd as *mut _ as *mut c_void;
    iov.iov_len = size_of::<user_fpsimd_state>();
    if ptrace(PTRACE_SETREGSET, child, NT_PRFPREG, &mut iov) == -1 {
        ksft_print_msg(c"FPSIMD set failed: (%s) %d\n".as_ptr(), strerror(errno), errno);
    }
}

unsafe fn fpmr_write_supported(config: *mut test_config) -> bool {
    fpmr_supported() && sve_sme_same(config)
}

unsafe fn fpmr_write_expected(_config: *mut test_config) {
    fill_random(ptr::addr_of_mut!(fpmr_expected) as *mut c_void, size_of::<uint64_t>());
    fpmr_expected &= FPMR_SAFE_BITS;
}

unsafe fn fpmr_write(child: pid_t, _config: *mut test_config) {
    let mut iov: iovec = zeroed();
    iov.iov_len = size_of::<uint64_t>();
    iov.iov_base = ptr::addr_of_mut!(fpmr_expected) as *mut c_void;
    if ptrace(PTRACE_SETREGSET, child, NT_ARM_FPMR, &mut iov) != 0 {
        ksft_print_msg(c"Failed to write FPMR: %s (%d)\n".as_ptr(), strerror(errno), errno);
    }
}

unsafe fn sve_write_expected(config: *mut test_config) {
    let vl = vl_expected(config);
    let sme_vq = __sve_vq_from_vl((*config).sme_vl_expected) as usize;
    if vl == 0 { return; }
    fill_random(ptr::addr_of_mut!(z_expected) as *mut c_void, __SVE_ZREGS_SIZE(__sve_vq_from_vl(vl) as usize));
    fill_random(ptr::addr_of_mut!(p_expected) as *mut c_void, __SVE_PREGS_SIZE(__sve_vq_from_vl(vl) as usize));
    if (svcr_expected & SVCR_SM as u64 != 0) && !fa64_supported() {
        memset(ptr::addr_of_mut!(ffr_expected) as *mut c_void, 0, __SVE_PREG_SIZE(sme_vq));
    } else {
        fill_random_ffr(ptr::addr_of_mut!(ffr_expected) as *mut c_void, __sve_vq_from_vl(vl) as usize);
    }
    /* Share the low bits of Z with V */
    fill_random(ptr::addr_of_mut!(v_expected) as *mut c_void, size_of_val_raw(ptr::addr_of!(v_expected)));
    fpsimd_to_sve(ptr::addr_of_mut!(v_expected) as *mut __uint128_t, ptr::addr_of_mut!(z_expected) as *mut c_char, vl);
    if (*config).sme_vl_in != (*config).sme_vl_expected {
        memset(ptr::addr_of_mut!(za_expected) as *mut c_void, 0, ZA_PT_ZA_SIZE(sme_vq));
        memset(ptr::addr_of_mut!(zt_expected) as *mut c_void, 0, size_of_val_raw(ptr::addr_of!(zt_expected)));
    }
}

unsafe fn sve_write_sve(child: pid_t, config: *mut test_config) {
    let vl = vl_expected(config);
    let vq = __sve_vq_from_vl(vl) as usize;
    if vl == 0 { return; }
    let mut iov = iovec { iov_base: malloc(SVE_PT_SIZE(vq, SVE_PT_REGS_SVE)), iov_len: SVE_PT_SIZE(vq, SVE_PT_REGS_SVE) };
    if iov.iov_base.is_null() {
        ksft_print_msg(c"Failed allocating %lu byte SVE write buffer\n".as_ptr(), iov.iov_len);
        return;
    }
    memset(iov.iov_base, 0, iov.iov_len);
    let sve = iov.iov_base as *mut user_sve_header;
    (*sve).size = iov.iov_len as u32;
    (*sve).flags = SVE_PT_REGS_SVE as u16;
    (*sve).vl = vl as u16;
    memcpy((iov.iov_base as *mut u8).add(SVE_PT_SVE_ZREG_OFFSET(vq, 0)) as *mut c_void, ptr::addr_of!(z_expected) as *const c_void, SVE_PT_SVE_ZREGS_SIZE(vq));
    memcpy((iov.iov_base as *mut u8).add(SVE_PT_SVE_PREG_OFFSET(vq, 0)) as *mut c_void, ptr::addr_of!(p_expected) as *const c_void, SVE_PT_SVE_PREGS_SIZE(vq));
    memcpy((iov.iov_base as *mut u8).add(SVE_PT_SVE_FFR_OFFSET(vq)) as *mut c_void, ptr::addr_of!(ffr_expected) as *const c_void, SVE_PT_SVE_PREG_SIZE(vq));
    let regset = if svcr_expected & SVCR_SM as u64 != 0 { NT_ARM_SSVE } else { NT_ARM_SVE };
    if ptrace(PTRACE_SETREGSET, child, regset, &mut iov) != 0 {
        ksft_print_msg(c"Failed to write SVE: %s (%d)\n".as_ptr(), strerror(errno), errno);
    }
    free(iov.iov_base);
}

unsafe fn sve_write_fpsimd(child: pid_t, config: *mut test_config) {
    let vl = vl_expected(config);
    let vq = __sve_vq_from_vl(vl) as usize;
    let mut iov = iovec { iov_base: malloc(SVE_PT_SIZE(vq, SVE_PT_REGS_FPSIMD)), iov_len: SVE_PT_SIZE(vq, SVE_PT_REGS_FPSIMD) };
    if iov.iov_base.is_null() {
        ksft_print_msg(c"Failed allocating %lu byte SVE write buffer\n".as_ptr(), iov.iov_len);
        return;
    }
    memset(iov.iov_base, 0, iov.iov_len);
    let sve = iov.iov_base as *mut user_sve_header;
    (*sve).size = iov.iov_len as u32;
    (*sve).flags = SVE_PT_REGS_FPSIMD as u16;
    (*sve).vl = vl as u16;
    let fpsimd = (iov.iov_base as *mut u8).add(SVE_PT_REGS_OFFSET) as *mut user_fpsimd_state;
    memcpy(ptr::addr_of_mut!((*fpsimd).vregs) as *mut c_void, ptr::addr_of!(v_expected) as *const c_void, size_of_val_raw(ptr::addr_of!(v_expected)));
    if ptrace(PTRACE_SETREGSET, child, NT_ARM_SVE, &mut iov) != 0 {
        ksft_print_msg(c"Failed to write SVE: %s (%d)\n".as_ptr(), strerror(errno), errno);
    }
    free(iov.iov_base);
}

unsafe fn za_write_supported(config: *mut test_config) -> bool {
    ((*config).svcr_in & SVCR_SM) == ((*config).svcr_expected & SVCR_SM)
}

unsafe fn za_write_expected(config: *mut test_config) {
    let sme_vq = __sve_vq_from_vl((*config).sme_vl_expected) as usize;
    if (*config).svcr_expected & SVCR_ZA != 0 {
        fill_random(ptr::addr_of_mut!(za_expected) as *mut c_void, ZA_PT_ZA_SIZE(sme_vq));
    } else {
        memset(ptr::addr_of_mut!(za_expected) as *mut c_void, 0, ZA_PT_ZA_SIZE(sme_vq));
        memset(ptr::addr_of_mut!(zt_expected) as *mut c_void, 0, size_of_val_raw(ptr::addr_of!(zt_expected)));
    }
    /* Changing the SME VL flushes ZT, SVE state */
    if (*config).sme_vl_in != (*config).sme_vl_expected {
        let sve_vq = __sve_vq_from_vl(vl_expected(config)) as usize;
        memset(ptr::addr_of_mut!(z_expected) as *mut c_void, 0, __SVE_ZREGS_SIZE(sve_vq));
        memset(ptr::addr_of_mut!(p_expected) as *mut c_void, 0, __SVE_PREGS_SIZE(sve_vq));
        memset(ptr::addr_of_mut!(ffr_expected) as *mut c_void, 0, __SVE_PREG_SIZE(sve_vq));
        memset(ptr::addr_of_mut!(zt_expected) as *mut c_void, 0, size_of_val_raw(ptr::addr_of!(zt_expected)));
        fpsimd_to_sve(ptr::addr_of_mut!(v_expected) as *mut __uint128_t, ptr::addr_of_mut!(z_expected) as *mut c_char, vl_expected(config));
    }
}

unsafe fn za_write(child: pid_t, config: *mut test_config) {
    let vq = __sve_vq_from_vl((*config).sme_vl_expected) as usize;
    let len = if (*config).svcr_expected & SVCR_ZA != 0 { ZA_PT_SIZE(vq) } else { size_of::<user_za_header>() };
    let mut iov = iovec { iov_base: malloc(len), iov_len: len };
    if iov.iov_base.is_null() {
        ksft_print_msg(c"Failed allocating %lu byte ZA write buffer\n".as_ptr(), iov.iov_len);
        return;
    }
    memset(iov.iov_base, 0, iov.iov_len);
    let za = iov.iov_base as *mut user_za_header;
    (*za).size = iov.iov_len as u32;
    (*za).vl = (*config).sme_vl_expected as u16;
    if (*config).svcr_expected & SVCR_ZA != 0 {
        memcpy((iov.iov_base as *mut u8).add(ZA_PT_ZA_OFFSET) as *mut c_void, ptr::addr_of!(za_expected) as *const c_void, ZA_PT_ZA_SIZE(vq));
    }
    if ptrace(PTRACE_SETREGSET, child, NT_ARM_ZA, &mut iov) != 0 {
        ksft_print_msg(c"Failed to write ZA: %s (%d)\n".as_ptr(), strerror(errno), errno);
    }
    free(iov.iov_base);
}

unsafe fn zt_write_supported(config: *mut test_config) -> bool {
    if !sme2_supported() { return false; }
    if (*config).sme_vl_in != (*config).sme_vl_expected { return false; }
    if (*config).svcr_expected & SVCR_ZA == 0 { return false; }
    if ((*config).svcr_in & SVCR_SM) != ((*config).svcr_expected & SVCR_SM) { return false; }
    true
}

unsafe fn zt_write_expected(config: *mut test_config) {
    let sme_vq = __sve_vq_from_vl((*config).sme_vl_expected) as usize;
    if (*config).svcr_expected & SVCR_ZA != 0 {
        fill_random(ptr::addr_of_mut!(zt_expected) as *mut c_void, size_of_val_raw(ptr::addr_of!(zt_expected)));
    } else {
        memset(ptr::addr_of_mut!(za_expected) as *mut c_void, 0, ZA_PT_ZA_SIZE(sme_vq));
        memset(ptr::addr_of_mut!(zt_expected) as *mut c_void, 0, size_of_val_raw(ptr::addr_of!(zt_expected)));
    }
}

unsafe fn zt_write(child: pid_t, _config: *mut test_config) {
    let mut iov = iovec { iov_base: ptr::addr_of_mut!(zt_expected) as *mut c_void, iov_len: ZT_SIG_REG_BYTES };
    if ptrace(PTRACE_SETREGSET, child, NT_ARM_ZT, &mut iov) != 0 {
        ksft_print_msg(c"Failed to write ZT: %s (%d)\n".as_ptr(), strerror(errno), errno);
    }
}

/* Actually run a test */
unsafe fn run_test(test: *mut test_definition, config: *mut test_config) {
    let mut name = [0 as c_char; 1024];
    let mut pass: bool;

    if sve_supported() && sme_supported() {
        snprintf(name.as_mut_ptr(), name.len(), c"%s, SVE %d->%d, SME %d/%x->%d/%x".as_ptr(), (*test).name, (*config).sve_vl_in, (*config).sve_vl_expected, (*config).sme_vl_in, (*config).svcr_in, (*config).sme_vl_expected, (*config).svcr_expected);
    } else if sve_supported() {
        snprintf(name.as_mut_ptr(), name.len(), c"%s, SVE %d->%d".as_ptr(), (*test).name, (*config).sve_vl_in, (*config).sve_vl_expected);
    } else if sme_supported() {
        snprintf(name.as_mut_ptr(), name.len(), c"%s, SME %d/%x->%d/%x".as_ptr(), (*test).name, (*config).sme_vl_in, (*config).svcr_in, (*config).sme_vl_expected, (*config).svcr_expected);
    } else {
        snprintf(name.as_mut_ptr(), name.len(), c"%s".as_ptr(), (*test).name);
    }

    if let Some(supported) = (*test).supported {
        if !supported(config) {
            ksft_test_result_skip(c"%s\n".as_ptr(), name.as_ptr());
            return;
        }
    }

    set_initial_values(config);
    if let Some(set_expected_values) = (*test).set_expected_values {
        set_expected_values(config);
    }

    let child = fork();
    if child < 0 {
        ksft_exit_fail_msg(c"fork() failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
    }
    /* run_child() never returns */
    if child == 0 {
        run_child(config);
    }

    pass = run_parent(child, test, config);
    if !check_memory_values(config) {
        pass = false;
    }

    ksft_test_result(pass, c"%s\n".as_ptr(), name.as_ptr());
}

unsafe fn run_tests(defs: *mut test_definition, count: c_int, config: *mut test_config) {
    let mut i = 0;
    while i < count {
        run_test(defs.add(i as usize), config);
        i += 1;
    }
}

static mut base_test_defs: [test_definition; 3] = [
    test_definition { name: c"No writes".as_ptr(), sve_vl_change: false, supported: Some(sve_sme_same), set_expected_values: None, modify_values: None },
    test_definition { name: c"FPSIMD write".as_ptr(), sve_vl_change: false, supported: Some(sve_sme_same), set_expected_values: Some(fpsimd_write_expected), modify_values: Some(fpsimd_write) },
    test_definition { name: c"FPMR write".as_ptr(), sve_vl_change: false, supported: Some(fpmr_write_supported), set_expected_values: Some(fpmr_write_expected), modify_values: Some(fpmr_write) },
];

static mut sve_test_defs: [test_definition; 2] = [
    test_definition { name: c"SVE write".as_ptr(), sve_vl_change: false, supported: Some(sve_write_supported), set_expected_values: Some(sve_write_expected), modify_values: Some(sve_write_sve) },
    test_definition { name: c"SVE write FPSIMD format".as_ptr(), sve_vl_change: false, supported: Some(sve_write_fpsimd_supported), set_expected_values: Some(fpsimd_write_expected), modify_values: Some(sve_write_fpsimd) },
];

static mut za_test_defs: [test_definition; 1] = [
    test_definition { name: c"ZA write".as_ptr(), sve_vl_change: false, supported: Some(za_write_supported), set_expected_values: Some(za_write_expected), modify_values: Some(za_write) },
];

static mut zt_test_defs: [test_definition; 1] = [
    test_definition { name: c"ZT write".as_ptr(), sve_vl_change: false, supported: Some(zt_write_supported), set_expected_values: Some(zt_write_expected), modify_values: Some(zt_write) },
];

static mut sve_vls: [c_int; MAX_NUM_VLS] = [0; MAX_NUM_VLS];
static mut sme_vls: [c_int; MAX_NUM_VLS] = [0; MAX_NUM_VLS];
static mut sve_vl_count: c_int = 0;
static mut sme_vl_count: c_int = 0;

unsafe fn probe_vls(name: *const c_char, vls: *mut c_int, vl_count: *mut c_int, set_vl: c_int) {
    let mut vq: c_uint;
    let mut vl: c_int;

    *vl_count = 0;

    vq = ARCH_VQ_MAX as c_uint;
    while vq > 0 {
        vl = prctl(set_vl, vq * 16);
        if vl == -1 {
            ksft_exit_fail_msg(c"SET_VL failed: %s (%d)\n".as_ptr(), strerror(errno), errno);
        }
        vl &= PR_SVE_VL_LEN_MASK;
        if *vl_count != 0 && vl == *vls.add((*vl_count - 1) as usize) {
            break;
        }
        vq = sve_vq_from_vl(vl);
        *vls.add(*vl_count as usize) = vl;
        *vl_count += 1;
        vq /= 2;
    }

    if *vl_count > 2 {
        /* Just use the minimum and maximum */
        *vls.add(1) = *vls.add((*vl_count - 1) as usize);
        ksft_print_msg(c"%d %s VLs, using %d and %d\n".as_ptr(), *vl_count, name, *vls.add(0), *vls.add(1));
        *vl_count = 2;
    } else {
        ksft_print_msg(c"%d %s VLs\n".as_ptr(), *vl_count, name);
    }
}

#[repr(C)]
#[derive(Copy, Clone)]
struct svcr_combination {
    svcr_in: c_int,
    svcr_expected: c_int,
}

static mut svcr_combinations: [svcr_combination; 15] = [
    svcr_combination { svcr_in: 0, svcr_expected: 0 },
    svcr_combination { svcr_in: 0, svcr_expected: SVCR_SM },
    svcr_combination { svcr_in: 0, svcr_expected: SVCR_ZA },
    /* Can't enable both SM and ZA with a single ptrace write */
    svcr_combination { svcr_in: SVCR_SM, svcr_expected: 0 },
    svcr_combination { svcr_in: SVCR_SM, svcr_expected: SVCR_SM },
    svcr_combination { svcr_in: SVCR_SM, svcr_expected: SVCR_ZA },
    svcr_combination { svcr_in: SVCR_SM, svcr_expected: SVCR_SM | SVCR_ZA },
    svcr_combination { svcr_in: SVCR_ZA, svcr_expected: 0 },
    svcr_combination { svcr_in: SVCR_ZA, svcr_expected: SVCR_SM },
    svcr_combination { svcr_in: SVCR_ZA, svcr_expected: SVCR_ZA },
    svcr_combination { svcr_in: SVCR_ZA, svcr_expected: SVCR_SM | SVCR_ZA },
    svcr_combination { svcr_in: SVCR_SM | SVCR_ZA, svcr_expected: 0 },
    svcr_combination { svcr_in: SVCR_SM | SVCR_ZA, svcr_expected: SVCR_SM },
    svcr_combination { svcr_in: SVCR_SM | SVCR_ZA, svcr_expected: SVCR_ZA },
    svcr_combination { svcr_in: SVCR_SM | SVCR_ZA, svcr_expected: SVCR_SM | SVCR_ZA },
];

unsafe fn run_sve_tests() {
    let mut test_config: test_config = zeroed();
    let mut i = 0;

    if !sve_supported() {
        return;
    }

    test_config.sme_vl_in = sme_vls[0];
    test_config.sme_vl_expected = sme_vls[0];
    test_config.svcr_in = 0;
    test_config.svcr_expected = 0;

    while i < sve_vl_count {
        test_config.sve_vl_in = sve_vls[i as usize];
        let mut j = 0;
        while j < sve_vl_count {
            test_config.sve_vl_expected = sve_vls[j as usize];
            run_tests(ptr::addr_of_mut!(base_test_defs) as *mut test_definition, ARRAY_SIZE(&base_test_defs), &mut test_config);
            if sve_supported() {
                run_tests(ptr::addr_of_mut!(sve_test_defs) as *mut test_definition, ARRAY_SIZE(&sve_test_defs), &mut test_config);
            }
            j += 1;
        }
        i += 1;
    }
}

unsafe fn run_sme_tests() {
    let mut test_config: test_config = zeroed();
    let mut i = 0;

    if !sme_supported() {
        return;
    }

    test_config.sve_vl_in = sve_vls[0];
    test_config.sve_vl_expected = sve_vls[0];

    /*
     * Every SME VL/SVCR combination
     */
    while i < sme_vl_count {
        test_config.sme_vl_in = sme_vls[i as usize];
        let mut j = 0;
        while j < sme_vl_count {
            test_config.sme_vl_expected = sme_vls[j as usize];
            let mut k = 0;
            while k < ARRAY_SIZE(&svcr_combinations) {
                test_config.svcr_in = svcr_combinations[k as usize].svcr_in;
                test_config.svcr_expected = svcr_combinations[k as usize].svcr_expected;
                run_tests(ptr::addr_of_mut!(base_test_defs) as *mut test_definition, ARRAY_SIZE(&base_test_defs), &mut test_config);
                run_tests(ptr::addr_of_mut!(sve_test_defs) as *mut test_definition, ARRAY_SIZE(&sve_test_defs), &mut test_config);
                run_tests(ptr::addr_of_mut!(za_test_defs) as *mut test_definition, ARRAY_SIZE(&za_test_defs), &mut test_config);
                if sme2_supported() {
                    run_tests(ptr::addr_of_mut!(zt_test_defs) as *mut test_definition, ARRAY_SIZE(&zt_test_defs), &mut test_config);
                }
                k += 1;
            }
            j += 1;
        }
        i += 1;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn main() -> c_int {
    let mut test_config: test_config = zeroed();
    let mut sa: sigaction = zeroed();
    let mut tests: c_int;
    let mut tmp: c_int;

    srandom(getpid() as c_uint);

    ksft_print_header();

    if sve_supported() {
        probe_vls(c"SVE".as_ptr(), ptr::addr_of_mut!(sve_vls) as *mut c_int, ptr::addr_of_mut!(sve_vl_count), PR_SVE_SET_VL);
        tests = ARRAY_SIZE(&base_test_defs) + ARRAY_SIZE(&sve_test_defs);
        tests *= sve_vl_count * sve_vl_count;
    } else {
        /* Only run the FPSIMD tests */
        sve_vl_count = 1;
        tests = ARRAY_SIZE(&base_test_defs);
    }

    if sme_supported() {
        probe_vls(c"SME".as_ptr(), ptr::addr_of_mut!(sme_vls) as *mut c_int, ptr::addr_of_mut!(sme_vl_count), PR_SME_SET_VL);
        tmp = ARRAY_SIZE(&base_test_defs) + ARRAY_SIZE(&sve_test_defs) + ARRAY_SIZE(&za_test_defs);
        if sme2_supported() {
            tmp += ARRAY_SIZE(&zt_test_defs);
        }
        tmp *= sme_vl_count * sme_vl_count;
        tmp *= ARRAY_SIZE(&svcr_combinations);
        tests += tmp;
    } else {
        sme_vl_count = 1;
    }

    if sme2_supported() {
        ksft_print_msg(c"SME2 supported\n".as_ptr());
    }
    if fa64_supported() {
        ksft_print_msg(c"FA64 supported\n".as_ptr());
    }
    if fpmr_supported() {
        ksft_print_msg(c"FPMR supported\n".as_ptr());
    }

    ksft_set_plan(tests);

    /* Get signal handers ready before we start any children */
    memset(&mut sa as *mut _ as *mut c_void, 0, size_of::<sigaction>());
    sa.sa_sigaction = handle_alarm;
    sa.sa_flags = SA_RESTART | SA_SIGINFO;
    sigemptyset(&mut sa.sa_mask);
    let ret = sigaction(SIGALRM, &sa, ptr::null_mut());
    if ret < 0 {
        ksft_print_msg(c"Failed to install SIGALRM handler: %s (%d)\n".as_ptr(), strerror(errno), errno);
    }

    /*
     * Run the test set if there is no SVE or SME, with those we
     * have to pick a VL for each run.
     */
    if !sve_supported() && !sme_supported() {
        test_config.sve_vl_in = 0;
        test_config.sve_vl_expected = 0;
        test_config.sme_vl_in = 0;
        test_config.sme_vl_expected = 0;
        test_config.svcr_in = 0;
        test_config.svcr_expected = 0;

        run_tests(ptr::addr_of_mut!(base_test_defs) as *mut test_definition, ARRAY_SIZE(&base_test_defs), &mut test_config);
    }

    run_sve_tests();
    run_sme_tests();

    ksft_finished();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
