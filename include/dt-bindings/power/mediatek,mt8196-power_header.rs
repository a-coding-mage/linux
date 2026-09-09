/* SPDX-License-Identifier: (GPL-2.0 OR MIT) */
/*
 * Copyright (c) 2025 Collabora Ltd
 *                    AngeloGioacchino Del Regno <angelogioacchino.delregno@collabora.com>
 */

/* SCPSYS Secure Power Manager - Direct Control */
pub mod scpsys_secure_power_manager_direct_control {
    pub const MT8196_POWER_DOMAIN_MD: i32 = 0;
    pub const MT8196_POWER_DOMAIN_CONN: i32 = 1;
    pub const MT8196_POWER_DOMAIN_SSUSB_P0: i32 = 2;
    pub const MT8196_POWER_DOMAIN_SSUSB_DP_PHY_P0: i32 = 3;
    pub const MT8196_POWER_DOMAIN_SSUSB_P1: i32 = 4;
    pub const MT8196_POWER_DOMAIN_SSUSB_P23: i32 = 5;
    pub const MT8196_POWER_DOMAIN_SSUSB_PHY_P2: i32 = 6;
    pub const MT8196_POWER_DOMAIN_PEXTP_MAC0: i32 = 7;
    pub const MT8196_POWER_DOMAIN_PEXTP_MAC1: i32 = 8;
    pub const MT8196_POWER_DOMAIN_PEXTP_MAC2: i32 = 9;
    pub const MT8196_POWER_DOMAIN_PEXTP_PHY0: i32 = 10;
    pub const MT8196_POWER_DOMAIN_PEXTP_PHY1: i32 = 11;
    pub const MT8196_POWER_DOMAIN_PEXTP_PHY2: i32 = 12;
    pub const MT8196_POWER_DOMAIN_AUDIO: i32 = 13;
    pub const MT8196_POWER_DOMAIN_ADSP_TOP_DORMANT: i32 = 14;
    pub const MT8196_POWER_DOMAIN_ADSP_INFRA: i32 = 15;
    pub const MT8196_POWER_DOMAIN_ADSP_AO: i32 = 16;
}

/* SCPSYS Secure Power Manager - HW Voter */
pub mod scpsys_secure_power_manager_hw_voter {
    pub const MT8196_POWER_DOMAIN_MM_PROC_DORMANT: i32 = 0;
    pub const MT8196_POWER_DOMAIN_SSR: i32 = 1;
}

/* HFRPSYS Multimedia Power Control (MMPC) - Direct Control */
pub mod hfrpsys_multimedia_power_control_direct_control {
    pub const MT8196_POWER_DOMAIN_EDPTX: i32 = 0;
    pub const MT8196_POWER_DOMAIN_DPTX: i32 = 1;
}

/* HFRPSYS MultiMedia Power Control (MMPC) - HW Voter */
pub mod hfrpsys_multimedia_power_control_hw_voter {
    pub const MT8196_POWER_DOMAIN_VDE0: i32 = 0;
    pub const MT8196_POWER_DOMAIN_VDE1: i32 = 1;
    pub const MT8196_POWER_DOMAIN_VDE_VCORE0: i32 = 2;
    pub const MT8196_POWER_DOMAIN_VEN0: i32 = 3;
    pub const MT8196_POWER_DOMAIN_VEN1: i32 = 4;
    pub const MT8196_POWER_DOMAIN_VEN2: i32 = 5;
    pub const MT8196_POWER_DOMAIN_DISP_VCORE: i32 = 6;
    pub const MT8196_POWER_DOMAIN_DIS0_DORMANT: i32 = 7;
    pub const MT8196_POWER_DOMAIN_DIS1_DORMANT: i32 = 8;
    pub const MT8196_POWER_DOMAIN_OVL0_DORMANT: i32 = 9;
    pub const MT8196_POWER_DOMAIN_OVL1_DORMANT: i32 = 10;
    pub const MT8196_POWER_DOMAIN_DISP_EDPTX_DORMANT: i32 = 11;
    pub const MT8196_POWER_DOMAIN_DISP_DPTX_DORMANT: i32 = 12;
    pub const MT8196_POWER_DOMAIN_MML0_SHUTDOWN: i32 = 13;
    pub const MT8196_POWER_DOMAIN_MML1_SHUTDOWN: i32 = 14;
    pub const MT8196_POWER_DOMAIN_MM_INFRA0: i32 = 15;
    pub const MT8196_POWER_DOMAIN_MM_INFRA1: i32 = 16;
    pub const MT8196_POWER_DOMAIN_MM_INFRA_AO: i32 = 17;
    pub const MT8196_POWER_DOMAIN_CSI_BS_RX: i32 = 18;
    pub const MT8196_POWER_DOMAIN_CSI_LS_RX: i32 = 19;
    pub const MT8196_POWER_DOMAIN_DSI_PHY0: i32 = 20;
    pub const MT8196_POWER_DOMAIN_DSI_PHY1: i32 = 21;
    pub const MT8196_POWER_DOMAIN_DSI_PHY2: i32 = 22;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
