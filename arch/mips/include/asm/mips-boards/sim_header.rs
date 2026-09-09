/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2005 MIPS Technologies, Inc.  All rights reserved.
 */

pub const STATS_ON: i32 = 1;
pub const STATS_OFF: i32 = 2;
pub const STATS_CLEAR: i32 = 3;
pub const STATS_DUMP: i32 = 4;
pub const TRACE_ON: i32 = 5;
pub const TRACE_OFF: i32 = 6;

/// Execute the simulator configuration instruction with an immediate code.
#[macro_export]
macro_rules! simcfg {
    ($code:expr) => {{
        unsafe {
            core::arch::asm!(
                "sltiu $0, $0, {code}",
                code = const $code,
                options(nostack, nomem, preserves_flags)
            );
        }
    }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
