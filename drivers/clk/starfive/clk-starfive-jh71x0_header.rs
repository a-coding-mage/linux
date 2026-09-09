/* SPDX-License-Identifier: GPL-2.0 */

// Register fields.  `BIT` and `GENMASK` correspond to the Linux bit helpers.
pub const JH71X0_CLK_ENABLE: u32 = 1u32 << 31;
pub const JH71X0_CLK_INVERT: u32 = 1u32 << 30;
pub const JH71X0_CLK_MUX_MASK: u32 = 0x0fu32 << 24;
pub const JH71X0_CLK_MUX_SHIFT: u32 = 24;
pub const JH71X0_CLK_DIV_MASK: u32 = 0x00ff_ffff;
pub const JH71X0_CLK_FRAC_MASK: u32 = 0xffu32 << 8;
pub const JH71X0_CLK_FRAC_SHIFT: u32 = 8;
pub const JH71X0_CLK_INT_MASK: u32 = 0xff;

// Fractional divider min/max.
pub const JH71X0_CLK_FRAC_MIN: u64 = 100;
pub const JH71X0_CLK_FRAC_MAX: u64 = 25599;

// Clock data.
#[repr(C)]
pub struct jh71x0_clk_data {
    pub name: *const core::ffi::c_char,
    pub flags: core::ffi::c_ulong,
    pub max: u32,
    pub parents: [u8; 4],
}

#[macro_export]
macro_rules! JH71X0_GATE {
    ($idx:expr, $name:expr, $flags:expr, $parent:expr) => {
        [$idx]: $crate::jh71x0_clk_data {
            name: $name,
            flags: CLK_SET_RATE_PARENT | ($flags),
            max: JH71X0_CLK_ENABLE,
            parents: [$parent, 0, 0, 0],
        }
    };
}

#[macro_export]
macro_rules! JH71X0__DIV {
    ($idx:expr, $name:expr, $max:expr, $parent:expr) => {
        [$idx]: $crate::jh71x0_clk_data {
            name: $name,
            flags: 0,
            max: $max,
            parents: [$parent, 0, 0, 0],
        }
    };
}

#[macro_export]
macro_rules! JH71X0_GDIV {
    ($idx:expr, $name:expr, $flags:expr, $max:expr, $parent:expr) => {
        [$idx]: $crate::jh71x0_clk_data {
            name: $name,
            flags: $flags,
            max: JH71X0_CLK_ENABLE | ($max),
            parents: [$parent, 0, 0, 0],
        }
    };
}

#[macro_export]
macro_rules! JH71X0_FDIV {
    ($idx:expr, $name:expr, $parent:expr) => {
        [$idx]: $crate::jh71x0_clk_data {
            name: $name,
            flags: 0,
            max: JH71X0_CLK_FRAC_MAX as u32,
            parents: [$parent, 0, 0, 0],
        }
    };
}

#[macro_export]
macro_rules! JH71X0__MUX {
    ($idx:expr, $name:expr, $flags:expr, $nparents:expr, $($parent:expr),* $(,)?) => {
        [$idx]: $crate::jh71x0_clk_data {
            name: $name,
            flags: $flags,
            max: (($nparents - 1) << JH71X0_CLK_MUX_SHIFT),
            parents: [$($parent),*],
        }
    };
}

#[macro_export]
macro_rules! JH71X0_GMUX {
    ($idx:expr, $name:expr, $flags:expr, $nparents:expr, $($parent:expr),* $(,)?) => {
        [$idx]: $crate::jh71x0_clk_data {
            name: $name,
            flags: $flags,
            max: JH71X0_CLK_ENABLE | (($nparents - 1) << JH71X0_CLK_MUX_SHIFT),
            parents: [$($parent),*],
        }
    };
}

#[macro_export]
macro_rules! JH71X0_MDIV {
    ($idx:expr, $name:expr, $max:expr, $nparents:expr, $($parent:expr),* $(,)?) => {
        [$idx]: $crate::jh71x0_clk_data {
            name: $name,
            flags: 0,
            max: (($nparents - 1) << JH71X0_CLK_MUX_SHIFT) | ($max),
            parents: [$($parent),*],
        }
    };
}

#[macro_export]
macro_rules! JH71X0__GMD {
    ($idx:expr, $name:expr, $flags:expr, $max:expr, $nparents:expr, $($parent:expr),* $(,)?) => {
        [$idx]: $crate::jh71x0_clk_data {
            name: $name,
            flags: $flags,
            max: JH71X0_CLK_ENABLE | (($nparents - 1) << JH71X0_CLK_MUX_SHIFT) | ($max),
            parents: [$($parent),*],
        }
    };
}

#[macro_export]
macro_rules! JH71X0__INV {
    ($idx:expr, $name:expr, $parent:expr) => {
        [$idx]: $crate::jh71x0_clk_data {
            name: $name,
            flags: CLK_SET_RATE_PARENT,
            max: JH71X0_CLK_INVERT,
            parents: [$parent, 0, 0, 0],
        }
    };
}

#[repr(C)]
pub struct jh71x0_clk {
    pub hw: clk_hw,
    pub idx: core::ffi::c_uint,
    pub max_div: core::ffi::c_uint,
}

#[repr(C)]
pub struct jh71x0_clk_priv {
    // Protect clk enable and set rate/parent from happening at the same time.
    pub rmw_lock: spinlock_t,
    pub dev: *mut device,
    pub base: *mut core::ffi::c_void,
    pub original_clk: *mut clk,
    pub pll_clk_nb: notifier_block,
    pub pll: [*mut clk_hw; 3],
    pub num_reg: core::ffi::c_uint,
    pub reg: [jh71x0_clk; 0], // flexible array member; allocated with num_reg entries
}

unsafe extern "C" {
    pub fn starfive_jh71x0_clk_ops(max: u32) -> *const clk_ops;
    pub fn jh71x0_clk_get(clkspec: *mut of_phandle_args, data: *mut core::ffi::c_void)
        -> *mut clk_hw;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
