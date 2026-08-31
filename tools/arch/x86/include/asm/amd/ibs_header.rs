/* SPDX-License-Identifier: GPL-2.0 */

/*
 * From PPR Vol 1 for AMD Family 19h Model 01h B1
 * 55898 Rev 0.35 - Feb 5, 2021
 */

/* Depends on ../msr-index.h for MSR_AMD64_IBS_REG_COUNT_MAX. */

/* IBS_OP_DATA2 DataSrc */
pub const IBS_DATA_SRC_LOC_CACHE: u64 = 2;
pub const IBS_DATA_SRC_DRAM: u64 = 3;
pub const IBS_DATA_SRC_REM_CACHE: u64 = 4;
pub const IBS_DATA_SRC_IO: u64 = 7;

/* IBS_OP_DATA2 DataSrc Extension */
pub const IBS_DATA_SRC_EXT_LOC_CACHE: u64 = 1;
pub const IBS_DATA_SRC_EXT_NEAR_CCX_CACHE: u64 = 2;
pub const IBS_DATA_SRC_EXT_DRAM: u64 = 3;
pub const IBS_DATA_SRC_EXT_FAR_CCX_CACHE: u64 = 5;
pub const IBS_DATA_SRC_EXT_PMEM: u64 = 6;
pub const IBS_DATA_SRC_EXT_IO: u64 = 7;
pub const IBS_DATA_SRC_EXT_EXT_MEM: u64 = 8;
pub const IBS_DATA_SRC_EXT_PEER_AGENT_MEM: u64 = 12;

/*
 * IBS Hardware MSRs
 */

#[inline]
pub const fn __ibs_get_bits(val: u64, shift: u32, width: u32) -> u64 {
    (val >> shift) & ((1u64 << width) - 1)
}

#[inline]
pub fn __ibs_set_bits(val: &mut u64, shift: u32, width: u32, field: u64) {
    let mask = ((1u64 << width) - 1) << shift;
    *val = (*val & !mask) | ((field << shift) & mask);
}

/* MSR 0xc0011030: IBS Fetch Control */
#[repr(C)]
#[derive(Copy, Clone)]
pub union ibs_fetch_ctl {
    pub val: u64,
}

