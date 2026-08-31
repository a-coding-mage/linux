// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2015 Josh Poimboeuf <jpoimboe@redhat.com>
 */

// Translated from objtool/arch/x86/decode.c. C include dependencies are
// expected to be supplied by the surrounding crate/module.

use core::ffi::{c_char, c_int, c_uchar, c_ulong, c_void};
use core::ptr;

type bool_ = bool;
type u8 = u8;
type u64 = u64;
type s64 = i64;
type size_t = usize;

extern "C" {
    static opts: objtool_options;

    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strncmp(s1: *const c_char, s2: *const c_char, n: size_t) -> c_int;
    fn memcpy(dest: *mut c_void, src: *const c_void, n: size_t) -> *mut c_void;
    fn calloc(nmemb: size_t, size: size_t) -> *mut c_void;

    fn insn_decode(insn: *mut insn, kaddr: *const c_void, buf_len: c_ulong, mode: c_int) -> c_int;
    fn insn_last_prefix_id(insn: *mut insn) -> c_int;

    fn reloc_addend(reloc: *mut reloc) -> s64;
    fn reloc_offset(reloc: *mut reloc) -> c_ulong;
    fn reloc_type(reloc: *mut reloc) -> c_uint;
    fn sec_size(sec: *mut section) -> c_ulong;
    fn is_text_sec(sec: *mut section) -> bool_;
    fn find_reloc_by_dest(elf: *const elf, sec: *mut c_void, offset: c_ulong) -> *mut reloc;
    fn pv_ops_idx_off(name: *const c_char) -> c_int;
    fn find_symbol_by_offset(sec: *mut section, offset: s64) -> *mut symbol;
    fn objtool_pv_add(file: *mut objtool_file, idx: c_int, func: *mut symbol);
    fn find_symbol_containing(sec: *const section, offset: c_ulong) -> *mut symbol;

    #[cfg(DISAS)]
    fn disas_info_init(
        dinfo: *mut disassemble_info,
        arch: c_int,
        mach32: c_int,
        mach64: c_int,
        syntax: *const c_char,
    ) -> c_int;
}

type c_uint = u32;

#[no_mangle]
pub static arch_reg_name: [*const c_char; CFI_NUM_REGS as usize] = [
    b"rax\0".as_ptr() as *const c_char,
    b"rcx\0".as_ptr() as *const c_char,
    b"rdx\0".as_ptr() as *const c_char,
    b"rbx\0".as_ptr() as *const c_char,
    b"rsp\0".as_ptr() as *const c_char,
    b"rbp\0".as_ptr() as *const c_char,
    b"rsi\0".as_ptr() as *const c_char,
    b"rdi\0".as_ptr() as *const c_char,
    b"r8\0".as_ptr() as *const c_char,
    b"r9\0".as_ptr() as *const c_char,
    b"r10\0".as_ptr() as *const c_char,
    b"r11\0".as_ptr() as *const c_char,
    b"r12\0".as_ptr() as *const c_char,
    b"r13\0".as_ptr() as *const c_char,
    b"r14\0".as_ptr() as *const c_char,
    b"r15\0".as_ptr() as *const c_char,
    b"ra\0".as_ptr() as *const c_char,
];

#[no_mangle]
pub unsafe extern "C" fn arch_ftrace_match(name: *const c_char) -> c_int {
    (strcmp(name, b"__fentry__\0".as_ptr() as *const c_char) == 0) as c_int
}

unsafe fn is_x86_64(elf: *const elf) -> c_int {
    match (*elf).ehdr.e_machine as c_int {
        EM_X86_64 => 1,
        EM_386 => 0,
        _ => {
            ERROR!(b"unexpected ELF machine type %d\0".as_ptr() as *const c_char, (*elf).ehdr.e_machine);
            -1
        }
    }
}

#[no_mangle]
pub extern "C" fn arch_callee_saved_reg(reg: c_uchar) -> bool_ {
    match reg as c_int {
        CFI_BP | CFI_BX | CFI_R12 | CFI_R13 | CFI_R14 | CFI_R15 => true,
        CFI_AX | CFI_CX | CFI_DX | CFI_SI | CFI_DI | CFI_SP | CFI_R8 | CFI_R9 | CFI_R10
        | CFI_R11 | CFI_RA | _ => false,
    }
}

/* Undo the effects of __pa_symbol() if necessary */
unsafe fn phys_to_virt(pa: c_ulong) -> c_ulong {
    let mut va: s64 = pa as s64;

    if va > 0 {
        va &= !(0x80000000 as s64);
    }

    va as c_ulong
}

