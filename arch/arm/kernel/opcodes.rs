// SPDX-License-Identifier: GPL-2.0-only
/*
 *  linux/arch/arm/kernel/opcodes.c
 *
 *  A32 condition code lookup feature moved from nwfpe/fpopcode.c
 */

// Dependencies supplied by the surrounding kernel Rust bindings/module.

const ARM_OPCODE_CONDITION_UNCOND: u32 = 0xf;

/*
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
 * Returns:
 * ARM_OPCODE_CONDTEST_FAIL   - if condition fails
 * ARM_OPCODE_CONDTEST_PASS   - if condition passes (including AL)
 * ARM_OPCODE_CONDTEST_UNCOND - if NV condition, or separate unconditional
 *                              opcode space from v5 onwards
 *
 * Code that tests whether a conditional instruction would pass its condition
 * check should check that return value == ARM_OPCODE_CONDTEST_PASS.
 *
 * Code that tests if a condition means that the instruction would be executed
 * (regardless of conditional or unconditional) should instead check that the
 * return value != ARM_OPCODE_CONDTEST_FAIL.
 */
#[no_mangle]
pub unsafe extern "C" fn arm_check_condition(opcode: u32, psr: u32) -> u32 {
    let cc_bits = opcode >> 28;
    let psr_cond = psr >> 28;
    let ret: u32;

    if cc_bits != ARM_OPCODE_CONDITION_UNCOND {
        if ((CC_MAP[cc_bits as usize] as u32 >> psr_cond) & 1) != 0 {
            ret = ARM_OPCODE_CONDTEST_PASS;
        } else {
            ret = ARM_OPCODE_CONDTEST_FAIL;
        }
    } else {
        ret = ARM_OPCODE_CONDTEST_UNCOND;
    }

    ret
}

// EXPORT_SYMBOL_GPL(arm_check_condition)

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
