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

// Dependencies supplied by the surrounding AMDGPU translation unit:
// amdgpu.h, hdp_v4_0.h, amdgpu_ras.h, hdp/hdp_4_0_offset.h,
// hdp/hdp_4_0_sh_mask.h, and uapi/linux/kfd_ioctl.h.

/* for Vega20 register name change */
const mmHDP_MEM_POWER_CTRL: u32 = 0x00d4;
const HDP_MEM_POWER_CTRL_IPH_MEM_POWER_CTRL_EN_MASK: u32 = 0x00000001;
const HDP_MEM_POWER_CTRL_IPH_MEM_POWER_LS_EN_MASK: u32 = 0x00000002;
const HDP_MEM_POWER_CTRL_RC_MEM_POWER_CTRL_EN_MASK: u32 = 0x00010000;
const HDP_MEM_POWER_CTRL_RC_MEM_POWER_LS_EN_MASK: u32 = 0x00020000;
const mmHDP_MEM_POWER_CTRL_BASE_IDX: u32 = 0;

unsafe fn hdp_v4_0_invalidate_hdp(
    adev: *mut amdgpu_device,
    ring: *mut amdgpu_ring,
) {
    if amdgpu_ip_version(adev, HDP_HWIP, 0) == IP_VERSION(4, 4, 0)
        || amdgpu_ip_version(adev, HDP_HWIP, 0) == IP_VERSION(4, 4, 2)
        || amdgpu_ip_version(adev, HDP_HWIP, 0) == IP_VERSION(4, 4, 5)
    {
        return;
    }

    if ring.is_null() || (*ring).funcs.is_null() || (*(*ring).funcs).emit_wreg.is_none() {
        WREG32_SOC15_NO_KIQ(HDP, 0, mmHDP_READ_CACHE_INVALIDATE, 1);
        RREG32_SOC15_NO_KIQ(HDP, 0, mmHDP_READ_CACHE_INVALIDATE);
    } else {
        amdgpu_ring_emit_wreg(ring, SOC15_REG_OFFSET(HDP, 0, mmHDP_READ_CACHE_INVALIDATE), 1);
    }
}

unsafe fn hdp_v4_0_query_ras_error_count(
    adev: *mut amdgpu_device,
    ras_error_status: *mut core::ffi::c_void,
) {
    let err_data = ras_error_status as *mut ras_err_data;

    (*err_data).ue_count = 0;
    (*err_data).ce_count = 0;

    if !amdgpu_ras_is_supported(adev, AMDGPU_RAS_BLOCK__HDP) {
        return;
    }

    /* HDP SRAM errors are uncorrectable ones (i.e. fatal errors) */
    (*err_data).ue_count += RREG32_SOC15(HDP, 0, mmHDP_EDC_CNT);
}

unsafe fn hdp_v4_0_reset_ras_error_count(adev: *mut amdgpu_device) {
    if !amdgpu_ras_is_supported(adev, AMDGPU_RAS_BLOCK__HDP) {
        return;
    }

    if amdgpu_ip_version(adev, HDP_HWIP, 0) >= IP_VERSION(4, 4, 0) {
        WREG32_SOC15(HDP, 0, mmHDP_EDC_CNT, 0);
    } else {
        /*read back hdp ras counter to reset it to 0 */
        RREG32_SOC15(HDP, 0, mmHDP_EDC_CNT);
    }
}

