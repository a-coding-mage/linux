/*
 * This header file contains assembly-language definitions for this specific
 * Xtensa processor's TIE extensions and options.
 *
 * Translated from tie-asm.h.  The xchal_ncp_store and xchal_ncp_load items
 * are assembler macros; their source bodies are retained below because they
 * have no direct Rust-language equivalent.
 */

/* Selection parameter values for save-area save/restore macros. */
pub const XTHAL_SAS_TIE: u32 = 0x0001; // custom extension or coprocessor
pub const XTHAL_SAS_OPT: u32 = 0x0002; // optional (and not a coprocessor)
pub const XTHAL_SAS_NOCC: u32 = 0x0004; // not used by compiler without special opts/code
pub const XTHAL_SAS_CC: u32 = 0x0008; // used by compiler without special opts/code
pub const XTHAL_SAS_CALR: u32 = 0x0010; // caller-saved
pub const XTHAL_SAS_CALE: u32 = 0x0020; // callee-saved
pub const XTHAL_SAS_GLOB: u32 = 0x0040; // global across function calls (in thread)
pub const XTHAL_SAS_ALL: u32 = 0xFFFF; // include all default NCP contents

pub const XCHAL_NCP_NUM_ATMPS: u32 = 2;
pub const XCHAL_SA_NUM_ATMPS: u32 = 2;

/*
 * Original assembler macro:
 *
 * .macro xchal_ncp_store ptr at1 at2 at3 at4 continue=0 ofs=-1 select=XTHAL_SAS_ALL
 *   xchal_sa_start \continue, \ofs
 *   .ifeq (XTHAL_SAS_OPT | XTHAL_SAS_CC | XTHAL_SAS_CALR) & ~\select
 *   xchal_sa_align \ptr, 0, 1024-8, 4, 4
 *   rsr \at1, ACCLO; rsr \at2, ACCHI
 *   s32i \at1, \ptr, .Lxchal_ofs_ + 0; s32i \at2, \ptr, .Lxchal_ofs_ + 4
 *   .set .Lxchal_ofs_, .Lxchal_ofs_ + 8
 *   .endif
 *   .ifeq (XTHAL_SAS_OPT | XTHAL_SAS_NOCC | XTHAL_SAS_CALR) & ~\select
 *   xchal_sa_align \ptr, 0, 1024-16, 4, 4
 *   rsr \at1, M0; rsr \at2, M1; s32i \at1, \ptr, .Lxchal_ofs_ + 0; s32i \at2, \ptr, .Lxchal_ofs_ + 4
 *   rsr \at1, M2; rsr \at2, M3; s32i \at1, \ptr, .Lxchal_ofs_ + 8; s32i \at2, \ptr, .Lxchal_ofs_ + 12
 *   .set .Lxchal_ofs_, .Lxchal_ofs_ + 16
 *   .endif
 *   .ifeq (XTHAL_SAS_OPT | XTHAL_SAS_NOCC | XTHAL_SAS_CALR) & ~\select
 *   xchal_sa_align \ptr, 0, 1024-4, 4, 4; rsr \at1, SCOMPARE1
 *   s32i \at1, \ptr, .Lxchal_ofs_ + 0; .set .Lxchal_ofs_, .Lxchal_ofs_ + 4
 *   .endif
 *   .ifeq (XTHAL_SAS_OPT | XTHAL_SAS_CC | XTHAL_SAS_GLOB) & ~\select
 *   xchal_sa_align \ptr, 0, 1024-4, 4, 4; rur \at1, THREADPTR
 *   s32i \at1, \ptr, .Lxchal_ofs_ + 0; .set .Lxchal_ofs_, .Lxchal_ofs_ + 4
 *   .endif
 * .endm
 *
 * xchal_ncp_load is the exact inverse assembler macro: it performs the same
 * four conditional save-area sections using l32i followed by wsr/wur to
 * restore ACCLO/ACCHI, M0..M3, SCOMPARE1, and THREADPTR, with offsets and
 * alignment identical to xchal_ncp_store.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
