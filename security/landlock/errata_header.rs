/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Landlock - Errata information
 *
 * Copyright © 2025 Microsoft Corporation
 */

// C dependency: #include <linux/init.h>

#[repr(C)]
pub struct landlock_erratum {
    pub abi: ::core::ffi::c_int,
    pub number: u8,
}

/* clang-format off */
macro_rules! LANDLOCK_ERRATUM {
    ($number:expr) => {
        landlock_erratum {
            abi: LANDLOCK_ERRATA_ABI,
            number: $number,
        }
    };
}
/* clang-format on */

/*
 * Some fixes may require user space to check if they are applied on the running
 * kernel before using a specific feature.  For instance, this applies when a
 * restriction was previously too restrictive and is now getting relaxed (for
 * compatibility or semantic reasons).  However, non-visible changes for
 * legitimate use (e.g. security fixes) do not require an erratum.
 */
#[used]
pub static landlock_errata_init: [landlock_erratum; 1] = [
    /*
     * Only Sparse may not implement __has_include.  If a compiler does not
     * implement __has_include, a warning will be printed at boot time (see
     * setup.c).
     *
     * The C source conditionally includes errata/abi-1.h through
     * errata/abi-6.h when __has_include is available.  Rust has no direct
     * source-local equivalent for conditionally including those future
     * dependency fragments; their entries belong here when supplied.
     *
     * For each new erratum, we need to include all the ABI files up to the impacted
     * ABI to make all potential future intermediate errata easy to backport.
     *
     * If such change involves more than one ABI addition, then it must be in a
     * dedicated commit with the same Fixes tag as used for the actual fix.
     *
     * Each commit creating a new security/landlock/errata/abi-*.h file must have a
     * Depends-on tag to reference the commit that previously added the line to
     * include this new file, except if the original Fixes tag is enough.
     *
     * Each erratum must be documented in its related ABI file, and a dedicated
     * commit must update Documentation/userspace-api/landlock.rst to include this
     * erratum.  This commit will not be backported.
     */
    landlock_erratum { abi: 0, number: 0 },
];

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
