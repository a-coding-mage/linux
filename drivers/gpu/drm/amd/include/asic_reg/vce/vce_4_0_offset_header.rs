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

// addressBlock: vce0_vce_dec
// base address: 0x22000
pub const mmVCE_STATUS: u32 = 0x0a01;
pub const mmVCE_STATUS_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CNTL: u32 = 0x0a05;
pub const mmVCE_VCPU_CNTL_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_OFFSET0: u32 = 0x0a09;
pub const mmVCE_VCPU_CACHE_OFFSET0_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_SIZE0: u32 = 0x0a0a;
pub const mmVCE_VCPU_CACHE_SIZE0_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_OFFSET1: u32 = 0x0a0b;
pub const mmVCE_VCPU_CACHE_OFFSET1_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_SIZE1: u32 = 0x0a0c;
pub const mmVCE_VCPU_CACHE_SIZE1_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_OFFSET2: u32 = 0x0a0d;
pub const mmVCE_VCPU_CACHE_OFFSET2_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_SIZE2: u32 = 0x0a0e;
pub const mmVCE_VCPU_CACHE_SIZE2_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_OFFSET3: u32 = 0x0a0f;
pub const mmVCE_VCPU_CACHE_OFFSET3_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_SIZE3: u32 = 0x0a10;
pub const mmVCE_VCPU_CACHE_SIZE3_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_OFFSET4: u32 = 0x0a11;
pub const mmVCE_VCPU_CACHE_OFFSET4_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_SIZE4: u32 = 0x0a12;
pub const mmVCE_VCPU_CACHE_SIZE4_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_OFFSET5: u32 = 0x0a13;
pub const mmVCE_VCPU_CACHE_OFFSET5_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_SIZE5: u32 = 0x0a14;
pub const mmVCE_VCPU_CACHE_SIZE5_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_OFFSET6: u32 = 0x0a15;
pub const mmVCE_VCPU_CACHE_OFFSET6_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_SIZE6: u32 = 0x0a16;
pub const mmVCE_VCPU_CACHE_SIZE6_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_OFFSET7: u32 = 0x0a17;
pub const mmVCE_VCPU_CACHE_OFFSET7_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_SIZE7: u32 = 0x0a18;
pub const mmVCE_VCPU_CACHE_SIZE7_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_OFFSET8: u32 = 0x0a19;
pub const mmVCE_VCPU_CACHE_OFFSET8_BASE_IDX: u32 = 0;
pub const mmVCE_VCPU_CACHE_SIZE8: u32 = 0x0a1a;
pub const mmVCE_VCPU_CACHE_SIZE8_BASE_IDX: u32 = 0;
pub const mmVCE_SOFT_RESET: u32 = 0x0a48;
pub const mmVCE_SOFT_RESET_BASE_IDX: u32 = 0;
pub const mmVCE_RB_BASE_LO2: u32 = 0x0a5b;
pub const mmVCE_RB_BASE_LO2_BASE_IDX: u32 = 0;
pub const mmVCE_RB_BASE_HI2: u32 = 0x0a5c;
pub const mmVCE_RB_BASE_HI2_BASE_IDX: u32 = 0;
pub const mmVCE_RB_SIZE2: u32 = 0x0a5d;
pub const mmVCE_RB_SIZE2_BASE_IDX: u32 = 0;
pub const mmVCE_RB_RPTR2: u32 = 0x0a5e;
pub const mmVCE_RB_RPTR2_BASE_IDX: u32 = 0;
pub const mmVCE_RB_WPTR2: u32 = 0x0a5f;
pub const mmVCE_RB_WPTR2_BASE_IDX: u32 = 0;
pub const mmVCE_RB_BASE_LO: u32 = 0x0a60;
pub const mmVCE_RB_BASE_LO_BASE_IDX: u32 = 0;
pub const mmVCE_RB_BASE_HI: u32 = 0x0a61;
pub const mmVCE_RB_BASE_HI_BASE_IDX: u32 = 0;
pub const mmVCE_RB_SIZE: u32 = 0x0a62;
pub const mmVCE_RB_SIZE_BASE_IDX: u32 = 0;
pub const mmVCE_RB_RPTR: u32 = 0x0a63;
pub const mmVCE_RB_RPTR_BASE_IDX: u32 = 0;
pub const mmVCE_RB_WPTR: u32 = 0x0a64;
pub const mmVCE_RB_WPTR_BASE_IDX: u32 = 0;
pub const mmVCE_RB_ARB_CTRL: u32 = 0x0a9f;
pub const mmVCE_RB_ARB_CTRL_BASE_IDX: u32 = 0;
pub const mmVCE_CLOCK_GATING_A: u32 = 0x0abe;
pub const mmVCE_CLOCK_GATING_A_BASE_IDX: u32 = 0;
pub const mmVCE_CLOCK_GATING_B: u32 = 0x0abf;
pub const mmVCE_CLOCK_GATING_B_BASE_IDX: u32 = 0;
pub const mmVCE_RB_BASE_LO3: u32 = 0x0ad4;
pub const mmVCE_RB_BASE_LO3_BASE_IDX: u32 = 0;
pub const mmVCE_RB_BASE_HI3: u32 = 0x0ad5;
pub const mmVCE_RB_BASE_HI3_BASE_IDX: u32 = 0;
pub const mmVCE_RB_SIZE3: u32 = 0x0ad6;
pub const mmVCE_RB_SIZE3_BASE_IDX: u32 = 0;
pub const mmVCE_RB_RPTR3: u32 = 0x0ad7;
pub const mmVCE_RB_RPTR3_BASE_IDX: u32 = 0;
pub const mmVCE_RB_WPTR3: u32 = 0x0ad8;
pub const mmVCE_RB_WPTR3_BASE_IDX: u32 = 0;
pub const mmVCE_SYS_INT_EN: u32 = 0x0b00;
pub const mmVCE_SYS_INT_EN_BASE_IDX: u32 = 0;
pub const mmVCE_SYS_INT_ACK: u32 = 0x0b01;
pub const mmVCE_SYS_INT_ACK_BASE_IDX: u32 = 0;
pub const mmVCE_SYS_INT_STATUS: u32 = 0x0b01;
pub const mmVCE_SYS_INT_STATUS_BASE_IDX: u32 = 0;

