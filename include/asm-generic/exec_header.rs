/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Generic process execution definitions.
 *
 * It should be possible to use these on really simple architectures,
 * but it serves more as a starting point for new ports.
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

macro_rules! arch_align_stack {
    ($x:expr) => {
        $x
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
