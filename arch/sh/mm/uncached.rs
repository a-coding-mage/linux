// SPDX-License-Identifier: GPL-2.0

// Dependencies supplied by the Linux headers/build configuration:
// <linux/sizes.h>, <asm/page.h>, and <asm/addrspace.h>.

const SZ_512M: usize = 512 * 1024 * 1024;

// `unsigned long` in the source; represented with the target's pointer-sized
// unsigned integer in Rust.
pub static mut cached_to_uncached: usize = SZ_512M;
pub static mut uncached_size: usize = SZ_512M;
pub static mut uncached_start: usize = 0;
pub static mut uncached_end: usize = 0;

// EXPORT_SYMBOL(uncached_start);
// EXPORT_SYMBOL(uncached_end);

pub unsafe fn virt_addr_uncached(kaddr: usize) -> i32 {
    ((kaddr >= uncached_start) && (kaddr < uncached_end)) as i32
}

// EXPORT_SYMBOL(virt_addr_uncached);

// `P2SEG` and `memory_end` are supplied by the architecture headers/build.
unsafe extern "C" {
    static memory_end: usize;
}

pub unsafe fn uncached_init() {
    #[cfg(any(feature = "CONFIG_29BIT", not(feature = "CONFIG_MMU")))]
    {
        // P2SEG
        uncached_start = P2SEG;
    }
    #[cfg(all(not(feature = "CONFIG_29BIT"), feature = "CONFIG_MMU"))]
    {
        uncached_start = memory_end;
    }
    uncached_end = uncached_start + uncached_size;
}

pub unsafe fn uncached_resize(size: usize) {
    uncached_size = size;
    uncached_end = uncached_start + uncached_size;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
