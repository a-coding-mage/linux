/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Common Intel AGPGART and GTT definitions.
 */

/* Intel registers */
pub const INTEL_APSIZE: u32 = 0xb4;
pub const INTEL_ATTBASE: u32 = 0xb8;
pub const INTEL_AGPCTRL: u32 = 0xb0;
pub const INTEL_NBXCFG: u32 = 0x50;
pub const INTEL_ERRSTS: u32 = 0x91;

/* Intel i830 registers */
pub const I830_GMCH_CTRL: u32 = 0x52;
pub const I830_GMCH_ENABLED: u32 = 0x4;
pub const I830_GMCH_MEM_MASK: u32 = 0x1;
pub const I830_GMCH_MEM_64M: u32 = 0x1;
pub const I830_GMCH_MEM_128M: u32 = 0;
pub const I830_GMCH_GMS_MASK: u32 = 0x70;
pub const I830_GMCH_GMS_DISABLED: u32 = 0x00;
pub const I830_GMCH_GMS_LOCAL: u32 = 0x10;
pub const I830_GMCH_GMS_STOLEN_512: u32 = 0x20;
pub const I830_GMCH_GMS_STOLEN_1024: u32 = 0x30;
pub const I830_GMCH_GMS_STOLEN_8192: u32 = 0x40;
pub const I830_RDRAM_CHANNEL_TYPE: u32 = 0x03010;
pub const fn I830_RDRAM_ND(x: u32) -> u32 {
    ((x) & 0x20) >> 5
}
pub const fn I830_RDRAM_DDT(x: u32) -> u32 {
    ((x) & 0x18) >> 3
}

/* This one is for I830MP w. an external graphic card */
pub const INTEL_I830_ERRSTS: u32 = 0x92;

/* Intel 855GM/852GM registers */
pub const I855_GMCH_GMS_MASK: u32 = 0xF0;
pub const I855_GMCH_GMS_STOLEN_0M: u32 = 0x0;
pub const I855_GMCH_GMS_STOLEN_1M: u32 = 0x1 << 4;
pub const I855_GMCH_GMS_STOLEN_4M: u32 = 0x2 << 4;
pub const I855_GMCH_GMS_STOLEN_8M: u32 = 0x3 << 4;
pub const I855_GMCH_GMS_STOLEN_16M: u32 = 0x4 << 4;
pub const I855_GMCH_GMS_STOLEN_32M: u32 = 0x5 << 4;
pub const I85X_CAPID: u32 = 0x44;
pub const I85X_VARIANT_MASK: u32 = 0x7;
pub const I85X_VARIANT_SHIFT: u32 = 5;
pub const I855_GME: u32 = 0x0;
pub const I855_GM: u32 = 0x4;
pub const I852_GME: u32 = 0x2;
pub const I852_GM: u32 = 0x5;

/* Intel i845 registers */
pub const INTEL_I845_AGPM: u32 = 0x51;
pub const INTEL_I845_ERRSTS: u32 = 0xc8;

/* Intel i860 registers */
pub const INTEL_I860_MCHCFG: u32 = 0x50;
pub const INTEL_I860_ERRSTS: u32 = 0xc8;

/* Intel i810 registers */
pub const I810_GMADR_BAR: u32 = 0;
pub const I810_MMADR_BAR: u32 = 1;
pub const I810_PTE_BASE: u32 = 0x10000;
pub const I810_PTE_MAIN_UNCACHED: u32 = 0x00000000;
pub const I810_PTE_LOCAL: u32 = 0x00000002;
pub const I810_PTE_VALID: u32 = 0x00000001;
pub const I830_PTE_SYSTEM_CACHED: u32 = 0x00000006;

