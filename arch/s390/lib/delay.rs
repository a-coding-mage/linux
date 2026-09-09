// SPDX-License-Identifier: GPL-2.0
/*
 *    Precise Delay Loops for S390
 *
 *    Copyright IBM Corp. 1999, 2008
 *    Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>,
 */

unsafe extern "C" {
    fn get_tod_clock_monotonic() -> u64;
    fn tod_after(a: u64, b: u64) -> bool;
    fn cpu_relax();
}

pub unsafe fn __delay(mut loops: u64) {
    /*
     * Loop 'loops' times. Callers must not assume a specific
     * amount of time passes before this function returns.
     */
    loops = (loops / 2).wrapping_add(1);
    core::arch::asm!(
        "0: brct {count}, 0b",
        count = inout(reg) loops,
        options(nostack),
    );
}

unsafe fn delay_loop(delta: u64) {
    let end = get_tod_clock_monotonic().wrapping_add(delta);
    while !tod_after(get_tod_clock_monotonic(), end) {
        cpu_relax();
    }
}

pub unsafe fn __udelay(usecs: u64) {
    delay_loop(usecs.wrapping_shl(12));
}

pub unsafe fn __ndelay(mut nsecs: u64) {
    nsecs = nsecs.wrapping_shl(9);
    nsecs /= 125;
    delay_loop(nsecs);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
