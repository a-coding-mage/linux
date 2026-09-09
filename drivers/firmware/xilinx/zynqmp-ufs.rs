// SPDX-License-Identifier: GPL-2.0
/*
 * Firmware Layer for UFS APIs
 *
 * Copyright (C) 2025 Advanced Micro Devices, Inc.
 */

/* Dependencies supplied by the surrounding firmware implementation. */
extern "C" {
    fn zynqmp_pm_sec_read_reg(node_id: u32, offset: u32, val: *mut u32) -> i32;
    fn zynqmp_pm_sec_mask_write_reg(
        node_id: u32,
        offset: u32,
        mask: u32,
        val: u32,
    ) -> i32;
}

/* Register Node IDs */
const PM_REGNODE_PMC_IOU_SLCR: u32 = 0x30000002; /* PMC IOU SLCR */
const PM_REGNODE_EFUSE_CACHE: u32 = 0x30000003; /* EFUSE Cache */

/* Register Offsets for PMC IOU SLCR */
const SRAM_CSR_OFFSET: u32 = 0x104C; /* SRAM Control and Status */
const TXRX_CFGRDY_OFFSET: u32 = 0x1054; /* M-PHY TX-RX Config ready */

/* Masks for SRAM Control and Status Register */
const SRAM_CSR_INIT_DONE_MASK: u32 = 1 << 0; /* SRAM initialization done */
const SRAM_CSR_EXT_LD_DONE_MASK: u32 = 1 << 1; /* SRAM External load done */
const SRAM_CSR_BYPASS_MASK: u32 = 1 << 2; /* Bypass SRAM interface */

/* Mask to check M-PHY TX-RX configuration readiness */
const TX_RX_CFG_RDY_MASK: u32 = 0xF;

/* Register Offsets for EFUSE Cache */
const UFS_CAL_1_OFFSET: u32 = 0xBE8; /* UFS Calibration Value */

/**
 * zynqmp_pm_is_mphy_tx_rx_config_ready - check M-PHY TX-RX config readiness
 * @is_ready:    Store output status (true/false)
 *
 * Return:    Returns 0 on success or error value on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn zynqmp_pm_is_mphy_tx_rx_config_ready(is_ready: *mut bool) -> i32 {
    let mut regval: u32 = 0;
    let ret: i32;

    if is_ready.is_null() {
        return -22; // -EINVAL
    }

    ret = zynqmp_pm_sec_read_reg(PM_REGNODE_PMC_IOU_SLCR, TXRX_CFGRDY_OFFSET, &mut regval);
    if ret != 0 {
        return ret;
    }

    regval &= TX_RX_CFG_RDY_MASK;
    if regval != 0 {
        *is_ready = true;
    } else {
        *is_ready = false;
    }

    ret
}
// EXPORT_SYMBOL_GPL(zynqmp_pm_is_mphy_tx_rx_config_ready);

/**
 * zynqmp_pm_is_sram_init_done - check SRAM initialization
 * @is_done:    Store output status (true/false)
 *
 * Return:    Returns 0 on success or error value on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn zynqmp_pm_is_sram_init_done(is_done: *mut bool) -> i32 {
    let mut regval: u32 = 0;
    let ret: i32;

    if is_done.is_null() {
        return -22; // -EINVAL
    }

    ret = zynqmp_pm_sec_read_reg(PM_REGNODE_PMC_IOU_SLCR, SRAM_CSR_OFFSET, &mut regval);
    if ret != 0 {
        return ret;
    }

    regval &= SRAM_CSR_INIT_DONE_MASK;
    if regval != 0 {
        *is_done = true;
    } else {
        *is_done = false;
    }

    ret
}
// EXPORT_SYMBOL_GPL(zynqmp_pm_is_sram_init_done);

/**
 * zynqmp_pm_set_sram_bypass - Set SRAM bypass Control
 *
 * Return:    Returns 0 on success or error value on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn zynqmp_pm_set_sram_bypass() -> i32 {
    let mut sram_csr: u32 = 0;
    let ret: i32;

    ret = zynqmp_pm_sec_read_reg(PM_REGNODE_PMC_IOU_SLCR, SRAM_CSR_OFFSET, &mut sram_csr);
    if ret != 0 {
        return ret;
    }

    sram_csr &= !SRAM_CSR_EXT_LD_DONE_MASK;
    sram_csr |= SRAM_CSR_BYPASS_MASK;

    zynqmp_pm_sec_mask_write_reg(
        PM_REGNODE_PMC_IOU_SLCR,
        SRAM_CSR_OFFSET,
        (1 << 2) | (1 << 1),
        sram_csr,
    )
}
// EXPORT_SYMBOL_GPL(zynqmp_pm_set_sram_bypass);

/**
 * zynqmp_pm_get_ufs_calibration_values - Read UFS calibration values
 * @val:    Store the calibration value
 *
 * Return:    Returns 0 on success or error value on failure.
 */
#[no_mangle]
pub unsafe extern "C" fn zynqmp_pm_get_ufs_calibration_values(val: *mut u32) -> i32 {
    zynqmp_pm_sec_read_reg(PM_REGNODE_EFUSE_CACHE, UFS_CAL_1_OFFSET, val)
}
// EXPORT_SYMBOL_GPL(zynqmp_pm_get_ufs_calibration_values);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
