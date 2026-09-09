/*
 * Cache operations for the cache instruction.
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * (C) Copyright 1996, 97, 99, 2002, 03 Ralf Baechle
 * (C) Copyright 1999 Silicon Graphics, Inc.
 */

/*
 * Most cache ops are split into a 2 bit field identifying the cache, and a 3
 * bit field identifying the cache operation.
 */
pub const CacheOp_Cache: u32 = 0x03;
pub const CacheOp_Op: u32 = 0x1c;

pub const Cache_I: u32 = 0x00;
pub const Cache_D: u32 = 0x01;
pub const Cache_T: u32 = 0x02;
pub const Cache_V: u32 = 0x02; /* Loongson-3 */
pub const Cache_S: u32 = 0x03;

pub const Index_Writeback_Inv: u32 = 0x00;
pub const Index_Load_Tag: u32 = 0x04;
pub const Index_Store_Tag: u32 = 0x08;
pub const Hit_Invalidate: u32 = 0x10;
pub const Hit_Writeback_Inv: u32 = 0x14; /* not with Cache_I though */
pub const Hit_Writeback: u32 = 0x18;

/* Cache Operations available on all MIPS processors with R4000-style caches */
pub const Index_Invalidate_I: u32 = Cache_I | Index_Writeback_Inv;
pub const Index_Writeback_Inv_D: u32 = Cache_D | Index_Writeback_Inv;
pub const Index_Load_Tag_I: u32 = Cache_I | Index_Load_Tag;
pub const Index_Load_Tag_D: u32 = Cache_D | Index_Load_Tag;
pub const Index_Store_Tag_I: u32 = Cache_I | Index_Store_Tag;
pub const Index_Store_Tag_D: u32 = Cache_D | Index_Store_Tag;
pub const Hit_Invalidate_I: u32 = Cache_I | Hit_Invalidate;
pub const Hit_Invalidate_D: u32 = Cache_D | Hit_Invalidate;
pub const Hit_Writeback_Inv_D: u32 = Cache_D | Hit_Writeback_Inv;

/* R4000-specific cacheops */
pub const Create_Dirty_Excl_D: u32 = Cache_D | 0x0c;
pub const Fill_I: u32 = Cache_I | 0x14;
pub const Hit_Writeback_I: u32 = Cache_I | Hit_Writeback;
pub const Hit_Writeback_D: u32 = Cache_D | Hit_Writeback;

/* R4000SC and R4400SC-specific cacheops */
pub const Cache_SI: u32 = 0x02;
pub const Cache_SD: u32 = 0x03;
pub const Index_Invalidate_SI: u32 = Cache_SI | Index_Writeback_Inv;
pub const Index_Writeback_Inv_SD: u32 = Cache_SD | Index_Writeback_Inv;
pub const Index_Load_Tag_SI: u32 = Cache_SI | Index_Load_Tag;
pub const Index_Load_Tag_SD: u32 = Cache_SD | Index_Load_Tag;
pub const Index_Store_Tag_SI: u32 = Cache_SI | Index_Store_Tag;
pub const Index_Store_Tag_SD: u32 = Cache_SD | Index_Store_Tag;
pub const Create_Dirty_Excl_SD: u32 = Cache_SD | 0x0c;
pub const Hit_Invalidate_SI: u32 = Cache_SI | Hit_Invalidate;
pub const Hit_Invalidate_SD: u32 = Cache_SD | Hit_Invalidate;
pub const Hit_Writeback_Inv_SD: u32 = Cache_SD | Hit_Writeback_Inv;
pub const Hit_Writeback_SD: u32 = Cache_SD | Hit_Writeback;
pub const Hit_Set_Virtual_SI: u32 = Cache_SI | 0x1c;
pub const Hit_Set_Virtual_SD: u32 = Cache_SD | 0x1c;

/* R5000-specific cacheops */
pub const R5K_Page_Invalidate_S: u32 = Cache_S | 0x14;

/* RM7000-specific cacheops */
pub const Page_Invalidate_T: u32 = Cache_T | 0x14;
pub const Index_Store_Tag_T: u32 = Cache_T | Index_Store_Tag;
pub const Index_Load_Tag_T: u32 = Cache_T | Index_Load_Tag;

/*
 * R10000-specific cacheops
 *
 * Cacheops 0x02, 0x06, 0x0a, 0x0c-0x0e, 0x16, 0x1a and 0x1e are unused.
 * Most of the _S cacheops are identical to the R4000SC _SD cacheops.
 */
pub const Index_Writeback_Inv_S: u32 = Cache_S | Index_Writeback_Inv;
pub const Index_Load_Tag_S: u32 = Cache_S | Index_Load_Tag;
pub const Index_Store_Tag_S: u32 = Cache_S | Index_Store_Tag;
pub const Hit_Invalidate_S: u32 = Cache_S | Hit_Invalidate;
pub const Cache_Barrier: u32 = 0x14;
pub const Hit_Writeback_Inv_S: u32 = Cache_S | Hit_Writeback_Inv;
pub const Index_Load_Data_I: u32 = Cache_I | 0x18;
pub const Index_Load_Data_D: u32 = Cache_D | 0x18;
pub const Index_Load_Data_S: u32 = Cache_S | 0x18;
pub const Index_Store_Data_I: u32 = Cache_I | 0x1c;
pub const Index_Store_Data_D: u32 = Cache_D | 0x1c;
pub const Index_Store_Data_S: u32 = Cache_S | 0x1c;

/* Loongson2-specific cacheops */
pub const Hit_Invalidate_I_Loongson2: u32 = Cache_I | 0x00;

/* Loongson3-specific cacheops */
pub const Index_Writeback_Inv_V: u32 = Cache_V | Index_Writeback_Inv;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
