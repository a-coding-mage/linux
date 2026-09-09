/* SPDX-License-Identifier: GPL-2.0 */
/****************************************************************************/

/*
 *	mcfintc.h -- support definitions for the simple ColdFire
 *		     Interrupt Controller
 *
 * 	(C) Copyright 2009,  Greg Ungerer <gerg@uclinux.org>
 */

/****************************************************************************/

/*
 * Most of the older ColdFire parts use the same simple interrupt
 * controller. This is currently used on the 5206, 5206e, 5249, 5307
 * and 5407 parts.
 *
 * The builtin peripherals are masked through dedicated bits in the
 * Interrupt Mask register (IMR) - and this is not indexed (or in any way
 * related to) the actual interrupt number they use. So knowing the IRQ
 * number doesn't explicitly map to a certain internal device for
 * interrupt control purposes.
 */

/*
 * Bit definitions for the ICR family of registers.
 */
pub const MCFSIM_ICR_AUTOVEC: u32 = 0x80; /* Auto-vectored intr */
pub const MCFSIM_ICR_LEVEL0: u32 = 0x00; /* Level 0 intr */
pub const MCFSIM_ICR_LEVEL1: u32 = 0x04; /* Level 1 intr */
pub const MCFSIM_ICR_LEVEL2: u32 = 0x08; /* Level 2 intr */
pub const MCFSIM_ICR_LEVEL3: u32 = 0x0c; /* Level 3 intr */
pub const MCFSIM_ICR_LEVEL4: u32 = 0x10; /* Level 4 intr */
pub const MCFSIM_ICR_LEVEL5: u32 = 0x14; /* Level 5 intr */
pub const MCFSIM_ICR_LEVEL6: u32 = 0x18; /* Level 6 intr */
pub const MCFSIM_ICR_LEVEL7: u32 = 0x1c; /* Level 7 intr */

pub const MCFSIM_ICR_PRI0: u32 = 0x00; /* Priority 0 intr */
pub const MCFSIM_ICR_PRI1: u32 = 0x01; /* Priority 1 intr */
pub const MCFSIM_ICR_PRI2: u32 = 0x02; /* Priority 2 intr */
pub const MCFSIM_ICR_PRI3: u32 = 0x03; /* Priority 3 intr */

/*
 * IMR bit position definitions. Not all ColdFire parts with this interrupt
 * controller actually support all of these interrupt sources. But the bit
 * numbers are the same in all cores.
 */
pub const MCFINTC_EINT1: u32 = 1; /* External int #1 */
pub const MCFINTC_EINT2: u32 = 2; /* External int #2 */
pub const MCFINTC_EINT3: u32 = 3; /* External int #3 */
pub const MCFINTC_EINT4: u32 = 4; /* External int #4 */
pub const MCFINTC_EINT5: u32 = 5; /* External int #5 */
pub const MCFINTC_EINT6: u32 = 6; /* External int #6 */
pub const MCFINTC_EINT7: u32 = 7; /* External int #7 */
pub const MCFINTC_SWT: u32 = 8; /* Software Watchdog */
pub const MCFINTC_TIMER1: u32 = 9;
pub const MCFINTC_TIMER2: u32 = 10;
pub const MCFINTC_I2C: u32 = 11; /* I2C / MBUS */
pub const MCFINTC_UART0: u32 = 12;
pub const MCFINTC_UART1: u32 = 13;
pub const MCFINTC_DMA0: u32 = 14;
pub const MCFINTC_DMA1: u32 = 15;
pub const MCFINTC_DMA2: u32 = 16;
pub const MCFINTC_DMA3: u32 = 17;
pub const MCFINTC_QSPI: u32 = 18;

/*
 * There is no one-is-one correspondance between the interrupt number (irq)
 * and the bit fields on the mask register. So we create a per-cpu type
 * mapping of irq to mask bit. The CPU platform code needs to register
 * its supported irq's at init time, using this function.
 */
unsafe extern "C" {
    pub static mut mcf_irq2imr: [u8; 0];

    pub fn mcf_autovector(irq: i32);
    pub fn mcf_setimr(index: i32);
    pub fn mcf_clrimr(index: i32);
}

#[inline]
pub unsafe fn mcf_mapirq2imr(irq: i32, imr: i32) {
    mcf_irq2imr.as_mut_ptr().add(irq as usize).write(imr as u8);
}

/****************************************************************************/

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