// addressBlock: vce0_ctl_dec
// base address: 0x22780
pub const mmVCE_UENC_CLOCK_GATING: u32 = 0x0bef;
pub const mmVCE_UENC_CLOCK_GATING_BASE_IDX: u32 = 0;
pub const mmVCE_UENC_REG_CLOCK_GATING: u32 = 0x0bf0;
pub const mmVCE_UENC_REG_CLOCK_GATING_BASE_IDX: u32 = 0;
pub const mmVCE_UENC_CLOCK_GATING_2: u32 = 0x0c10;
pub const mmVCE_UENC_CLOCK_GATING_2_BASE_IDX: u32 = 0;

// addressBlock: vce0_vce_sclk_dec
// base address: 0x23700
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR: u32 = 0x0fcc;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_CTRL2: u32 = 0x0fcf;
pub const mmVCE_LMI_CTRL2_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_SWAP_CNTL3: u32 = 0x0fd0;
pub const mmVCE_LMI_SWAP_CNTL3_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_CTRL: u32 = 0x0fd6;
pub const mmVCE_LMI_CTRL_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_STATUS: u32 = 0x0fd7;
pub const mmVCE_LMI_STATUS_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VM_CTRL: u32 = 0x0fd8;
pub const mmVCE_LMI_VM_CTRL_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_SWAP_CNTL: u32 = 0x0fdd;
pub const mmVCE_LMI_SWAP_CNTL_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_SWAP_CNTL1: u32 = 0x0fde;
pub const mmVCE_LMI_SWAP_CNTL1_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_SWAP_CNTL2: u32 = 0x0fe2;
pub const mmVCE_LMI_SWAP_CNTL2_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_CACHE_CTRL: u32 = 0x0fec;
pub const mmVCE_LMI_CACHE_CTRL_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR0: u32 = 0x1086;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR0_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR1: u32 = 0x1087;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR1_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR2: u32 = 0x1088;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR2_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR3: u32 = 0x1089;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR3_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR4: u32 = 0x108a;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR4_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR5: u32 = 0x108b;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR5_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR6: u32 = 0x108c;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR6_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR7: u32 = 0x108d;
pub const mmVCE_LMI_VCPU_CACHE_64BIT_BAR7_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR0: u32 = 0x1096;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR0_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR1: u32 = 0x1097;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR1_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR2: u32 = 0x1098;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR2_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR3: u32 = 0x1099;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR3_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR4: u32 = 0x109a;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR4_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR5: u32 = 0x109b;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR5_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR6: u32 = 0x109c;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR6_BASE_IDX: u32 = 0;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR7: u32 = 0x109d;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR7_BASE_IDX: u32 = 0;

// addressBlock: vce0_mmsch_dec
// base address: 0x23b00
pub const mmVCE_MMSCH_VF_VMID: u32 = 0x10cb;
pub const mmVCE_MMSCH_VF_VMID_BASE_IDX: u32 = 0;
pub const mmVCE_MMSCH_VF_CTX_ADDR_LO: u32 = 0x10cc;
pub const mmVCE_MMSCH_VF_CTX_ADDR_LO_BASE_IDX: u32 = 0;
pub const mmVCE_MMSCH_VF_CTX_ADDR_HI: u32 = 0x10cd;
pub const mmVCE_MMSCH_VF_CTX_ADDR_HI_BASE_IDX: u32 = 0;
pub const mmVCE_MMSCH_VF_CTX_SIZE: u32 = 0x10ce;
pub const mmVCE_MMSCH_VF_CTX_SIZE_BASE_IDX: u32 = 0;
pub const mmVCE_MMSCH_VF_GPCOM_ADDR_LO: u32 = 0x10cf;
pub const mmVCE_MMSCH_VF_GPCOM_ADDR_LO_BASE_IDX: u32 = 0;
pub const mmVCE_MMSCH_VF_GPCOM_ADDR_HI: u32 = 0x10d0;
pub const mmVCE_MMSCH_VF_GPCOM_ADDR_HI_BASE_IDX: u32 = 0;
pub const mmVCE_MMSCH_VF_GPCOM_SIZE: u32 = 0x10d1;
pub const mmVCE_MMSCH_VF_GPCOM_SIZE_BASE_IDX: u32 = 0;
pub const mmVCE_MMSCH_VF_MAILBOX_HOST: u32 = 0x10d2;
pub const mmVCE_MMSCH_VF_MAILBOX_HOST_BASE_IDX: u32 = 0;
pub const mmVCE_MMSCH_VF_MAILBOX_RESP: u32 = 0x10d3;
pub const mmVCE_MMSCH_VF_MAILBOX_RESP_BASE_IDX: u32 = 0;

// addressBlock: vce0_vce_rb_pg_dec
// base address: 0x23fa0
pub const mmVCE_HW_VERSION: u32 = 0x11e8;
pub const mmVCE_HW_VERSION_BASE_IDX: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
