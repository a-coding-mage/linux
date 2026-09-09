/* Translated from dal_asic_id.h. */

pub const SI_TAHITI_P_A0: i32 = 0x01;
pub const SI_TAHITI_P_B0: i32 = 0x05;
pub const SI_TAHITI_P_B1: i32 = 0x06;
pub const SI_PITCAIRN_PM_A0: i32 = 0x14;
pub const SI_PITCAIRN_PM_A1: i32 = 0x15;
pub const SI_CAPEVERDE_M_A0: i32 = 0x28;
pub const SI_CAPEVERDE_M_A1: i32 = 0x29;
pub const SI_OLAND_M_A0: i32 = 0x3C;
pub const SI_HAINAN_V_A0: i32 = 0x46;
pub const SI_UNKNOWN: i32 = 0xFF;

macro_rules! rev_between { ($n:ident, $lo:ident, $hi:ident) => { macro_rules! $n { ($rev:expr) => { $rev >= $lo && $rev < $hi }; } }; }
rev_between!(ASIC_REV_IS_TAHITI_P, SI_TAHITI_P_A0, SI_PITCAIRN_PM_A0);
rev_between!(ASIC_REV_IS_PITCAIRN_PM, SI_PITCAIRN_PM_A0, SI_CAPEVERDE_M_A0);
rev_between!(ASIC_REV_IS_CAPEVERDE_M, SI_CAPEVERDE_M_A0, SI_OLAND_M_A0);
rev_between!(ASIC_REV_IS_OLAND_M, SI_OLAND_M_A0, SI_HAINAN_V_A0);
rev_between!(ASIC_REV_IS_HAINAN_V, SI_HAINAN_V_A0, SI_UNKNOWN);

pub const CI_BONAIRE_M_A0: i32 = 0x14;
pub const CI_BONAIRE_M_A1: i32 = 0x15;
pub const CI_HAWAII_P_A0: i32 = 0x28;
pub const CI_UNKNOWN: i32 = 0xFF;
macro_rules! ASIC_REV_IS_BONAIRE_M { ($r:expr) => { $r >= CI_BONAIRE_M_A0 && $r < CI_HAWAII_P_A0 }; }
macro_rules! ASIC_REV_IS_HAWAII_P { ($r:expr) => { $r >= CI_HAWAII_P_A0 }; }

pub const KV_SPECTRE_A0: i32 = 0x01;
pub const KV_SPOOKY_A0: i32 = 0x41;
pub const KB_KALINDI_A0: i32 = 0x81;
pub const KB_KALINDI_A1: i32 = 0x82;
pub const BV_KALINDI_A2: i32 = 0x85;
pub const ML_GODAVARI_A0: i32 = 0xA1;
pub const ML_GODAVARI_A1: i32 = 0xA2;
pub const KV_UNKNOWN: i32 = 0xFF;
macro_rules! ASIC_REV_IS_KALINDI { ($r:expr) => { $r >= KB_KALINDI_A0 && $r < KV_UNKNOWN }; }
macro_rules! ASIC_REV_IS_BHAVANI { ($r:expr) => { $r >= BV_KALINDI_A2 && $r < ML_GODAVARI_A0 }; }
macro_rules! ASIC_REV_IS_GODAVARI { ($r:expr) => { $r >= ML_GODAVARI_A0 && $r < KV_UNKNOWN }; }

pub const VI_TONGA_P_A0: i32 = 20; pub const VI_TONGA_P_A1: i32 = 21; pub const VI_FIJI_P_A0: i32 = 60;
pub const VI_POLARIS10_P_A0: i32 = 80; pub const VI_POLARIS11_M_A0: i32 = 90; pub const VI_POLARIS12_V_A0: i32 = 100; pub const VI_VEGAM_A0: i32 = 110; pub const VI_UNKNOWN: i32 = 0xFF;
macro_rules! ASIC_REV_IS_TONGA_P { ($r:expr) => { $r >= VI_TONGA_P_A0 && $r < 40 }; }
macro_rules! ASIC_REV_IS_FIJI_P { ($r:expr) => { $r >= VI_FIJI_P_A0 && $r < 80 }; }
macro_rules! ASIC_REV_IS_POLARIS10_P { ($r:expr) => { $r >= VI_POLARIS10_P_A0 && $r < VI_POLARIS11_M_A0 }; }
macro_rules! ASIC_REV_IS_POLARIS11_M { ($r:expr) => { $r >= VI_POLARIS11_M_A0 && $r < VI_POLARIS12_V_A0 }; }
macro_rules! ASIC_REV_IS_POLARIS12_V { ($r:expr) => { $r >= VI_POLARIS12_V_A0 && $r < VI_VEGAM_A0 }; }
macro_rules! ASIC_REV_IS_VEGAM { ($r:expr) => { $r >= VI_VEGAM_A0 }; }

