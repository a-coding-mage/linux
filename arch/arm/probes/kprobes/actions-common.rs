// SPDX-License-Identifier: GPL-2.0-only
/*
 * arch/arm/probes/kprobes/actions-common.c
 *
 * Copyright (C) 2011 Jon Medhurst <tixy@yxit.co.uk>.
 *
 * Some contents moved here from arch/arm/include/asm/kprobes-arm.c which is
 * Copyright (C) 2006, 2007 Motorola Inc.
 */

// Dependencies supplied by the surrounding kernel translation unit.

unsafe fn simulate_ldm1stm1(
    insn: probes_opcode_t,
    _asi: *mut arch_probes_insn,
    regs: *mut pt_regs,
) {
    let rn = ((insn >> 16) & 0xf) as usize;
    let lbit = insn & (1 << 20);
    let wbit = insn & (1 << 21);
    let ubit = insn & (1 << 23);
    let pbit = insn & (1 << 24);
    let mut addr = (*regs).uregs[rn] as *mut libc::c_long;
    let mut reg_bit_vector: u32;
    let mut reg_count: isize;

    reg_count = 0;
    reg_bit_vector = insn & 0xffff;
    while reg_bit_vector != 0 {
        reg_bit_vector &= reg_bit_vector.wrapping_sub(1);
        reg_count += 1;
    }

    if ubit == 0 {
        addr = addr.offset(-reg_count);
    }
    addr = addr.offset(((!((pbit != 0) as i32) == !((ubit != 0) as i32)) as isize));

    reg_bit_vector = insn & 0xffff;
    while reg_bit_vector != 0 {
        let reg = reg_bit_vector.trailing_zeros() as usize;
        reg_bit_vector &= reg_bit_vector.wrapping_sub(1);
        if lbit != 0 {
            (*regs).uregs[reg] = *addr as _;
        } else {
            *addr = (*regs).uregs[reg] as _;
        }
        addr = addr.offset(1);
    }

    if wbit != 0 {
        if ubit == 0 {
            addr = addr.offset(-reg_count);
        }
        addr = addr.offset(-(((!((pbit != 0) as i32) == !((ubit != 0) as i32))) as isize));
        (*regs).uregs[rn] = addr as _;
    }
}

unsafe fn simulate_stm1_pc(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    let addr = (*regs).ARM_pc.wrapping_sub(4) as usize;
    (*regs).ARM_pc = (addr as long) + str_pc_offset;
    simulate_ldm1stm1(insn, asi, regs);
    (*regs).ARM_pc = (addr as long) + 4;
}

unsafe fn simulate_ldm1_pc(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    simulate_ldm1stm1(insn, asi, regs);
    load_write_pc((*regs).ARM_pc, regs);
}

unsafe fn emulate_generic_r0_12_noflags(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    // The original implementation is architecture-specific inline assembly.
    // Preserve the external operation as a dependency of this translation.
    extern "C" {
        fn emulate_generic_r0_12_noflags_asm(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs);
    }
    emulate_generic_r0_12_noflags_asm(insn, asi, regs);
}

unsafe fn emulate_generic_r2_14_noflags(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    emulate_generic_r0_12_noflags(insn, asi, ((*regs).uregs.as_mut_ptr().add(2)) as *mut pt_regs);
}

unsafe fn emulate_ldm_r3_15(insn: probes_opcode_t, asi: *mut arch_probes_insn, regs: *mut pt_regs) {
    emulate_generic_r0_12_noflags(insn, asi, ((*regs).uregs.as_mut_ptr().add(3)) as *mut pt_regs);
    load_write_pc((*regs).ARM_pc, regs);
}

unsafe fn kprobe_decode_ldmstm(insn: probes_opcode_t, asi: *mut arch_probes_insn, _h: *const decode_header) -> probes_insn {
    let mut handler: Option<probes_insn_handler_t> = None;
    let mut reglist = insn & 0xffff;
    let is_ldm = insn & 0x100000;
    let mut rn = (insn >> 16) & 0xf;

    if rn <= 12 && (reglist & 0xe000) == 0 {
        handler = Some(emulate_generic_r0_12_noflags);
    } else if rn >= 2 && (reglist & 0x8003) == 0 {
        rn -= 2;
        reglist >>= 2;
        handler = Some(emulate_generic_r2_14_noflags);
    } else if rn >= 3 && (reglist & 0x0007) == 0 {
        if is_ldm != 0 && (reglist & 0x8000) != 0 {
            rn -= 3;
            reglist >>= 3;
            handler = Some(emulate_ldm_r3_15);
        }
    }

    if let Some(handler) = handler {
        (*asi).insn[0] = __opcode_to_mem_arm((insn & 0xfff00000) | (rn << 16) | reglist);
        (*asi).insn_handler = handler;
        return INSN_GOOD;
    }

    let handler = if (reglist & 0x8000) != 0 {
        if is_ldm != 0 { simulate_ldm1_pc } else { simulate_stm1_pc }
    } else { simulate_ldm1stm1 };
    (*asi).insn_handler = handler;
    INSN_GOOD_NO_SLOT
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
