/*
 * Copyright (C) 2020  Advanced Micro Devices, Inc.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included
 * in all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS
 * OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE COPYRIGHT HOLDER(S) BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN
 * AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
 * CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
 */

// addressBlock: gc_grbmdec
// base address: 0x8000
pub const mmGRBM_CNTL: u32 = 0x0000;
pub const mmGRBM_CNTL_BASE_IDX: u32 = 0;
pub const mmGRBM_SKEW_CNTL: u32 = 0x0001;
pub const mmGRBM_SKEW_CNTL_BASE_IDX: u32 = 0;
pub const mmGRBM_STATUS2: u32 = 0x0002;
pub const mmGRBM_STATUS2_BASE_IDX: u32 = 0;
pub const mmGRBM_PWR_CNTL: u32 = 0x0003;
pub const mmGRBM_PWR_CNTL_BASE_IDX: u32 = 0;
pub const mmGRBM_STATUS: u32 = 0x0004;
pub const mmGRBM_STATUS_BASE_IDX: u32 = 0;
pub const mmGRBM_STATUS_SE0: u32 = 0x0005;
pub const mmGRBM_STATUS_SE0_BASE_IDX: u32 = 0;
pub const mmGRBM_STATUS_SE1: u32 = 0x0006;
pub const mmGRBM_STATUS_SE1_BASE_IDX: u32 = 0;
pub const mmGRBM_SOFT_RESET: u32 = 0x0008;
pub const mmGRBM_SOFT_RESET_BASE_IDX: u32 = 0;
pub const mmGRBM_GFX_CLKEN_CNTL: u32 = 0x000c;
pub const mmGRBM_GFX_CLKEN_CNTL_BASE_IDX: u32 = 0;
pub const mmGRBM_WAIT_IDLE_CLOCKS: u32 = 0x000d;
pub const mmGRBM_WAIT_IDLE_CLOCKS_BASE_IDX: u32 = 0;
pub const mmGRBM_STATUS_SE2: u32 = 0x000e;
pub const mmGRBM_STATUS_SE2_BASE_IDX: u32 = 0;
pub const mmGRBM_STATUS_SE3: u32 = 0x000f;
pub const mmGRBM_STATUS_SE3_BASE_IDX: u32 = 0;
pub const mmGRBM_READ_ERROR: u32 = 0x0016;
pub const mmGRBM_READ_ERROR_BASE_IDX: u32 = 0;
pub const mmGRBM_READ_ERROR2: u32 = 0x0017;
pub const mmGRBM_READ_ERROR2_BASE_IDX: u32 = 0;
pub const mmGRBM_INT_CNTL: u32 = 0x0018;
pub const mmGRBM_INT_CNTL_BASE_IDX: u32 = 0;
pub const mmGRBM_TRAP_OP: u32 = 0x0019;
pub const mmGRBM_TRAP_OP_BASE_IDX: u32 = 0;
pub const mmGRBM_TRAP_ADDR: u32 = 0x001a;
pub const mmGRBM_TRAP_ADDR_BASE_IDX: u32 = 0;
pub const mmGRBM_TRAP_ADDR_MSK: u32 = 0x001b;
pub const mmGRBM_TRAP_ADDR_MSK_BASE_IDX: u32 = 0;
pub const mmGRBM_TRAP_WD: u32 = 0x001c;
pub const mmGRBM_TRAP_WD_BASE_IDX: u32 = 0;
pub const mmGRBM_TRAP_WD_MSK: u32 = 0x001d;
pub const mmGRBM_TRAP_WD_MSK_BASE_IDX: u32 = 0;
pub const mmGRBM_DSM_BYPASS: u32 = 0x001e;
pub const mmGRBM_DSM_BYPASS_BASE_IDX: u32 = 0;
pub const mmGRBM_WRITE_ERROR: u32 = 0x001f;
pub const mmGRBM_WRITE_ERROR_BASE_IDX: u32 = 0;
pub const mmGRBM_IOV_ERROR: u32 = 0x0020;
pub const mmGRBM_IOV_ERROR_BASE_IDX: u32 = 0;
pub const mmGRBM_CHIP_REVISION: u32 = 0x0021;
pub const mmGRBM_CHIP_REVISION_BASE_IDX: u32 = 0;
pub const mmGRBM_GFX_CNTL: u32 = 0x0022;
pub const mmGRBM_GFX_CNTL_BASE_IDX: u32 = 0;
pub const mmGRBM_RSMU_CFG: u32 = 0x0023;
pub const mmGRBM_RSMU_CFG_BASE_IDX: u32 = 0;
pub const mmGRBM_IH_CREDIT: u32 = 0x0024;
pub const mmGRBM_IH_CREDIT_BASE_IDX: u32 = 0;
pub const mmGRBM_PWR_CNTL2: u32 = 0x0025;
pub const mmGRBM_PWR_CNTL2_BASE_IDX: u32 = 0;
pub const mmGRBM_UTCL2_INVAL_RANGE_START: u32 = 0x0026;
pub const mmGRBM_UTCL2_INVAL_RANGE_START_BASE_IDX: u32 = 0;
pub const mmGRBM_UTCL2_INVAL_RANGE_END: u32 = 0x0027;
pub const mmGRBM_UTCL2_INVAL_RANGE_END_BASE_IDX: u32 = 0;
pub const mmGRBM_RSMU_READ_ERROR: u32 = 0x0028;
pub const mmGRBM_RSMU_READ_ERROR_BASE_IDX: u32 = 0;
pub const mmGRBM_CHICKEN_BITS: u32 = 0x0029;
pub const mmGRBM_CHICKEN_BITS_BASE_IDX: u32 = 0;
pub const mmGRBM_FENCE_RANGE0: u32 = 0x002a;
pub const mmGRBM_FENCE_RANGE0_BASE_IDX: u32 = 0;
pub const mmGRBM_FENCE_RANGE1: u32 = 0x002b;
pub const mmGRBM_FENCE_RANGE1_BASE_IDX: u32 = 0;
pub const mmGRBM_NOWHERE: u32 = 0x003f;
pub const mmGRBM_NOWHERE_BASE_IDX: u32 = 0;
pub const mmGRBM_SCRATCH_REG0: u32 = 0x0040;
pub const mmGRBM_SCRATCH_REG0_BASE_IDX: u32 = 0;
pub const mmGRBM_SCRATCH_REG1: u32 = 0x0041;
pub const mmGRBM_SCRATCH_REG1_BASE_IDX: u32 = 0;
pub const mmGRBM_SCRATCH_REG2: u32 = 0x0042;
pub const mmGRBM_SCRATCH_REG2_BASE_IDX: u32 = 0;
pub const mmGRBM_SCRATCH_REG3: u32 = 0x0043;
pub const mmGRBM_SCRATCH_REG3_BASE_IDX: u32 = 0;
pub const mmGRBM_SCRATCH_REG4: u32 = 0x0044;
pub const mmGRBM_SCRATCH_REG4_BASE_IDX: u32 = 0;
pub const mmGRBM_SCRATCH_REG5: u32 = 0x0045;
pub const mmGRBM_SCRATCH_REG5_BASE_IDX: u32 = 0;
pub const mmGRBM_SCRATCH_REG6: u32 = 0x0046;
pub const mmGRBM_SCRATCH_REG6_BASE_IDX: u32 = 0;
pub const mmGRBM_SCRATCH_REG7: u32 = 0x0047;
pub const mmGRBM_SCRATCH_REG7_BASE_IDX: u32 = 0;