impl ibs_fetch_ctl {
    /* 0-15: instruction fetch max. count */
    #[inline]
    pub unsafe fn fetch_maxcnt(&self) -> u64 { __ibs_get_bits(self.val, 0, 16) }
    #[inline]
    pub unsafe fn set_fetch_maxcnt(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 0, 16, value); }
    /* 16-31: instruction fetch count */
    #[inline]
    pub unsafe fn fetch_cnt(&self) -> u64 { __ibs_get_bits(self.val, 16, 16) }
    #[inline]
    pub unsafe fn set_fetch_cnt(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 16, 16, value); }
    /* 32-47: instruction fetch latency */
    #[inline]
    pub unsafe fn fetch_lat(&self) -> u64 { __ibs_get_bits(self.val, 32, 16) }
    #[inline]
    pub unsafe fn set_fetch_lat(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 32, 16, value); }
    /* 48: instruction fetch enable */
    #[inline]
    pub unsafe fn fetch_en(&self) -> u64 { __ibs_get_bits(self.val, 48, 1) }
    #[inline]
    pub unsafe fn set_fetch_en(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 48, 1, value); }
    /* 49: instruction fetch valid */
    #[inline]
    pub unsafe fn fetch_val(&self) -> u64 { __ibs_get_bits(self.val, 49, 1) }
    #[inline]
    pub unsafe fn set_fetch_val(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 49, 1, value); }
    /* 50: instruction fetch complete */
    #[inline]
    pub unsafe fn fetch_comp(&self) -> u64 { __ibs_get_bits(self.val, 50, 1) }
    #[inline]
    pub unsafe fn set_fetch_comp(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 50, 1, value); }
    /* 51: i-cache miss */
    #[inline]
    pub unsafe fn ic_miss(&self) -> u64 { __ibs_get_bits(self.val, 51, 1) }
    #[inline]
    pub unsafe fn set_ic_miss(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 51, 1, value); }
    /* 52: physical address valid */
    #[inline]
    pub unsafe fn phy_addr_valid(&self) -> u64 { __ibs_get_bits(self.val, 52, 1) }
    #[inline]
    pub unsafe fn set_phy_addr_valid(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 52, 1, value); }
    /* 53-54: i-cache L1TLB page size (needs IbsPhyAddrValid) */
    #[inline]
    pub unsafe fn l1tlb_pgsz(&self) -> u64 { __ibs_get_bits(self.val, 53, 2) }
    #[inline]
    pub unsafe fn set_l1tlb_pgsz(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 53, 2, value); }
    /* 55: i-cache fetch missed in L1TLB */
    #[inline]
    pub unsafe fn l1tlb_miss(&self) -> u64 { __ibs_get_bits(self.val, 55, 1) }
    #[inline]
    pub unsafe fn set_l1tlb_miss(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 55, 1, value); }
    /* 56: i-cache fetch missed in L2TLB */
    #[inline]
    pub unsafe fn l2tlb_miss(&self) -> u64 { __ibs_get_bits(self.val, 56, 1) }
    #[inline]
    pub unsafe fn set_l2tlb_miss(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 56, 1, value); }
    /* 57: random tagging enable */
    #[inline]
    pub unsafe fn rand_en(&self) -> u64 { __ibs_get_bits(self.val, 57, 1) }
    #[inline]
    pub unsafe fn set_rand_en(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 57, 1, value); }
    /* 58: L2 miss for sampled fetch (needs IbsFetchComp) */
    #[inline]
    pub unsafe fn fetch_l2_miss(&self) -> u64 { __ibs_get_bits(self.val, 58, 1) }
    #[inline]
    pub unsafe fn set_fetch_l2_miss(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 58, 1, value); }
    /* 59: Collect L3 miss samples only */
    #[inline]
    pub unsafe fn l3_miss_only(&self) -> u64 { __ibs_get_bits(self.val, 59, 1) }
    #[inline]
    pub unsafe fn set_l3_miss_only(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 59, 1, value); }
    /* 60: Op cache miss for the sampled fetch */
    #[inline]
    pub unsafe fn fetch_oc_miss(&self) -> u64 { __ibs_get_bits(self.val, 60, 1) }
    #[inline]
    pub unsafe fn set_fetch_oc_miss(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 60, 1, value); }
    /* 61: L3 cache miss for the sampled fetch */
    #[inline]
    pub unsafe fn fetch_l3_miss(&self) -> u64 { __ibs_get_bits(self.val, 61, 1) }
    #[inline]
    pub unsafe fn set_fetch_l3_miss(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 61, 1, value); }
    /* 62-63: reserved */
    #[inline]
    pub unsafe fn reserved(&self) -> u64 { __ibs_get_bits(self.val, 62, 2) }
    #[inline]
    pub unsafe fn set_reserved(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 62, 2, value); }
}

/* MSR 0xc0011033: IBS Execution Control */
#[repr(C)]
#[derive(Copy, Clone)]
pub union ibs_op_ctl {
    pub val: u64,
}

