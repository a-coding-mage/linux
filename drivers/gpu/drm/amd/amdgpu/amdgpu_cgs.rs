/*
 * Copyright 2015 Advanced Micro Devices, Inc.
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

// Linux/DRM headers and symbols are supplied by external dependencies.

#[repr(C)]
pub struct amdgpu_cgs_device {
    pub base: cgs_device,
    pub adev: *mut amdgpu_device,
}

unsafe fn amdgpu_cgs_read_register(cgs_device: *mut cgs_device, offset: c_uint) -> u32 {
    let adev = (*(cgs_device as *mut amdgpu_cgs_device)).adev;
    RREG32(adev, offset)
}

unsafe fn amdgpu_cgs_write_register(cgs_device: *mut cgs_device, offset: c_uint, value: u32) {
    let adev = (*(cgs_device as *mut amdgpu_cgs_device)).adev;
    WREG32(adev, offset, value);
}

unsafe fn amdgpu_cgs_read_ind_register(
    cgs_device: *mut cgs_device,
    space: cgs_ind_reg,
    index: c_uint,
) -> u32 {
    let adev = (*(cgs_device as *mut amdgpu_cgs_device)).adev;
    match space {
        CGS_IND_REG__PCIE => RREG32_PCIE(adev, index),
        CGS_IND_REG__SMC => RREG32_SMC(adev, index),
        CGS_IND_REG__UVD_CTX => RREG32_UVD_CTX(adev, index),
        CGS_IND_REG__DIDT => RREG32_DIDT(adev, index),
        CGS_IND_REG_GC_CAC => RREG32_GC_CAC(adev, index),
        CGS_IND_REG_SE_CAC => RREG32_SE_CAC(adev, index),
        CGS_IND_REG__AUDIO_ENDPT => {
            DRM_ERROR!("audio endpt register access not implemented.\n");
            0
        }
        _ => {
            WARN!(1, "Invalid indirect register space");
            0
        }
    }
}

unsafe fn amdgpu_cgs_write_ind_register(
    cgs_device: *mut cgs_device,
    space: cgs_ind_reg,
    index: c_uint,
    value: u32,
) {
    let adev = (*(cgs_device as *mut amdgpu_cgs_device)).adev;
    match space {
        CGS_IND_REG__PCIE => WREG32_PCIE(adev, index, value),
        CGS_IND_REG__SMC => WREG32_SMC(adev, index, value),
        CGS_IND_REG__UVD_CTX => WREG32_UVD_CTX(adev, index, value),
        CGS_IND_REG__DIDT => WREG32_DIDT(adev, index, value),
        CGS_IND_REG_GC_CAC => WREG32_GC_CAC(adev, index, value),
        CGS_IND_REG_SE_CAC => WREG32_SE_CAC(adev, index, value),
        CGS_IND_REG__AUDIO_ENDPT => DRM_ERROR!("audio endpt register access not implemented.\n"),
        _ => WARN!(1, "Invalid indirect register space"),
    }
}

unsafe fn fw_type_convert(cgs_device: *mut cgs_device, fw_type: u32) -> AMDGPU_UCODE_ID {
    let adev = (*(cgs_device as *mut amdgpu_cgs_device)).adev;
    let mut result = AMDGPU_UCODE_ID_MAXIMUM;
    match fw_type {
        CGS_UCODE_ID_SDMA0 => result = AMDGPU_UCODE_ID_SDMA0,
        CGS_UCODE_ID_SDMA1 => result = AMDGPU_UCODE_ID_SDMA1,
        CGS_UCODE_ID_CP_CE => result = AMDGPU_UCODE_ID_CP_CE,
        CGS_UCODE_ID_CP_PFP => result = AMDGPU_UCODE_ID_CP_PFP,
        CGS_UCODE_ID_CP_ME => result = AMDGPU_UCODE_ID_CP_ME,
        CGS_UCODE_ID_CP_MEC | CGS_UCODE_ID_CP_MEC_JT1 => result = AMDGPU_UCODE_ID_CP_MEC1,
        CGS_UCODE_ID_CP_MEC_JT2 => {
            if (*adev).asic_type >= CHIP_TOPAZ { result = AMDGPU_UCODE_ID_CP_MEC1; }
            else { result = AMDGPU_UCODE_ID_CP_MEC2; }
        }
        CGS_UCODE_ID_RLC_G => result = AMDGPU_UCODE_ID_RLC_G,
        CGS_UCODE_ID_STORAGE => result = AMDGPU_UCODE_ID_STORAGE,
        _ => DRM_ERROR!("Firmware type not supported\n"),
    }
    result
}

unsafe fn amdgpu_get_firmware_version(cgs_device: *mut cgs_device, kind: cgs_ucode_id) -> u16 {
    let adev = (*(cgs_device as *mut amdgpu_cgs_device)).adev;
    match kind {
        CGS_UCODE_ID_SDMA0 => (*adev).sdma.instance[0].fw_version,
        CGS_UCODE_ID_SDMA1 => (*adev).sdma.instance[1].fw_version,
        CGS_UCODE_ID_CP_CE => (*adev).gfx.ce_fw_version,
        CGS_UCODE_ID_CP_PFP => (*adev).gfx.pfp_fw_version,
        CGS_UCODE_ID_CP_ME => (*adev).gfx.me_fw_version,
        CGS_UCODE_ID_CP_MEC | CGS_UCODE_ID_CP_MEC_JT1 | CGS_UCODE_ID_CP_MEC_JT2 => (*adev).gfx.mec_fw_version,
        CGS_UCODE_ID_RLC_G => (*adev).gfx.rlc_fw_version,
        CGS_UCODE_ID_STORAGE => 0,
        _ => { DRM_ERROR!("firmware type %d do not have version\n", kind); 0 }
    }
}

unsafe fn amdgpu_cgs_get_firmware_info(
    cgs_device: *mut cgs_device, type_: cgs_ucode_id, info: *mut cgs_firmware_info,
) -> c_int {
    let adev = (*(cgs_device as *mut amdgpu_cgs_device)).adev;
    if type_ != CGS_UCODE_ID_SMU && type_ != CGS_UCODE_ID_SMU_SK {
        let id = fw_type_convert(cgs_device, type_);
        if id >= AMDGPU_UCODE_ID_MAXIMUM { return -EINVAL; }
        let ucode = &mut (*adev).firmware.ucode[id as usize];
        if ucode.fw.is_null() { return -EINVAL; }
        let header = ucode.fw as *const gfx_firmware_header_v1_0;
        let mut gpu_addr = ucode.mc_addr;
        let mut data_size = le32_to_cpu((*header).header.ucode_size_bytes);
        if type_ == CGS_UCODE_ID_CP_MEC_JT1 || type_ == CGS_UCODE_ID_CP_MEC_JT2 {
            gpu_addr += ALIGN!(data_size, PAGE_SIZE);
            data_size = le32_to_cpu((*header).jt_size) << 2;
        }
        (*info).kptr = ucode.kaddr;
        (*info).image_size = data_size;
        (*info).mc_addr = gpu_addr;
        (*info).version = le32_to_cpu((*header).header.ucode_version) as u16;
        if type_ == CGS_UCODE_ID_CP_MEC { (*info).image_size = le32_to_cpu((*header).jt_offset) << 2; }
        (*info).fw_version = amdgpu_get_firmware_version(cgs_device, type_);
        (*info).feature_version = le32_to_cpu((*header).ucode_feature_version) as u16;
    } else {
        // SMC firmware-name selection and request logic are preserved below.
        let mut fw_name: *const c_char = core::ptr::null();
        if (*adev).pm.fw.is_null() {
            fw_name = match (*adev).asic_type {
                CHIP_BONAIRE => if (*adev).pdev.revision == 0x80 || (*adev).pdev.revision == 0x81 || (*adev).pdev.device == 0x665f { (*info).is_kicker = true; cstr!("bonaire_k_smc.bin") } else { cstr!("bonaire_smc.bin") },
                CHIP_HAWAII => if (*adev).pdev.revision == 0x80 { (*info).is_kicker = true; cstr!("hawaii_k_smc.bin") } else { cstr!("hawaii_smc.bin") },
                CHIP_TOPAZ => if ((*adev).pdev.device == 0x6900 && ((*adev).pdev.revision == 0x81 || (*adev).pdev.revision == 0x83 || (*adev).pdev.revision == 0xd1 || (*adev).pdev.revision == 0xd3)) || ((*adev).pdev.device == 0x6907 && (*adev).pdev.revision == 0x87) { (*info).is_kicker = true; cstr!("topaz_k_smc.bin") } else { cstr!("topaz_smc.bin") },
                CHIP_TONGA => if ((*adev).pdev.device == 0x6939 || (*adev).pdev.device == 0x6938) && (*adev).pdev.revision == 0xf1 { (*info).is_kicker = true; cstr!("tonga_k_smc.bin") } else { cstr!("tonga_smc.bin") },
                CHIP_FIJI => cstr!("fiji_smc.bin"), CHIP_POLARIS11 => if type_ == CGS_UCODE_ID_SMU_SK { cstr!("polaris11_smc_sk.bin") } else if ASICID_IS_P21!((*adev).pdev.device, (*adev).pdev.revision) { (*info).is_kicker = true; cstr!("polaris11_k_smc.bin") } else if ASICID_IS_P31!((*adev).pdev.device, (*adev).pdev.revision) { (*info).is_kicker = true; cstr!("polaris11_k2_smc.bin") } else { cstr!("polaris11_smc.bin") },
                CHIP_POLARIS10 => if type_ == CGS_UCODE_ID_SMU_SK { cstr!("polaris10_smc_sk.bin") } else if ASICID_IS_P20!((*adev).pdev.device, (*adev).pdev.revision) { (*info).is_kicker = true; cstr!("polaris10_k_smc.bin") } else if ASICID_IS_P30!((*adev).pdev.device, (*adev).pdev.revision) { (*info).is_kicker = true; cstr!("polaris10_k2_smc.bin") } else { cstr!("polaris10_smc.bin") },
                CHIP_POLARIS12 => if ASICID_IS_P23!((*adev).pdev.device, (*adev).pdev.revision) { (*info).is_kicker = true; cstr!("polaris12_k_smc.bin") } else { cstr!("polaris12_smc.bin") },
                CHIP_VEGAM => cstr!("vegam_smc.bin"), CHIP_VEGA10 => if (*adev).pdev.device == 0x687f && ((*adev).pdev.revision == 0xc0 || (*adev).pdev.revision == 0xc1 || (*adev).pdev.revision == 0xc3) { cstr!("vega10_acg_smc.bin") } else { cstr!("vega10_smc.bin") },
                CHIP_VEGA12 => cstr!("vega12_smc.bin"), CHIP_VEGA20 => cstr!("vega20_smc.bin"),
                _ => { drm_err!(adev_to_drm(adev), "SMC firmware not supported\n"); return -EINVAL; }
            };
            let err = amdgpu_ucode_request(adev, &mut (*adev).pm.fw, AMDGPU_UCODE_REQUIRED, cstr!("amdgpu/%s"), fw_name);
            if err != 0 { amdgpu_ucode_release(&mut (*adev).pm.fw); return err; }
        }
        let hdr = (*adev).pm.fw.data as *const smc_firmware_header_v1_0;
        amdgpu_ucode_print_smc_hdr(&(*hdr).header);
        (*adev).pm.fw_version = le32_to_cpu((*hdr).header.ucode_version);
        (*info).version = (*adev).pm.fw_version;
        (*info).image_size = le32_to_cpu((*hdr).header.ucode_size_bytes);
        (*info).ucode_start_address = le32_to_cpu((*hdr).ucode_start_addr);
        (*info).kptr = (*adev).pm.fw.data.add(le32_to_cpu((*hdr).header.ucode_array_offset_bytes) as usize) as *mut c_void;
    }
    0
}

static amdgpu_cgs_ops: cgs_ops = cgs_ops {
    read_register: Some(amdgpu_cgs_read_register), write_register: Some(amdgpu_cgs_write_register),
    read_ind_register: Some(amdgpu_cgs_read_ind_register), write_ind_register: Some(amdgpu_cgs_write_ind_register),
    get_firmware_info: Some(amdgpu_cgs_get_firmware_info),
};

pub unsafe fn amdgpu_cgs_create_device(adev: *mut amdgpu_device) -> *mut cgs_device {
    let cgs_device = kmalloc_obj::<amdgpu_cgs_device>();
    if cgs_device.is_null() { drm_err!(adev_to_drm(adev), "Couldn't allocate CGS device structure\n"); return core::ptr::null_mut(); }
    (*cgs_device).base.ops = &amdgpu_cgs_ops;
    (*cgs_device).adev = adev;
    cgs_device as *mut cgs_device
}

pub unsafe fn amdgpu_cgs_destroy_device(cgs_device: *mut cgs_device) { kfree(cgs_device as *mut c_void); }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
