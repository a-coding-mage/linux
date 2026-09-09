/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 Maxime Ripard. All rights reserved.
 */

// C dependencies: <linux/clk-provider.h> and "ccu_common.h".

#[repr(C)]
pub struct ccu_phase {
    pub shift: u8,
    pub width: u8,
    pub common: ccu_common,
}

// Equivalent of the C SUNXI_CCU_PHASE declaration macro.
#[macro_export]
macro_rules! SUNXI_CCU_PHASE {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $shift:expr, $width:expr, $flags:expr) => {
        let mut $struct: ccu_phase = ccu_phase {
            shift: $shift,
            width: $width,
            common: ccu_common {
                reg: $reg,
                hw: clk_hw_init!($name, $parent, &ccu_phase_ops, $flags),
            },
        };
    };
}

pub unsafe fn hw_to_ccu_phase(hw: *mut clk_hw) -> *mut ccu_phase {
    let common: *mut ccu_common = hw_to_ccu_common(hw);
    (common as *mut u8).sub(core::mem::offset_of!(ccu_phase, common)) as *mut ccu_phase
}

pub unsafe extern "C" {
    pub static ccu_phase_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
