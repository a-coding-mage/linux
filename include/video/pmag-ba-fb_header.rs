/*
 *	linux/include/video/pmag-ba-fb.h
 *
 *	TURBOchannel PMAG-BA Color Frame Buffer (CFB) card support,
 *	Copyright (C) 1999, 2000, 2001 by
 *	Michael Engel <engel@unix-ag.org>,
 *	Karsten Merker <merker@linuxtag.org>
 *	Copyright (c) 2005  Maciej W. Rozycki
 *
 *	This file is subject to the terms and conditions of the GNU General
 *	Public License.  See the file COPYING in the main directory of this
 *	archive for more details.
 */

/* IOmem resource offsets.  */
pub const PMAG_BA_FBMEM: u32 = 0x000000; /* frame buffer */
pub const PMAG_BA_BT459: u32 = 0x200000; /* Bt459 RAMDAC */
pub const PMAG_BA_IRQ: u32 = 0x300000; /* IRQ acknowledge */
pub const PMAG_BA_ROM: u32 = 0x380000; /* REX option ROM */
pub const PMAG_BA_BT438: u32 = 0x380000; /* Bt438 clock chip reset */
pub const PMAG_BA_SIZE: u32 = 0x400000; /* address space size */

/* Bt459 register offsets, byte-wide registers.  */
pub const BT459_ADDR_LO: u32 = 0x0; /* address low */
pub const BT459_ADDR_HI: u32 = 0x4; /* address high */
pub const BT459_DATA: u32 = 0x8; /* data window register */
pub const BT459_CMAP: u32 = 0xc; /* color map window register */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
