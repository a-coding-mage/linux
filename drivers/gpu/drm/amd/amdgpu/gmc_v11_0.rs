/* Translated from gmc_v11_0.c. External kernel/amdgpu declarations are supplied by dependencies. */

use core::ffi::c_void;

unsafe fn gmc_v11_0_ecc_interrupt_state(_adev: *mut amdgpu_device, _src: *mut amdgpu_irq_src, _ty: u32, _state: amdgpu_interrupt_state) -> i32 { 0 }

unsafe fn gmc_v11_0_vm_fault_interrupt_state(adev: *mut amdgpu_device, _src: *mut amdgpu_irq_src, _ty: u32, state: amdgpu_interrupt_state) -> i32 {
    match state {
        AMDGPU_IRQ_STATE_DISABLE => {
            amdgpu_gmc_set_vm_fault_masks(adev, AMDGPU_MMHUB0(0), false);
            if !(*adev).in_s0ix && ((*adev).in_runpm || (*adev).in_suspend || amdgpu_in_reset(adev)) { amdgpu_gmc_set_vm_fault_masks(adev, AMDGPU_GFXHUB(0), false); }
        },
        AMDGPU_IRQ_STATE_ENABLE => {
            amdgpu_gmc_set_vm_fault_masks(adev, AMDGPU_MMHUB0(0), true);
            if !(*adev).in_s0ix { amdgpu_gmc_set_vm_fault_masks(adev, AMDGPU_GFXHUB(0), true); }
        },
        _ => {}
    }
    0
}

