/*
 *  linux/fs/hfs/trans.c
 *
 * Copyright (C) 1995-1997  Paul H. Hargrove
 * This file may be distributed under the terms of the GNU General Public License.
 *
 * This file contains routines for converting between the Macintosh
 * character set and various other encodings.  This includes dealing
 * with ':' vs. '/' as the path-element separator.
 */

// Kernel/HFS declarations and the HFS_SB accessor are supplied by other files.

pub unsafe fn hfs_mac2asc(
    sb: *mut super_block,
    out: *mut ::core::ffi::c_char,
    input: *const hfs_name,
) -> i32 {
    let nls_disk = (*HFS_SB(sb)).nls_disk;
    let nls_io = (*HFS_SB(sb)).nls_io;
    let mut src = (*input).name as *const u8;
    let mut srclen = (*input).len as i32;
    if srclen > HFS_NAMELEN {
        srclen = HFS_NAMELEN;
    }
    let mut dst = out as *mut u8;
    let mut dstlen = HFS_MAX_NAMELEN;
    let mut size: i32;

    if !nls_io.is_null() {
        let mut ch: u16;
        while srclen > 0 {
            if !nls_disk.is_null() {
                size = ((*nls_disk).char2uni)(src as *const _, srclen, &mut ch);
                if size <= 0 {
                    ch = b'?' as u16;
                    size = 1;
                }
                src = src.add(size as usize);
                srclen -= size;
            } else {
                ch = *src as u16;
                src = src.add(1);
                srclen -= 1;
            }
            if ch == b'/' as u16 {
                ch = b':' as u16;
            }
            size = ((*nls_io).uni2char)(ch, dst as *mut _, dstlen);
            if size < 0 {
                if size == -ENAMETOOLONG {
                    break;
                }
                *dst = b'?';
                size = 1;
            }
            dst = dst.add(size as usize);
            dstlen -= size;
        }
    } else {
        while srclen > 0 {
            let ch = *src;
            src = src.add(1);
            *dst = if ch == b'/' { b':' } else { ch };
            dst = dst.add(1);
            srclen -= 1;
        }
    }
    dst.offset_from(out as *mut u8) as i32
}

pub unsafe fn hfs_asc2mac(
    sb: *mut super_block,
    out: *mut hfs_name,
    input: *const qstr,
) {
    let nls_disk = (*HFS_SB(sb)).nls_disk;
    let nls_io = (*HFS_SB(sb)).nls_io;
    let mut src = (*input).name as *const u8;
    let mut srclen = (*input).len as i32;
    let mut dst = (*out).name as *mut u8;
    let mut dstlen = HFS_NAMELEN;
    let mut size: i32;

    if !nls_io.is_null() {
        let mut ch: u16;
        while srclen > 0 && dstlen > 0 {
            size = ((*nls_io).char2uni)(src as *const _, srclen, &mut ch);
            if size < 0 {
                ch = b'?' as u16;
                size = 1;
            }
            src = src.add(size as usize);
            srclen -= size;
            if ch == b':' as u16 {
                ch = b'/' as u16;
            }
            if !nls_disk.is_null() {
                size = ((*nls_disk).uni2char)(ch, dst as *mut _, dstlen);
                if size < 0 {
                    if size == -ENAMETOOLONG {
                        break;
                    }
                    *dst = b'?';
                    size = 1;
                }
                dst = dst.add(size as usize);
                dstlen -= size;
            } else {
                *dst = if ch > 0xff { b'?' } else { ch as u8 };
                dst = dst.add(1);
                dstlen -= 1;
            }
        }
    } else {
        if dstlen > srclen {
            dstlen = srclen;
        }
        while dstlen > 0 {
            let ch = *src;
            src = src.add(1);
            *dst = if ch == b':' { b'/' } else { ch };
            dst = dst.add(1);
            dstlen -= 1;
        }
    }
    (*out).len = dst.offset_from((*out).name as *mut u8) as _;
    dstlen = HFS_NAMELEN - (*out).len as i32;
    while dstlen > 0 {
        *dst = 0;
        dst = dst.add(1);
        dstlen -= 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
