// SPDX-License-Identifier: GPL-2.0

// C dependencies:
// errno.h, regex.h, string.h, linux/kernel.h, linux/zalloc.h
// ../debug.h, ../perf_regs.h, ../../arch/riscv/include/perf_regs.h

use core::ffi::{c_char, c_int, c_ulong, c_void};
use core::ptr;

/*
 * RISC-V SDT argument formats (GCC 'nor' constraint):
 *
 * Register:  REG        e.g. a0, t1, s0, sp
 * Memory:    NUM(REG)   e.g. 8(a0), -20(s0)
 * Constant:  NUM        e.g. 99  (not supported by uprobe, skip)
 *
 * Note: 'zero' (x0) is hardwired to 0 and not in pt_regs; skip it.
 *
 * Uprobe target format:
 *   Register: %REG       e.g. %a0
 *   Memory:   +NUM(%REG) or -NUM(%REG)
 */

/* RISC-V register ABI names: ra, sp, gp, tp, t0-t6, s0-s11, a0-a7 */
const SDT_OP_REGEX1: *const c_char =
    b"^(ra|sp|gp|tp|t[0-6]|s[0-9]|s1[01]|a[0-7])$\0".as_ptr() as *const c_char;

/* RISC-V memory operand: [-]NUM(REG) */
const SDT_OP_REGEX2: *const c_char =
    b"^(\\-)?([0-9]+)\\((ra|sp|gp|tp|t[0-6]|s[0-9]|s1[01]|a[0-7])\\)$\0".as_ptr()
        as *const c_char;

// External constants supplied by translated headers.
extern "C" {
    static PERF_REGS_MASK: u64;
}

extern "C" {
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: usize,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
    fn regfree(preg: *mut regex_t);
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn zalloc(size: usize) -> *mut c_void;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn pr_debug4(fmt: *const c_char, ...);
}

#[repr(C)]
pub struct regex_t {
    _private: [c_ulong; 8],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmatch_t {
    pub rm_so: c_int,
    pub rm_eo: c_int,
}

const REG_EXTENDED: c_int = 1;
const ENOMEM: c_int = 12;
const SDT_ARG_SKIP: c_int = 0;
const SDT_ARG_VALID: c_int = 1;

extern "C" {
    static PERF_REG_RISCV_PC: c_int;
    static PERF_REG_RISCV_RA: c_int;
    static PERF_REG_RISCV_SP: c_int;
    static PERF_REG_RISCV_GP: c_int;
    static PERF_REG_RISCV_TP: c_int;
    static PERF_REG_RISCV_T0: c_int;
    static PERF_REG_RISCV_T1: c_int;
    static PERF_REG_RISCV_T2: c_int;
    static PERF_REG_RISCV_S0: c_int;
    static PERF_REG_RISCV_S1: c_int;
    static PERF_REG_RISCV_A0: c_int;
    static PERF_REG_RISCV_A1: c_int;
    static PERF_REG_RISCV_A2: c_int;
    static PERF_REG_RISCV_A3: c_int;
    static PERF_REG_RISCV_A4: c_int;
    static PERF_REG_RISCV_A5: c_int;
    static PERF_REG_RISCV_A6: c_int;
    static PERF_REG_RISCV_A7: c_int;
    static PERF_REG_RISCV_S2: c_int;
    static PERF_REG_RISCV_S3: c_int;
    static PERF_REG_RISCV_S4: c_int;
    static PERF_REG_RISCV_S5: c_int;
    static PERF_REG_RISCV_S6: c_int;
    static PERF_REG_RISCV_S7: c_int;
    static PERF_REG_RISCV_S8: c_int;
    static PERF_REG_RISCV_S9: c_int;
    static PERF_REG_RISCV_S10: c_int;
    static PERF_REG_RISCV_S11: c_int;
    static PERF_REG_RISCV_T3: c_int;
    static PERF_REG_RISCV_T4: c_int;
    static PERF_REG_RISCV_T5: c_int;
    static PERF_REG_RISCV_T6: c_int;
}

static mut SDT_OP_REGEX1_COMPILED: regex_t = regex_t { _private: [0; 8] };
static mut SDT_OP_REGEX2_COMPILED: regex_t = regex_t { _private: [0; 8] };

unsafe fn sdt_init_op_regex() -> c_int {
    static mut INITIALIZED: c_int = 0;
    let mut ret: c_int = 0;

    if INITIALIZED != 0 {
        return 0;
    }

    ret = regcomp(&mut SDT_OP_REGEX1_COMPILED, SDT_OP_REGEX1, REG_EXTENDED);
    if ret != 0 {
        pr_debug4(b"Regex compilation error.\n\0".as_ptr() as *const c_char);
        return -ret;
    }

    ret = regcomp(&mut SDT_OP_REGEX2_COMPILED, SDT_OP_REGEX2, REG_EXTENDED);
    if ret != 0 {
        regfree(&mut SDT_OP_REGEX1_COMPILED);
        pr_debug4(b"Regex compilation error.\n\0".as_ptr() as *const c_char);
        return -ret;
    }

    INITIALIZED = 1;
    0
}

/*
 * Parse OP and convert it into uprobe format.
 * Possible variants of OP (RISC-V, GCC 'nor' constraint):
 *
 *   Format         Example       Uprobe
 *   ----------------------------------------
 *   REG            a0            %a0
 *   NUM(REG)       8(a0)         +8(%a0)
 *   -NUM(REG)      -20(s0)       -20(%s0)
 *   NUM            99            (skip, constant not supported)
 */
#[no_mangle]
pub unsafe extern "C" fn __perf_sdt_arg_parse_op_riscv(
    old_op: *mut c_char,
    new_op: *mut *mut c_char,
) -> c_int {
    let mut ret: c_int;
    let mut new_len: c_int;
    let mut rm: [regmatch_t; 4] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 4];
    let prefix: c_char;