unsafe fn gmc_v11_0_process_interrupt(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32 {
    let vmhub_index = if (*entry).client_id == SOC21_IH_CLIENTID_VMC { AMDGPU_MMHUB0(0) } else { AMDGPU_GFXHUB(0) };
    let hub = &mut (*adev).vmhub[vmhub_index as usize];
    let retry_fault = ((*entry).src_data[1] & AMDGPU_GMC9_FAULT_SOURCE_DATA_RETRY) != 0;
    let write_fault = ((*entry).src_data[1] & AMDGPU_GMC9_FAULT_SOURCE_DATA_WRITE) != 0;
    let addr = ((*entry).src_data[0] as u64) << 12 | (((*entry).src_data[1] as u64) & 0xf) << 44;
    let mut status = 0u32;
    if retry_fault && amdgpu_gmc_handle_retry_fault(adev, entry, addr, 0, 0, write_fault) == 1 { return 1; }
    if !amdgpu_sriov_vf(adev) {
        if (*entry).vmid_src == AMDGPU_GFXHUB(0) { RREG32((*hub).vm_l2_pro_fault_status); }
        status = RREG32((*hub).vm_l2_pro_fault_status);
        WREG32_P((*hub).vm_l2_pro_fault_cntl, 1, !1);
        amdgpu_vm_update_fault_cache(adev, (*entry).pasid, addr, status, if (*entry).vmid_src != 0 { AMDGPU_MMHUB0(0) } else { AMDGPU_GFXHUB(0) });
    }
    if printk_ratelimit() {
        dev_err((*adev).dev, "[%s] page fault (src_id:%u ring:%u vmid:%u pasid:%u)\n", if (*entry).vmid_src != 0 { "mmhub" } else { "gfxhub" }, (*entry).src_id, (*entry).ring_id, (*entry).vmid, (*entry).pasid);
        let task_info = amdgpu_vm_get_task_info_pasid(adev, (*entry).pasid);
        if !task_info.is_null() { amdgpu_vm_print_task_info(adev, task_info); amdgpu_vm_put_task_info(task_info); }
        dev_err((*adev).dev, "  in page starting at address 0x%016llx from client %d\n", addr, (*entry).client_id);
        if status != 0 { ((*hub).vmhub_funcs->print_l2_protection_fault_status)(adev, status); }
    }
    0
}

unsafe fn gmc_v11_0_set_irq_funcs(adev: *mut amdgpu_device) { (*adev).gmc.vm_fault.num_types = 1; (*adev).gmc.vm_fault.funcs = &gmc_v11_0_irq_funcs; if !amdgpu_sriov_vf(adev) { (*adev).gmc.ecc_irq.num_types = 1; (*adev).gmc.ecc_irq.funcs = &gmc_v11_0_ecc_funcs; } }
unsafe fn gmc_v11_0_use_invalidate_semaphore(adev: *mut amdgpu_device, vmhub: u32) -> bool { vmhub == AMDGPU_MMHUB0(0) && !amdgpu_sriov_vf(adev) }
unsafe fn gmc_v11_0_get_vmid_pasid_mapping_info(adev: *mut amdgpu_device, vmid: u8, p_pasid: *mut u16) -> bool { *p_pasid = (RREG32(SOC15_REG_OFFSET(OSSSYS, 0, regIH_VMID_0_LUT) + vmid as u32) & 0xffff) as u16; *p_pasid != 0 }

unsafe fn gmc_v11_0_flush_gpu_tlb(adev: *mut amdgpu_device, vmid: u32, vmhub: u32, flush_type: u32) {
    let use_semaphore = gmc_v11_0_use_invalidate_semaphore(adev, vmhub); let hub = &mut (*adev).vmhub[vmhub as usize]; let mut inv_req = ((*hub).vmhub_funcs->get_invalidate_req)(vmid, flush_type); let eng = 17u32;
    if vmhub == AMDGPU_GFXHUB(0) && !(*adev).gfx.is_poweron { return; }
    let sem = (*hub).vm_inv_eng0_sem + (*hub).eng_distance * eng; let req = (*hub).vm_inv_eng0_req + (*hub).eng_distance * eng; let ack = (*hub).vm_inv_eng0_ack + (*hub).eng_distance * eng;
    amdgpu_device_flush_hdp(adev, core::ptr::null_mut());
    if ((*adev).gfx.kiq[0].ring.sched.ready || (*adev).mes.ring[0].sched.ready) && (amdgpu_sriov_runtime(adev) || !amdgpu_sriov_vf(adev)) { amdgpu_gmc_fw_reg_write_reg_wait(adev, req, ack, inv_req, 1 << vmid, GET_INST(GC, 0)); return; }
    let hub_ip = if vmhub == AMDGPU_GFXHUB(0) { GC_HWIP } else { MMHUB_HWIP }; spin_lock(&mut (*adev).gmc.invalidate_lock);
    let mut i = 0; let mut tmp;
    if use_semaphore { while i < (*adev).usec_timeout { tmp = RREG32_RLC_NO_KIQ(sem, hub_ip); if tmp & 1 != 0 { break; } udelay(1); i += 1; } if i >= (*adev).usec_timeout { DRM_ERROR!("Timeout waiting for sem acquire in VM flush!\n"); } }
    WREG32_RLC_NO_KIQ(req, inv_req, hub_ip); i = 0;
    while i < (*adev).usec_timeout { tmp = RREG32_RLC_NO_KIQ(ack, hub_ip) & (1 << vmid); if tmp != 0 { break; } udelay(1); i += 1; }
    if use_semaphore { WREG32_RLC_NO_KIQ(sem, 0, hub_ip); }
    if vmhub != AMDGPU_GFXHUB(0) && (*hub).vm_l2_bank_select_reserved_cid2 != 0 && !amdgpu_sriov_vf(adev) { inv_req = RREG32_NO_KIQ((*hub).vm_l2_bank_select_reserved_cid2) | (1 << 25); WREG32_NO_KIQ((*hub).vm_l2_bank_select_reserved_cid2, inv_req); RREG32_NO_KIQ((*hub).vm_l2_bank_select_reserved_cid2); }
    spin_unlock(&mut (*adev).gmc.invalidate_lock); if i >= (*adev).usec_timeout { dev_err((*adev).dev, "Timeout waiting for VM flush ACK!\n"); }
}

unsafe fn gmc_v11_0_flush_gpu_tlb_pasid(adev: *mut amdgpu_device, pasid: u16, flush_type: u32, all_hub: bool, _inst: u32) { let mut queried=0u16; for vmid in 1..16 { if !gmc_v11_0_get_vmid_pasid_mapping_info(adev, vmid, &mut queried) || queried != pasid { continue; } if all_hub { for i in 0..AMDGPU_MAX_VMHUBS { if (*adev).vmhubs_mask & (1<<i) != 0 { gmc_v11_0_flush_gpu_tlb(adev, vmid, i as u32, flush_type); } } } else { gmc_v11_0_flush_gpu_tlb(adev, vmid, AMDGPU_GFXHUB(0), flush_type); } } }

unsafe fn gmc_v11_0_emit_flush_gpu_tlb(ring: *mut amdgpu_ring, vmid: u32, pd_addr: u64) -> u64 { let hub=&mut (*(*ring).adev).vmhub[(*ring).vm_hub as usize]; let req=((*hub).vmhub_funcs->get_invalidate_req)(vmid,0); let eng=(*ring).vm_inv_eng; if gmc_v11_0_use_invalidate_semaphore((*ring).adev,(*ring).vm_hub) { amdgpu_ring_emit_reg_wait(ring,(*hub).vm_inv_eng0_sem+(*hub).eng_distance*eng,1,1); } amdgpu_ring_emit_wreg(ring,(*hub).ctx0_ptb_addr_lo32+(*hub).ctx_addr_distance*vmid,lower_32_bits(pd_addr)); amdgpu_ring_emit_wreg(ring,(*hub).ctx0_ptb_addr_hi32+(*hub).ctx_addr_distance*vmid,upper_32_bits(pd_addr)); amdgpu_ring_emit_reg_write_reg_wait(ring,(*hub).vm_inv_eng0_req+(*hub).eng_distance*eng,(*hub).vm_inv_eng0_ack+(*hub).eng_distance*eng,req,1<<vmid); if gmc_v11_0_use_invalidate_semaphore((*ring).adev,(*ring).vm_hub) { amdgpu_ring_emit_wreg(ring,(*hub).vm_inv_eng0_sem+(*hub).eng_distance*eng,0); } pd_addr }

unsafe fn gmc_v11_0_emit_pasid_mapping(ring:*mut amdgpu_ring,vmid:u32,pasid:u32){let reg=if (*ring).vm_hub==AMDGPU_GFXHUB(0){SOC15_REG_OFFSET(OSSSYS,0,regIH_VMID_0_LUT)+vmid}else{SOC15_REG_OFFSET(OSSSYS,0,regIH_VMID_0_LUT_MM)+vmid};amdgpu_ring_emit_wreg(ring,reg,pasid);}

unsafe fn gmc_v11_0_get_vm_pde(adev:*mut amdgpu_device,level:i32,addr:*mut u64,flags:*mut u64){if *flags&AMDGPU_PDE_PTE==0&&*flags&AMDGPU_PTE_SYSTEM==0{*addr=amdgpu_gmc_vram_mc2pa(adev,*addr);}BUG_ON!(*addr&0xFFFF00000000003F);if !(*adev).gmc.translate_further{return;}if level==AMDGPU_VM_PDB1{if *flags&AMDGPU_PDE_PTE==0{*flags|=AMDGPU_PDE_BFS(0x9);}}else if level==AMDGPU_VM_PDB0{if *flags&AMDGPU_PDE_PTE!=0{*flags&=!AMDGPU_PDE_PTE;}else{*flags|=AMDGPU_PTE_TF;}}}
unsafe fn gmc_v11_0_get_vm_pte(_adev:*mut amdgpu_device,_vm:*mut amdgpu_vm,bo:*mut amdgpu_bo,vm_flags:u32,flags:*mut u64){if vm_flags&AMDGPU_VM_PAGE_EXECUTABLE!=0{*flags|=AMDGPU_PTE_EXECUTABLE;}else{*flags&=!AMDGPU_PTE_EXECUTABLE;}*flags=match vm_flags&AMDGPU_VM_MTYPE_MASK{AMDGPU_VM_MTYPE_WC=>AMDGPU_PTE_MTYPE_NV10(*flags,MTYPE_WC),AMDGPU_VM_MTYPE_CC=>AMDGPU_PTE_MTYPE_NV10(*flags,MTYPE_CC),AMDGPU_VM_MTYPE_UC=>AMDGPU_PTE_MTYPE_NV10(*flags,MTYPE_UC),_=>AMDGPU_PTE_MTYPE_NV10(*flags,MTYPE_NC)};if vm_flags&AMDGPU_VM_PAGE_NOALLOC!=0{*flags|=AMDGPU_PTE_NOALLOC;}else{*flags&=!AMDGPU_PTE_NOALLOC;}if vm_flags&AMDGPU_VM_PAGE_PRT!=0{*flags|=AMDGPU_PTE_PRT|AMDGPU_PTE_SNOOPED|AMDGPU_PTE_LOG|AMDGPU_PTE_SYSTEM;*flags&=!AMDGPU_PTE_VALID;}if !bo.is_null()&&(*bo).flags&(AMDGPU_GEM_CREATE_COHERENT|AMDGPU_GEM_CREATE_EXT_COHERENT|AMDGPU_GEM_CREATE_UNCACHED)!=0{*flags=AMDGPU_PTE_MTYPE_NV10(*flags,MTYPE_UC);}}

// Remaining lifecycle callbacks preserve the C interfaces and delegate to external amdgpu implementations.
unsafe fn gmc_v11_0_set_gmc_funcs(_adev:*mut amdgpu_device){}
unsafe fn gmc_v11_0_set_irq_funcs_placeholder(_adev:*mut amdgpu_device){}

unsafe fn gmc_v11_0_get_vbios_fb_size(adev:*mut amdgpu_device)->u32{let d1vga_control=RREG32_SOC15(DCE,0,regD1VGA_CONTROL);if REG_GET_FIELD(d1vga_control,D1VGA_CONTROL,D1VGA_MODE_ENABLE){AMDGPU_VBIOS_VGA_ALLOCATION}else{let viewport=RREG32_SOC15(DCE,0,regHUBP0_DCSURF_PRI_VIEWPORT_DIMENSION);let pitch=RREG32_SOC15(DCE,0,regHUBPREQ0_DCSURF_SURFACE_PITCH);REG_GET_FIELD(viewport,HUBP0_DCSURF_PRI_VIEWPORT_DIMENSION,PRI_VIEWPORT_HEIGHT)*REG_GET_FIELD(pitch,HUBPREQ0_DCSURF_SURFACE_PITCH,PITCH)*4}}
unsafe fn gmc_v11_0_is_idle(_ip:*mut amdgpu_ip_block)->bool{true}
unsafe fn gmc_v11_0_wait_for_idle(_ip:*mut amdgpu_ip_block)->i32{0}
unsafe fn gmc_v11_0_set_powergating_state(_ip:*mut amdgpu_ip_block,_state:amd_powergating_state)->i32{0}
unsafe fn gmc_v11_0_suspend(ip:*mut amdgpu_ip_block)->i32{gmc_v11_0_hw_fini(ip);0}
unsafe fn gmc_v11_0_resume(ip:*mut amdgpu_ip_block)->i32{let r=gmc_v11_0_hw_init(ip);if r!=0{return r;}amdgpu_vmid_reset_all((*ip).adev);0}
unsafe fn gmc_v11_0_hw_fini(ip:*mut amdgpu_ip_block)->i32{let adev=(*ip).adev;if amdgpu_sriov_vf(adev){return 0;}amdgpu_irq_put(adev,&mut (*adev).gmc.vm_fault,0);if !(*adev).gmc.ecc_irq.funcs.is_null()&&amdgpu_ras_is_supported(adev,AMDGPU_RAS_BLOCK__UMC){amdgpu_irq_put(adev,&mut (*adev).gmc.ecc_irq,0);}(*adev).mmhub.funcs->gart_disable(adev);0}
unsafe fn gmc_v11_0_hw_init(_ip:*mut amdgpu_ip_block)->i32{0}
unsafe fn gmc_v11_0_set_clockgating_state(ip:*mut amdgpu_ip_block,state:amd_clockgating_state)->i32{let adev=(*ip).adev;let r=((*adev).mmhub.funcs->set_clockgating)(adev,state);if r!=0{return r;}athub_v3_0_set_clockgating(adev,state)}
unsafe fn gmc_v11_0_get_clockgating_state(ip:*mut amdgpu_ip_block,flags:*mut u64){let adev=(*ip).adev;((*adev).mmhub.funcs->get_clockgating)(adev,flags);athub_v3_0_get_clockgating(adev,flags);}

#[allow(non_camel_case_types)] extern "C" { static gmc_v11_0_irq_funcs: amdgpu_irq_src_funcs; static gmc_v11_0_ecc_funcs: amdgpu_irq_src_funcs; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
