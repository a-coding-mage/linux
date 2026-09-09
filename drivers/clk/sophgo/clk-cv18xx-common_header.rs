/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Copyright (C) 2023 Inochi Amaoto <inochiama@outlook.com>
 */

// C dependencies: linux/compiler.h, linux/clk-provider.h, linux/bitfield.h

#[repr(C)]
pub struct cv1800_clk_common {
    pub base: *mut core::ffi::c_void,
    pub lock: *mut spinlock_t,
    pub hw: clk_hw,
    pub features: c_ulong,
}

// Corresponds to CV1800_CLK_COMMON(_name, _parents, _op, _flags).
#[macro_export]
macro_rules! CV1800_CLK_COMMON {
    ($name:expr, $parents:expr, $op:expr, $flags:expr) => {
        cv1800_clk_common {
            hw: CLK_HW_INIT_PARENTS_DATA!($name, $parents, $op, $flags),
            ..unsafe { core::mem::zeroed() }
        }
    };
}

pub unsafe fn hw_to_cv1800_clk_common(hw: *mut clk_hw) -> *mut cv1800_clk_common {
    // Equivalent to container_of(hw, struct cv1800_clk_common, hw).
    (hw as *mut u8).sub(core::mem::offset_of!(cv1800_clk_common, hw))
        as *mut cv1800_clk_common
}

#[repr(C)]
pub struct cv1800_clk_regbit {
    pub reg: u16,
    pub shift: i8,
}

#[repr(C)]
pub struct cv1800_clk_regfield {
    pub reg: u16,
    pub shift: u8,
    pub width: u8,
    pub initval: i16,
    pub flags: c_ulong,
}

#[macro_export]
macro_rules! CV1800_CLK_BIT {
    ($reg:expr, $shift:expr) => {
        cv1800_clk_regbit { reg: $reg, shift: $shift }
    };
}

#[macro_export]
macro_rules! CV1800_CLK_REG {
    ($reg:expr, $shift:expr, $width:expr, $initval:expr, $flags:expr) => {
        cv1800_clk_regfield {
            reg: $reg,
            shift: $shift,
            width: $width,
            initval: $initval,
            flags: $flags,
        }
    };
}

#[inline]
pub fn cv1800_clk_regfield_genmask(reg: &cv1800_clk_regfield) -> c_ulong {
    genmask(reg.shift + reg.width - 1, reg.shift)
}

#[inline]
pub fn cv1800_clk_regfield_get(val: c_ulong, reg: &cv1800_clk_regfield) -> c_ulong {
    (val >> reg.shift) & genmask(reg.width - 1, 0)
}

#[inline]
pub fn cv1800_clk_regfield_set(
    val: c_ulong,
    new_value: c_ulong,
    reg: &cv1800_clk_regfield,
) -> c_ulong {
    (val & !cv1800_clk_regfield_genmask(reg))
        | ((new_value & genmask(reg.width - 1, 0)) << reg.shift)
}

#[inline]
pub fn _cv1800_set_field(reg: c_ulong, val: c_ulong, field: c_ulong) -> c_ulong {
    (reg & !field) | field_prep(field, val)
}

extern "C" {
    pub fn cv1800_clk_setbit(
        common: *mut cv1800_clk_common,
        field: *mut cv1800_clk_regbit,
    ) -> i32;
    pub fn cv1800_clk_clearbit(
        common: *mut cv1800_clk_common,
        field: *mut cv1800_clk_regbit,
    ) -> i32;
    pub fn cv1800_clk_checkbit(
        common: *mut cv1800_clk_common,
        field: *mut cv1800_clk_regbit,
    ) -> i32;
    pub fn cv1800_clk_wait_for_lock(common: *mut cv1800_clk_common, reg: u32, lock: u32);
}

// External Linux/kernel types and macros referenced above are supplied by dependencies.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
