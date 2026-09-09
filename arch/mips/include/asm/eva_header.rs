/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License. See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2014, Imagination Technologies Ltd.
 *
 * EVA functions for generic code
 */

// The C header includes <kernel-entry-init.h>, which supplies the
// platform_eva_init macro. That dependency is provided by the surrounding
// translation unit.

// The original definitions are active only for assembler sources
// (__ASSEMBLER__). The Rust macro below preserves that source-level interface.

#[cfg(feature = "config_eva")]
macro_rules! eva_init {
    () => {
        platform_eva_init!();
    };
}

#[cfg(not(feature = "config_eva"))]
macro_rules! eva_init {
    () => {};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
