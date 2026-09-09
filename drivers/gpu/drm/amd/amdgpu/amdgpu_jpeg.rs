/*
 * Copyright 2019 Advanced Micro Devices, Inc.
 * All Rights Reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the
 * "Software"), to deal in the Software without restriction, including
 * without limitation the rights to use, copy, modify, merge, publish,
 * distribute, sub license, and/or sell copies of the Software, and to
 * permit persons to whom the Software is furnished to do so, subject to the
 * following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NON-INFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDERS, AUTHORS AND/OR ITS SUPPLIERS BE LIABLE FOR ANY CLAIM,
 * DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR
 * OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE
 * USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding driver translation.

const JPEG_IDLE_TIMEOUT: u64 = 1000;

pub unsafe fn amdgpu_jpeg_sw_init(adev: *mut amdgpu_device) -> i32 {
    let mut r: i32;
    INIT_DELAYED_WORK(&mut (*(*adev).jpeg).idle_work, amdgpu_jpeg_idle_work_handler);
    mutex_init(&mut (*(*adev).jpeg).jpeg_pg_lock);
    atomic_set(&mut (*(*adev).jpeg).total_submission_cnt, 0);
    if (*adev).firmware.load_type == AMDGPU_FW_LOAD_PSP && ((*adev).pg_flags & AMD_PG_SUPPORT_JPEG_DPG) != 0 {
        (*(*adev).jpeg).indirect_sram = true;
    }
    for i in 0..(*(*adev).jpeg).num_jpeg_inst {
        if ((*(*adev).jpeg).harvest_config & (1u32 << i)) != 0 { continue; }
        if (*(*adev).jpeg).indirect_sram {
            r = amdgpu_bo_create_kernel(adev, 64 * 2 * 4, PAGE_SIZE,
                AMDGPU_GEM_DOMAIN_VRAM | AMDGPU_GEM_DOMAIN_GTT,
                &mut (*(*(*adev).jpeg).inst.add(i as usize)).dpg_sram_bo,
                &mut (*(*(*adev).jpeg).inst.add(i as usize)).dpg_sram_gpu_addr,
                &mut (*(*(*adev).jpeg).inst.add(i as usize)).dpg_sram_cpu_addr);
            if r != 0 { dev_err((*adev).dev, "JPEG %d (%d) failed to allocate DPG bo\n", i, r); return r; }
        }
    }
    0
}

pub unsafe fn amdgpu_jpeg_sw_fini(adev: *mut amdgpu_device) -> i32 {
    for i in 0..(*(*adev).jpeg).num_jpeg_inst {
        if ((*(*adev).jpeg).harvest_config & (1u32 << i)) != 0 { continue; }
        let inst = (*(*adev).jpeg).inst.add(i as usize);
        amdgpu_bo_free_kernel(&mut (*inst).dpg_sram_bo, &mut (*inst).dpg_sram_gpu_addr,
            &mut (*inst).dpg_sram_cpu_addr as *mut _ as *mut *mut core::ffi::c_void);
        for j in 0..(*(*adev).jpeg).num_jpeg_rings { amdgpu_ring_fini(&mut (*inst).ring_dec.add(j as usize)); }
    }
    if !(*(*adev).jpeg).reg_list.is_null() { amdgpu_jpeg_reg_dump_fini(adev); }
    mutex_destroy(&mut (*(*adev).jpeg).jpeg_pg_lock);
    0
}

pub unsafe fn amdgpu_jpeg_suspend(adev: *mut amdgpu_device) -> i32 { cancel_delayed_work_sync(&mut (*(*adev).jpeg).idle_work); 0 }
pub unsafe fn amdgpu_jpeg_resume(_adev: *mut amdgpu_device) -> i32 { 0 }

unsafe fn amdgpu_jpeg_idle_work_handler(work: *mut work_struct) {
    let adev = container_of(work, amdgpu_device, jpeg.idle_work.work);
    let mut fences = 0u32;
    for i in 0..(*(*adev).jpeg).num_jpeg_inst {
        if ((*(*adev).jpeg).harvest_config & (1u32 << i)) != 0 { continue; }
        let inst = (*(*adev).jpeg).inst.add(i as usize);
        for j in 0..(*(*adev).jpeg).num_jpeg_rings { fences += amdgpu_fence_count_emitted(&mut (*inst).ring_dec.add(j as usize)); }
    }
    if fences == 0 && atomic_read(&(*(*adev).jpeg).total_submission_cnt) == 0 {
        mutex_lock(&mut (*(*adev).jpeg).jpeg_pg_lock);
        amdgpu_device_ip_set_powergating_state(adev, AMD_IP_BLOCK_TYPE_JPEG, AMD_PG_STATE_GATE);
        mutex_unlock(&mut (*(*adev).jpeg).jpeg_pg_lock);
    } else { schedule_delayed_work(&mut (*(*adev).jpeg).idle_work, JPEG_IDLE_TIMEOUT); }
}

pub unsafe fn amdgpu_jpeg_ring_begin_use(ring: *mut amdgpu_ring) {
    let adev = (*ring).adev;
    if atomic_fetch_inc(&mut (*(*adev).jpeg).total_submission_cnt) == 0 { cancel_delayed_work_sync(&mut (*(*adev).jpeg).idle_work); }
    mutex_lock(&mut (*(*adev).jpeg).jpeg_pg_lock);
    amdgpu_device_ip_set_powergating_state(adev, AMD_IP_BLOCK_TYPE_JPEG, AMD_PG_STATE_UNGATE);
    mutex_unlock(&mut (*(*adev).jpeg).jpeg_pg_lock);
}
pub unsafe fn amdgpu_jpeg_ring_end_use(ring: *mut amdgpu_ring) {
    if atomic_dec_and_test(&mut (*(*(*ring).adev).jpeg).total_submission_cnt) { schedule_delayed_work(&mut (*(*(*ring).adev).jpeg).idle_work, JPEG_IDLE_TIMEOUT); }
}

pub unsafe fn amdgpu_jpeg_dec_ring_test_ring(ring: *mut amdgpu_ring) -> i32 {
    let adev = (*ring).adev; let mut tmp = 0u32;
    if amdgpu_sriov_vf(adev) { return 0; }
    let mut r = amdgpu_ring_alloc(ring, 3); if r != 0 { return r; }
    WREG32((*(*adev).jpeg).inst.add((*ring).me as usize).external.jpeg_pitch[(*ring).pipe as usize], 0xCAFEDEAD);
    RREG32((*(*adev).jpeg).inst.add((*ring).me as usize).external.jpeg_pitch[(*ring).pipe as usize]);
    amdgpu_ring_write(ring, PACKET0((*(*adev).jpeg).internal.jpeg_pitch[(*ring).pipe as usize], 0));
    amdgpu_ring_write(ring, 0xABADCAFE); amdgpu_ring_commit(ring);
    let mut i = 0; while i < (*adev).usec_timeout { tmp = RREG32((*(*adev).jpeg).inst.add((*ring).me as usize).external.jpeg_pitch[(*ring).pipe as usize]); if tmp == 0xABADCAFE { break; } udelay(1); i += 1; }
    if i >= (*adev).usec_timeout { r = -ETIMEDOUT; } r
}

unsafe fn amdgpu_jpeg_dec_set_reg(ring: *mut amdgpu_ring, _handle: u32, fence: *mut *mut dma_fence) -> i32 {
    let adev = (*ring).adev; let mut job: *mut amdgpu_job = core::ptr::null_mut(); let mut f: *mut dma_fence = core::ptr::null_mut();
    let mut r = amdgpu_job_alloc_with_ib(adev, core::ptr::null_mut(), core::ptr::null_mut(), 16 * 4, AMDGPU_IB_POOL_DIRECT, AMDGPU_KERNEL_JOB_ID_VCN_RING_TEST, &mut job); if r != 0 { return r; }
    let ib = &mut (*job).ibs[0]; ib.ptr[0] = PACKETJ((*(*adev).jpeg).internal.jpeg_pitch[(*ring).pipe as usize], 0, 0, PACKETJ_TYPE0); ib.ptr[1] = 0xDEADBEEF;
    let mut i = 2; while i < 16 { ib.ptr[i] = PACKETJ(0, 0, 0, PACKETJ_TYPE6); ib.ptr[i + 1] = 0; i += 2; } ib.length_dw = 16;
    r = amdgpu_job_submit_direct(job, ring, &mut f); if r != 0 { amdgpu_job_free(job); return r; }
    if !fence.is_null() { *fence = dma_fence_get(f); } dma_fence_put(f); 0
}

pub unsafe fn amdgpu_jpeg_dec_ring_test_ib(ring: *mut amdgpu_ring, timeout: i64) -> i64 {
    let adev = (*ring).adev; let mut tmp = 0u32; let mut fence: *mut dma_fence = core::ptr::null_mut();
    let mut r = amdgpu_jpeg_dec_set_reg(ring, 1, &mut fence); if r != 0 { return r as i64; }
    r = dma_fence_wait_timeout(fence, false, timeout); if r == 0 { r = -ETIMEDOUT; } else if r > 0 { r = 0; } else { dma_fence_put(fence); return r as i64; }
    if !amdgpu_sriov_vf(adev) { let mut i = 0; while i < (*adev).usec_timeout { tmp = RREG32((*(*adev).jpeg).inst.add((*ring).me as usize).external.jpeg_pitch[(*ring).pipe as usize]); if tmp == 0xDEADBEEF { break; } udelay(1); if amdgpu_emu_mode == 1 { udelay(10); } i += 1; } if i >= (*adev).usec_timeout { r = -ETIMEDOUT; } }
    dma_fence_put(fence); r as i64
}

pub unsafe fn amdgpu_jpeg_process_poison_irq(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32 {
    let ras_if = (*(*adev).jpeg).ras_if; if ras_if.is_null() { return 0; }
    let mut ih_data = ras_dispatch_if { entry, head: *ras_if }; amdgpu_ras_interrupt_dispatch(adev, &mut ih_data); 0
}

pub unsafe fn amdgpu_jpeg_ras_late_init(adev: *mut amdgpu_device, ras_block: *mut ras_common_if) -> i32 {
    let mut r = amdgpu_ras_block_late_init(adev, ras_block); if r != 0 { return r; }
    if amdgpu_ras_is_supported(adev, (*ras_block).block) { for i in 0..(*(*adev).jpeg).num_jpeg_inst { let inst = (*(*adev).jpeg).inst.add(i as usize); if ((*(*adev).jpeg).harvest_config & (1 << i)) != 0 || (*inst).ras_poison_irq.funcs.is_null() { continue; } r = amdgpu_irq_get(adev, &mut (*inst).ras_poison_irq, 0); if r != 0 { amdgpu_ras_block_late_fini(adev, ras_block); return r; } } } 0
}

pub unsafe fn amdgpu_jpeg_ras_sw_init(adev: *mut amdgpu_device) -> i32 {
    let ras = (*(*adev).jpeg).ras; if ras.is_null() { return 0; }
    let err = amdgpu_ras_register_ras_block(adev, &mut (*ras).ras_block); if err != 0 { dev_err((*adev).dev, "Failed to register jpeg ras block!\n"); return err; }
    strcpy((*ras).ras_block.ras_comm.name.as_mut_ptr(), c"jpeg".as_ptr()); (*ras).ras_block.ras_comm.block = AMDGPU_RAS_BLOCK__JPEG; (*ras).ras_block.ras_comm.type_ = AMDGPU_RAS_ERROR__POISON; (*(*adev).jpeg).ras_if = &mut (*ras).ras_block.ras_comm;
    if (*ras).ras_block.ras_late_init.is_none() { (*ras).ras_block.ras_late_init = Some(amdgpu_jpeg_ras_late_init); } 0
}

pub unsafe fn amdgpu_jpeg_psp_update_sram(adev: *mut amdgpu_device, inst_idx: i32, _ucode_id: AMDGPU_UCODE_ID) -> i32 {
    let inst = (*(*adev).jpeg).inst.add(inst_idx as usize); let ucode = amdgpu_firmware_info { ucode_id: AMDGPU_UCODE_ID_JPEG_RAM, mc_addr: (*inst).dpg_sram_gpu_addr, ucode_size: ((*inst).dpg_sram_curr_addr as usize - (*inst).dpg_sram_cpu_addr as usize) as u64 }; psp_execute_ip_fw_load(&mut (*adev).psp, &ucode)
}

pub unsafe fn amdgpu_debugfs_jpeg_sched_mask_init(adev: *mut amdgpu_device) { /* CONFIG_DEBUG_FS guarded implementation is supplied by the build. */ let _ = adev; }

