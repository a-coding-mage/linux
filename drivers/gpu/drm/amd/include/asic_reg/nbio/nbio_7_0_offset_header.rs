Warning: truncated output (original token count: 123946)
Total output lines: 4642

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



// addressBlock: nbio_iohub_nb_nbcfg_nb_cfgdec
// base address: 0x0
pub const cfgNB_NBCFG0_NB_VENDOR_ID: u32 = 0x0000;
pub const cfgNB_NBCFG0_NB_DEVICE_ID: u32 = 0x0002;
pub const cfgNB_NBCFG0_NB_COMMAND: u32 = 0x0004;
pub const cfgNB_NBCFG0_NB_STATUS: u32 = 0x0006;
pub const cfgNB_NBCFG0_NB_REVISION_ID: u32 = 0x0008;
pub const cfgNB_NBCFG0_NB_REGPROG_INF: u32 = 0x0009;
pub const cfgNB_NBCFG0_NB_SUB_CLASS: u32 = 0x000a;
pub const cfgNB_NBCFG0_NB_BASE_CODE: u32 = 0x000b;
pub const cfgNB_NBCFG0_NB_CACHE_LINE: u32 = 0x000c;
pub const cfgNB_NBCFG0_NB_LATENCY: u32 = 0x000d;
pub const cfgNB_NBCFG0_NB_HEADER: u32 = 0x000e;
pub const cfgNB_NBCFG0_NB_ADAPTER_ID: u32 = 0x002c;
pub const cfgNB_NBCFG0_NB_CAPABILITIES_PTR: u32 = 0x0034;
pub const cfgNB_NBCFG0_NB_HEADER_W: u32 = 0x0048;
pub const cfgNB_NBCFG0_NB_PCI_CTRL: u32 = 0x004c;
pub const cfgNB_NBCFG0_NB_ADAPTER_ID_W: u32 = 0x0050;
pub const cfgNB_NBCFG0_NB_SMN_INDEX_EXTENSION_0: u32 = 0x005c;
pub const cfgNB_NBCFG0_NB_SMN_INDEX_0: u32 = 0x0060;
pub const cfgNB_NBCFG0_NB_SMN_DATA_0: u32 = 0x0064;
pub const cfgNB_NBCFG0_NBCFG_SCRATCH_0: u32 = 0x0068;
pub const cfgNB_NBCFG0_NBCFG_SCRATCH_1: u32 = 0x006c;
pub const cfgNB_NBCFG0_NBCFG_SCRATCH_2: u32 = 0x0070;
pub const cfgNB_NBCFG0_NBCFG_SCRATCH_3: u32 = 0x0074;
pub const cfgNB_NBCFG0_NBCFG_SCRATCH_4: u32 = 0x0078;
pub const cfgNB_NBCFG0_NB_PCI_ARB: u32 = 0x0084;
pub const cfgNB_NBCFG0_NB_DRAM_SLOT1_BASE: u32 = 0x0088;
pub const cfgNB_NBCFG0_NB_TOP_OF_DRAM_SLOT1: u32 = 0x0090;
pub const cfgNB_NBCFG0_NB_SMN_INDEX_EXTENSION_1: u32 = 0x009c;
pub const cfgNB_NBCFG0_NB_SMN_INDEX_1: u32 = 0x00a0;
pub const cfgNB_NBCFG0_NB_SMN_DATA_1: u32 = 0x00a4;
pub const cfgNB_NBCFG0_NB_INDEX_DATA_MUTEX0: u32 = 0x00a8;
pub const cfgNB_NBCFG0_NB_INDEX_DATA_MUTEX1: u32 = 0x00ac;
pub const cfgNB_NBCFG0_NB_SMN_INDEX_EXTENSION_2: u32 = 0x00b4;
pub const cfgNB_NBCFG0_NB_SMN_INDEX_2: u32 = 0x00b8;
pub const cfgNB_NBCFG0_NB_SMN_DATA_2: u32 = 0x00bc;
pub const cfgNB_NBCFG0_NB_SMN_INDEX_EXTENSION_3: u32 = 0x00c0;
pub const cfgNB_NBCFG0_NB_SMN_INDEX_3: u32 = 0x00c4;
pub const cfgNB_NBCFG0_NB_SMN_DATA_3: u32 = 0x00c8;
pub const cfgNB_NBCFG0_NB_SMN_INDEX_EXTENSION_4: u32 = 0x00cc;
pub const cfgNB_NBCFG0_NB_SMN_INDEX_4: u32 = 0x00d0;
pub const cfgNB_NBCFG0_NB_SMN_DATA_4: u32 = 0x00d4;
pub const cfgNB_NBCFG0_NB_SMN_INDEX_EXTENSION_5: u32 = 0x00dc;
pub const cfgNB_NBCFG0_NB_SMN_INDEX_5: u32 = 0x00e0;
pub const cfgNB_NBCFG0_NB_SMN_DATA_5: u32 = 0x00e4;
pub const cfgNB_NBCFG0_NB_PERF_CNT_CTRL: u32 = 0x00f4;
pub const cfgNB_NBCFG0_NB_SMN_INDEX_6: u32 = 0x00f8;
pub const cfgNB_NBCFG0_NB_SMN_DATA_6: u32 = 0x00fc;


// addressBlock: nbio_iohub_iommu_l2_iommul2cfg
// base address: 0x0
pub const cfgIOMMU_L2_0_IOMMU_VENDOR_ID: u32 = 0x0000;
pub const cfgIOMMU_L2_0_IOMMU_DEVICE_ID: u32 = 0x0002;
pub const cfgIOMMU_L2_0_IOMMU_COMMAND: u32 = 0x0004;
pub const cfgIOMMU_L2_0_IOMMU_STATUS: u32 = 0x0006;
pub const cfgIOMMU_L2_0_IOMMU_REVISION_ID: u32 = 0x0008;
pub const cfgIOMMU_L2_0_IOMMU_REGPROG_INF: u32 = 0x0009;
pub const cfgIOMMU_L2_0_IOMMU_SUB_CLASS: u32 = 0x000a;
pub const cfgIOMMU_L2_0_IOMMU_BASE_CODE: u32 = 0x000b;
pub const cfgIOMMU_L2_0_IOMMU_CACHE_LINE: u32 = 0x000c;
pub const cfgIOMMU_L2_0_IOMMU_LATENCY: u32 = 0x000d;
pub const cfgIOMMU_L2_0_IOMMU_HEADER: u32 = 0x000e;
pub const cfgIOMMU_L2_0_IOMMU_BIST: u32 = 0x000f;
pub const cfgIOMMU_L2_0_IOMMU_ADAPTER_ID: u32 = 0x002c;
pub const cfgIOMMU_L2_0_IOMMU_CAPABILITIES_PTR: u32 = 0x0034;
pub const cfgIOMMU_L2_0_IOMMU_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgIOMMU_L2_0_IOMMU_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgIOMMU_L2_0_IOMMU_CAP_HEADER: u32 = 0x0040;
pub const cfgIOMMU_L2_0_IOMMU_CAP_BASE_LO: u32 = 0x0044;
pub const cfgIOMMU_L2_0_IOMMU_CAP_BASE_HI: u32 = 0x0048;
pub const cfgIOMMU_L2_0_IOMMU_CAP_RANGE: u32 = 0x004c;
pub const cfgIOMMU_L2_0_IOMMU_CAP_MISC: u32 = 0x0050;
pub const cfgIOMMU_L2_0_IOMMU_CAP_MISC_1: u32 = 0x0054;
pub const cfgIOMMU_L2_0_IOMMU_MSI_CAP: u32 = 0x0064;
pub const cfgIOMMU_L2_0_IOMMU_MSI_ADDR_LO: u32 = 0x0068;
pub const cfgIOMMU_L2_0_IOMMU_MSI_ADDR_HI: u32 = 0x006c;
pub const cfgIOMMU_L2_0_IOMMU_MSI_DATA: u32 = 0x0070;
pub const cfgIOMMU_L2_0_IOMMU_MSI_MAPPING_CAP: u32 = 0x0074;
pub const cfgIOMMU_L2_0_IOMMU_ADAPTER_ID_W: u32 = 0x0078;
pub const cfgIOMMU_L2_0_IOMMU_CONTROL_W: u32 = 0x007c;
pub const cfgIOMMU_L2_0_IOMMU_MMIO_CONTROL0_W: u32 = 0x0080;
pub const cfgIOMMU_L2_0_IOMMU_MMIO_CONTROL1_W: u32 = 0x0084;
pub const cfgIOMMU_L2_0_IOMMU_RANGE_W: u32 = 0x0088;
pub const cfgIOMMU_L2_0_IOMMU_DSFX_CONTROL: u32 = 0x008c;
pub const cfgIOMMU_L2_0_IOMMU_DSSX_DUMMY_0: u32 = 0x0090;
pub const cfgIOMMU_L2_0_IOMMU_DSCX_DUMMY_0: u32 = 0x0094;
pub const cfgIOMMU_L2_0_L2B_POISON_DVM_CNTRL: u32 = 0x0098;
pub const cfgIOMMU_L2_0_L2_IOHC_DmaReq_Stall_Control: u32 = 0x009c;
pub const cfgIOMMU_L2_0_IOHC_L2_HostRsp_Stall_Control: u32 = 0x00a0;
pub const cfgIOMMU_L2_0_SMMU_MMIO_IDR0_W: u32 = 0x00a4;
pub const cfgIOMMU_L2_0_SMMU_MMIO_IDR1_W: u32 = 0x00a8;
pub const cfgIOMMU_L2_0_SMMU_MMIO_IDR2_W: u32 = 0x00ac;
pub const cfgIOMMU_L2_0_SMMU_MMIO_IDR3_W: u32 = 0x00b0;
pub const cfgIOMMU_L2_0_SMMU_MMIO_IDR5_W: u32 = 0x00b8;
pub const cfgIOMMU_L2_0_SMMU_MMIO_IIDR_W: u32 = 0x00bc;
pub const cfgIOMMU_L2_0_SMMU_AIDR_W: u32 = 0x00c0;


// addressBlock: nbio_nbif0_bif_cfg_dev0_rc_bifcfgdecp
// base address: 0x0
pub const cfgBIF_CFG_DEV0_RC0_VENDOR_ID: u32 = 0x0000;
pub const cfgBIF_CFG_DEV0_RC0_DEVICE_ID: u32 = 0x0002;
pub const cfgBIF_CFG_DEV0_RC0_COMMAND: u32 = 0x0004;
pub const cfgBIF_CFG_DEV0_RC0_STATUS: u32 = 0x0006;
pub const cfgBIF_CFG_DEV0_RC0_REVISION_ID: u32 = 0x0008;
pub const cfgBIF_CFG_DEV0_RC0_PROG_INTERFACE: u32 = 0x0009;
pub const cfgBIF_CFG_DEV0_RC0_SUB_CLASS: u32 = 0x000a;
pub const cfgBIF_CFG_DEV0_RC0_BASE_CLASS: u32 = 0x000b;
pub const cfgBIF_CFG_DEV0_RC0_CACHE_LINE: u32 = 0x000c;
pub const cfgBIF_CFG_DEV0_RC0_LATENCY: u32 = 0x000d;
pub const cfgBIF_CFG_DEV0_RC0_HEADER: u32 = 0x000e;
pub const cfgBIF_CFG_DEV0_RC0_BIST: u32 = 0x000f;
pub const cfgBIF_CFG_DEV0_RC0_BASE_ADDR_1: u32 = 0x0010;
pub const cfgBIF_CFG_DEV0_RC0_SUB_BUS_NUMBER_LATENCY: u32 = 0x0018;
pub const cfgBIF_CFG_DEV0_RC0_IO_BASE_LIMIT: u32 = 0x001c;
pub const cfgBIF_CFG_DEV0_RC0_SECONDARY_STATUS: u32 = 0x001e;
pub const cfgBIF_CFG_DEV0_RC0_MEM_BASE_LIMIT: u32 = 0x0020;
pub const cfgBIF_CFG_DEV0_RC0_PREF_BASE_LIMIT: u32 = 0x0024;
pub const cfgBIF_CFG_DEV0_RC0_PREF_BASE_UPPER: u32 = 0x0028;
pub const cfgBIF_CFG_DEV0_RC0_PREF_LIMIT_UPPER: u32 = 0x002c;
pub const cfgBIF_CFG_DEV0_RC0_IO_BASE_LIMIT_HI: u32 = 0x0030;
pub const cfgBIF_CFG_DEV0_RC0_CAP_PTR: u32 = 0x0034;
pub const cfgBIF_CFG_DEV0_RC0_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgBIF_CFG_DEV0_RC0_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgBIF_CFG_DEV0_RC0_IRQ_BRIDGE_CNTL: u32 = 0x003e;
pub const cfgBIF_CFG_DEV0_RC0_EXT_BRIDGE_CNTL: u32 = 0x0040;
pub const cfgBIF_CFG_DEV0_RC0_PMI_CAP_LIST: u32 = 0x0050;
pub const cfgBIF_CFG_DEV0_RC0_PMI_CAP: u32 = 0x0052;
pub const cfgBIF_CFG_DEV0_RC0_PMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_CAP_LIST: u32 = 0x0058;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_CAP: u32 = 0x005a;
pub const cfgBIF_CFG_DEV0_RC0_DEVICE_CAP: u32 = 0x005c;
pub const cfgBIF_CFG_DEV0_RC0_DEVICE_CNTL: u32 = 0x0060;
pub const cfgBIF_CFG_DEV0_RC0_DEVICE_STATUS: u32 = 0x0062;
pub const cfgBIF_CFG_DEV0_RC0_LINK_CAP: u32 = 0x0064;
pub const cfgBIF_CFG_DEV0_RC0_LINK_CNTL: u32 = 0x0068;
pub const cfgBIF_CFG_DEV0_RC0_LINK_STATUS: u32 = 0x006a;
pub const cfgBIF_CFG_DEV0_RC0_SLOT_CAP: u32 = 0x006c;
pub const cfgBIF_CFG_DEV0_RC0_SLOT_CNTL: u32 = 0x0070;
pub const cfgBIF_CFG_DEV0_RC0_SLOT_STATUS: u32 = 0x0072;
pub const cfgBIF_CFG_DEV0_RC0_ROOT_CNTL: u32 = 0x0074;
pub const cfgBIF_CFG_DEV0_RC0_ROOT_CAP: u32 = 0x0076;
pub const cfgBIF_CFG_DEV0_RC0_ROOT_STATUS: u32 = 0x0078;
pub const cfgBIF_CFG_DEV0_RC0_DEVICE_CAP2: u32 = 0x007c;
pub const cfgBIF_CFG_DEV0_RC0_DEVICE_CNTL2: u32 = 0x0080;
pub const cfgBIF_CFG_DEV0_RC0_DEVICE_STATUS2: u32 = 0x0082;
pub const cfgBIF_CFG_DEV0_RC0_LINK_CAP2: u32 = 0x0084;
pub const cfgBIF_CFG_DEV0_RC0_LINK_CNTL2: u32 = 0x0088;
pub const cfgBIF_CFG_DEV0_RC0_LINK_STATUS2: u32 = 0x008a;
pub const cfgBIF_CFG_DEV0_RC0_SLOT_CAP2: u32 = 0x008c;
pub const cfgBIF_CFG_DEV0_RC0_SLOT_CNTL2: u32 = 0x0090;
pub const cfgBIF_CFG_DEV0_RC0_SLOT_STATUS2: u32 = 0x0092;
pub const cfgBIF_CFG_DEV0_RC0_MSI_CAP_LIST: u32 = 0x00a0;
pub const cfgBIF_CFG_DEV0_RC0_MSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgBIF_CFG_DEV0_RC0_MSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgBIF_CFG_DEV0_RC0_MSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_RC0_MSI_MSG_DATA: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_RC0_MSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_RC0_SSID_CAP_LIST: u32 = 0x00c0;
pub const cfgBIF_CFG_DEV0_RC0_SSID_CAP: u32 = 0x00c4;
pub const cfgBIF_CFG_DEV0_RC0_MSI_MAP_CAP_LIST: u32 = 0x00c8;
pub const cfgBIF_CFG_DEV0_RC0_MSI_MAP_CAP: u32 = 0x00ca;
pub const cfgBIF_CFG_DEV0_RC0_MSI_MAP_ADDR_LO: u32 = 0x00cc;
pub const cfgBIF_CFG_DEV0_RC0_MSI_MAP_ADDR_HI: u32 = 0x00d0;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_VC_ENH_CAP_LIST: u32 = 0x0110;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_PORT_VC_CAP_REG1: u32 = 0x0114;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_PORT_VC_CAP_REG2: u32 = 0x0118;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_PORT_VC_CNTL: u32 = 0x011c;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_PORT_VC_STATUS: u32 = 0x011e;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_VC0_RESOURCE_CAP: u32 = 0x0120;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_VC0_RESOURCE_CNTL: u32 = 0x0124;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_VC0_RESOURCE_STATUS: u32 = 0x012a;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_VC1_RESOURCE_CAP: u32 = 0x012c;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_VC1_RESOURCE_CNTL: u32 = 0x0130;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_VC1_RESOURCE_STATUS: u32 = 0x0136;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_DEV_SERIAL_NUM_ENH_CAP_LIST: u32 = 0x0140;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_DEV_SERIAL_NUM_DW1: u32 = 0x0144;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_DEV_SERIAL_NUM_DW2: u32 = 0x0148;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_ROOT_ERR_CMD: u32 = 0x017c;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_ROOT_ERR_STATUS: u32 = 0x0180;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_ERR_SRC_ID: u32 = 0x0184;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_SECONDARY_ENH_CAP_LIST: u32 = 0x0270;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LINK_CNTL3: u32 = 0x0274;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_ERROR_STATUS: u32 = 0x0278;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_0_EQUALIZATION_CNTL: u32 = 0x027c;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_1_EQUALIZATION_CNTL: u32 = 0x027e;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_2_EQUALIZATION_CNTL: u32 = 0x0280;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_3_EQUALIZATION_CNTL: u32 = 0x0282;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_4_EQUALIZATION_CNTL: u32 = 0x0284;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_5_EQUALIZATION_CNTL: u32 = 0x0286;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_6_EQUALIZATION_CNTL: u32 = 0x0288;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_7_EQUALIZATION_CNTL: u32 = 0x028a;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_8_EQUALIZATION_CNTL: u32 = 0x028c;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_9_EQUALIZATION_CNTL: u32 = 0x028e;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_10_EQUALIZATION_CNTL: u32 = 0x0290;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_11_EQUALIZATION_CNTL: u32 = 0x0292;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_12_EQUALIZATION_CNTL: u32 = 0x0294;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_13_EQUALIZATION_CNTL: u32 = 0x0296;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_14_EQUALIZATION_CNTL: u32 = 0x0298;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_LANE_15_EQUALIZATION_CNTL: u32 = 0x029a;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgBIF_CFG_DEV0_RC0_PCIE_ACS_CNTL: u32 = 0x02a6;


// addressBlock: nbio_nbif0_bif_cfg_dev1_rc_bifcfgdecp
// base address: 0x0
pub const cfgBIF_CFG_DEV1_RC0_VENDOR_ID: u32 = 0x0000;
pub const cfgBIF_CFG_DEV1_RC0_DEVICE_ID: u32 = 0x0002;
pub const cfgBIF_CFG_DEV1_RC0_COMMAND: u32 = 0x0004;
pub const cfgBIF_CFG_DEV1_RC0_STATUS: u32 = 0x0006;
pub const cfgBIF_CFG_DEV1_RC0_REVISION_ID: u32 = 0x0008;
pub const cfgBIF_CFG_DEV1_RC0_PROG_INTERFACE: u32 = 0x0009;
pub const cfgBIF_CFG_DEV1_RC0_SUB_CLASS: u32 = 0x000a;
pub const cfgBIF_CFG_DEV1_RC0_BASE_CLASS: u32 = 0x000b;
pub const cfgBIF_CFG_DEV1_RC0_CACHE_LINE: u32 = 0x000c;
pub const cfgBIF_CFG_DEV1_RC0_LATENCY: u32 = 0x000d;
pub const cfgBIF_CFG_DEV1_RC0_HEADER: u32 = 0x000e;
pub const cfgBIF_CFG_DEV1_RC0_BIST: u32 = 0x000f;
pub const cfgBIF_CFG_DEV1_RC0_BASE_ADDR_1: u32 = 0x0010;
pub const cfgBIF_CFG_DEV1_RC0_SUB_BUS_NUMBER_LATENCY: u32 = 0x0018;
pub const cfgBIF_CFG_DEV1_RC0_IO_BASE_LIMIT: u32 = 0x001c;
pub const cfgBIF_CFG_DEV1_RC0_SECONDARY_STATUS: u32 = 0x001e;
pub const cfgBIF_CFG_DEV1_RC0_MEM_BASE_LIMIT: u32 = 0x0020;
pub const cfgBIF_CFG_DEV1_RC0_PREF_BASE_LIMIT: u32 = 0x0024;
pub const cfgBIF_CFG_DEV1_RC0_PREF_BASE_UPPER: u32 = 0x0028;
pub const cfgBIF_CFG_DEV1_RC0_PREF_LIMIT_UPPER: u32 = 0x002c;
pub const cfgBIF_CFG_DEV1_RC0_IO_BASE_LIMIT_HI: u32 = 0x0030;
pub const cfgBIF_CFG_DEV1_RC0_CAP_PTR: u32 = 0x0034;
pub const cfgBIF_CFG_DEV1_RC0_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgBIF_CFG_DEV1_RC0_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgBIF_CFG_DEV1_RC0_IRQ_BRIDGE_CNTL: u32 = 0x003e;
pub const cfgBIF_CFG_DEV1_RC0_EXT_BRIDGE_CNTL: u32 = 0x0040;
pub const cfgBIF_CFG_DEV1_RC0_PMI_CAP_LIST: u32 = 0x0050;
pub const cfgBIF_CFG_DEV1_RC0_PMI_CAP: u32 = 0x0052;
pub const cfgBIF_CFG_DEV1_RC0_PMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_CAP_LIST: u32 = 0x0058;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_CAP: u32 = 0x005a;
pub const cfgBIF_CFG_DEV1_RC0_DEVICE_CAP: u32 = 0x005c;
pub const cfgBIF_CFG_DEV1_RC0_DEVICE_CNTL: u32 = 0x0060;
pub const cfgBIF_CFG_DEV1_RC0_DEVICE_STATUS: u32 = 0x0062;
pub const cfgBIF_CFG_DEV1_RC0_LINK_CAP: u32 = 0x0064;
pub const cfgBIF_CFG_DEV1_RC0_LINK_CNTL: u32 = 0x0068;
pub const cfgBIF_CFG_DEV1_RC0_LINK_STATUS: u32 = 0x006a;
pub const cfgBIF_CFG_DEV1_RC0_SLOT_CAP: u32 = 0x006c;
pub const cfgBIF_CFG_DEV1_RC0_SLOT_CNTL: u32 = 0x0070;
pub const cfgBIF_CFG_DEV1_RC0_SLOT_STATUS: u32 = 0x0072;
pub const cfgBIF_CFG_DEV1_RC0_ROOT_CNTL: u32 = 0x0074;
pub const cfgBIF_CFG_DEV1_RC0_ROOT_CAP: u32 = 0x0076;
pub const cfgBIF_CFG_DEV1_RC0_ROOT_STATUS: u32 = 0x0078;
pub const cfgBIF_CFG_DEV1_RC0_DEVICE_CAP2: u32 = 0x007c;
pub const cfgBIF_CFG_DEV1_RC0_DEVICE_CNTL2: u32 = 0x0080;
pub const cfgBIF_CFG_DEV1_RC0_DEVICE_STATUS2: u32 = 0x0082;
pub const cfgBIF_CFG_DEV1_RC0_LINK_CAP2: u32 = 0x0084;
pub const cfgBIF_CFG_DEV1_RC0_LINK_CNTL2: u32 = 0x0088;
pub const cfgBIF_CFG_DEV1_RC0_LINK_STATUS2: u32 = 0x008a;
pub const cfgBIF_CFG_DEV1_RC0_SLOT_CAP2: u32 = 0x008c;
pub const cfgBIF_CFG_DEV1_RC0_SLOT_CNTL2: u32 = 0x0090;
pub const cfgBIF_CFG_DEV1_RC0_SLOT_STATUS2: u32 = 0x0092;
pub const cfgBIF_CFG_DEV1_RC0_MSI_CAP_LIST: u32 = 0x00a0;
pub const cfgBIF_CFG_DEV1_RC0_MSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgBIF_CFG_DEV1_RC0_MSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgBIF_CFG_DEV1_RC0_MSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV1_RC0_MSI_MSG_DATA: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV1_RC0_MSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV1_RC0_SSID_CAP_LIST: u32 = 0x00c0;
pub const cfgBIF_CFG_DEV1_RC0_SSID_CAP: u32 = 0x00c4;
pub const cfgBIF_CFG_DEV1_RC0_MSI_MAP_CAP_LIST: u32 = 0x00c8;
pub const cfgBIF_CFG_DEV1_RC0_MSI_MAP_CAP: u32 = 0x00ca;
pub const cfgBIF_CFG_DEV1_RC0_MSI_MAP_ADDR_LO: u32 = 0x00cc;
pub const cfgBIF_CFG_DEV1_RC0_MSI_MAP_ADDR_HI: u32 = 0x00d0;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_VC_ENH_CAP_LIST: u32 = 0x0110;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_PORT_VC_CAP_REG1: u32 = 0x0114;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_PORT_VC_CAP_REG2: u32 = 0x0118;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_PORT_VC_CNTL: u32 = 0x011c;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_PORT_VC_STATUS: u32 = 0x011e;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_VC0_RESOURCE_CAP: u32 = 0x0120;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_VC0_RESOURCE_CNTL: u32 = 0x0124;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_VC0_RESOURCE_STATUS: u32 = 0x012a;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_VC1_RESOURCE_CAP: u32 = 0x012c;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_VC1_RESOURCE_CNTL: u32 = 0x0130;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_VC1_RESOURCE_STATUS: u32 = 0x0136;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_DEV_SERIAL_NUM_ENH_CAP_LIST: u32 = 0x0140;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_DEV_SERIAL_NUM_DW1: u32 = 0x0144;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_DEV_SERIAL_NUM_DW2: u32 = 0x0148;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_ROOT_ERR_CMD: u32 = 0x017c;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_ROOT_ERR_STATUS: u32 = 0x0180;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_ERR_SRC_ID: u32 = 0x0184;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_SECONDARY_ENH_CAP_LIST: u32 = 0x0270;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LINK_CNTL3: u32 = 0x0274;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_ERROR_STATUS: u32 = 0x0278;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_0_EQUALIZATION_CNTL: u32 = 0x027c;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_1_EQUALIZATION_CNTL: u32 = 0x027e;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_2_EQUALIZATION_CNTL: u32 = 0x0280;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_3_EQUALIZATION_CNTL: u32 = 0x0282;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_4_EQUALIZATION_CNTL: u32 = 0x0284;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_5_EQUALIZATION_CNTL: u32 = 0x0286;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_6_EQUALIZATION_CNTL: u32 = 0x0288;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_7_EQUALIZATION_CNTL: u32 = 0x028a;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_8_EQUALIZATION_CNTL: u32 = 0x028c;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_9_EQUALIZATION_CNTL: u32 = 0x028e;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_10_EQUALIZATION_CNTL: u32 = 0x0290;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_11_EQUALIZATION_CNTL: u32 = 0x0292;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_12_EQUALIZATION_CNTL: u32 = 0x0294;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_13_EQUALIZATION_CNTL: u32 = 0x0296;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_14_EQUALIZATION_CNTL: u32 = 0x0298;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_LANE_15_EQUALIZATION_CNTL: u32 = 0x029a;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgBIF_CFG_DEV1_RC0_PCIE_ACS_CNTL: u32 = 0x02a6;


// addressBlock: nbio_iohub_nb_pciedummy0_pciedummy_cfgdec
// base address: 0x0
pub const cfgNB_PCIEDUMMY0_0_DEVICE_VENDOR_ID: u32 = 0x0000;
pub const cfgNB_PCIEDUMMY0_0_STATUS_COMMAND: u32 = 0x0004;
pub const cfgNB_PCIEDUMMY0_0_CLASS_CODE_REVID: u32 = 0x0008;
pub const cfgNB_PCIEDUMMY0_0_HEADER_TYPE: u32 = 0x000c;
pub const cfgNB_PCIEDUMMY0_0_HEADER_TYPE_W: u32 = 0x0040;


// addressBlock: nbio_iohub_nb_pciedummy1_pciedummy_cfgdec
// base address: 0x0
pub const cfgNB_PCIEDUMMY1_0_DEVICE_VENDOR_ID: u32 = 0x0000;
pub const cfgNB_PCIEDUMMY1_0_STATUS_COMMAND: u32 = 0x0004;
pub const cfgNB_PCIEDUMMY1_0_CLASS_CODE_REVID: u32 = 0x0008;
pub const cfgNB_PCIEDUMMY1_0_HEADER_TYPE: u32 = 0x000c;
pub const cfgNB_PCIEDUMMY1_0_HEADER_TYPE_W: u32 = 0x0040;


