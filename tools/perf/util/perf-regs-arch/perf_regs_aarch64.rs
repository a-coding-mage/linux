// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_uint, c_ulong};

// Dependencies from the original C includes:
// errno.h, regex.h, string.h, sys/auxv.h, linux/kernel.h, linux/zalloc.h,
// ../debug.h, ../event.h, ../perf_regs.h, ../../perf-sys.h,
// ../../arch/arm64/include/perf_regs.h

#[repr(C)]
pub struct regex_t {
    _private: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct regmatch_t {
    pub rm_so: c_int,
    pub rm_eo: c_int,
}

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub config: u64,
    pub sample_type: u64,
    pub disabled: u64,
    pub exclude_kernel: u64,
    pub sample_period: u64,
    pub sample_regs_user: u64,
}

const ENOMEM: c_int = 12;
const REG_EXTENDED: c_int = 1;
const AT_HWCAP: c_ulong = 16;
const HWCAP_SVE: c_ulong = 1 << 22;

const PERF_TYPE_HARDWARE: u32 = 0;
const PERF_COUNT_HW_CPU_CYCLES: u64 = 0;
const PERF_SAMPLE_REGS_USER: u64 = 1 << 12;

const SDT_ARG_VALID: c_int = 0;
const SDT_ARG_SKIP: c_int = 1;

const PERF_REG_ARM64_X0: c_int = 0;
const PERF_REG_ARM64_X1: c_int = 1;
const PERF_REG_ARM64_X2: c_int = 2;
const PERF_REG_ARM64_X3: c_int = 3;
const PERF_REG_ARM64_X4: c_int = 4;
const PERF_REG_ARM64_X5: c_int = 5;
const PERF_REG_ARM64_X6: c_int = 6;
const PERF_REG_ARM64_X7: c_int = 7;
const PERF_REG_ARM64_X8: c_int = 8;
const PERF_REG_ARM64_X9: c_int = 9;
const PERF_REG_ARM64_X10: c_int = 10;
const PERF_REG_ARM64_X11: c_int = 11;
const PERF_REG_ARM64_X12: c_int = 12;
const PERF_REG_ARM64_X13: c_int = 13;
const PERF_REG_ARM64_X14: c_int = 14;
const PERF_REG_ARM64_X15: c_int = 15;
const PERF_REG_ARM64_X16: c_int = 16;
const PERF_REG_ARM64_X17: c_int = 17;
const PERF_REG_ARM64_X18: c_int = 18;
const PERF_REG_ARM64_X19: c_int = 19;
const PERF_REG_ARM64_X20: c_int = 20;
const PERF_REG_ARM64_X21: c_int = 21;
const PERF_REG_ARM64_X22: c_int = 22;
const PERF_REG_ARM64_X23: c_int = 23;
const PERF_REG_ARM64_X24: c_int = 24;
const PERF_REG_ARM64_X25: c_int = 25;
const PERF_REG_ARM64_X26: c_int = 26;
const PERF_REG_ARM64_X27: c_int = 27;
const PERF_REG_ARM64_X28: c_int = 28;
const PERF_REG_ARM64_X29: c_int = 29;
const PERF_REG_ARM64_LR: c_int = 30;
const PERF_REG_ARM64_SP: c_int = 31;
const PERF_REG_ARM64_PC: c_int = 32;
const PERF_REG_ARM64_VG: c_int = 46;

const PERF_REGS_MASK: u64 = u64::MAX;

const fn SMPL_REG_MASK(b: c_int) -> u64 {
    1_u64 << (b as u32)
}

/* %xNUM */
const SDT_OP_REGEX1: &[u8] = b"^(x[1-2]?[0-9]|3[0-1])$\0";

/* [sp], [sp, NUM] */
const SDT_OP_REGEX2: &[u8] = b"^\\[sp(, )?([0-9]+)?\\]$\0";

static mut sdt_op_regex1: regex_t = regex_t { _private: [] };
static mut sdt_op_regex2: regex_t = regex_t { _private: [] };

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
    fn zalloc(size: usize) -> *mut c_char;
    fn scnprintf(buf: *mut c_char, size: usize, fmt: *const c_char, ...) -> c_int;
    fn pr_debug4(fmt: *const c_char, ...);
    fn getauxval(type_: c_ulong) -> c_ulong;
    fn event_attr_init(attr: *mut perf_event_attr);
    fn sys_perf_event_open(
        attr: *mut perf_event_attr,
        pid: c_int,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulong,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
}

