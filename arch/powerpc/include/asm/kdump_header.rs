/* SPDX-License-Identifier: GPL-2.0 */

// Dependency: PAGE_OFFSET is supplied by asm/page.h.

pub const KDUMP_KERNELBASE: usize = 0x2000000;

/* How many bytes to reserve at zero for kdump. The reserve limit should
 * be greater or equal to the trampoline's end address.
 * Reserve to the end of the FWNMI area, see head_64.S */
pub const KDUMP_RESERVE_LIMIT: usize = 0x10000; /* 64K */

/*
 * On PPC64 translation is disabled during trampoline setup, so we use
 * physical addresses. Though on PPC32 translation is already enabled,
 * so we can't do the same. Luckily create_trampoline() creates relative
 * branches, so we can just add the PAGE_OFFSET and don't worry about it.
 *
 * Build-time condition: CONFIG_CRASH_DUMP and the target architecture
 * select whether these constants are present and which address form is used.
 */
#[cfg(target_pointer_width = "64")]
pub const KDUMP_TRAMPOLINE_START: usize = 0x0100;
#[cfg(target_pointer_width = "64")]
pub const KDUMP_TRAMPOLINE_END: usize = 0x3000;

#[cfg(not(target_pointer_width = "64"))]
pub const KDUMP_TRAMPOLINE_START: usize = 0x0100 + PAGE_OFFSET;
#[cfg(not(target_pointer_width = "64"))]
pub const KDUMP_TRAMPOLINE_END: usize = 0x3000 + PAGE_OFFSET;

pub const KDUMP_MIN_TCE_ENTRIES: usize = 2048;

/*
 * CONFIG_CRASH_DUMP and CONFIG_NONSTATIC_KERNEL conditional declarations.
 * The external functions are provided by the surrounding kernel.
 */
#[cfg(all(feature = "CONFIG_CRASH_DUMP", not(feature = "CONFIG_NONSTATIC_KERNEL")))]
unsafe extern "C" {
    pub fn reserve_kdump_trampoline();
    pub fn setup_kdump_trampoline();
}

// !CRASH_DUMP || !NONSTATIC_KERNEL
#[inline]
#[cfg(not(all(feature = "CONFIG_CRASH_DUMP", not(feature = "CONFIG_NONSTATIC_KERNEL"))))]
pub fn reserve_kdump_trampoline() {}

#[inline]
#[cfg(not(all(feature = "CONFIG_CRASH_DUMP", not(feature = "CONFIG_NONSTATIC_KERNEL"))))]
pub fn setup_kdump_trampoline() {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
