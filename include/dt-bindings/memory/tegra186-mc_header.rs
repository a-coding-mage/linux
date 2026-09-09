/* special clients */
pub const TEGRA186_SID_INVALID: u32 = 0x00;
pub const TEGRA186_SID_PASSTHROUGH: u32 = 0x7f;

/* host1x clients */
pub const TEGRA186_SID_HOST1X: u32 = 0x01;
pub const TEGRA186_SID_CSI: u32 = 0x02;
pub const TEGRA186_SID_VIC: u32 = 0x03;
pub const TEGRA186_SID_VI: u32 = 0x04;
pub const TEGRA186_SID_ISP: u32 = 0x05;
pub const TEGRA186_SID_NVDEC: u32 = 0x06;
pub const TEGRA186_SID_NVENC: u32 = 0x07;
pub const TEGRA186_SID_NVJPG: u32 = 0x08;
pub const TEGRA186_SID_NVDISPLAY: u32 = 0x09;
pub const TEGRA186_SID_TSEC: u32 = 0x0a;
pub const TEGRA186_SID_TSECB: u32 = 0x0b;
pub const TEGRA186_SID_SE: u32 = 0x0c;
pub const TEGRA186_SID_SE1: u32 = 0x0d;
pub const TEGRA186_SID_SE2: u32 = 0x0e;
pub const TEGRA186_SID_SE3: u32 = 0x0f;

/* GPU clients */
pub const TEGRA186_SID_GPU: u32 = 0x10;

/* other SoC clients */
pub const TEGRA186_SID_AFI: u32 = 0x11;
pub const TEGRA186_SID_HDA: u32 = 0x12;
pub const TEGRA186_SID_ETR: u32 = 0x13;
pub const TEGRA186_SID_EQOS: u32 = 0x14;
pub const TEGRA186_SID_UFSHC: u32 = 0x15;
pub const TEGRA186_SID_AON: u32 = 0x16;
pub const TEGRA186_SID_SDMMC4: u32 = 0x17;
pub const TEGRA186_SID_SDMMC3: u32 = 0x18;
pub const TEGRA186_SID_SDMMC2: u32 = 0x19;
pub const TEGRA186_SID_SDMMC1: u32 = 0x1a;
pub const TEGRA186_SID_XUSB_HOST: u32 = 0x1b;
pub const TEGRA186_SID_XUSB_DEV: u32 = 0x1c;
pub const TEGRA186_SID_SATA: u32 = 0x1d;
pub const TEGRA186_SID_APE: u32 = 0x1e;
pub const TEGRA186_SID_SCE: u32 = 0x1f;

/* GPC DMA clients */
pub const TEGRA186_SID_GPCDMA_0: u32 = 0x20;
pub const TEGRA186_SID_GPCDMA_1: u32 = 0x21;
pub const TEGRA186_SID_GPCDMA_2: u32 = 0x22;
pub const TEGRA186_SID_GPCDMA_3: u32 = 0x23;
pub const TEGRA186_SID_GPCDMA_4: u32 = 0x24;
pub const TEGRA186_SID_GPCDMA_5: u32 = 0x25;
pub const TEGRA186_SID_GPCDMA_6: u32 = 0x26;
pub const TEGRA186_SID_GPCDMA_7: u32 = 0x27;

/* APE DMA clients */
pub const TEGRA186_SID_APE_1: u32 = 0x28;
pub const TEGRA186_SID_APE_2: u32 = 0x29;
/* camera RTCPU */
pub const TEGRA186_SID_RCE: u32 = 0x2a;
/* camera RTCPU on host1x address space */
pub const TEGRA186_SID_RCE_1X: u32 = 0x2b;
/* APE DMA clients */
pub const TEGRA186_SID_APE_3: u32 = 0x2c;
/* camera RTCPU running on APE */
pub const TEGRA186_SID_APE_CAM: u32 = 0x2d;
pub const TEGRA186_SID_APE_CAM_1X: u32 = 0x2e;

/*
 * The BPMP has its SID value hardcoded in the firmware. Changing it requires
 * considerable effort.
 */
pub const TEGRA186_SID_BPMP: u32 = 0x32;
/* for SMMU tests */
pub const TEGRA186_SID_SMMU_TEST: u32 = 0x33;

/* host1x virtualization channels */
pub const TEGRA186_SID_HOST1X_CTX0: u32 = 0x38;
pub const TEGRA186_SID_HOST1X_CTX1: u32 = 0x39;
pub const TEGRA186_SID_HOST1X_CTX2: u32 = 0x3a;
pub const TEGRA186_SID_HOST1X_CTX3: u32 = 0x3b;
pub const TEGRA186_SID_HOST1X_CTX4: u32 = 0x3c;
pub const TEGRA186_SID_HOST1X_CTX5: u32 = 0x3d;
pub const TEGRA186_SID_HOST1X_CTX6: u32 = 0x3e;
pub const TEGRA186_SID_HOST1X_CTX7: u32 = 0x3f;

