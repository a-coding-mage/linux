/* SPDX-License-Identifier: GPL-2.0-only */
/* Copyright(c) 2025 Intel Corporation */

// Dependencies supplied by the surrounding translation unit:
// adf_accel_devices.h, adf_cfg_common.h, adf_dc.h, linux/bits.h,
// linux/time.h, and linux/units.h.

/* PCIe configuration space */
pub const ADF_GEN6_BAR_MASK: u32 = (1u32 << 0) | (1u32 << 2) | (1u32 << 4);
pub const ADF_GEN6_SRAM_BAR: u32 = 0;
pub const ADF_GEN6_PMISC_BAR: u32 = 1;
pub const ADF_GEN6_ETR_BAR: u32 = 2;
pub const ADF_6XXX_MAX_ACCELENGINES: u32 = 9;

/* Clocks frequency */
pub const ADF_GEN6_COUNTER_FREQ: u32 = 100 * HZ_PER_MHZ;

/* Physical function fuses */
pub const ADF_GEN6_FUSECTL0_OFFSET: u32 = 0x2c8;
pub const ADF_GEN6_FUSECTL1_OFFSET: u32 = 0x2cc;
pub const ADF_GEN6_FUSECTL4_OFFSET: u32 = 0x2d8;

/* Accelerators */
pub const ADF_GEN6_ACCELERATORS_MASK: u32 = 0x1;
pub const ADF_GEN6_MAX_ACCELERATORS: u32 = 1;

/* MSI-X interrupt */
pub const ADF_GEN6_SMIAPF_RP_X0_MASK_OFFSET: u32 = 0x41a040;
pub const ADF_GEN6_SMIAPF_RP_X1_MASK_OFFSET: u32 = 0x41a044;
pub const ADF_GEN6_SMIAPF_MASK_OFFSET: u32 = 0x41a084;
#[inline]
pub const fn ADF_GEN6_MSIX_RTTABLE_OFFSET(i: u32) -> u32 { 0x409000 + i * 4 }

/* Bank and ring configuration */
pub const ADF_GEN6_NUM_RINGS_PER_BANK: u32 = 2;
pub const ADF_GEN6_NUM_BANKS_PER_VF: u32 = 4;
pub const ADF_GEN6_ETR_MAX_BANKS: u32 = 64;
pub const ADF_GEN6_RX_RINGS_OFFSET: u32 = 1;
pub const ADF_GEN6_TX_RINGS_MASK: u32 = 0x1;

/* Arbiter configuration */
pub const ADF_GEN6_ARB_CONFIG: u32 = (1u32 << 31) | (1u32 << 6) | (1u32 << 0);
pub const ADF_GEN6_ARB_OFFSET: u32 = 0x000;
pub const ADF_GEN6_ARB_WRK_2_SER_MAP_OFFSET: u32 = 0x400;

/* Admin interface configuration */
pub const ADF_GEN6_ADMINMSGUR_OFFSET: u32 = 0x500574;
pub const ADF_GEN6_ADMINMSGLR_OFFSET: u32 = 0x500578;
pub const ADF_GEN6_MAILBOX_BASE_OFFSET: u32 = 0x600970;

/* Anti-rollback */
pub const ADF_GEN6_SVNCHECK_CSR_MSG: u32 = 0x640004;

/* Fuse bits */
pub const ADF_GEN6_ANTI_RB_FUSE_BIT: u32 = 1u32 << 24;
pub const ADF_GEN6_KPT_FUSE_BIT: u32 = 1u32 << 16;

/*
 * Watchdog timers
 * Timeout is in cycles. Clock speed may vary across products but this
 * value should be a few milli-seconds.
 */
pub const ADF_SSM_WDT_DEFAULT_VALUE: u64 = 0x7000000;
pub const ADF_SSM_WDT_PKE_DEFAULT_VALUE: u64 = 0x8000000;
pub const ADF_SSMWDTATHL_OFFSET: u32 = 0x5208;
pub const ADF_SSMWDTATHH_OFFSET: u32 = 0x520c;
pub const ADF_SSMWDTCNVL_OFFSET: u32 = 0x5408;
pub const ADF_SSMWDTCNVH_OFFSET: u32 = 0x540c;
pub const ADF_SSMWDTWCPL_OFFSET: u32 = 0x5608;
pub const ADF_SSMWDTWCPH_OFFSET: u32 = 0x560c;
pub const ADF_SSMWDTUCSL_OFFSET: u32 = 0x5808;
pub const ADF_SSMWDTUCSH_OFFSET: u32 = 0x580c;
pub const ADF_SSMWDTDCPRL_OFFSET: u32 = 0x5a08;
pub const ADF_SSMWDTDCPRH_OFFSET: u32 = 0x5a0c;
pub const ADF_SSMWDTWATL_OFFSET: u32 = 0x5c08;
pub const ADF_SSMWDTWATH_OFFSET: u32 = 0x5c0c;
pub const ADF_SSMWDTPKEL_OFFSET: u32 = 0x5e08;
pub const ADF_SSMWDTPKEH_OFFSET: u32 = 0x5e0c;

