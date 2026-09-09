/* SPDX-License-Identifier: GPL-2.0 */
/*
 * arch/arm/mach-sa1100/include/mach/irqs.h
 *
 * Copyright (C) 1996 Russell King
 * Copyright (C) 1998 Deborah Wallach (updates for SA1100/Brutus).
 * Copyright (C) 1999 Nicolas Pitre (full GPIO irq isolation)
 *
 * 2001/11/14\tRMK\tCleaned up and standardised a lot of the IRQs.
 */

pub const IRQ_GPIO0_SC: i32 = 1;
pub const IRQ_GPIO1_SC: i32 = 2;
pub const IRQ_GPIO2_SC: i32 = 3;
pub const IRQ_GPIO3_SC: i32 = 4;
pub const IRQ_GPIO4_SC: i32 = 5;
pub const IRQ_GPIO5_SC: i32 = 6;
pub const IRQ_GPIO6_SC: i32 = 7;
pub const IRQ_GPIO7_SC: i32 = 8;
pub const IRQ_GPIO8_SC: i32 = 9;
pub const IRQ_GPIO9_SC: i32 = 10;
pub const IRQ_GPIO10_SC: i32 = 11;
pub const IRQ_GPIO11_27: i32 = 12;
pub const IRQ_LCD: i32 = 13; /* LCD controller           */
pub const IRQ_Ser0UDC: i32 = 14; /* Ser. port 0 UDC          */
pub const IRQ_Ser1SDLC: i32 = 15; /* Ser. port 1 SDLC         */
pub const IRQ_Ser1UART: i32 = 16; /* Ser. port 1 UART         */
pub const IRQ_Ser2ICP: i32 = 17; /* Ser. port 2 ICP          */
pub const IRQ_Ser3UART: i32 = 18; /* Ser. port 3 UART         */
pub const IRQ_Ser4MCP: i32 = 19; /* Ser. port 4 MCP          */
pub const IRQ_Ser4SSP: i32 = 20; /* Ser. port 4 SSP          */
pub const IRQ_DMA0: i32 = 21; /* DMA controller channel 0 */
pub const IRQ_DMA1: i32 = 22; /* DMA controller channel 1 */
pub const IRQ_DMA2: i32 = 23; /* DMA controller channel 2 */
pub const IRQ_DMA3: i32 = 24; /* DMA controller channel 3 */
pub const IRQ_DMA4: i32 = 25; /* DMA controller channel 4 */
pub const IRQ_DMA5: i32 = 26; /* DMA controller channel 5 */
pub const IRQ_OST0: i32 = 27; /* OS Timer match 0         */
pub const IRQ_OST1: i32 = 28; /* OS Timer match 1         */
pub const IRQ_OST2: i32 = 29; /* OS Timer match 2         */
pub const IRQ_OST3: i32 = 30; /* OS Timer match 3         */
pub const IRQ_RTC1Hz: i32 = 31; /* RTC 1 Hz clock           */
pub const IRQ_RTCAlrm: i32 = 32; /* RTC Alarm                */

pub const IRQ_GPIO0: i32 = 33;
pub const IRQ_GPIO1: i32 = 34;
pub const IRQ_GPIO2: i32 = 35;
pub const IRQ_GPIO3: i32 = 36;
pub const IRQ_GPIO4: i32 = 37;
pub const IRQ_GPIO5: i32 = 38;
pub const IRQ_GPIO6: i32 = 39;
pub const IRQ_GPIO7: i32 = 40;
pub const IRQ_GPIO8: i32 = 41;
pub const IRQ_GPIO9: i32 = 42;
pub const IRQ_GPIO10: i32 = 43;
pub const IRQ_GPIO11: i32 = 44;
pub const IRQ_GPIO12: i32 = 45;
pub const IRQ_GPIO13: i32 = 46;
pub const IRQ_GPIO14: i32 = 47;
pub const IRQ_GPIO15: i32 = 48;
pub const IRQ_GPIO16: i32 = 49;
pub const IRQ_GPIO17: i32 = 50;
pub const IRQ_GPIO18: i32 = 51;
pub const IRQ_GPIO19: i32 = 52;
pub const IRQ_GPIO20: i32 = 53;
pub const IRQ_GPIO21: i32 = 54;
pub const IRQ_GPIO22: i32 = 55;
pub const IRQ_GPIO23: i32 = 56;
pub const IRQ_GPIO24: i32 = 57;
pub const IRQ_GPIO25: i32 = 58;
pub const IRQ_GPIO26: i32 = 59;
pub const IRQ_GPIO27: i32 = 60;

/*
 * The next 16 interrupts are for board specific purposes.  Since
 * the kernel can only run on one machine at a time, we can re-use
 * these.  If you need more, increase IRQ_BOARD_END, but keep it
 * within sensible limits.  IRQs 61 to 76 are available.
 */
pub const IRQ_BOARD_START: i32 = 61;
pub const IRQ_BOARD_END: i32 = 77;

/*
 * Figure out the MAX IRQ number.
 *
 * Neponset, SA1111 and UCB1x00 are sparse IRQ aware, so can dynamically
 * allocate their IRQs above NR_IRQS.
 *
 * LoCoMo has 4 additional IRQs, but is not sparse IRQ aware, and so has
 * to be included in the NR_IRQS calculation.
 *
 * CONFIG_SHARP_LOCOMO is a build-time C preprocessor condition.  The Rust
 * feature below preserves the same conditional intent.
 */
#[cfg(feature = "CONFIG_SHARP_LOCOMO")]
pub const NR_IRQS_LOCOMO: i32 = 4;
#[cfg(not(feature = "CONFIG_SHARP_LOCOMO"))]
pub const NR_IRQS_LOCOMO: i32 = 0;

/* C's #ifndef NR_IRQS: define this only when no external NR_IRQS exists. */
pub const NR_IRQS: i32 = IRQ_BOARD_START + NR_IRQS_LOCOMO;
pub const SA1100_NR_IRQS: i32 = IRQ_BOARD_START + NR_IRQS_LOCOMO;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
