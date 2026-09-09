/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2020 Intel Corporation */

// Dependencies supplied by the surrounding translation unit:
// adf_accel_devices.h, adf_cfg_common.h, adf_dc.h, and linux/units.h.

/* PCIe configuration space */
pub const ADF_GEN4_BAR_MASK: u32 = (1u32 << 0) | (1u32 << 2) | (1u32 << 4);
pub const ADF_GEN4_SRAM_BAR: u32 = 0;
pub const ADF_GEN4_PMISC_BAR: u32 = 1;
pub const ADF_GEN4_ETR_BAR: u32 = 2;

/* Clocks frequency */
pub const ADF_GEN4_KPT_COUNTER_FREQ: u32 = 100 * HZ_PER_MHZ;

/* Physical function fuses */
pub const ADF_GEN4_FUSECTL0_OFFSET: u32 = 0x2C8;
pub const ADF_GEN4_FUSECTL1_OFFSET: u32 = 0x2CC;
pub const ADF_GEN4_FUSECTL2_OFFSET: u32 = 0x2D0;
pub const ADF_GEN4_FUSECTL3_OFFSET: u32 = 0x2D4;
pub const ADF_GEN4_FUSECTL4_OFFSET: u32 = 0x2D8;
pub const ADF_GEN4_FUSECTL5_OFFSET: u32 = 0x2DC;

/* Accelerators */
pub const ADF_GEN4_ACCELERATORS_MASK: u32 = 0x1;
pub const ADF_GEN4_MAX_ACCELERATORS: u32 = 1;
pub const ADF_GEN4_ADMIN_ACCELENGINES: u32 = 1;

/* MSIX interrupt */
pub const ADF_GEN4_SMIAPF_RP_X0_MASK_OFFSET: u32 = 0x41A040;
pub const ADF_GEN4_SMIAPF_RP_X1_MASK_OFFSET: u32 = 0x41A044;
pub const ADF_GEN4_SMIAPF_MASK_OFFSET: u32 = 0x41A084;
#[inline]
pub const fn ADF_GEN4_MSIX_RTTABLE_OFFSET(i: u32) -> u32 { 0x409000 + i * 0x04 }

/* Bank and ring configuration */
pub const ADF_GEN4_MAX_RPS: u32 = 64;
pub const ADF_GEN4_NUM_RINGS_PER_BANK: u32 = 2;
pub const ADF_GEN4_NUM_BANKS_PER_VF: usize = 4;
pub const ADF_GEN4_ETR_MAX_BANKS: u32 = 64;
pub const ADF_GEN4_RX_RINGS_OFFSET: u32 = 1;
pub const ADF_GEN4_TX_RINGS_MASK: u32 = 0x1;

/* Arbiter configuration */
pub const ADF_GEN4_ARB_CONFIG: u32 = (1u32 << 31) | (1u32 << 6) | (1u32 << 0);
pub const ADF_GEN4_ARB_OFFSET: u32 = 0x0;
pub const ADF_GEN4_ARB_WRK_2_SER_MAP_OFFSET: u32 = 0x400;

/* Admin Interface Reg Offset */
pub const ADF_GEN4_ADMINMSGUR_OFFSET: u32 = 0x500574;
pub const ADF_GEN4_ADMINMSGLR_OFFSET: u32 = 0x500578;
pub const ADF_GEN4_MAILBOX_BASE_OFFSET: u32 = 0x600970;

/* Default ring mapping */
pub const ADF_GEN4_DEFAULT_RING_TO_SRV_MAP: u32 =
    (ASYM << ADF_CFG_SERV_RING_PAIR_0_SHIFT) |
    (SYM << ADF_CFG_SERV_RING_PAIR_1_SHIFT) |
    (ASYM << ADF_CFG_SERV_RING_PAIR_2_SHIFT) |
    (SYM << ADF_CFG_SERV_RING_PAIR_3_SHIFT);

/* WDT timers
 *
 * Timeout is in cycles. Clock speed may vary across products but this
 * value should be a few milli-seconds.
 */
