/* SPDX-License-Identifier: LGPL-2.1 OR MIT */
/*
 * ctype function definitions for NOLIBC
 * Copyright (C) 2017-2021 Willy Tarreau <w@1wt.eu>
 */

/* make sure to include all global symbols */
/* C dependency: "nolibc.h" */

/* C dependency: "std.h" */

/*
 * As much as possible, please keep functions alphabetically sorted.
 */

#[allow(dead_code)]
pub fn isascii(c: i32) -> i32 {
    /* 0x00..0x7f */
    ((c as u32) <= 0x7f) as i32
}

#[allow(dead_code)]
pub fn isblank(c: i32) -> i32 {
    (c == b'\t' as i32 || c == b' ' as i32) as i32
}

#[allow(dead_code)]
pub fn iscntrl(c: i32) -> i32 {
    /* 0x00..0x1f, 0x7f */
    ((c as u32) < 0x20 || c == 0x7f) as i32
}

#[allow(dead_code)]
pub fn isdigit(c: i32) -> i32 {
    ((c.wrapping_sub(b'0' as i32) as u32) < 10) as i32
}

#[allow(dead_code)]
pub fn isgraph(c: i32) -> i32 {
    /* 0x21..0x7e */
    ((c.wrapping_sub(0x21) as u32) < 0x5e) as i32
}

#[allow(dead_code)]
pub fn islower(c: i32) -> i32 {
    ((c.wrapping_sub(b'a' as i32) as u32) < 26) as i32
}

#[allow(dead_code)]
pub fn isprint(c: i32) -> i32 {
    /* 0x20..0x7e */
    ((c.wrapping_sub(0x20) as u32) < 0x5f) as i32
}

#[allow(dead_code)]
pub fn isspace(c: i32) -> i32 {
    /* \t is 0x9, \n is 0xA, \v is 0xB, \f is 0xC, \r is 0xD */
    ((c as u32) == b' ' as u32 || (c.wrapping_sub(0x09) as u32) < 5) as i32
}

#[allow(dead_code)]
pub fn isupper(c: i32) -> i32 {
    ((c.wrapping_sub(b'A' as i32) as u32) < 26) as i32
}

#[allow(dead_code)]
pub fn isxdigit(c: i32) -> i32 {
    (isdigit(c) != 0
        || (c.wrapping_sub(b'A' as i32) as u32) < 6
        || (c.wrapping_sub(b'a' as i32) as u32) < 6) as i32
}

#[allow(dead_code)]
pub fn isalpha(c: i32) -> i32 {
    (islower(c) != 0 || isupper(c) != 0) as i32
}

#[allow(dead_code)]
pub fn isalnum(c: i32) -> i32 {
    (isalpha(c) != 0 || isdigit(c) != 0) as i32
}

#[allow(dead_code)]
pub fn ispunct(c: i32) -> i32 {
    (isgraph(c) != 0 && isalnum(c) == 0) as i32
}
