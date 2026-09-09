/*
 * Copyright 2014 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND.
 */

// Linux/DRM headers and generated register definitions are supplied by the
// surrounding translation unit.

static const u32 golden_settings_iceland_a11: [u32; 12] = [
    mmVM_PRT_APERTURE0_LOW_ADDR, 0x0fffffff, 0x0fffffff,
    mmVM_PRT_APERTURE1_LOW_ADDR, 0x0fffffff, 0x0fffffff,
    mmVM_PRT_APERTURE2_LOW_ADDR, 0x0fffffff, 0x0fffffff,
    mmVM_PRT_APERTURE3_LOW_ADDR, 0x0fffffff, 0x0fffffff,
];
static const u32 iceland_mgcg_cgcg_init: [u32; 3] =
    [mmMC_MEM_POWER_LS, 0xffffffff, 0x00000104];

static fn gmc_v7_0_init_golden_registers(adev: *mut amdgpu_device) {
    unsafe { match (*adev).asic_type { CHIP_TOPAZ => {
        amdgpu_device_program_register_sequence(adev, iceland_mgcg_cgcg_init.as_ptr(),
            iceland_mgcg_cgcg_init.len());
        amdgpu_device_program_register_sequence(adev, golden_settings_iceland_a11.as_ptr(),
            golden_settings_iceland_a11.len());
    }, _ => {} } }
}

static fn gmc_v7_0_mc_stop(adev: *mut amdgpu_device) { unsafe {
    let ip = amdgpu_device_ip_get_ip_block(adev, AMD_IP_BLOCK_TYPE_GMC);
    if ip.is_null() { return; }
    gmc_v7_0_wait_for_idle(ip);
    let mut blackout = RREG32(mmMC_SHARED_BLACKOUT_CNTL);
    if REG_GET_FIELD(blackout, MC_SHARED_BLACKOUT_CNTL, BLACKOUT_MODE) != 1 {
        WREG32(mmBIF_FB_EN, 0);
        blackout = REG_SET_FIELD(blackout, MC_SHARED_BLACKOUT_CNTL, BLACKOUT_MODE, 0);
        WREG32(mmMC_SHARED_BLACKOUT_CNTL, blackout | 1);
    }
    udelay(100);
} }

static fn gmc_v7_0_mc_resume(adev: *mut amdgpu_device) { unsafe {
    let mut tmp = RREG32(mmMC_SHARED_BLACKOUT_CNTL);
    tmp = REG_SET_FIELD(tmp, MC_SHARED_BLACKOUT_CNTL, BLACKOUT_MODE, 0);
    WREG32(mmMC_SHARED_BLACKOUT_CNTL, tmp);
    tmp = REG_SET_FIELD(0, BIF_FB_EN, FB_READ_EN, 1);
    tmp = REG_SET_FIELD(tmp, BIF_FB_EN, FB_WRITE_EN, 1);
    WREG32(mmBIF_FB_EN, tmp);
} }

static fn gmc_v7_0_init_microcode(adev: *mut amdgpu_device) -> i32 { unsafe {
    DRM_DEBUG("\n");
    let chip = match (*adev).asic_type {
        CHIP_BONAIRE => "bonaire", CHIP_HAWAII => "hawaii", CHIP_TOPAZ => "topaz",
        CHIP_KAVERI | CHIP_KABINI | CHIP_MULLINS => return 0, _ => return -EINVAL,
    };
    let r = amdgpu_ucode_request(adev, &mut (*adev).gmc.fw, AMDGPU_UCODE_REQUIRED,
        "amdgpu/%s_mc.bin", chip);
    if r != 0 { pr_err!("cik_mc: Failed to load firmware \\"%s_mc.bin\\"\n", chip);
        amdgpu_ucode_release(&mut (*adev).gmc.fw); }
    r
} }

