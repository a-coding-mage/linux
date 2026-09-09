/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2009 Matt Fleming <matt@console-pimps.org>
 */

/* CONFIG_DWARF_UNWINDER */

/* DWARF expression operations */
pub const DW_OP_addr: u8 = 0x03;
pub const DW_OP_deref: u8 = 0x06;
pub const DW_OP_const1u: u8 = 0x08;
pub const DW_OP_const1s: u8 = 0x09;
pub const DW_OP_const2u: u8 = 0x0a;
pub const DW_OP_const2s: u8 = 0x0b;
pub const DW_OP_const4u: u8 = 0x0c;
pub const DW_OP_const4s: u8 = 0x0d;
pub const DW_OP_const8u: u8 = 0x0e;
pub const DW_OP_const8s: u8 = 0x0f;
pub const DW_OP_constu: u8 = 0x10;
pub const DW_OP_consts: u8 = 0x11;
pub const DW_OP_dup: u8 = 0x12;
pub const DW_OP_drop: u8 = 0x13;
pub const DW_OP_over: u8 = 0x14;
pub const DW_OP_pick: u8 = 0x15;
pub const DW_OP_swap: u8 = 0x16;
pub const DW_OP_rot: u8 = 0x17;
pub const DW_OP_xderef: u8 = 0x18;
pub const DW_OP_abs: u8 = 0x19;
pub const DW_OP_and: u8 = 0x1a;
pub const DW_OP_div: u8 = 0x1b;
pub const DW_OP_minus: u8 = 0x1c;
pub const DW_OP_mod: u8 = 0x1d;
pub const DW_OP_mul: u8 = 0x1e;
pub const DW_OP_neg: u8 = 0x1f;
pub const DW_OP_not: u8 = 0x20;
pub const DW_OP_or: u8 = 0x21;
pub const DW_OP_plus: u8 = 0x22;
pub const DW_OP_plus_uconst: u8 = 0x23;
pub const DW_OP_shl: u8 = 0x24;
pub const DW_OP_shr: u8 = 0x25;
pub const DW_OP_shra: u8 = 0x26;
pub const DW_OP_xor: u8 = 0x27;
pub const DW_OP_skip: u8 = 0x2f;
pub const DW_OP_bra: u8 = 0x28;
pub const DW_OP_eq: u8 = 0x29;
pub const DW_OP_ge: u8 = 0x2a;
pub const DW_OP_gt: u8 = 0x2b;
pub const DW_OP_le: u8 = 0x2c;
pub const DW_OP_lt: u8 = 0x2d;
pub const DW_OP_ne: u8 = 0x2e;

