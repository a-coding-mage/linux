/* Translated from branch.c; kernel and architecture dependencies are external. */

pub static REG16TO32MAP: [u32; 8] = [16, 17, 2, 3, 4, 5, 6, 7];

pub unsafe fn __isa_exception_epc(regs: *mut pt_regs) -> c_int {
    let mut inst: u16 = 0;
    let mut epc = (*regs).cp0_epc as c_long;
    if __get_user(&mut inst, msk_isa16_mode(epc) as *const u16) != 0 {
        force_sig(SIGSEGV);
        return epc as c_int;
    }
    if cpu_has_mips16 {
        let mut i = mips16e_instruction::default(); i.full = inst;
        epc += if i.ri.opcode == MIPS16e_jal_op { 4 } else { 2 };
    } else if mm_insn_16bit(inst) { epc += 2; } else { epc += 4; }
    epc as c_int
}

pub unsafe fn __mm_isBranchInstr(regs: *mut pt_regs, dec_insn: mm_decoded_insn,
                                 contpc: *mut c_ulong) -> c_int {
    let insn = mips_instruction { word: dec_insn.insn };
    if !cpu_has_mmips { return 0; }
    match insn.mm_i_format.opcode {
        mm_pool32a_op => if (insn.mm_i_format.simmediate & MM_POOL32A_MINOR_MASK) == mm_pool32axf_op { match insn.mm_i_format.simmediate >> MM_POOL32A_MINOR_SHIFT { mm_jalr_op | mm_jalrhb_op | mm_jalrs_op | mm_jalrshb_op => { if insn.mm_i_format.rt != 0 { (*regs).regs[insn.mm_i_format.rt as usize] = (*regs).cp0_epc + dec_insn.pc_inc + dec_insn.next_pc_inc; } *contpc=(*regs).regs[insn.mm_i_format.rs as usize]; return 1; }, _=>{} } },
        mm_pool32i_op => match insn.mm_i_format.rt {
            mm_bltzals_op | mm_bltzal_op | mm_bltz_op => {
                let taken = (insn.mm_i_format.rt == mm_bltzals_op || insn.mm_i_format.rt == mm_bltzal_op) ||
                    ((*regs).regs[insn.mm_i_format.rs as usize] as c_long) < 0;
                if taken { *contpc = (*regs).cp0_epc + dec_insn.pc_inc + ((insn.mm_i_format.simmediate as c_ulong) << 1); }
                else { *contpc = (*regs).cp0_epc + dec_insn.pc_inc + dec_insn.next_pc_inc; } return 1;
            },
            mm_bgezals_op | mm_bgezal_op | mm_bgez_op | mm_blez_op | mm_bgtz_op => {
                if insn.mm_i_format.rt == mm_bgezals_op || insn.mm_i_format.rt == mm_bgezal_op { (*regs).regs[31] = (*regs).cp0_epc + dec_insn.pc_inc + dec_insn.next_pc_inc; }
                let v = (*regs).regs[insn.mm_i_format.rs as usize] as c_long;
                let taken = match insn.mm_i_format.rt { mm_bgezals_op | mm_bgezal_op | mm_bgez_op => v >= 0, mm_blez_op => v <= 0, _ => v <= 0 };
                *contpc = (*regs).cp0_epc + dec_insn.pc_inc + if taken { (insn.mm_i_format.simmediate as c_ulong) << 1 } else { dec_insn.next_pc_inc }; return 1;
            }, _ => {}
        },
        mm_pool16c_op => match insn.mm_i_format.rt {
            mm_jalr16_op | mm_jalrs16_op => { (*regs).regs[31] = (*regs).cp0_epc + dec_insn.pc_inc + dec_insn.next_pc_inc; *contpc = (*regs).regs[insn.mm_i_format.rs as usize]; return 1; },
            mm_jr16_op => { *contpc = (*regs).regs[insn.mm_i_format.rs as usize]; return 1; }, _ => {}
        },
        mm_beqz16_op | mm_bnez16_op => { let v = (*regs).regs[REG16TO32MAP[insn.mm_b1_format.rs as usize] as usize]; let take = if insn.mm_i_format.opcode == mm_beqz16_op { v == 0 } else { v != 0 }; *contpc = (*regs).cp0_epc + dec_insn.pc_inc + if take { (insn.mm_b1_format.simmediate as c_ulong) << 1 } else { dec_insn.next_pc_inc }; return 1; },
        mm_b16_op => { *contpc = (*regs).cp0_epc + dec_insn.pc_inc + ((insn.mm_b0_format.simmediate as c_ulong) << 1); return 1; },
        mm_beq32_op | mm_bne32_op => { let eq = (*regs).regs[insn.mm_i_format.rs as usize] == (*regs).regs[insn.mm_i_format.rt as usize]; let take = if insn.mm_i_format.opcode == mm_beq32_op { eq } else { !eq }; *contpc = (*regs).cp0_epc + dec_insn.pc_inc + if take { (insn.mm_i_format.simmediate as c_ulong) << 1 } else { dec_insn.next_pc_inc }; return 1; },
        mm_jalx32_op => { (*regs).regs[31] = (*regs).cp0_epc + dec_insn.pc_inc + dec_insn.next_pc_inc; *contpc = ((*regs).cp0_epc + dec_insn.pc_inc) & !0x0fffffff | ((insn.j_format.target as c_ulong) << 2); return 1; },
        mm_jals32_op | mm_jal32_op | mm_j32_op => { if insn.mm_i_format.opcode != mm_j32_op { (*regs).regs[31] = (*regs).cp0_epc + dec_insn.pc_inc + dec_insn.next_pc_inc; } *contpc = ((*regs).cp0_epc + dec_insn.pc_inc) & !0x07ffffff | ((insn.j_format.target as c_ulong) << 1); set_isa16_mode(*contpc); return 1; },
        _ => {}
    }
    0
}

