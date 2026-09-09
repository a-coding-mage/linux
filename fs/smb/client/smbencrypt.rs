// SPDX-License-Identifier: GPL-2.0-or-later
/*
   Unix SMB/Netbios implementation.
   Version 1.9.
   SMB parameters and setup
   Copyright (C) Andrew Tridgell 1992-2000
   Copyright (C) Luke Kenneth Casson Leighton 1996-2000
   Modified by Jeremy Allison 1995.
   Copyright (C) Andrew Bartlett <abartlet@samba.org> 2002-2003
   Modified by Steve French (sfrench@us.ibm.com) 2002-2003

*/

// Linux and CIFS dependencies supplied by the surrounding translation unit.

#[repr(C)]
pub struct md4_ctx {
    _opaque: [u8; 0],
}

#[repr(C)]
pub struct nls_table {
    _opaque: [u8; 0],
}

type __le16 = u16;

extern "C" {
    fn cifs_md4_init(mctx: *mut md4_ctx) -> i32;
    fn cifs_md4_update(mctx: *mut md4_ctx, data: *const u8, len: i32) -> i32;
    fn cifs_md4_final(mctx: *mut md4_ctx, out: *mut u8) -> i32;
    fn cifs_dbg(level: i32, fmt: *const u8, ...);
    fn cifs_strtoUTF16(
        dst: *mut __le16,
        src: *const u8,
        maxlen: i32,
        codepage: *const nls_table,
    ) -> i32;
    fn memzero_explicit(s: *mut core::ffi::c_void, count: usize);
}

// The following came from the other byteorder.h to avoid include conflicts.
#[inline]
unsafe fn cval(buf: *const u8, pos: usize) -> u8 {
    *buf.add(pos)
}

#[inline]
unsafe fn ssvalx(buf: *mut u8, pos: usize, val: u16) {
    *buf.add(pos) = (val & 0xff) as u8;
    *buf.add(pos + 1) = (val >> 8) as u8;
}

#[inline]
unsafe fn ssval(buf: *mut u8, pos: usize, val: u16) {
    ssvalx(buf, pos, val);
}

/* produce a md4 message digest from data of length n bytes */
unsafe fn mdfour(md4_hash: *mut u8, link_str: *mut u8, link_len: i32) -> i32 {
    let mut rc: i32;
    let mut mctx = md4_ctx { _opaque: [] };

    rc = cifs_md4_init(&mut mctx);
    if rc != 0 {
        cifs_dbg(0, b"%s: Could not init MD4\0".as_ptr(), b"mdfour\0".as_ptr());
        return rc;
    }
    rc = cifs_md4_update(&mut mctx, link_str, link_len);
    if rc != 0 {
        cifs_dbg(0, b"%s: Could not update MD4\0".as_ptr(), b"mdfour\0".as_ptr());
        return rc;
    }
    rc = cifs_md4_final(&mut mctx, md4_hash);
    if rc != 0 {
        cifs_dbg(0, b"%s: Could not finalize MD4\0".as_ptr(), b"mdfour\0".as_ptr());
    }

    rc
}

/*
 * Creates the MD4 Hash of the users password in NT UNICODE.
 */
pub unsafe fn E_md4hash(
    passwd: *const u8,
    p16: *mut u8,
    codepage: *const nls_table,
) -> i32 {
    let len: i32;
    let mut wpwd: [__le16; 129] = [0; 129];

    /* Password cannot be longer than 128 characters */
    if !passwd.is_null() {
        /* Password must be converted to NT unicode */
        len = cifs_strtoUTF16(wpwd.as_mut_ptr(), passwd, 128, codepage);
    } else {
        len = 0;
        wpwd[0] = 0; /* Ensure string is null terminated */
    }

    let rc = mdfour(
        p16,
        wpwd.as_mut_ptr() as *mut u8,
        len.wrapping_mul(core::mem::size_of::<__le16>() as i32),
    );
    memzero_explicit(wpwd.as_mut_ptr() as *mut core::ffi::c_void, core::mem::size_of_val(&wpwd));

    rc
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
