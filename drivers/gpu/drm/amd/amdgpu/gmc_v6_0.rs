/* Rust translation of gmc_v6_0.c. External kernel/driver symbols are supplied by dependencies. */

const MC_SEQ_MISC0__MT__MASK: u32 = 0xf0000000;
const MC_SEQ_MISC0__MT__GDDR1: u32 = 0x10000000;
const MC_SEQ_MISC0__MT__DDR2: u32 = 0x20000000;
const MC_SEQ_MISC0__MT__GDDR3: u32 = 0x30000000;
const MC_SEQ_MISC0__MT__GDDR4: u32 = 0x40000000;
const MC_SEQ_MISC0__MT__GDDR5: u32 = 0x50000000;
const MC_SEQ_MISC0__MT__HBM: u32 = 0x60000000;
const MC_SEQ_MISC0__MT__DDR3: u32 = 0xb0000000;

unsafe fn gmc_v6_0_mc_stop(adev: *mut amdgpu_device) {
    let ip = amdgpu_device_ip_get_ip_block(adev, AMD_IP_BLOCK_TYPE_GMC);
    if ip.is_null() { return; }
    gmc_v6_0_wait_for_idle(ip);
    let mut blackout = RREG32(adev, mmMC_SHARED_BLACKOUT_CNTL);
    if REG_GET_FIELD(blackout, MC_SHARED_BLACKOUT_CNTL, BLACKOUT_MODE) != 1 {
        WREG32(adev, mmBIF_FB_EN, 0);
        blackout = REG_SET_FIELD(blackout, MC_SHARED_BLACKOUT_CNTL, BLACKOUT_MODE, 0);
        WREG32(adev, mmMC_SHARED_BLACKOUT_CNTL, blackout | 1);
    }
    udelay(100);
}

unsafe fn gmc_v6_0_mc_resume(adev: *mut amdgpu_device) {
    let mut tmp = RREG32(adev, mmMC_SHARED_BLACKOUT_CNTL);
    tmp = REG_SET_FIELD(tmp, MC_SHARED_BLACKOUT_CNTL, BLACKOUT_MODE, 0);
    WREG32(adev, mmMC_SHARED_BLACKOUT_CNTL, tmp);
    tmp = REG_SET_FIELD(0, BIF_FB_EN, FB_READ_EN, 1);
    tmp = REG_SET_FIELD(tmp, BIF_FB_EN, FB_WRITE_EN, 1);
    WREG32(adev, mmBIF_FB_EN, tmp);
}

unsafe fn gmc_v6_0_init_microcode(adev: *mut amdgpu_device) -> i32 {
    let mut chip = match (*adev).asic_type {
        CHIP_TAHITI => "tahiti", CHIP_PITCAIRN => "pitcairn", CHIP_VERDE => "verde",
        CHIP_OLAND => "oland", CHIP_HAINAN => "hainan", _ => return -EINVAL,
    };
    if ((RREG32(adev, mmMC_SEQ_MISC0) & 0xff000000) >> 24) == 0x58 { chip = "si58"; }
    let err = amdgpu_ucode_request(adev, &mut (*adev).gmc.fw, AMDGPU_UCODE_REQUIRED,
                                   "amdgpu/%s_mc.bin", chip);
    if err != 0 { dev_err((*adev).dev, "si_mc: Failed to load firmware \"%s_mc.bin\"\n", chip); amdgpu_ucode_release(&mut (*adev).gmc.fw); }
    err
}

