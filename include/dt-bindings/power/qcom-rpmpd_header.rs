/* SPDX-License-Identifier: GPL-2.0 */
/* Copyright (c) 2018, The Linux Foundation. All rights reserved. */

// Dependency: <dt-bindings/power/qcom,rpmhpd.h>

/* Generic RPM Power Domain Indexes */
pub const RPMPD_VDDCX: i32 = 0;
pub const RPMPD_VDDCX_AO: i32 = 1;
/* VFC and VFL are mutually exclusive and can not be present on the same platform */
pub const RPMPD_VDDCX_VFC: i32 = 2;
pub const RPMPD_VDDCX_VFL: i32 = 2;
pub const RPMPD_VDDMX: i32 = 3;
pub const RPMPD_VDDMX_AO: i32 = 4;
pub const RPMPD_VDDMX_VFL: i32 = 5;
pub const RPMPD_SSCCX: i32 = 6;
pub const RPMPD_SSCCX_VFL: i32 = 7;
pub const RPMPD_SSCMX: i32 = 8;
pub const RPMPD_SSCMX_VFL: i32 = 9;

/*
 * Platform-specific power domain bindings. Don't add new entries here, use
 * RPMPD_* above.
 */

/* MDM9607 Power Domains */
pub const MDM9607_VDDCX: i32 = RPMPD_VDDCX;
pub const MDM9607_VDDCX_AO: i32 = RPMPD_VDDCX_AO;
pub const MDM9607_VDDCX_VFL: i32 = RPMPD_VDDCX_VFL;
pub const MDM9607_VDDMX: i32 = RPMPD_VDDMX;
pub const MDM9607_VDDMX_AO: i32 = RPMPD_VDDMX_AO;
pub const MDM9607_VDDMX_VFL: i32 = RPMPD_VDDMX_VFL;

/* MSM8226 Power Domain Indexes */
pub const MSM8226_VDDCX: i32 = RPMPD_VDDCX;
pub const MSM8226_VDDCX_AO: i32 = RPMPD_VDDCX_AO;
pub const MSM8226_VDDCX_VFC: i32 = RPMPD_VDDCX_VFC;

/* MSM8939 Power Domains */
pub const MSM8939_VDDMDCX: i32 = 0;
pub const MSM8939_VDDMDCX_AO: i32 = 1;
pub const MSM8939_VDDMDCX_VFC: i32 = 2;
pub const MSM8939_VDDCX: i32 = 3;
pub const MSM8939_VDDCX_AO: i32 = 4;
pub const MSM8939_VDDCX_VFC: i32 = 5;
pub const MSM8939_VDDMX: i32 = 6;
pub const MSM8939_VDDMX_AO: i32 = 7;

/* MSM8916 Power Domain Indexes */
pub const MSM8916_VDDCX: i32 = RPMPD_VDDCX;
pub const MSM8916_VDDCX_AO: i32 = RPMPD_VDDCX_AO;
pub const MSM8916_VDDCX_VFC: i32 = RPMPD_VDDCX_VFC;
pub const MSM8916_VDDMX: i32 = RPMPD_VDDMX;
pub const MSM8916_VDDMX_AO: i32 = RPMPD_VDDMX_AO;

/* MSM8909 Power Domain Indexes */
pub const MSM8909_VDDCX: i32 = MSM8916_VDDCX;
pub const MSM8909_VDDCX_AO: i32 = MSM8916_VDDCX_AO;
pub const MSM8909_VDDCX_VFC: i32 = MSM8916_VDDCX_VFC;
pub const MSM8909_VDDMX: i32 = MSM8916_VDDMX;
pub const MSM8909_VDDMX_AO: i32 = MSM8916_VDDMX_AO;

/* MSM8917 Power Domain Indexes */
pub const MSM8917_VDDCX: i32 = RPMPD_VDDCX;
pub const MSM8917_VDDCX_AO: i32 = RPMPD_VDDCX_AO;
pub const MSM8917_VDDCX_VFL: i32 = RPMPD_VDDCX_VFL;
pub const MSM8917_VDDMX: i32 = RPMPD_VDDMX;
pub const MSM8917_VDDMX_AO: i32 = RPMPD_VDDMX_AO;