// addressBlock: gc_cppdec2; base address: 0xc600
pub const mmCPF_EDC_TAG_CNT: u32 = 0x1189; pub const mmCPF_EDC_TAG_CNT_BASE_IDX: u32 = 0;
pub const mmCPF_EDC_ROQ_CNT: u32 = 0x118a; pub const mmCPF_EDC_ROQ_CNT_BASE_IDX: u32 = 0;
pub const mmCPG_EDC_TAG_CNT: u32 = 0x118b; pub const mmCPG_EDC_TAG_CNT_BASE_IDX: u32 = 0;
pub const mmCPG_EDC_DMA_CNT: u32 = 0x118d; pub const mmCPG_EDC_DMA_CNT_BASE_IDX: u32 = 0;
pub const mmCPC_EDC_SCRATCH_CNT: u32 = 0x118e; pub const mmCPC_EDC_SCRATCH_CNT_BASE_IDX: u32 = 0;
pub const mmCPC_EDC_UCODE_CNT: u32 = 0x118f; pub const mmCPC_EDC_UCODE_CNT_BASE_IDX: u32 = 0;
pub const mmDC_EDC_STATE_CNT: u32 = 0x1191; pub const mmDC_EDC_STATE_CNT_BASE_IDX: u32 = 0;
pub const mmDC_EDC_CSINVOC_CNT: u32 = 0x1192; pub const mmDC_EDC_CSINVOC_CNT_BASE_IDX: u32 = 0;
pub const mmDC_EDC_RESTORE_CNT: u32 = 0x1193; pub const mmDC_EDC_RESTORE_CNT_BASE_IDX: u32 = 0;

