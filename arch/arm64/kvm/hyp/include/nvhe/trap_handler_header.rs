/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Trap handler helpers.
 *
 * Copyright (C) 2020 - Google LLC
 * Author: Marc Zyngier <maz@kernel.org>
 */

// Dependency supplied by the surrounding KVM host definitions.

/// Access a register from a trap-handler context.
#[macro_export]
macro_rules! cpu_reg {
    ($ctxt:expr, $r:expr) => {
        ($ctxt).regs.regs[$r]
    };
}

/// Declare a register value, retaining the C macro's intentionally unused
/// register-check declaration.
#[macro_export]
macro_rules! declare_reg {
    ($type:ty, $name:ident, $ctxt:expr, $reg:expr) => {
        let _: i32 = 0;
        let $name: $type = cpu_reg!($ctxt, $reg) as $type;
    };
}

extern "C" {
    pub fn inject_host_exception(esr: u64);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
