/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependencies corresponding to <linux/types.h> and <linux/fs.h> are
 * provided by the surrounding translation unit. */

/* The basic structures of the romfs filesystem */

pub const ROMBSIZE: usize = BLOCK_SIZE;
pub const ROMBSBITS: usize = BLOCK_SIZE_BITS;
pub const ROMBMASK: usize = ROMBSIZE - 1;
pub const ROMFS_MAGIC: u32 = 0x7275;

pub const ROMFS_MAXFN: usize = 128;

pub const fn __mkw(h: u32, l: u32) -> u32 {
    ((h & 0x00ff) << 8) | (l & 0x00ff)
}

pub const fn __mkl(h: u32, l: u32) -> u32 {
    ((h & 0xffff) << 16) | (l & 0xffff)
}

pub const fn __mk4(a: u32, b: u32, c: u32, d: u32) -> u32 {
    cpu_to_be32(__mkl(__mkw(a, b), __mkw(c, d)))
}

pub const ROMSB_WORD0: u32 = __mk4(b'-' as u32, b'r' as u32, b'o' as u32, b'm' as u32);
pub const ROMSB_WORD1: u32 = __mk4(b'1' as u32, b'f' as u32, b's' as u32, b'-' as u32);

/* On-disk "super block" */

#[repr(C)]
pub struct romfs_super_block {
    pub word0: __be32,
    pub word1: __be32,
    pub size: __be32,
    pub checksum: __be32,
    pub name: [i8; 0], /* volume name */
}

/* On disk inode */

#[repr(C)]
pub struct romfs_inode {
    pub next: __be32, /* low 4 bits see ROMFH_ */
    pub spec: __be32,
    pub size: __be32,
    pub checksum: __be32,
    pub name: [i8; 0],
}

pub const ROMFH_TYPE: u32 = 7;
pub const ROMFH_HRD: u32 = 0;
pub const ROMFH_DIR: u32 = 1;
pub const ROMFH_REG: u32 = 2;
pub const ROMFH_SYM: u32 = 3;
pub const ROMFH_BLK: u32 = 4;
pub const ROMFH_CHR: u32 = 5;
pub const ROMFH_SCK: u32 = 6;
pub const ROMFH_FIF: u32 = 7;
pub const ROMFH_EXEC: u32 = 8;

/* Alignment */

pub const ROMFH_SIZE: usize = 16;
pub const ROMFH_PAD: usize = ROMFH_SIZE - 1;
pub const ROMFH_MASK: usize = !ROMFH_PAD;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
