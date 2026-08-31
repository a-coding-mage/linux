/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Ptrace interface test helper functions
 *
 * Copyright (C) 2015 Anshuman Khandual, IBM Corporation.
 */

// C header guard/includes omitted. Symbols from inttypes.h, unistd.h, stdlib.h,
// string.h, malloc.h, errno.h, time.h, sys/ptrace.h, sys/ioctl.h, sys/uio.h,
// sys/types.h, sys/wait.h, sys/signal.h, sys/ipc.h, sys/shm.h, sys/user.h,
// sys/syscall.h, linux/elf.h, linux/types.h, linux/auxvec.h, reg.h, and
// utils.h are expected to be supplied by the surrounding translation.

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};
use core::mem::size_of;
use core::ptr;

pub type pid_t = c_int;
pub type __u64 = u64;
pub type u64 = u64;

pub const TEST_PASS: c_int = 0;
pub const TEST_FAIL: c_int = 1;

#[repr(C)]
pub struct fpr_regs {
    pub fpr: [__u64; 32],
    pub fpscr: __u64,
}

#[repr(C)]
pub struct tm_spr_regs {
    pub tm_tfhar: c_ulong,
    pub tm_texasr: c_ulong,
    pub tm_tfiar: c_ulong,
}

pub const NT_PPC_TAR: c_ulong = 0x103;
pub const NT_PPC_PPR: c_ulong = 0x104;
pub const NT_PPC_DSCR: c_ulong = 0x105;
pub const NT_PPC_EBB: c_ulong = 0x106;
pub const NT_PPC_PMU: c_ulong = 0x107;
pub const NT_PPC_TM_CGPR: c_ulong = 0x108;
pub const NT_PPC_TM_CFPR: c_ulong = 0x109;
pub const NT_PPC_TM_CVMX: c_ulong = 0x10a;
pub const NT_PPC_TM_CVSX: c_ulong = 0x10b;
pub const NT_PPC_TM_SPR: c_ulong = 0x10c;
pub const NT_PPC_TM_CTAR: c_ulong = 0x10d;
pub const NT_PPC_TM_CPPR: c_ulong = 0x10e;
pub const NT_PPC_TM_CDSCR: c_ulong = 0x10f;

#[repr(C)]
pub struct iovec {
    pub iov_base: *mut c_void,
    pub iov_len: usize,
}

// Defined by sys/user.h/reg.h in the original C translation unit.
#[repr(C)]
pub struct pt_regs {
    pub gpr: [c_ulong; 32],
}

unsafe extern "C" {
    fn ptrace(request: c_ulong, ...) -> c_long;
    fn waitpid(pid: pid_t, status: *mut c_int, options: c_int) -> pid_t;
    fn perror(s: *const c_char);
    fn malloc(size: usize) -> *mut c_void;
    fn free(ptr: *mut c_void);
    fn memcpy(dest: *mut c_void, src: *const c_void, n: usize) -> *mut c_void;
    fn syscall(number: c_long, ...) -> c_long;
    fn printf(format: *const c_char, ...) -> c_int;
    fn mfspr(spr: c_ulong) -> c_ulong;
    pub fn store_gpr(addr: *mut c_ulong);
}

// Constants/macros supplied by included system and local headers.
unsafe extern "C" {
    static PTRACE_ATTACH: c_ulong;
    static PTRACE_DETACH: c_ulong;
    static PTRACE_CONT: c_ulong;
    static PTRACE_GETREGSET: c_ulong;
    static PTRACE_SETREGSET: c_ulong;
    static PTRACE_GETFPREGS: c_ulong;
    static PTRACE_SETFPREGS: c_ulong;
    static PTRACE_GETREGS: c_ulong;
    static PTRACE_SETREGS: c_ulong;
    static PTRACE_PEEKUSER: c_ulong;
    static PTRACE_POKEUSER: c_ulong;
    static PTRACE_GETVRREGS: c_ulong;
    static PTRACE_SETVRREGS: c_ulong;
    static PTRACE_GETVSRREGS: c_ulong;
    static PTRACE_SETVSRREGS: c_ulong;
    static __NR_ptrace: c_long;
    static PT_FPR0: c_int;
    static SPRN_TFIAR: c_ulong;
    static TEXASR_FP: c_ulong;
    static TEXASR_DA: c_ulong;
    static TEXASR_NO: c_ulong;
    static TEXASR_FO: c_ulong;
    static TEXASR_SIC: c_ulong;
    static TEXASR_NTC: c_ulong;
    static TEXASR_TC: c_ulong;
    static TEXASR_TIC: c_ulong;
    static TEXASR_IC: c_ulong;
    static TEXASR_IFC: c_ulong;
    static TEXASR_ABT: c_ulong;
    static TEXASR_SPD: c_ulong;
    static TEXASR_HV: c_ulong;
    static TEXASR_PR: c_ulong;
    static TEXASR_FS: c_ulong;
    static TEXASR_TE: c_ulong;
    static TEXASR_ROT: c_ulong;
}

