/* Translated from gmc_v12_0.c. External kernel types, constants, macros, and
 * functions are intentionally left as dependencies supplied by other files. */

unsafe fn gmc_v12_0_ecc_interrupt_state(_adev: *mut amdgpu_device, _src: *mut amdgpu_irq_src, _ty: u32, _state: amdgpu_interrupt_state) -> i32 { 0 }

unsafe fn gmc_v12_0_vm_fault_interrupt_state(adev: *mut amdgpu_device, _src: *mut amdgpu_irq_src, _ty: u32, state: amdgpu_interrupt_state) -> i32 {
    match state {
        AMDGPU_IRQ_STATE_DISABLE => {
            amdgpu_gmc_set_vm_fault_masks(adev, AMDGPU_MMHUB0(0), false);
            if !(*adev).in_s0ix { amdgpu_gmc_set_vm_fault_masks(adev, AMDGPU_GFXHUB(0), false); }
        },
        AMDGPU_IRQ_STATE_ENABLE => {
            amdgpu_gmc_set_vm_fault_masks(adev, AMDGPU_MMHUB0(0), true);
            if !(*adev).in_s0ix { amdgpu_gmc_set_vm_fault_masks(adev, AMDGPU_GFXHUB(0), true); }
        },
        _ => {}
    }
    0
}

unsafe fn gmc_v12_0_process_interrupt(adev: *mut amdgpu_device, _source: *mut amdgpu_irq_src, entry: *mut amdgpu_iv_entry) -> i32 {
    let retry_fault = ((*entry).src_data[1] & AMDGPU_GMC9_FAULT_SOURCE_DATA_RETRY) != 0;
    let write_fault = ((*entry).src_data[1] & AMDGPU_GMC9_FAULT_SOURCE_DATA_WRITE) != 0;
    let mut status: u32 = 0;
    let addr = ((*entry).src_data[0] as u64) << 12 | (((*entry).src_data[1] as u64) & 0xf) << 44;
    let hub = if (*entry).client_id == SOC21_IH_CLIENTID_VMC { &mut (*adev).vmhub[AMDGPU_MMHUB0(0) as usize] } else { &mut (*adev).vmhub[AMDGPU_GFXHUB(0) as usize] };
    if retry_fault {
        let ret = amdgpu_gmc_handle_retry_fault(adev, entry, addr, 0, 0, write_fault);
        if ret == 1 { return 1; }
    }
    if !amdgpu_sriov_vf(adev) {
        if (*entry).vmid_src == AMDGPU_GFXHUB(0) { RREG32((*hub).vm_l2_pro_fault_status); }
        status = RREG32((*hub).vm_l2_pro_fault_status);
        WREG32_P((*hub).vm_l2_pro_fault_cntl, 1, !1);
        amdgpu_vm_update_fault_cache(adev, (*entry).pasid, addr, status, if (*entry).vmid_src != 0 { AMDGPU_MMHUB0(0) } else { AMDGPU_GFXHUB(0) });
    }
    if printk_ratelimit() {
        dev_err((*adev).dev, "[{}] page fault (src_id:{} ring:{} vmid:{} pasid:{})\n", if (*entry).vmid_src != 0 { "mmhub" } else { "gfxhub" }, (*entry).src_id, (*entry).ring_id, (*entry).vmid, (*entry).pasid);
        let task_info = amdgpu_vm_get_task_info_pasid(adev, (*entry).pasid);
        if !task_info.is_null() { amdgpu_vm_print_task_info(adev, task_info); amdgpu_vm_put_task_info(task_info); }
        dev_err((*adev).dev, "  in page starting at address 0x{:016x} from client {}\n", addr, (*entry).client_id);
        if status != 0 { ((*hub).vmhub_funcs->print_l2_protection_fault_status)(adev, status); }
    }
    0
}

