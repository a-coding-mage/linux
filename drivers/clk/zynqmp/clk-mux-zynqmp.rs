// SPDX-License-Identifier: GPL-2.0
/*
 * Zynq UltraScale+ MPSoC mux
 *
 *  Copyright (C) 2016-2018 Xilinx
 */

/*
 * DOC: basic adjustable multiplexer clock that cannot gate
 *
 * Traits of this clock:
 * prepare - clk_prepare only ensures that parents are prepared
 * enable - clk_enable only ensures that parents are enabled
 * rate - rate is only affected by parent switching.  No clk_set_rate support
 * parent - parent is adjustable through clk_set_parent
 */

// External kernel and ZynqMP declarations supplied by other translation units.
#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const core::ffi::c_char,
    pub ops: *const clk_ops,
    pub flags: c_ulong,
    pub parent_names: *const *const core::ffi::c_char,
    pub num_parents: u8,
}

#[repr(C)]
pub struct clk_ops {
    pub get_parent: Option<unsafe extern "C" fn(*mut clk_hw) -> u8>,
    pub set_parent: Option<unsafe extern "C" fn(*mut clk_hw, u8) -> c_int>,
    pub determine_rate: Option<unsafe extern "C" fn() -> c_int>,
}

#[repr(C)]
pub struct clock_topology {
    pub type_flag: u32,
    pub flag: u32,
}

type c_int = i32;
type c_ulong = usize;

extern "C" {
    fn zynqmp_pm_clock_getparent(clk_id: u32, val: *mut u32) -> c_int;
    fn zynqmp_pm_clock_setparent(clk_id: u32, index: u8) -> c_int;
    fn clk_hw_get_name(hw: *mut clk_hw) -> *const core::ffi::c_char;
    fn clk_hw_get_num_parents(hw: *mut clk_hw) -> u8;
    fn clk_hw_register(dev: *mut core::ffi::c_void, hw: *mut clk_hw) -> c_int;
    fn kzalloc(size: usize, flags: usize) -> *mut core::ffi::c_void;
    fn kfree(ptr: *mut core::ffi::c_void);
    fn __clk_mux_determine_rate_closest() -> c_int;
    fn zynqmp_clk_map_common_ccf_flags(flag: u32) -> c_ulong;
}

const ENOMEM: c_int = 12;
const ZYNQMP_CLK_MUX_INDEX_ONE: u32 = 1 << 0;
const ZYNQMP_CLK_MUX_INDEX_BIT: u32 = 1 << 1;
const ZYNQMP_CLK_MUX_HIWORD_MASK: u32 = 1 << 2;
const ZYNQMP_CLK_MUX_READ_ONLY: u32 = 1 << 3;
const ZYNQMP_CLK_MUX_ROUND_CLOSEST: u32 = 1 << 4;
const ZYNQMP_CLK_MUX_BIG_ENDIAN: u32 = 1 << 5;
const CLK_MUX_INDEX_ONE: c_ulong = 1 << 0;
const CLK_MUX_INDEX_BIT: c_ulong = 1 << 1;
const CLK_MUX_HIWORD_MASK: c_ulong = 1 << 2;
const CLK_MUX_READ_ONLY: c_ulong = 1 << 3;
const CLK_MUX_ROUND_CLOSEST: c_ulong = 1 << 4;
const CLK_MUX_BIG_ENDIAN: c_ulong = 1 << 5;

/* struct zynqmp_clk_mux - multiplexer clock */
#[repr(C)]
pub struct zynqmp_clk_mux {
    pub hw: clk_hw,
    pub flags: u8,
    pub clk_id: u32,
}

unsafe fn to_zynqmp_clk_mux(hw: *mut clk_hw) -> *mut zynqmp_clk_mux {
    hw as *mut zynqmp_clk_mux
}

