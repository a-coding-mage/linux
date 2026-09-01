// SPDX-License-Identifier: GPL-2.0
//
// Translated from C source:
//   perf/arch/x86/tests/insn-x86.c
//
// Original C dependencies:
//   <linux/types.h>
//   <string.h>
//   "debug.h"
//   "tests/tests.h"
//   "arch-tests.h"
//   "../../../../arch/x86/include/asm/insn.h"
//   "intel-pt-decoder/intel-pt-insn-decoder.h"

use core::ffi::{c_char, c_int};

// From the x86 instruction decoder dependency: maximum x86 instruction size.
const MAX_INSN_SIZE: usize = 15;

extern "C" {
    static INTEL_PT_OP_OTHER: c_int;
    static INTEL_PT_OP_CALL: c_int;
    static INTEL_PT_OP_RET: c_int;
    static INTEL_PT_OP_JCC: c_int;
    static INTEL_PT_OP_JMP: c_int;
    static INTEL_PT_OP_LOOP: c_int;
    static INTEL_PT_OP_IRET: c_int;
    static INTEL_PT_OP_INT: c_int;
    static INTEL_PT_OP_SYSCALL: c_int;
    static INTEL_PT_OP_SYSRET: c_int;
    static INTEL_PT_OP_VMENTRY: c_int;
    static INTEL_PT_OP_ERETS: c_int;
    static INTEL_PT_OP_ERETU: c_int;

    static INTEL_PT_BR_NO_BRANCH: c_int;
    static INTEL_PT_BR_INDIRECT: c_int;
    static INTEL_PT_BR_CONDITIONAL: c_int;
    static INTEL_PT_BR_UNCONDITIONAL: c_int;

    static INSN_MODE_64: c_int;
    static INSN_MODE_32: c_int;

    fn strlen(s: *const c_char) -> usize;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn pr_debug(fmt: *const c_char, ...);
    fn insn_decode(insn: *mut insn, data: *const u8, buf_len: usize, mode: c_int) -> c_int;
    fn intel_pt_get_insn(
        buf: *const u8,
        len: usize,
        x86_64: c_int,
        intel_pt_insn: *mut intel_pt_insn,
    ) -> c_int;
}

#[repr(C)]
pub struct test_suite {
    _private: [u8; 0],
}

#[repr(C)]
pub struct insn {
    pub length: c_int,
}

#[repr(C)]
pub struct intel_pt_insn {
    pub op: c_int,
    pub branch: c_int,
    pub rel: c_int,
}

#[repr(C)]
struct test_data {
    data: [u8; MAX_INSN_SIZE],
    expected_length: c_int,
    expected_rel: c_int,
    expected_op_str: *const c_char,
    expected_branch_str: *const c_char,
    asm_rep: *const c_char,
}

unsafe impl Sync for test_data {}

// Original C included generated entries from "insn-x86-dat-32.c" here.
// Those generated entries are external to this isolated source file.
const TEST_DATA_32: &[test_data] = &[
    test_data {
        data: {
            let mut data = [0u8; MAX_INSN_SIZE];
            data[0] = 0x0f;
            data[1] = 0x01;
            data[2] = 0xee;
            data
        },
        expected_length: 3,
        expected_rel: 0,
        expected_op_str: core::ptr::null(),
        expected_branch_str: core::ptr::null(),
        asm_rep: c"0f 01 ee             \trdpkru".as_ptr(),
    },
    test_data {
        data: {
            let mut data = [0u8; MAX_INSN_SIZE];
            data[0] = 0x0f;
            data[1] = 0x01;
            data[2] = 0xef;
            data
        },
        expected_length: 3,
        expected_rel: 0,
        expected_op_str: core::ptr::null(),
        expected_branch_str: core::ptr::null(),
        asm_rep: c"0f 01 ef             \twrpkru".as_ptr(),
    },
    test_data {
        data: [0u8; MAX_INSN_SIZE],
        expected_length: 0,
        expected_rel: 0,
        expected_op_str: core::ptr::null(),
        expected_branch_str: core::ptr::null(),
        asm_rep: core::ptr::null(),
    },
];

