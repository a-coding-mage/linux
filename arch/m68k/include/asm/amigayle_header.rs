/*
** asm-m68k/amigayle.h -- This header defines the registers of the gayle chip
**                        found on the Amiga 1200
**                        This information was found by disassembling card.resource,
**                        so the definitions may not be 100% correct
**                        anyone has an official doc ?
**
** Copyright 1997 by Alain Malek
**
** This file is subject to the terms and conditions of the GNU General Public
** License.  See the file COPYING in the main directory of this archive
** for more details.
**
** Created: 11/28/97 by Alain Malek
*/

/* Dependency supplied by asm/amigahw.h: zTwoBase. */

/* memory layout */

pub const GAYLE_RAM: usize = 0x600000 + zTwoBase;
pub const GAYLE_RAMSIZE: usize = 0x400000;
pub const GAYLE_ATTRIBUTE: usize = 0xa00000 + zTwoBase;
pub const GAYLE_ATTRIBUTESIZE: usize = 0x020000;
pub const GAYLE_IO: usize = 0xa20000 + zTwoBase; /* 16bit and even 8bit registers */
pub const GAYLE_IOSIZE: usize = 0x010000;
pub const GAYLE_IO_8BITODD: usize = 0xa30000 + zTwoBase; /* odd 8bit registers */

/* offset for accessing odd IO registers */
pub const GAYLE_ODD: usize = GAYLE_IO_8BITODD - GAYLE_IO - 1;

/* GAYLE registers */

#[repr(C)]
pub struct GAYLE {
    pub cardstatus: u8,
    pub pad0: [u8; 0x1000 - 1],

    pub intreq: u8,
    pub pad1: [u8; 0x1000 - 1],

    pub inten: u8,
    pub pad2: [u8; 0x1000 - 1],

    pub config: u8,
    pub pad3: [u8; 0x1000 - 1],
}

pub const GAYLE_ADDRESS: usize = 0xda8000; /* gayle main registers base address */

pub const GAYLE_RESET: usize = 0xa40000; /* write 0x00 to start reset,
                                             read 1 byte to stop reset */

pub const gayle: *mut GAYLE = (zTwoBase + GAYLE_ADDRESS) as *mut GAYLE;
pub const gayle_reset: *mut u8 = (zTwoBase + GAYLE_RESET) as *mut u8;

pub const gayle_attribute: *mut u8 = GAYLE_ATTRIBUTE as *mut u8;

/*
#if 0
#define gayle_inb(a) readb( GAYLE_IO+(a)+(((a)&1)*GAYLE_ODD) )
#define gayle_outb(v,a) writeb( v, GAYLE_IO+(a)+(((a)&1)*GAYLE_ODD) )
#define gayle_inw(a) readw( GAYLE_IO+(a) )
#define gayle_outw(v,a) writew( v, GAYLE_IO+(a) )
#endif
*/

/* GAYLE_CARDSTATUS bit def */

pub const GAYLE_CS_CCDET: u8 = 0x40; /* credit card detect */
pub const GAYLE_CS_BVD1: u8 = 0x20; /* battery voltage detect 1 */
pub const GAYLE_CS_SC: u8 = 0x20; /* credit card status change */
pub const GAYLE_CS_BVD2: u8 = 0x10; /* battery voltage detect 2 */
pub const GAYLE_CS_DA: u8 = 0x10; /* digital audio */
pub const GAYLE_CS_WR: u8 = 0x08; /* write enable (1 == enabled) */
pub const GAYLE_CS_BSY: u8 = 0x04; /* credit card busy */
pub const GAYLE_CS_IRQ: u8 = 0x04; /* interrupt request */

/* GAYLE_IRQ bit def */

pub const GAYLE_IRQ_IDE: u8 = 0x80;
pub const GAYLE_IRQ_CCDET: u8 = 0x40;
pub const GAYLE_IRQ_BVD1: u8 = 0x20;
pub const GAYLE_IRQ_SC: u8 = 0x20;
pub const GAYLE_IRQ_BVD2: u8 = 0x10;
pub const GAYLE_IRQ_DA: u8 = 0x10;
pub const GAYLE_IRQ_WR: u8 = 0x08;
pub const GAYLE_IRQ_BSY: u8 = 0x04;
pub const GAYLE_IRQ_IRQ: u8 = 0x04;
pub const GAYLE_IRQ_IDEACK1: u8 = 0x02;
pub const GAYLE_IRQ_IDEACK0: u8 = 0x01;

/* GAYLE_CONFIG bit def
   (bit 0-1 for program voltage, bit 2-3 for access speed */

pub const GAYLE_CFG_0V: u8 = 0x00;
pub const GAYLE_CFG_5V: u8 = 0x01;
pub const GAYLE_CFG_12V: u8 = 0x02;

pub const GAYLE_CFG_100NS: u8 = 0x08;
pub const GAYLE_CFG_150NS: u8 = 0x04;
pub const GAYLE_CFG_250NS: u8 = 0x00;
pub const GAYLE_CFG_720NS: u8 = 0x0c;

#[repr(C)]
pub struct gayle_ide_platform_data {
    pub base: libc::c_ulong,
    pub irqport: libc::c_ulong,
    pub explicit_ack: libc::c_int, /* A1200 IDE needs explicit ack */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