/* MSM8937 Power Domain Indexes */
pub const MSM8937_VDDCX: i32 = MSM8917_VDDCX;
pub const MSM8937_VDDCX_AO: i32 = MSM8917_VDDCX_AO;
pub const MSM8937_VDDCX_VFL: i32 = MSM8917_VDDCX_VFL;
pub const MSM8937_VDDMX: i32 = MSM8917_VDDMX;
pub const MSM8937_VDDMX_AO: i32 = MSM8917_VDDMX_AO;

/* QM215 Power Domain Indexes */
pub const QM215_VDDCX: i32 = MSM8917_VDDCX;
pub const QM215_VDDCX_AO: i32 = MSM8917_VDDCX_AO;
pub const QM215_VDDCX_VFL: i32 = MSM8917_VDDCX_VFL;
pub const QM215_VDDMX: i32 = MSM8917_VDDMX;
pub const QM215_VDDMX_AO: i32 = MSM8917_VDDMX_AO;

/* MSM8953 Power Domain Indexes */
pub const MSM8953_VDDMD: i32 = 0;
pub const MSM8953_VDDMD_AO: i32 = 1;
pub const MSM8953_VDDCX: i32 = 2;
pub const MSM8953_VDDCX_AO: i32 = 3;
pub const MSM8953_VDDCX_VFL: i32 = 4;
pub const MSM8953_VDDMX: i32 = 5;
pub const MSM8953_VDDMX_AO: i32 = 6;

/* MSM8974 Power Domain Indexes */
pub const MSM8974_VDDCX: i32 = 0;
pub const MSM8974_VDDCX_AO: i32 = 1;
pub const MSM8974_VDDCX_VFC: i32 = 2;
pub const MSM8974_VDDGFX: i32 = 3;
pub const MSM8974_VDDGFX_VFC: i32 = 4;

/* MSM8976 Power Domain Indexes */
pub const MSM8976_VDDCX: i32 = RPMPD_VDDCX;
pub const MSM8976_VDDCX_AO: i32 = RPMPD_VDDCX_AO;
pub const MSM8976_VDDCX_VFL: i32 = RPMPD_VDDCX_VFL;
pub const MSM8976_VDDMX: i32 = RPMPD_VDDMX;
pub const MSM8976_VDDMX_AO: i32 = RPMPD_VDDMX_AO;
pub const MSM8976_VDDMX_VFL: i32 = RPMPD_VDDMX_VFL;

/* MSM8994 Power Domain Indexes */
pub const MSM8994_VDDCX: i32 = 0;
pub const MSM8994_VDDCX_AO: i32 = 1;
pub const MSM8994_VDDCX_VFC: i32 = 2;
pub const MSM8994_VDDMX: i32 = 3;
pub const MSM8994_VDDMX_AO: i32 = 4;
pub const MSM8994_VDDGFX: i32 = 5;
pub const MSM8994_VDDGFX_VFC: i32 = 6;

/* MSM8996 Power Domain Indexes */
pub const MSM8996_VDDCX: i32 = 0;
pub const MSM8996_VDDCX_AO: i32 = 1;
pub const MSM8996_VDDCX_VFC: i32 = 2;
pub const MSM8996_VDDMX: i32 = 3;
pub const MSM8996_VDDMX_AO: i32 = 4;
pub const MSM8996_VDDSSCX: i32 = 5;
pub const MSM8996_VDDSSCX_VFC: i32 = 6;

/* MSM8998 Power Domain Indexes */
pub const MSM8998_VDDCX: i32 = RPMPD_VDDCX;
pub const MSM8998_VDDCX_AO: i32 = RPMPD_VDDCX_AO;
pub const MSM8998_VDDCX_VFL: i32 = RPMPD_VDDCX_VFL;
pub const MSM8998_VDDMX: i32 = RPMPD_VDDMX;
pub const MSM8998_VDDMX_AO: i32 = RPMPD_VDDMX_AO;
pub const MSM8998_VDDMX_VFL: i32 = RPMPD_VDDMX_VFL;
pub const MSM8998_SSCCX: i32 = RPMPD_SSCCX;
pub const MSM8998_SSCCX_VFL: i32 = RPMPD_SSCCX_VFL;
pub const MSM8998_SSCMX: i32 = RPMPD_SSCMX;
pub const MSM8998_SSCMX_VFL: i32 = RPMPD_SSCMX_VFL;

/* QCM2290 Power Domains */
pub const QCM2290_VDDCX: i32 = 0;
pub const QCM2290_VDDCX_AO: i32 = 1;
pub const QCM2290_VDDCX_VFL: i32 = 2;
pub const QCM2290_VDDMX: i32 = 3;
pub const QCM2290_VDDMX_AO: i32 = 4;
pub const QCM2290_VDDMX_VFL: i32 = 5;
pub const QCM2290_VDD_LPI_CX: i32 = 6;
pub const QCM2290_VDD_LPI_MX: i32 = 7;

