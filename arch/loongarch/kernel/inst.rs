// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// C headers omitted; their symbols are supplied by the surrounding kernel.

static mut patch_lock: raw_spinlock_t = DEFINE_RAW_SPINLOCK!();

pub unsafe fn simu_pc(regs: *mut pt_regs, insn: loongarch_instruction) {
    let pc = (*regs).csr_era;
    let rd = insn.reg1i20_format.rd;
    let imm = insn.reg1i20_format.immediate;

    if pc & 3 != 0 { pr_warn!("{}: invalid pc 0x{:x}\n", "simu_pc", pc); return; }
    match insn.reg1i20_format.opcode {
        pcaddi_op => (*regs).regs[rd] = pc + sign_extend64((imm << 2) as _, 21),
        pcaddu12i_op => (*regs).regs[rd] = pc + sign_extend64((imm << 12) as _, 31),
        pcaddu18i_op => (*regs).regs[rd] = pc + sign_extend64((imm << 18) as _, 37),
        pcalau12i_op => { (*regs).regs[rd] = pc + sign_extend64((imm << 12) as _, 31); (*regs).regs[rd] &= !((1 << 12) - 1); }
        _ => { pr_info!("{}: unknown opcode\n", "simu_pc"); return; }
    }
    (*regs).csr_era += LOONGARCH_INSN_SIZE;
}

pub unsafe fn simu_branch(regs: *mut pt_regs, insn: loongarch_instruction) {
    let pc = (*regs).csr_era;
    if pc & 3 != 0 { pr_warn!("{}: invalid pc 0x{:x}\n", "simu_branch", pc); return; }
    let imm_l = insn.reg0i26_format.immediate_l; let imm_h = insn.reg0i26_format.immediate_h;
    match insn.reg0i26_format.opcode {
        b_op => { (*regs).csr_era = pc + sign_extend64(((imm_h << 16 | imm_l) << 2) as _, 27); return; }
        bl_op => { (*regs).csr_era = pc + sign_extend64(((imm_h << 16 | imm_l) << 2) as _, 27); (*regs).regs[1] = pc + LOONGARCH_INSN_SIZE; return; }
        _ => {}
    }
    let imm_l = insn.reg1i21_format.immediate_l; let imm_h = insn.reg1i21_format.immediate_h; let rj = insn.reg1i21_format.rj;
    match insn.reg1i21_format.opcode {
        beqz_op => { (*regs).csr_era = if (*regs).regs[rj] == 0 { pc + sign_extend64(((imm_h << 16 | imm_l) << 2) as _, 22) } else { pc + LOONGARCH_INSN_SIZE }; return; }
        bnez_op => { (*regs).csr_era = if (*regs).regs[rj] != 0 { pc + sign_extend64(((imm_h << 16 | imm_l) << 2) as _, 22) } else { pc + LOONGARCH_INSN_SIZE }; return; }
        _ => {}
    }
    let imm = insn.reg2i16_format.immediate; let rj = insn.reg2i16_format.rj; let rd = insn.reg2i16_format.rd;
    let target = pc + sign_extend64((imm << 2) as _, 17);
    match insn.reg2i16_format.opcode {
        beq_op => (*regs).csr_era = if (*regs).regs[rj] == (*regs).regs[rd] { target } else { pc + LOONGARCH_INSN_SIZE },
        bne_op => (*regs).csr_era = if (*regs).regs[rj] != (*regs).regs[rd] { target } else { pc + LOONGARCH_INSN_SIZE },
        blt_op => (*regs).csr_era = if (*regs).regs[rj] as isize < (*regs).regs[rd] as isize { target } else { pc + LOONGARCH_INSN_SIZE },
        bge_op => (*regs).csr_era = if (*regs).regs[rj] as isize >= (*regs).regs[rd] as isize { target } else { pc + LOONGARCH_INSN_SIZE },
        bltu_op => (*regs).csr_era = if (*regs).regs[rj] < (*regs).regs[rd] { target } else { pc + LOONGARCH_INSN_SIZE },
        bgeu_op => (*regs).csr_era = if (*regs).regs[rj] >= (*regs).regs[rd] { target } else { pc + LOONGARCH_INSN_SIZE },
        jirl_op => { (*regs).csr_era = (*regs).regs[rj] + target - pc; (*regs).regs[rd] = pc + LOONGARCH_INSN_SIZE; }
        _ => { pr_info!("{}: unknown opcode\n", "simu_branch"); }
    }
}

