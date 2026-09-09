/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Linux driver for Disk-On-Chip devices. */

// C dependencies: linux/mtd/mtd.h and linux/mutex.h provide `mtd_info` and
// `mutex`, while the I/O primitives are supplied by the target kernel.

pub const DoC_Sig1: usize = 0;
pub const DoC_Sig2: usize = 1;

pub const DoC_ChipID: usize = 0x1000;
pub const DoC_DOCStatus: usize = 0x1001;
pub const DoC_DOCControl: usize = 0x1002;
pub const DoC_FloorSelect: usize = 0x1003;
pub const DoC_CDSNControl: usize = 0x1004;
pub const DoC_CDSNDeviceSelect: usize = 0x1005;
pub const DoC_ECCConf: usize = 0x1006;
pub const DoC_2k_ECCStatus: usize = 0x1007;
pub const DoC_CDSNSlowIO: usize = 0x100d;
pub const DoC_ECCSyndrome0: usize = 0x1010;
pub const DoC_ECCSyndrome1: usize = 0x1011;
pub const DoC_ECCSyndrome2: usize = 0x1012;
pub const DoC_ECCSyndrome3: usize = 0x1013;
pub const DoC_ECCSyndrome4: usize = 0x1014;
pub const DoC_ECCSyndrome5: usize = 0x1015;
pub const DoC_AliasResolution: usize = 0x101b;
pub const DoC_ConfigInput: usize = 0x101c;
pub const DoC_ReadPipeInit: usize = 0x101d;
pub const DoC_WritePipeTerm: usize = 0x101e;
pub const DoC_LastDataRead: usize = 0x101f;
pub const DoC_NOP: usize = 0x1020;
pub const DoC_Mil_CDSN_IO: usize = 0x0800;
pub const DoC_2k_CDSN_IO: usize = 0x1800;

pub const DoC_Mplus_NOP: usize = 0x1002;
pub const DoC_Mplus_AliasResolution: usize = 0x1004;
pub const DoC_Mplus_DOCControl: usize = 0x1006;
pub const DoC_Mplus_AccessStatus: usize = 0x1008;
pub const DoC_Mplus_DeviceSelect: usize = 0x1008;
pub const DoC_Mplus_Configuration: usize = 0x100a;
pub const DoC_Mplus_OutputControl: usize = 0x100c;
pub const DoC_Mplus_FlashControl: usize = 0x1020;
pub const DoC_Mplus_FlashSelect: usize = 0x1022;
pub const DoC_Mplus_FlashCmd: usize = 0x1024;
pub const DoC_Mplus_FlashAddress: usize = 0x1026;
pub const DoC_Mplus_FlashData0: usize = 0x1028;
pub const DoC_Mplus_FlashData1: usize = 0x1029;
pub const DoC_Mplus_ReadPipeInit: usize = 0x102a;
pub const DoC_Mplus_LastDataRead: usize = 0x102c;
pub const DoC_Mplus_LastDataRead1: usize = 0x102d;
pub const DoC_Mplus_WritePipeTerm: usize = 0x102e;
pub const DoC_Mplus_ECCSyndrome0: usize = 0x1040;
pub const DoC_Mplus_ECCSyndrome1: usize = 0x1041;
pub const DoC_Mplus_ECCSyndrome2: usize = 0x1042;
pub const DoC_Mplus_ECCSyndrome3: usize = 0x1043;
pub const DoC_Mplus_ECCSyndrome4: usize = 0x1044;
pub const DoC_Mplus_ECCSyndrome5: usize = 0x1045;
pub const DoC_Mplus_ECCConf: usize = 0x1046;
pub const DoC_Mplus_Toggle: usize = 0x1046;
pub const DoC_Mplus_DownloadStatus: usize = 0x1074;
pub const DoC_Mplus_CtrlConfirm: usize = 0x1076;
pub const DoC_Mplus_Power: usize = 0x1fff;

pub const DOC_IOREMAP_LEN: usize = 0x2000;
// On ARM DOC_IOREMAP_LEN is 0x8000 and on PPC it is 0x4000. The original
// architecture-specific ReadDOC_/WriteDOC_ definitions use kernel I/O APIs.
// On x86, USE_MEMCPY is defined.

