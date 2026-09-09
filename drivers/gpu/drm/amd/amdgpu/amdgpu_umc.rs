/*
 * Copyright 2019 Advanced Micro Devices, Inc.
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

// Dependencies are supplied by the surrounding kernel/amdgpu translation.
pub const MAX_UMC_POISON_POLLING_TIME_SYNC: u32 = 20;
pub const MAX_UMC_HASH_STRING_SIZE: usize = 256;

unsafe fn amdgpu_umc_convert_error_address(adev: *mut amdgpu_device, err_data: *mut ras_err_data, err_addr: u64, ch_inst: u32, umc_inst: u32) -> i32 {
    match amdgpu_ip_version(adev, UMC_HWIP, 0) {
        IP_VERSION_6_7_0 => { umc_v6_7_convert_error_address(adev, err_data, err_addr, ch_inst, umc_inst); }
        _ => { dev_warn((*adev).dev, "UMC address to Physical address translation is not supported\n"); return AMDGPU_RAS_FAIL; }
    }
    AMDGPU_RAS_SUCCESS
}

pub unsafe fn amdgpu_umc_page_retirement_mca(adev: *mut amdgpu_device, err_addr: u64, ch_inst: u32, umc_inst: u32) -> i32 {
    let mut err_data: ras_err_data = core::mem::zeroed();
    let mut ret = amdgpu_ras_error_data_init(&mut err_data);
    if ret != 0 { return ret; }
    err_data.err_addr = kzalloc_objs_eeprom_table_record((*adev).umc.max_ras_err_cnt_per_query);
    if err_data.err_addr.is_null() { dev_warn((*adev).dev, "Failed to alloc memory for umc error record in MCA notifier!\n"); ret = AMDGPU_RAS_FAIL; } else {
        err_data.err_addr_len = (*adev).umc.max_ras_err_cnt_per_query;
        ret = amdgpu_umc_convert_error_address(adev, &mut err_data, err_addr, ch_inst, umc_inst);
        if ret == 0 && amdgpu_bad_page_threshold != 0 { amdgpu_ras_add_bad_pages(adev, err_data.err_addr, err_data.err_addr_cnt, false); amdgpu_ras_save_bad_pages(adev, core::ptr::null_mut()); }
        kfree(err_data.err_addr);
        amdgpu_ras_error_data_fini(&mut err_data);
        return ret;
    }
    amdgpu_ras_error_data_fini(&mut err_data);
    ret
}

pub unsafe fn amdgpu_umc_handle_bad_pages(adev: *mut amdgpu_device, ras_error_status: *mut core::ffi::c_void) {
    let err_data = ras_error_status as *mut ras_err_data;
    let con = amdgpu_ras_get_context(adev);
    let mut error_query_mode: u32 = 0;
    let mut ret: i32 = 0;
    amdgpu_ras_get_error_query_mode(adev, &mut error_query_mode);
    (*err_data).err_addr = kzalloc_objs_eeprom_table_record((*adev).umc.max_ras_err_cnt_per_query);
    if (*err_data).err_addr.is_null() { dev_warn((*adev).dev, "Failed to alloc memory for umc error address record!\n"); } else { (*err_data).err_addr_len = (*adev).umc.max_ras_err_cnt_per_query; }
    mutex_lock(&mut (*con).page_retirement_lock);
    ret = amdgpu_dpm_get_ecc_info(adev, &mut (*con).umc_ecc as *mut _ as *mut core::ffi::c_void);
    if ret == -EOPNOTSUPP && error_query_mode == AMDGPU_RAS_DIRECT_ERROR_QUERY {
        if !(*adev).umc.ras.is_null() && !(*(*adev).umc.ras).ras_block.hw_ops.is_null() && !(*(*(*adev).umc.ras).ras_block.hw_ops).query_ras_error_count.is_none() { ((*(*(*adev).umc.ras).ras_block.hw_ops).query_ras_error_count.unwrap())(adev, ras_error_status); }
        if !(*adev).umc.ras.is_null() && !(*(*adev).umc.ras).ras_block.hw_ops.is_null() && !(*(*(*adev).umc.ras).ras_block.hw_ops).query_ras_error_address.is_none() && (*adev).umc.max_ras_err_cnt_per_query != 0 { (*(*err_data)).err_addr = kzalloc_objs_eeprom_table_record((*adev).umc.max_ras_err_cnt_per_query); if (*err_data).err_addr.is_null() { dev_warn((*adev).dev, "Failed to alloc memory for umc error address record!\n"); } else { (*err_data).err_addr_len = (*adev).umc.max_ras_err_cnt_per_query; } ((*(*(*adev).umc.ras).ras_block.hw_ops).query_ras_error_address.unwrap())(adev, ras_error_status); }
    } else if error_query_mode == AMDGPU_RAS_FIRMWARE_ERROR_QUERY || (ret == 0 && error_query_mode == AMDGPU_RAS_DIRECT_ERROR_QUERY) {
        if !(*adev).umc.ras.is_null() && !(*(*adev).umc.ras).ecc_info_query_ras_error_count.is_none() { ((*(*adev).umc.ras).ecc_info_query_ras_error_count.unwrap())(adev, ras_error_status); }
        if !(*adev).umc.ras.is_null() && !(*(*adev).umc.ras).ecc_info_query_ras_error_address.is_none() && (*adev).umc.max_ras_err_cnt_per_query != 0 { (*err_data).err_addr = kcalloc((*adev).umc.max_ras_err_cnt_per_query, core::mem::size_of::<eeprom_table_record>(), GFP_KERNEL); if (*err_data).err_addr.is_null() { dev_warn((*adev).dev, "Failed to alloc memory for umc error address record!\n"); } else { (*err_data).err_addr_len = (*adev).umc.max_ras_err_cnt_per_query; } ((*(*adev).umc.ras).ecc_info_query_ras_error_address.unwrap())(adev, ras_error_status); }
    }
    if (*err_data).ue_count != 0 || (*err_data).de_count != 0 { let err_count = (*err_data).ue_count + (*err_data).de_count; if amdgpu_bad_page_threshold != 0 && (*err_data).err_addr_cnt != 0 { amdgpu_ras_add_bad_pages(adev, (*err_data).err_addr, (*err_data).err_addr_cnt, false); amdgpu_ras_save_bad_pages(adev, &err_count); amdgpu_dpm_send_hbm_bad_pages_num(adev, (*con).eeprom_control.ras_num_bad_pages); if (*con).update_channel_flag { amdgpu_dpm_send_hbm_bad_channel_flag(adev, (*con).eeprom_control.bad_channel_bitmap); (*con).update_channel_flag = false; } } }
    kfree((*err_data).err_addr); (*err_data).err_addr = core::ptr::null_mut(); mutex_unlock(&mut (*con).page_retirement_lock);
}

unsafe fn amdgpu_umc_do_page_retirement(adev: *mut amdgpu_device, ras_error_status: *mut core::ffi::c_void, _entry: *mut amdgpu_iv_entry, reset: u32) -> i32 {
    let err_data = ras_error_status as *mut ras_err_data; let con = amdgpu_ras_get_context(adev); kgd2kfd_set_sram_ecc_flag((*adev).kfd.dev); amdgpu_umc_handle_bad_pages(adev, ras_error_status);
    if ((*err_data).ue_count != 0 || (*err_data).de_count != 0) && (reset != 0 || amdgpu_ras_is_rma(adev)) { (*con).gpu_reset_flags |= reset; amdgpu_ras_reset_gpu(adev); } AMDGPU_RAS_SUCCESS
}

pub unsafe fn amdgpu_umc_pasid_poison_handler(adev: *mut amdgpu_device, block: amdgpu_ras_block, pasid: u16, pasid_fn: pasid_notify, data: *mut core::ffi::c_void, reset: u32) -> i32 {
    if (*adev).gmc.xgmi.connected_to_cpu || (*adev).gmc.is_app_apu { if reset != 0 { kgd2kfd_set_sram_ecc_flag((*adev).kfd.dev); amdgpu_ras_reset_gpu(adev); } return AMDGPU_RAS_SUCCESS; }
    if !amdgpu_sriov_vf(adev) { if amdgpu_ip_version(adev, UMC_HWIP, 0) < IP_VERSION_12_0_0 { let mut ed: ras_err_data = core::mem::zeroed(); let mut head = ras_common_if { block: AMDGPU_RAS_BLOCK__UMC }; let obj = amdgpu_ras_find_obj(adev, &mut head); let ret = amdgpu_ras_error_data_init(&mut ed); if ret != 0 { return ret; } let ret = amdgpu_umc_do_page_retirement(adev, &mut ed as *mut _ as *mut _, core::ptr::null_mut(), reset); if ret == AMDGPU_RAS_SUCCESS && !obj.is_null() { (*obj).err_data.ue_count += ed.ue_count; (*obj).err_data.ce_count += ed.ce_count; } amdgpu_ras_error_data_fini(&mut ed); return ret; } let mut ih = ras_ih_info::default(); ih.block = block; ih.pasid = pasid; ih.reset = reset; ih.pasid_fn = pasid_fn; ih.data = data; amdgpu_ras_mgr_handle_consumer_interrupt(adev, &mut ih); } else if !(*adev).virt.ops.is_null() && !(*(*adev).virt.ops).ras_poison_handler.is_none() { ((*(*adev).virt.ops).ras_poison_handler.unwrap())(adev, block); } else { dev_warn((*adev).dev, "No ras_poison_handler interface in SRIOV!\n"); } AMDGPU_RAS_SUCCESS
}

pub unsafe fn amdgpu_umc_poison_handler(adev: *mut amdgpu_device, block: amdgpu_ras_block, reset: u32) -> i32 { amdgpu_umc_pasid_poison_handler(adev, block, 0, None, core::ptr::null_mut(), reset) }
pub unsafe fn amdgpu_umc_process_ras_data_cb(adev: *mut amdgpu_device, status: *mut core::ffi::c_void, entry: *mut amdgpu_iv_entry) -> i32 { amdgpu_umc_do_page_retirement(adev, status, entry, AMDGPU_RAS_GPU_RESET_MODE1_RESET) }

pub unsafe fn amdgpu_umc_fill_error_record(ed: *mut ras_err_data, err_addr: u64, retired_page: u64, channel_index: u32, umc_inst: u32) -> i32 { if ed.is_null() || (*ed).err_addr.is_null() || (*ed).err_addr_cnt >= (*ed).err_addr_len { return -EINVAL; } let rec = (*ed).err_addr.add((*ed).err_addr_cnt as usize); (*rec).address = err_addr; (*rec).retired_page = retired_page >> AMDGPU_GPU_PAGE_SHIFT; (*rec).ts = ktime_get_real_seconds() as u64; (*rec).err_type = AMDGPU_RAS_EEPROM_ERR_NON_RECOVERABLE; (*rec).cu = 0; (*rec).mem_channel = channel_index; (*rec).mcumc_id = umc_inst; (*ed).err_addr_cnt += 1; 0 }

// The remaining initialization, IRQ, and channel-loop routines retain their C
// interfaces; loop macros and externally-defined structures are supplied by
// the surrounding translation.
pub unsafe fn amdgpu_umc_ras_sw_init(adev: *mut amdgpu_device) -> i32 { let ras = (*adev).umc.ras; if ras.is_null() { return 0; } let err = amdgpu_ras_register_ras_block(adev, &mut (*ras).ras_block); if err != 0 { dev_err((*adev).dev, "Failed to register umc ras block!\n"); return err; } (*ras).ras_block.ras_comm.name = *b"umc\0"; (*ras).ras_block.ras_comm.block = AMDGPU_RAS_BLOCK__UMC; (*ras).ras_block.ras_comm.type_ = AMDGPU_RAS_ERROR__MULTI_UNCORRECTABLE; (*adev).umc.ras_if = &mut (*ras).ras_block.ras_comm; if (*ras).ras_block.ras_late_init.is_none() { (*ras).ras_block.ras_late_init = Some(amdgpu_umc_ras_late_init); } if (*ras).ras_block.ras_cb.is_none() { (*ras).ras_block.ras_cb = Some(amdgpu_umc_process_ras_data_cb); } 0 }

pub unsafe fn amdgpu_umc_ras_late_init(adev: *mut amdgpu_device, ras_block: *mut ras_common_if) -> i32 { let mut r = amdgpu_ras_block_late_init(adev, ras_block); if r != 0 { return r; } if amdgpu_sriov_vf(adev) { return r; } if amdgpu_ras_is_supported(adev, (*ras_block).block) { r = amdgpu_irq_get(adev, &mut (*adev).gmc.ecc_irq, 0); if r != 0 { amdgpu_ras_block_late_fini(adev, ras_block); return r; } } if !(*adev).umc.ras.is_null() && !(*(*adev).umc.ras).err_cnt_init.is_none() { ((*(*adev).umc.ras).err_cnt_init.unwrap())(adev); } 0 }

pub unsafe fn amdgpu_umc_process_ecc_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32 { let ras_if = (*adev).umc.ras_if; if ras_if.is_null() { return 0; } let mut ih = ras_dispatch_if { entry, head: *ras_if }; amdgpu_ras_interrupt_dispatch(adev, &mut ih); 0 }
pub unsafe fn amdgpu_umc_uniras_process_ecc_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, _entry: *mut amdgpu_iv_entry) -> i32 { let mut ih = ras_ih_info::default(); ih.block = RAS_BLOCK_ID__UMC; amdgpu_ras_mgr_dispatch_interrupt(adev, &mut ih); 0 }

unsafe fn amdgpu_umc_loop_all_aid(adev: *mut amdgpu_device, func: umc_func, data: *mut core::ffi::c_void) -> i32 {
    // Equivalent of for_each_set_bit over active_mask, with LOOP_UMC_CH_INST
    // supplied by the UMC version-specific headers.
    let mut umc_node_inst = 0u32;
    while umc_node_inst < (*adev).umc.node_inst_num * (*adev).umc.umc_inst_num {
        if ((*adev).umc.active_mask & (1u64 << umc_node_inst)) != 0 {
            let node_inst = umc_node_inst / (*adev).umc.umc_inst_num;
            let umc_inst = umc_node_inst % (*adev).umc.umc_inst_num;
            let mut ch_inst = 0u32;
            while ch_inst < UMC_CHANNEL_INSTANCE_COUNT {
                let ret = func(adev, node_inst, umc_inst, ch_inst, data);
                if ret != 0 { dev_err((*adev).dev, "Node %d umc %d ch %d func returns %d\n", node_inst, umc_inst, ch_inst, ret); return ret; }
                ch_inst += 1;
            }
        }
        umc_node_inst += 1;
    }
    0
}

pub unsafe fn amdgpu_umc_loop_channels(adev: *mut amdgpu_device, func: umc_func, data: *mut core::ffi::c_void) -> i32 {
    if (*adev).aid_mask != 0 { return amdgpu_umc_loop_all_aid(adev, func, data); }
    let mut node_inst = 0u32;
    let mut umc_inst = 0u32;
    let mut ch_inst = 0u32;
    if (*adev).umc.node_inst_num != 0 {
        while node_inst < (*adev).umc.node_inst_num {
            umc_inst = 0;
            while umc_inst < (*adev).umc.umc_inst_num {
                ch_inst = 0;
                while ch_inst < UMC_CHANNEL_INSTANCE_COUNT { let ret = func(adev, node_inst, umc_inst, ch_inst, data); if ret != 0 { dev_err((*adev).dev, "Node %d umc %d ch %d func returns %d\n", node_inst, umc_inst, ch_inst, ret); return ret; } ch_inst += 1; }
                umc_inst += 1;
            }
            node_inst += 1;
        }
    } else {
        while umc_inst < (*adev).umc.umc_inst_num { ch_inst = 0; while ch_inst < UMC_CHANNEL_INSTANCE_COUNT { let ret = func(adev, 0, umc_inst, ch_inst, data); if ret != 0 { dev_err((*adev).dev, "Umc %d ch %d func returns %d\n", umc_inst, ch_inst, ret); return ret; } ch_inst += 1; } umc_inst += 1; }
    }
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
