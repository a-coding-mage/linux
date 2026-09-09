/* SPDX-License-Identifier: GPL-2.0-or-later */
/* AFS fileserver XDR types
 *
 * Copyright (C) 2018 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

#[repr(C, packed)]
pub struct afs_xdr_AFSFetchStatus {
    pub if_version: __be32,
    pub type_: __be32,
    pub nlink: __be32,
    pub size_lo: __be32,
    pub data_version_lo: __be32,
    pub author: __be32,
    pub owner: __be32,
    pub caller_access: __be32,
    pub anon_access: __be32,
    pub mode: __be32,
    pub parent_vnode: __be32,
    pub parent_unique: __be32,
    pub seg_size: __be32,
    pub mtime_client: __be32,
    pub mtime_server: __be32,
    pub group: __be32,
    pub sync_counter: __be32,
    pub data_version_hi: __be32,
    pub lock_count: __be32,
    pub size_hi: __be32,
    pub abort_code: __be32,
}

pub const AFS_FSTATUS_VERSION: u32 = 1;

pub const AFS_DIR_HASHTBL_SIZE: usize = 128;
pub const AFS_DIR_DIRENT_SIZE: usize = 32;
pub const AFS_DIR_SLOTS_PER_BLOCK: usize = 64;
pub const AFS_DIR_BLOCK_SIZE: usize = 2048;
pub const AFS_DIR_BLOCKS_PER_PAGE: usize = PAGE_SIZE / AFS_DIR_BLOCK_SIZE;
pub const AFS_DIR_MAX_SLOTS: usize = 65536;
pub const AFS_DIR_BLOCKS_WITH_CTR: usize = 128;
pub const AFS_DIR_MAX_BLOCKS: usize = 1023;
pub const AFS_DIR_RESV_BLOCKS: usize = 1;
pub const AFS_DIR_RESV_BLOCKS0: usize = 13;

/*
 * Directory entry structure.
 */
#[repr(C, packed)]
pub struct afs_xdr_dirent_u {
    pub valid: u8,
    pub unused: [u8; 1],
    pub hash_next: __be16,
    pub vnode: __be32,
    pub unique: __be32,
    pub name: [u8; 0],
}

#[repr(C, packed)]
pub union afs_xdr_dirent {
    pub u: afs_xdr_dirent_u,
    pub extended_name: [u8; 32],
}

/*
 * Directory block header (one at the beginning of every 2048-byte block).
 */
#[repr(C, packed)]
pub struct afs_xdr_dir_hdr {
    pub npages: __be16,
    pub magic: __be16,
    pub reserved: u8,
    pub bitmap: [u8; 8],
    pub pad: [u8; 19],
}

/* AFS_DIR_MAGIC htons(1234). */
pub const AFS_DIR_MAGIC: u16 = 1234u16.to_be();

/*
 * Directory block layout
 */
#[repr(C, packed)]
pub struct afs_xdr_dir_block_meta {
    pub hdr: afs_xdr_dir_hdr,
    pub alloc_ctrs: [u8; AFS_DIR_BLOCKS_WITH_CTR],
    pub hashtable: [__be16; AFS_DIR_HASHTBL_SIZE],
}

#[repr(C, packed)]
pub union afs_xdr_dir_block {
    pub hdr: afs_xdr_dir_hdr,
    pub meta: afs_xdr_dir_block_meta,
    pub dirents: [afs_xdr_dirent; AFS_DIR_SLOTS_PER_BLOCK],
}

/*
 * Directory layout on a linux VM page.
 */
#[repr(C)]
pub struct afs_xdr_dir_page {
    pub blocks: [afs_xdr_dir_block; AFS_DIR_BLOCKS_PER_PAGE],
}

/*
 * Calculate the number of dirent slots required for any given name length.
 * The calculation is made assuming the part of the name in the first slot is
 * 16 bytes, rather than 20, but this miscalculation is now standardised.
 */
#[inline]
pub fn afs_dir_calc_slots(mut name_len: usize) -> u32 {
    name_len = name_len.wrapping_add(1); /* NUL-terminated */
    (1 + ((name_len + 15) / AFS_DIR_DIRENT_SIZE)) as u32
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