pub const ADF_SSM_WDT_DEFAULT_VALUE: u64 = 0x7000000u64;
pub const ADF_SSM_WDT_PKE_DEFAULT_VALUE: u32 = 0x8000000;
pub const ADF_SSMWDTL_OFFSET: u32 = 0x54;
pub const ADF_SSMWDTH_OFFSET: u32 = 0x5C;
pub const ADF_SSMWDTPKEL_OFFSET: u32 = 0x58;
pub const ADF_SSMWDTPKEH_OFFSET: u32 = 0x60;

/* Ring reset */
pub const ADF_RPRESET_POLL_TIMEOUT_US: u32 = 5 * USEC_PER_SEC;
pub const ADF_RPRESET_POLL_DELAY_US: u32 = 20;
pub const ADF_WQM_CSR_RPRESETCTL_RESET: u32 = 1u32 << 0;
pub const ADF_WQM_CSR_RPRESETCTL_DRAIN: u32 = 1u32 << 2;
#[inline]
pub const fn ADF_WQM_CSR_RPRESETCTL(bank: u32) -> u32 { 0x6000 + (bank << 3) }
pub const ADF_WQM_CSR_RPRESETSTS_STATUS: u32 = 1u32 << 0;
#[inline]
pub const fn ADF_WQM_CSR_RPRESETSTS(bank: u32) -> u32 { ADF_WQM_CSR_RPRESETCTL(bank) + 4 }

/* Ring interrupt */
pub const ADF_COALESCED_POLL_TIMEOUT_US: u32 = USEC_PER_SEC;
pub const ADF_COALESCED_POLL_DELAY_US: u32 = 1000;
#[inline]
pub const fn ADF_WQM_CSR_RPINTSOU(bank: u32) -> u32 { 0x200000 + (bank << 12) }
pub const ADF_WQM_CSR_RP_IDX_RX: u32 = 1;

/* Error source registers */
pub const ADF_GEN4_ERRSOU0: u32 = 0x41A200;
pub const ADF_GEN4_ERRSOU1: u32 = 0x41A204;
pub const ADF_GEN4_ERRSOU2: u32 = 0x41A208;
pub const ADF_GEN4_ERRSOU3: u32 = 0x41A20C;

/* Error source mask registers */
pub const ADF_GEN4_ERRMSK0: u32 = 0x41A210;
pub const ADF_GEN4_ERRMSK1: u32 = 0x41A214;
pub const ADF_GEN4_ERRMSK2: u32 = 0x41A218;
pub const ADF_GEN4_ERRMSK3: u32 = 0x41A21C;
pub const ADF_GEN4_VFLNOTIFY: u32 = 1u32 << 7;

/* Number of heartbeat counter pairs */
pub const ADF_NUM_HB_CNT_PER_AE: u32 = ADF_NUM_THREADS_PER_AE;

/* Rate Limiting */
pub const ADF_GEN4_RL_R2L_OFFSET: u32 = 0x508000;
pub const ADF_GEN4_RL_L2C_OFFSET: u32 = 0x509000;
pub const ADF_GEN4_RL_C2S_OFFSET: u32 = 0x508818;
pub const ADF_GEN4_RL_TOKEN_PCIEIN_BUCKET_OFFSET: u32 = 0x508800;
pub const ADF_GEN4_RL_TOKEN_PCIEOUT_BUCKET_OFFSET: u32 = 0x508804;

/* Arbiter threads mask with error value */
pub const ADF_GEN4_ENA_THD_MASK_ERROR: u32 = GENMASK(ADF_NUM_THREADS_PER_AE, 0);

/* PF2VM communication channel */
#[inline] pub const fn ADF_GEN4_PF2VM_OFFSET(i: u32) -> u32 { 0x40B010 + i * 0x20 }
#[inline] pub const fn ADF_GEN4_VM2PF_OFFSET(i: u32) -> u32 { 0x40B014 + i * 0x20 }
#[inline] pub const fn ADF_GEN4_VINTMSKPF2VM_OFFSET(i: u32) -> u32 { 0x40B00C + i * 0x20 }
#[inline] pub const fn ADF_GEN4_VINTSOUPF2VM_OFFSET(i: u32) -> u32 { 0x40B008 + i * 0x20 }
#[inline] pub const fn ADF_GEN4_VINTMSK_OFFSET(i: u32) -> u32 { 0x40B004 + i * 0x20 }
#[inline] pub const fn ADF_GEN4_VINTSOU_OFFSET(i: u32) -> u32 { 0x40B000 + i * 0x20 }