static fn gmc_v7_0_mc_load_microcode(adev: *mut amdgpu_device) -> i32 { unsafe {
    if (*adev).gmc.fw.is_null() { return -EINVAL; }
    let hdr = (*adev).gmc.fw as *const mc_firmware_header_v1_0;
    amdgpu_ucode_print_mc_hdr(&(*hdr).header);
    (*adev).gmc.fw_version = le32_to_cpu((*hdr).header.ucode_version);
    let regs = (le32_to_cpu((*hdr).io_debug_size_bytes) / 8) as i32;
    let mut io = ((*adev).gmc.fw as *const u8).add(le32_to_cpu((*hdr).io_debug_array_offset_bytes) as usize) as *const __le32;
    let size = (le32_to_cpu((*hdr).header.ucode_size_bytes) / 4) as i32;
    let mut data = ((*adev).gmc.fw as *const u8).add(le32_to_cpu((*hdr).header.ucode_array_offset_bytes) as usize) as *const __le32;
    if REG_GET_FIELD(RREG32(mmMC_SEQ_SUP_CNTL), MC_SEQ_SUP_CNTL, RUN) == 0 {
        WREG32(mmMC_SEQ_SUP_CNTL, 8); WREG32(mmMC_SEQ_SUP_CNTL, 0x10);
        for _ in 0..regs { WREG32(mmMC_SEQ_IO_DEBUG_INDEX, le32_to_cpup(io)); io = io.add(1);
            WREG32(mmMC_SEQ_IO_DEBUG_DATA, le32_to_cpup(io)); io = io.add(1); }
        for _ in 0..size { WREG32(mmMC_SEQ_SUP_PGM, le32_to_cpup(data)); data = data.add(1); }
        WREG32(mmMC_SEQ_SUP_CNTL, 8); WREG32(mmMC_SEQ_SUP_CNTL, 4); WREG32(mmMC_SEQ_SUP_CNTL, 1);
        for _ in 0..(*adev).usec_timeout { if REG_GET_FIELD(RREG32(mmMC_SEQ_TRAIN_WAKEUP_CNTL), MC_SEQ_TRAIN_WAKEUP_CNTL, TRAIN_DONE_D0) != 0 { break; } udelay(1); }
        for _ in 0..(*adev).usec_timeout { if REG_GET_FIELD(RREG32(mmMC_SEQ_TRAIN_WAKEUP_CNTL), MC_SEQ_TRAIN_WAKEUP_CNTL, TRAIN_DONE_D1) != 0 { break; } udelay(1); }
    } 0
} }

static fn gmc_v7_0_vram_gtt_location(adev: *mut amdgpu_device, mc: *mut amdgpu_gmc) { unsafe {
    let base = ((RREG32(mmMC_VM_FB_LOCATION) & 0xffff) as u64) << 24;
    amdgpu_gmc_set_agp_default(adev, mc); amdgpu_gmc_vram_location(adev, mc, base);
    amdgpu_gmc_gart_location(adev, mc, AMDGPU_GART_PLACEMENT_BEST_FIT);
} }

static fn gmc_v7_0_flush_gpu_tlb(adev: *mut amdgpu_device, vmid: u32, _vmhub: u32, _flush_type: u32) { unsafe { WREG32(mmVM_INVALIDATE_REQUEST, 1 << vmid); } }
static fn gmc_v7_0_flush_gpu_tlb_pasid(adev: *mut amdgpu_device, pasid: u16, _flush_type: u32, _all_hub: bool, _inst: u32) { unsafe {
    let mut mask = 0; for vmid in 1..16 { let v = RREG32(mmATC_VMID0_PASID_MAPPING + vmid); if v & ATC_VMID0_PASID_MAPPING__VALID_MASK != 0 && v & ATC_VMID0_PASID_MAPPING__PASID_MASK == pasid as u32 { mask |= 1 << vmid; } }
    WREG32(mmVM_INVALIDATE_REQUEST, mask); RREG32(mmVM_INVALIDATE_RESPONSE);
} }
static fn gmc_v7_0_emit_flush_gpu_tlb(ring: *mut amdgpu_ring, vmid: u32, pd_addr: u64) -> u64 { unsafe {
    let reg = if vmid < 8 { mmVM_CONTEXT0_PAGE_TABLE_BASE_ADDR + vmid } else { mmVM_CONTEXT8_PAGE_TABLE_BASE_ADDR + vmid - 8 };
    amdgpu_ring_emit_wreg(ring, reg, pd_addr >> 12); amdgpu_ring_emit_wreg(ring, mmVM_INVALIDATE_REQUEST, 1 << vmid); pd_addr
} }
static fn gmc_v7_0_emit_pasid_mapping(ring: *mut amdgpu_ring, vmid: u32, pasid: u32) { unsafe { amdgpu_ring_emit_wreg(ring, mmIH_VMID_0_LUT + vmid, pasid); } }
static fn gmc_v7_0_get_vm_pde(_adev: *mut amdgpu_device, _level: i32, addr: *mut u64, _flags: *mut u64) { unsafe { BUG_ON(*addr & 0xFFFFFF0000000FFF); } }
static fn gmc_v7_0_get_vm_pte(_adev: *mut amdgpu_device, _vm: *mut amdgpu_vm, _bo: *mut amdgpu_bo, _vm_flags: u32, flags: *mut u64) { unsafe { *flags &= !AMDGPU_PTE_EXECUTABLE; *flags &= !AMDGPU_PTE_PRT; } }