#[no_mangle]
pub unsafe extern "C" fn arch_insn_adjusted_addend(
    insn: *mut instruction,
    reloc: *mut reloc,
) -> s64 {
    let mut addend: s64 = reloc_addend(reloc);

    if arch_pc_relative_reloc(reloc) {
        addend += (*insn).offset as s64 + (*insn).len as s64 - reloc_offset(reloc) as s64;
    }

    phys_to_virt(addend as c_ulong) as s64
}

unsafe fn scan_for_insn(
    sec: *mut section,
    offset: c_ulong,
    insn_off: *mut c_ulong,
    insn_len: *mut c_uint,
) {
    let mut o: c_ulong = 0;
    let mut insn: insn = core::mem::zeroed();

    loop {
        insn_decode(
            &mut insn,
            (*(*sec).data).d_buf.add(o as usize) as *const c_void,
            sec_size(sec) - o,
            INSN_MODE_64,
        );

        if o + insn.length as c_ulong > offset {
            *insn_off = o;
            *insn_len = insn.length;
            return;
        }

        o += insn.length as c_ulong;
    }
}

#[no_mangle]
pub unsafe extern "C" fn arch_adjusted_addend(reloc: *mut reloc) -> u64 {
    let typ: c_uint = reloc_type(reloc);
    let addend: s64 = reloc_addend(reloc);
    let mut insn_off: c_ulong = 0;
    let mut insn_len: c_uint = 0;

    if typ == R_X86_64_PLT32 {
        return (addend + 4) as u64;
    }

    if typ != R_X86_64_PC32 || !is_text_sec((*(*reloc).sec).base) {
        return addend as u64;
    }

    scan_for_insn((*(*reloc).sec).base, reloc_offset(reloc), &mut insn_off, &mut insn_len);

    (addend + insn_off as s64 + insn_len as s64 - reloc_offset(reloc) as s64) as u64
}

#[no_mangle]
pub unsafe extern "C" fn arch_jump_destination(insn: *mut instruction) -> c_ulong {
    ((*insn).offset as s64 + (*insn).len as s64 + (*insn).immediate) as c_ulong
}

#[no_mangle]
pub unsafe extern "C" fn arch_pc_relative_reloc(reloc: *mut reloc) -> bool_ {
    /*
     * All relocation types where P (the address of the target)
     * is included in the computation.
     */
    match reloc_type(reloc) {
        R_X86_64_PC8 | R_X86_64_PC16 | R_X86_64_PC32 | R_X86_64_PC64 | R_X86_64_PLT32
        | R_X86_64_GOTPC32 | R_X86_64_GOTPCREL => true,
        _ => false,
    }
}

unsafe fn add_stack_op(ops_list: &mut *mut *mut stack_op) -> *mut stack_op {
    let op = calloc(1, core::mem::size_of::<stack_op>()) as *mut stack_op;
    if op.is_null() {
        return ptr::null_mut();
    }
    **ops_list = op;
    *ops_list = &mut (*op).next;
    op
}

/*
 * Helpers to decode ModRM/SIB:
 *
 * r/m| AX  CX  DX  BX |  SP |  BP |  SI  DI |
 *    | R8  R9 R10 R11 | R12 | R13 | R14 R15 |
 * Mod+----------------+-----+-----+---------+
 * 00 |    [r/m]       |[SIB]|[IP+]|  [r/m]  |
 * 01 |  [r/m + d8]    |[S+d]|   [r/m + d8]  |
 * 10 |  [r/m + d32]   |[S+D]|   [r/m + d32] |
 * 11 |                   r/ m               |
 */

unsafe fn has_notrack_prefix(insn: *mut insn) -> bool_ {
    let mut i: c_int = 0;

    while i < (*insn).prefixes.nbytes as c_int {
        if (*insn).prefixes.bytes[i as usize] == 0x3e {
            return true;
        }
        i += 1;
    }

    false
}

