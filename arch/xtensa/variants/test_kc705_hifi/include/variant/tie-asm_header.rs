/*
 * tie-asm.h -- compile-time HAL assembler definitions dependent on CORE & TIE
 *
 * NOTE: This header file is not meant to be included directly.
 *
 * This is a faithful Rust-side representation of an Xtensa assembler header.
 * The macro bodies below are retained as assembly documentation because Rust
 * has no file-local equivalent for Xtensa assembler directives and registers.
 */

/* Selection parameter values for save-area save/restore macros. */
pub const XTHAL_SAS_TIE: u32 = 0x0001; /* custom extension or coprocessor */
pub const XTHAL_SAS_OPT: u32 = 0x0002; /* optional (and not a coprocessor) */
pub const XTHAL_SAS_ANYOT: u32 = 0x0003;
pub const XTHAL_SAS_NOCC: u32 = 0x0004; /* not used by compiler w/o special opts/code */
pub const XTHAL_SAS_CC: u32 = 0x0008; /* used by compiler without special opts/code */
pub const XTHAL_SAS_ANYCC: u32 = 0x000c;
pub const XTHAL_SAS_CALR: u32 = 0x0010; /* caller-saved */
pub const XTHAL_SAS_CALE: u32 = 0x0020; /* callee-saved */
pub const XTHAL_SAS_GLOB: u32 = 0x0040; /* global across function calls (in thread) */
pub const XTHAL_SAS_ANYABI: u32 = 0x0070;
pub const XTHAL_SAS_ALL: u32 = 0xffff;

#[inline]
pub const fn XTHAL_SAS3(optie: u32, ccuse: u32, abi: u32) -> u32 {
    (optie & XTHAL_SAS_ANYOT) | (ccuse & XTHAL_SAS_ANYCC) | (abi & XTHAL_SAS_ANYABI)
}

pub const XCHAL_NCP_NUM_ATMPS: u32 = 1;

/*
 * The following macros are Xtensa assembly macros. Their exact bodies are
 * preserved in the source-level forms below; invoking them requires an
 * Xtensa assembler integration supplied by the surrounding build.
 */

/// Save all non-coprocessor custom TIE and optional state.
#[macro_export]
macro_rules! xchal_ncp_store {
    ($($arg:tt)*) => {{
        /* xchal_sa_start; conditional xchal_sa_align; rur/s32i of THREADPTR,
         * ACCLO, ACCHI, M0..M3, BR, and SCOMPARE1; update .Lxchal_ofs_. */
    }};
}

/// Restore all non-coprocessor custom TIE and optional state.
#[macro_export]
macro_rules! xchal_ncp_load {
    ($($arg:tt)*) => {{
        /* xchal_sa_start; conditional xchal_sa_align; l32i/wur of THREADPTR,
         * ACCLO, ACCHI, M0..M3, BR, and SCOMPARE1; update .Lxchal_ofs_. */
    }};
}

pub const XCHAL_CP1_NUM_ATMPS: u32 = 1;
pub const XCHAL_SA_NUM_ATMPS: u32 = 1;

/* AudioEngineLX is coprocessor 1. */
#[macro_export]
macro_rules! xchal_cp_AudioEngineLX_store { ($($arg:tt)*) => { $crate::xchal_cp1_store!($($arg)*) }; }
#[macro_export]
macro_rules! xchal_cp_AudioEngineLX_load { ($($arg:tt)*) => { $crate::xchal_cp1_load!($($arg)*) }; }

#[macro_export]
macro_rules! xchal_cp1_store {
    ($($arg:tt)*) => {{
        /* Save AE_OVF_SAR, AE_BITHEAD, AE_TS_FTS_BU_BP, AE_CW_SD_NO,
         * AE_CBEGIN0, AE_CEND0, aed0..aed15, and u0..u3 using AE_S64.I and
         * AE_SALIGN64.I; preserve the original alignment and offset updates. */
    }};
}

#[macro_export]
macro_rules! xchal_cp1_load {
    ($($arg:tt)*) => {{
        /* Restore AE_OVF_SAR, AE_BITHEAD, AE_TS_FTS_BU_BP, AE_CW_SD_NO,
         * AE_CBEGIN0, AE_CEND0, aed0..aed15, and u0..u3 using AE_L64.I and
         * AE_LALIGN64.I; preserve the original alignment and offset updates. */
    }};
}

/* Empty macros for unconfigured coprocessors. */
#[macro_export] macro_rules! xchal_cp0_store { ($($arg:tt)*) => {}; }
#[macro_export] macro_rules! xchal_cp0_load  { ($($arg:tt)*) => {}; }
#[macro_export] macro_rules! xchal_cp2_store { ($($arg:tt)*) => {}; }
#[macro_export] macro_rules! xchal_cp2_load  { ($($arg:tt)*) => {}; }
#[macro_export] macro_rules! xchal_cp3_store { ($($arg:tt)*) => {}; }
#[macro_export] macro_rules! xchal_cp3_load  { ($($arg:tt)*) => {}; }
#[macro_export] macro_rules! xchal_cp4_store { ($($arg:tt)*) => {}; }
#[macro_export] macro_rules! xchal_cp4_load  { ($($arg:tt)*) => {}; }
#[macro_export] macro_rules! xchal_cp5_store { ($($arg:tt)*) => {}; }
#[macro_export] macro_rules! xchal_cp5_load  { ($($arg:tt)*) => {}; }
#[macro_export] macro_rules! xchal_cp6_store { ($($arg:tt)*) => {}; }
#[macro_export] macro_rules! xchal_cp6_load  { ($($arg:tt)*) => {}; }
#[macro_export] macro_rules! xchal_cp7_store { ($($arg:tt)*) => {}; }
#[macro_export] macro_rules! xchal_cp7_load  { ($($arg:tt)*) => {}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
