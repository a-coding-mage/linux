/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * omap_hwmod_common_data.h - OMAP hwmod common macros and declarations
 *
 * Copyright (C) 2010-2011 Nokia Corporation
 * Copyright (C) 2010-2012 Texas Instruments, Inc.
 * Paul Walmsley
 * Benoît Cousson
 */

// Dependencies supplied by the corresponding translated headers:
// omap_hwmod.h, common.h, and display.h.

/* Common IP block data across OMAP2xxx */
extern "C" {
    pub static mut omap2xxx_l3_main_hwmod: omap_hwmod;
    pub static mut omap2xxx_l4_core_hwmod: omap_hwmod;
    pub static mut omap2xxx_l4_wkup_hwmod: omap_hwmod;
    pub static mut omap2xxx_mpu_hwmod: omap_hwmod;
    pub static mut omap2xxx_timer3_hwmod: omap_hwmod;
    pub static mut omap2xxx_timer4_hwmod: omap_hwmod;
    pub static mut omap2xxx_timer5_hwmod: omap_hwmod;
    pub static mut omap2xxx_timer6_hwmod: omap_hwmod;
    pub static mut omap2xxx_timer7_hwmod: omap_hwmod;
    pub static mut omap2xxx_timer8_hwmod: omap_hwmod;
    pub static mut omap2xxx_timer9_hwmod: omap_hwmod;
    pub static mut omap2xxx_timer10_hwmod: omap_hwmod;
    pub static mut omap2xxx_timer11_hwmod: omap_hwmod;
    pub static mut omap2xxx_timer12_hwmod: omap_hwmod;
    pub static mut omap2xxx_wd_timer2_hwmod: omap_hwmod;
    pub static mut omap2xxx_uart1_hwmod: omap_hwmod;
    pub static mut omap2xxx_uart2_hwmod: omap_hwmod;
    pub static mut omap2xxx_uart3_hwmod: omap_hwmod;
    pub static mut omap2xxx_dss_core_hwmod: omap_hwmod;
    pub static mut omap2xxx_dss_dispc_hwmod: omap_hwmod;
    pub static mut omap2xxx_dss_rfbi_hwmod: omap_hwmod;
    pub static mut omap2xxx_dss_venc_hwmod: omap_hwmod;
    pub static mut omap2xxx_gpio1_hwmod: omap_hwmod;
    pub static mut omap2xxx_gpio2_hwmod: omap_hwmod;
    pub static mut omap2xxx_gpio3_hwmod: omap_hwmod;
    pub static mut omap2xxx_gpio4_hwmod: omap_hwmod;
    pub static mut omap2xxx_mcspi1_hwmod: omap_hwmod;
    pub static mut omap2xxx_mcspi2_hwmod: omap_hwmod;
    pub static mut omap2xxx_gpmc_hwmod: omap_hwmod;
    pub static mut omap2xxx_rng_hwmod: omap_hwmod;
    pub static mut omap2xxx_sham_hwmod: omap_hwmod;
    pub static mut omap2xxx_aes_hwmod: omap_hwmod;

    /* Common interface data across OMAP2xxx */
    pub static mut omap2xxx_l3_main__l4_core: omap_hwmod_ocp_if;
    pub static mut omap2xxx_mpu__l3_main: omap_hwmod_ocp_if;
    pub static mut omap2xxx_dss__l3: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__l4_wkup: omap_hwmod_ocp_if;
    pub static mut omap2_l4_core__uart1: omap_hwmod_ocp_if;
    pub static mut omap2_l4_core__uart2: omap_hwmod_ocp_if;
    pub static mut omap2_l4_core__uart3: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__mcspi1: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__mcspi2: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__timer3: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__timer4: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__timer5: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__timer6: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__timer7: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__timer8: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__timer9: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__timer10: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__timer11: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__timer12: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__dss: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__dss_dispc: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__dss_rfbi: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__dss_venc: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__rng: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__sham: omap_hwmod_ocp_if;
    pub static mut omap2xxx_l4_core__aes: omap_hwmod_ocp_if;

    /* OMAP hwmod classes - forward declarations */
    pub static mut l3_hwmod_class: omap_hwmod_class;
    pub static mut l4_hwmod_class: omap_hwmod_class;
    pub static mut mpu_hwmod_class: omap_hwmod_class;
    pub static mut iva_hwmod_class: omap_hwmod_class;
    pub static mut omap2_uart_class: omap_hwmod_class;
    pub static mut omap2_dss_hwmod_class: omap_hwmod_class;
    pub static mut omap2_rfbi_hwmod_class: omap_hwmod_class;
    pub static mut omap2_venc_hwmod_class: omap_hwmod_class;
    pub static mut omap2_hdq1w_class: omap_hwmod_class;

    pub static mut omap2xxx_gpio_hwmod_class: omap_hwmod_class;
    pub static mut omap2xxx_mailbox_hwmod_class: omap_hwmod_class;
    pub static mut omap2xxx_mcspi_class: omap_hwmod_class;

    pub static mut omap2_3_dss_dispc_dev_attr: omap_dss_dispc_dev_attr;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
