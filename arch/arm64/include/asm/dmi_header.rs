/*
 * arch/arm64/include/asm/dmi.h
 *
 * Copyright (C) 2013 Linaro Limited.
 * Written by: Yi Li (yi.li@linaro.org)
 *
 * based on arch/ia64/include/asm/dmi.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 */

// C header guard: __ASM_DMI_H

// Dependency intent from <linux/io.h> and <linux/slab.h> is preserved through
// the referenced external symbols below.

/*
 * According to section 2.3.6 of the UEFI spec, the firmware should not
 * request a virtual mapping for configuration tables such as SMBIOS.
 * This means we have to map them before use.
 */
macro_rules! dmi_early_remap {
    ($x:expr, $l:expr) => {
        ioremap_cache($x, $l)
    };
}

macro_rules! dmi_early_unmap {
    ($x:expr, $l:expr) => {
        iounmap($x)
    };
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

macro_rules! dmi_alloc {
    ($l:expr) => {
        kzalloc($l, GFP_KERNEL)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
