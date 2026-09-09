/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * This file contains volume label definitions for DASD devices.
 *
 * Copyright IBM Corp. 2005
 *
 * Author(s): Volker Sameske <sameske@de.ibm.com>
 *
 */

use core::ffi::c_char;

#[repr(C, packed)]
pub struct vtoc_ttr {
    pub tt: u16,
    pub r: u8,
}

#[repr(C, packed)]
pub struct vtoc_cchhb {
    pub cc: u16,
    pub hh: u16,
    pub b: u8,
}

#[repr(C, packed)]
pub struct vtoc_cchh {
    pub cc: u16,
    pub hh: u16,
}

#[repr(C, packed)]
pub struct vtoc_labeldate {
    pub year: u8,
    pub day: u16,
}

#[repr(C, packed)]
pub struct vtoc_volume_label_cdl {
    pub volkey: [c_char; 4],
    pub vollbl: [c_char; 4],
    pub volid: [c_char; 6],
    pub security: u8,
    pub vtoc: vtoc_cchhb,
    pub res1: [c_char; 5],
    pub cisize: [c_char; 4],
    pub blkperci: [c_char; 4],
    pub labperci: [c_char; 4],
    pub res2: [c_char; 4],
    pub lvtoc: [c_char; 14],
    pub res3: [c_char; 29],
}

#[repr(C, packed)]
pub struct vtoc_volume_label_ldl {
    pub vollbl: [c_char; 4],
    pub volid: [c_char; 6],
    pub res3: [c_char; 69],
    pub ldl_version: c_char,
    pub formatted_blocks: u64,
}

#[repr(C, packed)]
pub struct vtoc_extent {
    pub typeind: u8,
    pub seqno: u8,
    pub llimit: vtoc_cchh,
    pub ulimit: vtoc_cchh,
}

#[repr(C, packed)]
pub struct vtoc_dev_const {
    pub DS4DSCYL: u16,
    pub DS4DSTRK: u16,
    pub DS4DEVTK: u16,
    pub DS4DEVI: u8,
    pub DS4DEVL: u8,
    pub DS4DEVK: u8,
    pub DS4DEVFG: u8,
    pub DS4DEVTL: u16,
    pub DS4DEVDT: u8,
    pub DS4DEVDB: u8,
}

#[repr(C, packed)]
pub struct vtoc_format1_label {
    pub DS1DSNAM: [c_char; 44],
    pub DS1FMTID: u8,
    pub DS1DSSN: [c_char; 6],
    pub DS1VOLSQ: u16,
    pub DS1CREDT: vtoc_labeldate,
    pub DS1EXPDT: vtoc_labeldate,
    pub DS1NOEPV: u8,
    pub DS1NOBDB: u8,
    pub DS1FLAG1: u8,
    pub DS1SYSCD: [c_char; 13],
    pub DS1REFD: vtoc_labeldate,
    pub DS1SMSFG: u8,
    pub DS1SCXTF: u8,
    pub DS1SCXTV: u16,
    pub DS1DSRG1: u8,
    pub DS1DSRG2: u8,
    pub DS1RECFM: u8,
    pub DS1OPTCD: u8,
    pub DS1BLKL: u16,
    pub DS1LRECL: u16,
    pub DS1KEYL: u8,
    pub DS1RKP: u16,
    pub DS1DSIND: u8,
    pub DS1SCAL1: u8,
    pub DS1SCAL3: [c_char; 3],
    pub DS1LSTAR: vtoc_ttr,
    pub DS1TRBAL: u16,
    pub res1: u16,
    pub DS1EXT1: vtoc_extent,
    pub DS1EXT2: vtoc_extent,
    pub DS1EXT3: vtoc_extent,
    pub DS1PTRDS: vtoc_cchhb,
}

#[repr(C, packed)]
pub struct vtoc_format4_label {
    pub DS4KEYCD: [c_char; 44],
    pub DS4IDFMT: u8,
    pub DS4HPCHR: vtoc_cchhb,
    pub DS4DSREC: u16,
    pub DS4HCCHH: vtoc_cchh,
    pub DS4NOATK: u16,
    pub DS4VTOCI: u8,
    pub DS4NOEXT: u8,
    pub DS4SMSFG: u8,
    pub DS4DEVAC: u8,
    pub DS4DEVCT: vtoc_dev_const,
    pub DS4AMTIM: [c_char; 8],
    pub DS4AMCAT: [c_char; 3],
    pub DS4R2TIM: [c_char; 8],
    pub res1: [c_char; 5],
    pub DS4F6PTR: [c_char; 5],
    pub DS4VTOCE: vtoc_extent,
    pub res2: [c_char; 10],
    pub DS4EFLVL: u8,
    pub DS4EFPTR: vtoc_cchhb,
    pub res3: c_char,
    pub DS4DCYL: u32,
    pub res4: [c_char; 2],
    pub DS4DEVF2: u8,
    pub res5: c_char,
}

#[repr(C, packed)]
pub struct vtoc_ds5ext {
    pub t: u16,
    pub fc: u16,
    pub ft: u8,
}

#[repr(C, packed)]
pub struct vtoc_format5_label {
    pub DS5KEYID: [c_char; 4],
    pub DS5AVEXT: vtoc_ds5ext,
    pub DS5EXTAV: [vtoc_ds5ext; 7],
    pub DS5FMTID: u8,
    pub DS5MAVET: [vtoc_ds5ext; 18],
    pub DS5PTRDS: vtoc_cchhb,
}

#[repr(C, packed)]
pub struct vtoc_ds7ext {
    pub a: u32,
    pub b: u32,
}

#[repr(C, packed)]
pub struct vtoc_format7_label {
    pub DS7KEYID: [c_char; 4],
    pub DS7EXTNT: [vtoc_ds7ext; 5],
    pub DS7FMTID: u8,
    pub DS7ADEXT: [vtoc_ds7ext; 11],
    pub res1: [c_char; 2],
    pub DS7PTRDS: vtoc_cchhb,
}

#[repr(C, packed)]
pub struct vtoc_cms_label {
    pub label_id: [u8; 4],
    pub vol_id: [u8; 6],
    pub version_id: u16,
    pub block_size: u32,
    pub origin_ptr: u32,
    pub usable_count: u32,
    pub formatted_count: u32,
    pub block_count: u32,
    pub used_count: u32,
    pub fst_size: u32,
    pub fst_count: u32,
    pub format_date: [u8; 6],
    pub reserved1: [u8; 2],
    pub disk_offset: u32,
    pub map_block: u32,
    pub hblk_disp: u32,
    pub user_disp: u32,
    pub reserved2: [u8; 4],
    pub segment_name: [u8; 8],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