pub const CZ_CARRIZO_A0: i32 = 0x01; pub const STONEY_A0: i32 = 0x61; pub const CZ_UNKNOWN: i32 = 0xFF;
macro_rules! ASIC_REV_IS_STONEY { ($r:expr) => { $r >= STONEY_A0 && $r < CZ_UNKNOWN }; }
pub const AI_UNKNOWN: i32 = 0xFF; pub const AI_GREENLAND_P_A0: i32 = 1; pub const AI_GREENLAND_P_A1: i32 = 2; pub const AI_VEGA12_P_A0: i32 = 20; pub const AI_VEGA20_P_A0: i32 = 40;
macro_rules! ASICREV_IS_GREENLAND_M { ($r:expr) => { $r < AI_VEGA12_P_A0 }; }
macro_rules! ASICREV_IS_GREENLAND_P { ($r:expr) => { $r < AI_VEGA12_P_A0 }; }
macro_rules! ASICREV_IS_VEGA12_P { ($r:expr) => { $r >= AI_VEGA12_P_A0 && $r < AI_VEGA20_P_A0 }; }
macro_rules! ASICREV_IS_VEGA20_P { ($r:expr) => { $r >= AI_VEGA20_P_A0 && $r < AI_UNKNOWN }; }

pub const INTERNAL_REV_RAVEN_A0: i32 = 0x00; pub const RAVEN_A0: i32 = 0x01; pub const RAVEN_B0: i32 = 0x21; pub const PICASSO_A0: i32 = 0x41; pub const RAVEN2_A0: i32 = 0x81; pub const RAVEN1_F0: i32 = 0xF0; pub const RAVEN_UNKNOWN: i32 = 0xFF; pub const RENOIR_A0: i32 = 0x91;
macro_rules! ASICREV_IS_RAVEN { ($r:expr) => { $r >= RAVEN_A0 && $r < RAVEN_UNKNOWN }; }
pub const PRID_DALI_DE: i32 = 0xDE; pub const PRID_DALI_DF: i32 = 0xDF; pub const PRID_DALI_E3: i32 = 0xE3; pub const PRID_DALI_E4: i32 = 0xE4;
pub const PRID_POLLOCK_94: i32 = 0x94; pub const PRID_POLLOCK_95: i32 = 0x95; pub const PRID_POLLOCK_E9: i32 = 0xE9; pub const PRID_POLLOCK_EA: i32 = 0xEA; pub const PRID_POLLOCK_EB: i32 = 0xEB;
macro_rules! ASICREV_IS_PICASSO { ($r:expr) => { $r >= PICASSO_A0 && $r < RAVEN2_A0 }; }
macro_rules! ASICREV_IS_RAVEN2 { ($r:expr) => { $r >= RAVEN2_A0 && $r < RENOIR_A0 }; }
macro_rules! ASICREV_IS_RV1_F0 { ($r:expr) => { $r >= RAVEN1_F0 && $r < RAVEN_UNKNOWN }; }
pub const FAMILY_RV: i32 = 142; pub const FAMILY_NV: i32 = 143;