unsafe fn hdp_v4_0_update_clock_gating(adev: *mut amdgpu_device, enable: bool) {
    let (mut def, mut data): (u32, u32);

    if amdgpu_ip_version(adev, HDP_HWIP, 0) == IP_VERSION(4, 0, 0)
        || amdgpu_ip_version(adev, HDP_HWIP, 0) == IP_VERSION(4, 0, 1)
        || amdgpu_ip_version(adev, HDP_HWIP, 0) == IP_VERSION(4, 1, 1)
        || amdgpu_ip_version(adev, HDP_HWIP, 0) == IP_VERSION(4, 1, 0)
    {
        def = RREG32(SOC15_REG_OFFSET(HDP, 0, mmHDP_MEM_POWER_LS));
        data = def;

        if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_HDP_LS) != 0 {
            data |= HDP_MEM_POWER_LS_LS_ENABLE_MASK;
        } else {
            data &= !HDP_MEM_POWER_LS_LS_ENABLE_MASK;
        }

        if def != data {
            WREG32(SOC15_REG_OFFSET(HDP, 0, mmHDP_MEM_POWER_LS), data);
        }
    } else {
        def = RREG32(SOC15_REG_OFFSET(HDP, 0, mmHDP_MEM_POWER_CTRL));
        data = def;

        if enable && ((*adev).cg_flags & AMD_CG_SUPPORT_HDP_LS) != 0 {
            data |= HDP_MEM_POWER_CTRL_IPH_MEM_POWER_CTRL_EN_MASK
                | HDP_MEM_POWER_CTRL_IPH_MEM_POWER_LS_EN_MASK
                | HDP_MEM_POWER_CTRL_RC_MEM_POWER_CTRL_EN_MASK
                | HDP_MEM_POWER_CTRL_RC_MEM_POWER_LS_EN_MASK;
        } else {
            data &= !(HDP_MEM_POWER_CTRL_IPH_MEM_POWER_CTRL_EN_MASK
                | HDP_MEM_POWER_CTRL_IPH_MEM_POWER_LS_EN_MASK
                | HDP_MEM_POWER_CTRL_RC_MEM_POWER_CTRL_EN_MASK
                | HDP_MEM_POWER_CTRL_RC_MEM_POWER_LS_EN_MASK);
        }

        if def != data {
            WREG32(SOC15_REG_OFFSET(HDP, 0, mmHDP_MEM_POWER_CTRL), data);
        }
    }
}

unsafe fn hdp_v4_0_get_clockgating_state(adev: *mut amdgpu_device, flags: *mut u64) {
    let data: i32;

    if amdgpu_ip_version(adev, HDP_HWIP, 0) == IP_VERSION(4, 4, 2)
        || amdgpu_ip_version(adev, HDP_HWIP, 0) == IP_VERSION(4, 4, 5)
    {
        /* Default enabled */
        *flags |= AMD_CG_SUPPORT_HDP_MGCG as u64;
        return;
    }
    /* AMD_CG_SUPPORT_HDP_LS */
    data = RREG32(SOC15_REG_OFFSET(HDP, 0, mmHDP_MEM_POWER_LS)) as i32;
    if (data & HDP_MEM_POWER_LS_LS_ENABLE_MASK as i32) != 0 {
        *flags |= AMD_CG_SUPPORT_HDP_LS as u64;
    }
}

unsafe fn hdp_v4_0_init_registers(adev: *mut amdgpu_device) {
    match amdgpu_ip_version(adev, HDP_HWIP, 0) {
        v if v == IP_VERSION(4, 2, 1) => {
            WREG32_FIELD15(HDP, 0, HDP_MMHUB_CNTL, HDP_MMHUB_GCC, 1);
        }
        _ => {}
    }

    /* Do not program registers if VF */
    if amdgpu_sriov_vf(adev) {
        return;
    }

    WREG32_FIELD15(HDP, 0, HDP_MISC_CNTL, FLUSH_INVALIDATE_CACHE, 1);

    if amdgpu_ip_version(adev, HDP_HWIP, 0) == IP_VERSION(4, 4, 0) {
        WREG32_FIELD15(HDP, 0, HDP_MISC_CNTL, READ_BUFFER_WATERMARK, 2);
    }

    WREG32_SOC15(HDP, 0, mmHDP_NONSURFACE_BASE, (*adev).gmc.vram_start >> 8);
    WREG32_SOC15(HDP, 0, mmHDP_NONSURFACE_BASE_HI, (*adev).gmc.vram_start >> 40);
}

static mut hdp_v4_0_ras_hw_ops: amdgpu_ras_block_hw_ops = amdgpu_ras_block_hw_ops {
    query_ras_error_count: Some(hdp_v4_0_query_ras_error_count),
    reset_ras_error_count: Some(hdp_v4_0_reset_ras_error_count),
};

static mut hdp_v4_0_ras: amdgpu_hdp_ras = amdgpu_hdp_ras {
    ras_block: amdgpu_ras_block {
        hw_ops: &raw mut hdp_v4_0_ras_hw_ops,
    },
};

const hdp_v4_0_funcs: amdgpu_hdp_funcs = amdgpu_hdp_funcs {
    flush_hdp: Some(amdgpu_hdp_generic_flush),
    invalidate_hdp: Some(hdp_v4_0_invalidate_hdp),
    update_clock_gating: Some(hdp_v4_0_update_clock_gating),
    get_clock_gating_state: Some(hdp_v4_0_get_clockgating_state),
    init_registers: Some(hdp_v4_0_init_registers),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
