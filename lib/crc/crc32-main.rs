// SPDX-License-Identifier: GPL-2.0-only
/*
 * Aug 8, 2011 Bob Pearson with help from Joakim Tjernlund and George Spelvin
 * cleaned up code to current version of sparse and added the slicing-by-8
 * algorithm to the closely similar existing slicing-by-4 algorithm.
 *
 * Oct 15, 2000 Matt Domsch <Matt_Domsch@dell.com>
 * Nicer crc32 functions/docs submitted by linux@horizon.com.  Thanks!
 * Code was from the public domain, copyright abandoned.  Code was
 * subsequently included in the kernel, thus was re-licensed under the
 * GNU GPL v2.
 *
 * Oct 12, 2000 Matt Domsch <Matt_Domsch@dell.com>
 * Same crc32 function was used in 5 other places in the kernel.
 * I made one version, and deleted the others.
 * There are various incantations of crc32().  Some use a seed of 0 or ~0.
 * Some xor at the end with ~0.  The generic crc32() function takes
 * seed as an argument, and doesn't xor at the end.  Then individual
 * users can do whatever they need.
 *   drivers/net/smc9194.c uses seed ~0, doesn't xor with ~0.
 *   fs/jffs2 uses seed 0, doesn't xor with ~0.
 *   fs/partitions/efi.c uses seed ~0, xor's with ~0.
 */

/* see: Documentation/staging/crc32.rst for a description of algorithms */

// The Linux crc32 table declarations are supplied by the surrounding build.
extern "C" {
    static crc32table_le: [u32; 256];
    static crc32table_be: [u32; 256];
    static crc32ctable_le: [u32; 256];
}

unsafe fn crc32_le_base(mut crc: u32, mut p: *const u8, mut len: usize) -> u32 {
    while len != 0 {
        crc = (crc >> 8) ^ crc32table_le[((crc & 255) as usize) ^ (*p as usize)];
        p = p.add(1);
        len -= 1;
    }
    crc
}

unsafe fn crc32_be_base(mut crc: u32, mut p: *const u8, mut len: usize) -> u32 {
    while len != 0 {
        crc = (crc << 8) ^ crc32table_be[((crc >> 24) as usize) ^ (*p as usize)];
        p = p.add(1);
        len -= 1;
    }
    crc
}

unsafe fn crc32c_base(mut crc: u32, mut p: *const u8, mut len: usize) -> u32 {
    while len != 0 {
        crc = (crc >> 8) ^ crc32ctable_le[((crc & 255) as usize) ^ (*p as usize)];
        p = p.add(1);
        len -= 1;
    }
    crc
}

// CONFIG_CRC32_ARCH selects architecture-provided implementations.
#[cfg(feature = "CONFIG_CRC32_ARCH")]
extern "C" {
    fn crc32_optimizations_arch() -> u32;
    fn crc32_le_arch(crc: u32, p: *const u8, len: usize) -> u32;
    fn crc32_be_arch(crc: u32, p: *const u8, len: usize) -> u32;
    fn crc32c_arch(crc: u32, p: *const u8, len: usize) -> u32;
}

#[cfg(feature = "CONFIG_CRC32_ARCH")]
pub unsafe fn crc32_optimizations() -> u32 {
    crc32_optimizations_arch()
}

#[cfg(not(feature = "CONFIG_CRC32_ARCH"))]
unsafe fn crc32_le_arch(crc: u32, p: *const u8, len: usize) -> u32 {
    crc32_le_base(crc, p, len)
}

#[cfg(not(feature = "CONFIG_CRC32_ARCH"))]
unsafe fn crc32_be_arch(crc: u32, p: *const u8, len: usize) -> u32 {
    crc32_be_base(crc, p, len)
}

#[cfg(not(feature = "CONFIG_CRC32_ARCH"))]
unsafe fn crc32c_arch(crc: u32, p: *const u8, len: usize) -> u32 {
    crc32c_base(crc, p, len)
}

pub unsafe fn crc32_le(crc: u32, p: *const core::ffi::c_void, len: usize) -> u32 {
    crc32_le_arch(crc, p as *const u8, len)
}

pub unsafe fn crc32_be(crc: u32, p: *const core::ffi::c_void, len: usize) -> u32 {
    crc32_be_arch(crc, p as *const u8, len)
}

pub unsafe fn crc32c(crc: u32, p: *const core::ffi::c_void, len: usize) -> u32 {
    crc32c_arch(crc, p as *const u8, len)
}

// EXPORT_SYMBOL(crc32_optimizations), EXPORT_SYMBOL(crc32_le),
// EXPORT_SYMBOL(crc32_be), EXPORT_SYMBOL(crc32c)
// MODULE_DESCRIPTION("CRC32 library functions");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
