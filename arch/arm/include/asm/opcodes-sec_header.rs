/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *
 * Copyright (C) 2012 ARM Limited
 */

// Dependency: <asm/opcodes.h>

macro_rules! __SMC {
    ($imm4:expr) => {
        __inst_arm_thumb32(
            0xE1600070u32 | (($imm4 & 0xF) << 0),
            0xF7F08000u32 | (($imm4 & 0xF) << 16),
        )
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
