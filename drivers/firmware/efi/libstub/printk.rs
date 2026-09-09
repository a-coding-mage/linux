// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the surrounding EFI stub and kernel translation.

pub static mut efi_loglevel: i32 = LOGLEVEL_NOTICE;

/// Write a UCS-2 encoded string to the console.
pub unsafe extern "C" fn efi_char16_puts(str_: *mut efi_char16_t) {
    efi_call_proto(
        efi_table_attr(efi_system_table, con_out),
        output_string,
        str_,
    );
}

unsafe fn utf8_to_utf32(s8: &mut *const u8) -> u32 {
    let c0: u8 = **s8;
    *s8 = s8.add(1);
    let mut cx = c0;
    let mut clen: usize = 0;

    /* The position of the most-significant 0 bit gives us the length of
     * a multi-octet encoding. */
    while (cx & 0x80) != 0 {
        clen += 1;
        cx = cx.wrapping_shl(1);
    }
    /* If the 0 bit is in position 8, this is a valid single-octet
     * encoding. If the 0 bit is in position 7 or positions 1-3, the
     * encoding is invalid. */
    if clen < 2 || clen > 4 {
        return c0 as u32;
    }

    let mut c32 = (cx >> clen) as u32;
    clen -= 1;
    for i in 0..clen {
        /* Trailing octets must have 10 in most significant bits. */
        cx = *(*s8).add(i) ^ 0x80;
        if (cx & 0xc0) != 0 {
            return c0 as u32;
        }
        c32 = (c32 << 6) | cx as u32;
    }
    /* Check for validity: Unicode range, no surrogate, correct length. */
    if c32 > 0x10ffff
        || (c32 & 0xf800) == 0xd800
        || clen != ((c32 >= 0x80) as usize
            + (c32 >= 0x800) as usize
            + (c32 >= 0x10000) as usize)
    {
        return c0 as u32;
    }
    *s8 = (*s8).add(clen);
    c32
}

/// Write a UTF-8 encoded string to the console.
pub unsafe extern "C" fn efi_puts(str_: *const core::ffi::c_char) {
    let mut buf: [efi_char16_t; 128] = [0; 128];
    let mut pos: usize = 0;
    let lim: usize = buf.len();
    let mut s8 = str_ as *const u8;

    while *s8 != 0 {
        if *s8 == b'\n' {
            buf[pos] = b'\r' as efi_char16_t;
            pos += 1;
        }
        let c32 = utf8_to_utf32(&mut s8);
        if c32 < 0x10000 {
            buf[pos] = c32 as efi_char16_t;
            pos += 1;
        } else {
            buf[pos] = ((0xd800 - (0x10000 >> 10)) + (c32 >> 10)) as efi_char16_t;
            pos += 1;
            buf[pos] = (0xdc00 + (c32 & 0x3ff)) as efi_char16_t;
            pos += 1;
        }
        if *s8 == 0 || pos >= lim - 2 {
            buf[pos] = 0;
            efi_char16_puts(buf.as_mut_ptr());
            pos = 0;
        }
    }
}

/// Print a kernel message.
pub unsafe extern "C" fn efi_printk(
    fmt: *const core::ffi::c_char,
    mut args: ...,
) -> i32 {
    let mut printf_buf = [0 as core::ffi::c_char; 256];
    let mut loglevel = printk_get_level(fmt);

    match loglevel {
        b'0'..=b'9' => loglevel -= b'0' as i32,
        _ => loglevel = -1,
    }

    if loglevel >= efi_loglevel {
        return 0;
    }
    if loglevel >= 0 {
        efi_puts(b"EFI stub: \0".as_ptr() as *const core::ffi::c_char);
    }

    fmt = printk_skip_level(fmt);
    va_start(&mut args, fmt);
    let printed = vsnprintf(printf_buf.as_mut_ptr(), printf_buf.len(), fmt, args);
    va_end(&mut args);

    efi_puts(printf_buf.as_ptr());
    if printed >= printf_buf.len() as i32 {
        efi_puts(b"[Message truncated]\n\0".as_ptr() as *const core::ffi::c_char);
        return -1;
    }
    printed
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
