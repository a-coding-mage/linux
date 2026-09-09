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

// Dependencies supplied by the surrounding translation unit:
// amdgpu.h, soc15.h, soc15_common.h, and arct_ip_offset.h.

pub unsafe fn arct_reg_base_init(adev: *mut amdgpu_device) -> i32 {
    /* HW has more IP blocks,  only initialized the block needed by our driver */
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
            (&mut NBIF0_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[MP0_HWIP as usize][i as usize] =
            (&mut MP0_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[MP1_HWIP as usize][i as usize] =
            (&mut MP1_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[UVD_HWIP as usize][i as usize] =
            (&mut UVD_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[DF_HWIP as usize][i as usize] =
            (&mut DF_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[OSSSYS_HWIP as usize][i as usize] =
            (&mut OSSSYS_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[SDMA0_HWIP as usize][i as usize] =
            (&mut SDMA0_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[SDMA1_HWIP as usize][i as usize] =
            (&mut SDMA1_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[SDMA2_HWIP as usize][i as usize] =
            (&mut SDMA2_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[SDMA3_HWIP as usize][i as usize] =
            (&mut SDMA3_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[SDMA4_HWIP as usize][i as usize] =
            (&mut SDMA4_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[SDMA5_HWIP as usize][i as usize] =
            (&mut SDMA5_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[SDMA6_HWIP as usize][i as usize] =
            (&mut SDMA6_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[SDMA7_HWIP as usize][i as usize] =
            (&mut SDMA7_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[SMUIO_HWIP as usize][i as usize] =
            (&mut SMUIO_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[THM_HWIP as usize][i as usize] =
            (&mut THM_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[UMC_HWIP as usize][i as usize] =
            (&mut UMC_BASE.instance[i as usize] as *mut _).cast::<u32>();
        (*adev).reg_offset[RSMU_HWIP as usize][i as usize] =
            (&mut RSMU_BASE.instance[i as usize] as *mut _).cast::<u32>();
        i += 1;
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
