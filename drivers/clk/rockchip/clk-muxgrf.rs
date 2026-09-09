// SPDX-License-Identifier: GPL-2.0-only

// External kernel declarations supplied by the surrounding translation unit.
use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

#[repr(C)]
pub struct regmap { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub flags: c_ulong,
    pub num_parents: u8,
    pub parent_names: *const *const c_char,
    pub ops: *const clk_ops,
}

pub type c_ulong = usize;

#[repr(C)]
pub struct clk_ops {
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> c_int>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut c_void) -> c_int>,
}

extern "C" {
    fn malloc(size: usize) -> *mut c_void;
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut c_uint) -> c_int;
    fn regmap_write(map: *mut regmap, reg: u32, val: c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: c_uint, val: c_uint) -> c_int;
    fn __clk_mux_determine_rate(hw: *mut clk_hw, req: *mut c_void) -> c_int;
    fn clk_register(dev: *mut c_void, hw: *mut clk_hw) -> *mut clk;
    fn pr_err(fmt: *const c_char, ...);
    fn kfree(ptr: *mut c_void);
}

const CLK_MUX_HIWORD_MASK: c_int = 1 << 2;
const ENOTSUPP: c_int = 524;
const ENOMEM: c_int = 12;

unsafe fn is_err<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0
}

#[repr(C)]
pub struct rockchip_muxgrf_clock {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub reg: u32,
    pub shift: u32,
    pub width: u32,
    pub flags: c_int,
}

unsafe fn to_muxgrf_clock(hw: *mut clk_hw) -> *mut rockchip_muxgrf_clock {
    hw as *mut rockchip_muxgrf_clock
}

unsafe extern "C" fn rockchip_muxgrf_get_parent(hw: *mut clk_hw) -> u8 {
    let mux = &mut *to_muxgrf_clock(hw);
    let mask: u32 = if mux.width == 0 { 0 } else { (1u32 << mux.width) - 1 };
    let mut val: c_uint = 0;

    regmap_read(mux.regmap, mux.reg, &mut val);

    val >>= mux.shift;
    val &= mask;

    val as u8
}

unsafe extern "C" fn rockchip_muxgrf_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    let mux = &mut *to_muxgrf_clock(hw);
    let mask: u32 = if mux.width + mux.shift == 0 {
        0
    } else {
        (((1u64 << (mux.width + mux.shift)) - 1) & !((1u64 << mux.shift) - 1)) as u32
    };
    let mut val = index as u32;

    val <<= mux.shift;

    if mux.flags & CLK_MUX_HIWORD_MASK != 0 {
        regmap_write(mux.regmap, mux.reg, val | (mask << 16))
    } else {
        regmap_update_bits(mux.regmap, mux.reg, mask, val)
    }
}

static rockchip_muxgrf_clk_ops: clk_ops = clk_ops {
    get_parent: Some(rockchip_muxgrf_get_parent),
    set_parent: Some(rockchip_muxgrf_set_parent),
    determine_rate: Some(__clk_mux_determine_rate),
};

pub unsafe extern "C" fn rockchip_clk_register_muxgrf(
    name: *const c_char,
    parent_names: *const *const c_char,
    num_parents: u8,
    flags: c_int,
    regmap: *mut regmap,
    reg: c_int,
    shift: c_int,
    width: c_int,
    mux_flags: c_int,
) -> *mut clk {
    if is_err(regmap) {
        // pr_err("%s: regmap not available\n", __func__);
        return (-ENOTSUPP as isize) as *mut clk;
    }

    let muxgrf_clock: *mut rockchip_muxgrf_clock =
        malloc(size_of::<rockchip_muxgrf_clock>()) as *mut rockchip_muxgrf_clock;
    if muxgrf_clock.is_null() {
        return (-ENOMEM as isize) as *mut clk;
    }

    let init = clk_init_data {
        name,
        flags: flags as c_ulong,
        num_parents,
        parent_names,
        ops: &rockchip_muxgrf_clk_ops,
    };

    (*muxgrf_clock).hw.init = malloc(size_of::<clk_init_data>()) as *const clk_init_data;
    ptr::write((*muxgrf_clock).hw.init as *mut clk_init_data, init);
    (*muxgrf_clock).regmap = regmap;
    (*muxgrf_clock).reg = reg as u32;
    (*muxgrf_clock).shift = shift as u32;
    (*muxgrf_clock).width = width as u32;
    (*muxgrf_clock).flags = mux_flags;

    let clk = clk_register(ptr::null_mut(), &mut (*muxgrf_clock).hw);
    if (clk as isize) < 0 {
        kfree(muxgrf_clock as *mut c_void);
    }

    clk
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