#[no_mangle]
pub unsafe extern "C" fn arch_decode_instruction(
    file: *mut objtool_file,
    sec: *const section,
    offset: c_ulong,
    maxlen: c_uint,
    insn: *mut instruction,
) -> c_int {
    let mut ops_list: *mut *mut stack_op = &mut (*insn).stack_ops;
    let elf: *const elf = (*file).elf;
    let mut ins: insn = core::mem::zeroed();
    let x86_64: c_int;
    let ret: c_int;
    let mut op1: c_uchar;
    let mut op2: c_uchar;
    let mut op3: c_uchar;
    let prefix: c_uchar;
    let mut rex: c_uchar = 0;
    let mut rex_b: c_uchar = 0;
    let mut rex_r: c_uchar = 0;
    let mut rex_w: c_uchar = 0;
    let mut rex_x: c_uchar = 0;
    let mut modrm: c_uchar = 0;
    let mut modrm_mod: c_uchar = 0;
    let mut modrm_rm: c_uchar = 0;
    let mut modrm_reg: c_uchar = 0;
    let mut sib: c_uchar = 0;
    /* sib_scale = 0, */
    let mut sib_index: c_uchar = 0;
    let mut sib_base: c_uchar = 0;
    let mut sym: *mut symbol;
    let mut imm: u64;

    x86_64 = is_x86_64(elf);
    if x86_64 == -1 {
        return -1;
    }

    ret = insn_decode(
        &mut ins,
        (*(*sec).data).d_buf.add(offset as usize) as *const c_void,
        maxlen as c_ulong,
        if x86_64 != 0 { INSN_MODE_64 } else { INSN_MODE_32 },
    );
    if ret < 0 {
        ERROR!(b"can't decode instruction at %s:0x%lx\0".as_ptr() as *const c_char, (*sec).name, offset);
        return -1;
    }

    (*insn).len = ins.length;
    (*insn).type_ = INSN_OTHER;

    if ins.vex_prefix.nbytes != 0 {
        return 0;
    }

    prefix = ins.prefixes.bytes[0];

    op1 = ins.opcode.bytes[0];
    op2 = ins.opcode.bytes[1];
    op3 = ins.opcode.bytes[2];

    if ins.rex_prefix.nbytes != 0 {
        rex = ins.rex_prefix.bytes[0];
        rex_w = X86_REX_W(rex) >> 3;
        rex_r = X86_REX_R(rex) >> 2;
        rex_x = X86_REX_X(rex) >> 1;
        rex_b = X86_REX_B(rex);
    }

    if ins.modrm.nbytes != 0 {
        modrm = ins.modrm.bytes[0];
        modrm_mod = X86_MODRM_MOD(modrm);
        modrm_reg = X86_MODRM_REG(modrm) + 8 * rex_r;
        modrm_rm = X86_MODRM_RM(modrm) + 8 * rex_b;
    }

    if ins.sib.nbytes != 0 {
        sib = ins.sib.bytes[0];
        /* sib_scale = X86_SIB_SCALE(sib); */
        sib_index = X86_SIB_INDEX(sib) + 8 * rex_x;
        sib_base = X86_SIB_BASE(sib) + 8 * rex_b;
    }

    let mut mod_is_mem = || modrm_mod != 3;
    let mut mod_is_reg = || modrm_mod == 3;
    let mut is_RIP = || (modrm_rm & 7) as c_int == CFI_BP && modrm_mod == 0;
    let mut have_SIB = || (modrm_rm & 7) as c_int == CFI_SP && mod_is_mem();
    let mut rm_is = |reg: c_int| -> bool {
        if have_SIB() {
            sib_base as c_int == reg
                && sib_index as c_int == CFI_SP
                && (sib_base as c_int != CFI_BP || modrm_mod != 0)
        } else {
            modrm_rm as c_int == reg
        }
    };
    let mut rm_is_mem = |reg: c_int| -> bool { mod_is_mem() && !is_RIP() && rm_is(reg) };
    let mut rm_is_reg = |reg: c_int| -> bool { mod_is_reg() && modrm_rm as c_int == reg };

    match op1 {
        0x1 | 0x29 => {
            if rex_w != 0 && rm_is_reg(CFI_SP) {
                /* add/sub reg, %rsp */
                let op = add_stack_op(&mut ops_list);
                if op.is_null() {
                    return -1;
                }
                (*op).src.type_ = OP_SRC_ADD;
                (*op).src.reg = modrm_reg;
                (*op).dest.type_ = OP_DEST_REG;
                (*op).dest.reg = CFI_SP as c_uchar;
            }
        }
        0x50..=0x57 => {
            /* push reg */
            let op = add_stack_op(&mut ops_list);
            if op.is_null() {
                return -1;
            }
            (*op).src.type_ = OP_SRC_REG;
            (*op).src.reg = (op1 & 0x7) + 8 * rex_b;
            (*op).dest.type_ = OP_DEST_PUSH;
        }
        0x58..=0x5f => {
            /* pop reg */
            let op = add_stack_op(&mut ops_list);
            if op.is_null() {
                return -1;
            }
            (*op).src.type_ = OP_SRC_POP;
            (*op).dest.type_ = OP_DEST_REG;
            (*op).dest.reg = (op1 & 0x7) + 8 * rex_b;
        }
        0x68 | 0x6a => {
            /* push immediate */
            let op = add_stack_op(&mut ops_list);
            if op.is_null() {
                return -1;
            }
            (*op).src.type_ = OP_SRC_CONST;
            (*op).dest.type_ = OP_DEST_PUSH;
        }
        0x70..=0x7f => {
            (*insn).type_ = INSN_JUMP_CONDITIONAL;
        }
        0x80..=0x83 => {
            /*
             * 1000 00sw : mod OP r/m : immediate
             *
             * s - sign extend immediate
             * w - imm8 / imm32
             *
             * OP: 000 ADD    100 AND
             *     001 OR     101 SUB
             *     010 ADC    110 XOR
             *     011 SBB    111 CMP
             */
            if rex_w != 0 && rm_is_reg(CFI_SP) {
                imm = ins.immediate.value;
                if (op1 & 2) != 0 {
                    if (op1 & 1) != 0 {
                        imm <<= 32;
                        imm = ((imm as s64) >> 32) as u64;
                    } else {
                        imm <<= 56;
                        imm = ((imm as s64) >> 56) as u64;
                    }
                }

                match modrm_reg & 7 {
                    5 | 0 => {
                        if (modrm_reg & 7) == 5 {
                            imm = (-(imm as s64)) as u64;
                        }
                        /* add/sub imm, %rsp */
                        let op = add_stack_op(&mut ops_list);
                        if op.is_null() {
                            return -1;
                        }
                        (*op).src.type_ = OP_SRC_ADD;
                        (*op).src.reg = CFI_SP as c_uchar;
                        (*op).src.offset = imm as s64;
                        (*op).dest.type_ = OP_DEST_REG;
                        (*op).dest.reg = CFI_SP as c_uchar;
                    }
                    4 => {
                        /* and imm, %rsp */
                        let op = add_stack_op(&mut ops_list);
                        if op.is_null() {
                            return -1;
                        }
                        (*op).src.type_ = OP_SRC_AND;
                        (*op).src.reg = CFI_SP as c_uchar;
                        (*op).src.offset = ins.immediate.value as s64;
                        (*op).dest.type_ = OP_DEST_REG;
                        (*op).dest.reg = CFI_SP as c_uchar;
                    }
                    _ => {}
                }
            }
        }
        0x89 => {
            if rex_w != 0 {
                if mod_is_reg() {
                    /* mov reg, reg */
                    let op = add_stack_op(&mut ops_list);
                    if op.is_null() {
                        return -1;
                    }
                    (*op).src.type_ = OP_SRC_REG;
                    (*op).src.reg = modrm_reg;
                    (*op).dest.type_ = OP_DEST_REG;
                    (*op).dest.reg = modrm_rm;
                } else if !is_RIP() {
                    if have_SIB() {
                        modrm_rm = sib_base;
                    }
                    if !have_SIB() || sib_index as c_int == CFI_SP {
                        if modrm_reg as c_int == CFI_SP {
                            /* mov %rsp, disp(%reg) */
                            let op = add_stack_op(&mut ops_list);
                            if op.is_null() {
                                return -1;
                            }
                            (*op).src.type_ = OP_SRC_REG;
                            (*op).src.reg = CFI_SP as c_uchar;
                            (*op).dest.type_ = OP_DEST_REG_INDIRECT;
                            (*op).dest.reg = modrm_rm;
                            (*op).dest.offset = ins.displacement.value as s64;
                        } else {
                            if decode_mov_reg_to_bp_sp_mem(
                                &mut ops_list,
                                modrm_reg,
                                modrm_mod,
                                modrm_rm,
                                sib_base,
                                sib_index,
                                &ins,
                            ) < 0 {
                                return -1;
                            }
                        }
                    }
                }
            }
        }
        0x88 => {
            if rex_w != 0 {
                if decode_mov_reg_to_bp_sp_mem(
                    &mut ops_list,
                    modrm_reg,
                    modrm_mod,
                    modrm_rm,
                    sib_base,
                    sib_index,
                    &ins,
                ) < 0 {
                    return -1;
                }
            }
        }
        0x8b => {
            if rex_w != 0 {
                if rm_is_mem(CFI_BP) {
                    /* mov disp(%rbp), reg */
                    let op = add_stack_op(&mut ops_list);
                    if op.is_null() {
                        return -1;
                    }
                    (*op).src.type_ = OP_SRC_REG_INDIRECT;
                    (*op).src.reg = CFI_BP as c_uchar;
                    (*op).src.offset = ins.displacement.value as s64;
                    (*op).dest.type_ = OP_DEST_REG;
                    (*op).dest.reg = modrm_reg;
                } else if rm_is_mem(CFI_SP) {
                    /* mov disp(%rsp), reg */
                    let op = add_stack_op(&mut ops_list);
                    if op.is_null() {
                        return -1;
                    }
                    (*op).src.type_ = OP_SRC_REG_INDIRECT;
                    (*op).src.reg = CFI_SP as c_uchar;
                    (*op).src.offset = ins.displacement.value as s64;
                    (*op).dest.type_ = OP_DEST_REG;
                    (*op).dest.reg = modrm_reg;
                }
            }
        }
        0x8d => {
            if mod_is_reg() {
                WARN!(b"invalid LEA encoding at %s:0x%lx\0".as_ptr() as *const c_char, (*sec).name, offset);
            } else if rex_w != 0 {
                if have_SIB() {
                    modrm_rm = sib_base;
                }
                if !have_SIB() || sib_index as c_int == CFI_SP {
                    if is_RIP() {
                        (*insn).type_ = INSN_LEA_RIP;
                    } else {
                        /* lea disp(%src), %dst */
                        let op = add_stack_op(&mut ops_list);
                        if op.is_null() {
                            return -1;
                        }
                        (*op).src.offset = ins.displacement.value as s64;
                        if (*op).src.offset == 0 {
                            /* lea (%src), %dst */
                            (*op).src.type_ = OP_SRC_REG;
                        } else {
                            /* lea disp(%src), %dst */
                            (*op).src.type_ = OP_SRC_ADD;
                        }
                        (*op).src.reg = modrm_rm;
                        (*op).dest.type_ = OP_DEST_REG;
                        (*op).dest.reg = modrm_reg;
                    }
                }
            }
        }
        0x8f => {
            /* pop to mem */
            let op = add_stack_op(&mut ops_list);
            if op.is_null() {
                return -1;
            }
            (*op).src.type_ = OP_SRC_POP;
            (*op).dest.type_ = OP_DEST_MEM;
        }
        0x90 => {
            if rex_b == 0 && prefix != 0xf3 {
                (*insn).type_ = INSN_NOP;
            }
        }
        0x9c => {
            /* pushf */
            let op = add_stack_op(&mut ops_list);
            if op.is_null() {
                return -1;
            }
            (*op).src.type_ = OP_SRC_CONST;
            (*op).dest.type_ = OP_DEST_PUSHF;
        }
        0x9d => {
            /* popf */
            let op = add_stack_op(&mut ops_list);
            if op.is_null() {
                return -1;
            }
            (*op).src.type_ = OP_SRC_POPF;
            (*op).dest.type_ = OP_DEST_MEM;
        }
        0x0f => {
            if op2 == 0x01 {
                match insn_last_prefix_id(&mut ins) {
                    INAT_PFX_REPE | INAT_PFX_REPNE => {
                        if modrm == 0xca {
                            /* eretu/erets */
                            (*insn).type_ = INSN_SYSRET;
                        }
                    }
                    _ => {
                        if modrm == 0xca {
                            (*insn).type_ = INSN_CLAC;
                        } else if modrm == 0xcb {
                            (*insn).type_ = INSN_STAC;
                        }
                    }
                }
            } else if op2 >= 0x80 && op2 <= 0x8f {
                (*insn).type_ = INSN_JUMP_CONDITIONAL;
            } else if op2 == 0x05 || op2 == 0x34 {
                /* syscall, sysenter */
                (*insn).type_ = INSN_SYSCALL;
            } else if op2 == 0x07 || op2 == 0x35 {
                /* sysret, sysexit */
                (*insn).type_ = INSN_SYSRET;
            } else if op2 == 0x0b || op2 == 0xb9 {
                /* ud2, ud1 */
                (*insn).type_ = INSN_BUG;
            } else if op2 == 0x1f {
                /* 0f 1f /0 := NOPL */
                if modrm_reg == 0 {
                    (*insn).type_ = INSN_NOP;
                }
            } else if op2 == 0x1e {
                if prefix == 0xf3 && (modrm == 0xfa || modrm == 0xfb) {
                    (*insn).type_ = INSN_ENDBR;
                }
            } else if op2 == 0x38 && op3 == 0xf8 {
                if ins.prefixes.nbytes == 1 && ins.prefixes.bytes[0] == 0xf2 {
                    /* ENQCMD cannot be used in the kernel. */
                    WARN!(b"ENQCMD instruction at %s:%lx\0".as_ptr() as *const c_char, (*sec).name, offset);
                }
            } else if op2 == 0xa0 || op2 == 0xa8 {
                /* push fs/gs */
                let op = add_stack_op(&mut ops_list);
                if op.is_null() {
                    return -1;
                }
                (*op).src.type_ = OP_SRC_CONST;
                (*op).dest.type_ = OP_DEST_PUSH;
            } else if op2 == 0xa1 || op2 == 0xa9 {
                /* pop fs/gs */
                let op = add_stack_op(&mut ops_list);
                if op.is_null() {
                    return -1;
                }
                (*op).src.type_ = OP_SRC_POP;
                (*op).dest.type_ = OP_DEST_MEM;
            }
        }
        0xc9 => {
            /*
             * leave
             *
             * equivalent to:
             * mov bp, sp
             * pop bp
             */
            let op = add_stack_op(&mut ops_list);
            if op.is_null() {
                return -1;
            }
            (*op).src.type_ = OP_SRC_REG;
            (*op).src.reg = CFI_BP as c_uchar;
            (*op).dest.type_ = OP_DEST_REG;
            (*op).dest.reg = CFI_SP as c_uchar;

            let op = add_stack_op(&mut ops_list);
            if op.is_null() {
                return -1;
            }
            (*op).src.type_ = OP_SRC_POP;
            (*op).dest.type_ = OP_DEST_REG;
            (*op).dest.reg = CFI_BP as c_uchar;
        }
        0xcc => {
            /* int3 */
            (*insn).type_ = INSN_TRAP;
        }
        0xe3 => {
            /* jecxz/jrcxz */
            (*insn).type_ = INSN_JUMP_CONDITIONAL;
        }
        0xe9 | 0xeb => {
            (*insn).type_ = INSN_JUMP_UNCONDITIONAL;
        }
        0xc2 | 0xc3 => {
            (*insn).type_ = INSN_RETURN;
        }
        0xc7 => {
            /* mov imm, r/m */
            if !opts.noinstr {
            } else if ins.length == 3 + 4 + 4
                && strncmp((*sec).name, b".init.text\0".as_ptr() as *const c_char, 10) == 0
            {
                let immr: *mut reloc;
                let disp: *mut reloc;
                let mut func: *mut symbol;
                let mut idx: c_int;

                immr = find_reloc_by_dest(elf, sec as *mut c_void, offset + 3);
                disp = find_reloc_by_dest(elf, sec as *mut c_void, offset + 7);

                if !immr.is_null()
                    && strncmp((*(*immr).sym).name, b"pv_ops\0".as_ptr() as *const c_char, 6) == 0
                {
                    idx = pv_ops_idx_off((*(*immr).sym).name);
                    if idx >= 0 {
                        idx += ((reloc_addend(immr) + 8) as usize / core::mem::size_of::<*mut c_void>())
                            as c_int;

                        func = (*disp).sym;
                        if (*(*disp).sym).type_ == STT_SECTION {
                            func = find_symbol_by_offset((*(*disp).sym).sec, reloc_addend(disp));
                        }
                        if func.is_null() {
                            ERROR!(b"no func for pv_ops[]\0".as_ptr() as *const c_char);
                            return -1;
                        }

                        objtool_pv_add(file, idx, func);
                    }
                }
            }
        }
        0xcf => {
            /* iret */
            /*
             * Handle sync_core(), which has an IRET to self.
             * All other IRET are in STT_NONE entry code.
             */
            sym = find_symbol_containing(sec, offset);
            if !sym.is_null() && (*sym).type_ == STT_FUNC {
                let op = add_stack_op(&mut ops_list);
                if op.is_null() {
                    return -1;
                }
                /* add $40, %rsp */
                (*op).src.type_ = OP_SRC_ADD;
                (*op).src.reg = CFI_SP as c_uchar;
                (*op).src.offset = 5 * 8;
                (*op).dest.type_ = OP_DEST_REG;
                (*op).dest.reg = CFI_SP as c_uchar;
            } else {
                (*insn).type_ = INSN_SYSRET;
            }
        }
        0xca | 0xcb => {
            /* retf */
            (*insn).type_ = INSN_SYSRET;
        }
        0xd6 => {
            /* udb */
            (*insn).type_ = INSN_BUG;
        }
        0xe0 | 0xe1 | 0xe2 => {
            /* loopne/loope/loop */
            (*insn).type_ = INSN_JUMP_CONDITIONAL;
        }
        0xe8 => {
            (*insn).type_ = INSN_CALL;
            /*
             * For the impact on the stack, a CALL behaves like
             * a PUSH of an immediate value (the return address).
             */
            let op = add_stack_op(&mut ops_list);
            if op.is_null() {
                return -1;
            }
            (*op).src.type_ = OP_SRC_CONST;
            (*op).dest.type_ = OP_DEST_PUSH;
        }
        0xfc => {
            (*insn).type_ = INSN_CLD;
        }
        0xfd => {
            (*insn).type_ = INSN_STD;
        }
        0xff => {
            if modrm_reg == 2 || modrm_reg == 3 {
                (*insn).type_ = INSN_CALL_DYNAMIC;
                if has_notrack_prefix(&mut ins) {
                    WARN!(b"notrack prefix found at %s:0x%lx\0".as_ptr() as *const c_char, (*sec).name, offset);
                }
            } else if modrm_reg == 4 {
                (*insn).type_ = INSN_JUMP_DYNAMIC;
                if has_notrack_prefix(&mut ins) {
                    WARN!(b"notrack prefix found at %s:0x%lx\0".as_ptr() as *const c_char, (*sec).name, offset);
                }
            } else if modrm_reg == 5 {
                /* jmpf */
                (*insn).type_ = INSN_SYSRET;
            } else if modrm_reg == 6 {
                /* push from mem */
                let op = add_stack_op(&mut ops_list);
                if op.is_null() {
                    return -1;
                }
                (*op).src.type_ = OP_SRC_CONST;
                (*op).dest.type_ = OP_DEST_PUSH;
            }
        }
        _ => {}
    }

    if ins.immediate.nbytes != 0 {
        (*insn).immediate = ins.immediate.value as s64;
        (*insn).immediate_len = ins.immediate.nbytes;
    } else if ins.displacement.nbytes != 0 {
        (*insn).immediate = ins.displacement.value as s64;
        (*insn).immediate_len = ins.displacement.nbytes;
    }

    0
}

