// SPDX-License-Identifier: GPL-2.0
// Rust translation of perf/util/annotate-arch/annotate-powerpc.c.
// C includes translated as external declarations for symbols provided by
// string.h, linux/compiler.h, linux/kernel.h, annotate-data.h, debug.h, disasm.h.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};

type size_t = usize;
type u32 = c_uint;

const fn PPC_OP(op: c_int) -> c_int {
    ((op >> 26) & 0x3f) as c_int
}

const fn PPC_21_30(R: c_int) -> c_int {
    ((R >> 1) & 0x3ff) as c_int
}

const fn PPC_22_30(R: c_int) -> c_int {
    ((R >> 1) & 0x1ff) as c_int
}

const MINUS_EXT_XO_FORM: c_int = 234;
const SUB_EXT_XO_FORM: c_int = 232;
const ADD_ZERO_EXT_XO_FORM: c_int = 202;
const SUB_ZERO_EXT_XO_FORM: c_int = 200;

const INSN_OP_SOURCE: usize = 0;
const INSN_OP_TARGET: usize = 1;

#[repr(C)]
pub struct arch {
    pub name: *const c_char,
    pub id: e_machine_and_e_flags,
    pub objdump: objdump,
    pub associate_instruction_ops:
        Option<unsafe extern "C" fn(*mut arch, *const c_char) -> *const ins_ops>,
    // Present in C only under HAVE_LIBDW_SUPPORT.
    pub update_insn_state: Option<
        unsafe extern "C" fn(*mut type_state, *mut data_loc_info, *mut Dwarf_Die, *mut disasm_line),
    >,
}