// addressBlock: nbio_nbif0_bif_cfg_dev0_epf0_bifcfgdecp
// base address: 0x0
pub const cfgVENDOR_ID: u32 = 0x0000;
pub const cfgDEVICE_ID: u32 = 0x0002;
pub const cfgCOMMAND: u32 = 0x0004;
pub const cfgSTATUS: u32 = 0x0006;
pub const cfgREVISION_ID: u32 = 0x0008;
pub const cfgPROG_INTERFACE: u32 = 0x0009;
pub const cfgSUB_CLASS: u32 = 0x000a;
pub const cfgBASE_CLASS: u32 = 0x000b;
pub const cfgCACHE_LINE: u32 = 0x000c;
pub const cfgLATENCY: u32 = 0x000d;
pub const cfgHEADER: u32 = 0x000e;
pub const cfgBIST: u32 = 0x000f;
pub const cfgBASE_ADDR_1: u32 = 0x0010;
pub const cfgBASE_ADDR_2: u32 = 0x0014;
pub const cfgBASE_ADDR_3: u32 = 0x0018;
pub const cfgBASE_ADDR_4: u32 = 0x001c;
pub const cfgBASE_ADDR_5: u32 = 0x0020;
pub const cfgBASE_ADDR_6: u32 = 0x0024;
pub const cfgADAPTER_ID: u32 = 0x002c;
pub const cfgROM_BASE_ADDR: u32 = 0x0030;
pub const cfgCAP_PTR: u32 = 0x0034;
pub const cfgINTERRUPT_LINE: u32 = 0x003c;
pub const cfgINTERRUPT_PIN: u32 = 0x003d;
pub const cfgMIN_GRANT: u32 = 0x003e;
pub const cfgMAX_LATENCY: u32 = 0x003f;
pub const cfgVENDOR_CAP_LIST: u32 = 0x0048;
pub const cfgADAPTER_ID_W: u32 = 0x004c;
pub const cfgPMI_CAP_LIST: u32 = 0x0050;
pub const cfgPMI_CAP: u32 = 0x0052;
pub const cfgPMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgPCIE_CAP_LIST: u32 = 0x0064;
pub const cfgPCIE_CAP: u32 = 0x0066;
pub const cfgDEVICE_CAP: u32 = 0x0068;
pub const cfgDEVICE_CNTL: u32 = 0x006c;
pub const cfgDEVICE_STATUS: u32 = 0x006e;
pub const cfgLINK_CAP: u32 = 0x0070;
pub const cfgLINK_CNTL: u32 = 0x0074;
pub const cfgLINK_STATUS: u32 = 0x0076;
pub const cfgDEVICE_CAP2: u32 = 0x0088;
pub const cfgDEVICE_CNTL2: u32 = 0x008c;
pub const cfgDEVICE_STATUS2: u32 = 0x008e;
pub const cfgLINK_CAP2: u32 = 0x0090;
pub const cfgLINK_CNTL2: u32 = 0x0094;
pub const cfgLINK_STATUS2: u32 = 0x0096;
pub const cfgSLOT_CAP2: u32 = 0x0098;
pub const cfgSLOT_CNTL2: u32 = 0x009c;
pub const cfgSLOT_STATUS2: u32 = 0x009e;
pub const cfgMSI_CAP_LIST: u32 = 0x00a0;
pub const cfgMSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgMSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgMSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgMSI_MSG_DATA: u32 = 0x00a8;
pub const cfgMSI_MASK: u32 = 0x00ac;
pub const cfgMSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgMSI_MASK_64: u32 = 0x00b0;
pub const cfgMSI_PENDING: u32 = 0x00b0;
pub const cfgMSI_PENDING_64: u32 = 0x00b4;
pub const cfgMSIX_CAP_LIST: u32 = 0x00c0;
pub const cfgMSIX_MSG_CNTL: u32 = 0x00c2;
pub const cfgMSIX_TABLE: u32 = 0x00c4;
pub const cfgMSIX_PBA: u32 = 0x00c8;
pub const cfgPCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgPCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgPCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgPCIE_VC_ENH_CAP_LIST: u32 = 0x0110;
pub const cfgPCIE_PORT_VC_CAP_REG1: u32 = 0x0114;
pub const cfgPCIE_PORT_VC_CAP_REG2: u32 = 0x0118;
pub const cfgPCIE_PORT_VC_CNTL: u32 = 0x011c;
pub const cfgPCIE_PORT_VC_STATUS: u32 = 0x011e;
pub const cfgPCIE_VC0_RESOURCE_CAP: u32 = 0x0120;
pub const cfgPCIE_VC0_RESOURCE_CNTL: u32 = 0x0124;
pub const cfgPCIE_VC0_RESOURCE_STATUS: u32 = 0x012a;
pub const cfgPCIE_VC1_RESOURCE_CAP: u32 = 0x012c;
pub const cfgPCIE_VC1_RESOURCE_CNTL: u32 = 0x0130;
pub const cfgPCIE_VC1_RESOURCE_STATUS: u32 = 0x0136;
pub const cfgPCIE_DEV_SERIAL_NUM_ENH_CAP_LIST: u32 = 0x0140;
pub const cfgPCIE_DEV_SERIAL_NUM_DW1: u32 = 0x0144;
pub const cfgPCIE_DEV_SERIAL_NUM_DW2: u32 = 0x0148;
pub const cfgPCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgPCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgPCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgPCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgPCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgPCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgPCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgPCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgPCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgPCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgPCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgPCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgPCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgPCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgPCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgPCIE_BAR_ENH_CAP_LIST: u32 = 0x0200;
pub const cfgPCIE_BAR1_CAP: u32 = 0x0204;
pub const cfgPCIE_BAR1_CNTL: u32 = 0x0208;
pub const cfgPCIE_BAR2_CAP: u32 = 0x020c;
pub const cfgPCIE_BAR2_CNTL: u32 = 0x0210;
pub const cfgPCIE_BAR3_CAP: u32 = 0x0214;
pub const cfgPCIE_BAR3_CNTL: u32 = 0x0218;
pub const cfgPCIE_BAR4_CAP: u32 = 0x021c;
pub const cfgPCIE_BAR4_CNTL: u32 = 0x0220;
pub const cfgPCIE_BAR5_CAP: u32 = 0x0224;
pub const cfgPCIE_BAR5_CNTL: u32 = 0x0228;
pub const cfgPCIE_BAR6_CAP: u32 = 0x022c;
pub const cfgPCIE_BAR6_CNTL: u32 = 0x0230;
pub const cfgPCIE_PWR_BUDGET_ENH_CAP_LIST: u32 = 0x0240;
pub const cfgPCIE_PWR_BUDGET_DATA_SELECT: u32 = 0x0244;
pub const cfgPCIE_PWR_BUDGET_DATA: u32 = 0x0248;
pub const cfgPCIE_PWR_BUDGET_CAP: u32 = 0x024c;
pub const cfgPCIE_DPA_ENH_CAP_LIST: u32 = 0x0250;
pub const cfgPCIE_DPA_CAP: u32 = 0x0254;
pub const cfgPCIE_DPA_LATENCY_INDICATOR: u32 = 0x0258;
pub const cfgPCIE_DPA_STATUS: u32 = 0x025c;
pub const cfgPCIE_DPA_CNTL: u32 = 0x025e;
pub const cfgPCIE_DPA_SUBSTATE_PWR_ALLOC_0: u32 = 0x0260;
pub const cfgPCIE_DPA_SUBSTATE_PWR_ALLOC_1: u32 = 0x0261;
pub const cfgPCIE_DPA_SUBSTATE_PWR_ALLOC_2: u32 = 0x0262;
pub const cfgPCIE_DPA_SUBSTATE_PWR_ALLOC_3: u32 = 0x0263;
pub const cfgPCIE_DPA_SUBSTATE_PWR_ALLOC_4: u32 = 0x0264;
pub const cfgPCIE_DPA_SUBSTATE_PWR_ALLOC_5: u32 = 0x0265;
pub const cfgPCIE_DPA_SUBSTATE_PWR_ALLOC_6: u32 = 0x0266;
pub const cfgPCIE_DPA_SUBSTATE_PWR_ALLOC_7: u32 = 0x0267;
pub const cfgPCIE_SECONDARY_ENH_CAP_LIST: u32 = 0x0270;
pub const cfgPCIE_LINK_CNTL3: u32 = 0x0274;
pub const cfgPCIE_LANE_ERROR_STATUS: u32 = 0x0278;
pub const cfgPCIE_LANE_0_EQUALIZATION_CNTL: u32 = 0x027c;
pub const cfgPCIE_LANE_1_EQUALIZATION_CNTL: u32 = 0x027e;
pub const cfgPCIE_LANE_2_EQUALIZATION_CNTL: u32 = 0x0280;
pub const cfgPCIE_LANE_3_EQUALIZATION_CNTL: u32 = 0x0282;
pub const cfgPCIE_LANE_4_EQUALIZATION_CNTL: u32 = 0x0284;
pub const cfgPCIE_LANE_5_EQUALIZATION_CNTL: u32 = 0x0286;
pub const cfgPCIE_LANE_6_EQUALIZATION_CNTL: u32 = 0x0288;
pub const cfgPCIE_LANE_7_EQUALIZATION_CNTL: u32 = 0x028a;
pub const cfgPCIE_LANE_8_EQUALIZATION_CNTL: u32 = 0x028c;
pub const cfgPCIE_LANE_9_EQUALIZATION_CNTL: u32 = 0x028e;
pub const cfgPCIE_LANE_10_EQUALIZATION_CNTL: u32 = 0x0290;
pub const cfgPCIE_LANE_11_EQUALIZATION_CNTL: u32 = 0x0292;
pub const cfgPCIE_LANE_12_EQUALIZATION_CNTL: u32 = 0x0294;
pub const cfgPCIE_LANE_13_EQUALIZATION_CNTL: u32 = 0x0296;
pub const cfgPCIE_LANE_14_EQUALIZATION_CNTL: u32 = 0x0298;
pub const cfgPCIE_LANE_15_EQUALIZATION_CNTL: u32 = 0x029a;
pub const cfgPCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgPCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgPCIE_ACS_CNTL: u32 = 0x02a6;
pub const cfgPCIE_ATS_ENH_CAP_LIST: u32 = 0x02b0;
pub const cfgPCIE_ATS_CAP: u32 = 0x02b4;
pub const cfgPCIE_ATS_CNTL: u32 = 0x02b6;
pub const cfgPCIE_PAGE_REQ_ENH_CAP_LIST: u32 = 0x02c0;
pub const cfgPCIE_PAGE_REQ_CNTL: u32 = 0x02c4;
pub const cfgPCIE_PAGE_REQ_STATUS: u32 = 0x02c6;
pub const cfgPCIE_OUTSTAND_PAGE_REQ_CAPACITY: u32 = 0x02c8;
pub const cfgPCIE_OUTSTAND_PAGE_REQ_ALLOC: u32 = 0x02cc;
pub const cfgPCIE_PASID_ENH_CAP_LIST: u32 = 0x02d0;
pub const cfgPCIE_PASID_CAP: u32 = 0x02d4;
pub const cfgPCIE_PASID_CNTL: u32 = 0x02d6;
pub const cfgPCIE_TPH_REQR_ENH_CAP_LIST: u32 = 0x02e0;
pub const cfgPCIE_TPH_REQR_CAP: u32 = 0x02e4;
pub const cfgPCIE_TPH_REQR_CNTL: u32 = 0x02e8;
pub const cfgPCIE_MC_ENH_CAP_LIST: u32 = 0x02f0;
pub const cfgPCIE_MC_CAP: u32 = 0x02f4;
pub const cfgPCIE_MC_CNTL: u32 = 0x02f6;
pub const cfgPCIE_MC_ADDR0: u32 = 0x02f8;
pub const cfgPCIE_MC_ADDR1: u32 = 0x02fc;
pub const cfgPCIE_MC_RCV0: u32 = 0x0300;
pub const cfgPCIE_MC_RCV1: u32 = 0x0304;
pub const cfgPCIE_MC_BLOCK_ALL0: u32 = 0x0308;
pub const cfgPCIE_MC_BLOCK_ALL1: u32 = 0x030c;
pub const cfgPCIE_MC_BLOCK_UNTRANSLATED_0: u32 = 0x0310;
pub const cfgPCIE_MC_BLOCK_UNTRANSLATED_1: u32 = 0x0314;
pub const cfgPCIE_LTR_ENH_CAP_LIST: u32 = 0x0320;
pub const cfgPCIE_LTR_CAP: u32 = 0x0324;
pub const cfgPCIE_ARI_ENH_CAP_LIST: u32 = 0x0328;
pub const cfgPCIE_ARI_CAP: u32 = 0x032c;
pub const cfgPCIE_ARI_CNTL: u32 = 0x032e;
pub const cfgPCIE_SRIOV_ENH_CAP_LIST: u32 = 0x0330;
pub const cfgPCIE_SRIOV_CAP: u32 = 0x0334;
pub const cfgPCIE_SRIOV_CONTROL: u32 = 0x0338;
pub const cfgPCIE_SRIOV_STATUS: u32 = 0x033a;
pub const cfgPCIE_SRIOV_INITIAL_VFS: u32 = 0x033c;
pub const cfgPCIE_SRIOV_TOTAL_VFS: u32 = 0x033e;
pub const cfgPCIE_SRIOV_NUM_VFS: u32 = 0x0340;
pub const cfgPCIE_SRIOV_FUNC_DEP_LINK: u32 = 0x0342;
pub const cfgPCIE_SRIOV_FIRST_VF_OFFSET: u32 = 0x0344;
pub const cfgPCIE_SRIOV_VF_STRIDE: u32 = 0x0346;
pub const cfgPCIE_SRIOV_VF_DEVICE_ID: u32 = 0x034a;
pub const cfgPCIE_SRIOV_SUPPORTED_PAGE_SIZE: u32 = 0x034c;
pub const cfgPCIE_SRIOV_SYSTEM_PAGE_SIZE: u32 = 0x0350;
pub const cfgPCIE_SRIOV_VF_BASE_ADDR_0: u32 = 0x0354;
pub const cfgPCIE_SRIOV_VF_BASE_ADDR_1: u32 = 0x0358;
pub const cfgPCIE_SRIOV_VF_BASE_ADDR_2: u32 = 0x035c;
pub const cfgPCIE_SRIOV_VF_BASE_ADDR_3: u32 = 0x0360;
pub const cfgPCIE_SRIOV_VF_BASE_ADDR_4: u32 = 0x0364;
pub const cfgPCIE_SRIOV_VF_BASE_ADDR_5: u32 = 0x0368;
pub const cfgPCIE_SRIOV_VF_MIGRATION_STATE_ARRAY_OFFSET: u32 = 0x036c;
pub const cfgPCIE_VENDOR_SPECIFIC_ENH_CAP_LIST_GPUIOV: u32 = 0x0400;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV: u32 = 0x0404;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_SRIOV_SHADOW: u32 = 0x0408;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_INTR_ENABLE: u32 = 0x040c;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_INTR_STATUS: u32 = 0x0410;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_RESET_CONTROL: u32 = 0x0414;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_HVVM_MBOX_DW0: u32 = 0x0418;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_HVVM_MBOX_DW1: u32 = 0x041c;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_HVVM_MBOX_DW2: u32 = 0x0420;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_CONTEXT: u32 = 0x0424;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_TOTAL_FB: u32 = 0x0428;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_OFFSETS: u32 = 0x042c;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF0_FB: u32 = 0x0430;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF1_FB: u32 = 0x0434;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF2_FB: u32 = 0x0438;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF3_FB: u32 = 0x043c;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF4_FB: u32 = 0x0440;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF5_FB: u32 = 0x0444;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF6_FB: u32 = 0x0448;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF7_FB: u32 = 0x044c;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF8_FB: u32 = 0x0450;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF9_FB: u32 = 0x0454;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF10_FB: u32 = 0x0458;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF11_FB: u32 = 0x045c;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF12_FB: u32 = 0x0460;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF13_FB: u32 = 0x0464;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF14_FB: u32 = 0x0468;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF15_FB: u32 = 0x046c;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW0: u32 = 0x0470;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW1: u32 = 0x0474;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW2: u32 = 0x0478;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW3: u32 = 0x047c;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW4: u32 = 0x0480;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW5: u32 = 0x0484;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW6: u32 = 0x0488;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW7: u32 = 0x048c;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW8: u32 = 0x0490;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW0: u32 = 0x04a0;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW1: u32 = 0x04a4;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW2: u32 = 0x04a8;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW3: u32 = 0x04ac;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW4: u32 = 0x04b0;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW5: u32 = 0x04b4;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW6: u32 = 0x04b8;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW7: u32 = 0x04bc;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW8: u32 = 0x04c0;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW0: u32 = 0x04d0;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW1: u32 = 0x04d4;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW2: u32 = 0x04d8;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW3: u32 = 0x04dc;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW4: u32 = 0x04e0;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW5: u32 = 0x04e4;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW6: u32 = 0x04e8;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW7: u32 = 0x04ec;
pub const cfgPCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW8: u32 = 0x04f0;
//#define cfgBIF_CFG_DEV0_EPF0_VENDOR_ID                                                                  0x0000
//#define cfgBIF_CFG_DEV0_EPF0_DEVICE_ID                                                                  0x0002
//#define cfgBIF_CFG_DEV0_EPF0_COMMAND                                                                    0x0004
//#define cfgBIF_CFG_DEV0_EPF0_STATUS                                                                     0x0006
//#define cfgBIF_CFG_DEV0_EPF0_REVISION_ID                                                                0x0008
//#define cfgBIF_CFG_DEV0_EPF0_PROG_INTERFACE                                                             0x0009
//#define cfgBIF_CFG_DEV0_EPF0_SUB_CLASS                                                                  0x000a
//#define cfgBIF_CFG_DEV0_EPF0_BASE_CLASS                                                                 0x000b
//#define cfgBIF_CFG_DEV0_EPF0_CACHE_LINE                                                                 0x000c
//#define cfgBIF_CFG_DEV0_EPF0_LATENCY                                                                    0x000d
//#define cfgBIF_CFG_DEV0_EPF0_HEADER                                                                     0x000e
//#define cfgBIF_CFG_DEV0_EPF0_BIST                                                                       0x000f
//#define cfgBIF_CFG_DEV0_EPF0_BASE_ADDR_1                                                                0x0010
//#define cfgBIF_CFG_DEV0_EPF0_BASE_ADDR_2                                                                0x0014
//#define cfgBIF_CFG_DEV0_EPF0_BASE_ADDR_3                                                                0x0018
//#define cfgBIF_CFG_DEV0_EPF0_BASE_ADDR_4                                                                0x001c
//#define cfgBIF_CFG_DEV0_EPF0_BASE_ADDR_5                                                                0x0020
//#define cfgBIF_CFG_DEV0_EPF0_BASE_ADDR_6                                                                0x0024
//#define cfgBIF_CFG_DEV0_EPF0_ADAPTER_ID                                                                 0x002c
//#define cfgBIF_CFG_DEV0_EPF0_ROM_BASE_ADDR                                                              0x0030
//#define cfgBIF_CFG_DEV0_EPF0_CAP_PTR                                                                    0x0034
//#define cfgBIF_CFG_DEV0_EPF0_INTERRUPT_LINE                                                             0x003c
//#define cfgBIF_CFG_DEV0_EPF0_INTERRUPT_PIN                                                              0x003d
//#define cfgBIF_CFG_DEV0_EPF0_MIN_GRANT                                                                  0x003e
//#define cfgBIF_CFG_DEV0_EPF0_MAX_LATENCY                                                                0x003f
//#define cfgBIF_CFG_DEV0_EPF0_VENDOR_CAP_LIST                                                            0x0048
//#define cfgBIF_CFG_DEV0_EPF0_ADAPTER_ID_W                                                               0x004c
//#define cfgBIF_CFG_DEV0_EPF0_PMI_CAP_LIST                                                               0x0050
//#define cfgBIF_CFG_DEV0_EPF0_PMI_CAP                                                                    0x0052
//#define cfgBIF_CFG_DEV0_EPF0_PMI_STATUS_CNTL                                                            0x0054
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_CAP_LIST                                                              0x0064
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_CAP                                                                   0x0066
//#define cfgBIF_CFG_DEV0_EPF0_DEVICE_CAP                                                                 0x0068
//#define cfgBIF_CFG_DEV0_EPF0_DEVICE_CNTL                                                                0x006c
//#define cfgBIF_CFG_DEV0_EPF0_DEVICE_STATUS                                                              0x006e
//#define cfgBIF_CFG_DEV0_EPF0_LINK_CAP                                                                   0x0070
//#define cfgBIF_CFG_DEV0_EPF0_LINK_CNTL                                                                  0x0074
//#define cfgBIF_CFG_DEV0_EPF0_LINK_STATUS                                                                0x0076
//#define cfgBIF_CFG_DEV0_EPF0_DEVICE_CAP2                                                                0x0088
//#define cfgBIF_CFG_DEV0_EPF0_DEVICE_CNTL2                                                               0x008c
//#define cfgBIF_CFG_DEV0_EPF0_DEVICE_STATUS2                                                             0x008e
//#define cfgBIF_CFG_DEV0_EPF0_LINK_CAP2                                                                  0x0090
//#define cfgBIF_CFG_DEV0_EPF0_LINK_CNTL2                                                                 0x0094
//#define cfgBIF_CFG_DEV0_EPF0_LINK_STATUS2                                                               0x0096
//#define cfgBIF_CFG_DEV0_EPF0_SLOT_CAP2                                                                  0x0098
//#define cfgBIF_CFG_DEV0_EPF0_SLOT_CNTL2                                                                 0x009c
//#define cfgBIF_CFG_DEV0_EPF0_SLOT_STATUS2                                                               0x009e
//#define cfgBIF_CFG_DEV0_EPF0_MSI_CAP_LIST                                                               0x00a0
//#define cfgBIF_CFG_DEV0_EPF0_MSI_MSG_CNTL                                                               0x00a2
//#define cfgBIF_CFG_DEV0_EPF0_MSI_MSG_ADDR_LO                                                            0x00a4
//#define cfgBIF_CFG_DEV0_EPF0_MSI_MSG_ADDR_HI                                                            0x00a8
//#define cfgBIF_CFG_DEV0_EPF0_MSI_MSG_DATA                                                               0x00a8
//#define cfgBIF_CFG_DEV0_EPF0_MSI_MASK                                                                   0x00ac
//#define cfgBIF_CFG_DEV0_EPF0_MSI_MSG_DATA_64                                                            0x00ac
//#define cfgBIF_CFG_DEV0_EPF0_MSI_MASK_64                                                                0x00b0
//#define cfgBIF_CFG_DEV0_EPF0_MSI_PENDING                                                                0x00b0
//#define cfgBIF_CFG_DEV0_EPF0_MSI_PENDING_64                                                             0x00b4
//#define cfgBIF_CFG_DEV0_EPF0_MSIX_CAP_LIST                                                              0x00c0
//#define cfgBIF_CFG_DEV0_EPF0_MSIX_MSG_CNTL                                                              0x00c2
//#define cfgBIF_CFG_DEV0_EPF0_MSIX_TABLE                                                                 0x00c4
//#define cfgBIF_CFG_DEV0_EPF0_MSIX_PBA                                                                   0x00c8
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST                                          0x0100
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR                                                   0x0104
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC1                                                      0x0108
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC2                                                      0x010c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VC_ENH_CAP_LIST                                                       0x0110
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PORT_VC_CAP_REG1                                                      0x0114
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PORT_VC_CAP_REG2                                                      0x0118
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PORT_VC_CNTL                                                          0x011c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PORT_VC_STATUS                                                        0x011e
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VC0_RESOURCE_CAP                                                      0x0120
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VC0_RESOURCE_CNTL                                                     0x0124
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VC0_RESOURCE_STATUS                                                   0x012a
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VC1_RESOURCE_CAP                                                      0x012c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VC1_RESOURCE_CNTL                                                     0x0130
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VC1_RESOURCE_STATUS                                                   0x0136
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DEV_SERIAL_NUM_ENH_CAP_LIST                                           0x0140
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DEV_SERIAL_NUM_DW1                                                    0x0144
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DEV_SERIAL_NUM_DW2                                                    0x0148
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST                                              0x0150
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_UNCORR_ERR_STATUS                                                     0x0154
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_UNCORR_ERR_MASK                                                       0x0158
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_UNCORR_ERR_SEVERITY                                                   0x015c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_CORR_ERR_STATUS                                                       0x0160
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_CORR_ERR_MASK                                                         0x0164
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_ADV_ERR_CAP_CNTL                                                      0x0168
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_HDR_LOG0                                                              0x016c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_HDR_LOG1                                                              0x0170
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_HDR_LOG2                                                              0x0174
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_HDR_LOG3                                                              0x0178
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_TLP_PREFIX_LOG0                                                       0x0188
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_TLP_PREFIX_LOG1                                                       0x018c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_TLP_PREFIX_LOG2                                                       0x0190
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_TLP_PREFIX_LOG3                                                       0x0194
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_BAR_ENH_CAP_LIST                                                      0x0200
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_BAR1_CAP                                                              0x0204
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_BAR1_CNTL                                                             0x0208
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_BAR2_CAP                                                              0x020c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_BAR2_CNTL                                                             0x0210
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_BAR3_CAP                                                              0x0214
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_BAR3_CNTL                                                             0x0218
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_BAR4_CAP                                                              0x021c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_BAR4_CNTL                                                             0x0220
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_BAR5_CAP                                                              0x0224
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_BAR5_CNTL                                                             0x0228
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_BAR6_CAP                                                              0x022c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_BAR6_CNTL                                                             0x0230
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PWR_BUDGET_ENH_CAP_LIST                                               0x0240
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PWR_BUDGET_DATA_SELECT                                                0x0244
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PWR_BUDGET_DATA                                                       0x0248
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PWR_BUDGET_CAP                                                        0x024c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DPA_ENH_CAP_LIST                                                      0x0250
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DPA_CAP                                                               0x0254
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DPA_LATENCY_INDICATOR                                                 0x0258
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DPA_STATUS                                                            0x025c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DPA_CNTL                                                              0x025e
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DPA_SUBSTATE_PWR_ALLOC_0                                              0x0260
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DPA_SUBSTATE_PWR_ALLOC_1                                              0x0261
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DPA_SUBSTATE_PWR_ALLOC_2                                              0x0262
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DPA_SUBSTATE_PWR_ALLOC_3                                              0x0263
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DPA_SUBSTATE_PWR_ALLOC_4                                              0x0264
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DPA_SUBSTATE_PWR_ALLOC_5                                              0x0265
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DPA_SUBSTATE_PWR_ALLOC_6                                              0x0266
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_DPA_SUBSTATE_PWR_ALLOC_7                                              0x0267
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SECONDARY_ENH_CAP_LIST                                                0x0270
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LINK_CNTL3                                                            0x0274
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_ERROR_STATUS                                                     0x0278
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_0_EQUALIZATION_CNTL                                              0x027c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_1_EQUALIZATION_CNTL                                              0x027e
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_2_EQUALIZATION_CNTL                                              0x0280
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_3_EQUALIZATION_CNTL                                              0x0282
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_4_EQUALIZATION_CNTL                                              0x0284
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_5_EQUALIZATION_CNTL                                              0x0286
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_6_EQUALIZATION_CNTL                                              0x0288
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_7_EQUALIZATION_CNTL                                              0x028a
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_8_EQUALIZATION_CNTL                                              0x028c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_9_EQUALIZATION_CNTL                                              0x028e
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_10_EQUALIZATION_CNTL                                             0x0290
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_11_EQUALIZATION_CNTL                                             0x0292
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_12_EQUALIZATION_CNTL                                             0x0294
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_13_EQUALIZATION_CNTL                                             0x0296
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_14_EQUALIZATION_CNTL                                             0x0298
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LANE_15_EQUALIZATION_CNTL                                             0x029a
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_ACS_ENH_CAP_LIST                                                      0x02a0
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_ACS_CAP                                                               0x02a4
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_ACS_CNTL                                                              0x02a6
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_ATS_ENH_CAP_LIST                                                      0x02b0
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_ATS_CAP                                                               0x02b4
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_ATS_CNTL                                                              0x02b6
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PAGE_REQ_ENH_CAP_LIST                                                 0x02c0
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PAGE_REQ_CNTL                                                         0x02c4
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PAGE_REQ_STATUS                                                       0x02c6
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_OUTSTAND_PAGE_REQ_CAPACITY                                            0x02c8
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_OUTSTAND_PAGE_REQ_ALLOC                                               0x02cc
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PASID_ENH_CAP_LIST                                                    0x02d0
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PASID_CAP                                                             0x02d4
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_PASID_CNTL                                                            0x02d6
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_TPH_REQR_ENH_CAP_LIST                                                 0x02e0
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_TPH_REQR_CAP                                                          0x02e4
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_TPH_REQR_CNTL                                                         0x02e8
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_MC_ENH_CAP_LIST                                                       0x02f0
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_MC_CAP                                                                0x02f4
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_MC_CNTL                                                               0x02f6
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_MC_ADDR0                                                              0x02f8
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_MC_ADDR1                                                              0x02fc
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_MC_RCV0                                                               0x0300
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_MC_RCV1                                                               0x0304
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_MC_BLOCK_ALL0                                                         0x0308
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_MC_BLOCK_ALL1                                                         0x030c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_MC_BLOCK_UNTRANSLATED_0                                               0x0310
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_MC_BLOCK_UNTRANSLATED_1                                               0x0314
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LTR_ENH_CAP_LIST                                                      0x0320
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_LTR_CAP                                                               0x0324
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_ARI_ENH_CAP_LIST                                                      0x0328
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_ARI_CAP                                                               0x032c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_ARI_CNTL                                                              0x032e
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_ENH_CAP_LIST                                                    0x0330
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_CAP                                                             0x0334
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_CONTROL                                                         0x0338
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_STATUS                                                          0x033a
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_INITIAL_VFS                                                     0x033c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_TOTAL_VFS                                                       0x033e
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_NUM_VFS                                                         0x0340
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_FUNC_DEP_LINK                                                   0x0342
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_FIRST_VF_OFFSET                                                 0x0344
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_VF_STRIDE                                                       0x0346
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_VF_DEVICE_ID                                                    0x034a
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_SUPPORTED_PAGE_SIZE                                             0x034c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_SYSTEM_PAGE_SIZE                                                0x0350
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_VF_BASE_ADDR_0                                                  0x0354
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_VF_BASE_ADDR_1                                                  0x0358
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_VF_BASE_ADDR_2                                                  0x035c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_VF_BASE_ADDR_3                                                  0x0360
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_VF_BASE_ADDR_4                                                  0x0364
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_VF_BASE_ADDR_5                                                  0x0368
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_SRIOV_VF_MIGRATION_STATE_ARRAY_OFFSET                                 0x036c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST_GPUIOV                                   0x0400
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV                                            0x0404
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_SRIOV_SHADOW                               0x0408
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_INTR_ENABLE                                0x040c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_INTR_STATUS                                0x0410
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_RESET_CONTROL                              0x0414
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_HVVM_MBOX_DW0                              0x0418
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_HVVM_MBOX_DW1                              0x041c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_HVVM_MBOX_DW2                              0x0420
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_CONTEXT                                    0x0424
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_TOTAL_FB                                   0x0428
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_OFFSETS                                    0x042c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF0_FB                                     0x0430
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF1_FB                                     0x0434
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF2_FB                                     0x0438
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF3_FB                                     0x043c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF4_FB                                     0x0440
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF5_FB                                     0x0444
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF6_FB                                     0x0448
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF7_FB                                     0x044c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF8_FB                                     0x0450
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF9_FB                                     0x0454
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF10_FB                                    0x0458
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF11_FB                                    0x045c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF12_FB                                    0x0460
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF13_FB                                    0x0464
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF14_FB                                    0x0468
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF15_FB                                    0x046c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW0                                 0x0470
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW1                                 0x0474
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW2                                 0x0478
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW3                                 0x047c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW4                                 0x0480
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW5                                 0x0484
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW6                                 0x0488
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW7                                 0x048c
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW8                                 0x0490
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW0                                 0x04a0
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW1                                 0x04a4
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW2                                 0x04a8
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW3                                 0x04ac
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW4                                 0x04b0
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW5                                 0x04b4
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW6                                 0x04b8
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW7                                 0x04bc
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW8                                 0x04c0
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW0                                 0x04d0
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW1                                 0x04d4
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW2                                 0x04d8
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW3                                 0x04dc
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW4                                 0x04e0
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW5                                 0x04e4
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW6                                 0x04e8
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW7                                 0x04ec
//#define cfgBIF_CFG_DEV0_EPF0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW8                                 0x04f0


// addressBlock: nbio_nbif0_bif_cfg_dev0_epf1_bifcfgdecp
// base address: 0x0
pub const cfgBIF_CFG_DEV0_EPF1_0_VENDOR_ID: u32 = 0x0000;
pub const cfgBIF_CFG_DEV0_EPF1_0_DEVICE_ID: u32 = 0x0002;
pub const cfgBIF_CFG_DEV0_EPF1_0_COMMAND: u32 = 0x0004;
pub const cfgBIF_CFG_DEV0_EPF1_0_STATUS: u32 = 0x0006;
pub const cfgBIF_CFG_DEV0_EPF1_0_REVISION_ID: u32 = 0x0008;
pub const cfgBIF_CFG_DEV0_EPF1_0_PROG_INTERFACE: u32 = 0x0009;
pub const cfgBIF_CFG_DEV0_EPF1_0_SUB_CLASS: u32 = 0x000a;
pub const cfgBIF_CFG_DEV0_EPF1_0_BASE_CLASS: u32 = 0x000b;
pub const cfgBIF_CFG_DEV0_EPF1_0_CACHE_LINE: u32 = 0x000c;
pub const cfgBIF_CFG_DEV0_EPF1_0_LATENCY: u32 = 0x000d;
pub const cfgBIF_CFG_DEV0_EPF1_0_HEADER: u32 = 0x000e;
pub const cfgBIF_CFG_DEV0_EPF1_0_BIST: u32 = 0x000f;
pub const cfgBIF_CFG_DEV0_EPF1_0_BASE_ADDR_1: u32 = 0x0010;
pub const cfgBIF_CFG_DEV0_EPF1_0_BASE_ADDR_2: u32 = 0x0014;
pub const cfgBIF_CFG_DEV0_EPF1_0_BASE_ADDR_3: u32 = 0x0018;
pub const cfgBIF_CFG_DEV0_EPF1_0_BASE_ADDR_4: u32 = 0x001c;
pub const cfgBIF_CFG_DEV0_EPF1_0_BASE_ADDR_5: u32 = 0x0020;
pub const cfgBIF_CFG_DEV0_EPF1_0_BASE_ADDR_6: u32 = 0x0024;
pub const cfgBIF_CFG_DEV0_EPF1_0_ADAPTER_ID: u32 = 0x002c;
pub const cfgBIF_CFG_DEV0_EPF1_0_ROM_BASE_ADDR: u32 = 0x0030;
pub const cfgBIF_CFG_DEV0_EPF1_0_CAP_PTR: u32 = 0x0034;
pub const cfgBIF_CFG_DEV0_EPF1_0_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgBIF_CFG_DEV0_EPF1_0_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgBIF_CFG_DEV0_EPF1_0_MIN_GRANT: u32 = 0x003e;
pub const cfgBIF_CFG_DEV0_EPF1_0_MAX_LATENCY: u32 = 0x003f;
pub const cfgBIF_CFG_DEV0_EPF1_0_VENDOR_CAP_LIST: u32 = 0x0048;
pub const cfgBIF_CFG_DEV0_EPF1_0_ADAPTER_ID_W: u32 = 0x004c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PMI_CAP_LIST: u32 = 0x0050;
pub const cfgBIF_CFG_DEV0_EPF1_0_PMI_CAP: u32 = 0x0052;
pub const cfgBIF_CFG_DEV0_EPF1_0_PMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_CAP_LIST: u32 = 0x0064;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_CAP: u32 = 0x0066;
pub const cfgBIF_CFG_DEV0_EPF1_0_DEVICE_CAP: u32 = 0x0068;
pub const cfgBIF_CFG_DEV0_EPF1_0_DEVICE_CNTL: u32 = 0x006c;
pub const cfgBIF_CFG_DEV0_EPF1_0_DEVICE_STATUS: u32 = 0x006e;
pub const cfgBIF_CFG_DEV0_EPF1_0_LINK_CAP: u32 = 0x0070;
pub const cfgBIF_CFG_DEV0_EPF1_0_LINK_CNTL: u32 = 0x0074;
pub const cfgBIF_CFG_DEV0_EPF1_0_LINK_STATUS: u32 = 0x0076;
pub const cfgBIF_CFG_DEV0_EPF1_0_DEVICE_CAP2: u32 = 0x0088;
pub const cfgBIF_CFG_DEV0_EPF1_0_DEVICE_CNTL2: u32 = 0x008c;
pub const cfgBIF_CFG_DEV0_EPF1_0_DEVICE_STATUS2: u32 = 0x008e;
pub const cfgBIF_CFG_DEV0_EPF1_0_LINK_CAP2: u32 = 0x0090;
pub const cfgBIF_CFG_DEV0_EPF1_0_LINK_CNTL2: u32 = 0x0094;
pub const cfgBIF_CFG_DEV0_EPF1_0_LINK_STATUS2: u32 = 0x0096;
pub const cfgBIF_CFG_DEV0_EPF1_0_SLOT_CAP2: u32 = 0x0098;
pub const cfgBIF_CFG_DEV0_EPF1_0_SLOT_CNTL2: u32 = 0x009c;
pub const cfgBIF_CFG_DEV0_EPF1_0_SLOT_STATUS2: u32 = 0x009e;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSI_CAP_LIST: u32 = 0x00a0;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSI_MSG_DATA: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSI_MASK: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSI_MASK_64: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSI_PENDING: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSI_PENDING_64: u32 = 0x00b4;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSIX_CAP_LIST: u32 = 0x00c0;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSIX_MSG_CNTL: u32 = 0x00c2;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSIX_TABLE: u32 = 0x00c4;
pub const cfgBIF_CFG_DEV0_EPF1_0_MSIX_PBA: u32 = 0x00c8;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VC_ENH_CAP_LIST: u32 = 0x0110;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PORT_VC_CAP_REG1: u32 = 0x0114;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PORT_VC_CAP_REG2: u32 = 0x0118;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PORT_VC_CNTL: u32 = 0x011c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PORT_VC_STATUS: u32 = 0x011e;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VC0_RESOURCE_CAP: u32 = 0x0120;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VC0_RESOURCE_CNTL: u32 = 0x0124;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VC0_RESOURCE_STATUS: u32 = 0x012a;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VC1_RESOURCE_CAP: u32 = 0x012c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VC1_RESOURCE_CNTL: u32 = 0x0130;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VC1_RESOURCE_STATUS: u32 = 0x0136;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DEV_SERIAL_NUM_ENH_CAP_LIST: u32 = 0x0140;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DEV_SERIAL_NUM_DW1: u32 = 0x0144;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DEV_SERIAL_NUM_DW2: u32 = 0x0148;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_BAR_ENH_CAP_LIST: u32 = 0x0200;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_BAR1_CAP: u32 = 0x0204;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_BAR1_CNTL: u32 = 0x0208;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_BAR2_CAP: u32 = 0x020c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_BAR2_CNTL: u32 = 0x0210;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_BAR3_CAP: u32 = 0x0214;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_BAR3_CNTL: u32 = 0x0218;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_BAR4_CAP: u32 = 0x021c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_BAR4_CNTL: u32 = 0x0220;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_BAR5_CAP: u32 = 0x0224;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_BAR5_CNTL: u32 = 0x0228;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_BAR6_CAP: u32 = 0x022c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_BAR6_CNTL: u32 = 0x0230;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PWR_BUDGET_ENH_CAP_LIST: u32 = 0x0240;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PWR_BUDGET_DATA_SELECT: u32 = 0x0244;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PWR_BUDGET_DATA: u32 = 0x0248;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PWR_BUDGET_CAP: u32 = 0x024c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DPA_ENH_CAP_LIST: u32 = 0x0250;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DPA_CAP: u32 = 0x0254;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DPA_LATENCY_INDICATOR: u32 = 0x0258;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DPA_STATUS: u32 = 0x025c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DPA_CNTL: u32 = 0x025e;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_0: u32 = 0x0260;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_1: u32 = 0x0261;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_2: u32 = 0x0262;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_3: u32 = 0x0263;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_4: u32 = 0x0264;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_5: u32 = 0x0265;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_6: u32 = 0x0266;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_7: u32 = 0x0267;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SECONDARY_ENH_CAP_LIST: u32 = 0x0270;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LINK_CNTL3: u32 = 0x0274;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_ERROR_STATUS: u32 = 0x0278;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_0_EQUALIZATION_CNTL: u32 = 0x027c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_1_EQUALIZATION_CNTL: u32 = 0x027e;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_2_EQUALIZATION_CNTL: u32 = 0x0280;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_3_EQUALIZATION_CNTL: u32 = 0x0282;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_4_EQUALIZATION_CNTL: u32 = 0x0284;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_5_EQUALIZATION_CNTL: u32 = 0x0286;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_6_EQUALIZATION_CNTL: u32 = 0x0288;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_7_EQUALIZATION_CNTL: u32 = 0x028a;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_8_EQUALIZATION_CNTL: u32 = 0x028c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_9_EQUALIZATION_CNTL: u32 = 0x028e;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_10_EQUALIZATION_CNTL: u32 = 0x0290;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_11_EQUALIZATION_CNTL: u32 = 0x0292;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_12_EQUALIZATION_CNTL: u32 = 0x0294;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_13_EQUALIZATION_CNTL: u32 = 0x0296;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_14_EQUALIZATION_CNTL: u32 = 0x0298;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LANE_15_EQUALIZATION_CNTL: u32 = 0x029a;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_ACS_CNTL: u32 = 0x02a6;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_ATS_ENH_CAP_LIST: u32 = 0x02b0;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_ATS_CAP: u32 = 0x02b4;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_ATS_CNTL: u32 = 0x02b6;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PAGE_REQ_ENH_CAP_LIST: u32 = 0x02c0;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PAGE_REQ_CNTL: u32 = 0x02c4;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PAGE_REQ_STATUS: u32 = 0x02c6;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_OUTSTAND_PAGE_REQ_CAPACITY: u32 = 0x02c8;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_OUTSTAND_PAGE_REQ_ALLOC: u32 = 0x02cc;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PASID_ENH_CAP_LIST: u32 = 0x02d0;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PASID_CAP: u32 = 0x02d4;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_PASID_CNTL: u32 = 0x02d6;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_TPH_REQR_ENH_CAP_LIST: u32 = 0x02e0;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_TPH_REQR_CAP: u32 = 0x02e4;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_TPH_REQR_CNTL: u32 = 0x02e8;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_MC_ENH_CAP_LIST: u32 = 0x02f0;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_MC_CAP: u32 = 0x02f4;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_MC_CNTL: u32 = 0x02f6;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_MC_ADDR0: u32 = 0x02f8;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_MC_ADDR1: u32 = 0x02fc;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_MC_RCV0: u32 = 0x0300;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_MC_RCV1: u32 = 0x0304;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_MC_BLOCK_ALL0: u32 = 0x0308;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_MC_BLOCK_ALL1: u32 = 0x030c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_MC_BLOCK_UNTRANSLATED_0: u32 = 0x0310;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_MC_BLOCK_UNTRANSLATED_1: u32 = 0x0314;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LTR_ENH_CAP_LIST: u32 = 0x0320;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_LTR_CAP: u32 = 0x0324;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_ARI_ENH_CAP_LIST: u32 = 0x0328;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_ARI_CAP: u32 = 0x032c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_ARI_CNTL: u32 = 0x032e;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_ENH_CAP_LIST: u32 = 0x0330;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_CAP: u32 = 0x0334;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_CONTROL: u32 = 0x0338;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_STATUS: u32 = 0x033a;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_INITIAL_VFS: u32 = 0x033c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_TOTAL_VFS: u32 = 0x033e;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_NUM_VFS: u32 = 0x0340;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_FUNC_DEP_LINK: u32 = 0x0342;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_FIRST_VF_OFFSET: u32 = 0x0344;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_VF_STRIDE: u32 = 0x0346;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_VF_DEVICE_ID: u32 = 0x034a;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_SUPPORTED_PAGE_SIZE: u32 = 0x034c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_SYSTEM_PAGE_SIZE: u32 = 0x0350;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_VF_BASE_ADDR_0: u32 = 0x0354;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_VF_BASE_ADDR_1: u32 = 0x0358;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_VF_BASE_ADDR_2: u32 = 0x035c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_VF_BASE_ADDR_3: u32 = 0x0360;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_VF_BASE_ADDR_4: u32 = 0x0364;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_VF_BASE_ADDR_5: u32 = 0x0368;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_SRIOV_VF_MIGRATION_STATE_ARRAY_OFFSET: u32 = 0x036c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST_GPUIOV: u32 = 0x0400;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV: u32 = 0x0404;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_SRIOV_SHADOW: u32 = 0x0408;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_INTR_ENABLE: u32 = 0x040c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_INTR_STATUS: u32 = 0x0410;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_RESET_CONTROL: u32 = 0x0414;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_HVVM_MBOX_DW0: u32 = 0x0418;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_HVVM_MBOX_DW1: u32 = 0x041c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_HVVM_MBOX_DW2: u32 = 0x0420;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_CONTEXT: u32 = 0x0424;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_TOTAL_FB: u32 = 0x0428;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_OFFSETS: u32 = 0x042c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF0_FB: u32 = 0x0430;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF1_FB: u32 = 0x0434;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF2_FB: u32 = 0x0438;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF3_FB: u32 = 0x043c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF4_FB: u32 = 0x0440;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF5_FB: u32 = 0x0444;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF6_FB: u32 = 0x0448;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF7_FB: u32 = 0x044c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF8_FB: u32 = 0x0450;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF9_FB: u32 = 0x0454;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF10_FB: u32 = 0x0458;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF11_FB: u32 = 0x045c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF12_FB: u32 = 0x0460;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF13_FB: u32 = 0x0464;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF14_FB: u32 = 0x0468;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VF15_FB: u32 = 0x046c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW0: u32 = 0x0470;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW1: u32 = 0x0474;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW2: u32 = 0x0478;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW3: u32 = 0x047c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW4: u32 = 0x0480;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW5: u32 = 0x0484;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW6: u32 = 0x0488;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW7: u32 = 0x048c;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_UVDSCH_DW8: u32 = 0x0490;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW0: u32 = 0x04a0;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW1: u32 = 0x04a4;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW2: u32 = 0x04a8;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW3: u32 = 0x04ac;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW4: u32 = 0x04b0;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW5: u32 = 0x04b4;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW6: u32 = 0x04b8;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW7: u32 = 0x04bc;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_VCESCH_DW8: u32 = 0x04c0;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW0: u32 = 0x04d0;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW1: u32 = 0x04d4;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW2: u32 = 0x04d8;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW3: u32 = 0x04dc;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW4: u32 = 0x04e0;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW5: u32 = 0x04e4;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW6: u32 = 0x04e8;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW7: u32 = 0x04ec;
pub const cfgBIF_CFG_DEV0_EPF1_0_PCIE_VENDOR_SPECIFIC_HDR_GPUIOV_GFXSCH_DW8: u32 = 0x04f0;


