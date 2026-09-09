/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the surrounding kernel translation:
// `ioremap`, `iounmap`, `ioremap_cache`, `memblock_alloc_low`, and `PAGE_SIZE`.

macro_rules! dmi_early_remap {
    ($x:expr, $l:expr) => {
        ioremap($x, $l)
    };
}

macro_rules! dmi_early_unmap {
    ($x:expr, $l:expr) => {{
        let _ = $l;
        iounmap($x)
    }};
}

macro_rules! dmi_remap {
    ($x:expr, $l:expr) => {
        ioremap_cache($x, $l)
    };
}

macro_rules! dmi_unmap {
    ($x:expr) => {
        iounmap($x)
    };
}

// MIPS initialize DMI scan before SLAB is ready, so we use memblock here.
macro_rules! dmi_alloc {
    ($l:expr) => {
        memblock_alloc_low($l, PAGE_SIZE)
    };
}

// Equivalent to CONFIG_MACH_LOONGSON64 in the source build configuration.
#[cfg(CONFIG_MACH_LOONGSON64)]
pub const SMBIOS_ENTRY_POINT_SCAN_START: usize = 0xFFFE000;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
