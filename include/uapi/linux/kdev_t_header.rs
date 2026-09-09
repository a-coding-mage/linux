/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * The original header excludes these externally visible definitions when
 * __KERNEL__ is defined. Preserve that build-time condition at the use site.
 */

/// Extract the major device number.
#[cfg(not(__KERNEL__))]
#[macro_export]
macro_rules! MAJOR {
    ($dev:expr) => {
        (($dev) >> 8)
    };
}

/// Extract the minor device number.
#[cfg(not(__KERNEL__))]
#[macro_export]
macro_rules! MINOR {
    ($dev:expr) => {
        (($dev) & 0xff)
    };
}

/// Construct a device number from major and minor numbers.
#[cfg(not(__KERNEL__))]
#[macro_export]
macro_rules! MKDEV {
    ($ma:expr, $mi:expr) => {
        (($ma) << 8 | ($mi))
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
