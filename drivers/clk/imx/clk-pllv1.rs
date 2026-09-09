// SPDX-License-Identifier: GPL-2.0
// Dependencies supplied by the Linux clock framework and clk.h are intentionally external.

use core::ffi::{c_char, c_void};

const MFN_BITS: u32 = 10;
const MFN_SIGN: u32 = 1 << (MFN_BITS - 1);
const MFN_MASK: u32 = MFN_SIGN - 1;

#[repr(C)]
pub struct clk_pllv1 {
    pub hw: clk_hw,
    pub base: *mut c_void,
    pub r#type: imx_pllv1_type,
}

// External framework types and functions are provided by the surrounding repository.
#[repr(C)]
pub struct clk_hw {
    pub init: *const clk_init_data,
}

#[repr(C)]
pub struct clk_init_data {
    pub name: *const c_char,
    pub ops: *const clk_ops,
    pub flags: u32,
    pub parent_names: *const *const c_char,
    pub num_parents: u8,
}

#[repr(C)]
pub struct clk_ops {
    pub recalc_rate: Option<unsafe extern "C" fn(*mut clk_hw, usize) -> usize>,
}

#[allow(non_camel_case_types)]
pub type imx_pllv1_type = u32;
pub const IMX_PLLV1_IMX1: imx_pllv1_type = 0;
pub const IMX_PLLV1_IMX21: imx_pllv1_type = 1;
pub const IMX_PLLV1_IMX27: imx_pllv1_type = 2;

extern "C" {
    fn clk_hw_register(dev: *mut c_void, hw: *mut clk_hw) -> i32;
}

unsafe fn is_imx1_pllv1(pll: *mut clk_pllv1) -> bool {
    (*pll).r#type == IMX_PLLV1_IMX1
}

unsafe fn is_imx21_pllv1(pll: *mut clk_pllv1) -> bool {
    (*pll).r#type == IMX_PLLV1_IMX21
}

unsafe fn is_imx27_pllv1(pll: *mut clk_pllv1) -> bool {
    (*pll).r#type == IMX_PLLV1_IMX27
}

unsafe fn mfn_is_negative(pll: *mut clk_pllv1, mfn: u32) -> bool {
    !is_imx1_pllv1(pll) && !is_imx21_pllv1(pll) && (mfn & MFN_SIGN) != 0
}

unsafe extern "C" fn clk_pllv1_recalc_rate(hw: *mut clk_hw, parent_rate: usize) -> usize {
    // hw is the first member of clk_pllv1, matching container_of in the C source.
    let pll = hw as *mut clk_pllv1;
    let reg = core::ptr::read_volatile((*pll).base as *const u32);

    let mut mfi = (reg >> 10) & 0xf;
    let mfn = reg & 0x3ff;
    let mfd = (reg >> 16) & 0x3ff;
    let pd = (reg >> 26) & 0xf;

    mfi = if mfi <= 5 { 5 } else { mfi };
    let mut mfn_abs = mfn as usize;

    if mfn_is_negative(pll, mfn) {
        if is_imx27_pllv1(pll) {
            mfn_abs = (mfn & MFN_MASK) as usize;
        } else {
            mfn_abs = ((1 << MFN_BITS) - mfn) as usize;
        }
    }

    let rate = (parent_rate * 2) / (pd as usize + 1);
    let ull = (rate * mfn_abs) / (mfd as usize + 1);

    if mfn_is_negative(pll, mfn) {
        rate * mfi as usize - ull
    } else {
        rate * mfi as usize + ull
    }
}

static clk_pllv1_ops: clk_ops = clk_ops {
    recalc_rate: Some(clk_pllv1_recalc_rate),
};

pub unsafe extern "C" fn imx_clk_hw_pllv1(
    r#type: imx_pllv1_type,
    name: *const c_char,
    parent: *const c_char,
    base: *mut c_void,
) -> *mut clk_hw {
    let pll = Box::into_raw(Box::new(clk_pllv1 {
        hw: clk_hw { init: core::ptr::null() },
        base,
        r#type,
    }));

    let init = Box::new(clk_init_data {
        name,
        ops: &clk_pllv1_ops,
        flags: 0,
        parent_names: &parent,
        num_parents: 1,
    });
    (*pll).hw.init = Box::into_raw(init);

    let hw = &mut (*pll).hw as *mut clk_hw;
    let ret = clk_hw_register(core::ptr::null_mut(), hw);
    if ret != 0 {
        drop(Box::from_raw(pll));
        return ret as isize as *mut clk_hw;
    }

    hw
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
