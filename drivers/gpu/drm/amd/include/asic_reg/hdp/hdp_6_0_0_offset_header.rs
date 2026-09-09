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
 *
 */



// addressBlock: hdp_hdpdec
// base address: 0x3c80
pub const regHDP_NONSURFACE_BASE: u32 = 0x0040u32;
pub const regHDP_NONSURFACE_BASE_BASE_IDX: u32 = 0u32;
pub const regHDP_NONSURFACE_INFO: u32 = 0x0041u32;
pub const regHDP_NONSURFACE_INFO_BASE_IDX: u32 = 0u32;
pub const regHDP_NONSURFACE_BASE_HI: u32 = 0x0042u32;
pub const regHDP_NONSURFACE_BASE_HI_BASE_IDX: u32 = 0u32;
pub const regHDP_SURFACE_WRITE_FLAGS: u32 = 0x00c4u32;
pub const regHDP_SURFACE_WRITE_FLAGS_BASE_IDX: u32 = 0u32;
pub const regHDP_SURFACE_READ_FLAGS: u32 = 0x00c5u32;
pub const regHDP_SURFACE_READ_FLAGS_BASE_IDX: u32 = 0u32;
pub const regHDP_SURFACE_WRITE_FLAGS_CLR: u32 = 0x00c6u32;
pub const regHDP_SURFACE_WRITE_FLAGS_CLR_BASE_IDX: u32 = 0u32;
pub const regHDP_SURFACE_READ_FLAGS_CLR: u32 = 0x00c7u32;
pub const regHDP_SURFACE_READ_FLAGS_CLR_BASE_IDX: u32 = 0u32;
pub const regHDP_NONSURF_FLAGS: u32 = 0x00c8u32;
pub const regHDP_NONSURF_FLAGS_BASE_IDX: u32 = 0u32;
pub const regHDP_NONSURF_FLAGS_CLR: u32 = 0x00c9u32;
pub const regHDP_NONSURF_FLAGS_CLR_BASE_IDX: u32 = 0u32;
pub const regHDP_HOST_PATH_CNTL: u32 = 0x00ccu32;
pub const regHDP_HOST_PATH_CNTL_BASE_IDX: u32 = 0u32;
pub const regHDP_SW_SEMAPHORE: u32 = 0x00cdu32;
pub const regHDP_SW_SEMAPHORE_BASE_IDX: u32 = 0u32;
pub const regHDP_DEBUG0: u32 = 0x00ceu32;
pub const regHDP_DEBUG0_BASE_IDX: u32 = 0u32;
pub const regHDP_LAST_SURFACE_HIT: u32 = 0x00d0u32;
pub const regHDP_LAST_SURFACE_HIT_BASE_IDX: u32 = 0u32;
pub const regHDP_OUTSTANDING_REQ: u32 = 0x00d2u32;
pub const regHDP_OUTSTANDING_REQ_BASE_IDX: u32 = 0u32;
pub const regHDP_MISC_CNTL: u32 = 0x00d3u32;
pub const regHDP_MISC_CNTL_BASE_IDX: u32 = 0u32;
pub const regHDP_MEM_POWER_CTRL: u32 = 0x00d4u32;
pub const regHDP_MEM_POWER_CTRL_BASE_IDX: u32 = 0u32;
pub const regHDP_MMHUB_CNTL: u32 = 0x00d5u32;
pub const regHDP_MMHUB_CNTL_BASE_IDX: u32 = 0u32;
pub const regHDP_VERSION: u32 = 0x00d7u32;
pub const regHDP_VERSION_BASE_IDX: u32 = 0u32;
pub const regHDP_CLK_CNTL: u32 = 0x00d8u32;
pub const regHDP_CLK_CNTL_BASE_IDX: u32 = 0u32;
pub const regHDP_MEMIO_CNTL: u32 = 0x00f6u32;
pub const regHDP_MEMIO_CNTL_BASE_IDX: u32 = 0u32;
pub const regHDP_MEMIO_ADDR: u32 = 0x00f7u32;
pub const regHDP_MEMIO_ADDR_BASE_IDX: u32 = 0u32;
pub const regHDP_MEMIO_STATUS: u32 = 0x00f8u32;
pub const regHDP_MEMIO_STATUS_BASE_IDX: u32 = 0u32;
pub const regHDP_MEMIO_WR_DATA: u32 = 0x00f9u32;
pub const regHDP_MEMIO_WR_DATA_BASE_IDX: u32 = 0u32;
pub const regHDP_MEMIO_RD_DATA: u32 = 0x00fau32;
pub const regHDP_MEMIO_RD_DATA_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_DIRECT2HDP_FIRST: u32 = 0x0100u32;
pub const regHDP_XDP_DIRECT2HDP_FIRST_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_FLUSH: u32 = 0x0101u32;
pub const regHDP_XDP_D2H_FLUSH_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_BAR_UPDATE: u32 = 0x0102u32;
pub const regHDP_XDP_D2H_BAR_UPDATE_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_3: u32 = 0x0103u32;
pub const regHDP_XDP_D2H_RSVD_3_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_4: u32 = 0x0104u32;
pub const regHDP_XDP_D2H_RSVD_4_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_5: u32 = 0x0105u32;
pub const regHDP_XDP_D2H_RSVD_5_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_6: u32 = 0x0106u32;
pub const regHDP_XDP_D2H_RSVD_6_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_7: u32 = 0x0107u32;
pub const regHDP_XDP_D2H_RSVD_7_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_8: u32 = 0x0108u32;
pub const regHDP_XDP_D2H_RSVD_8_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_9: u32 = 0x0109u32;
pub const regHDP_XDP_D2H_RSVD_9_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_10: u32 = 0x010au32;
pub const regHDP_XDP_D2H_RSVD_10_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_11: u32 = 0x010bu32;
pub const regHDP_XDP_D2H_RSVD_11_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_12: u32 = 0x010cu32;
pub const regHDP_XDP_D2H_RSVD_12_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_13: u32 = 0x010du32;
pub const regHDP_XDP_D2H_RSVD_13_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_14: u32 = 0x010eu32;
pub const regHDP_XDP_D2H_RSVD_14_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_15: u32 = 0x010fu32;
pub const regHDP_XDP_D2H_RSVD_15_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_16: u32 = 0x0110u32;
pub const regHDP_XDP_D2H_RSVD_16_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_17: u32 = 0x0111u32;
pub const regHDP_XDP_D2H_RSVD_17_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_18: u32 = 0x0112u32;
pub const regHDP_XDP_D2H_RSVD_18_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_19: u32 = 0x0113u32;
pub const regHDP_XDP_D2H_RSVD_19_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_20: u32 = 0x0114u32;
pub const regHDP_XDP_D2H_RSVD_20_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_21: u32 = 0x0115u32;
pub const regHDP_XDP_D2H_RSVD_21_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_22: u32 = 0x0116u32;
pub const regHDP_XDP_D2H_RSVD_22_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_23: u32 = 0x0117u32;
pub const regHDP_XDP_D2H_RSVD_23_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_24: u32 = 0x0118u32;
pub const regHDP_XDP_D2H_RSVD_24_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_25: u32 = 0x0119u32;
pub const regHDP_XDP_D2H_RSVD_25_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_26: u32 = 0x011au32;
pub const regHDP_XDP_D2H_RSVD_26_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_27: u32 = 0x011bu32;
pub const regHDP_XDP_D2H_RSVD_27_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_28: u32 = 0x011cu32;
pub const regHDP_XDP_D2H_RSVD_28_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_29: u32 = 0x011du32;
pub const regHDP_XDP_D2H_RSVD_29_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_30: u32 = 0x011eu32;
pub const regHDP_XDP_D2H_RSVD_30_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_31: u32 = 0x011fu32;
pub const regHDP_XDP_D2H_RSVD_31_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_32: u32 = 0x0120u32;
pub const regHDP_XDP_D2H_RSVD_32_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_33: u32 = 0x0121u32;
pub const regHDP_XDP_D2H_RSVD_33_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_D2H_RSVD_34: u32 = 0x0122u32;
pub const regHDP_XDP_D2H_RSVD_34_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_DIRECT2HDP_LAST: u32 = 0x0123u32;
pub const regHDP_XDP_DIRECT2HDP_LAST_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_BAR_CFG: u32 = 0x0124u32;
pub const regHDP_XDP_P2P_BAR_CFG_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_MBX_OFFSET: u32 = 0x0125u32;
pub const regHDP_XDP_P2P_MBX_OFFSET_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_MBX_ADDR0: u32 = 0x0126u32;
pub const regHDP_XDP_P2P_MBX_ADDR0_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_MBX_ADDR1: u32 = 0x0127u32;
pub const regHDP_XDP_P2P_MBX_ADDR1_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_MBX_ADDR2: u32 = 0x0128u32;
pub const regHDP_XDP_P2P_MBX_ADDR2_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_MBX_ADDR3: u32 = 0x0129u32;
pub const regHDP_XDP_P2P_MBX_ADDR3_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_MBX_ADDR4: u32 = 0x012au32;
pub const regHDP_XDP_P2P_MBX_ADDR4_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_MBX_ADDR5: u32 = 0x012bu32;
pub const regHDP_XDP_P2P_MBX_ADDR5_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_MBX_ADDR6: u32 = 0x012cu32;
pub const regHDP_XDP_P2P_MBX_ADDR6_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_HDP_MBX_MC_CFG: u32 = 0x012du32;
pub const regHDP_XDP_HDP_MBX_MC_CFG_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_HDP_MC_CFG: u32 = 0x012eu32;
pub const regHDP_XDP_HDP_MC_CFG_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_HST_CFG: u32 = 0x012fu32;
pub const regHDP_XDP_HST_CFG_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_HDP_IPH_CFG: u32 = 0x0131u32;
pub const regHDP_XDP_HDP_IPH_CFG_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_BAR0: u32 = 0x0134u32;
pub const regHDP_XDP_P2P_BAR0_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_BAR1: u32 = 0x0135u32;
pub const regHDP_XDP_P2P_BAR1_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_BAR2: u32 = 0x0136u32;
pub const regHDP_XDP_P2P_BAR2_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_BAR3: u32 = 0x0137u32;
pub const regHDP_XDP_P2P_BAR3_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_BAR4: u32 = 0x0138u32;
pub const regHDP_XDP_P2P_BAR4_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_BAR5: u32 = 0x0139u32;
pub const regHDP_XDP_P2P_BAR5_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_BAR6: u32 = 0x013au32;
pub const regHDP_XDP_P2P_BAR6_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_P2P_BAR7: u32 = 0x013bu32;
pub const regHDP_XDP_P2P_BAR7_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_FLUSH_ARMED_STS: u32 = 0x013cu32;
pub const regHDP_XDP_FLUSH_ARMED_STS_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_FLUSH_CNTR0_STS: u32 = 0x013du32;
pub const regHDP_XDP_FLUSH_CNTR0_STS_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_BUSY_STS: u32 = 0x013eu32;
pub const regHDP_XDP_BUSY_STS_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_STICKY: u32 = 0x013fu32;
pub const regHDP_XDP_STICKY_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_CHKN: u32 = 0x0140u32;
pub const regHDP_XDP_CHKN_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_BARS_ADDR_39_36: u32 = 0x0144u32;
pub const regHDP_XDP_BARS_ADDR_39_36_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_MC_VM_FB_LOCATION_BASE: u32 = 0x0145u32;
pub const regHDP_XDP_MC_VM_FB_LOCATION_BASE_BASE_IDX: u32 = 0u32;
pub const regHDP_XDP_MMHUB_ERROR: u32 = 0x014au32;
pub const regHDP_XDP_MMHUB_ERROR_BASE_IDX: u32 = 0u32;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
