/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  linux/include/linux/ext2_fs.h
 *
 * Copyright (C) 1992, 1993, 1994, 1995
 * Remy Card (card@masi.ibp.fr)
 * Laboratoire MASI - Institut Blaise Pascal
 * Universite Pierre et Marie Curie (Paris VI)
 *
 *  from
 *
 *  linux/include/linux/minix_fs.h
 *
 *  Copyright (C) 1991, 1992  Linus Torvalds
 */

// Dependencies supplied by the surrounding Linux translation:
// linux/types.h and linux/magic.h

pub const EXT2_NAME_LEN: usize = 255;

/*
 * Maximal count of links to a file
 */
pub const EXT2_LINK_MAX: u32 = 32000;

pub const EXT2_SB_MAGIC_OFFSET: usize = 0x38;
pub const EXT2_SB_BLOCKS_OFFSET: usize = 0x04;
pub const EXT2_SB_BSIZE_OFFSET: usize = 0x18;

/// Returns the size of an ext2 image, or zero when the superblock magic is invalid.
pub unsafe fn ext2_image_size(ext2_sb: *mut core::ffi::c_void) -> u64 {
    let p = ext2_sb as *mut u8;
    if *(p.add(EXT2_SB_MAGIC_OFFSET) as *mut u16)
        != crate::cpu_to_le16(crate::EXT2_SUPER_MAGIC)
    {
        return 0;
    }
    (crate::le32_to_cpup(p.add(EXT2_SB_BLOCKS_OFFSET) as *const u32) as u64)
        << crate::le32_to_cpup(p.add(EXT2_SB_BSIZE_OFFSET) as *const u32)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