    /*
     * Constant argument: pure integer with no trailing '(' (e.g. "99", "-1").
     * uprobe does not support immediate values, so skip them.
     * Memory operands like "8(a0)" or "-20(s0)" contain '(' so are NOT
     * treated as constants here; they will be matched by REGEX2 below.
     */
    if strchr(old_op, '(' as c_int).is_null()
        && ((*old_op >= b'0' as c_char && *old_op <= b'9' as c_char)
            || (*old_op == b'-' as c_char
                && *old_op.add(1) >= b'0' as c_char
                && *old_op.add(1) <= b'9' as c_char))
    {
        pr_debug4(
            b"Skipping unsupported SDT argument: %s\n\0".as_ptr() as *const c_char,
            old_op,
        );
        return SDT_ARG_SKIP;
    }

    ret = sdt_init_op_regex();
    if ret < 0 {
        return ret;
    }

    if regexec(&SDT_OP_REGEX1_COMPILED, old_op, 2, rm.as_mut_ptr(), 0) == 0 {
        /* REG --> %REG */
        new_len = 2; /* % NULL */
        new_len += rm[1].rm_eo - rm[1].rm_so;

        *new_op = zalloc(new_len as usize) as *mut c_char;
        if (*new_op).is_null() {
            return -ENOMEM;
        }

        scnprintf(
            *new_op,
            new_len as usize,
            b"%%%.*s\0".as_ptr() as *const c_char,
            rm[1].rm_eo - rm[1].rm_so,
            old_op.offset(rm[1].rm_so as isize),
        );
    } else if regexec(&SDT_OP_REGEX2_COMPILED, old_op, 4, rm.as_mut_ptr(), 0) == 0 {
        /*
         * NUM(REG) or -NUM(REG) --> +NUM(%REG) or -NUM(%REG)
         * rm[1]: optional '-'
         * rm[2]: decimal offset
         * rm[3]: register name
         */
        prefix = if rm[1].rm_so == -1 { b'+' as c_char } else { b'-' as c_char };

        new_len = 5; /* sign ( % ) NULL */
        new_len += rm[2].rm_eo - rm[2].rm_so;
        new_len += rm[3].rm_eo - rm[3].rm_so;

        *new_op = zalloc(new_len as usize) as *mut c_char;
        if (*new_op).is_null() {
            return -ENOMEM;
        }

        scnprintf(
            *new_op,
            new_len as usize,
            b"%c%.*s(%%%.*s)\0".as_ptr() as *const c_char,
            prefix as c_int,
            rm[2].rm_eo - rm[2].rm_so,
            old_op.offset(rm[2].rm_so as isize),
            rm[3].rm_eo - rm[3].rm_so,
            old_op.offset(rm[3].rm_so as isize),
        );
    } else {
        pr_debug4(
            b"Skipping unsupported SDT argument: %s\n\0".as_ptr() as *const c_char,
            old_op,
        );
        return SDT_ARG_SKIP;
    }

    SDT_ARG_VALID
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_mask_riscv(_intr: bool) -> u64 {
    PERF_REGS_MASK
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_name_riscv(id: c_int) -> *const c_char {
    if id == PERF_REG_RISCV_PC {
        return b"pc\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_RA {
        return b"ra\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_SP {
        return b"sp\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_GP {
        return b"gp\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_TP {
        return b"tp\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_T0 {
        return b"t0\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_T1 {
        return b"t1\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_T2 {
        return b"t2\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_S0 {
        return b"s0\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_S1 {
        return b"s1\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_A0 {
        return b"a0\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_A1 {
        return b"a1\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_A2 {
        return b"a2\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_A3 {
        return b"a3\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_A4 {
        return b"a4\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_A5 {
        return b"a5\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_A6 {
        return b"a6\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_A7 {
        return b"a7\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_S2 {
        return b"s2\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_S3 {
        return b"s3\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_S4 {
        return b"s4\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_S5 {
        return b"s5\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_S6 {
        return b"s6\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_S7 {
        return b"s7\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_S8 {
        return b"s8\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_S9 {
        return b"s9\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_S10 {
        return b"s10\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_S11 {
        return b"s11\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_T3 {
        return b"t3\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_T4 {
        return b"t4\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_T5 {
        return b"t5\0".as_ptr() as *const c_char;
    }
    if id == PERF_REG_RISCV_T6 {
        return b"t6\0".as_ptr() as *const c_char;
    }

    ptr::null()
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_ip_riscv() -> u64 {
    PERF_REG_RISCV_PC as u64
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_sp_riscv() -> u64 {
    PERF_REG_RISCV_SP as u64
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
