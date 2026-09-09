// SPDX-License-Identifier: GPL-2.0
//

// Dependency equivalents supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_uint, c_void};

type u8_ = u8;
type u32_ = u32;

#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct clk_ops {
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> c_int>,
    pub determine_rate: Option<unsafe extern "C" fn(*mut clk_hw, *mut c_void) -> c_int>,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
    pub flags: c_uint,
}

#[repr(C)]
pub struct imx_clk_gpr {
    pub hw: clk_hw,
    pub regmap: *mut regmap,
    pub mask: u32,
    pub reg: u32,
    pub mux_table: *const u32,
}

extern "C" {
    fn regmap_read(map: *mut regmap, reg: u32, val: *mut c_uint) -> c_int;
    fn regmap_update_bits(map: *mut regmap, reg: u32, mask: u32, val: u32) -> c_int;
    fn clk_mux_val_to_index(hw: *mut clk_hw, table: *const u32, flags: c_uint, val: u32) -> c_int;
    fn clk_mux_index_to_val(table: *const u32, flags: c_uint, index: u8) -> u32;
    fn __clk_mux_determine_rate(hw: *mut clk_hw, req: *mut c_void) -> c_int;
    fn syscon_regmap_lookup_by_compatible(compatible: *const c_char) -> *mut regmap;
    fn clk_hw_register(dev: *mut c_void, hw: *mut clk_hw) -> c_int;
    fn kfree(ptr: *mut c_void);
    fn kzalloc(size: usize, flags: c_uint) -> *mut c_void;
    fn ERR_PTR(err: isize) -> *mut c_void;
    fn IS_ERR(ptr: *mut c_void) -> bool;
    fn ERR_CAST(ptr: *mut c_void) -> *mut clk_hw;
    fn clk_hw_get_name(hw: *mut clk_hw) -> *const c_char;
}

unsafe fn to_imx_clk_gpr(hw: *mut clk_hw) -> *mut imx_clk_gpr {
    // `hw` is the first member of imx_clk_gpr, as in container_of().
    hw as *mut imx_clk_gpr
}

unsafe extern "C" fn imx_clk_gpr_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let priv_ = to_imx_clk_gpr(hw);
    let mut val: c_uint = 0;
    let mut ret = regmap_read((*priv_).regmap, (*priv_).reg, &mut val);
    if ret != 0 {
        return 0;
    }

    val &= (*priv_).mask;
    ret = clk_mux_val_to_index(hw, (*priv_).mux_table, 0, val);
    if ret < 0 {
        return 0;
    }

    ret as u8
}

unsafe extern "C" fn imx_clk_gpr_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    let priv_ = to_imx_clk_gpr(hw);
    let val = clk_mux_index_to_val((*priv_).mux_table, 0, index);
    regmap_update_bits((*priv_).regmap, (*priv_).reg, (*priv_).mask, val)
}

static imx_clk_gpr_mux_ops: clk_ops = clk_ops {
    get_parent: Some(imx_clk_gpr_mux_get_parent),
    set_parent: Some(imx_clk_gpr_mux_set_parent),
    determine_rate: Some(__clk_mux_determine_rate),
};

#[no_mangle]
pub unsafe extern "C" fn imx_clk_gpr_mux(
    name: *const c_char,
    compatible: *const c_char,
    reg: u32,
    parent_names: *const *const c_char,
    num_parents: u8,
    mux_table: *const u32,
    mask: u32,
) -> *mut clk_hw {
    let mut init: clk_init_data = core::mem::zeroed();
    let mut regmap = syscon_regmap_lookup_by_compatible(compatible);
    if IS_ERR(regmap as *mut c_void) {
        return ERR_CAST(regmap as *mut c_void);
    }

    let priv_ = kzalloc(core::mem::size_of::<imx_clk_gpr>(), 0) as *mut imx_clk_gpr;
    if priv_.is_null() {
        return ERR_PTR(-(12isize)) as *mut clk_hw;
    }

    init.name = name;
    init.ops = &imx_clk_gpr_mux_ops;
    init.parent_names = parent_names;
    init.num_parents = num_parents;
    init.flags = (1u32 << 0) | (1u32 << 1);

    (*priv_).hw.init = &init;
    (*priv_).regmap = regmap;
    (*priv_).mux_table = mux_table;
    (*priv_).reg = reg;
    (*priv_).mask = mask;

    let hw = &mut (*priv_).hw as *mut clk_hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(priv_ as *mut c_void);
        return ERR_PTR(ret as isize) as *mut clk_hw;
    }
    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
