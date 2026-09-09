/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: asm/cpufeature.h supplies cpu_feature_enabled and
// X86_FEATURE_OSPKE, while the PKRU instructions are supplied externally.

pub const PKRU_AD_BIT: u32 = 0x1u32;
pub const PKRU_WD_BIT: u32 = 0x2u32;
pub const PKRU_BITS_PER_PKEY: u32 = 2;

// CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS selects which declaration/value is used.
#[cfg(feature = "CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS")]
extern "C" {
    pub static mut init_pkru_value: u32;
}

#[cfg(feature = "CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS")]
#[inline(always)]
pub unsafe fn pkru_get_init_value() -> u32 {
    // READ_ONCE(init_pkru_value)
    core::ptr::read_volatile(core::ptr::addr_of!(init_pkru_value))
}

#[cfg(not(feature = "CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS"))]
pub const init_pkru_value: u32 = 0;

#[cfg(not(feature = "CONFIG_X86_INTEL_MEMORY_PROTECTION_KEYS"))]
#[inline(always)]
pub const fn pkru_get_init_value() -> u32 {
    0
}

#[inline(always)]
pub fn __pkru_allows_read(pkru: u32, pkey: u16) -> bool {
    let pkru_pkey_bits: i32 = (pkey as i32) * (PKRU_BITS_PER_PKEY as i32);
    !(pkru & (PKRU_AD_BIT << (pkru_pkey_bits as u32)) != 0)
}

#[inline(always)]
pub fn __pkru_allows_write(pkru: u32, pkey: u16) -> bool {
    let pkru_pkey_bits: i32 = (pkey as i32) * (PKRU_BITS_PER_PKEY as i32);
    /*
     * Access-disable disables writes too so we need to check
     * both bits here.
     */
    !(pkru & ((PKRU_AD_BIT | PKRU_WD_BIT) << (pkru_pkey_bits as u32)) != 0)
}

#[inline(always)]
pub unsafe fn read_pkru() -> u32 {
    if cpu_feature_enabled(X86_FEATURE_OSPKE) {
        return rdpkru();
    }
    0
}

#[inline(always)]
pub unsafe fn write_pkru(pkru: u32) {
    if !cpu_feature_enabled(X86_FEATURE_OSPKE) {
        return;
    }
    /*
     * WRPKRU is relatively expensive compared to RDPKRU.
     * Avoid WRPKRU when it would not change the value.
     */
    if pkru != rdpkru() {
        wrpkru(pkru);
    }
}

#[inline(always)]
pub unsafe fn pkru_write_default() {
    if !cpu_feature_enabled(X86_FEATURE_OSPKE) {
        return;
    }

    wrpkru(pkru_get_init_value());
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
