// SPDX-License-Identifier: GPL-2.0

// To silence -Wmissing-prototypes.
pub fn __bswapdi2(u: u64) -> u64 {
    u.swap_bytes()
}

// EXPORT_SYMBOL(__bswapdi2);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
