/*
** amigaints.h -- Amiga Linux interrupt handling structs and prototypes
**
** Copyright 1992 by Greg Harp
**
** This file is subject to the terms and conditions of the GNU General Public
** License.  See the file COPYING in the main directory of this archive
** for more details.
**
** Created 10/2/92 by Greg Harp
*/

// Dependency: symbols from <asm/irq.h> are supplied by the surrounding build.

/* Amiga Interrupt sources. */
pub const AUTO_IRQS: i32 = 8;
pub const AMI_STD_IRQS: i32 = 14;
pub const CIA_IRQS: i32 = 5;
pub const AMI_IRQS: i32 = 32; // AUTO_IRQS+AMI_STD_IRQS+2*CIA_IRQS

/* builtin serial port interrupts */
pub const IRQ_AMIGA_TBE: i32 = IRQ_USER + 0;
pub const IRQ_AMIGA_RBF: i32 = IRQ_USER + 11;

/* floppy disk interrupts */
pub const IRQ_AMIGA_DSKBLK: i32 = IRQ_USER + 1;
pub const IRQ_AMIGA_DSKSYN: i32 = IRQ_USER + 12;

/* software interrupts */
pub const IRQ_AMIGA_SOFT: i32 = IRQ_USER + 2;

/* interrupts from external hardware */
pub const IRQ_AMIGA_PORTS: i32 = IRQ_AUTO_2;
pub const IRQ_AMIGA_EXTER: i32 = IRQ_AUTO_6;

/* copper interrupt */
pub const IRQ_AMIGA_COPPER: i32 = IRQ_USER + 4;

/* vertical blanking interrupt */
pub const IRQ_AMIGA_VERTB: i32 = IRQ_USER + 5;

/* Blitter done interrupt */
pub const IRQ_AMIGA_BLIT: i32 = IRQ_USER + 6;

/* Audio interrupts */
pub const IRQ_AMIGA_AUD0: i32 = IRQ_USER + 7;
pub const IRQ_AMIGA_AUD1: i32 = IRQ_USER + 8;
pub const IRQ_AMIGA_AUD2: i32 = IRQ_USER + 9;
pub const IRQ_AMIGA_AUD3: i32 = IRQ_USER + 10;

/* CIA interrupt sources */
pub const IRQ_AMIGA_CIAA: i32 = IRQ_USER + 14;
pub const IRQ_AMIGA_CIAA_TA: i32 = IRQ_USER + 14;
pub const IRQ_AMIGA_CIAA_TB: i32 = IRQ_USER + 15;
pub const IRQ_AMIGA_CIAA_ALRM: i32 = IRQ_USER + 16;
pub const IRQ_AMIGA_CIAA_SP: i32 = IRQ_USER + 17;
pub const IRQ_AMIGA_CIAA_FLG: i32 = IRQ_USER + 18;
pub const IRQ_AMIGA_CIAB: i32 = IRQ_USER + 19;
pub const IRQ_AMIGA_CIAB_TA: i32 = IRQ_USER + 19;
pub const IRQ_AMIGA_CIAB_TB: i32 = IRQ_USER + 20;
pub const IRQ_AMIGA_CIAB_ALRM: i32 = IRQ_USER + 21;
pub const IRQ_AMIGA_CIAB_SP: i32 = IRQ_USER + 22;
pub const IRQ_AMIGA_CIAB_FLG: i32 = IRQ_USER + 23;

/* INTREQR masks */
pub const IF_SETCLR: u16 = 0x8000; // set/clr bit
pub const IF_INTEN: u16 = 0x4000; // master interrupt bit in INT* registers
pub const IF_EXTER: u16 = 0x2000; // external level 6 and CIA B interrupt
pub const IF_DSKSYN: u16 = 0x1000; // disk sync interrupt
pub const IF_RBF: u16 = 0x0800; // serial receive buffer full interrupt
pub const IF_AUD3: u16 = 0x0400; // audio channel 3 done interrupt
pub const IF_AUD2: u16 = 0x0200; // audio channel 2 done interrupt
pub const IF_AUD1: u16 = 0x0100; // audio channel 1 done interrupt
pub const IF_AUD0: u16 = 0x0080; // audio channel 0 done interrupt
pub const IF_BLIT: u16 = 0x0040; // blitter done interrupt
pub const IF_VERTB: u16 = 0x0020; // vertical blanking interrupt
pub const IF_COPER: u16 = 0x0010; // copper interrupt
pub const IF_PORTS: u16 = 0x0008; // external level 2 and CIA A interrupt
pub const IF_SOFT: u16 = 0x0004; // software initiated interrupt
pub const IF_DSKBLK: u16 = 0x0002; // diskblock DMA finished
pub const IF_TBE: u16 = 0x0001; // serial transmit buffer empty interrupt

/* CIA interrupt control register bits */
pub const CIA_ICR_TA: u8 = 0x01;
pub const CIA_ICR_TB: u8 = 0x02;
pub const CIA_ICR_ALRM: u8 = 0x04;
pub const CIA_ICR_SP: u8 = 0x08;
pub const CIA_ICR_FLG: u8 = 0x10;
pub const CIA_ICR_ALL: u8 = 0x1f;
pub const CIA_ICR_SETCLR: u8 = 0x80;

pub struct ciabase;

extern "C" {
    pub fn amiga_init_IRQ();

    /* Access CIA interrupt control registers only through these functions. */
    pub static mut ciaa_base: ciabase;
    pub static mut ciab_base: ciabase;

    pub fn cia_init_IRQ(base: *mut ciabase);
    pub fn cia_set_irq(base: *mut ciabase, mask: u8) -> u8;
    pub fn cia_able_irq(base: *mut ciabase, mask: u8) -> u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
