/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/fs/adfs/dir_fplus.h
 *
 *  Copyright (C) 1999 Russell King
 *
 *  Structures of directories on the F+ format disk
 */

pub const ADFS_FPLUS_NAME_LEN: usize = 255;

pub const BIGDIRSTARTNAME: u32 = ('S' as u32)
    | (('B' as u32) << 8)
    | (('P' as u32) << 16)
    | (('r' as u32) << 24);
pub const BIGDIRENDNAME: u32 = ('o' as u32)
    | (('v' as u32) << 8)
    | (('e' as u32) << 16)
    | (('n' as u32) << 24);

#[repr(C, packed)]
pub struct adfs_bigdirheader {
    pub startmasseq: u8,
    pub bigdirversion: [u8; 3],
    pub bigdirstartname: __le32,
    pub bigdirnamelen: __le32,
    pub bigdirsize: __le32,
    pub bigdirentries: __le32,
    pub bigdirnamesize: __le32,
    pub bigdirparent: __le32,
    pub bigdirname: [std::ffi::c_char; 1],
}

#[repr(C, packed)]
pub struct adfs_bigdirentry {
    pub bigdirload: __le32,
    pub bigdirexec: __le32,
    pub bigdirlen: __le32,
    pub bigdirindaddr: __le32,
    pub bigdirattr: __le32,
    pub bigdirobnamelen: __le32,
    pub bigdirobnameptr: __le32,
}

#[repr(C, packed)]
pub struct adfs_bigdirtail {
    pub bigdirendname: __le32,
    pub bigdirendmasseq: u8,
    pub reserved: [u8; 2],
    pub bigdircheckbyte: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
