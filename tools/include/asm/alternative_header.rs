/* SPDX-License-Identifier: GPL-2.0 */

/*
 * C header guard removed in Rust translation:
 * _TOOLS_ASM_ALTERNATIVE_ASM_H
 */

/*
 * Original conditional intent:
 *
 * #if defined(__s390x__)
 * #ifdef __ASSEMBLY__
 * .macro ALTERNATIVE oldinstr, newinstr, feature
 *      \oldinstr
 * .endm
 * #endif
 * #else
 *
 * Just disable it so we can build arch/x86/lib/memcpy_64.S for perf bench:
 *
 * #define ALTERNATIVE #
 *
 * #endif
 */

#[cfg(all(target_arch = "s390x", any()))]
macro_rules! ALTERNATIVE {
    ($oldinstr:tt, $newinstr:tt, $feature:tt) => {
        $oldinstr
    };
}

#[cfg(not(target_arch = "s390x"))]
macro_rules! ALTERNATIVE {
    ($($tokens:tt)*) => {};
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