/* host1x command buffers */
pub const TEGRA186_SID_HOST1X_VM0: u32 = 0x40;
pub const TEGRA186_SID_HOST1X_VM1: u32 = 0x41;
pub const TEGRA186_SID_HOST1X_VM2: u32 = 0x42;
pub const TEGRA186_SID_HOST1X_VM3: u32 = 0x43;
pub const TEGRA186_SID_HOST1X_VM4: u32 = 0x44;
pub const TEGRA186_SID_HOST1X_VM5: u32 = 0x45;
pub const TEGRA186_SID_HOST1X_VM6: u32 = 0x46;
pub const TEGRA186_SID_HOST1X_VM7: u32 = 0x47;

/* SE data buffers */
pub const TEGRA186_SID_SE_VM0: u32 = 0x48;
pub const TEGRA186_SID_SE_VM1: u32 = 0x49;
pub const TEGRA186_SID_SE_VM2: u32 = 0x4a;
pub const TEGRA186_SID_SE_VM3: u32 = 0x4b;
pub const TEGRA186_SID_SE_VM4: u32 = 0x4c;
pub const TEGRA186_SID_SE_VM5: u32 = 0x4d;
pub const TEGRA186_SID_SE_VM6: u32 = 0x4e;
pub const TEGRA186_SID_SE_VM7: u32 = 0x4f;

