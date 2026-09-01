/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2017 Josh Poimboeuf <jpoimboe@redhat.com>
 */

/* Translated from objtool/include/objtool/check.h. */
/* Dependencies in the original header: <stdbool.h>, <objtool/cfi.h>, <objtool/arch.h>. */

pub const INSN_CHUNK_BITS: u32 = 8;
pub const INSN_CHUNK_SIZE: u32 = 1 << INSN_CHUNK_BITS;
pub const INSN_CHUNK_MAX: u32 = INSN_CHUNK_SIZE - 1;

pub const VISITED_BRANCH: u32 = 0x01;
pub const VISITED_BRANCH_UACCESS: u32 = 0x02;
pub const VISITED_BRANCH_MASK: u32 = 0x03;
pub const VISITED_UNRET: u32 = 0x04;

#[repr(C)]
pub struct insn_state {
    pub cfi: cfi_state,
    pub uaccess_stack: u32,
    pub uaccess: bool,
    pub df: bool,
    pub noinstr: bool,
    pub instr: i8,
}

#[repr(C)]
pub struct alt_group {
    /*
     * Pointer from a replacement group to the original group.  NULL if it
     * *is* the original group.
     */
    pub orig_group: *mut alt_group,

    /* First and last instructions in the group */
    pub first_insn: *mut instruction,
    pub last_insn: *mut instruction,
    pub nop: *mut instruction,

    /*
     * Byte-offset-addressed len-sized array of pointers to CFI structs.
     * This is shared with the other alt_groups in the same alternative.
     */
    pub cfi: *mut *mut cfi_state,

    pub ignore: bool,
    pub feature: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum alternative_type {
    ALT_TYPE_INSTRUCTIONS,
    ALT_TYPE_JUMP_TABLE,
    ALT_TYPE_EX_TABLE,
}

#[repr(C)]
pub struct alternative {
    pub next: *mut alternative,
    pub insn: *mut instruction,
    pub type_: alternative_type,
}

#[repr(C)]
pub struct instruction_jump_table {
    pub _jump_table: *mut reloc,
    pub _jump_table_size: u64,
}

#[repr(C)]
pub union instruction_call_or_jump_table {
    pub _call_dest: *mut symbol,
    pub jump_table: instruction_jump_table,
}

#[repr(C)]
pub struct instruction {
    pub hash: hlist_node,
    pub call_node: list_head,
    pub sec: *mut section,
    pub offset: u64,
    pub immediate: u64,

    pub len: u8,
    pub prev_len: u8,
    pub type_: u8,
    pub instr: i8,

    /*
     * Original C bitfields:
     * u32 idx             : INSN_CHUNK_BITS,
     *     immediate_len   : 4,
     *     dead_end        : 1,
     *     ignore_alts     : 1,
     *     hint            : 1,
     *     save            : 1,
     *     restore         : 1,
     *     retpoline_safe  : 1,
     *     noendbr         : 1,
     *     unret           : 1,
     *     visited         : 4,
     *     no_reloc        : 1,
     *     hole            : 1,
     *     fake            : 1,
     *     trace           : 1;
     *     / * 4 bit hole * /
     */
    pub bitfield: u32,

