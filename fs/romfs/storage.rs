// SPDX-License-Identifier: GPL-2.0-or-later
/* RomFS storage access routines
 *
 * Copyright © 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// Kernel dependencies supplied by the surrounding translation unit.

#[cfg(all(not(feature = "CONFIG_ROMFS_ON_MTD"), not(feature = "CONFIG_ROMFS_ON_BLOCK")))]
compile_error!("no ROMFS backing store interface configured");

#[cfg(feature = "CONFIG_ROMFS_ON_MTD")]
unsafe fn romfs_mtd_read(sb: *mut super_block, mut pos: c_ulong,
                         buf: *mut c_void, buflen: usize) -> c_int {
    let mut rlen: usize = 0;
    let ret = mtd_read((*sb).s_mtd, pos, buflen, &mut rlen, buf);
    if ret < 0 || rlen != buflen { -EIO } else { 0 }
}

#[cfg(feature = "CONFIG_ROMFS_ON_MTD")]
unsafe fn romfs_mtd_strnlen(sb: *mut super_block, mut pos: c_ulong,
                            mut maxlen: usize) -> ssize_t {
    let mut n: ssize_t = 0;
    let mut buf = [0u8; 16];

    /* scan the string up to 16 bytes at a time */
    while maxlen > 0 {
        let segment = core::cmp::min(maxlen, 16);
        let mut len: usize = 0;
        let ret = mtd_read((*sb).s_mtd, pos, segment, &mut len, buf.as_mut_ptr() as *mut c_void);
        if ret < 0 { return ret as ssize_t; }
        let p = buf[..len].iter().position(|&v| v == 0);
        if let Some(i) = p { return n + i as ssize_t; }
        maxlen -= len;
        pos += len as c_ulong;
        n += len as ssize_t;
    }
    n
}

#[cfg(feature = "CONFIG_ROMFS_ON_MTD")]
unsafe fn romfs_mtd_strcmp(sb: *mut super_block, mut pos: c_ulong,
                           mut str_: *const c_char, mut size: usize) -> c_int {
    let mut buf = [0u8; 17];
    buf[0] = 0xff;

    /* scan the string up to 16 bytes at a time, and attempt to grab the
     * trailing NUL whilst we're at it */
    while size > 0 {
        let segment = core::cmp::min(size + 1, 17);
        let mut len: usize = 0;
        let ret = mtd_read((*sb).s_mtd, pos, segment, &mut len, buf.as_mut_ptr() as *mut c_void);
        if ret < 0 { return ret; }
        len -= 1;
        if core::slice::from_raw_parts(buf.as_ptr(), len)
            != core::slice::from_raw_parts(str_ as *const u8, len) { return 0; }
        buf[0] = buf[len];
        size -= len;
        pos += len as c_ulong;
        str_ = str_.add(len);
    }
    if buf[0] != 0 { return 0; }
    1
}

#[cfg(feature = "CONFIG_ROMFS_ON_BLOCK")]
unsafe fn romfs_blk_read(sb: *mut super_block, mut pos: c_ulong,
                         mut buf: *mut c_void, mut buflen: usize) -> c_int {
    /* copy the string up to blocksize bytes at a time */
    while buflen > 0 {
        let offset = pos & (ROMBSIZE - 1);
        let segment = core::cmp::min(buflen, (ROMBSIZE - offset) as usize);
        let bh = sb_bread(sb, pos >> ROMBSBITS);
        if bh.is_null() { return -EIO; }
        core::ptr::copy_nonoverlapping((*bh).b_data.add(offset as usize), buf as *mut u8, segment);
        brelse(bh);
        buf = (buf as *mut u8).add(segment) as *mut c_void;
        buflen -= segment;
        pos += segment as c_ulong;
    }
    0
}