unsafe fn sdt_init_op_regex() -> c_int {
    static mut initialized: c_int = 0;
    let mut ret: c_int = 0;

    if initialized != 0 {
        return 0;
    }

    ret = regcomp(
        &mut sdt_op_regex1,
        SDT_OP_REGEX1.as_ptr() as *const c_char,
        REG_EXTENDED,
    );
    if ret != 0 {
        pr_debug4(b"Regex compilation error.\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ret = regcomp(
        &mut sdt_op_regex2,
        SDT_OP_REGEX2.as_ptr() as *const c_char,
        REG_EXTENDED,
    );
    if ret != 0 {
        regfree(&mut sdt_op_regex1);
        pr_debug4(b"Regex compilation error.\n\0".as_ptr() as *const c_char);
        return ret;
    }

    initialized = 1;
    0
}

/*
 * SDT marker arguments on Arm64 uses %xREG or [sp, NUM], currently
 * support these two formats.
 */
#[no_mangle]
pub unsafe extern "C" fn __perf_sdt_arg_parse_op_arm64(
    old_op: *mut c_char,
    new_op: *mut *mut c_char,
) -> c_int {
    let mut ret: c_int;
    let mut new_len: c_int;
    let mut rm = [regmatch_t { rm_so: 0, rm_eo: 0 }; 5];

    ret = sdt_init_op_regex();
    if ret < 0 {
        return ret;
    }

    if regexec(&sdt_op_regex1, old_op, 3, rm.as_mut_ptr(), 0) == 0 {
        /* Extract xNUM */
        new_len = 2; /* % NULL */
        new_len += (rm[1].rm_eo - rm[1].rm_so) as c_int;

        *new_op = zalloc(new_len as usize);
        if (*new_op).is_null() {
            return -ENOMEM;
        }

        scnprintf(
            *new_op,
            new_len as usize,
            b"%%%.*s\0".as_ptr() as *const c_char,
            (rm[1].rm_eo - rm[1].rm_so) as c_int,
            old_op.offset(rm[1].rm_so as isize),
        );
    } else if regexec(&sdt_op_regex2, old_op, 5, rm.as_mut_ptr(), 0) == 0 {
        /* [sp], [sp, NUM] or [sp,NUM] */
        new_len = 7; /* + ( % s p ) NULL */

        /* If the argument is [sp], need to fill offset '0' */
        if rm[2].rm_so == -1 {
            new_len += 1;
        } else {
            new_len += (rm[2].rm_eo - rm[2].rm_so) as c_int;
        }

        *new_op = zalloc(new_len as usize);
        if (*new_op).is_null() {
            return -ENOMEM;
        }

        if rm[2].rm_so == -1 {
            scnprintf(
                *new_op,
                new_len as usize,
                b"+0(%%sp)\0".as_ptr() as *const c_char,
            );
        } else {
            scnprintf(
                *new_op,
                new_len as usize,
                b"+%.*s(%%sp)\0".as_ptr() as *const c_char,
                (rm[2].rm_eo - rm[2].rm_so) as c_int,
                old_op.offset(rm[2].rm_so as isize),
            );
        }
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
pub unsafe extern "C" fn __perf_reg_mask_arm64(intr: bool) -> u64 {
    let mut attr = perf_event_attr {
        type_: PERF_TYPE_HARDWARE,
        config: PERF_COUNT_HW_CPU_CYCLES,
        sample_type: PERF_SAMPLE_REGS_USER,
        disabled: 1,
        exclude_kernel: 1,
        sample_period: 1,
        sample_regs_user: PERF_REGS_MASK,
    };
    let fd: c_int;

    if intr {
        return PERF_REGS_MASK;
    }

    if (getauxval(AT_HWCAP) & HWCAP_SVE) != 0 {
        attr.sample_regs_user |= SMPL_REG_MASK(PERF_REG_ARM64_VG);
    }

    /*
     * Check if the pmu supports perf extended regs, before
     * returning the register mask to sample. Open the event
     * on the perf process to check this.
     */
    if attr.sample_regs_user != PERF_REGS_MASK {
        event_attr_init(&mut attr);
        fd = sys_perf_event_open(
            &mut attr,
            0,  /*pid=*/
            -1, /*cpu=*/
            -1, /*group_fd=*/
            0,  /*flags=*/
        );
        if fd != -1 {
            close(fd);
            return attr.sample_regs_user;
        }
    }
    PERF_REGS_MASK
}

#[no_mangle]
pub extern "C" fn __perf_reg_name_arm64(id: c_int) -> *const c_char {
    match id {
        PERF_REG_ARM64_X0 => b"x0\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X1 => b"x1\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X2 => b"x2\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X3 => b"x3\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X4 => b"x4\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X5 => b"x5\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X6 => b"x6\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X7 => b"x7\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X8 => b"x8\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X9 => b"x9\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X10 => b"x10\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X11 => b"x11\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X12 => b"x12\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X13 => b"x13\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X14 => b"x14\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X15 => b"x15\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X16 => b"x16\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X17 => b"x17\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X18 => b"x18\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X19 => b"x19\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X20 => b"x20\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X21 => b"x21\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X22 => b"x22\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X23 => b"x23\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X24 => b"x24\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X25 => b"x25\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X26 => b"x26\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X27 => b"x27\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X28 => b"x28\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_X29 => b"x29\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_SP => b"sp\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_LR => b"lr\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_PC => b"pc\0".as_ptr() as *const c_char,
        PERF_REG_ARM64_VG => b"vg\0".as_ptr() as *const c_char,
        _ => core::ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn __perf_reg_ip_arm64() -> u64 {
    PERF_REG_ARM64_PC as u64
}

#[no_mangle]
pub extern "C" fn __perf_reg_sp_arm64() -> u64 {
    PERF_REG_ARM64_SP as u64
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
