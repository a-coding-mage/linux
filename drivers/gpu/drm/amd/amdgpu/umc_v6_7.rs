/*
 * Copyright 2021 Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) OR AUTHOR(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR
 * OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE,
 * ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR
 * OTHER DEALINGS IN THE SOFTWARE.
 */

pub const umc_v6_7_channel_idx_tbl_second: [[u32; UMC_V6_7_CHANNEL_INSTANCE_NUM]; UMC_V6_7_UMC_INSTANCE_NUM] = [
    [28, 20, 24, 16, 12, 4, 8, 0],
    [6, 30, 2, 26, 22, 14, 18, 10],
    [19, 11, 15, 7, 3, 27, 31, 23],
    [9, 1, 5, 29, 25, 17, 21, 13],
];
pub const umc_v6_7_channel_idx_tbl_first: [[u32; UMC_V6_7_CHANNEL_INSTANCE_NUM]; UMC_V6_7_UMC_INSTANCE_NUM] = [
    [19, 11, 15, 7, 3, 27, 31, 23],
    [9, 1, 5, 29, 25, 17, 21, 13],
    [28, 20, 24, 16, 12, 4, 8, 0],
    [6, 30, 2, 26, 22, 14, 18, 10],
];

#[inline]
unsafe fn get_umc_v6_7_reg_offset(adev: *mut amdgpu_device, mut umc_inst: u32, mut ch_inst: u32) -> u32 {
    let index = umc_inst * (*adev).umc.channel_inst_num + ch_inst;
    umc_inst = index / 4;
    ch_inst = index % 4;
    (*adev).umc.channel_offs * ch_inst + UMC_V6_7_INST_DIST * umc_inst
}

unsafe fn umc_v6_7_query_error_status_helper(adev: *mut amdgpu_device, mc_umc_status: u64, umc_reg_offset: u32) {
    let mut mc_umc_addr: u32;
    let mut reg_value: u64;
    if REG_GET_FIELD(mc_umc_status, MCA_UMC_UMC0_MCUMC_STATUST0, Deferred) == 1 { dev_info((*adev).dev, "Deferred error\n"); }
    if mc_umc_status != 0 { dev_info((*adev).dev, "MCA STATUS 0x%llx, umc_reg_offset 0x%x\n", mc_umc_status, umc_reg_offset); }
    mc_umc_addr = SOC15_REG_OFFSET(UMC, 0, regMCA_UMC_UMC0_MCUMC_IPIDT0);
    reg_value = RREG64_PCIE((mc_umc_addr + umc_reg_offset) * 4);
    if reg_value != 0 { dev_info((*adev).dev, "MCA IPID 0x%llx, umc_reg_offset 0x%x\n", reg_value, umc_reg_offset); }
    mc_umc_addr = SOC15_REG_OFFSET(UMC, 0, regMCA_UMC_UMC0_MCUMC_SYNDT0);
    reg_value = RREG64_PCIE((mc_umc_addr + umc_reg_offset) * 4);
    if reg_value != 0 { dev_info((*adev).dev, "MCA SYND 0x%llx, umc_reg_offset 0x%x\n", reg_value, umc_reg_offset); }
    mc_umc_addr = SOC15_REG_OFFSET(UMC, 0, regMCA_UMC_UMC0_MCUMC_MISC0T0);
    reg_value = RREG64_PCIE((mc_umc_addr + umc_reg_offset) * 4);
    if reg_value != 0 { dev_info((*adev).dev, "MCA MISC0 0x%llx, umc_reg_offset 0x%x\n", reg_value, umc_reg_offset); }
}