// addressBlock: gc_gdsdec; base address: 0x9700
pub const mmGDS_EDC_CNT: u32 = 0x05c5; pub const mmGDS_EDC_CNT_BASE_IDX: u32 = 0;
pub const mmGDS_EDC_GRBM_CNT: u32 = 0x05c6; pub const mmGDS_EDC_GRBM_CNT_BASE_IDX: u32 = 0;
pub const mmGDS_EDC_OA_DED: u32 = 0x05c7; pub const mmGDS_EDC_OA_DED_BASE_IDX: u32 = 0;
pub const mmGDS_EDC_OA_PHY_CNT: u32 = 0x05cb; pub const mmGDS_EDC_OA_PHY_CNT_BASE_IDX: u32 = 0;
pub const mmGDS_EDC_OA_PIPE_CNT: u32 = 0x05cc; pub const mmGDS_EDC_OA_PIPE_CNT_BASE_IDX: u32 = 0;

// addressBlock: gc_shsdec; base address: 0x9000
pub const mmSPI_EDC_CNT: u32 = 0x0445; pub const mmSPI_EDC_CNT_BASE_IDX: u32 = 0;

// addressBlock: gc_sqdec; base address: 0x8c00
pub const mmSQC_EDC_CNT2: u32 = 0x032c; pub const mmSQC_EDC_CNT2_BASE_IDX: u32 = 0;
pub const mmSQC_EDC_CNT3: u32 = 0x032d; pub const mmSQC_EDC_CNT3_BASE_IDX: u32 = 0;
pub const mmSQC_EDC_PARITY_CNT3: u32 = 0x032e; pub const mmSQC_EDC_PARITY_CNT3_BASE_IDX: u32 = 0;
pub const mmSQC_EDC_CNT: u32 = 0x03a2; pub const mmSQC_EDC_CNT_BASE_IDX: u32 = 0;
pub const mmSQ_EDC_SEC_CNT: u32 = 0x03a3; pub const mmSQ_EDC_SEC_CNT_BASE_IDX: u32 = 0;
pub const mmSQ_EDC_DED_CNT: u32 = 0x03a4; pub const mmSQ_EDC_DED_CNT_BASE_IDX: u32 = 0;
pub const mmSQ_EDC_INFO: u32 = 0x03a5; pub const mmSQ_EDC_INFO_BASE_IDX: u32 = 0;
pub const mmSQ_EDC_CNT: u32 = 0x03a6; pub const mmSQ_EDC_CNT_BASE_IDX: u32 = 0;

// addressBlock: gc_tpdec; base address: 0x9400
pub const mmTA_EDC_CNT: u32 = 0x0586; pub const mmTA_EDC_CNT_BASE_IDX: u32 = 0;

// addressBlock: gc_tcdec; base address: 0xac00
pub const mmTCP_EDC_CNT: u32 = 0x0b17; pub const mmTCP_EDC_CNT_BASE_IDX: u32 = 0;
pub const mmTCP_EDC_CNT_NEW: u32 = 0x0b18; pub const mmTCP_EDC_CNT_NEW_BASE_IDX: u32 = 0;
pub const mmTCP_ATC_EDC_GATCL1_CNT: u32 = 0x12b1; pub const mmTCP_ATC_EDC_GATCL1_CNT_BASE_IDX: u32 = 0;
pub const mmTCI_EDC_CNT: u32 = 0x0b60; pub const mmTCI_EDC_CNT_BASE_IDX: u32 = 0;
pub const mmTCC_EDC_CNT: u32 = 0x0b82; pub const mmTCC_EDC_CNT_BASE_IDX: u32 = 0;
pub const mmTCC_EDC_CNT2: u32 = 0x0b83; pub const mmTCC_EDC_CNT2_BASE_IDX: u32 = 0;
pub const mmTCA_EDC_CNT: u32 = 0x0bc5; pub const mmTCA_EDC_CNT_BASE_IDX: u32 = 0;