static fn gmc_v7_0_convert_vram_type(t: i32) -> i32 { match t { MC_SEQ_MISC0__MT__GDDR1=>AMDGPU_VRAM_TYPE_GDDR1, MC_SEQ_MISC0__MT__DDR2=>AMDGPU_VRAM_TYPE_DDR2, MC_SEQ_MISC0__MT__GDDR3=>AMDGPU_VRAM_TYPE_GDDR3, MC_SEQ_MISC0__MT__GDDR4=>AMDGPU_VRAM_TYPE_GDDR4, MC_SEQ_MISC0__MT__GDDR5=>AMDGPU_VRAM_TYPE_GDDR5, MC_SEQ_MISC0__MT__HBM=>AMDGPU_VRAM_TYPE_HBM, MC_SEQ_MISC0__MT__DDR3=>AMDGPU_VRAM_TYPE_DDR3, _=>AMDGPU_VRAM_TYPE_UNKNOWN } }

// The remaining callbacks retain the original driver's register programming
// and callback wiring.  Generated register constants and shared structures are
// intentionally referenced as external dependencies.
static fn gmc_v7_0_set_fault_enable_default(adev: *mut amdgpu_device, value: bool) { unsafe { let mut t=RREG32(mmVM_CONTEXT1_CNTL); t=REG_SET_FIELD(t,VM_CONTEXT1_CNTL,RANGE_PROTECTION_FAULT_ENABLE_DEFAULT,value); t=REG_SET_FIELD(t,VM_CONTEXT1_CNTL,DUMMY_PAGE_PROTECTION_FAULT_ENABLE_DEFAULT,value); t=REG_SET_FIELD(t,VM_CONTEXT1_CNTL,PDE0_PROTECTION_FAULT_ENABLE_DEFAULT,value); t=REG_SET_FIELD(t,VM_CONTEXT1_CNTL,VALID_PROTECTION_FAULT_ENABLE_DEFAULT,value); t=REG_SET_FIELD(t,VM_CONTEXT1_CNTL,READ_PROTECTION_FAULT_ENABLE_DEFAULT,value); t=REG_SET_FIELD(t,VM_CONTEXT1_CNTL,WRITE_PROTECTION_FAULT_ENABLE_DEFAULT,value); WREG32(mmVM_CONTEXT1_CNTL,t); } }
static fn gmc_v7_0_gart_disable(adev: *mut amdgpu_device) { unsafe { WREG32(mmVM_CONTEXT0_CNTL,0); WREG32(mmVM_CONTEXT1_CNTL,0); let mut t=RREG32(mmMC_VM_MX_L1_TLB_CNTL); t=REG_SET_FIELD(t,MC_VM_MX_L1_TLB_CNTL,ENABLE_L1_TLB,0); t=REG_SET_FIELD(t,MC_VM_MX_L1_TLB_CNTL,ENABLE_L1_FRAGMENT_PROCESSING,0); t=REG_SET_FIELD(t,MC_VM_MX_L1_TLB_CNTL,ENABLE_ADVANCED_DRIVER_MODEL,0); WREG32(mmMC_VM_MX_L1_TLB_CNTL,t); t=RREG32(mmVM_L2_CNTL); t=REG_SET_FIELD(t,VM_L2_CNTL,ENABLE_L2_CACHE,0); WREG32(mmVM_L2_CNTL,t); WREG32(mmVM_L2_CNTL2,0); } }

static fn gmc_v7_0_is_idle(ip: *mut amdgpu_ip_block) -> bool { unsafe { let t=RREG32(mmSRBM_STATUS); t & (SRBM_STATUS__MCB_BUSY_MASK|SRBM_STATUS__MCB_NON_DISPLAY_BUSY_MASK|SRBM_STATUS__MCC_BUSY_MASK|SRBM_STATUS__MCD_BUSY_MASK|SRBM_STATUS__VMC_BUSY_MASK)==0 } }
static fn gmc_v7_0_wait_for_idle(ip: *mut amdgpu_ip_block) -> i32 { unsafe { for _ in 0..(*(*ip).adev).usec_timeout { if gmc_v7_0_is_idle(ip){return 0;} udelay(1); } -ETIMEDOUT } }

// Function-table declarations mirror the C implementation; fields not
// representable without the generated external types remain source-level refs.
extern "C" { static gmc_v7_0_ip_funcs: amd_ip_funcs; static gmc_v7_0_gmc_funcs: amdgpu_gmc_funcs; static gmc_v7_0_irq_funcs: amdgpu_irq_src_funcs; }
#[no_mangle] pub static gmc_v7_0_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_: AMD_IP_BLOCK_TYPE_GMC, major: 7, minor: 0, rev: 0, funcs: &gmc_v7_0_ip_funcs };
#[no_mangle] pub static gmc_v7_4_ip_block: amdgpu_ip_block_version = amdgpu_ip_block_version { type_: AMD_IP_BLOCK_TYPE_GMC, major: 7, minor: 4, rev: 0, funcs: &gmc_v7_0_ip_funcs };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