/* Ring reset */
pub const ADF_RPRESET_POLL_TIMEOUT_US: u32 = 5 * USEC_PER_SEC;
pub const ADF_RPRESET_POLL_DELAY_US: u32 = 20;
pub const ADF_WQM_CSR_RPRESETCTL_RESET: u32 = 1u32 << 0;
#[inline] pub const fn ADF_WQM_CSR_RPRESETCTL(bank: u32) -> u32 { 0x6000 + bank * 8 }
pub const ADF_WQM_CSR_RPRESETSTS_STATUS: u32 = 1u32 << 0;
#[inline] pub const fn ADF_WQM_CSR_RPRESETSTS(bank: u32) -> u32 { ADF_WQM_CSR_RPRESETCTL(bank) + 4 }

/* Controls and sets up the corresponding ring mode of operation */
#[inline] pub const fn ADF_GEN6_CSR_RINGMODECTL(bank: u32) -> u32 { 0x9000 + bank * 4 }
/* Specifies the traffic class to use for the transactions to/from the ring */
pub const ADF_GEN6_RINGMODECTL_TC_MASK: u32 = 0x7u32 << 16;
pub const ADF_GEN6_RINGMODECTL_TC_DEFAULT: u32 = 0x7;
/* Specifies usage of tc for the transactions to/from this ring */
pub const ADF_GEN6_RINGMODECTL_TC_EN_MASK: u32 = 0x3u32 << 19;
/*
 * Use the value programmed in the tc field for request descriptor
 * and metadata read transactions
 */
pub const ADF_GEN6_RINGMODECTL_TC_EN_OP1: u32 = 0x1;

/* VC0 Resource Control Register */
pub const ADF_GEN6_PVC0CTL_OFFSET: u32 = 0x204;
pub const ADF_GEN6_PVC0CTL_TCVCMAP_OFFSET: u32 = 1;
pub const ADF_GEN6_PVC0CTL_TCVCMAP_MASK: u32 = 0x7fu32 << 1;
pub const ADF_GEN6_PVC0CTL_TCVCMAP_DEFAULT: u32 = 0x3f;
/* VC1 Resource Control Register */
pub const ADF_GEN6_PVC1CTL_OFFSET: u32 = 0x210;
pub const ADF_GEN6_PVC1CTL_TCVCMAP_OFFSET: u32 = 1;
pub const ADF_GEN6_PVC1CTL_TCVCMAP_MASK: u32 = 0x7fu32 << 1;
pub const ADF_GEN6_PVC1CTL_TCVCMAP_DEFAULT: u32 = 0x40;
pub const ADF_GEN6_PVC1CTL_VCEN_OFFSET: u32 = 31;
pub const ADF_GEN6_PVC1CTL_VCEN_MASK: u32 = 1u32 << 31;
/* RW bit: 0x1 - enables a Virtual Channel, 0x0 - disables */
pub const ADF_GEN6_PVC1CTL_VCEN_ON: u32 = 0x1;

/* Error source mask registers */
pub const ADF_GEN6_ERRMSK0: u32 = 0x41a210;
pub const ADF_GEN6_ERRMSK1: u32 = 0x41a214;
pub const ADF_GEN6_ERRMSK2: u32 = 0x41a218;
pub const ADF_GEN6_ERRMSK3: u32 = 0x41a21c;
pub const ADF_GEN6_VFLNOTIFY: u32 = 1u32 << 7;

/* Number of heartbeat counter pairs */
pub const ADF_NUM_HB_CNT_PER_AE: u32 = ADF_NUM_THREADS_PER_AE;