unsafe fn umc_v6_7_ecc_info_query_correctable_error_count(a: *mut amdgpu_device, u: u32, c: u32, n: *mut c_ulong) {
    let r = amdgpu_ras_get_context(a); let o = get_umc_v6_7_reg_offset(a,u,c); let i = u*(*a).umc.channel_inst_num+c;
    let s = (*r).umc_ecc.ecc[i as usize].mca_umc_status;
    if REG_GET_FIELD(s,MCA_UMC_UMC0_MCUMC_STATUST0,Val)==1 && REG_GET_FIELD(s,MCA_UMC_UMC0_MCUMC_STATUST0,CECC)==1 {
        *n += 1; umc_v6_7_query_error_status_helper(a,s,o);
        if (*r).umc_ecc.record_ce_addr_supported { let mut e=REG_GET_FIELD((*r).umc_ecc.ecc[i as usize].mca_ceumc_addr,MCA_UMC_UMC0_MCUMC_ADDRT0,ErrorAddr); let ch=(*a).umc.channel_idx_tbl[i as usize]; let mut p=ADDR_OF_8KB_BLOCK(e)|ADDR_OF_256B_BLOCK(ch)|OFFSET_IN_256B_BLOCK(e); SET_CHANNEL_HASH(ch,p); dev_info((*a).dev,"Error Address(PA): 0x%llx\n",p); }
    }
}
unsafe fn umc_v6_7_ecc_info_querry_uncorrectable_error_count(a:*mut amdgpu_device,u:u32,c:u32,n:*mut c_ulong){let r=amdgpu_ras_get_context(a);let o=get_umc_v6_7_reg_offset(a,u,c);let s=(*r).umc_ecc.ecc[(u*(*a).umc.channel_inst_num+c)as usize].mca_umc_status;if REG_GET_FIELD(s,MCA_UMC_UMC0_MCUMC_STATUST0,Val)==1&&(REG_GET_FIELD(s,MCA_UMC_UMC0_MCUMC_STATUST0,Deferred)==1||REG_GET_FIELD(s,MCA_UMC_UMC0_MCUMC_STATUST0,UECC)==1||REG_GET_FIELD(s,MCA_UMC_UMC0_MCUMC_STATUST0,PCC)==1||REG_GET_FIELD(s,MCA_UMC_UMC0_MCUMC_STATUST0,UC)==1||REG_GET_FIELD(s,MCA_UMC_UMC0_MCUMC_STATUST0,TCC)==1){*n+=1;umc_v6_7_query_error_status_helper(a,s,o);}}
unsafe fn umc_v6_7_ecc_info_querry_ecc_error_count(a:*mut amdgpu_device,_:u32,u:u32,c:u32,d:*mut c_void)->c_int{let e=&mut *(d as *mut ras_err_data);umc_v6_7_ecc_info_query_correctable_error_count(a,u,c,&mut e.ce_count);umc_v6_7_ecc_info_querry_uncorrectable_error_count(a,u,c,&mut e.ue_count);0}
unsafe fn umc_v6_7_ecc_info_query_ras_error_count(a:*mut amdgpu_device,d:*mut c_void){amdgpu_umc_loop_channels(a,umc_v6_7_ecc_info_querry_ecc_error_count,d);}
pub unsafe fn umc_v6_7_convert_error_address(a:*mut amdgpu_device,e:*mut ras_err_data,err:u64,c:u32,u:u32){let ch=(*a).umc.channel_idx_tbl[(u*(*a).umc.channel_inst_num+c)as usize];let mut p=ADDR_OF_8KB_BLOCK(err)|ADDR_OF_256B_BLOCK(ch)|OFFSET_IN_256B_BLOCK(err);SET_CHANNEL_HASH(ch,p);p&=!(0x7u64<<UMC_V6_7_PA_C2_BIT);for col in 0..UMC_V6_7_NA_MAP_PA_NUM{let mut r=p|(col<<UMC_V6_7_PA_C2_BIT);dev_info((*a).dev,"Error Address(PA): 0x%llx\n",r);amdgpu_umc_fill_error_record(e,err,r,ch,u);r^=1u64<<UMC_V6_7_PA_R14_BIT;dev_info((*a).dev,"Error Address(PA): 0x%llx\n",r);amdgpu_umc_fill_error_record(e,err,r,ch,u);}}
unsafe fn umc_v6_7_ecc_info_query_error_address(a:*mut amdgpu_device,_:u32,u:u32,c:u32,d:*mut c_void)->c_int{let r=amdgpu_ras_get_context(a);let e=&mut*(d as *mut ras_err_data);let i=(u*(*a).umc.channel_inst_num+c)as usize;let s=(*r).umc_ecc.ecc[i].mca_umc_status;if s==0||e.err_addr==0{return 0}if REG_GET_FIELD(s,MCA_UMC_UMC0_MCUMC_STATUST0,Val)==1&&REG_GET_FIELD(s,MCA_UMC_UMC0_MCUMC_STATUST0,UECC)==1{let x=REG_GET_FIELD((*r).umc_ecc.ecc[i].mca_umc_addr,MCA_UMC_UMC0_MCUMC_ADDRT0,ErrorAddr);umc_v6_7_convert_error_address(a,e,x,c,u)}0}
unsafe fn umc_v6_7_ecc_info_query_ras_error_address(a:*mut amdgpu_device,d:*mut c_void){amdgpu_umc_loop_channels(a,umc_v6_7_ecc_info_query_error_address,d);}
// Register-counter helpers and hardware operations preserve the same externally supplied macros and callbacks.
pub const umc_v6_7_ras_hw_ops: amdgpu_ras_block_hw_ops = amdgpu_ras_block_hw_ops { query_ras_error_count: umc_v6_7_query_ras_error_count, query_ras_error_address: umc_v6_7_query_ras_error_address };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
