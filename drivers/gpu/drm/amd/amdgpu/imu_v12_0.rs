/*
 * Copyright 2023 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

// Firmware declarations and register definitions are supplied by the surrounding kernel bindings.

const TRANSFER_RAM_MASK: u32 = 0x001c0000;

unsafe fn imu_v12_0_init_microcode(adev: *mut amdgpu_device) -> i32 {
    let mut ucode_prefix = [0i8; 30];
    let mut err: i32;
    let mut imu_hdr: *const imu_firmware_header_v1_0;
    let mut info: *mut amdgpu_firmware_info = core::ptr::null_mut();
    DRM_DEBUG!("\n");
    amdgpu_ucode_ip_version_decode(adev, GC_HWIP, ucode_prefix.as_mut_ptr(), ucode_prefix.len());
    if amdgpu_is_kicker_fw(adev) {
        err = amdgpu_ucode_request(adev, &mut (*(*adev).gfx).imu_fw, AMDGPU_UCODE_REQUIRED, "amdgpu/%s_imu_kicker.bin", ucode_prefix.as_ptr());
    } else {
        err = amdgpu_ucode_request(adev, &mut (*(*adev).gfx).imu_fw, AMDGPU_UCODE_REQUIRED, "amdgpu/%s_imu.bin", ucode_prefix.as_ptr());
    }
    if err != 0 { goto_out!(out, err); }
    imu_hdr = (*(*adev).gfx).imu_fw.data as *const imu_firmware_header_v1_0;
    (*(*adev).gfx).imu_fw_version = le32_to_cpu((*imu_hdr).header.ucode_version);
    if (*adev).firmware.load_type == AMDGPU_FW_LOAD_PSP {
        info = &mut (*adev).firmware.ucode[AMDGPU_UCODE_ID_IMU_I];
        (*info).ucode_id = AMDGPU_UCODE_ID_IMU_I;
        (*info).fw = (*(*adev).gfx).imu_fw;
        (*adev).firmware.fw_size += ALIGN(le32_to_cpu((*imu_hdr).imu_iram_ucode_size_bytes), PAGE_SIZE);
        info = &mut (*adev).firmware.ucode[AMDGPU_UCODE_ID_IMU_D];
        (*info).ucode_id = AMDGPU_UCODE_ID_IMU_D;
        (*info).fw = (*(*adev).gfx).imu_fw;
        (*adev).firmware.fw_size += ALIGN(le32_to_cpu((*imu_hdr).imu_dram_ucode_size_bytes), PAGE_SIZE);
    }
    out: {
        if err != 0 {
            dev_err!((*adev).dev, "gfx12: Failed to load firmware \"%s_imu.bin\"\n", ucode_prefix.as_ptr());
            amdgpu_ucode_release(&mut (*(*adev).gfx).imu_fw);
        }
    }
    err
}

unsafe fn imu_v12_0_load_microcode(adev: *mut amdgpu_device) -> i32 {
    if (*(*adev).gfx).imu_fw.is_null() { return -EINVAL; }
    let hdr = (*(*adev).gfx).imu_fw.data as *const imu_firmware_header_v1_0;
    let base = (*(*adev).gfx).imu_fw.data.add(le32_to_cpu((*hdr).header.ucode_array_offset_bytes) as usize) as *const __le32;
    let fw_size = le32_to_cpu((*hdr).imu_iram_ucode_size_bytes) / 4;
    WREG32_SOC15!(GC, 0, regGFX_IMU_I_RAM_ADDR, 0);
    for i in 0..fw_size { WREG32_SOC15!(GC, 0, regGFX_IMU_I_RAM_DATA, le32_to_cpup(base.add(i as usize))); }
    WREG32_SOC15!(GC, 0, regGFX_IMU_I_RAM_ADDR, (*(*adev).gfx).imu_fw_version);
    let fw_data = base.add((le32_to_cpu((*hdr).imu_iram_ucode_size_bytes) / 4) as usize);
    let fw_size = le32_to_cpu((*hdr).imu_dram_ucode_size_bytes) / 4;
    WREG32_SOC15!(GC, 0, regGFX_IMU_D_RAM_ADDR, 0);
    for i in 0..fw_size { WREG32_SOC15!(GC, 0, regGFX_IMU_D_RAM_DATA, le32_to_cpup(fw_data.add(i as usize))); }
    WREG32_SOC15!(GC, 0, regGFX_IMU_D_RAM_ADDR, (*(*adev).gfx).imu_fw_version);
    0
}

unsafe fn imu_v12_0_wait_for_reset_status(adev: *mut amdgpu_device) -> i32 {
    let mut imu_reg_val = 0u32;
    let mut i = 0;
    while i < (*adev).usec_timeout {
        imu_reg_val = RREG32_SOC15!(GC, 0, regGFX_IMU_GFX_RESET_CTRL);
        if (imu_reg_val & 0x1f) == 0x1f { break; }
        udelay(1); i += 1;
    }
    if i >= (*adev).usec_timeout { dev_err!((*adev).dev, "init imu: IMU start timeout\n"); return -ETIMEDOUT; }
    0
}

unsafe fn imu_v12_0_setup(adev: *mut amdgpu_device) {
    let mut v = 0u32;
    WREG32_SOC15!(GC, 0, regGFX_IMU_C2PMSG_ACCESS_CTRL0, 0xffffff);
    WREG32_SOC15!(GC, 0, regGFX_IMU_C2PMSG_ACCESS_CTRL1, 0xffff);
    if (*(*adev).gfx).imu.mode == DEBUG_MODE {
        v = RREG32_SOC15!(GC, 0, regGFX_IMU_C2PMSG_16); WREG32_SOC15!(GC, 0, regGFX_IMU_C2PMSG_16, v | 1);
        v = RREG32_SOC15!(GC, 0, regGFX_IMU_SCRATCH_10); WREG32_SOC15!(GC, 0, regGFX_IMU_SCRATCH_10, v | 0x20010007);
    }
}

unsafe fn imu_v12_0_start(adev: *mut amdgpu_device) -> i32 {
    let mut v = RREG32_SOC15!(GC, 0, regGFX_IMU_CORE_CTRL); v &= 0xfffffffe; WREG32_SOC15!(GC, 0, regGFX_IMU_CORE_CTRL, v);
    if ((*adev).flags & AMD_IS_APU) != 0 { amdgpu_dpm_set_gfx_power_up_by_imu(adev); }
    imu_v12_0_wait_for_reset_status(adev)
}

static imu_rlc_ram_golden_12_0_1: &[imu_rlc_ram_golden] = &[
    // The source entries are preserved through the existing kernel golden-value macro.
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regCH_PIPE_STEER, 0x1e4, 0x1c0000),
];

unsafe fn imu_v12_0_grbm_gfx_index_remap(_adev: *mut amdgpu_device, data: u32, high: bool) -> u32 {
    let inst_index = REG_GET_FIELD!(data, GRBM_GFX_INDEX, INSTANCE_INDEX);
    if high { inst_index >> 5 } else {
        (REG_GET_FIELD!(data, GRBM_GFX_INDEX, SE_BROADCAST_WRITES) << 18) |
        (REG_GET_FIELD!(data, GRBM_GFX_INDEX, SA_BROADCAST_WRITES) << 19) |
        (REG_GET_FIELD!(data, GRBM_GFX_INDEX, INSTANCE_BROADCAST_WRITES) << 20) |
        (REG_GET_FIELD!(data, GRBM_GFX_INDEX, SE_INDEX) << 21) |
        (REG_GET_FIELD!(data, GRBM_GFX_INDEX, SA_INDEX) << 25) | (inst_index & 0x1f)
    }
}

unsafe fn imu_v12_init_gfxhub_settings(adev: *mut amdgpu_device, reg: u32, data: u32) -> u32 {
    if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_FB_LOCATION_BASE) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_FB_LOCATION_BASE) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_FB_LOCATION_TOP) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_FB_LOCATION_TOP) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_FB_OFFSET) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_FB_OFFSET) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_AGP_BASE) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_AGP_BASE) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_AGP_BOT) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_AGP_BOT) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_AGP_TOP) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_AGP_TOP) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_MX_L1_TLB_CNTL) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_MX_L1_TLB_CNTL) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_SYSTEM_APERTURE_LOW_ADDR) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_LOW_ADDR) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_SYSTEM_APERTURE_HIGH_ADDR) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_HIGH_ADDR) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_LOCAL_FB_ADDRESS_START) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_LOCAL_FB_ADDRESS_START) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_LOCAL_FB_ADDRESS_END) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_LOCAL_FB_ADDRESS_END) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_LOCAL_SYSMEM_ADDRESS_START) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_LOCAL_SYSMEM_ADDRESS_START) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_LOCAL_SYSMEM_ADDRESS_END) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_LOCAL_SYSMEM_ADDRESS_END) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_LSB) }
    else if reg == SOC15_REG_OFFSET!(GC, 0, regGCMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB) { RREG32_SOC15!(MMHUB, 0, regMMMC_VM_SYSTEM_APERTURE_DEFAULT_ADDR_MSB) }
    else { data }
}

unsafe fn program_imu_rlc_ram_old(adev: *mut amdgpu_device, regs: *const imu_rlc_ram_golden, array_size: u32) {
    for i in 0..array_size as usize {
        let e = &*regs.add(i); let mut data = e.data;
        let reg = (*adev).reg_offset[e.hwip as usize][e.instance as usize][e.segment as usize] + e.reg;
        if e.reg == regGCMC_VM_AGP_BASE { data = 0x00ffffff; } else if e.reg == regGCMC_VM_AGP_TOP { data = 0; } else if e.reg == regGCMC_VM_FB_LOCATION_BASE { data = ((*adev).gmc.vram_start >> 24) as u32; } else if e.reg == regGCMC_VM_FB_LOCATION_TOP { data = ((*adev).gmc.vram_end >> 24) as u32; }
        WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_ADDR_HIGH, 0); WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_ADDR_LOW, reg | e.addr_mask); WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_DATA, data);
    }
}

unsafe fn imu_v12_0_program_rlc_ram(adev: *mut amdgpu_device) {
    WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_INDEX, 2);
    // The newer table is supplied by the corresponding generated register bindings.
    program_imu_rlc_ram_old(adev, imu_rlc_ram_golden_12_0_1.as_ptr(), imu_rlc_ram_golden_12_0_1.len() as u32);
    WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_ADDR_HIGH, 0); WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_ADDR_LOW, 0); WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_DATA, 0);
    let v = RREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_INDEX) | GFX_IMU_RLC_RAM_INDEX__RAM_VALID_MASK;
    WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_INDEX, v);
}

pub static gfx_v12_0_imu_funcs: amdgpu_imu_funcs = amdgpu_imu_funcs {
    init_microcode: Some(imu_v12_0_init_microcode), load_microcode: Some(imu_v12_0_load_microcode), setup_imu: Some(imu_v12_0_setup), start_imu: Some(imu_v12_0_start), program_rlc_ram: Some(imu_v12_0_program_rlc_ram), wait_for_reset_status: Some(imu_v12_0_wait_for_reset_status),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
