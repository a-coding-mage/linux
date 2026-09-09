// SPDX-License-Identifier: GPL-2.0-or-later
/*
 *   Some of the source code in this file came from fs/cifs/cifs_unicode.c
 *
 *   Copyright (c) International Business Machines  Corp., 2000,2009
 *   Modified by Steve French (sfrench@us.ibm.com)
 *   Modified by Namjae Jeon (linkinjeon@kernel.org)
 */

// Linux/kernel and local headers provide the types, constants, and functions
// referenced below.

unsafe fn cifs_mapchar(
    target: *mut i8,
    from: *const u16,
    cp: *const nls_table,
    mapchar: bool,
) -> i32 {
    let mut len: i32 = 1;
    let src_char = *from;

    if !mapchar {
        goto_cp_convert(target, from, cp, &mut len);
        return len;
    }

    match src_char {
        UNI_COLON => *target = b':' as i8,
        UNI_ASTERISK => *target = b'*' as i8,
        UNI_QUESTION => *target = b'?' as i8,
        UNI_PIPE => *target = b'|' as i8,
        UNI_GRTRTHAN => *target = b'>' as i8,
        UNI_LESSTHAN => *target = b'<' as i8,
        _ => {
            goto_cp_convert(target, from, cp, &mut len);
            return len;
        }
    }
    len
}

unsafe fn goto_cp_convert(target: *mut i8, from: *const u16, cp: *const nls_table, len: &mut i32) {
    *len = ((*cp).uni2char)((*from), target, NLS_MAX_CHARSET_SIZE);
    if *len > 0 {
        return;
    }
    if strcmp((*cp).charset, b"utf8\0".as_ptr() as *const i8) == 0 {
        *len = utf16s_to_utf8s(from, 3, UTF16_LITTLE_ENDIAN, target, 6);
        if *len > 0 {
            return;
        }
    }
    *target = b'?' as i8;
    *len = 1;
}

unsafe fn smb_utf16_bytes(from: *const le16, maxbytes: i32, codepage: *const nls_table) -> i32 {
    let maxwords = maxbytes / 2;
    let mut outlen = 0;
    let mut tmp = [0i8; NLS_MAX_CHARSET_SIZE as usize];
    let mut ftmp = [0u16; 3];
    for i in 0..maxwords {
        ftmp[0] = get_unaligned_le16(from.add(i as usize));
        if ftmp[0] == 0 { break; }
        for j in 1..=2 {
            ftmp[j as usize] = if i + j < maxwords { get_unaligned_le16(from.add((i + j) as usize)) } else { 0 };
        }
        let charlen = cifs_mapchar(tmp.as_mut_ptr(), ftmp.as_ptr(), codepage, false);
        outlen += if charlen > 0 { charlen } else { 1 };
    }
    outlen
}

unsafe fn smb_from_utf16(to: *mut i8, from: *const le16, tolen: i32, fromlen: i32, codepage: *const nls_table, mapchar: bool) -> i32 {
    let nullsize = nls_nullsize(codepage);
    let fromwords = fromlen / 2;
    let safelen = tolen - (NLS_MAX_CHARSET_SIZE + nullsize);
    let mut outlen = 0;
    let mut tmp = [0i8; NLS_MAX_CHARSET_SIZE as usize];
    let mut ftmp = [0u16; 3];
    let mut i = 0;
    while i < fromwords {
        ftmp[0] = get_unaligned_le16(from.add(i as usize));
        if ftmp[0] == 0 { break; }
        for j in 1..=2 { ftmp[j as usize] = if i + j < fromwords { get_unaligned_le16(from.add((i+j) as usize)) } else { 0 }; }
        if outlen >= safelen {
            let charlen = cifs_mapchar(tmp.as_mut_ptr(), ftmp.as_ptr(), codepage, mapchar);
            if outlen + charlen > tolen - nullsize { break; }
        }
        let charlen = cifs_mapchar(to.add(outlen as usize), ftmp.as_ptr(), codepage, mapchar);
        outlen += charlen;
        if charlen == 4 { i += 1; } else if charlen >= 5 { i += 2; }
        i += 1;
    }
    for _ in 0..nullsize { *to.add(outlen as usize) = 0; outlen += 1; }
    outlen
}