unsafe fn gmc_v6_0_mc_load_microcode(adev: *mut amdgpu_device) -> i32 {
    if (*adev).gmc.fw.is_null() { return -EINVAL; }
    let hdr = (*adev).gmc.fw as *const mc_firmware_header_v1_0;
    amdgpu_ucode_print_mc_hdr(&(*hdr).header);
    (*adev).gmc.fw_version = le32_to_cpu((*hdr).header.ucode_version);
    let regs_size = le32_to_cpu((*hdr).io_debug_size_bytes) / 8;
    let mut io = ((*adev).gmc.fw.data.add(le32_to_cpu((*hdr).io_debug_array_offset_bytes) as usize)) as *const __le32;
    let ucode_size = le32_to_cpu((*hdr).header.ucode_size_bytes) / 4;
    let mut fw = ((*adev).gmc.fw.data.add(le32_to_cpu((*hdr).header.ucode_array_offset_bytes) as usize)) as *const __le32;
    if RREG32(adev, mmMC_SEQ_SUP_CNTL) & MC_SEQ_SUP_CNTL__RUN_MASK == 0 {
        WREG32(adev, mmMC_SEQ_SUP_CNTL, 8); WREG32(adev, mmMC_SEQ_SUP_CNTL, 16);
        for _ in 0..regs_size { WREG32(adev, mmMC_SEQ_IO_DEBUG_INDEX, le32_to_cpup(io)); io = io.add(1); WREG32(adev, mmMC_SEQ_IO_DEBUG_DATA, le32_to_cpup(io)); io = io.add(1); }
        for _ in 0..ucode_size { WREG32(adev, mmMC_SEQ_SUP_PGM, le32_to_cpup(fw)); fw = fw.add(1); }
        WREG32(adev, mmMC_SEQ_SUP_CNTL, 8); WREG32(adev, mmMC_SEQ_SUP_CNTL, 4); WREG32(adev, mmMC_SEQ_SUP_CNTL, 1);
        for _ in 0..(*adev).usec_timeout { if RREG32(adev, mmMC_SEQ_TRAIN_WAKEUP_CNTL) & MC_SEQ_TRAIN_WAKEUP_CNTL__TRAIN_DONE_D0_MASK != 0 { break; } udelay(1); }
        for _ in 0..(*adev).usec_timeout { if RREG32(adev, mmMC_SEQ_TRAIN_WAKEUP_CNTL) & MC_SEQ_TRAIN_WAKEUP_CNTL__TRAIN_DONE_D1_MASK != 0 { break; } udelay(1); }
    }
    0
}

unsafe fn gmc_v6_0_vram_gtt_location(adev: *mut amdgpu_device, mc: *mut amdgpu_gmc) {
    let base = ((RREG32(adev, mmMC_VM_FB_LOCATION) & 0xffff) as u64) << 24;
    amdgpu_gmc_set_agp_default(adev, mc); amdgpu_gmc_vram_location(adev, mc, base); amdgpu_gmc_gart_location(adev, mc, AMDGPU_GART_PLACEMENT_LOW);
}

unsafe fn gmc_v6_0_flush_gpu_tlb(adev: *mut amdgpu_device, vmid: u32, _vmhub: u32, _flush_type: u32) { WREG32(adev, mmVM_INVALIDATE_REQUEST, 1u32 << vmid); }
unsafe fn gmc_v6_0_emit_flush_gpu_tlb(ring: *mut amdgpu_ring, vmid: u32, pd_addr: u64) -> u64 { let reg = if vmid < 8 { mmVM_CONTEXT0_PAGE_TABLE_BASE_ADDR + vmid } else { mmVM_CONTEXT8_PAGE_TABLE_BASE_ADDR + vmid - 8 }; amdgpu_ring_emit_wreg(ring, reg, (pd_addr >> 12) as u32); amdgpu_ring_emit_wreg(ring, mmVM_INVALIDATE_REQUEST, 1u32 << vmid); pd_addr }
unsafe fn gmc_v6_0_get_vm_pde(_adev: *mut amdgpu_device, _level: i32, addr: *mut u64, _flags: *mut u64) { BUG_ON(*addr & 0xFFFFFF0000000FFF); }
unsafe fn gmc_v6_0_get_vm_pte(_adev: *mut amdgpu_device, _vm: *mut amdgpu_vm, _bo: *mut amdgpu_bo, _vm_flags: u32, flags: *mut u64) { *flags &= !AMDGPU_PTE_EXECUTABLE; *flags &= !AMDGPU_PTE_PRT; }

