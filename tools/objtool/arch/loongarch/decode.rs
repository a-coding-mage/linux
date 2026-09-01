// SPDX-License-Identifier: GPL-2.0-or-later
// Translated from objtool/arch/loongarch/decode.c.
// C include dependencies preserved as external Rust dependencies:
// string.h, objtool/check.h, objtool/disas.h, objtool/warn.h,
// asm/inst.h, asm/orc_types.h, linux/objtool_types.h, arch/elf.h.

type s64 = i64;

extern "C" {
    fn strcmp(s1: *const core::ffi::c_char, s2: *const core::ffi::c_char) -> core::ffi::c_int;
    fn calloc(nmemb: usize, size: usize) -> *mut core::ffi::c_void;
    fn reloc_addend(reloc: *mut reloc) -> s64;
    fn reloc_type(reloc: *mut reloc) -> core::ffi::c_int;
    fn reloc_offset(reloc: *mut reloc) -> core::ffi::c_ulong;
    fn sign_extend64(value: u64, index: core::ffi::c_int) -> s64;
    fn find_func_containing(sec: *const section, offset: core::ffi::c_ulong) -> *mut symbol;
    fn emit_jirl(
        inst: *mut loongarch_instruction,
        rj: core::ffi::c_uint,
        rd: core::ffi::c_uint,
        si16: core::ffi::c_int,
    );
    fn ERROR(fmt: *const core::ffi::c_char, ...);
}

#[no_mangle]
pub static mut arch_reg_name: [*const core::ffi::c_char; CFI_NUM_REGS as usize] = [
    b"zero\0".as_ptr() as *const core::ffi::c_char,
    b"ra\0".as_ptr() as *const core::ffi::c_char,
    b"tp\0".as_ptr() as *const core::ffi::c_char,
    b"sp\0".as_ptr() as *const core::ffi::c_char,
    b"a0\0".as_ptr() as *const core::ffi::c_char,
    b"a1\0".as_ptr() as *const core::ffi::c_char,
    b"a2\0".as_ptr() as *const core::ffi::c_char,
    b"a3\0".as_ptr() as *const core::ffi::c_char,
    b"a4\0".as_ptr() as *const core::ffi::c_char,
    b"a5\0".as_ptr() as *const core::ffi::c_char,
    b"a6\0".as_ptr() as *const core::ffi::c_char,
    b"a7\0".as_ptr() as *const core::ffi::c_char,
    b"t0\0".as_ptr() as *const core::ffi::c_char,
    b"t1\0".as_ptr() as *const core::ffi::c_char,
    b"t2\0".as_ptr() as *const core::ffi::c_char,
    b"t3\0".as_ptr() as *const core::ffi::c_char,
    b"t4\0".as_ptr() as *const core::ffi::c_char,
    b"t5\0".as_ptr() as *const core::ffi::c_char,
    b"t6\0".as_ptr() as *const core::ffi::c_char,
    b"t7\0".as_ptr() as *const core::ffi::c_char,
    b"t8\0".as_ptr() as *const core::ffi::c_char,
    b"u0\0".as_ptr() as *const core::ffi::c_char,
    b"fp\0".as_ptr() as *const core::ffi::c_char,
    b"s0\0".as_ptr() as *const core::ffi::c_char,
    b"s1\0".as_ptr() as *const core::ffi::c_char,
    b"s2\0".as_ptr() as *const core::ffi::c_char,
    b"s3\0".as_ptr() as *const core::ffi::c_char,
    b"s4\0".as_ptr() as *const core::ffi::c_char,
    b"s5\0".as_ptr() as *const core::ffi::c_char,
    b"s6\0".as_ptr() as *const core::ffi::c_char,
    b"s7\0".as_ptr() as *const core::ffi::c_char,
    b"s8\0".as_ptr() as *const core::ffi::c_char,
];

#[no_mangle]
pub unsafe extern "C" fn arch_ftrace_match(name: *const core::ffi::c_char) -> core::ffi::c_int {
    (strcmp(name, b"_mcount\0".as_ptr() as *const core::ffi::c_char) == 0) as core::ffi::c_int
}

