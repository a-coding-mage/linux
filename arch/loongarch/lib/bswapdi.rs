// SPDX-License-Identifier: GPL-2.0
// Dependency intent: linux/export.h, linux/compiler.h, and uapi/linux/swab.h

/* To silence -Wmissing-prototypes. */
pub unsafe extern "C" fn __bswapdi2(u: u64) -> u64 {
    ((u & 0x0000_0000_0000_00ff) << 56)
        | ((u & 0x0000_0000_0000_ff00) << 40)
        | ((u & 0x0000_0000_00ff_0000) << 24)
        | ((u & 0x0000_0000_ff00_0000) << 8)
        | ((u & 0x0000_00ff_0000_0000) >> 8)
        | ((u & 0x0000_ff00_0000_0000) >> 24)
        | ((u & 0x00ff_0000_0000_0000) >> 40)
        | ((u & 0xff00_0000_0000_0000) >> 56)
}

// Equivalent to EXPORT_SYMBOL(__bswapdi2).

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