macro_rules! FAIL_IF {
    ($expr:expr) => {
        if $expr != 0 {
            return TEST_FAIL;
        }
    };
}

/* Basic ptrace operations */
pub unsafe extern "C" fn start_trace(child: pid_t) -> c_int {
    let mut ret: c_int;

    ret = ptrace(PTRACE_ATTACH, child, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>()) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_ATTACH) failed".as_ptr());
        return TEST_FAIL;
    }
    ret = waitpid(child, ptr::null_mut(), 0);
    if ret != child {
        perror(c"waitpid() failed".as_ptr());
        return TEST_FAIL;
    }
    TEST_PASS
}

pub unsafe extern "C" fn stop_trace(child: pid_t) -> c_int {
    let ret = ptrace(PTRACE_DETACH, child, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>());
    if ret != 0 {
        perror(c"ptrace(PTRACE_DETACH) failed".as_ptr());
        return TEST_FAIL;
    }
    TEST_PASS
}

pub unsafe extern "C" fn cont_trace(child: pid_t) -> c_int {
    let ret = ptrace(PTRACE_CONT, child, ptr::null_mut::<c_void>(), ptr::null_mut::<c_void>());
    if ret != 0 {
        perror(c"ptrace(PTRACE_CONT) failed".as_ptr());
        return TEST_FAIL;
    }
    TEST_PASS
}

pub unsafe extern "C" fn ptrace_read_regs(child: pid_t, type_: c_ulong, regs: *mut c_ulong, n: c_int) -> c_int {
    let mut iov: iovec;
    let ret: c_long;

    FAIL_IF!(start_trace(child));

    iov = iovec {
        iov_base: regs.cast(),
        iov_len: n as usize * size_of::<c_ulong>(),
    };

    ret = ptrace(PTRACE_GETREGSET, child, type_, &mut iov);
    if ret != 0 {
        return ret as c_int;
    }

    FAIL_IF!(stop_trace(child));

    TEST_PASS
}

pub unsafe extern "C" fn ptrace_write_regs(child: pid_t, type_: c_ulong, regs: *mut c_ulong, n: c_int) -> c_long {
    let mut iov: iovec;
    let ret: c_long;

    FAIL_IF!(start_trace(child));

    iov = iovec {
        iov_base: regs.cast(),
        iov_len: n as usize * size_of::<c_ulong>(),
    };

    ret = ptrace(PTRACE_SETREGSET, child, type_, &mut iov);

    FAIL_IF!(stop_trace(child));

    ret
}

/* TAR, PPR, DSCR */
pub unsafe extern "C" fn show_tar_registers(child: pid_t, out: *mut c_ulong) -> c_int {
    let mut iov: iovec;
    let reg: *mut c_ulong;
    let mut ret: c_int;

    reg = malloc(size_of::<c_ulong>()).cast();
    if reg.is_null() {
        perror(c"malloc() failed".as_ptr());
        return TEST_FAIL;
    }
    iov = iovec { iov_base: reg.cast(), iov_len: size_of::<c_ulong>() };

    ret = ptrace(PTRACE_GETREGSET, child, NT_PPC_TAR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        free(reg.cast());
        return TEST_FAIL;
    }
    if !out.is_null() {
        *out.add(0) = *reg;
    }

    ret = ptrace(PTRACE_GETREGSET, child, NT_PPC_PPR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        free(reg.cast());
        return TEST_FAIL;
    }
    if !out.is_null() {
        *out.add(1) = *reg;
    }

    ret = ptrace(PTRACE_GETREGSET, child, NT_PPC_DSCR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        free(reg.cast());
        return TEST_FAIL;
    }
    if !out.is_null() {
        *out.add(2) = *reg;
    }

    free(reg.cast());
    TEST_PASS
}

