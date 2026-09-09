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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

// External kernel, AMDGPU, register, and generated-header dependencies are
// supplied by the surrounding translation unit.

const DF_3_6_SMN_REG_INST_DIST: u32 = 0x8;
const DF_3_6_INST_CNT: u32 = 8;
const DF_V3_6_MAX_COUNTERS: usize = 4;
const ARM_RETRY_USEC_TIMEOUT: i32 = 1000;
const ARM_RETRY_USEC_INTERVAL: i32 = 100;
const DEFERRED_ARM_MASK: u32 = 1 << 31;

#[inline]
fn df_v3_6_get_event(x: u64) -> u64 { x & 0xff }
#[inline]
fn df_v3_6_get_instance(x: u64) -> u64 { (x >> 8) & 0xff }
#[inline]
fn df_v3_6_get_unitmask(x: u64) -> u64 { (x >> 16) & 0xff }
const DF_V3_6_PERFMON_OVERFLOW: u64 = 0xffff_ffff_ffff;

static mut DF_V3_6_CHANNEL_NUMBER: [u32; 15] =
    [1, 2, 0, 4, 0, 8, 0, 16, 32, 0, 0, 0, 2, 4, 8];

unsafe fn df_v3_6_get_fica(adev: *mut amdgpu_device, ficaa_val: u32) -> u64 {
    let flags: c_ulong;
    let address = (*(*adev).nbio.funcs).get_pcie_index_offset(adev);
    let data = (*(*adev).nbio.funcs).get_pcie_data_offset(adev);
    spin_lock_irqsave(&mut (*adev).reg.pcie.lock, &mut { flags });
    WREG32(address, smnDF_PIE_AON_FabricIndirectConfigAccessAddress3);
    WREG32(data, ficaa_val);
    WREG32(address, smnDF_PIE_AON_FabricIndirectConfigAccessDataLo3);
    let ficadl_val = RREG32(data);
    WREG32(address, smnDF_PIE_AON_FabricIndirectConfigAccessDataHi3);
    let ficadh_val = RREG32(data);
    spin_unlock_irqrestore(&mut (*adev).reg.pcie.lock, flags);
    ((ficadh_val as u64) << 32) | ficadl_val as u64
}

unsafe fn df_v3_6_set_fica(adev: *mut amdgpu_device, ficaa_val: u32, ficadl_val: u32, ficadh_val: u32) {
    let flags: c_ulong;
    let address = (*(*adev).nbio.funcs).get_pcie_index_offset(adev);
    let data = (*(*adev).nbio.funcs).get_pcie_data_offset(adev);
    spin_lock_irqsave(&mut (*adev).reg.pcie.lock, &mut { flags });
    WREG32(address, smnDF_PIE_AON_FabricIndirectConfigAccessAddress3);
    WREG32(data, ficaa_val);
    WREG32(address, smnDF_PIE_AON_FabricIndirectConfigAccessDataLo3);
    WREG32(data, ficadl_val);
    WREG32(address, smnDF_PIE_AON_FabricIndirectConfigAccessDataHi3);
    WREG32(data, ficadh_val);
    spin_unlock_irqrestore(&mut (*adev).reg.pcie.lock, flags);
}

unsafe fn df_v3_6_perfmon_rreg(adev: *mut amdgpu_device, lo_addr: u32, lo_val: *mut u32, hi_addr: u32, hi_val: *mut u32) {
    let flags: c_ulong;
    let address = (*(*adev).nbio.funcs).get_pcie_index_offset(adev);
    let data = (*(*adev).nbio.funcs).get_pcie_data_offset(adev);
    spin_lock_irqsave(&mut (*adev).reg.pcie.lock, &mut { flags });
    WREG32(address, lo_addr); *lo_val = RREG32(data);
    WREG32(address, hi_addr); *hi_val = RREG32(data);
    spin_unlock_irqrestore(&mut (*adev).reg.pcie.lock, flags);
}