impl ibs_op_ctl {
    /* 0-15: periodic op max. count */
    #[inline]
    pub unsafe fn opmaxcnt(&self) -> u64 { __ibs_get_bits(self.val, 0, 16) }
    #[inline]
    pub unsafe fn set_opmaxcnt(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 0, 16, value); }
    /* 16: Collect L3 miss samples only */
    #[inline]
    pub unsafe fn l3_miss_only(&self) -> u64 { __ibs_get_bits(self.val, 16, 1) }
    #[inline]
    pub unsafe fn set_l3_miss_only(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 16, 1, value); }
    /* 17: op sampling enable */
    #[inline]
    pub unsafe fn op_en(&self) -> u64 { __ibs_get_bits(self.val, 17, 1) }
    #[inline]
    pub unsafe fn set_op_en(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 17, 1, value); }
    /* 18: op sample valid */
    #[inline]
    pub unsafe fn op_val(&self) -> u64 { __ibs_get_bits(self.val, 18, 1) }
    #[inline]
    pub unsafe fn set_op_val(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 18, 1, value); }
    /* 19: periodic op counter control */
    #[inline]
    pub unsafe fn cnt_ctl(&self) -> u64 { __ibs_get_bits(self.val, 19, 1) }
    #[inline]
    pub unsafe fn set_cnt_ctl(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 19, 1, value); }
    /* 20-26: upper 7 bits of periodic op maximum count */
    #[inline]
    pub unsafe fn opmaxcnt_ext(&self) -> u64 { __ibs_get_bits(self.val, 20, 7) }
    #[inline]
    pub unsafe fn set_opmaxcnt_ext(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 20, 7, value); }
    /* 27-31: reserved */
    #[inline]
    pub unsafe fn reserved0(&self) -> u64 { __ibs_get_bits(self.val, 27, 5) }
    #[inline]
    pub unsafe fn set_reserved0(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 27, 5, value); }
    /* 32-58: periodic op counter current count */
    #[inline]
    pub unsafe fn opcurcnt(&self) -> u64 { __ibs_get_bits(self.val, 32, 27) }
    #[inline]
    pub unsafe fn set_opcurcnt(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 32, 27, value); }
    /* 59-62: Load Latency threshold */
    #[inline]
    pub unsafe fn ldlat_thrsh(&self) -> u64 { __ibs_get_bits(self.val, 59, 4) }
    #[inline]
    pub unsafe fn set_ldlat_thrsh(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 59, 4, value); }
    /* 63: Load Latency enabled */
    #[inline]
    pub unsafe fn ldlat_en(&self) -> u64 { __ibs_get_bits(self.val, 63, 1) }
    #[inline]
    pub unsafe fn set_ldlat_en(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 63, 1, value); }
}

/* MSR 0xc0011035: IBS Op Data 1 */
#[repr(C)]
#[derive(Copy, Clone)]
pub union ibs_op_data {
    pub val: u64,
}

impl ibs_op_data {
    /* 0-15: op completion to retire count */
    #[inline]
    pub unsafe fn comp_to_ret_ctr(&self) -> u64 { __ibs_get_bits(self.val, 0, 16) }
    #[inline]
    pub unsafe fn set_comp_to_ret_ctr(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 0, 16, value); }
    /* 16-31: op tag to retire count */
    #[inline]
    pub unsafe fn tag_to_ret_ctr(&self) -> u64 { __ibs_get_bits(self.val, 16, 16) }
    #[inline]
    pub unsafe fn set_tag_to_ret_ctr(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 16, 16, value); }
    /* 32-33: reserved */
    #[inline]
    pub unsafe fn reserved1(&self) -> u64 { __ibs_get_bits(self.val, 32, 2) }
    #[inline]
    pub unsafe fn set_reserved1(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 32, 2, value); }
    /* 34: return op */
    #[inline]
    pub unsafe fn op_return(&self) -> u64 { __ibs_get_bits(self.val, 34, 1) }
    #[inline]
    pub unsafe fn set_op_return(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 34, 1, value); }
    /* 35: taken branch op */
    #[inline]
    pub unsafe fn op_brn_taken(&self) -> u64 { __ibs_get_bits(self.val, 35, 1) }
    #[inline]
    pub unsafe fn set_op_brn_taken(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 35, 1, value); }
    /* 36: mispredicted branch op */
    #[inline]
    pub unsafe fn op_brn_misp(&self) -> u64 { __ibs_get_bits(self.val, 36, 1) }
    #[inline]
    pub unsafe fn set_op_brn_misp(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 36, 1, value); }
    /* 37: branch op retired */
    #[inline]
    pub unsafe fn op_brn_ret(&self) -> u64 { __ibs_get_bits(self.val, 37, 1) }
    #[inline]
    pub unsafe fn set_op_brn_ret(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 37, 1, value); }
    /* 38: RIP is invalid */
    #[inline]
    pub unsafe fn op_rip_invalid(&self) -> u64 { __ibs_get_bits(self.val, 38, 1) }
    #[inline]
    pub unsafe fn set_op_rip_invalid(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 38, 1, value); }
    /* 39: fused branch op */
    #[inline]
    pub unsafe fn op_brn_fuse(&self) -> u64 { __ibs_get_bits(self.val, 39, 1) }
    #[inline]
    pub unsafe fn set_op_brn_fuse(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 39, 1, value); }
    /* 40: microcode op */
    #[inline]
    pub unsafe fn op_microcode(&self) -> u64 { __ibs_get_bits(self.val, 40, 1) }
    #[inline]
    pub unsafe fn set_op_microcode(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 40, 1, value); }
    /* 41-63: reserved */
    #[inline]
    pub unsafe fn reserved2(&self) -> u64 { __ibs_get_bits(self.val, 41, 23) }
    #[inline]
    pub unsafe fn set_reserved2(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 41, 23, value); }
}