#[repr(C)]
pub struct objdump {
    pub comment_char: c_char,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct e_machine_and_e_flags {
    _private: [u8; 0],
}

#[repr(C)]
pub struct ins {
    pub name: *const c_char,
}

#[repr(C)]
pub struct ins_ops {
    pub parse: Option<
        unsafe extern "C" fn(
            *const arch,
            *mut ins_operands,
            *mut map_symbol,
            *mut disasm_line,
        ) -> c_int,
    >,
    pub scnprintf:
        Option<unsafe extern "C" fn(*const ins, *mut c_char, size_t, *mut ins_operands, c_int) -> c_int>,
}

#[repr(C)]
pub struct operand {
    pub mem_ref: bool,
    pub multi_regs: bool,
}

#[repr(C)]
pub struct ins_operands {
    pub raw: *const c_char,
    pub source: operand,
    pub target: operand,
}

#[repr(C)]
pub struct map_symbol {
    _private: [u8; 0],
}

#[repr(C)]
pub struct raw_insn {
    pub raw_insn: c_int,
}

#[repr(C)]
pub struct disasm_line {
    pub raw: raw_insn,
    pub ins: ins,
    pub al: annotation_line,
}

#[repr(C)]
pub struct annotation_line {
    pub offset: u32,
}

#[repr(C)]
pub struct annotated_op_loc {
    pub reg1: c_int,
}

#[repr(C)]
pub struct annotated_insn_loc {
    pub ops: [annotated_op_loc; 2],
}

#[repr(C)]
pub struct type_state {
    pub regs: *mut type_state_reg,
}

#[repr(C)]
pub struct type_state_reg {
    pub ok: bool,
    pub type_: c_int,
    pub kind: c_int,
}

#[repr(C)]
pub struct data_loc_info {
    pub arch: *mut arch,
}

#[repr(C)]
pub struct Dwarf_Die {
    _private: [u8; 0],
}

#[repr(C)]
pub struct annotate_opts_t {
    pub show_asm_raw: bool,
}

unsafe extern "C" {
    static jump_ops: ins_ops;
    static call_ops: ins_ops;
    static ret_ops: ins_ops;
    static mut annotate_opts: annotate_opts_t;

    fn scnprintf(str: *mut c_char, size: size_t, format: *const c_char, ...) -> c_int;
    fn strlen(s: *const c_char) -> size_t;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn sprintf(str: *mut c_char, format: *const c_char, ...) -> c_int;
    fn strdup(s: *const c_char) -> *mut c_char;
    fn bsearch(
        key: *const c_void,
        base: *const c_void,
        nmemb: size_t,
        size: size_t,
        compar: Option<unsafe extern "C" fn(*const c_void, *const c_void) -> c_int>,
    ) -> *mut c_void;
    fn zalloc(size: size_t) -> *mut c_void;

    fn arch__associate_ins_ops(arch: *mut arch, name: *const c_char, ops: *const ins_ops);

    fn annotate_get_insn_location(
        arch: *mut arch,
        dl: *mut disasm_line,
        loc: *mut annotated_insn_loc,
    ) -> c_int;
    fn has_reg_type(state: *mut type_state, reg: c_int) -> bool;
    fn pr_debug_dtp(format: *const c_char, ...);
    fn pr_debug_type_name(type_: *mut c_int, kind: c_int);
}

unsafe extern "C" fn arithmetic__scnprintf(
    ins: *const ins,
    bf: *mut c_char,
    size: size_t,
    ops: *mut ins_operands,
    max_ins_name: c_int,
) -> c_int {
    unsafe {
        scnprintf(
            bf,
            size,
            c"%-*s %s".as_ptr(),
            max_ins_name,
            (*ins).name,
            (*ops).raw,
        )
    }
}

/*
 * Sets the fields: multi_regs and "mem_ref".
 * "mem_ref" is set for ops->source which is later used to
 * fill the objdump->memory_ref-char field. This ops is currently
 * used by powerpc and since binary instruction code is used to
 * extract opcode, regs and offset, no other parsing is needed here.
 *
 * Dont set multi regs for 4 cases since it has only one operand
 * for source:
 * - Add to Minus One Extended XO-form ( Ex: addme, addmeo )
 * - Subtract From Minus One Extended XO-form ( Ex: subfme )
 * - Add to Zero Extended XO-form ( Ex: addze, addzeo )
 * - Subtract From Zero Extended XO-form ( Ex: subfze )
 */
unsafe extern "C" fn arithmetic__parse(
    _arch: *const arch,
    ops: *mut ins_operands,
    _ms: *mut map_symbol,
    dl: *mut disasm_line,
) -> c_int {
    let opcode = unsafe { PPC_OP((*dl).raw.raw_insn) };

    unsafe {
        (*ops).source.mem_ref = false;
        if opcode == 31 {
            if (opcode != MINUS_EXT_XO_FORM)
                && (opcode != SUB_EXT_XO_FORM)
                && (opcode != ADD_ZERO_EXT_XO_FORM)
                && (opcode != SUB_ZERO_EXT_XO_FORM)
            {
                (*ops).source.multi_regs = true;
            }
        }

        (*ops).target.mem_ref = false;
        (*ops).target.multi_regs = false;
    }

    0
}

static arithmetic_ops: ins_ops = ins_ops {
    parse: Some(arithmetic__parse),
    scnprintf: Some(arithmetic__scnprintf),
};

unsafe extern "C" fn load_store__scnprintf(
    ins: *const ins,
    bf: *mut c_char,
    size: size_t,
    ops: *mut ins_operands,
    max_ins_name: c_int,
) -> c_int {
    unsafe {
        scnprintf(
            bf,
            size,
            c"%-*s %s".as_ptr(),
            max_ins_name,
            (*ins).name,
            (*ops).raw,
        )
    }
}

/*
 * Sets the fields: multi_regs and "mem_ref".
 * "mem_ref" is set for ops->source which is later used to
 * fill the objdump->memory_ref-char field. This ops is currently
 * used by powerpc and since binary instruction code is used to
 * extract opcode, regs and offset, no other parsing is needed here
 */
unsafe extern "C" fn load_store__parse(
    _arch: *const arch,
    ops: *mut ins_operands,
    _ms: *mut map_symbol,
    dl: *mut disasm_line,
) -> c_int {
    unsafe {
        (*ops).source.mem_ref = true;
        (*ops).source.multi_regs = false;
        /* opcode 31 is of X form */
        if PPC_OP((*dl).raw.raw_insn) == 31 {
            (*ops).source.multi_regs = true;
        }

        (*ops).target.mem_ref = false;
        (*ops).target.multi_regs = false;
    }

    0
}

static load_store_ops: ins_ops = ins_ops {
    parse: Some(load_store__parse),
    scnprintf: Some(load_store__scnprintf),
};

unsafe extern "C" fn powerpc__associate_instruction_ops(
    arch: *mut arch,
    name: *const c_char,
) -> *const ins_ops {
    let mut ops: *const ins_ops;

    /*
     * - Interested only if instruction starts with 'b'.
     * - Few start with 'b', but aren't branch instructions.
     */
    unsafe {
        if *name != b'b' as c_char
            || strncmp(name, c"bcd".as_ptr(), 3) == 0
            || strncmp(name, c"brinc".as_ptr(), 5) == 0
            || strncmp(name, c"bper".as_ptr(), 4) == 0
        {
            return core::ptr::null();
        }
    }

    unsafe {
        ops = &jump_ops;
    }

    let mut i = unsafe { strlen(name) as isize - 1 };
    if i < 0 {
        return core::ptr::null();
    }

    /* ignore optional hints at the end of the instructions */
    unsafe {
        if *name.offset(i) == b'+' as c_char || *name.offset(i) == b'-' as c_char {
            i -= 1;
        }

        if *name.offset(i) == b'l' as c_char
            || (*name.offset(i) == b'a' as c_char && *name.offset(i - 1) == b'l' as c_char)
        {
            /*
             * if the instruction ends up with 'l' or 'la', then
             * those are considered 'calls' since they update LR.
             * ... except for 'bnl' which is branch if not less than
             * and the absolute form of the same.
             */
            if strcmp(name, c"bnl".as_ptr()) != 0
                && strcmp(name, c"bnl+".as_ptr()) != 0
                && strcmp(name, c"bnl-".as_ptr()) != 0
                && strcmp(name, c"bnla".as_ptr()) != 0
                && strcmp(name, c"bnla+".as_ptr()) != 0
                && strcmp(name, c"bnla-".as_ptr()) != 0
            {
                ops = &call_ops;
            }
        }
        if *name.offset(i) == b'r' as c_char && *name.offset(i - 1) == b'l' as c_char {
            /*
             * instructions ending with 'lr' are considered to be
             * return instructions
             */
            ops = &ret_ops;
        }

        arch__associate_ins_ops(arch, name, ops);
    }
    ops
}

#[repr(C)]
struct insn_offset {
    name: *const c_char,
    value: c_int,
}

/*
 * There are memory instructions with opcode 31 which are
 * of X Form, Example:
 * ldx RT,RA,RB
 * ______________________________________
 * | 31 |  RT  |  RA |  RB |   21     |/|
 * --------------------------------------
 * 0    6     11    16    21         30 31
 *
 * But all instructions with opcode 31 are not memory.
 * Example: add RT,RA,RB
 *
 * Use bits 21 to 30 to check memory insns with 31 as opcode.
 * In ins_array below, for ldx instruction:
 * name => OP_31_XOP_LDX
 * value => 21
 */

static mut ins_array: [insn_offset; 56] = [
    insn_offset { name: c"OP_31_XOP_LXSIWZX".as_ptr(), value: 12 },
    insn_offset { name: c"OP_31_XOP_LWARX".as_ptr(), value: 20 },
    insn_offset { name: c"OP_31_XOP_LDX".as_ptr(), value: 21 },
    insn_offset { name: c"OP_31_XOP_LWZX".as_ptr(), value: 23 },
    insn_offset { name: c"OP_31_XOP_LDUX".as_ptr(), value: 53 },
    insn_offset { name: c"OP_31_XOP_LWZUX".as_ptr(), value: 55 },
    insn_offset { name: c"OP_31_XOP_LXSIWAX".as_ptr(), value: 76 },
    insn_offset { name: c"OP_31_XOP_LDARX".as_ptr(), value: 84 },
    insn_offset { name: c"OP_31_XOP_LBZX".as_ptr(), value: 87 },
    insn_offset { name: c"OP_31_XOP_LVX".as_ptr(), value: 103 },
    insn_offset { name: c"OP_31_XOP_LBZUX".as_ptr(), value: 119 },
    insn_offset { name: c"OP_31_XOP_STXSIWX".as_ptr(), value: 140 },
    insn_offset { name: c"OP_31_XOP_STDX".as_ptr(), value: 149 },
    insn_offset { name: c"OP_31_XOP_STWX".as_ptr(), value: 151 },
    insn_offset { name: c"OP_31_XOP_STDUX".as_ptr(), value: 181 },
    insn_offset { name: c"OP_31_XOP_STWUX".as_ptr(), value: 183 },
    insn_offset { name: c"OP_31_XOP_STBX".as_ptr(), value: 215 },
    insn_offset { name: c"OP_31_XOP_STVX".as_ptr(), value: 231 },
    insn_offset { name: c"OP_31_XOP_STBUX".as_ptr(), value: 247 },
    insn_offset { name: c"OP_31_XOP_LHZX".as_ptr(), value: 279 },
    insn_offset { name: c"OP_31_XOP_LHZUX".as_ptr(), value: 311 },
    insn_offset { name: c"OP_31_XOP_LXVDSX".as_ptr(), value: 332 },
    insn_offset { name: c"OP_31_XOP_LWAX".as_ptr(), value: 341 },
    insn_offset { name: c"OP_31_XOP_LHAX".as_ptr(), value: 343 },
    insn_offset { name: c"OP_31_XOP_LWAUX".as_ptr(), value: 373 },
    insn_offset { name: c"OP_31_XOP_LHAUX".as_ptr(), value: 375 },
    insn_offset { name: c"OP_31_XOP_STHX".as_ptr(), value: 407 },
    insn_offset { name: c"OP_31_XOP_STHUX".as_ptr(), value: 439 },
    insn_offset { name: c"OP_31_XOP_LXSSPX".as_ptr(), value: 524 },
    insn_offset { name: c"OP_31_XOP_LDBRX".as_ptr(), value: 532 },
    insn_offset { name: c"OP_31_XOP_LSWX".as_ptr(), value: 533 },
    insn_offset { name: c"OP_31_XOP_LWBRX".as_ptr(), value: 534 },
    insn_offset { name: c"OP_31_XOP_LFSUX".as_ptr(), value: 567 },
    insn_offset { name: c"OP_31_XOP_LXSDX".as_ptr(), value: 588 },
    insn_offset { name: c"OP_31_XOP_LSWI".as_ptr(), value: 597 },
    insn_offset { name: c"OP_31_XOP_LFDX".as_ptr(), value: 599 },
    insn_offset { name: c"OP_31_XOP_LFDUX".as_ptr(), value: 631 },
    insn_offset { name: c"OP_31_XOP_STXSSPX".as_ptr(), value: 652 },
    insn_offset { name: c"OP_31_XOP_STDBRX".as_ptr(), value: 660 },
    insn_offset { name: c"OP_31_XOP_STXWX".as_ptr(), value: 661 },
    insn_offset { name: c"OP_31_XOP_STWBRX".as_ptr(), value: 662 },
    insn_offset { name: c"OP_31_XOP_STFSX".as_ptr(), value: 663 },
    insn_offset { name: c"OP_31_XOP_STFSUX".as_ptr(), value: 695 },
    insn_offset { name: c"OP_31_XOP_STXSDX".as_ptr(), value: 716 },
    insn_offset { name: c"OP_31_XOP_STSWI".as_ptr(), value: 725 },
    insn_offset { name: c"OP_31_XOP_STFDX".as_ptr(), value: 727 },
    insn_offset { name: c"OP_31_XOP_STFDUX".as_ptr(), value: 759 },
    insn_offset { name: c"OP_31_XOP_LXVW4X".as_ptr(), value: 780 },
    insn_offset { name: c"OP_31_XOP_LHBRX".as_ptr(), value: 790 },
    insn_offset { name: c"OP_31_XOP_LXVD2X".as_ptr(), value: 844 },
    insn_offset { name: c"OP_31_XOP_LFIWAX".as_ptr(), value: 855 },
    insn_offset { name: c"OP_31_XOP_LFIWZX".as_ptr(), value: 887 },
    insn_offset { name: c"OP_31_XOP_STXVW4X".as_ptr(), value: 908 },
    insn_offset { name: c"OP_31_XOP_STHBRX".as_ptr(), value: 918 },
    insn_offset { name: c"OP_31_XOP_STXVD2X".as_ptr(), value: 972 },
    insn_offset { name: c"OP_31_XOP_STFIWX".as_ptr(), value: 983 },
];

/*
 * Arithmetic instructions which are having opcode as 31.
 * These instructions are tracked to save the register state
 * changes. Example:
 *
 * lwz	r10,264(r3)
 * add	r31, r3, r3
 * lwz	r9, 0(r31)
 *
 * Here instruction tracking needs to identify the "add"
 * instruction and save data type of r3 to r31. If a sample
 * is hit at next "lwz r9, 0(r31)", by this instruction tracking,
 * data type of r31 can be resolved.
 */
static mut arithmetic_ins_op_31: [insn_offset; 20] = [
    insn_offset { name: c"SUB_CARRY_XO_FORM".as_ptr(), value: 8 },
    insn_offset { name: c"MUL_HDW_XO_FORM1".as_ptr(), value: 9 },
    insn_offset { name: c"ADD_CARRY_XO_FORM".as_ptr(), value: 10 },
    insn_offset { name: c"MUL_HW_XO_FORM1".as_ptr(), value: 11 },
    insn_offset { name: c"SUB_XO_FORM".as_ptr(), value: 40 },
    insn_offset { name: c"MUL_HDW_XO_FORM".as_ptr(), value: 73 },
    insn_offset { name: c"MUL_HW_XO_FORM".as_ptr(), value: 75 },
    insn_offset { name: c"SUB_EXT_XO_FORM".as_ptr(), value: 136 },
    insn_offset { name: c"ADD_EXT_XO_FORM".as_ptr(), value: 138 },
    insn_offset { name: c"SUB_ZERO_EXT_XO_FORM".as_ptr(), value: 200 },
    insn_offset { name: c"ADD_ZERO_EXT_XO_FORM".as_ptr(), value: 202 },
    insn_offset { name: c"SUB_EXT_XO_FORM2".as_ptr(), value: 232 },
    insn_offset { name: c"MUL_DW_XO_FORM".as_ptr(), value: 233 },
    insn_offset { name: c"ADD_EXT_XO_FORM2".as_ptr(), value: 234 },
    insn_offset { name: c"MUL_W_XO_FORM".as_ptr(), value: 235 },
    insn_offset { name: c"ADD_XO_FORM".as_ptr(), value: 266 },
    insn_offset { name: c"DIV_DW_XO_FORM1".as_ptr(), value: 457 },
    insn_offset { name: c"DIV_W_XO_FORM1".as_ptr(), value: 459 },
    insn_offset { name: c"DIV_DW_XO_FORM".as_ptr(), value: 489 },
    insn_offset { name: c"DIV_W_XO_FORM".as_ptr(), value: 491 },
];

static mut arithmetic_two_ops: [insn_offset; 6] = [
    insn_offset { name: c"mulli".as_ptr(), value: 7 },
    insn_offset { name: c"subfic".as_ptr(), value: 8 },
    insn_offset { name: c"addic".as_ptr(), value: 12 },
    insn_offset { name: c"addic.".as_ptr(), value: 13 },
    insn_offset { name: c"addi".as_ptr(), value: 14 },
    insn_offset { name: c"addis".as_ptr(), value: 15 },
];

unsafe extern "C" fn cmp_offset(a: *const c_void, b: *const c_void) -> c_int {
    unsafe {
        let val1 = a as *const insn_offset;
        let val2 = b as *const insn_offset;

        (*val1).value - (*val2).value
    }
}

#[no_mangle]
pub unsafe extern "C" fn check_ppc_insn(dl: *mut disasm_line) -> *const ins_ops {
    unsafe {
        let raw_insn = (*dl).raw.raw_insn;
        let opcode = PPC_OP(raw_insn);
        let mem_insn_31 = PPC_21_30(raw_insn);
        let mut ret: *mut insn_offset;
        let mut mem_insns_31_opcode = insn_offset {
            name: c"OP_31_INSN".as_ptr(),
            value: mem_insn_31,
        };
        let mut name_insn = [0 as c_char; 32];

        /*
         * Instructions with opcode 32 to 63 are memory
         * instructions in powerpc
         */
        if (opcode & 0x20) != 0 {
            /*
             * Set name in case of raw instruction to
             * opcode to be used in insn-stat
             */
            if strlen((*dl).ins.name) == 0 {
                sprintf(name_insn.as_mut_ptr(), c"%d".as_ptr(), opcode);
                (*dl).ins.name = strdup(name_insn.as_ptr());
            }
            return &load_store_ops;
        } else if opcode == 31 {
            /* Check for memory instructions with opcode 31 */
            ret = bsearch(
                &mem_insns_31_opcode as *const insn_offset as *const c_void,
                core::ptr::addr_of_mut!(ins_array) as *const c_void,
                ins_array.len(),
                core::mem::size_of::<insn_offset>(),
                Some(cmp_offset),
            ) as *mut insn_offset;
            if !ret.is_null() {
                if strlen((*dl).ins.name) == 0 {
                    (*dl).ins.name = strdup((*ret).name);
                }
                return &load_store_ops;
            } else {
                mem_insns_31_opcode.value = PPC_22_30(raw_insn);
                ret = bsearch(
                    &mem_insns_31_opcode as *const insn_offset as *const c_void,
                    core::ptr::addr_of_mut!(arithmetic_ins_op_31) as *const c_void,
                    arithmetic_ins_op_31.len(),
                    core::mem::size_of::<insn_offset>(),
                    Some(cmp_offset),
                ) as *mut insn_offset;
                if !ret.is_null() {
                    return &arithmetic_ops;
                }
                /* Bits 21 to 30 has value 444 for "mr" insn ie, OR X form */
                if PPC_21_30(raw_insn) == 444 {
                    return &arithmetic_ops;
                }
            }
        } else {
            mem_insns_31_opcode.value = opcode;
            ret = bsearch(
                &mem_insns_31_opcode as *const insn_offset as *const c_void,
                core::ptr::addr_of_mut!(arithmetic_two_ops) as *const c_void,
                arithmetic_two_ops.len(),
                core::mem::size_of::<insn_offset>(),
                Some(cmp_offset),
            ) as *mut insn_offset;
            if !ret.is_null() {
                return &arithmetic_ops;
            }
        }

        core::ptr::null()
    }
}

/*
 * Instruction tracking function to track register state moves.
 * Example sequence:
 *    ld      r10,264(r3)
 *    mr      r31,r3
 *    <<after some sequence>
 *    ld      r9,312(r31)
 *
 * Previous instruction sequence shows that register state of r3
 * is moved to r31. update_insn_state_powerpc tracks these state
 * changes
 */
// C conditional: compiled only when HAVE_LIBDW_SUPPORT is defined.
unsafe extern "C" fn update_insn_state_powerpc(
    state: *mut type_state,
    dloc: *mut data_loc_info,
    _cu_die: *mut Dwarf_Die,
    dl: *mut disasm_line,
) {
    unsafe {
        let mut loc = core::mem::MaybeUninit::<annotated_insn_loc>::uninit();

        if annotate_get_insn_location((*dloc).arch, dl, loc.as_mut_ptr()) < 0 {
            return;
        }

        let mut loc = loc.assume_init();
        let src = &mut loc.ops[INSN_OP_SOURCE] as *mut annotated_op_loc;
        let dst = &mut loc.ops[INSN_OP_TARGET] as *mut annotated_op_loc;
        let mut tsr: *mut type_state_reg;
        let insn_offset: u32 = (*dl).al.offset;

        /*
         * Value 444 for bits 21:30 is for "mr"
         * instruction. "mr" is extended OR. So set the
         * source and destination reg correctly
         */
        if PPC_21_30((*dl).raw.raw_insn) == 444 {
            let src_reg = (*src).reg1;

            (*src).reg1 = (*dst).reg1;
            (*dst).reg1 = src_reg;
        }

        if !has_reg_type(state, (*dst).reg1) {
            return;
        }

        tsr = (*state).regs.offset((*dst).reg1 as isize);

        if !has_reg_type(state, (*src).reg1)
            || !(*(*state).regs.offset((*src).reg1 as isize)).ok
        {
            (*tsr).ok = false;
            return;
        }

        (*tsr).type_ = (*(*state).regs.offset((*src).reg1 as isize)).type_;
        (*tsr).kind = (*(*state).regs.offset((*src).reg1 as isize)).kind;
        (*tsr).ok = true;

        pr_debug_dtp(
            c"mov [%x] reg%d -> reg%d".as_ptr(),
            insn_offset,
            (*src).reg1,
            (*dst).reg1,
        );
        pr_debug_type_name(&mut (*tsr).type_, (*tsr).kind);
    }
}

#[no_mangle]
pub unsafe extern "C" fn arch__new_powerpc(
    id: *const e_machine_and_e_flags,
    _cpuid: *const c_char,
) -> *const arch {
    unsafe {
        let arch = zalloc(core::mem::size_of::<arch>()) as *mut arch;

        if arch.is_null() {
            return core::ptr::null();
        }

        (*arch).name = c"powerpc".as_ptr();
        (*arch).id = *id;
        (*arch).objdump.comment_char = b'#' as c_char;
        annotate_opts.show_asm_raw = true;
        (*arch).associate_instruction_ops = Some(powerpc__associate_instruction_ops);
        // C conditional: only assigned when HAVE_LIBDW_SUPPORT is defined.
        (*arch).update_insn_state = Some(update_insn_state_powerpc);
        arch
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
