// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright (C) 2011 IBM Corporation
 *
 * Author:
 * Mimi Zohar <zohar@us.ibm.com>
 */

extern "C" {
    pub static XATTR_NAME_POSIX_ACL_ACCESS: *const u8;
    pub static XATTR_NAME_POSIX_ACL_DEFAULT: *const u8;

    fn strlen(s: *const u8) -> usize;
    fn strncmp(s1: *const i8, s2: *const i8, n: usize) -> i32;
}

pub fn posix_xattr_acl(xattr: *const u8) -> i32 {
    unsafe {
        let xattr_len = strlen(xattr);

        if (strlen(XATTR_NAME_POSIX_ACL_ACCESS) == xattr_len)
            && (strncmp(
                XATTR_NAME_POSIX_ACL_ACCESS as *const i8,
                xattr as *const i8,
                xattr_len,
            ) == 0)
        {
            return 1;
        }
        if (strlen(XATTR_NAME_POSIX_ACL_DEFAULT) == xattr_len)
            && (strncmp(
                XATTR_NAME_POSIX_ACL_DEFAULT as *const i8,
                xattr as *const i8,
                xattr_len,
            ) == 0)
        {
            return 1;
        }
        0
    }
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
