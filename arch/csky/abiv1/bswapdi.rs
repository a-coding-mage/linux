// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

/// Byte-swap a 64-bit unsigned value.
///
/// Corresponds to the C `notrace __bswapdi2` implementation using
/// `___constant_swab64`.
#[no_mangle]
pub extern "C" fn __bswapdi2(u: u64) -> u64 {
    u.swap_bytes()
}

// C: EXPORT_SYMBOL(__bswapdi2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
