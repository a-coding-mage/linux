/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * opcodes-virt.h: Opcode definitions for the ARM virtualization extensions
 * Copyright (C) 2012  Linaro Limited
 */

// Dependency supplied by the translated opcode definitions: `asm/opcodes.h`.

macro_rules! __HVC {
    ($imm16:expr) => {
        __inst_arm_thumb32(
            0xE1400070u32
                | (((($imm16) as u32) & 0xFFF0) << 4)
                | (($imm16) as u32 & 0x000F),
            0xF7E08000u32
                | (((($imm16) as u32) & 0xF000) << 4)
                | (($imm16) as u32 & 0x0FFF),
        )
    };
}

macro_rules! __ERET {
    () => {
        __inst_arm_thumb32(0xE160006Eu32, 0xF3DE8F00u32)
    };
}

macro_rules! __MSR_ELR_HYP {
    ($regnum:expr) => {
        __inst_arm_thumb32(
            0xE12EF300u32 | ($regnum as u32),
            0xF3808E30u32 | (($regnum as u32) << 16),
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
