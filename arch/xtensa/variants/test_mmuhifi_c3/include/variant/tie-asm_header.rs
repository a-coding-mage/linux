/*
 * Rust translation of tie-asm.h.  The source declarations are Xtensa
 * assembler macros; their instruction bodies are retained below as comments
 * because Rust has no portable representation for these target-specific
 * assembler directives.
 */

pub const XTHAL_SAS_TIE: u32 = 0x0001;
pub const XTHAL_SAS_OPT: u32 = 0x0002;
pub const XTHAL_SAS_NOCC: u32 = 0x0004;
pub const XTHAL_SAS_CC: u32 = 0x0008;
pub const XTHAL_SAS_CALR: u32 = 0x0010;
pub const XTHAL_SAS_CALE: u32 = 0x0020;
pub const XTHAL_SAS_GLOB: u32 = 0x0040;
pub const XTHAL_SAS_ALL: u32 = 0xFFFF;

pub const XCHAL_NCP_NUM_ATMPS: u32 = 1;
pub const XCHAL_CP1_NUM_ATMPS: u32 = 1;
pub const XCHAL_SA_NUM_ATMPS: u32 = 1;

/*
 * Assembly macro declarations.  These preserve the original interfaces and
 * instruction sequences for a target-specific assembler integration.
 */
macro_rules! xchal_ncp_store {
    ($ptr:tt $at1:tt $at2:tt $at3:tt $at4:tt $( $rest:tt )*) => { /* xchal_sa_start; conditional BR, SCOMPARE1, and THREADPTR saves */ };
}
macro_rules! xchal_ncp_load {
    ($ptr:tt $at1:tt $at2:tt $at3:tt $at4:tt $( $rest:tt )*) => { /* xchal_sa_start; conditional BR, SCOMPARE1, and THREADPTR restores */ };
}

/* AudioEngineLX is coprocessor 1. */
macro_rules! xchal_cp_AudioEngineLX_store {
    ($($args:tt)*) => { xchal_cp1_store!($($args)*) };
}
macro_rules! xchal_cp_AudioEngineLX_load {
    ($($args:tt)*) => { xchal_cp1_load!($($args)*) };
}

macro_rules! xchal_cp1_store {
    ($ptr:tt $at1:tt $at2:tt $at3:tt $at4:tt $( $rest:tt )*) => {
        /*
         * xchal_sa_start/xchal_sa_align; rur240..rur243 and s32i; AE_SP24X2S.I
         * for aep0..aep7; AE_SQ56S.I for aeq0..aeq3; offsets +64/+112.
         */
    };
}
macro_rules! xchal_cp1_load {
    ($ptr:tt $at1:tt $at2:tt $at3:tt $at4:tt $( $rest:tt )*) => {
        /*
         * xchal_sa_start/xchal_sa_align; l32i and wur240..wur243;
         * AE_LQ56.I for aeq0..aeq3; AE_LP24X2.I for aep0..aep7;
         * offsets +80/+112.
         */
    };
}

/* Empty macros for unconfigured coprocessors, as in the source header. */
macro_rules! xchal_cp0_store { ($($args:tt)*) => {}; }
macro_rules! xchal_cp0_load  { ($($args:tt)*) => {}; }
macro_rules! xchal_cp2_store { ($($args:tt)*) => {}; }
macro_rules! xchal_cp2_load  { ($($args:tt)*) => {}; }
macro_rules! xchal_cp3_store { ($($args:tt)*) => {}; }
macro_rules! xchal_cp3_load  { ($($args:tt)*) => {}; }
macro_rules! xchal_cp4_store { ($($args:tt)*) => {}; }
macro_rules! xchal_cp4_load  { ($($args:tt)*) => {}; }
macro_rules! xchal_cp5_store { ($($args:tt)*) => {}; }
macro_rules! xchal_cp5_load  { ($($args:tt)*) => {}; }
macro_rules! xchal_cp6_store { ($($args:tt)*) => {}; }
macro_rules! xchal_cp6_load  { ($($args:tt)*) => {}; }
macro_rules! xchal_cp7_store { ($($args:tt)*) => {}; }
macro_rules! xchal_cp7_load  { ($($args:tt)*) => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