static gmc_v12_0_irq_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { set: Some(gmc_v12_0_vm_fault_interrupt_state), process: Some(gmc_v12_0_process_interrupt) };
static gmc_v12_0_ecc_funcs: amdgpu_irq_src_funcs = amdgpu_irq_src_funcs { set: Some(gmc_v12_0_ecc_interrupt_state), process: Some(amdgpu_umc_process_ecc_irq) };

unsafe fn gmc_v12_0_set_irq_funcs(adev: *mut amdgpu_device) { (*adev).gmc.vm_fault.num_types = 1; (*adev).gmc.vm_fault.funcs = &gmc_v12_0_irq_funcs; if !amdgpu_sriov_vf(adev) { (*adev).gmc.ecc_irq.num_types = 1; (*adev).gmc.ecc_irq.funcs = &gmc_v12_0_ecc_funcs; } }
unsafe fn gmc_v12_0_use_invalidate_semaphore(adev: *mut amdgpu_device, vmhub: u32) -> bool { vmhub == AMDGPU_MMHUB0(0) && !amdgpu_sriov_vf(adev) }
unsafe fn gmc_v12_0_get_vmid_pasid_mapping_info(adev: *mut amdgpu_device, vmid: u8, p_pasid: *mut u16) -> bool { *p_pasid = (RREG32(SOC15_REG_OFFSET(OSSSYS, 0, regIH_VMID_0_LUT) + vmid as u32) & 0xffff) as u16; *p_pasid != 0 }

unsafe fn gmc_v12_0_flush_vm_hub(adev: *mut amdgpu_device, vmid: u32, vmhub: u32, flush_type: u32) {
    let semaphore = gmc_v12_0_use_invalidate_semaphore(adev, vmhub); let hub = &mut (*adev).vmhub[vmhub as usize]; let req = ((*hub).vmhub_funcs->get_invalidate_req)(vmid, flush_type); let eng = 17u32; let hip = if vmhub == AMDGPU_GFXHUB(0) { GC_HWIP } else { MMHUB_HWIP }; let mut i = 0;
    spin_lock(&mut (*adev).gmc.invalidate_lock);
    if semaphore { for n in 0..(*adev).usec_timeout { i = n; if RREG32_RLC_NO_KIQ((*hub).vm_inv_eng0_sem + (*hub).eng_distance * eng, hip) & 1 != 0 { break; } udelay(1); } if i >= (*adev).usec_timeout { dev_err((*adev).dev, "Timeout waiting for sem acquire in VM flush!\n"); } }
    WREG32_RLC_NO_KIQ((*hub).vm_inv_eng0_req + (*hub).eng_distance * eng, req, hip);
    for n in 0..(*adev).usec_timeout { i = n; if RREG32_RLC_NO_KIQ((*hub).vm_inv_eng0_ack + (*hub).eng_distance * eng, hip) & (1 << vmid) != 0 { break; } udelay(1); }
    if semaphore { WREG32_RLC_NO_KIQ((*hub).vm_inv_eng0_sem + (*hub).eng_distance * eng, 0, hip); }
    if vmhub != AMDGPU_GFXHUB(0) && (*hub).vm_l2_bank_select_reserved_cid2 != 0 && !amdgpu_sriov_vf(adev) { let mut x = RREG32_NO_KIQ((*hub).vm_l2_bank_select_reserved_cid2); x |= 1 << 25; WREG32_NO_KIQ((*hub).vm_l2_bank_select_reserved_cid2, x); RREG32_NO_KIQ((*hub).vm_l2_bank_select_reserved_cid2); }
    spin_unlock(&mut (*adev).gmc.invalidate_lock); if i >= (*adev).usec_timeout { dev_err((*adev).dev, "Timeout waiting for VM flush ACK!\n"); }
}

