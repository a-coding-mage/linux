// SPDX-License-Identifier: GPL-2.0-only
/*
 * AArch64 loadable module support.
 *
 * Copyright (C) 2012 ARM Limited
 *
 * Author: Will Deacon <will.deacon@arm.com>
 */

// pr_fmt(fmt) = "Modules: " fmt

#[repr(C)]
enum Aarch64RelocOp { None, Abs, Prel, Page }

unsafe fn do_reloc(reloc_op: Aarch64RelocOp, place: *const u32, val: u64) -> u64 {
    match reloc_op {
        Aarch64RelocOp::Abs => val,
        Aarch64RelocOp::Prel => val.wrapping_sub(place as u64),
        Aarch64RelocOp::Page => (val & !0xfff).wrapping_sub((place as u64) & !0xfff),
        Aarch64RelocOp::None => 0,
    }
}

unsafe fn write_place<T: Copy>(place: *mut T, val: T, mod_: *mut Module) {
    if (*mod_).state == MODULE_STATE_UNFORMED { *place = val; }
    else { aarch64_insn_copy(place as *mut u8, &val as *const T as *const u8, core::mem::size_of::<T>()); }
}

unsafe fn reloc_data(op: Aarch64RelocOp, place: *mut core::ffi::c_void, val: u64, len: i32, me: *mut Module) -> i32 {
    let sval = do_reloc(op, place as *const u32, val) as i64;
    match len {
        16 => { write_place(place as *mut i16, sval as i16, me); match op { Aarch64RelocOp::Abs if sval < 0 || sval > u16::MAX as i64 => -ERANGE, Aarch64RelocOp::Prel if sval < i16::MIN as i64 || sval > i16::MAX as i64 => -ERANGE, Aarch64RelocOp::Abs | Aarch64RelocOp::Prel => 0, _ => 0 } },
        32 => { write_place(place as *mut i32, sval as i32, me); match op { Aarch64RelocOp::Abs if sval < 0 || sval > u32::MAX as i64 => -ERANGE, Aarch64RelocOp::Prel if sval < i32::MIN as i64 || sval > i32::MAX as i64 => -ERANGE, Aarch64RelocOp::Abs | Aarch64RelocOp::Prel => 0, _ => 0 } },
        64 => { write_place(place as *mut i64, sval, me); 0 },
        _ => 0,
    }
}

#[repr(C)]
enum Aarch64InsnMovwImmType { Movnz, Movkz }

unsafe fn reloc_insn_movw(op: Aarch64RelocOp, place: *mut u32, val: u64, lsb: u32, imm_type: Aarch64InsnMovwImmType, me: *mut Module) -> i32 {
    let sval = do_reloc(op, place, val) as i64;
    let mut imm = (sval >> lsb) as u64;
    let mut insn = le32_to_cpu(*place);
    if matches!(imm_type, Aarch64InsnMovwImmType::Movnz) { insn &= !(3 << 29); if sval >= 0 { insn |= 2 << 29; } else { imm = !imm; } }
    insn = aarch64_insn_encode_immediate(AARCH64_INSN_IMM_16, insn, imm);
    write_place(place, cpu_to_le32(insn), me);
    if imm > u16::MAX as u64 { -ERANGE } else { 0 }
}

unsafe fn reloc_insn_imm(op: Aarch64RelocOp, place: *mut u32, val: u64, lsb: u32, len: u32, imm_type: i32, me: *mut Module) -> i32 {
    let mut sval = (do_reloc(op, place, val) as i64) >> lsb;
    let imm_mask = (BIT(lsb + len) - 1) >> lsb;
    let imm = sval as u64 & imm_mask;
    let mut insn = le32_to_cpu(*place);
    insn = aarch64_insn_encode_immediate(imm_type, insn, imm);
    write_place(place, cpu_to_le32(insn), me);
    sval = ((sval as u64 & !(imm_mask >> 1)) as i64) >> (len - 1);
    if (sval + 1) as u64 >= 2 { -ERANGE } else { 0 }
}

unsafe fn reloc_insn_adrp(mod_: *mut Module, sechdrs: *mut Elf64_Shdr, place: *mut u32, val: u64, me: *mut Module) -> i32 {
    if !is_forbidden_offset_for_adrp(place) { return reloc_insn_imm(RELOC_OP_PAGE, place, val, 12, 21, AARCH64_INSN_IMM_ADR, me); }
    let mut insn;
    if reloc_insn_imm(RELOC_OP_PREL, place, val & !0xfff, 0, 21, AARCH64_INSN_IMM_ADR, me) == 0 { insn = le32_to_cpu(*place); insn &= !BIT(31); }
    else { let v = module_emit_veneer_for_adrp(mod_, sechdrs, place, val & !0xfff); if v == 0 { return -ENOEXEC; } insn = aarch64_insn_gen_branch_imm(place as u64, v, AARCH64_INSN_BRANCH_NOLINK); }
    write_place(place, cpu_to_le32(insn), me); 0
}

// The relocation dispatcher and finalization entry points retain the kernel ABI.
// External kernel types and constants are supplied by the surrounding translation.
unsafe extern "C" {
    fn apply_relocate_add(sechdrs: *mut Elf64_Shdr, strtab: *const i8, symindex: u32, relsec: u32, me: *mut Module) -> i32;
    fn module_finalize(hdr: *const Elf_Ehdr, sechdrs: *const Elf64_Shdr, me: *mut Module) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
