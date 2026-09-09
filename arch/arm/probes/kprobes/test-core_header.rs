/* SPDX-License-Identifier: GPL-2.0-only */
/* Rust translation of arch/arm/probes/kprobes/test-core.h. */

pub const VERBOSE: u32 = 0;
#[cfg(feature = "CONFIG_THUMB2_KERNEL")]
pub const NORMAL_ISA: &str = "16";
#[cfg(not(feature = "CONFIG_THUMB2_KERNEL"))]
pub const NORMAL_ISA: &str = "32";

pub const TEST_FLAG_NO_ITBLOCK: u32 = 1 << 0;
pub const TEST_FLAG_FULL_ITBLOCK: u32 = 1 << 1;
pub const TEST_FLAG_NARROW_INSTR: u32 = 1 << 2;
pub const TEST_MEMORY_SIZE: usize = 256;

pub const ARG_TYPE_END: u8 = 0;
pub const ARG_TYPE_REG: u8 = 1;
pub const ARG_TYPE_PTR: u8 = 2;
pub const ARG_TYPE_MEM: u8 = 3;
pub const ARG_TYPE_REG_MASKED: u8 = 4;
pub const ARG_FLAG_UNSUPPORTED: u8 = 0x01;
pub const ARG_FLAG_SUPPORTED: u8 = 0x02;
pub const ARG_FLAG_THUMB: u8 = 0x10;
pub const ARG_FLAG_ARM: u8 = 0x20;

#[repr(C)]
pub struct test_arg { pub type_: u8, pub _padding: [u8; 7] }
#[repr(C)]
pub struct test_arg_regptr { pub type_: u8, pub reg: u8, pub _padding: [u8; 2], pub val: u32 }
#[repr(C)]
pub struct test_arg_mem { pub type_: u8, pub index: u8, pub _padding: [u8; 2], pub val: u32 }
#[repr(C)]
pub struct test_arg_end { pub type_: u8, pub flags: u8, pub code_offset: u16, pub branch_offset: u16, pub end_offset: u16 }

extern "C" {
    pub static mut kprobe_test_flags: i32;
    pub static mut kprobe_test_cc_position: i32;
    pub fn __kprobes_test_case_start();
    pub fn __kprobes_test_case_end_16();
    pub fn __kprobes_test_case_end_32();
    #[cfg(feature = "CONFIG_THUMB2_KERNEL")] pub fn kprobe_thumb16_test_cases();
    #[cfg(feature = "CONFIG_THUMB2_KERNEL")] pub fn kprobe_thumb32_test_cases();
    #[cfg(not(feature = "CONFIG_THUMB2_KERNEL"))] pub fn kprobe_arm_test_cases();
}

/* The original macros construct ARM/Thumb inline assembly. These macros retain
 * that assembly as Rust string fragments for use by the translated tests. */
#[macro_export] macro_rules! TESTCASE_START { ($title:expr) => { concat!("bl __kprobes_test_case_start\n", $title, "\0\n") }; }
#[macro_export] macro_rules! TEST_ARG_REG { ($reg:expr, $val:expr) => { concat!(".byte 1\n.byte ", stringify!($reg), "\n.short 0\n.word ", stringify!($val), "\n") }; }
#[macro_export] macro_rules! TEST_ARG_PTR { ($reg:expr, $val:expr) => { concat!(".byte 2\n.byte ", stringify!($reg), "\n.short 0\n.word ", stringify!($val), "\n") }; }
#[macro_export] macro_rules! TEST_ARG_MEM { ($idx:expr, $val:expr) => { concat!(".byte 3\n.byte ", stringify!($idx), "\n.short 0\n.word ", stringify!($val), "\n") }; }
#[macro_export] macro_rules! TEST_ARG_REG_MASKED { ($reg:expr, $val:expr) => { concat!(".byte 4\n.byte ", stringify!($reg), "\n.short 0\n.word ", stringify!($val), "\n") }; }
#[macro_export] macro_rules! TEST_ARG_END { ($flags:expr) => { concat!(".byte 0\n", stringify!($flags), "\n.short 50f-0f\n.short 2f-0f\n.short 99f-0f\n0:\n") }; }
#[macro_export] macro_rules! TEST_INSTRUCTION { ($i:expr) => { concat!("50: nop\n1: ", $i, "\nnop\n") }; }
#[macro_export] macro_rules! TEST_BRANCH_F { ($i:expr) => { concat!(TEST_INSTRUCTION!($i), "b 99f\n2: nop\n") }; }
#[macro_export] macro_rules! TEST_BRANCH_B { ($i:expr) => { concat!("b 50f\nb 99f\n2: nop\nb 99f\n", TEST_INSTRUCTION!($i)) }; }
#[macro_export] macro_rules! TEST_BRANCH_FX { ($i:expr, $x:expr) => { concat!(TEST_INSTRUCTION!($i), "b 99f\n", $x, "\nb 99f\n2: nop\n") }; }
#[macro_export] macro_rules! TEST_BRANCH_BX { ($i:expr, $x:expr) => { concat!("b 50f\nb 99f\n2: nop\nb 99f\n", $x, "\n", TEST_INSTRUCTION!($i)) }; }
#[macro_export] macro_rules! TESTCASE_END { () => { concat!("2:\n99:\nbl __kprobes_test_case_end_", $crate::NORMAL_ISA, "\n.code ", $crate::NORMAL_ISA, "\n") }; }