/* Rate Limiting */
pub const ADF_GEN6_RL_R2L_OFFSET: u32 = 0x508000;
pub const ADF_GEN6_RL_L2C_OFFSET: u32 = 0x509000;
pub const ADF_GEN6_RL_C2S_OFFSET: u32 = 0x508818;
pub const ADF_GEN6_RL_TOKEN_PCIEIN_BUCKET_OFFSET: u32 = 0x508800;
pub const ADF_GEN6_RL_TOKEN_PCIEOUT_BUCKET_OFFSET: u32 = 0x508804;

/* Physical function fuses */
pub const ADF_6XXX_ACCELENGINES_MASK: u32 = 0x1ff;
pub const ADF_6XXX_ADMIN_AE_MASK: u32 = 0x1ff;

/* Firmware binaries */
pub const ADF_6XXX_FW: &str = "qat_6xxx.bin";
pub const ADF_6XXX_MMP: &str = "qat_6xxx_mmp.bin";
pub const ADF_6XXX_CY_OBJ: &str = "qat_6xxx_cy.bin";
pub const ADF_6XXX_DC_OBJ: &str = "qat_6xxx_dc.bin";
pub const ADF_6XXX_ADMIN_OBJ: &str = "qat_6xxx_admin.bin";
pub const ADF_6XXX_WCY_OBJ: &str = "qat_6xxx_wcy.bin";

/* RL constants */
pub const ADF_6XXX_RL_PCIE_SCALE_FACTOR_DIV: u32 = 100;
pub const ADF_6XXX_RL_PCIE_SCALE_FACTOR_MUL: u32 = 102;
pub const ADF_6XXX_RL_SCANS_PER_SEC: u32 = 954;
pub const ADF_6XXX_RL_MAX_TP_ASYM: u32 = 173750;
pub const ADF_6XXX_RL_MAX_TP_SYM: u32 = 95000;
pub const ADF_6XXX_RL_MAX_TP_DC: u32 = 40000;
pub const ADF_6XXX_RL_MAX_TP_DECOMP: u32 = 40000;
pub const ADF_6XXX_RL_SLICE_REF: u32 = 1000;

/* Clock frequency */
pub const ADF_6XXX_AE_FREQ: u32 = 1000 * HZ_PER_MHZ;

/* KPT */
pub const ADF_6XXX_KPT_MAX_SWK_COUNT_PER_FNPASID: u32 = 128;
pub const ADF_6XXX_KPT_MAX_SWK_TTL: u32 = 31536000;
pub const ADF_6XXX_KPT_DEFAULT_SWK_SHARED_MODE: u32 = 1;
pub const ADF_6XXX_KPT_DEFAULT_SWK_TTL: u32 = 0;
pub const ADF_6XXX_KPT_DEFAULT_SWK_CNT_PER_FN: u32 = 0;
pub const ADF_6XXX_KPT_DEFAULT_SWK_CNT_PER_PASID: u32 = 0;

#[repr(u32)]
pub enum icp_qat_gen6_slice_mask {
    ICP_ACCEL_GEN6_MASK_UCS_SLICE = 1u32 << 0,
    ICP_ACCEL_GEN6_MASK_AUTH_SLICE = 1u32 << 1,
    ICP_ACCEL_GEN6_MASK_PKE_SLICE = 1u32 << 2,
    ICP_ACCEL_GEN6_MASK_CPR_SLICE = 1u32 << 3,
    ICP_ACCEL_GEN6_MASK_DCPRZ_SLICE = 1u32 << 4,
    ICP_ACCEL_GEN6_MASK_EIA3_SLICE = 1u32 << 5,
    ICP_ACCEL_GEN6_MASK_WCP_WAT_SLICE = 1u32 << 6,
    ICP_ACCEL_GEN6_MASK_ZUC_256_SLICE = 1u32 << 7,
    ICP_ACCEL_GEN6_MASK_5G_SLICE = 1u32 << 8,
}

/* Return true if the device is a wireless crypto (WCY) SKU */
#[inline]
pub unsafe fn adf_6xxx_is_wcy(hw_data: *mut adf_hw_device_data) -> bool {
    !((*hw_data).fuses[ADF_FUSECTL1] & (ICP_ACCEL_GEN6_MASK_WCP_WAT_SLICE as u32) != 0)
}

extern "C" {
    pub fn adf_init_hw_data_6xxx(hw_data: *mut adf_hw_device_data);
    pub fn adf_clean_hw_data_6xxx(hw_data: *mut adf_hw_device_data);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
