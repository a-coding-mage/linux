/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2010 Google, Inc.
 *
 * Author:
 *	Colin Cross <ccross@google.com>
 *	Erik Gilling <konkers@google.com>
 */

// Dependencies from <linux/pgtable.h> and <linux/sizes.h> are represented by
// the literal values used by this header.

pub const TEGRA_IRAM_BASE: usize = 0x40000000;
pub const TEGRA_IRAM_SIZE: usize = 256 * 1024;
pub const TEGRA_ARM_PERIF_BASE: usize = 0x50040000;
pub const TEGRA_ARM_PERIF_SIZE: usize = 8 * 1024;
pub const TEGRA_ARM_INT_DIST_BASE: usize = 0x50041000;
pub const TEGRA_ARM_INT_DIST_SIZE: usize = 4 * 1024;
pub const TEGRA_TMR1_BASE: usize = 0x60005000;
pub const TEGRA_TMR1_SIZE: usize = 8;
pub const TEGRA_TMR2_BASE: usize = 0x60005008;
pub const TEGRA_TMR2_SIZE: usize = 8;
pub const TEGRA_TMRUS_BASE: usize = 0x60005010;
pub const TEGRA_TMRUS_SIZE: usize = 64;
pub const TEGRA_TMR3_BASE: usize = 0x60005050;
pub const TEGRA_TMR3_SIZE: usize = 8;
pub const TEGRA_TMR4_BASE: usize = 0x60005058;
pub const TEGRA_TMR4_SIZE: usize = 8;
pub const TEGRA_CLK_RESET_BASE: usize = 0x60006000;
pub const TEGRA_CLK_RESET_SIZE: usize = 4 * 1024;
pub const TEGRA_FLOW_CTRL_BASE: usize = 0x60007000;
pub const TEGRA_FLOW_CTRL_SIZE: usize = 20;
pub const TEGRA_SB_BASE: usize = 0x6000C200;
pub const TEGRA_SB_SIZE: usize = 256;
pub const TEGRA_EXCEPTION_VECTORS_BASE: usize = 0x6000F000;
pub const TEGRA_EXCEPTION_VECTORS_SIZE: usize = 4 * 1024;
pub const TEGRA_APB_MISC_BASE: usize = 0x70000000;
pub const TEGRA_APB_MISC_SIZE: usize = 4 * 1024;
pub const TEGRA_UARTA_BASE: usize = 0x70006000;
pub const TEGRA_UARTA_SIZE: usize = 64;
pub const TEGRA_UARTB_BASE: usize = 0x70006040;
pub const TEGRA_UARTB_SIZE: usize = 64;
pub const TEGRA_UARTC_BASE: usize = 0x70006200;
pub const TEGRA_UARTC_SIZE: usize = 256;
pub const TEGRA_UARTD_BASE: usize = 0x70006300;
pub const TEGRA_UARTD_SIZE: usize = 256;
pub const TEGRA_UARTE_BASE: usize = 0x70006400;
pub const TEGRA_UARTE_SIZE: usize = 256;
pub const TEGRA_PMC_BASE: usize = 0x7000E400;
pub const TEGRA_PMC_SIZE: usize = 256;
pub const TEGRA_EMC_BASE: usize = 0x7000F400;
pub const TEGRA_EMC_SIZE: usize = 1024;
pub const TEGRA_EMC0_BASE: usize = 0x7001A000;
pub const TEGRA_EMC0_SIZE: usize = 2 * 1024;
pub const TEGRA_EMC1_BASE: usize = 0x7001A800;
pub const TEGRA_EMC1_SIZE: usize = 2 * 1024;
pub const TEGRA124_EMC_BASE: usize = 0x7001B000;
pub const TEGRA124_EMC_SIZE: usize = 2 * 1024;
pub const TEGRA_CSITE_BASE: usize = 0x70040000;
pub const TEGRA_CSITE_SIZE: usize = 256 * 1024;

/* On TEGRA, many peripherals are very closely packed in
 * two 256MB io windows (that actually only use about 64KB
 * at the start of each).
 *
 * We will just map the first MMU section of each window (to minimize
 * pt entries needed) and provide a macro to transform physical
 * io addresses to an appropriate void __iomem *.
 */

pub const IO_IRAM_PHYS: usize = 0x40000000;
pub const IO_IRAM_VIRT: usize = 0xFE400000;
pub const IO_IRAM_SIZE: usize = 256 * 1024;
pub const IO_CPU_PHYS: usize = 0x50040000;
pub const IO_CPU_VIRT: usize = 0xFE440000;
pub const IO_CPU_SIZE: usize = 16 * 1024;
pub const IO_PPSB_PHYS: usize = 0x60000000;
pub const IO_PPSB_VIRT: usize = 0xFE200000;
// SECTION_SIZE is supplied by the MMU/page-table dependency.
pub const IO_PPSB_SIZE: usize = SECTION_SIZE;
pub const IO_APB_PHYS: usize = 0x70000000;
pub const IO_APB_VIRT: usize = 0xFE000000;
pub const IO_APB_SIZE: usize = SECTION_SIZE;

#[inline]
pub const fn io_to_virt_between(p: usize, st: usize, sz: usize) -> bool {
    p >= st && p < st + sz
}

#[inline]
pub const fn io_to_virt_xlate(p: usize, pst: usize, vst: usize) -> usize {
    p - pst + vst
}

#[inline]
pub const fn io_to_virt(n: usize) -> usize {
    if io_to_virt_between(n, IO_PPSB_PHYS, IO_PPSB_SIZE) {
        io_to_virt_xlate(n, IO_PPSB_PHYS, IO_PPSB_VIRT)
    } else if io_to_virt_between(n, IO_APB_PHYS, IO_APB_SIZE) {
        io_to_virt_xlate(n, IO_APB_PHYS, IO_APB_VIRT)
    } else if io_to_virt_between(n, IO_CPU_PHYS, IO_CPU_SIZE) {
        io_to_virt_xlate(n, IO_CPU_PHYS, IO_CPU_VIRT)
    } else if io_to_virt_between(n, IO_IRAM_PHYS, IO_IRAM_SIZE) {
        io_to_virt_xlate(n, IO_IRAM_PHYS, IO_IRAM_VIRT)
    } else {
        0
    }
}

#[inline]
pub const fn io_address(n: usize) -> usize {
    io_to_virt(n)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