unsafe extern "C" fn zynqmp_clk_mux_get_parent(hw: *mut clk_hw) -> u8 {
    let mux = to_zynqmp_clk_mux(hw);
    let clk_name = clk_hw_get_name(hw);
    let clk_id = (*mux).clk_id;
    let mut val: u32 = 0;
    let ret = zynqmp_pm_clock_getparent(clk_id, &mut val);

    if ret != 0 {
        // pr_debug("%s() getparent failed for clock: %s, ret = %d\n", __func__, clk_name, ret);
        // clk_core_get_parent_by_index() treats num_parents as an invalid index.
        return clk_hw_get_num_parents(hw);
    }

    val as u8
}

unsafe extern "C" fn zynqmp_clk_mux_set_parent(hw: *mut clk_hw, index: u8) -> c_int {
    let mux = to_zynqmp_clk_mux(hw);
    let _clk_name = clk_hw_get_name(hw);
    let clk_id = (*mux).clk_id;
    let ret = zynqmp_pm_clock_setparent(clk_id, index);

    if ret != 0 {
        // pr_debug("%s() set parent failed for clock: %s, ret = %d\n", __func__, clk_name, ret);
    }

    ret
}

static ZYNQMP_CLK_MUX_OPS: clk_ops = clk_ops {
    get_parent: Some(zynqmp_clk_mux_get_parent),
    set_parent: Some(zynqmp_clk_mux_set_parent),
    determine_rate: Some(__clk_mux_determine_rate_closest),
};

static ZYNQMP_CLK_MUX_RO_OPS: clk_ops = clk_ops {
    get_parent: Some(zynqmp_clk_mux_get_parent),
    set_parent: None,
    determine_rate: None,
};

unsafe fn zynqmp_clk_map_mux_ccf_flags(zynqmp_type_flag: u32) -> c_ulong {
    let mut ccf_flag: c_ulong = 0;
    if zynqmp_type_flag & ZYNQMP_CLK_MUX_INDEX_ONE != 0 { ccf_flag |= CLK_MUX_INDEX_ONE; }
    if zynqmp_type_flag & ZYNQMP_CLK_MUX_INDEX_BIT != 0 { ccf_flag |= CLK_MUX_INDEX_BIT; }
    if zynqmp_type_flag & ZYNQMP_CLK_MUX_HIWORD_MASK != 0 { ccf_flag |= CLK_MUX_HIWORD_MASK; }
    if zynqmp_type_flag & ZYNQMP_CLK_MUX_READ_ONLY != 0 { ccf_flag |= CLK_MUX_READ_ONLY; }
    if zynqmp_type_flag & ZYNQMP_CLK_MUX_ROUND_CLOSEST != 0 { ccf_flag |= CLK_MUX_ROUND_CLOSEST; }
    if zynqmp_type_flag & ZYNQMP_CLK_MUX_BIG_ENDIAN != 0 { ccf_flag |= CLK_MUX_BIG_ENDIAN; }
    ccf_flag
}

pub unsafe fn zynqmp_clk_register_mux(
    name: *const core::ffi::c_char,
    clk_id: u32,
    parents: *const *const core::ffi::c_char,
    num_parents: u8,
    nodes: *const clock_topology,
) -> *mut clk_hw {
    let mux = kzalloc(core::mem::size_of::<zynqmp_clk_mux>(), 0) as *mut zynqmp_clk_mux;
    if mux.is_null() {
        return (-ENOMEM as isize) as *mut clk_hw;
    }

    let ops = if (*nodes).type_flag & CLK_MUX_READ_ONLY != 0 {
        &ZYNQMP_CLK_MUX_RO_OPS
    } else {
        &ZYNQMP_CLK_MUX_OPS
    };
    let init = clk_init_data {
        name,
        ops,
        flags: zynqmp_clk_map_common_ccf_flags((*nodes).flag),
        parent_names: parents,
        num_parents,
    };
    (*mux).flags = zynqmp_clk_map_mux_ccf_flags((*nodes).type_flag) as u8;
    (*mux).hw.init = &init;
    (*mux).clk_id = clk_id;

    let hw = &mut (*mux).hw as *mut clk_hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        kfree(mux as *mut core::ffi::c_void);
        return (ret as isize) as *mut clk_hw;
    }
    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
