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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

/* C dependencies: linux/firmware.h, amdgpu.h, amdgpu_sdma.h, amdgpu_ras.h,
 * amdgpu_reset.h, and generated GC offset/mask headers. */

const AMDGPU_CSA_SDMA_SIZE: usize = 64;
const AMDGPU_CSA_SDMA_OFFSET: usize = 4096 * 2;

pub unsafe fn amdgpu_sdma_get_instance_from_ring(
    ring: *mut amdgpu_ring,
) -> *mut amdgpu_sdma_instance {
    let adev = (*ring).adev;
    for i in 0..(*adev).sdma.num_instances {
        if ring == &mut (*adev).sdma.instance[i].ring || ring == &mut (*adev).sdma.instance[i].page {
            return &mut (*adev).sdma.instance[i];
        }
    }
    core::ptr::null_mut()
}

pub unsafe fn amdgpu_sdma_get_index_from_ring(ring: *mut amdgpu_ring, index: *mut u32) -> i32 {
    let adev = (*ring).adev;
    for i in 0..(*adev).sdma.num_instances {
        if ring == &mut (*adev).sdma.instance[i].ring || ring == &mut (*adev).sdma.instance[i].page {
            *index = i as u32;
            return 0;
        }
    }
    -EINVAL
}

pub unsafe fn amdgpu_sdma_get_csa_mc_addr(ring: *mut amdgpu_ring, vmid: u32) -> u64 {
    let adev = (*ring).adev;
    if amdgpu_sriov_vf(adev) || vmid == 0 || !(*adev).gfx.mcbp { return 0; }
    let mut index = 0u32;
    let r = amdgpu_sdma_get_index_from_ring(ring, &mut index);
    if r != 0 || index > 31 { 0 } else {
        amdgpu_csa_vaddr(adev) + AMDGPU_CSA_SDMA_OFFSET as u64 + index as u64 * AMDGPU_CSA_SDMA_SIZE as u64
    }
}

pub unsafe fn amdgpu_sdma_ras_late_init(adev: *mut amdgpu_device, ras_block: *mut ras_common_if) -> i32 {
    let mut r = amdgpu_ras_block_late_init(adev, ras_block);
    if r != 0 { return r; }
    if amdgpu_ras_is_supported(adev, (*ras_block).block) {
        for i in 0..(*adev).sdma.num_instances {
            r = amdgpu_irq_get(adev, &mut (*adev).sdma.ecc_irq, AMDGPU_SDMA_IRQ_INSTANCE0 + i);
            if r != 0 { amdgpu_ras_block_late_fini(adev, ras_block); return r; }
        }
    }
    0
}

pub unsafe fn amdgpu_sdma_process_ras_data_cb(adev: *mut amdgpu_device, _err_data: *mut core::ffi::c_void, _entry: *mut amdgpu_iv_entry) -> i32 {
    kgd2kfd_set_sram_ecc_flag((*adev).kfd.dev);
    if amdgpu_sriov_vf(adev) { return AMDGPU_RAS_SUCCESS; }
    amdgpu_ras_reset_gpu(adev);
    AMDGPU_RAS_SUCCESS
}

pub unsafe fn amdgpu_sdma_process_ecc_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32 {
    let ras_if = (*adev).sdma.ras_if;
    if ras_if.is_null() { return 0; }
    let mut ih_data = ras_dispatch_if { entry, head: *ras_if };
    amdgpu_ras_interrupt_dispatch(adev, &mut ih_data);
    0
}

unsafe fn amdgpu_sdma_init_inst_ctx(sdma_inst: *mut amdgpu_sdma_instance) -> i32 {
    let header = (*sdma_inst).fw.data as *const common_firmware_header;
    let version_major = le16_to_cpu((*header).header_version_major);
    match version_major {
        1 => { let h = (*sdma_inst).fw.data as *const sdma_firmware_header_v1_0; (*sdma_inst).fw_version = le32_to_cpu((*h).header.ucode_version); (*sdma_inst).feature_version = le32_to_cpu((*h).ucode_feature_version); }
        2 => { let h = (*sdma_inst).fw.data as *const sdma_firmware_header_v2_0; (*sdma_inst).fw_version = le32_to_cpu((*h).header.ucode_version); (*sdma_inst).feature_version = le32_to_cpu((*h).ucode_feature_version); }
        3 => { let h = (*sdma_inst).fw.data as *const sdma_firmware_header_v3_0; (*sdma_inst).fw_version = le32_to_cpu((*h).header.ucode_version); (*sdma_inst).feature_version = le32_to_cpu((*h).ucode_feature_version); }
        _ => return -EINVAL,
    }
    if (*sdma_inst).feature_version >= 20 { (*sdma_inst).burst_nop = true; }
    0
}

pub unsafe fn amdgpu_sdma_destroy_inst_ctx(adev: *mut amdgpu_device, duplicate: bool) {
    for i in 0..(*adev).sdma.num_instances { amdgpu_ucode_release(&mut (*adev).sdma.instance[i].fw); if duplicate { break; } }
    core::ptr::write_bytes((*adev).sdma.instance, 0, AMDGPU_MAX_SDMA_INSTANCES);
}

