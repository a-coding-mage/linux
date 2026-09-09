/*
 * Rust translation of asm/mipsregs.h.
 *
 * The original header is primarily a preprocessor interface for MIPS
 * assembler.  Register tokens which contain `$` and assembler statements are
 * retained as string-producing macros; numeric fields are represented as
 * constants and preserve the original integer expressions.
 * Build-time CONFIG_* selections from the C header remain configuration
 * responsibilities of the consuming target.
 */

#![allow(non_upper_case_globals, non_snake_case, dead_code)]

pub const CP0_INDEX: &str = "$0";
pub const CP0_RANDOM: &str = "$1";
pub const CP0_ENTRYLO0: &str = "$2";
pub const CP0_ENTRYLO1: &str = "$3";
pub const CP0_CONTEXT: &str = "$4";
pub const CP0_PAGEMASK: &str = "$5";
pub const CP0_WIRED: &str = "$6";
pub const CP0_INFO: &str = "$7";
pub const CP0_BADVADDR: &str = "$8";
pub const CP0_COUNT: &str = "$9";
pub const CP0_ENTRYHI: &str = "$10";
pub const CP0_COMPARE: &str = "$11";
pub const CP0_STATUS: &str = "$12";
pub const CP0_CAUSE: &str = "$13";
pub const CP0_EPC: &str = "$14";
pub const CP0_PRID: &str = "$15";
pub const CP0_CONFIG: &str = "$16";
pub const CP0_LLADDR: &str = "$17";
pub const CP0_WATCHLO: &str = "$18";
pub const CP0_WATCHHI: &str = "$19";
pub const CP0_XCONTEXT: &str = "$20";
pub const CP0_FRAMEMASK: &str = "$21";
pub const CP0_DIAGNOSTIC: &str = "$22";
pub const CP0_DEBUG: &str = "$23";
pub const CP0_DEPC: &str = "$24";
pub const CP0_PERFORMANCE: &str = "$25";
pub const CP0_ECC: &str = "$26";
pub const CP0_CACHEERR: &str = "$27";
pub const CP0_TAGLO: &str = "$28";
pub const CP0_TAGHI: &str = "$29";
pub const CP0_ERROREPC: &str = "$30";
pub const CP0_DESAVE: &str = "$31";

pub const ENTRYLO_G: usize = 1usize << 0;
pub const ENTRYLO_V: usize = 1usize << 1;
pub const ENTRYLO_D: usize = 1usize << 2;
pub const ENTRYLO_C_SHIFT: usize = 3;
pub const ENTRYLO_C: usize = 7usize << ENTRYLO_C_SHIFT;
pub const R3K_ENTRYLO_G: usize = 1usize << 8;
pub const R3K_ENTRYLO_V: usize = 1usize << 9;
pub const R3K_ENTRYLO_D: usize = 1usize << 10;
pub const R3K_ENTRYLO_N: usize = 1usize << 11;
pub const MIPS_ENTRYLO_PFN_SHIFT: usize = 6;

pub const PM_4K: u32 = 0x00000000;
pub const PM_8K: u32 = 0x00002000;
pub const PM_16K: u32 = 0x00006000;
pub const PM_32K: u32 = 0x0000e000;
pub const PM_64K: u32 = 0x0001e000;
pub const PM_128K: u32 = 0x0003e000;
pub const PM_256K: u32 = 0x0007e000;
pub const PM_512K: u32 = 0x000fe000;
pub const PM_1M: u32 = 0x001fe000;
pub const PM_2M: u32 = 0x003fe000;
pub const PM_4M: u32 = 0x007fe000;
pub const PM_8M: u32 = 0x00ffe000;
pub const PM_16M: u32 = 0x01ffe000;
pub const PM_32M: u32 = 0x03ffe000;
pub const PM_64M: u32 = 0x07ffe000;
pub const PM_256M: u32 = 0x1fffe000;
pub const PM_1G: u32 = 0x7fffe000;

pub const ST0_IE: u32 = 0x00000001;
pub const ST0_EXL: u32 = 0x00000002;
pub const ST0_ERL: u32 = 0x00000004;
pub const ST0_KSU: u32 = 0x00000018;
pub const KSU_USER: u32 = 0x00000010;
pub const KSU_SUPERVISOR: u32 = 0x00000008;
pub const KSU_KERNEL: u32 = 0;
pub const ST0_IM: u32 = 0x0000ff00;
pub const ST0_CU: u32 = 0xf0000000;
pub const ST0_CU0: u32 = 0x10000000;
pub const ST0_CU1: u32 = 0x20000000;
pub const ST0_CU2: u32 = 0x40000000;
pub const ST0_CU3: u32 = 0x80000000;

pub const CAUSEB_EXCCODE: u32 = 2;
pub const CAUSEF_EXCCODE: u32 = 31 << CAUSEB_EXCCODE;
pub const CAUSEB_IP: u32 = 8;
pub const CAUSEF_IP: u32 = 255 << CAUSEB_IP;
pub const EXCCODE_INT: u32 = 0;
pub const EXCCODE_MOD: u32 = 1;
pub const EXCCODE_TLBL: u32 = 2;
pub const EXCCODE_TLBS: u32 = 3;
pub const EXCCODE_ADEL: u32 = 4;
pub const EXCCODE_ADES: u32 = 5;
pub const EXCCODE_SYS: u32 = 8;
pub const EXCCODE_BP: u32 = 9;
pub const EXCCODE_RI: u32 = 10;
pub const EXCCODE_CPU: u32 = 11;
pub const EXCCODE_OV: u32 = 12;
pub const EXCCODE_TR: u32 = 13;
pub const EXCCODE_FPE: u32 = 15;
pub const EXCCODE_CACHEERR: u32 = 30;

pub const FPU_CSR_RM: u32 = 0x00000003;
pub const FPU_CSR_RN: u32 = 0;
pub const FPU_CSR_RZ: u32 = 1;
pub const FPU_CSR_RU: u32 = 2;
pub const FPU_CSR_RD: u32 = 3;

#[inline]
pub const fn mm_insn_16bit(insn: u16) -> i32 {
    let opcode = (insn >> 10) & 0x7;
    if opcode >= 1 && opcode <= 3 { 1 } else { 0 }
}

/* C inline assembler accessors are intentionally exposed as declarative
 * placeholders: their bodies depend on the target assembler and kernel
 * linkage definitions supplied by the consuming build. */
#[macro_export]
macro_rules! read_c0_index { () => { unsafe { core::arch::asm!("mfc0 {0}, $0", out(reg) _) } }; }


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
