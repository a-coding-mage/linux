/*
 * Rust translation of tie-asm.h.  The source is an Xtensa assembler header;
 * its assembler macros are retained as Rust macro interfaces with their
 * target-specific instruction sequences documented below.
 */

pub const XTHAL_SAS_TIE: u32 = 0x0001;
pub const XTHAL_SAS_OPT: u32 = 0x0002;
pub const XTHAL_SAS_ANYOT: u32 = 0x0003;
pub const XTHAL_SAS_NOCC: u32 = 0x0004;
pub const XTHAL_SAS_CC: u32 = 0x0008;
pub const XTHAL_SAS_ANYCC: u32 = 0x000c;
pub const XTHAL_SAS_CALR: u32 = 0x0010;
pub const XTHAL_SAS_CALE: u32 = 0x0020;
pub const XTHAL_SAS_GLOB: u32 = 0x0040;
pub const XTHAL_SAS_ANYABI: u32 = 0x0070;
pub const XTHAL_SAS_ALL: u32 = 0xffff;

#[inline]
pub const fn xthal_sas3(optie: u32, ccuse: u32, abi: u32) -> u32 {
    (optie & XTHAL_SAS_ANYOT) | (ccuse & XTHAL_SAS_ANYCC) | (abi & XTHAL_SAS_ANYABI)
}

pub const XCHAL_NCP_NUM_ATMPS: u32 = 1;
pub const XCHAL_CP1_NUM_ATMPS: u32 = 1;
pub const XCHAL_SA_NUM_ATMPS: u32 = 1;

// These macros are Xtensa assembler definitions.  Rust has no portable
// representation for their register operands; preserve their call surfaces
// and target-specific instruction intent for an Xtensa backend.
macro_rules! xchal_ncp_store {
    ($($arg:tt)*) => {{ /* xchal_sa_start; rur/s32i THREADPTR, ACCLO, ACCHI, BR, SCOMPARE1, M0..M3; xchal_sa_align */ }};
}
macro_rules! xchal_ncp_load {
    ($($arg:tt)*) => {{ /* xchal_sa_start; l32i/wur THREADPTR, ACCLO, ACCHI, BR, SCOMPARE1, M0..M3; xchal_sa_align */ }};
}

macro_rules! xchal_cp1_store {
    ($($arg:tt)*) => {{ /* xchal_sa_start; store AE_OVF_SAR, AE_BITHEAD, AE_TS_FTS_BU_BP, AE_SD_NO, AE_CBEGIN0, AE_CEND0, aep0..aep7, aeq0..aeq3 */ }};
}
macro_rules! xchal_cp1_load {
    ($($arg:tt)*) => {{ /* xchal_sa_start; load AE_OVF_SAR, AE_BITHEAD, AE_TS_FTS_BU_BP, AE_SD_NO, AE_CBEGIN0, AE_CEND0, aep0..aep7, aeq0..aeq3 */ }};
}

macro_rules! xchal_cp_AudioEngineLX_store { ($($arg:tt)*) => { xchal_cp1_store!($($arg)*) }; }
macro_rules! xchal_cp_AudioEngineLX_load { ($($arg:tt)*) => { xchal_cp1_load!($($arg)*) }; }

macro_rules! xchal_cp0_store { ($($arg:tt)*) => {}; }
macro_rules! xchal_cp0_load  { ($($arg:tt)*) => {}; }
macro_rules! xchal_cp2_store { ($($arg:tt)*) => {}; }
macro_rules! xchal_cp2_load  { ($($arg:tt)*) => {}; }
macro_rules! xchal_cp3_store { ($($arg:tt)*) => {}; }
macro_rules! xchal_cp3_load  { ($($arg:tt)*) => {}; }
macro_rules! xchal_cp4_store { ($($arg:tt)*) => {}; }
macro_rules! xchal_cp4_load  { ($($arg:tt)*) => {}; }
macro_rules! xchal_cp5_store { ($($arg:tt)*) => {}; }
macro_rules! xchal_cp5_load  { ($($arg:tt)*) => {}; }
macro_rules! xchal_cp6_store { ($($arg:tt)*) => {}; }
macro_rules! xchal_cp6_load  { ($($arg:tt)*) => {}; }
macro_rules! xchal_cp7_store { ($($arg:tt)*) => {}; }
macro_rules! xchal_cp7_load  { ($($arg:tt)*) => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