pub const NV_NAVI10_P_A0: i32 = 1; pub const NV_NAVI12_P_A0: i32 = 10; pub const NV_NAVI14_M_A0: i32 = 20; pub const NV_SIENNA_CICHLID_P_A0: i32 = 40; pub const NV_DIMGREY_CAVEFISH_P_A0: i32 = 60; pub const NV_BEIGE_GOBY_P_A0: i32 = 70; pub const NV_UNKNOWN: i32 = 0xFF;
macro_rules! ASICREV_IS_NAVI10_P { ($r:expr) => { $r < NV_NAVI12_P_A0 }; } macro_rules! ASICREV_IS_NAVI12_P { ($r:expr) => { $r >= NV_NAVI12_P_A0 && $r < NV_NAVI14_M_A0 }; } macro_rules! ASICREV_IS_NAVI14_M { ($r:expr) => { $r >= NV_NAVI14_M_A0 && $r < NV_UNKNOWN }; }
macro_rules! ASICREV_IS_RENOIR { ($r:expr) => { $r >= RENOIR_A0 && $r < RAVEN1_F0 }; } macro_rules! ASICREV_IS_SIENNA_CICHLID_P { ($r:expr) => { $r >= NV_SIENNA_CICHLID_P_A0 && $r < NV_DIMGREY_CAVEFISH_P_A0 }; } macro_rules! ASICREV_IS_DIMGREY_CAVEFISH_P { ($r:expr) => { $r >= NV_DIMGREY_CAVEFISH_P_A0 && $r < NV_BEIGE_GOBY_P_A0 }; } macro_rules! ASICREV_IS_BEIGE_GOBY_P { ($r:expr) => { $r >= NV_BEIGE_GOBY_P_A0 && $r < NV_UNKNOWN }; }
pub const GREEN_SARDINE_A0: i32 = 0xA1; macro_rules! ASICREV_IS_GREEN_SARDINE { ($r:expr) => { $r >= GREEN_SARDINE_A0 && $r < 0xFF }; }
pub const DEVICE_ID_NV_13FE: i32 = 0x13FE; pub const DEVICE_ID_NV_143F: i32 = 0x143F; pub const DEVICE_ID_NV_13F9: i32 = 0x13F9; pub const DEVICE_ID_NV_13FA: i32 = 0x13FA; pub const DEVICE_ID_NV_13FB: i32 = 0x13FB; pub const DEVICE_ID_NV_13FC: i32 = 0x13FC; pub const DEVICE_ID_NV_13DB: i32 = 0x13DB;
pub const FAMILY_VGH: i32 = 144; pub const DEVICE_ID_VGH_163F: i32 = 0x163F; pub const DEVICE_ID_VGH_1435: i32 = 0x1435; pub const VANGOGH_A0: i32 = 0x01; pub const VANGOGH_UNKNOWN: i32 = 0xFF;
macro_rules! ASICREV_IS_VANGOGH { ($r:expr) => { $r >= VANGOGH_A0 && $r < VANGOGH_UNKNOWN }; }
pub const FAMILY_YELLOW_CARP: i32 = 146; pub const YELLOW_CARP_A0: i32 = 0x01; pub const YELLOW_CARP_B0: i32 = 0x20; pub const YELLOW_CARP_UNKNOWN: i32 = 0xFF;
macro_rules! ASICREV_IS_YELLOW_CARP { ($r:expr) => { $r >= YELLOW_CARP_A0 && $r < YELLOW_CARP_UNKNOWN }; }
pub const AMDGPU_FAMILY_GC_10_3_6: i32 = 149; pub const GC_10_3_6_A0: i32 = 0x01; pub const GC_10_3_6_UNKNOWN: i32 = 0xFF;
macro_rules! ASICREV_IS_GC_10_3_6 { ($r:expr) => { $r >= GC_10_3_6_A0 && $r < GC_10_3_6_UNKNOWN }; }
pub const AMDGPU_FAMILY_GC_10_3_7: i32 = 151; pub const GC_10_3_7_A0: i32 = 0x01; pub const GC_10_3_7_UNKNOWN: i32 = 0xFF;
macro_rules! ASICREV_IS_GC_10_3_7 { ($r:expr) => { $r >= GC_10_3_7_A0 && $r < GC_10_3_7_UNKNOWN }; }
pub const AMDGPU_FAMILY_GC_11_0_0: i32 = 145; pub const AMDGPU_FAMILY_GC_11_0_1: i32 = 148; pub const AMDGPU_FAMILY_GC_11_5_0: i32 = 150; pub const AMDGPU_FAMILY_GC_11_5_4: i32 = 154;
pub const GC_11_0_0_A0: i32 = 1; pub const GC_11_0_2_A0: i32 = 0x10; pub const GC_11_0_3_A0: i32 = 0x20; pub const GC_11_0_4_A0: i32 = 0xC0; pub const GC_11_UNKNOWN: i32 = 0xFF;
macro_rules! ASICREV_IS_GC_11_0_0 { ($r:expr) => { $r < GC_11_0_2_A0 }; } macro_rules! ASICREV_IS_GC_11_0_2 { ($r:expr) => { $r >= GC_11_0_2_A0 && $r < GC_11_0_3_A0 }; } macro_rules! ASICREV_IS_GC_11_0_3 { ($r:expr) => { $r >= GC_11_0_3_A0 && $r < GC_11_0_4_A0 }; } macro_rules! ASICREV_IS_GC_11_0_4 { ($r:expr) => { $r >= GC_11_0_4_A0 && $r < DCN4A_SOC_VAR_B_A0 }; } macro_rules! ASICREV_IS_DCN36 { ($r:expr) => { $r >= 0x50 && $r < 0xC0 }; }
pub const AMDGPU_FAMILY_GC_12_0_0: i32 = 152; pub const AMDGPU_FAMILY_GC_13_0_1: i32 = 153;
pub const DCN6_VARIANT0_A0: i32 = 1; pub const DCN6_VARIANT1_A0: i32 = DCN6_VARIANT0_A0; pub const DCN6_VARIANT2_A0: i32 = 0x10; pub const DCN6_VARIANT2_B0: i32 = 0x11; pub const DCN6_VARIANT3_A0: i32 = 0x20; pub const DCN6_VARIANT3_UPPER: i32 = 0x2F; pub const DCN6_UNKNOWN: i32 = 0xFF;
macro_rules! ASICREV_IS_DCN6_VARIANT0 { ($r:expr) => { $r < DCN6_VARIANT2_A0 }; } macro_rules! ASICREV_IS_DCN6_VARIANT1 { ($r:expr) => { ASICREV_IS_DCN_VARIANT0!($r) }; } macro_rules! ASICREV_IS_DCN6_VARIANT2 { ($r:expr) => { $r >= DCN6_VARIANT2_A0 && $r < DCN6_VARIANT3_A0 }; } macro_rules! ASICREV_IS_DCN6_VARIANT2_A0 { ($r:expr) => { $r >= DCN6_VARIANT2_A0 && $r < DCN6_VARIANT2_B0 }; } macro_rules! ASICREV_IS_DCN6_VARIANT2_B0 { ($r:expr) => { $r >= DCN6_VARIANT2_B0 && $r < DCN6_VARIANT3_A0 }; } macro_rules! ASICREV_IS_DCN6_VARIANT3 { ($r:expr) => { $r >= DCN6_VARIANT3_A0 && $r <= DCN6_VARIANT3_UPPER }; }
pub const GC_12_0_0_A0: i32 = 0x50; pub const GC_12_0_1_A0: i32 = 0x40; pub const GC_12_UNKNOWN: i32 = 0xFF;
macro_rules! ASICREV_IS_GC_12_0_1_A0 { ($r:expr) => { $r >= GC_12_0_1_A0 && $r < GC_12_0_0_A0 }; } macro_rules! ASICREV_IS_GC_12_0_0_A0 { ($r:expr) => { $r >= GC_12_0_0_A0 && $r < 0xFF }; } macro_rules! ASICREV_IS_DCN4 { ($r:expr) => { $r >= GC_12_0_1_A0 && $r < GC_12_0_0_A0 }; } macro_rules! ASICREV_IS_DCN401 { ($r:expr) => { $r >= GC_12_0_0_A0 && $r < GC_12_UNKNOWN }; }
pub const DCN4A_SOC_VAR_B_A0: i32 = 0xD0; macro_rules! ASICREV_IS_DCN4A_SOC_VAR_B { ($r:expr) => { $r >= DCN4A_SOC_VAR_B_A0 && $r < 0xE0 }; }

