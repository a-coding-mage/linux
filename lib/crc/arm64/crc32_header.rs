// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel translation unit:
// asm/alternative.h, asm/cpufeature.h, and asm/simd.h.

// The minimum input length to consider the 4-way interleaved code path
pub const min_len: usize = 1024;

unsafe extern "C" {
    pub fn crc32_le_arm64(crc: u32, p: *const u8, len: usize) -> u32;
    pub fn crc32c_le_arm64(crc: u32, p: *const u8, len: usize) -> u32;
    pub fn crc32_be_arm64(crc: u32, p: *const u8, len: usize) -> u32;

    pub fn crc32_le_arm64_4way(crc: u32, p: *const u8, len: usize) -> u32;
    pub fn crc32c_le_arm64_4way(crc: u32, p: *const u8, len: usize) -> u32;
    pub fn crc32_be_arm64_4way(crc: u32, p: *const u8, len: usize) -> u32;

    fn crc32_le_base(crc: u32, p: *const u8, len: usize) -> u32;
    fn crc32c_base(crc: u32, p: *const u8, len: usize) -> u32;
    fn crc32_be_base(crc: u32, p: *const u8, len: usize) -> u32;

    fn alternative_has_cap_likely(cap: u32) -> bool;
    fn cpu_have_named_feature(feature: u32) -> bool;
    fn may_use_simd() -> bool;
}

pub unsafe fn crc32_le_arch(mut crc: u32, mut p: *const u8, mut len: usize) -> u32 {
    if !alternative_has_cap_likely(ARM64_HAS_CRC32) {
        return crc32_le_base(crc, p, len);
    }

    if len >= min_len && cpu_have_named_feature(PMULL) && may_use_simd() {
        // scoped_ksimd() supplies the surrounding SIMD critical section.
        crc = crc32_le_arm64_4way(crc, p, len);

        p = p.add(len - (len % 64));
        len %= 64;

        if len == 0 {
            return crc;
        }
    }

    crc32_le_arm64(crc, p, len)
}

pub unsafe fn crc32c_arch(mut crc: u32, mut p: *const u8, mut len: usize) -> u32 {
    if !alternative_has_cap_likely(ARM64_HAS_CRC32) {
        return crc32c_base(crc, p, len);
    }

    if len >= min_len && cpu_have_named_feature(PMULL) && may_use_simd() {
        // scoped_ksimd() supplies the surrounding SIMD critical section.
        crc = crc32c_le_arm64_4way(crc, p, len);

        p = p.add(len - (len % 64));
        len %= 64;

        if len == 0 {
            return crc;
        }
    }

    crc32c_le_arm64(crc, p, len)
}

pub unsafe fn crc32_be_arch(mut crc: u32, mut p: *const u8, mut len: usize) -> u32 {
    if !alternative_has_cap_likely(ARM64_HAS_CRC32) {
        return crc32_be_base(crc, p, len);
    }

    if len >= min_len && cpu_have_named_feature(PMULL) && may_use_simd() {
        // scoped_ksimd() supplies the surrounding SIMD critical section.
        crc = crc32_be_arm64_4way(crc, p, len);

        p = p.add(len - (len % 64));
        len %= 64;

        if len == 0 {
            return crc;
        }
    }

    crc32_be_arm64(crc, p, len)
}

pub unsafe fn crc32_optimizations_arch() -> u32 {
    if alternative_has_cap_likely(ARM64_HAS_CRC32) {
        return CRC32_LE_OPTIMIZATION | CRC32_BE_OPTIMIZATION | CRC32C_OPTIMIZATION;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
