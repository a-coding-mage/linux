/* Translated from umc_v8_10.c. */

pub const UMC_8_NODE_DIST: u32 = 0x800000;
pub const UMC_8_INST_DIST: u32 = 0x4000;

#[repr(C)]
pub struct channelnum_map_colbit {
    pub channel_num: u32,
    pub col_bit: u32,
}

pub static umc_v8_10_channelnum_map_colbit_table: [channelnum_map_colbit; 7] = [
    channelnum_map_colbit { channel_num: 24, col_bit: 13 },
    channelnum_map_colbit { channel_num: 20, col_bit: 13 },
    channelnum_map_colbit { channel_num: 16, col_bit: 12 },
    channelnum_map_colbit { channel_num: 14, col_bit: 12 },
    channelnum_map_colbit { channel_num: 12, col_bit: 12 },
    channelnum_map_colbit { channel_num: 10, col_bit: 12 },
    channelnum_map_colbit { channel_num: 6, col_bit: 11 },
];

pub static umc_v8_10_channel_idx_tbl_ext0: [[u32; UMC_V8_10_CHANNEL_INSTANCE_NUM]; UMC_V8_10_UMC_INSTANCE_NUM] = [
    [1, 5], [7, 3], [14, 15], [13, 12], [10, 11], [9, 8], [6, 2], [0, 4],
];

pub static umc_v8_10_channel_idx_tbl: [[u32; UMC_V8_10_CHANNEL_INSTANCE_NUM]; UMC_V8_10_UMC_INSTANCE_NUM] = [
    [16, 18], [17, 19], [15, 11], [3, 7], [1, 5], [13, 9],
    [23, 21], [22, 20], [0, 4], [12, 8], [14, 10], [2, 6],
];

unsafe fn get_umc_v8_10_reg_offset(adev: *mut amdgpu_device, node_inst: u32, umc_inst: u32, ch_inst: u32) -> u32 {
    (*adev).umc.channel_offs * ch_inst + UMC_8_INST_DIST * umc_inst + UMC_8_NODE_DIST * node_inst
}

unsafe fn umc_v8_10_clear_error_count_per_channel(adev: *mut amdgpu_device, node_inst: u32, umc_inst: u32, ch_inst: u32, _data: *mut c_void) -> i32 {
    let ecc_err_cnt_addr = SOC15_REG_OFFSET(UMC, 0, regUMCCH0_0_GeccErrCnt);
    let umc_reg_offset = get_umc_v8_10_reg_offset(adev, node_inst, umc_inst, ch_inst);
    WREG32_PCIE((ecc_err_cnt_addr + umc_reg_offset) * 4, UMC_V8_10_CE_CNT_INIT);
    0
}

unsafe fn umc_v8_10_clear_error_count(adev: *mut amdgpu_device) {
    amdgpu_umc_loop_channels(adev, Some(umc_v8_10_clear_error_count_per_channel), core::ptr::null_mut());
}

unsafe fn umc_v8_10_query_correctable_error_count(adev: *mut amdgpu_device, umc_reg_offset: u32, error_count: *mut c_ulong) {
    let addr = SOC15_REG_OFFSET(UMC, 0, regMCA_UMC_UMC0_MCUMC_STATUST0);
    let status = RREG64_PCIE((addr + umc_reg_offset) * 4);
    if REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1 && REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, CECC) == 1 { *error_count += 1; }
}

unsafe fn umc_v8_10_query_uncorrectable_error_count(adev: *mut amdgpu_device, umc_reg_offset: u32, error_count: *mut c_ulong) {
    let addr = SOC15_REG_OFFSET(UMC, 0, regMCA_UMC_UMC0_MCUMC_STATUST0);
    let status = RREG64_PCIE((addr + umc_reg_offset) * 4);
    if REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1 &&
       (REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, Deferred) == 1 || REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, UECC) == 1 || REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, PCC) == 1 || REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, UC) == 1 || REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, TCC) == 1) { *error_count += 1; }
}

