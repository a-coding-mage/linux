/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/* bits.h: Macros for dealing with bitmasks. */

/* C preprocessor macros translated as Rust declarative macros. */
macro_rules! __GENMASK {
	($h:expr, $l:expr) => {
		(((!_UL!(0)) << ($l)) & ((!_UL!(0)) >> (__BITS_PER_LONG - 1 - ($h))))
	};
}

macro_rules! __GENMASK_ULL {
	($h:expr, $l:expr) => {
		(((!_ULL!(0)) << ($l))
			& ((!_ULL!(0)) >> (__BITS_PER_LONG_LONG - 1 - ($h))))
	};
}

macro_rules! __GENMASK_U128 {
	($h:expr, $l:expr) => {
		((_BIT128!($h) << 1) - (_BIT128!($l)))
	};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
