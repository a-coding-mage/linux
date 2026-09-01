// SPDX-License-Identifier: GPL-2.0

use core::ffi::{c_char, c_int, c_ulonglong, c_void};
use core::ptr;

use crate::{
    close, event_attr_init, perf_pmus__num_core_pmus, perf_pmus__scan_core, pr_debug4,
    scnprintf, sys_perf_event_open, zalloc, PERF_COUNT_HW_CPU_CYCLES, PERF_PMU_TYPE_SHIFT,
    PERF_REGS_MASK, PERF_REG_EXTENDED_MASK, PERF_REG_X86_AX, PERF_REG_X86_BP,
    PERF_REG_X86_BX, PERF_REG_X86_CS, PERF_REG_X86_CX, PERF_REG_X86_DI,
    PERF_REG_X86_DS, PERF_REG_X86_DX, PERF_REG_X86_ES, PERF_REG_X86_FLAGS,
    PERF_REG_X86_FS, PERF_REG_X86_GS, PERF_REG_X86_IP, PERF_REG_X86_R10,
    PERF_REG_X86_R11, PERF_REG_X86_R12, PERF_REG_X86_R13, PERF_REG_X86_R14,
    PERF_REG_X86_R15, PERF_REG_X86_R8, PERF_REG_X86_R9, PERF_REG_X86_SI,
    PERF_REG_X86_SP, PERF_REG_X86_SS, PERF_REG_X86_XMM0, PERF_REG_X86_XMM1,
    PERF_REG_X86_XMM10, PERF_REG_X86_XMM11, PERF_REG_X86_XMM12, PERF_REG_X86_XMM13,
    PERF_REG_X86_XMM14, PERF_REG_X86_XMM15, PERF_REG_X86_XMM2, PERF_REG_X86_XMM3,
    PERF_REG_X86_XMM4, PERF_REG_X86_XMM5, PERF_REG_X86_XMM6, PERF_REG_X86_XMM7,
    PERF_REG_X86_XMM8, PERF_REG_X86_XMM9, PERF_SAMPLE_REGS_INTR, PERF_TYPE_HARDWARE,
    PERF_TYPE_RAW, REG_EXTENDED, SDT_ARG_SKIP, SDT_ARG_VALID,
};
use crate::{perf_event_attr, perf_pmu, regex_t, regmatch_t};

const ENOMEM: c_int = 12;

unsafe extern "C" {
    fn regcomp(preg: *mut regex_t, regex: *const c_char, cflags: c_int) -> c_int;
    fn regexec(
        preg: *const regex_t,
        string: *const c_char,
        nmatch: usize,
        pmatch: *mut regmatch_t,
        eflags: c_int,
    ) -> c_int;
    fn strchr(s: *const c_char, c: c_int) -> *mut c_char;
    fn strcpy(dest: *mut c_char, src: *const c_char) -> *mut c_char;
    fn strncpy(dest: *mut c_char, src: *const c_char, n: usize) -> *mut c_char;
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: usize) -> c_int;
}

#[repr(C)]
struct sdt_name_reg {
    sdt_name: *const c_char,
    uprobe_name: *const c_char,
}