// addressBlock: nbio_nbif0_bif_cfg_dev0_epf2_bifcfgdecp
// base address: 0x0
pub const cfgBIF_CFG_DEV0_EPF2_0_VENDOR_ID: u32 = 0x0000;
pub const cfgBIF_CFG_DEV0_EPF2_0_DEVICE_ID: u32 = 0x0002;
pub const cfgBIF_CFG_DEV0_EPF2_0_COMMAND: u32 = 0x0004;
pub const cfgBIF_CFG_DEV0_EPF2_0_STATUS: u32 = 0x0006;
pub const cfgBIF_CFG_DEV0_EPF2_0_REVISION_ID: u32 = 0x0008;
pub const cfgBIF_CFG_DEV0_EPF2_0_PROG_INTERFACE: u32 = 0x0009;
pub const cfgBIF_CFG_DEV0_EPF2_0_SUB_CLASS: u32 = 0x000a;
pub const cfgBIF_CFG_DEV0_EPF2_0_BASE_CLASS: u32 = 0x000b;
pub const cfgBIF_CFG_DEV0_EPF2_0_CACHE_LINE: u32 = 0x000c;
pub const cfgBIF_CFG_DEV0_EPF2_0_LATENCY: u32 = 0x000d;
pub const cfgBIF_CFG_DEV0_EPF2_0_HEADER: u32 = 0x000e;
pub const cfgBIF_CFG_DEV0_EPF2_0_BIST: u32 = 0x000f;
pub const cfgBIF_CFG_DEV0_EPF2_0_BASE_ADDR_1: u32 = 0x0010;
pub const cfgBIF_CFG_DEV0_EPF2_0_BASE_ADDR_2: u32 = 0x0014;
pub const cfgBIF_CFG_DEV0_EPF2_0_BASE_ADDR_3: u32 = 0x0018;
pub const cfgBIF_CFG_DEV0_EPF2_0_BASE_ADDR_4: u32 = 0x001c;
pub const cfgBIF_CFG_DEV0_EPF2_0_BASE_ADDR_5: u32 = 0x0020;
pub const cfgBIF_CFG_DEV0_EPF2_0_BASE_ADDR_6: u32 = 0x0024;
pub const cfgBIF_CFG_DEV0_EPF2_0_ADAPTER_ID: u32 = 0x002c;
pub const cfgBIF_CFG_DEV0_EPF2_0_ROM_BASE_ADDR: u32 = 0x0030;
pub const cfgBIF_CFG_DEV0_EPF2_0_CAP_PTR: u32 = 0x0034;
pub const cfgBIF_CFG_DEV0_EPF2_0_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgBIF_CFG_DEV0_EPF2_0_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgBIF_CFG_DEV0_EPF2_0_MIN_GRANT: u32 = 0x003e;
pub const cfgBIF_CFG_DEV0_EPF2_0_MAX_LATENCY: u32 = 0x003f;
pub const cfgBIF_CFG_DEV0_EPF2_0_VENDOR_CAP_LIST: u32 = 0x0048;
pub const cfgBIF_CFG_DEV0_EPF2_0_ADAPTER_ID_W: u32 = 0x004c;
pub const cfgBIF_CFG_DEV0_EPF2_0_PMI_CAP_LIST: u32 = 0x0050;
pub const cfgBIF_CFG_DEV0_EPF2_0_PMI_CAP: u32 = 0x0052;
pub const cfgBIF_CFG_DEV0_EPF2_0_PMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgBIF_CFG_DEV0_EPF2_0_SBRN: u32 = 0x0060;
pub const cfgBIF_CFG_DEV0_EPF2_0_FLADJ: u32 = 0x0061;
pub const cfgBIF_CFG_DEV0_EPF2_0_DBESL_DBESLD: u32 = 0x0062;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_CAP_LIST: u32 = 0x0064;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_CAP: u32 = 0x0066;
pub const cfgBIF_CFG_DEV0_EPF2_0_DEVICE_CAP: u32 = 0x0068;
pub const cfgBIF_CFG_DEV0_EPF2_0_DEVICE_CNTL: u32 = 0x006c;
pub const cfgBIF_CFG_DEV0_EPF2_0_DEVICE_STATUS: u32 = 0x006e;
pub const cfgBIF_CFG_DEV0_EPF2_0_LINK_CAP: u32 = 0x0070;
pub const cfgBIF_CFG_DEV0_EPF2_0_LINK_CNTL: u32 = 0x0074;
pub const cfgBIF_CFG_DEV0_EPF2_0_LINK_STATUS: u32 = 0x0076;
pub const cfgBIF_CFG_DEV0_EPF2_0_DEVICE_CAP2: u32 = 0x0088;
pub const cfgBIF_CFG_DEV0_EPF2_0_DEVICE_CNTL2: u32 = 0x008c;
pub const cfgBIF_CFG_DEV0_EPF2_0_DEVICE_STATUS2: u32 = 0x008e;
pub const cfgBIF_CFG_DEV0_EPF2_0_LINK_CAP2: u32 = 0x0090;
pub const cfgBIF_CFG_DEV0_EPF2_0_LINK_CNTL2: u32 = 0x0094;
pub const cfgBIF_CFG_DEV0_EPF2_0_LINK_STATUS2: u32 = 0x0096;
pub const cfgBIF_CFG_DEV0_EPF2_0_SLOT_CAP2: u32 = 0x0098;
pub const cfgBIF_CFG_DEV0_EPF2_0_SLOT_CNTL2: u32 = 0x009c;
pub const cfgBIF_CFG_DEV0_EPF2_0_SLOT_STATUS2: u32 = 0x009e;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSI_CAP_LIST: u32 = 0x00a0;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSI_MSG_DATA: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSI_MASK: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSI_MASK_64: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSI_PENDING: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSI_PENDING_64: u32 = 0x00b4;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSIX_CAP_LIST: u32 = 0x00c0;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSIX_MSG_CNTL: u32 = 0x00c2;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSIX_TABLE: u32 = 0x00c4;
pub const cfgBIF_CFG_DEV0_EPF2_0_MSIX_PBA: u32 = 0x00c8;
pub const cfgBIF_CFG_DEV0_EPF2_0_SATA_CAP_0: u32 = 0x00d0;
pub const cfgBIF_CFG_DEV0_EPF2_0_SATA_CAP_1: u32 = 0x00d4;
pub const cfgBIF_CFG_DEV0_EPF2_0_SATA_IDP_INDEX: u32 = 0x00d8;
pub const cfgBIF_CFG_DEV0_EPF2_0_SATA_IDP_DATA: u32 = 0x00dc;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_BAR_ENH_CAP_LIST: u32 = 0x0200;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_BAR1_CAP: u32 = 0x0204;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_BAR1_CNTL: u32 = 0x0208;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_BAR2_CAP: u32 = 0x020c;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_BAR2_CNTL: u32 = 0x0210;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_BAR3_CAP: u32 = 0x0214;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_BAR3_CNTL: u32 = 0x0218;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_BAR4_CAP: u32 = 0x021c;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_BAR4_CNTL: u32 = 0x0220;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_BAR5_CAP: u32 = 0x0224;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_BAR5_CNTL: u32 = 0x0228;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_BAR6_CAP: u32 = 0x022c;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_BAR6_CNTL: u32 = 0x0230;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_PWR_BUDGET_ENH_CAP_LIST: u32 = 0x0240;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_PWR_BUDGET_DATA_SELECT: u32 = 0x0244;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_PWR_BUDGET_DATA: u32 = 0x0248;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_PWR_BUDGET_CAP: u32 = 0x024c;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_DPA_ENH_CAP_LIST: u32 = 0x0250;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_DPA_CAP: u32 = 0x0254;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_DPA_LATENCY_INDICATOR: u32 = 0x0258;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_DPA_STATUS: u32 = 0x025c;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_DPA_CNTL: u32 = 0x025e;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_0: u32 = 0x0260;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_1: u32 = 0x0261;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_2: u32 = 0x0262;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_3: u32 = 0x0263;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_4: u32 = 0x0264;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_5: u32 = 0x0265;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_6: u32 = 0x0266;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_7: u32 = 0x0267;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_ACS_CNTL: u32 = 0x02a6;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_ARI_ENH_CAP_LIST: u32 = 0x0328;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_ARI_CAP: u32 = 0x032c;
pub const cfgBIF_CFG_DEV0_EPF2_0_PCIE_ARI_CNTL: u32 = 0x032e;


// addressBlock: nbio_nbif0_bif_cfg_dev0_epf3_bifcfgdecp
// base address: 0x0
pub const cfgBIF_CFG_DEV0_EPF3_0_VENDOR_ID: u32 = 0x0000;
pub const cfgBIF_CFG_DEV0_EPF3_0_DEVICE_ID: u32 = 0x0002;
pub const cfgBIF_CFG_DEV0_EPF3_0_COMMAND: u32 = 0x0004;
pub const cfgBIF_CFG_DEV0_EPF3_0_STATUS: u32 = 0x0006;
pub const cfgBIF_CFG_DEV0_EPF3_0_REVISION_ID: u32 = 0x0008;
pub const cfgBIF_CFG_DEV0_EPF3_0_PROG_INTERFACE: u32 = 0x0009;
pub const cfgBIF_CFG_DEV0_EPF3_0_SUB_CLASS: u32 = 0x000a;
pub const cfgBIF_CFG_DEV0_EPF3_0_BASE_CLASS: u32 = 0x000b;
pub const cfgBIF_CFG_DEV0_EPF3_0_CACHE_LINE: u32 = 0x000c;
pub const cfgBIF_CFG_DEV0_EPF3_0_LATENCY: u32 = 0x000d;
pub const cfgBIF_CFG_DEV0_EPF3_0_HEADER: u32 = 0x000e;
pub const cfgBIF_CFG_DEV0_EPF3_0_BIST: u32 = 0x000f;
pub const cfgBIF_CFG_DEV0_EPF3_0_BASE_ADDR_1: u32 = 0x0010;
pub const cfgBIF_CFG_DEV0_EPF3_0_BASE_ADDR_2: u32 = 0x0014;
pub const cfgBIF_CFG_DEV0_EPF3_0_BASE_ADDR_3: u32 = 0x0018;
pub const cfgBIF_CFG_DEV0_EPF3_0_BASE_ADDR_4: u32 = 0x001c;
pub const cfgBIF_CFG_DEV0_EPF3_0_BASE_ADDR_5: u32 = 0x0020;
pub const cfgBIF_CFG_DEV0_EPF3_0_BASE_ADDR_6: u32 = 0x0024;
pub const cfgBIF_CFG_DEV0_EPF3_0_ADAPTER_ID: u32 = 0x002c;
pub const cfgBIF_CFG_DEV0_EPF3_0_ROM_BASE_ADDR: u32 = 0x0030;
pub const cfgBIF_CFG_DEV0_EPF3_0_CAP_PTR: u32 = 0x0034;
pub const cfgBIF_CFG_DEV0_EPF3_0_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgBIF_CFG_DEV0_EPF3_0_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgBIF_CFG_DEV0_EPF3_0_MIN_GRANT: u32 = 0x003e;
pub const cfgBIF_CFG_DEV0_EPF3_0_MAX_LATENCY: u32 = 0x003f;
pub const cfgBIF_CFG_DEV0_EPF3_0_VENDOR_CAP_LIST: u32 = 0x0048;
pub const cfgBIF_CFG_DEV0_EPF3_0_ADAPTER_ID_W: u32 = 0x004c;
pub const cfgBIF_CFG_DEV0_EPF3_0_PMI_CAP_LIST: u32 = 0x0050;
pub const cfgBIF_CFG_DEV0_EPF3_0_PMI_CAP: u32 = 0x0052;
pub const cfgBIF_CFG_DEV0_EPF3_0_PMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgBIF_CFG_DEV0_EPF3_0_SBRN: u32 = 0x0060;
pub const cfgBIF_CFG_DEV0_EPF3_0_FLADJ: u32 = 0x0061;
pub const cfgBIF_CFG_DEV0_EPF3_0_DBESL_DBESLD: u32 = 0x0062;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_CAP_LIST: u32 = 0x0064;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_CAP: u32 = 0x0066;
pub const cfgBIF_CFG_DEV0_EPF3_0_DEVICE_CAP: u32 = 0x0068;
pub const cfgBIF_CFG_DEV0_EPF3_0_DEVICE_CNTL: u32 = 0x006c;
pub const cfgBIF_CFG_DEV0_EPF3_0_DEVICE_STATUS: u32 = 0x006e;
pub const cfgBIF_CFG_DEV0_EPF3_0_LINK_CAP: u32 = 0x0070;
pub const cfgBIF_CFG_DEV0_EPF3_0_LINK_CNTL: u32 = 0x0074;
pub const cfgBIF_CFG_DEV0_EPF3_0_LINK_STATUS: u32 = 0x0076;
pub const cfgBIF_CFG_DEV0_EPF3_0_DEVICE_CAP2: u32 = 0x0088;
pub const cfgBIF_CFG_DEV0_EPF3_0_DEVICE_CNTL2: u32 = 0x008c;
pub const cfgBIF_CFG_DEV0_EPF3_0_DEVICE_STATUS2: u32 = 0x008e;
pub const cfgBIF_CFG_DEV0_EPF3_0_LINK_CAP2: u32 = 0x0090;
pub const cfgBIF_CFG_DEV0_EPF3_0_LINK_CNTL2: u32 = 0x0094;
pub const cfgBIF_CFG_DEV0_EPF3_0_LINK_STATUS2: u32 = 0x0096;
pub const cfgBIF_CFG_DEV0_EPF3_0_SLOT_CAP2: u32 = 0x0098;
pub const cfgBIF_CFG_DEV0_EPF3_0_SLOT_CNTL2: u32 = 0x009c;
pub const cfgBIF_CFG_DEV0_EPF3_0_SLOT_STATUS2: u32 = 0x009e;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSI_CAP_LIST: u32 = 0x00a0;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSI_MSG_DATA: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSI_MASK: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSI_MASK_64: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSI_PENDING: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSI_PENDING_64: u32 = 0x00b4;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSIX_CAP_LIST: u32 = 0x00c0;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSIX_MSG_CNTL: u32 = 0x00c2;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSIX_TABLE: u32 = 0x00c4;
pub const cfgBIF_CFG_DEV0_EPF3_0_MSIX_PBA: u32 = 0x00c8;
pub const cfgBIF_CFG_DEV0_EPF3_0_SATA_CAP_0: u32 = 0x00d0;
pub const cfgBIF_CFG_DEV0_EPF3_0_SATA_CAP_1: u32 = 0x00d4;
pub const cfgBIF_CFG_DEV0_EPF3_0_SATA_IDP_INDEX: u32 = 0x00d8;
pub const cfgBIF_CFG_DEV0_EPF3_0_SATA_IDP_DATA: u32 = 0x00dc;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_BAR_ENH_CAP_LIST: u32 = 0x0200;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_BAR1_CAP: u32 = 0x0204;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_BAR1_CNTL: u32 = 0x0208;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_BAR2_CAP: u32 = 0x020c;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_BAR2_CNTL: u32 = 0x0210;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_BAR3_CAP: u32 = 0x0214;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_BAR3_CNTL: u32 = 0x0218;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_BAR4_CAP: u32 = 0x021c;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_BAR4_CNTL: u32 = 0x0220;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_BAR5_CAP: u32 = 0x0224;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_BAR5_CNTL: u32 = 0x0228;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_BAR6_CAP: u32 = 0x022c;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_BAR6_CNTL: u32 = 0x0230;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_PWR_BUDGET_ENH_CAP_LIST: u32 = 0x0240;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_PWR_BUDGET_DATA_SELECT: u32 = 0x0244;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_PWR_BUDGET_DATA: u32 = 0x0248;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_PWR_BUDGET_CAP: u32 = 0x024c;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_DPA_ENH_CAP_LIST: u32 = 0x0250;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_DPA_CAP: u32 = 0x0254;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_DPA_LATENCY_INDICATOR: u32 = 0x0258;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_DPA_STATUS: u32 = 0x025c;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_DPA_CNTL: u32 = 0x025e;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_0: u32 = 0x0260;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_1: u32 = 0x0261;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_2: u32 = 0x0262;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_3: u32 = 0x0263;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_4: u32 = 0x0264;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_5: u32 = 0x0265;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_6: u32 = 0x0266;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_7: u32 = 0x0267;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_ACS_CNTL: u32 = 0x02a6;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_ARI_ENH_CAP_LIST: u32 = 0x0328;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_ARI_CAP: u32 = 0x032c;
pub const cfgBIF_CFG_DEV0_EPF3_0_PCIE_ARI_CNTL: u32 = 0x032e;


// addressBlock: nbio_nbif0_bif_cfg_dev0_epf4_bifcfgdecp
// base address: 0x0
pub const cfgBIF_CFG_DEV0_EPF4_0_VENDOR_ID: u32 = 0x0000;
pub const cfgBIF_CFG_DEV0_EPF4_0_DEVICE_ID: u32 = 0x0002;
pub const cfgBIF_CFG_DEV0_EPF4_0_COMMAND: u32 = 0x0004;
pub const cfgBIF_CFG_DEV0_EPF4_0_STATUS: u32 = 0x0006;
pub const cfgBIF_CFG_DEV0_EPF4_0_REVISION_ID: u32 = 0x0008;
pub const cfgBIF_CFG_DEV0_EPF4_0_PROG_INTERFACE: u32 = 0x0009;
pub const cfgBIF_CFG_DEV0_EPF4_0_SUB_CLASS: u32 = 0x000a;
pub const cfgBIF_CFG_DEV0_EPF4_0_BASE_CLASS: u32 = 0x000b;
pub const cfgBIF_CFG_DEV0_EPF4_0_CACHE_LINE: u32 = 0x000c;
pub const cfgBIF_CFG_DEV0_EPF4_0_LATENCY: u32 = 0x000d;
pub const cfgBIF_CFG_DEV0_EPF4_0_HEADER: u32 = 0x000e;
pub const cfgBIF_CFG_DEV0_EPF4_0_BIST: u32 = 0x000f;
pub const cfgBIF_CFG_DEV0_EPF4_0_BASE_ADDR_1: u32 = 0x0010;
pub const cfgBIF_CFG_DEV0_EPF4_0_BASE_ADDR_2: u32 = 0x0014;
pub const cfgBIF_CFG_DEV0_EPF4_0_BASE_ADDR_3: u32 = 0x0018;
pub const cfgBIF_CFG_DEV0_EPF4_0_BASE_ADDR_4: u32 = 0x001c;
pub const cfgBIF_CFG_DEV0_EPF4_0_BASE_ADDR_5: u32 = 0x0020;
pub const cfgBIF_CFG_DEV0_EPF4_0_BASE_ADDR_6: u32 = 0x0024;
pub const cfgBIF_CFG_DEV0_EPF4_0_ADAPTER_ID: u32 = 0x002c;
pub const cfgBIF_CFG_DEV0_EPF4_0_ROM_BASE_ADDR: u32 = 0x0030;
pub const cfgBIF_CFG_DEV0_EPF4_0_CAP_PTR: u32 = 0x0034;
pub const cfgBIF_CFG_DEV0_EPF4_0_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgBIF_CFG_DEV0_EPF4_0_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgBIF_CFG_DEV0_EPF4_0_MIN_GRANT: u32 = 0x003e;
pub const cfgBIF_CFG_DEV0_EPF4_0_MAX_LATENCY: u32 = 0x003f;
pub const cfgBIF_CFG_DEV0_EPF4_0_VENDOR_CAP_LIST: u32 = 0x0048;
pub const cfgBIF_CFG_DEV0_EPF4_0_ADAPTER_ID_W: u32 = 0x004c;
pub const cfgBIF_CFG_DEV0_EPF4_0_PMI_CAP_LIST: u32 = 0x0050;
pub const cfgBIF_CFG_DEV0_EPF4_0_PMI_CAP: u32 = 0x0052;
pub const cfgBIF_CFG_DEV0_EPF4_0_PMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgBIF_CFG_DEV0_EPF4_0_SBRN: u32 = 0x0060;
pub const cfgBIF_CFG_DEV0_EPF4_0_FLADJ: u32 = 0x0061;
pub const cfgBIF_CFG_DEV0_EPF4_0_DBESL_DBESLD: u32 = 0x0062;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_CAP_LIST: u32 = 0x0064;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_CAP: u32 = 0x0066;
pub const cfgBIF_CFG_DEV0_EPF4_0_DEVICE_CAP: u32 = 0x0068;
pub const cfgBIF_CFG_DEV0_EPF4_0_DEVICE_CNTL: u32 = 0x006c;
pub const cfgBIF_CFG_DEV0_EPF4_0_DEVICE_STATUS: u32 = 0x006e;
pub const cfgBIF_CFG_DEV0_EPF4_0_LINK_CAP: u32 = 0x0070;
pub const cfgBIF_CFG_DEV0_EPF4_0_LINK_CNTL: u32 = 0x0074;
pub const cfgBIF_CFG_DEV0_EPF4_0_LINK_STATUS: u32 = 0x0076;
pub const cfgBIF_CFG_DEV0_EPF4_0_DEVICE_CAP2: u32 = 0x0088;
pub const cfgBIF_CFG_DEV0_EPF4_0_DEVICE_CNTL2: u32 = 0x008c;
pub const cfgBIF_CFG_DEV0_EPF4_0_DEVICE_STATUS2: u32 = 0x008e;
pub const cfgBIF_CFG_DEV0_EPF4_0_LINK_CAP2: u32 = 0x0090;
pub const cfgBIF_CFG_DEV0_EPF4_0_LINK_CNTL2: u32 = 0x0094;
pub const cfgBIF_CFG_DEV0_EPF4_0_LINK_STATUS2: u32 = 0x0096;
pub const cfgBIF_CFG_DEV0_EPF4_0_SLOT_CAP2: u32 = 0x0098;
pub const cfgBIF_CFG_DEV0_EPF4_0_SLOT_CNTL2: u32 = 0x009c;
pub const cfgBIF_CFG_DEV0_EPF4_0_SLOT_STATUS2: u32 = 0x009e;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSI_CAP_LIST: u32 = 0x00a0;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSI_MSG_DATA: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSI_MASK: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSI_MASK_64: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSI_PENDING: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSI_PENDING_64: u32 = 0x00b4;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSIX_CAP_LIST: u32 = 0x00c0;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSIX_MSG_CNTL: u32 = 0x00c2;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSIX_TABLE: u32 = 0x00c4;
pub const cfgBIF_CFG_DEV0_EPF4_0_MSIX_PBA: u32 = 0x00c8;
pub const cfgBIF_CFG_DEV0_EPF4_0_SATA_CAP_0: u32 = 0x00d0;
pub const cfgBIF_CFG_DEV0_EPF4_0_SATA_CAP_1: u32 = 0x00d4;
pub const cfgBIF_CFG_DEV0_EPF4_0_SATA_IDP_INDEX: u32 = 0x00d8;
pub const cfgBIF_CFG_DEV0_EPF4_0_SATA_IDP_DATA: u32 = 0x00dc;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_BAR_ENH_CAP_LIST: u32 = 0x0200;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_BAR1_CAP: u32 = 0x0204;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_BAR1_CNTL: u32 = 0x0208;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_BAR2_CAP: u32 = 0x020c;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_BAR2_CNTL: u32 = 0x0210;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_BAR3_CAP: u32 = 0x0214;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_BAR3_CNTL: u32 = 0x0218;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_BAR4_CAP: u32 = 0x021c;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_BAR4_CNTL: u32 = 0x0220;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_BAR5_CAP: u32 = 0x0224;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_BAR5_CNTL: u32 = 0x0228;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_BAR6_CAP: u32 = 0x022c;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_BAR6_CNTL: u32 = 0x0230;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_PWR_BUDGET_ENH_CAP_LIST: u32 = 0x0240;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_PWR_BUDGET_DATA_SELECT: u32 = 0x0244;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_PWR_BUDGET_DATA: u32 = 0x0248;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_PWR_BUDGET_CAP: u32 = 0x024c;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_DPA_ENH_CAP_LIST: u32 = 0x0250;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_DPA_CAP: u32 = 0x0254;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_DPA_LATENCY_INDICATOR: u32 = 0x0258;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_DPA_STATUS: u32 = 0x025c;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_DPA_CNTL: u32 = 0x025e;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_0: u32 = 0x0260;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_1: u32 = 0x0261;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_2: u32 = 0x0262;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_3: u32 = 0x0263;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_4: u32 = 0x0264;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_5: u32 = 0x0265;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_6: u32 = 0x0266;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_7: u32 = 0x0267;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_ACS_CNTL: u32 = 0x02a6;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_ARI_ENH_CAP_LIST: u32 = 0x0328;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_ARI_CAP: u32 = 0x032c;
pub const cfgBIF_CFG_DEV0_EPF4_0_PCIE_ARI_CNTL: u32 = 0x032e;


// addressBlock: nbio_nbif0_bif_cfg_dev0_epf5_bifcfgdecp
// base address: 0x0
pub const cfgBIF_CFG_DEV0_EPF5_0_VENDOR_ID: u32 = 0x0000;
pub const cfgBIF_CFG_DEV0_EPF5_0_DEVICE_ID: u32 = 0x0002;
pub const cfgBIF_CFG_DEV0_EPF5_0_COMMAND: u32 = 0x0004;
pub const cfgBIF_CFG_DEV0_EPF5_0_STATUS: u32 = 0x0006;
pub const cfgBIF_CFG_DEV0_EPF5_0_REVISION_ID: u32 = 0x0008;
pub const cfgBIF_CFG_DEV0_EPF5_0_PROG_INTERFACE: u32 = 0x0009;
pub const cfgBIF_CFG_DEV0_EPF5_0_SUB_CLASS: u32 = 0x000a;
pub const cfgBIF_CFG_DEV0_EPF5_0_BASE_CLASS: u32 = 0x000b;
pub const cfgBIF_CFG_DEV0_EPF5_0_CACHE_LINE: u32 = 0x000c;
pub const cfgBIF_CFG_DEV0_EPF5_0_LATENCY: u32 = 0x000d;
pub const cfgBIF_CFG_DEV0_EPF5_0_HEADER: u32 = 0x000e;
pub const cfgBIF_CFG_DEV0_EPF5_0_BIST: u32 = 0x000f;
pub const cfgBIF_CFG_DEV0_EPF5_0_BASE_ADDR_1: u32 = 0x0010;
pub const cfgBIF_CFG_DEV0_EPF5_0_BASE_ADDR_2: u32 = 0x0014;
pub const cfgBIF_CFG_DEV0_EPF5_0_BASE_ADDR_3: u32 = 0x0018;
pub const cfgBIF_CFG_DEV0_EPF5_0_BASE_ADDR_4: u32 = 0x001c;
pub const cfgBIF_CFG_DEV0_EPF5_0_BASE_ADDR_5: u32 = 0x0020;
pub const cfgBIF_CFG_DEV0_EPF5_0_BASE_ADDR_6: u32 = 0x0024;
pub const cfgBIF_CFG_DEV0_EPF5_0_ADAPTER_ID: u32 = 0x002c;
pub const cfgBIF_CFG_DEV0_EPF5_0_ROM_BASE_ADDR: u32 = 0x0030;
pub const cfgBIF_CFG_DEV0_EPF5_0_CAP_PTR: u32 = 0x0034;
pub const cfgBIF_CFG_DEV0_EPF5_0_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgBIF_CFG_DEV0_EPF5_0_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgBIF_CFG_DEV0_EPF5_0_MIN_GRANT: u32 = 0x003e;
pub const cfgBIF_CFG_DEV0_EPF5_0_MAX_LATENCY: u32 = 0x003f;
pub const cfgBIF_CFG_DEV0_EPF5_0_VENDOR_CAP_LIST: u32 = 0x0048;
pub const cfgBIF_CFG_DEV0_EPF5_0_ADAPTER_ID_W: u32 = 0x004c;
pub const cfgBIF_CFG_DEV0_EPF5_0_PMI_CAP_LIST: u32 = 0x0050;
pub const cfgBIF_CFG_DEV0_EPF5_0_PMI_CAP: u32 = 0x0052;
pub const cfgBIF_CFG_DEV0_EPF5_0_PMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgBIF_CFG_DEV0_EPF5_0_SBRN: u32 = 0x0060;
pub const cfgBIF_CFG_DEV0_EPF5_0_FLADJ: u32 = 0x0061;
pub const cfgBIF_CFG_DEV0_EPF5_0_DBESL_DBESLD: u32 = 0x0062;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_CAP_LIST: u32 = 0x0064;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_CAP: u32 = 0x0066;
pub const cfgBIF_CFG_DEV0_EPF5_0_DEVICE_CAP: u32 = 0x0068;
pub const cfgBIF_CFG_DEV0_EPF5_0_DEVICE_CNTL: u32 = 0x006c;
pub const cfgBIF_CFG_DEV0_EPF5_0_DEVICE_STATUS: u32 = 0x006e;
pub const cfgBIF_CFG_DEV0_EPF5_0_LINK_CAP: u32 = 0x0070;
pub const cfgBIF_CFG_DEV0_EPF5_0_LINK_CNTL: u32 = 0x0074;
pub const cfgBIF_CFG_DEV0_EPF5_0_LINK_STATUS: u32 = 0x0076;
pub const cfgBIF_CFG_DEV0_EPF5_0_DEVICE_CAP2: u32 = 0x0088;
pub const cfgBIF_CFG_DEV0_EPF5_0_DEVICE_CNTL2: u32 = 0x008c;
pub const cfgBIF_CFG_DEV0_EPF5_0_DEVICE_STATUS2: u32 = 0x008e;
pub const cfgBIF_CFG_DEV0_EPF5_0_LINK_CAP2: u32 = 0x0090;
pub const cfgBIF_CFG_DEV0_EPF5_0_LINK_CNTL2: u32 = 0x0094;
pub const cfgBIF_CFG_DEV0_EPF5_0_LINK_STATUS2: u32 = 0x0096;
pub const cfgBIF_CFG_DEV0_EPF5_0_SLOT_CAP2: u32 = 0x0098;
pub const cfgBIF_CFG_DEV0_EPF5_0_SLOT_CNTL2: u32 = 0x009c;
pub const cfgBIF_CFG_DEV0_EPF5_0_SLOT_STATUS2: u32 = 0x009e;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSI_CAP_LIST: u32 = 0x00a0;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSI_MSG_DATA: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSI_MASK: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSI_MASK_64: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSI_PENDING: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSI_PENDING_64: u32 = 0x00b4;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSIX_CAP_LIST: u32 = 0x00c0;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSIX_MSG_CNTL: u32 = 0x00c2;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSIX_TABLE: u32 = 0x00c4;
pub const cfgBIF_CFG_DEV0_EPF5_0_MSIX_PBA: u32 = 0x00c8;
pub const cfgBIF_CFG_DEV0_EPF5_0_SATA_CAP_0: u32 = 0x00d0;
pub const cfgBIF_CFG_DEV0_EPF5_0_SATA_CAP_1: u32 = 0x00d4;
pub const cfgBIF_CFG_DEV0_EPF5_0_SATA_IDP_INDEX: u32 = 0x00d8;
pub const cfgBIF_CFG_DEV0_EPF5_0_SATA_IDP_DATA: u32 = 0x00dc;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_BAR_ENH_CAP_LIST: u32 = 0x0200;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_BAR1_CAP: u32 = 0x0204;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_BAR1_CNTL: u32 = 0x0208;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_BAR2_CAP: u32 = 0x020c;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_BAR2_CNTL: u32 = 0x0210;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_BAR3_CAP: u32 = 0x0214;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_BAR3_CNTL: u32 = 0x0218;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_BAR4_CAP: u32 = 0x021c;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_BAR4_CNTL: u32 = 0x0220;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_BAR5_CAP: u32 = 0x0224;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_BAR5_CNTL: u32 = 0x0228;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_BAR6_CAP: u32 = 0x022c;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_BAR6_CNTL: u32 = 0x0230;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_PWR_BUDGET_ENH_CAP_LIST: u32 = 0x0240;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_PWR_BUDGET_DATA_SELECT: u32 = 0x0244;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_PWR_BUDGET_DATA: u32 = 0x0248;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_PWR_BUDGET_CAP: u32 = 0x024c;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_DPA_ENH_CAP_LIST: u32 = 0x0250;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_DPA_CAP: u32 = 0x0254;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_DPA_LATENCY_INDICATOR: u32 = 0x0258;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_DPA_STATUS: u32 = 0x025c;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_DPA_CNTL: u32 = 0x025e;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_0: u32 = 0x0260;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_1: u32 = 0x0261;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_2: u32 = 0x0262;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_3: u32 = 0x0263;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_4: u32 = 0x0264;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_5: u32 = 0x0265;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_6: u32 = 0x0266;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_7: u32 = 0x0267;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_ACS_CNTL: u32 = 0x02a6;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_ARI_ENH_CAP_LIST: u32 = 0x0328;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_ARI_CAP: u32 = 0x032c;
pub const cfgBIF_CFG_DEV0_EPF5_0_PCIE_ARI_CNTL: u32 = 0x032e;