/* Literal, register, and branch ranges retain their DWARF numeric layout. */
pub const DW_OP_lit0: u8 = 0x30;
pub const DW_OP_lit1: u8 = 0x31;
pub const DW_OP_lit2: u8 = 0x32;
pub const DW_OP_lit3: u8 = 0x33;
pub const DW_OP_lit4: u8 = 0x34;
pub const DW_OP_lit5: u8 = 0x35;
pub const DW_OP_lit6: u8 = 0x36;
pub const DW_OP_lit7: u8 = 0x37;
pub const DW_OP_lit8: u8 = 0x38;
pub const DW_OP_lit9: u8 = 0x39;
pub const DW_OP_lit10: u8 = 0x3a;
pub const DW_OP_lit11: u8 = 0x3b;
pub const DW_OP_lit12: u8 = 0x3c;
pub const DW_OP_lit13: u8 = 0x3d;
pub const DW_OP_lit14: u8 = 0x3e;
pub const DW_OP_lit15: u8 = 0x3f;
pub const DW_OP_lit16: u8 = 0x40;
pub const DW_OP_lit17: u8 = 0x41;
pub const DW_OP_lit18: u8 = 0x42;
pub const DW_OP_lit19: u8 = 0x43;
pub const DW_OP_lit20: u8 = 0x44;
pub const DW_OP_lit21: u8 = 0x45;
pub const DW_OP_lit22: u8 = 0x46;
pub const DW_OP_lit23: u8 = 0x47;
pub const DW_OP_lit24: u8 = 0x48;
pub const DW_OP_lit25: u8 = 0x49;
pub const DW_OP_lit26: u8 = 0x4a;
pub const DW_OP_lit27: u8 = 0x4b;
pub const DW_OP_lit28: u8 = 0x4c;
pub const DW_OP_lit29: u8 = 0x4d;
pub const DW_OP_lit30: u8 = 0x4e;
pub const DW_OP_lit31: u8 = 0x4f;
pub const DW_OP_reg0: u8 = 0x50;
pub const DW_OP_reg1: u8 = 0x51;
pub const DW_OP_reg2: u8 = 0x52;
pub const DW_OP_reg3: u8 = 0x53;
pub const DW_OP_reg4: u8 = 0x54;
pub const DW_OP_reg5: u8 = 0x55;
pub const DW_OP_reg6: u8 = 0x56;
pub const DW_OP_reg7: u8 = 0x57;
pub const DW_OP_reg8: u8 = 0x58;
pub const DW_OP_reg9: u8 = 0x59;
pub const DW_OP_reg10: u8 = 0x5a;
pub const DW_OP_reg11: u8 = 0x5b;
pub const DW_OP_reg12: u8 = 0x5c;
pub const DW_OP_reg13: u8 = 0x5d;
pub const DW_OP_reg14: u8 = 0x5e;
pub const DW_OP_reg15: u8 = 0x5f;
pub const DW_OP_reg16: u8 = 0x60;
pub const DW_OP_reg17: u8 = 0x61;
pub const DW_OP_reg18: u8 = 0x62;
pub const DW_OP_reg19: u8 = 0x63;
pub const DW_OP_reg20: u8 = 0x64;
pub const DW_OP_reg21: u8 = 0x65;
pub const DW_OP_reg22: u8 = 0x66;
pub const DW_OP_reg23: u8 = 0x67;
pub const DW_OP_reg24: u8 = 0x68;
pub const DW_OP_reg25: u8 = 0x69;
pub const DW_OP_reg26: u8 = 0x6a;
pub const DW_OP_reg27: u8 = 0x6b;
pub const DW_OP_reg28: u8 = 0x6c;
pub const DW_OP_reg29: u8 = 0x6d;
pub const DW_OP_reg30: u8 = 0x6e;
pub const DW_OP_reg31: u8 = 0x6f;
pub const DW_OP_breg0: u8 = 0x70;
pub const DW_OP_breg1: u8 = 0x71;
pub const DW_OP_breg2: u8 = 0x72;
pub const DW_OP_breg3: u8 = 0x73;
pub const DW_OP_breg4: u8 = 0x74;
pub const DW_OP_breg5: u8 = 0x75;
pub const DW_OP_breg6: u8 = 0x76;
pub const DW_OP_breg7: u8 = 0x77;
pub const DW_OP_breg8: u8 = 0x78;
pub const DW_OP_breg9: u8 = 0x79;
pub const DW_OP_breg10: u8 = 0x7a;
pub const DW_OP_breg11: u8 = 0x7b;
pub const DW_OP_breg12: u8 = 0x7c;
pub const DW_OP_breg13: u8 = 0x7d;
pub const DW_OP_breg14: u8 = 0x7e;
pub const DW_OP_breg15: u8 = 0x7f;
pub const DW_OP_breg16: u8 = 0x80;
pub const DW_OP_breg17: u8 = 0x81;
pub const DW_OP_breg18: u8 = 0x82;
pub const DW_OP_breg19: u8 = 0x83;
pub const DW_OP_breg20: u8 = 0x84;
pub const DW_OP_breg21: u8 = 0x85;
pub const DW_OP_breg22: u8 = 0x86;
pub const DW_OP_breg23: u8 = 0x87;
pub const DW_OP_breg24: u8 = 0x88;
pub const DW_OP_breg25: u8 = 0x89;
pub const DW_OP_breg26: u8 = 0x8a;
pub const DW_OP_breg27: u8 = 0x8b;
pub const DW_OP_breg28: u8 = 0x8c;
pub const DW_OP_breg29: u8 = 0x8d;
pub const DW_OP_breg30: u8 = 0x8e;
pub const DW_OP_breg31: u8 = 0x8f;
pub const DW_OP_regx: u8 = 0x90;
pub const DW_OP_fbreg: u8 = 0x91;
pub const DW_OP_bregx: u8 = 0x92;
pub const DW_OP_piece: u8 = 0x93;
pub const DW_OP_deref_size: u8 = 0x94;
pub const DW_OP_xderef_size: u8 = 0x95;
pub const DW_OP_nop: u8 = 0x96;
pub const DW_OP_push_object_address: u8 = 0x97;
pub const DW_OP_call2: u8 = 0x98;
pub const DW_OP_call4: u8 = 0x99;
pub const DW_OP_call_ref: u8 = 0x9a;
pub const DW_OP_form_tls_address: u8 = 0x9b;
pub const DW_OP_call_frame_cfa: u8 = 0x9c;
pub const DW_OP_bit_piece: u8 = 0x9d;
pub const DW_OP_lo_user: u8 = 0xe0;
pub const DW_OP_hi_user: u8 = 0xff;

