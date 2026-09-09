/*
 * reset.c  -- common ColdFire SoC reset support
 *
 * (C) Copyright 2012, Greg Ungerer <gerg@uclinux.org>
 *
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file COPYING in the main directory of this archive
 * for more details.
 */

/* Dependencies supplied by the surrounding kernel and architecture code. */
extern "C" {
    static mut mach_reset: unsafe extern "C" fn();
    fn local_irq_disable();
    fn mcf_write8(value: u8, address: usize);
}

/*
 * There are 2 common methods amongst the ColdFire parts for reseting
 * the CPU. But there are couple of exceptions, the 5272 and the 547x
 * have something completely special to them, and we let their specific
 * subarch code handle them.
 */

#[cfg(MCFSIM_SYPCR)]
unsafe extern "C" fn mcf_cpu_reset() {
    local_irq_disable();
    /* Set watchdog to soft reset, and enabled */
    mcf_write8(0xc0, MCFSIM_SYPCR);
    loop {
        /* wait for watchdog to timeout */
    }
}

#[cfg(MCF_RCR)]
unsafe extern "C" fn mcf_cpu_reset() {
    local_irq_disable();
    mcf_write8(MCF_RCR_SWRESET, MCF_RCR);
}

unsafe extern "C" fn mcf_setup_reset() -> i32 {
    mach_reset = mcf_cpu_reset;
    0
}

/* Equivalent of the kernel's arch_initcall(mcf_setup_reset) registration. */
arch_initcall!(mcf_setup_reset);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
