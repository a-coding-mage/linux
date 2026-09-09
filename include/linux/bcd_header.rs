/* SPDX-License-Identifier: GPL-2.0 */

/* Dependency supplied by the surrounding kernel translation. */

/*
 * The C __builtin_constant_p selection has no direct Rust equivalent here;
 * the arithmetic macros preserve the same value semantics.
 */
macro_rules! bcd2bin {
	($x:expr) => {
		const_bcd2bin!($x)
	};
}

macro_rules! bin2bcd {
	($x:expr) => {
		const_bin2bcd!($x)
	};
}

macro_rules! bcd_is_valid {
	($x:expr) => {
		const_bcd_is_valid!($x)
	};
}

macro_rules! const_bcd2bin {
	($x:expr) => {
		(($x & 0x0f) + (($x >> 4) * 10))
	};
}

macro_rules! const_bin2bcd {
	($x:expr) => {
		((($x / 10) << 4) + $x % 10)
	};
}

macro_rules! const_bcd_is_valid {
	($x:expr) => {
		(($x & 0x0f) < 10 && ($x >> 4) < 10)
	};
}

extern "C" {
	pub fn _bcd2bin(val: u8) -> u32;
	pub fn _bin2bcd(val: u32) -> u8;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
