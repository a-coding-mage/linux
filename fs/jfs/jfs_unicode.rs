// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Copyright (C) International Business Machines Corp., 2000-2004
 */

// Linux kernel headers and JFS headers are supplied by the surrounding
// translation unit.

/*
 * NAME: jfs_strfromUCS()
 *
 * FUNCTION: Convert little-endian unicode string to character string
 */
pub unsafe fn jfs_strfromUCS_le(
    to: *mut core::ffi::c_char,
    from: *const u16,
    len: i32,
    codepage: *mut nls_table,
) -> i32 {
    let mut i: i32;
    let mut outlen: i32 = 0;
    static mut WARN_AGAIN: i32 = 5; /* Only warn up to 5 times total */
    let mut warn: i32 = if WARN_AGAIN != 0 { 1 } else { 0 }; /* once per string */

    if !codepage.is_null() {
        i = 0;
        while i < len && *from.add(i as usize) != 0 {
            let charlen: i32 = (*codepage).uni2char(
                u16::from_le(*from.add(i as usize)),
                to.add(outlen as usize),
                NLS_MAX_CHARSET_SIZE,
            );
            if charlen > 0 {
                outlen += charlen;
            } else {
                *to.add(outlen as usize) = b'?' as core::ffi::c_char;
                outlen += 1;
            }
            i += 1;
        }
    } else {
        i = 0;
        while i < len && *from.add(i as usize) != 0 {
            let value = u16::from_le(*from.add(i as usize));
            if value & 0xff00 != 0 {
                *to.add(i as usize) = b'?' as core::ffi::c_char;
                if warn != 0 {
                    warn -= 1;
                    WARN_AGAIN -= 1;
                    printk(KERN_ERR, "non-latin1 character 0x%x found in JFS file name\n", value);
                    printk(KERN_ERR, "mount with iocharset=utf8 to access\n");
                }
            } else {
                *to.add(i as usize) = value as u8 as core::ffi::c_char;
            }
            i += 1;
        }
        outlen = i;
    }
    *to.add(outlen as usize) = 0;
    outlen
}

/*
 * NAME: jfs_strtoUCS()
 *
 * FUNCTION: Convert character string to unicode string
 */
unsafe fn jfs_strtoUCS(
    to: *mut u16,
    mut from: *const u8,
    mut len: i32,
    codepage: *mut nls_table,
) -> i32 {
    let mut charlen: i32;
    let mut i: i32;

    if !codepage.is_null() {
        i = 0;
        while len != 0 && *from != 0 {
            charlen = (*codepage).char2uni(from, len, to.add(i as usize));
            if charlen < 1 {
                jfs_err!("jfs_strtoUCS: char2uni returned {}.", charlen);
                jfs_err!("charset = {}, char = 0x{:x}", (*codepage).charset, *from);
                return charlen;
            }
            i += 1;
            from = from.add(charlen as usize);
            len -= charlen;
        }
    } else {
        i = 0;
        while i < len && *from.add(i as usize) != 0 {
            *to.add(i as usize) = *from.add(i as usize) as u16;
            i += 1;
        }
    }

    *to.add(i as usize) = 0;
    i
}

/*
 * NAME: get_UCSname()
 *
 * FUNCTION: Allocate and translate to unicode string
 */
pub unsafe fn get_UCSname(uniName: *mut component_name, dentry: *mut dentry) -> i32 {
    let nls_tab: *mut nls_table = JFS_SBI((*dentry).d_sb).nls_tab;
    let length: i32 = (*dentry).d_name.len;

    if length > JFS_NAME_MAX {
        return -ENAMETOOLONG;
    }

    (*uniName).name = kmalloc_array(length + 1, core::mem::size_of::<u16>(), GFP_NOFS);
    if (*uniName).name.is_null() {
        return -ENOMEM;
    }

    (*uniName).namlen = jfs_strtoUCS(
        (*uniName).name,
        (*dentry).d_name.name,
        length,
        nls_tab,
    );
    if (*uniName).namlen < 0 {
        kfree((*uniName).name as *mut core::ffi::c_void);
        return (*uniName).namlen;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
