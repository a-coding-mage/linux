// SPDX-License-Identifier: GPL-2.0-only
/*
 * Copyright 2006 PathScale, Inc.  All Rights Reserved.
 */

// Dependencies supplied by the surrounding kernel translation unit.
use core::ffi::c_void;

extern "C" {
    fn __raw_writel(value: u32, addr: *mut u32);
    fn __raw_readl(addr: *const u32) -> u32;
    fn __raw_writeq(value: u64, addr: *mut u64);
}

/**
 * __iowrite32_copy - copy data to MMIO space, in 32-bit units
 * @to: destination, in MMIO space (must be 32-bit aligned)
 * @from: source (must be 32-bit aligned)
 * @count: number of 32-bit quantities to copy
 *
 * Copy data from kernel space to MMIO space, in units of 32 bits at a
 * time.  Order of access is not guaranteed, nor is a memory barrier
 * performed afterwards.
 */
// The C source conditionally defines this symbol when no prior definition exists.
#[cfg(not(feature = "__iowrite32_copy"))]
#[no_mangle]
pub unsafe extern "C" fn __iowrite32_copy(to: *mut c_void, from: *const c_void, count: usize) {
    let mut dst = to as *mut u32;
    let mut src = from as *const u32;
    let end = src.add(count);

    while src < end {
        __raw_writel(*src, dst);
        src = src.add(1);
        dst = dst.add(1);
    }
}

/**
 * __ioread32_copy - copy data from MMIO space, in 32-bit units
 * @to: destination (must be 32-bit aligned)
 * @from: source, in MMIO space (must be 32-bit aligned)
 * @count: number of 32-bit quantities to copy
 *
 * Copy data from MMIO space to kernel space, in units of 32 bits at a
 * time.  Order of access is not guaranteed, nor is a memory barrier
 * performed afterwards.
 */
#[no_mangle]
pub unsafe extern "C" fn __ioread32_copy(to: *mut c_void, from: *const c_void, count: usize) {
    let mut dst = to as *mut u32;
    let mut src = from as *const u32;
    let end = src.add(count);

    while src < end {
        *dst = __raw_readl(src);
        dst = dst.add(1);
        src = src.add(1);
    }
}

/**
 * __iowrite64_copy - copy data to MMIO space, in 64-bit or 32-bit units
 * @to: destination, in MMIO space (must be 64-bit aligned)
 * @from: source (must be 64-bit aligned)
 * @count: number of 64-bit quantities to copy
 *
 * Copy data from kernel space to MMIO space, in units of 32 or 64 bits at a
 * time.  Order of access is not guaranteed, nor is a memory barrier
 * performed afterwards.
 */
// The C source conditionally defines this symbol when no prior definition exists.
#[cfg(not(feature = "__iowrite64_copy"))]
#[no_mangle]
pub unsafe extern "C" fn __iowrite64_copy(to: *mut c_void, from: *const c_void, count: usize) {
    // CONFIG_64BIT selects the 64-bit implementation in the original build.
    #[cfg(feature = "CONFIG_64BIT")]
    {
        let mut dst = to as *mut u64;
        let mut src = from as *const u64;
        let end = src.add(count);

        while src < end {
            __raw_writeq(*src, dst);
            src = src.add(1);
            dst = dst.add(1);
        }
    }

    #[cfg(not(feature = "CONFIG_64BIT"))]
    {
        __iowrite32_copy(to, from, count.wrapping_mul(2));
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
