// SPDX-License-Identifier: MIT
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
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.
 */

const AMDGPU_PCIE_INDEX_FALLBACK: usize = 0x38 >> 2;
const AMDGPU_PCIE_INDEX_HI_FALLBACK: usize = 0x44 >> 2;
const AMDGPU_PCIE_DATA_FALLBACK: usize = 0x3c >> 2;

pub unsafe fn amdgpu_reg_access_init(adev: *mut amdgpu_device) {
    spin_lock_init(&mut (*adev).reg.smc.lock); (*adev).reg.smc.rreg = None; (*adev).reg.smc.wreg = None;
    spin_lock_init(&mut (*adev).reg.uvd_ctx.lock); (*adev).reg.uvd_ctx.rreg = None; (*adev).reg.uvd_ctx.wreg = None;
    spin_lock_init(&mut (*adev).reg.didt.lock); (*adev).reg.didt.rreg = None; (*adev).reg.didt.wreg = None;
    spin_lock_init(&mut (*adev).reg.gc_cac.lock); (*adev).reg.gc_cac.rreg = None; (*adev).reg.gc_cac.wreg = None;
    spin_lock_init(&mut (*adev).reg.se_cac.lock); (*adev).reg.se_cac.rreg = None; (*adev).reg.se_cac.wreg = None;
    spin_lock_init(&mut (*adev).reg.audio_endpt.lock); (*adev).reg.audio_endpt.rreg = None; (*adev).reg.audio_endpt.wreg = None;
    spin_lock_init(&mut (*adev).reg.pcie.lock);
    (*adev).reg.pcie.rreg = None; (*adev).reg.pcie.wreg = None; (*adev).reg.pcie.rreg_ext = None; (*adev).reg.pcie.wreg_ext = None;
    (*adev).reg.pcie.rreg64 = None; (*adev).reg.pcie.wreg64 = None; (*adev).reg.pcie.rreg64_ext = None; (*adev).reg.pcie.wreg64_ext = None;
    (*adev).reg.pcie.port_rreg = None; (*adev).reg.pcie.port_wreg = None;
}

macro_rules! reg32_pair { ($rd:ident, $wr:ident, $field:ident, $name:literal) => {
    pub unsafe fn $rd(a: *mut amdgpu_device, r: u32) -> u32 { if (*a).reg.$field.rreg.is_none() { dev_err_once((*a).dev, concat!($name, " register read not supported\n")); 0 } else { ((*a).reg.$field.rreg.unwrap())(a, r) } }
    pub unsafe fn $wr(a: *mut amdgpu_device, r: u32, v: u32) { if (*a).reg.$field.wreg.is_none() { dev_err_once((*a).dev, concat!($name, " register write not supported\n")); return; } ((*a).reg.$field.wreg.unwrap())(a, r, v); }
} }
reg32_pair!(amdgpu_reg_smc_rd32, amdgpu_reg_smc_wr32, smc, "SMC");
reg32_pair!(amdgpu_reg_uvd_ctx_rd32, amdgpu_reg_uvd_ctx_wr32, uvd_ctx, "UVD_CTX");
reg32_pair!(amdgpu_reg_didt_rd32, amdgpu_reg_didt_wr32, didt, "DIDT");
reg32_pair!(amdgpu_reg_gc_cac_rd32, amdgpu_reg_gc_cac_wr32, gc_cac, "GC_CAC");
reg32_pair!(amdgpu_reg_se_cac_rd32, amdgpu_reg_se_cac_wr32, se_cac, "SE_CAC");

pub unsafe fn amdgpu_reg_audio_endpt_rd32(a:*mut amdgpu_device,b:u32,r:u32)->u32 { if (*a).reg.audio_endpt.rreg.is_none(){dev_err_once((*a).dev,"AUDIO_ENDPT register read not supported\n");0}else{((*a).reg.audio_endpt.rreg.unwrap())(a,b,r)} }
pub unsafe fn amdgpu_reg_audio_endpt_wr32(a:*mut amdgpu_device,b:u32,r:u32,v:u32){if (*a).reg.audio_endpt.wreg.is_none(){dev_err_once((*a).dev,"AUDIO_ENDPT register write not supported\n");return}((*a).reg.audio_endpt.wreg.unwrap())(a,b,r,v)}