/* MSR 0xc0011036: IBS Op Data 2 */
#[repr(C)]
#[derive(Copy, Clone)]
pub union ibs_op_data2 {
    pub val: u64,
}

impl ibs_op_data2 {
    /* 0-2: data source low */
    #[inline]
    pub unsafe fn data_src_lo(&self) -> u64 { __ibs_get_bits(self.val, 0, 3) }
    #[inline]
    pub unsafe fn set_data_src_lo(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 0, 3, value); }
    /* 3: reserved */
    #[inline]
    pub unsafe fn reserved0(&self) -> u64 { __ibs_get_bits(self.val, 3, 1) }
    #[inline]
    pub unsafe fn set_reserved0(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 3, 1, value); }
    /* 4: destination node */
    #[inline]
    pub unsafe fn rmt_node(&self) -> u64 { __ibs_get_bits(self.val, 4, 1) }
    #[inline]
    pub unsafe fn set_rmt_node(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 4, 1, value); }
    /* 5: cache hit state */
    #[inline]
    pub unsafe fn cache_hit_st(&self) -> u64 { __ibs_get_bits(self.val, 5, 1) }
    #[inline]
    pub unsafe fn set_cache_hit_st(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 5, 1, value); }
    /* 6-7: data source high */
    #[inline]
    pub unsafe fn data_src_hi(&self) -> u64 { __ibs_get_bits(self.val, 6, 2) }
    #[inline]
    pub unsafe fn set_data_src_hi(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 6, 2, value); }
    /* 8: streaming store */
    #[inline]
    pub unsafe fn strm_st(&self) -> u64 { __ibs_get_bits(self.val, 8, 1) }
    #[inline]
    pub unsafe fn set_strm_st(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 8, 1, value); }
    /* 9: remote socket */
    #[inline]
    pub unsafe fn rmt_socket(&self) -> u64 { __ibs_get_bits(self.val, 9, 1) }
    #[inline]
    pub unsafe fn set_rmt_socket(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 9, 1, value); }
    /* 10-63: reserved */
    #[inline]
    pub unsafe fn reserved1(&self) -> u64 { __ibs_get_bits(self.val, 10, 54) }
    #[inline]
    pub unsafe fn set_reserved1(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 10, 54, value); }
}

/* MSR 0xc0011037: IBS Op Data 3 */
#[repr(C)]
#[derive(Copy, Clone)]
pub union ibs_op_data3 {
    pub val: u64,
}