// addressBlock: nbio_nbif0_bif_cfg_dev0_epf6_bifcfgdecp
// base address: 0x0
pub const cfgBIF_CFG_DEV0_EPF6_0_VENDOR_ID: u32 = 0x0000;
pub const cfgBIF_CFG_DEV0_EPF6_0_DEVICE_ID: u32 = 0x0002;
pub const cfgBIF_CFG_DEV0_EPF6_0_COMMAND: u32 = 0x0004;
pub const cfgBIF_CFG_DEV0_EPF6_0_STATUS: u32 = 0x0006;
pub const cfgBIF_CFG_DEV0_EPF6_0_REVISION_ID: u32 = 0x0008;
pub const cfgBIF_CFG_DEV0_EPF6_0_PROG_INTERFACE: u32 = 0x0009;
pub const cfgBIF_CFG_DEV0_EPF6_0_SUB_CLASS: u32 = 0x000a;
pub const cfgBIF_CFG_DEV0_EPF6_0_BASE_CLASS: u32 = 0x000b;
pub const cfgBIF_CFG_DEV0_EPF6_0_CACHE_LINE: u32 = 0x000c;
pub const cfgBIF_CFG_DEV0_EPF6_0_LATENCY: u32 = 0x000d;
pub const cfgBIF_CFG_DEV0_EPF6_0_HEADER: u32 = 0x000e;
pub const cfgBIF_CFG_DEV0_EPF6_0_BIST: u32 = 0x000f;
pub const cfgBIF_CFG_DEV0_EPF6_0_BASE_ADDR_1: u32 = 0x0010;
pub const cfgBIF_CFG_DEV0_EPF6_0_BASE_ADDR_2: u32 = 0x0014;
pub const cfgBIF_CFG_DEV0_EPF6_0_BASE_ADDR_3: u32 = 0x0018;
pub const cfgBIF_CFG_DEV0_EPF6_0_BASE_ADDR_4: u32 = 0x001c;
pub const cfgBIF_CFG_DEV0_EPF6_0_BASE_ADDR_5: u32 = 0x0020;
pub const cfgBIF_CFG_DEV0_EPF6_0_BASE_ADDR_6: u32 = 0x0024;
pub const cfgBIF_CFG_DEV0_EPF6_0_ADAPTER_ID: u32 = 0x002c;
pub const cfgBIF_CFG_DEV0_EPF6_0_ROM_BASE_ADDR: u32 = 0x0030;
pub const cfgBIF_CFG_DEV0_EPF6_0_CAP_PTR: u32 = 0x0034;
pub const cfgBIF_CFG_DEV0_EPF6_0_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgBIF_CFG_DEV0_EPF6_0_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgBIF_CFG_DEV0_EPF6_0_MIN_GRANT: u32 = 0x003e;
pub const cfgBIF_CFG_DEV0_EPF6_0_MAX_LATENCY: u32 = 0x003f;
pub const cfgBIF_CFG_DEV0_EPF6_0_VENDOR_CAP_LIST: u32 = 0x0048;
pub const cfgBIF_CFG_DEV0_EPF6_0_ADAPTER_ID_W: u32 = 0x004c;
pub const cfgBIF_CFG_DEV0_EPF6_0_PMI_CAP_LIST: u32 = 0x0050;
pub const cfgBIF_CFG_DEV0_EPF6_0_PMI_CAP: u32 = 0x0052;
pub const cfgBIF_CFG_DEV0_EPF6_0_PMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgBIF_CFG_DEV0_EPF6_0_SBRN: u32 = 0x0060;
pub const cfgBIF_CFG_DEV0_EPF6_0_FLADJ: u32 = 0x0061;
pub const cfgBIF_CFG_DEV0_EPF6_0_DBESL_DBESLD: u32 = 0x0062;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_CAP_LIST: u32 = 0x0064;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_CAP: u32 = 0x0066;
pub const cfgBIF_CFG_DEV0_EPF6_0_DEVICE_CAP: u32 = 0x0068;
pub const cfgBIF_CFG_DEV0_EPF6_0_DEVICE_CNTL: u32 = 0x006c;
pub const cfgBIF_CFG_DEV0_EPF6_0_DEVICE_STATUS: u32 = 0x006e;
pub const cfgBIF_CFG_DEV0_EPF6_0_LINK_CAP: u32 = 0x0070;
pub const cfgBIF_CFG_DEV0_EPF6_0_LINK_CNTL: u32 = 0x0074;
pub const cfgBIF_CFG_DEV0_EPF6_0_LINK_STATUS: u32 = 0x0076;
pub const cfgBIF_CFG_DEV0_EPF6_0_DEVICE_CAP2: u32 = 0x0088;
pub const cfgBIF_CFG_DEV0_EPF6_0_DEVICE_CNTL2: u32 = 0x008c;
pub const cfgBIF_CFG_DEV0_EPF6_0_DEVICE_STATUS2: u32 = 0x008e;
pub const cfgBIF_CFG_DEV0_EPF6_0_LINK_CAP2: u32 = 0x0090;
pub const cfgBIF_CFG_DEV0_EPF6_0_LINK_CNTL2: u32 = 0x0094;
pub const cfgBIF_CFG_DEV0_EPF6_0_LINK_STATUS2: u32 = 0x0096;
pub const cfgBIF_CFG_DEV0_EPF6_0_SLOT_CAP2: u32 = 0x0098;
pub const cfgBIF_CFG_DEV0_EPF6_0_SLOT_CNTL2: u32 = 0x009c;
pub const cfgBIF_CFG_DEV0_EPF6_0_SLOT_STATUS2: u32 = 0x009e;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSI_CAP_LIST: u32 = 0x00a0;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSI_MSG_DATA: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSI_MASK: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSI_MASK_64: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSI_PENDING: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSI_PENDING_64: u32 = 0x00b4;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSIX_CAP_LIST: u32 = 0x00c0;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSIX_MSG_CNTL: u32 = 0x00c2;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSIX_TABLE: u32 = 0x00c4;
pub const cfgBIF_CFG_DEV0_EPF6_0_MSIX_PBA: u32 = 0x00c8;
pub const cfgBIF_CFG_DEV0_EPF6_0_SATA_CAP_0: u32 = 0x00d0;
pub const cfgBIF_CFG_DEV0_EPF6_0_SATA_CAP_1: u32 = 0x00d4;
pub const cfgBIF_CFG_DEV0_EPF6_0_SATA_IDP_INDEX: u32 = 0x00d8;
pub const cfgBIF_CFG_DEV0_EPF6_0_SATA_IDP_DATA: u32 = 0x00dc;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_BAR_ENH_CAP_LIST: u32 = 0x0200;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_BAR1_CAP: u32 = 0x0204;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_BAR1_CNTL: u32 = 0x0208;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_BAR2_CAP: u32 = 0x020c;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_BAR2_CNTL: u32 = 0x0210;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_BAR3_CAP: u32 = 0x0214;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_BAR3_CNTL: u32 = 0x0218;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_BAR4_CAP: u32 = 0x021c;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_BAR4_CNTL: u32 = 0x0220;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_BAR5_CAP: u32 = 0x0224;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_BAR5_CNTL: u32 = 0x0228;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_BAR6_CAP: u32 = 0x022c;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_BAR6_CNTL: u32 = 0x0230;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_PWR_BUDGET_ENH_CAP_LIST: u32 = 0x0240;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_PWR_BUDGET_DATA_SELECT: u32 = 0x0244;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_PWR_BUDGET_DATA: u32 = 0x0248;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_PWR_BUDGET_CAP: u32 = 0x024c;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_DPA_ENH_CAP_LIST: u32 = 0x0250;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_DPA_CAP: u32 = 0x0254;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_DPA_LATENCY_INDICATOR: u32 = 0x0258;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_DPA_STATUS: u32 = 0x025c;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_DPA_CNTL: u32 = 0x025e;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_0: u32 = 0x0260;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_1: u32 = 0x0261;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_2: u32 = 0x0262;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_3: u32 = 0x0263;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_4: u32 = 0x0264;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_5: u32 = 0x0265;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_6: u32 = 0x0266;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_DPA_SUBSTATE_PWR_ALLOC_7: u32 = 0x0267;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_ACS_CNTL: u32 = 0x02a6;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_ARI_ENH_CAP_LIST: u32 = 0x0328;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_ARI_CAP: u32 = 0x032c;
pub const cfgBIF_CFG_DEV0_EPF6_0_PCIE_ARI_CNTL: u32 = 0x032e;


// addressBlock: nbio_nbif0_bif_cfg_dev0_epf7_bifcfgdecp
// base address: 0x0
pub const cfgBIF_CFG_DEV0_EPF7_0_VENDOR_ID: u32 = 0x0000;
pub const cfgBIF_CFG_DEV0_EPF7_0_DEVICE_ID: u32 = 0x0002;
pub const cfgBIF_CFG_DEV0_EPF7_0_COMMAND: u32 = 0x0004;
pub const cfgBIF_CFG_DEV0_EPF7_0_STATUS: u32 = 0x0006;
pub const cfgBIF_CFG_DEV0_EPF7_0_REVISION_ID: u32 = 0x0008;
pub const cfgBIF_CFG_DEV0_EPF7_0_PROG_INTERFACE: u32 = 0x0009;
pub const cfgBIF_CFG_DEV0_EPF7_0_SUB_CLASS: u32 = 0x000a;
pub const cfgBIF_CFG_DEV0_EPF7_0_BASE_CLASS: u32 = 0x000b;
pub const cfgBIF_CFG_DEV0_EPF7_0_CACHE_LINE: u32 = 0x000c;
pub const cfgBIF_CFG_DEV0_EPF7_0_LATENCY: u32 = 0x000d;
pub const cfgBIF_CFG_DEV0_EPF7_0_HEADER: u32 = 0x000e;
pub const cfgBIF_CFG_DEV0_EPF7_0_BIST: u32 = 0x000f;
pub const cfgBIF_CFG_DEV0_EPF7_0_BASE_ADDR_1: u32 = 0x0010;
pub const cfgBIF_CFG_DEV0_EPF7_0_BASE_ADDR_2: u32 = 0x0014;
pub const cfgBIF_CFG_DEV0_EPF7_0_BASE_ADDR_3: u32 = 0x0018;
pub const cfgBIF_CFG_DEV0_EPF7_0_BASE_ADDR_4: u32 = 0x001c;
pub const cfgBIF_CFG_DEV0_EPF7_0_BASE_ADDR_5: u32 = 0x0020;
pub const cfgBIF_CFG_DEV0_EPF7_0_BASE_ADDR_6: u32 = 0x0024;
pub const cfgBIF_CFG_DEV0_EPF7_0_ADAPTER_ID: u32 = 0x002c;
pub const cfgBIF_CFG_DEV0_EPF7_0_ROM_BASE_ADDR: u32 = 0x0030;
pub const cfgBIF_CFG_DEV0_EPF7_0_CAP_PTR: u32 = 0x0034;
pub const cfgBIF_CFG_DEV0_EPF7_0_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgBIF_CFG_DEV0_EPF7_0_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgBIF_CFG_DEV0_EPF7_0_MIN_GRANT: u32 = 0x003e;
pub const cfgBIF_CFG_DEV0_EPF7_0_MAX_LATENCY: u32 = 0x003f;
pub const cfgBIF_CFG_DEV0_EPF7_0_VENDOR_CAP_LIST: u32 = 0x0048;
pub const cfgBIF_CFG_DEV0_EPF7_0_ADAPTER_ID_W: u32 = 0x004c;
pub const cfgBIF_CFG_DEV0_EPF7_0_PMI_CAP_LIST: u32 = 0x0050;
pub const cfgBIF_CFG_DEV0_EPF7_0_PMI_CAP: u32 = 0x0052;
pub const cfgBIF_CFG_DEV0_EPF7_0_PMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgBIF_CFG_DEV0_EPF7_0_SBRN: u32 = 0x0060;
pub const cfgBIF_CFG_DEV0_EPF7_0_FLADJ: u32 = 0x0061;
pub const cfgBIF_CFG_DEV0_EPF7_0_DBESL_DBESLD: u32 = 0x0062;
pub const cfgBIF_CFG_DEV0_EPF7_0_PCIE_CAP_LIST: u32 = 0x0064;
pub const cfgBIF_CFG_DEV0_EPF7_0_PCIE_CAP: u32 = 0x0066;
pub const cfgBIF_CFG_DEV0_EPF7_0_DEVICE_CAP: u32 = 0x0068;
pub const cfgBIF_CFG_DEV0_EPF7_0_DEVICE_CNTL: u32 = 0x006c;
pub const cfgBIF_CFG_DEV0_EPF7_0_DEVICE_STATUS: u32 = 0x006e;
pub const cfgBIF_CFG_DEV0_EPF7_0_LINK_CAP: u32 = 0x0070;
pub const cfgBIF_CFG_DEV0_EPF7_0_LINK_CNTL: u32 = 0x0074;
pub const cfgBIF_CFG_DEV0_EPF7_0_LINK_STATUS: u32 = 0x0076;
pub const cfgBIF_CFG_DEV0_EPF7_0_DEVICE_CAP2: u32 = 0x0088;
pub const cfgBIF_CFG_DEV0_EPF7_0_DEVICE_CNTL2: u32 = 0x008c;
pub const cfgBIF_CFG_DEV0_EPF7_0_DEVICE_STATUS2: u32 = 0x008e;
pub const cfgBIF_CFG_DEV0_EPF7_0_LINK_CAP2: u32 = 0x0090;
pub const cfgBIF_CFG_DEV0_EPF7_0_LINK_CNTL2: u32 = 0x0094;
pub const cfgBIF_CFG_DEV0_EPF7_0_LINK_STATUS2: u32 = 0x0096;
pub const cfgBIF_CFG_DEV0_EPF7_0_SLOT_CAP2: u32 = 0x0098;
pub const cfgBIF_CFG_DEV0_EPF7_0_SLOT_CNTL2: u32 = 0x009c;
pub const cfgBIF_CFG_DEV0_EPF7_0_SLOT_STATUS2: u32 = 0x009e;
pub const cfgBIF_CFG_DEV0_EPF7_0_MSI_CAP_LIST: u32 = 0x00a0;
pub const cfgBIF_CFG_DEV0_EPF7_0_MSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgBIF_CFG_DEV0_EPF7_0_MSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgBIF_CFG_DEV0_EPF7_0_MSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF7_0_MSI_MSG_DATA: u32 = 0x00a8;
pub const cfgBIF_CFG_DEV0_EPF7_0_MSI_MASK: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF7_0_MSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgBIF_CFG_DEV0_EPF7_0_MSI_MASK_64: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF7_0_MSI_PENDING: u32 = 0x00b0;
pub const cfgBIF_CFG_DEV0_EPF7_0_MSI_PENDING_64: u32 = 0x00b4;
pub const cfgBIF_CFG_DEV0_EPF7_0_MSIX_CAP_LIST: u32 = 0x00c0;
pub const cfgBIF_CFG_DEV0_EPF7_0_MSIX_MSG_CNTL: u32 = 0x00c2;
#define cfgBIF_CFG_DEV0_EPF7_0_MSIX_TABLE                                …23946 tokens truncated…                                      0x00d0
pub const cfgBIFPLR2_0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgBIFPLR2_0_PCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgBIFPLR2_0_PCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgBIFPLR2_0_PCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgBIFPLR2_0_PCIE_VC_ENH_CAP_LIST: u32 = 0x0110;
pub const cfgBIFPLR2_0_PCIE_PORT_VC_CAP_REG1: u32 = 0x0114;
pub const cfgBIFPLR2_0_PCIE_PORT_VC_CAP_REG2: u32 = 0x0118;
pub const cfgBIFPLR2_0_PCIE_PORT_VC_CNTL: u32 = 0x011c;
pub const cfgBIFPLR2_0_PCIE_PORT_VC_STATUS: u32 = 0x011e;
pub const cfgBIFPLR2_0_PCIE_VC0_RESOURCE_CAP: u32 = 0x0120;
pub const cfgBIFPLR2_0_PCIE_VC0_RESOURCE_CNTL: u32 = 0x0124;
pub const cfgBIFPLR2_0_PCIE_VC0_RESOURCE_STATUS: u32 = 0x012a;
pub const cfgBIFPLR2_0_PCIE_VC1_RESOURCE_CAP: u32 = 0x012c;
pub const cfgBIFPLR2_0_PCIE_VC1_RESOURCE_CNTL: u32 = 0x0130;
pub const cfgBIFPLR2_0_PCIE_VC1_RESOURCE_STATUS: u32 = 0x0136;
pub const cfgBIFPLR2_0_PCIE_DEV_SERIAL_NUM_ENH_CAP_LIST: u32 = 0x0140;
pub const cfgBIFPLR2_0_PCIE_DEV_SERIAL_NUM_DW1: u32 = 0x0144;
pub const cfgBIFPLR2_0_PCIE_DEV_SERIAL_NUM_DW2: u32 = 0x0148;
pub const cfgBIFPLR2_0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgBIFPLR2_0_PCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgBIFPLR2_0_PCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgBIFPLR2_0_PCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgBIFPLR2_0_PCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgBIFPLR2_0_PCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgBIFPLR2_0_PCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgBIFPLR2_0_PCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgBIFPLR2_0_PCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgBIFPLR2_0_PCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgBIFPLR2_0_PCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgBIFPLR2_0_PCIE_ROOT_ERR_CMD: u32 = 0x017c;
pub const cfgBIFPLR2_0_PCIE_ROOT_ERR_STATUS: u32 = 0x0180;
pub const cfgBIFPLR2_0_PCIE_ERR_SRC_ID: u32 = 0x0184;
pub const cfgBIFPLR2_0_PCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgBIFPLR2_0_PCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgBIFPLR2_0_PCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgBIFPLR2_0_PCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgBIFPLR2_0_PCIE_SECONDARY_ENH_CAP_LIST: u32 = 0x0270;
pub const cfgBIFPLR2_0_PCIE_LINK_CNTL3: u32 = 0x0274;
pub const cfgBIFPLR2_0_PCIE_LANE_ERROR_STATUS: u32 = 0x0278;
pub const cfgBIFPLR2_0_PCIE_LANE_0_EQUALIZATION_CNTL: u32 = 0x027c;
pub const cfgBIFPLR2_0_PCIE_LANE_1_EQUALIZATION_CNTL: u32 = 0x027e;
pub const cfgBIFPLR2_0_PCIE_LANE_2_EQUALIZATION_CNTL: u32 = 0x0280;
pub const cfgBIFPLR2_0_PCIE_LANE_3_EQUALIZATION_CNTL: u32 = 0x0282;
pub const cfgBIFPLR2_0_PCIE_LANE_4_EQUALIZATION_CNTL: u32 = 0x0284;
pub const cfgBIFPLR2_0_PCIE_LANE_5_EQUALIZATION_CNTL: u32 = 0x0286;
pub const cfgBIFPLR2_0_PCIE_LANE_6_EQUALIZATION_CNTL: u32 = 0x0288;
pub const cfgBIFPLR2_0_PCIE_LANE_7_EQUALIZATION_CNTL: u32 = 0x028a;
pub const cfgBIFPLR2_0_PCIE_LANE_8_EQUALIZATION_CNTL: u32 = 0x028c;
pub const cfgBIFPLR2_0_PCIE_LANE_9_EQUALIZATION_CNTL: u32 = 0x028e;
pub const cfgBIFPLR2_0_PCIE_LANE_10_EQUALIZATION_CNTL: u32 = 0x0290;
pub const cfgBIFPLR2_0_PCIE_LANE_11_EQUALIZATION_CNTL: u32 = 0x0292;
pub const cfgBIFPLR2_0_PCIE_LANE_12_EQUALIZATION_CNTL: u32 = 0x0294;
pub const cfgBIFPLR2_0_PCIE_LANE_13_EQUALIZATION_CNTL: u32 = 0x0296;
pub const cfgBIFPLR2_0_PCIE_LANE_14_EQUALIZATION_CNTL: u32 = 0x0298;
pub const cfgBIFPLR2_0_PCIE_LANE_15_EQUALIZATION_CNTL: u32 = 0x029a;
pub const cfgBIFPLR2_0_PCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgBIFPLR2_0_PCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgBIFPLR2_0_PCIE_ACS_CNTL: u32 = 0x02a6;
pub const cfgBIFPLR2_0_PCIE_MC_ENH_CAP_LIST: u32 = 0x02f0;
pub const cfgBIFPLR2_0_PCIE_MC_CAP: u32 = 0x02f4;
pub const cfgBIFPLR2_0_PCIE_MC_CNTL: u32 = 0x02f6;
pub const cfgBIFPLR2_0_PCIE_MC_ADDR0: u32 = 0x02f8;
pub const cfgBIFPLR2_0_PCIE_MC_ADDR1: u32 = 0x02fc;
pub const cfgBIFPLR2_0_PCIE_MC_RCV0: u32 = 0x0300;
pub const cfgBIFPLR2_0_PCIE_MC_RCV1: u32 = 0x0304;
pub const cfgBIFPLR2_0_PCIE_MC_BLOCK_ALL0: u32 = 0x0308;
pub const cfgBIFPLR2_0_PCIE_MC_BLOCK_ALL1: u32 = 0x030c;
pub const cfgBIFPLR2_0_PCIE_MC_BLOCK_UNTRANSLATED_0: u32 = 0x0310;
pub const cfgBIFPLR2_0_PCIE_MC_BLOCK_UNTRANSLATED_1: u32 = 0x0314;
pub const cfgBIFPLR2_0_PCIE_MC_OVERLAY_BAR0: u32 = 0x0318;
pub const cfgBIFPLR2_0_PCIE_MC_OVERLAY_BAR1: u32 = 0x031c;
pub const cfgBIFPLR2_0_PCIE_L1_PM_SUB_CAP_LIST: u32 = 0x0370;
pub const cfgBIFPLR2_0_PCIE_L1_PM_SUB_CAP: u32 = 0x0374;
pub const cfgBIFPLR2_0_PCIE_L1_PM_SUB_CNTL: u32 = 0x0378;
pub const cfgBIFPLR2_0_PCIE_L1_PM_SUB_CNTL2: u32 = 0x037c;
pub const cfgBIFPLR2_0_PCIE_DPC_ENH_CAP_LIST: u32 = 0x0380;
pub const cfgBIFPLR2_0_PCIE_DPC_CAP_LIST: u32 = 0x0384;
pub const cfgBIFPLR2_0_PCIE_DPC_CNTL: u32 = 0x0386;
pub const cfgBIFPLR2_0_PCIE_DPC_STATUS: u32 = 0x0388;
pub const cfgBIFPLR2_0_PCIE_DPC_ERROR_SOURCE_ID: u32 = 0x038a;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_STATUS: u32 = 0x038c;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_MASK: u32 = 0x0390;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_SEVERITY: u32 = 0x0394;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_SYSERROR: u32 = 0x0398;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_EXCEPTION: u32 = 0x039c;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_HDR_LOG0: u32 = 0x03a0;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_HDR_LOG1: u32 = 0x03a4;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_HDR_LOG2: u32 = 0x03a8;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_HDR_LOG3: u32 = 0x03ac;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_IMPSPEC_LOG: u32 = 0x03b0;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_PREFIX_LOG0: u32 = 0x03b4;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_PREFIX_LOG1: u32 = 0x03b8;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_PREFIX_LOG2: u32 = 0x03bc;
pub const cfgBIFPLR2_0_PCIE_RP_PIO_PREFIX_LOG3: u32 = 0x03c0;
pub const cfgBIFPLR2_0_PCIE_ESM_CAP_LIST: u32 = 0x03c4;
pub const cfgBIFPLR2_0_PCIE_ESM_HEADER_1: u32 = 0x03c8;
pub const cfgBIFPLR2_0_PCIE_ESM_HEADER_2: u32 = 0x03cc;
pub const cfgBIFPLR2_0_PCIE_ESM_STATUS: u32 = 0x03ce;
pub const cfgBIFPLR2_0_PCIE_ESM_CTRL: u32 = 0x03d0;
pub const cfgBIFPLR2_0_PCIE_ESM_CAP_1: u32 = 0x03d4;
pub const cfgBIFPLR2_0_PCIE_ESM_CAP_2: u32 = 0x03d8;
pub const cfgBIFPLR2_0_PCIE_ESM_CAP_3: u32 = 0x03dc;
pub const cfgBIFPLR2_0_PCIE_ESM_CAP_4: u32 = 0x03e0;
pub const cfgBIFPLR2_0_PCIE_ESM_CAP_5: u32 = 0x03e4;
pub const cfgBIFPLR2_0_PCIE_ESM_CAP_6: u32 = 0x03e8;
pub const cfgBIFPLR2_0_PCIE_ESM_CAP_7: u32 = 0x03ec;


