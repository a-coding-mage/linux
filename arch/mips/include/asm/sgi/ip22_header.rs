/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * ip22.h: Definitions for SGI IP22 machines
 *
 * Copyright (C) 1996 David S. Miller
 * Copyright (C) 1997, 1998, 1999, 2000 Ralf Baechle
 */

// Dependencies supplied by the surrounding kernel translation:
// <irq.h>, <asm/sgi/ioc.h>

/*
 * These are the virtual IRQ numbers, we divide all IRQ's into
 * 'spaces', the 'space' determines where and how to enable/disable
 * that particular IRQ on an SGI machine. HPC DMA and MC DMA interrupts
 * are not supported this way. Driver is supposed to allocate HPC/MC
 * interrupt as shareable and then look to proper status bit (see
 * HAL2 driver). This will prevent many complications, trust me ;-)
 */

pub const SGINT_EISA: u32 = 0; /* 16 EISA irq levels (Indigo2) */
pub const SGINT_CPU: u32 = MIPS_CPU_IRQ_BASE; /* MIPS CPU define 8 interrupt sources */
pub const SGINT_LOCAL0: u32 = SGINT_CPU + 8; /* 8 local0 irq levels */
pub const SGINT_LOCAL1: u32 = SGINT_CPU + 16; /* 8 local1 irq levels */
pub const SGINT_LOCAL2: u32 = SGINT_CPU + 24; /* 8 local2 vectored irq levels */
pub const SGINT_LOCAL3: u32 = SGINT_CPU + 32; /* 8 local3 vectored irq levels */
pub const SGINT_END: u32 = SGINT_CPU + 40; /* End of 'spaces' */

/* Individual interrupt definitions for the Indy and Indigo2 */
pub const SGI_SOFT_0_IRQ: u32 = SGINT_CPU + 0;
pub const SGI_SOFT_1_IRQ: u32 = SGINT_CPU + 1;
pub const SGI_LOCAL_0_IRQ: u32 = SGINT_CPU + 2;
pub const SGI_LOCAL_1_IRQ: u32 = SGINT_CPU + 3;
pub const SGI_8254_0_IRQ: u32 = SGINT_CPU + 4;
pub const SGI_8254_1_IRQ: u32 = SGINT_CPU + 5;
pub const SGI_BUSERR_IRQ: u32 = SGINT_CPU + 6;
pub const SGI_TIMER_IRQ: u32 = SGINT_CPU + 7;

pub const SGI_FIFO_IRQ: u32 = SGINT_LOCAL0 + 0; /* FIFO full */
pub const SGI_GIO_0_IRQ: u32 = SGI_FIFO_IRQ; /* GIO-0 */
pub const SGI_WD93_0_IRQ: u32 = SGINT_LOCAL0 + 1; /* 1st onboard WD93 */
pub const SGI_WD93_1_IRQ: u32 = SGINT_LOCAL0 + 2; /* 2nd onboard WD93 */
pub const SGI_ENET_IRQ: u32 = SGINT_LOCAL0 + 3; /* onboard ethernet */
pub const SGI_MCDMA_IRQ: u32 = SGINT_LOCAL0 + 4; /* MC DMA done */
pub const SGI_PARPORT_IRQ: u32 = SGINT_LOCAL0 + 5; /* Parallel port */
pub const SGI_GIO_1_IRQ: u32 = SGINT_LOCAL0 + 6; /* GE / GIO-1 / 2nd-HPC */
pub const SGI_MAP_0_IRQ: u32 = SGINT_LOCAL0 + 7; /* Mappable interrupt 0 */

pub const SGI_GPL0_IRQ: u32 = SGINT_LOCAL1 + 0; /* General Purpose LOCAL1_N<0> */
pub const SGI_PANEL_IRQ: u32 = SGINT_LOCAL1 + 1; /* front panel */
pub const SGI_GPL2_IRQ: u32 = SGINT_LOCAL1 + 2; /* General Purpose LOCAL1_N<2> */
pub const SGI_MAP_1_IRQ: u32 = SGINT_LOCAL1 + 3; /* Mappable interrupt 1 */
pub const SGI_HPCDMA_IRQ: u32 = SGINT_LOCAL1 + 4; /* HPC DMA done */
pub const SGI_ACFAIL_IRQ: u32 = SGINT_LOCAL1 + 5; /* AC fail */
pub const SGI_VINO_IRQ: u32 = SGINT_LOCAL1 + 6; /* Indy VINO */
pub const SGI_GIO_2_IRQ: u32 = SGINT_LOCAL1 + 7; /* Vert retrace / GIO-2 */

/* Mapped interrupts. These interrupts may be mapped to either 0, or 1 */
pub const SGI_VERT_IRQ: u32 = SGINT_LOCAL2 + 0; /* INT3: newport vertical status */
pub const SGI_EISA_IRQ: u32 = SGINT_LOCAL2 + 3; /* EISA interrupts */
pub const SGI_KEYBD_IRQ: u32 = SGINT_LOCAL2 + 4; /* keyboard */
pub const SGI_SERIAL_IRQ: u32 = SGINT_LOCAL2 + 5; /* onboard serial */
pub const SGI_GIOEXP0_IRQ: u32 = SGINT_LOCAL2 + 6; /* Indy GIO EXP0 */
pub const SGI_GIOEXP1_IRQ: u32 = SGINT_LOCAL2 + 7; /* Indy GIO EXP1 */

macro_rules! ip22_is_fullhouse {
    () => {{ unsafe { (*sgioc).sysid & SGIOC_SYSID_FULLHOUSE } }};
}

unsafe extern "C" {
    pub fn ip22_eeprom_read(ctrl: *mut u32, reg: i32) -> u16;
    pub fn ip22_nvram_read(reg: i32) -> u16;
    pub fn ip22_be_interrupt(irq: i32);
    pub fn ip22_be_init(); /* __init */
    pub fn indy_8254timer_irq();
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
