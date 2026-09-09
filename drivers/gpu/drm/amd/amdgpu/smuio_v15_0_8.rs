/*
 * Copyright 2025 Advanced Micro Devices, Inc.
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

// Dependencies supplied by the surrounding amdgpu translation.

const SMUIO_MCM_CONFIG__HOST_GPU_XGMI_MASK: u32 = 0x00000001;
const SMUIO_MCM_CONFIG__ETHERNET_SWITCH_MASK: u32 = 0x00000008;
const SMUIO_MCM_CONFIG__CUSTOM_HBM_MASK: u32 = 0x00000001;

unsafe fn smuio_v15_0_8_get_rom_index_offset(adev: *mut amdgpu_device) -> u32 {
    SOC15_REG_OFFSET(SMUIO, 0, regROM_INDEX)
}

unsafe fn smuio_v15_0_8_get_rom_data_offset(adev: *mut amdgpu_device) -> u32 {
    SOC15_REG_OFFSET(SMUIO, 0, regROM_DATA)
}

unsafe fn smuio_v15_0_8_update_rom_clock_gating(
    adev: *mut amdgpu_device,
    enable: bool,
) {
    return;
}

unsafe fn smuio_v15_0_8_get_gpu_clock_counter(adev: *mut amdgpu_device) -> u64 {
    let clock: u64;
    let mut clock_counter_lo: u64;
    let clock_counter_hi_pre: u64;
    let clock_counter_hi_after: u64;

    preempt_disable();
    clock_counter_hi_pre = RREG32_SOC15(SMUIO, 0, regGOLDEN_TSC_COUNT_UPPER) as u64;
    clock_counter_lo = RREG32_SOC15(SMUIO, 0, regGOLDEN_TSC_COUNT_LOWER) as u64;
    /* the clock counter may be udpated during polling the counters */
    clock_counter_hi_after = RREG32_SOC15(SMUIO, 0, regGOLDEN_TSC_COUNT_UPPER) as u64;
    if clock_counter_hi_pre != clock_counter_hi_after {
        clock_counter_lo = RREG32_SOC15(SMUIO, 0, regGOLDEN_TSC_COUNT_LOWER) as u64;
    }
    preempt_enable();

    clock = clock_counter_lo | (clock_counter_hi_after << 32u64);

    clock
}

unsafe fn smuio_v15_0_8_get_clock_gating_state(
    adev: *mut amdgpu_device,
    flags: *mut u64,
) {
    let data: u32;

    /* CGTT_ROM_CLK_CTRL0 is not available for APU */
    if (*adev).flags & AMD_IS_APU != 0 {
        return;
    }

    data = RREG32_SOC15(SMUIO, 0, regCGTT_ROM_CLK_CTRL0);
    if data & CGTT_ROM_CLK_CTRL0__SOFT_OVERRIDE0_MASK == 0 {
        *flags |= AMD_CG_SUPPORT_ROM_MGCG;
    }
}

/**
 * smuio_v15_0_8_get_die_id - query die id from FCH.
 *
 * @adev: amdgpu device pointer
 *
 * Returns die id
 */
unsafe fn smuio_v15_0_8_get_die_id(adev: *mut amdgpu_device) -> u32 {
    let data = RREG32_SOC15(SMUIO, 0, regSMUIO_MCM_CONFIG);
    REG_GET_FIELD(data, SMUIO_MCM_CONFIG, DIE_ID)
}

/**
 * smuio_v15_0_8_get_socket_id - query socket id from FCH
 *
 * @adev: amdgpu device pointer
 *
 * Returns socket id
 */
unsafe fn smuio_v15_0_8_get_socket_id(adev: *mut amdgpu_device) -> u32 {
    let data = RREG32_SOC15(SMUIO, 0, regSMUIO_MCM_CONFIG);
    REG_GET_FIELD(data, SMUIO_MCM_CONFIG, SOCKET_ID)
}

/**
 * smuio_v15_0_8_is_host_gpu_xgmi_supported - detect xgmi interface between cpu and gpu/s.
 *
 * @adev: amdgpu device pointer
 *
 * Returns true on success or false otherwise.
 */
unsafe fn smuio_v15_0_8_is_host_gpu_xgmi_supported(adev: *mut amdgpu_device) -> bool {
    let mut data = RREG32_SOC15(SMUIO, 0, regSMUIO_MCM_CONFIG);
    data = REG_GET_FIELD(data, SMUIO_MCM_CONFIG, TOPOLOGY_ID);
    /* data[4:0]
     * bit 0 == 0 host-gpu interface is PCIE
     * bit 0 == 1 host-gpu interface is Alternate Protocal
     * for AMD, this is XGMI
     */
    data &= SMUIO_MCM_CONFIG__HOST_GPU_XGMI_MASK;

    data != 0
}

/* Disabled in the source (#if 0).
unsafe fn smuio_v15_0_8_is_connected_with_ethernet_switch(adev: *mut amdgpu_device) -> bool {
    if (*adev).flags & AMD_IS_APU == 0 { return false; }
    let mut data = RREG32_SOC15(SMUIO, 0, regSMUIO_MCM_CONFIG);
    data = REG_GET_FIELD(data, SMUIO_MCM_CONFIG, TOPOLOGY_ID);
    data &= SMUIO_MCM_CONFIG__ETHERNET_SWITCH_MASK;
    data == 0
}
*/

unsafe fn smuio_v15_0_8_get_pkg_type(adev: *mut amdgpu_device) -> amdgpu_pkg_type {
    let mut pkg_type: amdgpu_pkg_type;
    let mut data = RREG32_SOC15(SMUIO, 0, regSMUIO_MCM_CONFIG);
    data = REG_GET_FIELD(data, SMUIO_MCM_CONFIG, PKG_TYPE);

    /* data [3:0]
     bit 2 and bit 3 identifies the pkg type */
    match data & 0xC {
        0x0 => pkg_type = AMDGPU_PKG_TYPE_BB,
        0x8 => pkg_type = AMDGPU_PKG_TYPE_CEM,
        _ => pkg_type = AMDGPU_PKG_TYPE_UNKNOWN,
    }

    pkg_type
}

/* Disabled in the source (#if 0).
unsafe fn smuio_v15_0_8_is_custom_hbm_supported(adev: *mut amdgpu_device) -> bool {
    let mut data = RREG32_SOC15(SMUIO, 0, regSMUIO_MCM_CONFIG);
    data = REG_GET_FIELD(data, SMUIO_MCM_CONFIG, PKG_TYPE);
    data &= SMUIO_MCM_CONFIG__CUSTOM_HBM_MASK;
    data != 0
}
*/

pub static smuio_v15_0_8_funcs: amdgpu_smuio_funcs = amdgpu_smuio_funcs {
    get_rom_index_offset: Some(smuio_v15_0_8_get_rom_index_offset),
    get_rom_data_offset: Some(smuio_v15_0_8_get_rom_data_offset),
    get_gpu_clock_counter: Some(smuio_v15_0_8_get_gpu_clock_counter),
    get_die_id: Some(smuio_v15_0_8_get_die_id),
    get_socket_id: Some(smuio_v15_0_8_get_socket_id),
    is_host_gpu_xgmi_supported: Some(smuio_v15_0_8_is_host_gpu_xgmi_supported),
    update_rom_clock_gating: Some(smuio_v15_0_8_update_rom_clock_gating),
    get_clock_gating_state: Some(smuio_v15_0_8_get_clock_gating_state),
    get_pkg_type: Some(smuio_v15_0_8_get_pkg_type),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