impl ibs_op_data3 {
    /* 0: load op */
    #[inline]
    pub unsafe fn ld_op(&self) -> u64 { __ibs_get_bits(self.val, 0, 1) }
    #[inline]
    pub unsafe fn set_ld_op(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 0, 1, value); }
    /* 1: store op */
    #[inline]
    pub unsafe fn st_op(&self) -> u64 { __ibs_get_bits(self.val, 1, 1) }
    #[inline]
    pub unsafe fn set_st_op(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 1, 1, value); }
    /* 2: data cache L1TLB miss */
    #[inline]
    pub unsafe fn dc_l1tlb_miss(&self) -> u64 { __ibs_get_bits(self.val, 2, 1) }
    #[inline]
    pub unsafe fn set_dc_l1tlb_miss(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 2, 1, value); }
    /* 3: data cache L2TLB miss in 2M page */
    #[inline]
    pub unsafe fn dc_l2tlb_miss(&self) -> u64 { __ibs_get_bits(self.val, 3, 1) }
    #[inline]
    pub unsafe fn set_dc_l2tlb_miss(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 3, 1, value); }
    /* 4: data cache L1TLB hit in 2M page */
    #[inline]
    pub unsafe fn dc_l1tlb_hit_2m(&self) -> u64 { __ibs_get_bits(self.val, 4, 1) }
    #[inline]
    pub unsafe fn set_dc_l1tlb_hit_2m(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 4, 1, value); }
    /* 5: data cache L1TLB hit in 1G page */
    #[inline]
    pub unsafe fn dc_l1tlb_hit_1g(&self) -> u64 { __ibs_get_bits(self.val, 5, 1) }
    #[inline]
    pub unsafe fn set_dc_l1tlb_hit_1g(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 5, 1, value); }
    /* 6: data cache L2TLB hit in 2M page */
    #[inline]
    pub unsafe fn dc_l2tlb_hit_2m(&self) -> u64 { __ibs_get_bits(self.val, 6, 1) }
    #[inline]
    pub unsafe fn set_dc_l2tlb_hit_2m(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 6, 1, value); }
    /* 7: data cache miss */
    #[inline]
    pub unsafe fn dc_miss(&self) -> u64 { __ibs_get_bits(self.val, 7, 1) }
    #[inline]
    pub unsafe fn set_dc_miss(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 7, 1, value); }
    /* 8: misaligned access */
    #[inline]
    pub unsafe fn dc_mis_acc(&self) -> u64 { __ibs_get_bits(self.val, 8, 1) }
    #[inline]
    pub unsafe fn set_dc_mis_acc(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 8, 1, value); }
    /* 9-12: reserved */
    #[inline]
    pub unsafe fn reserved(&self) -> u64 { __ibs_get_bits(self.val, 9, 4) }
    #[inline]
    pub unsafe fn set_reserved(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 9, 4, value); }
    /* 13: write combining memory access */
    #[inline]
    pub unsafe fn dc_wc_mem_acc(&self) -> u64 { __ibs_get_bits(self.val, 13, 1) }
    #[inline]
    pub unsafe fn set_dc_wc_mem_acc(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 13, 1, value); }
    /* 14: uncacheable memory access */
    #[inline]
    pub unsafe fn dc_uc_mem_acc(&self) -> u64 { __ibs_get_bits(self.val, 14, 1) }
    #[inline]
    pub unsafe fn set_dc_uc_mem_acc(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 14, 1, value); }
    /* 15: locked operation */
    #[inline]
    pub unsafe fn dc_locked_op(&self) -> u64 { __ibs_get_bits(self.val, 15, 1) }
    #[inline]
    pub unsafe fn set_dc_locked_op(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 15, 1, value); }
    /* 16: DC miss with no MAB allocated */
    #[inline]
    pub unsafe fn dc_miss_no_mab_alloc(&self) -> u64 { __ibs_get_bits(self.val, 16, 1) }
    #[inline]
    pub unsafe fn set_dc_miss_no_mab_alloc(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 16, 1, value); }
    /* 17: data cache linear address valid */
    #[inline]
    pub unsafe fn dc_lin_addr_valid(&self) -> u64 { __ibs_get_bits(self.val, 17, 1) }
    #[inline]
    pub unsafe fn set_dc_lin_addr_valid(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 17, 1, value); }
    /* 18: data cache physical address valid */
    #[inline]
    pub unsafe fn dc_phy_addr_valid(&self) -> u64 { __ibs_get_bits(self.val, 18, 1) }
    #[inline]
    pub unsafe fn set_dc_phy_addr_valid(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 18, 1, value); }
    /* 19: data cache L2 hit in 1GB page */
    #[inline]
    pub unsafe fn dc_l2_tlb_hit_1g(&self) -> u64 { __ibs_get_bits(self.val, 19, 1) }
    #[inline]
    pub unsafe fn set_dc_l2_tlb_hit_1g(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 19, 1, value); }
    /* 20: L2 cache miss */
    #[inline]
    pub unsafe fn l2_miss(&self) -> u64 { __ibs_get_bits(self.val, 20, 1) }
    #[inline]
    pub unsafe fn set_l2_miss(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 20, 1, value); }
    /* 21: software prefetch */
    #[inline]
    pub unsafe fn sw_pf(&self) -> u64 { __ibs_get_bits(self.val, 21, 1) }
    #[inline]
    pub unsafe fn set_sw_pf(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 21, 1, value); }
    /* 22-25: load/store size in bytes */
    #[inline]
    pub unsafe fn op_mem_width(&self) -> u64 { __ibs_get_bits(self.val, 22, 4) }
    #[inline]
    pub unsafe fn set_op_mem_width(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 22, 4, value); }
    /* 26-31: outstanding mem reqs on DC fill */
    #[inline]
    pub unsafe fn op_dc_miss_open_mem_reqs(&self) -> u64 { __ibs_get_bits(self.val, 26, 6) }
    #[inline]
    pub unsafe fn set_op_dc_miss_open_mem_reqs(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 26, 6, value); }
    /* 32-47: data cache miss latency */
    #[inline]
    pub unsafe fn dc_miss_lat(&self) -> u64 { __ibs_get_bits(self.val, 32, 16) }
    #[inline]
    pub unsafe fn set_dc_miss_lat(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 32, 16, value); }
    /* 48-63: L1 TLB refill latency */
    #[inline]
    pub unsafe fn tlb_refill_lat(&self) -> u64 { __ibs_get_bits(self.val, 48, 16) }
    #[inline]
    pub unsafe fn set_tlb_refill_lat(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 48, 16, value); }
}

