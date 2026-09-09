/* SPDX-License-Identifier: GPL-2.0 */

// Dependency intent preserved from: #include <uapi/linux/adfs_fs.h>

/*
 * Calculate the boot block checksum on an ADFS drive.  Note that this will
 * appear to be correct if the sector contains all zeros, so also check that
 * the disk size is non-zero!!!
 */
pub unsafe fn adfs_checkbblk(ptr: *mut u8) -> i32 {
    let mut result: u32 = 0;
    let mut p: *mut u8 = ptr.add(511);

    loop {
        result = (result & 0xff).wrapping_add(result >> 8);
        p = p.sub(1);
        result = result.wrapping_add(*p as u32);
        if p == ptr {
            break;
        }
    }

    if (result & 0xff) != *ptr.add(511) as u32 {
        1
    } else {
        0
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
