// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap_hwmod_2xxx_interconnect_data.c - common interconnect data for OMAP2xxx
 *
 * Copyright (C) 2009-2011 Nokia Corporation
 * Paul Walmsley
 *
 * XXX handle crossbar/shared link difference for L3?
 * XXX these should be marked initdata for multi-OMAP kernels
 */

// Dependencies supplied by the surrounding translation unit/module:
// linux/sizes.h, omap_hwmod.h, l3_2xxx.h, l4_2xxx.h,
// omap_hwmod_common_data.h

/* Common interconnect data */

/* L3 -> L4_CORE interface */
pub static mut omap2xxx_l3_main__l4_core: omap_hwmod_ocp_if = omap_hwmod_ocp_if {
    master: unsafe { &raw mut omap2xxx_l3_main_hwmod },
    slave: unsafe { &raw mut omap2xxx_l4_core_hwmod },
    user: OCP_USER_MPU | OCP_USER_SDMA,
};

/* MPU -> L3 interface */
pub static mut omap2xxx_mpu__l3_main: omap_hwmod_ocp_if = omap_hwmod_ocp_if {
    master: unsafe { &raw mut omap2xxx_mpu_hwmod },
    slave: unsafe { &raw mut omap2xxx_l3_main_hwmod },
    user: OCP_USER_MPU,
};

/* DSS -> l3 */
pub static mut omap2xxx_dss__l3: omap_hwmod_ocp_if = omap_hwmod_ocp_if {
    master: unsafe { &raw mut omap2xxx_dss_core_hwmod },
    slave: unsafe { &raw mut omap2xxx_l3_main_hwmod },
    fw: omap_hwmod_ocp_if_fw { omap2: omap_hwmod_ocp_if_fw_omap2 {
        l3_perm_bit: OMAP2_L3_CORE_FW_CONNID_DSS,
        flags: OMAP_FIREWALL_L3,
    } },
    user: OCP_USER_MPU | OCP_USER_SDMA,
};

/* L4_CORE -> L4_WKUP interface */
pub static mut omap2xxx_l4_core__l4_wkup: omap_hwmod_ocp_if = omap_hwmod_ocp_if {
    master: unsafe { &raw mut omap2xxx_l4_core_hwmod }, slave: unsafe { &raw mut omap2xxx_l4_wkup_hwmod }, user: OCP_USER_MPU | OCP_USER_SDMA,
};

macro_rules! ocp_if {
    ($name:ident, $slave:ident, $clk:literal) => {
        pub static mut $name: omap_hwmod_ocp_if = omap_hwmod_ocp_if {
            master: unsafe { &raw mut omap2xxx_l4_core_hwmod },
            slave: unsafe { &raw mut $slave },
            clk: $clk,
            user: OCP_USER_MPU | OCP_USER_SDMA,
        };
    };
}

ocp_if!(omap2_l4_core__uart1, omap2xxx_uart1_hwmod, "uart1_ick");
ocp_if!(omap2_l4_core__uart2, omap2xxx_uart2_hwmod, "uart2_ick");
ocp_if!(omap2_l4_core__uart3, omap2xxx_uart3_hwmod, "uart3_ick");
ocp_if!(omap2xxx_l4_core__mcspi1, omap2xxx_mcspi1_hwmod, "mcspi1_ick");
ocp_if!(omap2xxx_l4_core__mcspi2, omap2xxx_mcspi2_hwmod, "mcspi2_ick");
ocp_if!(omap2xxx_l4_core__timer3, omap2xxx_timer3_hwmod, "gpt3_ick");
ocp_if!(omap2xxx_l4_core__timer4, omap2xxx_timer4_hwmod, "gpt4_ick");
ocp_if!(omap2xxx_l4_core__timer5, omap2xxx_timer5_hwmod, "gpt5_ick");
ocp_if!(omap2xxx_l4_core__timer6, omap2xxx_timer6_hwmod, "gpt6_ick");
ocp_if!(omap2xxx_l4_core__timer7, omap2xxx_timer7_hwmod, "gpt7_ick");
ocp_if!(omap2xxx_l4_core__timer8, omap2xxx_timer8_hwmod, "gpt8_ick");
ocp_if!(omap2xxx_l4_core__timer9, omap2xxx_timer9_hwmod, "gpt9_ick");
ocp_if!(omap2xxx_l4_core__timer10, omap2xxx_timer10_hwmod, "gpt10_ick");
ocp_if!(omap2xxx_l4_core__timer11, omap2xxx_timer11_hwmod, "gpt11_ick");
ocp_if!(omap2xxx_l4_core__timer12, omap2xxx_timer12_hwmod, "gpt12_ick");

macro_rules! fw_ocp_if {
    ($name:ident, $slave:ident, $region:ident, $clk:literal) => {
        pub static mut $name: omap_hwmod_ocp_if = omap_hwmod_ocp_if {
            master: unsafe { &raw mut omap2xxx_l4_core_hwmod }, slave: unsafe { &raw mut $slave }, clk: $clk,
            fw: omap_hwmod_ocp_if_fw { omap2: omap_hwmod_ocp_if_fw_omap2 { l4_fw_region: $region, flags: OMAP_FIREWALL_L4 } },
            user: OCP_USER_MPU | OCP_USER_SDMA,
        };
    };
}

fw_ocp_if!(omap2xxx_l4_core__dss, omap2xxx_dss_core_hwmod, OMAP2420_L4_CORE_FW_DSS_CORE_REGION, "dss_ick");
fw_ocp_if!(omap2xxx_l4_core__dss_dispc, omap2xxx_dss_dispc_hwmod, OMAP2420_L4_CORE_FW_DSS_DISPC_REGION, "dss_ick");
fw_ocp_if!(omap2xxx_l4_core__dss_rfbi, omap2xxx_dss_rfbi_hwmod, OMAP2420_L4_CORE_FW_DSS_CORE_REGION, "dss_ick");

pub static mut omap2xxx_l4_core__dss_venc: omap_hwmod_ocp_if = omap_hwmod_ocp_if {
    master: unsafe { &raw mut omap2xxx_l4_core_hwmod }, slave: unsafe { &raw mut omap2xxx_dss_venc_hwmod }, clk: "dss_ick",
    fw: omap_hwmod_ocp_if_fw { omap2: omap_hwmod_ocp_if_fw_omap2 { l4_fw_region: OMAP2420_L4_CORE_FW_DSS_VENC_REGION, flags: OMAP_FIREWALL_L4 } },
    flags: OCPIF_SWSUP_IDLE, user: OCP_USER_MPU | OCP_USER_SDMA,
};

ocp_if!(omap2xxx_l4_core__rng, omap2xxx_rng_hwmod, "rng_ick");
ocp_if!(omap2xxx_l4_core__sham, omap2xxx_sham_hwmod, "sha_ick");
ocp_if!(omap2xxx_l4_core__aes, omap2xxx_aes_hwmod, "aes_ick");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