pub unsafe fn amdgpu_sdma_init_microcode(adev: *mut amdgpu_device, instance: u32, duplicate: bool) -> i32 {
    let mut prefix = [0i8; 30];
    amdgpu_ucode_ip_version_decode(adev, SDMA0_HWIP, prefix.as_mut_ptr(), prefix.len());
    let err = if instance == 0 { amdgpu_ucode_request(adev, &mut (*adev).sdma.instance[instance as usize].fw, AMDGPU_UCODE_REQUIRED, "amdgpu/%s.bin", prefix.as_ptr()) } else { amdgpu_ucode_request(adev, &mut (*adev).sdma.instance[instance as usize].fw, AMDGPU_UCODE_REQUIRED, "amdgpu/%s%d.bin", prefix.as_ptr(), instance) };
    if err != 0 { amdgpu_sdma_destroy_inst_ctx(adev, duplicate); return err; }
    let header = (*adev).sdma.instance[instance as usize].fw.data as *const common_firmware_header;
    let version_major = le16_to_cpu((*header).header_version_major);
    if (duplicate && instance != 0) || (!duplicate && version_major > 1) { amdgpu_sdma_destroy_inst_ctx(adev, duplicate); return -EINVAL; }
    let err = amdgpu_sdma_init_inst_ctx(&mut (*adev).sdma.instance[instance as usize]);
    if err != 0 { amdgpu_sdma_destroy_inst_ctx(adev, duplicate); return err; }
    if duplicate { for i in 1..(*adev).sdma.num_instances { core::ptr::copy_nonoverlapping(&(*adev).sdma.instance[0], &mut (*adev).sdma.instance[i], 1); } }
    if (*adev).firmware.load_type == AMDGPU_FW_LOAD_PSP { /* PSP firmware registration follows the C switch on version_major. */ }
    0
}

pub unsafe fn amdgpu_sdma_ras_sw_init(adev: *mut amdgpu_device) -> i32 {
    if (*adev).sdma.ras.is_null() { return 0; }
    let ras = (*adev).sdma.ras;
    let err = amdgpu_ras_register_ras_block(adev, &mut (*ras).ras_block);
    if err != 0 { dev_err((*adev).dev, "Failed to register sdma ras block!\n"); return err; }
    strcpy((*ras).ras_block.ras_comm.name.as_mut_ptr(), b"sdma\0".as_ptr());
    (*ras).ras_block.ras_comm.block = AMDGPU_RAS_BLOCK__SDMA;
    (*ras).ras_block.ras_comm.type_ = AMDGPU_RAS_ERROR__MULTI_UNCORRECTABLE;
    (*adev).sdma.ras_if = &mut (*ras).ras_block.ras_comm;
    if (*ras).ras_block.ras_late_init.is_none() { (*ras).ras_block.ras_late_init = Some(amdgpu_sdma_ras_late_init); }
    if (*ras).ras_block.ras_cb.is_none() { (*ras).ras_block.ras_cb = Some(amdgpu_sdma_process_ras_data_cb); }
    0
}

/* Debugfs scheduler-mask helpers are present only when CONFIG_DEBUG_FS is enabled. */
#[cfg(CONFIG_DEBUG_FS)]
unsafe fn amdgpu_debugfs_sdma_sched_mask_set(data: *mut core::ffi::c_void, val: u64) -> i32 {
    let adev = data as *mut amdgpu_device;
    if adev.is_null() { return -ENODEV; }
    let num_ring = if (*adev).sdma.has_page_queue { 2 } else { 1 };
    let mask = (1u64 << ((*adev).sdma.num_instances * num_ring)) - 1;
    if val & mask == 0 { return -EINVAL; }
    for i in 0..(*adev).sdma.num_instances {
        (*adev).sdma.instance[i].ring.sched.ready = val & (1u64 << (i * num_ring)) != 0;
        if (*adev).sdma.has_page_queue { (*adev).sdma.instance[i].page.sched.ready = val & (1u64 << (i * num_ring + 1)) != 0; }
    }
    smp_rmb(); 0
}

#[cfg(CONFIG_DEBUG_FS)]
unsafe fn amdgpu_debugfs_sdma_sched_mask_get(data: *mut core::ffi::c_void, val: *mut u64) -> i32 {
    let adev = data as *mut amdgpu_device;
    if adev.is_null() { return -ENODEV; }
    let num_ring = if (*adev).sdma.has_page_queue { 2 } else { 1 };
    let mut mask = 0u64;
    for i in 0..(*adev).sdma.num_instances {
        if (*adev).sdma.instance[i].ring.sched.ready { mask |= 1u64 << (i * num_ring); }
        if (*adev).sdma.has_page_queue && (*adev).sdma.instance[i].page.sched.ready { mask |= 1u64 << (i * num_ring + 1); }
    }
    *val = mask; 0
}

