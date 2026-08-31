// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Copied from the kernel sources to tools/perf/:
 *
 * Generic barrier definitions.
 *
 * It should be possible to use these on really simple architectures,
 * but it serves more as a starting point for new ports.
 *
 * Copyright (C) 2007 Red Hat, Inc. All Rights Reserved.
 * Written by David Howells (dhowells@redhat.com)
 */

// C dependency: #include <linux/compiler.h>
// The fallback `mb()` definition depends on the compiler barrier supplied there.

/*
 * Force strict CPU ordering. And yes, this is required on UP too when we're
 * talking to devices.
 *
 * Fall back to compiler barriers if nothing better is provided.
 */

// C conditional intent:
// #ifndef mb
// #define mb() barrier()
// #endif
#[macro_export]
macro_rules! mb {
    () => {
        barrier!()
    };
}

// C conditional intent:
// #ifndef rmb
// #define rmb() mb()
// #endif
#[macro_export]
macro_rules! rmb {
    () => {
        mb!()
    };
}

// C conditional intent:
// #ifndef wmb
// #define wmb() mb()
// #endif
#[macro_export]
macro_rules! wmb {
    () => {
        mb!()
    };
}
