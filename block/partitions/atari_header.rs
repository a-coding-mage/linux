/* SPDX-License-Identifier: GPL-2.0 */
/*
 *  fs/partitions/atari.h
 *  Moved by Russell King from:
 *
 * linux/include/linux/atari_rootsec.h
 * definitions for Atari Rootsector layout
 * by Andreas Schwab (schwab@ls5.informatik.uni-dortmund.de)
 *
 * modified for ICD/Supra partitioning scheme restricted to at most 12
 * partitions
 * by Guenther Kelleter (guenther@pool.informatik.rwth-aachen.de)
 */

// Dependency intent: C types `u8`, `u16`, `u32`, and `__be32` are represented
// by their corresponding Rust-width integer types; `__be32` values retain
// their big-endian interpretation at use sites.

#[repr(C)]
pub struct partition_info {
    pub flg: u8,                 /* bit 0: active; bit 7: bootable */
    pub id: [std::ffi::c_char; 3], /* "GEM", "BGM", "XGM", or other */
    pub st: u32,                 /* start of partition */
    pub siz: u32,                /* length of partition */
}

#[repr(C, packed)]
pub struct rootsector {
    pub unused: [std::ffi::c_char; 0x156], /* room for boot code */
    pub icdpart: [partition_info; 8],      /* info for ICD-partitions 5..12 */
    pub unused2: [std::ffi::c_char; 0xc],
    pub hd_siz: u32,                       /* size of disk in blocks */
    pub part: [partition_info; 4],
    pub bsl_st: u32,                       /* start of bad sector list */
    pub bsl_cnt: u32,                      /* length of bad sector list */
    pub checksum: u16,                     /* checksum for bootable disks */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
