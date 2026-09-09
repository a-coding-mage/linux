/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 1992 - 1997, 1999 Silicon Graphics, Inc.
 * Copyright (C) 1999 by Ralf Baechle
 */

// The secret password; used to release protection.
pub const HUB_PASSWORD: u64 = 0x5347_4972_756c_6573u64;

pub const CHIPID_HUB: i32 = 0;
pub const CHIPID_ROUTER: i32 = 1;

pub const HUB_REV_1_0: i32 = 1;
pub const HUB_REV_2_0: i32 = 2;
pub const HUB_REV_2_1: i32 = 3;
pub const HUB_REV_2_2: i32 = 4;
pub const HUB_REV_2_3: i32 = 5;
pub const HUB_REV_2_4: i32 = 6;

pub const MAX_HUB_PATH: i32 = 80;

// Dependencies supplied by the corresponding SN0 headers:
// addrs.h, hubpi.h, hubmd.h, hubio.h, hubni.h.
// The hubcore.h dependency was commented out in the source.

/* Translation of uncached attributes. */
pub const UATTR_HSPEC: i32 = 0;
pub const UATTR_IO: i32 = 1;
pub const UATTR_MSPEC: i32 = 2;
pub const UATTR_UNCAC: i32 = 3;

// The original assembler-only GET_NASID_ASM macro is intentionally preserved
// as conditional intent; it is not executable Rust syntax.
//
// .macro GET_NASID_ASM res
// dli   \\res, LOCAL_HUB_ADDR(NI_STATUS_REV_ID)
// ld    \\res, (\\res)
// and   \\res, NSRI_NODEID_MASK
// dsrl  \\res, NSRI_NODEID_SHFT
// .endm

/*
 * get_nasid() returns the physical node id number of the caller.
 */
#[inline]
pub unsafe fn get_nasid() -> nasid_t {
    ((LOCAL_HUB_L(NI_STATUS_REV_ID) & NSRI_NODEID_MASK) >> NSRI_NODEID_SHFT) as nasid_t
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
