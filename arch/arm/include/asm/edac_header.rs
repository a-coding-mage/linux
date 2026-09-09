/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright 2011 Calxeda, Inc.
 * Based on PPC version Copyright 2007 MontaVista Software, Inc.
 */

/*
 * ECC atomic, DMA, SMP and interrupt safe scrub function.
 * Implements the per arch edac_atomic_scrub() that EDAC use for software
 * ECC scrubbing.  It reads memory and then writes back the original
 * value, allowing the hardware to detect and correct memory errors.
 */

/// Scrub memory atomically, preserving the original value while allowing
/// hardware to detect and correct memory errors.
///
/// The C implementation includes the body only when `__LINUX_ARM_ARCH__ >= 6`.
/// That build-time condition is preserved here as an architecture/configuration
/// intent; the ARM exclusive-access sequence is emitted for ARM targets.
#[inline]
pub unsafe fn edac_atomic_scrub(va: *mut core::ffi::c_void, size: u32) {
    #[cfg(target_arch = "arm")]
    {
        let mut virt_addr = va as *mut u32;
        let mut i: u32 = 0;

        while i < size / core::mem::size_of::<u32>() as u32 {
            let mut temp: u32;
            let mut temp2: u32;

            /* Very carefully read and write to memory atomically
             * so we are interrupt, DMA and SMP safe.
             */
            core::arch::asm!(
                "1:",
                "ldrex {temp}, [{addr}]",
                "strex {status}, {temp}, [{addr}]",
                "teq {status}, #0",
                "bne 1b",
                temp = out(reg) temp,
                status = out(reg) temp2,
                addr = in(reg) virt_addr,
                options(nostack)
            );

            i = i.wrapping_add(1);
            virt_addr = virt_addr.add(1);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
