// SPDX-License-Identifier: GPL-2.0-only
/* ----------------------------------------------------------------------- */
//
//   Copyright 2012 Intel Corporation; author H. Peter Anvin
//
// -----------------------------------------------------------------------

/*
 * earlycpio.c
 *
 * Find a specific cpio member; must precede any compressed content.
 * This is used to locate data items in the initramfs used by the
 * kernel itself during early boot (before the main initramfs is
 * decompressed.)  It is the responsibility of the initramfs creator
 * to ensure that these items are uncompressed at the head of the
 * blob.  Depending on the boot loader or package tool that may be a
 * separate file or part of the same file.
 */

use core::ffi::{c_char, c_void};

// Supplied by the corresponding kernel headers/dependencies.
pub const MAX_CPIO_FILE_NAME: usize = 256;

#[repr(C)]
pub struct cpio_data {
    pub data: *mut c_void,
    pub size: usize,
    pub name: [c_char; MAX_CPIO_FILE_NAME],
}

#[repr(i32)]
enum cpio_fields {
    C_MAGIC,
    C_INO,
    C_MODE,
    C_UID,
    C_GID,
    C_NLINK,
    C_MTIME,
    C_FILESIZE,
    C_MAJ,
    C_MIN,
    C_RMAJ,
    C_RMIN,
    C_NAMESIZE,
    C_CHKSUM,
    C_NFIELDS,
}

extern "C" {
    fn strlen(s: *const c_char) -> usize;
    fn memcmp(a: *const c_void, b: *const c_void, n: usize) -> i32;
    fn strscpy(dst: *mut c_char, src: *const c_char, count: usize) -> isize;
}

#[inline]
unsafe fn ptr_align(p: *const u8, align: usize) -> *const u8 {
    ((p as usize + align - 1) & !(align - 1)) as *const u8
}

extern "C" {
    fn pr_warn(fmt: *const c_char, ...);
}

/**
 * find_cpio_data - Search for files in an uncompressed cpio
 * @path:       The directory to search for, including a slash at the end
 * @data:       Pointer to the cpio archive or a header inside
 * @len:        Remaining length of the cpio based on data pointer
 * @nextoff:    When a matching file is found, this is the offset from the
 *              beginning of the cpio to the beginning of the next file, not the
 *              matching file itself. It can be used to iterate through the cpio
 *              to find all files inside of a directory path.
 *
 * Return:      &struct cpio_data containing the address, length and
 *              filename (with the directory path cut off) of the found file.
 *              If you search for a filename and not for files in a directory,
 *              pass the absolute path of the filename in the cpio and make sure
 *              the match returned an empty filename string.
 */
pub unsafe fn find_cpio_data(
    path: *const c_char,
    data: *mut c_void,
    mut len: usize,
    nextoff: *mut i64,
) -> cpio_data {
    let cpio_header_len = 8 * cpio_fields::C_NFIELDS as usize - 2;
    let mut cd = cpio_data {
        data: core::ptr::null_mut(),
        size: 0,
        name: [0; MAX_CPIO_FILE_NAME],
    };
    let mut p = data as *const u8;
    let mypathsize = strlen(path);
    let mut ch = [0u32; cpio_fields::C_NFIELDS as usize];

    while len > cpio_header_len {
        if *p == 0 {
            p = p.add(4);
            len -= 4;
            continue;
        }

        let mut j = 6;
        let mut invalid = false;
        for field in 0..cpio_fields::C_NFIELDS as usize {
            let mut v = 0u32;
            while j != 0 {
                v <<= 4;
                let c = *p;
                p = p.add(1);
                let x = c.wrapping_sub(b'0');
                if x < 10 {
                    v += x as u32;
                    j -= 1;
                    continue;
                }
                let x = (c | 0x20).wrapping_sub(b'a');
                if x < 6 {
                    v += x as u32 + 10;
                    j -= 1;
                    continue;
                }
                invalid = true;
                break;
            }
            if invalid { break; }
            ch[field] = v;
            j = 8;
        }
        if invalid || ch[cpio_fields::C_MAGIC as usize].wrapping_sub(0x070701) > 1 {
            break;
        }

        len -= cpio_header_len;
        let dptr = ptr_align(p.add(ch[cpio_fields::C_NAMESIZE as usize] as usize), 4);
        let nptr = ptr_align(dptr.add(ch[cpio_fields::C_FILESIZE as usize] as usize), 4);
        if nptr > p.add(len) || dptr < p || nptr < dptr { break; }

        if (ch[cpio_fields::C_MODE as usize] & 0o170000) == 0o100000
            && ch[cpio_fields::C_NAMESIZE as usize] as usize >= mypathsize
            && memcmp(p as *const c_void, path as *const c_void, mypathsize) == 0
        {
            if !nextoff.is_null() { *nextoff = nptr as isize as i64 - data as isize as i64; }
            if ch[cpio_fields::C_NAMESIZE as usize] as usize - mypathsize >= MAX_CPIO_FILE_NAME {
                pr_warn(b"File %s exceeding MAX_CPIO_FILE_NAME [%d]\n\0".as_ptr() as *const c_char, p, MAX_CPIO_FILE_NAME as i32);
            }
            strscpy(cd.name.as_mut_ptr(), p.add(mypathsize) as *const c_char, MAX_CPIO_FILE_NAME);
            cd.data = dptr as *mut c_void;
            cd.size = ch[cpio_fields::C_FILESIZE as usize] as usize;
            return cd;
        }
        len -= nptr.offset_from(p) as usize;
        p = nptr;
    }
    cd
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
