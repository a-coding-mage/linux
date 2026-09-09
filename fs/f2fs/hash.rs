// SPDX-License-Identifier: GPL-2.0
/*
 * fs/f2fs/hash.c
 *
 * Copyright (c) 2012 Samsung Electronics Co., Ltd.
 *             http://www.samsung.com/
 *
 * Portions of this code from linux/fs/ext3/hash.c
 *
 * Copyright (C) 2002 by Theodore Ts'o
 */

// Dependencies supplied by the surrounding kernel/f2fs translation.

/*
 * Hashing code copied from ext3
 */
const DELTA: u32 = 0x9E3779B9;

unsafe fn TEA_transform(buf: *mut u32, input: *const u32) {
    let mut sum: u32 = 0;
    let mut b0 = *buf.add(0);
    let mut b1 = *buf.add(1);
    let a = *input.add(0);
    let b = *input.add(1);
    let c = *input.add(2);
    let d = *input.add(3);
    let mut n: i32 = 16;

    loop {
        sum = sum.wrapping_add(DELTA);
        b0 = b0.wrapping_add(
            ((b1 << 4).wrapping_add(a)) ^
            (b1.wrapping_add(sum)) ^
            ((b1 >> 5).wrapping_add(b)),
        );
        b1 = b1.wrapping_add(
            ((b0 << 4).wrapping_add(c)) ^
            (b0.wrapping_add(sum)) ^
            ((b0 >> 5).wrapping_add(d)),
        );
        n -= 1;
        if n == 0 {
            break;
        }
    }

    *buf.add(0) = (*buf.add(0)).wrapping_add(b0);
    *buf.add(1) = (*buf.add(1)).wrapping_add(b1);
}

unsafe fn str2hashbuf(msg: *const u8, mut len: usize, buf: *mut u32, mut num: i32) {
    let mut pad: u32 = len as u32 | ((len as u32) << 8);
    pad |= pad << 16;

    let mut val = pad;
    if len > (num as usize) * 4 {
        len = (num as usize) * 4;
    }
    let mut i = 0usize;
    while i < len {
        if (i % 4) == 0 {
            val = pad;
        }
        val = (*msg.add(i) as u32).wrapping_add(val << 8);
        if (i % 4) == 3 {
            *buf = val;
            buf = buf.add(1);
            val = pad;
            num -= 1;
        }
        i += 1;
    }
    num -= 1;
    if num >= 0 {
        *buf = val;
        buf = buf.add(1);
    }
    while num >= 0 {
        *buf = pad;
        buf = buf.add(1);
        num -= 1;
    }
}

unsafe fn TEA_hash_name(mut p: *const u8, mut len: usize) -> u32 {
    let mut input = [0u32; 8];
    let mut buf = [0u32; 4];

    /* Initialize the default seed for the hash checksum functions */
    buf[0] = 0x67452301;
    buf[1] = 0xefcdab89;
    buf[2] = 0x98badcfe;
    buf[3] = 0x10325476;

    loop {
        str2hashbuf(p, len, input.as_mut_ptr(), 4);
        TEA_transform(buf.as_mut_ptr(), input.as_ptr());
        p = p.add(16);
        if len <= 16 {
            break;
        }
        len -= 16;
    }
    buf[0] & !F2FS_HASH_COL_BIT
}

/*
 * Compute @fname->hash.  For all directories, @fname->disk_name must be set.
 * For casefolded directories, @fname->usr_fname must be set, and also
 * @fname->cf_name if the filename is valid Unicode and is not "." or "..".
 */
pub unsafe fn f2fs_hash_filename(
    dir: *const crate::inode,
    fname: *mut crate::f2fs_filename,
) {
    let mut name = (*fname).disk_name.name;
    let mut len = (*fname).disk_name.len;

    WARN_ON_ONCE(name.is_null());

    if name_is_dot_dotdot(name, len) {
        (*fname).hash = 0;
        return;
    }

    // Preserves the source's CONFIG_UNICODE conditional compilation.
    #[cfg(CONFIG_UNICODE)]
    if IS_CASEFOLDED(dir) {
        /*
         * If the casefolded name is provided, hash it instead of the
         * on-disk name.  If the casefolded name is *not* provided, that
         * should only be because the name wasn't valid Unicode or was
         * "." or "..", so fall back to treating the name as an opaque
         * byte sequence.  Note that to handle encrypted directories,
         * the fallback must use usr_fname (plaintext) rather than
         * disk_name (ciphertext).
         */
        WARN_ON_ONCE((*fname).usr_fname.name.is_null());
        if !(*fname).cf_name.name.is_null() {
            name = (*fname).cf_name.name;
            len = (*fname).cf_name.len;
        } else {
            name = (*fname).usr_fname.name;
            len = (*fname).usr_fname.len;
        }
        if IS_ENCRYPTED(dir) {
            let tmp = QSTR_INIT(name, len);
            (*fname).hash = cpu_to_le32(fscrypt_fname_siphash(dir, &tmp));
            return;
        }
    }

    (*fname).hash = cpu_to_le32(TEA_hash_name(name, len));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
