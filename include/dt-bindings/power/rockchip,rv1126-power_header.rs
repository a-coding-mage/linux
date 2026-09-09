/* SPDX-License-Identifier: GPL-2.0 */

// Device-tree power-domain bindings for RV1126.

/* VD_CORE */
pub const RV1126_PD_CPU_0: u32 = 0;
pub const RV1126_PD_CPU_1: u32 = 1;
pub const RV1126_PD_CPU_2: u32 = 2;
pub const RV1126_PD_CPU_3: u32 = 3;
pub const RV1126_PD_CORE_ALIVE: u32 = 4;

/* VD_PMU */
pub const RV1126_PD_PMU: u32 = 5;
pub const RV1126_PD_PMU_ALIVE: u32 = 6;

/* VD_NPU */
pub const RV1126_PD_NPU: u32 = 7;

/* VD_VEPU */
pub const RV1126_PD_VEPU: u32 = 8;

/* VD_LOGIC */
pub const RV1126_PD_VI: u32 = 9;
pub const RV1126_PD_VO: u32 = 10;
pub const RV1126_PD_ISPP: u32 = 11;
pub const RV1126_PD_VDPU: u32 = 12;
pub const RV1126_PD_CRYPTO: u32 = 13;
pub const RV1126_PD_DDR: u32 = 14;
pub const RV1126_PD_NVM: u32 = 15;
pub const RV1126_PD_SDIO: u32 = 16;
pub const RV1126_PD_USB: u32 = 17;
pub const RV1126_PD_LOGIC_ALIVE: u32 = 18;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