// Original C included generated entries from "insn-x86-dat-64.c" here.
// Those generated entries are external to this isolated source file.
const TEST_DATA_64: &[test_data] = &[
    test_data {
        data: {
            let mut data = [0u8; MAX_INSN_SIZE];
            data[0] = 0x0f;
            data[1] = 0x01;
            data[2] = 0xee;
            data
        },
        expected_length: 3,
        expected_rel: 0,
        expected_op_str: core::ptr::null(),
        expected_branch_str: core::ptr::null(),
        asm_rep: c"0f 01 ee             \trdpkru".as_ptr(),
    },
    test_data {
        data: {
            let mut data = [0u8; MAX_INSN_SIZE];
            data[0] = 0x0f;
            data[1] = 0x01;
            data[2] = 0xef;
            data
        },
        expected_length: 3,
        expected_rel: 0,
        expected_op_str: core::ptr::null(),
        expected_branch_str: core::ptr::null(),
        asm_rep: c"0f 01 ef             \twrpkru".as_ptr(),
    },
    test_data {
        data: {
            let mut data = [0u8; MAX_INSN_SIZE];
            data[0] = 0xf2;
            data[1] = 0x0f;
            data[2] = 0x01;
            data[3] = 0xca;
            data
        },
        expected_length: 4,
        expected_rel: 0,
        expected_op_str: c"erets".as_ptr(),
        expected_branch_str: c"indirect".as_ptr(),
        asm_rep: c"f2 0f 01 ca  \terets".as_ptr(),
    },
    test_data {
        data: {
            let mut data = [0u8; MAX_INSN_SIZE];
            data[0] = 0xf3;
            data[1] = 0x0f;
            data[2] = 0x01;
            data[3] = 0xca;
            data
        },
        expected_length: 4,
        expected_rel: 0,
        expected_op_str: c"eretu".as_ptr(),
        expected_branch_str: c"indirect".as_ptr(),
        asm_rep: c"f3 0f 01 ca  \teretu".as_ptr(),
    },
    test_data {
        data: [0u8; MAX_INSN_SIZE],
        expected_length: 0,
        expected_rel: 0,
        expected_op_str: core::ptr::null(),
        expected_branch_str: core::ptr::null(),
        asm_rep: core::ptr::null(),
    },
];

unsafe fn get_op(op_str: *const c_char) -> c_int {
    #[repr(C)]
    struct val_data {
        name: *const c_char,
        val: c_int,
    }

    let vals = [
        val_data {
            name: c"other".as_ptr(),
            val: INTEL_PT_OP_OTHER,
        },
        val_data {
            name: c"call".as_ptr(),
            val: INTEL_PT_OP_CALL,
        },
        val_data {
            name: c"ret".as_ptr(),
            val: INTEL_PT_OP_RET,
        },
        val_data {
            name: c"jcc".as_ptr(),
            val: INTEL_PT_OP_JCC,
        },
        val_data {
            name: c"jmp".as_ptr(),
            val: INTEL_PT_OP_JMP,
        },
        val_data {
            name: c"loop".as_ptr(),
            val: INTEL_PT_OP_LOOP,
        },
        val_data {
            name: c"iret".as_ptr(),
            val: INTEL_PT_OP_IRET,
        },
        val_data {
            name: c"int".as_ptr(),
            val: INTEL_PT_OP_INT,
        },
        val_data {
            name: c"syscall".as_ptr(),
            val: INTEL_PT_OP_SYSCALL,
        },
        val_data {
            name: c"sysret".as_ptr(),
            val: INTEL_PT_OP_SYSRET,
        },
        val_data {
            name: c"vmentry".as_ptr(),
            val: INTEL_PT_OP_VMENTRY,
        },
        val_data {
            name: c"erets".as_ptr(),
            val: INTEL_PT_OP_ERETS,
        },
        val_data {
            name: c"eretu".as_ptr(),
            val: INTEL_PT_OP_ERETU,
        },
        val_data {
            name: core::ptr::null(),
            val: 0,
        },
    ];

    if op_str.is_null() || strlen(op_str) == 0 {
        return 0;
    }

    let mut i = 0usize;
    while !vals[i].name.is_null() {
        if strcmp(vals[i].name, op_str) == 0 {
            return vals[i].val;
        }
        i += 1;
    }

    pr_debug(c"Failed to get op\n".as_ptr());

    -1
}

unsafe fn get_branch(branch_str: *const c_char) -> c_int {
    #[repr(C)]
    struct val_data {
        name: *const c_char,
        val: c_int,
    }

    let vals = [
        val_data {
            name: c"no_branch".as_ptr(),
            val: INTEL_PT_BR_NO_BRANCH,
        },
        val_data {
            name: c"indirect".as_ptr(),
            val: INTEL_PT_BR_INDIRECT,
        },
        val_data {
            name: c"conditional".as_ptr(),
            val: INTEL_PT_BR_CONDITIONAL,
        },
        val_data {
            name: c"unconditional".as_ptr(),
            val: INTEL_PT_BR_UNCONDITIONAL,
        },
        val_data {
            name: core::ptr::null(),
            val: 0,
        },
    ];

    if branch_str.is_null() || strlen(branch_str) == 0 {
        return 0;
    }

    let mut i = 0usize;
    while !vals[i].name.is_null() {
        if strcmp(vals[i].name, branch_str) == 0 {
            return vals[i].val;
        }
        i += 1;
    }

    pr_debug(c"Failed to get branch\n".as_ptr());

    -1
}

