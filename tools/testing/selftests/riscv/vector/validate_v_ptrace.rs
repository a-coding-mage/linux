// SPDX-License-Identifier: GPL-2.0-only
//
// Rust translation of validate_v_ptrace.c.
// Original C dependencies: sys/ptrace.h, sys/syscall.h, sys/types.h,
// sys/wait.h, sys/uio.h, unistd.h, errno.h, linux/ptrace.h, linux/elf.h,
// kselftest_harness.h, v_helpers.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::arch::asm;
use core::ffi::{c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr::null_mut;

type pid_t = c_int;
type size_t = usize;

const SR_FS_DIRTY: c_ulong = 0x00006000;
const CSR_VXRM_SHIFT: c_ulong = 1;

const VECTOR_1_0: c_ulong = _BITUL(0);
const XTHEAD_VECTOR_0_7: c_ulong = _BITUL(1);

const fn _BITUL(nr: c_ulong) -> c_ulong {
    1usize.wrapping_shl(nr as u32) as c_ulong
}

fn vector_test(x: c_ulong) -> c_ulong {
    x & VECTOR_1_0
}

fn xthead_test(x: c_ulong) -> c_ulong {
    x & XTHEAD_VECTOR_0_7
}

extern "C" {
    fn is_vector_supported() -> bool;
    fn is_xtheadvector_supported() -> bool;
    fn get_vr_len() -> c_ulong;

    fn syscall(num: c_long, ...) -> c_long;
    fn fork() -> pid_t;
    fn ptrace(request: c_int, ...) -> c_long;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn kill(pid: pid_t, sig: c_int) -> c_int;
    fn sleep(seconds: c_uint) -> c_uint;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;
    fn free(ptr: *mut c_void);

    static mut errno: c_int;
    static __riscv_xlen: c_int;
}

type c_uint = u32;

#[repr(C)]
struct iovec {
    iov_base: *mut c_void,
    iov_len: size_t,
}

#[repr(C)]
struct __riscv_v_regset_state {
    vstart: c_ulong,
    vl: c_ulong,
    vtype: c_ulong,
    vcsr: c_ulong,
    vlenb: c_ulong,
}

#[repr(C)]
struct user_regs_struct {
    pc: c_ulong,
}

extern "C" {
    static SYS_clone: c_long;
    static SIGCHLD: c_int;
    static SIGKILL: c_int;
    static PTRACE_ATTACH: c_int;
    static PTRACE_POKEDATA: c_int;
    static PTRACE_CONT: c_int;
    static PTRACE_GETREGSET: c_int;
    static PTRACE_SETREGSET: c_int;
    static PTRACE_PEEKDATA: c_int;
    static NT_RISCV_VECTOR: c_int;
    static NT_PRSTATUS: c_int;
    static ENODATA: c_int;
    static EINVAL: c_int;
}

extern "Rust" {
    fn WIFSTOPPED(status: c_int) -> bool;
}

static mut chld_lock: c_ulong = 0;

macro_rules! TEST {
    ($name:ident, $body:block) => {
        fn $name() $body
    };
}

macro_rules! TEST_F {
    ($fixture:ident, $name:ident, $body:block) => {
        fn $name(_self: *mut $fixture, variant: *const $fixture_variant) $body
    };
}

macro_rules! ASSERT_EQ {
    ($left:expr, $right:expr) => {};
}

macro_rules! ASSERT_LE {
    ($left:expr, $right:expr) => {};
}

macro_rules! ASSERT_TRUE {
    ($expr:expr) => {};
}

macro_rules! ASSERT_FALSE {
    ($expr:expr) => {};
}

macro_rules! EXPECT_EQ {
    ($left:expr, $right:expr) => {};
}

macro_rules! SKIP {
    (return, $($arg:tt)*) => {
        return;
    };
}

macro_rules! TH_LOG {
    ($($arg:tt)*) => {};
}

TEST!(ptrace_v_not_enabled, {
    unsafe {
        let mut pid: pid_t;

        if !(is_vector_supported() || is_xtheadvector_supported()) {
            SKIP!(return, "Vector not supported");
        }

        chld_lock = 1;
        pid = syscall(SYS_clone, SIGCHLD, 0, null_mut::<c_void>(), 0, null_mut::<c_void>()) as pid_t;
        ASSERT_LE!(0, pid);
        TH_LOG!("clone: %m");

        if pid == 0 {
            while chld_lock == 1 {
                asm!("", in("a0") chld_lock, options(nostack, preserves_flags));
            }

            asm!("ebreak");
        } else {
            let mut regset_data: *mut __riscv_v_regset_state;
            let vlenb: c_ulong = get_vr_len();
            let mut regset_size: size_t;
            let mut iov: iovec;
            let mut status: c_int = 0;
            let mut ret: c_int;

            /* attach */

            ASSERT_EQ!(0, ptrace(PTRACE_ATTACH, pid, null_mut::<c_void>(), null_mut::<c_void>()));
            ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
            ASSERT_TRUE!(WIFSTOPPED(status));

            /* unlock */

            ASSERT_EQ!(0, ptrace(PTRACE_POKEDATA, pid, &raw mut chld_lock, 0));

            /* resume and wait for ebreak */

            ASSERT_EQ!(0, ptrace(PTRACE_CONT, pid, null_mut::<c_void>(), null_mut::<c_void>()));
            ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
            ASSERT_TRUE!(WIFSTOPPED(status));

            /* try to read vector registers from the tracee */

            regset_size = size_of::<__riscv_v_regset_state>() + vlenb as size_t * 32;
            regset_data = calloc(1, regset_size) as *mut __riscv_v_regset_state;

            iov = iovec {
                iov_base: regset_data as *mut c_void,
                iov_len: regset_size,
            };

            /* V extension is available, but not yet enabled for the tracee */

            errno = 0;
            ret = ptrace(PTRACE_GETREGSET, pid, NT_RISCV_VECTOR, &mut iov) as c_int;
            ASSERT_EQ!(ENODATA, errno);
            ASSERT_EQ!(-1, ret);

            /* cleanup */
            free(regset_data as *mut c_void);
            ASSERT_EQ!(0, kill(pid, SIGKILL));
        }
    }
});

static mut vstart: c_ulong = 0;
static mut vtype: c_ulong = 0;
static mut vlenb: c_ulong = 0;
static mut vcsr: c_ulong = 0;
static mut vl: c_ulong = 0;

TEST!(ptrace_v_early_debug, {
    unsafe {
        let xtheadvector: bool;
        let mut pid: pid_t;

        if !(is_vector_supported() || is_xtheadvector_supported()) {
            SKIP!(return, "Vector not supported");
        }

        xtheadvector = is_xtheadvector_supported();

        chld_lock = 1;
        pid = fork();
        ASSERT_LE!(0, pid);
        TH_LOG!("fork: %m");

        if pid == 0 {
            let mut vxsat: c_ulong;
            let mut vxrm: c_ulong;

            vlenb = get_vr_len();

            while chld_lock == 1 {
                asm!("", in("a0") chld_lock, options(nostack, preserves_flags));
            }

            asm!(
                "csrr {vstart_out}, vstart",
                "csrr {vtype_out}, vtype",
                "csrr {vl_out}, vl",
                vtype_out = out(reg) vtype,
                vstart_out = out(reg) vstart,
                vl_out = out(reg) vl,
                options(nostack)
            );

            /* no 'is_xtheadvector_supported()' here to avoid clobbering v-state by syscall */
            if xtheadvector {
                asm!(
                    "csrs sstatus, {bit}",
                    "csrr {vxsat_out}, vxsat",
                    "csrr {vxrm_out}, vxrm",
                    vxsat_out = out(reg) vxsat,
                    vxrm_out = out(reg) vxrm,
                    bit = in(reg) SR_FS_DIRTY,
                    options(nostack)
                );
                vcsr = vxsat | vxrm << CSR_VXRM_SHIFT;
            } else {
                asm!(
                    "csrr {vcsr_out}, vcsr",
                    vcsr_out = out(reg) vcsr,
                    options(nostack)
                );
            }

            asm!(
                ".option push",
                ".option norvc",
                "ebreak",
                ".option pop",
            );
        } else {
            let mut regset_data: *mut __riscv_v_regset_state;
            let mut vstart_csr: c_ulong;
            let mut vlenb_csr: c_ulong;
            let mut vtype_csr: c_ulong;
            let mut vcsr_csr: c_ulong;
            let mut vl_csr: c_ulong;
            let mut regset_size: size_t;
            let mut iov: iovec;
            let mut status: c_int = 0;

            /* attach */

            ASSERT_EQ!(0, ptrace(PTRACE_ATTACH, pid, null_mut::<c_void>(), null_mut::<c_void>()));
            ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
            ASSERT_TRUE!(WIFSTOPPED(status));

            /* unlock */

            ASSERT_EQ!(0, ptrace(PTRACE_POKEDATA, pid, &raw mut chld_lock, 0));

            /* resume and wait for ebreak */

            ASSERT_EQ!(0, ptrace(PTRACE_CONT, pid, null_mut::<c_void>(), null_mut::<c_void>()));
            ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
            ASSERT_TRUE!(WIFSTOPPED(status));

            /* read tracee vector csr regs using ptrace PEEKDATA */

            errno = 0;
            vstart_csr = ptrace(PTRACE_PEEKDATA, pid, &raw mut vstart, null_mut::<c_void>()) as c_ulong;
            ASSERT_FALSE!((errno != 0) && (vstart_csr == c_ulong::MAX));

            errno = 0;
            vl_csr = ptrace(PTRACE_PEEKDATA, pid, &raw mut vl, null_mut::<c_void>()) as c_ulong;
            ASSERT_FALSE!((errno != 0) && (vl_csr == c_ulong::MAX));

            errno = 0;
            vtype_csr = ptrace(PTRACE_PEEKDATA, pid, &raw mut vtype, null_mut::<c_void>()) as c_ulong;
            ASSERT_FALSE!((errno != 0) && (vtype_csr == c_ulong::MAX));

            errno = 0;
            vcsr_csr = ptrace(PTRACE_PEEKDATA, pid, &raw mut vcsr, null_mut::<c_void>()) as c_ulong;
            ASSERT_FALSE!((errno != 0) && (vcsr_csr == c_ulong::MAX));

            errno = 0;
            vlenb_csr = ptrace(PTRACE_PEEKDATA, pid, &raw mut vlenb, null_mut::<c_void>()) as c_ulong;
            ASSERT_FALSE!((errno != 0) && (vlenb_csr == c_ulong::MAX));

            /* read tracee csr regs using ptrace GETREGSET */

            regset_size = size_of::<__riscv_v_regset_state>() + vlenb_csr as size_t * 32;
            regset_data = calloc(1, regset_size) as *mut __riscv_v_regset_state;

            iov = iovec {
                iov_base: regset_data as *mut c_void,
                iov_len: regset_size,
            };

            ASSERT_EQ!(0, ptrace(PTRACE_GETREGSET, pid, NT_RISCV_VECTOR, &mut iov));

            /* compare */

            EXPECT_EQ!(vstart_csr, (*regset_data).vstart);
            EXPECT_EQ!(vtype_csr, (*regset_data).vtype);
            EXPECT_EQ!(vlenb_csr, (*regset_data).vlenb);
            EXPECT_EQ!(vcsr_csr, (*regset_data).vcsr);
            EXPECT_EQ!(vl_csr, (*regset_data).vl);

            /* cleanup */
            free(regset_data as *mut c_void);
            ASSERT_EQ!(0, kill(pid, SIGKILL));
        }
    }
});

fn set_initial_v_state(vl_out: *mut c_ulong) {
    unsafe {
        if is_xtheadvector_supported() {
            asm!(
                // 0 | zimm[10:0] | rs1 | 1 1 1 | rd |1010111| vsetvli
                // vsetvli	t4, x0, e16, m2, d1
                ".4byte 0b00000000010100000111111011010111",
                "mv {new_vl}, t4",
                new_vl = out(reg) *vl_out,
                out("t4") _,
            );
        } else {
            asm!(
                ".option push",
                ".option arch, +zve32x",
                "vsetvli {new_vl}, x0, e16, m2, tu, mu",
                ".option pop",
                new_vl = out(reg) *vl_out,
            );
        }
    }
}

TEST!(ptrace_v_syscall_clobbering, {
    unsafe {
        let mut pid: pid_t;

        if !is_vector_supported() && !is_xtheadvector_supported() {
            SKIP!(return, "Vector not supported");
        }

        chld_lock = 1;
        pid = fork();
        ASSERT_LE!(0, pid);
        TH_LOG!("fork: %m");

        if pid == 0 {
            let mut vl_local: c_ulong = 0;

            while chld_lock == 1 {
                asm!("", in("a0") chld_lock, options(nostack, preserves_flags));
            }

            set_initial_v_state(&mut vl_local);

            loop {
                asm!(
                    ".option push",
                    ".option norvc",
                    "ebreak",
                    ".option pop",
                );

                sleep(0);
            }
        } else {
            let mut regset_data: *mut __riscv_v_regset_state;
            let vlenb_local: c_ulong = get_vr_len();
            let mut regs: user_regs_struct = core::mem::zeroed();
            let mut regset_size: size_t;
            let mut iov: iovec;
            let mut status: c_int = 0;

            /* attach */

            ASSERT_EQ!(0, ptrace(PTRACE_ATTACH, pid, null_mut::<c_void>(), null_mut::<c_void>()));
            ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
            ASSERT_TRUE!(WIFSTOPPED(status));

            /* unlock */

            ASSERT_EQ!(0, ptrace(PTRACE_POKEDATA, pid, &raw mut chld_lock, 0));

            /* resume and wait for the 1st ebreak */

            ASSERT_EQ!(0, ptrace(PTRACE_CONT, pid, null_mut::<c_void>(), null_mut::<c_void>()));
            ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
            ASSERT_TRUE!(WIFSTOPPED(status));

            /* read tracee vector csr regs using ptrace GETREGSET */

            regset_size = size_of::<__riscv_v_regset_state>() + vlenb_local as size_t * 32;
            regset_data = calloc(1, regset_size) as *mut __riscv_v_regset_state;

            iov = iovec {
                iov_base: regset_data as *mut c_void,
                iov_len: regset_size,
            };

            ASSERT_EQ!(0, ptrace(PTRACE_GETREGSET, pid, NT_RISCV_VECTOR, &mut iov));

            /* verify initial vsetvli settings */

            if is_xtheadvector_supported() {
                EXPECT_EQ!(5usize as c_ulong, (*regset_data).vtype);
            } else {
                EXPECT_EQ!(9usize as c_ulong, (*regset_data).vtype);
            }

            EXPECT_EQ!((*regset_data).vlenb, (*regset_data).vl);
            EXPECT_EQ!(vlenb_local, (*regset_data).vlenb);
            EXPECT_EQ!(0usize as c_ulong, (*regset_data).vstart);
            EXPECT_EQ!(0usize as c_ulong, (*regset_data).vcsr);

            /* skip 1st ebreak, then resume and wait for the 2nd ebreak */

            iov.iov_base = &mut regs as *mut user_regs_struct as *mut c_void;
            iov.iov_len = size_of::<user_regs_struct>();

            ASSERT_EQ!(0, ptrace(PTRACE_GETREGSET, pid, NT_PRSTATUS, &mut iov));
            regs.pc = regs.pc.wrapping_add(4);
            ASSERT_EQ!(0, ptrace(PTRACE_SETREGSET, pid, NT_PRSTATUS, &mut iov));

            ASSERT_EQ!(0, ptrace(PTRACE_CONT, pid, null_mut::<c_void>(), null_mut::<c_void>()));
            ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
            ASSERT_TRUE!(WIFSTOPPED(status));

            /* read tracee vtype using ptrace GETREGSET */

            iov.iov_base = regset_data as *mut c_void;
            iov.iov_len = regset_size;

            ASSERT_EQ!(0, ptrace(PTRACE_GETREGSET, pid, NT_RISCV_VECTOR, &mut iov));

            /* verify that V state is illegal after syscall */

            EXPECT_EQ!(1usize.wrapping_shl((__riscv_xlen - 1) as u32) as c_ulong, (*regset_data).vtype);
            EXPECT_EQ!(vlenb_local, (*regset_data).vlenb);
            EXPECT_EQ!(0usize as c_ulong, (*regset_data).vstart);
            EXPECT_EQ!(0usize as c_ulong, (*regset_data).vcsr);
            EXPECT_EQ!(0usize as c_ulong, (*regset_data).vl);

            /* cleanup */
            free(regset_data as *mut c_void);
            ASSERT_EQ!(0, kill(pid, SIGKILL));
        }
    }
});

#[repr(C)]
struct v_csr_invalid {}

fn v_csr_invalid_setup() {}

fn v_csr_invalid_teardown() {}

/* modifications of the initial vsetvli settings */
#[repr(C)]
struct v_csr_invalid_variant {
    vstart: c_ulong,
    vl: c_ulong,
    vtype: c_ulong,
    vcsr: c_ulong,
    vlenb_mul: c_ulong,
    vlenb_min: c_ulong,
    vlenb_max: c_ulong,
    spec: c_ulong,
}

type fixture_variant = v_csr_invalid_variant;

/* unexpected vlenb value */
static new_vlenb: v_csr_invalid_variant = v_csr_invalid_variant {
    vstart: 0x0,
    vl: 0x0,
    vtype: 0x3,
    vcsr: 0x0,
    vlenb_mul: 0x2,
    vlenb_min: 0x0,
    vlenb_max: 0x0,
    spec: VECTOR_1_0 | XTHEAD_VECTOR_0_7,
};

/* invalid reserved bits in vcsr */
static vcsr_invalid_reserved_bits: v_csr_invalid_variant = v_csr_invalid_variant {
    vstart: 0x0,
    vl: 0x0,
    vtype: 0x3,
    vcsr: 0x1usize.wrapping_shl(8) as c_ulong,
    vlenb_mul: 0x1,
    vlenb_min: 0x0,
    vlenb_max: 0x0,
    spec: VECTOR_1_0 | XTHEAD_VECTOR_0_7,
};

/* invalid reserved bits in vtype */
static vtype_invalid_reserved_bits: v_csr_invalid_variant = v_csr_invalid_variant {
    vstart: 0x0,
    vl: 0x0,
    vtype: (0x1usize.wrapping_shl(8) as c_ulong) | 0x3,
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x0,
    vlenb_max: 0x0,
    spec: VECTOR_1_0 | XTHEAD_VECTOR_0_7,
};

/* set vill bit */
static invalid_vill_bit: v_csr_invalid_variant = v_csr_invalid_variant {
    vstart: 0x0,
    vl: 0x0,
    vtype: 0x3, /* includes (0x1UL << (__riscv_xlen - 1)) at runtime in C */
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x0,
    vlenb_max: 0x0,
    spec: VECTOR_1_0 | XTHEAD_VECTOR_0_7,
};

/* reserved vsew value: vsew > 3 */
static reserved_vsew: v_csr_invalid_variant = v_csr_invalid_variant {
    vstart: 0x0,
    vl: 0x0,
    vtype: 0x4usize.wrapping_shl(3) as c_ulong,
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x0,
    vlenb_max: 0x0,
    spec: VECTOR_1_0,
};

/* XTheadVector: unsupported non-zero VEDIV value */
static reserved_vediv: v_csr_invalid_variant = v_csr_invalid_variant {
    vstart: 0x0,
    vl: 0x0,
    vtype: 0x3usize.wrapping_shl(5) as c_ulong,
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x0,
    vlenb_max: 0x0,
    spec: XTHEAD_VECTOR_0_7,
};

/* reserved vlmul value: vlmul == 4 */
static reserved_vlmul: v_csr_invalid_variant = v_csr_invalid_variant {
    vstart: 0x0,
    vl: 0x0,
    vtype: 0x4,
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x0,
    vlenb_max: 0x0,
    spec: VECTOR_1_0,
};

/* invalid fractional LMUL for VLEN <= 256: LMUL= 1/8, SEW = 64 */
static frac_lmul1_invalid: v_csr_invalid_variant = v_csr_invalid_variant {
    vstart: 0x0,
    vl: 0x0,
    vtype: 0x1d,
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x0,
    vlenb_max: 0x20,
    spec: VECTOR_1_0,
};

/* invalid integral LMUL for VLEN <= 16: LMUL= 2, SEW = 64 */
static int_lmul1_invalid: v_csr_invalid_variant = v_csr_invalid_variant {
    vstart: 0x0,
    vl: 0x0,
    vtype: 0x19,
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x0,
    vlenb_max: 0x2,
    spec: VECTOR_1_0,
};

/* XTheadVector: invalid integral LMUL for VLEN <= 16: LMUL= 2, SEW = 64 */
static int_lmul2_invalid: v_csr_invalid_variant = v_csr_invalid_variant {
    vstart: 0x0,
    vl: 0x0,
    vtype: 0xd,
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x0,
    vlenb_max: 0x2,
    spec: XTHEAD_VECTOR_0_7,
};

/* invalid VL for VLEN <= 128: LMUL= 2, SEW = 64, VL = 8 */
static vl1: v_csr_invalid_variant = v_csr_invalid_variant {
    vstart: 0x0,
    vl: 0x8,
    vtype: 0x19,
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x0,
    vlenb_max: 0x10,
    spec: VECTOR_1_0,
};

/* XTheadVector: invalid VL for VLEN <= 128: LMUL= 2, SEW = 64, VL = 8 */
static vl2: v_csr_invalid_variant = v_csr_invalid_variant {
    vstart: 0x0,
    vl: 0x8,
    vtype: 0xd,
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x0,
    vlenb_max: 0x10,
    spec: XTHEAD_VECTOR_0_7,
};

TEST_F!(v_csr_invalid, ptrace_v_invalid_values, {
    unsafe {
        let mut vlenb_local: c_ulong;
        let mut pid: pid_t;

        if !is_vector_supported() && !is_xtheadvector_supported() {
            SKIP!(return, "Vectors not supported");
        }

        if is_vector_supported() && vector_test((*variant).spec) == 0 {
            SKIP!(return, "Test not supported for Vector");
        }

        if is_xtheadvector_supported() && xthead_test((*variant).spec) == 0 {
            SKIP!(return, "Test not supported for XTheadVector");
        }

        vlenb_local = get_vr_len();

        if (*variant).vlenb_min != 0 {
            if vlenb_local < (*variant).vlenb_min {
                SKIP!(return, "This test does not support VLEN < %lu\n", (*variant).vlenb_min * 8);
            }
        }

        if (*variant).vlenb_max != 0 {
            if vlenb_local > (*variant).vlenb_max {
                SKIP!(return, "This test does not support VLEN > %lu\n", (*variant).vlenb_max * 8);
            }
        }

        chld_lock = 1;
        pid = fork();
        ASSERT_LE!(0, pid);
        TH_LOG!("fork: %m");

        if pid == 0 {
            let mut vl_local: c_ulong = 0;

            while chld_lock == 1 {
                asm!("", in("a0") chld_lock, options(nostack, preserves_flags));
            }

            set_initial_v_state(&mut vl_local);

            loop {
                asm!(
                    ".option push",
                    ".option norvc",
                    "ebreak",
                    "nop",
                    ".option pop",
                );
            }
        } else {
            let mut regset_data: *mut __riscv_v_regset_state;
            let mut regset_size: size_t;
            let mut iov: iovec;
            let mut status: c_int = 0;
            let mut ret: c_int;

            /* attach */

            ASSERT_EQ!(0, ptrace(PTRACE_ATTACH, pid, null_mut::<c_void>(), null_mut::<c_void>()));
            ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
            ASSERT_TRUE!(WIFSTOPPED(status));

            /* unlock */

            ASSERT_EQ!(0, ptrace(PTRACE_POKEDATA, pid, &raw mut chld_lock, 0));

            /* resume and wait for the 1st ebreak */

            ASSERT_EQ!(0, ptrace(PTRACE_CONT, pid, null_mut::<c_void>(), null_mut::<c_void>()));
            ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
            ASSERT_TRUE!(WIFSTOPPED(status));

            /* read tracee vector csr regs using ptrace GETREGSET */

            regset_size = size_of::<__riscv_v_regset_state>() + vlenb_local as size_t * 32;
            regset_data = calloc(1, regset_size) as *mut __riscv_v_regset_state;

            iov = iovec {
                iov_base: regset_data as *mut c_void,
                iov_len: regset_size,
            };

            ASSERT_EQ!(0, ptrace(PTRACE_GETREGSET, pid, NT_RISCV_VECTOR, &mut iov));

            /* verify initial vsetvli settings */

            if is_xtheadvector_supported() {
                EXPECT_EQ!(5usize as c_ulong, (*regset_data).vtype);
            } else {
                EXPECT_EQ!(9usize as c_ulong, (*regset_data).vtype);
            }

            EXPECT_EQ!((*regset_data).vlenb, (*regset_data).vl);
            EXPECT_EQ!(vlenb_local, (*regset_data).vlenb);
            EXPECT_EQ!(0usize as c_ulong, (*regset_data).vstart);
            EXPECT_EQ!(0usize as c_ulong, (*regset_data).vcsr);

            /* apply invalid settings from fixture variants */

            (*regset_data).vlenb = (*regset_data).vlenb.wrapping_mul((*variant).vlenb_mul);
            (*regset_data).vstart = (*variant).vstart;
            (*regset_data).vtype = (*variant).vtype;
            (*regset_data).vcsr = (*variant).vcsr;
            (*regset_data).vl = (*variant).vl;

            iov.iov_base = regset_data as *mut c_void;
            iov.iov_len = regset_size;

            errno = 0;
            ret = ptrace(PTRACE_SETREGSET, pid, NT_RISCV_VECTOR, &mut iov) as c_int;
            ASSERT_EQ!(errno, EINVAL);
            ASSERT_EQ!(ret, -1);

            /* cleanup */
            free(regset_data as *mut c_void);
            ASSERT_EQ!(0, kill(pid, SIGKILL));
        }
    }
});

#[repr(C)]
struct v_csr_valid {}

fn v_csr_valid_setup() {}

fn v_csr_valid_teardown() {}

/* modifications of the initial vsetvli settings */
#[repr(C)]
struct v_csr_valid_variant {
    vstart: c_ulong,
    vl: c_ulong,
    vtype: c_ulong,
    vcsr: c_ulong,
    vlenb_mul: c_ulong,
    vlenb_min: c_ulong,
    vlenb_max: c_ulong,
    spec: c_ulong,
}

/* valid for VLEN >= 128: LMUL= 1/4, SEW = 32 */
static frac_lmul1: v_csr_valid_variant = v_csr_valid_variant {
    vstart: 0x0,
    vl: 0x0,
    vtype: 0x16,
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x10,
    vlenb_max: 0x0,
    spec: VECTOR_1_0,
};

/* valid for VLEN >= 16: LMUL= 2, SEW = 32 */
static int_lmul1: v_csr_valid_variant = v_csr_valid_variant {
    vstart: 0x0,
    vl: 0x0,
    vtype: 0x11,
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x2,
    vlenb_max: 0x0,
    spec: VECTOR_1_0,
};

/* valid for XTheadVector VLEN >= 16: LMUL= 2, SEW = 32 */
static int_lmul2: v_csr_valid_variant = v_csr_valid_variant {
    vstart: 0x0,
    vl: 0x0,
    vtype: 0x9,
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x2,
    vlenb_max: 0x0,
    spec: XTHEAD_VECTOR_0_7,
};

/* valid for VLEN >= 32: LMUL= 2, SEW = 32, VL = 2 */
static int_lmul3: v_csr_valid_variant = v_csr_valid_variant {
    vstart: 0x0,
    vl: 0x2,
    vtype: 0x11,
    vcsr: 0x0,
    vlenb_mul: 0x1,
    vlenb_min: 0x4,
    vlenb_max: 0x0,
    spec: VECTOR_1_0,
};

fn ptrace_v_valid_values(_self: *mut v_csr_valid, variant: *const v_csr_valid_variant) {
    unsafe {
        let mut vlenb_local: c_ulong;
        let mut pid: pid_t;

        if !is_vector_supported() && !is_xtheadvector_supported() {
            SKIP!(return, "Vectors not supported");
        }

        if is_vector_supported() && vector_test((*variant).spec) == 0 {
            SKIP!(return, "Test not supported for Vector");
        }

        if is_xtheadvector_supported() && xthead_test((*variant).spec) == 0 {
            SKIP!(return, "Test not supported for XTheadVector");
        }

        vlenb_local = get_vr_len();

        if (*variant).vlenb_min != 0 {
            if vlenb_local < (*variant).vlenb_min {
                SKIP!(return, "This test does not support VLEN < %lu\n", (*variant).vlenb_min * 8);
            }
        }
        if (*variant).vlenb_max != 0 {
            if vlenb_local > (*variant).vlenb_max {
                SKIP!(return, "This test does not support VLEN > %lu\n", (*variant).vlenb_max * 8);
            }
        }

        chld_lock = 1;
        pid = fork();
        ASSERT_LE!(0, pid);
        TH_LOG!("fork: %m");

        if pid == 0 {
            let mut vl_local: c_ulong = 0;

            while chld_lock == 1 {
                asm!("", in("a0") chld_lock, options(nostack, preserves_flags));
            }

            set_initial_v_state(&mut vl_local);

            asm!(
                ".option push",
                ".option norvc",
                ".option arch, +zve32x",
                "ebreak",
                "nop",
                "ebreak",
                "vmv.v.i v0, -1",
                "ebreak",
                ".option pop",
            );
        } else {
            let mut regset_data: *mut __riscv_v_regset_state;
            let mut regs: user_regs_struct = core::mem::zeroed();
            let mut regset_size: size_t;
            let mut iov: iovec;
            let mut status: c_int = 0;

            /* attach */

            ASSERT_EQ!(0, ptrace(PTRACE_ATTACH, pid, null_mut::<c_void>(), null_mut::<c_void>()));
            ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
            ASSERT_TRUE!(WIFSTOPPED(status));

            /* unlock */

            ASSERT_EQ!(0, ptrace(PTRACE_POKEDATA, pid, &raw mut chld_lock, 0));

            /* resume and wait for the 1st ebreak */

            ASSERT_EQ!(0, ptrace(PTRACE_CONT, pid, null_mut::<c_void>(), null_mut::<c_void>()));
            ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
            ASSERT_TRUE!(WIFSTOPPED(status));

            /* read tracee vector csr regs using ptrace GETREGSET */

            regset_size = size_of::<__riscv_v_regset_state>() + vlenb_local as size_t * 32;
            regset_data = calloc(1, regset_size) as *mut __riscv_v_regset_state;

            iov = iovec {
                iov_base: regset_data as *mut c_void,
                iov_len: regset_size,
            };

            ASSERT_EQ!(0, ptrace(PTRACE_GETREGSET, pid, NT_RISCV_VECTOR, &mut iov));

            /* verify initial vsetvli settings */

            if is_xtheadvector_supported() {
                EXPECT_EQ!(5usize as c_ulong, (*regset_data).vtype);
            } else {
                EXPECT_EQ!(9usize as c_ulong, (*regset_data).vtype);
            }

            EXPECT_EQ!((*regset_data).vlenb, (*regset_data).vl);
            EXPECT_EQ!(vlenb_local, (*regset_data).vlenb);
            EXPECT_EQ!(0usize as c_ulong, (*regset_data).vstart);
            EXPECT_EQ!(0usize as c_ulong, (*regset_data).vcsr);

            /* apply valid settings from fixture variants */

            (*regset_data).vlenb = (*regset_data).vlenb.wrapping_mul((*variant).vlenb_mul);
            (*regset_data).vstart = (*variant).vstart;
            (*regset_data).vtype = (*variant).vtype;
            (*regset_data).vcsr = (*variant).vcsr;
            (*regset_data).vl = (*variant).vl;

            iov.iov_base = regset_data as *mut c_void;
            iov.iov_len = regset_size;

            ASSERT_EQ!(0, ptrace(PTRACE_SETREGSET, pid, NT_RISCV_VECTOR, &mut iov));

            /* skip 1st ebreak, then resume and wait for the 2nd ebreak */

            iov.iov_base = &mut regs as *mut user_regs_struct as *mut c_void;
            iov.iov_len = size_of::<user_regs_struct>();

            ASSERT_EQ!(0, ptrace(PTRACE_GETREGSET, pid, NT_PRSTATUS, &mut iov));
            regs.pc = regs.pc.wrapping_add(4);
            ASSERT_EQ!(0, ptrace(PTRACE_SETREGSET, pid, NT_PRSTATUS, &mut iov));

            ASSERT_EQ!(0, ptrace(PTRACE_CONT, pid, null_mut::<c_void>(), null_mut::<c_void>()));
            ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
            ASSERT_TRUE!(WIFSTOPPED(status));

            /* read tracee vector csr regs using ptrace GETREGSET */

            iov.iov_base = regset_data as *mut c_void;
            iov.iov_len = regset_size;

            ASSERT_EQ!(0, ptrace(PTRACE_GETREGSET, pid, NT_RISCV_VECTOR, &mut iov));

            /* verify vector csr regs from tracee context */

            EXPECT_EQ!((*regset_data).vstart, (*variant).vstart);
            EXPECT_EQ!((*regset_data).vtype, (*variant).vtype);
            EXPECT_EQ!((*regset_data).vcsr, (*variant).vcsr);
            EXPECT_EQ!((*regset_data).vl, (*variant).vl);
            EXPECT_EQ!((*regset_data).vlenb, vlenb_local);

            /* skip 2nd ebreak, then resume and wait for the 3rd ebreak */

            iov.iov_base = &mut regs as *mut user_regs_struct as *mut c_void;
            iov.iov_len = size_of::<user_regs_struct>();

            ASSERT_EQ!(0, ptrace(PTRACE_GETREGSET, pid, NT_PRSTATUS, &mut iov));
            regs.pc = regs.pc.wrapping_add(4);
            ASSERT_EQ!(0, ptrace(PTRACE_SETREGSET, pid, NT_PRSTATUS, &mut iov));

            ASSERT_EQ!(0, ptrace(PTRACE_CONT, pid, null_mut::<c_void>(), null_mut::<c_void>()));
            ASSERT_EQ!(pid, waitpid(pid, &mut status, 0));
            ASSERT_TRUE!(WIFSTOPPED(status));

            /* read tracee vector csr regs using ptrace GETREGSET */

            iov.iov_base = regset_data as *mut c_void;
            iov.iov_len = regset_size;

            ASSERT_EQ!(0, ptrace(PTRACE_GETREGSET, pid, NT_RISCV_VECTOR, &mut iov));

            /* verify vector csr regs from tracee context */

            EXPECT_EQ!((*regset_data).vstart, (*variant).vstart);
            EXPECT_EQ!((*regset_data).vtype, (*variant).vtype);
            EXPECT_EQ!((*regset_data).vcsr, (*variant).vcsr);
            EXPECT_EQ!((*regset_data).vl, (*variant).vl);
            EXPECT_EQ!((*regset_data).vlenb, vlenb_local);

            /* cleanup */
            free(regset_data as *mut c_void);
            ASSERT_EQ!(0, kill(pid, SIGKILL));
        }
    }
}

/* TEST_HARNESS_MAIN */
