// SPDX-License-Identifier: GPL-2.0
/*
 * Landlock tests - Common scope restriction
 *
 * Copyright © 2024 Tahera Fahimi <fahimitahera@gmail.com>
 */

// C dependencies:
// #define _GNU_SOURCE
// #include <errno.h>
// #include <linux/landlock.h>
// #include <sys/prctl.h>
// #include "common.h"

const ACCESS_LAST: __u64 = LANDLOCK_SCOPE_SIGNAL;

#[test]
fn ruleset_with_unknown_scope() {
    let mut scoped_mask: __u64;

    scoped_mask = 1u64 << 63;
    while scoped_mask != ACCESS_LAST {
        let ruleset_attr = landlock_ruleset_attr {
            scoped: scoped_mask,
        };

        unsafe {
            ASSERT_EQ(
                -1,
                landlock_create_ruleset(
                    &ruleset_attr,
                    core::mem::size_of_val(&ruleset_attr),
                    0,
                ),
            );
            ASSERT_EQ(EINVAL, errno);
        }

        scoped_mask >>= 1;
    }
}

TEST_HARNESS_MAIN!();

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