macro_rules! c_str {
    ($s:expr) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

macro_rules! SDT_NAME_REG {
    ($n:literal, $m:literal) => {
        sdt_name_reg {
            sdt_name: c_str!(concat!("%", $n)),
            uprobe_name: c_str!(concat!("%", $m)),
        }
    };
}

const SDT_REG_TBL: &[sdt_name_reg] = &[
    SDT_NAME_REG!("eax", "ax"),
    SDT_NAME_REG!("rax", "ax"),
    SDT_NAME_REG!("al", "ax"),
    SDT_NAME_REG!("ah", "ax"),
    SDT_NAME_REG!("ebx", "bx"),
    SDT_NAME_REG!("rbx", "bx"),
    SDT_NAME_REG!("bl", "bx"),
    SDT_NAME_REG!("bh", "bx"),
    SDT_NAME_REG!("ecx", "cx"),
    SDT_NAME_REG!("rcx", "cx"),
    SDT_NAME_REG!("cl", "cx"),
    SDT_NAME_REG!("ch", "cx"),
    SDT_NAME_REG!("edx", "dx"),
    SDT_NAME_REG!("rdx", "dx"),
    SDT_NAME_REG!("dl", "dx"),
    SDT_NAME_REG!("dh", "dx"),
    SDT_NAME_REG!("esi", "si"),
    SDT_NAME_REG!("rsi", "si"),
    SDT_NAME_REG!("sil", "si"),
    SDT_NAME_REG!("edi", "di"),
    SDT_NAME_REG!("rdi", "di"),
    SDT_NAME_REG!("dil", "di"),
    SDT_NAME_REG!("ebp", "bp"),
    SDT_NAME_REG!("rbp", "bp"),
    SDT_NAME_REG!("bpl", "bp"),
    SDT_NAME_REG!("rsp", "sp"),
    SDT_NAME_REG!("esp", "sp"),
    SDT_NAME_REG!("spl", "sp"),

    /* rNN registers */
    SDT_NAME_REG!("r8b", "r8"),
    SDT_NAME_REG!("r8w", "r8"),
    SDT_NAME_REG!("r8d", "r8"),
    SDT_NAME_REG!("r9b", "r9"),
    SDT_NAME_REG!("r9w", "r9"),
    SDT_NAME_REG!("r9d", "r9"),
    SDT_NAME_REG!("r10b", "r10"),
    SDT_NAME_REG!("r10w", "r10"),
    SDT_NAME_REG!("r10d", "r10"),
    SDT_NAME_REG!("r11b", "r11"),
    SDT_NAME_REG!("r11w", "r11"),
    SDT_NAME_REG!("r11d", "r11"),
    SDT_NAME_REG!("r12b", "r12"),
    SDT_NAME_REG!("r12w", "r12"),
    SDT_NAME_REG!("r12d", "r12"),
    SDT_NAME_REG!("r13b", "r13"),
    SDT_NAME_REG!("r13w", "r13"),
    SDT_NAME_REG!("r13d", "r13"),
    SDT_NAME_REG!("r14b", "r14"),
    SDT_NAME_REG!("r14w", "r14"),
    SDT_NAME_REG!("r14d", "r14"),
    SDT_NAME_REG!("r15b", "r15"),
    SDT_NAME_REG!("r15w", "r15"),
    SDT_NAME_REG!("r15d", "r15"),
    sdt_name_reg {
        sdt_name: ptr::null(),
        uprobe_name: ptr::null(),
    },
];

/*
 * Perf only supports OP which is in  +/-NUM(REG)  form.
 * Here plus-minus sign, NUM and parenthesis are optional,
 * only REG is mandatory.
 *
 * SDT events also supports indirect addressing mode with a
 * symbol as offset, scaled mode and constants in OP. But
 * perf does not support them yet. Below are few examples.
 *
 * OP with scaled mode:
 *     (%rax,%rsi,8)
 *     10(%ras,%rsi,8)
 *
 * OP with indirect addressing mode:
 *     check_action(%rip)
 *     mp_+52(%rip)
 *     44+mp_(%rip)
 *
 * OP with constant values:
 *     $0
 *     $123
 *     $-1
 */
const SDT_OP_REGEX: *const c_char =
    b"^([+\\-]?)([0-9]*)(\\(?)(%[a-z][a-z0-9]+)(\\)?)$\0".as_ptr() as *const c_char;

static mut SDT_OP_REGEX_COMPILED: regex_t = unsafe { core::mem::zeroed() };

unsafe fn sdt_init_op_regex() -> c_int {
    static mut INITIALIZED: c_int = 0;
    let mut ret: c_int = 0;

    if INITIALIZED != 0 {
        return 0;
    }

    ret = regcomp(&raw mut SDT_OP_REGEX_COMPILED, SDT_OP_REGEX, REG_EXTENDED);
    if ret < 0 {
        pr_debug4(c_str!("Regex compilation error.\n"));
        return ret;
    }

    INITIALIZED = 1;
    0
}

/*
 * Max x86 register name length is 5(ex: %r15d). So, 6th char
 * should always contain NULL. This helps to find register name
 * length using strlen, instead of maintaining one more variable.
 */
const SDT_REG_NAME_SIZE: usize = 6;

/*
 * The uprobe parser does not support all gas register names;
 * so, we have to replace them (ex. for x86_64: %rax -> %ax).
 * Note: If register does not require renaming, just copy
 * paste as it is, but don't leave it empty.
 */
unsafe fn sdt_rename_register(sdt_reg: *mut c_char, sdt_len: c_int, uprobe_reg: *mut c_char) {
    let mut i: usize = 0;

    while !SDT_REG_TBL[i].sdt_name.is_null() {
        if strncmp(SDT_REG_TBL[i].sdt_name, sdt_reg, sdt_len as usize) == 0 {
            strcpy(uprobe_reg, SDT_REG_TBL[i].uprobe_name);
            return;
        }
        i += 1;
    }

    strncpy(uprobe_reg, sdt_reg, sdt_len as usize);
}

#[no_mangle]
pub unsafe extern "C" fn __perf_sdt_arg_parse_op_x86(
    old_op: *mut c_char,
    new_op: *mut *mut c_char,
) -> c_int {
    let mut new_reg: [c_char; SDT_REG_NAME_SIZE] = [0; SDT_REG_NAME_SIZE];
    let mut new_len: c_int = 0;
    let mut ret: c_int;
    /*
     * rm[0]:  +/-NUM(REG)
     * rm[1]:  +/-
     * rm[2]:  NUM
     * rm[3]:  (
     * rm[4]:  REG
     * rm[5]:  )
     */
    let mut rm: [regmatch_t; 6] = core::mem::zeroed();
    /*
     * Max prefix length is 2 as it may contains sign(+/-)
     * and displacement 0 (Both sign and displacement 0 are
     * optional so it may be empty). Use one more character
     * to hold last NULL so that strlen can be used to find
     * prefix length, instead of maintaining one more variable.
     */
    let mut prefix: [c_char; 3] = [0; 3];

    ret = sdt_init_op_regex();
    if ret < 0 {
        return ret;
    }

    /*
     * If unsupported OR does not match with regex OR
     * register name too long, skip it.
     */
    if !strchr(old_op, ',' as c_int).is_null()
        || !strchr(old_op, '$' as c_int).is_null()
        || regexec(&raw const SDT_OP_REGEX_COMPILED, old_op, 6, rm.as_mut_ptr(), 0) != 0
        || rm[4].rm_eo - rm[4].rm_so > SDT_REG_NAME_SIZE as _
    {
        pr_debug4(c_str!("Skipping unsupported SDT argument: %s\n"), old_op);
        return SDT_ARG_SKIP;
    }

    /*
     * Prepare prefix.
     * If SDT OP has parenthesis but does not provide
     * displacement, add 0 for displacement.
     *     SDT         Uprobe     Prefix
     *     -----------------------------
     *     +24(%rdi)   +24(%di)   +
     *     24(%rdi)    +24(%di)   +
     *     %rdi        %di
     *     (%rdi)      +0(%di)    +0
     *     -80(%rbx)   -80(%bx)   -
     */
    if rm[3].rm_so != rm[3].rm_eo {
        if rm[1].rm_so != rm[1].rm_eo {
            prefix[0] = *old_op.offset(rm[1].rm_so as isize);
        } else if rm[2].rm_so != rm[2].rm_eo {
            prefix[0] = '+' as c_char;
        } else {
            scnprintf(
                prefix.as_mut_ptr(),
                core::mem::size_of_val(&prefix),
                c_str!("+0"),
            );
        }
    }

    /* Rename register */
    sdt_rename_register(
        old_op.offset(rm[4].rm_so as isize),
        (rm[4].rm_eo - rm[4].rm_so) as c_int,
        new_reg.as_mut_ptr(),
    );

    /* Prepare final OP which should be valid for uprobe_events */
    new_len = (strlen(prefix.as_ptr())
        + (rm[2].rm_eo - rm[2].rm_so) as usize
        + (rm[3].rm_eo - rm[3].rm_so) as usize
        + strlen(new_reg.as_ptr())
        + (rm[5].rm_eo - rm[5].rm_so) as usize
        + 1) as c_int; /* NULL */

    *new_op = zalloc(new_len as usize) as *mut c_char;
    if (*new_op).is_null() {
        return -ENOMEM;
    }

    scnprintf(
        *new_op,
        new_len as usize,
        c_str!("%.*s%.*s%.*s%.*s%.*s"),
        strlen(prefix.as_ptr()) as c_int,
        prefix.as_ptr(),
        (rm[2].rm_eo - rm[2].rm_so) as c_int,
        old_op.offset(rm[2].rm_so as isize),
        (rm[3].rm_eo - rm[3].rm_so) as c_int,
        old_op.offset(rm[3].rm_so as isize),
        strlen(new_reg.as_ptr()) as c_int,
        new_reg.as_ptr(),
        (rm[5].rm_eo - rm[5].rm_so) as c_int,
        old_op.offset(rm[5].rm_so as isize),
    );

    SDT_ARG_VALID
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_mask_x86(intr: bool) -> u64 {
    let mut attr = perf_event_attr {
        type_: PERF_TYPE_HARDWARE,
        config: PERF_COUNT_HW_CPU_CYCLES,
        sample_type: PERF_SAMPLE_REGS_INTR,
        sample_regs_intr: PERF_REG_EXTENDED_MASK,
        precise_ip: 1,
        disabled: 1,
        exclude_kernel: 1,
        ..core::mem::zeroed()
    };
    let fd: c_int;

    if !intr {
        return PERF_REGS_MASK;
    }

    /*
     * In an unnamed union, init it here to build on older gcc versions
     */
    attr.sample_period = 1;

    if perf_pmus__num_core_pmus() > 1 {
        let mut pmu: *mut perf_pmu = ptr::null_mut();
        let mut type_: u64 = PERF_TYPE_RAW as u64;

        /*
         * The same register set is supported among different hybrid PMUs.
         * Only check the first available one.
         */
        loop {
            pmu = perf_pmus__scan_core(pmu);
            if pmu.is_null() {
                break;
            }
            type_ = (*pmu).type_ as u64;
            break;
        }
        attr.config |= type_ << PERF_PMU_TYPE_SHIFT;
    }

    event_attr_init(&mut attr);
    fd = sys_perf_event_open(&mut attr, 0, -1, -1, 0);
    if fd != -1 {
        close(fd);
        return PERF_REG_EXTENDED_MASK | PERF_REGS_MASK;
    }

    PERF_REGS_MASK
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_name_x86(id: c_int) -> *const c_char {
    match id {
        PERF_REG_X86_AX => return c_str!("AX"),
        PERF_REG_X86_BX => return c_str!("BX"),
        PERF_REG_X86_CX => return c_str!("CX"),
        PERF_REG_X86_DX => return c_str!("DX"),
        PERF_REG_X86_SI => return c_str!("SI"),
        PERF_REG_X86_DI => return c_str!("DI"),
        PERF_REG_X86_BP => return c_str!("BP"),
        PERF_REG_X86_SP => return c_str!("SP"),
        PERF_REG_X86_IP => return c_str!("IP"),
        PERF_REG_X86_FLAGS => return c_str!("FLAGS"),
        PERF_REG_X86_CS => return c_str!("CS"),
        PERF_REG_X86_SS => return c_str!("SS"),
        PERF_REG_X86_DS => return c_str!("DS"),
        PERF_REG_X86_ES => return c_str!("ES"),
        PERF_REG_X86_FS => return c_str!("FS"),
        PERF_REG_X86_GS => return c_str!("GS"),
        PERF_REG_X86_R8 => return c_str!("R8"),
        PERF_REG_X86_R9 => return c_str!("R9"),
        PERF_REG_X86_R10 => return c_str!("R10"),
        PERF_REG_X86_R11 => return c_str!("R11"),
        PERF_REG_X86_R12 => return c_str!("R12"),
        PERF_REG_X86_R13 => return c_str!("R13"),
        PERF_REG_X86_R14 => return c_str!("R14"),
        PERF_REG_X86_R15 => return c_str!("R15"),

        PERF_REG_X86_XMM0 | x if x == PERF_REG_X86_XMM0 + 1 => return c_str!("XMM0"),
        PERF_REG_X86_XMM1 | x if x == PERF_REG_X86_XMM1 + 1 => return c_str!("XMM1"),
        PERF_REG_X86_XMM2 | x if x == PERF_REG_X86_XMM2 + 1 => return c_str!("XMM2"),
        PERF_REG_X86_XMM3 | x if x == PERF_REG_X86_XMM3 + 1 => return c_str!("XMM3"),
        PERF_REG_X86_XMM4 | x if x == PERF_REG_X86_XMM4 + 1 => return c_str!("XMM4"),
        PERF_REG_X86_XMM5 | x if x == PERF_REG_X86_XMM5 + 1 => return c_str!("XMM5"),
        PERF_REG_X86_XMM6 | x if x == PERF_REG_X86_XMM6 + 1 => return c_str!("XMM6"),
        PERF_REG_X86_XMM7 | x if x == PERF_REG_X86_XMM7 + 1 => return c_str!("XMM7"),
        PERF_REG_X86_XMM8 | x if x == PERF_REG_X86_XMM8 + 1 => return c_str!("XMM8"),
        PERF_REG_X86_XMM9 | x if x == PERF_REG_X86_XMM9 + 1 => return c_str!("XMM9"),
        PERF_REG_X86_XMM10 | x if x == PERF_REG_X86_XMM10 + 1 => return c_str!("XMM10"),
        PERF_REG_X86_XMM11 | x if x == PERF_REG_X86_XMM11 + 1 => return c_str!("XMM11"),
        PERF_REG_X86_XMM12 | x if x == PERF_REG_X86_XMM12 + 1 => return c_str!("XMM12"),
        PERF_REG_X86_XMM13 | x if x == PERF_REG_X86_XMM13 + 1 => return c_str!("XMM13"),
        PERF_REG_X86_XMM14 | x if x == PERF_REG_X86_XMM14 + 1 => return c_str!("XMM14"),
        PERF_REG_X86_XMM15 | x if x == PERF_REG_X86_XMM15 + 1 => return c_str!("XMM15"),
        _ => return ptr::null(),
    }
}

#[no_mangle]
pub extern "C" fn __perf_reg_ip_x86() -> u64 {
    PERF_REG_X86_IP as u64
}

#[no_mangle]
pub extern "C" fn __perf_reg_sp_x86() -> u64 {
    PERF_REG_X86_SP as u64
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
