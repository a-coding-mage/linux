/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2020-2023 Intel Corporation
 */

// Translated from ivpu_hw_reg_io.h. Linux headers and ivpu_drv.h are external
// dependencies of this declaration-level translation.

pub const REG_POLL_SLEEP_US: u32 = 50;
pub const REG_IO_ERROR: u32 = 0xffff_ffff;

#[macro_export]
macro_rules! REGB_RD32 { ($reg:expr) => { ivpu_hw_reg_rd32(vdev, vdev.regb, $reg, stringify!($reg), module_path!()) }; }
#[macro_export]
macro_rules! REGB_RD64 { ($reg:expr) => { ivpu_hw_reg_rd64(vdev, vdev.regb, $reg, stringify!($reg), module_path!()) }; }
#[macro_export]
macro_rules! REGB_WR32 { ($reg:expr, $val:expr) => { ivpu_hw_reg_wr32(vdev, vdev.regb, $reg, $val, stringify!($reg), module_path!()) }; }
#[macro_export]
macro_rules! REGB_WR64 { ($reg:expr, $val:expr) => { ivpu_hw_reg_wr64(vdev, vdev.regb, $reg, $val, stringify!($reg), module_path!()) }; }

#[macro_export]
macro_rules! REGV_RD32 { ($reg:expr) => { ivpu_hw_reg_rd32(vdev, vdev.regv, $reg, stringify!($reg), module_path!()) }; }
#[macro_export]
macro_rules! REGV_RD64 { ($reg:expr) => { ivpu_hw_reg_rd64(vdev, vdev.regv, $reg, stringify!($reg), module_path!()) }; }
#[macro_export]
macro_rules! REGV_WR32 { ($reg:expr, $val:expr) => { ivpu_hw_reg_wr32(vdev, vdev.regv, $reg, $val, stringify!($reg), module_path!()) }; }
#[macro_export]
macro_rules! REGV_WR64 { ($reg:expr, $val:expr) => { ivpu_hw_reg_wr64(vdev, vdev.regv, $reg, $val, stringify!($reg), module_path!()) }; }
#[macro_export]
macro_rules! REGV_WR32I { ($reg:expr, $stride:expr, $index:expr, $val:expr) => { ivpu_hw_reg_wr32_index(vdev, vdev.regv, $reg, $stride, $index, $val, stringify!($reg), module_path!()) }; }