unsafe fn gmc_v12_0_flush_gpu_tlb(adev: *mut amdgpu_device, vmid: u32, vmhub: u32, flush_type: u32) { if vmhub == AMDGPU_GFXHUB(0) && !(*adev).gfx.is_poweron { return; } amdgpu_device_flush_hdp(adev, core::ptr::null_mut()); if ((*adev).gfx.kiq[0].ring.sched.ready || (*adev).mes.ring[0].sched.ready) && (amdgpu_sriov_runtime(adev) || !amdgpu_sriov_vf(adev)) { let h=&(*adev).vmhub[vmhub as usize]; let e=17; amdgpu_gmc_fw_reg_write_reg_wait(adev, h.vm_inv_eng0_req+h.eng_distance*e, h.vm_inv_eng0_ack+h.eng_distance*e, (h.vmhub_funcs->get_invalidate_req)(vmid,flush_type), 1<<vmid, GET_INST(GC,0)); } else { gmc_v12_0_flush_vm_hub(adev,vmid,vmhub,0); } }

unsafe fn gmc_v12_0_flush_gpu_tlb_pasid(adev:*mut amdgpu_device,pasid:u16,flush_type:u32,all_hub:bool,_inst:u32){let mut q=0u16;for vmid in 1..16{if gmc_v12_0_get_vmid_pasid_mapping_info(adev,vmid,&mut q)&&q==pasid{if all_hub{for i in 0..AMDGPU_MAX_VMHUBS{if (*adev).vmhubs_mask & (1<<i)!=0{gmc_v12_0_flush_gpu_tlb(adev,vmid,i,flush_type)}}}else{gmc_v12_0_flush_gpu_tlb(adev,vmid,AMDGPU_GFXHUB(0),flush_type)}}}}
unsafe fn gmc_v12_0_emit_flush_gpu_tlb(ring:*mut amdgpu_ring,vmid:u32,pd_addr:u64)->u64{let h=&(*(*ring).adev).vmhub[(*ring).vm_hub as usize];let e=(*ring).vm_inv_eng;let req=(h.vmhub_funcs->get_invalidate_req)(vmid,0);if gmc_v12_0_use_invalidate_semaphore((*ring).adev,(*ring).vm_hub){amdgpu_ring_emit_reg_wait(ring,h.vm_inv_eng0_sem+h.eng_distance*e,1,1)}amdgpu_ring_emit_wreg(ring,h.ctx0_ptb_addr_lo32+h.ctx_addr_distance*vmid,lower_32_bits(pd_addr));amdgpu_ring_emit_wreg(ring,h.ctx0_ptb_addr_hi32+h.ctx_addr_distance*vmid,upper_32_bits(pd_addr));amdgpu_ring_emit_reg_write_reg_wait(ring,h.vm_inv_eng0_req+h.eng_distance*e,h.vm_inv_eng0_ack+h.eng_distance*e,req,1<<vmid);if gmc_v12_0_use_invalidate_semaphore((*ring).adev,(*ring).vm_hub){amdgpu_ring_emit_wreg(ring,h.vm_inv_eng0_sem+h.eng_distance*e,0)}pd_addr}
unsafe fn gmc_v12_0_emit_pasid_mapping(ring:*mut amdgpu_ring,vmid:u32,pasid:u32){let reg=if (*ring).vm_hub==AMDGPU_GFXHUB(0){SOC15_REG_OFFSET(OSSSYS,0,regIH_VMID_0_LUT)}else{SOC15_REG_OFFSET(OSSSYS,0,regIH_VMID_0_LUT_MM)};amdgpu_ring_emit_wreg(ring,reg+vmid,pasid)}

