/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Parts of INFTL headers shared with userspace
 *
 */

pub const OSAK_VERSION: u32 = 0x5120;
pub const PERCENTUSED: u32 = 98;

pub const SECTORSIZE: u32 = 512;

/* Block Control Information */

#[repr(C, packed)]
pub struct inftl_bci {
    pub ECCsig: [u8; 6],
    pub Status: u8,
    pub Status1: u8,
}

#[repr(C, packed)]
pub struct inftl_unithead1 {
    pub virtualUnitNo: u16,
    pub prevUnitNo: u16,
    pub ANAC: u8,
    pub NACs: u8,
    pub parityPerField: u8,
    pub discarded: u8,
}

#[repr(C, packed)]
pub struct inftl_unithead2 {
    pub parityPerField: u8,
    pub ANAC: u8,
    pub prevUnitNo: u16,
    pub virtualUnitNo: u16,
    pub NACs: u8,
    pub discarded: u8,
}

#[repr(C, packed)]
pub struct inftl_unittail {
    pub Reserved: [u8; 4],
    pub EraseMark: u16,
    pub EraseMark1: u16,
}

#[repr(C)]
pub union inftl_uci {
    pub a: inftl_unithead1,
    pub b: inftl_unithead2,
    pub c: inftl_unittail,
}

#[repr(C)]
pub struct inftl_oob {
    pub b: inftl_bci,
    pub u: inftl_uci,
}

/* INFTL Media Header */

#[repr(C, packed)]
pub struct INFTLPartition {
    pub virtualUnits: u32,
    pub firstUnit: u32,
    pub lastUnit: u32,
    pub flags: u32,
    pub spareUnits: u32,
    pub Reserved0: u32,
    pub Reserved1: u32,
}

#[repr(C, packed)]
pub struct INFTLMediaHeader {
    pub bootRecordID: [i8; 8],
    pub NoOfBootImageBlocks: u32,
    pub NoOfBinaryPartitions: u32,
    pub NoOfBDTLPartitions: u32,
    pub BlockMultiplierBits: u32,
    pub FormatFlags: u32,
    pub OsakVersion: u32,
    pub PercentUsed: u32,
    pub Partitions: [INFTLPartition; 4],
}

/* Partition flag types */
pub const INFTL_BINARY: u32 = 0x20000000;
pub const INFTL_BDTL: u32 = 0x40000000;
pub const INFTL_LAST: u32 = 0x80000000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
