/*
 * Copyright 2021 Advanced Micro Devices, Inc.
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
// amdgpu_ras.h, amdgpu.h, amdgpu_mca.h,
// umc/umc_6_7_0_offset.h, and umc/umc_6_7_0_sh_mask.h.

pub unsafe fn amdgpu_mca_query_correctable_error_count(
    adev: *mut amdgpu_device,
    mc_status_addr: u64,
    error_count: *mut c_ulong,
) {
    let mc_status = RREG64_PCIE(adev, mc_status_addr);

    if REG_GET_FIELD(mc_status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1
        && REG_GET_FIELD(mc_status, MCA_UMC_UMC0_MCUMC_STATUST0, CECC) == 1
    {
        *error_count += 1;
    }
}

pub unsafe fn amdgpu_mca_query_uncorrectable_error_count(
    adev: *mut amdgpu_device,
    mc_status_addr: u64,
    error_count: *mut c_ulong,
) {
    let mc_status = RREG64_PCIE(adev, mc_status_addr);

    if REG_GET_FIELD(mc_status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1
        && (REG_GET_FIELD(mc_status, MCA_UMC_UMC0_MCUMC_STATUST0, Deferred) == 1
            || REG_GET_FIELD(mc_status, MCA_UMC_UMC0_MCUMC_STATUST0, UECC) == 1
            || REG_GET_FIELD(mc_status, MCA_UMC_UMC0_MCUMC_STATUST0, PCC) == 1
            || REG_GET_FIELD(mc_status, MCA_UMC_UMC0_MCUMC_STATUST0, UC) == 1
            || REG_GET_FIELD(mc_status, MCA_UMC_UMC0_MCUMC_STATUST0, TCC) == 1)
    {
        *error_count += 1;
    }
}

pub unsafe fn amdgpu_mca_reset_error_count(adev: *mut amdgpu_device, mc_status_addr: u64) {
    WREG64_PCIE(adev, mc_status_addr, 0u64);
}

pub unsafe fn amdgpu_mca_query_ras_error_count(
    adev: *mut amdgpu_device,
    mc_status_addr: u64,
    ras_error_status: *mut c_void,
) {
    let err_data = ras_error_status as *mut ras_err_data;

    amdgpu_mca_query_correctable_error_count(adev, mc_status_addr, &mut (*err_data).ce_count);
    amdgpu_mca_query_uncorrectable_error_count(adev, mc_status_addr, &mut (*err_data).ue_count);

    amdgpu_mca_reset_error_count(adev, mc_status_addr);
}

pub unsafe fn amdgpu_mca_mp0_ras_sw_init(adev: *mut amdgpu_device) -> c_int {
    let err: c_int;
    let ras: *mut amdgpu_mca_ras_block;

    if (*adev).mca.mp0.ras.is_null() {
        return 0;
    }

    ras = (*adev).mca.mp0.ras;

    err = amdgpu_ras_register_ras_block(adev, &mut (*ras).ras_block);
    if err != 0 {
        dev_err((*adev).dev, "Failed to register mca.mp0 ras block!\n");
        return err;
    }

    strcpy((*ras).ras_block.ras_comm.name.as_mut_ptr(), b"mca.mp0\0".as_ptr());
    (*ras).ras_block.ras_comm.block = AMDGPU_RAS_BLOCK__MCA;
    (*ras).ras_block.ras_comm.sub_block_index = AMDGPU_RAS_MCA_BLOCK__MP0;
    (*ras).ras_block.ras_comm.type_ = AMDGPU_RAS_ERROR__MULTI_UNCORRECTABLE;
    (*adev).mca.mp0.ras_if = &mut (*ras).ras_block.ras_comm;

    0
}

pub unsafe fn amdgpu_mca_mp1_ras_sw_init(adev: *mut amdgpu_device) -> c_int {
    let err: c_int;
    let ras: *mut amdgpu_mca_ras_block;

    if (*adev).mca.mp1.ras.is_null() {
        return 0;
    }

    ras = (*adev).mca.mp1.ras;

    err = amdgpu_ras_register_ras_block(adev, &mut (*ras).ras_block);
    if err != 0 {
        dev_err((*adev).dev, "Failed to register mca.mp1 ras block!\n");
        return err;
    }

    strcpy((*ras).ras_block.ras_comm.name.as_mut_ptr(), b"mca.mp1\0".as_ptr());
    (*ras).ras_block.ras_comm.block = AMDGPU_RAS_BLOCK__MCA;
    (*ras).ras_block.ras_comm.sub_block_index = AMDGPU_RAS_MCA_BLOCK__MP1;
    (*ras).ras_block.ras_comm.type_ = AMDGPU_RAS_ERROR__MULTI_UNCORRECTABLE;
    (*adev).mca.mp1.ras_if = &mut (*ras).ras_block.ras_comm;

    0
}

pub unsafe fn amdgpu_mca_mpio_ras_sw_init(adev: *mut amdgpu_device) -> c_int {
    let err: c_int;
    let ras: *mut amdgpu_mca_ras_block;

    if (*adev).mca.mpio.ras.is_null() {
        return 0;
    }

    ras = (*adev).mca.mpio.ras;

    err = amdgpu_ras_register_ras_block(adev, &mut (*ras).ras_block);
    if err != 0 {
        dev_err((*adev).dev, "Failed to register mca.mpio ras block!\n");
        return err;
    }

    strcpy((*ras).ras_block.ras_comm.name.as_mut_ptr(), b"mca.mpio\0".as_ptr());
    (*ras).ras_block.ras_comm.block = AMDGPU_RAS_BLOCK__MCA;
    (*ras).ras_block.ras_comm.sub_block_index = AMDGPU_RAS_MCA_BLOCK__MPIO;
    (*ras).ras_block.ras_comm.type_ = AMDGPU_RAS_ERROR__MULTI_UNCORRECTABLE;
    (*adev).mca.mpio.ras_if = &mut (*ras).ras_block.ras_comm;

    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
