// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Josh Poimboeuf <jpoimboe@redhat.com>
 */

// C header dependencies:
// <stdbool.h>
// <linux/list.h>
// <objtool/objtool.h>
// <objtool/cfi.h>

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum insn_type {
    INSN_JUMP_CONDITIONAL,
    INSN_JUMP_UNCONDITIONAL,
    INSN_JUMP_DYNAMIC,
    INSN_JUMP_DYNAMIC_CONDITIONAL,
    INSN_CALL,
    INSN_CALL_DYNAMIC,
    INSN_RETURN,
    INSN_SYSCALL,
    INSN_SYSRET,
    INSN_BUG,
    INSN_NOP,
    INSN_STAC,
    INSN_CLAC,
    INSN_STD,
    INSN_CLD,
    INSN_TRAP,
    INSN_ENDBR,
    INSN_LEA_RIP,
    INSN_OTHER,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum op_dest_type {
    OP_DEST_REG,
    OP_DEST_REG_INDIRECT,
    OP_DEST_MEM,
    OP_DEST_PUSH,
    OP_DEST_PUSHF,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_dest {
    pub type_: op_dest_type,
    pub reg: ::std::os::raw::c_uchar,
    pub offset: ::std::os::raw::c_int,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum op_src_type {
    OP_SRC_REG,
    OP_SRC_REG_INDIRECT,
    OP_SRC_CONST,
    OP_SRC_POP,
    OP_SRC_POPF,
    OP_SRC_ADD,
    OP_SRC_AND,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct op_src {
    pub type_: op_src_type,
    pub reg: ::std::os::raw::c_uchar,
    pub offset: ::std::os::raw::c_int,
}

#[repr(C)]
pub struct stack_op {
    pub next: *mut stack_op,
    pub dest: op_dest,
    pub src: op_src,
}

#[repr(C)]
pub struct instruction {
    _unused: [u8; 0],
}

extern "C" {
    pub fn arch_ftrace_match(name: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;

    pub fn arch_initial_func_cfi_state(state: *mut cfi_init_state);

    pub fn arch_decode_instruction(
        file: *mut objtool_file,
        sec: *const section,
        offset: ::std::os::raw::c_ulong,
        maxlen: ::std::os::raw::c_uint,
        insn: *mut instruction,
    ) -> ::std::os::raw::c_int;

    pub fn arch_jump_opcode_bytes(
        file: *mut objtool_file,
        insn: *mut instruction,
        buf: *mut ::std::os::raw::c_uchar,
    ) -> usize;

    pub fn arch_callee_saved_reg(reg: ::std::os::raw::c_uchar) -> bool;

    pub fn arch_jump_destination(insn: *mut instruction) -> ::std::os::raw::c_ulong;

    pub fn arch_insn_adjusted_addend(insn: *mut instruction, reloc: *mut reloc) -> s64;
    pub fn arch_adjusted_addend(reloc: *mut reloc) -> u64;

    pub fn arch_nop_insn(len: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;
    pub fn arch_ret_insn(len: ::std::os::raw::c_int) -> *const ::std::os::raw::c_char;

    pub fn arch_decode_hint_reg(
        sp_reg: u8,
        base: *mut ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn arch_is_retpoline(sym: *mut symbol) -> bool;
    pub fn arch_is_rethunk(sym: *mut symbol) -> bool;
    pub fn arch_is_embedded_insn(sym: *mut symbol) -> bool;

    pub fn arch_rewrite_retpolines(file: *mut objtool_file) -> ::std::os::raw::c_int;

    pub fn arch_pc_relative_reloc(reloc: *mut reloc) -> bool;
    pub fn arch_absolute_reloc(elf: *mut elf, reloc: *mut reloc) -> bool;

    pub fn arch_reloc_size(reloc: *mut reloc) -> ::std::os::raw::c_uint;
    pub fn arch_jump_table_sym_offset(
        reloc: *mut reloc,
        table: *mut reloc,
    ) -> ::std::os::raw::c_ulong;

    pub static arch_reg_name: [*const ::std::os::raw::c_char; CFI_NUM_REGS];
}

// Original C conditional: #ifdef DISAS
// C dependencies in that block:
// <bfd.h>
// <dis-asm.h>
#[cfg(DISAS)]
extern "C" {
    pub fn arch_disas_info_init(dinfo: *mut disassemble_info) -> ::std::os::raw::c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
