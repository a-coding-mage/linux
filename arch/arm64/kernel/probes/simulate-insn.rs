// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm64/kernel/probes/simulate-insn.c
 *
 * Copyright (C) 2013 Linaro Limited.
 */

// Dependencies supplied by the surrounding kernel translation.

#[inline]
fn bbl_displacement(insn: u32) -> i32 {
    sign_extend32((insn & 0x03ff_ffff) << 2, 27)
}

#[inline]
fn bcond_displacement(insn: u32) -> i32 {
    sign_extend32(((insn >> 5) & 0x7ffff) << 2, 20)
}

#[inline]
fn cbz_displacement(insn: u32) -> i32 {
    sign_extend32(((insn >> 5) & 0x7ffff) << 2, 20)
}

#[inline]
fn tbz_displacement(insn: u32) -> i32 {
    sign_extend32(((insn >> 5) & 0x3fff) << 2, 15)
}

#[inline]
fn ldr_displacement(insn: u32) -> i32 {
    sign_extend32(((insn >> 5) & 0x7ffff) << 2, 20)
}

#[inline]
unsafe fn set_x_reg(regs: *mut pt_regs, reg: i32, val: u64) {
    pt_regs_write_reg(regs, reg, val);
}

#[inline]
unsafe fn set_w_reg(regs: *mut pt_regs, reg: i32, val: u64) {
    pt_regs_write_reg(regs, reg, lower_32_bits(val));
}

#[inline]
unsafe fn get_x_reg(regs: *mut pt_regs, reg: i32) -> u64 {
    pt_regs_read_reg(regs, reg)
}

#[inline]
unsafe fn get_w_reg(regs: *mut pt_regs, reg: i32) -> u32 {
    lower_32_bits(pt_regs_read_reg(regs, reg))
}

#[inline]
unsafe fn update_lr(regs: *mut pt_regs, addr: i64) -> i32 {
    let mut err: i32 = 0;

    if user_mode(regs) && task_gcs_el0_enabled(current) {
        push_user_gcs(addr, &mut err);
        if err != 0 {
            force_sig(SIGSEGV);
            return err;
        }
    }
    procedure_link_pointer_set(regs, addr);
    err
}

unsafe fn check_cbz(opcode: u32, regs: *mut pt_regs) -> bool {
    let xn = (opcode & 0x1f) as i32;
    if opcode & (1 << 31) != 0 { get_x_reg(regs, xn) == 0 } else { get_w_reg(regs, xn) == 0 }
}

unsafe fn check_cbnz(opcode: u32, regs: *mut pt_regs) -> bool {
    let xn = (opcode & 0x1f) as i32;
    if opcode & (1 << 31) != 0 { get_x_reg(regs, xn) != 0 } else { get_w_reg(regs, xn) != 0 }
}

unsafe fn check_tbz(opcode: u32, regs: *mut pt_regs) -> bool {
    let xn = (opcode & 0x1f) as i32;
    let bit_pos = ((opcode & (1 << 31)) >> 26) | ((opcode >> 19) & 0x1f);
    ((get_x_reg(regs, xn) >> bit_pos) & 1) == 0
}

unsafe fn check_tbnz(opcode: u32, regs: *mut pt_regs) -> bool {
    let xn = (opcode & 0x1f) as i32;
    let bit_pos = ((opcode & (1 << 31)) >> 26) | ((opcode >> 19) & 0x1f);
    ((get_x_reg(regs, xn) >> bit_pos) & 1) != 0
}

/* instruction simulation functions */
pub unsafe fn simulate_adr_adrp(opcode: u32, addr: i64, regs: *mut pt_regs) {
    let xn = (opcode & 0x1f) as i64;
    let mut imm = (((opcode >> 3) & 0x1ffffc) | ((opcode >> 29) & 0x3)) as i64;
    imm = sign_extend64(imm, 20);
    let val = if opcode & 0x80000000 != 0 { (imm << 12) + (addr & 0xfffffffffffff000u64 as i64) } else { imm + addr };
    set_x_reg(regs, xn as i32, val as u64);
    instruction_pointer_set(regs, instruction_pointer(regs) + 4);
}