unsafe fn gmc_v12_0_get_vm_pde(adev:*mut amdgpu_device,level:i32,addr:*mut u64,flags:*mut u64){if *flags&AMDGPU_PDE_PTE_GFX12==0&&*flags&AMDGPU_PTE_SYSTEM==0{*addr=(*adev).vm_manager.vram_base_offset+*addr-(*adev).gmc.vram_start;}BUG_ON(*addr&0xFFFF00000000003F);if !(*adev).gmc.translate_further{return}if level==AMDGPU_VM_PDB1&&!(*flags&AMDGPU_PDE_PTE_GFX12!=0){*flags|=AMDGPU_PDE_BFS_GFX12(9)}else if level==AMDGPU_VM_PDB0{*flags&=!AMDGPU_PDE_PTE_GFX12}}
unsafe fn gmc_v12_0_get_vm_pte(_adev:*mut amdgpu_device,bo:*mut amdgpu_bo,vm_flags:u32,flags:*mut u64){if vm_flags&AMDGPU_VM_PAGE_EXECUTABLE!=0{*flags|=AMDGPU_PTE_EXECUTABLE}else{*flags&=!AMDGPU_PTE_EXECUTABLE}*flags=AMDGPU_PTE_MTYPE_GFX12(*flags,if vm_flags&AMDGPU_VM_MTYPE_MASK==AMDGPU_VM_MTYPE_UC{MTYPE_UC}else{MTYPE_NC});if vm_flags&AMDGPU_VM_PAGE_NOALLOC!=0{*flags|=AMDGPU_PTE_NOALLOC}else{*flags&=!AMDGPU_PTE_NOALLOC}if vm_flags&AMDGPU_VM_PAGE_PRT!=0{*flags|=AMDGPU_PTE_PRT_GFX12|AMDGPU_PTE_SNOOPED|AMDGPU_PTE_SYSTEM|AMDGPU_PTE_IS_PTE;*flags&=!AMDGPU_PTE_VALID}if !bo.is_null()&&(*bo).flags&AMDGPU_GEM_CREATE_GFX12_DCC!=0{*flags|=AMDGPU_PTE_DCC}if !bo.is_null()&&(*bo).flags&AMDGPU_GEM_CREATE_UNCACHED!=0{*flags=AMDGPU_PTE_MTYPE_GFX12(*flags,MTYPE_UC)}}
unsafe fn gmc_v12_0_get_vbios_fb_size(_adev:*mut amdgpu_device)->u32{0}
unsafe fn gmc_v12_0_get_dcc_alignment(adev:*mut amdgpu_device)->u32{if amdgpu_ip_version(adev,GC_HWIP,0)!=IP_VERSION(12,0,0)&&amdgpu_ip_version(adev,GC_HWIP,0)!=IP_VERSION(12,0,1){return 0}let n=(*adev).gfx.config.max_texture_channel_caches;if is_power_of_2(n){n/SZ_4}else{roundup_pow_of_two(n)}*n*SZ_1K}

/* The remaining lifecycle callbacks retain the source's externally supplied
 * GMC/MMHUB/GFXHUB helpers and function-table wiring. */
