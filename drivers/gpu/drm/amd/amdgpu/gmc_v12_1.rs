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
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// Dependencies supplied by the surrounding kernel translation.

unsafe fn gmc_v12_1_vm_fault_interrupt_state(
    adev: *mut amdgpu_device, _src: *mut amdgpu_irq_src, _type: c_uint,
    state: amdgpu_interrupt_state,
) -> c_int {
    let mut hub: *mut amdgpu_vmhub;
    let mut tmp: u32;
    let mut reg: u32;
    match state {
        AMDGPU_IRQ_STATE_DISABLE | AMDGPU_IRQ_STATE_ENABLE => {
            for_each_set_bit!(j, (*adev).vmhubs_mask, AMDGPU_MAX_VMHUBS, {
                hub = &mut (*adev).vmhub[j] as *mut _;
                for i in 0..16 {
                    reg = (*hub).vm_context0_cntl + i;
                    if (*adev).in_s0ix && j == AMDGPU_GFXHUB(0) { continue; }
                    if j >= AMDGPU_MMHUB0(0) { tmp = RREG32_SOC15_IP!(MMHUB, reg); }
                    else { tmp = RREG32_XCC!(reg, j); }
                    if state == AMDGPU_IRQ_STATE_DISABLE { tmp &= !(*hub).vm_cntx_cntl_vm_fault; }
                    else { tmp |= (*hub).vm_cntx_cntl_vm_fault; }
                    if j >= AMDGPU_MMHUB0(0) { WREG32_SOC15_IP!(MMHUB, reg, tmp); }
                    else { WREG32_XCC!(reg, tmp, j); }
                }
            });
        }
        _ => {}
    }
    0
}

