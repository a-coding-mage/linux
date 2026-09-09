/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: declarations from <uapi/linux/auxvec.h> are supplied elsewhere.

pub const AT_VECTOR_SIZE_BASE: usize = 24; /* NEW_AUX_ENT entries in auxiliary table */
// number of "#define AT_.*" above, minus {AT_NULL, AT_IGNORE, AT_NOTELF}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
