// SPDX-License-Identifier: GPL-2.0+

// Dependencies supplied by the surrounding kernel translation.
use crate::{instruction_pointer, instruction_pointer_set, pt_regs, sign_extend32};

#[inline]
unsafe fn csky_insn_reg_get_val(regs: *mut pt_regs, index: usize, ptr: *mut usize) -> bool {
    if index < 14 { *ptr = *(&(*regs).a0 as *const _).add(index); }
    if index > 15 && index < 31 { *ptr = *(*regs).exregs.as_ptr().add(index - 16); }
    match index {
        14 => { *ptr = (*regs).usp; }
        15 => { *ptr = (*regs).lr; }
        31 => { *ptr = (*regs).tls; }
        _ => return false,
    }
    true
}

#[inline]
unsafe fn csky_insn_reg_set_val(regs: *mut pt_regs, index: usize, val: usize) -> bool {
    if index < 14 { *(&mut (*regs).a0 as *mut _).add(index) = val; }
    if index > 15 && index < 31 { *(*regs).exregs.as_mut_ptr().add(index - 16) = val; }
    match index {
        14 => (*regs).usp = val,
        15 => (*regs).lr = val,
        31 => (*regs).tls = val,
        _ => return false,
    }
    true
}

pub unsafe fn simulate_br16(opcode: u32, addr: isize, regs: *mut pt_regs) { instruction_pointer_set(regs, addr + sign_extend32(((opcode & 0x3ff) << 1), 9) as isize); }
pub unsafe fn simulate_br32(opcode: u32, addr: isize, regs: *mut pt_regs) { instruction_pointer_set(regs, addr + sign_extend32((opcode & 0xffff0000) >> 15, 15) as isize); }
pub unsafe fn simulate_bt16(opcode: u32, addr: isize, regs: *mut pt_regs) { if (*regs).sr & 1 != 0 { simulate_br16(opcode, addr, regs) } else { instruction_pointer_set(regs, addr + 2) } }
pub unsafe fn simulate_bt32(opcode: u32, addr: isize, regs: *mut pt_regs) { if (*regs).sr & 1 != 0 { simulate_br32(opcode, addr, regs) } else { instruction_pointer_set(regs, addr + 4) } }
pub unsafe fn simulate_bf16(opcode: u32, addr: isize, regs: *mut pt_regs) { if (*regs).sr & 1 == 0 { simulate_br16(opcode, addr, regs) } else { instruction_pointer_set(regs, addr + 2) } }
pub unsafe fn simulate_bf32(opcode: u32, addr: isize, regs: *mut pt_regs) { if (*regs).sr & 1 == 0 { simulate_br32(opcode, addr, regs) } else { instruction_pointer_set(regs, addr + 4) } }

pub unsafe fn simulate_jmp16(opcode: u32, _addr: isize, regs: *mut pt_regs) { let mut tmp = ((opcode >> 2) & 0xf) as usize; csky_insn_reg_get_val(regs, tmp, &mut tmp); instruction_pointer_set(regs, (tmp & 0xfffffffe) as isize); }
pub unsafe fn simulate_jmp32(opcode: u32, _addr: isize, regs: *mut pt_regs) { let mut tmp = (opcode & 0x1f) as usize; csky_insn_reg_get_val(regs, tmp, &mut tmp); instruction_pointer_set(regs, (tmp & 0xfffffffe) as isize); }
pub unsafe fn simulate_jsr16(opcode: u32, addr: isize, regs: *mut pt_regs) { let mut tmp = ((opcode >> 2) & 0xf) as usize; csky_insn_reg_get_val(regs, tmp, &mut tmp); (*regs).lr = (addr + 2) as usize; instruction_pointer_set(regs, (tmp & 0xfffffffe) as isize); }
pub unsafe fn simulate_jsr32(opcode: u32, addr: isize, regs: *mut pt_regs) { let mut tmp = (opcode & 0x1f) as usize; csky_insn_reg_get_val(regs, tmp, &mut tmp); (*regs).lr = (addr + 4) as usize; instruction_pointer_set(regs, (tmp & 0xfffffffe) as isize); }

pub unsafe fn simulate_lrw16(opcode: u32, _addr: isize, regs: *mut pt_regs) { let tmp = ((opcode & 0x300) >> 3); let offset = (((opcode & 0x1f) | tmp) << 2) as usize; let reg = ((opcode & 0xe0) >> 5) as usize; let val = *((instruction_pointer(regs) as usize + offset) as *const u32) as usize; csky_insn_reg_set_val(regs, reg, val); }
pub unsafe fn simulate_lrw32(opcode: u32, _addr: isize, regs: *mut pt_regs) { let offset = ((opcode & 0xffff0000) >> 14) as usize; let reg = (opcode & 0x1f) as usize; let val = *(((instruction_pointer(regs) as usize + offset) & 0xfffffffc) as *const u32) as usize; csky_insn_reg_set_val(regs, reg, val); }

