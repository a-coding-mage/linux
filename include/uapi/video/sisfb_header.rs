/* SPDX-License-Identifier: GPL-2.0+ WITH Linux-syscall-note */
/*
 * sisfb.h - definitions for the SiS framebuffer driver
 *
 * Copyright (C) 2001-2005 by Thomas Winischhofer, Vienna, Austria.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the named License,
 * or any later version.
 */

/* C dependencies: linux/types.h and asm/ioctl.h provide the integer types
 * and _IOR/_IOW/_IOWR ioctl encodings used below. */

pub const CRT2_DEFAULT: u32 = 0x00000001;
pub const CRT2_LCD: u32 = 0x00000002;
pub const CRT2_TV: u32 = 0x00000004;
pub const CRT2_VGA: u32 = 0x00000008;
pub const TV_NTSC: u32 = 0x00000010;
pub const TV_PAL: u32 = 0x00000020;
pub const TV_HIVISION: u32 = 0x00000040;
pub const TV_YPBPR: u32 = 0x00000080;
pub const TV_AVIDEO: u32 = 0x00000100;
pub const TV_SVIDEO: u32 = 0x00000200;
pub const TV_SCART: u32 = 0x00000400;
pub const TV_PALM: u32 = 0x00001000;
pub const TV_PALN: u32 = 0x00002000;
pub const TV_NTSCJ: u32 = 0x00001000;
pub const TV_CHSCART: u32 = 0x00008000;
pub const TV_CHYPBPR525I: u32 = 0x00010000;
pub const CRT1_VGA: u32 = 0x00000000;
pub const CRT1_LCDA: u32 = 0x00020000;
pub const VGA2_CONNECTED: u32 = 0x00040000;
pub const VB_DISPTYPE_CRT1: u32 = 0x00080000;
pub const VB_SINGLE_MODE: u32 = 0x20000000;
pub const VB_MIRROR_MODE: u32 = 0x40000000;
pub const VB_DUALVIEW_MODE: u32 = 0x80000000;

pub const CRT2_ENABLE: u32 = CRT2_LCD | CRT2_TV | CRT2_VGA;
pub const TV_STANDARD: u32 = TV_NTSC | TV_PAL | TV_PALM | TV_PALN | TV_NTSCJ;
pub const TV_INTERFACE: u32 = TV_AVIDEO | TV_SVIDEO | TV_SCART | TV_HIVISION | TV_YPBPR | TV_CHSCART | TV_CHYPBPR525I;
pub const TV_YPBPR525I: u32 = TV_NTSC;
pub const TV_YPBPR525P: u32 = TV_PAL;
pub const TV_YPBPR750P: u32 = TV_PALM;
pub const TV_YPBPR1080I: u32 = TV_PALN;
pub const TV_YPBPRALL: u32 = TV_YPBPR525I | TV_YPBPR525P | TV_YPBPR750P | TV_YPBPR1080I;
pub const VB_DISPTYPE_DISP2: u32 = CRT2_ENABLE;
pub const VB_DISPTYPE_CRT2: u32 = CRT2_ENABLE;
pub const VB_DISPTYPE_DISP1: u32 = VB_DISPTYPE_CRT1;
pub const VB_DISPMODE_SINGLE: u32 = VB_SINGLE_MODE;
pub const VB_DISPMODE_MIRROR: u32 = VB_MIRROR_MODE;
pub const VB_DISPMODE_DUAL: u32 = VB_DUALVIEW_MODE;
/* C source refers to externally supplied SINGLE_MODE/MIRROR_MODE/DUALVIEW_MODE. */
pub const VB_DISPLAY_MODE: u32 = SINGLE_MODE | MIRROR_MODE | DUALVIEW_MODE;

pub const SISFB_ID: u32 = 0x53495346;

