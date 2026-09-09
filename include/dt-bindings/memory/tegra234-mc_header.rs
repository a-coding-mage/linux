/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/* Copyright (c) 2018-2022, NVIDIA CORPORATION. All rights reserved. */


/* special clients */
pub const TEGRA234_SID_INVALID: u32 = 0x00;
pub const TEGRA234_SID_PASSTHROUGH: u32 = 0x7f;

/* ISO stream IDs */
pub const TEGRA234_SID_ISO_NVDISPLAY: u32 = 0x01;
pub const TEGRA234_SID_ISO_VI: u32 = 0x02;
pub const TEGRA234_SID_ISO_VIFALC: u32 = 0x03;
pub const TEGRA234_SID_ISO_VI2: u32 = 0x04;
pub const TEGRA234_SID_ISO_VI2FALC: u32 = 0x05;
pub const TEGRA234_SID_ISO_VI_VM2: u32 = 0x06;
pub const TEGRA234_SID_ISO_VI2_VM2: u32 = 0x07;

/* NISO0 stream IDs */
pub const TEGRA234_SID_AON: u32 = 0x01;
pub const TEGRA234_SID_APE: u32 = 0x02;
pub const TEGRA234_SID_HDA: u32 = 0x03;
pub const TEGRA234_SID_GPCDMA: u32 = 0x04;
pub const TEGRA234_SID_ETR: u32 = 0x05;
pub const TEGRA234_SID_MGBE: u32 = 0x06;
pub const TEGRA234_SID_NVDISPLAY: u32 = 0x07;
pub const TEGRA234_SID_DCE: u32 = 0x08;
pub const TEGRA234_SID_PSC: u32 = 0x09;
pub const TEGRA234_SID_RCE: u32 = 0x0a;
pub const TEGRA234_SID_SCE: u32 = 0x0b;
pub const TEGRA234_SID_UFSHC: u32 = 0x0c;
pub const TEGRA234_SID_APE_1: u32 = 0x0d;
pub const TEGRA234_SID_GPCDMA_1: u32 = 0x0e;
pub const TEGRA234_SID_GPCDMA_2: u32 = 0x0f;
pub const TEGRA234_SID_GPCDMA_3: u32 = 0x10;
pub const TEGRA234_SID_GPCDMA_4: u32 = 0x11;
pub const TEGRA234_SID_PCIE0: u32 = 0x12;
pub const TEGRA234_SID_PCIE4: u32 = 0x13;
pub const TEGRA234_SID_PCIE5: u32 = 0x14;
pub const TEGRA234_SID_PCIE6: u32 = 0x15;
pub const TEGRA234_SID_RCE_VM2: u32 = 0x16;
pub const TEGRA234_SID_RCE_SERVER: u32 = 0x17;
pub const TEGRA234_SID_SMMU_TEST: u32 = 0x18;
pub const TEGRA234_SID_UFS_1: u32 = 0x19;
pub const TEGRA234_SID_UFS_2: u32 = 0x1a;
pub const TEGRA234_SID_UFS_3: u32 = 0x1b;
pub const TEGRA234_SID_UFS_4: u32 = 0x1c;
pub const TEGRA234_SID_UFS_5: u32 = 0x1d;
pub const TEGRA234_SID_UFS_6: u32 = 0x1e;
pub const TEGRA234_SID_PCIE9: u32 = 0x1f;
pub const TEGRA234_SID_VSE_GPCDMA_VM0: u32 = 0x20;
pub const TEGRA234_SID_VSE_GPCDMA_VM1: u32 = 0x21;
pub const TEGRA234_SID_VSE_GPCDMA_VM2: u32 = 0x22;
pub const TEGRA234_SID_NVDLA1: u32 = 0x23;
pub const TEGRA234_SID_NVENC: u32 = 0x24;
pub const TEGRA234_SID_NVJPG1: u32 = 0x25;
pub const TEGRA234_SID_OFA: u32 = 0x26;
pub const TEGRA234_SID_MGBE_VF1: u32 = 0x49;
pub const TEGRA234_SID_MGBE_VF2: u32 = 0x4a;
pub const TEGRA234_SID_MGBE_VF3: u32 = 0x4b;
pub const TEGRA234_SID_MGBE_VF4: u32 = 0x4c;
pub const TEGRA234_SID_MGBE_VF5: u32 = 0x4d;
pub const TEGRA234_SID_MGBE_VF6: u32 = 0x4e;
pub const TEGRA234_SID_MGBE_VF7: u32 = 0x4f;
pub const TEGRA234_SID_MGBE_VF8: u32 = 0x50;
pub const TEGRA234_SID_MGBE_VF9: u32 = 0x51;
pub const TEGRA234_SID_MGBE_VF10: u32 = 0x52;
pub const TEGRA234_SID_MGBE_VF11: u32 = 0x53;
pub const TEGRA234_SID_MGBE_VF12: u32 = 0x54;
pub const TEGRA234_SID_MGBE_VF13: u32 = 0x55;
pub const TEGRA234_SID_MGBE_VF14: u32 = 0x56;
pub const TEGRA234_SID_MGBE_VF15: u32 = 0x57;
pub const TEGRA234_SID_MGBE_VF16: u32 = 0x58;
pub const TEGRA234_SID_MGBE_VF17: u32 = 0x59;
pub const TEGRA234_SID_MGBE_VF18: u32 = 0x5a;
pub const TEGRA234_SID_MGBE_VF19: u32 = 0x5b;
pub const TEGRA234_SID_MGBE_VF20: u32 = 0x5c;
pub const TEGRA234_SID_APE_2: u32 = 0x5e;
pub const TEGRA234_SID_APE_3: u32 = 0x5f;
pub const TEGRA234_SID_UFS_7: u32 = 0x60;
pub const TEGRA234_SID_UFS_8: u32 = 0x61;
pub const TEGRA234_SID_UFS_9: u32 = 0x62;
pub const TEGRA234_SID_UFS_10: u32 = 0x63;
pub const TEGRA234_SID_UFS_11: u32 = 0x64;
pub const TEGRA234_SID_UFS_12: u32 = 0x65;
pub const TEGRA234_SID_UFS_13: u32 = 0x66;
pub const TEGRA234_SID_UFS_14: u32 = 0x67;
pub const TEGRA234_SID_UFS_15: u32 = 0x68;
pub const TEGRA234_SID_UFS_16: u32 = 0x69;
pub const TEGRA234_SID_UFS_17: u32 = 0x6a;
pub const TEGRA234_SID_UFS_18: u32 = 0x6b;
pub const TEGRA234_SID_UFS_19: u32 = 0x6c;
pub const TEGRA234_SID_UFS_20: u32 = 0x6d;
pub const TEGRA234_SID_GPCDMA_5: u32 = 0x6e;
pub const TEGRA234_SID_GPCDMA_6: u32 = 0x6f;
pub const TEGRA234_SID_GPCDMA_7: u32 = 0x70;
pub const TEGRA234_SID_GPCDMA_8: u32 = 0x71;
pub const TEGRA234_SID_GPCDMA_9: u32 = 0x72;

