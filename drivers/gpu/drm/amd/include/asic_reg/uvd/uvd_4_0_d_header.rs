/*
 *
 * Copyright (C) 2016 Advanced Micro Devices, Inc.
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

// Translated from C header guard UVD_4_0_D_H.

pub const ixUVD_CGC_CTRL2: u32 = 0x00C1;
pub const ixUVD_CGC_MEM_CTRL: u32 = 0x00C0;
pub const ixUVD_LMI_ADDR_EXT2: u32 = 0x00AB;
pub const ixUVD_LMI_CACHE_CTRL: u32 = 0x009B;
pub const ixUVD_LMI_SWAP_CNTL2: u32 = 0x00AA;
pub const ixUVD_MIF_CURR_ADDR_CONFIG: u32 = 0x0048;
pub const ixUVD_MIF_RECON1_ADDR_CONFIG: u32 = 0x0114;
pub const ixUVD_MIF_REF_ADDR_CONFIG: u32 = 0x004C;
pub const mmUVD_CGC_CTRL: u32 = 0x3D2C;
pub const mmUVD_CGC_GATE: u32 = 0x3D2A;
pub const mmUVD_CGC_STATUS: u32 = 0x3D2B;
pub const mmUVD_CGC_UDEC_STATUS: u32 = 0x3D2D;
pub const mmUVD_CONTEXT_ID: u32 = 0x3DBD;
pub const mmUVD_CTX_DATA: u32 = 0x3D29;
pub const mmUVD_CTX_INDEX: u32 = 0x3D28;
pub const mmUVD_ENGINE_CNTL: u32 = 0x3BC6;
pub const mmUVD_GPCOM_VCPU_CMD: u32 = 0x3BC3;
pub const mmUVD_GPCOM_VCPU_DATA0: u32 = 0x3BC4;
pub const mmUVD_GPCOM_VCPU_DATA1: u32 = 0x3BC5;
pub const mmUVD_GP_SCRATCH4: u32 = 0x3D38;
pub const mmUVD_LMI_ADDR_EXT: u32 = 0x3D65;
pub const mmUVD_LMI_CTRL: u32 = 0x3D66;
pub const mmUVD_LMI_CTRL2: u32 = 0x3D3D;
pub const mmUVD_LMI_EXT40_ADDR: u32 = 0x3D26;
pub const mmUVD_LMI_STATUS: u32 = 0x3D67;
pub const mmUVD_LMI_SWAP_CNTL: u32 = 0x3D6D;
pub const mmUVD_MASTINT_EN: u32 = 0x3D40;
pub const mmUVD_MPC_CNTL: u32 = 0x3D77;
pub const mmUVD_MPC_SET_ALU: u32 = 0x3D7E;
pub const mmUVD_MPC_SET_MUX: u32 = 0x3D7D;
pub const mmUVD_MPC_SET_MUXA0: u32 = 0x3D79;
pub const mmUVD_MPC_SET_MUXA1: u32 = 0x3D7A;
pub const mmUVD_MPC_SET_MUXB0: u32 = 0x3D7B;
pub const mmUVD_MPC_SET_MUXB1: u32 = 0x3D7C;
pub const mmUVD_MP_SWAP_CNTL: u32 = 0x3D6F;
pub const mmUVD_NO_OP: u32 = 0x3BFF;
pub const mmUVD_PGFSM_CONFIG: u32 = 0x38F8;
pub const mmUVD_PGFSM_READ_TILE1: u32 = 0x38FA;
pub const mmUVD_PGFSM_READ_TILE2: u32 = 0x38FB;
pub const mmUVD_POWER_STATUS: u32 = 0x38FC;
pub const mmUVD_RBC_IB_BASE: u32 = 0x3DA1;
pub const mmUVD_RBC_IB_SIZE: u32 = 0x3DA2;
pub const mmUVD_RBC_IB_SIZE_UPDATE: u32 = 0x3DF1;
pub const mmUVD_RBC_RB_BASE: u32 = 0x3DA3;
pub const mmUVD_RBC_RB_CNTL: u32 = 0x3DA9;
pub const mmUVD_RBC_RB_RPTR: u32 = 0x3DA4;
pub const mmUVD_RBC_RB_RPTR_ADDR: u32 = 0x3DAA;
pub const mmUVD_RBC_RB_WPTR: u32 = 0x3DA5;
pub const mmUVD_RBC_RB_WPTR_CNTL: u32 = 0x3DA6;
pub const mmUVD_SEMA_ADDR_HIGH: u32 = 0x3BC1;
pub const mmUVD_SEMA_ADDR_LOW: u32 = 0x3BC0;
pub const mmUVD_SEMA_CMD: u32 = 0x3BC2;
pub const mmUVD_SEMA_CNTL: u32 = 0x3D00;
pub const mmUVD_SEMA_SIGNAL_INCOMPLETE_TIMEOUT_CNTL: u32 = 0x3DB3;
pub const mmUVD_SEMA_TIMEOUT_STATUS: u32 = 0x3DB0;
pub const mmUVD_SEMA_WAIT_FAULT_TIMEOUT_CNTL: u32 = 0x3DB2;
pub const mmUVD_SEMA_WAIT_INCOMPLETE_TIMEOUT_CNTL: u32 = 0x3DB1;
pub const mmUVD_SOFT_RESET: u32 = 0x3DA0;
pub const mmUVD_STATUS: u32 = 0x3DAF;
pub const mmUVD_UDEC_ADDR_CONFIG: u32 = 0x3BD3;
pub const mmUVD_UDEC_DB_ADDR_CONFIG: u32 = 0x3BD4;
pub const mmUVD_UDEC_DBW_ADDR_CONFIG: u32 = 0x3BD5;
pub const mmUVD_VCPU_CACHE_OFFSET0: u32 = 0x3D36;
pub const mmUVD_VCPU_CACHE_OFFSET1: u32 = 0x3D38;
pub const mmUVD_VCPU_CACHE_OFFSET2: u32 = 0x3D3A;
pub const mmUVD_VCPU_CACHE_SIZE0: u32 = 0x3D37;
pub const mmUVD_VCPU_CACHE_SIZE1: u32 = 0x3D39;
pub const mmUVD_VCPU_CACHE_SIZE2: u32 = 0x3D3B;
pub const mmUVD_VCPU_CNTL: u32 = 0x3D98;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