#[repr(C)]
pub struct sisfb_info {
    pub sisfb_id: u32,
    pub chip_id: u32,
    pub memory: u32,
    pub heapstart: u32,
    pub fbvidmode: u8,
    pub sisfb_version: u8,
    pub sisfb_revision: u8,
    pub sisfb_patchlevel: u8,
    pub sisfb_caps: u8,
    pub sisfb_tqlen: u32,
    pub sisfb_pcibus: u32,
    pub sisfb_pcislot: u32,
    pub sisfb_pcifunc: u32,
    pub sisfb_lcdpdc: u8,
    pub sisfb_lcda: u8,
    pub sisfb_vbflags: u32,
    pub sisfb_currentvbflags: u32,
    pub sisfb_scalelcd: u32,
    pub sisfb_specialtiming: u32,
    pub sisfb_haveemi: u8,
    pub sisfb_emi30: u8,
    pub sisfb_emi31: u8,
    pub sisfb_emi32: u8,
    pub sisfb_emi33: u8,
    pub sisfb_haveemilcd: u8,
    pub sisfb_lcdpdca: u8,
    pub sisfb_tvxpos: u16,
    pub sisfb_tvypos: u16,
    pub sisfb_heapsize: u32,
    pub sisfb_videooffset: u32,
    pub sisfb_curfstn: u32,
    pub sisfb_curdstn: u32,
    pub sisfb_pci_vendor: u16,
    pub sisfb_vbflags2: u32,
    pub sisfb_can_post: u8,
    pub sisfb_card_posted: u8,
    pub sisfb_was_boot_device: u8,
    pub reserved: [u8; 183],
}

pub const SISFB_CMD_GETVBFLAGS: u32 = 0x55AA0001;
pub const SISFB_CMD_SWITCHCRT1: u32 = 0x55AA0010;
pub const SISFB_CMD_ERR_OK: u32 = 0x80000000;
pub const SISFB_CMD_ERR_LOCKED: u32 = 0x80000001;
pub const SISFB_CMD_ERR_EARLY: u32 = 0x80000002;
pub const SISFB_CMD_ERR_NOVB: u32 = 0x80000003;
pub const SISFB_CMD_ERR_NOCRT2: u32 = 0x80000004;
pub const SISFB_CMD_ERR_UNKNOWN: u32 = 0x8000ffff;
pub const SISFB_CMD_ERR_OTHER: u32 = 0x80010000;

#[repr(C)]
pub struct sisfb_cmd {
    pub sisfb_cmd: u32,
    pub sisfb_arg: [u32; 16],
    pub sisfb_result: [u32; 4],
}

/* ioctl encodings remain dependent on the external asm/ioctl.h definitions. */
pub const SISFB_GET_INFO_SIZE: u32 = _IOR(0xF3, 0x00, u32);
pub const SISFB_GET_INFO: u32 = _IOR(0xF3, 0x01, sisfb_info);
pub const SISFB_GET_VBRSTATUS: u32 = _IOR(0xF3, 0x02, u32);
pub const SISFB_GET_AUTOMAXIMIZE: u32 = _IOR(0xF3, 0x03, u32);
pub const SISFB_SET_AUTOMAXIMIZE: u32 = _IOW(0xF3, 0x03, u32);
pub const SISFB_GET_TVPOSOFFSET: u32 = _IOR(0xF3, 0x04, u32);
pub const SISFB_SET_TVPOSOFFSET: u32 = _IOW(0xF3, 0x04, u32);
pub const SISFB_COMMAND: u32 = _IOWR(0xF3, 0x05, sisfb_cmd);
pub const SISFB_SET_LOCK: u32 = _IOW(0xF3, 0x06, u32);
pub const SISFB_GET_INFO_OLD: u32 = _IOR(b'n', 0xF8, u32);
pub const SISFB_GET_VBRSTATUS_OLD: u32 = _IOR(b'n', 0xF9, u32);
pub const SISFB_GET_AUTOMAXIMIZE_OLD: u32 = _IOR(b'n', 0xFA, u32);
pub const SISFB_SET_AUTOMAXIMIZE_OLD: u32 = _IOW(b'n', 0xFA, u32);

#[repr(C)]
pub struct sis_memreq {
    pub offset: u32,
    pub size: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