unsafe fn decode_mov_reg_to_bp_sp_mem(
    ops_list: &mut *mut *mut stack_op,
    modrm_reg: c_uchar,
    modrm_mod: c_uchar,
    modrm_rm: c_uchar,
    sib_base: c_uchar,
    sib_index: c_uchar,
    ins: *const insn,
) -> c_int {
    let mod_is_mem = || modrm_mod != 3;
    let is_RIP = || (modrm_rm & 7) as c_int == CFI_BP && modrm_mod == 0;
    let have_SIB = || (modrm_rm & 7) as c_int == CFI_SP && mod_is_mem();
    let rm_is = |reg: c_int| -> bool {
        if have_SIB() {
            sib_base as c_int == reg
                && sib_index as c_int == CFI_SP
                && (sib_base as c_int != CFI_BP || modrm_mod != 0)
        } else {
            modrm_rm as c_int == reg
        }
    };
    let rm_is_mem = |reg: c_int| -> bool { mod_is_mem() && !is_RIP() && rm_is(reg) };

    if rm_is_mem(CFI_BP) {
        /* mov reg, disp(%rbp) */
        let op = add_stack_op(ops_list);
        if op.is_null() {
            return -1;
        }
        (*op).src.type_ = OP_SRC_REG;
        (*op).src.reg = modrm_reg;
        (*op).dest.type_ = OP_DEST_REG_INDIRECT;
        (*op).dest.reg = CFI_BP as c_uchar;
        (*op).dest.offset = (*ins).displacement.value as s64;
    } else if rm_is_mem(CFI_SP) {
        /* mov reg, disp(%rsp) */
        let op = add_stack_op(ops_list);
        if op.is_null() {
            return -1;
        }
        (*op).src.type_ = OP_SRC_REG;
        (*op).src.reg = modrm_reg;
        (*op).dest.type_ = OP_DEST_REG_INDIRECT;
        (*op).dest.reg = CFI_SP as c_uchar;
        (*op).dest.offset = (*ins).displacement.value as s64;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn arch_jump_opcode_bytes(
    _file: *mut objtool_file,
    insn: *mut instruction,
    buf: *mut c_uchar,
) -> size_t {
    let len: size_t = ((*insn).len - (*insn).immediate_len) as size_t;
    memcpy(
        buf as *mut c_void,
        (*(*(*insn).sec).data).d_buf.add((*insn).offset as usize) as *const c_void,
        len,
    );
    len
}

#[no_mangle]
pub unsafe extern "C" fn arch_initial_func_cfi_state(state: *mut cfi_init_state) {
    let mut i: c_int = 0;

    while i < CFI_NUM_REGS {
        (*state).regs[i as usize].base = CFI_UNDEFINED;
        (*state).regs[i as usize].offset = 0;
        i += 1;
    }

    /* initial CFA (call frame address) */
    (*state).cfa.base = CFI_SP;
    (*state).cfa.offset = 8;

    /* initial RA (return address) */
    (*state).regs[CFI_RA as usize].base = CFI_CFA;
    (*state).regs[CFI_RA as usize].offset = -8;
}

#[no_mangle]
pub unsafe extern "C" fn arch_nop_insn(len: c_int) -> *const c_char {
    static nops: [[c_char; 5]; 5] = [
        [BYTES_NOP1 as c_char, 0, 0, 0, 0],
        BYTES_NOP2,
        BYTES_NOP3,
        BYTES_NOP4,
        BYTES_NOP5,
    ];

    if len < 1 || len > 5 {
        ERROR!(b"invalid NOP size: %d\n\0".as_ptr() as *const c_char, len);
        return ptr::null();
    }

    nops[(len - 1) as usize].as_ptr()
}

const BYTE_RET: c_uchar = 0xC3;

#[no_mangle]
pub unsafe extern "C" fn arch_ret_insn(len: c_int) -> *const c_char {
    static ret: [[c_char; 5]; 5] = [
        [BYTE_RET as c_char, 0, 0, 0, 0],
        [BYTE_RET as c_char, 0xccu8 as c_char, 0, 0, 0],
        [BYTE_RET as c_char, 0xccu8 as c_char, BYTES_NOP1 as c_char, 0, 0],
        [BYTE_RET as c_char, 0xccu8 as c_char, BYTES_NOP2[0], BYTES_NOP2[1], 0],
        [BYTE_RET as c_char, 0xccu8 as c_char, BYTES_NOP3[0], BYTES_NOP3[1], BYTES_NOP3[2]],
    ];

    if len < 1 || len > 5 {
        ERROR!(b"invalid RET size: %d\n\0".as_ptr() as *const c_char, len);
        return ptr::null();
    }

    ret[(len - 1) as usize].as_ptr()
}

#[no_mangle]
pub unsafe extern "C" fn arch_decode_hint_reg(sp_reg: u8, base: *mut c_int) -> c_int {
    match sp_reg as c_int {
        ORC_REG_UNDEFINED => {
            *base = CFI_UNDEFINED;
        }
        ORC_REG_AX => {
            *base = CFI_AX;
        }
        ORC_REG_DX => {
            *base = CFI_DX;
        }
        ORC_REG_SP => {
            *base = CFI_SP;
        }
        ORC_REG_BP => {
            *base = CFI_BP;
        }
        ORC_REG_DI => {
            *base = CFI_DI;
        }
        ORC_REG_R10 => {
            *base = CFI_R10;
        }
        ORC_REG_R13 => {
            *base = CFI_R13;
        }
        ORC_REG_SP_INDIRECT => {
            *base = CFI_SP_INDIRECT;
        }
        ORC_REG_BP_INDIRECT => {
            *base = CFI_BP_INDIRECT;
        }
        _ => {
            return -1;
        }
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn arch_is_retpoline(sym: *mut symbol) -> bool_ {
    strncmp((*sym).name, b"__x86_indirect_\0".as_ptr() as *const c_char, 15) == 0
        || strncmp(
            (*sym).name,
            b"__pi___x86_indirect_\0".as_ptr() as *const c_char,
            20,
        ) == 0
}

#[no_mangle]
pub unsafe extern "C" fn arch_is_rethunk(sym: *mut symbol) -> bool_ {
    strcmp((*sym).name, b"__x86_return_thunk\0".as_ptr() as *const c_char) == 0
        || strcmp(
            (*sym).name,
            b"__pi___x86_return_thunk\0".as_ptr() as *const c_char,
        ) == 0
}

#[no_mangle]
pub unsafe extern "C" fn arch_is_embedded_insn(sym: *mut symbol) -> bool_ {
    strcmp((*sym).name, b"retbleed_return_thunk\0".as_ptr() as *const c_char) == 0
        || strcmp((*sym).name, b"srso_alias_safe_ret\0".as_ptr() as *const c_char) == 0
        || strcmp((*sym).name, b"srso_safe_ret\0".as_ptr() as *const c_char) == 0
}

#[no_mangle]
pub unsafe extern "C" fn arch_reloc_size(reloc: *mut reloc) -> c_uint {
    match reloc_type(reloc) {
        R_X86_64_32 | R_X86_64_32S | R_X86_64_PC32 | R_X86_64_PLT32 => 4,
        _ => 8,
    }
}

#[no_mangle]
pub unsafe extern "C" fn arch_absolute_reloc(_elf: *mut elf, reloc: *mut reloc) -> bool_ {
    match reloc_type(reloc) {
        R_X86_64_32 | R_X86_64_32S | R_X86_64_64 => true,
        _ => false,
    }
}

#[cfg(DISAS)]
#[no_mangle]
pub unsafe extern "C" fn arch_disas_info_init(dinfo: *mut disassemble_info) -> c_int {
    disas_info_init(
        dinfo,
        bfd_arch_i386,
        bfd_mach_i386_i386,
        bfd_mach_x86_64,
        b"att\0".as_ptr() as *const c_char,
    )
}
