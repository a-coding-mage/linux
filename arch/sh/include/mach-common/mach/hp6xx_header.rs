/* SPDX-License-Identifier: GPL-2.0
 *
 * Copyright (C) 2003, 2004, 2005  Andriy Skulysh
 */

// Dependency supplied by the surrounding translated code:
// #include <linux/sh_intc.h>
// #include <asm/hd64461.h>
// #include <asm/io.h>

pub const HP680_BTN_IRQ: i32 = evt2irq(0x600); // IRQ0_IRQ
pub const HP680_TS_IRQ: i32 = evt2irq(0x660); // IRQ3_IRQ
pub const HP680_HD64461_IRQ: i32 = evt2irq(0x680); // IRQ4_IRQ

pub const DAC_LCD_BRIGHTNESS: u32 = 0;
pub const DAC_SPEAKER_VOLUME: u32 = 1;

pub const PGDR_OPENED: u32 = 0x01;
pub const PGDR_MAIN_BATTERY_OUT: u32 = 0x04;
pub const PGDR_PLAY_BUTTON: u32 = 0x08;
pub const PGDR_REWIND_BUTTON: u32 = 0x10;
pub const PGDR_RECORD_BUTTON: u32 = 0x20;

pub const PHDR_TS_PEN_DOWN: u32 = 0x08;

pub const PJDR_LED_BLINK: u32 = 0x02;

pub const PKDR_LED_GREEN: u32 = 0x10;

/* HP Palmtop 620lx/660lx speaker on/off */
pub const PKDR_SPEAKER: u32 = 0x20;

pub const SCPDR_TS_SCAN_ENABLE: u32 = 0x20;
pub const SCPDR_TS_SCAN_Y: u32 = 0x02;
pub const SCPDR_TS_SCAN_X: u32 = 0x01;

pub const SCPCR_TS_ENABLE: u32 = 0x405;
pub const SCPCR_TS_MASK: u32 = 0xc0f;

pub const ADC_CHANNEL_TS_Y: u32 = 1;
pub const ADC_CHANNEL_TS_X: u32 = 2;
pub const ADC_CHANNEL_BATTERY: u32 = 3;
pub const ADC_CHANNEL_BACKUP: u32 = 4;
pub const ADC_CHANNEL_CHARGE: u32 = 5;

/* HP Jornada 680/690 speaker on/off */
pub const HD64461_GPADR_SPEAKER: u32 = 0x01;
pub const HD64461_GPADR_PCMCIA0: u32 = 0x02 | 0x08;

pub const HD64461_GPBDR_LCDOFF: u32 = 0x01;
pub const HD64461_GPBDR_LCD_CONTRAST_MASK: u32 = 0x78;
pub const HD64461_GPBDR_LED_RED: u32 = 0x80;

pub const PJDR: usize = 0xa4000130;
pub const PKDR: usize = 0xa4000132;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