// addressBlock: nbio_pcie0_bifplr3_cfgdecp
// base address: 0x0
pub const cfgBIFPLR3_0_VENDOR_ID: u32 = 0x0000;
pub const cfgBIFPLR3_0_DEVICE_ID: u32 = 0x0002;
pub const cfgBIFPLR3_0_COMMAND: u32 = 0x0004;
pub const cfgBIFPLR3_0_STATUS: u32 = 0x0006;
pub const cfgBIFPLR3_0_REVISION_ID: u32 = 0x0008;
pub const cfgBIFPLR3_0_PROG_INTERFACE: u32 = 0x0009;
pub const cfgBIFPLR3_0_SUB_CLASS: u32 = 0x000a;
pub const cfgBIFPLR3_0_BASE_CLASS: u32 = 0x000b;
pub const cfgBIFPLR3_0_CACHE_LINE: u32 = 0x000c;
pub const cfgBIFPLR3_0_LATENCY: u32 = 0x000d;
pub const cfgBIFPLR3_0_HEADER: u32 = 0x000e;
pub const cfgBIFPLR3_0_BIST: u32 = 0x000f;
pub const cfgBIFPLR3_0_SUB_BUS_NUMBER_LATENCY: u32 = 0x0018;
pub const cfgBIFPLR3_0_IO_BASE_LIMIT: u32 = 0x001c;
pub const cfgBIFPLR3_0_SECONDARY_STATUS: u32 = 0x001e;
pub const cfgBIFPLR3_0_MEM_BASE_LIMIT: u32 = 0x0020;
pub const cfgBIFPLR3_0_PREF_BASE_LIMIT: u32 = 0x0024;
pub const cfgBIFPLR3_0_PREF_BASE_UPPER: u32 = 0x0028;
pub const cfgBIFPLR3_0_PREF_LIMIT_UPPER: u32 = 0x002c;
pub const cfgBIFPLR3_0_IO_BASE_LIMIT_HI: u32 = 0x0030;
pub const cfgBIFPLR3_0_CAP_PTR: u32 = 0x0034;
pub const cfgBIFPLR3_0_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgBIFPLR3_0_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgBIFPLR3_0_IRQ_BRIDGE_CNTL: u32 = 0x003e;
pub const cfgBIFPLR3_0_EXT_BRIDGE_CNTL: u32 = 0x0040;
pub const cfgBIFPLR3_0_PMI_CAP_LIST: u32 = 0x0050;
pub const cfgBIFPLR3_0_PMI_CAP: u32 = 0x0052;
pub const cfgBIFPLR3_0_PMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgBIFPLR3_0_PCIE_CAP_LIST: u32 = 0x0058;
pub const cfgBIFPLR3_0_PCIE_CAP: u32 = 0x005a;
pub const cfgBIFPLR3_0_DEVICE_CAP: u32 = 0x005c;
pub const cfgBIFPLR3_0_DEVICE_CNTL: u32 = 0x0060;
pub const cfgBIFPLR3_0_DEVICE_STATUS: u32 = 0x0062;
pub const cfgBIFPLR3_0_LINK_CAP: u32 = 0x0064;
pub const cfgBIFPLR3_0_LINK_CNTL: u32 = 0x0068;
pub const cfgBIFPLR3_0_LINK_STATUS: u32 = 0x006a;
pub const cfgBIFPLR3_0_SLOT_CAP: u32 = 0x006c;
pub const cfgBIFPLR3_0_SLOT_CNTL: u32 = 0x0070;
pub const cfgBIFPLR3_0_SLOT_STATUS: u32 = 0x0072;
pub const cfgBIFPLR3_0_ROOT_CNTL: u32 = 0x0074;
pub const cfgBIFPLR3_0_ROOT_CAP: u32 = 0x0076;
pub const cfgBIFPLR3_0_ROOT_STATUS: u32 = 0x0078;
pub const cfgBIFPLR3_0_DEVICE_CAP2: u32 = 0x007c;
pub const cfgBIFPLR3_0_DEVICE_CNTL2: u32 = 0x0080;
pub const cfgBIFPLR3_0_DEVICE_STATUS2: u32 = 0x0082;
pub const cfgBIFPLR3_0_LINK_CAP2: u32 = 0x0084;
pub const cfgBIFPLR3_0_LINK_CNTL2: u32 = 0x0088;
pub const cfgBIFPLR3_0_LINK_STATUS2: u32 = 0x008a;
pub const cfgBIFPLR3_0_SLOT_CAP2: u32 = 0x008c;
pub const cfgBIFPLR3_0_SLOT_CNTL2: u32 = 0x0090;
pub const cfgBIFPLR3_0_SLOT_STATUS2: u32 = 0x0092;
pub const cfgBIFPLR3_0_MSI_CAP_LIST: u32 = 0x00a0;
pub const cfgBIFPLR3_0_MSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgBIFPLR3_0_MSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgBIFPLR3_0_MSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgBIFPLR3_0_MSI_MSG_DATA: u32 = 0x00a8;
pub const cfgBIFPLR3_0_MSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgBIFPLR3_0_SSID_CAP_LIST: u32 = 0x00c0;
pub const cfgBIFPLR3_0_SSID_CAP: u32 = 0x00c4;
pub const cfgBIFPLR3_0_MSI_MAP_CAP_LIST: u32 = 0x00c8;
pub const cfgBIFPLR3_0_MSI_MAP_CAP: u32 = 0x00ca;
pub const cfgBIFPLR3_0_MSI_MAP_ADDR_LO: u32 = 0x00cc;
pub const cfgBIFPLR3_0_MSI_MAP_ADDR_HI: u32 = 0x00d0;
pub const cfgBIFPLR3_0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgBIFPLR3_0_PCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgBIFPLR3_0_PCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgBIFPLR3_0_PCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgBIFPLR3_0_PCIE_VC_ENH_CAP_LIST: u32 = 0x0110;
pub const cfgBIFPLR3_0_PCIE_PORT_VC_CAP_REG1: u32 = 0x0114;
pub const cfgBIFPLR3_0_PCIE_PORT_VC_CAP_REG2: u32 = 0x0118;
pub const cfgBIFPLR3_0_PCIE_PORT_VC_CNTL: u32 = 0x011c;
pub const cfgBIFPLR3_0_PCIE_PORT_VC_STATUS: u32 = 0x011e;
pub const cfgBIFPLR3_0_PCIE_VC0_RESOURCE_CAP: u32 = 0x0120;
pub const cfgBIFPLR3_0_PCIE_VC0_RESOURCE_CNTL: u32 = 0x0124;
pub const cfgBIFPLR3_0_PCIE_VC0_RESOURCE_STATUS: u32 = 0x012a;
pub const cfgBIFPLR3_0_PCIE_VC1_RESOURCE_CAP: u32 = 0x012c;
pub const cfgBIFPLR3_0_PCIE_VC1_RESOURCE_CNTL: u32 = 0x0130;
pub const cfgBIFPLR3_0_PCIE_VC1_RESOURCE_STATUS: u32 = 0x0136;
pub const cfgBIFPLR3_0_PCIE_DEV_SERIAL_NUM_ENH_CAP_LIST: u32 = 0x0140;
pub const cfgBIFPLR3_0_PCIE_DEV_SERIAL_NUM_DW1: u32 = 0x0144;
pub const cfgBIFPLR3_0_PCIE_DEV_SERIAL_NUM_DW2: u32 = 0x0148;
pub const cfgBIFPLR3_0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgBIFPLR3_0_PCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgBIFPLR3_0_PCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgBIFPLR3_0_PCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgBIFPLR3_0_PCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgBIFPLR3_0_PCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgBIFPLR3_0_PCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgBIFPLR3_0_PCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgBIFPLR3_0_PCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgBIFPLR3_0_PCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgBIFPLR3_0_PCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgBIFPLR3_0_PCIE_ROOT_ERR_CMD: u32 = 0x017c;
pub const cfgBIFPLR3_0_PCIE_ROOT_ERR_STATUS: u32 = 0x0180;
pub const cfgBIFPLR3_0_PCIE_ERR_SRC_ID: u32 = 0x0184;
pub const cfgBIFPLR3_0_PCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgBIFPLR3_0_PCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgBIFPLR3_0_PCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgBIFPLR3_0_PCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgBIFPLR3_0_PCIE_SECONDARY_ENH_CAP_LIST: u32 = 0x0270;
pub const cfgBIFPLR3_0_PCIE_LINK_CNTL3: u32 = 0x0274;
pub const cfgBIFPLR3_0_PCIE_LANE_ERROR_STATUS: u32 = 0x0278;
pub const cfgBIFPLR3_0_PCIE_LANE_0_EQUALIZATION_CNTL: u32 = 0x027c;
pub const cfgBIFPLR3_0_PCIE_LANE_1_EQUALIZATION_CNTL: u32 = 0x027e;
pub const cfgBIFPLR3_0_PCIE_LANE_2_EQUALIZATION_CNTL: u32 = 0x0280;
pub const cfgBIFPLR3_0_PCIE_LANE_3_EQUALIZATION_CNTL: u32 = 0x0282;
pub const cfgBIFPLR3_0_PCIE_LANE_4_EQUALIZATION_CNTL: u32 = 0x0284;
pub const cfgBIFPLR3_0_PCIE_LANE_5_EQUALIZATION_CNTL: u32 = 0x0286;
pub const cfgBIFPLR3_0_PCIE_LANE_6_EQUALIZATION_CNTL: u32 = 0x0288;
pub const cfgBIFPLR3_0_PCIE_LANE_7_EQUALIZATION_CNTL: u32 = 0x028a;
pub const cfgBIFPLR3_0_PCIE_LANE_8_EQUALIZATION_CNTL: u32 = 0x028c;
pub const cfgBIFPLR3_0_PCIE_LANE_9_EQUALIZATION_CNTL: u32 = 0x028e;
pub const cfgBIFPLR3_0_PCIE_LANE_10_EQUALIZATION_CNTL: u32 = 0x0290;
pub const cfgBIFPLR3_0_PCIE_LANE_11_EQUALIZATION_CNTL: u32 = 0x0292;
pub const cfgBIFPLR3_0_PCIE_LANE_12_EQUALIZATION_CNTL: u32 = 0x0294;
pub const cfgBIFPLR3_0_PCIE_LANE_13_EQUALIZATION_CNTL: u32 = 0x0296;
pub const cfgBIFPLR3_0_PCIE_LANE_14_EQUALIZATION_CNTL: u32 = 0x0298;
pub const cfgBIFPLR3_0_PCIE_LANE_15_EQUALIZATION_CNTL: u32 = 0x029a;
pub const cfgBIFPLR3_0_PCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgBIFPLR3_0_PCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgBIFPLR3_0_PCIE_ACS_CNTL: u32 = 0x02a6;
pub const cfgBIFPLR3_0_PCIE_MC_ENH_CAP_LIST: u32 = 0x02f0;
pub const cfgBIFPLR3_0_PCIE_MC_CAP: u32 = 0x02f4;
pub const cfgBIFPLR3_0_PCIE_MC_CNTL: u32 = 0x02f6;
pub const cfgBIFPLR3_0_PCIE_MC_ADDR0: u32 = 0x02f8;
pub const cfgBIFPLR3_0_PCIE_MC_ADDR1: u32 = 0x02fc;
pub const cfgBIFPLR3_0_PCIE_MC_RCV0: u32 = 0x0300;
pub const cfgBIFPLR3_0_PCIE_MC_RCV1: u32 = 0x0304;
pub const cfgBIFPLR3_0_PCIE_MC_BLOCK_ALL0: u32 = 0x0308;
pub const cfgBIFPLR3_0_PCIE_MC_BLOCK_ALL1: u32 = 0x030c;
pub const cfgBIFPLR3_0_PCIE_MC_BLOCK_UNTRANSLATED_0: u32 = 0x0310;
pub const cfgBIFPLR3_0_PCIE_MC_BLOCK_UNTRANSLATED_1: u32 = 0x0314;
pub const cfgBIFPLR3_0_PCIE_MC_OVERLAY_BAR0: u32 = 0x0318;
pub const cfgBIFPLR3_0_PCIE_MC_OVERLAY_BAR1: u32 = 0x031c;
pub const cfgBIFPLR3_0_PCIE_L1_PM_SUB_CAP_LIST: u32 = 0x0370;
pub const cfgBIFPLR3_0_PCIE_L1_PM_SUB_CAP: u32 = 0x0374;
pub const cfgBIFPLR3_0_PCIE_L1_PM_SUB_CNTL: u32 = 0x0378;
pub const cfgBIFPLR3_0_PCIE_L1_PM_SUB_CNTL2: u32 = 0x037c;
pub const cfgBIFPLR3_0_PCIE_DPC_ENH_CAP_LIST: u32 = 0x0380;
pub const cfgBIFPLR3_0_PCIE_DPC_CAP_LIST: u32 = 0x0384;
pub const cfgBIFPLR3_0_PCIE_DPC_CNTL: u32 = 0x0386;
pub const cfgBIFPLR3_0_PCIE_DPC_STATUS: u32 = 0x0388;
pub const cfgBIFPLR3_0_PCIE_DPC_ERROR_SOURCE_ID: u32 = 0x038a;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_STATUS: u32 = 0x038c;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_MASK: u32 = 0x0390;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_SEVERITY: u32 = 0x0394;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_SYSERROR: u32 = 0x0398;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_EXCEPTION: u32 = 0x039c;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_HDR_LOG0: u32 = 0x03a0;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_HDR_LOG1: u32 = 0x03a4;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_HDR_LOG2: u32 = 0x03a8;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_HDR_LOG3: u32 = 0x03ac;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_IMPSPEC_LOG: u32 = 0x03b0;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_PREFIX_LOG0: u32 = 0x03b4;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_PREFIX_LOG1: u32 = 0x03b8;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_PREFIX_LOG2: u32 = 0x03bc;
pub const cfgBIFPLR3_0_PCIE_RP_PIO_PREFIX_LOG3: u32 = 0x03c0;
pub const cfgBIFPLR3_0_PCIE_ESM_CAP_LIST: u32 = 0x03c4;
pub const cfgBIFPLR3_0_PCIE_ESM_HEADER_1: u32 = 0x03c8;
pub const cfgBIFPLR3_0_PCIE_ESM_HEADER_2: u32 = 0x03cc;
pub const cfgBIFPLR3_0_PCIE_ESM_STATUS: u32 = 0x03ce;
pub const cfgBIFPLR3_0_PCIE_ESM_CTRL: u32 = 0x03d0;
pub const cfgBIFPLR3_0_PCIE_ESM_CAP_1: u32 = 0x03d4;
pub const cfgBIFPLR3_0_PCIE_ESM_CAP_2: u32 = 0x03d8;
pub const cfgBIFPLR3_0_PCIE_ESM_CAP_3: u32 = 0x03dc;
pub const cfgBIFPLR3_0_PCIE_ESM_CAP_4: u32 = 0x03e0;
pub const cfgBIFPLR3_0_PCIE_ESM_CAP_5: u32 = 0x03e4;
pub const cfgBIFPLR3_0_PCIE_ESM_CAP_6: u32 = 0x03e8;
pub const cfgBIFPLR3_0_PCIE_ESM_CAP_7: u32 = 0x03ec;


// addressBlock: nbio_pcie0_bifplr4_cfgdecp
// base address: 0x0
pub const cfgBIFPLR4_0_VENDOR_ID: u32 = 0x0000;
pub const cfgBIFPLR4_0_DEVICE_ID: u32 = 0x0002;
pub const cfgBIFPLR4_0_COMMAND: u32 = 0x0004;
pub const cfgBIFPLR4_0_STATUS: u32 = 0x0006;
pub const cfgBIFPLR4_0_REVISION_ID: u32 = 0x0008;
pub const cfgBIFPLR4_0_PROG_INTERFACE: u32 = 0x0009;
pub const cfgBIFPLR4_0_SUB_CLASS: u32 = 0x000a;
pub const cfgBIFPLR4_0_BASE_CLASS: u32 = 0x000b;
pub const cfgBIFPLR4_0_CACHE_LINE: u32 = 0x000c;
pub const cfgBIFPLR4_0_LATENCY: u32 = 0x000d;
pub const cfgBIFPLR4_0_HEADER: u32 = 0x000e;
pub const cfgBIFPLR4_0_BIST: u32 = 0x000f;
pub const cfgBIFPLR4_0_SUB_BUS_NUMBER_LATENCY: u32 = 0x0018;
pub const cfgBIFPLR4_0_IO_BASE_LIMIT: u32 = 0x001c;
pub const cfgBIFPLR4_0_SECONDARY_STATUS: u32 = 0x001e;
pub const cfgBIFPLR4_0_MEM_BASE_LIMIT: u32 = 0x0020;
pub const cfgBIFPLR4_0_PREF_BASE_LIMIT: u32 = 0x0024;
pub const cfgBIFPLR4_0_PREF_BASE_UPPER: u32 = 0x0028;
pub const cfgBIFPLR4_0_PREF_LIMIT_UPPER: u32 = 0x002c;
pub const cfgBIFPLR4_0_IO_BASE_LIMIT_HI: u32 = 0x0030;
pub const cfgBIFPLR4_0_CAP_PTR: u32 = 0x0034;
pub const cfgBIFPLR4_0_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgBIFPLR4_0_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgBIFPLR4_0_IRQ_BRIDGE_CNTL: u32 = 0x003e;
pub const cfgBIFPLR4_0_EXT_BRIDGE_CNTL: u32 = 0x0040;
pub const cfgBIFPLR4_0_PMI_CAP_LIST: u32 = 0x0050;
pub const cfgBIFPLR4_0_PMI_CAP: u32 = 0x0052;
pub const cfgBIFPLR4_0_PMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgBIFPLR4_0_PCIE_CAP_LIST: u32 = 0x0058;
pub const cfgBIFPLR4_0_PCIE_CAP: u32 = 0x005a;
pub const cfgBIFPLR4_0_DEVICE_CAP: u32 = 0x005c;
pub const cfgBIFPLR4_0_DEVICE_CNTL: u32 = 0x0060;
pub const cfgBIFPLR4_0_DEVICE_STATUS: u32 = 0x0062;
pub const cfgBIFPLR4_0_LINK_CAP: u32 = 0x0064;
pub const cfgBIFPLR4_0_LINK_CNTL: u32 = 0x0068;
pub const cfgBIFPLR4_0_LINK_STATUS: u32 = 0x006a;
pub const cfgBIFPLR4_0_SLOT_CAP: u32 = 0x006c;
pub const cfgBIFPLR4_0_SLOT_CNTL: u32 = 0x0070;
pub const cfgBIFPLR4_0_SLOT_STATUS: u32 = 0x0072;
pub const cfgBIFPLR4_0_ROOT_CNTL: u32 = 0x0074;
pub const cfgBIFPLR4_0_ROOT_CAP: u32 = 0x0076;
pub const cfgBIFPLR4_0_ROOT_STATUS: u32 = 0x0078;
pub const cfgBIFPLR4_0_DEVICE_CAP2: u32 = 0x007c;
pub const cfgBIFPLR4_0_DEVICE_CNTL2: u32 = 0x0080;
pub const cfgBIFPLR4_0_DEVICE_STATUS2: u32 = 0x0082;
pub const cfgBIFPLR4_0_LINK_CAP2: u32 = 0x0084;
pub const cfgBIFPLR4_0_LINK_CNTL2: u32 = 0x0088;
pub const cfgBIFPLR4_0_LINK_STATUS2: u32 = 0x008a;
pub const cfgBIFPLR4_0_SLOT_CAP2: u32 = 0x008c;
pub const cfgBIFPLR4_0_SLOT_CNTL2: u32 = 0x0090;
pub const cfgBIFPLR4_0_SLOT_STATUS2: u32 = 0x0092;
pub const cfgBIFPLR4_0_MSI_CAP_LIST: u32 = 0x00a0;
pub const cfgBIFPLR4_0_MSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgBIFPLR4_0_MSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgBIFPLR4_0_MSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgBIFPLR4_0_MSI_MSG_DATA: u32 = 0x00a8;
pub const cfgBIFPLR4_0_MSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgBIFPLR4_0_SSID_CAP_LIST: u32 = 0x00c0;
pub const cfgBIFPLR4_0_SSID_CAP: u32 = 0x00c4;
pub const cfgBIFPLR4_0_MSI_MAP_CAP_LIST: u32 = 0x00c8;
pub const cfgBIFPLR4_0_MSI_MAP_CAP: u32 = 0x00ca;
pub const cfgBIFPLR4_0_MSI_MAP_ADDR_LO: u32 = 0x00cc;
pub const cfgBIFPLR4_0_MSI_MAP_ADDR_HI: u32 = 0x00d0;
pub const cfgBIFPLR4_0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgBIFPLR4_0_PCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgBIFPLR4_0_PCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgBIFPLR4_0_PCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgBIFPLR4_0_PCIE_VC_ENH_CAP_LIST: u32 = 0x0110;
pub const cfgBIFPLR4_0_PCIE_PORT_VC_CAP_REG1: u32 = 0x0114;
pub const cfgBIFPLR4_0_PCIE_PORT_VC_CAP_REG2: u32 = 0x0118;
pub const cfgBIFPLR4_0_PCIE_PORT_VC_CNTL: u32 = 0x011c;
pub const cfgBIFPLR4_0_PCIE_PORT_VC_STATUS: u32 = 0x011e;
pub const cfgBIFPLR4_0_PCIE_VC0_RESOURCE_CAP: u32 = 0x0120;
pub const cfgBIFPLR4_0_PCIE_VC0_RESOURCE_CNTL: u32 = 0x0124;
pub const cfgBIFPLR4_0_PCIE_VC0_RESOURCE_STATUS: u32 = 0x012a;
pub const cfgBIFPLR4_0_PCIE_VC1_RESOURCE_CAP: u32 = 0x012c;
pub const cfgBIFPLR4_0_PCIE_VC1_RESOURCE_CNTL: u32 = 0x0130;
pub const cfgBIFPLR4_0_PCIE_VC1_RESOURCE_STATUS: u32 = 0x0136;
pub const cfgBIFPLR4_0_PCIE_DEV_SERIAL_NUM_ENH_CAP_LIST: u32 = 0x0140;
pub const cfgBIFPLR4_0_PCIE_DEV_SERIAL_NUM_DW1: u32 = 0x0144;
pub const cfgBIFPLR4_0_PCIE_DEV_SERIAL_NUM_DW2: u32 = 0x0148;
pub const cfgBIFPLR4_0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgBIFPLR4_0_PCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgBIFPLR4_0_PCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgBIFPLR4_0_PCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgBIFPLR4_0_PCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgBIFPLR4_0_PCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgBIFPLR4_0_PCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgBIFPLR4_0_PCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgBIFPLR4_0_PCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgBIFPLR4_0_PCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgBIFPLR4_0_PCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgBIFPLR4_0_PCIE_ROOT_ERR_CMD: u32 = 0x017c;
pub const cfgBIFPLR4_0_PCIE_ROOT_ERR_STATUS: u32 = 0x0180;
pub const cfgBIFPLR4_0_PCIE_ERR_SRC_ID: u32 = 0x0184;
pub const cfgBIFPLR4_0_PCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgBIFPLR4_0_PCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgBIFPLR4_0_PCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgBIFPLR4_0_PCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgBIFPLR4_0_PCIE_SECONDARY_ENH_CAP_LIST: u32 = 0x0270;
pub const cfgBIFPLR4_0_PCIE_LINK_CNTL3: u32 = 0x0274;
pub const cfgBIFPLR4_0_PCIE_LANE_ERROR_STATUS: u32 = 0x0278;
pub const cfgBIFPLR4_0_PCIE_LANE_0_EQUALIZATION_CNTL: u32 = 0x027c;
pub const cfgBIFPLR4_0_PCIE_LANE_1_EQUALIZATION_CNTL: u32 = 0x027e;
pub const cfgBIFPLR4_0_PCIE_LANE_2_EQUALIZATION_CNTL: u32 = 0x0280;
pub const cfgBIFPLR4_0_PCIE_LANE_3_EQUALIZATION_CNTL: u32 = 0x0282;
pub const cfgBIFPLR4_0_PCIE_LANE_4_EQUALIZATION_CNTL: u32 = 0x0284;
pub const cfgBIFPLR4_0_PCIE_LANE_5_EQUALIZATION_CNTL: u32 = 0x0286;
pub const cfgBIFPLR4_0_PCIE_LANE_6_EQUALIZATION_CNTL: u32 = 0x0288;
pub const cfgBIFPLR4_0_PCIE_LANE_7_EQUALIZATION_CNTL: u32 = 0x028a;
pub const cfgBIFPLR4_0_PCIE_LANE_8_EQUALIZATION_CNTL: u32 = 0x028c;
pub const cfgBIFPLR4_0_PCIE_LANE_9_EQUALIZATION_CNTL: u32 = 0x028e;
pub const cfgBIFPLR4_0_PCIE_LANE_10_EQUALIZATION_CNTL: u32 = 0x0290;
pub const cfgBIFPLR4_0_PCIE_LANE_11_EQUALIZATION_CNTL: u32 = 0x0292;
pub const cfgBIFPLR4_0_PCIE_LANE_12_EQUALIZATION_CNTL: u32 = 0x0294;
pub const cfgBIFPLR4_0_PCIE_LANE_13_EQUALIZATION_CNTL: u32 = 0x0296;
pub const cfgBIFPLR4_0_PCIE_LANE_14_EQUALIZATION_CNTL: u32 = 0x0298;
pub const cfgBIFPLR4_0_PCIE_LANE_15_EQUALIZATION_CNTL: u32 = 0x029a;
pub const cfgBIFPLR4_0_PCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgBIFPLR4_0_PCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgBIFPLR4_0_PCIE_ACS_CNTL: u32 = 0x02a6;
pub const cfgBIFPLR4_0_PCIE_MC_ENH_CAP_LIST: u32 = 0x02f0;
pub const cfgBIFPLR4_0_PCIE_MC_CAP: u32 = 0x02f4;
pub const cfgBIFPLR4_0_PCIE_MC_CNTL: u32 = 0x02f6;
pub const cfgBIFPLR4_0_PCIE_MC_ADDR0: u32 = 0x02f8;
pub const cfgBIFPLR4_0_PCIE_MC_ADDR1: u32 = 0x02fc;
pub const cfgBIFPLR4_0_PCIE_MC_RCV0: u32 = 0x0300;
pub const cfgBIFPLR4_0_PCIE_MC_RCV1: u32 = 0x0304;
pub const cfgBIFPLR4_0_PCIE_MC_BLOCK_ALL0: u32 = 0x0308;
pub const cfgBIFPLR4_0_PCIE_MC_BLOCK_ALL1: u32 = 0x030c;
pub const cfgBIFPLR4_0_PCIE_MC_BLOCK_UNTRANSLATED_0: u32 = 0x0310;
pub const cfgBIFPLR4_0_PCIE_MC_BLOCK_UNTRANSLATED_1: u32 = 0x0314;
pub const cfgBIFPLR4_0_PCIE_MC_OVERLAY_BAR0: u32 = 0x0318;
pub const cfgBIFPLR4_0_PCIE_MC_OVERLAY_BAR1: u32 = 0x031c;
pub const cfgBIFPLR4_0_PCIE_L1_PM_SUB_CAP_LIST: u32 = 0x0370;
pub const cfgBIFPLR4_0_PCIE_L1_PM_SUB_CAP: u32 = 0x0374;
pub const cfgBIFPLR4_0_PCIE_L1_PM_SUB_CNTL: u32 = 0x0378;
pub const cfgBIFPLR4_0_PCIE_L1_PM_SUB_CNTL2: u32 = 0x037c;
pub const cfgBIFPLR4_0_PCIE_DPC_ENH_CAP_LIST: u32 = 0x0380;
pub const cfgBIFPLR4_0_PCIE_DPC_CAP_LIST: u32 = 0x0384;
pub const cfgBIFPLR4_0_PCIE_DPC_CNTL: u32 = 0x0386;
pub const cfgBIFPLR4_0_PCIE_DPC_STATUS: u32 = 0x0388;
pub const cfgBIFPLR4_0_PCIE_DPC_ERROR_SOURCE_ID: u32 = 0x038a;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_STATUS: u32 = 0x038c;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_MASK: u32 = 0x0390;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_SEVERITY: u32 = 0x0394;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_SYSERROR: u32 = 0x0398;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_EXCEPTION: u32 = 0x039c;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_HDR_LOG0: u32 = 0x03a0;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_HDR_LOG1: u32 = 0x03a4;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_HDR_LOG2: u32 = 0x03a8;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_HDR_LOG3: u32 = 0x03ac;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_IMPSPEC_LOG: u32 = 0x03b0;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_PREFIX_LOG0: u32 = 0x03b4;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_PREFIX_LOG1: u32 = 0x03b8;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_PREFIX_LOG2: u32 = 0x03bc;
pub const cfgBIFPLR4_0_PCIE_RP_PIO_PREFIX_LOG3: u32 = 0x03c0;
pub const cfgBIFPLR4_0_PCIE_ESM_CAP_LIST: u32 = 0x03c4;
pub const cfgBIFPLR4_0_PCIE_ESM_HEADER_1: u32 = 0x03c8;
pub const cfgBIFPLR4_0_PCIE_ESM_HEADER_2: u32 = 0x03cc;
pub const cfgBIFPLR4_0_PCIE_ESM_STATUS: u32 = 0x03ce;
pub const cfgBIFPLR4_0_PCIE_ESM_CTRL: u32 = 0x03d0;
pub const cfgBIFPLR4_0_PCIE_ESM_CAP_1: u32 = 0x03d4;
pub const cfgBIFPLR4_0_PCIE_ESM_CAP_2: u32 = 0x03d8;
pub const cfgBIFPLR4_0_PCIE_ESM_CAP_3: u32 = 0x03dc;
pub const cfgBIFPLR4_0_PCIE_ESM_CAP_4: u32 = 0x03e0;
pub const cfgBIFPLR4_0_PCIE_ESM_CAP_5: u32 = 0x03e4;
pub const cfgBIFPLR4_0_PCIE_ESM_CAP_6: u32 = 0x03e8;
pub const cfgBIFPLR4_0_PCIE_ESM_CAP_7: u32 = 0x03ec;


// addressBlock: nbio_pcie0_bifplr5_cfgdecp
// base address: 0x0
pub const cfgBIFPLR5_0_VENDOR_ID: u32 = 0x0000;
pub const cfgBIFPLR5_0_DEVICE_ID: u32 = 0x0002;
pub const cfgBIFPLR5_0_COMMAND: u32 = 0x0004;
pub const cfgBIFPLR5_0_STATUS: u32 = 0x0006;
pub const cfgBIFPLR5_0_REVISION_ID: u32 = 0x0008;
pub const cfgBIFPLR5_0_PROG_INTERFACE: u32 = 0x0009;
pub const cfgBIFPLR5_0_SUB_CLASS: u32 = 0x000a;
pub const cfgBIFPLR5_0_BASE_CLASS: u32 = 0x000b;
pub const cfgBIFPLR5_0_CACHE_LINE: u32 = 0x000c;
pub const cfgBIFPLR5_0_LATENCY: u32 = 0x000d;
pub const cfgBIFPLR5_0_HEADER: u32 = 0x000e;
pub const cfgBIFPLR5_0_BIST: u32 = 0x000f;
pub const cfgBIFPLR5_0_SUB_BUS_NUMBER_LATENCY: u32 = 0x0018;
pub const cfgBIFPLR5_0_IO_BASE_LIMIT: u32 = 0x001c;
pub const cfgBIFPLR5_0_SECONDARY_STATUS: u32 = 0x001e;
pub const cfgBIFPLR5_0_MEM_BASE_LIMIT: u32 = 0x0020;
pub const cfgBIFPLR5_0_PREF_BASE_LIMIT: u32 = 0x0024;
pub const cfgBIFPLR5_0_PREF_BASE_UPPER: u32 = 0x0028;
pub const cfgBIFPLR5_0_PREF_LIMIT_UPPER: u32 = 0x002c;
pub const cfgBIFPLR5_0_IO_BASE_LIMIT_HI: u32 = 0x0030;
pub const cfgBIFPLR5_0_CAP_PTR: u32 = 0x0034;
pub const cfgBIFPLR5_0_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgBIFPLR5_0_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgBIFPLR5_0_IRQ_BRIDGE_CNTL: u32 = 0x003e;
pub const cfgBIFPLR5_0_EXT_BRIDGE_CNTL: u32 = 0x0040;
pub const cfgBIFPLR5_0_PMI_CAP_LIST: u32 = 0x0050;
pub const cfgBIFPLR5_0_PMI_CAP: u32 = 0x0052;
pub const cfgBIFPLR5_0_PMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgBIFPLR5_0_PCIE_CAP_LIST: u32 = 0x0058;
pub const cfgBIFPLR5_0_PCIE_CAP: u32 = 0x005a;
pub const cfgBIFPLR5_0_DEVICE_CAP: u32 = 0x005c;
pub const cfgBIFPLR5_0_DEVICE_CNTL: u32 = 0x0060;
pub const cfgBIFPLR5_0_DEVICE_STATUS: u32 = 0x0062;
pub const cfgBIFPLR5_0_LINK_CAP: u32 = 0x0064;
pub const cfgBIFPLR5_0_LINK_CNTL: u32 = 0x0068;
pub const cfgBIFPLR5_0_LINK_STATUS: u32 = 0x006a;
pub const cfgBIFPLR5_0_SLOT_CAP: u32 = 0x006c;
pub const cfgBIFPLR5_0_SLOT_CNTL: u32 = 0x0070;
pub const cfgBIFPLR5_0_SLOT_STATUS: u32 = 0x0072;
pub const cfgBIFPLR5_0_ROOT_CNTL: u32 = 0x0074;
pub const cfgBIFPLR5_0_ROOT_CAP: u32 = 0x0076;
pub const cfgBIFPLR5_0_ROOT_STATUS: u32 = 0x0078;
pub const cfgBIFPLR5_0_DEVICE_CAP2: u32 = 0x007c;
pub const cfgBIFPLR5_0_DEVICE_CNTL2: u32 = 0x0080;
pub const cfgBIFPLR5_0_DEVICE_STATUS2: u32 = 0x0082;
pub const cfgBIFPLR5_0_LINK_CAP2: u32 = 0x0084;
pub const cfgBIFPLR5_0_LINK_CNTL2: u32 = 0x0088;
pub const cfgBIFPLR5_0_LINK_STATUS2: u32 = 0x008a;
pub const cfgBIFPLR5_0_SLOT_CAP2: u32 = 0x008c;
pub const cfgBIFPLR5_0_SLOT_CNTL2: u32 = 0x0090;
pub const cfgBIFPLR5_0_SLOT_STATUS2: u32 = 0x0092;
pub const cfgBIFPLR5_0_MSI_CAP_LIST: u32 = 0x00a0;
pub const cfgBIFPLR5_0_MSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgBIFPLR5_0_MSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgBIFPLR5_0_MSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgBIFPLR5_0_MSI_MSG_DATA: u32 = 0x00a8;
pub const cfgBIFPLR5_0_MSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgBIFPLR5_0_SSID_CAP_LIST: u32 = 0x00c0;
pub const cfgBIFPLR5_0_SSID_CAP: u32 = 0x00c4;
pub const cfgBIFPLR5_0_MSI_MAP_CAP_LIST: u32 = 0x00c8;
pub const cfgBIFPLR5_0_MSI_MAP_CAP: u32 = 0x00ca;
pub const cfgBIFPLR5_0_MSI_MAP_ADDR_LO: u32 = 0x00cc;
pub const cfgBIFPLR5_0_MSI_MAP_ADDR_HI: u32 = 0x00d0;
pub const cfgBIFPLR5_0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgBIFPLR5_0_PCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgBIFPLR5_0_PCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgBIFPLR5_0_PCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgBIFPLR5_0_PCIE_VC_ENH_CAP_LIST: u32 = 0x0110;
pub const cfgBIFPLR5_0_PCIE_PORT_VC_CAP_REG1: u32 = 0x0114;
pub const cfgBIFPLR5_0_PCIE_PORT_VC_CAP_REG2: u32 = 0x0118;
pub const cfgBIFPLR5_0_PCIE_PORT_VC_CNTL: u32 = 0x011c;
pub const cfgBIFPLR5_0_PCIE_PORT_VC_STATUS: u32 = 0x011e;
pub const cfgBIFPLR5_0_PCIE_VC0_RESOURCE_CAP: u32 = 0x0120;
pub const cfgBIFPLR5_0_PCIE_VC0_RESOURCE_CNTL: u32 = 0x0124;
pub const cfgBIFPLR5_0_PCIE_VC0_RESOURCE_STATUS: u32 = 0x012a;
pub const cfgBIFPLR5_0_PCIE_VC1_RESOURCE_CAP: u32 = 0x012c;
pub const cfgBIFPLR5_0_PCIE_VC1_RESOURCE_CNTL: u32 = 0x0130;
pub const cfgBIFPLR5_0_PCIE_VC1_RESOURCE_STATUS: u32 = 0x0136;
pub const cfgBIFPLR5_0_PCIE_DEV_SERIAL_NUM_ENH_CAP_LIST: u32 = 0x0140;
pub const cfgBIFPLR5_0_PCIE_DEV_SERIAL_NUM_DW1: u32 = 0x0144;
pub const cfgBIFPLR5_0_PCIE_DEV_SERIAL_NUM_DW2: u32 = 0x0148;
pub const cfgBIFPLR5_0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgBIFPLR5_0_PCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgBIFPLR5_0_PCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgBIFPLR5_0_PCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgBIFPLR5_0_PCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgBIFPLR5_0_PCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgBIFPLR5_0_PCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgBIFPLR5_0_PCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgBIFPLR5_0_PCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgBIFPLR5_0_PCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgBIFPLR5_0_PCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgBIFPLR5_0_PCIE_ROOT_ERR_CMD: u32 = 0x017c;
pub const cfgBIFPLR5_0_PCIE_ROOT_ERR_STATUS: u32 = 0x0180;
pub const cfgBIFPLR5_0_PCIE_ERR_SRC_ID: u32 = 0x0184;
pub const cfgBIFPLR5_0_PCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgBIFPLR5_0_PCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgBIFPLR5_0_PCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgBIFPLR5_0_PCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgBIFPLR5_0_PCIE_SECONDARY_ENH_CAP_LIST: u32 = 0x0270;
pub const cfgBIFPLR5_0_PCIE_LINK_CNTL3: u32 = 0x0274;
pub const cfgBIFPLR5_0_PCIE_LANE_ERROR_STATUS: u32 = 0x0278;
pub const cfgBIFPLR5_0_PCIE_LANE_0_EQUALIZATION_CNTL: u32 = 0x027c;
pub const cfgBIFPLR5_0_PCIE_LANE_1_EQUALIZATION_CNTL: u32 = 0x027e;
pub const cfgBIFPLR5_0_PCIE_LANE_2_EQUALIZATION_CNTL: u32 = 0x0280;
pub const cfgBIFPLR5_0_PCIE_LANE_3_EQUALIZATION_CNTL: u32 = 0x0282;
pub const cfgBIFPLR5_0_PCIE_LANE_4_EQUALIZATION_CNTL: u32 = 0x0284;
pub const cfgBIFPLR5_0_PCIE_LANE_5_EQUALIZATION_CNTL: u32 = 0x0286;
pub const cfgBIFPLR5_0_PCIE_LANE_6_EQUALIZATION_CNTL: u32 = 0x0288;
pub const cfgBIFPLR5_0_PCIE_LANE_7_EQUALIZATION_CNTL: u32 = 0x028a;
pub const cfgBIFPLR5_0_PCIE_LANE_8_EQUALIZATION_CNTL: u32 = 0x028c;
pub const cfgBIFPLR5_0_PCIE_LANE_9_EQUALIZATION_CNTL: u32 = 0x028e;
pub const cfgBIFPLR5_0_PCIE_LANE_10_EQUALIZATION_CNTL: u32 = 0x0290;
pub const cfgBIFPLR5_0_PCIE_LANE_11_EQUALIZATION_CNTL: u32 = 0x0292;
pub const cfgBIFPLR5_0_PCIE_LANE_12_EQUALIZATION_CNTL: u32 = 0x0294;
pub const cfgBIFPLR5_0_PCIE_LANE_13_EQUALIZATION_CNTL: u32 = 0x0296;
pub const cfgBIFPLR5_0_PCIE_LANE_14_EQUALIZATION_CNTL: u32 = 0x0298;
pub const cfgBIFPLR5_0_PCIE_LANE_15_EQUALIZATION_CNTL: u32 = 0x029a;
pub const cfgBIFPLR5_0_PCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgBIFPLR5_0_PCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgBIFPLR5_0_PCIE_ACS_CNTL: u32 = 0x02a6;
pub const cfgBIFPLR5_0_PCIE_MC_ENH_CAP_LIST: u32 = 0x02f0;
pub const cfgBIFPLR5_0_PCIE_MC_CAP: u32 = 0x02f4;
pub const cfgBIFPLR5_0_PCIE_MC_CNTL: u32 = 0x02f6;
pub const cfgBIFPLR5_0_PCIE_MC_ADDR0: u32 = 0x02f8;
pub const cfgBIFPLR5_0_PCIE_MC_ADDR1: u32 = 0x02fc;
pub const cfgBIFPLR5_0_PCIE_MC_RCV0: u32 = 0x0300;
pub const cfgBIFPLR5_0_PCIE_MC_RCV1: u32 = 0x0304;
pub const cfgBIFPLR5_0_PCIE_MC_BLOCK_ALL0: u32 = 0x0308;
pub const cfgBIFPLR5_0_PCIE_MC_BLOCK_ALL1: u32 = 0x030c;
pub const cfgBIFPLR5_0_PCIE_MC_BLOCK_UNTRANSLATED_0: u32 = 0x0310;
pub const cfgBIFPLR5_0_PCIE_MC_BLOCK_UNTRANSLATED_1: u32 = 0x0314;
pub const cfgBIFPLR5_0_PCIE_MC_OVERLAY_BAR0: u32 = 0x0318;
pub const cfgBIFPLR5_0_PCIE_MC_OVERLAY_BAR1: u32 = 0x031c;
pub const cfgBIFPLR5_0_PCIE_L1_PM_SUB_CAP_LIST: u32 = 0x0370;
pub const cfgBIFPLR5_0_PCIE_L1_PM_SUB_CAP: u32 = 0x0374;
pub const cfgBIFPLR5_0_PCIE_L1_PM_SUB_CNTL: u32 = 0x0378;
pub const cfgBIFPLR5_0_PCIE_L1_PM_SUB_CNTL2: u32 = 0x037c;
pub const cfgBIFPLR5_0_PCIE_DPC_ENH_CAP_LIST: u32 = 0x0380;
pub const cfgBIFPLR5_0_PCIE_DPC_CAP_LIST: u32 = 0x0384;
pub const cfgBIFPLR5_0_PCIE_DPC_CNTL: u32 = 0x0386;
pub const cfgBIFPLR5_0_PCIE_DPC_STATUS: u32 = 0x0388;
pub const cfgBIFPLR5_0_PCIE_DPC_ERROR_SOURCE_ID: u32 = 0x038a;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_STATUS: u32 = 0x038c;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_MASK: u32 = 0x0390;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_SEVERITY: u32 = 0x0394;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_SYSERROR: u32 = 0x0398;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_EXCEPTION: u32 = 0x039c;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_HDR_LOG0: u32 = 0x03a0;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_HDR_LOG1: u32 = 0x03a4;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_HDR_LOG2: u32 = 0x03a8;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_HDR_LOG3: u32 = 0x03ac;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_IMPSPEC_LOG: u32 = 0x03b0;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_PREFIX_LOG0: u32 = 0x03b4;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_PREFIX_LOG1: u32 = 0x03b8;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_PREFIX_LOG2: u32 = 0x03bc;
pub const cfgBIFPLR5_0_PCIE_RP_PIO_PREFIX_LOG3: u32 = 0x03c0;
pub const cfgBIFPLR5_0_PCIE_ESM_CAP_LIST: u32 = 0x03c4;
pub const cfgBIFPLR5_0_PCIE_ESM_HEADER_1: u32 = 0x03c8;
pub const cfgBIFPLR5_0_PCIE_ESM_HEADER_2: u32 = 0x03cc;
pub const cfgBIFPLR5_0_PCIE_ESM_STATUS: u32 = 0x03ce;
pub const cfgBIFPLR5_0_PCIE_ESM_CTRL: u32 = 0x03d0;
pub const cfgBIFPLR5_0_PCIE_ESM_CAP_1: u32 = 0x03d4;
pub const cfgBIFPLR5_0_PCIE_ESM_CAP_2: u32 = 0x03d8;
pub const cfgBIFPLR5_0_PCIE_ESM_CAP_3: u32 = 0x03dc;
pub const cfgBIFPLR5_0_PCIE_ESM_CAP_4: u32 = 0x03e0;
pub const cfgBIFPLR5_0_PCIE_ESM_CAP_5: u32 = 0x03e4;
pub const cfgBIFPLR5_0_PCIE_ESM_CAP_6: u32 = 0x03e8;
pub const cfgBIFPLR5_0_PCIE_ESM_CAP_7: u32 = 0x03ec;


