/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/*
 * We need to put in some extra aux table entries to tell glibc what
 * the cache block size is, so it can use the dcbz instruction safely.
 */
pub const AT_DCACHEBSIZE: i32 = 19;
pub const AT_ICACHEBSIZE: i32 = 20;
pub const AT_UCACHEBSIZE: i32 = 21;
/* A special ignored type value for PPC, for glibc compatibility.  */
pub const AT_IGNOREPPC: i32 = 22;

/* The vDSO location. We have to use the same value as x86 for glibc's
 * sake :-)
 */
pub const AT_SYSINFO_EHDR: i32 = 33;

/*
 * AT_*CACHEBSIZE above represent the cache *block* size which is
 * the size that is affected by the cache management instructions.
 *
 * It doesn't nececssarily matches the cache *line* size which is
 * more of a performance tuning hint. Additionally the latter can
 * be different for the different cache levels.
 *
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

pub const AT_MINSIGSTKSZ: i32 = 51; /* stack needed for signal delivery */

pub const AT_VECTOR_SIZE_ARCH: i32 = 15; /* entries in ARCH_DLINFO */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