/* QCS404 Power Domains */
pub const QCS404_VDDMX: i32 = 0;
pub const QCS404_VDDMX_AO: i32 = 1;
pub const QCS404_VDDMX_VFL: i32 = 2;
pub const QCS404_LPICX: i32 = 3;
pub const QCS404_LPICX_VFL: i32 = 4;
pub const QCS404_LPIMX: i32 = 5;
pub const QCS404_LPIMX_VFL: i32 = 6;

/* SDM660 Power Domains */
pub const SDM660_VDDCX: i32 = RPMPD_VDDCX;
pub const SDM660_VDDCX_AO: i32 = RPMPD_VDDCX_AO;
pub const SDM660_VDDCX_VFL: i32 = RPMPD_VDDCX_VFL;
pub const SDM660_VDDMX: i32 = RPMPD_VDDMX;
pub const SDM660_VDDMX_AO: i32 = RPMPD_VDDMX_AO;
pub const SDM660_VDDMX_VFL: i32 = RPMPD_VDDMX_VFL;
pub const SDM660_SSCCX: i32 = RPMPD_SSCCX;
pub const SDM660_SSCCX_VFL: i32 = RPMPD_SSCCX_VFL;
pub const SDM660_SSCMX: i32 = RPMPD_SSCMX;
pub const SDM660_SSCMX_VFL: i32 = RPMPD_SSCMX_VFL;

/* SM6115 Power Domains */
pub const SM6115_VDDCX: i32 = 0;
pub const SM6115_VDDCX_AO: i32 = 1;
pub const SM6115_VDDCX_VFL: i32 = 2;
pub const SM6115_VDDMX: i32 = 3;
pub const SM6115_VDDMX_AO: i32 = 4;
pub const SM6115_VDDMX_VFL: i32 = 5;
pub const SM6115_VDD_LPI_CX: i32 = 6;
pub const SM6115_VDD_LPI_MX: i32 = 7;

/* SM6125 Power Domains */
pub const SM6125_VDDCX: i32 = RPMPD_VDDCX;
pub const SM6125_VDDCX_AO: i32 = RPMPD_VDDCX_AO;
pub const SM6125_VDDCX_VFL: i32 = RPMPD_VDDCX_VFL;
pub const SM6125_VDDMX: i32 = RPMPD_VDDMX;
pub const SM6125_VDDMX_AO: i32 = RPMPD_VDDMX_AO;
pub const SM6125_VDDMX_VFL: i32 = RPMPD_VDDMX_VFL;

/* SM6375 Power Domain Indexes */
pub const SM6375_VDDCX: i32 = 0;
pub const SM6375_VDDCX_AO: i32 = 1;
pub const SM6375_VDDCX_VFL: i32 = 2;
pub const SM6375_VDDMX: i32 = 3;
pub const SM6375_VDDMX_AO: i32 = 4;
pub const SM6375_VDDMX_VFL: i32 = 5;
pub const SM6375_VDDGX: i32 = 6;
pub const SM6375_VDDGX_AO: i32 = 7;
pub const SM6375_VDD_LPI_CX: i32 = 8;
pub const SM6375_VDD_LPI_MX: i32 = 9;

/* RPM SMD Power Domain performance levels */
pub const RPM_SMD_LEVEL_RETENTION: i32 = 16;
pub const RPM_SMD_LEVEL_RETENTION_PLUS: i32 = 32;
pub const RPM_SMD_LEVEL_MIN_SVS: i32 = 48;
pub const RPM_SMD_LEVEL_LOW_SVS: i32 = 64;
pub const RPM_SMD_LEVEL_SVS: i32 = 128;
pub const RPM_SMD_LEVEL_SVS_PLUS: i32 = 192;
pub const RPM_SMD_LEVEL_NOM: i32 = 256;
pub const RPM_SMD_LEVEL_NOM_PLUS: i32 = 320;
pub const RPM_SMD_LEVEL_TURBO: i32 = 384;
pub const RPM_SMD_LEVEL_TURBO_NO_CPR: i32 = 416;
pub const RPM_SMD_LEVEL_TURBO_HIGH: i32 = 448;
pub const RPM_SMD_LEVEL_BINNING: i32 = 512;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
