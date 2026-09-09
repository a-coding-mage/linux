/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Defines for sysctl handling in NTFS Linux kernel driver.
 *
 * Copyright (C) 1997 Martin von Löwis, Régis Duchesne
 * Copyright (c) 2002-2004 Anton Altaparmakov
 */

/*
 * The C header guard and include directives have no executable Rust
 * equivalent.
 */

#[cfg(all(feature = "DEBUG", feature = "CONFIG_SYSCTL"))]
unsafe extern "C" {
    pub fn ntfs_sysctl(add: i32) -> i32;
}

#[cfg(not(all(feature = "DEBUG", feature = "CONFIG_SYSCTL")))]
/* Just return success. */
pub fn ntfs_sysctl(_add: i32) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
