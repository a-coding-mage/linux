// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024 Christian Brauner <brauner@kernel.org>

// C dependencies: <fcntl.h>, <sched.h>, <stdio.h>, <string.h>,
// <sys/stat.h>, <sys/mount.h>, <unistd.h>, "statmount.h",
// "kselftest_harness.h".

// From statmount.h.
const LSMT_ROOT: u64 = !0u64;

// From listmount_test.c fallback when LISTMOUNT_REVERSE is not already defined.
const LISTMOUNT_REVERSE: u32 = 1 << 0; /* List later mounts first */

const LISTMNT_BUFFER: usize = 10;

unsafe extern "C" {
    fn listmount(
        mnt_id: u64,
        request_mask: u64,
        last_mnt_id: u64,
        list: *mut u64,
        num: usize,
        flags: u32,
    ) -> isize;
}

/* Check that all mount ids are in increasing order. */
fn listmount_forward() {
    let mut list: [u64; LISTMNT_BUFFER] = [0; LISTMNT_BUFFER];
    let mut last_mnt_id: u64 = 0;

    loop {
        let nr_mounts: isize;

        nr_mounts = unsafe {
            listmount(
                LSMT_ROOT,
                0,
                last_mnt_id,
                list.as_mut_ptr(),
                LISTMNT_BUFFER,
                0,
            )
        };
        assert!(nr_mounts >= 0);
        if nr_mounts == 0 {
            break;
        }

        for cur in 0..(nr_mounts as usize) {
            if cur < (nr_mounts as usize) - 1 {
                assert!(list[cur] < list[cur + 1]);
            }
            last_mnt_id = list[cur];
        }
    }
}

/* Check that all mount ids are in decreasing order. */
fn listmount_backward() {
    let mut list: [u64; LISTMNT_BUFFER] = [0; LISTMNT_BUFFER];
    let mut last_mnt_id: u64 = 0;

    loop {
        let nr_mounts: isize;

        nr_mounts = unsafe {
            listmount(
                LSMT_ROOT,
                0,
                last_mnt_id,
                list.as_mut_ptr(),
                LISTMNT_BUFFER,
                LISTMOUNT_REVERSE,
            )
        };
        assert!(nr_mounts >= 0);
        if nr_mounts == 0 {
            break;
        }

        for cur in 0..(nr_mounts as usize) {
            if cur < (nr_mounts as usize) - 1 {
                assert!(list[cur] > list[cur + 1]);
            }
            last_mnt_id = list[cur];
        }
    }
}

// TEST_HARNESS_MAIN
fn main() {
    listmount_forward();
    listmount_backward();
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
