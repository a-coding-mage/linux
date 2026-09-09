/* SPDX-License-Identifier: GPL-2.0 */

/* Dependencies: linux/build_bug.h, linux/compiler_attributes.h,
 * and uapi/linux/fcntl.h provide the corresponding assertion support and
 * RWH_WRITE_LIFE_* constants.
 */

/* Block storage write lifetime hint values. */
#[repr(u8)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum rw_hint {
	WRITE_LIFE_NOT_SET = RWH_WRITE_LIFE_NOT_SET,
	WRITE_LIFE_NONE = RWH_WRITE_LIFE_NONE,
	WRITE_LIFE_SHORT = RWH_WRITE_LIFE_SHORT,
	WRITE_LIFE_MEDIUM = RWH_WRITE_LIFE_MEDIUM,
	WRITE_LIFE_LONG = RWH_WRITE_LIFE_LONG,
	WRITE_LIFE_EXTREME = RWH_WRITE_LIFE_EXTREME,
	WRITE_LIFE_HINT_NR,
}

/* Sparse ignores __packed annotations on enums, hence the #ifndef below. */
/* Equivalent to the C static assertion when __CHECKER__ is not defined. */
#[cfg(not(__CHECKER__))]
const _: () = assert!(core::mem::size_of::<rw_hint>() == 1);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