unsafe fn amdgpu_get_jpeg_reset_mask(dev: *mut device, _attr: *mut device_attribute, buf: *mut i8) -> isize { let adev = drm_to_adev(dev_get_drvdata(dev)); if adev.is_null() { return -ENODEV as isize; } amdgpu_show_reset_mask(buf, (*(*adev).jpeg).supported_reset) }
pub unsafe fn amdgpu_jpeg_sysfs_reset_mask_init(adev: *mut amdgpu_device) -> i32 { if (*(*adev).jpeg).num_jpeg_inst != 0 { return device_create_file((*adev).dev, &dev_attr_jpeg_reset_mask); } 0 }
pub unsafe fn amdgpu_jpeg_sysfs_reset_mask_fini(adev: *mut amdgpu_device) { if !(*(*adev).dev).kobj.sd.is_null() && (*(*adev).jpeg).num_jpeg_inst != 0 { device_remove_file((*adev).dev, &dev_attr_jpeg_reset_mask); } }

pub unsafe fn amdgpu_jpeg_reg_dump_init(adev: *mut amdgpu_device, reg: *const amdgpu_hwip_reg_entry, count: u32) -> i32 { (*(*adev).jpeg).ip_dump = kcalloc(((*(*adev).jpeg).num_jpeg_inst * count) as usize, core::mem::size_of::<u32>(), GFP_KERNEL); if (*(*adev).jpeg).ip_dump.is_null() { dev_err((*adev).dev, "Failed to allocate memory for JPEG IP Dump\n"); return -ENOMEM; } (*(*adev).jpeg).reg_list = reg; (*(*adev).jpeg).reg_count = count; 0 }
unsafe fn amdgpu_jpeg_reg_dump_fini(adev: *mut amdgpu_device) { kfree((*(*adev).jpeg).ip_dump); (*(*adev).jpeg).reg_list = core::ptr::null(); (*(*adev).jpeg).reg_count = 0; }

