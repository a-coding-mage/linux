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

// addressBlock: hdp_hdpdec
// base address: 0x3c80

pub const regHDP_MMHUB_TLVL: u32 = 0x0000;
pub const regHDP_MMHUB_TLVL_BASE_IDX: u32 = 0;
pub const regHDP_MMHUB_UNITID: u32 = 0x0001;
pub const regHDP_MMHUB_UNITID_BASE_IDX: u32 = 0;
pub const regHDP_NONSURFACE_BASE: u32 = 0x0040;
pub const regHDP_NONSURFACE_BASE_BASE_IDX: u32 = 0;
pub const regHDP_NONSURFACE_INFO: u32 = 0x0041;
pub const regHDP_NONSURFACE_INFO_BASE_IDX: u32 = 0;
pub const regHDP_NONSURFACE_BASE_HI: u32 = 0x0042;
pub const regHDP_NONSURFACE_BASE_HI_BASE_IDX: u32 = 0;
pub const regHDP_SURFACE_WRITE_FLAGS: u32 = 0x00c4;
pub const regHDP_SURFACE_WRITE_FLAGS_BASE_IDX: u32 = 0;
pub const regHDP_SURFACE_READ_FLAGS: u32 = 0x00c5;
pub const regHDP_SURFACE_READ_FLAGS_BASE_IDX: u32 = 0;
pub const regHDP_SURFACE_WRITE_FLAGS_CLR: u32 = 0x00c6;
pub const regHDP_SURFACE_WRITE_FLAGS_CLR_BASE_IDX: u32 = 0;
pub const regHDP_SURFACE_READ_FLAGS_CLR: u32 = 0x00c7;
pub const regHDP_SURFACE_READ_FLAGS_CLR_BASE_IDX: u32 = 0;
pub const regHDP_NONSURF_FLAGS: u32 = 0x00c8;
pub const regHDP_NONSURF_FLAGS_BASE_IDX: u32 = 0;
pub const regHDP_NONSURF_FLAGS_CLR: u32 = 0x00c9;
pub const regHDP_NONSURF_FLAGS_CLR_BASE_IDX: u32 = 0;
pub const regHDP_HOST_PATH_CNTL: u32 = 0x00cc;
pub const regHDP_HOST_PATH_CNTL_BASE_IDX: u32 = 0;
pub const regHDP_SW_SEMAPHORE: u32 = 0x00cd;
pub const regHDP_SW_SEMAPHORE_BASE_IDX: u32 = 0;
pub const regHDP_DEBUG0: u32 = 0x00ce;
pub const regHDP_DEBUG0_BASE_IDX: u32 = 0;
pub const regHDP_LAST_SURFACE_HIT: u32 = 0x00d0;
pub const regHDP_LAST_SURFACE_HIT_BASE_IDX: u32 = 0;
pub const regHDP_OUTSTANDING_REQ: u32 = 0x00d2;
pub const regHDP_OUTSTANDING_REQ_BASE_IDX: u32 = 0;
pub const regHDP_MISC_CNTL: u32 = 0x00d3;
pub const regHDP_MISC_CNTL_BASE_IDX: u32 = 0;
pub const regHDP_MEM_POWER_CTRL: u32 = 0x00d4;
pub const regHDP_MEM_POWER_CTRL_BASE_IDX: u32 = 0;
pub const regHDP_MMHUB_CNTL: u32 = 0x00d5;
pub const regHDP_MMHUB_CNTL_BASE_IDX: u32 = 0;
pub const regHDP_VERSION: u32 = 0x00d7;
pub const regHDP_VERSION_BASE_IDX: u32 = 0;
pub const regHDP_CLK_CNTL: u32 = 0x00d8;
pub const regHDP_CLK_CNTL_BASE_IDX: u32 = 0;
pub const regHDP_MEMIO_CNTL: u32 = 0x00f6;
pub const regHDP_MEMIO_CNTL_BASE_IDX: u32 = 0;
pub const regHDP_MEMIO_ADDR: u32 = 0x00f7;
pub const regHDP_MEMIO_ADDR_BASE_IDX: u32 = 0;
pub const regHDP_MEMIO_STATUS: u32 = 0x00f8;
pub const regHDP_MEMIO_STATUS_BASE_IDX: u32 = 0;
pub const regHDP_MEMIO_WR_DATA: u32 = 0x00f9;
pub const regHDP_MEMIO_WR_DATA_BASE_IDX: u32 = 0;
pub const regHDP_MEMIO_RD_DATA: u32 = 0x00fa;
pub const regHDP_MEMIO_RD_DATA_BASE_IDX: u32 = 0;
pub const regHDP_XDP_DIRECT2HDP_FIRST: u32 = 0x0100;
pub const regHDP_XDP_DIRECT2HDP_FIRST_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_FLUSH: u32 = 0x0101;
pub const regHDP_XDP_D2H_FLUSH_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_BAR_UPDATE: u32 = 0x0102;
pub const regHDP_XDP_D2H_BAR_UPDATE_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_3: u32 = 0x0103;
pub const regHDP_XDP_D2H_RSVD_3_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_4: u32 = 0x0104;
pub const regHDP_XDP_D2H_RSVD_4_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_5: u32 = 0x0105;
pub const regHDP_XDP_D2H_RSVD_5_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_6: u32 = 0x0106;
pub const regHDP_XDP_D2H_RSVD_6_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_7: u32 = 0x0107;
pub const regHDP_XDP_D2H_RSVD_7_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_8: u32 = 0x0108;
pub const regHDP_XDP_D2H_RSVD_8_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_9: u32 = 0x0109;
pub const regHDP_XDP_D2H_RSVD_9_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_10: u32 = 0x010a;
pub const regHDP_XDP_D2H_RSVD_10_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_11: u32 = 0x010b;
pub const regHDP_XDP_D2H_RSVD_11_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_12: u32 = 0x010c;
pub const regHDP_XDP_D2H_RSVD_12_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_13: u32 = 0x010d;
pub const regHDP_XDP_D2H_RSVD_13_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_14: u32 = 0x010e;
pub const regHDP_XDP_D2H_RSVD_14_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_15: u32 = 0x010f;
pub const regHDP_XDP_D2H_RSVD_15_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_16: u32 = 0x0110;
pub const regHDP_XDP_D2H_RSVD_16_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_17: u32 = 0x0111;
pub const regHDP_XDP_D2H_RSVD_17_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_18: u32 = 0x0112;
pub const regHDP_XDP_D2H_RSVD_18_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_19: u32 = 0x0113;
pub const regHDP_XDP_D2H_RSVD_19_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_20: u32 = 0x0114;
pub const regHDP_XDP_D2H_RSVD_20_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_21: u32 = 0x0115;
pub const regHDP_XDP_D2H_RSVD_21_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_22: u32 = 0x0116;
pub const regHDP_XDP_D2H_RSVD_22_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_23: u32 = 0x0117;
pub const regHDP_XDP_D2H_RSVD_23_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_24: u32 = 0x0118;
pub const regHDP_XDP_D2H_RSVD_24_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_25: u32 = 0x0119;
pub const regHDP_XDP_D2H_RSVD_25_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_26: u32 = 0x011a;
pub const regHDP_XDP_D2H_RSVD_26_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_27: u32 = 0x011b;
pub const regHDP_XDP_D2H_RSVD_27_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_28: u32 = 0x011c;
pub const regHDP_XDP_D2H_RSVD_28_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_29: u32 = 0x011d;
pub const regHDP_XDP_D2H_RSVD_29_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_30: u32 = 0x011e;
pub const regHDP_XDP_D2H_RSVD_30_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_31: u32 = 0x011f;
pub const regHDP_XDP_D2H_RSVD_31_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_32: u32 = 0x0120;
pub const regHDP_XDP_D2H_RSVD_32_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_33: u32 = 0x0121;
pub const regHDP_XDP_D2H_RSVD_33_BASE_IDX: u32 = 0;
pub const regHDP_XDP_D2H_RSVD_34: u32 = 0x0122;
pub const regHDP_XDP_D2H_RSVD_34_BASE_IDX: u32 = 0;
pub const regHDP_XDP_DIRECT2HDP_LAST: u32 = 0x0123;
pub const regHDP_XDP_DIRECT2HDP_LAST_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_BAR_CFG: u32 = 0x0124;
pub const regHDP_XDP_P2P_BAR_CFG_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_MBX_OFFSET: u32 = 0x0125;
pub const regHDP_XDP_P2P_MBX_OFFSET_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_MBX_ADDR0: u32 = 0x0126;
pub const regHDP_XDP_P2P_MBX_ADDR0_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_MBX_ADDR1: u32 = 0x0127;
pub const regHDP_XDP_P2P_MBX_ADDR1_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_MBX_ADDR2: u32 = 0x0128;
pub const regHDP_XDP_P2P_MBX_ADDR2_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_MBX_ADDR3: u32 = 0x0129;
pub const regHDP_XDP_P2P_MBX_ADDR3_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_MBX_ADDR4: u32 = 0x012a;
pub const regHDP_XDP_P2P_MBX_ADDR4_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_MBX_ADDR5: u32 = 0x012b;
pub const regHDP_XDP_P2P_MBX_ADDR5_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_MBX_ADDR6: u32 = 0x012c;
pub const regHDP_XDP_P2P_MBX_ADDR6_BASE_IDX: u32 = 0;
pub const regHDP_XDP_HDP_MBX_MC_CFG: u32 = 0x012d;
pub const regHDP_XDP_HDP_MBX_MC_CFG_BASE_IDX: u32 = 0;
pub const regHDP_XDP_HDP_MC_CFG: u32 = 0x012e;
pub const regHDP_XDP_HDP_MC_CFG_BASE_IDX: u32 = 0;
pub const regHDP_XDP_HST_CFG: u32 = 0x012f;
pub const regHDP_XDP_HST_CFG_BASE_IDX: u32 = 0;
pub const regHDP_XDP_HDP_IPH_CFG: u32 = 0x0131;
pub const regHDP_XDP_HDP_IPH_CFG_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_BAR0: u32 = 0x0134;
pub const regHDP_XDP_P2P_BAR0_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_BAR1: u32 = 0x0135;
pub const regHDP_XDP_P2P_BAR1_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_BAR2: u32 = 0x0136;
pub const regHDP_XDP_P2P_BAR2_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_BAR3: u32 = 0x0137;
pub const regHDP_XDP_P2P_BAR3_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_BAR4: u32 = 0x0138;
pub const regHDP_XDP_P2P_BAR4_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_BAR5: u32 = 0x0139;
pub const regHDP_XDP_P2P_BAR5_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_BAR6: u32 = 0x013a;
pub const regHDP_XDP_P2P_BAR6_BASE_IDX: u32 = 0;
pub const regHDP_XDP_P2P_BAR7: u32 = 0x013b;
pub const regHDP_XDP_P2P_BAR7_BASE_IDX: u32 = 0;
pub const regHDP_XDP_FLUSH_ARMED_STS: u32 = 0x013c;
pub const regHDP_XDP_FLUSH_ARMED_STS_BASE_IDX: u32 = 0;
pub const regHDP_XDP_FLUSH_CNTR0_STS: u32 = 0x013d;
pub const regHDP_XDP_FLUSH_CNTR0_STS_BASE_IDX: u32 = 0;
pub const regHDP_XDP_BUSY_STS: u32 = 0x013e;
pub const regHDP_XDP_BUSY_STS_BASE_IDX: u32 = 0;
pub const regHDP_XDP_STICKY: u32 = 0x013f;
pub const regHDP_XDP_STICKY_BASE_IDX: u32 = 0;
pub const regHDP_XDP_CHKN: u32 = 0x0140;
pub const regHDP_XDP_CHKN_BASE_IDX: u32 = 0;
pub const regHDP_XDP_BARS_ADDR_39_36: u32 = 0x0144;
pub const regHDP_XDP_BARS_ADDR_39_36_BASE_IDX: u32 = 0;
pub const regHDP_XDP_MC_VM_FB_LOCATION_BASE: u32 = 0x0145;
pub const regHDP_XDP_MC_VM_FB_LOCATION_BASE_BASE_IDX: u32 = 0;
pub const regHDP_XDP_GPU_IOV_VIOLATION_LOG: u32 = 0x0148;
pub const regHDP_XDP_GPU_IOV_VIOLATION_LOG_BASE_IDX: u32 = 0;
pub const regHDP_XDP_GPU_IOV_VIOLATION_LOG2: u32 = 0x0149;
pub const regHDP_XDP_GPU_IOV_VIOLATION_LOG2_BASE_IDX: u32 = 0;
pub const regHDP_XDP_MMHUB_ERROR: u32 = 0x014a;
pub const regHDP_XDP_MMHUB_ERROR_BASE_IDX: u32 = 0;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