#[cfg(feature = "CONFIG_ROMFS_ON_BLOCK")]
unsafe fn romfs_blk_strnlen(sb: *mut super_block, mut pos: c_ulong,
                            mut limit: usize) -> ssize_t {
    let mut n: ssize_t = 0;
    /* scan the string up to blocksize bytes at a time */
    while limit > 0 {
        let offset = pos & (ROMBSIZE - 1);
        let segment = core::cmp::min(limit, (ROMBSIZE - offset) as usize);
        let bh = sb_bread(sb, pos >> ROMBSBITS);
        if bh.is_null() { return -EIO; }
        let buf = (*bh).b_data.add(offset as usize);
        let found = core::slice::from_raw_parts(buf, segment).iter().position(|&v| v == 0);
        brelse(bh);
        if let Some(i) = found { return n + i as ssize_t; }
        limit -= segment;
        pos += segment as c_ulong;
        n += segment as ssize_t;
    }
    n
}

#[cfg(feature = "CONFIG_ROMFS_ON_BLOCK")]
unsafe fn romfs_blk_strcmp(sb: *mut super_block, mut pos: c_ulong,
                           mut str_: *const c_char, mut size: usize) -> c_int {
    let mut terminated = false;
    /* compare string up to a block at a time */
    while size > 0 {
        let offset = pos & (ROMBSIZE - 1);
        let segment = core::cmp::min(size, (ROMBSIZE - offset) as usize);
        let bh = sb_bread(sb, pos >> ROMBSBITS);
        if bh.is_null() { return -EIO; }
        let matched = core::slice::from_raw_parts((*bh).b_data.add(offset as usize), segment)
            == core::slice::from_raw_parts(str_ as *const u8, segment);
        size -= segment;
        pos += segment as c_ulong;
        str_ = str_.add(segment);
        let mut matched = matched;
        if matched && size == 0 && offset + segment as c_ulong < ROMBSIZE {
            if *(*bh).b_data.add((offset as usize) + segment) == 0 { terminated = true; }
            else { matched = false; }
        }
        brelse(bh);
        if !matched { return 0; }
    }
    if !terminated {
        BUG_ON((pos & (ROMBSIZE - 1)) != 0);
        let bh = sb_bread(sb, pos >> ROMBSBITS);
        if bh.is_null() { return -EIO; }
        let matched = (*(*bh).b_data) == 0;
        brelse(bh);
        if !matched { return 0; }
    }
    1
}

pub unsafe fn romfs_dev_read(sb: *mut super_block, pos: c_ulong,
                             buf: *mut c_void, buflen: usize) -> c_int {
    let limit = romfs_maxsize(sb);
    if pos >= limit || buflen > limit - pos { return -EIO; }
    #[cfg(feature = "CONFIG_ROMFS_ON_MTD")]
    if !(*sb).s_mtd.is_null() { return romfs_mtd_read(sb, pos, buf, buflen); }
    #[cfg(feature = "CONFIG_ROMFS_ON_BLOCK")]
    if !(*sb).s_bdev.is_null() { return romfs_blk_read(sb, pos, buf, buflen); }
    -EIO
}

pub unsafe fn romfs_dev_strnlen(sb: *mut super_block, pos: c_ulong,
                                mut maxlen: usize) -> ssize_t {
    let limit = romfs_maxsize(sb);
    if pos >= limit { return -EIO; }
    if maxlen > limit - pos { maxlen = limit - pos; }
    #[cfg(feature = "CONFIG_ROMFS_ON_MTD")]
    if !(*sb).s_mtd.is_null() { return romfs_mtd_strnlen(sb, pos, maxlen); }
    #[cfg(feature = "CONFIG_ROMFS_ON_BLOCK")]
    if !(*sb).s_bdev.is_null() { return romfs_blk_strnlen(sb, pos, maxlen); }
    -EIO
}

pub unsafe fn romfs_dev_strcmp(sb: *mut super_block, pos: c_ulong,
                               str_: *const c_char, size: usize) -> c_int {
    let limit = romfs_maxsize(sb);
    if pos >= limit { return -EIO; }
    if size > ROMFS_MAXFN { return -ENAMETOOLONG; }
    if size + 1 > limit - pos { return -EIO; }
    #[cfg(feature = "CONFIG_ROMFS_ON_MTD")]
    if !(*sb).s_mtd.is_null() { return romfs_mtd_strcmp(sb, pos, str_, size); }
    #[cfg(feature = "CONFIG_ROMFS_ON_BLOCK")]
    if !(*sb).s_bdev.is_null() { return romfs_blk_strcmp(sb, pos, str_, size); }
    -EIO
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
