// SPDX-License-Identifier: GPL-2.0
//
// The following identifiers are supplied by generated build-time dependencies
// and by boot.h in the surrounding translation unit.

pub static kernel_version: &[u8] = concat!(
    UTS_RELEASE,
    " (",
    LINUX_COMPILE_BY,
    "@",
    LINUX_COMPILE_HOST,
    ") ",
    UTS_VERSION,
)
.as_bytes();

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
