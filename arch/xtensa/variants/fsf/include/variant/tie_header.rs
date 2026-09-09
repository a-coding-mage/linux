/*
 * This header file describes this specific Xtensa processor's TIE extensions
 * that extend basic Xtensa core functionality.  It is customized to this
 * Xtensa processor configuration.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1999-2007 Tensilica Inc.
 */

// C header guard: _XTENSA_CORE_TIE_H

pub const XCHAL_CP_NUM: u32 = 0; // number of coprocessors
pub const XCHAL_CP_MAX: u32 = 0; // max CP ID + 1 (0 if none)
pub const XCHAL_CP_MASK: u32 = 0x00; // bitmask of all CPs by ID
pub const XCHAL_CP_PORT_MASK: u32 = 0x00; // bitmask of only port CPs

/*  Filler info for unassigned coprocessors, to simplify arrays etc:  */
pub const XCHAL_NCP_SA_SIZE: u32 = 0;
pub const XCHAL_NCP_SA_ALIGN: u32 = 1;
pub const XCHAL_CP0_SA_SIZE: u32 = 0;
pub const XCHAL_CP0_SA_ALIGN: u32 = 1;
pub const XCHAL_CP1_SA_SIZE: u32 = 0;
pub const XCHAL_CP1_SA_ALIGN: u32 = 1;
pub const XCHAL_CP2_SA_SIZE: u32 = 0;
pub const XCHAL_CP2_SA_ALIGN: u32 = 1;
pub const XCHAL_CP3_SA_SIZE: u32 = 0;
pub const XCHAL_CP3_SA_ALIGN: u32 = 1;
pub const XCHAL_CP4_SA_SIZE: u32 = 0;
pub const XCHAL_CP4_SA_ALIGN: u32 = 1;
pub const XCHAL_CP5_SA_SIZE: u32 = 0;
pub const XCHAL_CP5_SA_ALIGN: u32 = 1;
pub const XCHAL_CP6_SA_SIZE: u32 = 0;
pub const XCHAL_CP6_SA_ALIGN: u32 = 1;
pub const XCHAL_CP7_SA_SIZE: u32 = 0;
pub const XCHAL_CP7_SA_ALIGN: u32 = 1;

/*  Save area for non-coprocessor optional and custom (TIE) state:  */
// XCHAL_NCP_SA_SIZE and XCHAL_NCP_SA_ALIGN are defined above, as in the C header.

/*  Total save area for optional and custom state (NCP + CPn):  */
pub const XCHAL_TOTAL_SA_SIZE: u32 = 0; // with 16-byte align padding
pub const XCHAL_TOTAL_SA_ALIGN: u32 = 1; // actual minimum alignment

pub const XCHAL_NCP_SA_NUM: u32 = 0;
#[macro_export]
macro_rules! XCHAL_NCP_SA_LIST { ($s:ident) => {}; }
pub const XCHAL_CP0_SA_NUM: u32 = 0;
#[macro_export]
macro_rules! XCHAL_CP0_SA_LIST { ($s:ident) => {}; }
pub const XCHAL_CP1_SA_NUM: u32 = 0;
#[macro_export]
macro_rules! XCHAL_CP1_SA_LIST { ($s:ident) => {}; }
pub const XCHAL_CP2_SA_NUM: u32 = 0;
#[macro_export]
macro_rules! XCHAL_CP2_SA_LIST { ($s:ident) => {}; }
pub const XCHAL_CP3_SA_NUM: u32 = 0;
#[macro_export]
macro_rules! XCHAL_CP3_SA_LIST { ($s:ident) => {}; }
pub const XCHAL_CP4_SA_NUM: u32 = 0;
#[macro_export]
macro_rules! XCHAL_CP4_SA_LIST { ($s:ident) => {}; }
pub const XCHAL_CP5_SA_NUM: u32 = 0;
#[macro_export]
macro_rules! XCHAL_CP5_SA_LIST { ($s:ident) => {}; }
pub const XCHAL_CP6_SA_NUM: u32 = 0;
#[macro_export]
macro_rules! XCHAL_CP6_SA_LIST { ($s:ident) => {}; }
pub const XCHAL_CP7_SA_NUM: u32 = 0;
#[macro_export]
macro_rules! XCHAL_CP7_SA_LIST { ($s:ident) => {}; }

/* Byte length of instruction from its first nibble (op0 field), per FLIX.  */
pub const XCHAL_OP0_FORMAT_LENGTHS: [u32; 16] =
    [3, 3, 3, 3, 3, 3, 3, 3, 2, 2, 2, 2, 2, 2, 3, 3];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
