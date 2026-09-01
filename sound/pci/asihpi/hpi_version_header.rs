/* SPDX-License-Identifier: GPL-2.0 */
/** HPI Version Definitions
Development releases have odd minor version.
Production releases have even minor version.

\file hpi_version.h
*/

/* Use single digits for versions less that 10 to avoid octal. */
/* *** HPI_VER is the only edit required to update version *** */
/** HPI version */
pub const HPI_VER: i32 = HPI_VERSION_CONSTRUCTOR(4, 14, 3);

/** HPI version string in dotted decimal format */
pub const HPI_VER_STRING: &[u8; 8] = b"4.14.03\0";

/** Library version as documented in hpi-api-versions.txt */
pub const HPI_LIB_VER: i32 = HPI_VERSION_CONSTRUCTOR(10, 4, 0);

/** Construct hpi version number from major, minor, release numbers */
pub const fn HPI_VERSION_CONSTRUCTOR(maj: i32, min: i32, r: i32) -> i32 {
    (maj << 16) + (min << 8) + r
}

/** Extract major version from hpi version number */
pub const fn HPI_VER_MAJOR(v: i32) -> i32 {
    v >> 16
}

/** Extract minor version from hpi version number */
pub const fn HPI_VER_MINOR(v: i32) -> i32 {
    (v >> 8) & 0xFF
}

/** Extract release from hpi version number */
pub const fn HPI_VER_RELEASE(v: i32) -> i32 {
    v & 0xFF
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
