/* SPDX-License-Identifier: GPL-2.0 */
/* delay.h: Linux delay routines on sparc64.
 *
 * Copyright (C) 1996, 2004, 2007 David S. Miller (davem@davemloft.net).
 */

/* The declarations below are omitted by the original header when assembling
 * rather than compiling C/Rust. */

unsafe extern "C" {
    pub fn __delay(loops: u64);
    pub fn udelay(usecs: u64);
}

#[macro_export]
macro_rules! mdelay {
    ($n:expr) => {
        $crate::udelay(($n) * 1000)
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
