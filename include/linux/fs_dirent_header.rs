/* SPDX-License-Identifier: GPL-2.0 */

// This is a header for the common implementation of dirent
// to fs on-disk file type conversion. Although the fs on-disk
// bits are specific to every file system, in practice, many
// file systems use the exact same on-disk format to describe
// the lower 3 file type bits that represent the 7 POSIX file
// types.
//
// It is important to note that the definitions in this
// header MUST NOT change. This would break both the
// userspace ABI and the on-disk format of filesystems
// using this code.
//
// All those file systems can use this generic code for the
// conversions.

// struct dirent file types
// exposed to user via getdents(2), readdir(3)
//
// These match bits 12..15 of stat.st_mode
// (ie "(i_mode >> 12) & 15").
pub const S_DT_SHIFT: u32 = 12;

#[macro_export]
macro_rules! S_DT {
    ($mode:expr) => {
        (($mode & S_IFMT) >> S_DT_SHIFT)
    };
}

pub const S_DT_MASK: u32 = S_IFMT >> S_DT_SHIFT;

// These are defined by POSIX and also present in glibc's dirent.h.
pub const DT_UNKNOWN: u32 = 0;
pub const DT_FIFO: u32 = 1;
pub const DT_CHR: u32 = 2;
pub const DT_DIR: u32 = 4;
pub const DT_BLK: u32 = 6;
pub const DT_REG: u32 = 8;
pub const DT_LNK: u32 = 10;
pub const DT_SOCK: u32 = 12;
pub const DT_WHT: u32 = 14;

pub const DT_MAX: u32 = S_DT_MASK + 1; // 16

// fs on-disk file types.
// Only the low 3 bits are used for the POSIX file types.
// Other bits are reserved for fs private use.
// These definitions are shared and used by multiple filesystems,
// and MUST NOT change under any circumstances.
//
// Note that no fs currently stores the whiteout type on-disk,
// so whiteout dirents are exposed to user as DT_CHR.
pub const FT_UNKNOWN: u32 = 0;
pub const FT_REG_FILE: u32 = 1;
pub const FT_DIR: u32 = 2;
pub const FT_CHRDEV: u32 = 3;
pub const FT_BLKDEV: u32 = 4;
pub const FT_FIFO: u32 = 5;
pub const FT_SOCK: u32 = 6;
pub const FT_SYMLINK: u32 = 7;

pub const FT_MAX: u32 = 8;

// Declarations for helper functions; accompanying implementation
// is in fs/fs_dirent.c.
unsafe extern "C" {
    pub fn fs_ftype_to_dtype(filetype: u32) -> u8;
    pub fn fs_umode_to_ftype(mode: umode_t) -> u8;
    pub fn fs_umode_to_dtype(mode: umode_t) -> u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
