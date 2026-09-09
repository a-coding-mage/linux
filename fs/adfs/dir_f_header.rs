/* SPDX-License-Identifier: GPL-2.0-only */
/*
 *  linux/fs/adfs/dir_f.h
 *
 *  Copyright (C) 1999 Russell King
 *
 *  Structures of directories on the F format disk
 */

/*
 * Directory header
 */
#[repr(C, packed)]
pub struct adfs_dirheader {
    pub startmasseq: __u8,
    pub startname: [__u8; 4],
}

pub const ADFS_NEWDIR_SIZE: usize = 2048;
pub const ADFS_NUM_DIR_ENTRIES: usize = 77;

/*
 * Directory entries
 */
pub const ADFS_F_NAME_LEN: usize = 10;

#[repr(C, packed)]
pub struct adfs_direntry {
    pub dirobname: [core::ffi::c_char; ADFS_F_NAME_LEN],
    pub dirload: [__u8; 4],
    pub direxec: [__u8; 4],
    pub dirlen: [__u8; 4],
    pub dirinddiscadd: [__u8; 3],
    pub newdiratts: __u8,
}

/*
 * Directory tail
 */
#[repr(C, packed)]
pub struct adfs_olddirtail {
    pub dirlastmask: __u8,
    pub dirname: [core::ffi::c_char; 10],
    pub dirparent: [__u8; 3],
    pub dirtitle: [core::ffi::c_char; 19],
    pub reserved: [__u8; 14],
    pub endmasseq: __u8,
    pub endname: [__u8; 4],
    pub dircheckbyte: __u8,
}

#[repr(C, packed)]
pub struct adfs_newdirtail {
    pub dirlastmask: __u8,
    pub reserved: [__u8; 2],
    pub dirparent: [__u8; 3],
    pub dirtitle: [core::ffi::c_char; 19],
    pub dirname: [core::ffi::c_char; 10],
    pub endmasseq: __u8,
    pub endname: [__u8; 4],
    pub dircheckbyte: __u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
