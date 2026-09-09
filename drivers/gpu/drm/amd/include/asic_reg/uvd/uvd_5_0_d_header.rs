/*
 * UVD_5_0 Register documentation
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

pub const mmUVD_SEMA_ADDR_LOW: u32 = 0x3bc0;
pub const mmUVD_SEMA_ADDR_HIGH: u32 = 0x3bc1;
pub const mmUVD_SEMA_CMD: u32 = 0x3bc2;
pub const mmUVD_GPCOM_VCPU_CMD: u32 = 0x3bc3;
pub const mmUVD_GPCOM_VCPU_DATA0: u32 = 0x3bc4;
pub const mmUVD_GPCOM_VCPU_DATA1: u32 = 0x3bc5;
pub const mmUVD_ENGINE_CNTL: u32 = 0x3bc6;
pub const mmUVD_UDEC_ADDR_CONFIG: u32 = 0x3bd3;
pub const mmUVD_UDEC_DB_ADDR_CONFIG: u32 = 0x3bd4;
pub const mmUVD_UDEC_DBW_ADDR_CONFIG: u32 = 0x3bd5;
pub const mmUVD_NO_OP: u32 = 0x3bff;
pub const mmUVD_LMI_RBC_RB_64BIT_BAR_LOW: u32 = 0x3c69;
pub const mmUVD_LMI_RBC_RB_64BIT_BAR_HIGH: u32 = 0x3c68;
pub const mmUVD_LMI_RBC_IB_64BIT_BAR_LOW: u32 = 0x3c67;
pub const mmUVD_LMI_RBC_IB_64BIT_BAR_HIGH: u32 = 0x3c66;
pub const mmUVD_LMI_VCPU_CACHE_64BIT_BAR_LOW: u32 = 0x3c5f;
pub const mmUVD_LMI_VCPU_CACHE_64BIT_BAR_HIGH: u32 = 0x3c5e;
pub const mmUVD_SEMA_CNTL: u32 = 0x3d00;
pub const mmUVD_LMI_EXT40_ADDR: u32 = 0x3d26;
pub const mmUVD_CTX_INDEX: u32 = 0x3d28;
pub const mmUVD_CTX_DATA: u32 = 0x3d29;
pub const mmUVD_CGC_GATE: u32 = 0x3d2a;
pub const mmUVD_CGC_STATUS: u32 = 0x3d2b;
pub const mmUVD_CGC_CTRL: u32 = 0x3d2c;
pub const mmUVD_CGC_UDEC_STATUS: u32 = 0x3d2d;
pub const mmUVD_LMI_CTRL2: u32 = 0x3d3d;
pub const mmUVD_MASTINT_EN: u32 = 0x3d40;
pub const mmUVD_LMI_ADDR_EXT: u32 = 0x3d65;
pub const mmUVD_LMI_CTRL: u32 = 0x3d66;
pub const mmUVD_LMI_STATUS: u32 = 0x3d67;
pub const mmUVD_LMI_SWAP_CNTL: u32 = 0x3d6d;
pub const mmUVD_MP_SWAP_CNTL: u32 = 0x3d6f;
pub const mmUVD_MPC_CNTL: u32 = 0x3d77;
pub const mmUVD_MPC_SET_MUXA0: u32 = 0x3d79;
pub const mmUVD_MPC_SET_MUXA1: u32 = 0x3d7a;
pub const mmUVD_MPC_SET_MUXB0: u32 = 0x3d7b;
pub const mmUVD_MPC_SET_MUXB1: u32 = 0x3d7c;
pub const mmUVD_MPC_SET_MUX: u32 = 0x3d7d;
pub const mmUVD_MPC_SET_ALU: u32 = 0x3d7e;
pub const mmUVD_VCPU_CACHE_OFFSET0: u32 = 0x3d82;
pub const mmUVD_VCPU_CACHE_SIZE0: u32 = 0x3d83;
pub const mmUVD_VCPU_CACHE_OFFSET1: u32 = 0x3d84;
pub const mmUVD_VCPU_CACHE_SIZE1: u32 = 0x3d85;
pub const mmUVD_VCPU_CACHE_OFFSET2: u32 = 0x3d86;
pub const mmUVD_VCPU_CACHE_SIZE2: u32 = 0x3d87;
pub const mmUVD_VCPU_CNTL: u32 = 0x3d98;
pub const mmUVD_SOFT_RESET: u32 = 0x3da0;
pub const mmUVD_LMI_RBC_IB_VMID: u32 = 0x3da1;
pub const mmUVD_RBC_IB_SIZE: u32 = 0x3da2;
pub const mmUVD_LMI_RBC_RB_VMID: u32 = 0x3da3;
pub const mmUVD_RBC_RB_RPTR: u32 = 0x3da4;
pub const mmUVD_RBC_RB_WPTR: u32 = 0x3da5;
pub const mmUVD_RBC_RB_WPTR_CNTL: u32 = 0x3da6;
pub const mmUVD_RBC_RB_CNTL: u32 = 0x3da9;
pub const mmUVD_RBC_RB_RPTR_ADDR: u32 = 0x3daa;
pub const mmUVD_STATUS: u32 = 0x3daf;
pub const mmUVD_SEMA_TIMEOUT_STATUS: u32 = 0x3db0;
pub const mmUVD_SEMA_WAIT_INCOMPLETE_TIMEOUT_CNTL: u32 = 0x3db1;
pub const mmUVD_SEMA_WAIT_FAULT_TIMEOUT_CNTL: u32 = 0x3db2;
pub const mmUVD_SEMA_SIGNAL_INCOMPLETE_TIMEOUT_CNTL: u32 = 0x3db3;
pub const mmUVD_CONTEXT_ID: u32 = 0x3dbd;
pub const mmUVD_RBC_IB_SIZE_UPDATE: u32 = 0x3df1;
pub const mmUVD_SUVD_CGC_GATE: u32 = 0x3be4;
pub const mmUVD_SUVD_CGC_STATUS: u32 = 0x3be5;
pub const mmUVD_SUVD_CGC_CTRL: u32 = 0x3be6;
pub const ixUVD_LMI_VMID_INTERNAL: u32 = 0x99;
pub const ixUVD_LMI_VMID_INTERNAL2: u32 = 0x9a;
pub const ixUVD_LMI_CACHE_CTRL: u32 = 0x9b;
pub const ixUVD_LMI_SWAP_CNTL2: u32 = 0xaa;
pub const ixUVD_LMI_ADDR_EXT2: u32 = 0xab;
pub const ixUVD_CGC_MEM_CTRL: u32 = 0xc0;
pub const ixUVD_CGC_CTRL2: u32 = 0xc1;
pub const ixUVD_LMI_VMID_INTERNAL3: u32 = 0x162;
pub const mmUVD_PGFSM_CONFIG: u32 = 0x38c0;
pub const mmUVD_PGFSM_READ_TILE1: u32 = 0x38c2;
pub const mmUVD_PGFSM_READ_TILE2: u32 = 0x38c3;
pub const mmUVD_POWER_STATUS: u32 = 0x38c4;
pub const mmUVD_PGFSM_READ_TILE3: u32 = 0x38c5;
pub const mmUVD_PGFSM_READ_TILE4: u32 = 0x38c6;
pub const mmUVD_PGFSM_READ_TILE5: u32 = 0x38c8;
pub const mmUVD_PGFSM_READ_TILE6: u32 = 0x38ee;
pub const mmUVD_PGFSM_READ_TILE7: u32 = 0x38ef;
pub const mmUVD_MIF_CURR_ADDR_CONFIG: u32 = 0x3992;
pub const mmUVD_MIF_REF_ADDR_CONFIG: u32 = 0x3993;
pub const mmUVD_MIF_RECON1_ADDR_CONFIG: u32 = 0x39c5;
pub const ixUVD_MIF_SCLR_ADDR_CONFIG: u32 = 0x4;
pub const mmUVD_JPEG_ADDR_CONFIG: u32 = 0x3a1f;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