#[no_mangle]
pub unsafe extern "C" fn arch_jump_destination(insn: *mut instruction) -> core::ffi::c_ulong {
    (*insn).offset + (((*insn).immediate) << 2) as core::ffi::c_ulong
}

#[no_mangle]
pub unsafe extern "C" fn arch_insn_adjusted_addend(
    _insn: *mut instruction,
    reloc: *mut reloc,
) -> s64 {
    reloc_addend(reloc)
}

#[no_mangle]
pub unsafe extern "C" fn arch_pc_relative_reloc(_reloc: *mut reloc) -> bool {
    false
}

#[no_mangle]
pub unsafe extern "C" fn arch_callee_saved_reg(reg: u8) -> bool {
    match reg as core::ffi::c_int {
        CFI_RA | CFI_FP => true,
        r if r >= CFI_S0 && r <= CFI_S8 => true,
        _ => false,
    }
}

#[no_mangle]
pub unsafe extern "C" fn arch_decode_hint_reg(sp_reg: u8, base: *mut core::ffi::c_int) -> core::ffi::c_int {
    match sp_reg as core::ffi::c_int {
        ORC_REG_UNDEFINED => {
            *base = CFI_UNDEFINED;
        }
        ORC_REG_SP => {
            *base = CFI_SP;
        }
        ORC_REG_FP => {
            *base = CFI_FP;
        }
        _ => return -1,
    }

    0
}

unsafe fn is_loongarch(elf: *const elf) -> bool {
    if (*elf).ehdr.e_machine == EM_LOONGARCH {
        return true;
    }

    ERROR(
        b"unexpected ELF machine type %d\0".as_ptr() as *const core::ffi::c_char,
        (*elf).ehdr.e_machine,
    );
    false
}

unsafe fn add_op(ops_list: *mut *mut stack_op) -> *mut stack_op {
    let op = calloc(1, core::mem::size_of::<stack_op>()) as *mut stack_op;
    if op.is_null() {
        return core::ptr::null_mut();
    }

    *ops_list = op;
    op
}

unsafe fn decode_insn_reg0i26_fomat(inst: loongarch_instruction, insn: *mut instruction) -> bool {
    match inst.reg0i26_format.opcode {
        b_op => {
            (*insn).r#type = INSN_JUMP_UNCONDITIONAL;
            (*insn).immediate = sign_extend64(
                ((inst.reg0i26_format.immediate_h << 16) | inst.reg0i26_format.immediate_l) as u64,
                25,
            );
        }
        bl_op => {
            (*insn).r#type = INSN_CALL;
            (*insn).immediate = sign_extend64(
                ((inst.reg0i26_format.immediate_h << 16) | inst.reg0i26_format.immediate_l) as u64,
                25,
            );
        }
        _ => return false,
    }

    true
}

unsafe fn decode_insn_reg1i21_fomat(inst: loongarch_instruction, insn: *mut instruction) -> bool {
    match inst.reg1i21_format.opcode {
        beqz_op | bnez_op | bceqz_op => {
            (*insn).r#type = INSN_JUMP_CONDITIONAL;
            (*insn).immediate = sign_extend64(
                ((inst.reg1i21_format.immediate_h << 16) | inst.reg1i21_format.immediate_l) as u64,
                20,
            );
        }
        _ => return false,
    }

    true
}

