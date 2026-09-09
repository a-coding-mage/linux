/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * Copyright © 1999-2010 David Woodhouse <dwmw2@infradead.org> et al.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 */

#[repr(C)]
pub struct erase_info_user {
    pub start: u32,
    pub length: u32,
}

#[repr(C)]
pub struct erase_info_user64 {
    pub start: u64,
    pub length: u64,
}

#[repr(C)]
pub struct mtd_oob_buf {
    pub start: u32,
    pub length: u32,
    pub ptr: *mut u8,
}

#[repr(C)]
pub struct mtd_oob_buf64 {
    pub start: u64,
    pub pad: u32,
    pub length: u32,
    pub usr_ptr: u64,
}

pub const MTD_OPS_PLACE_OOB: i32 = 0;
pub const MTD_OPS_AUTO_OOB: i32 = 1;
pub const MTD_OPS_RAW: i32 = 2;

#[repr(C)]
pub struct mtd_write_req {
    pub start: u64,
    pub len: u64,
    pub ooblen: u64,
    pub usr_data: u64,
    pub usr_oob: u64,
    pub mode: u8,
    pub padding: [u8; 7],
}

#[repr(C)]
pub struct mtd_read_req_ecc_stats {
    pub uncorrectable_errors: u32,
    pub corrected_bitflips: u32,
    pub max_bitflips: u32,
}

#[repr(C)]
pub struct mtd_read_req {
    pub start: u64,
    pub len: u64,
    pub ooblen: u64,
    pub usr_data: u64,
    pub usr_oob: u64,
    pub mode: u8,
    pub padding: [u8; 7],
    pub ecc_stats: mtd_read_req_ecc_stats,
}

pub const MTD_ABSENT: u32 = 0;
pub const MTD_RAM: u32 = 1;
pub const MTD_ROM: u32 = 2;
pub const MTD_NORFLASH: u32 = 3;
pub const MTD_NANDFLASH: u32 = 4;
pub const MTD_DATAFLASH: u32 = 6;
pub const MTD_UBIVOLUME: u32 = 7;
pub const MTD_MLCNANDFLASH: u32 = 8;
pub const MTD_WRITEABLE: u32 = 0x400;
pub const MTD_BIT_WRITEABLE: u32 = 0x800;
pub const MTD_NO_ERASE: u32 = 0x1000;
pub const MTD_POWERUP_LOCK: u32 = 0x2000;
pub const MTD_SLC_ON_MLC_EMULATION: u32 = 0x4000;
pub const MTD_CAP_ROM: u32 = 0;
pub const MTD_CAP_RAM: u32 = MTD_WRITEABLE | MTD_BIT_WRITEABLE | MTD_NO_ERASE;
pub const MTD_CAP_NORFLASH: u32 = MTD_WRITEABLE | MTD_BIT_WRITEABLE;
pub const MTD_CAP_NANDFLASH: u32 = MTD_WRITEABLE;
pub const MTD_CAP_NVRAM: u32 = MTD_WRITEABLE | MTD_BIT_WRITEABLE | MTD_NO_ERASE;

pub const MTD_NANDECC_OFF: u32 = 0;
pub const MTD_NANDECC_PLACE: u32 = 1;
pub const MTD_NANDECC_AUTOPLACE: u32 = 2;
pub const MTD_NANDECC_PLACEONLY: u32 = 3;
pub const MTD_NANDECC_AUTOPL_USR: u32 = 4;
pub const MTD_OTP_OFF: u32 = 0;
pub const MTD_OTP_FACTORY: u32 = 1;
pub const MTD_OTP_USER: u32 = 2;

#[repr(C)]
pub struct mtd_info_user { pub type_: u8, pub flags: u32, pub size: u32, pub erasesize: u32, pub writesize: u32, pub oobsize: u32, pub padding: u64 }
#[repr(C)]
pub struct region_info_user { pub offset: u32, pub erasesize: u32, pub numblocks: u32, pub regionindex: u32 }
#[repr(C)]
pub struct otp_info { pub start: u32, pub length: u32, pub locked: u32 }