// addressBlock: nbio_pcie0_bifplr6_cfgdecp
// base address: 0x0
pub const cfgBIFPLR6_0_VENDOR_ID: u32 = 0x0000;
pub const cfgBIFPLR6_0_DEVICE_ID: u32 = 0x0002;
pub const cfgBIFPLR6_0_COMMAND: u32 = 0x0004;
pub const cfgBIFPLR6_0_STATUS: u32 = 0x0006;
pub const cfgBIFPLR6_0_REVISION_ID: u32 = 0x0008;
pub const cfgBIFPLR6_0_PROG_INTERFACE: u32 = 0x0009;
pub const cfgBIFPLR6_0_SUB_CLASS: u32 = 0x000a;
pub const cfgBIFPLR6_0_BASE_CLASS: u32 = 0x000b;
pub const cfgBIFPLR6_0_CACHE_LINE: u32 = 0x000c;
pub const cfgBIFPLR6_0_LATENCY: u32 = 0x000d;
pub const cfgBIFPLR6_0_HEADER: u32 = 0x000e;
pub const cfgBIFPLR6_0_BIST: u32 = 0x000f;
pub const cfgBIFPLR6_0_SUB_BUS_NUMBER_LATENCY: u32 = 0x0018;
pub const cfgBIFPLR6_0_IO_BASE_LIMIT: u32 = 0x001c;
pub const cfgBIFPLR6_0_SECONDARY_STATUS: u32 = 0x001e;
pub const cfgBIFPLR6_0_MEM_BASE_LIMIT: u32 = 0x0020;
pub const cfgBIFPLR6_0_PREF_BASE_LIMIT: u32 = 0x0024;
pub const cfgBIFPLR6_0_PREF_BASE_UPPER: u32 = 0x0028;
pub const cfgBIFPLR6_0_PREF_LIMIT_UPPER: u32 = 0x002c;
pub const cfgBIFPLR6_0_IO_BASE_LIMIT_HI: u32 = 0x0030;
pub const cfgBIFPLR6_0_CAP_PTR: u32 = 0x0034;
pub const cfgBIFPLR6_0_INTERRUPT_LINE: u32 = 0x003c;
pub const cfgBIFPLR6_0_INTERRUPT_PIN: u32 = 0x003d;
pub const cfgBIFPLR6_0_IRQ_BRIDGE_CNTL: u32 = 0x003e;
pub const cfgBIFPLR6_0_EXT_BRIDGE_CNTL: u32 = 0x0040;
pub const cfgBIFPLR6_0_PMI_CAP_LIST: u32 = 0x0050;
pub const cfgBIFPLR6_0_PMI_CAP: u32 = 0x0052;
pub const cfgBIFPLR6_0_PMI_STATUS_CNTL: u32 = 0x0054;
pub const cfgBIFPLR6_0_PCIE_CAP_LIST: u32 = 0x0058;
pub const cfgBIFPLR6_0_PCIE_CAP: u32 = 0x005a;
pub const cfgBIFPLR6_0_DEVICE_CAP: u32 = 0x005c;
pub const cfgBIFPLR6_0_DEVICE_CNTL: u32 = 0x0060;
pub const cfgBIFPLR6_0_DEVICE_STATUS: u32 = 0x0062;
pub const cfgBIFPLR6_0_LINK_CAP: u32 = 0x0064;
pub const cfgBIFPLR6_0_LINK_CNTL: u32 = 0x0068;
pub const cfgBIFPLR6_0_LINK_STATUS: u32 = 0x006a;
pub const cfgBIFPLR6_0_SLOT_CAP: u32 = 0x006c;
pub const cfgBIFPLR6_0_SLOT_CNTL: u32 = 0x0070;
pub const cfgBIFPLR6_0_SLOT_STATUS: u32 = 0x0072;
pub const cfgBIFPLR6_0_ROOT_CNTL: u32 = 0x0074;
pub const cfgBIFPLR6_0_ROOT_CAP: u32 = 0x0076;
pub const cfgBIFPLR6_0_ROOT_STATUS: u32 = 0x0078;
pub const cfgBIFPLR6_0_DEVICE_CAP2: u32 = 0x007c;
pub const cfgBIFPLR6_0_DEVICE_CNTL2: u32 = 0x0080;
pub const cfgBIFPLR6_0_DEVICE_STATUS2: u32 = 0x0082;
pub const cfgBIFPLR6_0_LINK_CAP2: u32 = 0x0084;
pub const cfgBIFPLR6_0_LINK_CNTL2: u32 = 0x0088;
pub const cfgBIFPLR6_0_LINK_STATUS2: u32 = 0x008a;
pub const cfgBIFPLR6_0_SLOT_CAP2: u32 = 0x008c;
pub const cfgBIFPLR6_0_SLOT_CNTL2: u32 = 0x0090;
pub const cfgBIFPLR6_0_SLOT_STATUS2: u32 = 0x0092;
pub const cfgBIFPLR6_0_MSI_CAP_LIST: u32 = 0x00a0;
pub const cfgBIFPLR6_0_MSI_MSG_CNTL: u32 = 0x00a2;
pub const cfgBIFPLR6_0_MSI_MSG_ADDR_LO: u32 = 0x00a4;
pub const cfgBIFPLR6_0_MSI_MSG_ADDR_HI: u32 = 0x00a8;
pub const cfgBIFPLR6_0_MSI_MSG_DATA: u32 = 0x00a8;
pub const cfgBIFPLR6_0_MSI_MSG_DATA_64: u32 = 0x00ac;
pub const cfgBIFPLR6_0_SSID_CAP_LIST: u32 = 0x00c0;
pub const cfgBIFPLR6_0_SSID_CAP: u32 = 0x00c4;
pub const cfgBIFPLR6_0_MSI_MAP_CAP_LIST: u32 = 0x00c8;
pub const cfgBIFPLR6_0_MSI_MAP_CAP: u32 = 0x00ca;
pub const cfgBIFPLR6_0_MSI_MAP_ADDR_LO: u32 = 0x00cc;
pub const cfgBIFPLR6_0_MSI_MAP_ADDR_HI: u32 = 0x00d0;
pub const cfgBIFPLR6_0_PCIE_VENDOR_SPECIFIC_ENH_CAP_LIST: u32 = 0x0100;
pub const cfgBIFPLR6_0_PCIE_VENDOR_SPECIFIC_HDR: u32 = 0x0104;
pub const cfgBIFPLR6_0_PCIE_VENDOR_SPECIFIC1: u32 = 0x0108;
pub const cfgBIFPLR6_0_PCIE_VENDOR_SPECIFIC2: u32 = 0x010c;
pub const cfgBIFPLR6_0_PCIE_VC_ENH_CAP_LIST: u32 = 0x0110;
pub const cfgBIFPLR6_0_PCIE_PORT_VC_CAP_REG1: u32 = 0x0114;
pub const cfgBIFPLR6_0_PCIE_PORT_VC_CAP_REG2: u32 = 0x0118;
pub const cfgBIFPLR6_0_PCIE_PORT_VC_CNTL: u32 = 0x011c;
pub const cfgBIFPLR6_0_PCIE_PORT_VC_STATUS: u32 = 0x011e;
pub const cfgBIFPLR6_0_PCIE_VC0_RESOURCE_CAP: u32 = 0x0120;
pub const cfgBIFPLR6_0_PCIE_VC0_RESOURCE_CNTL: u32 = 0x0124;
pub const cfgBIFPLR6_0_PCIE_VC0_RESOURCE_STATUS: u32 = 0x012a;
pub const cfgBIFPLR6_0_PCIE_VC1_RESOURCE_CAP: u32 = 0x012c;
pub const cfgBIFPLR6_0_PCIE_VC1_RESOURCE_CNTL: u32 = 0x0130;
pub const cfgBIFPLR6_0_PCIE_VC1_RESOURCE_STATUS: u32 = 0x0136;
pub const cfgBIFPLR6_0_PCIE_DEV_SERIAL_NUM_ENH_CAP_LIST: u32 = 0x0140;
pub const cfgBIFPLR6_0_PCIE_DEV_SERIAL_NUM_DW1: u32 = 0x0144;
pub const cfgBIFPLR6_0_PCIE_DEV_SERIAL_NUM_DW2: u32 = 0x0148;
pub const cfgBIFPLR6_0_PCIE_ADV_ERR_RPT_ENH_CAP_LIST: u32 = 0x0150;
pub const cfgBIFPLR6_0_PCIE_UNCORR_ERR_STATUS: u32 = 0x0154;
pub const cfgBIFPLR6_0_PCIE_UNCORR_ERR_MASK: u32 = 0x0158;
pub const cfgBIFPLR6_0_PCIE_UNCORR_ERR_SEVERITY: u32 = 0x015c;
pub const cfgBIFPLR6_0_PCIE_CORR_ERR_STATUS: u32 = 0x0160;
pub const cfgBIFPLR6_0_PCIE_CORR_ERR_MASK: u32 = 0x0164;
pub const cfgBIFPLR6_0_PCIE_ADV_ERR_CAP_CNTL: u32 = 0x0168;
pub const cfgBIFPLR6_0_PCIE_HDR_LOG0: u32 = 0x016c;
pub const cfgBIFPLR6_0_PCIE_HDR_LOG1: u32 = 0x0170;
pub const cfgBIFPLR6_0_PCIE_HDR_LOG2: u32 = 0x0174;
pub const cfgBIFPLR6_0_PCIE_HDR_LOG3: u32 = 0x0178;
pub const cfgBIFPLR6_0_PCIE_ROOT_ERR_CMD: u32 = 0x017c;
pub const cfgBIFPLR6_0_PCIE_ROOT_ERR_STATUS: u32 = 0x0180;
pub const cfgBIFPLR6_0_PCIE_ERR_SRC_ID: u32 = 0x0184;
pub const cfgBIFPLR6_0_PCIE_TLP_PREFIX_LOG0: u32 = 0x0188;
pub const cfgBIFPLR6_0_PCIE_TLP_PREFIX_LOG1: u32 = 0x018c;
pub const cfgBIFPLR6_0_PCIE_TLP_PREFIX_LOG2: u32 = 0x0190;
pub const cfgBIFPLR6_0_PCIE_TLP_PREFIX_LOG3: u32 = 0x0194;
pub const cfgBIFPLR6_0_PCIE_SECONDARY_ENH_CAP_LIST: u32 = 0x0270;
pub const cfgBIFPLR6_0_PCIE_LINK_CNTL3: u32 = 0x0274;
pub const cfgBIFPLR6_0_PCIE_LANE_ERROR_STATUS: u32 = 0x0278;
pub const cfgBIFPLR6_0_PCIE_LANE_0_EQUALIZATION_CNTL: u32 = 0x027c;
pub const cfgBIFPLR6_0_PCIE_LANE_1_EQUALIZATION_CNTL: u32 = 0x027e;
pub const cfgBIFPLR6_0_PCIE_LANE_2_EQUALIZATION_CNTL: u32 = 0x0280;
pub const cfgBIFPLR6_0_PCIE_LANE_3_EQUALIZATION_CNTL: u32 = 0x0282;
pub const cfgBIFPLR6_0_PCIE_LANE_4_EQUALIZATION_CNTL: u32 = 0x0284;
pub const cfgBIFPLR6_0_PCIE_LANE_5_EQUALIZATION_CNTL: u32 = 0x0286;
pub const cfgBIFPLR6_0_PCIE_LANE_6_EQUALIZATION_CNTL: u32 = 0x0288;
pub const cfgBIFPLR6_0_PCIE_LANE_7_EQUALIZATION_CNTL: u32 = 0x028a;
pub const cfgBIFPLR6_0_PCIE_LANE_8_EQUALIZATION_CNTL: u32 = 0x028c;
pub const cfgBIFPLR6_0_PCIE_LANE_9_EQUALIZATION_CNTL: u32 = 0x028e;
pub const cfgBIFPLR6_0_PCIE_LANE_10_EQUALIZATION_CNTL: u32 = 0x0290;
pub const cfgBIFPLR6_0_PCIE_LANE_11_EQUALIZATION_CNTL: u32 = 0x0292;
pub const cfgBIFPLR6_0_PCIE_LANE_12_EQUALIZATION_CNTL: u32 = 0x0294;
pub const cfgBIFPLR6_0_PCIE_LANE_13_EQUALIZATION_CNTL: u32 = 0x0296;
pub const cfgBIFPLR6_0_PCIE_LANE_14_EQUALIZATION_CNTL: u32 = 0x0298;
pub const cfgBIFPLR6_0_PCIE_LANE_15_EQUALIZATION_CNTL: u32 = 0x029a;
pub const cfgBIFPLR6_0_PCIE_ACS_ENH_CAP_LIST: u32 = 0x02a0;
pub const cfgBIFPLR6_0_PCIE_ACS_CAP: u32 = 0x02a4;
pub const cfgBIFPLR6_0_PCIE_ACS_CNTL: u32 = 0x02a6;
pub const cfgBIFPLR6_0_PCIE_MC_ENH_CAP_LIST: u32 = 0x02f0;
pub const cfgBIFPLR6_0_PCIE_MC_CAP: u32 = 0x02f4;
pub const cfgBIFPLR6_0_PCIE_MC_CNTL: u32 = 0x02f6;
pub const cfgBIFPLR6_0_PCIE_MC_ADDR0: u32 = 0x02f8;
pub const cfgBIFPLR6_0_PCIE_MC_ADDR1: u32 = 0x02fc;
pub const cfgBIFPLR6_0_PCIE_MC_RCV0: u32 = 0x0300;
pub const cfgBIFPLR6_0_PCIE_MC_RCV1: u32 = 0x0304;
pub const cfgBIFPLR6_0_PCIE_MC_BLOCK_ALL0: u32 = 0x0308;
pub const cfgBIFPLR6_0_PCIE_MC_BLOCK_ALL1: u32 = 0x030c;
pub const cfgBIFPLR6_0_PCIE_MC_BLOCK_UNTRANSLATED_0: u32 = 0x0310;
pub const cfgBIFPLR6_0_PCIE_MC_BLOCK_UNTRANSLATED_1: u32 = 0x0314;
pub const cfgBIFPLR6_0_PCIE_MC_OVERLAY_BAR0: u32 = 0x0318;
pub const cfgBIFPLR6_0_PCIE_MC_OVERLAY_BAR1: u32 = 0x031c;
pub const cfgBIFPLR6_0_PCIE_L1_PM_SUB_CAP_LIST: u32 = 0x0370;
pub const cfgBIFPLR6_0_PCIE_L1_PM_SUB_CAP: u32 = 0x0374;
pub const cfgBIFPLR6_0_PCIE_L1_PM_SUB_CNTL: u32 = 0x0378;
pub const cfgBIFPLR6_0_PCIE_L1_PM_SUB_CNTL2: u32 = 0x037c;
pub const cfgBIFPLR6_0_PCIE_DPC_ENH_CAP_LIST: u32 = 0x0380;
pub const cfgBIFPLR6_0_PCIE_DPC_CAP_LIST: u32 = 0x0384;
pub const cfgBIFPLR6_0_PCIE_DPC_CNTL: u32 = 0x0386;
pub const cfgBIFPLR6_0_PCIE_DPC_STATUS: u32 = 0x0388;
pub const cfgBIFPLR6_0_PCIE_DPC_ERROR_SOURCE_ID: u32 = 0x038a;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_STATUS: u32 = 0x038c;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_MASK: u32 = 0x0390;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_SEVERITY: u32 = 0x0394;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_SYSERROR: u32 = 0x0398;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_EXCEPTION: u32 = 0x039c;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_HDR_LOG0: u32 = 0x03a0;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_HDR_LOG1: u32 = 0x03a4;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_HDR_LOG2: u32 = 0x03a8;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_HDR_LOG3: u32 = 0x03ac;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_IMPSPEC_LOG: u32 = 0x03b0;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_PREFIX_LOG0: u32 = 0x03b4;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_PREFIX_LOG1: u32 = 0x03b8;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_PREFIX_LOG2: u32 = 0x03bc;
pub const cfgBIFPLR6_0_PCIE_RP_PIO_PREFIX_LOG3: u32 = 0x03c0;
pub const cfgBIFPLR6_0_PCIE_ESM_CAP_LIST: u32 = 0x03c4;
pub const cfgBIFPLR6_0_PCIE_ESM_HEADER_1: u32 = 0x03c8;
pub const cfgBIFPLR6_0_PCIE_ESM_HEADER_2: u32 = 0x03cc;
pub const cfgBIFPLR6_0_PCIE_ESM_STATUS: u32 = 0x03ce;
pub const cfgBIFPLR6_0_PCIE_ESM_CTRL: u32 = 0x03d0;
pub const cfgBIFPLR6_0_PCIE_ESM_CAP_1: u32 = 0x03d4;
pub const cfgBIFPLR6_0_PCIE_ESM_CAP_2: u32 = 0x03d8;
pub const cfgBIFPLR6_0_PCIE_ESM_CAP_3: u32 = 0x03dc;
pub const cfgBIFPLR6_0_PCIE_ESM_CAP_4: u32 = 0x03e0;
pub const cfgBIFPLR6_0_PCIE_ESM_CAP_5: u32 = 0x03e4;
pub const cfgBIFPLR6_0_PCIE_ESM_CAP_6: u32 = 0x03e8;
pub const cfgBIFPLR6_0_PCIE_ESM_CAP_7: u32 = 0x03ec;


// addressBlock: nbio_dbgu0_dbgudec
// base address: 0x700
pub const mmport_a_addr: u32 = 0x01ac;
#define mmport_a_addr_BASE_IDX                                                                         1
pub const mmport_a_data_lo: u32 = 0x01ad;
#define mmport_a_data_lo_BASE_IDX                                                                      1
pub const mmport_a_data_hi: u32 = 0x01ae;
#define mmport_a_data_hi_BASE_IDX                                                                      1
pub const mmport_b_addr: u32 = 0x01af;
#define mmport_b_addr_BASE_IDX                                                                         1
pub const mmport_b_data_lo: u32 = 0x01b0;
#define mmport_b_data_lo_BASE_IDX                                                                      1
pub const mmport_b_data_hi: u32 = 0x01b1;
#define mmport_b_data_hi_BASE_IDX                                                                      1
pub const mmport_c_addr: u32 = 0x01b2;
#define mmport_c_addr_BASE_IDX                                                                         1
pub const mmport_c_data_lo: u32 = 0x01b3;
#define mmport_c_data_lo_BASE_IDX                                                                      1
pub const mmport_c_data_hi: u32 = 0x01b4;
#define mmport_c_data_hi_BASE_IDX                                                                      1
pub const mmport_d_addr: u32 = 0x01b5;
#define mmport_d_addr_BASE_IDX                                                                         1
pub const mmport_d_data_lo: u32 = 0x01b6;
#define mmport_d_data_lo_BASE_IDX                                                                      1
pub const mmport_d_data_hi: u32 = 0x01b7;
#define mmport_d_data_hi_BASE_IDX                                                                      1