pub unsafe extern "C" fn write_tar_registers(child: pid_t, tar: c_ulong, ppr: c_ulong, dscr: c_ulong) -> c_int {
    let mut iov: iovec;
    let reg: *mut c_ulong;
    let mut ret: c_int;

    reg = malloc(size_of::<c_ulong>()).cast();
    if reg.is_null() {
        perror(c"malloc() failed".as_ptr());
        return TEST_FAIL;
    }

    iov = iovec { iov_base: reg.cast(), iov_len: size_of::<c_ulong>() };

    *reg = tar;
    ret = ptrace(PTRACE_SETREGSET, child, NT_PPC_TAR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_SETREGSET) failed".as_ptr());
        free(reg.cast());
        return TEST_FAIL;
    }

    *reg = ppr;
    ret = ptrace(PTRACE_SETREGSET, child, NT_PPC_PPR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_SETREGSET) failed".as_ptr());
        free(reg.cast());
        return TEST_FAIL;
    }

    *reg = dscr;
    ret = ptrace(PTRACE_SETREGSET, child, NT_PPC_DSCR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_SETREGSET) failed".as_ptr());
        free(reg.cast());
        return TEST_FAIL;
    }

    free(reg.cast());
    TEST_PASS
}

pub unsafe extern "C" fn show_tm_checkpointed_state(child: pid_t, out: *mut c_ulong) -> c_int {
    let mut iov: iovec;
    let reg: *mut c_ulong;
    let mut ret: c_int;

    reg = malloc(size_of::<c_ulong>()).cast();
    if reg.is_null() {
        perror(c"malloc() failed".as_ptr());
        return TEST_FAIL;
    }

    iov = iovec { iov_base: reg.cast(), iov_len: size_of::<c_ulong>() };

    ret = ptrace(PTRACE_GETREGSET, child, NT_PPC_TM_CTAR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        free(reg.cast());
        return TEST_FAIL;
    }
    if !out.is_null() {
        *out.add(0) = *reg;
    }

    ret = ptrace(PTRACE_GETREGSET, child, NT_PPC_TM_CPPR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        free(reg.cast());
        return TEST_FAIL;
    }
    if !out.is_null() {
        *out.add(1) = *reg;
    }

    ret = ptrace(PTRACE_GETREGSET, child, NT_PPC_TM_CDSCR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        free(reg.cast());
        return TEST_FAIL;
    }
    if !out.is_null() {
        *out.add(2) = *reg;
    }

    free(reg.cast());
    TEST_PASS
}

pub unsafe extern "C" fn write_ckpt_tar_registers(child: pid_t, tar: c_ulong, ppr: c_ulong, dscr: c_ulong) -> c_int {
    let mut iov: iovec;
    let reg: *mut c_ulong;
    let mut ret: c_int;

    reg = malloc(size_of::<c_ulong>()).cast();
    if reg.is_null() {
        perror(c"malloc() failed".as_ptr());
        return TEST_FAIL;
    }

    iov = iovec { iov_base: reg.cast(), iov_len: size_of::<c_ulong>() };

    *reg = tar;
    ret = ptrace(PTRACE_SETREGSET, child, NT_PPC_TM_CTAR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        free(reg.cast());
        return TEST_FAIL;
    }

    *reg = ppr;
    ret = ptrace(PTRACE_SETREGSET, child, NT_PPC_TM_CPPR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        free(reg.cast());
        return TEST_FAIL;
    }

    *reg = dscr;
    ret = ptrace(PTRACE_SETREGSET, child, NT_PPC_TM_CDSCR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        free(reg.cast());
        return TEST_FAIL;
    }

    free(reg.cast());
    TEST_PASS
}

