/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * cifs_unicode:  Unicode kernel case support
 *
 * Function:
 *     Convert a unicode character to upper or lower case using
 *     compressed tables.
 *
 *   Copyright (c) International Business Machines  Corp., 2000,2009
 *
 * Notes:
 *     These APIs are based on the C library functions.  The semantics
 *     should match the C functions but with expanded size operands.
 *
 *     The upper/lower functions are based on a table created by mkupr.
 *     This is a compressed table of upper and lower case conversion.
 */

// Dependencies supplied by the surrounding translation unit:
// asm/byteorder.h, linux/types.h, linux/nls.h, nls_ucs2_utils.h, and cifsglob.h.

/*
 * Macs use an older "SFM" mapping of the symbols above. Fortunately it does
 * not conflict (although almost does) with the mapping above.
 */

pub const SFM_DOUBLEQUOTE: u16 = 0xF020;
pub const SFM_ASTERISK: u16 = 0xF021;
pub const SFM_QUESTION: u16 = 0xF025;
pub const SFM_COLON: u16 = 0xF022;
pub const SFM_GRTRTHAN: u16 = 0xF024;
pub const SFM_LESSTHAN: u16 = 0xF023;
pub const SFM_PIPE: u16 = 0xF027;
pub const SFM_SLASH: u16 = 0xF026;
pub const SFM_SPACE: u16 = 0xF028;
pub const SFM_PERIOD: u16 = 0xF029;

/*
 * Mapping mechanism to use when one of the seven reserved characters is
 * encountered.  We can only map using one of the mechanisms at a time
 * since otherwise readdir could return directory entries which we would
 * not be able to open
 *
 * NO_MAP_UNI_RSVD  = do not perform any remapping of the character
 * SFM_MAP_UNI_RSVD = map reserved characters using SFM scheme (MAC compatible)
 * SFU_MAP_UNI_RSVD = map reserved characters ala SFU ("mapchars" option)
 */

pub const NO_MAP_UNI_RSVD: i32 = 0;
pub const SFM_MAP_UNI_RSVD: i32 = 1;
pub const SFU_MAP_UNI_RSVD: i32 = 2;

extern "C" {
    pub fn cifs_from_utf16(
        to: *mut core::ffi::c_char,
        from: *const u16,
        tolen: i32,
        fromlen: i32,
        codepage: *const nls_table,
        map_type: i32,
    ) -> i32;
    pub fn cifs_utf16_bytes(from: *const u16, maxbytes: i32, codepage: *const nls_table) -> i32;
    pub fn cifs_strtoUTF16(
        to: *mut u16,
        from: *const core::ffi::c_char,
        len: i32,
        codepage: *const nls_table,
    ) -> i32;
    pub fn cifs_strndup_from_utf16(
        src: *const core::ffi::c_char,
        maxlen: i32,
        is_unicode: bool,
        codepage: *const nls_table,
    ) -> *mut core::ffi::c_char;
    pub fn cifsConvertToUTF16(
        target: *mut u16,
        source: *const core::ffi::c_char,
        srclen: i32,
        cp: *const nls_table,
        map_chars: i32,
    ) -> i32;
    pub fn cifs_strndup_to_utf16(
        src: *const core::ffi::c_char,
        maxlen: i32,
        utf16_len: *mut i32,
        cp: *const nls_table,
        remap: i32,
    ) -> *mut u16;
    pub fn cifs_toupper(input: i32) -> i32;
    pub fn cifs_sb_flags(cifs_sb: *const cifs_sb_info) -> u32;
}

#[repr(C)]
pub struct nls_table {
    _private: [u8; 0],
}

#[repr(C)]
pub struct cifs_sb_info {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn cifs_remap(cifs_sb: *const cifs_sb_info) -> i32 {
    let sbflags: u32 = cifs_sb_flags(cifs_sb);

    if sbflags & CIFS_MOUNT_MAP_SFM_CHR != 0 {
        return SFM_MAP_UNI_RSVD;
    }
    if sbflags & CIFS_MOUNT_MAP_SPECIAL_CHR != 0 {
        return SFU_MAP_UNI_RSVD;
    }

    NO_MAP_UNI_RSVD
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