// addressBlock: nbio_iohub_iommu_l2mmio_l2mmiocfg
// base address: 0x0
pub const mmIOMMU_MMIO_DEVTBL_BASE_0: u32 = 0x0000;
#define mmIOMMU_MMIO_DEVTBL_BASE_0_BASE_IDX                                                            0
pub const mmIOMMU_MMIO_DEVTBL_BASE_1: u32 = 0x0001;
#define mmIOMMU_MMIO_DEVTBL_BASE_1_BASE_IDX                                                            0
pub const mmIOMMU_MMIO_CMD_BASE_0: u32 = 0x0002;
#define mmIOMMU_MMIO_CMD_BASE_0_BASE_IDX                                                               0
pub const mmIOMMU_MMIO_CMD_BASE_1: u32 = 0x0003;
#define mmIOMMU_MMIO_CMD_BASE_1_BASE_IDX                                                               0
pub const mmIOMMU_MMIO_EVENT_BASE_0: u32 = 0x0004;
#define mmIOMMU_MMIO_EVENT_BASE_0_BASE_IDX                                                             0
pub const mmIOMMU_MMIO_EVENT_BASE_1: u32 = 0x0005;
#define mmIOMMU_MMIO_EVENT_BASE_1_BASE_IDX                                                             0
pub const mmIOMMU_MMIO_CNTRL_0: u32 = 0x0006;
#define mmIOMMU_MMIO_CNTRL_0_BASE_IDX                                                                  0
pub const mmIOMMU_MMIO_CNTRL_1: u32 = 0x0007;
#define mmIOMMU_MMIO_CNTRL_1_BASE_IDX                                                                  0
pub const mmIOMMU_MMIO_EXCL_BASE_0: u32 = 0x0008;
#define mmIOMMU_MMIO_EXCL_BASE_0_BASE_IDX                                                              0
pub const mmIOMMU_MMIO_EXCL_BASE_1: u32 = 0x0009;
#define mmIOMMU_MMIO_EXCL_BASE_1_BASE_IDX                                                              0
pub const mmIOMMU_MMIO_EXCL_LIM_0: u32 = 0x000a;
#define mmIOMMU_MMIO_EXCL_LIM_0_BASE_IDX                                                               0
pub const mmIOMMU_MMIO_EXCL_LIM_1: u32 = 0x000b;
#define mmIOMMU_MMIO_EXCL_LIM_1_BASE_IDX                                                               0
pub const mmIOMMU_MMIO_EFR_0: u32 = 0x000c;
#define mmIOMMU_MMIO_EFR_0_BASE_IDX                                                                    0
pub const mmIOMMU_MMIO_EFR_1: u32 = 0x000d;
#define mmIOMMU_MMIO_EFR_1_BASE_IDX                                                                    0
pub const mmIOMMU_MMIO_PPR_BASE_0: u32 = 0x000e;
#define mmIOMMU_MMIO_PPR_BASE_0_BASE_IDX                                                               0
pub const mmIOMMU_MMIO_PPR_BASE_1: u32 = 0x000f;
#define mmIOMMU_MMIO_PPR_BASE_1_BASE_IDX                                                               0
pub const mmIOMMU_MMIO_HW_ERR_UPPER_0: u32 = 0x0010;
#define mmIOMMU_MMIO_HW_ERR_UPPER_0_BASE_IDX                                                           0
pub const mmIOMMU_MMIO_HW_ERR_UPPER_1: u32 = 0x0011;
#define mmIOMMU_MMIO_HW_ERR_UPPER_1_BASE_IDX                                                           0
pub const mmIOMMU_MMIO_HW_ERR_LOWER_0: u32 = 0x0012;
#define mmIOMMU_MMIO_HW_ERR_LOWER_0_BASE_IDX                                                           0
pub const mmIOMMU_MMIO_HW_ERR_LOWER_1: u32 = 0x0013;
#define mmIOMMU_MMIO_HW_ERR_LOWER_1_BASE_IDX                                                           0
pub const mmIOMMU_MMIO_HW_ERR_STATUS_0: u32 = 0x0000;
#define mmIOMMU_MMIO_HW_ERR_STATUS_0_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_HW_ERR_STATUS_1: u32 = 0x0001;
#define mmIOMMU_MMIO_HW_ERR_STATUS_1_BASE_IDX                                                          1
pub const mmSMI_FILTER_REGISTER_0_0: u32 = 0x0004;
#define mmSMI_FILTER_REGISTER_0_0_BASE_IDX                                                             1
pub const mmSMI_FILTER_REGISTER_0_1: u32 = 0x0005;
#define mmSMI_FILTER_REGISTER_0_1_BASE_IDX                                                             1
pub const mmSMI_FILTER_REGISTER_1_0: u32 = 0x0006;
#define mmSMI_FILTER_REGISTER_1_0_BASE_IDX                                                             1
pub const mmSMI_FILTER_REGISTER_1_1: u32 = 0x0007;
#define mmSMI_FILTER_REGISTER_1_1_BASE_IDX                                                             1
pub const mmSMI_FILTER_REGISTER_2_0: u32 = 0x0008;
#define mmSMI_FILTER_REGISTER_2_0_BASE_IDX                                                             1
pub const mmSMI_FILTER_REGISTER_2_1: u32 = 0x0009;
#define mmSMI_FILTER_REGISTER_2_1_BASE_IDX                                                             1
pub const mmSMI_FILTER_REGISTER_3_0: u32 = 0x000a;
#define mmSMI_FILTER_REGISTER_3_0_BASE_IDX                                                             1
pub const mmSMI_FILTER_REGISTER_3_1: u32 = 0x000b;
#define mmSMI_FILTER_REGISTER_3_1_BASE_IDX                                                             1
pub const mmIOMMU_MMIO_GA_LOG_BASE_0: u32 = 0x0024;
#define mmIOMMU_MMIO_GA_LOG_BASE_0_BASE_IDX                                                            1
pub const mmIOMMU_MMIO_GA_LOG_BASE_1: u32 = 0x0025;
#define mmIOMMU_MMIO_GA_LOG_BASE_1_BASE_IDX                                                            1
pub const mmIOMMU_MMIO_GA_LOG_TAILPTR_ADDR_0: u32 = 0x0026;
#define mmIOMMU_MMIO_GA_LOG_TAILPTR_ADDR_0_BASE_IDX                                                    1
pub const mmIOMMU_MMIO_GA_LOG_TAILPTR_ADDR_1: u32 = 0x0027;
#define mmIOMMU_MMIO_GA_LOG_TAILPTR_ADDR_1_BASE_IDX                                                    1
pub const mmIOMMU_MMIO_PPR_B_BASE_0: u32 = 0x0028;
#define mmIOMMU_MMIO_PPR_B_BASE_0_BASE_IDX                                                             1
pub const mmIOMMU_MMIO_PPR_B_BASE_1: u32 = 0x0029;
#define mmIOMMU_MMIO_PPR_B_BASE_1_BASE_IDX                                                             1
pub const mmIOMMU_MMIO_EVENT_B_BASE_0: u32 = 0x002a;
#define mmIOMMU_MMIO_EVENT_B_BASE_0_BASE_IDX                                                           1
pub const mmIOMMU_MMIO_EVENT_B_BASE_1: u32 = 0x002b;
#define mmIOMMU_MMIO_EVENT_B_BASE_1_BASE_IDX                                                           1
pub const mmIOMMU_MMIO_DEVTBL_1_BASE_0: u32 = 0x002c;
#define mmIOMMU_MMIO_DEVTBL_1_BASE_0_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DEVTBL_1_BASE_1: u32 = 0x002d;
#define mmIOMMU_MMIO_DEVTBL_1_BASE_1_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DEVTBL_2_BASE_0: u32 = 0x002e;
#define mmIOMMU_MMIO_DEVTBL_2_BASE_0_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DEVTBL_2_BASE_1: u32 = 0x002f;
#define mmIOMMU_MMIO_DEVTBL_2_BASE_1_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DEVTBL_3_BASE_0: u32 = 0x0030;
#define mmIOMMU_MMIO_DEVTBL_3_BASE_0_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DEVTBL_3_BASE_1: u32 = 0x0031;
#define mmIOMMU_MMIO_DEVTBL_3_BASE_1_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DEVTBL_4_BASE_0: u32 = 0x0032;
#define mmIOMMU_MMIO_DEVTBL_4_BASE_0_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DEVTBL_4_BASE_1: u32 = 0x0033;
#define mmIOMMU_MMIO_DEVTBL_4_BASE_1_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DEVTBL_5_BASE_0: u32 = 0x0034;
#define mmIOMMU_MMIO_DEVTBL_5_BASE_0_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DEVTBL_5_BASE_1: u32 = 0x0035;
#define mmIOMMU_MMIO_DEVTBL_5_BASE_1_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DEVTBL_6_BASE_0: u32 = 0x0036;
#define mmIOMMU_MMIO_DEVTBL_6_BASE_0_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DEVTBL_6_BASE_1: u32 = 0x0037;
#define mmIOMMU_MMIO_DEVTBL_6_BASE_1_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DEVTBL_7_BASE_0: u32 = 0x0038;
#define mmIOMMU_MMIO_DEVTBL_7_BASE_0_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DEVTBL_7_BASE_1: u32 = 0x0039;
#define mmIOMMU_MMIO_DEVTBL_7_BASE_1_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_DSFX: u32 = 0x003a;
#define mmIOMMU_MMIO_DSFX_BASE_IDX                                                                     1
pub const mmIOMMU_MMIO_DSCX: u32 = 0x003c;
#define mmIOMMU_MMIO_DSCX_BASE_IDX                                                                     1
pub const mmIOMMU_MMIO_DSSX: u32 = 0x003e;
#define mmIOMMU_MMIO_DSSX_BASE_IDX                                                                     1
pub const mmIOMMU_MMIO_CAP_MISC: u32 = 0x0040;
#define mmIOMMU_MMIO_CAP_MISC_BASE_IDX                                                                 1
pub const mmIOMMU_MMIO_CAP_MISC_1: u32 = 0x0041;
#define mmIOMMU_MMIO_CAP_MISC_1_BASE_IDX                                                               1
pub const mmIOMMU_MMIO_MSI_CAP: u32 = 0x0042;
#define mmIOMMU_MMIO_MSI_CAP_BASE_IDX                                                                  1
pub const mmIOMMU_MMIO_MSI_ADDR_LO: u32 = 0x0043;
#define mmIOMMU_MMIO_MSI_ADDR_LO_BASE_IDX                                                              1
pub const mmIOMMU_MMIO_MSI_ADDR_HI: u32 = 0x0044;
#define mmIOMMU_MMIO_MSI_ADDR_HI_BASE_IDX                                                              1
pub const mmIOMMU_MMIO_MSI_DATA: u32 = 0x0045;
#define mmIOMMU_MMIO_MSI_DATA_BASE_IDX                                                                 1
pub const mmIOMMU_MMIO_MSI_MAPPING_CAP: u32 = 0x0046;
#define mmIOMMU_MMIO_MSI_MAPPING_CAP_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_CONTROL_W: u32 = 0x0047;
#define mmIOMMU_MMIO_CONTROL_W_BASE_IDX                                                                1
pub const mmIOMMU_MARC_BASE_LO_0: u32 = 0x006c;
#define mmIOMMU_MARC_BASE_LO_0_BASE_IDX                                                                1
pub const mmIOMMU_MARC_BASE_HI_0: u32 = 0x006d;
#define mmIOMMU_MARC_BASE_HI_0_BASE_IDX                                                                1
pub const mmIOMMU_MARC_RELOC_LO_0: u32 = 0x006e;
#define mmIOMMU_MARC_RELOC_LO_0_BASE_IDX                                                               1
pub const mmIOMMU_MARC_RELOC_HI_0: u32 = 0x006f;
#define mmIOMMU_MARC_RELOC_HI_0_BASE_IDX                                                               1
pub const mmIOMMU_MARC_LEN_LO_0: u32 = 0x0070;
#define mmIOMMU_MARC_LEN_LO_0_BASE_IDX                                                                 1
pub const mmIOMMU_MARC_LEN_HI_0: u32 = 0x0071;
#define mmIOMMU_MARC_LEN_HI_0_BASE_IDX                                                                 1
pub const mmIOMMU_MARC_BASE_LO_1: u32 = 0x0072;
#define mmIOMMU_MARC_BASE_LO_1_BASE_IDX                                                                1
pub const mmIOMMU_MARC_BASE_HI_1: u32 = 0x0073;
#define mmIOMMU_MARC_BASE_HI_1_BASE_IDX                                                                1
pub const mmIOMMU_MARC_RELOC_LO_1: u32 = 0x0074;
#define mmIOMMU_MARC_RELOC_LO_1_BASE_IDX                                                               1
pub const mmIOMMU_MARC_RELOC_HI_1: u32 = 0x0075;
#define mmIOMMU_MARC_RELOC_HI_1_BASE_IDX                                                               1
pub const mmIOMMU_MARC_LEN_LO_1: u32 = 0x0076;
#define mmIOMMU_MARC_LEN_LO_1_BASE_IDX                                                                 1
pub const mmIOMMU_MARC_LEN_HI_1: u32 = 0x0077;
#define mmIOMMU_MARC_LEN_HI_1_BASE_IDX                                                                 1
pub const mmIOMMU_MARC_BASE_LO_2: u32 = 0x0078;
#define mmIOMMU_MARC_BASE_LO_2_BASE_IDX                                                                1
pub const mmIOMMU_MARC_BASE_HI_2: u32 = 0x0079;
#define mmIOMMU_MARC_BASE_HI_2_BASE_IDX                                                                1
pub const mmIOMMU_MARC_RELOC_LO_2: u32 = 0x007a;
#define mmIOMMU_MARC_RELOC_LO_2_BASE_IDX                                                               1
pub const mmIOMMU_MARC_RELOC_HI_2: u32 = 0x007b;
#define mmIOMMU_MARC_RELOC_HI_2_BASE_IDX                                                               1
pub const mmIOMMU_MARC_LEN_LO_2: u32 = 0x007c;
#define mmIOMMU_MARC_LEN_LO_2_BASE_IDX                                                                 1
pub const mmIOMMU_MARC_LEN_HI_2: u32 = 0x007d;
#define mmIOMMU_MARC_LEN_HI_2_BASE_IDX                                                                 1
pub const mmIOMMU_MARC_BASE_LO_3: u32 = 0x007e;
#define mmIOMMU_MARC_BASE_LO_3_BASE_IDX                                                                1
pub const mmIOMMU_MARC_BASE_HI_3: u32 = 0x007f;
#define mmIOMMU_MARC_BASE_HI_3_BASE_IDX                                                                1
pub const mmIOMMU_MARC_RELOC_LO_3: u32 = 0x0080;
#define mmIOMMU_MARC_RELOC_LO_3_BASE_IDX                                                               1
pub const mmIOMMU_MARC_RELOC_HI_3: u32 = 0x0081;
#define mmIOMMU_MARC_RELOC_HI_3_BASE_IDX                                                               1
pub const mmIOMMU_MARC_LEN_LO_3: u32 = 0x0082;
#define mmIOMMU_MARC_LEN_LO_3_BASE_IDX                                                                 1
pub const mmIOMMU_MARC_LEN_HI_3: u32 = 0x0083;
#define mmIOMMU_MARC_LEN_HI_3_BASE_IDX                                                                 1
pub const mmIOMMU_MMIO_CMD_BUF_HDPTR_0: u32 = 0x07ec;
#define mmIOMMU_MMIO_CMD_BUF_HDPTR_0_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_CMD_BUF_HDPTR_1: u32 = 0x07ed;
#define mmIOMMU_MMIO_CMD_BUF_HDPTR_1_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_CMD_BUF_TAILPTR_0: u32 = 0x07ee;
#define mmIOMMU_MMIO_CMD_BUF_TAILPTR_0_BASE_IDX                                                        1
pub const mmIOMMU_MMIO_CMD_BUF_TAILPTR_1: u32 = 0x07ef;
#define mmIOMMU_MMIO_CMD_BUF_TAILPTR_1_BASE_IDX                                                        1
pub const mmIOMMU_MMIO_EVENT_BUF_HDPTR_0: u32 = 0x07f0;
#define mmIOMMU_MMIO_EVENT_BUF_HDPTR_0_BASE_IDX                                                        1
pub const mmIOMMU_MMIO_EVENT_BUF_HDPTR_1: u32 = 0x07f1;
#define mmIOMMU_MMIO_EVENT_BUF_HDPTR_1_BASE_IDX                                                        1
pub const mmIOMMU_MMIO_EVENT_BUF_TAILPTR_0: u32 = 0x07f2;
#define mmIOMMU_MMIO_EVENT_BUF_TAILPTR_0_BASE_IDX                                                      1
pub const mmIOMMU_MMIO_EVENT_BUF_TAILPTR_1: u32 = 0x07f3;
#define mmIOMMU_MMIO_EVENT_BUF_TAILPTR_1_BASE_IDX                                                      1
pub const mmIOMMU_MMIO_STATUS_0: u32 = 0x07f4;
#define mmIOMMU_MMIO_STATUS_0_BASE_IDX                                                                 1
pub const mmIOMMU_MMIO_STATUS_1: u32 = 0x07f5;
#define mmIOMMU_MMIO_STATUS_1_BASE_IDX                                                                 1
pub const mmIOMMU_MMIO_PPR_BUF_HDPTR_0: u32 = 0x07f8;
#define mmIOMMU_MMIO_PPR_BUF_HDPTR_0_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_PPR_BUF_HDPTR_1: u32 = 0x07f9;
#define mmIOMMU_MMIO_PPR_BUF_HDPTR_1_BASE_IDX                                                          1
pub const mmIOMMU_MMIO_PPR_BUF_TAILPTR_0: u32 = 0x07fa;
#define mmIOMMU_MMIO_PPR_BUF_TAILPTR_0_BASE_IDX                                                        1
pub const mmIOMMU_MMIO_PPR_BUF_TAILPTR_1: u32 = 0x07fb;
#define mmIOMMU_MMIO_PPR_BUF_TAILPTR_1_BASE_IDX                                                        1
pub const mmIOMMU_MMIO_GA_BUF_HDPTR_0: u32 = 0x07fc;
#define mmIOMMU_MMIO_GA_BUF_HDPTR_0_BASE_IDX                                                           1
pub const mmIOMMU_MMIO_GA_BUF_HDPTR_1: u32 = 0x07fd;
#define mmIOMMU_MMIO_GA_BUF_HDPTR_1_BASE_IDX                                                           1
pub const mmIOMMU_MMIO_GA_BUF_TAILPTR_0: u32 = 0x07fe;
#define mmIOMMU_MMIO_GA_BUF_TAILPTR_0_BASE_IDX                                                         1
pub const mmIOMMU_MMIO_GA_BUF_TAILPTR_1: u32 = 0x07ff;
#define mmIOMMU_MMIO_GA_BUF_TAILPTR_1_BASE_IDX                                                         1
pub const mmIOMMU_MMIO_PPR_B_BUF_HDPTR_0: u32 = 0x0800;
#define mmIOMMU_MMIO_PPR_B_BUF_HDPTR_0_BASE_IDX                                                        1
pub const mmIOMMU_MMIO_PPR_B_BUF_HDPTR_1: u32 = 0x0801;
#define mmIOMMU_MMIO_PPR_B_BUF_HDPTR_1_BASE_IDX                                                        1
pub const mmIOMMU_MMIO_PPR_B_BUF_TAILPTR_0: u32 = 0x0802;
#define mmIOMMU_MMIO_PPR_B_BUF_TAILPTR_0_BASE_IDX                                                      1
pub const mmIOMMU_MMIO_PPR_B_BUF_TAILPTR_1: u32 = 0x0803;
#define mmIOMMU_MMIO_PPR_B_BUF_TAILPTR_1_BASE_IDX                                                      1
pub const mmIOMMU_MMIO_EVENT_B_BUF_HDPTR_0: u32 = 0x0808;
#define mmIOMMU_MMIO_EVENT_B_BUF_HDPTR_0_BASE_IDX                                                      1
pub const mmIOMMU_MMIO_EVENT_B_BUF_HDPTR_1: u32 = 0x0809;
#define mmIOMMU_MMIO_EVENT_B_BUF_HDPTR_1_BASE_IDX                                                      1
pub const mmIOMMU_MMIO_EVENT_B_BUF_TAILPTR_0: u32 = 0x080a;
#define mmIOMMU_MMIO_EVENT_B_BUF_TAILPTR_0_BASE_IDX                                                    1
pub const mmIOMMU_MMIO_EVENT_B_BUF_TAILPTR_1: u32 = 0x080b;
#define mmIOMMU_MMIO_EVENT_B_BUF_TAILPTR_1_BASE_IDX                                                    1
pub const mmIOMMU_MMIO_PPR_AUTORESP_0: u32 = 0x080c;
#define mmIOMMU_MMIO_PPR_AUTORESP_0_BASE_IDX                                                           1
pub const mmIOMMU_MMIO_PPR_OVERFLOW_EARLY_0: u32 = 0x080e;
#define mmIOMMU_MMIO_PPR_OVERFLOW_EARLY_0_BASE_IDX                                                     1
pub const mmIOMMU_MMIO_PPR_B_OVERFLOW_EARLY_0: u32 = 0x0810;
#define mmIOMMU_MMIO_PPR_B_OVERFLOW_EARLY_0_BASE_IDX                                                   1
pub const mmIOMMU_MMIO_COUNTER_CONFIG_0: u32 = 0x02e0;
#define mmIOMMU_MMIO_COUNTER_CONFIG_0_BASE_IDX                                                         2
pub const mmIOMMU_MMIO_COUNTER_CONFIG_1: u32 = 0x02e1;
#define mmIOMMU_MMIO_COUNTER_CONFIG_1_BASE_IDX                                                         2
pub const mmIOMMU_MMIO_COUNTER_PASID_BANK_LOCK_0: u32 = 0x02e2;
#define mmIOMMU_MMIO_COUNTER_PASID_BANK_LOCK_0_BASE_IDX                                                2
pub const mmIOMMU_MMIO_COUNTER_PASID_BANK_LOCK_1: u32 = 0x02e3;
#define mmIOMMU_MMIO_COUNTER_PASID_BANK_LOCK_1_BASE_IDX                                                2
pub const mmIOMMU_MMIO_COUNTER_DOMAIN_BANK_LOCK_0: u32 = 0x02e4;
#define mmIOMMU_MMIO_COUNTER_DOMAIN_BANK_LOCK_0_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_DOMAIN_BANK_LOCK_1: u32 = 0x02e5;
#define mmIOMMU_MMIO_COUNTER_DOMAIN_BANK_LOCK_1_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_DEVID_BANK_LOCK_0: u32 = 0x02e6;
#define mmIOMMU_MMIO_COUNTER_DEVID_BANK_LOCK_0_BASE_IDX                                                2
pub const mmIOMMU_MMIO_COUNTER_DEVID_BANK_LOCK_1: u32 = 0x02e7;
#define mmIOMMU_MMIO_COUNTER_DEVID_BANK_LOCK_1_BASE_IDX                                                2
pub const mmIOMMU_MMIO_COUNTER_BANK_0_CNT_0_0: u32 = 0xf2e0;
#define mmIOMMU_MMIO_COUNTER_BANK_0_CNT_0_0_BASE_IDX                                                   2
pub const mmIOMMU_MMIO_COUNTER_BANK_0_CNT_0_1: u32 = 0xf2e1;
#define mmIOMMU_MMIO_COUNTER_BANK_0_CNT_0_1_BASE_IDX                                                   2
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_0_0: u32 = 0xf2e2;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_0_0_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_0_1: u32 = 0xf2e3;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_0_1_BASE_IDX                                               2
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_0_0: u32 = 0xf2e4;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_0_0_BASE_IDX                                               2
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_0_1: u32 = 0xf2e5;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_0_1_BASE_IDX                                               2
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_0_0: u32 = 0xf2e6;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_0_0_BASE_IDX                                              2
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_0_1: u32 = 0xf2e7;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_0_1_BASE_IDX                                              2
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_0_0: u32 = 0xf2e8;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_0_0_BASE_IDX                                            2
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_0_1: u32 = 0xf2e9;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_0_1_BASE_IDX                                            2
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_0_0: u32 = 0xf2ea;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_0_0_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_0_1: u32 = 0xf2eb;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_0_1_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_BANK_0_CNT_1_0: u32 = 0xf320;
#define mmIOMMU_MMIO_COUNTER_BANK_0_CNT_1_0_BASE_IDX                                                   2
pub const mmIOMMU_MMIO_COUNTER_BANK_0_CNT_1_1: u32 = 0xf321;
#define mmIOMMU_MMIO_COUNTER_BANK_0_CNT_1_1_BASE_IDX                                                   2
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_1_0: u32 = 0xf322;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_1_0_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_1_1: u32 = 0xf323;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_1_1_BASE_IDX                                               2
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_1_0: u32 = 0xf324;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_1_0_BASE_IDX                                               2
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_1_1: u32 = 0xf325;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_1_1_BASE_IDX                                               2
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_1_0: u32 = 0xf326;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_1_0_BASE_IDX                                              2
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_1_1: u32 = 0xf327;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_1_1_BASE_IDX                                              2
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_1_0: u32 = 0xf328;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_1_0_BASE_IDX                                            2
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_1_1: u32 = 0xf329;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_1_1_BASE_IDX                                            2
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_1_0: u32 = 0xf32a;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_1_0_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_1_1: u32 = 0xf32b;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_1_1_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_BANK_0_CNT_2_0: u32 = 0xf360;
#define mmIOMMU_MMIO_COUNTER_BANK_0_CNT_2_0_BASE_IDX                                                   2
pub const mmIOMMU_MMIO_COUNTER_BANK_0_CNT_2_1: u32 = 0xf361;
#define mmIOMMU_MMIO_COUNTER_BANK_0_CNT_2_1_BASE_IDX                                                   2
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_2_0: u32 = 0xf362;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_2_0_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_2_1: u32 = 0xf363;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_2_1_BASE_IDX                                               2
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_2_0: u32 = 0xf364;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_2_0_BASE_IDX                                               2
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_2_1: u32 = 0xf365;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_2_1_BASE_IDX                                               2
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_2_0: u32 = 0xf366;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_2_0_BASE_IDX                                              2
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_2_1: u32 = 0xf367;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_2_1_BASE_IDX                                              2
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_2_0: u32 = 0xf368;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_2_0_BASE_IDX                                            2
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_2_1: u32 = 0xf369;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_2_1_BASE_IDX                                            2
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_2_0: u32 = 0xf36a;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_2_0_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_2_1: u32 = 0xf36b;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_2_1_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_BANK_0_CNT_3_0: u32 = 0xf3a0;
#define mmIOMMU_MMIO_COUNTER_BANK_0_CNT_3_0_BASE_IDX                                                   2
pub const mmIOMMU_MMIO_COUNTER_BANK_0_CNT_3_1: u32 = 0xf3a1;
#define mmIOMMU_MMIO_COUNTER_BANK_0_CNT_3_1_BASE_IDX                                                   2
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_3_0: u32 = 0xf3a2;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_3_0_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_3_1: u32 = 0xf3a3;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_0_CNT_3_1_BASE_IDX                                               2
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_3_0: u32 = 0xf3a4;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_3_0_BASE_IDX                                               2
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_3_1: u32 = 0xf3a5;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_0_CNT_3_1_BASE_IDX                                               2
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_3_0: u32 = 0xf3a6;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_3_0_BASE_IDX                                              2
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_3_1: u32 = 0xf3a7;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_0_CNT_3_1_BASE_IDX                                              2
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_3_0: u32 = 0xf3a8;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_3_0_BASE_IDX                                            2
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_3_1: u32 = 0xf3a9;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_0_CNT_3_1_BASE_IDX                                            2
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_3_0: u32 = 0xf3aa;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_3_0_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_3_1: u32 = 0xf3ab;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_0_CNT_3_1_BASE_IDX                                               2
pub const mmIOMMU_MMIO_COUNTER_BANK_1_CNT_0_0: u32 = 0x0000;
#define mmIOMMU_MMIO_COUNTER_BANK_1_CNT_0_0_BASE_IDX                                                   3
pub const mmIOMMU_MMIO_COUNTER_BANK_1_CNT_0_1: u32 = 0x0001;
#define mmIOMMU_MMIO_COUNTER_BANK_1_CNT_0_1_BASE_IDX                                                   3
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_0_0: u32 = 0x0002;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_0_0_BASE_IDX                                               3
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_0_1: u32 = 0x0003;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_0_1_BASE_IDX                                               3
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_0_0: u32 = 0x0004;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_0_0_BASE_IDX                                               3
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_0_1: u32 = 0x0005;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_0_1_BASE_IDX                                               3
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_0_0: u32 = 0x0006;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_0_0_BASE_IDX                                              3
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_0_1: u32 = 0x0007;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_0_1_BASE_IDX                                              3
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_0_0: u32 = 0x0008;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_0_0_BASE_IDX                                            3
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_0_1: u32 = 0x0009;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_0_1_BASE_IDX                                            3
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_0_0: u32 = 0x000a;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_0_0_BASE_IDX                                               3
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_0_1: u32 = 0x000b;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_0_1_BASE_IDX                                               3
pub const mmIOMMU_MMIO_COUNTER_BANK_1_CNT_1_0: u32 = 0x0040;
#define mmIOMMU_MMIO_COUNTER_BANK_1_CNT_1_0_BASE_IDX                                                   3
pub const mmIOMMU_MMIO_COUNTER_BANK_1_CNT_1_1: u32 = 0x0041;
#define mmIOMMU_MMIO_COUNTER_BANK_1_CNT_1_1_BASE_IDX                                                   3
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_1_0: u32 = 0x0042;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_1_0_BASE_IDX                                               3
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_1_1: u32 = 0x0043;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_1_1_BASE_IDX                                               3
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_1_0: u32 = 0x0044;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_1_0_BASE_IDX                                               3
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_1_1: u32 = 0x0045;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_1_1_BASE_IDX                                               3
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_1_0: u32 = 0x0046;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_1_0_BASE_IDX                                              3
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_1_1: u32 = 0x0047;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_1_1_BASE_IDX                                              3
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_1_0: u32 = 0x0048;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_1_0_BASE_IDX                                            3
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_1_1: u32 = 0x0049;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_1_1_BASE_IDX                                            3
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_1_0: u32 = 0x004a;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_1_0_BASE_IDX                                               3
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_1_1: u32 = 0x004b;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_1_1_BASE_IDX                                               3
pub const mmIOMMU_MMIO_COUNTER_BANK_1_CNT_2_0: u32 = 0x0080;
#define mmIOMMU_MMIO_COUNTER_BANK_1_CNT_2_0_BASE_IDX                                                   3
pub const mmIOMMU_MMIO_COUNTER_BANK_1_CNT_2_1: u32 = 0x0081;
#define mmIOMMU_MMIO_COUNTER_BANK_1_CNT_2_1_BASE_IDX                                                   3
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_2_0: u32 = 0x0082;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_2_0_BASE_IDX                                               3
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_2_1: u32 = 0x0083;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_2_1_BASE_IDX                                               3
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_2_0: u32 = 0x0084;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_2_0_BASE_IDX                                               3
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_2_1: u32 = 0x0085;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_2_1_BASE_IDX                                               3
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_2_0: u32 = 0x0086;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_2_0_BASE_IDX                                              3
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_2_1: u32 = 0x0087;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_2_1_BASE_IDX                                              3
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_2_0: u32 = 0x0088;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_2_0_BASE_IDX                                            3
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_2_1: u32 = 0x0089;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_2_1_BASE_IDX                                            3
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_2_0: u32 = 0x008a;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_2_0_BASE_IDX                                               3
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_2_1: u32 = 0x008b;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_2_1_BASE_IDX                                               3
pub const mmIOMMU_MMIO_COUNTER_BANK_1_CNT_3_0: u32 = 0x00c0;
#define mmIOMMU_MMIO_COUNTER_BANK_1_CNT_3_0_BASE_IDX                                                   3
pub const mmIOMMU_MMIO_COUNTER_BANK_1_CNT_3_1: u32 = 0x00c1;
#define mmIOMMU_MMIO_COUNTER_BANK_1_CNT_3_1_BASE_IDX                                                   3
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_3_0: u32 = 0x00c2;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_3_0_BASE_IDX                                               3
pub const mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_3_1: u32 = 0x00c3;
#define mmIOMMU_MMIO_COUNTER_SRC_BANK_1_CNT_3_1_BASE_IDX                                               3
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_3_0: u32 = 0x00c4;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_3_0_BASE_IDX                                               3
pub const mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_3_1: u32 = 0x00c5;
#define mmIOMMU_MMIO_PASID_MATCH_BANK_1_CNT_3_1_BASE_IDX                                               3
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_3_0: u32 = 0x00c6;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_3_0_BASE_IDX                                              3
pub const mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_3_1: u32 = 0x00c7;
#define mmIOMMU_MMIO_DOMAIN_MATCH_BANK_1_CNT_3_1_BASE_IDX                                              3
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_3_0: u32 = 0x00c8;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_3_0_BASE_IDX                                            3
pub const mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_3_1: u32 = 0x00c9;
#define mmIOMMU_MMIO_DEVICEID_MATCH_BANK_1_CNT_3_1_BASE_IDX                                            3
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_3_0: u32 = 0x00ca;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_3_0_BASE_IDX                                               3
pub const mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_3_1: u32 = 0x00cb;
#define mmIOMMU_MMIO_COUNTER_RPT_BANK_1_CNT_3_1_BASE_IDX                                               3


// addressBlock: nbio_nbif0_bif_bx_pf_SYSPFVFDEC
// base address: 0x0
pub const mmMM_INDEX: u32 = 0x0000;
#define mmMM_INDEX_BASE_IDX                                                                            0
pub const mmMM_DATA: u32 = 0x0001;
#define mmMM_DATA_BASE_IDX                                                                             0
pub const mmMM_INDEX_HI: u32 = 0x0006;
#define mmMM_INDEX_HI_BASE_IDX                                                                         0


// addressBlock: nbio_nbif0_bif_bx_pf_SYSDEC
// base address: 0x0
pub const mmSYSHUB_INDEX_OVLP: u32 = 0x0008;
#define mmSYSHUB_INDEX_OVLP_BASE_IDX                                                                   0
pub const mmSYSHUB_DATA_OVLP: u32 = 0x0009;
#define mmSYSHUB_DATA_OVLP_BASE_IDX                                                                    0
pub const mmPCIE_INDEX: u32 = 0x000c;
#define mmPCIE_INDEX_BASE_IDX                                                                          0
pub const mmPCIE_DATA: u32 = 0x000d;
#define mmPCIE_DATA_BASE_IDX                                                                           0
pub const mmPCIE_INDEX2: u32 = 0x000e;
#define mmPCIE_INDEX2_BASE_IDX                                                                         0
pub const mmPCIE_DATA2: u32 = 0x000f;
#define mmPCIE_DATA2_BASE_IDX                                                                          0
pub const mmSBIOS_SCRATCH_0: u32 = 0x0034;
#define mmSBIOS_SCRATCH_0_BASE_IDX                                                                     1
pub const mmSBIOS_SCRATCH_1: u32 = 0x0035;
#define mmSBIOS_SCRATCH_1_BASE_IDX                                                                     1
pub const mmSBIOS_SCRATCH_2: u32 = 0x0036;
#define mmSBIOS_SCRATCH_2_BASE_IDX                                                                     1
pub const mmSBIOS_SCRATCH_3: u32 = 0x0037;
#define mmSBIOS_SCRATCH_3_BASE_IDX                                                                     1
pub const mmBIOS_SCRATCH_0: u32 = 0x0038;
#define mmBIOS_SCRATCH_0_BASE_IDX                                                                      1
pub const mmBIOS_SCRATCH_1: u32 = 0x0039;
#define mmBIOS_SCRATCH_1_BASE_IDX                                                                      1
pub const mmBIOS_SCRATCH_2: u32 = 0x003a;
#define mmBIOS_SCRATCH_2_BASE_IDX                                                                      1
pub const mmBIOS_SCRATCH_3: u32 = 0x003b;
#define mmBIOS_SCRATCH_3_BASE_IDX                                                                      1
pub const mmBIOS_SCRATCH_4: u32 = 0x003c;
#define mmBIOS_SCRATCH_4_BASE_IDX                                                                      1
pub const mmBIOS_SCRATCH_5: u32 = 0x003d;
#define mmBIOS_SCRATCH_5_BASE_IDX                                                                      1
pub const mmBIOS_SCRATCH_6: u32 = 0x003e;
#define mmBIOS_SCRATCH_6_BASE_IDX                                                                      1
pub const mmBIOS_SCRATCH_7: u32 = 0x003f;
#define mmBIOS_SCRATCH_7_BASE_IDX                                                                      1
pub const mmBIOS_SCRATCH_8: u32 = 0x0040;
#define mmBIOS_SCRATCH_8_BASE_IDX                                                                      1
pub const mmBIOS_SCRATCH_9: u32 = 0x0041;
#define mmBIOS_SCRATCH_9_BASE_IDX                                                                      1
pub const mmBIOS_SCRATCH_10: u32 = 0x0042;
#define mmBIOS_SCRATCH_10_BASE_IDX                                                                     1
pub const mmBIOS_SCRATCH_11: u32 = 0x0043;
#define mmBIOS_SCRATCH_11_BASE_IDX                                                                     1
pub const mmBIOS_SCRATCH_12: u32 = 0x0044;
#define mmBIOS_SCRATCH_12_BASE_IDX                                                                     1
pub const mmBIOS_SCRATCH_13: u32 = 0x0045;
#define mmBIOS_SCRATCH_13_BASE_IDX                                                                     1
pub const mmBIOS_SCRATCH_14: u32 = 0x0046;
#define mmBIOS_SCRATCH_14_BASE_IDX                                                                     1
pub const mmBIOS_SCRATCH_15: u32 = 0x0047;
#define mmBIOS_SCRATCH_15_BASE_IDX                                                                     1
pub const mmBIF_RLC_INTR_CNTL: u32 = 0x004c;
#define mmBIF_RLC_INTR_CNTL_BASE_IDX                                                                   1
pub const mmBIF_VCE_INTR_CNTL: u32 = 0x004d;
#define mmBIF_VCE_INTR_CNTL_BASE_IDX                                                                   1
pub const mmBIF_UVD_INTR_CNTL: u32 = 0x004e;
#define mmBIF_UVD_INTR_CNTL_BASE_IDX                                                                   1
pub const mmGFX_MMIOREG_CAM_ADDR0: u32 = 0x006c;
#define mmGFX_MMIOREG_CAM_ADDR0_BASE_IDX                                                               1
pub const mmGFX_MMIOREG_CAM_REMAP_ADDR0: u32 = 0x006d;
#define mmGFX_MMIOREG_CAM_REMAP_ADDR0_BASE_IDX                                                         1
pub const mmGFX_MMIOREG_CAM_ADDR1: u32 = 0x006e;
#define mmGFX_MMIOREG_CAM_ADDR1_BASE_IDX                                                               1
pub const mmGFX_MMIOREG_CAM_REMAP_ADDR1: u32 = 0x006f;
#define mmGFX_MMIOREG_CAM_REMAP_ADDR1_BASE_IDX                                                         1
pub const mmGFX_MMIOREG_CAM_ADDR2: u32 = 0x0070;
#define mmGFX_MMIOREG_CAM_ADDR2_BASE_IDX                                                               1
pub const mmGFX_MMIOREG_CAM_REMAP_ADDR2: u32 = 0x0071;
#define mmGFX_MMIOREG_CAM_REMAP_ADDR2_BASE_IDX                                                         1
pub const mmGFX_MMIOREG_CAM_ADDR3: u32 = 0x0072;
#define mmGFX_MMIOREG_CAM_ADDR3_BASE_IDX                                                               1
pub const mmGFX_MMIOREG_CAM_REMAP_ADDR3: u32 = 0x0073;
#define mmGFX_MMIOREG_CAM_REMAP_ADDR3_BASE_IDX                                                         1
pub const mmGFX_MMIOREG_CAM_ADDR4: u32 = 0x0074;
#define mmGFX_MMIOREG_CAM_ADDR4_BASE_IDX                                                               1
pub const mmGFX_MMIOREG_CAM_REMAP_ADDR4: u32 = 0x0075;
#define mmGFX_MMIOREG_CAM_REMAP_ADDR4_BASE_IDX                                                         1
pub const mmGFX_MMIOREG_CAM_ADDR5: u32 = 0x0076;
#define mmGFX_MMIOREG_CAM_ADDR5_BASE_IDX                                                               1
pub const mmGFX_MMIOREG_CAM_REMAP_ADDR5: u32 = 0x0077;
#define mmGFX_MMIOREG_CAM_REMAP_ADDR5_BASE_IDX                                                         1
pub const mmGFX_MMIOREG_CAM_ADDR6: u32 = 0x0078;
#define mmGFX_MMIOREG_CAM_ADDR6_BASE_IDX                                                               1
pub const mmGFX_MMIOREG_CAM_REMAP_ADDR6: u32 = 0x0079;
#define mmGFX_MMIOREG_CAM_REMAP_ADDR6_BASE_IDX                                                         1
pub const mmGFX_MMIOREG_CAM_ADDR7: u32 = 0x007a;
#define mmGFX_MMIOREG_CAM_ADDR7_BASE_IDX                                                               1
pub const mmGFX_MMIOREG_CAM_REMAP_ADDR7: u32 = 0x007b;
#define mmGFX_MMIOREG_CAM_REMAP_ADDR7_BASE_IDX                                                         1
pub const mmGFX_MMIOREG_CAM_CNTL: u32 = 0x007c;
#define mmGFX_MMIOREG_CAM_CNTL_BASE_IDX                                                                1
pub const mmGFX_MMIOREG_CAM_ZERO_CPL: u32 = 0x007d;
#define mmGFX_MMIOREG_CAM_ZERO_CPL_BASE_IDX                                                            1
pub const mmGFX_MMIOREG_CAM_ONE_CPL: u32 = 0x007e;
#define mmGFX_MMIOREG_CAM_ONE_CPL_BASE_IDX                                                             1
pub const mmGFX_MMIOREG_CAM_PROGRAMMABLE_CPL: u32 = 0x007f;
#define mmGFX_MMIOREG_CAM_PROGRAMMABLE_CPL_BASE_IDX                                                    1


// addressBlock: nbio_nbif0_syshub_mmreg_ind_syshubdec
// base address: 0x0
pub const mmSYSHUB_INDEX: u32 = 0x0008;
#define mmSYSHUB_INDEX_BASE_IDX                                                                        0
pub const mmSYSHUB_DATA: u32 = 0x0009;
#define mmSYSHUB_DATA_BASE_IDX                                                                         0


// addressBlock: nbio_nbif0_rcc_strap_BIFDEC1
// base address: 0x0
pub const mmRCC_DEV0_EPF0_STRAP0: u32 = 0x000f;
#define mmRCC_DEV0_EPF0_STRAP0_BASE_IDX                                                                2


// addressBlock: nbio_nbif0_rcc_ep_dev0_BIFDEC1
// base address: 0x0
pub const mmEP_PCIE_SCRATCH: u32 = 0x0023;
#define mmEP_PCIE_SCRATCH_BASE_IDX                                                                     2
pub const mmEP_PCIE_CNTL: u32 = 0x0025;
#define mmEP_PCIE_CNTL_BASE_IDX                                                                        2
pub const mmEP_PCIE_INT_CNTL: u32 = 0x0026;
#define mmEP_PCIE_INT_CNTL_BASE_IDX                                                                    2
pub const mmEP_PCIE_INT_STATUS: u32 = 0x0027;
#define mmEP_PCIE_INT_STATUS_BASE_IDX                                                                  2
pub const mmEP_PCIE_RX_CNTL2: u32 = 0x0028;
#define mmEP_PCIE_RX_CNTL2_BASE_IDX                                                                    2
pub const mmEP_PCIE_BUS_CNTL: u32 = 0x0029;
#define mmEP_PCIE_BUS_CNTL_BASE_IDX                                                                    2
pub const mmEP_PCIE_CFG_CNTL: u32 = 0x002a;
#define mmEP_PCIE_CFG_CNTL_BASE_IDX                                                                    2
pub const mmEP_PCIE_TX_LTR_CNTL: u32 = 0x002c;
#define mmEP_PCIE_TX_LTR_CNTL_BASE_IDX                                                                 2
pub const mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_0: u32 = 0x002d;
#define mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_0_BASE_IDX                                                    2
pub const mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_1: u32 = 0x002d;
#define mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_1_BASE_IDX                                                    2
pub const mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_2: u32 = 0x002d;
#define mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_2_BASE_IDX                                                    2
pub const mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_3: u32 = 0x002d;
#define mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_3_BASE_IDX                                                    2
pub const mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_4: u32 = 0x002e;
#define mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_4_BASE_IDX                                                    2
pub const mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_5: u32 = 0x002e;
#define mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_5_BASE_IDX                                                    2
pub const mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_6: u32 = 0x002e;
#define mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_6_BASE_IDX                                                    2
pub const mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_7: u32 = 0x002e;
#define mmPCIE_F1_DPA_SUBSTATE_PWR_ALLOC_7_BASE_IDX                                                    2
pub const mmEP_PCIE_F0_DPA_CAP: u32 = 0x0032;
#define mmEP_PCIE_F0_DPA_CAP_BASE_IDX                                                                  2
pub const mmEP_PCIE_F0_DPA_LATENCY_INDICATOR: u32 = 0x0033;
#define mmEP_PCIE_F0_DPA_LATENCY_INDICATOR_BASE_IDX                                                    2
pub const mmEP_PCIE_F0_DPA_CNTL: u32 = 0x0033;
#define mmEP_PCIE_F0_DPA_CNTL_BASE_IDX                                                                 2
pub const mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_0: u32 = 0x0033;
#define mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_0_BASE_IDX                                                    2
pub const mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_1: u32 = 0x0034;
#define mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_1_BASE_IDX                                                    2
pub const mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_2: u32 = 0x0034;
#define mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_2_BASE_IDX                                                    2
pub const mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_3: u32 = 0x0034;
#define mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_3_BASE_IDX                                                    2
pub const mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_4: u32 = 0x0034;
#define mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_4_BASE_IDX                                                    2
pub const mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_5: u32 = 0x0035;
#define mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_5_BASE_IDX                                                    2
pub const mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_6: u32 = 0x0035;
#define mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_6_BASE_IDX                                                    2
pub const mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_7: u32 = 0x0035;
#define mmPCIE_F0_DPA_SUBSTATE_PWR_ALLOC_7_BASE_IDX                                                    2
pub const mmEP_PCIE_PME_CONTROL: u32 = 0x0035;
#define mmEP_PCIE_PME_CONTROL_BASE_IDX                                                                 2
pub const mmEP_PCIEP_RESERVED: u32 = 0x0036;
#define mmEP_PCIEP_RESERVED_BASE_IDX                                                                   2
pub const mmEP_PCIE_TX_CNTL: u32 = 0x0038;
#define mmEP_PCIE_TX_CNTL_BASE_IDX                                                                     2
pub const mmEP_PCIE_TX_REQUESTER_ID: u32 = 0x0039;
#define mmEP_PCIE_TX_REQUESTER_ID_BASE_IDX                                                             2
pub const mmEP_PCIE_ERR_CNTL: u32 = 0x003a;
#define mmEP_PCIE_ERR_CNTL_BASE_IDX                                                                    2
pub const mmEP_PCIE_RX_CNTL: u32 = 0x003b;
#define mmEP_PCIE_RX_CNTL_BASE_IDX                                                                     2
pub const mmEP_PCIE_LC_SPEED_CNTL: u32 = 0x003c;
#define mmEP_PCIE_LC_SPEED_CNTL_BASE_IDX                                                               2


// addressBlock: nbio_nbif0_rcc_dwn_dev0_BIFDEC1
// base address: 0x0
pub const mmDN_PCIE_RESERVED: u32 = 0x0040;
#define mmDN_PCIE_RESERVED_BASE_IDX                                                                    2
pub const mmDN_PCIE_SCRATCH: u32 = 0x0041;
#define mmDN_PCIE_SCRATCH_BASE_IDX                                                                     2
pub const mmDN_PCIE_CNTL: u32 = 0x0043;
#define mmDN_PCIE_CNTL_BASE_IDX                                                                        2
pub const mmDN_PCIE_CONFIG_CNTL: u32 = 0x0044;
#define mmDN_PCIE_CONFIG_CNTL_BASE_IDX                                                                 2
pub const mmDN_PCIE_RX_CNTL2: u32 = 0x0045;
#define mmDN_PCIE_RX_CNTL2_BASE_IDX                                                                    2
pub const mmDN_PCIE_BUS_CNTL: u32 = 0x0046;
#define mmDN_PCIE_BUS_CNTL_BASE_IDX                                                                    2
pub const mmDN_PCIE_CFG_CNTL: u32 = 0x0047;
#define mmDN_PCIE_CFG_CNTL_BASE_IDX                                                                    2


