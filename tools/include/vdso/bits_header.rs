/* SPDX-License-Identifier: GPL-2.0 */

/* Original C header included <vdso/const.h> for UL() and ULL() constant macros. */

#[macro_export]
macro_rules! BIT {
    ($nr:expr) => {
        (1usize << ($nr))
    };
}

#[macro_export]
macro_rules! BIT_ULL {
    ($nr:expr) => {
        (1u64 << ($nr))
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