unsafe fn df_v3_6_perfmon_wreg(adev: *mut amdgpu_device, lo_addr: u32, lo_val: u32, hi_addr: u32, hi_val: u32) {
    let flags: c_ulong;
    let address = (*(*adev).nbio.funcs).get_pcie_index_offset(adev);
    let data = (*(*adev).nbio.funcs).get_pcie_data_offset(adev);
    spin_lock_irqsave(&mut (*adev).reg.pcie.lock, &mut { flags });
    WREG32(address, lo_addr); WREG32(data, lo_val);
    WREG32(address, hi_addr); WREG32(data, hi_val);
    spin_unlock_irqrestore(&mut (*adev).reg.pcie.lock, flags);
}

unsafe fn df_v3_6_perfmon_arm_with_status(adev: *mut amdgpu_device, lo_addr: u32, lo_val: u32, hi_addr: u32, hi_val: u32) -> i32 {
    let flags: c_ulong;
    let address = (*(*adev).nbio.funcs).get_pcie_index_offset(adev);
    let data = (*(*adev).nbio.funcs).get_pcie_data_offset(adev);
    spin_lock_irqsave(&mut (*adev).reg.pcie.lock, &mut { flags });
    WREG32(address, lo_addr); WREG32(data, lo_val);
    WREG32(address, hi_addr); WREG32(data, hi_val);
    WREG32(address, lo_addr); let lo_val_rb = RREG32(data);
    WREG32(address, hi_addr); let hi_val_rb = RREG32(data);
    spin_unlock_irqrestore(&mut (*adev).reg.pcie.lock, flags);
    if lo_val != lo_val_rb || hi_val != hi_val_rb { -EBUSY } else { 0 }
}

unsafe fn df_v3_6_perfmon_arm_with_retry(adev: *mut amdgpu_device, lo_addr: u32, lo_val: u32, hi_addr: u32, hi_val: u32) -> i32 {
    let mut countdown = ARM_RETRY_USEC_TIMEOUT;
    while countdown != 0 {
        if df_v3_6_perfmon_arm_with_status(adev, lo_addr, lo_val, hi_addr, hi_val) == 0 { break; }
        countdown -= ARM_RETRY_USEC_INTERVAL;
        udelay(ARM_RETRY_USEC_INTERVAL as u32);
    }
    if countdown > 0 { 0 } else { -ETIME }
}

unsafe fn df_v3_6_pmc_has_counter(adev: *mut amdgpu_device, config: u64, counter_idx: usize) -> bool {
    (config & 0x0fff_ffff) == (*adev).df_perfmon_config_assign_mask[counter_idx] as u64
}

unsafe fn df_v3_6_pmc_get_addr(adev: *mut amdgpu_device, config: u64, counter_idx: usize, is_ctrl: bool, lo: *mut u32, hi: *mut u32) {
    if !df_v3_6_pmc_has_counter(adev, config, counter_idx) { return; }
    let (l, h) = match counter_idx {
        0 => (if is_ctrl { smnPerfMonCtlLo4 } else { smnPerfMonCtrLo4 }, if is_ctrl { smnPerfMonCtlHi4 } else { smnPerfMonCtrHi4 }),
        1 => (if is_ctrl { smnPerfMonCtlLo5 } else { smnPerfMonCtrLo5 }, if is_ctrl { smnPerfMonCtlHi5 } else { smnPerfMonCtrHi5 }),
        2 => (if is_ctrl { smnPerfMonCtlLo6 } else { smnPerfMonCtrLo6 }, if is_ctrl { smnPerfMonCtlHi6 } else { smnPerfMonCtrHi6 }),
        3 => (if is_ctrl { smnPerfMonCtlLo7 } else { smnPerfMonCtrLo7 }, if is_ctrl { smnPerfMonCtlHi7 } else { smnPerfMonCtrHi7 }),
        _ => return,
    }; *lo = l; *hi = h;
}

unsafe fn df_v3_6_pmc_get_read_settings(adev: *mut amdgpu_device, config: u64, idx: usize, lo: *mut u32, hi: *mut u32) { df_v3_6_pmc_get_addr(adev, config, idx, false, lo, hi); }

