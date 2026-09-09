/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/* Copyright (c) 2025, NVIDIA CORPORATION. All rights reserved. */

#define DT_BINDINGS_MEMORY_NVIDIA_TEGRA264_H

#define TEGRA264_SID(x) ((x) << 8)

/*
 * SMMU stream IDs
 */

pub const TEGRA264_SID_AON: u32 = TEGRA264_SID(0x01);
pub const TEGRA264_SID_APE: u32 = TEGRA264_SID(0x02);
pub const TEGRA264_SID_ETR: u32 = TEGRA264_SID(0x03);
pub const TEGRA264_SID_BPMP: u32 = TEGRA264_SID(0x04);
pub const TEGRA264_SID_DCE: u32 = TEGRA264_SID(0x05);
pub const TEGRA264_SID_EQOS: u32 = TEGRA264_SID(0x06);
pub const TEGRA264_SID_GPCDMA: u32 = TEGRA264_SID(0x08);
pub const TEGRA264_SID_DISP: u32 = TEGRA264_SID(0x09);
pub const TEGRA264_SID_HDA: u32 = TEGRA264_SID(0x0a);
pub const TEGRA264_SID_HOST1X: u32 = TEGRA264_SID(0x0b);
pub const TEGRA264_SID_ISP0: u32 = TEGRA264_SID(0x0c);
pub const TEGRA264_SID_ISP1: u32 = TEGRA264_SID(0x0d);
pub const TEGRA264_SID_PMA0: u32 = TEGRA264_SID(0x0e);
pub const TEGRA264_SID_FSI0: u32 = TEGRA264_SID(0x0f);
pub const TEGRA264_SID_FSI1: u32 = TEGRA264_SID(0x10);
pub const TEGRA264_SID_PVA: u32 = TEGRA264_SID(0x11);
pub const TEGRA264_SID_SDMMC0: u32 = TEGRA264_SID(0x12);
pub const TEGRA264_SID_MGBE0: u32 = TEGRA264_SID(0x13);
pub const TEGRA264_SID_MGBE1: u32 = TEGRA264_SID(0x14);
pub const TEGRA264_SID_MGBE2: u32 = TEGRA264_SID(0x15);
pub const TEGRA264_SID_MGBE3: u32 = TEGRA264_SID(0x16);
pub const TEGRA264_SID_MSSSEQ: u32 = TEGRA264_SID(0x17);
pub const TEGRA264_SID_SE: u32 = TEGRA264_SID(0x18);
pub const TEGRA264_SID_SEU1: u32 = TEGRA264_SID(0x19);
pub const TEGRA264_SID_SEU2: u32 = TEGRA264_SID(0x1a);
pub const TEGRA264_SID_SEU3: u32 = TEGRA264_SID(0x1b);
pub const TEGRA264_SID_PSC: u32 = TEGRA264_SID(0x1c);
pub const TEGRA264_SID_OESP: u32 = TEGRA264_SID(0x23);
pub const TEGRA264_SID_SB: u32 = TEGRA264_SID(0x24);
pub const TEGRA264_SID_XSPI0: u32 = TEGRA264_SID(0x25);
pub const TEGRA264_SID_TSEC: u32 = TEGRA264_SID(0x29);
pub const TEGRA264_SID_UFS: u32 = TEGRA264_SID(0x2a);
pub const TEGRA264_SID_RCE: u32 = TEGRA264_SID(0x2b);
pub const TEGRA264_SID_RCE1: u32 = TEGRA264_SID(0x2c);
pub const TEGRA264_SID_VI: u32 = TEGRA264_SID(0x2e);
pub const TEGRA264_SID_VI1: u32 = TEGRA264_SID(0x2f);
pub const TEGRA264_SID_VIC: u32 = TEGRA264_SID(0x30);
pub const TEGRA264_SID_XUSB_DEV: u32 = TEGRA264_SID(0x32);
pub const TEGRA264_SID_XUSB_DEV1: u32 = TEGRA264_SID(0x33);
pub const TEGRA264_SID_XUSB_DEV2: u32 = TEGRA264_SID(0x34);
pub const TEGRA264_SID_XUSB_DEV3: u32 = TEGRA264_SID(0x35);
pub const TEGRA264_SID_XUSB_DEV4: u32 = TEGRA264_SID(0x36);
pub const TEGRA264_SID_XUSB_DEV5: u32 = TEGRA264_SID(0x37);