pub const I810_SMRAM_MISCC: u32 = 0x70;
pub const I810_GFX_MEM_WIN_SIZE: u32 = 0x00010000;
pub const I810_GFX_MEM_WIN_32M: u32 = 0x00010000;
pub const I810_GMS: u32 = 0x000000c0;
pub const I810_GMS_DISABLE: u32 = 0x00000000;
pub const I810_PGETBL_CTL: u32 = 0x2020;
pub const I810_PGETBL_ENABLED: u32 = 0x00000001;
/* Note: PGETBL_CTL2 has a different offset on G33. */
pub const I965_PGETBL_CTL2: u32 = 0x20c4;
pub const I965_PGETBL_SIZE_MASK: u32 = 0x0000000e;
pub const I965_PGETBL_SIZE_512KB: u32 = 0 << 1;
pub const I965_PGETBL_SIZE_256KB: u32 = 1 << 1;
pub const I965_PGETBL_SIZE_128KB: u32 = 2 << 1;
pub const I965_PGETBL_SIZE_1MB: u32 = 3 << 1;
pub const I965_PGETBL_SIZE_2MB: u32 = 4 << 1;
pub const I965_PGETBL_SIZE_1_5MB: u32 = 5 << 1;
pub const G33_GMCH_SIZE_MASK: u32 = 3 << 8;
pub const G33_GMCH_SIZE_1M: u32 = 1 << 8;
pub const G33_GMCH_SIZE_2M: u32 = 2 << 8;
pub const G4x_GMCH_SIZE_MASK: u32 = 0xf << 8;
pub const G4x_GMCH_SIZE_1M: u32 = 0x1 << 8;
pub const G4x_GMCH_SIZE_2M: u32 = 0x3 << 8;
pub const G4x_GMCH_SIZE_VT_EN: u32 = 0x8 << 8;
pub const G4x_GMCH_SIZE_VT_1M: u32 = G4x_GMCH_SIZE_1M | G4x_GMCH_SIZE_VT_EN;
pub const G4x_GMCH_SIZE_VT_1_5M: u32 = (0x2 << 8) | G4x_GMCH_SIZE_VT_EN;
pub const G4x_GMCH_SIZE_VT_2M: u32 = G4x_GMCH_SIZE_2M | G4x_GMCH_SIZE_VT_EN;

pub const GFX_FLSH_CNTL: u32 = 0x2170;  /* 915+ */

pub const I810_DRAM_CTL: u32 = 0x3000;
pub const I810_DRAM_ROW_0: u32 = 0x00000001;
pub const I810_DRAM_ROW_0_SDRAM: u32 = 0x00000001;

/* Intel 815 register */
pub const INTEL_815_APCONT: u32 = 0x51;
pub const INTEL_815_ATTBASE_MASK: u32 = ! 0x1FFFFFFF;

/* Intel i820 registers */
pub const INTEL_I820_RDCR: u32 = 0x51;
pub const INTEL_I820_ERRSTS: u32 = 0xc8;

/* Intel i840 registers */
pub const INTEL_I840_MCHCFG: u32 = 0x50;
pub const INTEL_I840_ERRSTS: u32 = 0xc8;

/* Intel i850 registers */
pub const INTEL_I850_MCHCFG: u32 = 0x50;
pub const INTEL_I850_ERRSTS: u32 = 0xc8;

/* intel 915G registers */
pub const I915_GMADR_BAR: u32 = 2;
pub const I915_MMADR_BAR: u32 = 0;
pub const I915_PTE_BAR: u32 = 3;
pub const I915_GMCH_GMS_STOLEN_48M: u32 = 0x6 << 4;
pub const I915_GMCH_GMS_STOLEN_64M: u32 = 0x7 << 4;
pub const G33_GMCH_GMS_STOLEN_128M: u32 = 0x8 << 4;
pub const G33_GMCH_GMS_STOLEN_256M: u32 = 0x9 << 4;
pub const INTEL_GMCH_GMS_STOLEN_96M: u32 = 0xa << 4;
pub const INTEL_GMCH_GMS_STOLEN_160M: u32 = 0xb << 4;
pub const INTEL_GMCH_GMS_STOLEN_224M: u32 = 0xc << 4;
pub const INTEL_GMCH_GMS_STOLEN_352M: u32 = 0xd << 4;

pub const I915_IFPADDR: u32 = 0x60;
pub const I830_HIC: u32 = 0x70;

/* Intel 965G registers */
pub const I965_MSAC: u32 = 0x62;
pub const I965_IFPADDR: u32 = 0x70;

/* Intel 7505 registers */
pub const INTEL_I7505_APSIZE: u32 = 0x74;
pub const INTEL_I7505_NCAPID: u32 = 0x60;
pub const INTEL_I7505_NISTAT: u32 = 0x6c;
pub const INTEL_I7505_ATTBASE: u32 = 0x78;
pub const INTEL_I7505_ERRSTS: u32 = 0x42;
pub const INTEL_I7505_AGPCTRL: u32 = 0x70;
pub const INTEL_I7505_MCHCFG: u32 = 0x50;

