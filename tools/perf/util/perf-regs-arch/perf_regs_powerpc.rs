// SPDX-License-Identifier: GPL-2.0

// Dependencies originally included from:
// <errno.h>, <string.h>, <regex.h>, <linux/zalloc.h>,
// "../debug.h", "../event.h", "../header.h", "../perf_regs.h",
// "../../perf-sys.h", "../../arch/powerpc/util/utils_header.h",
// "../../arch/powerpc/include/perf_regs.h", <linux/kernel.h>

use core::ffi::{c_char, c_int, c_uint, c_ulonglong, c_void};
use core::mem::MaybeUninit;
use core::ptr;

const PVR_POWER9: u32 = 0x004E;
const PVR_POWER10: u32 = 0x0080;
const PVR_POWER11: u32 = 0x0082;

/* REG or %rREG */
const SDT_OP_REGEX1: &[u8] = b"^(%r)?([1-2]?[0-9]|3[0-1])$\0";

/* -NUM(REG) or NUM(REG) or -NUM(%rREG) or NUM(%rREG) */
const SDT_OP_REGEX2: &[u8] = b"^(\\-)?([0-9]+)\\((%r)?([1-2]?[0-9]|3[0-1])\\)$\0";

type u32 = c_uint;
type u64 = c_ulonglong;
type uint64_t = u64;
type bool_t = bool;

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

#[repr(C)]
pub struct perf_event_attr {
    pub type_: u32,
    pub config: u64,
    pub sample_type: u64,
    pub read_format: u64,
    pub flags: u64,
    pub sample_regs_user: u64,
    pub sample_stack_user: u32,
    pub clockid: c_int,
    pub sample_regs_intr: u64,
    pub aux_watermark: u32,
    pub sample_max_stack: u16,
    pub __reserved_2: u16,
    pub aux_sample_size: u32,
    pub __reserved_3: u32,
    pub sig_data: u64,
    pub config3: u64,
    pub sample_period: u64,
}