unsafe fn df_v3_6_pmc_get_ctrl_settings(adev: *mut amdgpu_device, config: u64, idx: usize, lo: *mut u32, hi: *mut u32, lov: *mut u32, hiv: *mut u32, enable: bool) -> i32 {
    df_v3_6_pmc_get_addr(adev, config, idx, true, lo, hi);
    if *lo == 0 || *hi == 0 { return -ENXIO; }
    let eventsel = (df_v3_6_get_event(config) as u32) & 0x3f;
    let unitmask = (df_v3_6_get_unitmask(config) as u32) & 0xf;
    let instance = df_v3_6_get_instance(config) as u32;
    let instance_10 = instance & 3; let instance_5432 = (instance >> 2) & 0xf; let instance_76 = (instance >> 6) & 3;
    *lov = (unitmask << 8) | (instance_10 << 6) | eventsel;
    if enable { *lov |= 1 << 22; } else { *lov &= !(1 << 22); }
    *hiv = (instance_76 << 29) | instance_5432;
    0
}

unsafe fn df_v3_6_pmc_add_cntr(adev: *mut amdgpu_device, config: u64) -> i32 {
    for i in 0..DF_V3_6_MAX_COUNTERS { if (*adev).df_perfmon_config_assign_mask[i] == 0 { (*adev).df_perfmon_config_assign_mask[i] = (config & 0x0fff_ffff) as u32; return i as i32; } }
    -ENOSPC
}

unsafe fn df_v3_6_pmc_set_deferred(adev: *mut amdgpu_device, config: u64, idx: usize, deferred: bool) -> i32 {
    if !df_v3_6_pmc_has_counter(adev, config, idx) { return -EINVAL; }
    if deferred { (*adev).df_perfmon_config_assign_mask[idx] |= DEFERRED_ARM_MASK; } else { (*adev).df_perfmon_config_assign_mask[idx] &= !DEFERRED_ARM_MASK; } 0
}
unsafe fn df_v3_6_pmc_is_deferred(adev: *mut amdgpu_device, config: u64, idx: usize) -> bool { df_v3_6_pmc_has_counter(adev, config, idx) && ((*adev).df_perfmon_config_assign_mask[idx] & DEFERRED_ARM_MASK) != 0 }
unsafe fn df_v3_6_pmc_release_cntr(adev: *mut amdgpu_device, config: u64, idx: usize) { if df_v3_6_pmc_has_counter(adev, config, idx) { (*adev).df_perfmon_config_assign_mask[idx] = 0; } }

unsafe fn df_v3_6_reset_perfmon_cntr(adev: *mut amdgpu_device, config: u64, idx: usize) { let mut lo=0; let mut hi=0; df_v3_6_pmc_get_read_settings(adev, config, idx, &mut lo, &mut hi); if lo != 0 && hi != 0 { df_v3_6_perfmon_wreg(adev, lo, 0, hi, 0); } }

unsafe fn df_v3_6_pmc_start(adev: *mut amdgpu_device, config: u64, idx: usize, is_add: bool) -> i32 {
    match (*adev).asic_type { CHIP_VEGA20 | CHIP_ARCTURUS => { if is_add { return df_v3_6_pmc_add_cntr(adev, config); } let mut lo=0;let mut hi=0;let mut lv=0;let mut hv=0; let ret=df_v3_6_pmc_get_ctrl_settings(adev,config,idx,&mut lo,&mut hi,&mut lv,&mut hv,true); if ret != 0 { return ret; } let err=df_v3_6_perfmon_arm_with_retry(adev,lo,lv,hi,hv); if err != 0 { return df_v3_6_pmc_set_deferred(adev,config,idx,true); } 0 }, _ => 0 }
}