unsafe fn gmc_v12_1_process_interrupt(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> c_int {
    let mut retry_fault = false;
    let mut write_fault = false;
    let node_id = (*entry).node_id;
    let mut addr = ((*entry).src_data[0] as u64) << 12;
    addr |= (((*entry).src_data[1] as u64) & 0x1fff) << 44;
    if (*entry).src_id == UTCL2_1_0__SRCID__RETRY {
        retry_fault = true;
        write_fault = ((*entry).src_data[1] & AMDGPU_GMC121_FAULT_SOURCE_DATA_WRITE) != 0;
    }
    let (hub_name, vmhub) = if (*entry).client_id == SOC_V1_0_IH_CLIENTID_VMC {
        ("mmhub0", AMDGPU_MMHUB0(node_id / 4))
    } else {
        let mut xcc_id = 0;
        if !(*adev).gfx.funcs.is_null() && (*(*adev).gfx.funcs).ih_node_to_logical_xcc.is_some() {
            xcc_id = (*(*adev).gfx.funcs).ih_node_to_logical_xcc.unwrap()(adev, node_id);
            if xcc_id < 0 { xcc_id = 0; }
        }
        ("gfxhub0", xcc_id as u32)
    };
    let hub = &mut (*adev).vmhub[vmhub] as *mut amdgpu_vmhub;
    if retry_fault {
        if (*adev).irq.retry_cam_enabled {
            if (*entry).ih == &mut (*adev).irq.ih { amdgpu_irq_delegate(adev, entry, 8); return 1; }
            let cam_index = (*entry).src_data[3] & 0x3ff;
            let ret = amdgpu_vm_handle_fault(adev, (*entry).pasid, (*entry).vmid, node_id, addr, (*entry).timestamp, write_fault);
            WDOORBELL32!( (*adev).irq.retry_cam_doorbell_index, cam_index);
            if ret != 0 { return 1; }
        } else {
            if (*entry).ih != &mut (*adev).irq.ih_soft && amdgpu_gmc_filter_faults(adev, (*entry).ih, addr, (*entry).pasid, (*entry).timestamp) != 0 { return 1; }
            if (*entry).ih == &mut (*adev).irq.ih { amdgpu_irq_delegate(adev, entry, 8); return 1; }
            if amdgpu_vm_handle_fault(adev, (*entry).pasid, (*entry).vmid, node_id, addr, (*entry).timestamp, write_fault) != 0 { return 1; }
        }
    }
    if kgd2kfd_vmfault_fast_path(adev, entry, retry_fault) != 0 { return 1; }
    if printk_ratelimit() == 0 { return 0; }
    dev_err!((*adev).dev, "[%s] %s page fault (src_id:%u ring:%u vmid:%u pasid:%u)\n", hub_name, if retry_fault { "retry" } else { "no-retry" }, (*entry).src_id, (*entry).ring_id, (*entry).vmid, (*entry).pasid);
    let task_info = amdgpu_vm_get_task_info_pasid(adev, (*entry).pasid);
    if !task_info.is_null() { amdgpu_vm_print_task_info(adev, task_info); amdgpu_vm_put_task_info(task_info); }
    dev_err!((*adev).dev, "  in page starting at address 0x%016llx from IH client %d (%s)\n", addr, (*entry).client_id, soc_v1_0_ih_clientid_name[(*entry).client_id]);
    if !(*adev).irq.ih_funcs.is_null() && (*(*adev).irq.ih_funcs).node_id_to_die_name.is_some() {
        let mut die_name_buf = [0i8; 32];
        let die_name = (*(*adev).irq.ih_funcs).node_id_to_die_name.unwrap()(adev, node_id, die_name_buf.as_mut_ptr(), die_name_buf.len());
        if !die_name.is_null() { dev_err!((*adev).dev, "  cookie node_id %d fault from die %s\n", node_id, die_name); }
    }
    if amdgpu_sriov_vf(adev) { return 0; }
    if (*entry).vmid_src == AMDGPU_GFXHUB(0) { RREG32!((*hub).vm_l2_pro_fault_status); }
    let status = RREG32!((*hub).vm_l2_pro_fault_status);
    if status == 0 { return 0; }
    WREG32_P!((*hub).vm_l2_pro_fault_cntl, 1, !1);
    amdgpu_vm_update_fault_cache(adev, (*entry).pasid, addr, status, vmhub);
    ((*hub).vmhub_funcs).as_ref().unwrap().print_l2_protection_fault_status(adev, status);
    0
}

unsafe fn gmc_v12_1_get_vmid_pasid_mapping_info(adev: *mut amdgpu_device, vmid: u8, inst: u8, p_pasid: *mut u16) -> bool {
    let index = if inst / 4 != 0 { 0xA + inst % 4 } else { 0x2 + inst % 4 };
    WREG32!(SOC15_REG_OFFSET!(OSSSYS, 0, regIH_VMID_LUT_INDEX), index);
    *p_pasid = (RREG32!(SOC15_REG_OFFSET!(OSSSYS, 0, regIH_VMID_0_LUT) + vmid as u32) & 0xffff) as u16;
    *p_pasid != 0
}

unsafe fn gmc_v12_1_use_invalidate_semaphore(adev: *mut amdgpu_device, vmhub: u32) -> bool { !AMDGPU_IS_GFXHUB(vmhub) && !amdgpu_sriov_vf(adev) }

unsafe fn gmc_v12_1_flush_vm_hub(adev: *mut amdgpu_device, vmid: u32, vmhub: u32, flush_type: u32) {
    let use_semaphore = gmc_v12_1_use_invalidate_semaphore(adev, vmhub);
    let hub = &mut (*adev).vmhub[vmhub] as *mut amdgpu_vmhub;
    let mut inv_req = ((*hub).vmhub_funcs).as_ref().unwrap().get_invalidate_req(vmid, flush_type);
    let eng = 17u32;
    let hub_ip = if AMDGPU_IS_GFXHUB(vmhub) { GC_HWIP } else { MMHUB_HWIP };
    spin_lock!((*adev).gmc.invalidate_lock);
    let mut i = 0;
    if use_semaphore { for n in 0..(*adev).usec_timeout { i = n; let tmp = RREG32_RLC_NO_KIQ!((*hub).vm_inv_eng0_sem + (*hub).eng_distance * eng, hub_ip); if tmp & 1 != 0 { break; } udelay(1); } if i >= (*adev).usec_timeout { DRM_ERROR!("Timeout waiting for sem acquire in VM flush!\n"); } }
    WREG32_RLC_NO_KIQ!((*hub).vm_inv_eng0_req + (*hub).eng_distance * eng, inv_req, hub_ip);
    for n in 0..(*adev).usec_timeout { i = n; let tmp = RREG32_RLC_NO_KIQ!((*hub).vm_inv_eng0_ack + (*hub).eng_distance * eng, hub_ip) & (1 << vmid); if tmp != 0 { break; } udelay(1); }
    if use_semaphore { WREG32_RLC_NO_KIQ!((*hub).vm_inv_eng0_sem + (*hub).eng_distance * eng, 0, hub_ip); }
    if !AMDGPU_IS_GFXHUB(vmhub) && (*hub).vm_l2_bank_select_reserved_cid2 != 0 && !amdgpu_sriov_vf(adev) { inv_req = RREG32_NO_KIQ!((*hub).vm_l2_bank_select_reserved_cid2); inv_req |= 1 << 25; WREG32_NO_KIQ!((*hub).vm_l2_bank_select_reserved_cid2, inv_req); RREG32_NO_KIQ!((*hub).vm_l2_bank_select_reserved_cid2); }
    spin_unlock!((*adev).gmc.invalidate_lock);
    if i >= (*adev).usec_timeout { dev_err!((*adev).dev, "Timeout waiting for VM flush ACK!\n"); }
}

unsafe fn gmc_v12_1_flush_gpu_tlb(adev: *mut amdgpu_device, vmid: u32, vmhub: u32, flush_type: u32) {
    if AMDGPU_IS_GFXHUB(vmhub) && !(*adev).gfx.is_poweron { return; }
    let inst = if vmhub >= AMDGPU_MMHUB0(0) { 0 } else { vmhub };
    if ((*adev).gfx.kiq[inst].ring.sched.ready || (*adev).mes.ring[MES_PIPE_INST!(inst, 0)].sched.ready) && (amdgpu_sriov_runtime(adev) || !amdgpu_sriov_vf(adev)) { let hub = &(*adev).vmhub[vmhub]; let eng = 17; let req = hub.vm_inv_eng0_req + hub.eng_distance * eng; let ack = hub.vm_inv_eng0_ack + hub.eng_distance * eng; amdgpu_gmc_fw_reg_write_reg_wait(adev, req, ack, hub.vmhub_funcs.as_ref().unwrap().get_invalidate_req(vmid, flush_type), 1 << vmid, inst); return; }
    gmc_v12_1_flush_vm_hub(adev, vmid, vmhub, 0);
}

unsafe fn gmc_v12_1_flush_gpu_tlb_pasid(adev: *mut amdgpu_device, pasid: u16, flush_type: u32, all_hub: bool, inst: u32) {
    let mut queried = 0u16;
    if (*adev).enable_uni_mes && (*adev).mes.ring[0].sched.ready && ((*adev).mes.sched_version & AMDGPU_MES_VERSION_MASK) >= 0x6f { let mut input = mes_inv_tlbs_pasid_input { xcc_id: inst, pasid, flush_type, hub_id: 0 }; if !amdgpu_gfx_is_master_xcc(adev, inst) { return; } input.hub_id = AMDGPU_GFXHUB(0); (*adev).mes.funcs.as_ref().unwrap().invalidate_tlbs_pasid(&mut (*adev).mes, &input); if all_hub { for h in [AMDGPU_MMHUB0(0), AMDGPU_MMHUB1(0)] { if test_bit!(h, (*adev).vmhubs_mask) { input.hub_id = h; (*adev).mes.funcs.as_ref().unwrap().invalidate_tlbs_pasid(&mut (*adev).mes, &input); } } } return; }
    for vmid in 1..16 { if !gmc_v12_1_get_vmid_pasid_mapping_info(adev, vmid, inst as u8, &mut queried) || queried != pasid { continue; } if all_hub { for_each_set_bit!(i, (*adev).vmhubs_mask, AMDGPU_MAX_VMHUBS, { gmc_v12_1_flush_gpu_tlb(adev, vmid, i, flush_type); }); } else { gmc_v12_1_flush_gpu_tlb(adev, vmid, AMDGPU_GFXHUB(inst), flush_type); } }
}

unsafe fn gmc_v12_1_emit_flush_gpu_tlb(ring: *mut amdgpu_ring, vmid: u32, pd_addr: u64) -> u64 {
    let use_semaphore = gmc_v12_1_use_invalidate_semaphore((*ring).adev, (*ring).vm_hub);
    let hub = &(*(*ring).adev).vmhub[(*ring).vm_hub]; let req = hub.vmhub_funcs.as_ref().unwrap().get_invalidate_req(vmid, 0); let eng = (*ring).vm_inv_eng;
    if use_semaphore { amdgpu_ring_emit_reg_wait(ring, hub.vm_inv_eng0_sem + hub.eng_distance * eng, 1, 1); }
    amdgpu_ring_emit_wreg(ring, hub.ctx0_ptb_addr_lo32 + hub.ctx_addr_distance * vmid, lower_32_bits(pd_addr));
    amdgpu_ring_emit_wreg(ring, hub.ctx0_ptb_addr_hi32 + hub.ctx_addr_distance * vmid, upper_32_bits(pd_addr));
    amdgpu_ring_emit_reg_write_reg_wait(ring, hub.vm_inv_eng0_req + hub.eng_distance * eng, hub.vm_inv_eng0_ack + hub.eng_distance * eng, req, 1 << vmid);
    if use_semaphore { amdgpu_ring_emit_wreg(ring, hub.vm_inv_eng0_sem + hub.eng_distance * eng, 0); } pd_addr
}

unsafe fn gmc_v12_1_emit_pasid_mapping(ring: *mut amdgpu_ring, vmid: u32, pasid: u32) { let reg = if (*ring).vm_hub == AMDGPU_GFXHUB(0) { SOC15_REG_OFFSET!(OSSSYS, 0, regIH_VMID_0_LUT) + vmid } else { SOC15_REG_OFFSET!(OSSSYS, 0, regIH_VMID_0_LUT_MM) + vmid }; amdgpu_ring_emit_wreg(ring, reg, pasid); }

unsafe fn gmc_v12_1_get_vm_pde(adev: *mut amdgpu_device, level: c_int, addr: *mut u64, flags: *mut u64) { if (*flags & AMDGPU_PDE_PTE_GFX12) == 0 && (*flags & AMDGPU_PTE_SYSTEM) == 0 { *addr = (*adev).vm_manager.vram_base_offset + *addr - (*adev).gmc.vram_start; } BUG_ON!(*addr & 0xFFFF00000000003F); *flags |= AMDGPU_PTE_SNOOPED; if !(*adev).gmc.translate_further { return; } if level == AMDGPU_VM_PDB1 { if (*flags & AMDGPU_PDE_PTE_GFX12) == 0 { *flags |= AMDGPU_PDE_BFS_GFX12!(0x9); } } else if level == AMDGPU_VM_PDB0 && (*flags & AMDGPU_PDE_PTE_GFX12) != 0 { *flags &= !AMDGPU_PDE_PTE_GFX12; } }

unsafe fn gmc_v12_1_get_coherence_flags(adev: *mut amdgpu_device, bo: *mut amdgpu_bo, flags: *mut u64) { let bo_adev = amdgpu_ttm_adev((*bo).tbo.bdev); let is_vram = !(*bo).tbo.resource.is_null() && (*(*bo).tbo.resource).mem_type == TTM_PL_VRAM; let coherent = (*bo).flags & (AMDGPU_GEM_CREATE_COHERENT | AMDGPU_GEM_CREATE_EXT_COHERENT) != 0; let ext_coherent = (*bo).flags & AMDGPU_GEM_CREATE_EXT_COHERENT != 0; let uncached = (*bo).flags & AMDGPU_GEM_CREATE_UNCACHED != 0; let gc_ip_version = amdgpu_ip_version(adev, GC_HWIP, 0); let mut snoop = false; let mut is_local = false; let mut mtype = MTYPE_NC; if gc_ip_version == IP_VERSION!(12, 1, 0) { let is_aid_a1 = (*adev).rev_id & 0x10 != 0; let mtype_local = if is_aid_a1 { MTYPE_RW } else { MTYPE_NC }; let mtype_remote = if is_aid_a1 { MTYPE_NC } else { MTYPE_UC }; is_local = is_vram && adev == bo_adev; snoop = true; mtype = if uncached { MTYPE_UC } else if ext_coherent { if is_local { mtype_local } else { MTYPE_UC } } else if is_local { mtype_local } else { mtype_remote }; } else if uncached || coherent { mtype = MTYPE_UC; } if mtype != MTYPE_NC { *flags = AMDGPU_PTE_MTYPE_GFX12!(*flags, mtype); } if is_local || (*adev).have_atomics_support { *flags |= AMDGPU_PTE_BUS_ATOMICS; } if snoop { *flags |= AMDGPU_PTE_SNOOPED; } }

unsafe fn gmc_v12_1_get_vm_pte(adev: *mut amdgpu_device, _vm: *mut amdgpu_vm, bo: *mut amdgpu_bo, vm_flags: u32, flags: *mut u64) { if vm_flags & AMDGPU_VM_PAGE_EXECUTABLE != 0 { *flags |= AMDGPU_PTE_EXECUTABLE; } else { *flags &= !AMDGPU_PTE_EXECUTABLE; } *flags = AMDGPU_PTE_MTYPE_GFX12!(*flags, match vm_flags & AMDGPU_VM_MTYPE_MASK { AMDGPU_VM_MTYPE_RW => MTYPE_RW, AMDGPU_VM_MTYPE_UC => MTYPE_UC, _ => MTYPE_NC }); if (*flags & AMDGPU_PTE_VALID) != 0 && !bo.is_null() { gmc_v12_1_get_coherence_flags(adev, bo, flags); } }

static gmc_v12_1_gmc_funcs: amdgpu_gmc_funcs = amdgpu_gmc_funcs { flush_gpu_tlb: Some(gmc_v12_1_flush_gpu_tlb), flush_gpu_tlb_pasid: Some(gmc_v12_1_flush_gpu_tlb_pasid), emit_flush_gpu_tlb: Some(gmc_v12_1_emit_flush_gpu_tlb), emit_pasid_mapping: Some(gmc_v12_1_emit_pasid_mapping), get_vm_pde: Some(gmc_v12_1_get_vm_pde), get_vm_pte: Some(gmc_v12_1_get_vm_pte), query_mem_partition_mode: Some(amdgpu_gmc_query_memory_partition), request_mem_partition_mode: Some(amdgpu_gmc_request_memory_partition) };
pub unsafe fn gmc_v12_1_set_gmc_funcs(adev: *mut amdgpu_device) { (*adev).gmc.gmc_funcs = &gmc_v12_1_gmc_funcs; }
static gmc_v12_1_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { set: Some(gmc_v12_1_vm_fault_interrupt_state), process: Some(gmc_v12_1_process_interrupt) };
static gmc_v12_1_ecc_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { process: Some(amdgpu_umc_uniras_process_ecc_irq) };
pub unsafe fn gmc_v12_1_set_irq_funcs(adev: *mut amdgpu_device) { (*adev).gmc.vm_fault.num_types = 1; (*adev).gmc.vm_fault.funcs = &gmc_v12_1_irq_funcs; (*adev).gmc.ecc_irq.num_types = 1; (*adev).gmc.ecc_irq.funcs = &gmc_v12_1_ecc_funcs; }
pub unsafe fn gmc_v12_1_init_vram_info(adev: *mut amdgpu_device) { (*adev).gmc.vram_type = AMDGPU_VRAM_TYPE_HBM4; (*adev).gmc.vram_width = 384 * 64; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