unsafe fn test_data_item(dat: *const test_data, x86_64: c_int) -> c_int {
    let mut intel_pt_insn = core::mem::MaybeUninit::<intel_pt_insn>::uninit();
    let mut op: c_int;
    let mut branch: c_int;
    let mut ret: c_int;
    let mut insn = core::mem::MaybeUninit::<insn>::uninit();

    ret = insn_decode(
        insn.as_mut_ptr(),
        (*dat).data.as_ptr(),
        MAX_INSN_SIZE,
        if x86_64 != 0 {
            INSN_MODE_64
        } else {
            INSN_MODE_32
        },
    );
    if ret < 0 {
        pr_debug(c"Failed to decode: %s\n".as_ptr(), (*dat).asm_rep);
        return -1;
    }

    let insn = insn.assume_init();
    if insn.length != (*dat).expected_length {
        pr_debug(
            c"Failed to decode length (%d vs expected %d): %s\n".as_ptr(),
            insn.length,
            (*dat).expected_length,
            (*dat).asm_rep,
        );
        return -1;
    }

    op = get_op((*dat).expected_op_str);
    branch = get_branch((*dat).expected_branch_str);

    if intel_pt_get_insn(
        (*dat).data.as_ptr(),
        MAX_INSN_SIZE,
        x86_64,
        intel_pt_insn.as_mut_ptr(),
    ) != 0
    {
        pr_debug(c"Intel PT failed to decode: %s\n".as_ptr(), (*dat).asm_rep);
        return -1;
    }

    let intel_pt_insn = intel_pt_insn.assume_init();
    if intel_pt_insn.op as c_int != op {
        pr_debug(
            c"Failed to decode 'op' value (%d vs expected %d): %s\n".as_ptr(),
            intel_pt_insn.op,
            op,
            (*dat).asm_rep,
        );
        return -1;
    }

    if intel_pt_insn.branch as c_int != branch {
        pr_debug(
            c"Failed to decode 'branch' value (%d vs expected %d): %s\n".as_ptr(),
            intel_pt_insn.branch,
            branch,
            (*dat).asm_rep,
        );
        return -1;
    }

    if intel_pt_insn.rel != (*dat).expected_rel {
        pr_debug(
            c"Failed to decode 'rel' value (%#x vs expected %#x): %s\n".as_ptr(),
            intel_pt_insn.rel,
            (*dat).expected_rel,
            (*dat).asm_rep,
        );
        return -1;
    }

    pr_debug(c"Decoded ok: %s\n".as_ptr(), (*dat).asm_rep);

    0
}

unsafe fn test_data_set(dat_set: *const test_data, x86_64: c_int) -> c_int {
    let mut dat: *const test_data;
    let mut ret: c_int = 0;

    dat = dat_set;
    while (*dat).expected_length != 0 {
        if test_data_item(dat, x86_64) != 0 {
            ret = -1;
        }
        dat = dat.add(1);
    }

    ret
}

/**
 * test__insn_x86 - test x86 instruction decoder - new instructions.
 *
 * This function implements a test that decodes a selection of instructions and
 * checks the results.  The Intel PT function that further categorizes
 * instructions (i.e. intel_pt_get_insn()) is also checked.
 *
 * The instructions are originally in insn-x86-dat-src.c which has been
 * processed by scripts gen-insn-x86-dat.sh and gen-insn-x86-dat.awk to produce
 * insn-x86-dat-32.c and insn-x86-dat-64.c which are included into this program.
 * i.e. to add new instructions to the test, edit insn-x86-dat-src.c, run the
 * gen-insn-x86-dat.sh script, make perf, and then run the test.
 *
 * If the test passes %0 is returned, otherwise %-1 is returned.  Use the
 * verbose (-v) option to see all the instructions and whether or not they
 * decoded successfully.
 */
#[no_mangle]
pub unsafe extern "C" fn test__insn_x86(
    _test: *mut test_suite,
    _subtest: c_int,
) -> c_int {
    let mut ret: c_int = 0;

    if test_data_set(TEST_DATA_32.as_ptr(), 0) != 0 {
        ret = -1;
    }

    if test_data_set(TEST_DATA_64.as_ptr(), 1) != 0 {
        ret = -1;
    }

    ret
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