/* FPR */
pub unsafe extern "C" fn show_fpr(child: pid_t, fpr: *mut __u64) -> c_int {
    let regs: *mut fpr_regs;
    let ret: c_int;

    regs = malloc(size_of::<fpr_regs>()).cast();
    ret = ptrace(PTRACE_GETFPREGS, child, ptr::null_mut::<c_void>(), regs) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        return TEST_FAIL;
    }

    if !fpr.is_null() {
        let mut i = 0;
        while i < 32 {
            *fpr.add(i) = (*regs).fpr[i];
            i += 1;
        }
    }
    TEST_PASS
}

pub unsafe extern "C" fn write_fpr(child: pid_t, val: __u64) -> c_int {
    let regs: *mut fpr_regs;
    let mut ret: c_int;

    regs = malloc(size_of::<fpr_regs>()).cast();
    ret = ptrace(PTRACE_GETFPREGS, child, ptr::null_mut::<c_void>(), regs) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        return TEST_FAIL;
    }

    let mut i = 0;
    while i < 32 {
        (*regs).fpr[i] = val;
        i += 1;
    }

    ret = ptrace(PTRACE_SETFPREGS, child, ptr::null_mut::<c_void>(), regs) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        return TEST_FAIL;
    }
    TEST_PASS
}

pub unsafe extern "C" fn show_ckpt_fpr(child: pid_t, fpr: *mut __u64) -> c_int {
    let regs: *mut fpr_regs;
    let mut iov: iovec;
    let ret: c_int;

    regs = malloc(size_of::<fpr_regs>()).cast();
    iov = iovec { iov_base: regs.cast(), iov_len: size_of::<fpr_regs>() };

    ret = ptrace(PTRACE_GETREGSET, child, NT_PPC_TM_CFPR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        return TEST_FAIL;
    }

    if !fpr.is_null() {
        let mut i = 0;
        while i < 32 {
            *fpr.add(i) = (*regs).fpr[i];
            i += 1;
        }
    }

    TEST_PASS
}

pub unsafe extern "C" fn write_ckpt_fpr(child: pid_t, val: c_ulong) -> c_int {
    let regs: *mut fpr_regs;
    let mut iov: iovec;
    let mut ret: c_int;

    regs = malloc(size_of::<fpr_regs>()).cast();
    iov = iovec { iov_base: regs.cast(), iov_len: size_of::<fpr_regs>() };

    ret = ptrace(PTRACE_GETREGSET, child, NT_PPC_TM_CFPR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        return TEST_FAIL;
    }

    let mut i = 0;
    while i < 32 {
        (*regs).fpr[i] = val as __u64;
        i += 1;
    }

    ret = ptrace(PTRACE_SETREGSET, child, NT_PPC_TM_CFPR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        return TEST_FAIL;
    }
    TEST_PASS
}

/* GPR */
pub unsafe extern "C" fn show_gpr(child: pid_t, gpr: *mut c_ulong) -> c_int {
    let regs: *mut pt_regs;
    let ret: c_int;

    regs = malloc(size_of::<pt_regs>()).cast();
    if regs.is_null() {
        perror(c"malloc() failed".as_ptr());
        return TEST_FAIL;
    }

    ret = ptrace(PTRACE_GETREGS, child, ptr::null_mut::<c_void>(), regs) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        return TEST_FAIL;
    }

    if !gpr.is_null() {
        let mut i = 14;
        while i < 32 {
            *gpr.add(i - 14) = (*regs).gpr[i];
            i += 1;
        }
    }

    TEST_PASS
}

pub unsafe extern "C" fn sys_ptrace(request: c_ulong, pid: pid_t, addr: c_ulong, data: c_ulong) -> c_long {
    syscall(__NR_ptrace, request, pid, addr as *mut c_void, data)
}

// 33 because of FPSCR
pub const PT_NUM_FPRS: usize = 33 * (size_of::<__u64>() / size_of::<c_ulong>());