pub const DW_EH_PE_absptr: u8 = 0x00;
pub const DW_EH_PE_omit: u8 = 0xff;
pub const DW_EH_PE_uleb128: u8 = 0x01;
pub const DW_EH_PE_udata2: u8 = 0x02;
pub const DW_EH_PE_udata4: u8 = 0x03;
pub const DW_EH_PE_udata8: u8 = 0x04;
pub const DW_EH_PE_sleb128: u8 = 0x09;
pub const DW_EH_PE_sdata2: u8 = 0x0a;
pub const DW_EH_PE_sdata4: u8 = 0x0b;
pub const DW_EH_PE_sdata8: u8 = 0x0c;
pub const DW_EH_PE_signed: u8 = 0x09;
pub const DW_EH_PE_pcrel: u8 = 0x10;
pub const DWARF_ARCH_RA_REG: u32 = 17;

/* Kernel dependencies supplied by other translated files. */
extern "C" {
    pub type list_head;
    pub type rb_node;
    pub type module;
    pub type Elf_Ehdr;
    pub type Elf_Shdr;
}

/* Read either the frame pointer (r14) or the stack pointer (r15). */
#[inline(always)]
pub unsafe fn dwarf_read_arch_reg(reg: u32) -> usize {
    let mut value: usize;
    match reg {
        14 => core::arch::asm!("mov r14, {0}", out(reg) value),
        15 => core::arch::asm!("mov r15, {0}", out(reg) value),
        _ => panic!("BUG"),
    }
    value
}

#[repr(C)]
pub struct dwarf_cie {
    pub length: usize, pub cie_id: usize, pub version: u8,
    pub augmentation: *const i8, pub code_alignment_factor: u32,
    pub data_alignment_factor: i32, pub return_address_reg: u32,
    pub initial_instructions: *mut u8, pub instructions_end: *mut u8,
    pub encoding: u8, pub cie_pointer: usize, pub flags: usize,
    pub link: list_head, pub node: rb_node,
}
pub const DWARF_CIE_Z_AUGMENTATION: usize = 1 << 0;

#[repr(C)]
pub struct dwarf_fde {
    pub length: usize, pub cie_pointer: usize, pub cie: *mut dwarf_cie,
    pub initial_location: usize, pub address_range: usize,
    pub instructions: *mut u8, pub end: *mut u8, pub link: list_head,
    pub node: rb_node,
}

#[repr(C)]
pub struct dwarf_frame {
    pub prev: *mut dwarf_frame, pub next: *mut dwarf_frame, pub pc: usize,
    pub reg_list: list_head, pub cfa: usize, pub cfa_register: u32,
    pub cfa_offset: u32, pub cfa_expr: *mut u8, pub cfa_expr_len: u32,
    pub flags: usize, pub return_addr: usize,
}
pub const DWARF_FRAME_CFA_REG_OFFSET: usize = 1 << 0;
pub const DWARF_FRAME_CFA_REG_EXP: usize = 1 << 1;

