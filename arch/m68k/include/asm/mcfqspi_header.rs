/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Definitions for Freescale Coldfire QSPI module
 *
 * Copyright 2010 Steven King <sfking@fdwdc.com>
 */

/**
 * struct mcfqspi_cs_control - chip select control for the coldfire qspi driver
 * @setup: setup the control; allocate gpio's, etc. May be NULL.
 * @teardown: finish with the control; free gpio's, etc. May be NULL.
 * @select: output the signals to select the device.  Can not be NULL.
 * @deselect: output the signals to deselect the device. Can not be NULL.
 *
 * The QSPI module has 4 hardware chip selects.  We don't use them.  Instead
 * platforms are required to supply a mcfqspi_cs_control as a part of the
 * platform data for each QSPI master controller.  Only the select and
 * deselect functions are required.
 */
#[repr(C)]
pub struct mcfqspi_cs_control {
    pub setup: Option<unsafe extern "C" fn(*mut mcfqspi_cs_control) -> i32>,
    pub teardown: Option<unsafe extern "C" fn(*mut mcfqspi_cs_control)>,
    pub select: Option<unsafe extern "C" fn(*mut mcfqspi_cs_control, u8, bool)>,
    pub deselect: Option<unsafe extern "C" fn(*mut mcfqspi_cs_control, u8, bool)>,
}

/**
 * struct mcfqspi_platform_data - platform data for the coldfire qspi driver
 * @bus_num: board specific identifier for this qspi driver.
 * @num_chipselects: number of chip selects supported by this qspi driver.
 * @cs_control: platform dependent chip select control.
 */
#[repr(C)]
pub struct mcfqspi_platform_data {
    pub bus_num: i16,
    pub num_chipselect: u16,
    pub cs_control: *mut mcfqspi_cs_control,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
