/* SPDX-License-Identifier: GPL-2.0-only */

// C header guard: __ASM_DMI_H
// Dependencies supplied by the surrounding kernel translation:
// linux/io.h and linux/slab.h

// #define dmi_early_remap(x, l) memremap(x, l, MEMREMAP_WB)
#[macro_export]
macro_rules! dmi_early_remap {
    ($x:expr, $l:expr) => {
        memremap($x, $l, MEMREMAP_WB)
    };
}

// #define dmi_early_unmap(x, l) memunmap(x)
#[macro_export]
macro_rules! dmi_early_unmap {
    ($x:expr, $l:expr) => {
        memunmap($x)
    };
}

// #define dmi_remap(x, l) memremap(x, l, MEMREMAP_WB)
#[macro_export]
macro_rules! dmi_remap {
    ($x:expr, $l:expr) => {
        memremap($x, $l, MEMREMAP_WB)
    };
}

// #define dmi_unmap(x) memunmap(x)
#[macro_export]
macro_rules! dmi_unmap {
    ($x:expr) => {
        memunmap($x)
    };
}

// #define dmi_alloc(l) kzalloc(l, GFP_KERNEL)
#[macro_export]
macro_rules! dmi_alloc {
    ($l:expr) => {
        kzalloc($l, GFP_KERNEL)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
