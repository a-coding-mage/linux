/* SPDX-License-Identifier: GPL-2.0 */

/* 16M alignment for crash kernel regions */
pub const CRASH_ALIGN: usize = SZ_16M;

/*
 * Keep the crash kernel below this limit.
 *
 * Earlier 32-bits kernels would limit the kernel to the low 512 MB range
 * due to mapping restrictions.
 *
 * 64-bit kdump kernels need to be restricted to be under 64 TB, which is
 * the upper limit of system RAM in 4-level paging mode. Since the kdump
 * jump could be from 5-level paging to 4-level paging, the jump will fail if
 * the kernel is put above 64 TB, and during the 1st kernel bootup there's
 * no good way to detect the paging mode of the target kernel which will be
 * loaded for dumping.
 */
unsafe extern "C" {
    pub fn swiotlb_size_or_default() -> usize;
}

/* CONFIG_X86_32 selects the 32-bit limits; CONFIG_X86_64 selects the 64-bit limits. */
#[cfg(any(target_arch = "x86", feature = "CONFIG_X86_32"))]
pub const CRASH_ADDR_LOW_MAX: usize = SZ_512M;
#[cfg(any(target_arch = "x86", feature = "CONFIG_X86_32"))]
pub const CRASH_ADDR_HIGH_MAX: usize = SZ_512M;

#[cfg(any(target_arch = "x86_64", feature = "CONFIG_X86_64"))]
pub const CRASH_ADDR_LOW_MAX: usize = SZ_4G;
#[cfg(any(target_arch = "x86_64", feature = "CONFIG_X86_64"))]
pub const CRASH_ADDR_HIGH_MAX: usize = SZ_64T;

pub const DEFAULT_CRASH_KERNEL_LOW_SIZE: usize = crash_low_size_default();

#[inline]
pub unsafe fn crash_low_size_default() -> usize {
    // CONFIG_X86_64 provides the calculated default; other architectures return zero.
    #[cfg(any(target_arch = "x86_64", feature = "CONFIG_X86_64"))]
    {
        let swiotlb_size = unsafe { swiotlb_size_or_default() };
        core::cmp::max(swiotlb_size.wrapping_add(8usize << 20), 256usize << 20)
    }

    #[cfg(not(any(target_arch = "x86_64", feature = "CONFIG_X86_64")))]
    {
        0
    }
}

/* HAVE_ARCH_ADD_CRASH_RES_TO_IOMEM_EARLY */
pub const HAVE_ARCH_ADD_CRASH_RES_TO_IOMEM_EARLY: bool = true;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