/* NISO1 stream IDs */
pub const TEGRA234_SID_SDMMC1A: u32 = 0x01;
pub const TEGRA234_SID_SDMMC4: u32 = 0x02;
pub const TEGRA234_SID_EQOS: u32 = 0x03;
pub const TEGRA234_SID_HWMP_PMA: u32 = 0x04;
pub const TEGRA234_SID_PCIE1: u32 = 0x05;
pub const TEGRA234_SID_PCIE2: u32 = 0x06;
pub const TEGRA234_SID_PCIE3: u32 = 0x07;
pub const TEGRA234_SID_PCIE7: u32 = 0x08;
pub const TEGRA234_SID_PCIE8: u32 = 0x09;
pub const TEGRA234_SID_PCIE10: u32 = 0x0b;
pub const TEGRA234_SID_QSPI0: u32 = 0x0c;
pub const TEGRA234_SID_QSPI1: u32 = 0x0d;
pub const TEGRA234_SID_XUSB_HOST: u32 = 0x0e;
pub const TEGRA234_SID_XUSB_DEV: u32 = 0x0f;
pub const TEGRA234_SID_BPMP: u32 = 0x10;
pub const TEGRA234_SID_FSI: u32 = 0x11;
pub const TEGRA234_SID_PVA0_VM0: u32 = 0x12;
pub const TEGRA234_SID_PVA0_VM1: u32 = 0x13;
pub const TEGRA234_SID_PVA0_VM2: u32 = 0x14;
pub const TEGRA234_SID_PVA0_VM3: u32 = 0x15;
pub const TEGRA234_SID_PVA0_VM4: u32 = 0x16;
pub const TEGRA234_SID_PVA0_VM5: u32 = 0x17;
pub const TEGRA234_SID_PVA0_VM6: u32 = 0x18;
pub const TEGRA234_SID_PVA0_VM7: u32 = 0x19;
pub const TEGRA234_SID_XUSB_VF0: u32 = 0x1a;
pub const TEGRA234_SID_XUSB_VF1: u32 = 0x1b;
pub const TEGRA234_SID_XUSB_VF2: u32 = 0x1c;
pub const TEGRA234_SID_XUSB_VF3: u32 = 0x1d;
pub const TEGRA234_SID_EQOS_VF1: u32 = 0x1e;
pub const TEGRA234_SID_EQOS_VF2: u32 = 0x1f;
pub const TEGRA234_SID_EQOS_VF3: u32 = 0x20;
pub const TEGRA234_SID_EQOS_VF4: u32 = 0x21;
pub const TEGRA234_SID_ISP_VM2: u32 = 0x22;
pub const TEGRA234_SID_HOST1X: u32 = 0x27;
pub const TEGRA234_SID_ISP: u32 = 0x28;
pub const TEGRA234_SID_NVDEC: u32 = 0x29;
pub const TEGRA234_SID_NVJPG: u32 = 0x2a;
pub const TEGRA234_SID_NVDLA0: u32 = 0x2b;
pub const TEGRA234_SID_PVA0: u32 = 0x2c;
pub const TEGRA234_SID_SES_SE0: u32 = 0x2d;
pub const TEGRA234_SID_SES_SE1: u32 = 0x2e;
pub const TEGRA234_SID_SES_SE2: u32 = 0x2f;
pub const TEGRA234_SID_SEU1_SE0: u32 = 0x30;
pub const TEGRA234_SID_SEU1_SE1: u32 = 0x31;
pub const TEGRA234_SID_SEU1_SE2: u32 = 0x32;
pub const TEGRA234_SID_TSEC: u32 = 0x33;
pub const TEGRA234_SID_VIC: u32 = 0x34;
pub const TEGRA234_SID_HC_VM0: u32 = 0x3d;
pub const TEGRA234_SID_HC_VM1: u32 = 0x3e;
pub const TEGRA234_SID_HC_VM2: u32 = 0x3f;
pub const TEGRA234_SID_HC_VM3: u32 = 0x40;
pub const TEGRA234_SID_HC_VM4: u32 = 0x41;
pub const TEGRA234_SID_HC_VM5: u32 = 0x42;
pub const TEGRA234_SID_HC_VM6: u32 = 0x43;
pub const TEGRA234_SID_HC_VM7: u32 = 0x44;
pub const TEGRA234_SID_SE_VM0: u32 = 0x45;
pub const TEGRA234_SID_SE_VM1: u32 = 0x46;
pub const TEGRA234_SID_SE_VM2: u32 = 0x47;
pub const TEGRA234_SID_ISPFALC: u32 = 0x48;
pub const TEGRA234_SID_NISO1_SMMU_TEST: u32 = 0x49;
pub const TEGRA234_SID_TSEC_VM0: u32 = 0x4a;

