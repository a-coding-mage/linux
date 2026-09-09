/* SPDX-License-Identifier: (GPL-2.0+ OR MIT) */
/*
 * Copyright (c) 2023 Neil Armstrong <neil.armstrong@linaro.org>
 */

// Dependencies supplied by the Linux clock and device-tree subsystems are
// intentionally left as external crate/module items.

#[repr(C)]
pub struct platform_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct meson_clk_hw_data {
    pub hws: *mut *mut clk_hw,
    pub num: ::core::ffi::c_uint,
}

unsafe extern "C" {
    pub fn meson_clk_hw_get(
        clkspec: *mut of_phandle_args,
        clk_hw_data: *mut ::core::ffi::c_void,
    ) -> *mut clk_hw;
}

#[repr(C)]
pub struct meson_clkc_data {
    pub init_regs: *const reg_sequence,
    pub init_count: ::core::ffi::c_uint,
    pub hw_clks: meson_clk_hw_data,
}

unsafe extern "C" {
    pub fn meson_clkc_syscon_probe(pdev: *mut platform_device) -> ::core::ffi::c_int;
    pub fn meson_clkc_mmio_probe(pdev: *mut platform_device) -> ::core::ffi::c_int;
}

// __MESON_PCLK(_name, _reg, _bit, _ops, _pdata, _flags)
#[macro_export]
macro_rules! __MESON_PCLK {
    ($name:ident, $reg:expr, $bit:expr, $ops:expr, $pdata:expr, $flags:expr) => {
        static mut $name: clk_regmap = clk_regmap {
            data: &clk_regmap_gate_data { offset: $reg, bit_idx: $bit },
            hw: clk_hw { init: &clk_init_data {
                name: stringify!($name),
                ops: $ops,
                parent_data: $pdata,
                num_parents: 1,
                flags: $flags,
            } },
        };
    };
}

#[macro_export]
macro_rules! MESON_PCLK {
    ($name:ident, $reg:expr, $bit:expr, $pdata:expr, $flags:expr) => {
        $crate::__MESON_PCLK!($name, $reg, $bit, &clk_regmap_gate_ops, $pdata, $flags);
    };
}

#[macro_export]
macro_rules! MESON_PCLK_RO {
    ($name:ident, $reg:expr, $bit:expr, $pdata:expr, $flags:expr) => {
        $crate::__MESON_PCLK!($name, $reg, $bit, &clk_regmap_gate_ro_ops, $pdata, $flags);
    };
}

/* Helpers for the usual sel/div/gate composite clocks. */
// The generated C identifiers are supplied explicitly because Rust macro_rules!
// has no stable identifier-concatenation operator.
#[macro_export]
macro_rules! MESON_COMP_SEL {
    ($var:ident, $prefix:ident, $name:ident, $reg:expr, $shift:expr, $mask:expr,
     $pdata:expr, $table:expr, $dflags:expr, $iflags:expr) => {
        static mut $var: clk_regmap = clk_regmap {
            data: &clk_regmap_mux_data {
                offset: $reg, mask: $mask, shift: $shift, flags: $dflags, table: $table,
            },
            hw: clk_hw { init: &clk_init_data {
                name: concat!(stringify!($name), "_sel"),
                ops: &clk_regmap_mux_ops,
                parent_data: $pdata,
                num_parents: $pdata.len(),
                flags: $iflags,
            } },
        };
    };
}

#[macro_export]
macro_rules! MESON_COMP_DIV {
    ($var:ident, $sel_hw:expr, $name:ident, $reg:expr, $shift:expr, $width:expr,
     $dflags:expr, $iflags:expr) => {
        static mut $var: clk_regmap = clk_regmap {
            data: &clk_regmap_div_data {
                offset: $reg, shift: $shift, width: $width, flags: $dflags,
            },
            hw: clk_hw { init: &clk_init_data {
                name: concat!(stringify!($name), "_div"),
                ops: &clk_regmap_divider_ops,
                parent_hws: &[$sel_hw],
                num_parents: 1,
                flags: $iflags,
            } },
        };
    };
}

#[macro_export]
macro_rules! MESON_COMP_GATE {
    ($var:ident, $div_hw:expr, $name:ident, $reg:expr, $bit:expr, $iflags:expr) => {
        static mut $var: clk_regmap = clk_regmap {
            data: &clk_regmap_gate_data { offset: $reg, bit_idx: $bit },
            hw: clk_hw { init: &clk_init_data {
                name: stringify!($name),
                ops: &clk_regmap_gate_ops,
                parent_hws: &[$div_hw],
                num_parents: 1,
                flags: $iflags,
            } },
        };
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
