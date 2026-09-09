/*
 *	linux/include/video/pmagb-b-fb.h
 *
 *	TURBOchannel PMAGB-B Smart Frame Buffer (SFB) card support,
 *	Copyright (C) 1999, 2000, 2001 by
 *	Michael Engel <engel@unix-ag.org> and
 *	Karsten Merker <merker@linuxtag.org>
 *	Copyright (c) 2005  Maciej W. Rozycki
 *
 *	This file is subject to the terms and conditions of the GNU General
 *	Public License.  See the file COPYING in the main directory of this
 *	archive for more details.
 */

/* IOmem resource offsets.  */
pub const PMAGB_B_ROM: u32 = 0x000000; /* REX option ROM */
pub const PMAGB_B_SFB: u32 = 0x100000; /* SFB ASIC */
pub const PMAGB_B_GP0: u32 = 0x140000; /* general purpose output 0 */
pub const PMAGB_B_GP1: u32 = 0x180000; /* general purpose output 1 */
pub const PMAGB_B_BT459: u32 = 0x1c0000; /* Bt459 RAMDAC */
pub const PMAGB_B_FBMEM: u32 = 0x200000; /* frame buffer */
pub const PMAGB_B_SIZE: u32 = 0x400000; /* address space size */

/* IOmem register offsets.  */
pub const SFB_REG_VID_HOR: u32 = 0x64; /* video horizontal setup */
pub const SFB_REG_VID_VER: u32 = 0x68; /* video vertical setup */
pub const SFB_REG_VID_BASE: u32 = 0x6c; /* video base address */
pub const SFB_REG_TCCLK_COUNT: u32 = 0x78; /* TURBOchannel clock count */
pub const SFB_REG_VIDCLK_COUNT: u32 = 0x7c; /* video clock count */

/* Video horizontal setup register constants.  All bits are r/w.  */
pub const SFB_VID_HOR_BP_SHIFT: u32 = 0x15; /* back porch */
pub const SFB_VID_HOR_BP_MASK: u32 = 0x7f;
pub const SFB_VID_HOR_SYN_SHIFT: u32 = 0x0e; /* sync pulse */
pub const SFB_VID_HOR_SYN_MASK: u32 = 0x7f;
pub const SFB_VID_HOR_FP_SHIFT: u32 = 0x09; /* front porch */
pub const SFB_VID_HOR_FP_MASK: u32 = 0x1f;
pub const SFB_VID_HOR_PIX_SHIFT: u32 = 0x00; /* active video */
pub const SFB_VID_HOR_PIX_MASK: u32 = 0x1ff;

/* Video vertical setup register constants.  All bits are r/w.  */
pub const SFB_VID_VER_BP_SHIFT: u32 = 0x16; /* back porch */
pub const SFB_VID_VER_BP_MASK: u32 = 0x3f;
pub const SFB_VID_VER_SYN_SHIFT: u32 = 0x10; /* sync pulse */
pub const SFB_VID_VER_SYN_MASK: u32 = 0x3f;
pub const SFB_VID_VER_FP_SHIFT: u32 = 0x0b; /* front porch */
pub const SFB_VID_VER_FP_MASK: u32 = 0x1f;
pub const SFB_VID_VER_SL_SHIFT: u32 = 0x00; /* active scan lines */
pub const SFB_VID_VER_SL_MASK: u32 = 0x7ff;

/* Video base address register constants.  All bits are r/w.  */
pub const SFB_VID_BASE_MASK: u32 = 0x1ff; /* video base row address */

/* Bt459 register offsets, byte-wide registers.  */
pub const BT459_ADDR_LO: u32 = 0x0; /* address low */
pub const BT459_ADDR_HI: u32 = 0x4; /* address high */
pub const BT459_DATA: u32 = 0x8; /* data window register */
pub const BT459_CMAP: u32 = 0xc; /* color map window register */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
