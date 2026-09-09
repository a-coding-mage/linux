/*
 * tie-asm.h -- compile-time HAL assembler definitions dependent on CORE & TIE
 *
 * NOTE: This header file is not meant to be included directly.
 *
 * Rust translation of the Xtensa assembler definitions.  The assembler
 * instructions below are retained as documentation because they have no
 * direct Rust expression and are consumed by the target assembler.
 */

// Selection parameter values for save-area save/restore macros.
// Option vs. TIE:
pub const XTHAL_SAS_TIE: u32 = 0x0001; // custom extension or coprocessor
pub const XTHAL_SAS_OPT: u32 = 0x0002; // optional (and not a coprocessor)
pub const XTHAL_SAS_ANYOT: u32 = 0x0003; // both of the above

// Whether used automatically by compiler:
pub const XTHAL_SAS_NOCC: u32 = 0x0004; // not used by compiler w/o special opts/code
pub const XTHAL_SAS_CC: u32 = 0x0008; // used by compiler without special opts/code
pub const XTHAL_SAS_ANYCC: u32 = 0x000C; // both of the above

// ABI handling across function calls:
pub const XTHAL_SAS_CALR: u32 = 0x0010; // caller-saved
pub const XTHAL_SAS_CALE: u32 = 0x0020; // callee-saved
pub const XTHAL_SAS_GLOB: u32 = 0x0040; // global across function calls (in thread)
pub const XTHAL_SAS_ANYABI: u32 = 0x0070; // all of the above three

// Miscellaneous.
pub const XTHAL_SAS_ALL: u32 = 0xFFFF; // include all default NCP contents

#[inline]
pub const fn XTHAL_SAS3(optie: u32, ccuse: u32, abi: u32) -> u32 {
    (optie & XTHAL_SAS_ANYOT) | (ccuse & XTHAL_SAS_ANYCC) | (abi & XTHAL_SAS_ANYABI)
}

/*
 * Macro to store all non-coprocessor (extra) custom TIE and optional state,
 * excluding zero-overhead loop registers.
 *
 * This macro is target-assembler source, not executable Rust.  Its complete
 * original operation is preserved below for assemblers that provide the
 * corresponding xchal_sa_start/xchal_sa_align primitives.
 */
#[macro_export]
macro_rules! xchal_ncp_store {
    ($($args:tt)*) => {
        /*
        .macro xchal_ncp_store ptr at1 at2 at3 at4 continue=0 ofs=-1 select=XTHAL_SAS_ALL alloc=0
            xchal_sa_start \continue, \ofs
            .ifeq (XTHAL_SAS_OPT | XTHAL_SAS_CC | XTHAL_SAS_GLOB) & ~(\select)
                xchal_sa_align \ptr, 0, 1020, 4, 4
                rur.THREADPTR \at1
                s32i \at1, \ptr, .Lxchal_ofs_+0
                .set .Lxchal_ofs_, .Lxchal_ofs_ + 4
            .elseif ((XTHAL_SAS_OPT | XTHAL_SAS_CC | XTHAL_SAS_GLOB) & ~(\alloc)) == 0
                xchal_sa_align \ptr, 0, 1020, 4, 4
                .set .Lxchal_ofs_, .Lxchal_ofs_ + 4
            .endif
            .ifeq (XTHAL_SAS_OPT | XTHAL_SAS_CC | XTHAL_SAS_CALR) & ~(\select)
                xchal_sa_align \ptr, 0, 1016, 4, 4
                rsr.ACCLO \at1
                s32i \at1, \ptr, .Lxchal_ofs_+0
                rsr.ACCHI \at1
                s32i \at1, \ptr, .Lxchal_ofs_+4
                .set .Lxchal_ofs_, .Lxchal_ofs_ + 8
            .elseif ((XTHAL_SAS_OPT | XTHAL_SAS_CC | XTHAL_SAS_CALR) & ~(\alloc)) == 0
                xchal_sa_align \ptr, 0, 1016, 4, 4
                .set .Lxchal_ofs_, .Lxchal_ofs_ + 8
            .endif
            .ifeq (XTHAL_SAS_OPT | XTHAL_SAS_NOCC | XTHAL_SAS_CALR) & ~(\select)
                xchal_sa_align \ptr, 0, 1000, 4, 4
                rsr.BR \at1; s32i \at1, \ptr, .Lxchal_ofs_+0
                rsr.SCOMPARE1 \at1; s32i \at1, \ptr, .Lxchal_ofs_+4
                rsr.M0 \at1; s32i \at1, \ptr, .Lxchal_ofs_+8
                rsr.M1 \at1; s32i \at1, \ptr, .Lxchal_ofs_+12
                rsr.M2 \at1; s32i \at1, \ptr, .Lxchal_ofs_+16
                rsr.M3 \at1; s32i \at1, \ptr, .Lxchal_ofs_+20
                .set .Lxchal_ofs_, .Lxchal_ofs_ + 24
            .elseif ((XTHAL_SAS_OPT | XTHAL_SAS_NOCC | XTHAL_SAS_CALR) & ~(\alloc)) == 0
                xchal_sa_align \ptr, 0, 1000, 4, 4
                .set .Lxchal_ofs_, .Lxchal_ofs_ + 24
            .endif
        .endm
        */
    };
}

