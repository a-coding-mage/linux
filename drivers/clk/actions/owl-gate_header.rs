/* SPDX-License-Identifier: GPL-2.0+ */
//
// OWL gate clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// Dependency intent: declarations from "owl-common.h" are supplied by the
// surrounding translation unit.

#[repr(C)]
pub struct owl_gate_hw {
    pub reg: u32,
    pub bit_idx: u8,
    pub gate_flags: u8,
}

#[repr(C)]
pub struct owl_gate {
    pub gate_hw: owl_gate_hw,
    pub common: owl_clk_common,
}

#[macro_export]
macro_rules! OWL_GATE_HW {
    ($reg:expr, $bit_idx:expr, $gate_flags:expr) => {
        owl_gate_hw {
            reg: $reg,
            bit_idx: $bit_idx,
            gate_flags: $gate_flags,
        }
    };
}

#[macro_export]
macro_rules! OWL_GATE {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $bit_idx:expr,
     $gate_flags:expr, $flags:expr) => {
        let mut $struct: owl_gate = owl_gate {
            gate_hw: OWL_GATE_HW!($reg, $bit_idx, $gate_flags),
            common: owl_clk_common {
                regmap: core::ptr::null_mut(),
                hw: clk_hw {
                    init: CLK_HW_INIT!($name, $parent, &owl_gate_ops, $flags),
                },
            },
        };
    };
}

#[macro_export]
macro_rules! OWL_GATE_NO_PARENT {
    ($struct:ident, $name:expr, $reg:expr, $bit_idx:expr,
     $gate_flags:expr, $flags:expr) => {
        let mut $struct: owl_gate = owl_gate {
            gate_hw: OWL_GATE_HW!($reg, $bit_idx, $gate_flags),
            common: owl_clk_common {
                regmap: core::ptr::null_mut(),
                hw: clk_hw {
                    init: CLK_HW_INIT_NO_PARENT!($name, &owl_gate_ops, $flags),
                },
            },
        };
    };
}

pub unsafe fn hw_to_owl_gate(hw: *mut clk_hw) -> *mut owl_gate {
    let common: *mut owl_clk_common = hw_to_owl_clk_common(hw);
    // C container_of(common, struct owl_gate, common).
    (common as *mut u8).sub(core::mem::offset_of!(owl_gate, common)) as *mut owl_gate
}

pub unsafe extern "C" fn owl_gate_set(
    common: *const owl_clk_common,
    gate_hw: *const owl_gate_hw,
    enable: bool,
);

pub unsafe extern "C" fn owl_gate_clk_is_enabled(
    common: *const owl_clk_common,
    gate_hw: *const owl_gate_hw,
) -> i32;

// External definition supplied by the clock driver implementation.
extern "C" {
    pub static owl_gate_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