#[macro_export]
macro_rules! REG_FLD { ($reg:ident, $fld:ident) => { $reg::$fld##_MASK }; }
#[macro_export]
macro_rules! REG_FLD_NUM { ($reg:ident, $fld:ident, $num:expr) => { FIELD_PREP($reg::$fld##_MASK, $num) }; }
#[macro_export]
macro_rules! REG_GET_FLD { ($reg:ident, $fld:ident, $val:expr) => { FIELD_GET($reg::$fld##_MASK, $val) }; }
#[macro_export]
macro_rules! REG_CLR_FLD { ($reg:ident, $fld:ident, $val:expr) => { ($val) & !($reg::$fld##_MASK) }; }
#[macro_export]
macro_rules! REG_SET_FLD { ($reg:ident, $fld:ident, $val:expr) => { ($val) | ($reg::$fld##_MASK) }; }
#[macro_export]
macro_rules! REG_SET_FLD_NUM { ($reg:ident, $fld:ident, $num:expr, $val:expr) => { (($val) & !($reg::$fld##_MASK)) | FIELD_PREP($reg::$fld##_MASK, $num) }; }
#[macro_export]
macro_rules! REG_TEST_FLD { ($reg:ident, $fld:ident, $val:expr) => { ($reg::$fld##_MASK) == (($val) & ($reg::$fld##_MASK)) }; }
#[macro_export]
macro_rules! REG_TEST_FLD_NUM { ($reg:ident, $fld:ident, $num:expr, $val:expr) => { ($num) == FIELD_GET($reg::$fld##_MASK, $val) }; }

#[macro_export]
macro_rules! REGB_POLL_FLD { ($reg:ident, $fld:ident, $exp:expr, $timeout:expr) => { ivpu_hw_reg_poll_fld(vdev, vdev.regb, $reg, $reg::$fld##_MASK, FIELD_PREP($reg::$fld##_MASK, $exp), $timeout, module_path!(), stringify!($reg), stringify!($fld)) }; }
#[macro_export]
macro_rules! REGV_POLL_FLD { ($reg:ident, $fld:ident, $exp:expr, $timeout:expr) => { ivpu_hw_reg_poll_fld(vdev, vdev.regv, $reg, $reg::$fld##_MASK, FIELD_PREP($reg::$fld##_MASK, $exp), $timeout, module_path!(), stringify!($reg), stringify!($fld)) }; }

extern "C" {
    pub static mut ivpu_hw_failure: fault_attr;
}

#[repr(C)]
pub struct fault_attr {
    _private: [u8; 0],
}

#[inline]
pub unsafe fn ivpu_hw_reg_poll_fld(
    vdev: *mut ivpu_device, base: *mut core::ffi::c_void,
    reg_offset: u32, reg_mask: u32, exp_masked_val: u32, timeout_us: u32,
    func_name: *const core::ffi::c_char, reg_name: *const core::ffi::c_char,
    fld_name: *const core::ffi::c_char,
) -> i32 {
    let mut reg_val: u32 = 0;
    let mut ret: i32;
    ivpu_dbg!(vdev, REG, "%s : %s (0x%08x) POLL %s started (exp_val 0x%x)\n", func_name, reg_name, reg_offset, fld_name, exp_masked_val);
    ret = read_poll_timeout!(readl, reg_val, (reg_val & reg_mask) == exp_masked_val, REG_POLL_SLEEP_US, timeout_us, false, (base as *mut u8).add(reg_offset as usize));
    // CONFIG_FAULT_INJECTION is a build-time condition from the C source.
    #[cfg(CONFIG_FAULT_INJECTION)]
    if should_fail(&mut ivpu_hw_failure, 1) { ret = -ETIMEDOUT; }
    ivpu_dbg!(vdev, REG, "%s : %s (0x%08x) POLL %s %s (reg_val 0x%08x)\n", func_name, reg_name, reg_offset, fld_name, if ret != 0 { "ETIMEDOUT" } else { "OK" }, reg_val);
    ret
}

#[inline]
pub unsafe fn ivpu_hw_reg_rd32(vdev: *mut ivpu_device, base: *mut core::ffi::c_void, reg: u32, name: *const core::ffi::c_char, func: *const core::ffi::c_char) -> u32 {
    let val = readl!((base as *mut u8).add(reg as usize));
    ivpu_dbg!(vdev, REG, "%s : %s (0x%08x) RD: 0x%08x\n", func, name, reg, val); val
}

#[inline]
pub unsafe fn ivpu_hw_reg_rd64(vdev: *mut ivpu_device, base: *mut core::ffi::c_void, reg: u32, name: *const core::ffi::c_char, func: *const core::ffi::c_char) -> u64 {
    let val = readq!((base as *mut u8).add(reg as usize));
    ivpu_dbg!(vdev, REG, "%s : %s (0x%08x) RD: 0x%016llx\n", func, name, reg, val); val
}

#[inline]
pub unsafe fn ivpu_hw_reg_wr32(vdev: *mut ivpu_device, base: *mut core::ffi::c_void, reg: u32, val: u32, name: *const core::ffi::c_char, func: *const core::ffi::c_char) {
    ivpu_dbg!(vdev, REG, "%s : %s (0x%08x) WR: 0x%08x\n", func, name, reg, val); writel!(val, (base as *mut u8).add(reg as usize));
}

#[inline]
pub unsafe fn ivpu_hw_reg_wr64(vdev: *mut ivpu_device, base: *mut core::ffi::c_void, reg: u32, val: u64, name: *const core::ffi::c_char, func: *const core::ffi::c_char) {
    ivpu_dbg!(vdev, REG, "%s : %s (0x%08x) WR: 0x%016llx\n", func, name, reg, val); writeq!(val, (base as *mut u8).add(reg as usize));
}

#[inline]
pub unsafe fn ivpu_hw_reg_wr32_index(vdev: *mut ivpu_device, base: *mut core::ffi::c_void, mut reg: u32, stride: u32, index: u32, val: u32, name: *const core::ffi::c_char, func: *const core::ffi::c_char) {
    reg = reg.wrapping_add(index.wrapping_mul(stride));
    ivpu_dbg!(vdev, REG, "%s WR: %s_%d (0x%08x) <= 0x%08x\n", func, name, index, reg, val); writel!(val, (base as *mut u8).add(reg as usize));
}

// External types/macros supplied by ivpu_drv.h and Linux headers.
pub enum ivpu_device {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
