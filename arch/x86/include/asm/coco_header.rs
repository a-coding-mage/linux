/* SPDX-License-Identifier: GPL-2.0 */

// The C header includes asm/asm.h and asm/types.h; their supplied symbols are
// expected to be available from the surrounding translation unit.

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum cc_vendor {
    CC_VENDOR_NONE,
    CC_VENDOR_AMD,
    CC_VENDOR_INTEL,
}

// CONFIG_ARCH_HAS_CC_PLATFORM is a build-time configuration condition carried
// over from the original header.
#[cfg(CONFIG_ARCH_HAS_CC_PLATFORM)]
unsafe extern "C" {
    pub static mut cc_vendor: cc_vendor;
    pub static mut cc_mask: u64;
}

#[cfg(CONFIG_ARCH_HAS_CC_PLATFORM)]
#[inline]
pub unsafe fn cc_get_mask() -> u64 {
    unsafe { cc_mask }
}

#[cfg(CONFIG_ARCH_HAS_CC_PLATFORM)]
#[inline]
pub unsafe fn cc_set_mask(mask: u64) {
    unsafe {
        cc_mask = mask;
    }
}

#[cfg(CONFIG_ARCH_HAS_CC_PLATFORM)]
unsafe extern "C" {
    pub fn cc_mkenc(val: u64) -> u64;
    pub fn cc_mkdec(val: u64) -> u64;
    pub fn cc_random_init();
}

#[cfg(not(CONFIG_ARCH_HAS_CC_PLATFORM))]
pub const cc_vendor: cc_vendor = cc_vendor::CC_VENDOR_NONE;

#[cfg(not(CONFIG_ARCH_HAS_CC_PLATFORM))]
#[inline]
pub const fn cc_get_mask() -> u64 {
    0
}

#[cfg(not(CONFIG_ARCH_HAS_CC_PLATFORM))]
#[inline]
pub const fn cc_mkenc(val: u64) -> u64 {
    val
}

#[cfg(not(CONFIG_ARCH_HAS_CC_PLATFORM))]
#[inline]
pub const fn cc_mkdec(val: u64) -> u64 {
    val
}

#[cfg(not(CONFIG_ARCH_HAS_CC_PLATFORM))]
#[inline]
pub const fn cc_random_init() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
