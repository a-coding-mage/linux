/*
 * include/asm-xtensa/dma.h
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003 - 2005 Tensilica Inc.
 */

/* C header guard: _XTENSA_DMA_H */

/* Dependency: <asm/io.h> is required for byte I/O. */

/*
 * This is only to be defined if we have PC-like DMA.
 * By default this is not true on an Xtensa processor,
 * however on boards with a PCI bus, such functionality
 * might be emulated externally.
 *
 * NOTE:  there still exists driver code that assumes
 * this is defined, eg. drivers/sound/soundcard.c (as of 2.4).
 */
pub const MAX_DMA_CHANNELS: i32 = 8;

/*
 * The maximum virtual address to which DMA transfers
 * can be performed on this platform.
 *
 * NOTE: This is board (platform) specific, not processor-specific!
 *
 * NOTE: This assumes DMA transfers can only be performed on
 *     the section of physical memory contiguously mapped in virtual
 *     space for the kernel.  For the Xtensa architecture, this
 *     means the maximum possible size of this DMA area is
 *     the size of the statically mapped kernel segment
 *     (XCHAL_KSEG_{CACHED,BYPASS}_SIZE), ie. 128 MB.
 *
 * NOTE: When the entire KSEG area is DMA capable, we subtract
 *     one from the max address so that the virt_to_phys() macro
 *     works correctly on the address (otherwise the address
 *     enters another area, and virt_to_phys() may not return
 *     the value desired).
 */

/* MAX_DMA_ADDRESS is conditionally defined by the build configuration. */
pub const MAX_DMA_ADDRESS: usize = PAGE_OFFSET + XCHAL_KIO_SIZE - 1;

/* Reserve and release a DMA channel */
unsafe extern "C" {
    pub fn request_dma(dmanr: u32, device_id: *const core::ffi::c_char) -> core::ffi::c_int;
    pub fn free_dma(dmanr: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
