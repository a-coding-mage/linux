/*
 * VCE_3_0 Register documentation
 *
 * Copyright (C) 2014  Advanced Micro Devices, Inc.
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

pub const VCE_STATUS__JOB_BUSY_MASK: u32 = 0x1;
pub const VCE_STATUS__JOB_BUSY__SHIFT: u32 = 0x0;
pub const VCE_STATUS__VCPU_REPORT_MASK: u32 = 0xfe;
pub const VCE_STATUS__VCPU_REPORT__SHIFT: u32 = 0x1;
pub const VCE_STATUS__UENC_BUSY_MASK: u32 = 0x100;
pub const VCE_STATUS__UENC_BUSY__SHIFT: u32 = 0x8;
pub const VCE_STATUS__VCE_CONFIGURATION_MASK: u32 = 0xc00000;
pub const VCE_STATUS__VCE_CONFIGURATION__SHIFT: u32 = 0x16;
pub const VCE_STATUS__VCE_INSTANCE_ID_MASK: u32 = 0x3000000;
pub const VCE_STATUS__VCE_INSTANCE_ID__SHIFT: u32 = 0x18;
pub const VCE_VCPU_CNTL__CLK_EN_MASK: u32 = 0x1;
pub const VCE_VCPU_CNTL__CLK_EN__SHIFT: u32 = 0x0;
pub const VCE_VCPU_CNTL__RBBM_SOFT_RESET_MASK: u32 = 0x40000;
pub const VCE_VCPU_CNTL__RBBM_SOFT_RESET__SHIFT: u32 = 0x12;
pub const VCE_VCPU_CACHE_OFFSET0__OFFSET_MASK: u32 = 0xfffffff;
pub const VCE_VCPU_CACHE_OFFSET0__OFFSET__SHIFT: u32 = 0x0;
pub const VCE_VCPU_CACHE_SIZE0__SIZE_MASK: u32 = 0xffffff;
pub const VCE_VCPU_CACHE_SIZE0__SIZE__SHIFT: u32 = 0x0;
pub const VCE_VCPU_CACHE_OFFSET1__OFFSET_MASK: u32 = 0xfffffff;
pub const VCE_VCPU_CACHE_OFFSET1__OFFSET__SHIFT: u32 = 0x0;
pub const VCE_VCPU_CACHE_SIZE1__SIZE_MASK: u32 = 0xffffff;
pub const VCE_VCPU_CACHE_SIZE1__SIZE__SHIFT: u32 = 0x0;
pub const VCE_VCPU_CACHE_OFFSET2__OFFSET_MASK: u32 = 0xfffffff;
pub const VCE_VCPU_CACHE_OFFSET2__OFFSET__SHIFT: u32 = 0x0;
pub const VCE_VCPU_CACHE_SIZE2__SIZE_MASK: u32 = 0xffffff;
pub const VCE_VCPU_CACHE_SIZE2__SIZE__SHIFT: u32 = 0x0;
pub const VCE_SOFT_RESET__ECPU_SOFT_RESET_MASK: u32 = 0x1;
pub const VCE_SOFT_RESET__ECPU_SOFT_RESET__SHIFT: u32 = 0x0;
pub const VCE_RB_BASE_LO2__RB_BASE_LO_MASK: u32 = 0xffffffc0;
pub const VCE_RB_BASE_LO2__RB_BASE_LO__SHIFT: u32 = 0x6;
pub const VCE_RB_BASE_HI2__RB_BASE_HI_MASK: u32 = 0xffffffff;
pub const VCE_RB_BASE_HI2__RB_BASE_HI__SHIFT: u32 = 0x0;
pub const VCE_RB_SIZE2__RB_SIZE_MASK: u32 = 0x7ffff0;
pub const VCE_RB_SIZE2__RB_SIZE__SHIFT: u32 = 0x4;
pub const VCE_RB_RPTR2__RB_RPTR_MASK: u32 = 0x7ffff0;
pub const VCE_RB_RPTR2__RB_RPTR__SHIFT: u32 = 0x4;
pub const VCE_RB_WPTR2__RB_WPTR_MASK: u32 = 0x7ffff0;
pub const VCE_RB_WPTR2__RB_WPTR__SHIFT: u32 = 0x4;
pub const VCE_RB_BASE_LO__RB_BASE_LO_MASK: u32 = 0xffffffc0;
pub const VCE_RB_BASE_LO__RB_BASE_LO__SHIFT: u32 = 0x6;
pub const VCE_RB_BASE_HI__RB_BASE_HI_MASK: u32 = 0xffffffff;
pub const VCE_RB_BASE_HI__RB_BASE_HI__SHIFT: u32 = 0x0;
pub const VCE_RB_SIZE__RB_SIZE_MASK: u32 = 0x7ffff0;
pub const VCE_RB_SIZE__RB_SIZE__SHIFT: u32 = 0x4;
pub const VCE_RB_RPTR__RB_RPTR_MASK: u32 = 0x7ffff0;
pub const VCE_RB_RPTR__RB_RPTR__SHIFT: u32 = 0x4;
pub const VCE_RB_WPTR__RB_WPTR_MASK: u32 = 0x7ffff0;
pub const VCE_RB_WPTR__RB_WPTR__SHIFT: u32 = 0x4;
pub const VCE_RB_ARB_CTRL__VCE_CGTT_OVERRIDE_MASK: u32 = 0x10000;
pub const VCE_RB_ARB_CTRL__VCE_CGTT_OVERRIDE__SHIFT: u32 = 0x10;
pub const VCE_RB_BASE_LO3__RB_BASE_LO_MASK: u32 = 0xffffffc0;
pub const VCE_RB_BASE_LO3__RB_BASE_LO__SHIFT: u32 = 0x6;
pub const VCE_RB_BASE_HI3__RB_BASE_HI_MASK: u32 = 0xffffffff;
pub const VCE_RB_BASE_HI3__RB_BASE_HI__SHIFT: u32 = 0x0;
pub const VCE_RB_SIZE3__RB_SIZE_MASK: u32 = 0x7ffff0;
pub const VCE_RB_SIZE3__RB_SIZE__SHIFT: u32 = 0x4;
pub const VCE_RB_RPTR3__RB_RPTR_MASK: u32 = 0x7ffff0;
pub const VCE_RB_RPTR3__RB_RPTR__SHIFT: u32 = 0x4;
pub const VCE_RB_WPTR3__RB_WPTR_MASK: u32 = 0x7ffff0;
pub const VCE_RB_WPTR3__RB_WPTR__SHIFT: u32 = 0x4;
pub const VCE_UENC_DMA_DCLK_CTRL__WRDMCLK_FORCEON_MASK: u32 = 0x1;
pub const VCE_UENC_DMA_DCLK_CTRL__WRDMCLK_FORCEON__SHIFT: u32 = 0x0;
pub const VCE_UENC_DMA_DCLK_CTRL__RDDMCLK_FORCEON_MASK: u32 = 0x2;
pub const VCE_UENC_DMA_DCLK_CTRL__RDDMCLK_FORCEON__SHIFT: u32 = 0x1;
pub const VCE_UENC_DMA_DCLK_CTRL__REGCLK_FORCEON_MASK: u32 = 0x4;
pub const VCE_UENC_DMA_DCLK_CTRL__REGCLK_FORCEON__SHIFT: u32 = 0x2;
pub const VCE_SYS_INT_EN__VCE_SYS_INT_TRAP_INTERRUPT_EN_MASK: u32 = 0x8;
pub const VCE_SYS_INT_EN__VCE_SYS_INT_TRAP_INTERRUPT_EN__SHIFT: u32 = 0x3;
pub const VCE_SYS_INT_STATUS__VCE_SYS_INT_TRAP_INTERRUPT_INT_MASK: u32 = 0x8;
pub const VCE_SYS_INT_STATUS__VCE_SYS_INT_TRAP_INTERRUPT_INT__SHIFT: u32 = 0x3;
pub const VCE_SYS_INT_ACK__VCE_SYS_INT_TRAP_INTERRUPT_ACK_MASK: u32 = 0x8;
pub const VCE_SYS_INT_ACK__VCE_SYS_INT_TRAP_INTERRUPT_ACK__SHIFT: u32 = 0x3;
pub const VCE_LMI_VCPU_CACHE_40BIT_BAR__BAR_MASK: u32 = 0xffffffff;
pub const VCE_LMI_VCPU_CACHE_40BIT_BAR__BAR__SHIFT: u32 = 0x0;
pub const VCE_LMI_CTRL2__STALL_ARB_UMC_MASK: u32 = 0x100;
pub const VCE_LMI_CTRL2__STALL_ARB_UMC__SHIFT: u32 = 0x8;
pub const VCE_LMI_SWAP_CNTL3__RD_MC_CID_SWAP_MASK: u32 = 0x3;
pub const VCE_LMI_SWAP_CNTL3__RD_MC_CID_SWAP__SHIFT: u32 = 0x0;
pub const VCE_LMI_CTRL__VCPU_DATA_COHERENCY_EN_MASK: u32 = 0x200000;
pub const VCE_LMI_CTRL__VCPU_DATA_COHERENCY_EN__SHIFT: u32 = 0x15;
pub const VCE_LMI_SWAP_CNTL__VCPU_W_MC_SWAP_MASK: u32 = 0x3;
pub const VCE_LMI_SWAP_CNTL__VCPU_W_MC_SWAP__SHIFT: u32 = 0x0;
pub const VCE_LMI_SWAP_CNTL__WR_MC_CID_SWAP_MASK: u32 = 0x3ffc;
pub const VCE_LMI_SWAP_CNTL__WR_MC_CID_SWAP__SHIFT: u32 = 0x2;
pub const VCE_LMI_SWAP_CNTL1__VCPU_R_MC_SWAP_MASK: u32 = 0x3;
pub const VCE_LMI_SWAP_CNTL1__VCPU_R_MC_SWAP__SHIFT: u32 = 0x0;
pub const VCE_LMI_SWAP_CNTL1__RD_MC_CID_SWAP_MASK: u32 = 0x3ffc;
pub const VCE_LMI_SWAP_CNTL1__RD_MC_CID_SWAP__SHIFT: u32 = 0x2;
pub const VCE_LMI_SWAP_CNTL2__WR_MC_CID_SWAP_MASK: u32 = 0xff;
pub const VCE_LMI_SWAP_CNTL2__WR_MC_CID_SWAP__SHIFT: u32 = 0x0;
pub const VCE_LMI_CACHE_CTRL__VCPU_EN_MASK: u32 = 0x1;
pub const VCE_LMI_CACHE_CTRL__VCPU_EN__SHIFT: u32 = 0x0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