unsafe fn df_v3_6_pmc_stop(adev: *mut amdgpu_device, config: u64, idx: usize, remove: bool) -> i32 {
    match (*adev).asic_type { CHIP_VEGA20 | CHIP_ARCTURUS => { let mut lo=0;let mut hi=0;let mut lv=0;let mut hv=0; let ret=df_v3_6_pmc_get_ctrl_settings(adev,config,idx,&mut lo,&mut hi,&mut lv,&mut hv,false); if ret != 0{return ret;} df_v3_6_perfmon_wreg(adev,lo,lv,hi,hv); if remove {df_v3_6_reset_perfmon_cntr(adev,config,idx);df_v3_6_pmc_release_cntr(adev,config,idx);} 0 }, _ => 0 }
}

unsafe fn df_v3_6_pmc_get_count(adev: *mut amdgpu_device, config: u64, idx: usize, count: *mut u64) {
    *count=0; match (*adev).asic_type { CHIP_VEGA20 | CHIP_ARCTURUS => { let mut lo=0;let mut hi=0;let mut lv=0;let mut hv=0; df_v3_6_pmc_get_read_settings(adev,config,idx,&mut lo,&mut hi); if lo==0||hi==0{return;} if df_v3_6_pmc_is_deferred(adev,config,idx) { if df_v3_6_perfmon_arm_with_status(adev,lo,lv,hi,hv)!=0{return;} df_v3_6_pmc_set_deferred(adev,config,idx,false); } df_v3_6_perfmon_rreg(adev,lo,&mut lv,hi,&mut hv); *count=((hv as u64)<<32)|(lv as u64); if *count>=DF_V3_6_PERFMON_OVERFLOW {*count=0;} }, _=>{} }
}

unsafe fn df_v3_6_query_ras_poison_mode(adev: *mut amdgpu_device) -> bool {
    let lo=RREG32_SOC15(DF,0,mmDF_CS_UMC_AON0_HardwareAssertMaskLow);
    let hi=RREG32_SOC15(DF,0,mmDF_NCS_PG0_HardwareAssertMaskHigh);
    let v0=REG_GET_FIELD(lo,DF_CS_UMC_AON0_HardwareAssertMaskLow,HWAssertMsk0);
    let v1=REG_GET_FIELD(lo,DF_CS_UMC_AON0_HardwareAssertMaskLow,HWAssertMsk1);
    let v28=REG_GET_FIELD(hi,DF_NCS_PG0_HardwareAssertMaskHigh,HWAssertMsk28);
    let v31=REG_GET_FIELD(hi,DF_NCS_PG0_HardwareAssertMaskHigh,HWAssertMsk31);
    if v0!=0 && v1!=0 && v28!=0 && v31!=0 { true } else { false }
}
unsafe fn df_v3_6_query_hashes(adev:*mut amdgpu_device) { (*adev).df.hash_status.hash_64k=false; (*adev).df.hash_status.hash_2m=false; (*adev).df.hash_status.hash_1g=false; if ((*adev).asic_type==CHIP_ARCTURUS && df_v3_6_get_fb_channel_number(adev)==0xe)||((*adev).asic_type==CHIP_ALDEBARAN && df_v3_6_get_fb_channel_number(adev)==0x1e) { let t=RREG32_SOC15(DF,0,mmDF_CS_UMC_AON0_DfGlobalCtrl); (*adev).df.hash_status.hash_64k=REG_GET_FIELD(t,DF_CS_UMC_AON0_DfGlobalCtrl,GlbHashIntlvCtl64K)!=0; (*adev).df.hash_status.hash_2m=REG_GET_FIELD(t,DF_CS_UMC_AON0_DfGlobalCtrl,GlbHashIntlvCtl2M)!=0; (*adev).df.hash_status.hash_1g=REG_GET_FIELD(t,DF_CS_UMC_AON0_DfGlobalCtrl,GlbHashIntlvCtl1G)!=0; } }
unsafe fn df_v3_6_sw_init(adev:*mut amdgpu_device) { let _=device_create_file((*adev).dev,&dev_attr_df_cntr_avail); for i in 0..AMDGPU_MAX_DF_PERFMONS {(*adev).df_perfmon_config_assign_mask[i]=0;} df_v3_6_query_hashes(adev); }
unsafe fn df_v3_6_sw_fini(adev:*mut amdgpu_device) { if (*(*adev).dev).kobj.sd.is_some() {device_remove_file((*adev).dev,&dev_attr_df_cntr_avail);} }
unsafe fn df_v3_6_update_medium_grain_clock_gating(adev:*mut amdgpu_device, enable:bool) { if (*adev).cg_flags&AMD_CG_SUPPORT_DF_MGCG != 0 {df_v3_6_enable_broadcast_mode(adev,true); let mut t=RREG32_SOC15(DF,0,mmDF_PIE_AON0_DfGlobalClkGater); t &= !DF_PIE_AON0_DfGlobalClkGater__MGCGMode_MASK; t |= if enable {DF_V3_6_MGCG_ENABLE_15_CYCLE_DELAY}else{DF_V3_6_MGCG_DISABLE}; WREG32_SOC15(DF,0,mmDF_PIE_AON0_DfGlobalClkGater,t); df_v3_6_enable_broadcast_mode(adev,false);} }

