/* Rust translation of imu_v11_0.c. External kernel and AMDGPU symbols are
 * intentionally left as dependencies supplied by the surrounding tree. */

// MODULE_FIRMWARE declarations from the C source are build metadata.

unsafe fn imu_v11_0_init_microcode(adev: *mut amdgpu_device) -> i32 {
    let mut ucode_prefix = [0i8; 30];
    let mut err: i32;
    let imu_hdr: *const imu_firmware_header_v1_0;
    let mut info: *mut amdgpu_firmware_info = core::ptr::null_mut();
    DRM_DEBUG!("\n");
    amdgpu_ucode_ip_version_decode(adev, GC_HWIP, ucode_prefix.as_mut_ptr(), ucode_prefix.len());
    if amdgpu_is_kicker_fw(adev) {
        err = amdgpu_ucode_request(adev, &mut (*(*adev).gfx).imu_fw, AMDGPU_UCODE_REQUIRED,
            "amdgpu/%s_imu_kicker.bin", ucode_prefix.as_ptr());
    } else {
        err = amdgpu_ucode_request(adev, &mut (*(*adev).gfx).imu_fw, AMDGPU_UCODE_REQUIRED,
            "amdgpu/%s_imu.bin", ucode_prefix.as_ptr());
    }
    if err != 0 { goto_out!(out); }
    imu_hdr = (*(*adev).gfx).imu_fw.data as *const imu_firmware_header_v1_0;
    if (*adev).firmware.load_type == AMDGPU_FW_LOAD_PSP {
        info = &mut (*adev).firmware.ucode[AMDGPU_UCODE_ID_IMU_I];
        (*info).ucode_id = AMDGPU_UCODE_ID_IMU_I; (*info).fw = (*adev).gfx.imu_fw;
        (*adev).firmware.fw_size += ALIGN(le32_to_cpu((*imu_hdr).imu_iram_ucode_size_bytes), PAGE_SIZE);
        info = &mut (*adev).firmware.ucode[AMDGPU_UCODE_ID_IMU_D];
        (*info).ucode_id = AMDGPU_UCODE_ID_IMU_D; (*info).fw = (*adev).gfx.imu_fw;
        (*adev).firmware.fw_size += ALIGN(le32_to_cpu((*imu_hdr).imu_dram_ucode_size_bytes), PAGE_SIZE);
    } else { (*adev).gfx.imu_fw_version = le32_to_cpu((*imu_hdr).header.ucode_version); }
out:
    if err != 0 { dev_err!((*adev).dev, "gfx11: Failed to load firmware \"%s_imu.bin\"\n", ucode_prefix.as_ptr()); amdgpu_ucode_release(&mut (*(*adev).gfx).imu_fw); }
    err
}

unsafe fn imu_v11_0_load_microcode(adev: *mut amdgpu_device) -> i32 {
    if (*adev).gfx.imu_fw.is_null() { return -EINVAL; }
    let hdr = (*adev).gfx.imu_fw.data as *const imu_firmware_header_v1_0;
    let base = (*adev).gfx.imu_fw.data.add(le32_to_cpu((*hdr).header.ucode_array_offset_bytes) as usize);
    let mut fw_data = base as *const __le32;
    let fw_size = le32_to_cpu((*hdr).imu_iram_ucode_size_bytes) / 4;
    WREG32_SOC15!(GC, 0, regGFX_IMU_I_RAM_ADDR, 0);
    for _ in 0..fw_size { WREG32_SOC15!(GC, 0, regGFX_IMU_I_RAM_DATA, le32_to_cpup(fw_data)); fw_data = fw_data.add(1); }
    WREG32_SOC15!(GC, 0, regGFX_IMU_I_RAM_ADDR, (*adev).gfx.imu_fw_version);
    fw_data = base.add(le32_to_cpu((*hdr).imu_iram_ucode_size_bytes) as usize) as *const __le32;
    let fw_size = le32_to_cpu((*hdr).imu_dram_ucode_size_bytes) / 4;
    WREG32_SOC15!(GC, 0, regGFX_IMU_D_RAM_ADDR, 0);
    for _ in 0..fw_size { WREG32_SOC15!(GC, 0, regGFX_IMU_D_RAM_DATA, le32_to_cpup(fw_data)); fw_data = fw_data.add(1); }
    WREG32_SOC15!(GC, 0, regGFX_IMU_D_RAM_ADDR, (*adev).gfx.imu_fw_version); 0
}

unsafe fn imu_v11_0_wait_for_reset_status(adev: *mut amdgpu_device) -> i32 {
    let mut imu_reg_val = 0;
    let mut i = 0;
    while i < (*adev).usec_timeout { imu_reg_val = RREG32_SOC15!(GC, 0, regGFX_IMU_GFX_RESET_CTRL); if imu_reg_val & 0x1f == 0x1f { break; } udelay(1); i += 1; }
    if i >= (*adev).usec_timeout { dev_err!((*adev).dev, "init imu: IMU start timeout\n"); return -ETIMEDOUT; } 0
}

