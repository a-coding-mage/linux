/* SPDX-License-Identifier: GPL-2.0-or-later */
/*  Architecture specific parts of HP's STI (framebuffer) driver.
 *  Structures are HP-UX compatible for XFree86 usage.
 * 
 *    Linux/PA-RISC Project (http://www.parisc-linux.org/)
 *    Copyright (C) 2001 Helge Deller (deller a parisc-linux org)
 */

/* upper 32 bits of graphics id (HP/UX identifier) */

pub const GRFGATOR: u32 = 8;
pub const S9000_ID_S300: u32 = 9;
pub const GRFBOBCAT: u32 = 9;
pub const GRFCATSEYE: u32 = 9;
pub const S9000_ID_98720: u32 = 10;
pub const GRFRBOX: u32 = 10;
pub const S9000_ID_98550: u32 = 11;
pub const GRFFIREEYE: u32 = 11;
pub const S9000_ID_A1096A: u32 = 12;
pub const GRFHYPERION: u32 = 12;
pub const S9000_ID_FRI: u32 = 13;
pub const S9000_ID_98730: u32 = 14;
pub const GRFDAVINCI: u32 = 14;
pub const S9000_ID_98705: u32 = 0x26C08070; /* Tigershark */
pub const S9000_ID_98736: u32 = 0x26D148AB;
pub const S9000_ID_A1659A: u32 = 0x26D1482A; /* CRX 8 plane color (=ELK) */
pub const S9000_ID_ELK: u32 = S9000_ID_A1659A;
pub const S9000_ID_A1439A: u32 = 0x26D148EE; /* CRX24 = CRX+ (24-plane color) */
pub const S9000_ID_A1924A: u32 = 0x26D1488C; /* GRX gray-scale */
pub const S9000_ID_ELM: u32 = S9000_ID_A1924A;
pub const S9000_ID_98765: u32 = 0x27480DEF;
pub const S9000_ID_ELK_768: u32 = 0x27482101;
pub const S9000_ID_STINGER: u32 = 0x27A4A402;
pub const S9000_ID_TIMBER: u32 = 0x27F12392; /* Bushmaster (710) Graphics */
pub const S9000_ID_TOMCAT: u32 = 0x27FCCB6D; /* dual-headed ELK (Dual CRX) */
pub const S9000_ID_ARTIST: u32 = 0x2B4DED6D; /* Artist (Gecko/712 & 715) onboard Graphics */
pub const S9000_ID_HCRX: u32 = 0x2BCB015A; /* Hyperdrive/Hyperbowl (A4071A) Graphics */
pub const CRX24_OVERLAY_PLANES: u32 = 0x920825AA; /* Overlay planes on CRX24 */

pub const CRT_ID_ELK_1024: u32 = S9000_ID_ELK_768; /* Elk 1024x768  CRX */
pub const CRT_ID_ELK_1280: u32 = S9000_ID_A1659A; /* Elk 1280x1024 CRX */
pub const CRT_ID_ELK_1024DB: u32 = 0x27849CA5; /* Elk 1024x768 double buffer */
pub const CRT_ID_ELK_GS: u32 = S9000_ID_A1924A; /* Elk 1280x1024 GreyScale */
pub const CRT_ID_CRX24: u32 = S9000_ID_A1439A; /* Piranha */
pub const CRT_ID_VISUALIZE_EG: u32 = 0x2D08C0A7; /* Graffiti, A4450A (built-in B132+/B160L) */
pub const CRT_ID_THUNDER: u32 = 0x2F23E5FC; /* Thunder 1 VISUALIZE 48 */
pub const CRT_ID_THUNDER2: u32 = 0x2F8D570E; /* Thunder 2 VISUALIZE 48 XP */
pub const CRT_ID_HCRX: u32 = S9000_ID_HCRX; /* Hyperdrive HCRX */
pub const CRT_ID_CRX48Z: u32 = S9000_ID_STINGER; /* Stinger */
pub const CRT_ID_DUAL_CRX: u32 = S9000_ID_TOMCAT; /* Tomcat */
pub const CRT_ID_PVRX: u32 = S9000_ID_98705; /* Tigershark */
pub const CRT_ID_TIMBER: u32 = S9000_ID_TIMBER; /* Timber (710 builtin) */
pub const CRT_ID_TVRX: u32 = S9000_ID_98765; /* TVRX (gto/falcon) */
pub const CRT_ID_ARTIST: u32 = S9000_ID_ARTIST; /* Artist */
pub const CRT_ID_SUMMIT: u32 = 0x2FC1066B; /* Summit FX2, FX4, FX6 ... */
pub const CRT_ID_LEGO: u32 = 0x35ACDA30; /* Lego FX5, FX10 ... */
pub const CRT_ID_PINNACLE: u32 = 0x35ACDA16; /* Pinnacle FXe */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
