// SPDX-License-Identifier: GPL-2.0

// Translated from perf/util/perf-regs-arch/perf_regs_s390.c.
// External constants, types, and functions are supplied by the surrounding perf
// and Linux bindings in the final repository.

use core::ffi::{c_char, c_int, c_void};
use core::ptr;

const SDT_OP_REGEX1: &[u8] = b"^(%r([0-9]|1[0-5]))$\0";
const SDT_OP_REGEX2: &[u8] = b"^([+-]?[0-9]+\\(%r([0-9]|1[0-5])\\))$\0";

const REG_EXTENDED: c_int = 1;
const EINVAL: c_int = 22;
const ENOMEM: c_int = 12;

#[repr(C)]
pub struct regex_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmatch_t {
    pub rm_so: isize,
    pub rm_eo: isize,
}

unsafe extern "C" {
    static PERF_REGS_MASK: u64;

    static PERF_REG_S390_R0: c_int;
    static PERF_REG_S390_R1: c_int;
    static PERF_REG_S390_R2: c_int;
    static PERF_REG_S390_R3: c_int;
    static PERF_REG_S390_R4: c_int;
    static PERF_REG_S390_R5: c_int;
    static PERF_REG_S390_R6: c_int;
    static PERF_REG_S390_R7: c_int;
    static PERF_REG_S390_R8: c_int;
    static PERF_REG_S390_R9: c_int;
    static PERF_REG_S390_R10: c_int;
    static PERF_REG_S390_R11: c_int;
    static PERF_REG_S390_R12: c_int;
    static PERF_REG_S390_R13: c_int;
    static PERF_REG_S390_R14: c_int;
    static PERF_REG_S390_R15: c_int;
    static PERF_REG_S390_FP0: c_int;
    static PERF_REG_S390_FP1: c_int;
    static PERF_REG_S390_FP2: c_int;
    static PERF_REG_S390_FP3: c_int;
    static PERF_REG_S390_FP4: c_int;
    static PERF_REG_S390_FP5: c_int;
    static PERF_REG_S390_FP6: c_int;
    static PERF_REG_S390_FP7: c_int;
    static PERF_REG_S390_FP8: c_int;
    static PERF_REG_S390_FP9: c_int;
    static PERF_REG_S390_FP10: c_int;
    static PERF_REG_S390_FP11: c_int;
    static PERF_REG_S390_FP12: c_int;
    static PERF_REG_S390_FP13: c_int;
    static PERF_REG_S390_FP14: c_int;
    static PERF_REG_S390_FP15: c_int;
    static PERF_REG_S390_MASK: c_int;
    static PERF_REG_S390_PC: c_int;

    static SDT_ARG_SKIP: c_int;
    static SDT_ARG_VALID: c_int;

    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: usize,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
    fn regfree(preg: *mut regex_t);
    fn zalloc(size: usize) -> *mut c_void;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn pr_debug4(fmt: *const c_char, ...);
}