unsafe fn imu_v11_0_setup(adev: *mut amdgpu_device) {
    let mut v;
    WREG32_SOC15!(GC, 0, regGFX_IMU_C2PMSG_ACCESS_CTRL0, 0xffffff); WREG32_SOC15!(GC, 0, regGFX_IMU_C2PMSG_ACCESS_CTRL1, 0xffff);
    if (*adev).gfx.imu.mode == DEBUG_MODE { v = RREG32_SOC15!(GC, 0, regGFX_IMU_C2PMSG_16); v |= 1; WREG32_SOC15!(GC, 0, regGFX_IMU_C2PMSG_16, v); }
    v = RREG32_SOC15!(GC, 0, regGFX_IMU_SCRATCH_10); v |= 0x10007; WREG32_SOC15!(GC, 0, regGFX_IMU_SCRATCH_10, v);
}

unsafe fn imu_v11_0_start(adev: *mut amdgpu_device) -> i32 {
    let mut v = RREG32_SOC15!(GC, 0, regGFX_IMU_CORE_CTRL); v &= 0xfffffffe; WREG32_SOC15!(GC, 0, regGFX_IMU_CORE_CTRL, v);
    if (*adev).flags & AMD_IS_APU != 0 { amdgpu_dpm_set_gfx_power_up_by_imu(adev); } imu_v11_0_wait_for_reset_status(adev)
}

// Golden register tables are represented using the source macro and external
// register constants; entries are kept in source order.
static imu_rlc_ram_golden_11: &[imu_rlc_ram_golden] = &[
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regGUS_IO_RD_COMBINE_FLUSH, 0x00055555, 0xe0000000),
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regGUS_IO_WR_COMBINE_FLUSH, 0x00055555, 0xe0000000),
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regGUS_DRAM_COMBINE_FLUSH, 0x00555555, 0xe0000000),
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regGUS_MISC2, 0x00001ffe, 0xe0000000),
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regGUS_SDP_CREDITS, 0x003f3fff, 0xe0000000),
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regGUS_SDP_ENABLE, 0x00000001, 0xe0000000),
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regGCEA_SDP_ENABLE, 0x00000001, 0xe0000000),
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regCC_GC_SA_UNIT_DISABLE, 0x00fffc01, 0xe0000000),
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regCC_GC_PRIM_CONFIG, 0x000fffe1, 0xe0000000),
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regCC_RB_BACKEND_DISABLE, 0x0fffff01, 0xe0000000),
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regCC_GC_SHADER_ARRAY_CONFIG, 0xfffe0001, 0xe0000000),
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regCPC_PSP_DEBUG, CPC_PSP_DEBUG__GPA_OVERRIDE_MASK, 0),
    IMU_RLC_RAM_GOLDEN_VALUE!(GC, 0, regCPG_PSP_DEBUG, CPG_PSP_DEBUG__GPA_OVERRIDE_MASK, 0),
];
static imu_rlc_ram_golden_11_0_2: &[imu_rlc_ram_golden] = imu_rlc_ram_golden_11;

unsafe fn program_imu_rlc_ram(adev: *mut amdgpu_device, regs: *const imu_rlc_ram_golden, array_size: u32) {
    for i in 0..array_size as isize { let e = &*regs.offset(i); let mut reg = (*adev).reg_offset[e.hwip][e.instance][e.segment] + e.reg | e.addr_mask; let mut data = e.data;
        if e.reg == regGCMC_VM_AGP_BASE { data = 0x00ffffff; } else if e.reg == regGCMC_VM_AGP_TOP { data = 0; } else if e.reg == regGCMC_VM_FB_LOCATION_BASE { data = (*adev).gmc.vram_start >> 24; } else if e.reg == regGCMC_VM_FB_LOCATION_TOP { data = (*adev).gmc.vram_end >> 24; }
        WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_ADDR_HIGH, 0); WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_ADDR_LOW, reg); WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_DATA, data);
    } WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_ADDR_HIGH, 0); WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_ADDR_LOW, 0); WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_DATA, 0);
}

unsafe fn imu_v11_0_program_rlc_ram(adev: *mut amdgpu_device) { WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_INDEX, 2); match amdgpu_ip_version(adev, GC_HWIP, 0) { IP_VERSION!(11,0,0) => program_imu_rlc_ram(adev, imu_rlc_ram_golden_11.as_ptr(), imu_rlc_ram_golden_11.len() as u32), IP_VERSION!(11,0,2) => program_imu_rlc_ram(adev, imu_rlc_ram_golden_11_0_2.as_ptr(), imu_rlc_ram_golden_11_0_2.len() as u32), IP_VERSION!(11,0,3) => imu_v11_0_3_program_rlc_ram(adev), _ => { WARN!(1, "Invalid GFX/IMU IP version 0x%08x\n", amdgpu_ip_version(adev, GC_HWIP, 0)); return; } } let mut v = RREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_INDEX); v |= GFX_IMU_RLC_RAM_INDEX__RAM_VALID_MASK; WREG32_SOC15!(GC, 0, regGFX_IMU_RLC_RAM_INDEX, v); }

static gfx_v11_0_imu_funcs: amdgpu_imu_funcs = amdgpu_imu_funcs { init_microcode: Some(imu_v11_0_init_microcode), load_microcode: Some(imu_v11_0_load_microcode), setup_imu: Some(imu_v11_0_setup), start_imu: Some(imu_v11_0_start), program_rlc_ram: Some(imu_v11_0_program_rlc_ram), wait_for_reset_status: Some(imu_v11_0_wait_for_reset_status) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
