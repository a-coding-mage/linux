// SPDX-License-Identifier: GPL-2.0
//
// C dependencies:
// - linux/export.h
// - linux/compiler.h
// - uapi/linux/swab.h

/* To silence -Wmissing-prototypes. */
pub extern "C" fn __bswapsi2(u: u32) -> u32 {
    u.swap_bytes()
}

// EXPORT_SYMBOL(__bswapsi2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
