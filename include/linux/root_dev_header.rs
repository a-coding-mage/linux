/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies corresponding to <linux/major.h>, <linux/types.h>, and
// <linux/kdev_t.h> are supplied by other translated files.

// MKDEV(major, minor) encodes a device number as (major << 20) | minor.
// The referenced major constants and `dev_t` remain external dependencies.
pub const Root_NFS: dev_t = ((UNNAMED_MAJOR as dev_t) << 20) | 255;
pub const Root_CIFS: dev_t = ((UNNAMED_MAJOR as dev_t) << 20) | 254;
pub const Root_Generic: dev_t = ((UNNAMED_MAJOR as dev_t) << 20) | 253;
pub const Root_RAM0: dev_t = ((RAMDISK_MAJOR as dev_t) << 20) | 0;

extern "C" {
    pub static mut ROOT_DEV: dev_t;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