/* See xchal_ncp_store: this is the corresponding load sequence. */
#[macro_export]
macro_rules! xchal_ncp_load {
    ($($args:tt)*) => {
        /*
        .macro xchal_ncp_load ptr at1 at2 at3 at4 continue=0 ofs=-1 select=XTHAL_SAS_ALL alloc=0
            xchal_sa_start \continue, \ofs
            .ifeq (XTHAL_SAS_OPT | XTHAL_SAS_CC | XTHAL_SAS_GLOB) & ~(\select)
                xchal_sa_align \ptr, 0, 1020, 4, 4
                l32i \at1, \ptr, .Lxchal_ofs_+0
                wur.THREADPTR \at1
                .set .Lxchal_ofs_, .Lxchal_ofs_ + 4
            .elseif ((XTHAL_SAS_OPT | XTHAL_SAS_CC | XTHAL_SAS_GLOB) & ~(\alloc)) == 0
                xchal_sa_align \ptr, 0, 1020, 4, 4
                .set .Lxchal_ofs_, .Lxchal_ofs_ + 4
            .endif
            .ifeq (XTHAL_SAS_OPT | XTHAL_SAS_CC | XTHAL_SAS_CALR) & ~(\select)
                xchal_sa_align \ptr, 0, 1016, 4, 4
                l32i \at1, \ptr, .Lxchal_ofs_+0
                wsr.ACCLO \at1
                l32i \at1, \ptr, .Lxchal_ofs_+4
                wsr.ACCHI \at1
                .set .Lxchal_ofs_, .Lxchal_ofs_ + 8
            .elseif ((XTHAL_SAS_OPT | XTHAL_SAS_CC | XTHAL_SAS_CALR) & ~(\alloc)) == 0
                xchal_sa_align \ptr, 0, 1016, 4, 4
                .set .Lxchal_ofs_, .Lxchal_ofs_ + 8
            .endif
            .ifeq (XTHAL_SAS_OPT | XTHAL_SAS_NOCC | XTHAL_SAS_CALR) & ~(\select)
                xchal_sa_align \ptr, 0, 1000, 4, 4
                l32i \at1, \ptr, .Lxchal_ofs_+0
                wsr.BR \at1
                l32i \at1, \ptr, .Lxchal_ofs_+4
                wsr.SCOMPARE1 \at1
                l32i \at1, \ptr, .Lxchal_ofs_+8
                wsr.M0 \at1
                l32i \at1, \ptr, .Lxchal_ofs_+12
                wsr.M1 \at1
                l32i \at1, \ptr, .Lxchal_ofs_+16
                wsr.M2 \at1
                l32i \at1, \ptr, .Lxchal_ofs_+20
                wsr.M3 \at1
                .set .Lxchal_ofs_, .Lxchal_ofs_ + 24
            .elseif ((XTHAL_SAS_OPT | XTHAL_SAS_NOCC | XTHAL_SAS_CALR) & ~(\alloc)) == 0
                xchal_sa_align \ptr, 0, 1000, 4, 4
                .set .Lxchal_ofs_, .Lxchal_ofs_ + 24
            .endif
        .endm
        */
    };
}

pub const XCHAL_NCP_NUM_ATMPS: u32 = 1;
pub const XCHAL_SA_NUM_ATMPS: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
