/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-2-Clause) */
/*
 * Copyright (c) 2023, Qualcomm Innovation Center, Inc. All rights reserved.
 */

/* Generic RPMH Power Domain Indexes */
pub const RPMHPD_CX: i32 = 0;
pub const RPMHPD_CX_AO: i32 = 1;
pub const RPMHPD_EBI: i32 = 2;
pub const RPMHPD_GFX: i32 = 3;
pub const RPMHPD_LCX: i32 = 4;
pub const RPMHPD_LMX: i32 = 5;
pub const RPMHPD_MMCX: i32 = 6;
pub const RPMHPD_MMCX_AO: i32 = 7;
pub const RPMHPD_MX: i32 = 8;
pub const RPMHPD_MX_AO: i32 = 9;
pub const RPMHPD_MXC: i32 = 10;
pub const RPMHPD_MXC_AO: i32 = 11;
pub const RPMHPD_MSS: i32 = 12;
pub const RPMHPD_NSP: i32 = 13;
pub const RPMHPD_NSP0: i32 = 14;
pub const RPMHPD_NSP1: i32 = 15;
pub const RPMHPD_QPHY: i32 = 16;
pub const RPMHPD_DDR: i32 = 17;
pub const RPMHPD_XO: i32 = 18;
pub const RPMHPD_NSP2: i32 = 19;
pub const RPMHPD_GMXC: i32 = 20;
pub const RPMHPD_DCX: i32 = 21;
pub const RPMHPD_GBX: i32 = 22;
pub const RPMHPD_NSP3: i32 = 23;
pub const RPMHPD_GFX1: i32 = 24;

/* RPMh Power Domain performance levels */
pub const RPMH_REGULATOR_LEVEL_RETENTION: i32 = 16;
pub const RPMH_REGULATOR_LEVEL_MIN_SVS: i32 = 48;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D3_0: i32 = 49;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D3: i32 = 50;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D2_1: i32 = 51;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D2: i32 = 52;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D1_1: i32 = 54;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D1_0: i32 = 55;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D1: i32 = 56;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D0_0: i32 = 59;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_D0: i32 = 60;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS: i32 = 64;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_P1: i32 = 72;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_L0: i32 = 76;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_L1: i32 = 80;
pub const RPMH_REGULATOR_LEVEL_LOW_SVS_L2: i32 = 96;
pub const RPMH_REGULATOR_LEVEL_SVS: i32 = 128;
pub const RPMH_REGULATOR_LEVEL_SVS_L0: i32 = 144;
pub const RPMH_REGULATOR_LEVEL_SVS_L1: i32 = 192;
pub const RPMH_REGULATOR_LEVEL_SVS_L2: i32 = 224;
pub const RPMH_REGULATOR_LEVEL_SVS_L2_0: i32 = 225;
pub const RPMH_REGULATOR_LEVEL_NOM: i32 = 256;
pub const RPMH_REGULATOR_LEVEL_NOM_L0: i32 = 288;
pub const RPMH_REGULATOR_LEVEL_NOM_L1: i32 = 320;
pub const RPMH_REGULATOR_LEVEL_NOM_L2: i32 = 336;
pub const RPMH_REGULATOR_LEVEL_TURBO: i32 = 384;
pub const RPMH_REGULATOR_LEVEL_TURBO_L0: i32 = 400;
pub const RPMH_REGULATOR_LEVEL_TURBO_L1: i32 = 416;
pub const RPMH_REGULATOR_LEVEL_TURBO_L1_0: i32 = 417;
pub const RPMH_REGULATOR_LEVEL_TURBO_L1_1: i32 = 418;
pub const RPMH_REGULATOR_LEVEL_TURBO_L1_2: i32 = 419;
pub const RPMH_REGULATOR_LEVEL_TURBO_L2: i32 = 432;
pub const RPMH_REGULATOR_LEVEL_TURBO_L3: i32 = 448;
pub const RPMH_REGULATOR_LEVEL_TURBO_L3_0: i32 = 449;
pub const RPMH_REGULATOR_LEVEL_TURBO_L3_1: i32 = 450;
pub const RPMH_REGULATOR_LEVEL_TURBO_L3_2: i32 = 451;
pub const RPMH_REGULATOR_LEVEL_TURBO_L4: i32 = 452;
pub const RPMH_REGULATOR_LEVEL_TURBO_L5: i32 = 456;
pub const RPMH_REGULATOR_LEVEL_SUPER_TURBO: i32 = 464;
pub const RPMH_REGULATOR_LEVEL_SUPER_TURBO_NO_CPR: i32 = 480;

/* Platform-specific power domain bindings. Don't add new entries here, use
 * RPMHPD_* above.
 */

