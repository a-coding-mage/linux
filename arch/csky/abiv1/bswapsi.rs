// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// <linux/export.h>
// <linux/compiler.h>
// <uapi/linux/swab.h>

/// Byte-swap a 32-bit unsigned integer.
///
/// Corresponds to the C `notrace __bswapsi2` implementation using
/// `___constant_swab32(u)`.
#[no_mangle]
pub unsafe extern "C" fn __bswapsi2(u: u32) -> u32 {
    u.swap_bytes()
}

// EXPORT_SYMBOL(__bswapsi2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