pub unsafe fn amdgpu_jpeg_dump_ip_state(ip_block: *mut amdgpu_ip_block) { let adev = (*ip_block).adev; if (*(*adev).jpeg).ip_dump.is_null() { return; } for i in 0..(*(*adev).jpeg).num_jpeg_inst { if ((*(*adev).jpeg).harvest_config & (1 << i)) != 0 { continue; } let inst_id = GET_INST(JPEG, i); let off = i * (*(*adev).jpeg).reg_count; (*(*adev).jpeg).ip_dump[off as usize] = RREG32(SOC15_REG_ENTRY_OFFSET_INST((*(*adev).jpeg).reg_list.add(0), inst_id)); let powered = ((*(*adev).jpeg).ip_dump[off as usize] & 1) != 1; if powered { for j in 1..(*(*adev).jpeg).reg_count { (*(*adev).jpeg).ip_dump[(off+j) as usize] = RREG32(SOC15_REG_ENTRY_OFFSET_INST((*(*adev).jpeg).reg_list.add(j as usize), inst_id)); } } } }

pub unsafe fn amdgpu_jpeg_print_ip_state(ip_block: *mut amdgpu_ip_block, p: *mut drm_printer) { let adev = (*ip_block).adev; if (*(*adev).jpeg).ip_dump.is_null() { return; } drm_printf(p, "num_instances:%d\n", (*(*adev).jpeg).num_jpeg_inst); for i in 0..(*(*adev).jpeg).num_jpeg_inst { if ((*(*adev).jpeg).harvest_config & (1 << i)) != 0 { drm_printf(p, "\nHarvested Instance:JPEG%d Skipping dump\n", i); continue; } let off = i * (*(*adev).jpeg).reg_count; let powered = ((*(*adev).jpeg).ip_dump[off as usize] & 1) != 1; if powered { drm_printf(p, "Active Instance:JPEG%d\n", i); for j in 0..(*(*adev).jpeg).reg_count { drm_printf(p, "%-50s \t 0x%08x\n", (*(*adev).jpeg).reg_list.add(j as usize).as_ref().unwrap().reg_name, (*(*adev).jpeg).ip_dump[(off+j) as usize]); } } else { drm_printf(p, "\nInactive Instance:JPEG%d\n", i); } } }

