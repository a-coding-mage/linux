// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copyright (C) 2023 Oracle.  All Rights Reserved.
 * Author: Darrick J. Wong <djwong@kernel.org>
 */

// The C translation unit includes the XFS platform, filesystem, format,
// directory/attribute, mount, transaction, and test interfaces.

/* 4096 random bytes */
#[repr(align(8))]
static mut TEST_BUF: [u8; 4096] = [0; 4096];

#[repr(C)]
#[derive(Copy, Clone)]
struct DahashTest {
    start: u16,
    length: u16,
    dahash: u32,
    ascii_ci_dahash: u32,
}

// The generated test vector contains the source file's complete table of
// (start, length, dahash, ascii_ci_dahash) values.
extern "C" {
    static test: [DahashTest; 0];
    fn xfs_da_hashname(name: *const u8, length: u16) -> u32;
    fn xfs_ascii_ci_hashname(name: *const XfsName) -> u32;
    fn printk(format: *const u8, ...) -> i32;
}

#[repr(C)]
struct XfsName {
    name: *const u8,
    len: u16,
}

#[no_mangle]
pub unsafe extern "C" fn xfs_dahash_test() -> i32 {
    let mut i: usize = 0;
    let mut errors: u32 = 0;

    while i < test.len() {
        let mut xname = XfsName {
            name: core::ptr::null(),
            len: 0,
        };
        let hash: u32;

        hash = xfs_da_hashname(
            TEST_BUF.as_ptr().add(test[i].start as usize),
            test[i].length,
        );
        if hash != test[i].dahash {
            errors += 1;
        }

        xname.name = TEST_BUF.as_ptr().add(test[i].start as usize);
        xname.len = test[i].length;
        hash = xfs_ascii_ci_hashname(&xname);
        if hash != test[i].ascii_ci_dahash {
            errors += 1;
        }
        i += 1;
    }

    if errors != 0 {
        printk(b"xfs dir/attr hash test failed %u times!\0".as_ptr(), errors);
        return -34; // -ERANGE
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