/*
 * memory client IDs
 */

/* PTW read client mapped to SOC SMMU0 */
pub const TEGRA264_MEMORY_CLIENT_PTCR: u32 = 0x00;
/* HOST1X read client */
pub const TEGRA264_MEMORY_CLIENT_HOST1XR: u32 = 0x16;
pub const TEGRA264_MEMORY_CLIENT_MPCORER: u32 = 0x27;
/* Platform security (PSC) Read clients */
pub const TEGRA264_MEMORY_CLIENT_PSCR: u32 = 0x33;
/* PSC Write clients */
pub const TEGRA264_MEMORY_CLIENT_PSCW: u32 = 0x34;
/* ISP0 Read client */
pub const TEGRA264_MEMORY_CLIENT_ISP0R: u32 = 0x37;
pub const TEGRA264_MEMORY_CLIENT_MPCOREW: u32 = 0x39;
/* ISP0 Write client */
pub const TEGRA264_MEMORY_CLIENT_ISP0W: u32 = 0x44;
/* ISP1 Write client */
pub const TEGRA264_MEMORY_CLIENT_ISP1W: u32 = 0x45;
/* ISP FALCON Read client */
pub const TEGRA264_MEMORY_CLIENT_ISPFALCONR: u32 = 0x47;
/* ISP FALCON Write client */
pub const TEGRA264_MEMORY_CLIENT_ISPFALCONW: u32 = 0x4f;
/* MGBE2 Read mccif */
pub const TEGRA264_MEMORY_CLIENT_MGBE2R: u32 = 0x5c;
pub const TEGRA264_MEMORY_CLIENT_OFAR2MC: u32 = 0x5d;
pub const TEGRA264_MEMORY_CLIENT_OFAW2MC: u32 = 0x5e;
/* MGBE2 Write mccif */
pub const TEGRA264_MEMORY_CLIENT_MGBE2W: u32 = 0x5f;
/* MGBE3 Read mccif */
pub const TEGRA264_MEMORY_CLIENT_MGBE3R: u32 = 0x61;
/* MGBE3 Write mccif */
pub const TEGRA264_MEMORY_CLIENT_MGBE3W: u32 = 0x65;
/* SEU1 Memory Read Client */
pub const TEGRA264_MEMORY_CLIENT_SEU1RD: u32 = 0x68;
/* SEU1 Memory Write Client */
pub const TEGRA264_MEMORY_CLIENT_SEU1WR: u32 = 0x69;
/* VIC read client */
pub const TEGRA264_MEMORY_CLIENT_VICR: u32 = 0x6c;
/* VIC Write client */
pub const TEGRA264_MEMORY_CLIENT_VICW: u32 = 0x6d;
/* VI R5 Write client */
pub const TEGRA264_MEMORY_CLIENT_VIW: u32 = 0x72;
/* QSPI Read Client */
pub const TEGRA264_MEMORY_CLIENT_XSPI0R: u32 = 0x75;
/* QSPI Write Client */
pub const TEGRA264_MEMORY_CLIENT_XSPI0W: u32 = 0x76;
pub const TEGRA264_MEMORY_CLIENT_NVDECSRD2MC: u32 = 0x78;
pub const TEGRA264_MEMORY_CLIENT_NVDECSWR2MC: u32 = 0x79;
/* Audio processor(APE) Read client */
pub const TEGRA264_MEMORY_CLIENT_APER: u32 = 0x7a;
/* Audio processor(APE) Write client */
pub const TEGRA264_MEMORY_CLIENT_APEW: u32 = 0x7b;
/* SEU0 read client */
pub const TEGRA264_MEMORY_CLIENT_SER: u32 = 0x80;
/* SEU0 write client */
pub const TEGRA264_MEMORY_CLIENT_SEW: u32 = 0x81;
/* AXI AP and DFD/Coresight1-AUX0/1 Read clients both share the same interface on MSS */
pub const TEGRA264_MEMORY_CLIENT_AXIAPR: u32 = 0x82;
/* AXI AP and DFD/Coresight1-AUX0/1 Write clients both share the same interface on MSS */
pub const TEGRA264_MEMORY_CLIENT_AXIAPW: u32 = 0x83;
/* ETR or DFD/Coresight0 Read Client */
pub const TEGRA264_MEMORY_CLIENT_ETRR: u32 = 0x84;
/* ETR or DFD/Coresight0 Write Client */
pub const TEGRA264_MEMORY_CLIENT_ETRW: u32 = 0x85;
/* Security(tsec) Read client */
pub const TEGRA264_MEMORY_CLIENT_TSECR: u32 = 0x86;
/* Security(tsec) Write client */
pub const TEGRA264_MEMORY_CLIENT_TSECW: u32 = 0x87;
/* BPMP read client */
pub const TEGRA264_MEMORY_CLIENT_BPMPR: u32 = 0x93;
/* BPMP write client */
pub const TEGRA264_MEMORY_CLIENT_BPMPW: u32 = 0x94;
/* AON Read Client */
pub const TEGRA264_MEMORY_CLIENT_AONR: u32 = 0x97;
/* AON write client */
pub const TEGRA264_MEMORY_CLIENT_AONW: u32 = 0x98;
/* GPCDMA debug Read client */
pub const TEGRA264_MEMORY_CLIENT_GPCDMAR: u32 = 0x99;
/* GPCDMA debug Write client */
pub const TEGRA264_MEMORY_CLIENT_GPCDMAW: u32 = 0x9a;
/* Audio DMA Read client */
pub const TEGRA264_MEMORY_CLIENT_APEDMAR: u32 = 0x9f;
/* Audio DMA Write client */
pub const TEGRA264_MEMORY_CLIENT_APEDMAW: u32 = 0xa0;
/* mss internal memqual MIU0 reads */
pub const TEGRA264_MEMORY_CLIENT_MIU0R: u32 = 0xa6;
/* mss internal memqual MIU0 writes */
pub const TEGRA264_MEMORY_CLIENT_MIU0W: u32 = 0xa7;
/* mss internal memqual MIU1 reads */
pub const TEGRA264_MEMORY_CLIENT_MIU1R: u32 = 0xa8;
/* mss internal memqual MIU1 writes */
pub const TEGRA264_MEMORY_CLIENT_MIU1W: u32 = 0xa9;
/* mss internal memqual MIU2 reads */
pub const TEGRA264_MEMORY_CLIENT_MIU2R: u32 = 0xae;
/* mss internal memqual MIU2 writes */
pub const TEGRA264_MEMORY_CLIENT_MIU2W: u32 = 0xaf;
/* mss internal memqual MIU3 reads */
pub const TEGRA264_MEMORY_CLIENT_MIU3R: u32 = 0xb0;
/* mss internal memqual MIU3 writes */
pub const TEGRA264_MEMORY_CLIENT_MIU3W: u32 = 0xb1;
/* mss internal memqual MIU4 reads */
pub const TEGRA264_MEMORY_CLIENT_MIU4R: u32 = 0xb2;
/* mss internal memqual MIU4 writes */
pub const TEGRA264_MEMORY_CLIENT_MIU4W: u32 = 0xb3;
pub const TEGRA264_MEMORY_CLIENT_GPUR02MC: u32 = 0xb6;
pub const TEGRA264_MEMORY_CLIENT_GPUW02MC: u32 = 0xb7;
/* VI Falcon Read client */
pub const TEGRA264_MEMORY_CLIENT_VIFALCONR: u32 = 0xbc;
/* VI Falcon Write client */
pub const TEGRA264_MEMORY_CLIENT_VIFALCONW: u32 = 0xbd;
/* Read Client of RCE */
pub const TEGRA264_MEMORY_CLIENT_RCER: u32 = 0xd2;
/* Write client of RCE */
pub const TEGRA264_MEMORY_CLIENT_RCEW: u32 = 0xd3;
pub const TEGRA264_MEMORY_CLIENT_NVENC1SRD2MC: u32 = 0xd6;
pub const TEGRA264_MEMORY_CLIENT_NVENC1SWR2MC: u32 = 0xd7;
/* PCIE0/MSI Write clients */
pub const TEGRA264_MEMORY_CLIENT_PCIE0W: u32 = 0xd9;
/* PCIE1/RPX4 Read clients */
pub const TEGRA264_MEMORY_CLIENT_PCIE1R: u32 = 0xda;
/* PCIE1/RPX4 Write clients */
pub const TEGRA264_MEMORY_CLIENT_PCIE1W: u32 = 0xdb;
/* PCIE2/DMX4 Read clients */
pub const TEGRA264_MEMORY_CLIENT_PCIE2AR: u32 = 0xdc;
/* PCIE2/DMX4 Write clients */
pub const TEGRA264_MEMORY_CLIENT_PCIE2AW: u32 = 0xdd;
/* PCIE3/RPX4 Read clients */
pub const TEGRA264_MEMORY_CLIENT_PCIE3R: u32 = 0xde;
/* PCIE3/RPX4 Write clients */
pub const TEGRA264_MEMORY_CLIENT_PCIE3W: u32 = 0xdf;
/* PCIE4/DMX8 Read clients */
pub const TEGRA264_MEMORY_CLIENT_PCIE4R: u32 = 0xe0;
/* PCIE4/DMX8 Write clients */
pub const TEGRA264_MEMORY_CLIENT_PCIE4W: u32 = 0xe1;
/* PCIE5/DMX4 Read clients */
pub const TEGRA264_MEMORY_CLIENT_PCIE5R: u32 = 0xe2;
/* PCIE5/DMX4 Write clients */
pub const TEGRA264_MEMORY_CLIENT_PCIE5W: u32 = 0xe3;
/* mss internal memqual MIU5 reads */
pub const TEGRA264_MEMORY_CLIENT_MIU5R: u32 = 0xfc;
/* mss internal memqual MIU5 writes */
pub const TEGRA264_MEMORY_CLIENT_MIU5W: u32 = 0xfd;
/* mss internal memqual MIU6 writes */
pub const TEGRA264_MEMORY_CLIENT_MIU6W: u32 = 0xff;
pub const TEGRA264_MEMORY_CLIENT_RISTR: u32 = 0x100;
pub const TEGRA264_MEMORY_CLIENT_RISTW: u32 = 0x101;
/* OESP (Pluton) Read client */
pub const TEGRA264_MEMORY_CLIENT_OESPR: u32 = 0x102;
/* OESP (Pluton) Write client */
pub const TEGRA264_MEMORY_CLIENT_OESPW: u32 = 0x103;
/* mss internal memqual MIU7 writes */
pub const TEGRA264_MEMORY_CLIENT_MIU7W: u32 = 0x105;
/* mss internal memqual MIU8 reads */
pub const TEGRA264_MEMORY_CLIENT_MIU8R: u32 = 0x106;
/* mss internal memqual MIU8 writes */
pub const TEGRA264_MEMORY_CLIENT_MIU8W: u32 = 0x107;
/* mss internal memqual MIU9 reads */
pub const TEGRA264_MEMORY_CLIENT_MIU9R: u32 = 0x108;
/* mss internal memqual MIU9 writes */
pub const TEGRA264_MEMORY_CLIENT_MIU9W: u32 = 0x109;
/* HWPM Write Interface */
pub const TEGRA264_MEMORY_CLIENT_PMA0AWR: u32 = 0x122;
pub const TEGRA264_MEMORY_CLIENT_NVJPG1SRD2MC: u32 = 0x123;
pub const TEGRA264_MEMORY_CLIENT_NVJPG1SWR2MC: u32 = 0x124;
/* CTW read client mapped to SMMU0 */
pub const TEGRA264_MEMORY_CLIENT_SMMU0CTWR: u32 = 0x12e;
/* CMDQV read client mapped to SMMU0 */
pub const TEGRA264_MEMORY_CLIENT_SMMU0CMDQVR: u32 = 0x12f;
/* CMDQV write client mapped to SMMU0 */
pub const TEGRA264_MEMORY_CLIENT_SMMU0CMDQVW: u32 = 0x130;
/* EVNTQ write client mapped to SMMU0 */
pub const TEGRA264_MEMORY_CLIENT_SMMU0EVNTQW: u32 = 0x131;
/* PTW read client mapped to SMMU1 */
pub const TEGRA264_MEMORY_CLIENT_SMMU1PTWR: u32 = 0x132;
/* CTW read client mapped to SMMU1 */
pub const TEGRA264_MEMORY_CLIENT_SMMU1CTWR: u32 = 0x134;
/* CMDQV read client mapped to SMMU1 */
pub const TEGRA264_MEMORY_CLIENT_SMMU1CMDQVR: u32 = 0x135;
/* CMDQV write client mapped to SMMU1 */
pub const TEGRA264_MEMORY_CLIENT_SMMU1CMDQVW: u32 = 0x136;
/* EVNTQ write client mapped to SMMU1 */
pub const TEGRA264_MEMORY_CLIENT_SMMU1EVNTQW: u32 = 0x137;
/* PTW read client mapped to SMMU2 */
pub const TEGRA264_MEMORY_CLIENT_SMMU2PTWR: u32 = 0x138;
/* CTW read client mapped to SMMU2 */
pub const TEGRA264_MEMORY_CLIENT_SMMU2CTWR: u32 = 0x13a;
/* CMDQV read client mapped to SMMU2 */
pub const TEGRA264_MEMORY_CLIENT_SMMU2CMDQVR: u32 = 0x13b;
/* CMDQV write client mapped to SMMU2 */
pub const TEGRA264_MEMORY_CLIENT_SMMU2CMDQVW: u32 = 0x13c;
/* EVNTQ write client mapped to SMMU2 */
pub const TEGRA264_MEMORY_CLIENT_SMMU2EVNTQW: u32 = 0x13d;
/* CMDQ read client mapped to SMMU0 */
pub const TEGRA264_MEMORY_CLIENT_SMMU0CMDQR: u32 = 0x144;
/* CMDQ read client mapped to SMMU1 */
pub const TEGRA264_MEMORY_CLIENT_SMMU1CMDQR: u32 = 0x145;
/* CMDQ read client mapped to SMMU2 */
pub const TEGRA264_MEMORY_CLIENT_SMMU2CMDQR: u32 = 0x146;
/* Audio processor1(APE1) Read client */
pub const TEGRA264_MEMORY_CLIENT_APE1R: u32 = 0x150;
/* Audio processor1(APE1) Write client */
pub const TEGRA264_MEMORY_CLIENT_APE1W: u32 = 0x151;
/* UFS Read client */
pub const TEGRA264_MEMORY_CLIENT_UFSR: u32 = 0x15c;
/* UFS write client */
pub const TEGRA264_MEMORY_CLIENT_UFSW: u32 = 0x15d;
/* XUSB HOST Read Client */
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEVR: u32 = 0x166;
/* XUSB HOST Write Client */
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEVW: u32 = 0x167;
/* XUSB SS0 Read Client */
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV1R: u32 = 0x168;
/* XUSB SS1 Write Client */
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV2W: u32 = 0x169;
/* XUSB SS2 Read Client */
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV3R: u32 = 0x16a;
/* XUSB SS2 Write Client */
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV3W: u32 = 0x16b;
/* XUSB SS3 Read Client */
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV4R: u32 = 0x16c;
/* XUSB SS3 Write Client */
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV4W: u32 = 0x16d;
/* XUSB DEV Read Client */
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV5R: u32 = 0x16e;
/* XUSB DEV Write Client */
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV5W: u32 = 0x16f;
/* DCE Read client */
pub const TEGRA264_MEMORY_CLIENT_DCER: u32 = 0x17a;
/* DCE Write client */
pub const TEGRA264_MEMORY_CLIENT_DCEW: u32 = 0x17b;
/* HDA Read client */
pub const TEGRA264_MEMORY_CLIENT_HDAR: u32 = 0x17c;
/* HDA Write client */
pub const TEGRA264_MEMORY_CLIENT_HDAW: u32 = 0x17d;
/* DISPNISO read client */
pub const TEGRA264_MEMORY_CLIENT_DISPNISOR: u32 = 0x17e;
/* DISPNISO write client */
pub const TEGRA264_MEMORY_CLIENT_DISPNISOW: u32 = 0x17f;
/* XUSB SS0 Write Client */
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV1W: u32 = 0x180;
/* XUSB SS1 Read Client */
pub const TEGRA264_MEMORY_CLIENT_XUSB_DEV2R: u32 = 0x181;
/* Disp ISO Read Client */
pub const TEGRA264_MEMORY_CLIENT_DISPR: u32 = 0x182;
/* MSSSEQ Read Client */
pub const TEGRA264_MEMORY_CLIENT_MSSSEQR: u32 = 0x185;
/* MSSSEQ Write Client */
pub const TEGRA264_MEMORY_CLIENT_MSSSEQW: u32 = 0x186;
/* PTW read client mapped to SMMU3 */
pub const TEGRA264_MEMORY_CLIENT_SMMU3PTWR: u32 = 0x18b;
/* CTW read client mapped to SMMU3 */
pub const TEGRA264_MEMORY_CLIENT_SMMU3CTWR: u32 = 0x18d;
/* CMDQV read client mapped to SMMU3 */
pub const TEGRA264_MEMORY_CLIENT_SMMU3CMDQVR: u32 = 0x18e;
/* CMDQV write client mapped to SMMU3 */
pub const TEGRA264_MEMORY_CLIENT_SMMU3CMDQVW: u32 = 0x18f;
/* EVNTQ write client mapped to SMMU3 */
pub const TEGRA264_MEMORY_CLIENT_SMMU3EVNTQW: u32 = 0x190;
/* CMDQ read client mapped to SMMU3 */
pub const TEGRA264_MEMORY_CLIENT_SMMU3CMDQR: u32 = 0x191;
/* PTW read client mapped to SMMU4 */
pub const TEGRA264_MEMORY_CLIENT_SMMU4PTWR: u32 = 0x192;
/* CTW read client mapped to SMMU4 */
pub const TEGRA264_MEMORY_CLIENT_SMMU4CTWR: u32 = 0x194;
/* CMDQV read client mapped to SMMU4 */
pub const TEGRA264_MEMORY_CLIENT_SMMU4CMDQVR: u32 = 0x195;
/* CMDQV write client mapped to SMMU4 */
pub const TEGRA264_MEMORY_CLIENT_SMMU4CMDQVW: u32 = 0x196;
/* EVNTQ write client mapped to SMMU4 */
pub const TEGRA264_MEMORY_CLIENT_SMMU4EVNTQW: u32 = 0x197;
/* CMDQ read client mapped to SMMU4 */
pub const TEGRA264_MEMORY_CLIENT_SMMU4CMDQR: u32 = 0x198;
/* MGBE0 Read mccif */
pub const TEGRA264_MEMORY_CLIENT_MGBE0R: u32 = 0x1a2;
/* MGBE0 Write mccif */
pub const TEGRA264_MEMORY_CLIENT_MGBE0W: u32 = 0x1a3;
/* MGBE1 Read mccif */
pub const TEGRA264_MEMORY_CLIENT_MGBE1R: u32 = 0x1a4;
/* MGBE1 Write mccif */
pub const TEGRA264_MEMORY_CLIENT_MGBE1W: u32 = 0x1a5;
/* VI1 R5 Write client */
pub const TEGRA264_MEMORY_CLIENT_VI1W: u32 = 0x1a6;
/* VI Falcon1 Read client */
pub const TEGRA264_MEMORY_CLIENT_VIFALCON1R: u32 = 0x1a7;
/* VI Falcon1 Write client */
pub const TEGRA264_MEMORY_CLIENT_VIFALCON1W: u32 = 0x1a8;
/* ISP FALCON1 Read client */
pub const TEGRA264_MEMORY_CLIENT_ISPFALCON1R: u32 = 0x1a9;
/* ISP FALCON1 Write client */
pub const TEGRA264_MEMORY_CLIENT_ISPFALCON1W: u32 = 0x1aa;
/* Read Client of RCE1 */
pub const TEGRA264_MEMORY_CLIENT_RCE1R: u32 = 0x1ab;
/* Write client of RCE1 */
pub const TEGRA264_MEMORY_CLIENT_RCE1W: u32 = 0x1ac;
/* SEU2 Read client */
pub const TEGRA264_MEMORY_CLIENT_SEU2R: u32 = 0x1ad;
/* SEU2 Write client */
pub const TEGRA264_MEMORY_CLIENT_SEU2W: u32 = 0x1ae;
/* SEU3 Read client */
pub const TEGRA264_MEMORY_CLIENT_SEU3R: u32 = 0x1af;
/* SEU3 Write client */
pub const TEGRA264_MEMORY_CLIENT_SEU3W: u32 = 0x1b0;
/* PVA0 Falcon Read mccif */
pub const TEGRA264_MEMORY_CLIENT_PVA0R: u32 = 0x1b1;
/* PVA0 Falcon Write mccif */
pub const TEGRA264_MEMORY_CLIENT_PVA0W: u32 = 0x1b2;
/* PVA1 Read mccif */
pub const TEGRA264_MEMORY_CLIENT_PVA1R: u32 = 0x1b3;
/* PVA1 Write mccif */
pub const TEGRA264_MEMORY_CLIENT_PVA1W: u32 = 0x1b4;
/* PVA2 Read mccif */
pub const TEGRA264_MEMORY_CLIENT_PVA2R: u32 = 0x1b5;
/* PVA2 Write mccif */
pub const TEGRA264_MEMORY_CLIENT_PVA2W: u32 = 0x1b6;
/* ISP3 Write client */
pub const TEGRA264_MEMORY_CLIENT_ISP3W: u32 = 0x1b7;
/* ISP2 Read Client */
pub const TEGRA264_MEMORY_CLIENT_ISP2R: u32 = 0x1b8;
/* ISP2 Write Client */
pub const TEGRA264_MEMORY_CLIENT_ISP2W: u32 = 0x1b9;
/* EQOS Read mccif */
pub const TEGRA264_MEMORY_CLIENT_EQOSR: u32 = 0x1bc;
/* EQOS Write mccif */
pub const TEGRA264_MEMORY_CLIENT_EQOSW: u32 = 0x1bd;
/* FSI0 Read mccif */
pub const TEGRA264_MEMORY_CLIENT_FSI0R: u32 = 0x1be;
/* FSI0 Write mccif */
pub const TEGRA264_MEMORY_CLIENT_FSI0W: u32 = 0x1bf;
/* FSI1 Read mccif */
pub const TEGRA264_MEMORY_CLIENT_FSI1R: u32 = 0x1c0;
/* FSI1 Write mccif */
pub const TEGRA264_MEMORY_CLIENT_FSI1W: u32 = 0x1c1;
/* SDMMC0 Read mccif */
pub const TEGRA264_MEMORY_CLIENT_SDMMC0R: u32 = 0x1c2;
/* SDMMC0 Write mccif */
pub const TEGRA264_MEMORY_CLIENT_SDMMC0W: u32 = 0x1c3;
/* Strongbox (SB) read client */
pub const TEGRA264_MEMORY_CLIENT_SBR: u32 = 0x1c6;
/* Strongbox (SB) write client */
pub const TEGRA264_MEMORY_CLIENT_SBW: u32 = 0x1c7;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU0R: u32 = 0x1c8;
pub const TEGRA264_MEMORY_CLIENT_HSS_MIU0W: u32 = 0x1c9;


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