unsafe fn decode_insn_reg2i12_fomat(
    inst: loongarch_instruction,
    insn: *mut instruction,
    ops_list: *mut *mut stack_op,
    _op: *mut stack_op,
) -> bool {
    match inst.reg2i12_format.opcode {
        addid_op => {
            if inst.reg2i12_format.rd == CFI_SP || inst.reg2i12_format.rj == CFI_SP {
                /* addi.d sp,sp,si12 or addi.d fp,sp,si12 or addi.d sp,fp,si12 */
                (*insn).immediate = sign_extend64(inst.reg2i12_format.immediate as u64, 11);
                let op = add_op(ops_list);
                if op.is_null() {
                    return true;
                }
                (*op).src.r#type = OP_SRC_ADD;
                (*op).src.reg = inst.reg2i12_format.rj;
                (*op).src.offset = (*insn).immediate;
                (*op).dest.r#type = OP_DEST_REG;
                (*op).dest.reg = inst.reg2i12_format.rd;
                *ops_list = &mut (*op).next;
            }
            if inst.reg2i12_format.rd == CFI_SP && inst.reg2i12_format.rj == CFI_FP {
                /* addi.d sp,fp,si12 */
                let func = find_func_containing((*insn).sec, (*insn).offset);

                if func.is_null() {
                    return false;
                }

                (*func).frame_pointer = true;
            }
        }
        ldd_op => {
            if inst.reg2i12_format.rj == CFI_SP {
                /* ld.d rd,sp,si12 */
                (*insn).immediate = sign_extend64(inst.reg2i12_format.immediate as u64, 11);
                let op = add_op(ops_list);
                if op.is_null() {
                    return true;
                }
                (*op).src.r#type = OP_SRC_REG_INDIRECT;
                (*op).src.reg = CFI_SP;
                (*op).src.offset = (*insn).immediate;
                (*op).dest.r#type = OP_DEST_REG;
                (*op).dest.reg = inst.reg2i12_format.rd;
                *ops_list = &mut (*op).next;
            }
        }
        std_op => {
            if inst.reg2i12_format.rj == CFI_SP {
                /* st.d rd,sp,si12 */
                (*insn).immediate = sign_extend64(inst.reg2i12_format.immediate as u64, 11);
                let op = add_op(ops_list);
                if op.is_null() {
                    return true;
                }
                (*op).src.r#type = OP_SRC_REG;
                (*op).src.reg = inst.reg2i12_format.rd;
                (*op).dest.r#type = OP_DEST_REG_INDIRECT;
                (*op).dest.reg = CFI_SP;
                (*op).dest.offset = (*insn).immediate;
                *ops_list = &mut (*op).next;
            }
        }
        andi_op => {
            if inst.reg2i12_format.rd == 0
                && inst.reg2i12_format.rj == 0
                && inst.reg2i12_format.immediate == 0
            {
                /* andi r0,r0,0 */
                (*insn).r#type = INSN_NOP;
            }
        }
        _ => return false,
    }

    true
}

unsafe fn decode_insn_reg2i14_fomat(
    inst: loongarch_instruction,
    insn: *mut instruction,
    ops_list: *mut *mut stack_op,
    _op: *mut stack_op,
) -> bool {
    match inst.reg2i14_format.opcode {
        ldptrd_op => {
            if inst.reg2i14_format.rj == CFI_SP {
                /* ldptr.d rd,sp,si14 */
                (*insn).immediate = sign_extend64(inst.reg2i14_format.immediate as u64, 13);
                let op = add_op(ops_list);
                if op.is_null() {
                    return true;
                }
                (*op).src.r#type = OP_SRC_REG_INDIRECT;
                (*op).src.reg = CFI_SP;
                (*op).src.offset = (*insn).immediate;
                (*op).dest.r#type = OP_DEST_REG;
                (*op).dest.reg = inst.reg2i14_format.rd;
                *ops_list = &mut (*op).next;
            }
        }
        stptrd_op => {
            if inst.reg2i14_format.rj == CFI_SP {
                /* stptr.d ra,sp,0 */
                if inst.reg2i14_format.rd == LOONGARCH_GPR_RA
                    && inst.reg2i14_format.immediate == 0
                {
                    return true;
                }

                /* stptr.d rd,sp,si14 */
                (*insn).immediate = sign_extend64(inst.reg2i14_format.immediate as u64, 13);
                let op = add_op(ops_list);
                if op.is_null() {
                    return true;
                }
                (*op).src.r#type = OP_SRC_REG;
                (*op).src.reg = inst.reg2i14_format.rd;
                (*op).dest.r#type = OP_DEST_REG_INDIRECT;
                (*op).dest.reg = CFI_SP;
                (*op).dest.offset = (*insn).immediate;
                *ops_list = &mut (*op).next;
            }
        }
        _ => return false,
    }

    true
}

