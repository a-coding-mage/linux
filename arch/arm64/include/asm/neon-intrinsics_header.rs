/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2018 Linaro, Ltd. <ard.biesheuvel@linaro.org>
 */

// Translated from the C header <asm/neon-intrinsics.h>.

// Dependency supplied by the surrounding kernel translation:
// #include <asm-generic/int-ll64.h>

/*
 * In the kernel, u64/s64 are [un]signed long long, not [un]signed long.
 * The original header redefines __INT64_TYPE__ and __UINT64_TYPE__ to force
 * gcc-stdint.h to define uint64_t / int64_t compatibly.
 */

// C preprocessor configuration preserved from the source:
// #ifdef __INT64_TYPE__
// #undef __INT64_TYPE__
// #define __INT64_TYPE__ long long
// #endif
// #ifdef __UINT64_TYPE__
// #undef __UINT64_TYPE__
// #define __UINT64_TYPE__ unsigned long long
// #endif

/*
 * genksyms chokes on the ARM NEON intrinsics system header, but nothing it
 * defines is exported, so the original header disregards it during genksyms.
 */

// Dependency supplied by the surrounding kernel translation when not running
// genksyms:
// #ifndef __GENKSYMS__
// #include <arm_neon.h>
// #endif

// When compiling with Clang, the original header ignores:
// #pragma clang diagnostic ignored "-Wincompatible-pointer-types"


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
