// SPDX-License-Identifier: GPL-2.0-only
/*
 * crc4.c - simple crc-4 calculations.
 */

// Dependencies supplied by the surrounding kernel translation are not
// represented here. The original file includes linux/crc4.h, linux/export.h,
// and linux/module.h.

static CRC4_TAB: [u8; 16] = [
	0x0, 0x7, 0xe, 0x9, 0xb, 0xc, 0x5, 0x2,
	0x1, 0x6, 0xf, 0x8, 0xa, 0xd, 0x4, 0x3,
];

/**
 * crc4 - calculate the 4-bit crc of a value.
 * @c:    starting crc4
 * @x:    value to checksum
 * @bits: number of bits in @x to checksum
 *
 * Returns the crc4 value of @x, using polynomial 0b10111.
 *
 * The @x value is treated as left-aligned, and bits above @bits are ignored
 * in the crc calculations.
 */
pub extern "C" fn crc4(mut c: u8, mut x: u64, mut bits: i32) -> u8 {
	let mut i: i32;

	/* mask off anything above the top bit */
	x &= (1u64 << bits) - 1;

	/* Align to 4-bits */
	bits = (bits + 3) & !0x3;

	/* Calculate crc4 over four-bit nibbles, starting at the MSbit */
	i = bits - 4;
	while i >= 0 {
		c = CRC4_TAB[(c ^ ((x >> i) & 0xf) as u8) as usize];
		i -= 4;
	}

	c
}

// Original export: EXPORT_SYMBOL_GPL(crc4);
// Original module metadata: MODULE_DESCRIPTION("CRC4 calculations");
// Original module metadata: MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