pub unsafe fn simulate_pop16(opcode: u32, _addr: isize, regs: *mut pt_regs) { let mut tmp = (*regs).usp as *mut usize; for i in 0..(opcode & 0xf) { csky_insn_reg_set_val(regs, i as usize + 4, *tmp); tmp = tmp.add(1); } if opcode & 0x10 != 0 { csky_insn_reg_set_val(regs, 15, *tmp); tmp = tmp.add(1); } (*regs).usp = tmp as usize; instruction_pointer_set(regs, (*regs).lr as isize); }

pub unsafe fn simulate_pop32(opcode: u32, _addr: isize, regs: *mut pt_regs) { let mut tmp = (*regs).usp as *mut usize; for i in 0..((opcode & 0xf0000) >> 16) { csky_insn_reg_set_val(regs, i as usize + 4, *tmp); tmp = tmp.add(1); } if opcode & 0x100000 != 0 { csky_insn_reg_set_val(regs, 15, *tmp); tmp = tmp.add(1); } for i in 0..((opcode & 0xe00000) >> 21) { csky_insn_reg_set_val(regs, i as usize + 16, *tmp); tmp = tmp.add(1); } if opcode & 0x1000000 != 0 { csky_insn_reg_set_val(regs, 29, *tmp); tmp = tmp.add(1); } (*regs).usp = tmp as usize; instruction_pointer_set(regs, (*regs).lr as isize); }

unsafe fn branch_reg(opcode: u32, addr: isize, regs: *mut pt_regs, pred: impl FnOnce(usize) -> bool) { let mut tmp = (opcode & 0x1f) as usize; csky_insn_reg_get_val(regs, tmp, &mut tmp); if pred(tmp) { simulate_br32(opcode, addr, regs) } else { instruction_pointer_set(regs, addr + 4) } }
pub unsafe fn simulate_bez32(opcode: u32, addr: isize, regs: *mut pt_regs) { branch_reg(opcode, addr, regs, |v| v == 0); }
pub unsafe fn simulate_bnez32(opcode: u32, addr: isize, regs: *mut pt_regs) { branch_reg(opcode, addr, regs, |v| v != 0); }
pub unsafe fn simulate_bhsz32(opcode: u32, addr: isize, regs: *mut pt_regs) { branch_reg(opcode, addr, regs, |v| (v as isize) >= 0); }
pub unsafe fn simulate_bhz32(opcode: u32, addr: isize, regs: *mut pt_regs) { branch_reg(opcode, addr, regs, |v| (v as isize) > 0); }
pub unsafe fn simulate_blsz32(opcode: u32, addr: isize, regs: *mut pt_regs) { branch_reg(opcode, addr, regs, |v| (v as isize) <= 0); }
pub unsafe fn simulate_blz32(opcode: u32, addr: isize, regs: *mut pt_regs) { branch_reg(opcode, addr, regs, |v| (v as isize) < 0); }

pub unsafe fn simulate_bnezad32(opcode: u32, addr: isize, regs: *mut pt_regs) { let mut tmp = (opcode & 0x1f) as usize; let mut val = 0usize; csky_insn_reg_get_val(regs, tmp, &mut val); val = val.wrapping_sub(1); if (val as isize) > 0 { simulate_br32(opcode, addr, regs) } else { instruction_pointer_set(regs, addr + 4) } csky_insn_reg_set_val(regs, tmp, val); }
pub unsafe fn simulate_bsr32(opcode: u32, addr: isize, regs: *mut pt_regs) { let tmp = ((opcode & 0xffff) << 16) | ((opcode & 0xffff0000) >> 16); instruction_pointer_set(regs, addr + sign_extend32((tmp & 0x3ffffff) << 1, 15) as isize); (*regs).lr = (addr + 4) as usize; }
pub unsafe fn simulate_jmpi32(opcode: u32, _addr: isize, regs: *mut pt_regs) { let offset = ((opcode & 0xffff0000) >> 14) as usize; let val = *(((instruction_pointer(regs) as usize + offset) & 0xfffffffc) as *const u32); instruction_pointer_set(regs, val as isize); }
pub unsafe fn simulate_jsri32(opcode: u32, addr: isize, regs: *mut pt_regs) { let offset = ((opcode & 0xffff0000) >> 14) as usize; let val = *(((instruction_pointer(regs) as usize + offset) & 0xfffffffc) as *const u32); (*regs).lr = (addr + 4) as usize; instruction_pointer_set(regs, val as isize); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