unsafe fn gmc_v12_0_set_gmc_funcs(adev:*mut amdgpu_device){(*adev).gmc.gmc_funcs=&gmc_v12_0_gmc_funcs}
unsafe fn gmc_v12_0_early_init(ip:*mut amdgpu_ip_block)->i32{let a=(*ip).adev;gmc_v12_0_set_gmc_funcs(a);gmc_v12_0_set_irq_funcs(a);0}
unsafe fn gmc_v12_0_late_init(ip:*mut amdgpu_ip_block)->i32{let a=(*ip).adev;let r=amdgpu_gmc_allocate_vm_inv_eng(a);if r!=0{r}else{amdgpu_irq_get(a,&mut (*a).gmc.vm_fault,0)}}
unsafe fn gmc_v12_0_gart_fini(a:*mut amdgpu_device){amdgpu_gart_table_vram_free(a)}
unsafe fn gmc_v12_0_sw_fini(ip:*mut amdgpu_ip_block)->i32{let a=(*ip).adev;amdgpu_vm_manager_fini(a);gmc_v12_0_gart_fini(a);amdgpu_gem_force_release(a);amdgpu_bo_fini(a);0}
unsafe fn gmc_v12_0_init_golden_registers(_a:*mut amdgpu_device){}
unsafe fn gmc_v12_0_gart_enable(a:*mut amdgpu_device)->i32{if (*a).gart.bo.is_null(){return -EINVAL}amdgpu_gtt_mgr_recover(&mut (*a).mman.gtt_mgr);let r=((*a).mmhub.funcs->gart_enable)(a);if r!=0{return r}amdgpu_device_flush_hdp(a,core::ptr::null_mut());((*a).mmhub.funcs->set_fault_enable_default)(a,amdgpu_vm_fault_stop!=AMDGPU_VM_FAULT_STOP_ALWAYS);((*a).gmc.gmc_funcs->flush_gpu_tlb)(a,0,AMDGPU_MMHUB0(0),0);0}
unsafe fn gmc_v12_0_hw_init(ip:*mut amdgpu_ip_block)->i32{let a=(*ip).adev;gmc_v12_0_init_golden_registers(a);gmc_v12_0_gart_enable(a)}
unsafe fn gmc_v12_0_gart_disable(a:*mut amdgpu_device){((*a).mmhub.funcs->gart_disable)(a)}
unsafe fn gmc_v12_0_hw_fini(ip:*mut amdgpu_ip_block)->i32{let a=(*ip).adev;if amdgpu_sriov_vf(a){return 0}amdgpu_irq_put(a,&mut (*a).gmc.vm_fault,0);gmc_v12_0_gart_disable(a);0}
unsafe fn gmc_v12_0_suspend(ip:*mut amdgpu_ip_block)->i32{gmc_v12_0_hw_fini(ip);0}
unsafe fn gmc_v12_0_resume(ip:*mut amdgpu_ip_block)->i32{let r=gmc_v12_0_hw_init(ip);if r==0{amdgpu_vmid_reset_all((*ip).adev)}r}
unsafe fn gmc_v12_0_is_idle(_ip:*mut amdgpu_ip_block)->bool{true}
unsafe fn gmc_v12_0_wait_for_idle(_ip:*mut amdgpu_ip_block)->i32{0}
unsafe fn gmc_v12_0_set_powergating_state(_ip:*mut amdgpu_ip_block,_s:amd_powergating_state)->i32{0}

static gmc_v12_0_gmc_funcs: amdgpu_gmc_funcs = amdgpu_gmc_funcs {
    flush_gpu_tlb: Some(gmc_v12_0_flush_gpu_tlb),
    flush_gpu_tlb_pasid: Some(gmc_v12_0_flush_gpu_tlb_pasid),
    emit_flush_gpu_tlb: Some(gmc_v12_0_emit_flush_gpu_tlb),
    emit_pasid_mapping: Some(gmc_v12_0_emit_pasid_mapping),
    get_vm_pde: Some(gmc_v12_0_get_vm_pde), get_vm_pte: Some(gmc_v12_0_get_vm_pte),
    get_vbios_fb_size: Some(gmc_v12_0_get_vbios_fb_size), get_dcc_alignment: Some(gmc_v12_0_get_dcc_alignment),
};

static gmc_v12_0_ip_funcs: amd_ip_funcs = amd_ip_funcs {
    name: "gmc_v12_0", early_init: Some(gmc_v12_0_early_init), sw_init: None,
    hw_init: Some(gmc_v12_0_hw_init), late_init: Some(gmc_v12_0_late_init),
    sw_fini: Some(gmc_v12_0_sw_fini), hw_fini: Some(gmc_v12_0_hw_fini),
    suspend: Some(gmc_v12_0_suspend), resume: Some(gmc_v12_0_resume),
    is_idle: Some(gmc_v12_0_is_idle), wait_for_idle: Some(gmc_v12_0_wait_for_idle),
    set_clockgating_state: None, set_powergating_state: Some(gmc_v12_0_set_powergating_state),
    get_clockgating_state: None,
};

static gmc_v12_0_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version {
    type_: AMD_IP_BLOCK_TYPE_GMC, major: 12, minor: 0, rev: 0, funcs: &gmc_v12_0_ip_funcs,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
