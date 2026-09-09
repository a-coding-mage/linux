/*
 * Microchip KSZ series switch platform data
 *
 * Copyright (C) 2017
 *
 * Permission to use, copy, modify, and/or distribute this software for any
 * purpose with or without fee is hereby granted, provided that the above
 * copyright notice and this permission notice appear in all copies.
 *
 * THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL WARRANTIES
 * WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED WARRANTIES OF
 * MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE AUTHOR BE LIABLE FOR
 * ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL DAMAGES OR ANY DAMAGES
 * WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR PROFITS, WHETHER IN AN
 * ACTION OF CONTRACT, NEGLIGENCE OR OTHER TORTIOUS ACTION, ARISING OUT OF
 * OR IN CONNECTION WITH THE USE OR PERFORMANCE OF THIS SOFTWARE.
 */

// Dependencies supplied by the corresponding Linux headers:
// use linux::platform_data::dsa::dsa_chip_data;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ksz_chip_id {
	KSZ8463_CHIP_ID = 0x8463,
	KSZ8563_CHIP_ID = 0x8563,
	KSZ8795_CHIP_ID = 0x8795,
	KSZ8794_CHIP_ID = 0x8794,
	KSZ8765_CHIP_ID = 0x8765,
	KSZ88X3_CHIP_ID = 0x8830,
	KSZ8864_CHIP_ID = 0x8864,
	KSZ8895_CHIP_ID = 0x8895,
	KSZ9477_CHIP_ID = 0x00947700,
	KSZ9896_CHIP_ID = 0x00989600,
	KSZ9897_CHIP_ID = 0x00989700,
	KSZ9893_CHIP_ID = 0x00989300,
	KSZ9563_CHIP_ID = 0x00956300,
	KSZ8567_CHIP_ID = 0x00856700,
	KSZ9567_CHIP_ID = 0x00956700,
	LAN9370_CHIP_ID = 0x00937000,
	LAN9371_CHIP_ID = 0x00937100,
	LAN9372_CHIP_ID = 0x00937200,
	LAN9373_CHIP_ID = 0x00937300,
	LAN9374_CHIP_ID = 0x00937400,
	LAN9646_CHIP_ID = 0x00964600,
}

#[repr(C)]
pub struct ksz_platform_data {
	/* Must be first such that dsa_register_switch() can access it */
	pub cd: dsa_chip_data,
	pub chip_id: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
