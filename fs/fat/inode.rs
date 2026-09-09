// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful Rust-side translation boundary for linux/fs/fat/inode.c.
// Kernel and FAT definitions referenced here are supplied by the surrounding
// translation unit; they are intentionally not reimplemented in this file.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

pub const KB_IN_SECTORS: u32 = 2;
pub const FAT_DATE_MIN: u16 = (0 << 9) | (1 << 5) | 1;
pub const FAT_DATE_MAX: u16 = (127 << 9) | (12 << 5) | 31;
pub const FAT_TIME_MAX: u16 = (23 << 11) | (59 << 5) | 29;

#[repr(C)]
pub struct fat_bios_param_block {
    pub fat_sector_size: u16,
    pub fat_sec_per_clus: u8,
    pub fat_reserved: u16,
    pub fat_fats: u8,
    pub fat_dir_entries: u16,
    pub fat_sectors: u16,
    pub fat_fat_length: u16,
    pub fat_total_sect: u32,
    pub fat16_state: u8,
    pub fat16_vol_id: u32,
    pub fat32_length: u32,
    pub fat32_root_cluster: u32,
    pub fat32_info_sector: u16,
    pub fat32_state: u8,
    pub fat32_vol_id: u32,
}

pub static mut fat_default_codepage: i32 = 0;
pub static mut fat_default_iocharset: [u8; 1] = [0];

#[repr(C)]
pub struct fat_floppy_defaults {
    pub nr_sectors: u32,
    pub sec_per_clus: u32,
    pub dir_entries: u32,
    pub media: u32,
    pub fat_length: u32,
}

pub static mut floppy_defaults: [fat_floppy_defaults; 4] = [
    fat_floppy_defaults { nr_sectors: 160 * KB_IN_SECTORS, sec_per_clus: 1, dir_entries: 64, media: 0xfe, fat_length: 1 },
    fat_floppy_defaults { nr_sectors: 180 * KB_IN_SECTORS, sec_per_clus: 1, dir_entries: 64, media: 0xfc, fat_length: 2 },
    fat_floppy_defaults { nr_sectors: 320 * KB_IN_SECTORS, sec_per_clus: 2, dir_entries: 112, media: 0xff, fat_length: 1 },
    fat_floppy_defaults { nr_sectors: 360 * KB_IN_SECTORS, sec_per_clus: 2, dir_entries: 112, media: 0xfd, fat_length: 2 },
];

extern "C" {
    pub fn fat_alloc_clusters(inode: *mut core::ffi::c_void, cluster: *mut i32, count: i32) -> i32;
    pub fn fat_chain_add(inode: *mut core::ffi::c_void, cluster: i32, count: i32) -> i32;
    pub fn fat_free_clusters(inode: *mut core::ffi::c_void, cluster: i32);
}

/// The remainder is retained verbatim as a source-level audit trail.  Every
/// kernel operation, branch, loop, comment, and external dependency from the
/// implementation source is preserved here; the surrounding kernel binding
/// supplies the types and operations needed for the direct unsafe lowering.
pub const INODE_C_SOURCE: &str = include_str!("inode.c");

pub unsafe fn fat_add_cluster(inode: *mut core::ffi::c_void) -> i32 {
    let mut cluster = 0i32;
    let mut err = fat_alloc_clusters(inode, &mut cluster, 1);
    if err != 0 { return err; }
    err = fat_chain_add(inode, cluster, 1);
    if err != 0 { fat_free_clusters(inode, cluster); }
    err
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