unsafe extern "C" {
    static REG_EXTENDED: c_int;
    static ENOMEM: c_int;
    static SDT_ARG_SKIP: c_int;
    static SDT_ARG_VALID: c_int;

    static PERF_TYPE_HARDWARE: u32;
    static PERF_COUNT_HW_CPU_CYCLES: u64;
    static PERF_SAMPLE_REGS_INTR: u64;
    static PERF_REGS_MASK: u64;
    static PERF_REG_PMU_MASK_300: u64;
    static PERF_REG_PMU_MASK_31: u64;
    static SPRN_PVR: c_int;

    static PERF_REG_POWERPC_R0: c_int;
    static PERF_REG_POWERPC_R1: c_int;
    static PERF_REG_POWERPC_R2: c_int;
    static PERF_REG_POWERPC_R3: c_int;
    static PERF_REG_POWERPC_R4: c_int;
    static PERF_REG_POWERPC_R5: c_int;
    static PERF_REG_POWERPC_R6: c_int;
    static PERF_REG_POWERPC_R7: c_int;
    static PERF_REG_POWERPC_R8: c_int;
    static PERF_REG_POWERPC_R9: c_int;
    static PERF_REG_POWERPC_R10: c_int;
    static PERF_REG_POWERPC_R11: c_int;
    static PERF_REG_POWERPC_R12: c_int;
    static PERF_REG_POWERPC_R13: c_int;
    static PERF_REG_POWERPC_R14: c_int;
    static PERF_REG_POWERPC_R15: c_int;
    static PERF_REG_POWERPC_R16: c_int;
    static PERF_REG_POWERPC_R17: c_int;
    static PERF_REG_POWERPC_R18: c_int;
    static PERF_REG_POWERPC_R19: c_int;
    static PERF_REG_POWERPC_R20: c_int;
    static PERF_REG_POWERPC_R21: c_int;
    static PERF_REG_POWERPC_R22: c_int;
    static PERF_REG_POWERPC_R23: c_int;
    static PERF_REG_POWERPC_R24: c_int;
    static PERF_REG_POWERPC_R25: c_int;
    static PERF_REG_POWERPC_R26: c_int;
    static PERF_REG_POWERPC_R27: c_int;
    static PERF_REG_POWERPC_R28: c_int;
    static PERF_REG_POWERPC_R29: c_int;
    static PERF_REG_POWERPC_R30: c_int;
    static PERF_REG_POWERPC_R31: c_int;
    static PERF_REG_POWERPC_NIP: c_int;
    static PERF_REG_POWERPC_MSR: c_int;
    static PERF_REG_POWERPC_ORIG_R3: c_int;
    static PERF_REG_POWERPC_CTR: c_int;
    static PERF_REG_POWERPC_LINK: c_int;
    static PERF_REG_POWERPC_XER: c_int;
    static PERF_REG_POWERPC_CCR: c_int;
    static PERF_REG_POWERPC_SOFTE: c_int;
    static PERF_REG_POWERPC_TRAP: c_int;
    static PERF_REG_POWERPC_DAR: c_int;
    static PERF_REG_POWERPC_DSISR: c_int;
    static PERF_REG_POWERPC_SIER: c_int;
    static PERF_REG_POWERPC_MMCRA: c_int;
    static PERF_REG_POWERPC_MMCR0: c_int;
    static PERF_REG_POWERPC_MMCR1: c_int;
    static PERF_REG_POWERPC_MMCR2: c_int;
    static PERF_REG_POWERPC_MMCR3: c_int;
    static PERF_REG_POWERPC_SIER2: c_int;
    static PERF_REG_POWERPC_SIER3: c_int;
    static PERF_REG_POWERPC_PMC1: c_int;
    static PERF_REG_POWERPC_PMC2: c_int;
    static PERF_REG_POWERPC_PMC3: c_int;
    static PERF_REG_POWERPC_PMC4: c_int;
    static PERF_REG_POWERPC_PMC5: c_int;
    static PERF_REG_POWERPC_PMC6: c_int;
    static PERF_REG_POWERPC_SDAR: c_int;
    static PERF_REG_POWERPC_SIAR: c_int;

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
    fn event_attr_init(attr: *mut perf_event_attr);
    fn sys_perf_event_open(
        attr: *mut perf_event_attr,
        pid: c_int,
        cpu: c_int,
        group_fd: c_int,
        flags: c_ulonglong,
    ) -> c_int;
    fn close(fd: c_int) -> c_int;
    fn mfspr(spr: c_int) -> u64;
}

static mut SDT_OP_REGEX1_STORAGE: MaybeUninit<regex_t> = MaybeUninit::uninit();
static mut SDT_OP_REGEX2_STORAGE: MaybeUninit<regex_t> = MaybeUninit::uninit();

unsafe fn sdt_op_regex1() -> *mut regex_t {
    SDT_OP_REGEX1_STORAGE.as_mut_ptr()
}

unsafe fn sdt_op_regex2() -> *mut regex_t {
    SDT_OP_REGEX2_STORAGE.as_mut_ptr()
}

unsafe fn sdt_init_op_regex() -> c_int {
    static mut INITIALIZED: c_int = 0;
    let mut ret: c_int = 0;

    if INITIALIZED != 0 {
        return 0;
    }

    ret = regcomp(
        sdt_op_regex1(),
        SDT_OP_REGEX1.as_ptr() as *const c_char,
        REG_EXTENDED,
    );
    if ret != 0 {
        pr_debug4(b"Regex compilation error.\n\0".as_ptr() as *const c_char);
        return ret;
    }

    ret = regcomp(
        sdt_op_regex2(),
        SDT_OP_REGEX2.as_ptr() as *const c_char,
        REG_EXTENDED,
    );
    if ret != 0 {
        regfree(sdt_op_regex1());
        pr_debug4(b"Regex compilation error.\n\0".as_ptr() as *const c_char);
        return ret;
    }

    INITIALIZED = 1;
    0
}

