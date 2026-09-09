/* SPDX-License-Identifier: GPL-2.0 */

// Translated from ccu_mux.h. Declarations supplied by the included kernel and
// CCU headers remain external dependencies.

#[repr(C)]
pub struct ccu_mux_fixed_prediv {
    pub index: u8,
    pub div: u16,
}

#[repr(C)]
pub struct ccu_mux_var_prediv {
    pub index: u8,
    pub shift: u8,
    pub width: u8,
}

#[repr(C)]
pub struct ccu_mux_internal {
    pub shift: u8,
    pub width: u8,
    pub table: *const u8,

    pub fixed_predivs: *const ccu_mux_fixed_prediv,
    pub n_predivs: u8,

    pub var_predivs: *const ccu_mux_var_prediv,
    pub n_var_predivs: u8,
}

#[macro_export]
macro_rules! _SUNXI_CCU_MUX_TABLE {
    ($shift:expr, $width:expr, $table:expr) => {
        ccu_mux_internal {
            shift: $shift,
            width: $width,
            table: $table,
            fixed_predivs: core::ptr::null(),
            n_predivs: 0,
            var_predivs: core::ptr::null(),
            n_var_predivs: 0,
        }
    };
}

#[macro_export]
macro_rules! _SUNXI_CCU_MUX {
    ($shift:expr, $width:expr) => {
        _SUNXI_CCU_MUX_TABLE!($shift, $width, core::ptr::null())
    };
}

#[repr(C)]
pub struct ccu_mux {
    pub enable: u32,
    pub mux: ccu_mux_internal,
    pub common: ccu_common,
}

#[macro_export]
macro_rules! SUNXI_CCU_MUX_TABLE_WITH_GATE_FEAT {
    ($struct:ident, $name:expr, $parents:expr, $table:expr, $reg:expr,
     $shift:expr, $width:expr, $gate:expr, $flags:expr, $features:expr) => {
        let $struct = ccu_mux {
            enable: $gate,
            mux: _SUNXI_CCU_MUX_TABLE!($shift, $width, $table),
            common: ccu_common {
                reg: $reg,
                hw: CLK_HW_INIT_PARENTS!($name, $parents, &ccu_mux_ops, $flags),
                features: $features,
            },
        };
    };
}

#[macro_export]
macro_rules! SUNXI_CCU_MUX_TABLE_WITH_GATE_CLOSEST {
    ($struct:ident, $name:expr, $parents:expr, $table:expr, $reg:expr,
     $shift:expr, $width:expr, $gate:expr, $flags:expr) => {
        SUNXI_CCU_MUX_TABLE_WITH_GATE_FEAT!($struct, $name, $parents, $table,
            $reg, $shift, $width, $gate, $flags, CCU_FEATURE_CLOSEST_RATE)
    };
}

#[macro_export]
macro_rules! SUNXI_CCU_MUX_TABLE_WITH_GATE {
    ($struct:ident, $name:expr, $parents:expr, $table:expr, $reg:expr,
     $shift:expr, $width:expr, $gate:expr, $flags:expr) => {
        SUNXI_CCU_MUX_TABLE_WITH_GATE_FEAT!($struct, $name, $parents, $table,
            $reg, $shift, $width, $gate, $flags, 0)
    };
}

#[macro_export]
macro_rules! SUNXI_CCU_MUX_WITH_GATE {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr, $shift:expr,
     $width:expr, $gate:expr, $flags:expr) => {
        SUNXI_CCU_MUX_TABLE_WITH_GATE!($struct, $name, $parents,
            core::ptr::null(), $reg, $shift, $width, $gate, $flags)
    };
}

#[macro_export]
macro_rules! SUNXI_CCU_MUX {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr, $shift:expr,
     $width:expr, $flags:expr) => {
        SUNXI_CCU_MUX_TABLE_WITH_GATE!($struct, $name, $parents,
            core::ptr::null(), $reg, $shift, $width, 0, $flags)
    };
}

#[macro_export]
macro_rules! SUNXI_CCU_MUX_DATA_WITH_GATE {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr, $shift:expr,
     $width:expr, $gate:expr, $flags:expr) => {
        let $struct = ccu_mux {
            enable: $gate,
            mux: _SUNXI_CCU_MUX!($shift, $width),
            common: ccu_common {
                reg: $reg,
                hw: CLK_HW_INIT_PARENTS_DATA!($name, $parents, &ccu_mux_ops, $flags),
            },
        };
    };
}

#[macro_export]
macro_rules! SUNXI_CCU_MUX_DATA {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr, $shift:expr,
     $width:expr, $flags:expr) => {
        SUNXI_CCU_MUX_DATA_WITH_GATE!($struct, $name, $parents, $reg,
            $shift, $width, 0, $flags)
    };
}

#[macro_export]
macro_rules! SUNXI_CCU_MUX_HW_WITH_GATE {
    ($struct:ident, $name:expr, $parents:expr, $reg:expr, $shift:expr,
     $width:expr, $gate:expr, $flags:expr) => {
        let $struct = ccu_mux {
            enable: $gate,
            mux: _SUNXI_CCU_MUX!($shift, $width),
            common: ccu_common {
                reg: $reg,
                hw: CLK_HW_INIT_PARENTS_HW!($name, $parents, &ccu_mux_ops, $flags),
            },
        };
    };
}

pub unsafe fn hw_to_ccu_mux(hw: *mut clk_hw) -> *mut ccu_mux {
    let common = hw_to_ccu_common(hw);
    container_of!(common, ccu_mux, common)
}

extern "C" {
    pub static ccu_mux_ops: clk_ops;

    pub fn ccu_mux_helper_apply_prediv(
        common: *mut ccu_common,
        cm: *mut ccu_mux_internal,
        parent_index: i32,
        parent_rate: c_ulong,
    ) -> c_ulong;
    pub fn ccu_mux_helper_determine_rate(
        common: *mut ccu_common,
        cm: *mut ccu_mux_internal,
        req: *mut clk_rate_request,
        round: Option<unsafe extern "C" fn(*mut ccu_mux_internal, *mut clk_rate_request, *mut core::ffi::c_void) -> i32>,
        data: *mut core::ffi::c_void,
    ) -> i32;
    pub fn ccu_mux_helper_get_parent(common: *mut ccu_common, cm: *mut ccu_mux_internal) -> u8;
    pub fn ccu_mux_helper_set_parent(common: *mut ccu_common, cm: *mut ccu_mux_internal, index: u8) -> i32;
}

#[repr(C)]
pub struct ccu_mux_nb {
    pub clk_nb: notifier_block,
    pub common: *mut ccu_common,
    pub cm: *mut ccu_mux_internal,
    pub delay_us: u32, // How many us to wait after reparenting
    pub bypass_index: u8, // Which parent to temporarily use
    pub original_index: u8, // This is set by the notifier callback
}

#[macro_export]
macro_rules! to_ccu_mux_nb {
    ($nb:expr) => {
        container_of!($nb, ccu_mux_nb, clk_nb)
    };
}

extern "C" {
    pub fn ccu_mux_notifier_register(clk: *mut clk, mux_nb: *mut ccu_mux_nb) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