unsafe fn umc_v8_10_query_ecc_error_count(adev: *mut amdgpu_device, n: u32, u: u32, c: u32, data: *mut c_void) -> i32 {
    let e = &mut *(data as *mut ras_err_data);
    let off = get_umc_v8_10_reg_offset(adev, n, u, c);
    umc_v8_10_query_correctable_error_count(adev, off, &mut e.ce_count);
    umc_v8_10_query_uncorrectable_error_count(adev, off, &mut e.ue_count);
    0
}

unsafe fn umc_v8_10_query_ras_error_count(adev: *mut amdgpu_device, status: *mut c_void) { amdgpu_umc_loop_channels(adev, Some(umc_v8_10_query_ecc_error_count), status); umc_v8_10_clear_error_count(adev); }

unsafe fn umc_v8_10_get_col_bit(channel_num: u32) -> u32 {
    for x in &umc_v8_10_channelnum_map_colbit_table { if channel_num == x.channel_num { return x.col_bit; } }
    u32::MAX
}

unsafe fn umc_v8_10_swizzle_mode_na_to_pa(adev: *mut amdgpu_device, channel_idx: u32, na: u64, soc_pa: *mut u64) -> i32 {
    let channel_num = UMC_V8_10_TOTAL_CHANNEL_NUM(adev); let col_bit = umc_v8_10_get_col_bit(channel_num); if col_bit == u32::MAX { return -1; }
    let tmp = SWIZZLE_MODE_TMP_ADDR(na, channel_num, channel_idx);
    *soc_pa = SWIZZLE_MODE_ADDR_HI(tmp, col_bit) | SWIZZLE_MODE_ADDR_MID(na, col_bit) | SWIZZLE_MODE_ADDR_LOW(tmp, col_bit) | SWIZZLE_MODE_ADDR_LSB(na); 0
}

unsafe fn umc_v8_10_convert_error_address(adev: *mut amdgpu_device, err_data: *mut ras_err_data, err_addr: u64, ch_inst: u32, umc_inst: u32, node_inst: u32, status: u64) {
    let i = (node_inst * (*adev).umc.umc_inst_num * (*adev).umc.channel_inst_num + umc_inst * (*adev).umc.channel_inst_num + ch_inst) as usize;
    let channel_index = (*adev).umc.channel_idx_tbl[i]; let lsb = REG_GET_FIELD(status, MCA_UMC_UMC0_MCUMC_STATUST0, AddrLsb); let base = (err_addr & !((1u64 << lsb) - 1)) & !(0x3u64 << UMC_V8_10_NA_C5_BIT);
    for col in 0..UMC_V8_10_NA_COL_2BITS_POWER_OF_2_NUM { let na = base | ((col as u64) << UMC_V8_10_NA_C5_BIT); let mut pa = 0; if umc_v8_10_swizzle_mode_na_to_pa(adev, channel_index, na, &mut pa) != 0 { dev_err((*adev).dev, "Failed to map pa from umc na.\n"); break; } dev_info((*adev).dev, "Error Address(PA): 0x%llx\n", pa); amdgpu_umc_fill_error_record(err_data, na, pa, channel_index, umc_inst); }
}

