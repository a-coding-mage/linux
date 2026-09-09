/* SPDX-License-Identifier: GPL-2.0 */

// Dependencies supplied by the corresponding translated headers:
// ccu_common.h, ccu_frac.h, and ccu_mux.h.

#[repr(C)]
pub struct ccu_mult_internal {
    pub offset: u8,
    pub shift: u8,
    pub width: u8,
    pub min: u8,
    pub max: u8,
}

#[macro_export]
macro_rules! _SUNXI_CCU_MULT_OFFSET_MIN_MAX {
    ($shift:expr, $width:expr, $offset:expr, $min:expr, $max:expr) => {
        ccu_mult_internal {
            min: $min,
            max: $max,
            offset: $offset,
            shift: $shift,
            width: $width,
        }
    };
}

#[macro_export]
macro_rules! _SUNXI_CCU_MULT_MIN {
    ($shift:expr, $width:expr, $min:expr) => {
        $crate::_SUNXI_CCU_MULT_OFFSET_MIN_MAX!($shift, $width, 1, $min, 0)
    };
}

#[macro_export]
macro_rules! _SUNXI_CCU_MULT_OFFSET {
    ($shift:expr, $width:expr, $offset:expr) => {
        $crate::_SUNXI_CCU_MULT_OFFSET_MIN_MAX!($shift, $width, $offset, 1, 0)
    };
}

#[macro_export]
macro_rules! _SUNXI_CCU_MULT {
    ($shift:expr, $width:expr) => {
        $crate::_SUNXI_CCU_MULT_OFFSET_MIN_MAX!($shift, $width, 1, 1, 0)
    };
}

#[repr(C)]
pub struct ccu_mult {
    pub enable: u32,
    pub lock: u32,

    pub frac: ccu_frac_internal,
    pub mult: ccu_mult_internal,
    pub mux: ccu_mux_internal,
    pub common: ccu_common,
}

// C macro retained as a Rust declaration-building macro. The CLK_HW_INIT
// expression and ccu_mult_ops are supplied by the translated dependencies.
#[macro_export]
macro_rules! SUNXI_CCU_N_WITH_GATE_LOCK {
    ($struct_name:ident, $name:expr, $parent:expr, $reg:expr,
     $mshift:expr, $mwidth:expr, $gate:expr, $lock:expr,
     $flags:expr) => {
        let $struct_name = ccu_mult {
            enable: $gate,
            lock: $lock,
            mult: $crate::_SUNXI_CCU_MULT!($mshift, $mwidth),
            common: ccu_common {
                reg: $reg,
                hw: clk_hw_init!($name, $parent, &ccu_mult_ops, $flags),
            },
            ..unsafe { core::mem::zeroed() }
        };
    };
}

pub unsafe fn hw_to_ccu_mult(hw: *mut clk_hw) -> *mut ccu_mult {
    let common: *mut ccu_common = hw_to_ccu_common(hw);

    container_of!(common, ccu_mult, common)
}

extern "C" {
    pub static ccu_mult_ops: clk_ops;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
