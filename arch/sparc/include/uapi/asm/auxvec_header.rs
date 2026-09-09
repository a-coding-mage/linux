// Translated from sparc/include/uapi/asm/auxvec.h.

pub const AT_SYSINFO_EHDR: i32 = 33;

/* Avoid overlap with other AT_* values since they are consolidated in
 * glibc and any overlaps can cause problems
 */
pub const AT_ADI_BLKSZ: i32 = 48;
pub const AT_ADI_NBITS: i32 = 49;
pub const AT_ADI_UEONADI: i32 = 50;

pub const AT_VECTOR_SIZE_ARCH: i32 = 4;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
