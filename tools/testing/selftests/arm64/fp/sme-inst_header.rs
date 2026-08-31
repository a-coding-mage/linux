// SPDX-License-Identifier: GPL-2.0-only
// Copyright (C) 2021-2 ARM Limited.
// Original author: Mark Brown <broonie@kernel.org>

// C/assembler header guard removed in Rust translation.

pub const REG_FPMR: u32 = S3_3_C4_C4_2;

/*
 * RDSVL X\nx, #\imm
 */
#[macro_export]
macro_rules! rdsvl {
    ($nx:expr, $imm:expr) => {{
        const INST: u32 = 0x04bf_5800u32 | (($imm as u32) << 5) | ($nx as u32);
        unsafe {
            core::arch::asm!(".inst {inst}", inst = const INST);
        }
    }};
}

#[macro_export]
macro_rules! smstop {
    () => {{
        unsafe {
            core::arch::asm!("msr S0_3_C4_C6_3, xzr");
        }
    }};
}

#[macro_export]
macro_rules! smstart_za {
    () => {{
        unsafe {
            core::arch::asm!("msr S0_3_C4_C5_3, xzr");
        }
    }};
}

#[macro_export]
macro_rules! smstart_sm {
    () => {{
        unsafe {
            core::arch::asm!("msr S0_3_C4_C3_3, xzr");
        }
    }};
}

/*
 * LDR (vector to ZA array):
 *	LDR ZA[\nw, #\offset], [X\nxbase, #\offset, MUL VL]
 */
#[macro_export]
macro_rules! _ldr_za {
    ($nw:expr, $nxbase:expr) => {{
        $crate::_ldr_za!($nw, $nxbase, 0)
    }};
    ($nw:expr, $nxbase:expr, $offset:expr) => {{
        const INST: u32 = 0xe100_0000u32
            | ((($nw as u32) & 3) << 13)
            | (($nxbase as u32) << 5)
            | (($offset as u32) & 7);
        unsafe {
            core::arch::asm!(".inst {inst}", inst = const INST);
        }
    }};
}

/*
 * STR (vector from ZA array):
 *	STR ZA[\nw, #\offset], [X\nxbase, #\offset, MUL VL]
 */
#[macro_export]
macro_rules! _str_za {
    ($nw:expr, $nxbase:expr) => {{
        $crate::_str_za!($nw, $nxbase, 0)
    }};
    ($nw:expr, $nxbase:expr, $offset:expr) => {{
        const INST: u32 = 0xe120_0000u32
            | ((($nw as u32) & 3) << 13)
            | (($nxbase as u32) << 5)
            | (($offset as u32) & 7);
        unsafe {
            core::arch::asm!(".inst {inst}", inst = const INST);
        }
    }};
}

/*
 * LDR (ZT0)
 *
 *	LDR ZT0, nx
 */
#[macro_export]
macro_rules! _ldr_zt {
    ($nx:expr) => {{
        const INST: u32 = 0xe11f_8000u32 | ((($nx as u32) & 0x1f) << 5);
        unsafe {
            core::arch::asm!(".inst {inst}", inst = const INST);
        }
    }};
}

/*
 * STR (ZT0)
 *
 *	STR ZT0, nx
 */
#[macro_export]
macro_rules! _str_zt {
    ($nx:expr) => {{
        const INST: u32 = 0xe13f_8000u32 | ((($nx as u32) & 0x1f) << 5);
        unsafe {
            core::arch::asm!(".inst {inst}", inst = const INST);
        }
    }};
}
