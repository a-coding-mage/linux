/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright 2004-2007, 2014 Freescale Semiconductor, Inc. All Rights Reserved.
 * Copyright 2008 Juergen Beisert, kernel@pengutronix.de
 */

// C dependencies: asm/io.h, soc/imx/revision.h, linux/sizes.h,
// and the declarations from mxc.h, mx3x.h, mx31.h, mx35.h, mx2x.h, mx27.h.

/// Equivalent of `((unsigned long)(addr) - mod_BASE_ADDR < mod_SIZE)`.
#[macro_export]
macro_rules! addr_in_module {
    ($addr:expr, $base:expr, $size:expr) => {
        (($addr as usize).wrapping_sub($base as usize) < $size as usize)
    };
}

/// Convert a physical i.MX module address to its virtual module address.
#[macro_export]
macro_rules! imx_io_p2v_module {
    ($addr:expr, $base:expr, $size:expr, $base_virt:expr) => {{
        let addr = $addr;
        if (addr as usize).wrapping_sub($base as usize) < $size as usize {
            (addr as usize).wrapping_sub($base as usize) + $base_virt as usize
        } else {
            0
        }
    }};
}

/*
 * This mapping is deliberately arithmetic so it is identical on all i.MX
 * machines and can also be used from assembler.  It maps the SoC IO regions
 * into the common virtual range beginning at 0xf4000000; the intervening
 * address space remains available for per-machine use.
 */
/// The common i.MX physical-to-virtual IO mapping.
#[macro_export]
macro_rules! imx_io_p2v {
    ($x:expr) => {{
        let x = $x as usize;
        ((x & 0x8000_0000usize) >> 7)
            | (0xf400_0000usize
                + ((x & 0x5000_0000usize) >> 6)
                + ((x & 0x0b00_0000usize) >> 4)
                + (x & 0x000f_ffffusize))
    }};
}

// `IOMEM(IMX_IO_P2V(x))`; IOMEM is supplied by the IO dependency.
#[macro_export]
macro_rules! imx_io_address {
    ($x:expr) => {
        IOMEM(imx_io_p2v!($x))
    };
}

// The C macro expands identifiers by token pasting; Rust callers supply the
// corresponding values explicitly.
#[macro_export]
macro_rules! imx_map_entry {
    ($soc:ident, $name:ident, $type:expr,
     $io_p2v:path, $base:expr, $size:expr, $phys_to_pfn:path) => {
        .virtual = $io_p2v($base),
        .pfn = $phys_to_pfn($base),
        .length = $size,
        .type = $type,
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