pub unsafe fn smb_strtoUTF16(to: *mut le16, mut from: *const i8, mut len: i32, codepage: *const nls_table) -> i32 {
    let mut i: i32;
    let mut wchar_to: wchar_t = 0;
    if strcmp((*codepage).charset, b"utf8\0".as_ptr() as *const i8) == 0 {
        i = utf8s_to_utf16s(from, len, UTF16_LITTLE_ENDIAN, to as *mut wchar_t, len);
        if i >= 0 { put_unaligned_le16(0, to.add(i as usize)); return i; }
    }
    i = 0;
    while len > 0 && *from != 0 {
        let charlen = ((*codepage).char2uni)(from, len, &mut wchar_to);
        let used = if charlen < 1 { wchar_to = 0x003f; 1 } else { charlen };
        put_unaligned_le16(wchar_to as u16, to.add(i as usize));
        i += 1; from = from.add(used as usize); len -= used;
    }
    put_unaligned_le16(0, to.add(i as usize));
    i
}

pub unsafe fn smb_strndup_from_utf16(src: *const i8, maxlen: i32, is_unicode: bool, codepage: *const nls_table) -> *mut i8 {
    let len = if is_unicode { smb_utf16_bytes(src as *const le16, maxlen, codepage) + nls_nullsize(codepage) } else { strnlen(src, maxlen) + 1 };
    let dst = kmalloc(len, KSMBD_DEFAULT_GFP);
    if dst.is_null() { return ERR_PTR(-ENOMEM); }
    if is_unicode { smb_from_utf16(dst, src as *const le16, len, maxlen, codepage, false); }
    else { strscpy(dst, src, len); }
    dst
}

pub unsafe fn smbConvertToUTF16(target: *mut le16, source: *const i8, srclen: i32, cp: *const nls_table, mapchars: i32) -> i32 {
    if mapchars == 0 { return smb_strtoUTF16(target, source, srclen, cp); }
    let mut i = 0; let mut j = 0;
    while i < srclen {
        let c = *source.add(i as usize); let mut charlen = 1; let mut dst: u16;
        match c as u8 {
            0 => { put_unaligned(0, target.add(j as usize)); return j; }
            b':' => dst = UNI_COLON, b'*' => dst = UNI_ASTERISK, b'?' => dst = UNI_QUESTION,
            b'<' => dst = UNI_LESSTHAN, b'>' => dst = UNI_GRTRTHAN, b'|' => dst = UNI_PIPE,
            _ => {
                let mut tmp: wchar_t = 0;
                charlen = ((*cp).char2uni)(source.add(i as usize), srclen-i, &mut tmp);
                if charlen > 0 {
                    dst = tmp as u16;
                } else {
                    if strcmp((*cp).charset, b"utf8\0".as_ptr() as *const i8) == 0 && (*source.add(i as usize) as u8) & 0x80 != 0 {
                        let mut u: unicode_t = 0;
                        charlen = utf8_to_utf32(source.add(i as usize), 6, &mut u);
                        if charlen >= 0 {
                            let mut wchar_to = [0 as wchar_t; 6];
                            let ret = utf8s_to_utf16s(source.add(i as usize), charlen, UTF16_LITTLE_ENDIAN, wchar_to.as_mut_ptr(), 6);
                            if ret >= 0 {
                                i += charlen;
                                dst = cpu_to_le16(wchar_to[0] as u16);
                                put_unaligned(dst, target.add(j as usize));
                                if charlen == 4 {
                                    j += 1;
                                    put_unaligned(cpu_to_le16(wchar_to[1] as u16), target.add(j as usize));
                                } else if charlen >= 5 {
                                    j += 1;
                                    put_unaligned(cpu_to_le16(wchar_to[1] as u16), target.add(j as usize));
                                    j += 1;
                                    put_unaligned(cpu_to_le16(wchar_to[2] as u16), target.add(j as usize));
                                }
                                continue;
                            }
                        }
                    }
                    dst = 0x003f;
                    charlen = 1;
                }
            }
        }
        i += charlen; put_unaligned(cpu_to_le16(dst), target.add(j as usize)); j += 1;
    }
    j
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