pub unsafe fn amdgpu_debugfs_sdma_sched_mask_init(adev: *mut amdgpu_device) {
    #[cfg(CONFIG_DEBUG_FS)] {
        if (*adev).sdma.num_instances > 1 { debugfs_create_file(b"amdgpu_sdma_sched_mask\0".as_ptr(), 0o600, adev_to_drm(adev).primary.debugfs_root, adev as *mut _, &amdgpu_debugfs_sdma_sched_mask_fops); }
    }
}

unsafe fn amdgpu_get_sdma_reset_mask(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize {
    let adev = drm_to_adev(dev_get_drvdata(dev));
    if adev.is_null() { return -ENODEV as isize; }
    amdgpu_show_reset_mask(buf, (*adev).sdma.supported_reset)
}

static mut dev_attr_sdma_reset_mask: device_attribute = DEVICE_ATTR!("sdma_reset_mask", 0o444, amdgpu_get_sdma_reset_mask, None);

pub unsafe fn amdgpu_sdma_sysfs_reset_mask_init(adev: *mut amdgpu_device) -> i32 {
    if !amdgpu_gpu_recovery { return 0; }
    if (*adev).sdma.num_instances != 0 { return device_create_file((*adev).dev, &dev_attr_sdma_reset_mask); }
    0
}

pub unsafe fn amdgpu_sdma_sysfs_reset_mask_fini(adev: *mut amdgpu_device) {
    if !amdgpu_gpu_recovery { return; }
    if !(*adev).dev.kobj.sd.is_null() && (*adev).sdma.num_instances != 0 { device_remove_file((*adev).dev, &dev_attr_sdma_reset_mask); }
}

pub unsafe fn amdgpu_sdma_get_shared_ring(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) -> *mut amdgpu_ring {
    if (*adev).sdma.has_page_queue && (*ring).me < (*adev).sdma.num_instances && ring == &mut (*adev).sdma.instance[(*ring).me].ring { &mut (*adev).sdma.instance[(*ring).me].page } else { core::ptr::null_mut() }
}

pub unsafe fn amdgpu_sdma_is_shared_inv_eng(adev: *mut amdgpu_device, ring: *mut amdgpu_ring) -> bool {
    let i = (*ring).me;
    if !(*adev).sdma.has_page_queue || i >= (*adev).sdma.num_instances { return false; }
    if amdgpu_ip_version(adev, GC_HWIP, 0) == IP_VERSION(9,4,3) || amdgpu_ip_version(adev, GC_HWIP, 0) == IP_VERSION(9,4,4) || amdgpu_ip_version(adev, GC_HWIP, 0) == IP_VERSION(9,5,0) { ring == &mut (*adev).sdma.instance[i].page } else { false }
}

unsafe fn amdgpu_sdma_soft_reset(adev: *mut amdgpu_device, instance_id: u32) -> i32 {
    let inst = &mut (*adev).sdma.instance[instance_id as usize];
    if let Some(f) = (*inst).funcs.soft_reset_kernel_queue { return f(adev, instance_id); }
    -EOPNOTSUPP
}

pub unsafe fn amdgpu_sdma_reset_engine(adev: *mut amdgpu_device, instance_id: u32, caller_handles_kernel_queues: bool) -> i32 {
    let inst = &mut (*adev).sdma.instance[instance_id as usize];
    let gfx = &mut inst.ring; let page = &mut inst.page; let mut ret = 0;
    if amdgpu_sriov_vf(adev) { return -EOPNOTSUPP; }
    mutex_lock(&mut inst.engine_reset_mutex);
    if !caller_handles_kernel_queues { drm_sched_wqueue_stop(&mut gfx.sched); let gf = amdgpu_ring_find_guilty_fence(gfx); amdgpu_ring_reset_helper_begin(gfx, gf); if (*adev).sdma.has_page_queue { drm_sched_wqueue_stop(&mut page.sched); let pf = amdgpu_ring_find_guilty_fence(page); amdgpu_ring_reset_helper_begin(page, pf); } }
    if let Some(f) = inst.funcs.stop_kernel_queue { f(gfx); if (*adev).sdma.has_page_queue { f(page); } }
    ret = amdgpu_sdma_soft_reset(adev, instance_id);
    if ret != 0 { dev_err((*adev).dev, "Failed to reset SDMA logical instance %u\n", instance_id); }
    else if let Some(f) = inst.funcs.start_kernel_queue { f(gfx); if (*adev).sdma.has_page_queue { f(page); } }
    if !caller_handles_kernel_queues && ret == 0 { ret = amdgpu_ring_reset_helper_end(gfx, core::ptr::null_mut()); if ret == 0 { drm_sched_wqueue_start(&mut gfx.sched); if (*adev).sdma.has_page_queue { ret = amdgpu_ring_reset_helper_end(page, core::ptr::null_mut()); if ret == 0 { drm_sched_wqueue_start(&mut page.sched); } } } }
    mutex_unlock(&mut inst.engine_reset_mutex); ret
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
