/* SPDX-License-Identifier: GPL-2.0-only */
/* Direct Rust translation of arch/arm/include/asm/opcodes.h. */

extern "C" {
    pub fn arm_check_condition(opcode: u32, psr: u32) -> u32;
}

pub const ARM_OPCODE_CONDTEST_FAIL: u32 = 0;
pub const ARM_OPCODE_CONDTEST_PASS: u32 = 1;
pub const ARM_OPCODE_CONDTEST_UNCOND: u32 = 2;

macro_rules! ___asm_opcode_swab32 {
    ($x:expr) => {
        (($x << 24) & 0xFF000000)
            | (($x << 8) & 0x00FF0000)
            | (($x >> 8) & 0x0000FF00)
            | (($x >> 24) & 0x000000FF)
    };
}
macro_rules! ___asm_opcode_swab16 {
    ($x:expr) => { (($x << 8) & 0xFF00) | (($x >> 8) & 0x00FF) };
}
macro_rules! ___asm_opcode_swahb32 {
    ($x:expr) => { (($x << 8) & 0xFF00FF00) | (($x >> 8) & 0x00FF00FF) };
}
macro_rules! ___asm_opcode_swahw32 {
    ($x:expr) => { (($x << 16) & 0xFFFF0000) | (($x >> 16) & 0x0000FFFF) };
}
macro_rules! ___asm_opcode_identity32 { ($x:expr) => { $x & 0xFFFFFFFF }; }
macro_rules! ___asm_opcode_identity16 { ($x:expr) => { $x & 0xFFFF }; }

/* C build-time __ASSEMBLY__ and CONFIG_CPU_ENDIAN_* branches are retained as
 * macro-level Rust equivalents; external swab symbols are supplied elsewhere. */
macro_rules! ___opcode_swab32 { ($x:expr) => { ($x as u32).swap_bytes() }; }
macro_rules! ___opcode_swab16 { ($x:expr) => { ($x as u16).swap_bytes() }; }
macro_rules! ___opcode_swahb32 { ($x:expr) => { (($x << 8) & 0xFF00FF00) | (($x >> 8) & 0x00FF00FF) }; }
macro_rules! ___opcode_swahw32 { ($x:expr) => { (($x << 16) & 0xFFFF0000) | (($x >> 16) & 0x0000FFFF) }; }
macro_rules! ___opcode_identity32 { ($x:expr) => { $x as u32 }; }
macro_rules! ___opcode_identity16 { ($x:expr) => { $x as u16 }; }

macro_rules! __opcode_to_mem_arm { ($x:expr) => { ___opcode_identity32!($x) }; }
macro_rules! __opcode_to_mem_thumb16 { ($x:expr) => { ___opcode_identity16!($x) }; }
macro_rules! __opcode_to_mem_thumb32 { ($x:expr) => { ___opcode_swahw32!($x) }; }
macro_rules! ___asm_opcode_to_mem_arm { ($x:expr) => { ___asm_opcode_identity32!($x) }; }
macro_rules! ___asm_opcode_to_mem_thumb16 { ($x:expr) => { ___asm_opcode_identity16!($x) }; }
macro_rules! ___asm_opcode_to_mem_thumb32 { ($x:expr) => { ___asm_opcode_swahw32!($x) }; }

macro_rules! __mem_to_opcode_arm { ($x:expr) => { __opcode_to_mem_arm!($x) }; }
macro_rules! __mem_to_opcode_thumb16 { ($x:expr) => { __opcode_to_mem_thumb16!($x) }; }
macro_rules! __mem_to_opcode_thumb32 { ($x:expr) => { __opcode_to_mem_thumb32!($x) }; }

macro_rules! __opcode_is_thumb32 {
    ($x:expr) => { (($x & 0xF8000000) == 0xE8000000) || (($x & 0xF0000000) == 0xF0000000) };
}
macro_rules! __opcode_is_thumb16 {
    ($x:expr) => { (($x & 0xFFFF0000) == 0) && !(($x & 0xF800) == 0xE800 || ($x & 0xF000) == 0xF000) };
}
macro_rules! __opcode_thumb32_first { ($x:expr) => { ___opcode_identity16!(($x) >> 16) }; }
macro_rules! __opcode_thumb32_second { ($x:expr) => { ___opcode_identity16!($x) }; }
macro_rules! __opcode_thumb32_compose { ($first:expr, $second:expr) => { (___opcode_identity32!(___opcode_identity16!($first)) << 16) | ___opcode_identity32!(___opcode_identity16!($second)) }; }
macro_rules! ___asm_opcode_thumb32_first { ($x:expr) => { ___asm_opcode_identity16!(($x) >> 16) }; }
macro_rules! ___asm_opcode_thumb32_second { ($x:expr) => { ___asm_opcode_identity16!($x) }; }
macro_rules! ___asm_opcode_thumb32_compose { ($first:expr, $second:expr) => { (___asm_opcode_identity32!(___asm_opcode_identity16!($first)) << 16) | ___asm_opcode_identity32!(___asm_opcode_identity16!($second)) }; }

/* Instruction-emission macros retain the original assembler text interface. */
macro_rules! __inst_arm { ($x:expr) => { concat!(".long ", stringify!($x), "\n\t") }; }
macro_rules! __inst_thumb16 { ($x:expr) => { concat!(".short ", stringify!($x), "\n\t") }; }
macro_rules! __inst_thumb32 { ($first:expr, $second:expr) => { concat!(".short ", stringify!($first), ", ", stringify!($second), "\n\t") }; }
macro_rules! __inst_arm_thumb16 { ($arm:expr, $thumb:expr) => { __inst_arm!($arm) }; }
macro_rules! __inst_arm_thumb32 { ($arm:expr, $thumb:expr) => { __inst_arm!($arm) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
