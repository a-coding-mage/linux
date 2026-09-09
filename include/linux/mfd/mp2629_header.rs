/* SPDX-License-Identifier: GPL-2.0+ */
/*
 * Copyright 2020 Monolithic Power Systems, Inc
 */

// Dependencies supplied by the surrounding kernel translation.
pub struct device;
pub struct regmap;

#[repr(C)]
pub struct mp2629_data {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum mp2629_adc_chan {
    MP2629_BATT_VOLT = 0,
    MP2629_SYSTEM_VOLT,
    MP2629_INPUT_VOLT,
    MP2629_BATT_CURRENT,
    MP2629_INPUT_CURRENT,
    MP2629_ADC_CHAN_END,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
