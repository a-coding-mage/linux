/*
 * Copyright (C) 2017  Advanced Micro Devices, Inc.
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



// addressBlock: uvd_uvd_pg_dec
// base address: 0x1fb00
pub const mmUVD_PGFSM_CONFIG: u32 = 0x00c0;
pub const mmUVD_PGFSM_CONFIG_BASE_IDX: u32 = 1;
pub const mmUVD_PGFSM_STATUS: u32 = 0x00c1;
pub const mmUVD_PGFSM_STATUS_BASE_IDX: u32 = 1;
pub const mmUVD_POWER_STATUS: u32 = 0x00c4;
pub const mmUVD_POWER_STATUS_BASE_IDX: u32 = 1;
pub const mmCC_UVD_HARVESTING: u32 = 0x00c7;
pub const mmCC_UVD_HARVESTING_BASE_IDX: u32 = 1;
pub const mmUVD_DPG_LMA_CTL: u32 = 0x00d1;
pub const mmUVD_DPG_LMA_CTL_BASE_IDX: u32 = 1;
pub const mmUVD_DPG_LMA_DATA: u32 = 0x00d2;
pub const mmUVD_DPG_LMA_DATA_BASE_IDX: u32 = 1;
pub const mmUVD_DPG_LMA_MASK: u32 = 0x00d3;
pub const mmUVD_DPG_LMA_MASK_BASE_IDX: u32 = 1;
pub const mmUVD_DPG_PAUSE: u32 = 0x00d4;
pub const mmUVD_DPG_PAUSE_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH1: u32 = 0x00d5;
pub const mmUVD_SCRATCH1_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH2: u32 = 0x00d6;
pub const mmUVD_SCRATCH2_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH3: u32 = 0x00d7;
pub const mmUVD_SCRATCH3_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH4: u32 = 0x00d8;
pub const mmUVD_SCRATCH4_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH5: u32 = 0x00d9;
pub const mmUVD_SCRATCH5_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH6: u32 = 0x00da;
pub const mmUVD_SCRATCH6_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH7: u32 = 0x00db;
pub const mmUVD_SCRATCH7_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH8: u32 = 0x00dc;
pub const mmUVD_SCRATCH8_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH9: u32 = 0x00dd;
pub const mmUVD_SCRATCH9_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH10: u32 = 0x00de;
pub const mmUVD_SCRATCH10_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH11: u32 = 0x00df;
pub const mmUVD_SCRATCH11_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH12: u32 = 0x00e0;
pub const mmUVD_SCRATCH12_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH13: u32 = 0x00e1;
pub const mmUVD_SCRATCH13_BASE_IDX: u32 = 1;
pub const mmUVD_SCRATCH14: u32 = 0x00e2;
pub const mmUVD_SCRATCH14_BASE_IDX: u32 = 1;
pub const mmUVD_DPG_LMI_VCPU_CACHE_64BIT_BAR_LOW: u32 = 0x00e5;
pub const mmUVD_DPG_LMI_VCPU_CACHE_64BIT_BAR_LOW_BASE_IDX: u32 = 1;
pub const mmUVD_DPG_LMI_VCPU_CACHE_64BIT_BAR_HIGH: u32 = 0x00e6;
pub const mmUVD_DPG_LMI_VCPU_CACHE_64BIT_BAR_HIGH_BASE_IDX: u32 = 1;
pub const mmUVD_DPG_VCPU_CACHE_OFFSET0: u32 = 0x00e7;
pub const mmUVD_DPG_VCPU_CACHE_OFFSET0_BASE_IDX: u32 = 1;


// addressBlock: uvd_uvdgendec
// base address: 0x1fc00
pub const mmUVD_LCM_CGC_CNTRL: u32 = 0x0123;
pub const mmUVD_LCM_CGC_CNTRL_BASE_IDX: u32 = 1;

pub const mmUVD_MIF_CURR_UV_ADDR_CONFIG: u32 = 0x0184;
pub const mmUVD_MIF_CURR_UV_ADDR_CONFIG_BASE_IDX: u32 = 1;
pub const mmUVD_MIF_REF_UV_ADDR_CONFIG: u32 = 0x0185;
pub const mmUVD_MIF_REF_UV_ADDR_CONFIG_BASE_IDX: u32 = 1;
pub const mmUVD_MIF_RECON1_UV_ADDR_CONFIG: u32 = 0x0186;
pub const mmUVD_MIF_RECON1_UV_ADDR_CONFIG_BASE_IDX: u32 = 1;
pub const mmUVD_MIF_CURR_ADDR_CONFIG: u32 = 0x0192;
pub const mmUVD_MIF_CURR_ADDR_CONFIG_BASE_IDX: u32 = 1;
pub const mmUVD_MIF_REF_ADDR_CONFIG: u32 = 0x0193;
pub const mmUVD_MIF_REF_ADDR_CONFIG_BASE_IDX: u32 = 1;
pub const mmUVD_MIF_RECON1_ADDR_CONFIG: u32 = 0x01c5;
pub const mmUVD_MIF_RECON1_ADDR_CONFIG_BASE_IDX: u32 = 1;

// addressBlock: uvd_uvdnpdec
// base address: 0x20000
pub const mmUVD_JPEG_CNTL: u32 = 0x0200;
pub const mmUVD_JPEG_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_RB_BASE: u32 = 0x0201;
pub const mmUVD_JPEG_RB_BASE_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_RB_WPTR: u32 = 0x0202;
pub const mmUVD_JPEG_RB_WPTR_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_RB_RPTR: u32 = 0x0203;
pub const mmUVD_JPEG_RB_RPTR_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_RB_SIZE: u32 = 0x0204;
pub const mmUVD_JPEG_RB_SIZE_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_ADDR_CONFIG: u32 = 0x021f;
pub const mmUVD_JPEG_ADDR_CONFIG_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_PITCH: u32 = 0x0222;
pub const mmUVD_JPEG_PITCH_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_GPCOM_CMD: u32 = 0x022c;
pub const mmUVD_JPEG_GPCOM_CMD_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_GPCOM_DATA0: u32 = 0x022d;
pub const mmUVD_JPEG_GPCOM_DATA0_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_GPCOM_DATA1: u32 = 0x022e;
pub const mmUVD_JPEG_GPCOM_DATA1_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_JRB_BASE_LO: u32 = 0x022f;
pub const mmUVD_JPEG_JRB_BASE_LO_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_JRB_BASE_HI: u32 = 0x0230;
pub const mmUVD_JPEG_JRB_BASE_HI_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_JRB_SIZE: u32 = 0x0232;
pub const mmUVD_JPEG_JRB_SIZE_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_JRB_RPTR: u32 = 0x0233;
pub const mmUVD_JPEG_JRB_RPTR_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_JRB_WPTR: u32 = 0x0234;
pub const mmUVD_JPEG_JRB_WPTR_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_UV_ADDR_CONFIG: u32 = 0x0238;
pub const mmUVD_JPEG_UV_ADDR_CONFIG_BASE_IDX: u32 = 1;
pub const mmUVD_SEMA_ADDR_LOW: u32 = 0x03c0;
pub const mmUVD_SEMA_ADDR_LOW_BASE_IDX: u32 = 1;
pub const mmUVD_SEMA_ADDR_HIGH: u32 = 0x03c1;
pub const mmUVD_SEMA_ADDR_HIGH_BASE_IDX: u32 = 1;
pub const mmUVD_SEMA_CMD: u32 = 0x03c2;
pub const mmUVD_SEMA_CMD_BASE_IDX: u32 = 1;
pub const mmUVD_GPCOM_VCPU_CMD: u32 = 0x03c3;
pub const mmUVD_GPCOM_VCPU_CMD_BASE_IDX: u32 = 1;
pub const mmUVD_GPCOM_VCPU_DATA0: u32 = 0x03c4;
pub const mmUVD_GPCOM_VCPU_DATA0_BASE_IDX: u32 = 1;
pub const mmUVD_GPCOM_VCPU_DATA1: u32 = 0x03c5;
pub const mmUVD_GPCOM_VCPU_DATA1_BASE_IDX: u32 = 1;
pub const mmUVD_ENGINE_CNTL: u32 = 0x03c6;
pub const mmUVD_ENGINE_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_UDEC_DBW_UV_ADDR_CONFIG: u32 = 0x03d2;
pub const mmUVD_UDEC_DBW_UV_ADDR_CONFIG_BASE_IDX: u32 = 1;
pub const mmUVD_UDEC_ADDR_CONFIG: u32 = 0x03d3;
pub const mmUVD_UDEC_ADDR_CONFIG_BASE_IDX: u32 = 1;
pub const mmUVD_UDEC_DB_ADDR_CONFIG: u32 = 0x03d4;
pub const mmUVD_UDEC_DB_ADDR_CONFIG_BASE_IDX: u32 = 1;
pub const mmUVD_UDEC_DBW_ADDR_CONFIG: u32 = 0x03d5;
pub const mmUVD_UDEC_DBW_ADDR_CONFIG_BASE_IDX: u32 = 1;
pub const mmUVD_SUVD_CGC_GATE: u32 = 0x03e4;
pub const mmUVD_SUVD_CGC_GATE_BASE_IDX: u32 = 1;
pub const mmUVD_SUVD_CGC_STATUS: u32 = 0x03e5;
pub const mmUVD_SUVD_CGC_STATUS_BASE_IDX: u32 = 1;
pub const mmUVD_SUVD_CGC_CTRL: u32 = 0x03e6;
pub const mmUVD_SUVD_CGC_CTRL_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_VCPU_CACHE1_64BIT_BAR_LOW: u32 = 0x03ec;
pub const mmUVD_LMI_VCPU_CACHE1_64BIT_BAR_LOW_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_VCPU_CACHE1_64BIT_BAR_HIGH: u32 = 0x03ed;
pub const mmUVD_LMI_VCPU_CACHE1_64BIT_BAR_HIGH_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_VCPU_CACHE2_64BIT_BAR_LOW: u32 = 0x03f0;
pub const mmUVD_LMI_VCPU_CACHE2_64BIT_BAR_LOW_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_VCPU_CACHE2_64BIT_BAR_HIGH: u32 = 0x03f1;
pub const mmUVD_LMI_VCPU_CACHE2_64BIT_BAR_HIGH_BASE_IDX: u32 = 1;
pub const mmUVD_NO_OP: u32 = 0x03ff;
pub const mmUVD_NO_OP_BASE_IDX: u32 = 1;
pub const mmUVD_JPEG_CNTL2: u32 = 0x0404;
pub const mmUVD_JPEG_CNTL2_BASE_IDX: u32 = 1;
pub const mmUVD_VERSION: u32 = 0x0409;
pub const mmUVD_VERSION_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH8: u32 = 0x040a;
pub const mmUVD_GP_SCRATCH8_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH9: u32 = 0x040b;
pub const mmUVD_GP_SCRATCH9_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH10: u32 = 0x040c;
pub const mmUVD_GP_SCRATCH10_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH11: u32 = 0x040d;
pub const mmUVD_GP_SCRATCH11_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH12: u32 = 0x040e;
pub const mmUVD_GP_SCRATCH12_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH13: u32 = 0x040f;
pub const mmUVD_GP_SCRATCH13_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH14: u32 = 0x0410;
pub const mmUVD_GP_SCRATCH14_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH15: u32 = 0x0411;
pub const mmUVD_GP_SCRATCH15_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH16: u32 = 0x0412;
pub const mmUVD_GP_SCRATCH16_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH17: u32 = 0x0413;
pub const mmUVD_GP_SCRATCH17_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH18: u32 = 0x0414;
pub const mmUVD_GP_SCRATCH18_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH19: u32 = 0x0415;
pub const mmUVD_GP_SCRATCH19_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH20: u32 = 0x0416;
pub const mmUVD_GP_SCRATCH20_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH21: u32 = 0x0417;
pub const mmUVD_GP_SCRATCH21_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH22: u32 = 0x0418;
pub const mmUVD_GP_SCRATCH22_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH23: u32 = 0x0419;
pub const mmUVD_GP_SCRATCH23_BASE_IDX: u32 = 1;
pub const mmUVD_RB_BASE_LO2: u32 = 0x0421;
pub const mmUVD_RB_BASE_LO2_BASE_IDX: u32 = 1;
pub const mmUVD_RB_BASE_HI2: u32 = 0x0422;
pub const mmUVD_RB_BASE_HI2_BASE_IDX: u32 = 1;
pub const mmUVD_RB_SIZE2: u32 = 0x0423;
pub const mmUVD_RB_SIZE2_BASE_IDX: u32 = 1;
pub const mmUVD_RB_RPTR2: u32 = 0x0424;
pub const mmUVD_RB_RPTR2_BASE_IDX: u32 = 1;
pub const mmUVD_RB_WPTR2: u32 = 0x0425;
pub const mmUVD_RB_WPTR2_BASE_IDX: u32 = 1;
pub const mmUVD_RB_BASE_LO: u32 = 0x0426;
pub const mmUVD_RB_BASE_LO_BASE_IDX: u32 = 1;
pub const mmUVD_RB_BASE_HI: u32 = 0x0427;
pub const mmUVD_RB_BASE_HI_BASE_IDX: u32 = 1;
pub const mmUVD_RB_SIZE: u32 = 0x0428;
pub const mmUVD_RB_SIZE_BASE_IDX: u32 = 1;
pub const mmUVD_RB_RPTR: u32 = 0x0429;
pub const mmUVD_RB_RPTR_BASE_IDX: u32 = 1;
pub const mmUVD_RB_WPTR: u32 = 0x042a;
pub const mmUVD_RB_WPTR_BASE_IDX: u32 = 1;
pub const mmUVD_RB_WPTR4: u32 = 0x0456;
pub const mmUVD_RB_WPTR4_BASE_IDX: u32 = 1;
pub const mmUVD_JRBC_RB_RPTR: u32 = 0x0457;
pub const mmUVD_JRBC_RB_RPTR_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_JPEG_VMID: u32 = 0x045d;
pub const mmUVD_LMI_JPEG_VMID_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_VCPU_CACHE_64BIT_BAR_HIGH: u32 = 0x045e;
pub const mmUVD_LMI_VCPU_CACHE_64BIT_BAR_HIGH_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_VCPU_CACHE_64BIT_BAR_LOW: u32 = 0x045f;
pub const mmUVD_LMI_VCPU_CACHE_64BIT_BAR_LOW_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_RBC_IB_64BIT_BAR_HIGH: u32 = 0x0466;
pub const mmUVD_LMI_RBC_IB_64BIT_BAR_HIGH_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_RBC_IB_64BIT_BAR_LOW: u32 = 0x0467;
pub const mmUVD_LMI_RBC_IB_64BIT_BAR_LOW_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_RBC_RB_64BIT_BAR_HIGH: u32 = 0x0468;
pub const mmUVD_LMI_RBC_RB_64BIT_BAR_HIGH_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_RBC_RB_64BIT_BAR_LOW: u32 = 0x0469;
pub const mmUVD_LMI_RBC_RB_64BIT_BAR_LOW_BASE_IDX: u32 = 1;


// addressBlock: uvd_uvddec
// base address: 0x20c00
pub const mmUVD_SEMA_CNTL: u32 = 0x0500;
pub const mmUVD_SEMA_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_JRBC_RB_64BIT_BAR_LOW: u32 = 0x0503;
pub const mmUVD_LMI_JRBC_RB_64BIT_BAR_LOW_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_JRBC_RB_64BIT_BAR_HIGH: u32 = 0x0504;
pub const mmUVD_LMI_JRBC_RB_64BIT_BAR_HIGH_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_JRBC_IB_64BIT_BAR_LOW: u32 = 0x0505;
pub const mmUVD_LMI_JRBC_IB_64BIT_BAR_LOW_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_JRBC_IB_64BIT_BAR_HIGH: u32 = 0x0506;
pub const mmUVD_LMI_JRBC_IB_64BIT_BAR_HIGH_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_JRBC_IB_VMID: u32 = 0x0507;
pub const mmUVD_LMI_JRBC_IB_VMID_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_JRBC_RB_VMID: u32 = 0x0508;
pub const mmUVD_LMI_JRBC_RB_VMID_BASE_IDX: u32 = 1;
pub const mmUVD_JRBC_RB_WPTR: u32 = 0x0509;
pub const mmUVD_JRBC_RB_WPTR_BASE_IDX: u32 = 1;
pub const mmUVD_JRBC_RB_CNTL: u32 = 0x050a;
pub const mmUVD_JRBC_RB_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_JRBC_IB_SIZE: u32 = 0x050b;
pub const mmUVD_JRBC_IB_SIZE_BASE_IDX: u32 = 1;
pub const mmUVD_JRBC_LMI_SWAP_CNTL: u32 = 0x050d;
pub const mmUVD_JRBC_LMI_SWAP_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_JRBC_RB_MEM_WR_64BIT_BAR_LOW: u32 = 0x050e;
pub const mmUVD_LMI_JRBC_RB_MEM_WR_64BIT_BAR_LOW_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_JRBC_RB_MEM_WR_64BIT_BAR_HIGH: u32 = 0x050f;
pub const mmUVD_LMI_JRBC_RB_MEM_WR_64BIT_BAR_HIGH_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_JRBC_RB_MEM_RD_64BIT_BAR_LOW: u32 = 0x0510;
pub const mmUVD_LMI_JRBC_RB_MEM_RD_64BIT_BAR_LOW_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_JRBC_RB_MEM_RD_64BIT_BAR_HIGH: u32 = 0x0511;
pub const mmUVD_LMI_JRBC_RB_MEM_RD_64BIT_BAR_HIGH_BASE_IDX: u32 = 1;
pub const mmUVD_JRBC_RB_REF_DATA: u32 = 0x0512;
pub const mmUVD_JRBC_RB_REF_DATA_BASE_IDX: u32 = 1;
pub const mmUVD_JRBC_RB_COND_RD_TIMER: u32 = 0x0513;
pub const mmUVD_JRBC_RB_COND_RD_TIMER_BASE_IDX: u32 = 1;
pub const mmUVD_JRBC_EXTERNAL_REG_BASE: u32 = 0x0517;
pub const mmUVD_JRBC_EXTERNAL_REG_BASE_BASE_IDX: u32 = 1;
pub const mmUVD_JRBC_SOFT_RESET: u32 = 0x0519;
pub const mmUVD_JRBC_SOFT_RESET_BASE_IDX: u32 = 1;
pub const mmUVD_JRBC_STATUS: u32 = 0x051a;
pub const mmUVD_JRBC_STATUS_BASE_IDX: u32 = 1;
pub const mmUVD_RB_RPTR3: u32 = 0x051b;
pub const mmUVD_RB_RPTR3_BASE_IDX: u32 = 1;
pub const mmUVD_RB_WPTR3: u32 = 0x051c;
pub const mmUVD_RB_WPTR3_BASE_IDX: u32 = 1;
pub const mmUVD_RB_BASE_LO3: u32 = 0x051d;
pub const mmUVD_RB_BASE_LO3_BASE_IDX: u32 = 1;
pub const mmUVD_RB_BASE_HI3: u32 = 0x051e;
pub const mmUVD_RB_BASE_HI3_BASE_IDX: u32 = 1;
pub const mmUVD_RB_SIZE3: u32 = 0x051f;
pub const mmUVD_RB_SIZE3_BASE_IDX: u32 = 1;
pub const mmJPEG_CGC_GATE: u32 = 0x0526;
pub const mmJPEG_CGC_GATE_BASE_IDX: u32 = 1;
pub const mmUVD_CTX_INDEX: u32 = 0x0528;
pub const mmUVD_CTX_INDEX_BASE_IDX: u32 = 1;
pub const mmUVD_CTX_DATA: u32 = 0x0529;
pub const mmUVD_CTX_DATA_BASE_IDX: u32 = 1;
pub const mmUVD_CGC_GATE: u32 = 0x052a;
pub const mmUVD_CGC_GATE_BASE_IDX: u32 = 1;
pub const mmUVD_CGC_STATUS: u32 = 0x052b;
pub const mmUVD_CGC_STATUS_BASE_IDX: u32 = 1;
pub const mmUVD_CGC_CTRL: u32 = 0x052c;
pub const mmUVD_CGC_CTRL_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH0: u32 = 0x0534;
pub const mmUVD_GP_SCRATCH0_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH1: u32 = 0x0535;
pub const mmUVD_GP_SCRATCH1_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH2: u32 = 0x0536;
pub const mmUVD_GP_SCRATCH2_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH3: u32 = 0x0537;
pub const mmUVD_GP_SCRATCH3_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH4: u32 = 0x0538;
pub const mmUVD_GP_SCRATCH4_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH5: u32 = 0x0539;
pub const mmUVD_GP_SCRATCH5_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH6: u32 = 0x053a;
pub const mmUVD_GP_SCRATCH6_BASE_IDX: u32 = 1;
pub const mmUVD_GP_SCRATCH7: u32 = 0x053b;
pub const mmUVD_GP_SCRATCH7_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_VCPU_CACHE_VMID: u32 = 0x053c;
pub const mmUVD_LMI_VCPU_CACHE_VMID_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_CTRL2: u32 = 0x053d;
pub const mmUVD_LMI_CTRL2_BASE_IDX: u32 = 1;
pub const mmUVD_MASTINT_EN: u32 = 0x0540;
pub const mmUVD_MASTINT_EN_BASE_IDX: u32 = 1;
pub const mmUVD_SYS_INT_EN: u32 = 0x0541;
pub const mmUVD_SYS_INT_EN_BASE_IDX: u32 = 1;
pub const mmJPEG_CGC_CTRL: u32 = 0x0565;
pub const mmJPEG_CGC_CTRL_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_CTRL: u32 = 0x0566;
pub const mmUVD_LMI_CTRL_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_STATUS: u32 = 0x0567;
pub const mmUVD_LMI_STATUS_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_VM_CTRL: u32 = 0x0568;
pub const mmUVD_LMI_VM_CTRL_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_SWAP_CNTL: u32 = 0x056d;
pub const mmUVD_LMI_SWAP_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_MPC_CNTL: u32 = 0x0577;
pub const mmUVD_MPC_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_MPC_SET_MUXA0: u32 = 0x0579;
pub const mmUVD_MPC_SET_MUXA0_BASE_IDX: u32 = 1;
pub const mmUVD_MPC_SET_MUXA1: u32 = 0x057a;
pub const mmUVD_MPC_SET_MUXA1_BASE_IDX: u32 = 1;
pub const mmUVD_MPC_SET_MUXB0: u32 = 0x057b;
pub const mmUVD_MPC_SET_MUXB0_BASE_IDX: u32 = 1;
pub const mmUVD_MPC_SET_MUXB1: u32 = 0x057c;
pub const mmUVD_MPC_SET_MUXB1_BASE_IDX: u32 = 1;
pub const mmUVD_MPC_SET_MUX: u32 = 0x057d;
pub const mmUVD_MPC_SET_MUX_BASE_IDX: u32 = 1;
pub const mmUVD_MPC_SET_ALU: u32 = 0x057e;
pub const mmUVD_MPC_SET_ALU_BASE_IDX: u32 = 1;
pub const mmUVD_GPCOM_SYS_CMD: u32 = 0x057f;
pub const mmUVD_GPCOM_SYS_CMD_BASE_IDX: u32 = 1;
pub const mmUVD_GPCOM_SYS_DATA0: u32 = 0x0580;
pub const mmUVD_GPCOM_SYS_DATA0_BASE_IDX: u32 = 1;
pub const mmUVD_GPCOM_SYS_DATA1: u32 = 0x0581;
pub const mmUVD_GPCOM_SYS_DATA1_BASE_IDX: u32 = 1;
pub const mmUVD_VCPU_CACHE_OFFSET0: u32 = 0x0582;
pub const mmUVD_VCPU_CACHE_OFFSET0_BASE_IDX: u32 = 1;
pub const mmUVD_VCPU_CACHE_SIZE0: u32 = 0x0583;
pub const mmUVD_VCPU_CACHE_SIZE0_BASE_IDX: u32 = 1;
pub const mmUVD_VCPU_CACHE_OFFSET1: u32 = 0x0584;
pub const mmUVD_VCPU_CACHE_OFFSET1_BASE_IDX: u32 = 1;
pub const mmUVD_VCPU_CACHE_SIZE1: u32 = 0x0585;
pub const mmUVD_VCPU_CACHE_SIZE1_BASE_IDX: u32 = 1;
pub const mmUVD_VCPU_CACHE_OFFSET2: u32 = 0x0586;
pub const mmUVD_VCPU_CACHE_OFFSET2_BASE_IDX: u32 = 1;
pub const mmUVD_VCPU_CACHE_SIZE2: u32 = 0x0587;
pub const mmUVD_VCPU_CACHE_SIZE2_BASE_IDX: u32 = 1;
pub const mmUVD_VCPU_CNTL: u32 = 0x0598;
pub const mmUVD_VCPU_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_SOFT_RESET: u32 = 0x05a0;
pub const mmUVD_SOFT_RESET_BASE_IDX: u32 = 1;
pub const mmUVD_LMI_RBC_IB_VMID: u32 = 0x05a1;
pub const mmUVD_LMI_RBC_IB_VMID_BASE_IDX: u32 = 1;
pub const mmUVD_RBC_IB_SIZE: u32 = 0x05a2;
pub const mmUVD_RBC_IB_SIZE_BASE_IDX: u32 = 1;
pub const mmUVD_RBC_RB_RPTR: u32 = 0x05a4;
pub const mmUVD_RBC_RB_RPTR_BASE_IDX: u32 = 1;
pub const mmUVD_RBC_RB_WPTR: u32 = 0x05a5;
pub const mmUVD_RBC_RB_WPTR_BASE_IDX: u32 = 1;
pub const mmUVD_RBC_RB_WPTR_CNTL: u32 = 0x05a6;
pub const mmUVD_RBC_RB_WPTR_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_RBC_RB_CNTL: u32 = 0x05a9;
pub const mmUVD_RBC_RB_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_RBC_RB_RPTR_ADDR: u32 = 0x05aa;
pub const mmUVD_RBC_RB_RPTR_ADDR_BASE_IDX: u32 = 1;
pub const mmUVD_STATUS: u32 = 0x05af;
pub const mmUVD_STATUS_BASE_IDX: u32 = 1;
pub const mmUVD_SEMA_TIMEOUT_STATUS: u32 = 0x05b0;
pub const mmUVD_SEMA_TIMEOUT_STATUS_BASE_IDX: u32 = 1;
pub const mmUVD_SEMA_WAIT_INCOMPLETE_TIMEOUT_CNTL: u32 = 0x05b1;
pub const mmUVD_SEMA_WAIT_INCOMPLETE_TIMEOUT_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_SEMA_WAIT_FAULT_TIMEOUT_CNTL: u32 = 0x05b2;
pub const mmUVD_SEMA_WAIT_FAULT_TIMEOUT_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_SEMA_SIGNAL_INCOMPLETE_TIMEOUT_CNTL: u32 = 0x05b3;
pub const mmUVD_SEMA_SIGNAL_INCOMPLETE_TIMEOUT_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_CONTEXT_ID: u32 = 0x05bd;
pub const mmUVD_CONTEXT_ID_BASE_IDX: u32 = 1;
pub const mmUVD_CONTEXT_ID2: u32 = 0x05bf;
pub const mmUVD_CONTEXT_ID2_BASE_IDX: u32 = 1;
pub const mmUVD_RBC_WPTR_POLL_CNTL: u32 = 0x05d8;
pub const mmUVD_RBC_WPTR_POLL_CNTL_BASE_IDX: u32 = 1;
pub const mmUVD_RBC_WPTR_POLL_ADDR: u32 = 0x05d9;
pub const mmUVD_RBC_WPTR_POLL_ADDR_BASE_IDX: u32 = 1;
pub const mmUVD_RB_BASE_LO4: u32 = 0x05df;
pub const mmUVD_RB_BASE_LO4_BASE_IDX: u32 = 1;
pub const mmUVD_RB_BASE_HI4: u32 = 0x05e0;
pub const mmUVD_RB_BASE_HI4_BASE_IDX: u32 = 1;
pub const mmUVD_RB_SIZE4: u32 = 0x05e1;
pub const mmUVD_RB_SIZE4_BASE_IDX: u32 = 1;
pub const mmUVD_RB_RPTR4: u32 = 0x05e2;
pub const mmUVD_RB_RPTR4_BASE_IDX: u32 = 1;




// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