#[repr(C)]
pub struct dwarf_reg {
    pub link: list_head, pub number: u32, pub addr: usize, pub flags: usize,
}
pub const DWARF_REG_OFFSET: usize = 1 << 0;
pub const DWARF_VAL_OFFSET: usize = 1 << 1;
pub const DWARF_UNDEFINED: usize = 1 << 2;

pub const DW_CFA_advance_loc: u8 = 0x40; pub const DW_CFA_offset: u8 = 0x80;
pub const DW_CFA_restore: u8 = 0xc0; pub const DW_CFA_nop: u8 = 0x00;
pub const DW_CFA_set_loc: u8 = 0x01; pub const DW_CFA_advance_loc1: u8 = 0x02;
pub const DW_CFA_advance_loc2: u8 = 0x03; pub const DW_CFA_advance_loc4: u8 = 0x04;
pub const DW_CFA_offset_extended: u8 = 0x05; pub const DW_CFA_restore_extended: u8 = 0x06;
pub const DW_CFA_undefined: u8 = 0x07; pub const DW_CFA_same_value: u8 = 0x08;
pub const DW_CFA_register: u8 = 0x09; pub const DW_CFA_remember_state: u8 = 0x0a;
pub const DW_CFA_restore_state: u8 = 0x0b; pub const DW_CFA_def_cfa: u8 = 0x0c;
pub const DW_CFA_def_cfa_register: u8 = 0x0d; pub const DW_CFA_def_cfa_offset: u8 = 0x0e;
pub const DW_CFA_def_cfa_expression: u8 = 0x0f; pub const DW_CFA_expression: u8 = 0x10;
pub const DW_CFA_offset_extended_sf: u8 = 0x11; pub const DW_CFA_def_cfa_sf: u8 = 0x12;
pub const DW_CFA_def_cfa_offset_sf: u8 = 0x13; pub const DW_CFA_val_offset: u8 = 0x14;
pub const DW_CFA_val_offset_sf: u8 = 0x15; pub const DW_CFA_val_expression: u8 = 0x16;
pub const DW_CFA_lo_user: u8 = 0x1c; pub const DW_CFA_hi_user: u8 = 0x3f;
pub const DW_CFA_GNU_args_size: u8 = 0x2e;
pub const DW_CFA_GNU_negative_offset_extended: u8 = 0x2f;

#[inline]
pub fn DW_CFA_opcode(insn: usize) -> u32 { (insn & 0xc0) as u32 }
#[inline]
pub fn DW_CFA_operand(insn: usize) -> u32 { (insn & 0x3f) as u32 }

pub const DW_EH_FRAME_CIE: usize = 0;
pub const DW_CIE_ID: u32 = 0xffff_ffff;
pub const DW64_CIE_ID: u64 = 0xffff_ffff_ffff_ffff;
pub const DW_EXT_LO: u32 = 0xffff_fff0;
pub const DW_EXT_HI: u32 = 0xffff_ffff;
pub const DW_EXT_DWARF64: u32 = DW_EXT_HI;

/* The disabled-unwinder branch provides an empty inline initializer and
 * module_dwarf_finalize(...)=0/module_dwarf_cleanup(...)=do { } while (0). */
#[inline]
pub fn dwarf_unwinder_init() {}

unsafe extern "C" {
    pub fn dwarf_unwind_stack(arg1: usize, arg2: *mut dwarf_frame) -> *mut dwarf_frame;
    pub fn dwarf_free_frame(frame: *mut dwarf_frame);
    pub fn module_dwarf_finalize(hdr: *const Elf_Ehdr, sechdrs: *const Elf_Shdr, me: *mut module) -> i32;
    pub fn module_dwarf_cleanup(module: *mut module);
}

/* CFI_STARTPROC, CFI_ENDPROC, CFI_DEF_CFA, CFI_REGISTER, CFI_REL_OFFSET,
 * and CFI_UNDEFINED expand to their corresponding assembler .cfi directives.
 * With CONFIG_DWARF_UNWINDER disabled they expand to the assembler comment
 * character and the no-op inline unwinder initializer/module macros apply.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
