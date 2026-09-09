/*
 * Copyright 2016 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C preprocessor token-pasting (ip##_HWIP, reg##_BASE_IDX, and prefix##...) is
// represented by explicit macro arguments in Rust; the referenced symbols are
// supplied by the surrounding translation unit.

macro_rules! GET_INST { ($adev:expr, $ip:expr, $inst:expr) => {{
    if $adev.ip_map.logical_to_dev_inst.is_some() { $adev.ip_map.logical_to_dev_inst($adev, $ip, $inst) } else { $inst }
}}; }
macro_rules! GET_MASK { ($adev:expr, $ip:expr, $mask:expr) => {{
    if $adev.ip_map.logical_to_dev_mask.is_some() { $adev.ip_map.logical_to_dev_mask($adev, $ip, $mask) } else { $mask }
}}; }

macro_rules! SOC15_REG_OFFSET { ($adev:expr, $ip:expr, $inst:expr, $base_idx:expr, $reg:expr) => {
    $adev.reg_offset[$ip][$inst][$base_idx] + $reg
} }
macro_rules! SOC15_REG_OFFSET1 { ($adev:expr, $ip:expr, $inst:expr, $base_idx:expr, $reg:expr, $offset:expr) => {
    $adev.reg_offset[$ip][$inst][$base_idx] + $reg + $offset
} }

macro_rules! __WREG32_SOC15_RLC__ { ($adev:expr, $reg:expr, $value:expr, $flag:expr, $hwip:expr, $inst:expr) => {
    ($adev.gfx.rlc.reg_funcs.wreg32)($adev, $reg, $value, $flag, $hwip, $inst)
} }
macro_rules! __RREG32_SOC15_RLC__ { ($adev:expr, $reg:expr, $flag:expr, $hwip:expr, $inst:expr) => {
    ($adev.gfx.rlc.reg_funcs.rreg32)($adev, $reg, $flag, $hwip, $inst)
} }

macro_rules! RREG32_SOC15 { ($adev:expr, $ip:expr, $inst:expr, $base_idx:expr, $reg:expr, $hwip:expr) => {
    __RREG32_SOC15_RLC!($adev, $adev.reg_offset[$hwip][$inst][$base_idx] + $reg, 0, $hwip, $inst)
} }
macro_rules! RREG32_SOC15_IP { ($adev:expr, $reg:expr, $hwip:expr) => { __RREG32_SOC15_RLC!($adev, $reg, 0, $hwip, 0) } }
macro_rules! RREG32_SOC15_IP_NO_KIQ { ($adev:expr, $reg:expr, $inst:expr, $hwip:expr) => { __RREG32_SOC15_RLC!($adev, $reg, AMDGPU_REGS_NO_KIQ, $hwip, $inst) } }
macro_rules! RREG32_SOC15_NO_KIQ { ($adev:expr, $hwip:expr, $inst:expr, $base_idx:expr, $reg:expr) => { __RREG32_SOC15_RLC!($adev, $adev.reg_offset[$hwip][$inst][$base_idx] + $reg, AMDGPU_REGS_NO_KIQ, $hwip, $inst) } }
macro_rules! RREG32_SOC15_OFFSET { ($adev:expr, $hwip:expr, $inst:expr, $base_idx:expr, $reg:expr, $offset:expr) => { __RREG32_SOC15_RLC!($adev, $adev.reg_offset[$hwip][$inst][$base_idx] + $reg + $offset, 0, $hwip, $inst) } }

macro_rules! WREG32_SOC15 { ($adev:expr, $hwip:expr, $inst:expr, $base_idx:expr, $reg:expr, $value:expr) => { __WREG32_SOC15_RLC!($adev, $adev.reg_offset[$hwip][$inst][$base_idx] + $reg, $value, 0, $hwip, $inst) } }
macro_rules! WREG32_SOC15_IP { ($adev:expr, $hwip:expr, $reg:expr, $value:expr) => { __WREG32_SOC15_RLC!($adev, $reg, $value, 0, $hwip, 0) } }
macro_rules! WREG32_SOC15_IP_NO_KIQ { ($adev:expr, $hwip:expr, $reg:expr, $value:expr, $inst:expr) => { __WREG32_SOC15_RLC!($adev, $reg, $value, AMDGPU_REGS_NO_KIQ, $hwip, $inst) } }
macro_rules! WREG32_SOC15_NO_KIQ { ($adev:expr, $hwip:expr, $inst:expr, $base_idx:expr, $reg:expr, $value:expr) => { __WREG32_SOC15_RLC!($adev, $adev.reg_offset[$hwip][$inst][$base_idx] + $reg, $value, AMDGPU_REGS_NO_KIQ, $hwip, $inst) } }
macro_rules! WREG32_SOC15_OFFSET { ($adev:expr, $hwip:expr, $inst:expr, $base_idx:expr, $reg:expr, $offset:expr, $value:expr) => { __WREG32_SOC15_RLC!($adev, $adev.reg_offset[$hwip][$inst][$base_idx] + $reg + $offset, $value, 0, $hwip, $inst) } }

macro_rules! SOC15_WAIT_ON_RREG { ($adev:expr, $ip:expr, $inst:expr, $base_idx:expr, $reg:expr, $expected:expr, $mask:expr) => {
    amdgpu_device_wait_on_rreg($adev, $inst, $adev.reg_offset[$ip][$inst][$base_idx] + $reg, stringify!($reg), $expected, $mask)
} }
macro_rules! SOC15_WAIT_ON_RREG_OFFSET { ($adev:expr, $ip:expr, $inst:expr, $base_idx:expr, $reg:expr, $offset:expr, $expected:expr, $mask:expr) => {
    amdgpu_device_wait_on_rreg($adev, $inst, $adev.reg_offset[$ip][$inst][$base_idx] + $reg + $offset, stringify!($reg), $expected, $mask)
} }

macro_rules! WREG32_RLC { ($adev:expr, $reg:expr, $value:expr) => { __WREG32_SOC15_RLC!($adev, $reg, $value, AMDGPU_REGS_RLC, GC_HWIP, 0) } }
macro_rules! RREG32_RLC { ($adev:expr, $reg:expr) => { __RREG32_SOC15_RLC!($adev, $reg, AMDGPU_REGS_RLC, GC_HWIP, 0) } }
macro_rules! WREG32_RLC_NO_KIQ { ($adev:expr, $reg:expr, $value:expr, $hwip:expr) => { __WREG32_SOC15_RLC!($adev, $reg, $value, AMDGPU_REGS_NO_KIQ | AMDGPU_REGS_RLC, $hwip, 0) } }
macro_rules! RREG32_RLC_NO_KIQ { ($adev:expr, $reg:expr, $hwip:expr) => { __RREG32_SOC15_RLC!($adev, $reg, AMDGPU_REGS_NO_KIQ | AMDGPU_REGS_RLC, $hwip, 0) } }

macro_rules! RREG32_SOC15_RLC { ($adev:expr, $ip:expr, $inst:expr, $base_idx:expr, $reg:expr) => { __RREG32_SOC15_RLC!($adev, $adev.reg_offset[$ip][$inst][$base_idx] + $reg, AMDGPU_REGS_RLC, $ip, $inst) } }
macro_rules! WREG32_SOC15_RLC { ($adev:expr, $ip:expr, $inst:expr, $base_idx:expr, $reg:expr, $value:expr) => { __WREG32_SOC15_RLC!($adev, $adev.reg_offset[$ip][$inst][$base_idx] + $reg, $value, AMDGPU_REGS_RLC, $ip, $inst) } }
macro_rules! WREG32_SOC15_OFFSET_RLC { ($adev:expr, $ip:expr, $inst:expr, $base_idx:expr, $reg:expr, $offset:expr, $value:expr) => { __WREG32_SOC15_RLC!($adev, $adev.reg_offset[$ip][$inst][$base_idx] + $reg + $offset, $value, AMDGPU_REGS_RLC, $ip, $inst) } }
macro_rules! RREG32_SOC15_OFFSET_RLC { ($adev:expr, $ip:expr, $inst:expr, $base_idx:expr, $reg:expr, $offset:expr) => { __RREG32_SOC15_RLC!($adev, $adev.reg_offset[$ip][$inst][$base_idx] + $reg + $offset, AMDGPU_REGS_RLC, $ip, $inst) } }

macro_rules! RREG32_SOC15_EXT { ($adev:expr, $ip:expr, $inst:expr, $base_idx:expr, $reg:expr) => {
    RREG32_PCIE_EXT((($adev.reg_offset[$ip][$inst][$base_idx] + $reg) * 4) + amdgpu_reg_get_smn_base64($adev, $ip, $inst))
} }
macro_rules! WREG32_SOC15_EXT { ($adev:expr, $ip:expr, $inst:expr, $base_idx:expr, $reg:expr, $value:expr) => {
    WREG32_PCIE_EXT((($adev.reg_offset[$ip][$inst][$base_idx] + $reg) * 4) + amdgpu_reg_get_smn_base64($adev, $ip, $inst), $value)
} }

macro_rules! WREG32_FIELD15 { ($adev:expr, $ip:expr, $idx:expr, $base_idx:expr, $reg:expr, $field:expr, $val:expr, $hwip:expr) => {{
    let reg__ = $adev.reg_offset[$hwip][$idx][$base_idx] + $reg;
    let mut val__ = __RREG32_SOC15_RLC!($adev, reg__, 0, $hwip, $idx);
    val__ &= !REG_FIELD_MASK($reg, $field);
    val__ |= ($val) << REG_FIELD_SHIFT($reg, $field);
    __WREG32_SOC15_RLC!($adev, reg__, val__, 0, $hwip, $idx);
}}; }
macro_rules! WREG32_FIELD15_PREREG { ($adev:expr, $ip:expr, $idx:expr, $base_idx:expr, $reg:expr, $field:expr, $val:expr, $hwip:expr) => {
    WREG32_FIELD15!($adev, $ip, $idx, $base_idx, $reg, $field, $val, $hwip)
} }
macro_rules! WREG32_FIELD15_RLC { ($adev:expr, $ip:expr, $idx:expr, $base_idx:expr, $reg:expr, $field:expr, $val:expr, $hwip:expr) => {{
    let reg__ = $adev.reg_offset[$hwip][$idx][$base_idx] + $reg;
    let mut val__ = __RREG32_SOC15_RLC!($adev, reg__, AMDGPU_REGS_RLC, $hwip, $idx);
    val__ &= !REG_FIELD_MASK($reg, $field);
    val__ |= ($val) << REG_FIELD_SHIFT($reg, $field);
    __WREG32_SOC15_RLC!($adev, reg__, val__, AMDGPU_REGS_RLC, $hwip, $idx);
}}; }

macro_rules! WREG32_SOC15_RLC_SHADOW { ($adev:expr, $ip:expr, $inst:expr, $base_idx:expr, $reg:expr, $value:expr, $hwip:expr) => {
    __WREG32_SOC15_RLC!($adev, $adev.reg_offset[$hwip][$inst][$base_idx] + $reg, $value, AMDGPU_REGS_RLC, GC_HWIP, $inst)
} }
macro_rules! WREG32_SOC15_RLC_EX { ($adev:expr, $ip:expr, $inst:expr, $base_idx:expr, $reg:expr, $value:expr, $hwip:expr, $prefix:expr) => {
    WREG32_RLC_EX!($adev, $adev.reg_offset[GC_HWIP][$inst][$base_idx] + $reg, $value, $inst, $prefix)
} }
macro_rules! WREG32_RLC_EX { ($adev:expr, $reg:expr, $value:expr, $inst:expr, $prefix:expr) => {{
    if amdgpu_sriov_fullaccess($adev) {
        let mut i: u32 = 0;
        let retries: u32 = 50000;
        WREG32($adev, $prefix.scratch_reg0, $value);
        WREG32($adev, $prefix.scratch_reg1, $reg | 0x80000000);
        WREG32($adev, $prefix.rlc_spare_int, 0x1);
        while i < retries {
            let tmp = RREG32($adev, $prefix.scratch_reg1);
            if tmp & 0x80000000 == 0 { break; }
            udelay(10);
            i += 1;
        }
        if i >= retries { pr_err!("timeout: rlcg program reg:0x{:05x} failed !\n", $reg); }
    } else { WREG32($adev, $reg, $value); }
}}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