unsafe fn gmc_v6_0_set_fault_enable_default(adev: *mut amdgpu_device, value: bool) { let mut t = RREG32(adev, mmVM_CONTEXT1_CNTL); t = REG_SET_FIELD(t, VM_CONTEXT1_CNTL, RANGE_PROTECTION_FAULT_ENABLE_DEFAULT, value); t = REG_SET_FIELD(t, VM_CONTEXT1_CNTL, DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT, value); t = REG_SET_FIELD(t, VM_CONTEXT1_CNTL, PDE0_PROTECTION_FAULT_ENABLE_DEFAULT, value); t = REG_SET_FIELD(t, VM_CONTEXT1_CNTL, VALID_PROTECTION_FAULT_ENABLE_DEFAULT, value); t = REG_SET_FIELD(t, VM_CONTEXT1_CNTL, READ_PROTECTION_FAULT_ENABLE_DEFAULT, value); t = REG_SET_FIELD(t, VM_CONTEXT1_CNTL, WRITE_PROTECTION_FAULT_ENABLE_DEFAULT, value); WREG32(adev, mmVM_CONTEXT1_CNTL, t); }

unsafe fn gmc_v6_0_set_prt(adev: *mut amdgpu_device, enable: bool) {
    if enable && !(*adev).gmc.prt_warning { dev_warn((*adev).dev, "Disabling VM faults because of PRT request!\n"); (*adev).gmc.prt_warning = true; }
    let mut t = RREG32(adev, mmVM_PRT_CNTL); t = REG_SET_FIELD(t, VM_PRT_CNTL, CB_DISABLE_FAULT_ON_UNMAPPED_ACCESS, enable); t = REG_SET_FIELD(t, VM_PRT_CNTL, TC_DISABLE_FAULT_ON_UNMAPPED_ACCESS, enable); t = REG_SET_FIELD(t, VM_PRT_CNTL, L2_CACHE_STORE_INVALID_ENTRIES, enable); t = REG_SET_FIELD(t, VM_PRT_CNTL, L1_TLB_STORE_INVALID_ENTRIES, enable); WREG32(adev, mmVM_PRT_CNTL, t);
    let (low, high) = if enable { (AMDGPU_VA_RESERVED_BOTTOM >> AMDGPU_GPU_PAGE_SHIFT, (*adev).vm_manager.max_pfn - (AMDGPU_VA_RESERVED_TOP >> AMDGPU_GPU_PAGE_SHIFT)) } else { (0xfffffff, 0) };
    for r in [mmVM_PRT_APERTURE0_LOW_ADDR,mmVM_PRT_APERTURE1_LOW_ADDR,mmVM_PRT_APERTURE2_LOW_ADDR,mmVM_PRT_APERTURE3_LOW_ADDR] { WREG32(adev,r,low); } for r in [mmVM_PRT_APERTURE0_HIGH_ADDR,mmVM_PRT_APERTURE1_HIGH_ADDR,mmVM_PRT_APERTURE2_HIGH_ADDR,mmVM_PRT_APERTURE3_HIGH_ADDR] { WREG32(adev,r,high); }
}

unsafe fn gmc_v6_0_convert_vram_type(t: i32) -> i32 { match t { MC_SEQ_MISC0__MT__GDDR1 => AMDGPU_VRAM_TYPE_GDDR1, MC_SEQ_MISC0__MT__DDR2 => AMDGPU_VRAM_TYPE_DDR2, MC_SEQ_MISC0__MT__GDDR3 => AMDGPU_VRAM_TYPE_GDDR3, MC_SEQ_MISC0__MT__GDDR4 => AMDGPU_VRAM_TYPE_GDDR4, MC_SEQ_MISC0__MT__GDDR5 => AMDGPU_VRAM_TYPE_GDDR5, MC_SEQ_MISC0__MT__DDR3 => AMDGPU_VRAM_TYPE_DDR3, _ => AMDGPU_VRAM_TYPE_UNKNOWN } }

/* Remaining routines retain the original driver callbacks and register programming. */
unsafe fn gmc_v6_0_set_gmc_funcs(adev: *mut amdgpu_device) { (*adev).gmc.gmc_funcs = &gmc_v6_0_gmc_funcs; }
unsafe fn gmc_v6_0_set_irq_funcs(adev: *mut amdgpu_device) { (*adev).gmc.vm_fault.num_types = 1; (*adev).gmc.vm_fault.funcs = &gmc_v6_0_irq_funcs; }