#[repr(C)]
pub struct nand_oobinfo { pub useecc: u32, pub eccbytes: u32, pub oobfree: [[u32; 2]; 8], pub eccpos: [u32; 32] }
#[repr(C)]
pub struct nand_oobfree { pub offset: u32, pub length: u32 }
pub const MTD_MAX_OOBFREE_ENTRIES: usize = 8;
pub const MTD_MAX_ECCPOS_ENTRIES: usize = 64;
#[repr(C)]
pub struct nand_ecclayout_user {
    pub eccbytes: u32,
    pub eccpos: [u32; MTD_MAX_ECCPOS_ENTRIES],
    pub oobavail: u32,
    pub oobfree: [nand_oobfree; MTD_MAX_OOBFREE_ENTRIES],
}
#[repr(C)]
pub struct mtd_ecc_stats { pub corrected: u32, pub failed: u32, pub badblocks: u32, pub bbtblocks: u32 }

/* ioctl values depend on the platform's _IO* definitions. */
pub const MEMGETINFO: _IOR_type = _IOR!('M', 1, mtd_info_user);
pub const MEMERASE: _IOW_type = _IOW!('M', 2, erase_info_user);
pub const MEMWRITEOOB: _IOWR_type = _IOWR!('M', 3, mtd_oob_buf);
pub const MEMREADOOB: _IOWR_type = _IOWR!('M', 4, mtd_oob_buf);
pub const MEMLOCK: _IOW_type = _IOW!('M', 5, erase_info_user);
pub const MEMUNLOCK: _IOW_type = _IOW!('M', 6, erase_info_user);
pub const MEMGETREGIONCOUNT: _IOR_type = _IOR!('M', 7, i32);
pub const MEMGETREGIONINFO: _IOWR_type = _IOWR!('M', 8, region_info_user);
pub const MEMGETOOBSEL: _IOR_type = _IOR!('M', 10, nand_oobinfo);
pub const MEMGETBADBLOCK: _IOW_type = _IOW!('M', 11, i64);
pub const MEMSETBADBLOCK: _IOW_type = _IOW!('M', 12, i64);
pub const OTPSELECT: _IOR_type = _IOR!('M', 13, i32);
pub const OTPGETREGIONCOUNT: _IOW_type = _IOW!('M', 14, i32);
pub const OTPGETREGIONINFO: _IOW_type = _IOW!('M', 15, otp_info);
pub const OTPLOCK: _IOR_type = _IOR!('M', 16, otp_info);
pub const ECCGETLAYOUT: _IOR_type = _IOR!('M', 17, nand_ecclayout_user);
pub const ECCGETSTATS: _IOR_type = _IOR!('M', 18, mtd_ecc_stats);
pub const MTDFILEMODE: _IO_type = _IO!('M', 19);
pub const MEMERASE64: _IOW_type = _IOW!('M', 20, erase_info_user64);
pub const MEMWRITEOOB64: _IOWR_type = _IOWR!('M', 21, mtd_oob_buf64);
pub const MEMREADOOB64: _IOWR_type = _IOWR!('M', 22, mtd_oob_buf64);
pub const MEMISLOCKED: _IOR_type = _IOR!('M', 23, erase_info_user);
pub const MEMWRITE: _IOWR_type = _IOWR!('M', 24, mtd_write_req);
pub const OTPERASE: _IOW_type = _IOW!('M', 25, otp_info);
pub const MEMREAD: _IOWR_type = _IOWR!('M', 26, mtd_read_req);

pub const MTD_FILE_MODE_NORMAL: u32 = MTD_OTP_OFF;
pub const MTD_FILE_MODE_OTP_FACTORY: u32 = MTD_OTP_FACTORY;
pub const MTD_FILE_MODE_OTP_USER: u32 = MTD_OTP_USER;
pub const MTD_FILE_MODE_RAW: u32 = 3;

pub unsafe fn mtd_type_is_nand_user(mtd: *const mtd_info_user) -> bool {
    (*mtd).type_ == MTD_NANDFLASH as u8 || (*mtd).type_ == MTD_MLCNANDFLASH as u8
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