pub unsafe fn insns_not_supported(insn: loongarch_instruction) -> bool {
    match insn.reg3_format.opcode { amswapw_op..=ammindbdu_op => { pr_notice!("atomic memory access instructions are not supported\n"); return true; }, scq_op => { pr_notice!("sc.q instruction is not supported\n"); return true; }, _ => {} }
    match insn.reg2i14_format.opcode { llw_op | lld_op | scw_op | scd_op => { pr_notice!("ll and sc instructions are not supported\n"); return true; }, _ => {} }
    match insn.reg2_format.opcode { llacqw_op | llacqd_op | screlw_op | screld_op => { pr_notice!("llacq and screl instructions are not supported\n"); return true; }, _ => {} }
    if insn.reg1i21_format.opcode == bceqz_op { pr_notice!("bceqz and bcnez instructions are not supported\n"); return true; }
    false
}

pub unsafe fn insns_need_simulation(insn: *const loongarch_instruction) -> bool { is_pc_ins(insn) || is_branch_ins(insn) }
pub unsafe fn arch_simulate_insn(insn: loongarch_instruction, regs: *mut pt_regs) { if is_pc_ins(&insn) { simu_pc(regs, insn); } else if is_branch_ins(&insn) { simu_branch(regs, insn); } }

pub unsafe fn larch_insn_read(addr: *mut core::ffi::c_void, insnp: *mut u32) -> i32 { let mut val = 0; let ret = copy_from_kernel_nofault(&mut val as *mut _, addr, LOONGARCH_INSN_SIZE); if ret == 0 { *insnp = val; } ret }
pub unsafe fn larch_insn_write(addr: *mut core::ffi::c_void, insn: u32) -> i32 { if addr as usize & 3 != 0 { return -EINVAL; } let mut flags = 0; raw_spin_lock_irqsave(&raw mut patch_lock, &mut flags); let ret = copy_to_kernel_nofault(addr, &insn as *const _, LOONGARCH_INSN_SIZE); raw_spin_unlock_irqrestore(&raw mut patch_lock, flags); ret }
pub unsafe fn larch_insn_patch_text(addr: *mut core::ffi::c_void, insn: u32) -> i32 { let ret = larch_insn_write(addr, insn); if ret == 0 { flush_icache_range(addr as usize, addr as usize + LOONGARCH_INSN_SIZE); } ret }

#[repr(C)] struct insn_copy { dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, len: usize, cpu: u32 }
unsafe fn text_copy_cb(data: *mut core::ffi::c_void) -> i32 { let copy = &mut *(data as *mut insn_copy); if smp_processor_id() == copy.cpu { let ret = copy_to_kernel_nofault(copy.dst, copy.src, copy.len); if ret != 0 { pr_err!("{}: operation failed\n", "text_copy_cb"); return ret; } } flush_icache_range(copy.dst as usize, copy.dst as usize + copy.len); 0 }
pub unsafe fn larch_insn_text_copy(dst: *mut core::ffi::c_void, src: *mut core::ffi::c_void, len: usize) -> i32 { let mut copy = insn_copy { dst, src, len, cpu: raw_smp_processor_id() }; lockdep_assert_cpus_held(); let start = round_down(dst as usize, PAGE_SIZE); let end = round_up(dst as usize + len, PAGE_SIZE); let err = set_memory_rw(start, (end-start)/PAGE_SIZE); if err != 0 { pr_info!("{}: set_memory_rw() failed\n", "larch_insn_text_copy"); return err; } let ret = stop_machine_cpuslocked(text_copy_cb, &mut copy as *mut _ as _, cpu_online_mask); let err = set_memory_rox(start, (end-start)/PAGE_SIZE); if err != 0 { pr_info!("{}: set_memory_rox() failed\n", "larch_insn_text_copy"); return err; } ret }