pub unsafe extern "C" fn peek_fprs(child: pid_t) -> *mut __u64 {
    let fprs: *mut c_ulong;
    let mut p: *mut c_ulong;
    let mut addr: c_ulong;
    let mut ret: c_long;
    let mut i: usize;

    fprs = malloc(size_of::<c_ulong>() * PT_NUM_FPRS).cast();
    if fprs.is_null() {
        perror(c"malloc() failed".as_ptr());
        return ptr::null_mut();
    }

    i = 0;
    p = fprs;
    while i < PT_NUM_FPRS {
        addr = size_of::<c_ulong>() as c_ulong * (PT_FPR0 as c_ulong + i as c_ulong);
        ret = sys_ptrace(PTRACE_PEEKUSER, child, addr, p as c_ulong);
        if ret != 0 {
            perror(c"ptrace(PTRACE_PEEKUSR) failed".as_ptr());
            return ptr::null_mut();
        }
        i += 1;
        p = p.add(1);
    }

    addr = size_of::<c_ulong>() as c_ulong * (PT_FPR0 as c_ulong + i as c_ulong);
    ret = sys_ptrace(PTRACE_PEEKUSER, child, addr, &mut addr as *mut c_ulong as c_ulong);
    if ret == 0 {
        printf(c"ptrace(PTRACE_PEEKUSR) succeeded unexpectedly!\n".as_ptr());
        return ptr::null_mut();
    }

    fprs.cast()
}

pub unsafe extern "C" fn poke_fprs(child: pid_t, fprs: *mut c_ulong) -> c_int {
    let mut p: *mut c_ulong;
    let mut addr: c_ulong;
    let mut ret: c_long;
    let mut i: usize;

    i = 0;
    p = fprs;
    while i < PT_NUM_FPRS {
        addr = size_of::<c_ulong>() as c_ulong * (PT_FPR0 as c_ulong + i as c_ulong);
        ret = sys_ptrace(PTRACE_POKEUSER, child, addr, *p);
        if ret != 0 {
            perror(c"ptrace(PTRACE_POKEUSR) failed".as_ptr());
            return -1;
        }
        i += 1;
        p = p.add(1);
    }

    addr = size_of::<c_ulong>() as c_ulong * (PT_FPR0 as c_ulong + i as c_ulong);
    ret = sys_ptrace(PTRACE_POKEUSER, child, addr, addr);
    if ret == 0 {
        printf(c"ptrace(PTRACE_POKEUSR) succeeded unexpectedly!\n".as_ptr());
        return -1;
    }

    0
}

pub unsafe extern "C" fn write_gpr(child: pid_t, val: c_ulong) -> c_int {
    let regs: *mut pt_regs;
    let mut ret: c_int;

    regs = malloc(size_of::<pt_regs>()).cast();
    if regs.is_null() {
        perror(c"malloc() failed".as_ptr());
        return TEST_FAIL;
    }

    ret = ptrace(PTRACE_GETREGS, child, ptr::null_mut::<c_void>(), regs) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        return TEST_FAIL;
    }

    let mut i = 14;
    while i < 32 {
        (*regs).gpr[i] = val;
        i += 1;
    }

    ret = ptrace(PTRACE_SETREGS, child, ptr::null_mut::<c_void>(), regs) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        return TEST_FAIL;
    }
    TEST_PASS
}

pub unsafe extern "C" fn show_ckpt_gpr(child: pid_t, gpr: *mut c_ulong) -> c_int {
    let regs: *mut pt_regs;
    let mut iov: iovec;
    let ret: c_int;

    regs = malloc(size_of::<pt_regs>()).cast();
    if regs.is_null() {
        perror(c"malloc() failed".as_ptr());
        return TEST_FAIL;
    }

    iov = iovec { iov_base: regs.cast(), iov_len: size_of::<pt_regs>() };

    ret = ptrace(PTRACE_GETREGSET, child, NT_PPC_TM_CGPR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        return TEST_FAIL;
    }

    if !gpr.is_null() {
        let mut i = 14;
        while i < 32 {
            *gpr.add(i - 14) = (*regs).gpr[i];
            i += 1;
        }
    }

    TEST_PASS
}