/* memory client IDs */
pub const TEGRA186_MEMORY_CLIENT_PTCR: u32 = 0x00;
/* Misses from System Memory Management Unit (SMMU) Page Table Cache (PTC) */
pub const TEGRA186_MEMORY_CLIENT_AFIR: u32 = 0x0e;
/* PCIE reads */
pub const TEGRA186_MEMORY_CLIENT_HDAR: u32 = 0x15;
/* High-definition audio (HDA) reads */
pub const TEGRA186_MEMORY_CLIENT_HOST1XDMAR: u32 = 0x16;
pub const TEGRA186_MEMORY_CLIENT_NVENCSRD: u32 = 0x1c;
/* SATA reads */
pub const TEGRA186_MEMORY_CLIENT_SATAR: u32 = 0x1f;
/* Reads from Cortex-A9 4 CPU cores via the L2 cache */
pub const TEGRA186_MEMORY_CLIENT_MPCORER: u32 = 0x27;
pub const TEGRA186_MEMORY_CLIENT_NVENCSWR: u32 = 0x2b;
/* PCIE writes */
pub const TEGRA186_MEMORY_CLIENT_AFIW: u32 = 0x31;
/* High-definition audio (HDA) writes */
pub const TEGRA186_MEMORY_CLIENT_HDAW: u32 = 0x35;
/* Writes from Cortex-A9 4 CPU cores via the L2 cache */
pub const TEGRA186_MEMORY_CLIENT_MPCOREW: u32 = 0x39;
/* SATA writes */
pub const TEGRA186_MEMORY_CLIENT_SATAW: u32 = 0x3d;
/* ISP Read client for Crossbar A */
pub const TEGRA186_MEMORY_CLIENT_ISPRA: u32 = 0x44;
/* ISP Write client for Crossbar A */
pub const TEGRA186_MEMORY_CLIENT_ISPWA: u32 = 0x46;
/* ISP Write client Crossbar B */
pub const TEGRA186_MEMORY_CLIENT_ISPWB: u32 = 0x47;
/* XUSB reads */
pub const TEGRA186_MEMORY_CLIENT_XUSB_HOSTR: u32 = 0x4a;
/* XUSB_HOST writes */
pub const TEGRA186_MEMORY_CLIENT_XUSB_HOSTW: u32 = 0x4b;
/* XUSB reads */
pub const TEGRA186_MEMORY_CLIENT_XUSB_DEVR: u32 = 0x4c;
/* XUSB_DEV writes */
pub const TEGRA186_MEMORY_CLIENT_XUSB_DEVW: u32 = 0x4d;
/* TSEC Memory Return Data Client Description */
pub const TEGRA186_MEMORY_CLIENT_TSECSRD: u32 = 0x54;
/* TSEC Memory Write Client Description */
pub const TEGRA186_MEMORY_CLIENT_TSECSWR: u32 = 0x55;
/* 3D, ltcx reads instance 0 */
pub const TEGRA186_MEMORY_CLIENT_GPUSRD: u32 = 0x58;
/* 3D, ltcx writes instance 0 */
pub const TEGRA186_MEMORY_CLIENT_GPUSWR: u32 = 0x59;
/* sdmmca memory read client */
pub const TEGRA186_MEMORY_CLIENT_SDMMCRA: u32 = 0x60;
/* sdmmcbmemory read client */
pub const TEGRA186_MEMORY_CLIENT_SDMMCRAA: u32 = 0x61;
/* sdmmc memory read client */
pub const TEGRA186_MEMORY_CLIENT_SDMMCR: u32 = 0x62;
/* sdmmcd memory read client */
pub const TEGRA186_MEMORY_CLIENT_SDMMCRAB: u32 = 0x63;
/* sdmmca memory write client */
pub const TEGRA186_MEMORY_CLIENT_SDMMCWA: u32 = 0x64;
/* sdmmcb memory write client */
pub const TEGRA186_MEMORY_CLIENT_SDMMCWAA: u32 = 0x65;
/* sdmmc memory write client */
pub const TEGRA186_MEMORY_CLIENT_SDMMCW: u32 = 0x66;
/* sdmmcd memory write client */
pub const TEGRA186_MEMORY_CLIENT_SDMMCWAB: u32 = 0x67;
pub const TEGRA186_MEMORY_CLIENT_VICSRD: u32 = 0x6c;
pub const TEGRA186_MEMORY_CLIENT_VICSWR: u32 = 0x6d;
/* VI Write client */
pub const TEGRA186_MEMORY_CLIENT_VIW: u32 = 0x72;
pub const TEGRA186_MEMORY_CLIENT_NVDECSRD: u32 = 0x78;
pub const TEGRA186_MEMORY_CLIENT_NVDECSWR: u32 = 0x79;
/* Audio Processing (APE) engine reads */
pub const TEGRA186_MEMORY_CLIENT_APER: u32 = 0x7a;
/* Audio Processing (APE) engine writes */
pub const TEGRA186_MEMORY_CLIENT_APEW: u32 = 0x7b;
pub const TEGRA186_MEMORY_CLIENT_NVJPGSRD: u32 = 0x7e;
pub const TEGRA186_MEMORY_CLIENT_NVJPGSWR: u32 = 0x7f;
/* SE Memory Return Data Client Description */
pub const TEGRA186_MEMORY_CLIENT_SESRD: u32 = 0x80;
pub const TEGRA186_MEMORY_CLIENT_SESWR: u32 = 0x81;
/* ETR reads */
pub const TEGRA186_MEMORY_CLIENT_ETRR: u32 = 0x84;
pub const TEGRA186_MEMORY_CLIENT_ETRW: u32 = 0x85;
/* TSECB Memory Return Data Client Description */
pub const TEGRA186_MEMORY_CLIENT_TSECSRDB: u32 = 0x86;
pub const TEGRA186_MEMORY_CLIENT_TSECSWRB: u32 = 0x87;
/* 3D, ltcx reads instance 1 */
pub const TEGRA186_MEMORY_CLIENT_GPUSRD2: u32 = 0x88;
pub const TEGRA186_MEMORY_CLIENT_GPUSWR2: u32 = 0x89;
/* AXI Switch read client */
pub const TEGRA186_MEMORY_CLIENT_AXISR: u32 = 0x8c;
pub const TEGRA186_MEMORY_CLIENT_AXISW: u32 = 0x8d;
/* EQOS read client */
pub const TEGRA186_MEMORY_CLIENT_EQOSR: u32 = 0x8e;
pub const TEGRA186_MEMORY_CLIENT_EQOSW: u32 = 0x8f;
/* UFSHC read client */
pub const TEGRA186_MEMORY_CLIENT_UFSHCR: u32 = 0x90;
pub const TEGRA186_MEMORY_CLIENT_UFSHCW: u32 = 0x91;
/* NVDISPLAY read client */
pub const TEGRA186_MEMORY_CLIENT_NVDISPLAYR: u32 = 0x92;
/* BPMP read client */
pub const TEGRA186_MEMORY_CLIENT_BPMPR: u32 = 0x93;
pub const TEGRA186_MEMORY_CLIENT_BPMPW: u32 = 0x94;
/* BPMPDMA read client */
pub const TEGRA186_MEMORY_CLIENT_BPMPDMAR: u32 = 0x95;
pub const TEGRA186_MEMORY_CLIENT_BPMPDMAW: u32 = 0x96;
/* AON read client */
pub const TEGRA186_MEMORY_CLIENT_AONR: u32 = 0x97;
pub const TEGRA186_MEMORY_CLIENT_AONW: u32 = 0x98;
/* AONDMA read client */
pub const TEGRA186_MEMORY_CLIENT_AONDMAR: u32 = 0x99;
pub const TEGRA186_MEMORY_CLIENT_AONDMAW: u32 = 0x9a;
/* SCE read client */
pub const TEGRA186_MEMORY_CLIENT_SCER: u32 = 0x9b;
pub const TEGRA186_MEMORY_CLIENT_SCEW: u32 = 0x9c;
/* SCEDMA read client */
pub const TEGRA186_MEMORY_CLIENT_SCEDMAR: u32 = 0x9d;
pub const TEGRA186_MEMORY_CLIENT_SCEDMAW: u32 = 0x9e;
/* APEDMA read client */
pub const TEGRA186_MEMORY_CLIENT_APEDMAR: u32 = 0x9f;
pub const TEGRA186_MEMORY_CLIENT_APEDMAW: u32 = 0xa0;
/* NVDISPLAY read client instance 2 */
pub const TEGRA186_MEMORY_CLIENT_NVDISPLAYR1: u32 = 0xa1;
pub const TEGRA186_MEMORY_CLIENT_VICSRD1: u32 = 0xa2;
pub const TEGRA186_MEMORY_CLIENT_NVDECSRD1: u32 = 0xa3;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
