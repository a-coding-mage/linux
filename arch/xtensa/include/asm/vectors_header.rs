/*
 * arch/xtensa/include/asm/xchal_vaddr_remap.h
 *
 * Xtensa macros for MMU V3 Support. Deals with re-mapping the Virtual
 * Memory Addresses from "Virtual == Physical" to their prevvious V2 MMU
 * mappings (KSEG at 0xD0000000 and KIO at 0XF0000000).
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2008 - 2012 Tensilica Inc.
 *
 * Pete Delaney <piet@tensilica.com>
 * Marc Gauthier <marc@tensilica.com
 */

// Dependencies supplied by asm/core.h and asm/kmem_layout.h are expected to
// provide the XCHAL_*, CONFIG_*, and _vecbase symbols used below.

// The original preprocessor condition is:
// defined(CONFIG_MMU) && XCHAL_HAVE_PTP_MMU && XCHAL_HAVE_SPANNING_WAY.
// Select the corresponding branch through the surrounding build configuration.
#[cfg(all(feature = "config_mmu", feature = "xchal_have_ptp_mmu", feature = "xchal_have_spanning_way", feature = "config_kernel_virtual_address"))]
pub const KERNELOFFSET: usize = CONFIG_KERNEL_VIRTUAL_ADDRESS;

#[cfg(all(feature = "config_mmu", feature = "xchal_have_ptp_mmu", feature = "xchal_have_spanning_way", not(feature = "config_kernel_virtual_address")))]
pub const KERNELOFFSET: usize = CONFIG_KERNEL_LOAD_ADDRESS + XCHAL_KSEG_CACHED_VADDR - XCHAL_KSEG_PADDR;

#[cfg(not(all(feature = "config_mmu", feature = "xchal_have_ptp_mmu", feature = "xchal_have_spanning_way")))]
pub const KERNELOFFSET: usize = CONFIG_KERNEL_LOAD_ADDRESS;

pub const RESET_VECTOR1_VADDR: usize = XCHAL_RESET_VECTOR1_VADDR;

#[cfg(feature = "config_vectors_addr")]
pub const VECBASE_VADDR: usize = CONFIG_VECTORS_ADDR;

// In the source this branch refers to the linker/assembly symbol _vecbase.
#[cfg(not(feature = "config_vectors_addr"))]
pub const VECBASE_VADDR: usize = _vecbase;

// The following declarations correspond to the XCHAL_HAVE_VECBASE branch.
#[cfg(feature = "xchal_have_vecbase")]
pub const USER_VECTOR_VADDR: usize = VECBASE_VADDR + XCHAL_USER_VECOFS;
#[cfg(feature = "xchal_have_vecbase")]
pub const KERNEL_VECTOR_VADDR: usize = VECBASE_VADDR + XCHAL_KERNEL_VECOFS;
#[cfg(feature = "xchal_have_vecbase")]
pub const DOUBLEEXC_VECTOR_VADDR: usize = VECBASE_VADDR + XCHAL_DOUBLEEXC_VECOFS;
#[cfg(feature = "xchal_have_vecbase")]
pub const WINDOW_VECTORS_VADDR: usize = VECBASE_VADDR + XCHAL_WINDOW_OF4_VECOFS;
#[cfg(feature = "xchal_have_vecbase")]
pub const INTLEVEL2_VECTOR_VADDR: usize = VECBASE_VADDR + XCHAL_INTLEVEL2_VECOFS;
#[cfg(feature = "xchal_have_vecbase")]
pub const INTLEVEL3_VECTOR_VADDR: usize = VECBASE_VADDR + XCHAL_INTLEVEL3_VECOFS;
#[cfg(feature = "xchal_have_vecbase")]
pub const INTLEVEL4_VECTOR_VADDR: usize = VECBASE_VADDR + XCHAL_INTLEVEL4_VECOFS;
#[cfg(feature = "xchal_have_vecbase")]
pub const INTLEVEL5_VECTOR_VADDR: usize = VECBASE_VADDR + XCHAL_INTLEVEL5_VECOFS;
#[cfg(feature = "xchal_have_vecbase")]
pub const INTLEVEL6_VECTOR_VADDR: usize = VECBASE_VADDR + XCHAL_INTLEVEL6_VECOFS;
#[cfg(feature = "xchal_have_vecbase")]
pub const INTLEVEL7_VECTOR_VADDR: usize = VECBASE_VADDR + XCHAL_INTLEVEL7_VECOFS;
#[cfg(feature = "xchal_have_vecbase")]
pub const DEBUG_VECTOR_VADDR: usize = VECBASE_VADDR + XCHAL_DEBUG_VECOFS;

/*
 * These XCHAL_* #defines from varian/core.h
 * are not valid to use with V3 MMU. Non-XCHAL
 * constants are defined above and should be used.
 */

// The original #else branch is selected when XCHAL_HAVE_VECBASE is absent.
#[cfg(not(feature = "xchal_have_vecbase"))]
pub const USER_VECTOR_VADDR: usize = XCHAL_USER_VECTOR_VADDR;
#[cfg(not(feature = "xchal_have_vecbase"))]
pub const KERNEL_VECTOR_VADDR: usize = XCHAL_KERNEL_VECTOR_VADDR;
#[cfg(not(feature = "xchal_have_vecbase"))]
pub const DOUBLEEXC_VECTOR_VADDR: usize = XCHAL_DOUBLEEXC_VECTOR_VADDR;
#[cfg(not(feature = "xchal_have_vecbase"))]
pub const WINDOW_VECTORS_VADDR: usize = XCHAL_WINDOW_VECTORS_VADDR;
#[cfg(not(feature = "xchal_have_vecbase"))]
pub const INTLEVEL2_VECTOR_VADDR: usize = XCHAL_INTLEVEL2_VECTOR_VADDR;
#[cfg(not(feature = "xchal_have_vecbase"))]
pub const INTLEVEL3_VECTOR_VADDR: usize = XCHAL_INTLEVEL3_VECTOR_VADDR;
#[cfg(not(feature = "xchal_have_vecbase"))]
pub const INTLEVEL4_VECTOR_VADDR: usize = XCHAL_INTLEVEL4_VECTOR_VADDR;
#[cfg(not(feature = "xchal_have_vecbase"))]
pub const INTLEVEL5_VECTOR_VADDR: usize = XCHAL_INTLEVEL5_VECTOR_VADDR;
#[cfg(not(feature = "xchal_have_vecbase"))]
pub const INTLEVEL6_VECTOR_VADDR: usize = XCHAL_INTLEVEL6_VECTOR_VADDR;
#[cfg(not(feature = "xchal_have_vecbase"))]
pub const INTLEVEL7_VECTOR_VADDR: usize = XCHAL_INTLEVEL6_VECTOR_VADDR;
#[cfg(not(feature = "xchal_have_vecbase"))]
pub const DEBUG_VECTOR_VADDR: usize = XCHAL_DEBUG_VECTOR_VADDR;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