pub unsafe fn simulate_b_bl(opcode: u32, addr: i64, regs: *mut pt_regs) {
    let disp = bbl_displacement(opcode) as i64;
    if opcode & (1 << 31) != 0 && update_lr(regs, addr + 4) != 0 { return; }
    instruction_pointer_set(regs, addr + disp);
}

pub unsafe fn simulate_b_cond(opcode: u32, addr: i64, regs: *mut pt_regs) {
    let mut disp: i64 = 4;
    if aarch32_opcode_cond_checks[(opcode & 0xf) as usize](regs_ref_pstate(regs) & 0xffffffff) { disp = bcond_displacement(opcode) as i64; }
    instruction_pointer_set(regs, addr + disp);
}

pub unsafe fn simulate_br_blr(opcode: u32, addr: i64, regs: *mut pt_regs) {
    let xn = ((opcode >> 5) & 0x1f) as i32;
    let b_target = get_x_reg(regs, xn);
    if ((opcode >> 21) & 0x3) == 1 && update_lr(regs, addr + 4) != 0 { return; }
    instruction_pointer_set(regs, b_target as i64);
}

pub unsafe fn simulate_ret(_opcode: u32, _addr: i64, regs: *mut pt_regs) {
    let mut err = 0;
    let xn = ((_opcode >> 5) & 0x1f) as i32;
    let r_target = get_x_reg(regs, xn);
    if user_mode(regs) && task_gcs_el0_enabled(current) {
        let ret_addr = pop_user_gcs(&mut err);
        if err != 0 || ret_addr != r_target { force_sig(SIGSEGV); return; }
    }
    instruction_pointer_set(regs, r_target as i64);
}

pub unsafe fn simulate_cbz_cbnz(opcode: u32, addr: i64, regs: *mut pt_regs) {
    let mut disp: i64 = 4;
    if opcode & (1 << 24) != 0 { if check_cbnz(opcode, regs) { disp = cbz_displacement(opcode) as i64; } } else if check_cbz(opcode, regs) { disp = cbz_displacement(opcode) as i64; }
    instruction_pointer_set(regs, addr + disp);
}

pub unsafe fn simulate_tbz_tbnz(opcode: u32, addr: i64, regs: *mut pt_regs) {
    let mut disp: i64 = 4;
    if opcode & (1 << 24) != 0 { if check_tbnz(opcode, regs) { disp = tbz_displacement(opcode) as i64; } } else if check_tbz(opcode, regs) { disp = tbz_displacement(opcode) as i64; }
    instruction_pointer_set(regs, addr + disp);
}

pub unsafe fn simulate_ldr_literal(opcode: u32, addr: i64, regs: *mut pt_regs) {
    let load_addr = (addr + ldr_displacement(opcode) as i64) as *const u8;
    let xn = (opcode & 0x1f) as i32;
    if opcode & (1 << 30) != 0 { set_x_reg(regs, xn, core::ptr::read_volatile(load_addr as *const u64)); } else { set_w_reg(regs, xn, core::ptr::read_volatile(load_addr as *const u32) as u64); }
    instruction_pointer_set(regs, instruction_pointer(regs) + 4);
}

pub unsafe fn simulate_ldrsw_literal(opcode: u32, addr: i64, regs: *mut pt_regs) {
    let load_addr = (addr + ldr_displacement(opcode) as i64) as *const i32;
    let xn = (opcode & 0x1f) as i32;
    set_x_reg(regs, xn, core::ptr::read_volatile(load_addr) as i64 as u64);
    instruction_pointer_set(regs, instruction_pointer(regs) + 4);
}

pub unsafe fn simulate_nop(_opcode: u32, _addr: i64, regs: *mut pt_regs) {
    arm64_skip_faulting_instruction(regs, AARCH64_INSN_SIZE);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