pub unsafe fn amdgpu_reg_pcie_rd32(a:*mut amdgpu_device,r:u32)->u32{if (*a).reg.pcie.rreg.is_none(){dev_err_once((*a).dev,"PCIE register read not supported\n");0}else{((*a).reg.pcie.rreg.unwrap())(a,r)}}
pub unsafe fn amdgpu_reg_pcie_wr32(a:*mut amdgpu_device,r:u32,v:u32){if (*a).reg.pcie.wreg.is_none(){dev_err_once((*a).dev,"PCIE register write not supported\n");return}((*a).reg.pcie.wreg.unwrap())(a,r,v)}
pub unsafe fn amdgpu_reg_pcie_ext_rd32(a:*mut amdgpu_device,r:u64)->u32{if (*a).reg.pcie.rreg_ext.is_none(){dev_err_once((*a).dev,"PCIE EXT register read not supported\n");0}else{((*a).reg.pcie.rreg_ext.unwrap())(a,r)}}
pub unsafe fn amdgpu_reg_pcie_ext_wr32(a:*mut amdgpu_device,r:u64,v:u32){if (*a).reg.pcie.wreg_ext.is_none(){dev_err_once((*a).dev,"PCIE EXT register write not supported\n");return}((*a).reg.pcie.wreg_ext.unwrap())(a,r,v)}
pub unsafe fn amdgpu_reg_pcie_rd64(a:*mut amdgpu_device,r:u32)->u64{if (*a).reg.pcie.rreg64.is_none(){dev_err_once((*a).dev,"PCIE 64-bit register read not supported\n");0}else{((*a).reg.pcie.rreg64.unwrap())(a,r)}}
pub unsafe fn amdgpu_reg_pcie_wr64(a:*mut amdgpu_device,r:u32,v:u64){if (*a).reg.pcie.wreg64.is_none(){dev_err_once((*a).dev,"PCIE 64-bit register write not supported\n");return}((*a).reg.pcie.wreg64.unwrap())(a,r,v)}
pub unsafe fn amdgpu_reg_pcie_ext_rd64(a:*mut amdgpu_device,r:u64)->u64{if (*a).reg.pcie.rreg64_ext.is_none(){dev_err_once((*a).dev,"PCIE EXT 64-bit register read not supported\n");0}else{((*a).reg.pcie.rreg64_ext.unwrap())(a,r)}}
pub unsafe fn amdgpu_reg_pcie_ext_wr64(a:*mut amdgpu_device,r:u64,v:u64){if (*a).reg.pcie.wreg64_ext.is_none(){dev_err_once((*a).dev,"PCIE EXT 64-bit register write not supported\n");return}((*a).reg.pcie.wreg64_ext.unwrap())(a,r,v)}
pub unsafe fn amdgpu_reg_pciep_rd32(a:*mut amdgpu_device,r:u32)->u32{if (*a).reg.pcie.port_rreg.is_none(){dev_err_once((*a).dev,"PCIEP register read not supported\n");0}else{((*a).reg.pcie.port_rreg.unwrap())(a,r)}}
pub unsafe fn amdgpu_reg_pciep_wr32(a:*mut amdgpu_device,r:u32,v:u32){if (*a).reg.pcie.port_wreg.is_none(){dev_err_once((*a).dev,"PCIEP register write not supported\n");return}((*a).reg.pcie.port_wreg.unwrap())(a,r,v)}

unsafe fn amdgpu_reg_get_smn_base_version(a:*mut amdgpu_device)->i32{if amdgpu_sriov_vf(a){return -EOPNOTSUPP}let id=((*a).pdev).device>>4;if id==0x74a||id==0x74b||id==0x75a||id==0x75b{1}else{-EOPNOTSUPP}}
pub unsafe fn amdgpu_reg_get_smn_base64(a:*mut amdgpu_device,b:amd_hw_ip_block_type,d:i32)->u64{if (*a).reg.smn.get_smn_base.is_none(){match amdgpu_reg_get_smn_base_version(a){1=>amdgpu_reg_smn_v1_0_get_base(a,b,d),_=>{dev_err_once((*a).dev,"SMN base address query not supported for this device\n");0}}}else{((*a).reg.smn.get_smn_base.unwrap())(a,b,d)}}
pub unsafe fn amdgpu_reg_smn_v1_0_get_base(a:*mut amdgpu_device,b:amd_hw_ip_block_type,d:i32)->u64{if d==0{return 0}match b{XGMI_HWIP|NBIO_HWIP|MP0_HWIP|UMC_HWIP|DF_HWIP=>((d as u64&3)<<32)|(1<<34),_=>{dev_warn_once((*a).dev,"SMN base address query not supported for this block %d\n",b);0}}}