pub unsafe extern "C" fn write_ckpt_gpr(child: pid_t, val: c_ulong) -> c_int {
    let regs: *mut pt_regs;
    let mut iov: iovec;
    let mut ret: c_int;

    regs = malloc(size_of::<pt_regs>()).cast();
    if regs.is_null() {
        perror(c"malloc() failed\n".as_ptr());
        return TEST_FAIL;
    }
    iov = iovec { iov_base: regs.cast(), iov_len: size_of::<pt_regs>() };

    ret = ptrace(PTRACE_GETREGSET, child, NT_PPC_TM_CGPR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        return TEST_FAIL;
    }

    let mut i = 14;
    while i < 32 {
        (*regs).gpr[i] = val;
        i += 1;
    }

    ret = ptrace(PTRACE_SETREGSET, child, NT_PPC_TM_CGPR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        return TEST_FAIL;
    }
    TEST_PASS
}

/* VMX */
pub unsafe extern "C" fn show_vmx(child: pid_t, vmx: *mut [c_ulong; 2]) -> c_int {
    let ret = ptrace(PTRACE_GETVRREGS, child, 0, vmx) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETVRREGS) failed".as_ptr());
        return TEST_FAIL;
    }
    TEST_PASS
}

pub unsafe extern "C" fn show_vmx_ckpt(child: pid_t, vmx: *mut [c_ulong; 2]) -> c_int {
    let mut regs = [[0 as c_ulong; 2]; 34];
    let mut iov: iovec;
    let ret: c_int;

    iov = iovec { iov_base: regs.as_mut_ptr().cast(), iov_len: size_of::<[[c_ulong; 2]; 34]>() };
    ret = ptrace(PTRACE_GETREGSET, child, NT_PPC_TM_CVMX, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET, NT_PPC_TM_CVMX) failed".as_ptr());
        return TEST_FAIL;
    }
    memcpy(vmx.cast(), regs.as_ptr().cast(), size_of::<[[c_ulong; 2]; 34]>());
    TEST_PASS
}

pub unsafe extern "C" fn write_vmx(child: pid_t, vmx: *mut [c_ulong; 2]) -> c_int {
    let ret = ptrace(PTRACE_SETVRREGS, child, 0, vmx) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_SETVRREGS) failed".as_ptr());
        return TEST_FAIL;
    }
    TEST_PASS
}

pub unsafe extern "C" fn write_vmx_ckpt(child: pid_t, vmx: *mut [c_ulong; 2]) -> c_int {
    let mut regs = [[0 as c_ulong; 2]; 34];
    let mut iov: iovec;
    let ret: c_int;

    memcpy(regs.as_mut_ptr().cast(), vmx.cast(), size_of::<[[c_ulong; 2]; 34]>());
    iov = iovec { iov_base: regs.as_mut_ptr().cast(), iov_len: size_of::<[[c_ulong; 2]; 34]>() };
    ret = ptrace(PTRACE_SETREGSET, child, NT_PPC_TM_CVMX, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_SETREGSET, NT_PPC_TM_CVMX) failed".as_ptr());
        return TEST_FAIL;
    }
    TEST_PASS
}

/* VSX */
pub unsafe extern "C" fn show_vsx(child: pid_t, vsx: *mut c_ulong) -> c_int {
    let ret = ptrace(PTRACE_GETVSRREGS, child, 0, vsx) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETVSRREGS) failed".as_ptr());
        return TEST_FAIL;
    }
    TEST_PASS
}

pub unsafe extern "C" fn show_vsx_ckpt(child: pid_t, vsx: *mut c_ulong) -> c_int {
    let mut regs = [0 as c_ulong; 32];
    let mut iov: iovec;
    let ret: c_int;

    iov = iovec { iov_base: regs.as_mut_ptr().cast(), iov_len: size_of::<[c_ulong; 32]>() };
    ret = ptrace(PTRACE_GETREGSET, child, NT_PPC_TM_CVSX, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET, NT_PPC_TM_CVSX) failed".as_ptr());
        return TEST_FAIL;
    }
    memcpy(vsx.cast(), regs.as_ptr().cast(), size_of::<[c_ulong; 32]>());
    TEST_PASS
}

pub unsafe extern "C" fn write_vsx(child: pid_t, vsx: *mut c_ulong) -> c_int {
    let ret = ptrace(PTRACE_SETVSRREGS, child, 0, vsx) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_SETVSRREGS) failed".as_ptr());
        return TEST_FAIL;
    }
    TEST_PASS
}

