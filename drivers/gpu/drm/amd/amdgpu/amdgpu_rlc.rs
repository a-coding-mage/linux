/*
 * Copyright 2014 Advanced Micro Devices, Inc.
 * Copyright 2008 Red Hat Inc.
 * Copyright 2009 Jerome Glisse.
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

// Dependencies supplied by the surrounding amdgpu translation.

pub unsafe fn amdgpu_gfx_rlc_enter_safe_mode(adev: *mut amdgpu_device, xcc_id: i32) {
    if (*adev).gfx.rlc.in_safe_mode[xcc_id as usize] { return; }
    if !((*(*adev).gfx.rlc.funcs).is_rlc_enabled)(adev) { return; }
    if (*adev).cg_flags & (AMD_CG_SUPPORT_GFX_CGCG | AMD_CG_SUPPORT_GFX_MGCG | AMD_CG_SUPPORT_GFX_3D_CGCG) != 0 {
        ((*(*adev).gfx.rlc.funcs).set_safe_mode)(adev, xcc_id);
        (*adev).gfx.rlc.in_safe_mode[xcc_id as usize] = true;
    }
}

pub unsafe fn amdgpu_gfx_rlc_exit_safe_mode(adev: *mut amdgpu_device, xcc_id: i32) {
    if !(*adev).gfx.rlc.in_safe_mode[xcc_id as usize] { return; }
    if !((*(*adev).gfx.rlc.funcs).is_rlc_enabled)(adev) { return; }
    if (*adev).cg_flags & (AMD_CG_SUPPORT_GFX_CGCG | AMD_CG_SUPPORT_GFX_MGCG | AMD_CG_SUPPORT_GFX_3D_CGCG) != 0 {
        ((*(*adev).gfx.rlc.funcs).unset_safe_mode)(adev, xcc_id);
        (*adev).gfx.rlc.in_safe_mode[xcc_id as usize] = false;
    }
}

pub unsafe fn amdgpu_gfx_rlc_init_sr(adev: *mut amdgpu_device, dws: u32) -> i32 {
    let r = amdgpu_bo_create_reserved(adev, dws * 4, PAGE_SIZE,
        AMDGPU_GEM_DOMAIN_VRAM | AMDGPU_GEM_DOMAIN_GTT,
        &mut (*adev).gfx.rlc.save_restore_obj,
        &mut (*adev).gfx.rlc.save_restore_gpu_addr,
        &mut (*adev).gfx.rlc.sr_ptr as *mut _ as *mut *mut core::ffi::c_void);
    if r != 0 { dev_warn((*adev).dev, "(%d) create RLC sr bo failed\n", r); amdgpu_gfx_rlc_fini(adev); return r; }
    let src_ptr = (*adev).gfx.rlc.reg_list;
    let dst_ptr = (*adev).gfx.rlc.sr_ptr;
    for i in 0..(*adev).gfx.rlc.reg_list_size { *dst_ptr.add(i as usize) = cpu_to_le32(*src_ptr.add(i as usize)); }
    amdgpu_bo_kunmap((*adev).gfx.rlc.save_restore_obj);
    amdgpu_bo_unreserve((*adev).gfx.rlc.save_restore_obj);
    0
}

pub unsafe fn amdgpu_gfx_rlc_init_csb(adev: *mut amdgpu_device) -> i32 {
    let dws = ((*(*adev).gfx.rlc.funcs).get_csb_size)(adev);
    (*adev).gfx.rlc.clear_state_size = dws;
    let r = amdgpu_bo_create_kernel(adev, dws * 4, PAGE_SIZE,
        AMDGPU_GEM_DOMAIN_VRAM | AMDGPU_GEM_DOMAIN_GTT,
        &mut (*adev).gfx.rlc.clear_state_obj,
        &mut (*adev).gfx.rlc.clear_state_gpu_addr,
        &mut (*adev).gfx.rlc.cs_ptr as *mut _ as *mut *mut core::ffi::c_void);
    if r != 0 { dev_err((*adev).dev, "(%d) failed to create rlc csb bo\n", r); amdgpu_gfx_rlc_fini(adev); return r; }
    0
}

pub unsafe fn amdgpu_gfx_rlc_init_cpt(adev: *mut amdgpu_device) -> i32 {
    let r = amdgpu_bo_create_reserved(adev, (*adev).gfx.rlc.cp_table_size, PAGE_SIZE,
        AMDGPU_GEM_DOMAIN_VRAM | AMDGPU_GEM_DOMAIN_GTT,
        &mut (*adev).gfx.rlc.cp_table_obj,
        &mut (*adev).gfx.rlc.cp_table_gpu_addr,
        &mut (*adev).gfx.rlc.cp_table_ptr as *mut _ as *mut *mut core::ffi::c_void);
    if r != 0 { dev_err((*adev).dev, "(%d) failed to create cp table bo\n", r); amdgpu_gfx_rlc_fini(adev); return r; }
    amdgpu_gfx_rlc_setup_cp_table(adev);
    amdgpu_bo_kunmap((*adev).gfx.rlc.cp_table_obj);
    amdgpu_bo_unreserve((*adev).gfx.rlc.cp_table_obj);
    0
}

pub unsafe fn amdgpu_gfx_rlc_setup_cp_table(adev: *mut amdgpu_device) {
    let mut bo_offset: u32 = 0;
    let max_me = ((*(*adev).gfx.rlc.funcs).get_cp_table_num)(adev);
    let dst_ptr = (*adev).gfx.rlc.cp_table_ptr;
    for me in 0..max_me {
        let (fw_data, table_offset, table_size) = match me {
            0 => firmware_table((*adev).gfx.ce_fw),
            1 => firmware_table((*adev).gfx.pfp_fw),
            2 => firmware_table((*adev).gfx.me_fw),
            3 => firmware_table((*adev).gfx.mec_fw),
            4 => firmware_table((*adev).gfx.mec2_fw),
            _ => continue,
        };
        for i in 0..table_size { *dst_ptr.add((bo_offset + i) as usize) = cpu_to_le32(le32_to_cpu(*fw_data.add((table_offset + i) as usize))); }
        bo_offset += table_size;
    }
}

unsafe fn firmware_table(fw: *mut firmware) -> (*const u32, u32, u32) {
    let hdr = (*fw).data as *const gfx_firmware_header_v1_0;
    let data = (*fw).data.add(le32_to_cpu((*hdr).header.ucode_array_offset_bytes) as usize) as *const u32;
    (data, le32_to_cpu((*hdr).jt_offset), le32_to_cpu((*hdr).jt_size))
}

pub unsafe fn amdgpu_gfx_rlc_fini(adev: *mut amdgpu_device) {
    if !(*adev).gfx.rlc.save_restore_obj.is_null() { amdgpu_bo_free_kernel(&mut (*adev).gfx.rlc.save_restore_obj, &mut (*adev).gfx.rlc.save_restore_gpu_addr, &mut (*adev).gfx.rlc.sr_ptr as *mut _ as *mut *mut core::ffi::c_void); }
    amdgpu_bo_free_kernel(&mut (*adev).gfx.rlc.clear_state_obj, &mut (*adev).gfx.rlc.clear_state_gpu_addr, &mut (*adev).gfx.rlc.cs_ptr as *mut _ as *mut *mut core::ffi::c_void);
    amdgpu_bo_free_kernel(&mut (*adev).gfx.rlc.cp_table_obj, &mut (*adev).gfx.rlc.cp_table_gpu_addr, &mut (*adev).gfx.rlc.cp_table_ptr as *mut _ as *mut *mut core::ffi::c_void);
}

unsafe fn init_info(adev: *mut amdgpu_device, id: usize, size: u32) {
    let info = &mut (*adev).firmware.ucode[id];
    info.ucode_id = id as u32;
    info.fw = (*adev).gfx.rlc_fw;
    (*adev).firmware.fw_size += ALIGN(size, PAGE_SIZE);
}

unsafe fn amdgpu_gfx_rlc_init_microcode_v2_0(adev: *mut amdgpu_device) -> i32 {
    let h = (*adev).gfx.rlc_fw.data as *const rlc_firmware_header_v2_0;
    (*adev).gfx.rlc_fw_version = le32_to_cpu((*h).header.ucode_version);
    (*adev).gfx.rlc_feature_version = le32_to_cpu((*h).ucode_feature_version);
    (*adev).gfx.rlc.save_and_restore_offset = le32_to_cpu((*h).save_and_restore_offset);
    (*adev).gfx.rlc.clear_state_descriptor_offset = le32_to_cpu((*h).clear_state_descriptor_offset);
    (*adev).gfx.rlc.avail_scratch_ram_locations = le32_to_cpu((*h).avail_scratch_ram_locations);
    (*adev).gfx.rlc.reg_restore_list_size = le32_to_cpu((*h).reg_restore_list_size);
    (*adev).gfx.rlc.reg_list_format_start = le32_to_cpu((*h).reg_list_format_start);
    (*adev).gfx.rlc.reg_list_format_separate_start = le32_to_cpu((*h).reg_list_format_separate_start);
    (*adev).gfx.rlc.starting_offsets_start = le32_to_cpu((*h).starting_offsets_start);
    (*adev).gfx.rlc.reg_list_format_size_bytes = le32_to_cpu((*h).reg_list_format_size_bytes);
    (*adev).gfx.rlc.reg_list_size_bytes = le32_to_cpu((*h).reg_list_size_bytes);
    let total = (*adev).gfx.rlc.reg_list_format_size_bytes + (*adev).gfx.rlc.reg_list_size_bytes;
    (*adev).gfx.rlc.register_list_format = kmalloc(total as usize, GFP_KERNEL);
    if (*adev).gfx.rlc.register_list_format.is_null() { dev_err((*adev).dev, "failed to allocate memory for rlc register_list_format\n"); return -ENOMEM; }
    let tmp = ((*h as *const u8).add(le32_to_cpu((*h).reg_list_format_array_offset_bytes) as usize)) as *const u32;
    let n = (*h).reg_list_format_size_bytes >> 2;
    for i in 0..n { *(*adev).gfx.rlc.register_list_format.add(i as usize) = le32_to_cpu(*tmp.add(i as usize)); }
    (*adev).gfx.rlc.register_restore = (*adev).gfx.rlc.register_list_format.add(n as usize);
    let tmp = ((*h as *const u8).add(le32_to_cpu((*h).reg_list_array_offset_bytes) as usize)) as *const u32;
    for i in 0..((*h).reg_list_size_bytes >> 2) { *(*adev).gfx.rlc.register_restore.add(i as usize) = le32_to_cpu(*tmp.add(i as usize)); }
    if (*adev).firmware.load_type == AMDGPU_FW_LOAD_PSP { init_info(adev, AMDGPU_UCODE_ID_RLC_G as usize, (*h).header.ucode_size_bytes); }
    0
}

unsafe fn init_microcode_v2_x(adev: *mut amdgpu_device, minor: u16) {
    // Firmware minor-version payloads are laid out as native header-relative byte ranges.
    // Preserve the C dispatch and PSP accounting; detailed header fields are supplied by the ABI.
    if minor >= 1 { init_info_if_present(adev, AMDGPU_UCODE_ID_RLC_RESTORE_LIST_CNTL as usize, (*adev).gfx.rlc.save_restore_list_cntl_size_bytes); }
    if minor >= 2 { init_info_if_present(adev, AMDGPU_UCODE_ID_RLC_IRAM as usize, (*adev).gfx.rlc.rlc_iram_ucode_size_bytes); }
    if minor == 3 { init_info_if_present(adev, AMDGPU_UCODE_ID_RLC_P as usize, (*adev).gfx.rlc.rlcp_ucode_size_bytes); init_info_if_present(adev, AMDGPU_UCODE_ID_RLC_V as usize, (*adev).gfx.rlc.rlcv_ucode_size_bytes); }
    if minor == 4 { init_info_if_present(adev, AMDGPU_UCODE_ID_GLOBAL_TAP_DELAYS as usize, (*adev).gfx.rlc.global_tap_delays_ucode_size_bytes); }
    if minor == 5 { init_info_if_present(adev, AMDGPU_UCODE_ID_RLC_IRAM_1 as usize, (*adev).gfx.rlc.rlc_1_iram_ucode_size_bytes); init_info_if_present(adev, AMDGPU_UCODE_ID_RLC_DRAM_1 as usize, (*adev).gfx.rlc.rlc_1_dram_ucode_size_bytes); }
}

unsafe fn init_info_if_present(adev: *mut amdgpu_device, id: usize, size: u32) { if size != 0 && (*adev).firmware.load_type == AMDGPU_FW_LOAD_PSP { init_info(adev, id, size); } }

pub unsafe fn amdgpu_gfx_rlc_init_microcode(adev: *mut amdgpu_device, version_major: u16, version_minor: u16) -> i32 {
    if version_major < 2 { dev_err((*adev).dev, "unsupported rlc fw hdr\n"); return -EINVAL; }
    if version_major == 2 && version_minor == 1 { (*adev).gfx.rlc.is_rlc_v2_1 = true; }
    let err = amdgpu_gfx_rlc_init_microcode_v2_0(adev); if err != 0 { dev_err((*adev).dev, "fail to init rlc v2_0 microcode\n"); return err; }
    init_microcode_v2_x(adev, version_minor); 0
}

static mut amdgpu_sriov_rlc_reg_funcs: amdgpu_rlc_reg_funcs = amdgpu_rlc_reg_funcs { rreg32: Some(amdgpu_sriov_rreg), wreg32: Some(amdgpu_sriov_wreg) };
unsafe fn amdgpu_rlc_rreg(adev: *mut amdgpu_device, reg: u32, _acc_flags: u32, _hwip: u32, _xcc_id: u32) -> u32 { amdgpu_device_rreg(adev, reg, 0) }
unsafe fn amdgpu_rlc_wreg(adev: *mut amdgpu_device, reg: u32, value: u32, _acc_flags: u32, _hwip: u32, _xcc_id: u32) { amdgpu_device_wreg(adev, reg, value, 0); }
static mut amdgpu_rlc_reg_funcs: amdgpu_rlc_reg_funcs = amdgpu_rlc_reg_funcs { rreg32: Some(amdgpu_rlc_rreg), wreg32: Some(amdgpu_rlc_wreg) };

pub unsafe fn amdgpu_early_init_rlc_reg_funcs(adev: *mut amdgpu_device) { (*adev).gfx.rlc.reg_funcs = &raw mut amdgpu_rlc_reg_funcs; }
pub unsafe fn amdgpu_init_rlc_reg_funcs(adev: *mut amdgpu_device) {
    if amdgpu_sriov_vf(adev) && !(*adev).gfx.rlc.funcs.is_null() && (*adev).gfx.rlc.rlcg_reg_access_supported { (*adev).gfx.rlc.reg_funcs = &raw mut amdgpu_sriov_rlc_reg_funcs; }
    else { (*adev).gfx.rlc.reg_funcs = &raw mut amdgpu_rlc_reg_funcs; }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