pub const DEVICE_ID_SI_TAHITI_P_6780: i32 = 0x6780; pub const DEVICE_ID_SI_PITCAIRN_PM_6800: i32 = 0x6800; pub const DEVICE_ID_SI_PITCAIRN_PM_6808: i32 = 0x6808; pub const DEVICE_ID_SI_CAPEVERDE_M_6820: i32 = 0x6820; pub const DEVICE_ID_SI_CAPEVERDE_M_6828: i32 = 0x6828; pub const DEVICE_ID_SI_OLAND_M_6600: i32 = 0x6600; pub const DEVICE_ID_SI_OLAND_M_6608: i32 = 0x6608; pub const DEVICE_ID_SI_HAINAN_V_6660: i32 = 0x6660;
pub const DEVICE_ID_KALINDI_9834: i32 = 0x9834; pub const DEVICE_ID_TEMASH_9839: i32 = 0x9839; pub const DEVICE_ID_TEMASH_983D: i32 = 0x983D; pub const DEVICE_ID_RENOIR_1636: i32 = 0x1636;
pub const FAMILY_SI: i32 = 110; pub const FAMILY_CI: i32 = 120; pub const FAMILY_KV: i32 = 125; pub const FAMILY_VI: i32 = 130; pub const FAMILY_CZ: i32 = 135; pub const FAMILY_AI: i32 = 141; pub const FAMILY_UNKNOWN: i32 = 0xFF;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
