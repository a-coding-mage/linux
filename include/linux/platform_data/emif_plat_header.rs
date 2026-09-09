/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Definitions for TI EMIF device platform data
 *
 * Copyright (C) 2012 Texas Instruments, Inc.
 *
 * Aneesh V <aneesh@ti.com>
 */

// Low power modes - EMIF_PWR_MGMT_CTRL
pub const EMIF_LP_MODE_DISABLE: u32 = 0;
pub const EMIF_LP_MODE_CLOCK_STOP: u32 = 1;
pub const EMIF_LP_MODE_SELF_REFRESH: u32 = 2;
pub const EMIF_LP_MODE_PWR_DN: u32 = 4;

// Hardware capabilities
pub const EMIF_HW_CAPS_LL_INTERFACE: u32 = 0x00000001;

/*
 * EMIF IP Revisions
 *	EMIF4D  - Used in OMAP4
 *	EMIF4D5 - Used in OMAP5
 */
pub const EMIF_4D: u32 = 1;
pub const EMIF_4D5: u32 = 2;

/*
 * PHY types
 *	ATTILAPHY  - Used in OMAP4
 *	INTELLIPHY - Used in OMAP5
 */
pub const EMIF_PHY_TYPE_ATTILAPHY: u32 = 1;
pub const EMIF_PHY_TYPE_INTELLIPHY: u32 = 2;

// Custom config requests
pub const EMIF_CUSTOM_CONFIG_LPMODE: u32 = 0x00000001;
pub const EMIF_CUSTOM_CONFIG_TEMP_ALERT_POLL_INTERVAL: u32 = 0x00000002;
pub const EMIF_CUSTOM_CONFIG_EXTENDED_TEMP_PART: u32 = 0x00000004;

/**
 * struct ddr_device_info - All information about the DDR device except AC
 *		timing parameters
 * @type:	Device type (LPDDR2-S4, LPDDR2-S2 etc)
 * @density:	Device density
 * @io_width:	Bus width
 * @cs1_used:	Whether there is a DDR device attached to the second
 *		chip-select(CS1) of this EMIF instance
 * @cal_resistors_per_cs: Whether there is one calibration resistor per
 *		chip-select or whether it's a single one for both
 * @manufacturer: Manufacturer name string
 */
#[repr(C)]
pub struct ddr_device_info {
    pub type_: u32,
    pub density: u32,
    pub io_width: u32,
    pub cs1_used: u32,
    pub cal_resistors_per_cs: u32,
    pub manufacturer: [i8; 10],
}

/**
 * struct emif_custom_configs - Custom configuration parameters/policies
 *		passed from the platform layer
 * @mask:	Mask to indicate which configs are requested
 * @lpmode:	LPMODE to be used in PWR_MGMT_CTRL register
 * @lpmode_timeout_performance: Timeout before LPMODE entry when higher
 *		performance is desired at the cost of power (typically
 *		at higher OPPs)
 * @lpmode_timeout_power: Timeout before LPMODE entry when better power
 *		savings is desired and performance is not important
 *		(typically at lower loads indicated by lower OPPs)
 * @lpmode_freq_threshold: The DDR frequency threshold to identify between
 *		the above two cases:
 *		timeout = (freq >= lpmode_freq_threshold) ?
 *			lpmode_timeout_performance :
 *			lpmode_timeout_power;
 * @temp_alert_poll_interval_ms: LPDDR2 MR4 polling interval at nominal
 *		temperature(in milliseconds). When temperature is high
 *		polling is done 4 times as frequently.
 */
#[repr(C)]
pub struct emif_custom_configs {
    pub mask: u32,
    pub lpmode: u32,
    pub lpmode_timeout_performance: u32,
    pub lpmode_timeout_power: u32,
    pub lpmode_freq_threshold: u32,
    pub temp_alert_poll_interval_ms: u32,
}

/**
 * struct emif_platform_data - Platform data passed on EMIF platform
 *				device creation. Used by the driver.
 * @hw_caps:		Hw capabilities of the EMIF IP in the respective SoC
 * @device_info:	Device info structure containing information such
 *			as type, bus width, density etc
 * @timings:		Timings information from device datasheet passed
 *			as an array of 'struct lpddr2_timings'. Can be NULL
 *			if if default timings are ok
 * @timings_arr_size:	Size of the timings array. Depends on the number
 *			of different frequencies for which timings data
 *			is provided
 * @min_tck:		Minimum value of some timing parameters in terms
 *			of number of cycles. Can be NULL if default values
 *			are ok
 * @custom_configs:	Custom configurations requested by SoC or board
 *			code and the data for them. Can be NULL if default
 *			configurations done by the driver are ok. See
 *			documentation for 'struct emif_custom_configs' for
 *			more details
 */
#[repr(C)]
pub struct emif_platform_data {
    pub hw_caps: u32,
    pub device_info: *mut ddr_device_info,
    pub timings: *const lpddr2_timings,
    pub timings_arr_size: u32,
    pub min_tck: *const lpddr2_min_tck,
    pub custom_configs: *mut emif_custom_configs,
    pub ip_rev: u32,
    pub phy_type: u32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