pub unsafe fn amdgpu_device_rreg(a:*mut amdgpu_device,r:u32,f:u32)->u32{if amdgpu_device_skip_hw_access(a){return 0}let ret=if r.wrapping_mul(4)<(*a).rmmio_size{if f&AMDGPU_REGS_NO_KIQ==0&&amdgpu_sriov_runtime(a)&&down_read_trylock(&(*a).reset_domain.sem){let x=amdgpu_kiq_rreg(a,r,0);up_read(&(*a).reset_domain.sem);x}else{readl((*a).rmmio.add((r*4)as usize))}}else{amdgpu_reg_pcie_rd32(a,r*4)};trace_amdgpu_device_rreg((*a).pdev.device,r,ret);ret}
pub unsafe fn amdgpu_mm_rreg8(a:*mut amdgpu_device,o:u32)->u8{if amdgpu_device_skip_hw_access(a){0}else if o<(*a).rmmio_size{readb((*a).rmmio.add(o as usize))}else{dev_err((*a).dev,"invalid MMIO read offset 0x%x (rmmio size 0x%x)\n",o,(*a).rmmio_size as u32);0}}
pub unsafe fn amdgpu_mm_wreg8(a:*mut amdgpu_device,o:u32,v:u8){if amdgpu_device_skip_hw_access(a){return}if o<(*a).rmmio_size{writeb(v,(*a).rmmio.add(o as usize))}else{dev_err((*a).dev,"invalid MMIO write offset 0x%x (rmmio size 0x%x)\n",o,(*a).rmmio_size as u32)}}

pub unsafe fn amdgpu_device_wreg(a:*mut amdgpu_device,r:u32,v:u32,f:u32){if amdgpu_device_skip_hw_access(a){return}if r*4<(*a).rmmio_size{if f&AMDGPU_REGS_NO_KIQ==0&&amdgpu_sriov_runtime(a)&&down_read_trylock(&(*a).reset_domain.sem){amdgpu_kiq_wreg(a,r,v,0);up_read(&(*a).reset_domain.sem)}else{writel(v,(*a).rmmio.add((r*4)as usize))}}else{amdgpu_reg_pcie_wr32(a,r*4,v)}trace_amdgpu_device_wreg((*a).pdev.device,r,v)}
pub unsafe fn amdgpu_device_xcc_rreg(a:*mut amdgpu_device,r:u32,f:u32,x:u32)->u32{if amdgpu_device_skip_hw_access(a){return 0}if r*4<(*a).rmmio_size{if amdgpu_sriov_vf(a)&&!amdgpu_sriov_runtime(a)&&(*a).gfx.rlc.rlcg_reg_access_supported{let mut q=0; if amdgpu_virt_get_rlcg_reg_access_flag(a,f,GC_HWIP,false,&mut q){return amdgpu_virt_rlcg_reg_rw(a,r,0,q,GET_INST!(GC,x))}}if f&AMDGPU_REGS_NO_KIQ==0&&amdgpu_sriov_runtime(a)&&down_read_trylock(&(*a).reset_domain.sem){let z=amdgpu_kiq_rreg(a,r,x);up_read(&(*a).reset_domain.sem);z}else{readl((*a).rmmio.add((r*4)as usize))}}else{amdgpu_reg_pcie_rd32(a,r*4)}}
pub unsafe fn amdgpu_device_xcc_wreg(a:*mut amdgpu_device,r:u32,v:u32,f:u32,x:u32){if amdgpu_device_skip_hw_access(a){return}if r*4<(*a).rmmio_size{if amdgpu_sriov_vf(a)&&!amdgpu_sriov_runtime(a)&&(*a).gfx.rlc.rlcg_reg_access_supported{let mut q=0;if amdgpu_virt_get_rlcg_reg_access_flag(a,f,GC_HWIP,true,&mut q){amdgpu_virt_rlcg_reg_rw(a,r,v,q,GET_INST!(GC,x));return}}if f&AMDGPU_REGS_NO_KIQ==0&&amdgpu_sriov_runtime(a)&&down_read_trylock(&(*a).reset_domain.sem){amdgpu_kiq_wreg(a,r,v,x);up_read(&(*a).reset_domain.sem)}else{writel(v,(*a).rmmio.add((r*4)as usize))}}else{amdgpu_reg_pcie_wr32(a,r*4,v)}}

