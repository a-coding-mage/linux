/* SPDX-License-Identifier: GPL-2.0-only WITH Linux-syscall-note */
/*
 * Copyright (C) 2012 ARM Ltd.
 * Copyright (C) 2015 Regents of the University of California
 */

/* vDSO location */
pub const AT_SYSINFO_EHDR: i32 = 33;

/*
 * The set of entries below represent more extensive information
 * about the caches, in the form of two entry per cache type,
 * one entry containing the cache size in bytes, and the other
 * containing the cache line size in bytes in the bottom 16 bits
 * and the cache associativity in the next 16 bits.
 *
 * The associativity is such that if N is the 16-bit value, the
 * cache is N way set associative. A value if 0xffff means fully
 * associative, a value of 1 means directly mapped.
 *
 * For all these fields, a value of 0 means that the information
 * is not known.
 */
pub const AT_L1I_CACHESIZE: i32 = 40;
pub const AT_L1I_CACHEGEOMETRY: i32 = 41;
pub const AT_L1D_CACHESIZE: i32 = 42;
pub const AT_L1D_CACHEGEOMETRY: i32 = 43;
pub const AT_L2_CACHESIZE: i32 = 44;
pub const AT_L2_CACHEGEOMETRY: i32 = 45;
pub const AT_L3_CACHESIZE: i32 = 46;
pub const AT_L3_CACHEGEOMETRY: i32 = 47;

/* entries in ARCH_DLINFO */
pub const AT_VECTOR_SIZE_ARCH: i32 = 10;
pub const AT_MINSIGSTKSZ: i32 = 51;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
