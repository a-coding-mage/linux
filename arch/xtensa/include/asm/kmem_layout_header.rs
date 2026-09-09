/*
 * Kernel virtual memory layout definitions.
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License.  See the file "COPYING" in the main directory of
 * this archive for more details.
 *
 * Copyright (C) 2016 Cadence Design Systems Inc.
 */

/* C dependencies: <asm/core.h> and <asm/types.h>. */

/* The following items are present when CONFIG_MMU is enabled. */
#[cfg(feature = "CONFIG_MMU")]
pub const XCHAL_PAGE_TABLE_VADDR: usize = 0x8000_0000;
#[cfg(feature = "CONFIG_MMU")]
pub const XCHAL_PAGE_TABLE_SIZE: usize = 0x0040_0000;

/* Fixed TLB translations in the processor. */

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_MMU_V2"))]
pub const XCHAL_KSEG_CACHED_VADDR: usize = 0xd000_0000;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_MMU_V2"))]
pub const XCHAL_KSEG_BYPASS_VADDR: usize = 0xd800_0000;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_MMU_V2"))]
pub const XCHAL_KSEG_SIZE: usize = 0x0800_0000;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_MMU_V2"))]
pub const XCHAL_KSEG_ALIGNMENT: usize = 0x0800_0000;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_MMU_V2"))]
pub const XCHAL_KSEG_TLB_WAY: usize = 5;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_MMU_V2"))]
pub const XCHAL_KIO_TLB_WAY: usize = 6;

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_256M"))]
pub const XCHAL_KSEG_CACHED_VADDR: usize = 0xb000_0000;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_256M"))]
pub const XCHAL_KSEG_BYPASS_VADDR: usize = 0xc000_0000;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_256M"))]
pub const XCHAL_KSEG_SIZE: usize = 0x1000_0000;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_256M"))]
pub const XCHAL_KSEG_ALIGNMENT: usize = 0x1000_0000;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_256M"))]
pub const XCHAL_KSEG_TLB_WAY: usize = 6;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_256M"))]
pub const XCHAL_KIO_TLB_WAY: usize = 6;

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_512M"))]
pub const XCHAL_KSEG_CACHED_VADDR: usize = 0xa000_0000;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_512M"))]
pub const XCHAL_KSEG_BYPASS_VADDR: usize = 0xc000_0000;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_512M"))]
pub const XCHAL_KSEG_SIZE: usize = 0x2000_0000;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_512M"))]
pub const XCHAL_KSEG_ALIGNMENT: usize = 0x1000_0000;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_512M"))]
pub const XCHAL_KSEG_TLB_WAY: usize = 6;
#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_XTENSA_KSEG_512M"))]
pub const XCHAL_KIO_TLB_WAY: usize = 6;

#[cfg(all(feature = "CONFIG_MMU", feature = "CONFIG_KSEG_PADDR"))]
pub const XCHAL_KSEG_PADDR: usize = CONFIG_KSEG_PADDR;
#[cfg(all(feature = "CONFIG_MMU", not(feature = "CONFIG_KSEG_PADDR")))]
pub const XCHAL_KSEG_PADDR: usize = 0x0000_0000;

/* KIO definition. */
#[cfg(feature = "XCHAL_HAVE_PTP_MMU")]
pub const XCHAL_KIO_CACHED_VADDR: usize = 0xe000_0000;
#[cfg(feature = "XCHAL_HAVE_PTP_MMU")]
pub const XCHAL_KIO_BYPASS_VADDR: usize = 0xf000_0000;
#[cfg(feature = "XCHAL_HAVE_PTP_MMU")]
pub const XCHAL_KIO_DEFAULT_PADDR: usize = 0xf000_0000;
#[cfg(not(feature = "XCHAL_HAVE_PTP_MMU"))]
pub const XCHAL_KIO_BYPASS_VADDR: usize = XCHAL_KIO_PADDR;
#[cfg(not(feature = "XCHAL_HAVE_PTP_MMU"))]
pub const XCHAL_KIO_DEFAULT_PADDR: usize = 0x9000_0000;
pub const XCHAL_KIO_SIZE: usize = 0x1000_0000;

/* CONFIG_USE_OF and XCHAL_HAVE_SPANNING_WAY are build-time conditions. */
#[cfg(any(
    all(not(feature = "XCHAL_HAVE_PTP_MMU"), feature = "CONFIG_USE_OF"),
    all(feature = "XCHAL_HAVE_SPANNING_WAY", feature = "CONFIG_USE_OF")
))]
extern "C" {
    pub static mut xtensa_kio_paddr: usize;
}

#[cfg(any(
    all(not(feature = "XCHAL_HAVE_PTP_MMU"), feature = "CONFIG_USE_OF"),
    all(feature = "XCHAL_HAVE_SPANNING_WAY", feature = "CONFIG_USE_OF")
))]
#[inline]
pub unsafe fn xtensa_get_kio_paddr() -> usize {
    xtensa_kio_paddr
}

#[cfg(any(
    all(not(feature = "XCHAL_HAVE_PTP_MMU"), feature = "CONFIG_USE_OF"),
    all(feature = "XCHAL_HAVE_SPANNING_WAY", feature = "CONFIG_USE_OF")
))]
pub const XCHAL_KIO_PADDR: usize = unsafe { xtensa_get_kio_paddr() };
#[cfg(not(any(
    all(not(feature = "XCHAL_HAVE_PTP_MMU"), feature = "CONFIG_USE_OF"),
    all(feature = "XCHAL_HAVE_SPANNING_WAY", feature = "CONFIG_USE_OF")
)))]
pub const XCHAL_KIO_PADDR: usize = XCHAL_KIO_DEFAULT_PADDR;

/* KERNEL_STACK definition. */
#[cfg(not(feature = "CONFIG_KASAN"))]
pub const KERNEL_STACK_SHIFT: usize = 13;
#[cfg(feature = "CONFIG_KASAN")]
pub const KERNEL_STACK_SHIFT: usize = 15;
pub const KERNEL_STACK_SIZE: usize = 1usize << KERNEL_STACK_SHIFT;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