/* SA8775P Power Domain Indexes */
pub const SA8775P_CX: i32 = 0; pub const SA8775P_CX_AO: i32 = 1; pub const SA8775P_DDR: i32 = 2; pub const SA8775P_EBI: i32 = 3; pub const SA8775P_GFX: i32 = 4; pub const SA8775P_LCX: i32 = 5; pub const SA8775P_LMX: i32 = 6; pub const SA8775P_MMCX: i32 = 7; pub const SA8775P_MMCX_AO: i32 = 8; pub const SA8775P_MSS: i32 = 9; pub const SA8775P_MX: i32 = 10; pub const SA8775P_MX_AO: i32 = 11; pub const SA8775P_MXC: i32 = 12; pub const SA8775P_MXC_AO: i32 = 13; pub const SA8775P_NSP0: i32 = 14; pub const SA8775P_NSP1: i32 = 15; pub const SA8775P_XO: i32 = 16;

/* SDM670 Power Domain Indexes */
pub const SDM670_MX: i32 = 0; pub const SDM670_MX_AO: i32 = 1; pub const SDM670_CX: i32 = 2; pub const SDM670_CX_AO: i32 = 3; pub const SDM670_LMX: i32 = 4; pub const SDM670_LCX: i32 = 5; pub const SDM670_GFX: i32 = 6; pub const SDM670_MSS: i32 = 7;

/* SDM845 Power Domain Indexes */
pub const SDM845_EBI: i32 = 0; pub const SDM845_MX: i32 = 1; pub const SDM845_MX_AO: i32 = 2; pub const SDM845_CX: i32 = 3; pub const SDM845_CX_AO: i32 = 4; pub const SDM845_LMX: i32 = 5; pub const SDM845_LCX: i32 = 6; pub const SDM845_GFX: i32 = 7; pub const SDM845_MSS: i32 = 8;

/* SDX55 Power Domain Indexes */
pub const SDX55_MSS: i32 = 0; pub const SDX55_MX: i32 = 1; pub const SDX55_CX: i32 = 2;
/* SDX65 Power Domain Indexes */
pub const SDX65_MSS: i32 = 0; pub const SDX65_MX: i32 = 1; pub const SDX65_MX_AO: i32 = 2; pub const SDX65_CX: i32 = 3; pub const SDX65_CX_AO: i32 = 4; pub const SDX65_MXC: i32 = 5;
/* SM6350 Power Domain Indexes */
pub const SM6350_CX: i32 = 0; pub const SM6350_GFX: i32 = 1; pub const SM6350_LCX: i32 = 2; pub const SM6350_LMX: i32 = 3; pub const SM6350_MSS: i32 = 4; pub const SM6350_MX: i32 = 5;
/* SM8150 Power Domain Indexes */
pub const SM8150_MSS: i32 = 0; pub const SM8150_EBI: i32 = 1; pub const SM8150_LMX: i32 = 2; pub const SM8150_LCX: i32 = 3; pub const SM8150_GFX: i32 = 4; pub const SM8150_MX: i32 = 5; pub const SM8150_MX_AO: i32 = 6; pub const SM8150_CX: i32 = 7; pub const SM8150_CX_AO: i32 = 8; pub const SM8150_MMCX: i32 = 9; pub const SM8150_MMCX_AO: i32 = 10;
/* SA8155P is a special case, kept for backwards compatibility */
pub const SA8155P_CX: i32 = SM8150_CX; pub const SA8155P_CX_AO: i32 = SM8150_CX_AO; pub const SA8155P_EBI: i32 = SM8150_EBI; pub const SA8155P_GFX: i32 = SM8150_GFX; pub const SA8155P_MSS: i32 = SM8150_MSS; pub const SA8155P_MX: i32 = SM8150_MX; pub const SA8155P_MX_AO: i32 = SM8150_MX_AO;