// Remaining software lifecycle, clock-gating, hash-query, and sysfs glue are
// direct translations of the C implementation and retain external kernel APIs.
unsafe fn df_v3_6_enable_broadcast_mode(adev:*mut amdgpu_device, enable:bool) {
    if enable { let mut t=RREG32_SOC15(DF,0,mmFabricConfigAccessControl); t &= !FabricConfigAccessControl__CfgRegInstAccEn_MASK; WREG32_SOC15(DF,0,mmFabricConfigAccessControl,t); }
    else { WREG32_SOC15(DF,0,mmFabricConfigAccessControl,mmFabricConfigAccessControl_DEFAULT); }
}
unsafe fn df_v3_6_get_fb_channel_number(adev:*mut amdgpu_device)->u32 { let mut t=if (*adev).asic_type==CHIP_ALDEBARAN {RREG32_SOC15(DF,0,mmDF_GCM_AON0_DramMegaBaseAddress0)&ALDEBARAN_DF_CS_UMC_AON0_DramBaseAddress0__IntLvNumChan_MASK}else{RREG32_SOC15(DF,0,mmDF_CS_UMC_AON0_DramBaseAddress0)&DF_CS_UMC_AON0_DramBaseAddress0__IntLvNumChan_MASK}; t >>= DF_CS_UMC_AON0_DramBaseAddress0__IntLvNumChan__SHIFT; t }
unsafe fn df_v3_6_get_hbm_channel_number(adev:*mut amdgpu_device)->u32 { let n=(*(*adev).df.funcs).get_fb_channel_number(adev) as usize; if n>=15 {0} else {DF_V3_6_CHANNEL_NUMBER[n]} }
unsafe fn df_v3_6_get_clockgating_state(adev:*mut amdgpu_device, flags:*mut u64) { if RREG32_SOC15(DF,0,mmDF_PIE_AON0_DfGlobalClkGater)&DF_V3_6_MGCG_ENABLE_15_CYCLE_DELAY != 0 {*flags|=AMD_CG_SUPPORT_DF_MGCG;} }

// Function-table initialization preserves the externally visible interface.
#[allow(non_upper_case_globals)]
pub static df_v3_6_funcs: amdgpu_df_funcs = amdgpu_df_funcs {
    sw_init: Some(df_v3_6_sw_init), sw_fini: Some(df_v3_6_sw_fini),
    enable_broadcast_mode: Some(df_v3_6_enable_broadcast_mode),
    get_fb_channel_number: Some(df_v3_6_get_fb_channel_number),
    get_hbm_channel_number: Some(df_v3_6_get_hbm_channel_number),
    update_medium_grain_clock_gating: Some(df_v3_6_update_medium_grain_clock_gating),
    get_clockgating_state: Some(df_v3_6_get_clockgating_state),
    pmc_start: Some(df_v3_6_pmc_start), pmc_stop: Some(df_v3_6_pmc_stop),
    pmc_get_count: Some(df_v3_6_pmc_get_count),
    get_fica: Some(df_v3_6_get_fica), set_fica: Some(df_v3_6_set_fica),
    query_ras_poison_mode: Some(df_v3_6_query_ras_poison_mode),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
