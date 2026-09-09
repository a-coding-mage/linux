// SPDX-License-Identifier: GPL-2.0-only

use core::ffi::c_void;

// External declarations supplied by the surrounding kernel I/O implementation.
extern "C" {
    fn __ioread32_copy(to: *mut c_void, from: *const c_void, count: usize);
    fn __raw_readq(addr: *const u64) -> u64;
}

/**
 * __ioread64_copy - copy data from MMIO space, in 64-bit units
 * @to: destination (must be 64-bit aligned)
 * @from: source, in MMIO space (must be 64-bit aligned)
 * @count: number of 64-bit quantities to copy
 *
 * Copy data from MMIO space to kernel space, in units of 32 or 64 bits at a
 * time.  Order of access is not guaranteed, nor is a memory barrier
 * performed afterwards.
 */
#[cfg(CONFIG_64BIT)]
pub unsafe fn __ioread64_copy(to: *mut c_void, from: *const c_void, count: usize) {
    let mut dst = to as *mut u64;
    let mut src = from as *const u64;
    let end = src.add(count);

    while src < end {
        *dst = __raw_readq(src);
        dst = dst.add(1);
        src = src.add(1);
    }
}

#[cfg(not(CONFIG_64BIT))]
pub unsafe fn __ioread64_copy(to: *mut c_void, from: *const c_void, count: usize) {
    __ioread32_copy(to, from, count.wrapping_mul(2));
}

// EXPORT_SYMBOL_GPL(__ioread64_copy);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
