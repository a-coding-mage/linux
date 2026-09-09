/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * OpenRISC Linux
 *
 * Copyright (C) 2010-2011 Jonas Bonn <jonas@southpole.se>
 */

// The C header guard is intentionally omitted in Rust.

#[cfg(CONFIG_OPENRISC_HAVE_INST_FL1)]
#[inline]
pub fn fls(x: u32) -> i32 {
    let mut ret: i32;

    unsafe {
        core::arch::asm!(
            "l.fl1 {ret}, {x}",
            ret = out(reg) ret,
            x = in(reg) x,
        );
    }

    ret
}

// When CONFIG_OPENRISC_HAVE_INST_FL1 is not enabled, the C header includes
// <asm-generic/bitops/fls.h>; that external dependency is supplied elsewhere.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
