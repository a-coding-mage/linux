// SPDX-License-Identifier: GPL-2.0
/*
 * Hyp portion of the (not much of an) Emulation layer for 32bit guests.
 *
 * Copyright (C) 2012,2013 - ARM Ltd
 * Author: Marc Zyngier <marc.zyngier@arm.com>
 *
 * based on arch/arm/kvm/emulate.c
 * Copyright (C) 2012 - Virtual Open Systems and Columbia University
 * Author: Christoffer Dall <c.dall@virtualopensystems.com>
 */

// External kernel declarations supplied by the surrounding build.
use core::ffi::c_int;

#[repr(C)]
pub struct kvm_vcpu {
    _private: [u8; 0],
}

extern "C" {
    fn kvm_vcpu_trap_get_class(vcpu: *const kvm_vcpu) -> u32;
    fn kvm_vcpu_get_condition(vcpu: *const kvm_vcpu) -> c_int;
    fn vcpu_cpsr(vcpu: *const kvm_vcpu) -> *mut usize;
    fn vcpu_pc(vcpu: *const kvm_vcpu) -> *mut u32;
    fn kvm_vcpu_trap_il_is32bit(vcpu: *const kvm_vcpu) -> bool;
}

// ESR_ELx_EC_* values, PSR_AA32_* values, and their definitions are supplied
// by the kernel headers included by the original C source.

/*
 * stolen from arch/arm/kernel/opcodes.c
 *
 * condition code lookup table
 * index into the table is test code: EQ, NE, ... LT, GT, AL, NV
 *
 * bit position in short is condition code: NZCV
 */
static CC_MAP: [u16; 16] = [
    0xF0F0, // EQ == Z set
    0x0F0F, // NE
    0xCCCC, // CS == C set
    0x3333, // CC
    0xFF00, // MI == N set
    0x00FF, // PL
    0xAAAA, // VS == V set
    0x5555, // VC
    0x0C0C, // HI == C set && Z clear
    0xF3F3, // LS == C clear || Z set
    0xAA55, // GE == (N==V)
    0x55AA, // LT == (N!=V)
    0x0A05, // GT == (!Z && (N==V))
    0xF5FA, // LE == (Z || (N!=V))
    0xFFFF, // AL always
    0,       // NV
];

/*
 * Check if a trapped instruction should have been executed or not.
 */
pub unsafe fn kvm_condition_valid32(vcpu: *const kvm_vcpu) -> bool {
    let cpsr: usize;
    let cpsr_cond: usize;
    let mut cond: c_int;

    /*
     * These are the exception classes that could fire with a
     * conditional instruction.
     */
    match kvm_vcpu_trap_get_class(vcpu) {
        ESR_ELx_EC_CP15_32
        | ESR_ELx_EC_CP15_64
        | ESR_ELx_EC_CP14_MR
        | ESR_ELx_EC_CP14_LS
        | ESR_ELx_EC_FP_ASIMD
        | ESR_ELx_EC_CP10_ID
        | ESR_ELx_EC_CP14_64
        | ESR_ELx_EC_SVC32 => {}
        _ => return true,
    }

    /* Is condition field valid? */
    cond = kvm_vcpu_get_condition(vcpu);
    if cond == 0xE {
        return true;
    }

    cpsr = *vcpu_cpsr(vcpu);

    if cond < 0 {
        /* This can happen in Thumb mode: examine IT state. */
        let mut it: usize;

        it = ((cpsr >> 8) & 0xFC) | ((cpsr >> 25) & 0x3);

        /* it == 0 => unconditional. */
        if it == 0 {
            return true;
        }

        /* The cond for this insn works out as the top 4 bits. */
        cond = (it >> 4) as c_int;
    }

    cpsr_cond = cpsr >> 28;

    if ((CC_MAP[cond as usize] as usize >> cpsr_cond) & 1) == 0 {
        return false;
    }

    true
}

/**
 * kvm_adjust_itstate - adjust ITSTATE when emulating instructions in IT-block
 * @vcpu:      The VCPU pointer
 *
 * When exceptions occur while instructions are executed in Thumb IF-THEN
 * blocks, the ITSTATE field of the CPSR is not advanced (updated), so we have
 * to do this little bit of work manually. The fields map like this:
 *
 * IT[7:0] -> CPSR[26:25],CPSR[15:10]
 */
unsafe fn kvm_adjust_itstate(vcpu: *mut kvm_vcpu) {
    let mut itbits: usize;
    let mut cond: usize;
    let mut cpsr = *vcpu_cpsr(vcpu);
    let is_arm = (cpsr & PSR_AA32_T_BIT) == 0;

    if is_arm || (cpsr & PSR_AA32_IT_MASK) == 0 {
        return;
    }

    cond = (cpsr & 0xe000) >> 13;
    itbits = (cpsr & 0x1c00) >> (10 - 2);
    itbits |= (cpsr & (0x3 << 25)) >> 25;

    /* Perform ITAdvance (see page A2-52 in ARM DDI 0406C) */
    if (itbits & 0x7) == 0 {
        itbits = 0;
        cond = 0;
    } else {
        itbits = (itbits << 1) & 0x1f;
    }

    cpsr &= !PSR_AA32_IT_MASK;
    cpsr |= cond << 13;
    cpsr |= (itbits & 0x1c) << (10 - 2);
    cpsr |= (itbits & 0x3) << 25;
    *vcpu_cpsr(vcpu) = cpsr;
}

/**
 * kvm_skip_instr32 - skip a trapped instruction and proceed to the next
 * @vcpu: The vcpu pointer
 */
pub unsafe fn kvm_skip_instr32(vcpu: *mut kvm_vcpu) {
    let mut pc = *vcpu_pc(vcpu);
    let is_thumb: bool;

    is_thumb = (*vcpu_cpsr(vcpu) & PSR_AA32_T_BIT) != 0;
    if is_thumb && !kvm_vcpu_trap_il_is32bit(vcpu) {
        pc = pc.wrapping_add(2);
    } else {
        pc = pc.wrapping_add(4);
    }

    *vcpu_pc(vcpu) = pc;

    kvm_adjust_itstate(vcpu);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
