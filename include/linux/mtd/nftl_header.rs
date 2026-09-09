/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright © 1999-2010 David Woodhouse <dwmw2@infradead.org>
 */

// Dependencies supplied by the corresponding Linux MTD headers:
// mtd_blktrans_dev, mtd_info, NFTLMediaHeader, and erase_info.

/* these info are used in ReplUnitTable */
pub const BLOCK_NIL: u16 = 0xffff; // last block of a chain
pub const BLOCK_FREE: u16 = 0xfffe; // free block
pub const BLOCK_NOTEXPLORED: u16 = 0xfffd; // non explored block, only used during mounting
pub const BLOCK_RESERVED: u16 = 0xfffc; // bios block or bad block

#[repr(C)]
pub struct NFTLrecord {
    pub mbd: mtd_blktrans_dev,
    pub MediaUnit: u16,
    pub SpareMediaUnit: u16,
    pub EraseSize: u32,
    pub MediaHdr: NFTLMediaHeader,
    pub usecount: i32,
    pub heads: u8,
    pub sectors: u8,
    pub cylinders: u16,
    pub numvunits: u16,
    pub lastEUN: u16, // should be suppressed
    pub numfreeEUNs: u16,
    pub LastFreeEUN: u16, // To speed up finding a free EUN
    pub head: i32,
    pub sect: i32,
    pub cyl: i32,
    pub EUNtable: *mut u16, // [numvunits]: First EUN for each virtual unit
    pub ReplUnitTable: *mut u16, // [numEUNs]: ReplUnitNumber for each
    pub nb_blocks: u32, // number of physical blocks
    pub nb_boot_blocks: u32, // number of blocks used by the bios
    pub instr: erase_info,
}

unsafe extern "C" {
    pub fn NFTL_mount(s: *mut NFTLrecord) -> i32;
    pub fn NFTL_formatblock(s: *mut NFTLrecord, block: i32) -> i32;

    pub fn nftl_read_oob(
        mtd: *mut mtd_info,
        offs: i64,
        len: usize,
        retlen: *mut usize,
        buf: *mut u8,
    ) -> i32;
    pub fn nftl_write_oob(
        mtd: *mut mtd_info,
        offs: i64,
        len: usize,
        retlen: *mut usize,
        buf: *mut u8,
    ) -> i32;
}

pub const NFTL_MAJOR: i32 = 93;
pub const MAX_NFTLS: i32 = 16;
pub const MAX_SECTORS_PER_UNIT: i32 = 64;
pub const NFTL_PARTN_BITS: i32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