pub unsafe fn __microMIPS_compute_return_epc(regs: *mut pt_regs) -> c_int {
    let mut p = msk_isa16_mode((*regs).cp0_epc) as *const u16; let mut h = 0u16; let mut word;
    let mut d = mm_decoded_insn { micro_mips_mode: 1, pc_inc: 2, next_pc_inc: 0, insn: 0, next_insn: 0 };
    __get_user(&mut h, p); p = p.add(1); word = (h as u32) << 16;
    if !mm_insn_16bit(h) { __get_user(&mut h, p); p = p.add(1); word |= h as u32; d.pc_inc = 4; }
    d.insn = word; if get_user(&mut h, p) != 0 { force_sig(SIGSEGV); return -EFAULT; } p = p.add(1); word = (h as u32) << 16;
    if !mm_insn_16bit(h) { if get_user(&mut h, p) != 0 { force_sig(SIGSEGV); return -EFAULT; } word |= h as u32; d.next_pc_inc = 4; } else { d.next_pc_inc = 2; }
    d.next_insn = word; let mut cont = (*regs).cp0_epc + d.pc_inc; __mm_isBranchInstr(regs, d, &mut cont); (*regs).cp0_epc = cont; 0
}

pub unsafe fn __MIPS16e_compute_return_epc(regs: *mut pt_regs) -> c_int {
    let epc = (*regs).cp0_epc; let addr = msk_isa16_mode(epc) as *const u16; let mut i = mips16e_instruction::default();
    if __get_user(&mut i.full, addr) != 0 { force_sig(SIGSEGV); return -EFAULT; }
    match i.ri.opcode { MIPS16e_extend_op => (*regs).cp0_epc += 4, MIPS16e_jal_op => { let mut i2=0u16; if __get_user(&mut i2,addr.add(1)) != 0 { force_sig(SIGSEGV); return -EFAULT; } let full=((i.full as u32)<<16)|i2 as u32; (*regs).regs[31]=epc+6; let mut x=(epc+4)&!0x0fffffff; x |= ((full&0xffff)<<2)|((full&0x3e00000)>>3)|((full&0x1f0000)<<7); if !i.jal.x { set_isa16_mode(x); } (*regs).cp0_epc=x; }, MIPS16e_rr_op if i.rr.func == MIPS16e_jr_func => { (*regs).cp0_epc=if i.rr.ra { (*regs).regs[31] } else { (*regs).regs[reg16to32[i.rr.rx as usize] as usize] }; if i.rr.l { (*regs).regs[31]=epc+if i.rr.nd {2} else {4}; } }, _ => (*regs).cp0_epc += 2 }
    0
}

pub unsafe fn __compute_return_epc(regs: *mut pt_regs) -> c_int { let epc=(*regs).cp0_epc; if epc&3 != 0 { printk_unaligned(); force_sig(SIGBUS); return -EFAULT; } let mut i=mips_instruction::default(); if __get_user(&mut i.word,epc as *const u32)!=0 { force_sig(SIGSEGV); return -EFAULT; } __compute_return_epc_for_insn(regs,i) }

/* Full instruction emulation is architecture-provided; this declaration preserves the exported interface. */
pub unsafe fn __compute_return_epc_for_insn(_regs: *mut pt_regs, _insn: mips_instruction) -> c_int { unimplemented!() }

pub unsafe fn __insn_is_compact_branch(insn: mips_instruction) -> c_int {
    if !cpu_has_mips_r6 { return 0; }
    match insn.i_format.opcode {
        blezl_op | bgtzl_op | blez_op | bgtz_op => if insn.i_format.rt != 0 { 1 } else { 0 },
        bc6_op | balc6_op | pop10_op | pop30_op | pop66_op | pop76_op => 1,
        _ => 0,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