pub fn larch_insn_gen_nop() -> u32 { INSN_NOP }
pub unsafe fn larch_insn_gen_b(pc: usize, dest: usize) -> u32 { let offset = dest as isize - pc as isize; let mut insn = loongarch_instruction::default(); if offset & 3 != 0 || offset < -SZ_128M || offset >= SZ_128M { pr_warn!("The generated b instruction is out of range.\n"); return INSN_BREAK; } emit_b(&mut insn, offset >> 2); insn.word }
pub unsafe fn larch_insn_gen_bl(pc: usize, dest: usize) -> u32 { let offset = dest as isize - pc as isize; let mut insn = loongarch_instruction::default(); if offset & 3 != 0 || offset < -SZ_128M || offset >= SZ_128M { pr_warn!("The generated bl instruction is out of range.\n"); return INSN_BREAK; } emit_bl(&mut insn, offset >> 2); insn.word }
pub unsafe fn larch_insn_gen_break(imm: i32) -> u32 { let mut insn = loongarch_instruction::default(); if imm < 0 || imm >= SZ_32K { pr_warn!("The generated break instruction is out of range.\n"); return INSN_BREAK; } emit_break(&mut insn, imm); insn.word }
pub unsafe fn larch_insn_gen_or(rd: loongarch_gpr, rj: loongarch_gpr, rk: loongarch_gpr) -> u32 { let mut insn = loongarch_instruction::default(); emit_or(&mut insn, rd, rj, rk); insn.word }
pub unsafe fn larch_insn_gen_move(rd: loongarch_gpr, rj: loongarch_gpr) -> u32 { larch_insn_gen_or(rd, rj, 0) }
pub unsafe fn larch_insn_gen_lu12iw(rd: loongarch_gpr, imm: i32) -> u32 { let mut insn = loongarch_instruction::default(); if imm < -SZ_512K || imm >= SZ_512K { pr_warn!("The generated lu12i.w instruction is out of range.\n"); return INSN_BREAK; } emit_lu12iw(&mut insn, rd, imm); insn.word }
pub unsafe fn larch_insn_gen_lu32id(rd: loongarch_gpr, imm: i32) -> u32 { let mut insn = loongarch_instruction::default(); if imm < -SZ_512K || imm >= SZ_512K { pr_warn!("The generated lu32i.d instruction is out of range.\n"); return INSN_BREAK; } emit_lu32id(&mut insn, rd, imm); insn.word }
pub unsafe fn larch_insn_gen_lu52id(rd: loongarch_gpr, rj: loongarch_gpr, imm: i32) -> u32 { let mut insn = loongarch_instruction::default(); if imm < -SZ_2K || imm >= SZ_2K { pr_warn!("The generated lu52i.d instruction is out of range.\n"); return INSN_BREAK; } emit_lu52id(&mut insn, rd, rj, imm); insn.word }
pub unsafe fn larch_insn_gen_beq(rd: loongarch_gpr, rj: loongarch_gpr, imm: i32) -> u32 { let mut insn = loongarch_instruction::default(); if imm & 3 != 0 || imm < -SZ_128K || imm >= SZ_128K { pr_warn!("The generated beq instruction is out of range.\n"); return INSN_BREAK; } emit_beq(&mut insn, rj, rd, imm >> 2); insn.word }
pub unsafe fn larch_insn_gen_bne(rd: loongarch_gpr, rj: loongarch_gpr, imm: i32) -> u32 { let mut insn = loongarch_instruction::default(); if imm & 3 != 0 || imm < -SZ_128K || imm >= SZ_128K { pr_warn!("The generated bne instruction is out of range.\n"); return INSN_BREAK; } emit_bne(&mut insn, rj, rd, imm >> 2); insn.word }
pub unsafe fn larch_insn_gen_jirl(rd: loongarch_gpr, rj: loongarch_gpr, imm: i32) -> u32 { let mut insn = loongarch_instruction::default(); if imm & 3 != 0 || imm < -SZ_128K || imm >= SZ_128K { pr_warn!("The generated jirl instruction is out of range.\n"); return INSN_BREAK; } emit_jirl(&mut insn, rd, rj, imm >> 2); insn.word }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
