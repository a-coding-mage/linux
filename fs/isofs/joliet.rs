// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/isofs/joliet.c
 *
 *  (C) 1996 Gordon Chaffee
 *
 *  Joliet: Microsoft's Unicode extensions to iso9660
 */

// Dependencies supplied by the surrounding kernel/isofs translation unit.

/* Convert Unicode 16 to UTF-8 or ASCII. */
unsafe fn uni16_to_x8(
    ascii: *mut u8,
    uni: *mut __be16,
    mut len: i32,
    nls: *mut nls_table,
) -> i32 {
    let mut ip = uni;
    let mut op = ascii;

    while {
        let ch = get_unaligned(ip);
        ch != 0 && len != 0
    } {
        let ch = get_unaligned(ip);
        let llen = ((*nls).uni2char)(
            be16_to_cpu(ch),
            op,
            NLS_MAX_CHARSET_SIZE,
        );
        if llen > 0 {
            op = op.add(llen as usize);
        } else {
            *op = b'?';
            op = op.add(1);
        }
        ip = ip.add(1);

        len -= 1;
    }
    *op = 0;
    op.offset_from(ascii) as i32
}

pub unsafe fn get_joliet_filename(
    de: *mut iso_directory_record,
    outname: *mut u8,
    inode: *mut inode,
) -> u8 {
    let nls: *mut nls_table;
    let mut len: u8 = 0;

    nls = (*ISOFS_SB((*inode).i_sb)).s_nls_iocharset;

    if nls.is_null() {
        len = utf16s_to_utf8s(
            (*de).name as *const wchar_t,
            ((*de).name_len[0] >> 1) as usize,
            UTF16_BIG_ENDIAN,
            outname,
            PAGE_SIZE,
        ) as u8;
    } else {
        len = uni16_to_x8(
            outname,
            (*de).name as *mut __be16,
            ((*de).name_len[0] >> 1) as i32,
            nls,
        ) as u8;
    }
    if (len > 2) && (*outname.add((len - 2) as usize) == b';') && (*outname.add((len - 1) as usize) == b'1') {
        len -= 2;
    }

    /*
     * Windows doesn't like periods at the end of a name,
     * so neither do we
     */
    while (len >= 2) && (*outname.add((len - 1) as usize) == b'.') {
        len -= 1;
    }

    len
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
