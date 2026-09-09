// SPDX-License-Identifier: GPL-2.0
/*
 *  linux/fs/hpfs/name.c
 *
 *  Mikulas Patocka (mikulas@artax.karlin.mff.cuni.cz), 1998-1999
 *
 *  operations with filenames
 */

#[inline]
unsafe fn not_allowed_char(c: u8) -> i32 {
    (c < b' ' || c == b'"' || c == b'*' || c == b'/' || c == b':' || c == b'<' ||
        c == b'>' || c == b'?' || c == b'\\' || c == b'|') as i32
}

#[inline]
unsafe fn no_dos_char(c: u8) -> i32 {
    /* Characters that are allowed in HPFS but not in DOS */
    (c == b'+' || c == b',' || c == b';' || c == b'=' || c == b'[' || c == b']') as i32
}

#[inline]
unsafe fn upcase(dir: *mut u8, a: u8) -> u8 {
    if a < 128 || a == 255 {
        return if a >= b'a' && a <= b'z' { a.wrapping_sub(0x20) } else { a };
    }
    if dir.is_null() { return a; }
    *dir.add((a - 128) as usize)
}

pub unsafe fn hpfs_upcase(dir: *mut u8, a: u8) -> u8 {
    upcase(dir, a)
}

#[inline]
unsafe fn locase(dir: *mut u8, a: u8) -> u8 {
    if a < 128 || a == 255 {
        return if a >= b'A' && a <= b'Z' { a.wrapping_add(0x20) } else { a };
    }
    if dir.is_null() { return a; }
    *dir.add(a as usize)
}

pub unsafe fn hpfs_chk_name(name: *const u8, len: *mut u32) -> i32 {
    let mut i: i32;
    if *len > 254 { return -ENAMETOOLONG; }
    hpfs_adjust_length(name, len);
    if *len == 0 { return -EINVAL; }
    i = 0;
    while i < *len as i32 {
        if not_allowed_char(*name.add(i as usize)) != 0 { return -EINVAL; }
        i += 1;
    }
    if *len == 1 && *name == b'.' { return -EINVAL; }
    if *len == 2 && *name == b'.' && *name.add(1) == b'.' { return -EINVAL; }
    0
}

pub unsafe fn hpfs_translate_name(
    s: *mut super_block, from: *mut u8, len: u32, lc: i32, lng: i32,
) -> *mut u8 {
    let mut to: *mut u8;
    let mut i: i32;
    if (*hpfs_sb(s)).sb_chk >= 2 && (hpfs_is_name_long(from, len) != lng) {
        pr_err("Long name flag mismatch - name ");
        i = 0;
        while i < len as i32 { pr_cont("%c", *from.add(i as usize)); i += 1; }
        pr_cont(" misidentified as {}.\n", if lng != 0 { "short" } else { "long" });
        pr_err("It's nothing serious. It could happen because of bug in OS/2.\nSet checks=normal to disable this message.\n");
    }
    if lc == 0 { return from; }
    to = kmalloc(len, GFP_KERNEL);
    if to.is_null() {
        pr_err("can't allocate memory for name conversion buffer\n");
        return from;
    }
    i = 0;
    while i < len as i32 {
        *to.add(i as usize) = locase((*hpfs_sb(s)).sb_cp_table, *from.add(i as usize));
        i += 1;
    }
    to
}

pub unsafe fn hpfs_compare_names(
    s: *mut super_block, n1: *const u8, l1: u32, n2: *const u8, l2: u32, last: i32,
) -> i32 {
    let l = if l1 < l2 { l1 } else { l2 };
    if last != 0 { return -1; }
    let mut i = 0;
    while i < l {
        let c1 = upcase((*hpfs_sb(s)).sb_cp_table, *n1.add(i as usize));
        let c2 = upcase((*hpfs_sb(s)).sb_cp_table, *n2.add(i as usize));
        if c1 < c2 { return -1; }
        if c1 > c2 { return 1; }
        i += 1;
    }
    if l1 < l2 { return -1; }
    if l1 > l2 { return 1; }
    0
}

pub unsafe fn hpfs_is_name_long(name: *const u8, len: u32) -> i32 {
    let mut i = 0;
    while i < len && *name.add(i as usize) != b'.' {
        if no_dos_char(*name.add(i as usize)) != 0 { return 1; }
        i += 1;
    }
    if i == 0 || i > 8 { return 1; }
    if i == len { return 0; }
    let mut j = i + 1;
    while j < len {
        if *name.add(j as usize) == b'.' || no_dos_char(*name.add(i as usize)) != 0 { return 1; }
        j += 1;
    }
    (j - i > 4) as i32
}

/* OS/2 clears dots and spaces at the end of file name, so we have to */

pub unsafe fn hpfs_adjust_length(name: *const u8, len: *mut u32) {
    if *len == 0 { return; }
    if *len == 1 && *name == b'.' { return; }
    if *len == 2 && *name == b'.' && *name.add(1) == b'.' { return; }
    while *len != 0 && (*name.add((*len - 1) as usize) == b'.' || *name.add((*len - 1) as usize) == b' ') {
        *len -= 1;
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