pub unsafe extern "C" fn write_vsx_ckpt(child: pid_t, vsx: *mut c_ulong) -> c_int {
    let mut regs = [0 as c_ulong; 32];
    let mut iov: iovec;
    let ret: c_int;

    memcpy(regs.as_mut_ptr().cast(), vsx.cast(), size_of::<[c_ulong; 32]>());
    iov = iovec { iov_base: regs.as_mut_ptr().cast(), iov_len: size_of::<[c_ulong; 32]>() };
    ret = ptrace(PTRACE_SETREGSET, child, NT_PPC_TM_CVSX, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_SETREGSET, NT_PPC_TM_CVSX) failed".as_ptr());
        return TEST_FAIL;
    }
    TEST_PASS
}

/* TM SPR */
pub unsafe extern "C" fn show_tm_spr(child: pid_t, out: *mut tm_spr_regs) -> c_int {
    let regs: *mut tm_spr_regs;
    let mut iov: iovec;
    let ret: c_int;

    regs = malloc(size_of::<tm_spr_regs>()).cast();
    if regs.is_null() {
        perror(c"malloc() failed".as_ptr());
        return TEST_FAIL;
    }

    iov = iovec { iov_base: regs.cast(), iov_len: size_of::<tm_spr_regs>() };

    ret = ptrace(PTRACE_GETREGSET, child, NT_PPC_TM_SPR, &mut iov) as c_int;
    if ret != 0 {
        perror(c"ptrace(PTRACE_GETREGSET) failed".as_ptr());
        return TEST_FAIL;
    }

    if !out.is_null() {
        memcpy(out.cast(), regs.cast(), size_of::<tm_spr_regs>());
    }

    TEST_PASS
}

/* Analyse TEXASR after TM failure */
#[inline]
pub unsafe extern "C" fn get_tfiar() -> c_ulong {
    mfspr(SPRN_TFIAR)
}

pub unsafe extern "C" fn analyse_texasr(texasr: c_ulong) {
    printf(c"TEXASR: %16lx\t".as_ptr(), texasr);

    if texasr & TEXASR_FP != 0 {
        printf(c"TEXASR_FP  ".as_ptr());
    }
    if texasr & TEXASR_DA != 0 {
        printf(c"TEXASR_DA  ".as_ptr());
    }
    if texasr & TEXASR_NO != 0 {
        printf(c"TEXASR_NO  ".as_ptr());
    }
    if texasr & TEXASR_FO != 0 {
        printf(c"TEXASR_FO  ".as_ptr());
    }
    if texasr & TEXASR_SIC != 0 {
        printf(c"TEXASR_SIC  ".as_ptr());
    }
    if texasr & TEXASR_NTC != 0 {
        printf(c"TEXASR_NTC  ".as_ptr());
    }
    if texasr & TEXASR_TC != 0 {
        printf(c"TEXASR_TC  ".as_ptr());
    }
    if texasr & TEXASR_TIC != 0 {
        printf(c"TEXASR_TIC  ".as_ptr());
    }
    if texasr & TEXASR_IC != 0 {
        printf(c"TEXASR_IC  ".as_ptr());
    }
    if texasr & TEXASR_IFC != 0 {
        printf(c"TEXASR_IFC  ".as_ptr());
    }
    if texasr & TEXASR_ABT != 0 {
        printf(c"TEXASR_ABT  ".as_ptr());
    }
    if texasr & TEXASR_SPD != 0 {
        printf(c"TEXASR_SPD  ".as_ptr());
    }
    if texasr & TEXASR_HV != 0 {
        printf(c"TEXASR_HV  ".as_ptr());
    }
    if texasr & TEXASR_PR != 0 {
        printf(c"TEXASR_PR  ".as_ptr());
    }
    if texasr & TEXASR_FS != 0 {
        printf(c"TEXASR_FS  ".as_ptr());
    }
    if texasr & TEXASR_TE != 0 {
        printf(c"TEXASR_TE  ".as_ptr());
    }
    if texasr & TEXASR_ROT != 0 {
        printf(c"TEXASR_ROT  ".as_ptr());
    }

    printf(c"TFIAR :%lx\n".as_ptr(), get_tfiar());
}