    pub alt_group: *mut alt_group,
    pub jump_dest: *mut instruction,
    pub first_jump_src: *mut instruction,
    pub call_or_jump_table: instruction_call_or_jump_table,
    pub alts: *mut alternative,
    pub _sym: *mut symbol,
    pub stack_ops: *mut stack_op,
    pub cfi: *mut cfi_state,
}

pub const INSTRUCTION_IDX_SHIFT: u32 = 0;
pub const INSTRUCTION_IDX_MASK: u32 = ((1u32 << INSN_CHUNK_BITS) - 1) << INSTRUCTION_IDX_SHIFT;
pub const INSTRUCTION_IMMEDIATE_LEN_SHIFT: u32 = INSTRUCTION_IDX_SHIFT + INSN_CHUNK_BITS;
pub const INSTRUCTION_IMMEDIATE_LEN_MASK: u32 = 0x0f << INSTRUCTION_IMMEDIATE_LEN_SHIFT;
pub const INSTRUCTION_DEAD_END_SHIFT: u32 = INSTRUCTION_IMMEDIATE_LEN_SHIFT + 4;
pub const INSTRUCTION_DEAD_END_MASK: u32 = 1 << INSTRUCTION_DEAD_END_SHIFT;
pub const INSTRUCTION_IGNORE_ALTS_SHIFT: u32 = INSTRUCTION_DEAD_END_SHIFT + 1;
pub const INSTRUCTION_IGNORE_ALTS_MASK: u32 = 1 << INSTRUCTION_IGNORE_ALTS_SHIFT;
pub const INSTRUCTION_HINT_SHIFT: u32 = INSTRUCTION_IGNORE_ALTS_SHIFT + 1;
pub const INSTRUCTION_HINT_MASK: u32 = 1 << INSTRUCTION_HINT_SHIFT;
pub const INSTRUCTION_SAVE_SHIFT: u32 = INSTRUCTION_HINT_SHIFT + 1;
pub const INSTRUCTION_SAVE_MASK: u32 = 1 << INSTRUCTION_SAVE_SHIFT;
pub const INSTRUCTION_RESTORE_SHIFT: u32 = INSTRUCTION_SAVE_SHIFT + 1;
pub const INSTRUCTION_RESTORE_MASK: u32 = 1 << INSTRUCTION_RESTORE_SHIFT;
pub const INSTRUCTION_RETPOLINE_SAFE_SHIFT: u32 = INSTRUCTION_RESTORE_SHIFT + 1;
pub const INSTRUCTION_RETPOLINE_SAFE_MASK: u32 = 1 << INSTRUCTION_RETPOLINE_SAFE_SHIFT;
pub const INSTRUCTION_NOENDBR_SHIFT: u32 = INSTRUCTION_RETPOLINE_SAFE_SHIFT + 1;
pub const INSTRUCTION_NOENDBR_MASK: u32 = 1 << INSTRUCTION_NOENDBR_SHIFT;
pub const INSTRUCTION_UNRET_SHIFT: u32 = INSTRUCTION_NOENDBR_SHIFT + 1;
pub const INSTRUCTION_UNRET_MASK: u32 = 1 << INSTRUCTION_UNRET_SHIFT;
pub const INSTRUCTION_VISITED_SHIFT: u32 = INSTRUCTION_UNRET_SHIFT + 1;
pub const INSTRUCTION_VISITED_MASK: u32 = 0x0f << INSTRUCTION_VISITED_SHIFT;
pub const INSTRUCTION_NO_RELOC_SHIFT: u32 = INSTRUCTION_VISITED_SHIFT + 4;
pub const INSTRUCTION_NO_RELOC_MASK: u32 = 1 << INSTRUCTION_NO_RELOC_SHIFT;
pub const INSTRUCTION_HOLE_SHIFT: u32 = INSTRUCTION_NO_RELOC_SHIFT + 1;
pub const INSTRUCTION_HOLE_MASK: u32 = 1 << INSTRUCTION_HOLE_SHIFT;
pub const INSTRUCTION_FAKE_SHIFT: u32 = INSTRUCTION_HOLE_SHIFT + 1;
pub const INSTRUCTION_FAKE_MASK: u32 = 1 << INSTRUCTION_FAKE_SHIFT;
pub const INSTRUCTION_TRACE_SHIFT: u32 = INSTRUCTION_FAKE_SHIFT + 1;
pub const INSTRUCTION_TRACE_MASK: u32 = 1 << INSTRUCTION_TRACE_SHIFT;

/*
 * Return the symbol associated with an instruction.  For alternative
 * replacements, return the symbol of the original code being replaced rather
 * than NULL.  insn->_sym reflects the actual location in the ELF file.
 */
pub unsafe fn insn_sym(insn: *mut instruction) -> *mut symbol {
    let mut sym = unsafe { (*insn)._sym };

    if (sym.is_null() || unsafe { !is_func_sym(sym) })
        && unsafe { !(*insn).alt_group.is_null() }
        && unsafe { !(*(*insn).alt_group).orig_group.is_null() }
    {
        sym = unsafe { (*(*(*(*insn).alt_group).orig_group).first_insn)._sym };
    }

    sym
}

pub unsafe fn insn_func(insn: *mut instruction) -> *mut symbol {
    let mut sym = unsafe { insn_sym(insn) };

    if !sym.is_null() && unsafe { (*sym).type_ != STT_FUNC } {
        sym = core::ptr::null_mut();
    }

    sym
}

pub unsafe fn is_static_jump(insn: *mut instruction) -> bool {
    unsafe {
        (*insn).type_ == INSN_JUMP_CONDITIONAL || (*insn).type_ == INSN_JUMP_UNCONDITIONAL
    }
}

pub unsafe fn is_dynamic_jump(insn: *mut instruction) -> bool {
    unsafe {
        (*insn).type_ == INSN_JUMP_DYNAMIC || (*insn).type_ == INSN_JUMP_DYNAMIC_CONDITIONAL
    }
}

pub unsafe fn is_jump(insn: *mut instruction) -> bool {
    unsafe { is_static_jump(insn) || is_dynamic_jump(insn) }
}

pub unsafe fn insn_call_dest(insn: *mut instruction) -> *mut symbol {
    if unsafe { (*insn).type_ == INSN_JUMP_DYNAMIC || (*insn).type_ == INSN_CALL_DYNAMIC } {
        return core::ptr::null_mut();
    }

    unsafe { (*insn).call_or_jump_table._call_dest }
}

unsafe extern "C" {
    pub fn find_insn(file: *mut objtool_file, sec: *mut section, offset: u64) -> *mut instruction;

    pub fn next_insn_same_sec(file: *mut objtool_file, insn: *mut instruction) -> *mut instruction;
    pub fn next_insn_same_func(file: *mut objtool_file, insn: *mut instruction) -> *mut instruction;

    pub fn insn_reloc(file: *mut objtool_file, insn: *mut instruction) -> *mut reloc;

    pub fn decode_file(file: *mut objtool_file) -> i32;
    pub fn free_insns(file: *mut objtool_file);

    pub fn objtool_disas_insn(insn: *mut instruction) -> *const core::ffi::c_char;

    pub static mut sym_name_max_len: usize;
    pub static mut objtool_disas_ctx: *mut disas_context;
    pub fn pv_ops_idx_off(symname: *const core::ffi::c_char) -> i32;
}

/*
 * Original C iteration macros:
 *
 * #define func_for_each_insn(file, func, insn)
 *     for (insn = find_insn(file, func->sec, func->offset);
 *          insn;
 *          insn = next_insn_same_func(file, insn))
 *
 * #define sec_for_each_insn(file, _sec, insn)
 *     for (insn = find_insn(file, _sec, 0);
 *          insn && insn->sec == _sec;
 *          insn = next_insn_same_sec(file, insn))
 *
 * #define sym_for_each_insn(file, sym, insn)
 *     for (insn = find_insn(file, sym->sec, sym->offset);
 *          insn && insn->offset < sym->offset + sym->len;
 *          insn = next_insn_same_sec(file, insn))
 */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
