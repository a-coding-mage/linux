// SPDX-License-Identifier: GPL-2.0+

// External kernel and instruction-decoder dependencies are supplied by the
// surrounding translation unit.

#[inline]
unsafe fn rv_insn_reg_get_val(regs: *mut pt_regs, index: u32, ptr: *mut usize) -> bool {
    if index == 0 {
        *ptr = 0;
    } else if index <= 31 {
        *ptr = *((regs as *mut usize).add(index as usize));
    } else {
        return false;
    }

    true
}

#[inline]
unsafe fn rv_insn_reg_set_val(regs: *mut pt_regs, index: u32, val: usize) -> bool {
    if index == 0 {
        return true;
    } else if index <= 31 {
        *((regs as *mut usize).add(index as usize)) = val;
    } else {
        return false;
    }

    true
}

pub unsafe fn simulate_jal(opcode: u32, addr: usize, regs: *mut pt_regs) -> bool {
    /*
     *     31    30       21    20     19        12 11 7 6      0
     * imm [20] | imm[10:1] | imm[11] | imm[19:12] | rd | opcode
     *     1         10          1           8       5    JAL/J
     */
    let index: u32 = RV_EXTRACT_RD_REG(opcode);

    let ret = rv_insn_reg_set_val(regs, index, addr + 4);
    if !ret {
        return ret;
    }

    let imm: i32 = RV_EXTRACT_JTYPE_IMM(opcode);

    instruction_pointer_set(regs, addr.wrapping_add(imm as usize));

    ret
}

pub unsafe fn simulate_jalr(opcode: u32, addr: usize, regs: *mut pt_regs) -> bool {
    /*
     * 31          20 19 15 14 12 11 7 6      0
     *  offset[11:0] | rs1 | 010 | rd | opcode
     *      12         5      3    5    JALR/JR
     */
    let imm: u32 = RV_EXTRACT_ITYPE_IMM(opcode);
    let rd_index: u32 = RV_EXTRACT_RD_REG(opcode);
    let rs1_index: u32 = RV_EXTRACT_RS1_REG(opcode);
    let mut base_addr: usize = 0;

    let mut ret = rv_insn_reg_get_val(regs, rs1_index, &mut base_addr);
    if !ret {
        return ret;
    }

    ret = rv_insn_reg_set_val(regs, rd_index, addr + 4);
    if !ret {
        return ret;
    }

    instruction_pointer_set(
        regs,
        (base_addr.wrapping_add(sign_extend32(imm, 11) as usize)) & !1,
    );

    ret
}

pub unsafe fn simulate_auipc(opcode: u32, addr: usize, regs: *mut pt_regs) -> bool {
    /*
     * auipc instruction:
     *  31        12 11 7 6      0
     * | imm[31:12] | rd | opcode |
     *        20       5     7
     */

    let rd_idx: u32 = RV_EXTRACT_RD_REG(opcode);
    let rd_val: usize = addr.wrapping_add(RV_EXTRACT_UTYPE_IMM(opcode) as i32 as usize);

    if !rv_insn_reg_set_val(regs, rd_idx, rd_val) {
        return false;
    }

    instruction_pointer_set(regs, addr + 4);

    true
}

