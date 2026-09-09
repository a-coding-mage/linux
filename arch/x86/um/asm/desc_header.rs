/* SPDX-License-Identifier: GPL-2.0 */

/* Taken from asm-i386/desc.h, it's the only thing we need. The rest wouldn't
 * compile, and has never been used. */
macro_rules! LDT_empty {
	($info:expr) => {
		($info).base_addr == 0
			&& ($info).limit == 0
			&& ($info).contents == 0
			&& ($info).read_exec_only == 1
			&& ($info).seg_32bit == 0
			&& ($info).limit_in_pages == 0
			&& ($info).seg_not_present == 1
			&& ($info).useable == 0
	};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
