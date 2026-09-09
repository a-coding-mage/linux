/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2016 Maxime Ripard. All rights reserved.
 */

// Dependency intent preserved from <linux/clk-provider.h> and "ccu_common.h".

#[repr(C)]
pub struct ccu_gate {
    pub enable: u32,
    pub common: ccu_common,
}

#[macro_export]
macro_rules! SUNXI_CCU_GATE {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $gate:expr, $flags:expr) => {
        let $struct = ccu_gate {
            enable: $gate,
            common: ccu_common {
                reg: $reg,
                hw: clk_hw {
                    init: CLK_HW_INIT!($name, $parent, &ccu_gate_ops, $flags),
                },
                ..core::default::Default::default()
            },
        };
    };
}

#[macro_export]
macro_rules! SUNXI_CCU_GATE_HW {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $gate:expr, $flags:expr) => {
        let $struct = ccu_gate {
            enable: $gate,
            common: ccu_common {
                reg: $reg,
                hw: clk_hw {
                    init: CLK_HW_INIT_HW!($name, $parent, &ccu_gate_ops, $flags),
                },
                ..core::default::Default::default()
            },
        };
    };
}

#[macro_export]
macro_rules! SUNXI_CCU_GATE_FW {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $gate:expr, $flags:expr) => {
        let $struct = ccu_gate {
            enable: $gate,
            common: ccu_common {
                reg: $reg,
                hw: clk_hw {
                    init: CLK_HW_INIT_FW_NAME!($name, $parent, &ccu_gate_ops, $flags),
                },
                ..core::default::Default::default()
            },
        };
    };
}

/* The following macros allow the re-use of the data structure holding the parent info. */
#[macro_export]
macro_rules! SUNXI_CCU_GATE_HWS {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $gate:expr, $flags:expr) => {
        let $struct = ccu_gate {
            enable: $gate,
            common: ccu_common {
                reg: $reg,
                hw: clk_hw {
                    init: CLK_HW_INIT_HWS!($name, $parent, &ccu_gate_ops, $flags),
                },
                ..core::default::Default::default()
            },
        };
    };
}

#[macro_export]
macro_rules! SUNXI_CCU_GATE_HWS_WITH_PREDIV {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $gate:expr, $prediv:expr, $flags:expr) => {
        let $struct = ccu_gate {
            enable: $gate,
            common: ccu_common {
                reg: $reg,
                prediv: $prediv,
                features: CCU_FEATURE_ALL_PREDIV,
                hw: clk_hw {
                    init: CLK_HW_INIT_HWS!($name, $parent, &ccu_gate_ops, $flags),
                },
                ..core::default::Default::default()
            },
        };
    };
}

#[macro_export]
macro_rules! SUNXI_CCU_GATE_DATA {
    ($struct:ident, $name:expr, $data:expr, $reg:expr, $gate:expr, $flags:expr) => {
        let $struct = ccu_gate {
            enable: $gate,
            common: ccu_common {
                reg: $reg,
                hw: clk_hw {
                    init: CLK_HW_INIT_PARENTS_DATA!($name, $data, &ccu_gate_ops, $flags),
                },
                ..core::default::Default::default()
            },
        };
    };
}

#[macro_export]
macro_rules! SUNXI_CCU_GATE_DATA_WITH_PREDIV {
    ($struct:ident, $name:expr, $parent:expr, $reg:expr, $gate:expr, $prediv:expr, $flags:expr) => {
        let $struct = ccu_gate {
            enable: $gate,
            common: ccu_common {
                reg: $reg,
                prediv: $prediv,
                features: CCU_FEATURE_ALL_PREDIV,
                hw: clk_hw {
                    init: CLK_HW_INIT_PARENTS_DATA!($name, $parent, &ccu_gate_ops, $flags),
                },
                ..core::default::Default::default()
            },
        };
    };
}

#[inline]
pub unsafe fn hw_to_ccu_gate(hw: *mut clk_hw) -> *mut ccu_gate {
    let common = hw_to_ccu_common(hw);
    container_of!(common, ccu_gate, common)
}

extern "C" {
    pub fn ccu_gate_helper_disable(common: *mut ccu_common, gate: u32);
    pub fn ccu_gate_helper_enable(common: *mut ccu_common, gate: u32) -> i32;
    pub fn ccu_gate_helper_is_enabled(common: *mut ccu_common, gate: u32) -> i32;
    pub static ccu_gate_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