#[repr(C)]
pub struct adf_gen4_vfmig {
    pub mstate_mgr: *mut adf_mstate_mgr,
    pub bank_stopped: [bool; ADF_GEN4_NUM_BANKS_PER_VF],
}

unsafe extern "C" {
    pub fn adf_gen4_set_ssm_wdtimer(accel_dev: *mut adf_accel_dev);
    pub fn adf_gen4_enable_error_correction(accel_dev: *mut adf_accel_dev);
    pub fn adf_gen4_enable_ints(accel_dev: *mut adf_accel_dev);
    pub fn adf_gen4_get_accel_mask(self_: *mut adf_hw_device_data) -> u32;
    pub fn adf_gen4_get_admin_info(admin_csrs_info: *mut admin_info);
    pub fn adf_gen4_get_arb_info(arb_info: *mut arb_info);
    pub fn adf_gen4_get_etr_bar_id(self_: *mut adf_hw_device_data) -> u32;
    pub fn adf_gen4_get_heartbeat_clock(self_: *mut adf_hw_device_data) -> u32;
    pub fn adf_gen4_get_misc_bar_id(self_: *mut adf_hw_device_data) -> u32;
    pub fn adf_gen4_get_num_accels(self_: *mut adf_hw_device_data) -> u32;
    pub fn adf_gen4_get_num_aes(self_: *mut adf_hw_device_data) -> u32;
    pub fn adf_gen4_get_sku(self_: *mut adf_hw_device_data) -> dev_sku_info;
    pub fn adf_gen4_get_sram_bar_id(self_: *mut adf_hw_device_data) -> u32;
    pub fn adf_gen4_init_device(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_gen4_ring_pair_reset(accel_dev: *mut adf_accel_dev, bank_number: u32) -> i32;
    pub fn adf_gen4_set_msix_default_rttable(accel_dev: *mut adf_accel_dev);
    pub fn adf_gen4_init_thd2arb_map(accel_dev: *mut adf_accel_dev) -> i32;
    pub fn adf_gen4_get_ring_to_svc_map(accel_dev: *mut adf_accel_dev) -> u16;
    pub fn adf_gen4_bank_quiesce_coal_timer(accel_dev: *mut adf_accel_dev, bank_idx: u32, timeout_ms: i32) -> i32;
    pub fn adf_gen4_bank_drain_start(accel_dev: *mut adf_accel_dev, bank_number: u32, timeout_us: i32) -> i32;
    pub fn adf_gen4_bank_drain_finish(accel_dev: *mut adf_accel_dev, bank_number: u32);
    pub fn adf_gen4_services_supported(service_mask: c_ulong) -> bool;
    pub fn adf_gen4_init_dc_ops(dc_ops: *mut adf_dc_ops);
    pub fn adf_gen4_init_num_svc_aes(device_data: *mut adf_rl_hw_data);
    pub fn adf_gen4_get_svc_slice_cnt(accel_dev: *mut adf_accel_dev, svc: adf_base_services) -> u32;
}

#[repr(C)]
pub enum icp_qat_gen4_slice_mask {
    ICP_ACCEL_GEN4_MASK_CIPHER_SLICE = 1u32 << 0,
    ICP_ACCEL_GEN4_MASK_AUTH_SLICE = 1u32 << 1,
    ICP_ACCEL_GEN4_MASK_PKE_SLICE = 1u32 << 2,
    ICP_ACCEL_GEN4_MASK_COMPRESS_SLICE = 1u32 << 3,
    ICP_ACCEL_GEN4_MASK_UCS_SLICE = 1u32 << 4,
    ICP_ACCEL_GEN4_MASK_EIA3_SLICE = 1u32 << 5,
    ICP_ACCEL_GEN4_MASK_SMX_SLICE = 1u32 << 7,
    ICP_ACCEL_GEN4_MASK_WCP_WAT_SLICE = 1u32 << 8,
    ICP_ACCEL_GEN4_MASK_ZUC_256_SLICE = 1u32 << 9,
}

#[repr(C)]
pub enum adf_gen4_rp_groups { RP_GROUP_0, RP_GROUP_1, RP_GROUP_COUNT }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