// addressBlock: nbio_nbif0_rcc_dwnp_dev0_BIFDEC1
// base address: 0x0
pub const mmPCIE_ERR_CNTL: u32 = 0x004f;
#define mmPCIE_ERR_CNTL_BASE_IDX                                                                       2
pub const mmPCIE_RX_CNTL: u32 = 0x0050;
#define mmPCIE_RX_CNTL_BASE_IDX                                                                        2
pub const mmPCIE_LC_SPEED_CNTL: u32 = 0x0051;
#define mmPCIE_LC_SPEED_CNTL_BASE_IDX                                                                  2
pub const mmPCIE_LC_CNTL2: u32 = 0x0052;
#define mmPCIE_LC_CNTL2_BASE_IDX                                                                       2
pub const mmPCIEP_STRAP_MISC: u32 = 0x0053;
#define mmPCIEP_STRAP_MISC_BASE_IDX                                                                    2
pub const mmLTR_MSG_INFO_FROM_EP: u32 = 0x0054;
#define mmLTR_MSG_INFO_FROM_EP_BASE_IDX                                                                2


// addressBlock: nbio_nbif0_rcc_dev0_BIFPFVFDEC1
// base address: 0x0
pub const mmRCC_ERR_LOG: u32 = 0x0085;
#define mmRCC_ERR_LOG_BASE_IDX                                                                         2
pub const mmRCC_DOORBELL_APER_EN: u32 = 0x00c0;
#define mmRCC_DOORBELL_APER_EN_BASE_IDX                                                                2
pub const mmRCC_CONFIG_MEMSIZE: u32 = 0x00c3;
#define mmRCC_CONFIG_MEMSIZE_BASE_IDX                                                                  2
pub const mmRCC_CONFIG_RESERVED: u32 = 0x00c4;
#define mmRCC_CONFIG_RESERVED_BASE_IDX                                                                 2
pub const mmRCC_IOV_FUNC_IDENTIFIER: u32 = 0x00c5;
#define mmRCC_IOV_FUNC_IDENTIFIER_BASE_IDX                                                             2


// addressBlock: nbio_nbif0_rcc_dev0_BIFDEC1
// base address: 0x0
pub const mmRCC_ERR_INT_CNTL: u32 = 0x0086;
#define mmRCC_ERR_INT_CNTL_BASE_IDX                                                                    2
pub const mmRCC_BACO_CNTL_MISC: u32 = 0x0087;
#define mmRCC_BACO_CNTL_MISC_BASE_IDX                                                                  2
pub const mmRCC_RESET_EN: u32 = 0x0088;
#define mmRCC_RESET_EN_BASE_IDX                                                                        2
pub const mmRCC_VDM_SUPPORT: u32 = 0x0089;
#define mmRCC_VDM_SUPPORT_BASE_IDX                                                                     2
pub const mmRCC_PEER_REG_RANGE0: u32 = 0x00be;
#define mmRCC_PEER_REG_RANGE0_BASE_IDX                                                                 2
pub const mmRCC_PEER_REG_RANGE1: u32 = 0x00bf;
#define mmRCC_PEER_REG_RANGE1_BASE_IDX                                                                 2
pub const mmRCC_BUS_CNTL: u32 = 0x00c1;
#define mmRCC_BUS_CNTL_BASE_IDX                                                                        2
pub const mmRCC_CONFIG_CNTL: u32 = 0x00c2;
#define mmRCC_CONFIG_CNTL_BASE_IDX                                                                     2
pub const mmRCC_CONFIG_F0_BASE: u32 = 0x00c6;
#define mmRCC_CONFIG_F0_BASE_BASE_IDX                                                                  2
pub const mmRCC_CONFIG_APER_SIZE: u32 = 0x00c7;
#define mmRCC_CONFIG_APER_SIZE_BASE_IDX                                                                2
pub const mmRCC_CONFIG_REG_APER_SIZE: u32 = 0x00c8;
#define mmRCC_CONFIG_REG_APER_SIZE_BASE_IDX                                                            2
pub const mmRCC_XDMA_LO: u32 = 0x00c9;
#define mmRCC_XDMA_LO_BASE_IDX                                                                         2
pub const mmRCC_XDMA_HI: u32 = 0x00ca;
#define mmRCC_XDMA_HI_BASE_IDX                                                                         2
pub const mmRCC_FEATURES_CONTROL_MISC: u32 = 0x00cb;
#define mmRCC_FEATURES_CONTROL_MISC_BASE_IDX                                                           2
pub const mmRCC_BUSNUM_CNTL1: u32 = 0x00cc;
#define mmRCC_BUSNUM_CNTL1_BASE_IDX                                                                    2
pub const mmRCC_BUSNUM_LIST0: u32 = 0x00cd;
#define mmRCC_BUSNUM_LIST0_BASE_IDX                                                                    2
pub const mmRCC_BUSNUM_LIST1: u32 = 0x00ce;
#define mmRCC_BUSNUM_LIST1_BASE_IDX                                                                    2
pub const mmRCC_BUSNUM_CNTL2: u32 = 0x00cf;
#define mmRCC_BUSNUM_CNTL2_BASE_IDX                                                                    2
pub const mmRCC_CAPTURE_HOST_BUSNUM: u32 = 0x00d0;
#define mmRCC_CAPTURE_HOST_BUSNUM_BASE_IDX                                                             2
pub const mmRCC_HOST_BUSNUM: u32 = 0x00d1;
#define mmRCC_HOST_BUSNUM_BASE_IDX                                                                     2
pub const mmRCC_PEER0_FB_OFFSET_HI: u32 = 0x00d2;
#define mmRCC_PEER0_FB_OFFSET_HI_BASE_IDX                                                              2
pub const mmRCC_PEER0_FB_OFFSET_LO: u32 = 0x00d3;
#define mmRCC_PEER0_FB_OFFSET_LO_BASE_IDX                                                              2
pub const mmRCC_PEER1_FB_OFFSET_HI: u32 = 0x00d4;
#define mmRCC_PEER1_FB_OFFSET_HI_BASE_IDX                                                              2
pub const mmRCC_PEER1_FB_OFFSET_LO: u32 = 0x00d5;
#define mmRCC_PEER1_FB_OFFSET_LO_BASE_IDX                                                              2
pub const mmRCC_PEER2_FB_OFFSET_HI: u32 = 0x00d6;
#define mmRCC_PEER2_FB_OFFSET_HI_BASE_IDX                                                              2
pub const mmRCC_PEER2_FB_OFFSET_LO: u32 = 0x00d7;
#define mmRCC_PEER2_FB_OFFSET_LO_BASE_IDX                                                              2
pub const mmRCC_PEER3_FB_OFFSET_HI: u32 = 0x00d8;
#define mmRCC_PEER3_FB_OFFSET_HI_BASE_IDX                                                              2
pub const mmRCC_PEER3_FB_OFFSET_LO: u32 = 0x00d9;
#define mmRCC_PEER3_FB_OFFSET_LO_BASE_IDX                                                              2
pub const mmRCC_CMN_LINK_CNTL: u32 = 0x00de;
#define mmRCC_CMN_LINK_CNTL_BASE_IDX                                                                   2
pub const mmRCC_EP_REQUESTERID_RESTORE: u32 = 0x00df;
#define mmRCC_EP_REQUESTERID_RESTORE_BASE_IDX                                                          2
pub const mmRCC_LTR_LSWITCH_CNTL: u32 = 0x00e0;
#define mmRCC_LTR_LSWITCH_CNTL_BASE_IDX                                                                2
pub const mmRCC_MH_ARB_CNTL: u32 = 0x00e1;
#define mmRCC_MH_ARB_CNTL_BASE_IDX                                                                     2


// addressBlock: nbio_nbif0_bif_bx_pf_BIFDEC1
// base address: 0x0
pub const mmBIF_MM_INDACCESS_CNTL: u32 = 0x00e6;
#define mmBIF_MM_INDACCESS_CNTL_BASE_IDX                                                               2
pub const mmBUS_CNTL: u32 = 0x00e7;
#define mmBUS_CNTL_BASE_IDX                                                                            2
pub const mmBIF_SCRATCH0: u32 = 0x00e8;
#define mmBIF_SCRATCH0_BASE_IDX                                                                        2
pub const mmBIF_SCRATCH1: u32 = 0x00e9;
#define mmBIF_SCRATCH1_BASE_IDX                                                                        2
pub const mmBX_RESET_EN: u32 = 0x00ed;
#define mmBX_RESET_EN_BASE_IDX                                                                         2
pub const mmMM_CFGREGS_CNTL: u32 = 0x00ee;
#define mmMM_CFGREGS_CNTL_BASE_IDX                                                                     2
pub const mmBX_RESET_CNTL: u32 = 0x00f0;
#define mmBX_RESET_CNTL_BASE_IDX                                                                       2
pub const mmINTERRUPT_CNTL: u32 = 0x00f1;
#define mmINTERRUPT_CNTL_BASE_IDX                                                                      2
pub const mmINTERRUPT_CNTL2: u32 = 0x00f2;
#define mmINTERRUPT_CNTL2_BASE_IDX                                                                     2
pub const mmCLKREQB_PAD_CNTL: u32 = 0x00f8;
#define mmCLKREQB_PAD_CNTL_BASE_IDX                                                                    2
pub const mmBIF_FEATURES_CONTROL_MISC: u32 = 0x00fb;
#define mmBIF_FEATURES_CONTROL_MISC_BASE_IDX                                                           2
pub const mmBIF_DOORBELL_CNTL: u32 = 0x00fc;
#define mmBIF_DOORBELL_CNTL_BASE_IDX                                                                   2
pub const mmBIF_DOORBELL_INT_CNTL: u32 = 0x00fd;
#define mmBIF_DOORBELL_INT_CNTL_BASE_IDX                                                               2
pub const mmBIF_FB_EN: u32 = 0x00ff;
#define mmBIF_FB_EN_BASE_IDX                                                                           2
pub const mmBIF_BUSY_DELAY_CNTR: u32 = 0x0100;
#define mmBIF_BUSY_DELAY_CNTR_BASE_IDX                                                                 2
pub const mmBIF_MST_TRANS_PENDING_VF: u32 = 0x0109;
#define mmBIF_MST_TRANS_PENDING_VF_BASE_IDX                                                            2
pub const mmBIF_SLV_TRANS_PENDING_VF: u32 = 0x010a;
#define mmBIF_SLV_TRANS_PENDING_VF_BASE_IDX                                                            2
pub const mmBACO_CNTL: u32 = 0x010b;
#define mmBACO_CNTL_BASE_IDX                                                                           2
pub const mmBIF_BACO_EXIT_TIME0: u32 = 0x010c;
#define mmBIF_BACO_EXIT_TIME0_BASE_IDX                                                                 2
pub const mmBIF_BACO_EXIT_TIMER1: u32 = 0x010d;
#define mmBIF_BACO_EXIT_TIMER1_BASE_IDX                                                                2
pub const mmBIF_BACO_EXIT_TIMER2: u32 = 0x010e;
#define mmBIF_BACO_EXIT_TIMER2_BASE_IDX                                                                2
pub const mmBIF_BACO_EXIT_TIMER3: u32 = 0x010f;
#define mmBIF_BACO_EXIT_TIMER3_BASE_IDX                                                                2
pub const mmBIF_BACO_EXIT_TIMER4: u32 = 0x0110;
#define mmBIF_BACO_EXIT_TIMER4_BASE_IDX                                                                2
pub const mmMEM_TYPE_CNTL: u32 = 0x0111;
#define mmMEM_TYPE_CNTL_BASE_IDX                                                                       2
pub const mmSMU_BIF_VDDGFX_PWR_STATUS: u32 = 0x0113;
#define mmSMU_BIF_VDDGFX_PWR_STATUS_BASE_IDX                                                           2
pub const mmBIF_VDDGFX_GFX0_LOWER: u32 = 0x0114;
#define mmBIF_VDDGFX_GFX0_LOWER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_GFX0_UPPER: u32 = 0x0115;
#define mmBIF_VDDGFX_GFX0_UPPER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_GFX1_LOWER: u32 = 0x0116;
#define mmBIF_VDDGFX_GFX1_LOWER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_GFX1_UPPER: u32 = 0x0117;
#define mmBIF_VDDGFX_GFX1_UPPER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_GFX2_LOWER: u32 = 0x0118;
#define mmBIF_VDDGFX_GFX2_LOWER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_GFX2_UPPER: u32 = 0x0119;
#define mmBIF_VDDGFX_GFX2_UPPER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_GFX3_LOWER: u32 = 0x011a;
#define mmBIF_VDDGFX_GFX3_LOWER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_GFX3_UPPER: u32 = 0x011b;
#define mmBIF_VDDGFX_GFX3_UPPER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_GFX4_LOWER: u32 = 0x011c;
#define mmBIF_VDDGFX_GFX4_LOWER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_GFX4_UPPER: u32 = 0x011d;
#define mmBIF_VDDGFX_GFX4_UPPER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_GFX5_LOWER: u32 = 0x011e;
#define mmBIF_VDDGFX_GFX5_LOWER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_GFX5_UPPER: u32 = 0x011f;
#define mmBIF_VDDGFX_GFX5_UPPER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_RSV1_LOWER: u32 = 0x0120;
#define mmBIF_VDDGFX_RSV1_LOWER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_RSV1_UPPER: u32 = 0x0121;
#define mmBIF_VDDGFX_RSV1_UPPER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_RSV2_LOWER: u32 = 0x0122;
#define mmBIF_VDDGFX_RSV2_LOWER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_RSV2_UPPER: u32 = 0x0123;
#define mmBIF_VDDGFX_RSV2_UPPER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_RSV3_LOWER: u32 = 0x0124;
#define mmBIF_VDDGFX_RSV3_LOWER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_RSV3_UPPER: u32 = 0x0125;
#define mmBIF_VDDGFX_RSV3_UPPER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_RSV4_LOWER: u32 = 0x0126;
#define mmBIF_VDDGFX_RSV4_LOWER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_RSV4_UPPER: u32 = 0x0127;
#define mmBIF_VDDGFX_RSV4_UPPER_BASE_IDX                                                               2
pub const mmBIF_VDDGFX_FB_CMP: u32 = 0x0128;
#define mmBIF_VDDGFX_FB_CMP_BASE_IDX                                                                   2
pub const mmBIF_DOORBELL_GBLAPER1_LOWER: u32 = 0x0129;
#define mmBIF_DOORBELL_GBLAPER1_LOWER_BASE_IDX                                                         2
pub const mmBIF_DOORBELL_GBLAPER1_UPPER: u32 = 0x012a;
#define mmBIF_DOORBELL_GBLAPER1_UPPER_BASE_IDX                                                         2
pub const mmBIF_DOORBELL_GBLAPER2_LOWER: u32 = 0x012b;
#define mmBIF_DOORBELL_GBLAPER2_LOWER_BASE_IDX                                                         2
pub const mmBIF_DOORBELL_GBLAPER2_UPPER: u32 = 0x012c;
#define mmBIF_DOORBELL_GBLAPER2_UPPER_BASE_IDX                                                         2
pub const mmREMAP_HDP_MEM_FLUSH_CNTL: u32 = 0x012d;
#define mmREMAP_HDP_MEM_FLUSH_CNTL_BASE_IDX                                                            2
pub const mmREMAP_HDP_REG_FLUSH_CNTL: u32 = 0x012e;
#define mmREMAP_HDP_REG_FLUSH_CNTL_BASE_IDX                                                            2
pub const mmBIF_RB_CNTL: u32 = 0x012f;
#define mmBIF_RB_CNTL_BASE_IDX                                                                         2
pub const mmBIF_RB_BASE: u32 = 0x0130;
#define mmBIF_RB_BASE_BASE_IDX                                                                         2
pub const mmBIF_RB_RPTR: u32 = 0x0131;
#define mmBIF_RB_RPTR_BASE_IDX                                                                         2
pub const mmBIF_RB_WPTR: u32 = 0x0132;
#define mmBIF_RB_WPTR_BASE_IDX                                                                         2
pub const mmBIF_RB_WPTR_ADDR_HI: u32 = 0x0133;
#define mmBIF_RB_WPTR_ADDR_HI_BASE_IDX                                                                 2
pub const mmBIF_RB_WPTR_ADDR_LO: u32 = 0x0134;
#define mmBIF_RB_WPTR_ADDR_LO_BASE_IDX                                                                 2
pub const mmMAILBOX_INDEX: u32 = 0x0135;
#define mmMAILBOX_INDEX_BASE_IDX                                                                       2
pub const mmBIF_UVD_GPUIOV_CFG_SIZE: u32 = 0x0143;
#define mmBIF_UVD_GPUIOV_CFG_SIZE_BASE_IDX                                                             2
pub const mmBIF_VCE_GPUIOV_CFG_SIZE: u32 = 0x0144;
#define mmBIF_VCE_GPUIOV_CFG_SIZE_BASE_IDX                                                             2
pub const mmBIF_GFX_SDMA_GPUIOV_CFG_SIZE: u32 = 0x0145;
#define mmBIF_GFX_SDMA_GPUIOV_CFG_SIZE_BASE_IDX                                                        2
pub const mmBIF_PERSTB_PAD_CNTL: u32 = 0x0148;
#define mmBIF_PERSTB_PAD_CNTL_BASE_IDX                                                                 2
pub const mmBIF_PX_EN_PAD_CNTL: u32 = 0x0149;
#define mmBIF_PX_EN_PAD_CNTL_BASE_IDX                                                                  2
pub const mmBIF_REFPADKIN_PAD_CNTL: u32 = 0x014a;
#define mmBIF_REFPADKIN_PAD_CNTL_BASE_IDX                                                              2
pub const mmBIF_CLKREQB_PAD_CNTL: u32 = 0x014b;
#define mmBIF_CLKREQB_PAD_CNTL_BASE_IDX                                                                2


// addressBlock: nbio_nbif0_bif_bx_pf_BIFPFVFDEC1
// base address: 0x0
pub const mmBIF_BME_STATUS: u32 = 0x00eb;
#define mmBIF_BME_STATUS_BASE_IDX                                                                      2
pub const mmBIF_ATOMIC_ERR_LOG: u32 = 0x00ec;
#define mmBIF_ATOMIC_ERR_LOG_BASE_IDX                                                                  2
pub const mmDOORBELL_SELFRING_GPA_APER_BASE_HIGH: u32 = 0x00f3;
#define mmDOORBELL_SELFRING_GPA_APER_BASE_HIGH_BASE_IDX                                                2
pub const mmDOORBELL_SELFRING_GPA_APER_BASE_LOW: u32 = 0x00f4;
#define mmDOORBELL_SELFRING_GPA_APER_BASE_LOW_BASE_IDX                                                 2
pub const mmDOORBELL_SELFRING_GPA_APER_CNTL: u32 = 0x00f5;
#define mmDOORBELL_SELFRING_GPA_APER_CNTL_BASE_IDX                                                     2
pub const mmHDP_REG_COHERENCY_FLUSH_CNTL: u32 = 0x00f6;
#define mmHDP_REG_COHERENCY_FLUSH_CNTL_BASE_IDX                                                        2
pub const mmHDP_MEM_COHERENCY_FLUSH_CNTL: u32 = 0x00f7;
#define mmHDP_MEM_COHERENCY_FLUSH_CNTL_BASE_IDX                                                        2
pub const mmGPU_HDP_FLUSH_REQ: u32 = 0x0106;
#define mmGPU_HDP_FLUSH_REQ_BASE_IDX                                                                   2
pub const mmGPU_HDP_FLUSH_DONE: u32 = 0x0107;
#define mmGPU_HDP_FLUSH_DONE_BASE_IDX                                                                  2
pub const mmBIF_TRANS_PENDING: u32 = 0x0108;
#define mmBIF_TRANS_PENDING_BASE_IDX                                                                   2
pub const mmMAILBOX_MSGBUF_TRN_DW0: u32 = 0x0136;
#define mmMAILBOX_MSGBUF_TRN_DW0_BASE_IDX                                                              2
pub const mmMAILBOX_MSGBUF_TRN_DW1: u32 = 0x0137;
#define mmMAILBOX_MSGBUF_TRN_DW1_BASE_IDX                                                              2
pub const mmMAILBOX_MSGBUF_TRN_DW2: u32 = 0x0138;
#define mmMAILBOX_MSGBUF_TRN_DW2_BASE_IDX                                                              2
pub const mmMAILBOX_MSGBUF_TRN_DW3: u32 = 0x0139;
#define mmMAILBOX_MSGBUF_TRN_DW3_BASE_IDX                                                              2
pub const mmMAILBOX_MSGBUF_RCV_DW0: u32 = 0x013a;
#define mmMAILBOX_MSGBUF_RCV_DW0_BASE_IDX                                                              2
pub const mmMAILBOX_MSGBUF_RCV_DW1: u32 = 0x013b;
#define mmMAILBOX_MSGBUF_RCV_DW1_BASE_IDX                                                              2
pub const mmMAILBOX_MSGBUF_RCV_DW2: u32 = 0x013c;
#define mmMAILBOX_MSGBUF_RCV_DW2_BASE_IDX                                                              2
pub const mmMAILBOX_MSGBUF_RCV_DW3: u32 = 0x013d;
#define mmMAILBOX_MSGBUF_RCV_DW3_BASE_IDX                                                              2
pub const mmMAILBOX_CONTROL: u32 = 0x013e;
#define mmMAILBOX_CONTROL_BASE_IDX                                                                     2
pub const mmMAILBOX_INT_CNTL: u32 = 0x013f;
#define mmMAILBOX_INT_CNTL_BASE_IDX                                                                    2
pub const mmBIF_VMHV_MAILBOX: u32 = 0x0140;
#define mmBIF_VMHV_MAILBOX_BASE_IDX                                                                    2


// addressBlock: nbio_nbif0_gdc_GDCDEC
// base address: 0x0
pub const mmNGDC_SDP_PORT_CTRL: u32 = 0x01c2;
#define mmNGDC_SDP_PORT_CTRL_BASE_IDX                                                                  2
pub const mmSHUB_REGS_IF_CTL: u32 = 0x01c3;
#define mmSHUB_REGS_IF_CTL_BASE_IDX                                                                    2
pub const mmNGDC_RESERVED_0: u32 = 0x01cb;
#define mmNGDC_RESERVED_0_BASE_IDX                                                                     2
pub const mmNGDC_RESERVED_1: u32 = 0x01cc;
#define mmNGDC_RESERVED_1_BASE_IDX                                                                     2
pub const mmNGDC_SDP_PORT_CTRL_SOCCLK: u32 = 0x01cd;
#define mmNGDC_SDP_PORT_CTRL_SOCCLK_BASE_IDX                                                           2
pub const mmBIF_SDMA0_DOORBELL_RANGE: u32 = 0x01d0;
#define mmBIF_SDMA0_DOORBELL_RANGE_BASE_IDX                                                            2
pub const mmBIF_SDMA1_DOORBELL_RANGE: u32 = 0x01d1;
#define mmBIF_SDMA1_DOORBELL_RANGE_BASE_IDX                                                            2
pub const mmBIF_IH_DOORBELL_RANGE: u32 = 0x01d2;
#define mmBIF_IH_DOORBELL_RANGE_BASE_IDX                                                               2
pub const mmBIF_MMSCH0_DOORBELL_RANGE: u32 = 0x01d3;
#define mmBIF_MMSCH0_DOORBELL_RANGE_BASE_IDX                                                           2
pub const mmATDMA_MISC_CNTL: u32 = 0x01dd;
#define mmATDMA_MISC_CNTL_BASE_IDX                                                                     2
pub const mmBIF_DOORBELL_FENCE_CNTL: u32 = 0x01de;
#define mmBIF_DOORBELL_FENCE_CNTL_BASE_IDX                                                             2
pub const mmS2A_MISC_CNTL: u32 = 0x01df;
#define mmS2A_MISC_CNTL_BASE_IDX                                                                       2
pub const mmGDC_PG_MISC_CNTL: u32 = 0x01f0;
#define mmGDC_PG_MISC_CNTL_BASE_IDX                                                                    2


// addressBlock: nbio_nbif0_rcc_dev0_BIFDEC2
// base address: 0x0
pub const mmGFXMSIX_VECT0_ADDR_LO: u32 = 0x0400;
#define mmGFXMSIX_VECT0_ADDR_LO_BASE_IDX                                                               3
pub const mmGFXMSIX_VECT0_ADDR_HI: u32 = 0x0401;
#define mmGFXMSIX_VECT0_ADDR_HI_BASE_IDX                                                               3
pub const mmGFXMSIX_VECT0_MSG_DATA: u32 = 0x0402;
#define mmGFXMSIX_VECT0_MSG_DATA_BASE_IDX                                                              3
pub const mmGFXMSIX_VECT0_CONTROL: u32 = 0x0403;
#define mmGFXMSIX_VECT0_CONTROL_BASE_IDX                                                               3
pub const mmGFXMSIX_VECT1_ADDR_LO: u32 = 0x0404;
#define mmGFXMSIX_VECT1_ADDR_LO_BASE_IDX                                                               3
pub const mmGFXMSIX_VECT1_ADDR_HI: u32 = 0x0405;
#define mmGFXMSIX_VECT1_ADDR_HI_BASE_IDX                                                               3
pub const mmGFXMSIX_VECT1_MSG_DATA: u32 = 0x0406;
#define mmGFXMSIX_VECT1_MSG_DATA_BASE_IDX                                                              3
pub const mmGFXMSIX_VECT1_CONTROL: u32 = 0x0407;
#define mmGFXMSIX_VECT1_CONTROL_BASE_IDX                                                               3
pub const mmGFXMSIX_VECT2_ADDR_LO: u32 = 0x0408;
#define mmGFXMSIX_VECT2_ADDR_LO_BASE_IDX                                                               3
pub const mmGFXMSIX_VECT2_ADDR_HI: u32 = 0x0409;
#define mmGFXMSIX_VECT2_ADDR_HI_BASE_IDX                                                               3
pub const mmGFXMSIX_VECT2_MSG_DATA: u32 = 0x040a;
#define mmGFXMSIX_VECT2_MSG_DATA_BASE_IDX                                                              3
pub const mmGFXMSIX_VECT2_CONTROL: u32 = 0x040b;
#define mmGFXMSIX_VECT2_CONTROL_BASE_IDX                                                               3
pub const mmGFXMSIX_PBA: u32 = 0x0800;
#define mmGFXMSIX_PBA_BASE_IDX                                                                         3


// addressBlock: syshub_mmreg_ind_syshubind
// base address: 0x0
pub const ixSYSHUB_MMREG_IND_SYSHUB_DS_CTRL_SOCCLK: u32 = 0x10000;
pub const ixSYSHUB_MMREG_IND_SYSHUB_DS_CTRL2_SOCCLK: u32 = 0x10004;
pub const ixSYSHUB_MMREG_IND_SYSHUB_BGEN_ENHANCEMENT_BYPASS_EN_SOCCLK: u32 = 0x10008;
pub const ixSYSHUB_MMREG_IND_SYSHUB_BGEN_ENHANCEMENT_IMM_EN_SOCCLK: u32 = 0x1000c;
pub const ixSYSHUB_MMREG_IND_DMA_CLK0_SW0_SYSHUB_QOS_CNTL: u32 = 0x10010;
pub const ixSYSHUB_MMREG_IND_DMA_CLK0_SW1_SYSHUB_QOS_CNTL: u32 = 0x10014;
pub const ixSYSHUB_MMREG_IND_DMA_CLK0_SW2_SYSHUB_QOS_CNTL: u32 = 0x10018;
pub const ixSYSHUB_MMREG_IND_DMA_CLK0_SW0_CL0_CNTL: u32 = 0x1001c;
pub const ixSYSHUB_MMREG_IND_DMA_CLK0_SW0_CL1_CNTL: u32 = 0x10020;
pub const ixSYSHUB_MMREG_IND_DMA_CLK0_SW0_CL2_CNTL: u32 = 0x10024;
pub const ixSYSHUB_MMREG_IND_DMA_CLK0_SW0_CL3_CNTL: u32 = 0x10028;
pub const ixSYSHUB_MMREG_IND_DMA_CLK0_SW0_CL4_CNTL: u32 = 0x1002c;
pub const ixSYSHUB_MMREG_IND_DMA_CLK0_SW0_CL5_CNTL: u32 = 0x10030;
pub const ixSYSHUB_MMREG_IND_DMA_CLK0_SW1_CL0_CNTL: u32 = 0x10034;
pub const ixSYSHUB_MMREG_IND_DMA_CLK0_SW2_CL0_CNTL: u32 = 0x10038;
pub const ixSYSHUB_MMREG_IND_HST_CLK0_SW0_CL0_CNTL: u32 = 0x10100;
pub const ixSYSHUB_MMREG_IND_HST_CLK0_SW0_CL1_CNTL: u32 = 0x10104;
pub const ixSYSHUB_MMREG_IND_HST_CLK0_SW0_CL2_CNTL: u32 = 0x10108;
pub const ixSYSHUB_MMREG_IND_HST_CLK0_SW1_CL0_CNTL: u32 = 0x1010c;
pub const ixSYSHUB_MMREG_IND_HST_CLK0_SW1_CL1_CNTL: u32 = 0x10110;
pub const ixSYSHUB_MMREG_IND_HST_CLK0_SW1_CL2_CNTL: u32 = 0x10114;
pub const ixSYSHUB_MMREG_IND_HST_CLK0_SW1_CL3_CNTL: u32 = 0x10118;
pub const ixSYSHUB_MMREG_IND_HST_CLK0_SW1_CL4_CNTL: u32 = 0x1011c;
pub const ixSYSHUB_MMREG_IND_SYSHUB_CG_CNTL: u32 = 0x10300;
pub const ixSYSHUB_MMREG_IND_SYSHUB_TRANS_IDLE: u32 = 0x10308;
pub const ixSYSHUB_MMREG_IND_SYSHUB_HP_TIMER: u32 = 0x1030c;
pub const ixSYSHUB_MMREG_IND_SYSHUB_MGCG_CTRL_SOCCLK: u32 = 0x10310;
pub const ixSYSHUB_MMREG_IND_SYSUB_CPF_DOORBELL_RS_RESET: u32 = 0x10314;
pub const ixSYSHUB_MMREG_IND_SYSHUB_SCRATCH: u32 = 0x10f00;
pub const ixSYSHUB_MMREG_IND_SYSHUB_CL_MASK: u32 = 0x10f04;
pub const ixSYSHUB_MMREG_IND_SYSHUB_DS_CTRL_SHUBCLK: u32 = 0x11000;
pub const ixSYSHUB_MMREG_IND_SYSHUB_DS_CTRL2_SHUBCLK: u32 = 0x11004;
pub const ixSYSHUB_MMREG_IND_SYSHUB_BGEN_ENHANCEMENT_BYPASS_EN_SHUBCLK: u32 = 0x11008;
pub const ixSYSHUB_MMREG_IND_SYSHUB_BGEN_ENHANCEMENT_IMM_EN_SHUBCLK: u32 = 0x1100c;
pub const ixSYSHUB_MMREG_IND_DMA_CLK1_SW0_SYSHUB_QOS_CNTL: u32 = 0x11010;
pub const ixSYSHUB_MMREG_IND_DMA_CLK1_SW1_SYSHUB_QOS_CNTL: u32 = 0x11014;
pub const ixSYSHUB_MMREG_IND_DMA_CLK1_SW0_CL0_CNTL: u32 = 0x11018;
pub const ixSYSHUB_MMREG_IND_DMA_CLK1_SW0_CL1_CNTL: u32 = 0x1101c;
pub const ixSYSHUB_MMREG_IND_DMA_CLK1_SW0_CL2_CNTL: u32 = 0x11020;
pub const ixSYSHUB_MMREG_IND_DMA_CLK1_SW0_CL3_CNTL: u32 = 0x11024;
pub const ixSYSHUB_MMREG_IND_DMA_CLK1_SW0_CL4_CNTL: u32 = 0x11028;
pub const ixSYSHUB_MMREG_IND_DMA_CLK1_SW1_CL0_CNTL: u32 = 0x1102c;
pub const ixSYSHUB_MMREG_IND_DMA_CLK1_SW1_CL1_CNTL: u32 = 0x11030;
pub const ixSYSHUB_MMREG_IND_DMA_CLK1_SW1_CL2_CNTL: u32 = 0x11034;
pub const ixSYSHUB_MMREG_IND_DMA_CLK1_SW1_CL3_CNTL: u32 = 0x11038;
pub const ixSYSHUB_MMREG_IND_DMA_CLK1_SW1_CL4_CNTL: u32 = 0x1103c;
pub const ixSYSHUB_MMREG_IND_SYSHUB_MGCG_CTRL_SHUBCLK: u32 = 0x11040;
pub const ixSYSHUB_MMREG_IND_NIC400_0_ASIB_0_FN_MOD: u32 = 0x20108;
pub const ixSYSHUB_MMREG_IND_NIC400_0_AMIB_0_FN_MOD_BM_ISS: u32 = 0x30008;
pub const ixSYSHUB_MMREG_IND_NIC400_0_AMIB_1_FN_MOD_BM_ISS: u32 = 0x31008;
pub const ixSYSHUB_MMREG_IND_NIC400_1_ASIB_0_FN_MOD: u32 = 0x40108;
pub const ixSYSHUB_MMREG_IND_NIC400_1_AMIB_0_FN_MOD: u32 = 0x50008;
pub const ixSYSHUB_MMREG_IND_NIC400_1_AMIB_1_FN_MOD: u32 = 0x51008;
pub const ixSYSHUB_MMREG_IND_NIC400_1_AMIB_2_FN_MOD: u32 = 0x52008;
pub const ixSYSHUB_MMREG_IND_NIC400_2_ASIB_0_FN_MOD: u32 = 0x60108;
pub const ixSYSHUB_MMREG_IND_NIC400_2_ASIB_1_FN_MOD: u32 = 0x61108;
pub const ixSYSHUB_MMREG_IND_NIC400_2_ASIB_2_FN_MOD: u32 = 0x62108;
pub const ixSYSHUB_MMREG_IND_NIC400_2_ASIB_3_FN_MOD: u32 = 0x63108;
pub const ixSYSHUB_MMREG_IND_NIC400_2_ASIB_4_FN_MOD: u32 = 0x64108;
pub const ixSYSHUB_MMREG_IND_NIC400_2_AMIB_0_FN_MOD_BM_ISS: u32 = 0x70008;
pub const ixSYSHUB_MMREG_IND_NIC400_5_ASIB_0_FN_MOD: u32 = 0xc0108;
pub const ixSYSHUB_MMREG_IND_NIC400_5_ASIB_1_FN_MOD: u32 = 0xc1108;
pub const ixSYSHUB_MMREG_IND_NIC400_5_ASIB_2_FN_MOD: u32 = 0xc2108;
pub const ixSYSHUB_MMREG_IND_NIC400_5_ASIB_3_FN_MOD: u32 = 0xc3108;
pub const ixSYSHUB_MMREG_IND_NIC400_5_ASIB_4_FN_MOD: u32 = 0xc4108;
pub const ixSYSHUB_MMREG_IND_NIC400_5_AMIB_0_FN_MOD: u32 = 0xd0008;
pub const ixSYSHUB_MMREG_IND_NIC400_4_ASIB_0_FN_MOD: u32 = 0xe0108;
pub const ixSYSHUB_MMREG_IND_NIC400_4_ASIB_1_FN_MOD: u32 = 0xe1108;
pub const ixSYSHUB_MMREG_IND_NIC400_4_AMIB_0_FN_MOD: u32 = 0xf0008;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
