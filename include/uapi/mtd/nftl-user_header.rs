/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright © 1999-2010 David Woodhouse <dwmw2@infradead.org>
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin St, Fifth Floor, Boston, MA  02110-1301  USA
 *
 */

/* Dependency: <linux/types.h> */

/* Block Control Information */
#[repr(C, packed)]
pub struct nftl_bci {
    pub ECCSig: [u8; 6],
    pub Status: u8,
    pub Status1: u8,
}

/* Unit Control Information */
#[repr(C, packed)]
pub struct nftl_uci0 {
    pub VirtUnitNum: u16,
    pub ReplUnitNum: u16,
    pub SpareVirtUnitNum: u16,
    pub SpareReplUnitNum: u16,
}

#[repr(C, packed)]
pub struct nftl_uci1 {
    pub WearInfo: u32,
    pub EraseMark: u16,
    pub EraseMark1: u16,
}

#[repr(C, packed)]
pub struct nftl_uci2 {
    pub FoldMark: u16,
    pub FoldMark1: u16,
    pub unused: u32,
}

#[repr(C)]
pub union nftl_uci {
    pub a: nftl_uci0,
    pub b: nftl_uci1,
    pub c: nftl_uci2,
}

#[repr(C)]
pub struct nftl_oob {
    pub b: nftl_bci,
    pub u: nftl_uci,
}

/* NFTL Media Header */
#[repr(C, packed)]
pub struct NFTLMediaHeader {
    pub DataOrgID: [std::os::raw::c_char; 6],
    pub NumEraseUnits: u16,
    pub FirstPhysicalEUN: u16,
    pub FormattedSize: u32,
    pub UnitSizeFactor: u8,
}

pub const MAX_ERASE_ZONES: u32 = 8192 - 512;

pub const ERASE_MARK: u32 = 0x3c69;
pub const SECTOR_FREE: u32 = 0xff;
pub const SECTOR_USED: u32 = 0x55;
pub const SECTOR_IGNORE: u32 = 0x11;
pub const SECTOR_DELETED: u32 = 0x00;

pub const FOLD_MARK_IN_PROGRESS: u32 = 0x5555;

pub const ZONE_GOOD: u32 = 0xff;
pub const ZONE_BAD_ORIGINAL: u32 = 0;
pub const ZONE_BAD_MARKED: u32 = 7;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