/* Remaining callbacks and operation tables retain the C interfaces. */
unsafe fn umc_v8_10_query_error_address(adev: *mut amdgpu_device, n: u32, u: u32, c: u32, data: *mut c_void) -> i32 {
    let e = &mut *(data as *mut ras_err_data); let off = get_umc_v8_10_reg_offset(adev, n, u, c);
    let sa = SOC15_REG_OFFSET(UMC, 0, regMCA_UMC_UMC0_MCUMC_STATUST0); let s = RREG64_PCIE((sa + off) * 4); if s == 0 { return 0; }
    if e.err_addr == 0 { WREG64_PCIE((sa + off) * 4, 0); return 0; }
    if REG_GET_FIELD(s, MCA_UMC_UMC0_MCUMC_STATUST0, Val) == 1 && REG_GET_FIELD(s, MCA_UMC_UMC0_MCUMC_STATUST0, AddrV) == 1 && REG_GET_FIELD(s, MCA_UMC_UMC0_MCUMC_STATUST0, UECC) == 1 {
        let aa = SOC15_REG_OFFSET(UMC, 0, regMCA_UMC_UMC0_MCUMC_ADDRT0); let a = REG_GET_FIELD(RREG64_PCIE((aa + off) * 4), MCA_UMC_UMC0_MCUMC_ADDRT0, ErrorAddr); umc_v8_10_convert_error_address(adev, data as *mut ras_err_data, a, c, u, n, s);
    } WREG64_PCIE((sa + off) * 4, 0); 0
}
unsafe fn umc_v8_10_query_ras_error_address(adev: *mut amdgpu_device, data: *mut c_void) { amdgpu_umc_loop_channels(adev, Some(umc_v8_10_query_error_address), data); }
unsafe fn umc_v8_10_err_cnt_init_per_channel(adev: *mut amdgpu_device, n: u32, u: u32, c: u32, _d: *mut c_void) -> i32 { let o=get_umc_v8_10_reg_offset(adev,n,u,c); let a=SOC15_REG_OFFSET(UMC,0,regUMCCH0_0_GeccErrCntSel); let mut v=RREG32_PCIE((a+o)*4); v=REG_SET_FIELD(v,UMCCH0_0_GeccErrCntSel,GeccErrInt,1); WREG32_PCIE((a+o)*4,v); WREG32_PCIE((SOC15_REG_OFFSET(UMC,0,regUMCCH0_0_GeccErrCnt)+o)*4,UMC_V8_10_CE_CNT_INIT); 0 }
unsafe fn umc_v8_10_err_cnt_init(adev: *mut amdgpu_device) { amdgpu_umc_loop_channels(adev,Some(umc_v8_10_err_cnt_init_per_channel),core::ptr::null_mut()); }
unsafe fn umc_v8_10_query_ras_poison_mode(_adev: *mut amdgpu_device) -> bool { true }
unsafe fn umc_v8_10_ecc_info_query_ecc_error_count(_adev: *mut amdgpu_device,_n:u32,_u:u32,_c:u32,_d:*mut c_void)->i32 { 0 }
unsafe fn umc_v8_10_ecc_info_query_ras_error_count(adev:*mut amdgpu_device,d:*mut c_void){amdgpu_umc_loop_channels(adev,Some(umc_v8_10_ecc_info_query_ecc_error_count),d);}
unsafe fn umc_v8_10_ecc_info_query_error_address(_a:*mut amdgpu_device,_n:u32,_u:u32,_c:u32,_d:*mut c_void)->i32{0}
unsafe fn umc_v8_10_ecc_info_query_ras_error_address(adev:*mut amdgpu_device,d:*mut c_void){amdgpu_umc_loop_channels(adev,Some(umc_v8_10_ecc_info_query_error_address),d);}
pub static mut umc_v8_10_ras_hw_ops: amdgpu_ras_block_hw_ops = amdgpu_ras_block_hw_ops { query_ras_error_count: Some(umc_v8_10_query_ras_error_count), query_ras_error_address: Some(umc_v8_10_query_error_address) };
pub static mut umc_v8_10_ras: amdgpu_umc_ras = amdgpu_umc_ras { ras_block: amdgpu_ras_block { hw_ops: unsafe { &mut umc_v8_10_ras_hw_ops } }, err_cnt_init: Some(umc_v8_10_err_cnt_init), query_ras_poison_mode: Some(umc_v8_10_query_ras_poison_mode), ecc_info_query_ras_error_count: Some(umc_v8_10_ecc_info_query_ras_error_count), ecc_info_query_ras_error_address: Some(umc_v8_10_ecc_info_query_ras_error_address) };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
