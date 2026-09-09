/* SPDX-License-Identifier: ((GPL-2.0 WITH Linux-syscall-note) OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 */

/**
 * SOF ABI versioning is based on Semantic Versioning where we have a given
 * MAJOR.MINOR.PATCH version number. See https://semver.org/
 *
 * Rules for incrementing or changing version :-
 *
 * 1) Increment MAJOR version if you make incompatible API changes. MINOR and
 *    PATCH should be reset to 0.
 *
 * 2) Increment MINOR version if you add backwards compatible features or
 *    changes. PATCH should be reset to 0.
 *
 * 3) Increment PATCH version if you add backwards compatible bug fixes.
 */

// Dependency: <linux/types.h>

/* SOF ABI version major, minor and patch numbers */
pub const SOF_ABI_MAJOR: u32 = 3;
pub const SOF_ABI_MINOR: u32 = 23;
pub const SOF_ABI_PATCH: u32 = 1;

/* SOF ABI version number. Format within 32bit word is MMmmmppp */
pub const SOF_ABI_MAJOR_SHIFT: u32 = 24;
pub const SOF_ABI_MAJOR_MASK: u32 = 0xff;
pub const SOF_ABI_MINOR_SHIFT: u32 = 12;
pub const SOF_ABI_MINOR_MASK: u32 = 0xfff;
pub const SOF_ABI_PATCH_SHIFT: u32 = 0;
pub const SOF_ABI_PATCH_MASK: u32 = 0xfff;

pub const fn SOF_ABI_VER(major: u32, minor: u32, patch: u32) -> u32 {
    (major << SOF_ABI_MAJOR_SHIFT)
        | (minor << SOF_ABI_MINOR_SHIFT)
        | (patch << SOF_ABI_PATCH_SHIFT)
}

pub const fn SOF_ABI_VERSION_MAJOR(version: u32) -> u32 {
    (version >> SOF_ABI_MAJOR_SHIFT) & SOF_ABI_MAJOR_MASK
}

pub const fn SOF_ABI_VERSION_MINOR(version: u32) -> u32 {
    (version >> SOF_ABI_MINOR_SHIFT) & SOF_ABI_MINOR_MASK
}

pub const fn SOF_ABI_VERSION_PATCH(version: u32) -> u32 {
    (version >> SOF_ABI_PATCH_SHIFT) & SOF_ABI_PATCH_MASK
}

pub const fn SOF_ABI_VERSION_INCOMPATIBLE(sof_ver: u32, client_ver: u32) -> bool {
    SOF_ABI_VERSION_MAJOR(sof_ver) != SOF_ABI_VERSION_MAJOR(client_ver)
}

pub const SOF_ABI_VERSION: u32 = SOF_ABI_VER(SOF_ABI_MAJOR, SOF_ABI_MINOR, SOF_ABI_PATCH);

/* SOF ABI magic number "SOF\0". */
pub const SOF_ABI_MAGIC: u32 = 0x00464F53;
/* SOF IPC4 ABI magic number "SOF4". */
pub const SOF_IPC4_ABI_MAGIC: u32 = 0x34464F53;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