pub const DOC_MODE_RESET: u8 = 0;
pub const DOC_MODE_NORMAL: u8 = 1;
pub const DOC_MODE_RESERVED1: u8 = 2;
pub const DOC_MODE_RESERVED2: u8 = 3;
pub const DOC_MODE_CLR_ERR: u8 = 0x80;
pub const DOC_MODE_RST_LAT: u8 = 0x10;
pub const DOC_MODE_BDECT: u8 = 0x08;
pub const DOC_MODE_MDWREN: u8 = 0x04;

pub const DOC_ChipID_Doc2k: u8 = 0x20;
pub const DOC_ChipID_Doc2kTSOP: u8 = 0x21;
pub const DOC_ChipID_DocMil: u8 = 0x30;
pub const DOC_ChipID_DocMilPlus32: u8 = 0x40;
pub const DOC_ChipID_DocMilPlus16: u8 = 0x41;
pub const CDSN_CTRL_FR_B: u8 = 0x80;
pub const CDSN_CTRL_FR_B0: u8 = 0x40;
pub const CDSN_CTRL_FR_B1: u8 = 0x80;
pub const CDSN_CTRL_ECC_IO: u8 = 0x20;
pub const CDSN_CTRL_FLASH_IO: u8 = 0x10;
pub const CDSN_CTRL_WP: u8 = 0x08;
pub const CDSN_CTRL_ALE: u8 = 0x04;
pub const CDSN_CTRL_CLE: u8 = 0x02;
pub const CDSN_CTRL_CE: u8 = 0x01;
pub const DOC_ECC_RESET: u8 = 0;
pub const DOC_ECC_ERROR: u8 = 0x80;
pub const DOC_ECC_RW: u8 = 0x20;
pub const DOC_ECC__EN: u8 = 0x08;
pub const DOC_TOGGLE_BIT: u8 = 0x04;
pub const DOC_ECC_RESV: u8 = 0x02;
pub const DOC_ECC_IGNORE: u8 = 0x01;
pub const DOC_FLASH_CE: u8 = 0x80;
pub const DOC_FLASH_WP: u8 = 0x40;
pub const DOC_FLASH_BANK: u8 = 0x02;
pub const DOC_ECC_EN: u8 = DOC_ECC__EN | DOC_ECC_RESV;
pub const DOC_ECC_DIS: u8 = DOC_ECC_RESV;

#[repr(C)]
pub struct Nand {
    pub floor: i8,
    pub chip: i8,
    pub curadr: libc::c_ulong,
    pub curmode: u8,
}

pub const MAX_FLOORS: usize = 4;
pub const MAX_CHIPS: usize = 4;
pub const MAX_FLOORS_MIL: usize = 1;
pub const MAX_CHIPS_MIL: usize = 1;
pub const MAX_FLOORS_MPLUS: usize = 2;
pub const MAX_CHIPS_MPLUS: usize = 1;
pub const ADDR_COLUMN: usize = 1;
pub const ADDR_PAGE: usize = 2;
pub const ADDR_COLUMN_PAGE: usize = 3;

#[repr(C)]
pub struct DiskOnChip {
    pub physadr: libc::c_ulong,
    pub virtadr: *mut core::ffi::c_void,
    pub totlen: libc::c_ulong,
    pub ChipID: u8,
    pub ioreg: libc::c_int,
    pub mfr: libc::c_ulong,
    pub id: libc::c_ulong,
    pub chipshift: libc::c_int,
    pub page256: i8,
    pub pageadrlen: i8,
    pub interleave: i8,
    pub erasesize: libc::c_ulong,
    pub curfloor: libc::c_int,
    pub curchip: libc::c_int,
    pub numchips: libc::c_int,
    pub chips: *mut Nand,
    pub nextdoc: *mut mtd_info,
    pub lock: mutex,
}

extern "C" {
    pub fn doc_decode_ecc(sector: *mut u8, ecc1: *mut u8);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
