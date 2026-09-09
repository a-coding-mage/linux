/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2024 Intel Corporation
 *
 * based on arch/arm64/include/asm/dmi.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// Dependencies supplied by the surrounding kernel translation:
// linux/io.h and linux/slab.h

#[macro_export]
macro_rules! dmi_early_remap {
    ($x:expr, $l:expr) => {
        memremap($x, $l, MEMREMAP_WB)
    };
}

#[macro_export]
macro_rules! dmi_early_unmap {
    ($x:expr, $l:expr) => {
        memunmap($x)
    };
}

#[macro_export]
macro_rules! dmi_remap {
    ($x:expr, $l:expr) => {
        memremap($x, $l, MEMREMAP_WB)
    };
}

#[macro_export]
macro_rules! dmi_unmap {
    ($x:expr) => {
        memunmap($x)
    };
}

#[macro_export]
macro_rules! dmi_alloc {
    ($l:expr) => {
        kzalloc($l, GFP_KERNEL)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
