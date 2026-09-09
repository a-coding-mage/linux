/*
 * PPC EDAC common defs
 *
 * Author: Dave Jiang <djiang@mvista.com>
 *
 * 2007 (c) MontaVista Software, Inc. This file is licensed under
 * the terms of the GNU General Public License version 2. This program
 * is licensed "as is" without any warranty of any kind, whether express
 * or implied.
 */

/*
 * ECC atomic, DMA, SMP and interrupt safe scrub function.
 * Implements the per arch edac_atomic_scrub() that EDAC use for software
 * ECC scrubbing.  It reads memory and then writes back the original
 * value, allowing the hardware to detect and correct memory errors.
 */
pub unsafe fn edac_atomic_scrub(va: *mut core::ffi::c_void, size: u32) {
    let mut virt_addr = va as *mut u32;
    let mut temp: u32;
    let mut i: u32 = 0;

    while i < size / core::mem::size_of::<u32>() as u32 {
        /* Very carefully read and write to memory atomically
         * so we are interrupt, DMA and SMP safe.
         */
        core::arch::asm!(
            "1:",
            "lwarx {temp}, 0, {addr}",
            "stwcx. {temp}, 0, {addr}",
            "bne- 1b",
            "isync",
            temp = lateout(reg) temp,
            addr = in(reg) virt_addr,
            /* The C source declares condition-register field 0 and memory
             * as clobbered by this atomic sequence.
             */
            options(nostack)
        );
        virt_addr = virt_addr.add(1);
        i = i.wrapping_add(1);
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
