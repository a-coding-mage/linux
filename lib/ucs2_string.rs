// SPDX-License-Identifier: GPL-2.0
// Translated from linux/ucs2_string.h and linux/module.h dependencies.

type Ucs2Char = u16;

const E2BIG: isize = 7;
const INT_MAX: usize = 0x7fff_ffff;

/* Return the number of unicode characters in data */
pub unsafe fn ucs2_strnlen(mut s: *const Ucs2Char, maxlength: usize) -> usize {
    let mut length: usize = 0;

    while length < maxlength && {
        let c = *s;
        s = s.add(1);
        c != 0
    } {
        length += 1;
    }
    length
}

pub unsafe fn ucs2_strlen(s: *const Ucs2Char) -> usize {
    ucs2_strnlen(s, usize::MAX)
}

/*
 * Return the number of bytes is the length of this string
 * Note: this is NOT the same as the number of unicode characters
 */
pub unsafe fn ucs2_strsize(data: *const Ucs2Char, maxlength: usize) -> usize {
    ucs2_strnlen(data, maxlength / core::mem::size_of::<Ucs2Char>())
        * core::mem::size_of::<Ucs2Char>()
}

/**
 * ucs2_strscpy() - Copy a UCS2 string into a sized buffer.
 *
 * Like strscpy(), only for UCS2 strings.
 */
pub unsafe fn ucs2_strscpy(
    dst: *mut Ucs2Char,
    src: *const Ucs2Char,
    count: usize,
) -> isize {
    let mut res: isize;

    /* Ensure that we have space for at least one NUL-character. */
    if count == 0 || count > INT_MAX / core::mem::size_of::<Ucs2Char>() {
        return -E2BIG;
    }

    /* Copy at most 'count' characters, returning on a NUL-terminator. */
    res = 0;
    while res < count as isize {
        let c = *src.add(res as usize);
        *dst.add(res as usize) = c;

        if c == 0 {
            return res;
        }
        res += 1;
    }

    /* Enforce proper NUL-termination and return error. */
    *dst.add(count - 1) = 0;
    -E2BIG
}

pub unsafe fn ucs2_strncmp(
    mut a: *const Ucs2Char,
    mut b: *const Ucs2Char,
    mut len: usize,
) -> i32 {
    loop {
        if len == 0 {
            return 0;
        }
        if *a < *b {
            return -1;
        }
        if *a > *b {
            return 1;
        }
        if *a == 0 {
            return 0;
        }
        a = a.add(1);
        b = b.add(1);
        len -= 1;
    }
}

pub unsafe fn ucs2_utf8size(src: *const Ucs2Char) -> usize {
    let mut i: usize = 0;
    let mut j: usize = 0;

    while *src.add(i) != 0 {
        let c = *src.add(i);
        if c >= 0x800 {
            j += 3;
        } else if c >= 0x80 {
            j += 2;
        } else {
            j += 1;
        }
        i += 1;
    }
    j
}

/*
 * copy at most maxlength bytes of whole utf8 characters to dest from the
 * ucs2 string src.
 *
 * The return value is the number of characters copied, not including the
 * final NUL character.
 */
pub unsafe fn ucs2_as_utf8(
    dest: *mut u8,
    src: *const Ucs2Char,
    mut maxlength: usize,
) -> usize {
    let mut i: usize = 0;
    let mut j: usize = 0;
    let limit = ucs2_strnlen(src, maxlength);

    while maxlength != 0 && i < limit {
        let c = *src.add(i);

        if c >= 0x800 {
            if maxlength < 3 {
                break;
            }
            maxlength -= 3;
            *dest.add(j) = (0xe0 | ((c & 0xf000) >> 12)) as u8;
            j += 1;
            *dest.add(j) = (0x80 | ((c & 0x0fc0) >> 6)) as u8;
            j += 1;
            *dest.add(j) = (0x80 | (c & 0x003f)) as u8;
            j += 1;
        } else if c >= 0x80 {
            if maxlength < 2 {
                break;
            }
            maxlength -= 2;
            *dest.add(j) = (0xc0 | ((c & 0x7c0) >> 6)) as u8;
            j += 1;
            *dest.add(j) = (0x80 | (c & 0x03f)) as u8;
            j += 1;
        } else {
            maxlength -= 1;
            *dest.add(j) = (c & 0x7f) as u8;
            j += 1;
        }
        i += 1;
    }
    if maxlength != 0 {
        *dest.add(j) = 0;
    }
    j
}

// MODULE_DESCRIPTION("UCS2 string handling");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
