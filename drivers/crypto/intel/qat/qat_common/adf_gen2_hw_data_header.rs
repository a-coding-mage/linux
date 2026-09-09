/* SPDX-License-Identifier: (BSD-3-Clause OR GPL-2.0-only) */
/* Copyright(c) 2020 Intel Corporation */

// C dependencies: adf_accel_devices.h, adf_cfg_common.h

pub const ADF_GEN2_RX_RINGS_OFFSET: u32 = 8;
pub const ADF_GEN2_TX_RINGS_MASK: u32 = 0xFF;

/* AE to function map */
pub const AE2FUNCTION_MAP_A_OFFSET: u32 = 0x3A400 + 0x190;
pub const AE2FUNCTION_MAP_B_OFFSET: u32 = 0x3A400 + 0x310;
pub const AE2FUNCTION_MAP_REG_SIZE: u32 = 4;
pub const AE2FUNCTION_MAP_VALID: u32 = BIT(7);

#[macro_export]
macro_rules! READ_CSR_AE2FUNCTION_MAP_A {
    ($pmisc_bar_addr:expr, $index:expr) => {
        ADF_CSR_RD($pmisc_bar_addr, AE2FUNCTION_MAP_A_OFFSET + AE2FUNCTION_MAP_REG_SIZE * ($index))
    };
}
#[macro_export]
macro_rules! WRITE_CSR_AE2FUNCTION_MAP_A {
    ($pmisc_bar_addr:expr, $index:expr, $value:expr) => {
        ADF_CSR_WR($pmisc_bar_addr, AE2FUNCTION_MAP_A_OFFSET + AE2FUNCTION_MAP_REG_SIZE * ($index), $value)
    };
}
#[macro_export]
macro_rules! READ_CSR_AE2FUNCTION_MAP_B {
    ($pmisc_bar_addr:expr, $index:expr) => {
        ADF_CSR_RD($pmisc_bar_addr, AE2FUNCTION_MAP_B_OFFSET + AE2FUNCTION_MAP_REG_SIZE * ($index))
    };
}
#[macro_export]
macro_rules! WRITE_CSR_AE2FUNCTION_MAP_B {
    ($pmisc_bar_addr:expr, $index:expr, $value:expr) => {
        ADF_CSR_WR($pmisc_bar_addr, AE2FUNCTION_MAP_B_OFFSET + AE2FUNCTION_MAP_REG_SIZE * ($index), $value)
    };
}

/* Admin Interface Offsets */
pub const ADF_ADMINMSGUR_OFFSET: u32 = 0x3A000 + 0x574;
pub const ADF_ADMINMSGLR_OFFSET: u32 = 0x3A000 + 0x578;
pub const ADF_MAILBOX_BASE_OFFSET: u32 = 0x20970;

/* Arbiter configuration */
pub const ADF_ARB_OFFSET: u32 = 0x30000;
pub const ADF_ARB_WRK_2_SER_MAP_OFFSET: u32 = 0x180;
pub const ADF_ARB_CONFIG: u32 = BIT(31) | BIT(6) | BIT(0);

/* Power gating */
pub const ADF_POWERGATE_DC: u32 = BIT(23);
pub const ADF_POWERGATE_PKE: u32 = BIT(24);

/* Default ring mapping */
pub const ADF_GEN2_DEFAULT_RING_TO_SRV_MAP: u32 =
    CRYPTO << ADF_CFG_SERV_RING_PAIR_0_SHIFT |
    CRYPTO << ADF_CFG_SERV_RING_PAIR_1_SHIFT |
    UNUSED << ADF_CFG_SERV_RING_PAIR_2_SHIFT |
    COMP << ADF_CFG_SERV_RING_PAIR_3_SHIFT;

/* WDT timers
 *
 * Timeout is in cycles. Clock speed may vary across products but this
 * value should be a few milli-seconds.
 */
pub const ADF_SSM_WDT_DEFAULT_VALUE: u32 = 0x200000;
pub const ADF_SSM_WDT_PKE_DEFAULT_VALUE: u32 = 0x2000000;
pub const ADF_SSMWDT_OFFSET: u32 = 0x54;
pub const ADF_SSMWDTPKE_OFFSET: u32 = 0x58;

#[inline]
pub const fn ADF_SSMWDT(i: u32) -> u32 { ADF_SSMWDT_OFFSET + i * 0x4000 }
#[inline]
pub const fn ADF_SSMWDTPKE(i: u32) -> u32 { ADF_SSMWDTPKE_OFFSET + i * 0x4000 }

/* Error detection and correction */
#[inline]
pub const fn ADF_GEN2_AE_CTX_ENABLES(i: u32) -> u32 { i * 0x1000 + 0x20818 }
#[inline]
pub const fn ADF_GEN2_AE_MISC_CONTROL(i: u32) -> u32 { i * 0x1000 + 0x20960 }
pub const ADF_GEN2_ENABLE_AE_ECC_ERR: u32 = BIT(28);
pub const ADF_GEN2_ENABLE_AE_ECC_PARITY_CORR: u32 = BIT(24) | BIT(12);
#[inline]
pub const fn ADF_GEN2_UERRSSMSH(i: u32) -> u32 { i * 0x4000 + 0x18 }
#[inline]
pub const fn ADF_GEN2_CERRSSMSH(i: u32) -> u32 { i * 0x4000 + 0x10 }
pub const ADF_GEN2_ERRSSMSH_EN: u32 = BIT(3);

/* Number of heartbeat counter pairs */
pub const ADF_NUM_HB_CNT_PER_AE: u32 = ADF_NUM_THREADS_PER_AE;

/* Interrupts */
pub const ADF_GEN2_SMIAPF0_MASK_OFFSET: u32 = 0x3A000 + 0x28;
pub const ADF_GEN2_SMIAPF1_MASK_OFFSET: u32 = 0x3A000 + 0x30;
pub const ADF_GEN2_SMIA1_MASK: u32 = 0x1;

extern "C" {
    pub fn adf_gen2_get_num_accels(self_: *mut adf_hw_device_data) -> u32;
    pub fn adf_gen2_get_num_aes(self_: *mut adf_hw_device_data) -> u32;
    pub fn adf_gen2_enable_error_correction(accel_dev: *mut adf_accel_dev);
    pub fn adf_gen2_cfg_iov_thds(accel_dev: *mut adf_accel_dev, enable: bool, num_a_regs: i32, num_b_regs: i32);
    pub fn adf_gen2_get_admin_info(admin_csrs_info: *mut admin_info);
    pub fn adf_gen2_get_arb_info(arb_info: *mut arb_info);
    pub fn adf_gen2_enable_ints(accel_dev: *mut adf_accel_dev);
    pub fn adf_gen2_get_accel_cap(accel_dev: *mut adf_accel_dev) -> u32;
    pub fn adf_gen2_set_ssm_wdtimer(accel_dev: *mut adf_accel_dev);
    pub fn adf_gen2_init_dc_ops(dc_ops: *mut adf_dc_ops);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
