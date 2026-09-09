/*
 * Copyright 2017 Advanced Micro Devices, Inc.
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
 *
 */

// Dependencies supplied by the surrounding AMDGPU translation unit.

pub unsafe fn vega10_reg_base_init(adev: *mut amdgpu_device) -> i32 {
    /* HW has more IP blocks,  only initialized the blocke beend by our driver  */
    let mut i: u32 = 0;
    while i < MAX_INSTANCE {
        (*adev).reg_offset[GC_HWIP as usize][i as usize] =
            (&mut GC_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[HDP_HWIP as usize][i as usize] =
            (&mut HDP_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[MMHUB_HWIP as usize][i as usize] =
            (&mut MMHUB_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[ATHUB_HWIP as usize][i as usize] =
            (&mut ATHUB_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[NBIO_HWIP as usize][i as usize] =
            (&mut NBIO_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[MP0_HWIP as usize][i as usize] =
            (&mut MP0_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[MP1_HWIP as usize][i as usize] =
            (&mut MP1_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[UVD_HWIP as usize][i as usize] =
            (&mut UVD_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[VCE_HWIP as usize][i as usize] =
            (&mut VCE_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[VCN_HWIP as usize][i as usize] =
            (&mut VCN_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[DF_HWIP as usize][i as usize] =
            (&mut DF_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[DCE_HWIP as usize][i as usize] =
            (&mut DCE_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[OSSSYS_HWIP as usize][i as usize] =
            (&mut OSSSYS_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[SDMA0_HWIP as usize][i as usize] =
            (&mut SDMA0_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[SDMA1_HWIP as usize][i as usize] =
            (&mut SDMA1_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[SMUIO_HWIP as usize][i as usize] =
            (&mut SMUIO_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[PWR_HWIP as usize][i as usize] =
            (&mut PWR_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[NBIF_HWIP as usize][i as usize] =
            (&mut NBIF_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[THM_HWIP as usize][i as usize] =
            (&mut THM_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[CLK_HWIP as usize][i as usize] =
            (&mut CLK_BASE.instance[i as usize] as *mut _).cast::<u32>();
        i += 1;
    }
    0
}

pub unsafe fn vega10_doorbell_index_init(adev: *mut amdgpu_device) {
    (*adev).doorbell_index.kiq = AMDGPU_DOORBELL64_KIQ;
    (*adev).doorbell_index.mec_ring0 = AMDGPU_DOORBELL64_MEC_RING0;
    (*adev).doorbell_index.mec_ring1 = AMDGPU_DOORBELL64_MEC_RING1;
    (*adev).doorbell_index.mec_ring2 = AMDGPU_DOORBELL64_MEC_RING2;
    (*adev).doorbell_index.mec_ring3 = AMDGPU_DOORBELL64_MEC_RING3;
    (*adev).doorbell_index.mec_ring4 = AMDGPU_DOORBELL64_MEC_RING4;
    (*adev).doorbell_index.mec_ring5 = AMDGPU_DOORBELL64_MEC_RING5;
    (*adev).doorbell_index.mec_ring6 = AMDGPU_DOORBELL64_MEC_RING6;
    (*adev).doorbell_index.mec_ring7 = AMDGPU_DOORBELL64_MEC_RING7;
    (*adev).doorbell_index.userqueue_start = AMDGPU_DOORBELL64_USERQUEUE_START;
    (*adev).doorbell_index.userqueue_end = AMDGPU_DOORBELL64_USERQUEUE_END;
    (*adev).doorbell_index.gfx_ring0 = AMDGPU_DOORBELL64_GFX_RING0;
    (*adev).doorbell_index.sdma_engine[0] = AMDGPU_DOORBELL64_sDMA_ENGINE0;
    (*adev).doorbell_index.sdma_engine[1] = AMDGPU_DOORBELL64_sDMA_ENGINE1;
    (*adev).doorbell_index.ih = AMDGPU_DOORBELL64_IH;
    (*adev).doorbell_index.uvd_vce.uvd_ring0_1 = AMDGPU_DOORBELL64_UVD_RING0_1;
    (*adev).doorbell_index.uvd_vce.uvd_ring2_3 = AMDGPU_DOORBELL64_UVD_RING2_3;
    (*adev).doorbell_index.uvd_vce.uvd_ring4_5 = AMDGPU_DOORBELL64_UVD_RING4_5;
    (*adev).doorbell_index.uvd_vce.uvd_ring6_7 = AMDGPU_DOORBELL64_UVD_RING6_7;
    (*adev).doorbell_index.uvd_vce.vce_ring0_1 = AMDGPU_DOORBELL64_VCE_RING0_1;
    (*adev).doorbell_index.uvd_vce.vce_ring2_3 = AMDGPU_DOORBELL64_VCE_RING2_3;
    (*adev).doorbell_index.uvd_vce.vce_ring4_5 = AMDGPU_DOORBELL64_VCE_RING4_5;
    (*adev).doorbell_index.uvd_vce.vce_ring6_7 = AMDGPU_DOORBELL64_VCE_RING6_7;
    (*adev).doorbell_index.vcn.vcn_ring0_1 = AMDGPU_DOORBELL64_VCN0_1;
    (*adev).doorbell_index.vcn.vcn_ring2_3 = AMDGPU_DOORBELL64_VCN2_3;
    (*adev).doorbell_index.vcn.vcn_ring4_5 = AMDGPU_DOORBELL64_VCN4_5;
    (*adev).doorbell_index.vcn.vcn_ring6_7 = AMDGPU_DOORBELL64_VCN6_7;

    (*adev).doorbell_index.first_non_cp = AMDGPU_DOORBELL64_FIRST_NON_CP;
    (*adev).doorbell_index.last_non_cp = AMDGPU_DOORBELL64_LAST_NON_CP;

    /* In unit of dword doorbell */
    (*adev).doorbell_index.max_assignment = AMDGPU_DOORBELL64_MAX_ASSIGNMENT << 1;
    (*adev).doorbell_index.sdma_doorbell_range = 4;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
