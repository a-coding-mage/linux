// SPDX-License-Identifier: GPL-2.0
/*
 *
 * Copyright (C) 2019-2021 Paragon Software GmbH, All rights reserved.
 *
 */

// Dependencies supplied by the surrounding NTFS implementation.

unsafe extern "C" {
    fn partial_name_hash(c: u32, hash: usize) -> usize;
}

#[inline]
unsafe fn upcase_unicode_char(upcase: *const u16, chr: u16) -> u16 {
    if chr < b'a' as u16 {
        return chr;
    }

    if chr <= b'z' as u16 {
        return chr - (b'a' as u16 - b'A' as u16);
    }

    *upcase.add(chr as usize)
}

/*
 * ntfs_cmp_names
 *
 * Thanks Kari Argillander <kari.argillander@gmail.com> for idea and implementation 'bothcase'
 *
 * Straight way to compare names:
 * - Case insensitive
 * - If name equals and 'bothcases' then
 * - Case sensitive
 * 'Straight way' code scans input names twice in worst case.
 * Optimized code scans input names only once.
 */
pub unsafe fn ntfs_cmp_names(
    mut s1: *const u16,
    l1: usize,
    mut s2: *const u16,
    l2: usize,
    upcase: *const u16,
    bothcase: bool,
) -> i32 {
    let mut diff1: i32 = 0;
    let mut diff2: i32;
    let mut len = core::cmp::min(l1, l2);

    if !bothcase && !upcase.is_null() {
        while len != 0 {
            let diff2 = upcase_unicode_char(upcase, (*s1).to_le()) as i32
                - upcase_unicode_char(upcase, (*s2).to_le()) as i32;
            if diff2 != 0 {
                return diff2;
            }
            s1 = s1.add(1);
            s2 = s2.add(1);
            len -= 1;
        }
        let diff2 = (l1 as i64 - l2 as i64) as i32;
        return if diff2 != 0 { diff2 } else { diff1 };
    }

    while len != 0 {
        diff1 = (*s1).to_le() as i32 - (*s2).to_le() as i32;
        if diff1 != 0 {
            if bothcase && !upcase.is_null() {
                break;
            }
            return diff1;
        }
        s1 = s1.add(1);
        s2 = s2.add(1);
        len -= 1;
    }
    if len == 0 {
        return (l1 as i64 - l2 as i64) as i32;
    }

    while len != 0 {
        diff2 = upcase_unicode_char(upcase, (*s1).to_le()) as i32
            - upcase_unicode_char(upcase, (*s2).to_le()) as i32;
        if diff2 != 0 {
            return diff2;
        }
        s1 = s1.add(1);
        s2 = s2.add(1);
        len -= 1;
    }

    diff2 = (l1 as i64 - l2 as i64) as i32;
    if diff2 != 0 { diff2 } else { diff1 }
}

pub unsafe fn ntfs_cmp_names_cpu(
    uni1: *const cpu_str,
    uni2: *const le_str,
    upcase: *const u16,
    bothcase: bool,
) -> i32 {
    ntfs_cmp_names((*uni1).name, (*uni1).len, (*uni2).name, (*uni2).len, upcase, bothcase)
}

/* Helper function for ntfs_d_hash. */
pub unsafe fn ntfs_names_hash(
    mut name: *const u16,
    mut len: usize,
    upcase: *const u16,
    mut hash: usize,
) -> usize {
    while len != 0 {
        let c = upcase_unicode_char(upcase, *name) as u32;
        hash = partial_name_hash(c, hash);
        name = name.add(1);
        len -= 1;
    }

    hash
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