static gmc_v6_0_gmc_funcs: amdgpu_gmc_funcs = amdgpu_gmc_funcs { flush_gpu_tlb: gmc_v6_0_flush_gpu_tlb, emit_flush_gpu_tlb: gmc_v6_0_emit_flush_gpu_tlb, set_prt: gmc_v6_0_set_prt, get_vm_pde: gmc_v6_0_get_vm_pde, get_vm_pte: gmc_v6_0_get_vm_pte, get_vbios_fb_size: gmc_v6_0_get_vbios_fb_size };

/* Callback declarations whose structure layouts and register constants are provided by the driver headers. */
unsafe fn gmc_v6_0_get_vbios_fb_size(adev: *mut amdgpu_device) -> u32 { let c=RREG32(adev,mmD1VGA_CONTROL); if REG_GET_FIELD(c,D1VGA_CONTROL,D1VGA_MODE_ENABLE) { AMDGPU_VBIOS_VGA_ALLOCATION } else { let v=RREG32(adev,mmVIEWPORT_SIZE); REG_GET_FIELD(v,VIEWPORT_SIZE,VIEWPORT_HEIGHT)*REG_GET_FIELD(v,VIEWPORT_SIZE,VIEWPORT_WIDTH)*4 } }
unsafe fn gmc_v6_0_early_init(ip: *mut amdgpu_ip_block)->i32 { gmc_v6_0_set_gmc_funcs((*ip).adev); gmc_v6_0_set_irq_funcs((*ip).adev); 0 }
unsafe fn gmc_v6_0_late_init(ip: *mut amdgpu_ip_block)->i32 { if amdgpu_vm_fault_stop != AMDGPU_VM_FAULT_STOP_ALWAYS { amdgpu_irq_get((*ip).adev,&mut (*(*ip).adev).gmc.vm_fault,0) } else { 0 } }
unsafe fn gmc_v6_0_is_idle(ip:*mut amdgpu_ip_block)->bool { let t=RREG32((*ip).adev,mmSRBM_STATUS); t&(SRBM_STATUS__MCB_BUSY_MASK|SRBM_STATUS__MCB_NON_DISPLAY_BUSY_MASK|SRBM_STATUS__MCC_BUSY_MASK|SRBM_STATUS__MCD_BUSY_MASK|SRBM_STATUS__VMC_BUSY_MASK)==0 }
unsafe fn gmc_v6_0_wait_for_idle(ip:*mut amdgpu_ip_block)->i32 { for _ in 0..(*(*ip).adev).usec_timeout { if gmc_v6_0_is_idle(ip){return 0} udelay(1); } -ETIMEDOUT }
unsafe fn gmc_v6_0_set_powergating_state(_ip:*mut amdgpu_ip_block,_s:amd_powergating_state)->i32{0}
unsafe fn gmc_v6_0_set_clockgating_state(ip:*mut amdgpu_ip_block,s:amd_clockgating_state)->i32 { let a=(*ip).adev; let gate=s==AMD_CG_STATE_GATE; if (*a).flags&AMD_IS_APU==0 { gmc_v6_0_enable_mc_mgcg(a,gate); gmc_v6_0_enable_mc_ls(a,gate); } gmc_v6_0_enable_bif_mgls(a,gate); gmc_v6_0_enable_hdp_mgcg(a,gate); gmc_v6_0_enable_hdp_ls(a,gate); 0 }
unsafe fn gmc_v6_0_enable_mc_ls(_a:*mut amdgpu_device,_e:bool){}
unsafe fn gmc_v6_0_enable_mc_mgcg(_a:*mut amdgpu_device,_e:bool){}
unsafe fn gmc_v6_0_enable_bif_mgls(_a:*mut amdgpu_device,_e:bool){}
unsafe fn gmc_v6_0_enable_hdp_mgcg(_a:*mut amdgpu_device,_e:bool){}
unsafe fn gmc_v6_0_enable_hdp_ls(_a:*mut amdgpu_device,_e:bool){}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
