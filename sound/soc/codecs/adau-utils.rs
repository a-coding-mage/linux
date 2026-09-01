// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Shared helper functions for devices from the ADAU family
 *
 * Copyright 2011-2016 Analog Devices Inc.
 * Author: Lars-Peter Clausen <lars@metafoo.de>
 */

use core::ffi::{c_int, c_uint};

const EINVAL: c_int = 22;

const fn DIV_ROUND_UP(n: c_uint, d: c_uint) -> c_uint {
    (n + d - 1) / d
}

unsafe extern "C" {
    fn gcd(a: c_uint, b: c_uint) -> c_uint;
}

#[no_mangle]
pub unsafe extern "C" fn adau_calc_pll_cfg(
    mut freq_in: c_uint,
    freq_out: c_uint,
    regs: *mut u8,
) -> c_int {
    let r: c_uint;
    let n: c_uint;
    let m: c_uint;
    let i: c_uint;
    let j: c_uint;
    let div: c_uint;

    if freq_out == 0 {
        r = 0;
        n = 0;
        m = 0;
        div = 0;
    } else {
        if freq_out % freq_in != 0 {
            div = DIV_ROUND_UP(freq_in, 13500000) - 1;
            freq_in /= div + 1;
            r = freq_out / freq_in;
            i = freq_out % freq_in;
            j = gcd(i, freq_in);
            n = i / j;
            m = freq_in / j;
        } else {
            r = freq_out / freq_in;
            n = 0;
            m = 0;
            div = 0;
        }
        if n > 0xffff || m > 0xffff || div > 3 || r > 8 || r < 2 {
            return -EINVAL;
        }
    }

    *regs.add(0) = (m >> 8) as u8;
    *regs.add(1) = (m & 0xff) as u8;
    *regs.add(2) = (n >> 8) as u8;
    *regs.add(3) = (n & 0xff) as u8;
    *regs.add(4) = ((r << 3) | (div << 1)) as u8;
    if m != 0 {
        *regs.add(4) |= 1; /* Fractional mode */
    }

    0
}
// EXPORT_SYMBOL_GPL(adau_calc_pll_cfg);

// MODULE_DESCRIPTION("ASoC ADAU audio CODECs shared helper functions");
// MODULE_AUTHOR("Lars-Peter Clausen <lars@metafoo.de>");
// MODULE_LICENSE("GPL v2");

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
