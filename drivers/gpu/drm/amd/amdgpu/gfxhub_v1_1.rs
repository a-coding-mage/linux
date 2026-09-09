/*
 * Copyright 2018 Advanced Micro Devices, Inc.
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

const MM_MC_VM_XGMI_LFB_CNTL_ALDE: u32 = 0x0978;
const MM_MC_VM_XGMI_LFB_CNTL_ALDE_BASE_IDX: u32 = 0;
const MM_MC_VM_XGMI_LFB_SIZE_ALDE: u32 = 0x0979;
const MM_MC_VM_XGMI_LFB_SIZE_ALDE_BASE_IDX: u32 = 0;

// MC_VM_XGMI_LFB_CNTL
const MC_VM_XGMI_LFB_CNTL_ALDE_PF_LFB_REGION_SHIFT: u32 = 0x0;
const MC_VM_XGMI_LFB_CNTL_ALDE_PF_MAX_REGION_SHIFT: u32 = 0x4;
const MC_VM_XGMI_LFB_CNTL_ALDE_PF_LFB_REGION_MASK: u32 = 0x0000000f;
const MC_VM_XGMI_LFB_CNTL_ALDE_PF_MAX_REGION_MASK: u32 = 0x000000f0;

// MC_VM_XGMI_LFB_SIZE
const MC_VM_XGMI_LFB_SIZE_ALDE_PF_LFB_SIZE_SHIFT: u32 = 0x0;
const MC_VM_XGMI_LFB_SIZE_ALDE_PF_LFB_SIZE_MASK: u32 = 0x0001ffff;

pub unsafe fn gfxhub_v1_1_get_xgmi_info(adev: *mut amdgpu_device) -> i32 {
    let mut max_num_physical_nodes: u32;
    let mut max_physical_node_id: u32;
    let xgmi_lfb_cntl: u32;
    let max_region: u32;
    let seg_size: u64;

    if (*adev).asic_type == CHIP_ALDEBARAN {
        xgmi_lfb_cntl = RREG32_SOC15(GC, 0, MM_MC_VM_XGMI_LFB_CNTL_ALDE);
        seg_size = (REG_GET_FIELD(
            RREG32_SOC15(GC, 0, MM_MC_VM_XGMI_LFB_SIZE_ALDE),
            MC_VM_XGMI_LFB_SIZE,
            PF_LFB_SIZE,
        ) as u64) << 24;
        max_region = REG_GET_FIELD(
            xgmi_lfb_cntl,
            MC_VM_XGMI_LFB_CNTL_ALDE,
            PF_MAX_REGION,
        );
    } else {
        xgmi_lfb_cntl = RREG32_SOC15(GC, 0, mmMC_VM_XGMI_LFB_CNTL);
        seg_size = (REG_GET_FIELD(
            RREG32_SOC15(GC, 0, mmMC_VM_XGMI_LFB_SIZE),
            MC_VM_XGMI_LFB_SIZE,
            PF_LFB_SIZE,
        ) as u64) << 24;
        max_region = REG_GET_FIELD(xgmi_lfb_cntl, MC_VM_XGMI_LFB_CNTL, PF_MAX_REGION);
    }

    match (*adev).asic_type {
        CHIP_VEGA20 => {
            max_num_physical_nodes = 4;
            max_physical_node_id = 3;
        }
        CHIP_ARCTURUS => {
            max_num_physical_nodes = 8;
            max_physical_node_id = 7;
        }
        CHIP_ALDEBARAN => {
            max_num_physical_nodes = 16;
            max_physical_node_id = 15;
        }
        _ => return -EINVAL,
    }

    /* PF_MAX_REGION=0 means xgmi is disabled */
    if max_region != 0 || (*adev).gmc.xgmi.connected_to_cpu {
        (*adev).gmc.xgmi.num_physical_nodes = max_region + 1;

        if (*adev).gmc.xgmi.num_physical_nodes > max_num_physical_nodes {
            return -EINVAL;
        }

        if (*adev).asic_type == CHIP_ALDEBARAN {
            (*adev).gmc.xgmi.physical_node_id = REG_GET_FIELD(
                xgmi_lfb_cntl,
                MC_VM_XGMI_LFB_CNTL_ALDE,
                PF_LFB_REGION,
            );
        } else {
            (*adev).gmc.xgmi.physical_node_id = REG_GET_FIELD(
                xgmi_lfb_cntl,
                MC_VM_XGMI_LFB_CNTL,
                PF_LFB_REGION,
            );
        }

        if (*adev).gmc.xgmi.physical_node_id > max_physical_node_id {
            return -EINVAL;
        }

        (*adev).gmc.xgmi.node_segment_size = seg_size;
    }

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