/* MSR 0xc001103c: IBS Fetch Control Extended */
#[repr(C)]
#[derive(Copy, Clone)]
pub union ic_ibs_extd_ctl {
    pub val: u64,
}

impl ic_ibs_extd_ctl {
    /* 0-15: ITLB Refill latency for sampled fetch */
    #[inline]
    pub unsafe fn itlb_refill_lat(&self) -> u64 { __ibs_get_bits(self.val, 0, 16) }
    #[inline]
    pub unsafe fn set_itlb_refill_lat(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 0, 16, value); }
    /* 16-63: reserved */
    #[inline]
    pub unsafe fn reserved(&self) -> u64 { __ibs_get_bits(self.val, 16, 48) }
    #[inline]
    pub unsafe fn set_reserved(&mut self, value: u64) { __ibs_set_bits(&mut self.val, 16, 48, value); }
}

/*
 * IBS driver related
 */

#[repr(C)]
#[derive(Copy, Clone)]
pub union perf_ibs_data_anon {
    pub data: [u32; 0], /* data buffer starts here */
    pub caps: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct perf_ibs_data {
    pub size: u32,
    pub __bindgen_anon_1: perf_ibs_data_anon,
    pub regs: [u64; MSR_AMD64_IBS_REG_COUNT_MAX],
}