unsafe fn decode_insn_reg2i16_fomat(inst: loongarch_instruction, insn: *mut instruction) -> bool {
    match inst.reg2i16_format.opcode {
        jirl_op => {
            if inst.reg2i16_format.rd == 0
                && inst.reg2i16_format.rj == CFI_RA
                && inst.reg2i16_format.immediate == 0
            {
                /* jirl r0,ra,0 */
                (*insn).r#type = INSN_RETURN;
            } else if inst.reg2i16_format.rd == CFI_RA {
                /* jirl ra,rj,offs16 */
                (*insn).r#type = INSN_CALL_DYNAMIC;
            } else if inst.reg2i16_format.rd == CFI_A0
                && inst.reg2i16_format.immediate == 0
            {
                /*
                 * jirl a0,t0,0
                 * this is a special case in loongarch_suspend_enter,
                 * just treat it as a call instruction.
                 */
                (*insn).r#type = INSN_CALL_DYNAMIC;
            } else if inst.reg2i16_format.rd == 0
                && inst.reg2i16_format.immediate == 0
            {
                /* jirl r0,rj,0 */
                (*insn).r#type = INSN_JUMP_DYNAMIC;
            } else if inst.reg2i16_format.rd == 0
                && inst.reg2i16_format.immediate != 0
            {
                /*
                 * jirl r0,t0,12
                 * this is a rare case in JUMP_VIRT_ADDR,
                 * just ignore it due to it is harmless for tracing.
                 */
            } else {
                /* jirl rd,rj,offs16 */
                (*insn).r#type = INSN_JUMP_UNCONDITIONAL;
                (*insn).immediate = sign_extend64(inst.reg2i16_format.immediate as u64, 15);
            }
        }
        beq_op | bne_op | blt_op | bge_op | bltu_op | bgeu_op => {
            (*insn).r#type = INSN_JUMP_CONDITIONAL;
            (*insn).immediate = sign_extend64(inst.reg2i16_format.immediate as u64, 15);
        }
        _ => return false,
    }

    true
}

unsafe fn decode_insn_reg3_fomat(inst: loongarch_instruction, insn: *mut instruction) -> bool {
    match inst.reg3_format.opcode {
        amswapw_op => {
            if inst.reg3_format.rd == LOONGARCH_GPR_ZERO
                && inst.reg3_format.rk == LOONGARCH_GPR_RA
                && inst.reg3_format.rj == LOONGARCH_GPR_ZERO
            {
                /* amswap.w $zero, $ra, $zero */
                (*insn).r#type = INSN_BUG;
            }
        }
        _ => return false,
    }

    true
}

#[no_mangle]
pub unsafe extern "C" fn arch_decode_instruction(
    file: *mut objtool_file,
    sec: *const section,
    offset: core::ffi::c_ulong,
    maxlen: core::ffi::c_uint,
    insn: *mut instruction,
) -> core::ffi::c_int {
    let mut ops_list: *mut *mut stack_op = &mut (*insn).stack_ops;
    let elf = (*file).elf;
    let op: *mut stack_op = core::ptr::null_mut();
    let inst: loongarch_instruction;

    if !is_loongarch(elf) {
        return -1;
    }

    if maxlen < LOONGARCH_INSN_SIZE {
        return 0;
    }

    (*insn).len = LOONGARCH_INSN_SIZE;
    (*insn).r#type = INSN_OTHER;
    (*insn).immediate = 0;

    inst = *((*(*sec).data).d_buf.add(offset as usize) as *const loongarch_instruction);

    if decode_insn_reg0i26_fomat(inst, insn) {
        return 0;
    }
    if decode_insn_reg1i21_fomat(inst, insn) {
        return 0;
    }
    if decode_insn_reg2i12_fomat(inst, insn, ops_list, op) {
        return 0;
    }
    if decode_insn_reg2i14_fomat(inst, insn, ops_list, op) {
        return 0;
    }
    if decode_insn_reg2i16_fomat(inst, insn) {
        return 0;
    }
    if decode_insn_reg3_fomat(inst, insn) {
        return 0;
    }

    if inst.word == 0 {
        /* andi $zero, $zero, 0x0 */
        (*insn).r#type = INSN_NOP;
    } else if inst.reg0i15_format.opcode == break_op && inst.reg0i15_format.immediate == 0x0 {
        /* break 0x0 */
        (*insn).r#type = INSN_TRAP;
    } else if inst.reg0i15_format.opcode == break_op && inst.reg0i15_format.immediate == 0x1 {
        /* break 0x1 */
        (*insn).r#type = INSN_BUG;
    } else if inst.reg2_format.opcode == ertn_op {
        /* ertn */
        (*insn).r#type = INSN_RETURN;
    }

    0
}

