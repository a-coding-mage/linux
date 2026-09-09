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

pub const mmVCE_STATUS: u32 = 0x8001;
pub const mmVCE_VCPU_CNTL: u32 = 0x8005;
pub const mmVCE_VCPU_CACHE_OFFSET0: u32 = 0x8009;
pub const mmVCE_VCPU_CACHE_SIZE0: u32 = 0x800a;
pub const mmVCE_VCPU_CACHE_OFFSET1: u32 = 0x800b;
pub const mmVCE_VCPU_CACHE_SIZE1: u32 = 0x800c;
pub const mmVCE_VCPU_CACHE_OFFSET2: u32 = 0x800d;
pub const mmVCE_VCPU_CACHE_SIZE2: u32 = 0x800e;
pub const mmVCE_SOFT_RESET: u32 = 0x8048;
pub const mmVCE_RB_BASE_LO2: u32 = 0x805b;
pub const mmVCE_RB_BASE_HI2: u32 = 0x805c;
pub const mmVCE_RB_SIZE2: u32 = 0x805d;
pub const mmVCE_RB_RPTR2: u32 = 0x805e;
pub const mmVCE_RB_WPTR2: u32 = 0x805f;
pub const mmVCE_RB_BASE_LO: u32 = 0x8060;
pub const mmVCE_RB_BASE_HI: u32 = 0x8061;
pub const mmVCE_RB_SIZE: u32 = 0x8062;
pub const mmVCE_RB_RPTR: u32 = 0x8063;
pub const mmVCE_RB_WPTR: u32 = 0x8064;
pub const mmVCE_RB_ARB_CTRL: u32 = 0x809f;
pub const mmVCE_CLOCK_GATING_A: u32 = 0x80be;
pub const mmVCE_CLOCK_GATING_B: u32 = 0x80bf;
pub const mmVCE_RB_BASE_LO3: u32 = 0x80d4;
pub const mmVCE_RB_BASE_HI3: u32 = 0x80d5;
pub const mmVCE_RB_SIZE3: u32 = 0x80d6;
pub const mmVCE_RB_RPTR3: u32 = 0x80d7;
pub const mmVCE_RB_WPTR3: u32 = 0x80d8;
pub const mmVCE_UENC_DMA_DCLK_CTRL: u32 = 0x8390;
pub const mmVCE_UENC_CLOCK_GATING: u32 = 0x81ef;
pub const mmVCE_UENC_REG_CLOCK_GATING: u32 = 0x81f0;
pub const mmVCE_UENC_CLOCK_GATING_2: u32 = 0x8210;
pub const mmVCE_SYS_INT_EN: u32 = 0x8540;
pub const mmVCE_SYS_INT_STATUS: u32 = 0x8541;
pub const mmVCE_SYS_INT_ACK: u32 = 0x8541;
pub const mmVCE_LMI_VCPU_CACHE_40BIT_BAR: u32 = 0x8597;
pub const mmVCE_LMI_CTRL2: u32 = 0x859d;
pub const mmVCE_LMI_SWAP_CNTL3: u32 = 0x859e;
pub const mmVCE_LMI_CTRL: u32 = 0x85a6;
pub const mmVCE_LMI_STATUS: u32 = 0x85a7;
pub const mmVCE_LMI_VM_CTRL: u32 = 0x85a8;
pub const mmVCE_LMI_SWAP_CNTL: u32 = 0x85ad;
pub const mmVCE_LMI_SWAP_CNTL1: u32 = 0x85ae;
pub const mmVCE_LMI_SWAP_CNTL2: u32 = 0x85b3;
pub const mmVCE_LMI_MISC_CTRL: u32 = 0x85b5;
pub const mmVCE_LMI_CACHE_CTRL: u32 = 0x85bd;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