/* pci devices ids */
pub const PCI_DEVICE_ID_INTEL_E7221_HB: u32 = 0x2588;
pub const PCI_DEVICE_ID_INTEL_E7221_IG: u32 = 0x258a;
pub const PCI_DEVICE_ID_INTEL_82946GZ_HB: u32 = 0x2970;
pub const PCI_DEVICE_ID_INTEL_82946GZ_IG: u32 = 0x2972;
pub const PCI_DEVICE_ID_INTEL_82G35_HB: u32 = 0x2980;
pub const PCI_DEVICE_ID_INTEL_82G35_IG: u32 = 0x2982;
pub const PCI_DEVICE_ID_INTEL_82965Q_HB: u32 = 0x2990;
pub const PCI_DEVICE_ID_INTEL_82965Q_IG: u32 = 0x2992;
pub const PCI_DEVICE_ID_INTEL_82965G_HB: u32 = 0x29A0;
pub const PCI_DEVICE_ID_INTEL_82965G_IG: u32 = 0x29A2;
pub const PCI_DEVICE_ID_INTEL_82965GM_HB: u32 = 0x2A00;
pub const PCI_DEVICE_ID_INTEL_82965GM_IG: u32 = 0x2A02;
pub const PCI_DEVICE_ID_INTEL_82965GME_HB: u32 = 0x2A10;
pub const PCI_DEVICE_ID_INTEL_82965GME_IG: u32 = 0x2A12;
pub const PCI_DEVICE_ID_INTEL_82945GME_HB: u32 = 0x27AC;
pub const PCI_DEVICE_ID_INTEL_82945GME_IG: u32 = 0x27AE;
pub const PCI_DEVICE_ID_INTEL_PINEVIEW_M_HB: u32 = 0xA010;
pub const PCI_DEVICE_ID_INTEL_PINEVIEW_M_IG: u32 = 0xA011;
pub const PCI_DEVICE_ID_INTEL_PINEVIEW_HB: u32 = 0xA000;
pub const PCI_DEVICE_ID_INTEL_PINEVIEW_IG: u32 = 0xA001;
pub const PCI_DEVICE_ID_INTEL_G33_HB: u32 = 0x29C0;
pub const PCI_DEVICE_ID_INTEL_G33_IG: u32 = 0x29C2;
pub const PCI_DEVICE_ID_INTEL_Q35_HB: u32 = 0x29B0;
pub const PCI_DEVICE_ID_INTEL_Q35_IG: u32 = 0x29B2;
pub const PCI_DEVICE_ID_INTEL_Q33_HB: u32 = 0x29D0;
pub const PCI_DEVICE_ID_INTEL_Q33_IG: u32 = 0x29D2;
pub const PCI_DEVICE_ID_INTEL_B43_HB: u32 = 0x2E40;
pub const PCI_DEVICE_ID_INTEL_B43_IG: u32 = 0x2E42;
pub const PCI_DEVICE_ID_INTEL_B43_1_HB: u32 = 0x2E90;
pub const PCI_DEVICE_ID_INTEL_B43_1_IG: u32 = 0x2E92;
pub const PCI_DEVICE_ID_INTEL_GM45_HB: u32 = 0x2A40;
pub const PCI_DEVICE_ID_INTEL_GM45_IG: u32 = 0x2A42;
pub const PCI_DEVICE_ID_INTEL_EAGLELAKE_HB: u32 = 0x2E00;
pub const PCI_DEVICE_ID_INTEL_EAGLELAKE_IG: u32 = 0x2E02;
pub const PCI_DEVICE_ID_INTEL_Q45_HB: u32 = 0x2E10;
pub const PCI_DEVICE_ID_INTEL_Q45_IG: u32 = 0x2E12;
pub const PCI_DEVICE_ID_INTEL_G45_HB: u32 = 0x2E20;
pub const PCI_DEVICE_ID_INTEL_G45_IG: u32 = 0x2E22;
pub const PCI_DEVICE_ID_INTEL_G41_HB: u32 = 0x2E30;
pub const PCI_DEVICE_ID_INTEL_G41_IG: u32 = 0x2E32;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_D_HB: u32 = 0x0040;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_D2_HB: u32 = 0x0069;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_D_IG: u32 = 0x0042;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_M_HB: u32 = 0x0044;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_MA_HB: u32 = 0x0062;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_MC2_HB: u32 = 0x006a;
pub const PCI_DEVICE_ID_INTEL_IRONLAKE_M_IG: u32 = 0x0046;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