pub unsafe fn amdgpu_mm_wreg_mmio_rlc(a:*mut amdgpu_device,r:u32,v:u32,x:u32){if amdgpu_device_skip_hw_access(a){return}if amdgpu_sriov_fullaccess(a)&&(*a).gfx.rlc.funcs.is_some()&&(*(*a).gfx.rlc.funcs).is_rlcg_access_range.is_some(){if ((*(*a).gfx.rlc.funcs).is_rlcg_access_range.unwrap())(a,r){amdgpu_sriov_wreg(a,r,v,0,0,x);return}}else if r*4>=(*a).rmmio_size{amdgpu_reg_pcie_wr32(a,r*4,v)}else{writel(v,(*a).rmmio.add((r*4)as usize))}}

pub unsafe fn amdgpu_device_indirect_rreg(a:*mut amdgpu_device,ra:u32)->u32{let i=((*a).nbio.funcs).get_pcie_index_offset(a);let d=((*a).nbio.funcs).get_pcie_data_offset(a);let mut f=0;spin_lock_irqsave(&mut (*a).reg.pcie.lock,&mut f);let ip=(*a).rmmio.add((i*4)as usize);let dp=(*a).rmmio.add((d*4)as usize);writel(ra,ip);readl(ip);let r=readl(dp);spin_unlock_irqrestore(&mut (*a).reg.pcie.lock,f);r}
pub unsafe fn amdgpu_device_indirect_rreg_ext(a:*mut amdgpu_device,ra:u64)->u32{let (i,d) = if (*a).nbio.funcs.is_null(){(AMDGPU_PCIE_INDEX_FALLBACK,AMDGPU_PCIE_DATA_FALLBACK)}else{(((*a).nbio.funcs).get_pcie_index_offset(a) as usize,((*a).nbio.funcs).get_pcie_data_offset(a) as usize)};let hi=if ra>>32!=0{if (*a).nbio.funcs.is_null(){AMDGPU_PCIE_INDEX_HI_FALLBACK}else{((*a).nbio.funcs).get_pcie_index_hi_offset(a) as usize}}else{0};let mut f=0;spin_lock_irqsave(&mut (*a).reg.pcie.lock,&mut f);let ip=(*a).rmmio.add(i*4);let dp=(*a).rmmio.add(d*4);let hp=(*a).rmmio.add(hi*4);writel(ra as u32,ip);readl(ip);if hi!=0{writel(((ra>>32)&0xff)as u32,hp);readl(hp)}let r=readl(dp);if hi!=0{writel(0,hp);readl(hp)}spin_unlock_irqrestore(&mut (*a).reg.pcie.lock,f);r}

pub unsafe fn amdgpu_device_wait_on_rreg(a:*mut amdgpu_device,inst:u32,ra:u32,name:*const i8,expected:u32,mask:u32)->u32{let mut ret=0;let mut old=0;let mut tmp=RREG32!(a,ra);let mut loop_=(*a).usec_timeout;while tmp&mask!=expected{if old!=tmp{loop_=(*a).usec_timeout;old=tmp}else{udelay(1)}tmp=RREG32!(a,ra);loop_-=1;if loop_==0{dev_warn((*a).dev,"Register(%d) [%s] failed to reach value 0x%08x != 0x%08xn",inst,name,expected,tmp&mask);ret=-ETIMEDOUT as u32;break}}ret}

pub unsafe fn amdgpu_read_indexed_register(a:*mut amdgpu_device,se:u32,sh:u32,off:u32)->u32{mutex_lock(&mut (*a).grbm_idx_mutex);if se!=0xffffffff||sh!=0xffffffff{amdgpu_gfx_select_se_sh(a,se,sh,0xffffffff,0)}let v=RREG32!(a,off);if se!=0xffffffff||sh!=0xffffffff{amdgpu_gfx_select_se_sh(a,0xffffffff,0xffffffff,0xffffffff,0)}mutex_unlock(&mut (*a).grbm_idx_mutex);v}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