static mut sdt_op_regex1: regex_t = regex_t { _private: [] };
static mut sdt_op_regex2: regex_t = regex_t { _private: [] };

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __perf_reg_mask_s390(_intr: bool) -> u64 {
    unsafe { PERF_REGS_MASK }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __perf_reg_name_s390(id: c_int) -> *const c_char {
    unsafe {
        if id == PERF_REG_S390_R0 {
            return c"R0".as_ptr();
        }
        if id == PERF_REG_S390_R1 {
            return c"R1".as_ptr();
        }
        if id == PERF_REG_S390_R2 {
            return c"R2".as_ptr();
        }
        if id == PERF_REG_S390_R3 {
            return c"R3".as_ptr();
        }
        if id == PERF_REG_S390_R4 {
            return c"R4".as_ptr();
        }
        if id == PERF_REG_S390_R5 {
            return c"R5".as_ptr();
        }
        if id == PERF_REG_S390_R6 {
            return c"R6".as_ptr();
        }
        if id == PERF_REG_S390_R7 {
            return c"R7".as_ptr();
        }
        if id == PERF_REG_S390_R8 {
            return c"R8".as_ptr();
        }
        if id == PERF_REG_S390_R9 {
            return c"R9".as_ptr();
        }
        if id == PERF_REG_S390_R10 {
            return c"R10".as_ptr();
        }
        if id == PERF_REG_S390_R11 {
            return c"R11".as_ptr();
        }
        if id == PERF_REG_S390_R12 {
            return c"R12".as_ptr();
        }
        if id == PERF_REG_S390_R13 {
            return c"R13".as_ptr();
        }
        if id == PERF_REG_S390_R14 {
            return c"R14".as_ptr();
        }
        if id == PERF_REG_S390_R15 {
            return c"R15".as_ptr();
        }
        if id == PERF_REG_S390_FP0 {
            return c"FP0".as_ptr();
        }
        if id == PERF_REG_S390_FP1 {
            return c"FP1".as_ptr();
        }
        if id == PERF_REG_S390_FP2 {
            return c"FP2".as_ptr();
        }
        if id == PERF_REG_S390_FP3 {
            return c"FP3".as_ptr();
        }
        if id == PERF_REG_S390_FP4 {
            return c"FP4".as_ptr();
        }
        if id == PERF_REG_S390_FP5 {
            return c"FP5".as_ptr();
        }
        if id == PERF_REG_S390_FP6 {
            return c"FP6".as_ptr();
        }
        if id == PERF_REG_S390_FP7 {
            return c"FP7".as_ptr();
        }
        if id == PERF_REG_S390_FP8 {
            return c"FP8".as_ptr();
        }
        if id == PERF_REG_S390_FP9 {
            return c"FP9".as_ptr();
        }
        if id == PERF_REG_S390_FP10 {
            return c"FP10".as_ptr();
        }
        if id == PERF_REG_S390_FP11 {
            return c"FP11".as_ptr();
        }
        if id == PERF_REG_S390_FP12 {
            return c"FP12".as_ptr();
        }
        if id == PERF_REG_S390_FP13 {
            return c"FP13".as_ptr();
        }
        if id == PERF_REG_S390_FP14 {
            return c"FP14".as_ptr();
        }
        if id == PERF_REG_S390_FP15 {
            return c"FP15".as_ptr();
        }
        if id == PERF_REG_S390_MASK {
            return c"MASK".as_ptr();
        }
        if id == PERF_REG_S390_PC {
            return c"PC".as_ptr();
        }

        ptr::null()
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __perf_reg_ip_s390() -> u64 {
    unsafe { PERF_REG_S390_PC as u64 }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn __perf_reg_sp_s390() -> u64 {
    unsafe { PERF_REG_S390_R15 as u64 }
}

/* %rXX */
/* +-###(%rXX) */
unsafe fn sdt_init_op_regex() -> c_int {
    static mut initialized: c_int = 0;
    let mut ret: c_int = 0;

    unsafe {
        if initialized != 0 {
            return 0;
        }

        ret = regcomp(
            &raw mut sdt_op_regex1,
            SDT_OP_REGEX1.as_ptr() as *const c_char,
            REG_EXTENDED,
        );
        if ret != 0 {
            pr_debug4(
                c"Regex compilation error, initialized %d\n".as_ptr(),
                initialized,
            );
            initialized = 0;
            return ret;
        }
        initialized = 1;

        ret = regcomp(
            &raw mut sdt_op_regex2,
            SDT_OP_REGEX2.as_ptr() as *const c_char,
            REG_EXTENDED,
        );
        if ret != 0 {
            regfree(&raw mut sdt_op_regex1);
            pr_debug4(
                c"Regex compilation error, initialized %d\n".as_ptr(),
                initialized,
            );
            initialized = 0;
            return ret;
        }
        initialized = 2;

        0
    }
}

/*
 * Parse OP and convert it into uprobe format, which is, +/-NUM(%gprREG).
 * Possible variants of OP are:
 *	Format		Example
 *	-------------------------
 *	NUM(%rREG)	48(%r1)
 *	-NUM(%rREG)	-48(%r1)
 *	+NUM(%rREG)	+48(%r1)
 *	%rREG		%r1
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn __perf_sdt_arg_parse_op_s390(
    old_op: *mut c_char,
    new_op: *mut *mut c_char,
) -> c_int {
    let mut ret: c_int;
    let mut new_len: c_int;
    let mut rm: [regmatch_t; 6] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 6];

    unsafe {
        *new_op = ptr::null_mut();
        ret = sdt_init_op_regex();
        if ret != 0 {
            return -EINVAL;
        }

        if regexec(
            &raw const sdt_op_regex1,
            old_op,
            rm.len(),
            rm.as_mut_ptr(),
            0,
        ) == 0
            || regexec(
                &raw const sdt_op_regex2,
                old_op,
                rm.len(),
                rm.as_mut_ptr(),
                0,
            ) == 0
        {
            new_len = 1; /* NULL byte */
            new_len += (rm[1].rm_eo - rm[1].rm_so) as c_int;
            *new_op = zalloc(new_len as usize) as *mut c_char;
            if (*new_op).is_null() {
                return -ENOMEM;
            }

            scnprintf(
                *new_op,
                new_len as usize,
                c"%.*s".as_ptr(),
                (rm[1].rm_eo - rm[1].rm_so) as c_int,
                old_op.offset(rm[1].rm_so),
            );
        } else {
            pr_debug4(c"Skipping unsupported SDT argument: %s\n".as_ptr(), old_op);
            return SDT_ARG_SKIP;
        }

        SDT_ARG_VALID
    }
}