/* Shared stream IDs */
pub const TEGRA234_SID_HOST1X_CTX0: u32 = 0x35;
pub const TEGRA234_SID_HOST1X_CTX1: u32 = 0x36;
pub const TEGRA234_SID_HOST1X_CTX2: u32 = 0x37;
pub const TEGRA234_SID_HOST1X_CTX3: u32 = 0x38;
pub const TEGRA234_SID_HOST1X_CTX4: u32 = 0x39;
pub const TEGRA234_SID_HOST1X_CTX5: u32 = 0x3a;
pub const TEGRA234_SID_HOST1X_CTX6: u32 = 0x3b;
pub const TEGRA234_SID_HOST1X_CTX7: u32 = 0x3c;

/*
 * memory client IDs
 */

/* Misses from System Memory Management Unit (SMMU) Page Table Cache (PTC) */
pub const TEGRA234_MEMORY_CLIENT_PTCR: u32 = 0x00;
/* MSS internal memqual MIU7 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU7R: u32 = 0x01;
/* MSS internal memqual MIU7 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU7W: u32 = 0x02;
/* MSS internal memqual MIU8 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU8R: u32 = 0x03;
/* MSS internal memqual MIU8 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU8W: u32 = 0x04;
/* MSS internal memqual MIU9 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU9R: u32 = 0x05;
/* MSS internal memqual MIU9 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU9W: u32 = 0x06;
/* MSS internal memqual MIU10 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU10R: u32 = 0x07;
/* MSS internal memqual MIU10 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU10W: u32 = 0x08;
/* MSS internal memqual MIU11 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU11R: u32 = 0x09;
/* MSS internal memqual MIU11 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU11W: u32 = 0x0a;
/* MSS internal memqual MIU12 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU12R: u32 = 0x0b;
/* MSS internal memqual MIU12 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU12W: u32 = 0x0c;
/* MSS internal memqual MIU13 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU13R: u32 = 0x0d;
/* MSS internal memqual MIU13 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU13W: u32 = 0x0e;
pub const TEGRA234_MEMORY_CLIENT_NVL5RHP: u32 = 0x13;
pub const TEGRA234_MEMORY_CLIENT_NVL5R: u32 = 0x14;
/* High-definition audio (HDA) read clients */
pub const TEGRA234_MEMORY_CLIENT_HDAR: u32 = 0x15;
/* Host channel data read clients */
pub const TEGRA234_MEMORY_CLIENT_HOST1XDMAR: u32 = 0x16;
pub const TEGRA234_MEMORY_CLIENT_NVL5W: u32 = 0x17;
pub const TEGRA234_MEMORY_CLIENT_NVL6RHP: u32 = 0x18;
pub const TEGRA234_MEMORY_CLIENT_NVL6R: u32 = 0x19;
pub const TEGRA234_MEMORY_CLIENT_NVL6W: u32 = 0x1a;
pub const TEGRA234_MEMORY_CLIENT_NVL7RHP: u32 = 0x1b;
pub const TEGRA234_MEMORY_CLIENT_NVENCSRD: u32 = 0x1c;
pub const TEGRA234_MEMORY_CLIENT_NVL7R: u32 = 0x1d;
pub const TEGRA234_MEMORY_CLIENT_NVL7W: u32 = 0x1e;
pub const TEGRA234_MEMORY_CLIENT_NVL8RHP: u32 = 0x20;
pub const TEGRA234_MEMORY_CLIENT_NVL8R: u32 = 0x21;
pub const TEGRA234_MEMORY_CLIENT_NVL8W: u32 = 0x22;
pub const TEGRA234_MEMORY_CLIENT_NVL9RHP: u32 = 0x23;
pub const TEGRA234_MEMORY_CLIENT_NVL9R: u32 = 0x24;
pub const TEGRA234_MEMORY_CLIENT_NVL9W: u32 = 0x25;
/* PCIE6 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE6AR: u32 = 0x28;
/* PCIE6 write clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE6AW: u32 = 0x29;
/* PCIE7 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE7AR: u32 = 0x2a;
pub const TEGRA234_MEMORY_CLIENT_NVENCSWR: u32 = 0x2b;
/* DLA0ARDB read clients */
pub const TEGRA234_MEMORY_CLIENT_DLA0RDB: u32 = 0x2c;
/* DLA0ARDB1 read clients */
pub const TEGRA234_MEMORY_CLIENT_DLA0RDB1: u32 = 0x2d;
/* DLA0 writes */
pub const TEGRA234_MEMORY_CLIENT_DLA0WRB: u32 = 0x2e;
/* DLA1ARDB read clients */
pub const TEGRA234_MEMORY_CLIENT_DLA1RDB: u32 = 0x2f;
/* PCIE7 write clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE7AW: u32 = 0x30;
/* PCIE8 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE8AR: u32 = 0x32;
/* High-definition audio (HDA) write clients */
pub const TEGRA234_MEMORY_CLIENT_HDAW: u32 = 0x35;
/* Writes from Cortex-A9 4 CPU cores via the L2 cache */
pub const TEGRA234_MEMORY_CLIENT_MPCOREW: u32 = 0x39;
/* OFAA client */
pub const TEGRA234_MEMORY_CLIENT_OFAR1: u32 = 0x3a;
/* PCIE8 write clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE8AW: u32 = 0x3b;
/* PCIE9 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE9AR: u32 = 0x3c;
/* PCIE6r1 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE6AR1: u32 = 0x3d;
/* PCIE9 write clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE9AW: u32 = 0x3e;
/* PCIE10 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE10AR: u32 = 0x3f;
/* PCIE10 write clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE10AW: u32 = 0x40;
/* ISP read client for Crossbar A */
pub const TEGRA234_MEMORY_CLIENT_ISPRA: u32 = 0x44;
/* ISP read client 1 for Crossbar A */
pub const TEGRA234_MEMORY_CLIENT_ISPFALR: u32 = 0x45;
/* ISP Write client for Crossbar A */
pub const TEGRA234_MEMORY_CLIENT_ISPWA: u32 = 0x46;
/* ISP Write client Crossbar B */
pub const TEGRA234_MEMORY_CLIENT_ISPWB: u32 = 0x47;
/* PCIE10r1 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE10AR1: u32 = 0x48;
/* PCIE7r1 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE7AR1: u32 = 0x49;
/* XUSB_HOST read clients */
pub const TEGRA234_MEMORY_CLIENT_XUSB_HOSTR: u32 = 0x4a;
/* XUSB_HOST write clients */
pub const TEGRA234_MEMORY_CLIENT_XUSB_HOSTW: u32 = 0x4b;
/* XUSB read clients */
pub const TEGRA234_MEMORY_CLIENT_XUSB_DEVR: u32 = 0x4c;
/* XUSB_DEV write clients */
pub const TEGRA234_MEMORY_CLIENT_XUSB_DEVW: u32 = 0x4d;
/* TSEC Memory Return Data Client Description */
pub const TEGRA234_MEMORY_CLIENT_TSECSRD: u32 = 0x54;
/* TSEC Memory Write Client Description */
pub const TEGRA234_MEMORY_CLIENT_TSECSWR: u32 = 0x55;
/* XSPI writes */
pub const TEGRA234_MEMORY_CLIENT_XSPI1W: u32 = 0x56;
/* MGBE0 read client */
pub const TEGRA234_MEMORY_CLIENT_MGBEARD: u32 = 0x58;
/* MGBEB read client */
pub const TEGRA234_MEMORY_CLIENT_MGBEBRD: u32 = 0x59;
/* MGBEC read client */
pub const TEGRA234_MEMORY_CLIENT_MGBECRD: u32 = 0x5a;
/* MGBED read client */
pub const TEGRA234_MEMORY_CLIENT_MGBEDRD: u32 = 0x5b;
/* MGBE0 write client */
pub const TEGRA234_MEMORY_CLIENT_MGBEAWR: u32 = 0x5c;
/* OFAA client */
pub const TEGRA234_MEMORY_CLIENT_OFAR: u32 = 0x5d;
/* OFAA writes */
pub const TEGRA234_MEMORY_CLIENT_OFAW: u32 = 0x5e;
/* MGBEB write client */
pub const TEGRA234_MEMORY_CLIENT_MGBEBWR: u32 = 0x5f;
/* sdmmca memory read client */
pub const TEGRA234_MEMORY_CLIENT_SDMMCRA: u32 = 0x60;
/* MGBEC write client */
pub const TEGRA234_MEMORY_CLIENT_MGBECWR: u32 = 0x61;
/* sdmmcd memory read client */
pub const TEGRA234_MEMORY_CLIENT_SDMMCRAB: u32 = 0x63;
/* sdmmca memory write client */
pub const TEGRA234_MEMORY_CLIENT_SDMMCWA: u32 = 0x64;
/* MGBED write client */
pub const TEGRA234_MEMORY_CLIENT_MGBEDWR: u32 = 0x65;
/* sdmmcd memory write client */
pub const TEGRA234_MEMORY_CLIENT_SDMMCWAB: u32 = 0x67;
/* SE Memory Return Data Client Description */
pub const TEGRA234_MEMORY_CLIENT_SEU1RD: u32 = 0x68;
/* SE Memory Write Client Description */
pub const TEGRA234_MEMORY_CLIENT_SUE1WR: u32 = 0x69;
pub const TEGRA234_MEMORY_CLIENT_VICSRD: u32 = 0x6c;
pub const TEGRA234_MEMORY_CLIENT_VICSWR: u32 = 0x6d;
/* DLA1ARDB1 read clients */
pub const TEGRA234_MEMORY_CLIENT_DLA1RDB1: u32 = 0x6e;
/* DLA1 writes */
pub const TEGRA234_MEMORY_CLIENT_DLA1WRB: u32 = 0x6f;
/* VI FLACON read clients */
pub const TEGRA234_MEMORY_CLIENT_VI2FALR: u32 = 0x71;
/* VI Write client */
pub const TEGRA234_MEMORY_CLIENT_VI2W: u32 = 0x70;
/* VI Write client */
pub const TEGRA234_MEMORY_CLIENT_VIW: u32 = 0x72;
/* NISO display read client */
pub const TEGRA234_MEMORY_CLIENT_NVDISPNISOR: u32 = 0x73;
/* NVDISPNISO writes */
pub const TEGRA234_MEMORY_CLIENT_NVDISPNISOW: u32 = 0x74;
/* XSPI client */
pub const TEGRA234_MEMORY_CLIENT_XSPI0R: u32 = 0x75;
/* XSPI writes */
pub const TEGRA234_MEMORY_CLIENT_XSPI0W: u32 = 0x76;
/* XSPI client */
pub const TEGRA234_MEMORY_CLIENT_XSPI1R: u32 = 0x77;
pub const TEGRA234_MEMORY_CLIENT_NVDECSRD: u32 = 0x78;
pub const TEGRA234_MEMORY_CLIENT_NVDECSWR: u32 = 0x79;
/* Audio Processing (APE) engine read clients */
pub const TEGRA234_MEMORY_CLIENT_APER: u32 = 0x7a;
/* Audio Processing (APE) engine write clients */
pub const TEGRA234_MEMORY_CLIENT_APEW: u32 = 0x7b;
/* VI2FAL writes */
pub const TEGRA234_MEMORY_CLIENT_VI2FALW: u32 = 0x7c;
pub const TEGRA234_MEMORY_CLIENT_NVJPGSRD: u32 = 0x7e;
pub const TEGRA234_MEMORY_CLIENT_NVJPGSWR: u32 = 0x7f;
/* SE Memory Return Data Client Description */
pub const TEGRA234_MEMORY_CLIENT_SESRD: u32 = 0x80;
/* SE Memory Write Client Description */
pub const TEGRA234_MEMORY_CLIENT_SESWR: u32 = 0x81;
/* AXI AP and DFD-AUX0/1 read clients Both share the same interface on the on MSS */
pub const TEGRA234_MEMORY_CLIENT_AXIAPR: u32 = 0x82;
/* AXI AP and DFD-AUX0/1 write clients Both sahre the same interface on MSS */
pub const TEGRA234_MEMORY_CLIENT_AXIAPW: u32 = 0x83;
/* ETR read clients */
pub const TEGRA234_MEMORY_CLIENT_ETRR: u32 = 0x84;
/* ETR write clients */
pub const TEGRA234_MEMORY_CLIENT_ETRW: u32 = 0x85;
/* AXI Switch read client */
pub const TEGRA234_MEMORY_CLIENT_AXISR: u32 = 0x8c;
/* AXI Switch write client */
pub const TEGRA234_MEMORY_CLIENT_AXISW: u32 = 0x8d;
/* EQOS read client */
pub const TEGRA234_MEMORY_CLIENT_EQOSR: u32 = 0x8e;
/* EQOS write client */
pub const TEGRA234_MEMORY_CLIENT_EQOSW: u32 = 0x8f;
/* UFSHC read client */
pub const TEGRA234_MEMORY_CLIENT_UFSHCR: u32 = 0x90;
/* UFSHC write client */
pub const TEGRA234_MEMORY_CLIENT_UFSHCW: u32 = 0x91;
/* NVDISPLAY read client */
pub const TEGRA234_MEMORY_CLIENT_NVDISPLAYR: u32 = 0x92;
/* BPMP read client */
pub const TEGRA234_MEMORY_CLIENT_BPMPR: u32 = 0x93;
/* BPMP write client */
pub const TEGRA234_MEMORY_CLIENT_BPMPW: u32 = 0x94;
/* BPMPDMA read client */
pub const TEGRA234_MEMORY_CLIENT_BPMPDMAR: u32 = 0x95;
/* BPMPDMA write client */
pub const TEGRA234_MEMORY_CLIENT_BPMPDMAW: u32 = 0x96;
/* AON read client */
pub const TEGRA234_MEMORY_CLIENT_AONR: u32 = 0x97;
/* AON write client */
pub const TEGRA234_MEMORY_CLIENT_AONW: u32 = 0x98;
/* AONDMA read client */
pub const TEGRA234_MEMORY_CLIENT_AONDMAR: u32 = 0x99;
/* AONDMA write client */
pub const TEGRA234_MEMORY_CLIENT_AONDMAW: u32 = 0x9a;
/* SCE read client */
pub const TEGRA234_MEMORY_CLIENT_SCER: u32 = 0x9b;
/* SCE write client */
pub const TEGRA234_MEMORY_CLIENT_SCEW: u32 = 0x9c;
/* SCEDMA read client */
pub const TEGRA234_MEMORY_CLIENT_SCEDMAR: u32 = 0x9d;
/* SCEDMA write client */
pub const TEGRA234_MEMORY_CLIENT_SCEDMAW: u32 = 0x9e;
/* APEDMA read client */
pub const TEGRA234_MEMORY_CLIENT_APEDMAR: u32 = 0x9f;
/* APEDMA write client */
pub const TEGRA234_MEMORY_CLIENT_APEDMAW: u32 = 0xa0;
/* NVDISPLAY read client instance 2 */
pub const TEGRA234_MEMORY_CLIENT_NVDISPLAYR1: u32 = 0xa1;
pub const TEGRA234_MEMORY_CLIENT_VICSRD1: u32 = 0xa2;
/* MSS internal memqual MIU0 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU0R: u32 = 0xa6;
/* MSS internal memqual MIU0 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU0W: u32 = 0xa7;
/* MSS internal memqual MIU1 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU1R: u32 = 0xa8;
/* MSS internal memqual MIU1 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU1W: u32 = 0xa9;
/* MSS internal memqual MIU2 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU2R: u32 = 0xae;
/* MSS internal memqual MIU2 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU2W: u32 = 0xaf;
/* MSS internal memqual MIU3 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU3R: u32 = 0xb0;
/* MSS internal memqual MIU3 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU3W: u32 = 0xb1;
/* MSS internal memqual MIU4 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU4R: u32 = 0xb2;
/* MSS internal memqual MIU4 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU4W: u32 = 0xb3;
pub const TEGRA234_MEMORY_CLIENT_DPMUR: u32 = 0xb4;
pub const TEGRA234_MEMORY_CLIENT_DPMUW: u32 = 0xb5;
pub const TEGRA234_MEMORY_CLIENT_NVL0R: u32 = 0xb6;
pub const TEGRA234_MEMORY_CLIENT_NVL0W: u32 = 0xb7;
pub const TEGRA234_MEMORY_CLIENT_NVL1R: u32 = 0xb8;
pub const TEGRA234_MEMORY_CLIENT_NVL1W: u32 = 0xb9;
pub const TEGRA234_MEMORY_CLIENT_NVL2R: u32 = 0xba;
pub const TEGRA234_MEMORY_CLIENT_NVL2W: u32 = 0xbb;
/* VI FLACON read clients */
pub const TEGRA234_MEMORY_CLIENT_VIFALR: u32 = 0xbc;
/* VIFAL write clients */
pub const TEGRA234_MEMORY_CLIENT_VIFALW: u32 = 0xbd;
/* DLA0ARDA read clients */
pub const TEGRA234_MEMORY_CLIENT_DLA0RDA: u32 = 0xbe;
/* DLA0 Falcon read clients */
pub const TEGRA234_MEMORY_CLIENT_DLA0FALRDB: u32 = 0xbf;
/* DLA0 write clients */
pub const TEGRA234_MEMORY_CLIENT_DLA0WRA: u32 = 0xc0;
/* DLA0 write clients */
pub const TEGRA234_MEMORY_CLIENT_DLA0FALWRB: u32 = 0xc1;
/* DLA1ARDA read clients */
pub const TEGRA234_MEMORY_CLIENT_DLA1RDA: u32 = 0xc2;
/* DLA1 Falcon read clients */
pub const TEGRA234_MEMORY_CLIENT_DLA1FALRDB: u32 = 0xc3;
/* DLA1 write clients */
pub const TEGRA234_MEMORY_CLIENT_DLA1WRA: u32 = 0xc4;
/* DLA1 write clients */
pub const TEGRA234_MEMORY_CLIENT_DLA1FALWRB: u32 = 0xc5;
/* PVA0RDA read clients */
pub const TEGRA234_MEMORY_CLIENT_PVA0RDA: u32 = 0xc6;
/* PVA0RDB read clients */
pub const TEGRA234_MEMORY_CLIENT_PVA0RDB: u32 = 0xc7;
/* PVA0RDC read clients */
pub const TEGRA234_MEMORY_CLIENT_PVA0RDC: u32 = 0xc8;
/* PVA0WRA write clients */
pub const TEGRA234_MEMORY_CLIENT_PVA0WRA: u32 = 0xc9;
/* PVA0WRB write clients */
pub const TEGRA234_MEMORY_CLIENT_PVA0WRB: u32 = 0xca;
/* PVA0WRC write clients */
pub const TEGRA234_MEMORY_CLIENT_PVA0WRC: u32 = 0xcb;
/* RCE read client */
pub const TEGRA234_MEMORY_CLIENT_RCER: u32 = 0xd2;
/* RCE write client */
pub const TEGRA234_MEMORY_CLIENT_RCEW: u32 = 0xd3;
/* RCEDMA read client */
pub const TEGRA234_MEMORY_CLIENT_RCEDMAR: u32 = 0xd4;
/* RCEDMA write client */
pub const TEGRA234_MEMORY_CLIENT_RCEDMAW: u32 = 0xd5;
/* PCIE0 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE0R: u32 = 0xd8;
/* PCIE0 write clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE0W: u32 = 0xd9;
/* PCIE1 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE1R: u32 = 0xda;
/* PCIE1 write clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE1W: u32 = 0xdb;
/* PCIE2 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE2AR: u32 = 0xdc;
/* PCIE2 write clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE2AW: u32 = 0xdd;
/* PCIE3 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE3R: u32 = 0xde;
/* PCIE3 write clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE3W: u32 = 0xdf;
/* PCIE4 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE4R: u32 = 0xe0;
/* PCIE4 write clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE4W: u32 = 0xe1;
/* PCIE5 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE5R: u32 = 0xe2;
/* PCIE5 write clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE5W: u32 = 0xe3;
/* ISP read client 1 for Crossbar A */
pub const TEGRA234_MEMORY_CLIENT_ISPFALW: u32 = 0xe4;
pub const TEGRA234_MEMORY_CLIENT_NVL3R: u32 = 0xe5;
pub const TEGRA234_MEMORY_CLIENT_NVL3W: u32 = 0xe6;
pub const TEGRA234_MEMORY_CLIENT_NVL4R: u32 = 0xe7;
pub const TEGRA234_MEMORY_CLIENT_NVL4W: u32 = 0xe8;
/* DLA0ARDA1 read clients */
pub const TEGRA234_MEMORY_CLIENT_DLA0RDA1: u32 = 0xe9;
/* DLA1ARDA1 read clients */
pub const TEGRA234_MEMORY_CLIENT_DLA1RDA1: u32 = 0xea;
/* PVA0RDA1 read clients */
pub const TEGRA234_MEMORY_CLIENT_PVA0RDA1: u32 = 0xeb;
/* PVA0RDB1 read clients */
pub const TEGRA234_MEMORY_CLIENT_PVA0RDB1: u32 = 0xec;
/* PCIE5r1 read clients */
pub const TEGRA234_MEMORY_CLIENT_PCIE5R1: u32 = 0xef;
pub const TEGRA234_MEMORY_CLIENT_NVENCSRD1: u32 = 0xf0;
/* ISP read client for Crossbar A */
pub const TEGRA234_MEMORY_CLIENT_ISPRA1: u32 = 0xf2;
pub const TEGRA234_MEMORY_CLIENT_NVL0RHP: u32 = 0xf4;
pub const TEGRA234_MEMORY_CLIENT_NVL1RHP: u32 = 0xf5;
pub const TEGRA234_MEMORY_CLIENT_NVL2RHP: u32 = 0xf6;
pub const TEGRA234_MEMORY_CLIENT_NVL3RHP: u32 = 0xf7;
pub const TEGRA234_MEMORY_CLIENT_NVL4RHP: u32 = 0xf8;
/* MSS internal memqual MIU5 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU5R: u32 = 0xfc;
/* MSS internal memqual MIU5 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU5W: u32 = 0xfd;
/* MSS internal memqual MIU6 read clients */
pub const TEGRA234_MEMORY_CLIENT_MIU6R: u32 = 0xfe;
/* MSS internal memqual MIU6 write clients */
pub const TEGRA234_MEMORY_CLIENT_MIU6W: u32 = 0xff;
pub const TEGRA234_MEMORY_CLIENT_NVJPG1SRD: u32 = 0x123;
pub const TEGRA234_MEMORY_CLIENT_NVJPG1SWR: u32 = 0x124;

/* ICC ID's for dummy MC clients used to represent CPU Clusters */
pub const TEGRA_ICC_MC_CPU_CLUSTER0: u32 = 1003;
pub const TEGRA_ICC_MC_CPU_CLUSTER1: u32 = 1004;
pub const TEGRA_ICC_MC_CPU_CLUSTER2: u32 = 1005;



// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