// addressBlock: gc_tpdec; base address: 0x9400
pub const mmTD_EDC_CNT: u32 = 0x052e; pub const mmTD_EDC_CNT_BASE_IDX: u32 = 0;
// mmTA_EDC_CNT and its base index are defined above.

// addressBlock: gc_ea_gceadec2; base address: 0x9c00
pub const mmGCEA_EDC_CNT: u32 = 0x0706; pub const mmGCEA_EDC_CNT_BASE_IDX: u32 = 0;
pub const mmGCEA_EDC_CNT2: u32 = 0x0707; pub const mmGCEA_EDC_CNT2_BASE_IDX: u32 = 0;
pub const mmGCEA_EDC_CNT3: u32 = 0x071b; pub const mmGCEA_EDC_CNT3_BASE_IDX: u32 = 0;
pub const mmGCEA_ERR_STATUS: u32 = 0x0712; pub const mmGCEA_ERR_STATUS_BASE_IDX: u32 = 0;

// addressBlock: gc_gfxudec; base address: 0x30000
pub const mmSCRATCH_REG0: u32 = 0x2040; pub const mmSCRATCH_REG0_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REG1: u32 = 0x2041; pub const mmSCRATCH_REG1_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REG2: u32 = 0x2042; pub const mmSCRATCH_REG2_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REG3: u32 = 0x2043; pub const mmSCRATCH_REG3_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REG4: u32 = 0x2044; pub const mmSCRATCH_REG4_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REG5: u32 = 0x2045; pub const mmSCRATCH_REG5_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REG6: u32 = 0x2046; pub const mmSCRATCH_REG6_BASE_IDX: u32 = 1;
pub const mmSCRATCH_REG7: u32 = 0x2047; pub const mmSCRATCH_REG7_BASE_IDX: u32 = 1;
pub const mmGRBM_GFX_INDEX: u32 = 0x2200; pub const mmGRBM_GFX_INDEX_BASE_IDX: u32 = 1;

// addressBlock: gc_utcl2_atcl2dec; base address: 0xa000
pub const mmATC_L2_CACHE_4K_DSM_INDEX: u32 = 0x080e; pub const mmATC_L2_CACHE_4K_DSM_INDEX_BASE_IDX: u32 = 0;
pub const mmATC_L2_CACHE_2M_DSM_INDEX: u32 = 0x080f; pub const mmATC_L2_CACHE_2M_DSM_INDEX_BASE_IDX: u32 = 0;
pub const mmATC_L2_CACHE_4K_DSM_CNTL: u32 = 0x0810; pub const mmATC_L2_CACHE_4K_DSM_CNTL_BASE_IDX: u32 = 0;
pub const mmATC_L2_CACHE_2M_DSM_CNTL: u32 = 0x0811; pub const mmATC_L2_CACHE_2M_DSM_CNTL_BASE_IDX: u32 = 0;

// addressBlock: gc_utcl2_vml2pfdec; base address: 0xa100
pub const mmVML2_MEM_ECC_INDEX: u32 = 0x0860; pub const mmVML2_MEM_ECC_INDEX_BASE_IDX: u32 = 0;
pub const mmVML2_WALKER_MEM_ECC_INDEX: u32 = 0x0861; pub const mmVML2_WALKER_MEM_ECC_INDEX_BASE_IDX: u32 = 0;
pub const mmUTCL2_MEM_ECC_INDEX: u32 = 0x0862; pub const mmUTCL2_MEM_ECC_INDEX_BASE_IDX: u32 = 0;
pub const mmVML2_MEM_ECC_CNTL: u32 = 0x0863; pub const mmVML2_MEM_ECC_CNTL_BASE_IDX: u32 = 0;
pub const mmVML2_WALKER_MEM_ECC_CNTL: u32 = 0x0864; pub const mmVML2_WALKER_MEM_ECC_CNTL_BASE_IDX: u32 = 0;
pub const mmUTCL2_MEM_ECC_CNTL: u32 = 0x0865; pub const mmUTCL2_MEM_ECC_CNTL_BASE_IDX: u32 = 0;

// addressBlock: gc_rlcpdec; base address: 0x3b000
pub const mmRLC_EDC_CNT: u32 = 0x4d40; pub const mmRLC_EDC_CNT_BASE_IDX: u32 = 1;
pub const mmRLC_EDC_CNT2: u32 = 0x4d41; pub const mmRLC_EDC_CNT2_BASE_IDX: u32 = 1;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