#[no_mangle]
pub unsafe extern "C" fn arch_nop_insn(len: core::ffi::c_int) -> *const core::ffi::c_char {
    static mut nop: u32 = 0;

    if len != LOONGARCH_INSN_SIZE as core::ffi::c_int {
        ERROR(
            b"invalid NOP size: %d\n\0".as_ptr() as *const core::ffi::c_char,
            len,
        );
        return core::ptr::null();
    }

    nop = LOONGARCH_INSN_NOP;

    &nop as *const u32 as *const core::ffi::c_char
}

#[no_mangle]
pub unsafe extern "C" fn arch_ret_insn(len: core::ffi::c_int) -> *const core::ffi::c_char {
    static mut ret: u32 = 0;

    if len != LOONGARCH_INSN_SIZE as core::ffi::c_int {
        ERROR(
            b"invalid RET size: %d\n\0".as_ptr() as *const core::ffi::c_char,
            len,
        );
        return core::ptr::null();
    }

    emit_jirl(
        &mut ret as *mut u32 as *mut loongarch_instruction,
        LOONGARCH_GPR_RA,
        LOONGARCH_GPR_ZERO,
        0,
    );

    &ret as *const u32 as *const core::ffi::c_char
}

#[no_mangle]
pub unsafe extern "C" fn arch_initial_func_cfi_state(state: *mut cfi_init_state) {
    let mut i: core::ffi::c_int = 0;

    while i < CFI_NUM_REGS {
        (*state).regs[i as usize].base = CFI_UNDEFINED;
        (*state).regs[i as usize].offset = 0;
        i += 1;
    }

    /* initial CFA (call frame address) */
    (*state).cfa.base = CFI_SP;
    (*state).cfa.offset = 0;
}

#[no_mangle]
pub unsafe extern "C" fn arch_reloc_size(reloc: *mut reloc) -> core::ffi::c_uint {
    match reloc_type(reloc) {
        R_LARCH_32 | R_LARCH_32_PCREL => 4,
        _ => 8,
    }
}

#[no_mangle]
pub unsafe extern "C" fn arch_jump_table_sym_offset(
    reloc: *mut reloc,
    table: *mut reloc,
) -> core::ffi::c_ulong {
    match reloc_type(reloc) {
        R_LARCH_32_PCREL | R_LARCH_64_PCREL => {
            (*(*reloc).sym).offset
                + reloc_addend(reloc) as core::ffi::c_ulong
                - (reloc_offset(reloc) - reloc_offset(table))
        }
        _ => (*(*reloc).sym).offset + reloc_addend(reloc) as core::ffi::c_ulong,
    }
}

// Original C code conditionally compiled this block under DISAS.
#[cfg(DISAS)]
#[no_mangle]
pub unsafe extern "C" fn arch_disas_info_init(dinfo: *mut disassemble_info) -> core::ffi::c_int {
    disas_info_init(
        dinfo,
        bfd_arch_loongarch,
        bfd_mach_loongarch32,
        bfd_mach_loongarch64,
        core::ptr::null_mut(),
    )
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