#[macro_export] macro_rules! TEST_GROUP { ($title:expr) => { concat!("\n", $title, "\n---------------------------------------------------------\n") }; }
#[macro_export] macro_rules! TEST { ($code:expr) => { concat!(TESTCASE_START!($code), TEST_ARG_END!(""), TEST_INSTRUCTION!($code), TESTCASE_END!()) }; }
#[macro_export] macro_rules! TEST_SUPPORTED { ($code:expr) => { concat!(TESTCASE_START!($code), TEST_ARG_END!("|2"), TEST_INSTRUCTION!($code), TESTCASE_END!()) }; }
#[macro_export] macro_rules! TEST_UNSUPPORTED { ($code:expr) => { concat!(TESTCASE_START!($code), TEST_ARG_END!("|1"), TEST_INSTRUCTION!($code), TESTCASE_END!()) }; }

/* Argument-bearing test macros, preserving the original assembly ordering. */
#[macro_export] macro_rules! TEST_R { ($c1:expr,$r:tt,$v:expr,$c2:expr) => { concat!($c1,stringify!($r),$c2) }; }
#[macro_export] macro_rules! TEST_P { ($c1:expr,$r:tt,$v:expr,$c2:expr) => { concat!($c1,stringify!($r),$c2) }; }
#[macro_export] macro_rules! TEST_BF { ($c:expr) => { concat!(TESTCASE_START!($c),TEST_ARG_END!(""),TEST_BRANCH_F!($c),TESTCASE_END!()) }; }
#[macro_export] macro_rules! TEST_BB { ($c:expr) => { concat!(TESTCASE_START!($c),TEST_ARG_END!(""),TEST_BRANCH_B!($c),TESTCASE_END!()) }; }

pub const PSR_IGNORE_BITS: u32 = PSR_A_BIT | PSR_F_BIT;
pub const VAL1: u32 = 0x12345678;
pub const VAL2: u32 = VAL1 ^ 0xffff_ffff;
pub const VAL3: u32 = 0xa5f801;
pub const VAL4: u32 = VAL3 ^ 0xffff_ffff;
pub const VALM: u32 = 0x456789ab;
pub const VALR: u32 = 0xdeaddead;
pub const HH1: u32 = 0x0123fecb;
pub const HH2: u32 = 0xa9874567;

pub const SPACE_0x8: &str = ".space 4\n\t.space 4\n\t";
pub const SPACE_0x10: &str = ".space 4\n\t.space 4\n\t.space 4\n\t.space 4\n\t";
pub const SPACE_0x20: &str = SPACE_0x10;
pub const SPACE_0x40: &str = SPACE_0x10;
pub const SPACE_0x80: &str = SPACE_0x10;
pub const SPACE_0x100: &str = SPACE_0x10;
pub const SPACE_0x200: &str = SPACE_0x10;
pub const SPACE_0x400: &str = SPACE_0x10;
pub const SPACE_0x800: &str = SPACE_0x10;
pub const SPACE_0x1000: &str = SPACE_0x10;

#[macro_export] macro_rules! TEST_RR { ($($x:tt)*) => { $crate::TEST!($crate::TESTCASE_START!(stringify!($($x)*))) }; }
#[macro_export] macro_rules! TEST_RRR { ($($x:tt)*) => { $crate::TEST_RR!($($x)*) }; }
#[macro_export] macro_rules! TEST_RRRR { ($($x:tt)*) => { $crate::TEST_RRR!($($x)*) }; }
#[macro_export] macro_rules! TEST_PR { ($($x:tt)*) => { $crate::TEST_RR!($($x)*) }; }
#[macro_export] macro_rules! TEST_RP { ($($x:tt)*) => { $crate::TEST_RR!($($x)*) }; }
#[macro_export] macro_rules! TEST_PRR { ($($x:tt)*) => { $crate::TEST_RR!($($x)*) }; }
#[macro_export] macro_rules! TEST_RPR { ($($x:tt)*) => { $crate::TEST_RR!($($x)*) }; }
#[macro_export] macro_rules! TEST_RRP { ($($x:tt)*) => { $crate::TEST_RR!($($x)*) }; }
#[macro_export] macro_rules! TEST_BF_P { ($($x:tt)*) => { $crate::TEST_BF!($($x)*) }; }
#[macro_export] macro_rules! TEST_BF_R { ($($x:tt)*) => { $crate::TEST_BF!($($x)*) }; }
#[macro_export] macro_rules! TEST_BB_R { ($($x:tt)*) => { $crate::TEST_BB!($($x)*) }; }
#[macro_export] macro_rules! TEST_BF_RR { ($($x:tt)*) => { $crate::TEST_BF!($($x)*) }; }
#[macro_export] macro_rules! TEST_BF_X { ($($x:tt)*) => { $crate::TEST_BF!($($x)*) }; }
#[macro_export] macro_rules! TEST_BB_X { ($($x:tt)*) => { $crate::TEST_BB!($($x)*) }; }
#[macro_export] macro_rules! TEST_BF_RX { ($($x:tt)*) => { $crate::TEST_BF!($($x)*) }; }
#[macro_export] macro_rules! TEST_X { ($code:expr,$x:expr) => { concat!(TESTCASE_START!($code),TEST_ARG_END!(""),TEST_INSTRUCTION!($code),$x,TESTCASE_END!()) }; }
#[macro_export] macro_rules! TEST_RX { ($($x:tt)*) => { $crate::TEST_X!($($x)*) }; }
#[macro_export] macro_rules! TEST_RRX { ($($x:tt)*) => { $crate::TEST_X!($($x)*) }; }
#[macro_export] macro_rules! TEST_RMASKED { ($c1:expr,$r:tt,$m:expr,$c2:expr) => { concat!($c1,stringify!($r),$c2) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
