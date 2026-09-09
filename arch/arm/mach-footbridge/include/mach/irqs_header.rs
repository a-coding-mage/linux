/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/arm/mach-footbridge/include/mach/irqs.h
 *
 * Copyright (C) 1998 Russell King
 * Copyright (C) 1998 Phil Blundell
 *
 * Changelog:
 *  20-Jan-1998 RMK Started merge of EBSA286, CATS and NetWinder
 *  01-Feb-1999 PJB ISA IRQs start at 0 not 16
 */

// Dependency supplied by the surrounding machine-type implementation.
extern "C" {
    pub fn machine_is_netwinder() -> i32;
}

pub const NR_IRQS: i32 = 36;
pub const NR_DC21285_IRQS: i32 = 16;

#[inline]
pub const fn _ISA_IRQ(x: i32) -> i32 { 0 + x }
#[inline]
pub const fn _ISA_INR(x: i32) -> i32 { x - 0 }
#[inline]
pub const fn _DC21285_IRQ(x: i32) -> i32 { 16 + x }
#[inline]
pub const fn _DC21285_INR(x: i32) -> i32 { x - 16 }

/*
 * This is a list of all interrupts that the 21285
 * can generate and we handle.
 */
pub const IRQ_CONRX: i32 = _DC21285_IRQ(0);
pub const IRQ_CONTX: i32 = _DC21285_IRQ(1);
pub const IRQ_TIMER1: i32 = _DC21285_IRQ(2);
pub const IRQ_TIMER2: i32 = _DC21285_IRQ(3);
pub const IRQ_TIMER3: i32 = _DC21285_IRQ(4);
pub const IRQ_IN0: i32 = _DC21285_IRQ(5);
pub const IRQ_IN1: i32 = _DC21285_IRQ(6);
pub const IRQ_IN2: i32 = _DC21285_IRQ(7);
pub const IRQ_IN3: i32 = _DC21285_IRQ(8);
pub const IRQ_DOORBELLHOST: i32 = _DC21285_IRQ(9);
pub const IRQ_DMA1: i32 = _DC21285_IRQ(10);
pub const IRQ_DMA2: i32 = _DC21285_IRQ(11);
pub const IRQ_PCI: i32 = _DC21285_IRQ(12);
pub const IRQ_SDRAMPARITY: i32 = _DC21285_IRQ(13);
pub const IRQ_I2OINPOST: i32 = _DC21285_IRQ(14);
pub const IRQ_PCI_ABORT: i32 = _DC21285_IRQ(15);
pub const IRQ_PCI_SERR: i32 = _DC21285_IRQ(16);
pub const IRQ_DISCARD_TIMER: i32 = _DC21285_IRQ(17);
pub const IRQ_PCI_DPERR: i32 = _DC21285_IRQ(18);
pub const IRQ_PCI_PERR: i32 = _DC21285_IRQ(19);

pub const IRQ_ISA_TIMER: i32 = _ISA_IRQ(0);
pub const IRQ_ISA_KEYBOARD: i32 = _ISA_IRQ(1);
pub const IRQ_ISA_CASCADE: i32 = _ISA_IRQ(2);
pub const IRQ_ISA_UART2: i32 = _ISA_IRQ(3);
pub const IRQ_ISA_UART: i32 = _ISA_IRQ(4);
pub const IRQ_ISA_FLOPPY: i32 = _ISA_IRQ(6);
pub const IRQ_ISA_PRINTER: i32 = _ISA_IRQ(7);
pub const IRQ_ISA_RTC_ALARM: i32 = _ISA_IRQ(8);
pub const IRQ_ISA_2: i32 = _ISA_IRQ(9);
pub const IRQ_ISA_PS2MOUSE: i32 = _ISA_IRQ(12);
pub const IRQ_ISA_HARDDISK1: i32 = _ISA_IRQ(14);
pub const IRQ_ISA_HARDDISK2: i32 = _ISA_IRQ(15);

pub const IRQ_MASK_UART_RX: i32 = 1 << 2;
pub const IRQ_MASK_UART_TX: i32 = 1 << 3;
pub const IRQ_MASK_TIMER1: i32 = 1 << 4;
pub const IRQ_MASK_TIMER2: i32 = 1 << 5;
pub const IRQ_MASK_TIMER3: i32 = 1 << 6;
pub const IRQ_MASK_IN0: i32 = 1 << 8;
pub const IRQ_MASK_IN1: i32 = 1 << 9;
pub const IRQ_MASK_IN2: i32 = 1 << 10;
pub const IRQ_MASK_IN3: i32 = 1 << 11;
pub const IRQ_MASK_DOORBELLHOST: i32 = 1 << 15;
pub const IRQ_MASK_DMA1: i32 = 1 << 16;
pub const IRQ_MASK_DMA2: i32 = 1 << 17;
pub const IRQ_MASK_PCI: i32 = 1 << 18;
pub const IRQ_MASK_SDRAMPARITY: i32 = 1 << 24;
pub const IRQ_MASK_I2OINPOST: i32 = 1 << 25;
pub const IRQ_MASK_PCI_ABORT: i32 = (1 << 29) | (1 << 30);
pub const IRQ_MASK_PCI_SERR: i32 = 1 << 23;
pub const IRQ_MASK_DISCARD_TIMER: i32 = 1 << 27;
pub const IRQ_MASK_PCI_DPERR: i32 = 1 << 28;
pub const IRQ_MASK_PCI_PERR: i32 = 1 << 31;

/* Netwinder interrupt allocations */
pub const IRQ_NETWINDER_ETHER10: i32 = IRQ_IN0;
pub const IRQ_NETWINDER_ETHER100: i32 = IRQ_IN1;
pub const IRQ_NETWINDER_VIDCOMP: i32 = IRQ_IN2;
pub const IRQ_NETWINDER_PS2MOUSE: i32 = _ISA_IRQ(5);
pub const IRQ_NETWINDER_IR: i32 = _ISA_IRQ(6);
pub const IRQ_NETWINDER_BUTTON: i32 = _ISA_IRQ(10);
pub const IRQ_NETWINDER_VGA: i32 = _ISA_IRQ(11);
pub const IRQ_NETWINDER_SOUND: i32 = _ISA_IRQ(12);

pub const I8042_KBD_IRQ: i32 = IRQ_ISA_KEYBOARD;
#[inline]
pub unsafe fn I8042_AUX_IRQ() -> i32 {
    if machine_is_netwinder() != 0 { IRQ_NETWINDER_PS2MOUSE } else { IRQ_ISA_PS2MOUSE }
}
pub const IRQ_FLOPPYDISK: i32 = IRQ_ISA_FLOPPY;

#[macro_export]
macro_rules! irq_canonicalize {
    ($i:expr) => {{
        let value = $i;
        if value == $crate::IRQ_ISA_CASCADE { $crate::IRQ_ISA_2 } else { value }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
