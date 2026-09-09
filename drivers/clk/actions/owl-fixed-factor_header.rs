/* SPDX-License-Identifier: GPL-2.0+ */
//
// OWL fixed factor clock driver
//
// Copyright (c) 2014 Actions Semi Inc.
// Author: David Liu <liuwei@actions-semi.com>
//
// Copyright (c) 2018 Linaro Ltd.
// Author: Manivannan Sadhasivam <manivannan.sadhasivam@linaro.org>

// Dependency supplied by owl-common.h.

/// Construct an OWL fixed-factor clock.
#[macro_export]
macro_rules! OWL_FIX_FACT {
    ($struct:ident, $name:expr, $parent:expr, $mul:expr, $div:expr, $flags:expr) => {
        let $struct = clk_fixed_factor {
            mult: $mul,
            div: $div,
            hw: clk_hw {
                init: CLK_HW_INIT(
                    $name,
                    $parent,
                    &clk_fixed_factor_ops,
                    $flags,
                ),
            },
        };
    };
}

extern "C" {
    pub static clk_fixed_factor_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
