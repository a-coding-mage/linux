// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (C) 2020-2022 Loongson Technology Corporation Limited
 */

// Supplied by the corresponding kernel timing and processor dependencies.
extern "C" {
    fn get_cycles() -> u64;
    fn cpu_relax();
    static HZ: u64;
    static lpj_fine: u64;
}

#[no_mangle]
pub unsafe extern "C" fn __delay(cycles: u64) {
    let t0: u64 = get_cycles();

    while (get_cycles().wrapping_sub(t0) as u64) < cycles {
        cpu_relax();
    }
}

// EXPORT_SYMBOL(__delay);

/*
 * Division by multiplication: you don't have to worry about
 * loss of precision.
 *
 * Use only for very small delays ( < 1 msec).\tShould probably use a
 * lookup table, really, as the multiplications take much too long with
 * short delays.  This is a "reasonable" implementation, though (and
 * the first constant multiplications gets optimized away if the delay
 * is a constant)
 */

#[no_mangle]
pub unsafe extern "C" fn __udelay(us: u64) {
    __delay(
        us.wrapping_mul(0x000010c7u64)
            .wrapping_mul(HZ)
            .wrapping_mul(lpj_fine)
            >> 32,
    );
}

// EXPORT_SYMBOL(__udelay);

#[no_mangle]
pub unsafe extern "C" fn __ndelay(ns: u64) {
    __delay(
        ns.wrapping_mul(0x00000005u64)
            .wrapping_mul(HZ)
            .wrapping_mul(lpj_fine)
            >> 32,
    );
}

// EXPORT_SYMBOL(__ndelay);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
