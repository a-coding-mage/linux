/* SPDX-License-Identifier: GPL-2.0 */
// Header guard: _CAN_PLATFORM_SJA1000_H

/* clock divider register */
pub const CDR_CLKOUT_MASK: u32 = 0x07;
pub const CDR_CLK_OFF: u32 = 0x08; /* Clock off (CLKOUT pin) */
pub const CDR_RXINPEN: u32 = 0x20; /* TX1 output is RX irq output */
pub const CDR_CBP: u32 = 0x40; /* CAN input comparator bypass */
pub const CDR_PELICAN: u32 = 0x80; /* PeliCAN mode */

/* output control register */
pub const OCR_MODE_BIPHASE: u32 = 0x00;
pub const OCR_MODE_TEST: u32 = 0x01;
pub const OCR_MODE_NORMAL: u32 = 0x02;
pub const OCR_MODE_CLOCK: u32 = 0x03;
pub const OCR_MODE_MASK: u32 = 0x03;
pub const OCR_TX0_INVERT: u32 = 0x04;
pub const OCR_TX0_PULLDOWN: u32 = 0x08;
pub const OCR_TX0_PULLUP: u32 = 0x10;
pub const OCR_TX0_PUSHPULL: u32 = 0x18;
pub const OCR_TX1_INVERT: u32 = 0x20;
pub const OCR_TX1_PULLDOWN: u32 = 0x40;
pub const OCR_TX1_PULLUP: u32 = 0x80;
pub const OCR_TX1_PUSHPULL: u32 = 0xc0;
pub const OCR_TX_MASK: u32 = 0xfc;
pub const OCR_TX_SHIFT: u32 = 2;

#[repr(C)]
pub struct sja1000_platform_data {
	pub osc_freq: u32, /* CAN bus oscillator frequency in Hz */

	pub ocr: u8, /* output control register */
	pub cdr: u8, /* clock divider register */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
