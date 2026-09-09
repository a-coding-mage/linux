/* SPDX-License-Identifier: GPL-2.0 */

/*
 * 64-bit user address space can have multiple limits.
 * For now supported values are:
 */
pub const TASK_SIZE_64TB: u64 = 0x0000_4000_0000_0000;
pub const TASK_SIZE_128TB: u64 = 0x0000_8000_0000_0000;
pub const TASK_SIZE_512TB: u64 = 0x0002_0000_0000_0000;
pub const TASK_SIZE_1PB: u64 = 0x0004_0000_0000_0000;
pub const TASK_SIZE_2PB: u64 = 0x0008_0000_0000_0000;

/* With 52 bits in the address we can support up to 4PB of range. */
pub const TASK_SIZE_4PB: u64 = 0x0010_0000_0000_0000;

/* CONFIG_PPC_64K_PAGES selects the following values. */
#[cfg(feature = "CONFIG_PPC_64K_PAGES")]
pub const TASK_SIZE_USER64: u64 = TASK_SIZE_4PB;
#[cfg(feature = "CONFIG_PPC_64K_PAGES")]
pub const DEFAULT_MAP_WINDOW_USER64: u64 = TASK_SIZE_128TB;
#[cfg(feature = "CONFIG_PPC_64K_PAGES")]
pub const TASK_CONTEXT_SIZE: u64 = TASK_SIZE_512TB;

#[cfg(not(feature = "CONFIG_PPC_64K_PAGES"))]
pub const TASK_SIZE_USER64: u64 = TASK_SIZE_64TB;
#[cfg(not(feature = "CONFIG_PPC_64K_PAGES"))]
pub const DEFAULT_MAP_WINDOW_USER64: u64 = TASK_SIZE_64TB;

/* We do not need extended context ids for 4K page size. */
#[cfg(not(feature = "CONFIG_PPC_64K_PAGES"))]
pub const TASK_CONTEXT_SIZE: u64 = TASK_SIZE_64TB;

/* External symbols/macros supplied by the surrounding kernel translation. */
unsafe extern "C" {
    fn is_32bit_task() -> bool;
    fn PAGE_ALIGN(value: u64) -> u64;
}

pub const TASK_SIZE_USER32: u64 = 0x0000_0001_0000_0000 - (1 * PAGE_SIZE);

#[inline]
pub unsafe fn TASK_SIZE() -> u64 {
    if is_32bit_task() { TASK_SIZE_USER32 } else { TASK_SIZE_USER64 }
}

#[inline]
pub unsafe fn TASK_UNMAPPED_BASE_USER32() -> u64 {
    PAGE_ALIGN(TASK_SIZE_USER32 / 4)
}

#[inline]
pub unsafe fn TASK_UNMAPPED_BASE_USER64() -> u64 {
    PAGE_ALIGN(DEFAULT_MAP_WINDOW_USER64 / 4)
}

/* This decides where the kernel will search for a free chunk of vm space during mmap's. */
#[inline]
pub unsafe fn TASK_UNMAPPED_BASE() -> u64 {
    if is_32bit_task() { TASK_UNMAPPED_BASE_USER32() } else { TASK_UNMAPPED_BASE_USER64() }
}

/* Initial task size value for user applications. */
#[cfg(feature = "CONFIG_PPC_BOOK3S_64")]
#[inline]
pub unsafe fn DEFAULT_MAP_WINDOW() -> u64 {
    if is_32bit_task() { TASK_SIZE_USER32 } else { DEFAULT_MAP_WINDOW_USER64 }
}

#[cfg(not(feature = "CONFIG_PPC_BOOK3S_64"))]
#[inline]
pub unsafe fn DEFAULT_MAP_WINDOW() -> u64 { TASK_SIZE() }

pub const STACK_TOP_USER64: u64 = DEFAULT_MAP_WINDOW_USER64;
pub const STACK_TOP_USER32: u64 = TASK_SIZE_USER32;
pub const STACK_TOP_MAX: u64 = TASK_SIZE_USER64;

#[inline]
pub unsafe fn STACK_TOP() -> u64 {
    if is_32bit_task() { STACK_TOP_USER32 } else { STACK_TOP_USER64 }
}

#[inline]
pub unsafe fn arch_get_mmap_base(addr: u64, base: u64) -> u64 {
    if addr > DEFAULT_MAP_WINDOW() { base + TASK_SIZE() - DEFAULT_MAP_WINDOW() } else { base }
}

#[inline]
pub unsafe fn arch_get_mmap_end(addr: u64, len: u64, flags: u64) -> u64 {
    if addr > DEFAULT_MAP_WINDOW()
        || ((flags & MAP_FIXED) != 0 && addr + len > DEFAULT_MAP_WINDOW())
    {
        TASK_SIZE()
    } else {
        DEFAULT_MAP_WINDOW()
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
