/* SPDX-License-Identifier: GPL-2.0
 *
 * Header file for MCDI FW interaction for CDX bus.
 *
 * Copyright (C) 2022-2023, Advanced Micro Devices, Inc.
 */

// Dependencies supplied by the corresponding C headers:
// <linux/cdx/mcdi.h>, "mcdid.h", and "../cdx.h".

pub struct cdx_mcdi;
pub struct cdx_dev_params;

extern "C" {
    /**
     * cdx_mcdi_get_num_buses - Get the total number of buses on
     *\tthe controller.
     * @cdx: pointer to MCDI interface.
     *
     * Return: total number of buses available on the controller,
     *\t<0 on failure
     */
    pub fn cdx_mcdi_get_num_buses(cdx: *mut cdx_mcdi) -> i32;

    /**
     * cdx_mcdi_get_num_devs - Get the total number of devices on
     *\ta particular bus of the controller.
     * @cdx: pointer to MCDI interface.
     * @bus_num: Bus number.
     *
     * Return: total number of devices available on the bus, <0 on failure
     */
    pub fn cdx_mcdi_get_num_devs(cdx: *mut cdx_mcdi, bus_num: i32) -> i32;

    /**
     * cdx_mcdi_get_dev_config - Get configuration for a particular
     *\tbus_num:dev_num
     * @cdx: pointer to MCDI interface.
     * @bus_num: Bus number.
     * @dev_num: Device number.
     * @dev_params: Pointer to cdx_dev_params, this is populated by this
     *\tdevice with the configuration corresponding to the provided
     *\tbus_num:dev_num.
     *
     * Return: 0 total number of devices available on the bus, <0 on failure
     */
    pub fn cdx_mcdi_get_dev_config(
        cdx: *mut cdx_mcdi,
        bus_num: u8,
        dev_num: u8,
        dev_params: *mut cdx_dev_params,
    ) -> i32;

    /**
     * cdx_mcdi_bus_enable - Enable CDX bus represented by bus_num
     * @cdx: pointer to MCDI interface.
     * @bus_num: Bus number.
     *
     * Return: 0 on success, <0 on failure
     */
    pub fn cdx_mcdi_bus_enable(cdx: *mut cdx_mcdi, bus_num: u8) -> i32;

    /**
     * cdx_mcdi_bus_disable - Disable CDX bus represented by bus_num
     * @cdx: pointer to MCDI interface.
     * @bus_num: Bus number.
     *
     * Return: 0 on success, <0 on failure
     */
    pub fn cdx_mcdi_bus_disable(cdx: *mut cdx_mcdi, bus_num: u8) -> i32;

    /**
     * cdx_mcdi_write_msi - Write MSI configuration for CDX device
     * @cdx: pointer to MCDI interface.
     * @bus_num: Bus number.
     * @dev_num: Device number.
     * @msi_vector: Device-relative MSI vector number.
     *\tMust be < MSI_COUNT reported for the device.
     * @msi_address: MSI address to be used by the hardware. Typically, on ARM
     *\tsystems this address is translated by the IOMMU (if enabled) and
     *\tit is the responsibility of the entity managing the IOMMU (APU kernel)
     *\tto supply the correct IOVA here.
     * @msi_data: MSI data to be used by the hardware. On versal-net, only the
     *\tlower 16-bits are used, the remaining bits are ignored and should be
     *\tset to zero.
     *
     * Return: 0 on success, <0 on failure
     */
    pub fn cdx_mcdi_write_msi(
        cdx: *mut cdx_mcdi,
        bus_num: u8,
        dev_num: u8,
        msi_vector: u32,
        msi_address: u64,
        msi_data: u32,
    ) -> i32;

    /**
     * cdx_mcdi_reset_device - Reset cdx device represented by bus_num:dev_num
     * @cdx: pointer to MCDI interface.
     * @bus_num: Bus number.
     * @dev_num: Device number.
     *
     * Return: 0 on success, <0 on failure
     */
    pub fn cdx_mcdi_reset_device(cdx: *mut cdx_mcdi, bus_num: u8, dev_num: u8) -> i32;

    /**
     * cdx_mcdi_bus_master_enable - Set/Reset bus mastering for cdx device
     *\t\t\t\trepresented by bus_num:dev_num
     * @cdx: pointer to MCDI interface.
     * @bus_num: Bus number.
     * @dev_num: Device number.
     * @enable: Enable bus mastering if set, disable otherwise.
     *
     * Return: 0 on success, <0 on failure
     */
    pub fn cdx_mcdi_bus_master_enable(
        cdx: *mut cdx_mcdi,
        bus_num: u8,
        dev_num: u8,
        enable: bool,
    ) -> i32;

    /**
     * cdx_mcdi_msi_enable - Enable/Disable MSIs for cdx device represented
     *\t\t\t by bus_num:dev_num
     * @cdx: pointer to MCDI interface.
     * @bus_num: Bus number.
     * @dev_num: Device number.
     * @enable: Enable msi's if set, disable otherwise.
     *
     * Return: 0 on success, <0 on failure
     */
    pub fn cdx_mcdi_msi_enable(
        cdx: *mut cdx_mcdi,
        bus_num: u8,
        dev_num: u8,
        enable: bool,
    ) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