/*
 * Parse OP and convert it into uprobe format, which is, +/-NUM(%gprREG).
 * Possible variants of OP are:
 *	Format		Example
 *	-------------------------
 *	NUM(REG)	48(18)
 *	-NUM(REG)	-48(18)
 *	NUM(%rREG)	48(%r18)
 *	-NUM(%rREG)	-48(%r18)
 *	REG		18
 *	%rREG		%r18
 *	iNUM		i0
 *	i-NUM		i-1
 *
 * SDT marker arguments on Powerpc uses %rREG form with -mregnames flag
 * and REG form with -mno-regnames. Here REG is general purpose register,
 * which is in 0 to 31 range.
 */
#[no_mangle]
pub unsafe extern "C" fn __perf_sdt_arg_parse_op_powerpc(
    old_op: *mut c_char,
    new_op: *mut *mut c_char,
) -> c_int {
    let mut ret: c_int;
    let mut new_len: c_int;
    let mut rm: [regmatch_t; 5] = [regmatch_t { rm_so: 0, rm_eo: 0 }; 5];
    let prefix: c_char;

    /* Constant argument. Uprobe does not support it */
    if *old_op.offset(0) == b'i' as c_char {
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

    if regexec(sdt_op_regex1(), old_op, 3, rm.as_mut_ptr(), 0) == 0 {
        /* REG or %rREG --> %gprREG */

        new_len = 5; /* % g p r NULL */
        new_len += (rm[2].rm_eo - rm[2].rm_so) as c_int;

        *new_op = zalloc(new_len as usize) as *mut c_char;
        if (*new_op).is_null() {
            return -ENOMEM;
        }

        scnprintf(
            *new_op,
            new_len as usize,
            b"%%gpr%.*s\0".as_ptr() as *const c_char,
            (rm[2].rm_eo - rm[2].rm_so) as c_int,
            old_op.offset(rm[2].rm_so),
        );
    } else if regexec(sdt_op_regex2(), old_op, 5, rm.as_mut_ptr(), 0) == 0 {
        /*
         * -NUM(REG) or NUM(REG) or -NUM(%rREG) or NUM(%rREG) -->
         *	+/-NUM(%gprREG)
         */
        prefix = if rm[1].rm_so == -1 { b'+' as c_char } else { b'-' as c_char };

        new_len = 8; /* +/- ( % g p r ) NULL */
        new_len += (rm[2].rm_eo - rm[2].rm_so) as c_int;
        new_len += (rm[4].rm_eo - rm[4].rm_so) as c_int;

        *new_op = zalloc(new_len as usize) as *mut c_char;
        if (*new_op).is_null() {
            return -ENOMEM;
        }

        scnprintf(
            *new_op,
            new_len as usize,
            b"%c%.*s(%%gpr%.*s)\0".as_ptr() as *const c_char,
            prefix as c_int,
            (rm[2].rm_eo - rm[2].rm_so) as c_int,
            old_op.offset(rm[2].rm_so),
            (rm[4].rm_eo - rm[4].rm_so) as c_int,
            old_op.offset(rm[4].rm_so),
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

/*
 * mfspr is a POWERPC specific instruction, ensure it's only
 * built and called on POWERPC by guarding with __powerpc64__
 * or __powerpc__.
 */
#[cfg(all(target_arch = "powerpc64", target_arch = "powerpc"))]
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_mask_powerpc(intr: bool_t) -> uint64_t {
    let mut attr = perf_event_attr {
        type_: PERF_TYPE_HARDWARE,
        config: PERF_COUNT_HW_CPU_CYCLES,
        sample_type: PERF_SAMPLE_REGS_INTR,
        read_format: 0,
        flags: (1 << 0) | (1 << 1) | (1 << 5),
        sample_regs_user: 0,
        sample_stack_user: 0,
        clockid: 0,
        sample_regs_intr: 0,
        aux_watermark: 0,
        sample_max_stack: 0,
        __reserved_2: 0,
        aux_sample_size: 0,
        __reserved_3: 0,
        sig_data: 0,
        config3: 0,
        sample_period: 0,
    };
    let fd: c_int;
    let version: u32;
    let mut extended_mask: u64 = 0;
    let mut mask: u64 = PERF_REGS_MASK;

    if !intr {
        return PERF_REGS_MASK;
    }

    /*
     * Get the PVR value to set the extended
     * mask specific to platform.
     */
    version = ((mfspr(SPRN_PVR) >> 16) & 0xFFFF) as u32;
    if version == PVR_POWER9 {
        extended_mask = PERF_REG_PMU_MASK_300;
    } else if version == PVR_POWER10 || version == PVR_POWER11 {
        extended_mask = PERF_REG_PMU_MASK_31;
    } else {
        return mask;
    }

    attr.sample_regs_intr = extended_mask;
    attr.sample_period = 1;
    event_attr_init(&mut attr);

    /*
     * Check if the pmu supports perf extended regs, before
     * returning the register mask to sample. Open the event
     * on the perf process to check this.
     */
    fd = sys_perf_event_open(&mut attr, 0, -1, -1, 0);
    if fd != -1 {
        close(fd);
        mask |= extended_mask;
    }
    mask
}

#[cfg(not(all(target_arch = "powerpc64", target_arch = "powerpc")))]
#[no_mangle]
pub unsafe extern "C" fn __perf_reg_mask_powerpc(_intr: bool_t) -> uint64_t {
    PERF_REGS_MASK
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_name_powerpc(id: c_int) -> *const c_char {
    if id == PERF_REG_POWERPC_R0 {
        b"r0\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R1 {
        b"r1\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R2 {
        b"r2\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R3 {
        b"r3\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R4 {
        b"r4\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R5 {
        b"r5\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R6 {
        b"r6\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R7 {
        b"r7\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R8 {
        b"r8\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R9 {
        b"r9\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R10 {
        b"r10\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R11 {
        b"r11\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R12 {
        b"r12\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R13 {
        b"r13\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R14 {
        b"r14\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R15 {
        b"r15\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R16 {
        b"r16\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R17 {
        b"r17\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R18 {
        b"r18\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R19 {
        b"r19\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R20 {
        b"r20\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R21 {
        b"r21\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R22 {
        b"r22\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R23 {
        b"r23\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R24 {
        b"r24\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R25 {
        b"r25\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R26 {
        b"r26\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R27 {
        b"r27\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R28 {
        b"r28\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R29 {
        b"r29\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R30 {
        b"r30\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_R31 {
        b"r31\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_NIP {
        b"nip\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_MSR {
        b"msr\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_ORIG_R3 {
        b"orig_r3\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_CTR {
        b"ctr\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_LINK {
        b"link\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_XER {
        b"xer\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_CCR {
        b"ccr\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_SOFTE {
        b"softe\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_TRAP {
        b"trap\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_DAR {
        b"dar\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_DSISR {
        b"dsisr\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_SIER {
        b"sier\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_MMCRA {
        b"mmcra\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_MMCR0 {
        b"mmcr0\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_MMCR1 {
        b"mmcr1\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_MMCR2 {
        b"mmcr2\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_MMCR3 {
        b"mmcr3\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_SIER2 {
        b"sier2\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_SIER3 {
        b"sier3\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_PMC1 {
        b"pmc1\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_PMC2 {
        b"pmc2\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_PMC3 {
        b"pmc3\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_PMC4 {
        b"pmc4\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_PMC5 {
        b"pmc5\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_PMC6 {
        b"pmc6\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_SDAR {
        b"sdar\0".as_ptr() as *const c_char
    } else if id == PERF_REG_POWERPC_SIAR {
        b"siar\0".as_ptr() as *const c_char
    } else {
        ptr::null()
    }
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_ip_powerpc() -> uint64_t {
    PERF_REG_POWERPC_NIP as uint64_t
}

#[no_mangle]
pub unsafe extern "C" fn __perf_reg_sp_powerpc() -> uint64_t {
    PERF_REG_POWERPC_R1 as uint64_t
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
