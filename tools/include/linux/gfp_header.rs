/* SPDX-License-Identifier: GPL-2.0 */

// C header dependencies preserved for the translated repository:
// #include <linux/types.h>
// #include <linux/gfp_types.h>

/* Helper macro to avoid gfp flags if they are the default one */
#[macro_export]
macro_rules! __default_gfp {
    ($a:expr $(, $rest:expr)*) => {
        $a
    };
}

#[macro_export]
macro_rules! default_gfp {
    () => {
        GFP_KERNEL
    };
    ($a:expr $(, $rest:expr)*) => {
        __default_gfp!($a $(, $rest)*, GFP_KERNEL)
    };
}

#[inline]
pub unsafe fn gfpflags_allow_blocking(gfp_flags: gfp_t) -> bool {
    (gfp_flags & __GFP_DIRECT_RECLAIM) != 0
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
