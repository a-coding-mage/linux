/*
 * Copyright 2020 Advanced Micro Devices, Inc.
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

const SMUIO_MCM_CONFIG_HOST_GPU_XGMI_MASK: u32 = 0x00000001;

unsafe fn smuio_v13_0_get_rom_index_offset(adev: *mut amdgpu_device) -> u32 {
    SOC15_REG_OFFSET!(SMUIO, 0, regROM_INDEX)
}

unsafe fn smuio_v13_0_get_rom_data_offset(adev: *mut amdgpu_device) -> u32 {
    SOC15_REG_OFFSET!(SMUIO, 0, regROM_DATA)
}

unsafe fn smuio_v13_0_update_rom_clock_gating(adev: *mut amdgpu_device, enable: bool) {
    let (mut def, mut data): (u32, u32);

    /* enable/disable ROM CG is not supported on APU */
    if (*adev).flags & AMD_IS_APU != 0 {
        return;
    }

    def = RREG32_SOC15!(SMUIO, 0, regCGTT_ROM_CLK_CTRL0);
    data = def;

    if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_ROM_MGCG != 0) {
        data &= !(CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE0_MASK |
            CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE1_MASK);
    } else {
        data |= CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE0_MASK |
            CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE1_MASK;
    }

    if def != data {
        WREG32_SOC15!(SMUIO, 0, regCGTT_ROM_CLK_CTRL0, data);
    }
}

unsafe fn smuio_v13_0_get_clock_gating_state(adev: *mut amdgpu_device, flags: *mut u64) {
    let data: u32;

    /* CGTT_ROM_CLK_CTRL0 is not available for APU */
    if (*adev).flags & AMD_IS_APU != 0 {
        return;
    }

    data = RREG32_SOC15!(SMUIO, 0, regCGTT_ROM_CLK_CTRL0);
    if data & CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE0_MASK == 0 {
        *flags |= AMD_CG_SUPPORT_ROM_MGCG as u64;
    }
}

/**
 * smuio_v13_0_get_die_id - query die id from FCH.
 *
 * @adev: amdgpu device pointer
 *
 * Returns die id
 */
unsafe fn smuio_v13_0_get_die_id(adev: *mut amdgpu_device) -> u32 {
    let data = RREG32_SOC15!(SMUIO, 0, regSMUIO_MCM_CONFIG);
    REG_GET_FIELD!(data, SMUIO_MCM_CONFIG, DIE_ID)
}

/**
 * smuio_v13_0_get_socket_id - query socket id from FCH
 *
 * @adev: amdgpu device pointer
 *
 * Returns socket id
 */
unsafe fn smuio_v13_0_get_socket_id(adev: *mut amdgpu_device) -> u32 {
    let data = RREG32_SOC15!(SMUIO, 0, regSMUIO_MCM_CONFIG);
    REG_GET_FIELD!(data, SMUIO_MCM_CONFIG, SOCKET_ID)
}

/**
 * smuio_v13_0_is_host_gpu_xgmi_supported - detect xgmi interface between cpu and gpu/s.
 *
 * @adev: amdgpu device pointer
 *
 * Returns true on success or false otherwise.
 */
unsafe fn smuio_v13_0_is_host_gpu_xgmi_supported(adev: *mut amdgpu_device) -> bool {
    let mut data = RREG32_SOC15!(SMUIO, 0, regSMUIO_MCM_CONFIG);
    data = REG_GET_FIELD!(data, SMUIO_MCM_CONFIG, TOPOLOGY_ID);
    /* data[4:0]
     * bit 0 == 0 host-gpu interface is PCIE
     * bit 0 == 1 host-gpu interface is Alternate Protocal
     * for AMD, this is XGMI
     */
    data &= SMUIO_MCM_CONFIG_HOST_GPU_XGMI_MASK;

    data != 0
}

unsafe fn smuio_v13_0_get_pkg_type(adev: *mut amdgpu_device) -> amdgpu_pkg_type {
    let data = REG_GET_FIELD!(
        RREG32_SOC15!(SMUIO, 0, regSMUIO_MCM_CONFIG),
        SMUIO_MCM_CONFIG,
        TOPOLOGY_ID
    );

    match data {
        0x4 | 0xC => AMDGPU_PKG_TYPE_CEM,
        _ => AMDGPU_PKG_TYPE_OAM,
    }
}

pub static smuio_v13_0_funcs: amdgpu_smuio_funcs = amdgpu_smuio_funcs {
    get_rom_index_offset: Some(smuio_v13_0_get_rom_index_offset),
    get_rom_data_offset: Some(smuio_v13_0_get_rom_data_offset),
    get_die_id: Some(smuio_v13_0_get_die_id),
    get_socket_id: Some(smuio_v13_0_get_socket_id),
    is_host_gpu_xgmi_supported: Some(smuio_v13_0_is_host_gpu_xgmi_supported),
    update_rom_clock_gating: Some(smuio_v13_0_update_rom_clock_gating),
    get_clock_gating_state: Some(smuio_v13_0_get_clock_gating_state),
    get_pkg_type: Some(smuio_v13_0_get_pkg_type),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
