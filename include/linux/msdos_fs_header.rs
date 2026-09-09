/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent: declarations from <uapi/linux/msdos_fs.h> are supplied
// by the surrounding translation unit.

/* media of boot sector */
#[inline]
fn fat_valid_media(media: u8) -> i32 {
    if 0xf8 <= media || media == 0xf0 {
        1
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
