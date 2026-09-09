/*
 * Copyright (c) 2017 Oracle and/or its affiliates. All rights reserved.
 */

// #define BUILD_VDSO32
pub const BUILD_VDSO32: bool = true;

// The C source conditionally reconfigures the included implementation when
// CONFIG_SPARC64 is enabled. These build-time conditions are preserved here
// as configuration intent for the dependent translation.
//
// #ifdef CONFIG_SPARC64
// #undef CONFIG_64BIT
// #undef CONFIG_SPARC64
// #define BUILD_VDSO32_64
// #define CONFIG_32BIT
// #undef CONFIG_QUEUED_RWLOCKS
// #undef CONFIG_QUEUED_SPINLOCKS
// #endif

// The original file includes ../vclock_gettime.c. Its declarations and
// definitions are supplied by that external translation unit.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
