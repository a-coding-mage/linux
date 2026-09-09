/*
 *      linux/drivers/video/maxinefb.h
 *
 *      DECstation 5000/xx onboard framebuffer support, Copyright (C) 1999 by
 *      Michael Engel <engel@unix-ag.org> and Karsten Merker <merker@guug.de>
 *      This file is subject to the terms and conditions of the GNU General
 *      Public License.  See the file COPYING in the main directory of this
 *      archive for more details.
 */

// C dependency: <asm/addrspace.h>.  CKSEG1ADDR maps a physical address into
// the uncached, unmapped MIPS KSEG1 address space.
const fn ckseg1addr(address: u64) -> u64 {
    address | 0xffff_ffff_a000_0000
}

/*
 * IMS332 video controller register base address
 */
pub const MAXINEFB_IMS332_ADDRESS: u64 = ckseg1addr(0x1c14_0000);

/*
 * Begin of DECstation 5000/xx onboard framebuffer memory, default resolution
 * is 1024x768x8
 */
pub const DS5000_xx_ONBOARD_FBMEM_START: u64 = ckseg1addr(0x0a00_0000);

/*
 *      The IMS 332 video controller used in the DECstation 5000/xx series
 *      uses 32 bits wide registers; the following defines declare the
 *      register numbers, to get the real offset, these have to be multiplied
 *      by four.
 */

pub const IMS332_REG_CURSOR_RAM: u32 = 0x200; /* hardware cursor bitmap */

/*
 * The color palette entries have the form 0x00BBGGRR
 */
pub const IMS332_REG_COLOR_PALETTE: u32 = 0x100; /* color palette, 256 entries */
pub const IMS332_REG_CURSOR_COLOR_PALETTE: u32 = 0x0a1; /* cursor color palette, */
                                                        /* 3 entries             */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