pub unsafe fn simulate_branch(opcode: u32, addr: usize, regs: *mut pt_regs) -> bool {
    /*
     * branch instructions:
     *      31    30       25 24 20 19 15 14    12 11       8    7      6      0
     * | imm[12] | imm[10:5] | rs2 | rs1 | funct3 | imm[4:1] | imm[11] | opcode |
     *     1           6        5     5      3         4         1         7
     *     imm[12|10:5]        rs2   rs1    000       imm[4:1|11]       1100011  BEQ
     *     imm[12|10:5]        rs2   rs1    001       imm[4:1|11]       1100011  BNE
     *     imm[12|10:5]        rs2   rs1    100       imm[4:1|11]       1100011  BLT
     *     imm[12|10:5]        rs2   rs1    101       imm[4:1|11]       1100011  BGE
     *     imm[12|10:5]        rs2   rs1    110       imm[4:1|11]       1100011  BLTU
     *     imm[12|10:5]        rs2   rs1    111       imm[4:1|11]       1100011  BGEU
     */

    let mut rs1_val: usize = 0;
    let mut rs2_val: usize = 0;
    if !rv_insn_reg_get_val(regs, RV_EXTRACT_RS1_REG(opcode), &mut rs1_val)
        || !rv_insn_reg_get_val(regs, RV_EXTRACT_RS2_REG(opcode), &mut rs2_val)
    {
        return false;
    }

    let offset_tmp: i32 = RV_EXTRACT_BTYPE_IMM(opcode);
    let offset: i32 = match RV_EXTRACT_FUNCT3(opcode) {
        RVG_FUNCT3_BEQ => if rs1_val == rs2_val { offset_tmp } else { 4 },
        RVG_FUNCT3_BNE => if rs1_val != rs2_val { offset_tmp } else { 4 },
        RVG_FUNCT3_BLT => if (rs1_val as isize) < (rs2_val as isize) { offset_tmp } else { 4 },
        RVG_FUNCT3_BGE => if (rs1_val as isize) >= (rs2_val as isize) { offset_tmp } else { 4 },
        RVG_FUNCT3_BLTU => if rs1_val < rs2_val { offset_tmp } else { 4 },
        RVG_FUNCT3_BGEU => if rs1_val >= rs2_val { offset_tmp } else { 4 },
        _ => return false,
    };

    instruction_pointer_set(regs, addr.wrapping_add(offset as usize));

    true
}

pub unsafe fn simulate_c_j(opcode: u32, addr: usize, regs: *mut pt_regs) -> bool {
    let offset: i32 = RVC_EXTRACT_JTYPE_IMM(opcode);
    instruction_pointer_set(regs, addr.wrapping_add(offset as usize));
    true
}

pub unsafe fn simulate_c_jal(opcode: u32, addr: usize, regs: *mut pt_regs) -> bool {
    (*regs).ra = addr + 2;
    simulate_c_j(opcode, addr, regs)
}

unsafe fn simulate_c_jr_jalr(
    opcode: u32,
    addr: usize,
    regs: *mut pt_regs,
    is_jalr: bool,
) -> bool {
    /*
     *  15    12 11  7 6   2 1  0
     * | funct4 | rs1 | rs2 | op |
     *     4       5     5    2
     */

    let rs1: u32 = RVC_EXTRACT_C2_RS1_REG(opcode);
    if rs1 == 0 {
        return false;
    }

    let mut jump_addr: usize = 0;
    if !rv_insn_reg_get_val(regs, rs1, &mut jump_addr) {
        return false;
    }

    if is_jalr && !rv_insn_reg_set_val(regs, 1, addr + 2) {
        return false;
    }

    instruction_pointer_set(regs, jump_addr);
    true
}

pub unsafe fn simulate_c_jr(opcode: u32, addr: usize, regs: *mut pt_regs) -> bool {
    simulate_c_jr_jalr(opcode, addr, regs, false)
}

pub unsafe fn simulate_c_jalr(opcode: u32, addr: usize, regs: *mut pt_regs) -> bool {
    simulate_c_jr_jalr(opcode, addr, regs, true)
}

unsafe fn simulate_c_bnez_beqz(
    opcode: u32,
    addr: usize,
    regs: *mut pt_regs,
    is_bnez: bool,
) -> bool {
    /*
     *  15    13 12           10 9    7 6                 2 1  0
     * | funct3 | offset[8|4:3] | rs1' | offset[7:6|2:1|5] | op |
     *     3            3          3             5           2
     */

    let rs1: u32 = 0x8 | ((opcode >> 7) & 0x7);
    let mut rs1_val: usize = 0;
    if !rv_insn_reg_get_val(regs, rs1, &mut rs1_val) {
        return false;
    }

    let offset: i32 = if (rs1_val != 0 && is_bnez) || (rs1_val == 0 && !is_bnez) {
        RVC_EXTRACT_BTYPE_IMM(opcode)
    } else {
        2
    };

    instruction_pointer_set(regs, addr.wrapping_add(offset as usize));
    true
}

pub unsafe fn simulate_c_bnez(opcode: u32, addr: usize, regs: *mut pt_regs) -> bool {
    simulate_c_bnez_beqz(opcode, addr, regs, true)
}

pub unsafe fn simulate_c_beqz(opcode: u32, addr: usize, regs: *mut pt_regs) -> bool {
    simulate_c_bnez_beqz(opcode, addr, regs, false)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
