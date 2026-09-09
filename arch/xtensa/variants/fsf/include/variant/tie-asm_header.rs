/*
 * This header file contains assembly-language definitions (assembly
 * macros, etc.) for this specific Xtensa processor's TIE extensions
 * and options.  It is customized to this Xtensa processor configuration.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1999-2008 Tensilica Inc.
 */

/* Selection parameter values for save-area save/restore macros. */
pub const XTHAL_SAS_TIE: u32 = 0x0001; // custom extension or coprocessor
pub const XTHAL_SAS_OPT: u32 = 0x0002; // optional (and not a coprocessor)
pub const XTHAL_SAS_NOCC: u32 = 0x0004; // not used by compiler w/o special opts/code
pub const XTHAL_SAS_CC: u32 = 0x0008; // used by compiler without special opts/code
pub const XTHAL_SAS_CALR: u32 = 0x0010; // caller-saved
pub const XTHAL_SAS_CALE: u32 = 0x0020; // callee-saved
pub const XTHAL_SAS_GLOB: u32 = 0x0040; // global across function calls (in thread)
pub const XTHAL_SAS_ALL: u32 = 0xFFFF; // include all default NCP contents

pub const XCHAL_NCP_NUM_ATMPS: u32 = 1;
pub const XCHAL_SA_NUM_ATMPS: u32 = 1;

/*
 * These are GNU Xtensa assembler macros.  Rust has no direct equivalent for
 * assembler directives such as .macro, .ifeq, .set, or xchal_sa_start;
 * preserve their exact source-level bodies here for use by the target
 * assembler integration.
 *
 * .macro xchal_ncp_store ptr at1 at2 at3 at4 continue=0 ofs=-1 select=XTHAL_SAS_ALL
 *   xchal_sa_start  \\continue, \\ofs
 *   .ifeq (XTHAL_SAS_OPT | XTHAL_SAS_CC | XTHAL_SAS_GLOB) & ~\\select
 *   xchal_sa_align  \\ptr, 0, 1024-4, 4, 4
 *   rur  \\at1, THREADPTR
 *   s32i \\at1, \\ptr, .Lxchal_ofs_ + 0
 *   .set .Lxchal_ofs_, .Lxchal_ofs_ + 4
 *   .endif
 * .endm
 */

/*
 * .macro xchal_ncp_load ptr at1 at2 at3 at4 continue=0 ofs=-1 select=XTHAL_SAS_ALL
 *   xchal_sa_start  \\continue, \\ofs
 *   .ifeq (XTHAL_SAS_OPT | XTHAL_SAS_CC | XTHAL_SAS_GLOB) & ~\\select
 *   xchal_sa_align  \\ptr, 0, 1024-4, 4, 4
 *   l32i \\at1, \\ptr, .Lxchal_ofs_ + 0
 *   wur  \\at1, THREADPTR
 *   .set .Lxchal_ofs_, .Lxchal_ofs_ + 4
 *   .endif
 * .endm
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
