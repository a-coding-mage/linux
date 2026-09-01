// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2019 Hangzhou C-SKY Microsystems co.,ltd.
// Mapping of DWARF debug register numbers into register names.

use core::ffi::{c_char, c_int, c_uint};

// Original C dependencies:
//   <errno.h>
//   <stddef.h>
//   <dwarf-regs.h>
//   "../../../arch/csky/include/uapi/asm/perf_regs.h"
// The C source forces __CSKYABIV2__ before including perf_regs.h so that the
// V2 perf register definitions are visible.

unsafe extern "C" {
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
}

const ENOENT: c_int = 2;

const CSKY_ABIV2_MAX_REGS: usize = 73;
static csky_dwarf_regs_table_abiv2: [Option<&'static [u8]>; CSKY_ABIV2_MAX_REGS] = [
    /* r0 ~ r8 */
    Some(b"%a0\0"),
    Some(b"%a1\0"),
    Some(b"%a2\0"),
    Some(b"%a3\0"),
    Some(b"%regs0\0"),
    Some(b"%regs1\0"),
    Some(b"%regs2\0"),
    Some(b"%regs3\0"),
    /* r9 ~ r15 */
    Some(b"%regs4\0"),
    Some(b"%regs5\0"),
    Some(b"%regs6\0"),
    Some(b"%regs7\0"),
    Some(b"%regs8\0"),
    Some(b"%regs9\0"),
    Some(b"%sp\0"),
    Some(b"%lr\0"),
    /* r16 ~ r23 */
    Some(b"%exregs0\0"),
    Some(b"%exregs1\0"),
    Some(b"%exregs2\0"),
    Some(b"%exregs3\0"),
    Some(b"%exregs4\0"),
    Some(b"%exregs5\0"),
    Some(b"%exregs6\0"),
    Some(b"%exregs7\0"),
    /* r24 ~ r31 */
    Some(b"%exregs8\0"),
    Some(b"%exregs9\0"),
    Some(b"%exregs10\0"),
    Some(b"%exregs11\0"),
    Some(b"%exregs12\0"),
    Some(b"%exregs13\0"),
    Some(b"%exregs14\0"),
    Some(b"%tls\0"),
    Some(b"%pc\0"),
    None,
    None,
    None,
    Some(b"%hi\0"),
    Some(b"%lo\0"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(b"%epc\0"),
];

const CSKY_ABIV1_MAX_REGS: usize = 57;
static csky_dwarf_regs_table_abiv1: [Option<&'static [u8]>; CSKY_ABIV1_MAX_REGS] = [
    /* r0 ~ r8 */
    Some(b"%sp\0"),
    Some(b"%regs9\0"),
    Some(b"%a0\0"),
    Some(b"%a1\0"),
    Some(b"%a2\0"),
    Some(b"%a3\0"),
    Some(b"%regs0\0"),
    Some(b"%regs1\0"),
    /* r9 ~ r15 */
    Some(b"%regs2\0"),
    Some(b"%regs3\0"),
    Some(b"%regs4\0"),
    Some(b"%regs5\0"),
    Some(b"%regs6\0"),
    Some(b"%regs7\0"),
    Some(b"%regs8\0"),
    Some(b"%lr\0"),
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    None,
    Some(b"%epc\0"),
];

pub unsafe extern "C" fn __get_csky_regstr(n: c_uint, flags: c_uint) -> *const c_char {
    if flags & EF_CSKY_ABIV2 != 0 {
        return if (n as usize) < CSKY_ABIV2_MAX_REGS {
            csky_dwarf_regs_table_abiv2[n as usize]
                .map_or(core::ptr::null(), |s| s.as_ptr() as *const c_char)
        } else {
            core::ptr::null()
        };
    }

    if (n as usize) < CSKY_ABIV1_MAX_REGS {
        csky_dwarf_regs_table_abiv1[n as usize]
            .map_or(core::ptr::null(), |s| s.as_ptr() as *const c_char)
    } else {
        core::ptr::null()
    }
}

unsafe fn __get_dwarf_regnum(
    regstr: &[Option<&'static [u8]>],
    num_regstr: usize,
    name: *const c_char,
) -> c_int {
    for i in 0..num_regstr {
        if let Some(s) = regstr[i] {
            if strcmp(s.as_ptr() as *const c_char, name) == 0 {
                return i as c_int;
            }
        }
    }
    -ENOENT
}

pub unsafe extern "C" fn __get_csky_regnum(name: *const c_char, flags: c_uint) -> c_int {
    if flags & EF_CSKY_ABIV2 != 0 {
        return __get_dwarf_regnum(
            &csky_dwarf_regs_table_abiv2,
            CSKY_ABIV2_MAX_REGS,
            name,
        );
    }

    __get_dwarf_regnum(
        &csky_dwarf_regs_table_abiv1,
        CSKY_ABIV1_MAX_REGS,
        name,
    )
}

pub unsafe extern "C" fn __get_dwarf_regnum_for_perf_regnum_csky(
    perf_regnum: c_int,
    flags: c_uint,
) -> c_int {
    let mut idx = 0usize;

    if flags & EF_CSKY_ABIV2 != 0 {
        idx += 1;
    }

    let pair = if perf_regnum == PERF_REG_CSKY_TLS {
        [-ENOENT, 31]
    } else if perf_regnum == PERF_REG_CSKY_LR {
        [15, 15]
    } else if perf_regnum == PERF_REG_CSKY_PC {
        [-ENOENT, 32]
    /* TODO: PERF_REG_CSKY_SR */
    } else if perf_regnum == PERF_REG_CSKY_SP {
        [0, 14]
    /* TODO: PERF_REG_CSKY_ORIG_A0 */
    } else if perf_regnum == PERF_REG_CSKY_A0 {
        [2, 0]
    } else if perf_regnum == PERF_REG_CSKY_A1 {
        [3, 1]
    } else if perf_regnum == PERF_REG_CSKY_A2 {
        [4, 2]
    } else if perf_regnum == PERF_REG_CSKY_A3 {
        [5, 3]
    } else if perf_regnum == PERF_REG_CSKY_REGS0 {
        [6, 4]
    } else if perf_regnum == PERF_REG_CSKY_REGS1 {
        [7, 5]
    } else if perf_regnum == PERF_REG_CSKY_REGS2 {
        [8, 6]
    } else if perf_regnum == PERF_REG_CSKY_REGS3 {
        [9, 7]
    } else if perf_regnum == PERF_REG_CSKY_REGS4 {
        [10, 8]
    } else if perf_regnum == PERF_REG_CSKY_REGS5 {
        [11, 9]
    } else if perf_regnum == PERF_REG_CSKY_REGS6 {
        [12, 10]
    } else if perf_regnum == PERF_REG_CSKY_REGS7 {
        [13, 11]
    } else if perf_regnum == PERF_REG_CSKY_REGS8 {
        [14, 12]
    } else if perf_regnum == PERF_REG_CSKY_REGS9 {
        [1, 13]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS0 {
        [-ENOENT, 16]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS1 {
        [-ENOENT, 17]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS2 {
        [-ENOENT, 18]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS3 {
        [-ENOENT, 19]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS4 {
        [-ENOENT, 20]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS5 {
        [-ENOENT, 21]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS6 {
        [-ENOENT, 22]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS7 {
        [-ENOENT, 23]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS8 {
        [-ENOENT, 24]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS9 {
        [-ENOENT, 25]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS10 {
        [-ENOENT, 26]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS11 {
        [-ENOENT, 27]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS12 {
        [-ENOENT, 28]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS13 {
        [-ENOENT, 29]
    } else if perf_regnum == PERF_REG_CSKY_EXREGS14 {
        [-ENOENT, 30]
    /* TODO: PERF_REG_CSKY_HI */
    /* TODO: PERF_REG_CSKY_LO */
    /* TODO: PERF_REG_CSKY_DCSR */
    } else {
        [0, 0]
    };

    if perf_regnum < 0 || pair[idx] == 0 {
        return -ENOENT;
    }

    pair[idx]
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
