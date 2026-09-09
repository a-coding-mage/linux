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
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// C dependencies: linux/firmware.h, amdgpu.h, amdgpu_imu.h, amdgpu_dpm.h,
// imu_v12_1.h, gc_12_1_0_offset.h, gc_12_1_0_sh_mask.h,
// mmhub_4_2_0_offset.h.
// MODULE_FIRMWARE("amdgpu/gc_12_1_0_imu.bin");

const TRANSFER_RAM_MASK: u32 = 0x001c0000;

unsafe fn imu_v12_1_init_microcode(adev: *mut amdgpu_device) -> i32 {
    let mut ucode_prefix = [0i8; 15];
    let mut err: i32;
    let imu_hdr: *const imu_firmware_header_v1_0;
    let mut info: *mut amdgpu_firmware_info = core::ptr::null_mut();

    DRM_DEBUG!("\n");

    amdgpu_ucode_ip_version_decode(adev, GC_HWIP, ucode_prefix.as_mut_ptr(), ucode_prefix.len());
    err = amdgpu_ucode_request(
        adev,
        &mut (*(*adev).gfx).imu_fw,
        AMDGPU_UCODE_REQUIRED,
        "amdgpu/%s_imu.bin",
        ucode_prefix.as_ptr(),
    );
    if err != 0 {
        goto_out: {
            dev_err!((*adev).dev, "gfx12: Failed to load firmware \"%s_imu.bin\"\n", ucode_prefix.as_ptr());
            amdgpu_ucode_release(&mut (*(*adev).gfx).imu_fw);
        }
        return err;
    }

    imu_hdr = (*(*adev).gfx).imu_fw.data as *const imu_firmware_header_v1_0;
    (*(*adev).gfx).imu_fw_version = le32_to_cpu((*imu_hdr).header.ucode_version);

    if (*adev).firmware.load_type == AMDGPU_FW_LOAD_PSP {
        info = &mut (*adev).firmware.ucode[AMDGPU_UCODE_ID_IMU_I] as *mut amdgpu_firmware_info;
        (*info).ucode_id = AMDGPU_UCODE_ID_IMU_I;
        (*info).fw = (*(*adev).gfx).imu_fw;
        (*adev).firmware.fw_size += ALIGN(le32_to_cpu((*imu_hdr).imu_iram_ucode_size_bytes), PAGE_SIZE);
        info = &mut (*adev).firmware.ucode[AMDGPU_UCODE_ID_IMU_D] as *mut amdgpu_firmware_info;
        (*info).ucode_id = AMDGPU_UCODE_ID_IMU_D;
        (*info).fw = (*(*adev).gfx).imu_fw;
        (*adev).firmware.fw_size += ALIGN(le32_to_cpu((*imu_hdr).imu_dram_ucode_size_bytes), PAGE_SIZE);
    }

    0
}

unsafe fn imu_v12_1_xcc_load_microcode(adev: *mut amdgpu_device, xcc_id: i32) {
    let hdr = (*(*adev).gfx).imu_fw.data as *const imu_firmware_header_v1_0;
    let mut fw_data = ((*(*adev).gfx).imu_fw.data.add(le32_to_cpu((*hdr).header.ucode_array_offset_bytes) as usize)) as *const __le32;
    let fw_size = le32_to_cpu((*hdr).imu_iram_ucode_size_bytes) / 4;

    WREG32_SOC15!(GC, GET_INST!(GC, xcc_id), regGFX_IMU_I_RAM_ADDR, 0);
    for _i in 0..fw_size {
        WREG32_SOC15!(GC, GET_INST!(GC, xcc_id), regGFX_IMU_I_RAM_DATA, le32_to_cpup(fw_data));
        fw_data = fw_data.add(1);
    }
    WREG32_SOC15!(GC, GET_INST!(GC, xcc_id), regGFX_IMU_I_RAM_ADDR, (*(*adev).gfx).imu_fw_version);

    fw_data = (*(*adev).gfx).imu_fw.data
        .add((le32_to_cpu((*hdr).header.ucode_array_offset_bytes) + le32_to_cpu((*hdr).imu_iram_ucode_size_bytes)) as usize)
        as *const __le32;
    let fw_size = le32_to_cpu((*hdr).imu_dram_ucode_size_bytes) / 4;
    WREG32_SOC15!(GC, GET_INST!(GC, xcc_id), regGFX_IMU_D_RAM_ADDR, 0);
    for _i in 0..fw_size {
        WREG32_SOC15!(GC, GET_INST!(GC, xcc_id), regGFX_IMU_D_RAM_DATA, le32_to_cpup(fw_data));
        fw_data = fw_data.add(1);
    }
    WREG32_SOC15!(GC, GET_INST!(GC, xcc_id), regGFX_IMU_D_RAM_ADDR, (*(*adev).gfx).imu_fw_version);
}

unsafe fn imu_v12_1_load_microcode(adev: *mut amdgpu_device) -> i32 {
    if (*(*adev).gfx).imu_fw.is_null() { return -EINVAL; }
    let num_xcc = NUM_XCC!((*(*adev).gfx).xcc_mask);
    for i in 0..num_xcc { imu_v12_1_xcc_load_microcode(adev, i); }
    0
}

unsafe fn imu_v12_1_switch_compute_partition(adev: *mut amdgpu_device, num_xccs_per_xcp: i32, compute_partition_mode: i32) -> i32 {
    if !(*adev).psp.funcs.is_null() {
        let ret = psp_spatial_partition(&mut (*adev).psp, compute_partition_mode);
        if ret != 0 { return ret; }
    }
    (*(*adev).gfx).num_xcc_per_xcp = num_xccs_per_xcp;
    0
}

const gfx_v12_1_imu_funcs: amdgpu_imu_funcs = amdgpu_imu_funcs {
    init_microcode: Some(imu_v12_1_init_microcode),
    load_microcode: Some(imu_v12_1_load_microcode),
    switch_compute_partition: Some(imu_v12_1_switch_compute_partition),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
