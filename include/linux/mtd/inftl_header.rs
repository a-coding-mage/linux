/* SPDX-License-Identifier: GPL-2.0 */
/*
 *	inftl.h -- defines to support the Inverse NAND Flash Translation Layer
 *
 *	(C) Copyright 2002, Greg Ungerer (gerg@snapgear.com)
 */

/* This is a kernel header; the original includes kernel MTD/NFTL headers. */

/* #ifndef INFTL_MAJOR */
pub const INFTL_MAJOR: i32 = 96;
/* #endif */
pub const INFTL_PARTN_BITS: i32 = 4;

/* The following types are supplied by the corresponding kernel headers. */
extern "C" {
    pub type mtd_blktrans_dev;
    pub type INFTLMediaHeader;
    pub type erase_info;
    pub type mtd_info;
}

#[repr(C)]
pub struct INFTLrecord {
    pub mbd: mtd_blktrans_dev,
    pub MediaUnit: u16,
    pub EraseSize: u32,
    pub MediaHdr: INFTLMediaHeader,
    pub usecount: i32,
    pub heads: u8,
    pub sectors: u8,
    pub cylinders: u16,
    pub numvunits: u16,
    pub firstEUN: u16,
    pub lastEUN: u16,
    pub numfreeEUNs: u16,
    pub LastFreeEUN: u16, /* To speed up finding a free EUN */
    pub head: i32,
    pub sect: i32,
    pub cyl: i32,
    pub PUtable: *mut u16, /* Physical Unit Table */
    pub VUtable: *mut u16, /* Virtual Unit Table */
    pub nb_blocks: u32, /* number of physical blocks */
    pub nb_boot_blocks: u32, /* number of blocks used by the bios */
    pub instr: erase_info,
}

extern "C" {
    pub fn INFTL_mount(s: *mut INFTLrecord) -> i32;
    pub fn INFTL_formatblock(s: *mut INFTLrecord, block: i32) -> i32;

    pub fn INFTL_dumptables(s: *mut INFTLrecord);
    pub fn INFTL_dumpVUchains(s: *mut INFTLrecord);

    pub fn inftl_read_oob(
        mtd: *mut mtd_info,
        offs: i64,
        len: usize,
        retlen: *mut usize,
        buf: *mut u8,
    ) -> i32;
    pub fn inftl_write_oob(
        mtd: *mut mtd_info,
        offs: i64,
        len: usize,
        retlen: *mut usize,
        buf: *mut u8,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
