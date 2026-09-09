/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2017, Michael Ellerman, IBM Corporation.
 */

// C header guard: _LINUX_SET_MEMORY_H_

// CONFIG_ARCH_HAS_SET_MEMORY selects the architecture-provided declarations.
// Otherwise, these are the local no-op fallbacks.
#[cfg(not(CONFIG_ARCH_HAS_SET_MEMORY))]
#[inline]
pub unsafe fn set_memory_ro(_addr: usize, _numpages: i32) -> i32 {
    0
}

#[cfg(not(CONFIG_ARCH_HAS_SET_MEMORY))]
#[inline]
pub unsafe fn set_memory_rw(_addr: usize, _numpages: i32) -> i32 {
    0
}

#[cfg(not(CONFIG_ARCH_HAS_SET_MEMORY))]
#[inline]
pub unsafe fn set_memory_x(_addr: usize, _numpages: i32) -> i32 {
    0
}

#[cfg(not(CONFIG_ARCH_HAS_SET_MEMORY))]
#[inline]
pub unsafe fn set_memory_nx(_addr: usize, _numpages: i32) -> i32 {
    0
}

// <asm/set_memory.h> supplies these declarations when CONFIG_ARCH_HAS_SET_MEMORY
// is enabled.

// Preserved from the C conditional definition: define this only when the
// architecture does not provide set_memory_rox.
#[cfg(not(set_memory_rox))]
#[inline]
pub unsafe fn set_memory_rox(addr: usize, numpages: i32) -> i32 {
    let ret = set_memory_ro(addr, numpages);
    if ret != 0 {
        return ret;
    }
    set_memory_x(addr, numpages)
}

// `struct page` is supplied by the surrounding kernel translation.
#[cfg(not(CONFIG_ARCH_HAS_SET_DIRECT_MAP))]
#[inline]
pub unsafe fn set_direct_map_invalid_noflush(_page: *mut page) -> i32 {
    0
}

#[cfg(not(CONFIG_ARCH_HAS_SET_DIRECT_MAP))]
#[inline]
pub unsafe fn set_direct_map_default_noflush(_page: *mut page) -> i32 {
    0
}

#[cfg(not(CONFIG_ARCH_HAS_SET_DIRECT_MAP))]
#[inline]
pub unsafe fn set_direct_map_valid_noflush(
    _page: *mut page,
    _nr: usize,
    _valid: bool,
) -> i32 {
    0
}

#[cfg(not(CONFIG_ARCH_HAS_SET_DIRECT_MAP))]
#[inline]
pub unsafe fn kernel_page_present(_page: *mut page) -> bool {
    true
}

// CONFIG_ARCH_HAS_SET_DIRECT_MAP: some architectures, e.g. ARM64, can
// disable direct map modifications at boot time. Let them override this query.
#[cfg(all(CONFIG_ARCH_HAS_SET_DIRECT_MAP, not(can_set_direct_map)))]
#[inline]
pub unsafe fn can_set_direct_map() -> bool {
    true
}

// CONFIG_X86_64 selects architecture declarations; otherwise use no-op forms.
#[cfg(CONFIG_X86_64)]
extern "C" {
    pub fn set_mce_nospec(pfn: usize) -> i32;
    pub fn clear_mce_nospec(pfn: usize) -> i32;
}

#[cfg(not(CONFIG_X86_64))]
#[inline]
pub unsafe fn set_mce_nospec(_pfn: usize) -> i32 {
    0
}

#[cfg(not(CONFIG_X86_64))]
#[inline]
pub unsafe fn clear_mce_nospec(_pfn: usize) -> i32 {
    0
}

#[cfg(not(CONFIG_ARCH_HAS_MEM_ENCRYPT))]
#[inline]
pub unsafe fn set_memory_encrypted(_addr: usize, _numpages: i32) -> i32 {
    0
}

#[cfg(not(CONFIG_ARCH_HAS_MEM_ENCRYPT))]
#[inline]
pub unsafe fn set_memory_decrypted(_addr: usize, _numpages: i32) -> i32 {
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