#[inline] unsafe fn amdgpu_jpeg_reg_valid(reg: u32) -> bool { !(reg < JPEG_REG_RANGE_START || reg > JPEG_REG_RANGE_END || (reg >= JPEG_ATOMIC_RANGE_START && reg <= JPEG_ATOMIC_RANGE_END)) }

pub unsafe fn amdgpu_jpeg_dec_parse_cs(parser: *mut amdgpu_cs_parser, _job: *mut amdgpu_job, ib: *mut amdgpu_ib) -> i32 { let adev = (*parser).adev; let mut i = 0; while i < (*ib).length_dw { let packet = (*ib).ptr[i as usize]; let reg = CP_PACKETJ_GET_REG(packet); let res = CP_PACKETJ_GET_RES(packet); let cond = CP_PACKETJ_GET_COND(packet); let typ = CP_PACKETJ_GET_TYPE(packet); if res != 0 { return -EINVAL; } match typ { PACKETJ_TYPE0 | PACKETJ_TYPE3 => { let expected = if typ == PACKETJ_TYPE0 { PACKETJ_CONDITION_CHECK0 } else { PACKETJ_CONDITION_CHECK3 }; if cond != expected || !amdgpu_jpeg_reg_valid(reg) { dev_err((*adev).dev, "Invalid packet [0x%08x]!\n", packet); return -EINVAL; } }, PACKETJ_TYPE6 => { if packet == CP_PACKETJ_NOP { i += 2; continue; } dev_err((*adev).dev, "Invalid packet [0x%08x]!\n", packet); return -EINVAL; }, _ => { dev_err((*adev).dev, "Unknown packet type %d !\n", typ); return -EINVAL; } } i += 2; } 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