/* SM8250 Power Domain Indexes */
pub const SM8250_CX: i32 = 0; pub const SM8250_CX_AO: i32 = 1; pub const SM8250_EBI: i32 = 2; pub const SM8250_GFX: i32 = 3; pub const SM8250_LCX: i32 = 4; pub const SM8250_LMX: i32 = 5; pub const SM8250_MMCX: i32 = 6; pub const SM8250_MMCX_AO: i32 = 7; pub const SM8250_MX: i32 = 8; pub const SM8250_MX_AO: i32 = 9;
/* SM8350 Power Domain Indexes */
pub const SM8350_CX: i32 = 0; pub const SM8350_CX_AO: i32 = 1; pub const SM8350_EBI: i32 = 2; pub const SM8350_GFX: i32 = 3; pub const SM8350_LCX: i32 = 4; pub const SM8350_LMX: i32 = 5; pub const SM8350_MMCX: i32 = 6; pub const SM8350_MMCX_AO: i32 = 7; pub const SM8350_MX: i32 = 8; pub const SM8350_MX_AO: i32 = 9; pub const SM8350_MXC: i32 = 10; pub const SM8350_MXC_AO: i32 = 11; pub const SM8350_MSS: i32 = 12;
/* SM8450 Power Domain Indexes */
pub const SM8450_CX: i32 = 0; pub const SM8450_CX_AO: i32 = 1; pub const SM8450_EBI: i32 = 2; pub const SM8450_GFX: i32 = 3; pub const SM8450_LCX: i32 = 4; pub const SM8450_LMX: i32 = 5; pub const SM8450_MMCX: i32 = 6; pub const SM8450_MMCX_AO: i32 = 7; pub const SM8450_MX: i32 = 8; pub const SM8450_MX_AO: i32 = 9; pub const SM8450_MXC: i32 = 10; pub const SM8450_MXC_AO: i32 = 11; pub const SM8450_MSS: i32 = 12;
/* SM8550 Power Domain Indexes */
pub const SM8550_CX: i32 = 0; pub const SM8550_CX_AO: i32 = 1; pub const SM8550_EBI: i32 = 2; pub const SM8550_GFX: i32 = 3; pub const SM8550_LCX: i32 = 4; pub const SM8550_LMX: i32 = 5; pub const SM8550_MMCX: i32 = 6; pub const SM8550_MMCX_AO: i32 = 7; pub const SM8550_MX: i32 = 8; pub const SM8550_MX_AO: i32 = 9; pub const SM8550_MXC: i32 = 10; pub const SM8550_MXC_AO: i32 = 11; pub const SM8550_MSS: i32 = 12; pub const SM8550_NSP: i32 = 13;
/* QDU1000/QRU1000 Power Domain Indexes */
pub const QDU1000_EBI: i32 = 0; pub const QDU1000_MSS: i32 = 1; pub const QDU1000_CX: i32 = 2; pub const QDU1000_MX: i32 = 3;
/* SC7180 Power Domain Indexes */
pub const SC7180_CX: i32 = 0; pub const SC7180_CX_AO: i32 = 1; pub const SC7180_GFX: i32 = 2; pub const SC7180_MX: i32 = 3; pub const SC7180_MX_AO: i32 = 4; pub const SC7180_LMX: i32 = 5; pub const SC7180_LCX: i32 = 6; pub const SC7180_MSS: i32 = 7;
/* SC7280 Power Domain Indexes */
pub const SC7280_CX: i32 = 0; pub const SC7280_CX_AO: i32 = 1; pub const SC7280_EBI: i32 = 2; pub const SC7280_GFX: i32 = 3; pub const SC7280_MX: i32 = 4; pub const SC7280_MX_AO: i32 = 5; pub const SC7280_LMX: i32 = 6; pub const SC7280_LCX: i32 = 7; pub const SC7280_MSS: i32 = 8;
/* SC8180X Power Domain Indexes */
pub const SC8180X_CX: i32 = 0; pub const SC8180X_CX_AO: i32 = 1; pub const SC8180X_EBI: i32 = 2; pub const SC8180X_GFX: i32 = 3; pub const SC8180X_LCX: i32 = 4; pub const SC8180X_LMX: i32 = 5; pub const SC8180X_MMCX: i32 = 6; pub const SC8180X_MMCX_AO: i32 = 7; pub const SC8180X_MSS: i32 = 8; pub const SC8180X_MX: i32 = 9; pub const SC8180X_MX_AO: i32 = 10;
/* SC8280XP Power Domain Indexes */
pub const SC8280XP_CX: i32 = 0; pub const SC8280XP_CX_AO: i32 = 1; pub const SC8280XP_DDR: i32 = 2; pub const SC8280XP_EBI: i32 = 3; pub const SC8280XP_GFX: i32 = 4; pub const SC8280XP_LCX: i32 = 5; pub const SC8280XP_LMX: i32 = 6; pub const SC8280XP_MMCX: i32 = 7; pub const SC8280XP_MMCX_AO: i32 = 8; pub const SC8280XP_MSS: i32 = 9; pub const SC8280XP_MX: i32 = 10; pub const SC8280XP_MXC: i32 = 12; pub const SC8280XP_MX_AO: i32 = 11; pub const SC8280XP_NSP: i32 = 13; pub const SC8280XP_QPHY: i32 = 14; pub const SC8280XP_XO: i32 = 15; pub const SC8280XP_MXC_AO: i32 = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
